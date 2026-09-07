//! The position-visibility measurement, re-taken against the **built adapter**.
//!
//! Phase 2 measured four SQL scripts through `pgbench` and chose `xid8` +
//! `pg_snapshot_xmin` on the numbers. `.kb/open-questions/postgres-arm-c-structural-cost.md`
//! records what that could not answer: *"no connection pooling, no transaction
//! lifetime tied to a trait method, no cursor, no error mapping"* — four gaps
//! between "a SQL strategy passed" and "an adapter passed". This crate closes
//! them by calling the shipped `PostgresEventStore`.
//!
//! # The pairing
//!
//! **Baseline** is `PostgresEventStore::new_naive` — the same adapter with the
//! visibility predicate removed, which is unguarded `nextval()` and is what
//! Postgres does by default. **Arm C** is `PostgresEventStore::new`, the shipped
//! store. One adapter against itself with exactly the mechanism under test
//! switched off, so nothing else differs: same pool, same transaction lifetime,
//! same cursor, same error mapping, same schema.
//!
//! # The design is paired, and that is not a stylistic choice
//!
//! `experiments/position-visibility/README.md` §4 records why: the obvious
//! sequential design produced a baseline that moved **2.7× to 3.0×** between its
//! first and last measurement, because on a virtual disk an instance that has
//! been fsyncing for twenty minutes is not the instance that started. A drift of
//! 3× is larger than two of the three effects being measured.
//!
//! So the baseline is re-measured **between** arms and each ratio is taken
//! against the mean of the baselines immediately before and after, with the
//! residual drift reported in its own column. A reader can then see how much the
//! ratio is being asked to survive.
//!
//! # What is measured
//!
//! 1. **Steady-state throughput** — appends per second through `append`, at
//!    several concurrency levels, paired.
//! 2. **Staleness** — append, then poll `head()` on a fresh read until it admits
//!    the position just written. Control with nothing else running, and again
//!    with a write transaction deliberately held open on the same cluster.
//!
//! The second is the one that characterises arm C. `pg_snapshot_xmin` is the
//! oldest transaction in flight **anywhere in the cluster**, so the bound is a
//! property of the server rather than of this store's workload.

use std::sync::Arc;
use std::time::{Duration, Instant};

use happenstance_core::{
    AppendCondition, AppendError, Event, Query, QueryItem, SendEventStore, Tags,
    read_decision_model,
};
use happenstance_postgres::event_store::PostgresEventStore;
use happenstance_postgres::migration;
use happenstance_postgres::sqlx::postgres::PgPoolOptions;
use happenstance_postgres::sqlx::{Executor, PgPool};

/// Which arm a pass is measuring.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    /// Unguarded `nextval()`. The baseline, and the defect.
    Baseline,
    /// `xid8` + `pg_snapshot_xmin`. The shipped mechanism.
    ArmC,
}

impl Arm {
    fn label(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::ArmC => "arm_c",
        }
    }

    fn open(self, pool: PgPool) -> PostgresEventStore {
        match self {
            Self::Baseline => PostgresEventStore::new_naive(pool),
            Self::ArmC => PostgresEventStore::new(pool),
        }
    }
}

/// One pass's result.
///
/// Only the rate leaves this struct; the arm and the level are the caller's own
/// loop variables and carrying copies here would be two places to get them wrong.
struct Pass {
    appends: u64,
    seconds: f64,
}

impl Pass {
    fn tps(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let appends = self.appends as f64;
        appends / self.seconds
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("HS_PG_URL").map_err(|_| {
        "set HS_PG_URL to a Postgres this harness may write to, e.g. \
         postgres://postgres:postgres@localhost:5432/postgres"
    })?;
    let tag = std::env::var("HS_TAG").unwrap_or_default();
    let seconds: f64 = env_or("HS_RUN_SECS", 20.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let repeats = env_or("HS_REPEATS", 3.0) as usize;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let samples = env_or("HS_STALENESS_SAMPLES", 5.0) as usize;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let seed_rows = env_or("HS_SEED_ROWS", 10_000.0) as u64;
    let warmup = env_or("HS_WARMUP_SECS", 5.0);
    let settle = env_or("HS_SETTLE_SECS", 5.0);
    let levels: Vec<usize> = std::env::var("HS_LEVELS")
        .unwrap_or_else(|_| "1 8 32".to_owned())
        .split_whitespace()
        .filter_map(|level| level.parse().ok())
        .collect();

    println!("# position-visibility, re-measured against the built adapter");
    println!("# tag={tag} run_secs={seconds} levels={levels:?} repeats={repeats} staleness_samples={samples} seed_rows={seed_rows} warmup_secs={warmup} settle_secs={settle}");
    println!("# baseline = PostgresEventStore::new_naive (unguarded nextval)");
    println!("# arm_c    = PostgresEventStore::new      (xid8 + pg_snapshot_xmin)");
    println!();

    conditions(&url).await?;
    println!();

    throughput(&url, &levels, seconds, repeats, seed_rows, warmup, settle).await?;
    println!();

    staleness(&url, samples).await?;
    Ok(())
}

fn env_or(name: &str, fallback: f64) -> f64 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(fallback)
}

/// The settings every number below stands on.
///
/// `fsync` first, because it is the one the whole experiment rests on: under
/// `fsync=off` a commit costs nothing, and what these mechanisms charge for is
/// exactly the length of the interval a lock is held across a durable commit. A
/// pass taken with it off would show every arm as free.
async fn conditions(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new().max_connections(1).connect(url).await?;
    println!("## conditions");
    for setting in [
        "fsync",
        "synchronous_commit",
        "wal_sync_method",
        "full_page_writes",
        "max_connections",
        "server_version",
    ] {
        let value: String = sqlx::query_scalar(&format!("SHOW {setting}"))
            .fetch_one(&pool)
            .await?;
        println!("{setting} = {value}");
    }

    let fsync: String = sqlx::query_scalar("SHOW fsync").fetch_one(&pool).await?;
    assert_eq!(
        fsync, "on",
        "refusing to measure under fsync=off: it would show every arm as free, \
         because what these mechanisms charge for is the interval a lock is held \
         across a DURABLE commit"
    );
    pool.close().await;
    Ok(())
}

/// A fresh schema, migrated, with the pool pointed at it.
async fn fresh_schema(url: &str, name: &str, size: u32) -> Result<PgPool, Box<dyn std::error::Error>> {
    let admin = PgPoolOptions::new().max_connections(1).connect(url).await?;
    admin
        .execute(format!(r#"DROP SCHEMA IF EXISTS "{name}" CASCADE"#).as_str())
        .await?;
    admin
        .execute(format!(r#"CREATE SCHEMA "{name}""#).as_str())
        .await?;
    admin.close().await;

    let owned = name.to_owned();
    let pool = PgPoolOptions::new()
        .max_connections(size)
        .after_connect(move |connection, _| {
            let schema = owned.clone();
            Box::pin(async move {
                connection
                    .execute(format!(r#"SET search_path TO "{schema}""#).as_str())
                    .await?;
                Ok(())
            })
        })
        .connect(url)
        .await?;
    migration::apply(&pool).await?;
    Ok(pool)
}

/// Pre-populates the log, because a table growing from empty is not a table.
///
/// Phase 2 seeded 10,000 rows and this harness's first pass seeded none — which
/// is most of where its baseline drift came from. An empty `event` table has a
/// different plan, a different index depth and a different IO profile from one
/// with history in it, and all three move *during* a run that starts empty. The
/// pass then measures the table filling up rather than the mechanism.
///
/// Seeded by raw insert rather than through `append`, so the seeding cost is not
/// attributed to either arm and both start from an identical table.
async fn seed(pool: &PgPool, rows: u64) -> Result<(), Box<dyn std::error::Error>> {
    if rows == 0 {
        return Ok(());
    }
    sqlx::query(
        "INSERT INTO event \
         (position, event_type, data, metadata, tags, origin_store, origin_position, recorded_at) \
         SELECT nextval('event_position_seq'), 'Seeded', '\\x00'::bytea, NULL, \
                ARRAY['course:c' || (g % 1000)::text]::text[], \
                uuid_send(gen_random_uuid()), nextval('event_position_seq'), 0 \
         FROM generate_series(1, $1) AS g",
    )
    .bind(i64::try_from(rows).unwrap_or(i64::MAX))
    .execute(pool)
    .await?;
    sqlx::query("ANALYZE event").execute(pool).await?;
    Ok(())
}

/// Runs the DCB command loop for `seconds` from `clients` concurrent writers.
///
/// One "operation" is probe-then-append: read the consistency boundary, then
/// conditionally append under it. That is phase 2's workload
/// (`bench/append.sql`) and it is the shape that makes the ratio mean something —
/// the read is where the visibility predicate lives, and the predicate is the
/// only difference between the arms.
#[allow(clippy::too_many_arguments)]
async fn one_pass(
    url: &str,
    arm: Arm,
    clients: usize,
    seconds: f64,
    ordinal: usize,
    seed_rows: u64,
    warmup: f64,
    settle: f64,
) -> Result<Pass, Box<dyn std::error::Error>> {
    let schema = format!("pv_{}_{ordinal}", arm.label());
    let size = u32::try_from(clients).unwrap_or(u32::MAX) + 2;
    let pool = fresh_schema(url, &schema, size).await?;
    let store = Arc::new(arm.open(pool.clone()));

    seed(&pool, seed_rows).await?;

    // Warm up before the clock starts: the pool is lazy, the first statement on
    // each connection pays parse and plan, and the first commits on a fresh
    // schema pay page allocation. Phase 2 discards five seconds for the same
    // reason.
    if warmup > 0.0 {
        let warm_deadline = Instant::now() + Duration::from_secs_f64(warmup);
        let warm_store = Arc::clone(&store);
        while Instant::now() < warm_deadline {
            let batch = [Event::new("Warmup", &b"x"[..])?];
            let _ = SendEventStore::append(warm_store.as_ref(), &batch, None).await;
        }
    }

    let deadline = Instant::now() + Duration::from_secs_f64(seconds);
    let started = Instant::now();

    let mut writers = Vec::with_capacity(clients);
    for client in 0..clients {
        let store = Arc::clone(&store);
        writers.push(tokio::spawn(async move {
            let mut count: u64 = 0;
            let mut round: u64 = 0;
            while Instant::now() < deadline {
                round += 1;

                // The tag space is 1,000 wide and spread across clients, exactly
                // as phase 2's `bench/append.sql` does with
                // `random(1, 1000)`. Wide enough that two writers rarely share a
                // boundary, so this measures the mechanism rather than the
                // SERIALIZABLE conflict rate a narrow space would manufacture.
                #[allow(clippy::cast_possible_truncation)]
                let slot = (client as u64 * 7919 + round) % 1000;
                let Ok(tags) = Tags::from_pairs([("course", format!("c{slot}").as_str())]) else {
                    continue;
                };
                let Ok(item) = QueryItem::tagged(tags.clone()) else {
                    continue;
                };
                let query = Query::from_item(item);

                // PROBE, then append under it. This is the shape of a DCB
                // command and it is the whole reason the ratio means anything:
                // `read_decision_model` is a **read**, so it carries the
                // visibility predicate, and the predicate is the only thing that
                // differs between the two arms.
                //
                // An append-only workload measures NOTHING here, because
                // `append` is byte-identical across the arms — the `xact_id`
                // column has a DEFAULT and the condition deliberately does not
                // carry the frontier. A first pass made exactly that mistake and
                // reported arm C as 1.69x FASTER than baseline at one client,
                // which is the impossible number that exposed it.
                let Ok((_, after)) = read_decision_model(store.as_ref(), &query).await else {
                    continue;
                };

                let Ok(event) = Event::new("CourseCapacityChanged", &b"xxxxxxxx"[..]) else {
                    continue;
                };
                let batch = [event.with_tags(tags)];
                let condition = AppendCondition::new(query).after_opt(after);

                // A lost race is a legitimate outcome of a DCB command, not an
                // error: it is counted as work done, because the probe and the
                // transaction both happened. Counting only winners would make a
                // contended arm look slow for succeeding less often.
                match SendEventStore::append(store.as_ref(), &batch, Some(&condition)).await {
                    Ok(_) | Err(AppendError::ConditionViolated(_)) => count += 1,
                    Err(_) => {}
                }
            }
            count
        }));
    }

    let mut appends = 0;
    for writer in writers {
        appends += writer.await?;
    }
    let elapsed = started.elapsed().as_secs_f64();
    pool.close().await;

    // Drop this pass's schema before the next one starts. Without it every pass
    // leaves 10,000 seeded rows and a GIN index behind under its own ordinal
    // name, so by the ninth pass the database is carrying nine of them — and the
    // baselines decline monotonically through the run for a reason that has
    // nothing to do with either arm. That was the dominant drift term, and it
    // was mine rather than the disk's.
    let admin = PgPoolOptions::new().max_connections(1).connect(url).await?;
    admin
        .execute(format!(r#"DROP SCHEMA IF EXISTS "{schema}" CASCADE"#).as_str())
        .await?;
    admin.close().await;

    // Let the server settle before the next pass is timed. Dropping a seeded
    // schema leaves autovacuum and a checkpoint behind, and on a virtual disk
    // that work lands in whatever is timed next — which showed up as a 1.7x
    // spread between two baselines inside ONE triple, larger than any effect
    // being measured.
    if settle > 0.0 {
        tokio::time::sleep(Duration::from_secs_f64(settle)).await;
    }

    Ok(Pass {
        appends,
        seconds: elapsed,
    })
}

/// The paired throughput measurement, repeated.
///
/// At each level: baseline, arm C, baseline. The arm's ratio is taken against the
/// **mean of its two bracketing baselines**, and the drift between them is
/// reported so a reader can see how much the ratio is being asked to survive.
///
/// # Why it repeats
///
/// One triple cannot resolve this effect. The first pass taken against the built
/// adapter put arm C at 0.939–1.027 of baseline while the bracketing drift ran
/// 1.04–1.12 — so at 8 clients the drift was twice the effect, and at 64 the sign
/// flipped. A single number from that would be a decimal point standing where an
/// error bar belongs, which is what §4 of the parent README says the sequential
/// design was thrown out for.
///
/// So each level's triple runs `HS_REPEATS` times and every ratio is printed. The
/// record quotes the range, not a mean of things that disagree.
#[allow(clippy::too_many_arguments)]
async fn throughput(
    url: &str,
    levels: &[usize],
    seconds: f64,
    repeats: usize,
    seed_rows: u64,
    warmup: f64,
    settle: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("## throughput, paired, {repeats} repetition(s) per level");
    println!("clients,repeat,arm,tps,tps_ratio,bracketing_baseline_tps,baseline_drift");

    let mut ordinal = 0;
    for &clients in levels {
        let mut ratios: Vec<f64> = Vec::with_capacity(repeats);
        for repeat in 1..=repeats {
            ordinal += 1;
            let before = one_pass(url, Arm::Baseline, clients, seconds, ordinal, seed_rows, warmup, settle).await?;
            ordinal += 1;
            let arm = one_pass(url, Arm::ArmC, clients, seconds, ordinal, seed_rows, warmup, settle).await?;
            ordinal += 1;
            let after = one_pass(url, Arm::Baseline, clients, seconds, ordinal, seed_rows, warmup, settle).await?;

            let bracketing = (before.tps() + after.tps()) / 2.0;
            let drift = if before.tps() < after.tps() {
                after.tps() / before.tps()
            } else {
                before.tps() / after.tps()
            };
            let ratio = arm.tps() / bracketing;
            ratios.push(ratio);

            println!(
                "{clients},{repeat},baseline,{:.1},1.000,{bracketing:.1},{drift:.3}",
                before.tps()
            );
            println!(
                "{clients},{repeat},arm_c,{:.1},{ratio:.3},{bracketing:.1},{drift:.3}",
                arm.tps()
            );
            println!(
                "{clients},{repeat},baseline,{:.1},1.000,{bracketing:.1},{drift:.3}",
                after.tps()
            );
        }
        ratios.sort_by(f64::total_cmp);
        println!(
            "# level {clients}: arm_c ratio min {:.3} median {:.3} max {:.3}",
            ratios[0],
            ratios[ratios.len() / 2],
            ratios[ratios.len() - 1]
        );
    }
    Ok(())
}

/// How long after `append` returns before `head()` admits the position.
///
/// This is the number that characterises arm C, and it is reported twice: once
/// with nothing else running, and once with a write transaction deliberately held
/// open on the same cluster. The second is the point — `pg_snapshot_xmin` is the
/// oldest transaction in flight **anywhere on the server**, so an unrelated
/// migration, batch job or idle-in-transaction connection is enough.
async fn staleness(url: &str, samples: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("## staleness (append -> head admits it), milliseconds, {samples} sample(s)");
    println!("scenario,arm,min_ms,median_ms,max_ms");

    for arm in [Arm::Baseline, Arm::ArmC] {
        // Sampled rather than taken once. The first pass put the *baseline's*
        // control at 6.132 ms and its held figure at 1.711 ms — two numbers that
        // should be identical, three and a half times apart. A single sample
        // cannot tell a millisecond from a scheduling hiccup, and the unloaded
        // figure is the one a caller is told to plan for.
        let mut control = Vec::with_capacity(samples);
        let mut held = Vec::with_capacity(samples);
        for _ in 0..samples {
            control.push(staleness_once(url, arm, None).await?);
        }
        for _ in 0..samples {
            held.push(staleness_once(url, arm, Some(Duration::from_secs(5))).await?);
        }
        control.sort_by(f64::total_cmp);
        held.sort_by(f64::total_cmp);

        println!(
            "control,{},{:.3},{:.3},{:.3}",
            arm.label(),
            control[0],
            control[control.len() / 2],
            control[control.len() - 1]
        );
        println!(
            "held_5s,{},{:.3},{:.3},{:.3}",
            arm.label(),
            held[0],
            held[held.len() / 2],
            held[held.len() - 1]
        );
    }
    Ok(())
}

/// One staleness sample, optionally behind a held write transaction.
async fn staleness_once(
    url: &str,
    arm: Arm,
    hold: Option<Duration>,
) -> Result<f64, Box<dyn std::error::Error>> {
    let schema = format!("pv_stale_{}", arm.label());
    let pool = fresh_schema(url, &schema, 8).await?;
    seed(&pool, 10_000).await?;
    let store = arm.open(pool.clone());

    // The holder writes to a table of its own and keeps its transaction open.
    // It touches nothing this store reads, which is the finding: the bound is a
    // property of the cluster, not of the event log.
    let holder = if let Some(duration) = hold {
        let holder_pool = PgPoolOptions::new().max_connections(1).connect(url).await?;
        Some(tokio::spawn(async move {
            let mut transaction = holder_pool.begin().await.expect("holder should begin");
            sqlx::query("SELECT pg_current_xact_id()")
                .execute(&mut *transaction)
                .await
                .expect("holder should take an xid");
            tokio::time::sleep(duration).await;
            let _ = transaction.rollback().await;
            holder_pool.close().await;
        }))
    } else {
        None
    };

    // Let the holder establish itself before the sample starts.
    if hold.is_some() {
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    let batch = [Event::new("Measured", &b"x"[..])?];
    let written = SendEventStore::append(&store, &batch, None).await?;
    let started = Instant::now();

    // Poll until the frontier admits it. Bounded so a mechanism that never
    // admits reports rather than hangs.
    let limit = Duration::from_secs(30);
    loop {
        let head = SendEventStore::head(&store).await?;
        if head.is_some_and(|position| position >= written) {
            break;
        }
        if started.elapsed() > limit {
            break;
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    let waited = started.elapsed().as_secs_f64() * 1000.0;

    if let Some(holder) = holder {
        let _ = holder.await;
    }
    pool.close().await;
    Ok(waited)
}
