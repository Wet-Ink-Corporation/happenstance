//! The case that could overturn this crate's finding: **no tag is selective**.
//!
//! # Why this is the adversarial case
//!
//! `chain-exists` wins by making the seed arm the outer loop. One index seek per
//! seed row replaces a materialisation of the chained arm — so the win is
//! proportional to how *small* the seed is. Every cell in
//! [`guard_cost`](../results/guard-cost.md) uses `row:r7`, which matches one
//! event in ninety-seven.
//!
//! Take that away and the mechanism has nothing to work with. Here both tags
//! match **every** event, so:
//!
//! * `chain-exists` does `|log|` index seeks;
//! * `chain-as-shipped` does one sequential scan to materialise, then `|log|`
//!   probes into a list.
//!
//! A sequential scan is cheaper per row than a b-tree seek. This is therefore
//! the shape where the correlated form could plausibly *lose*, and a
//! recommendation that had not looked would be a recommendation with an unlooked
//! corner.
//!
//! # Why it is a separate store rather than a fifth scenario
//!
//! Adding a third tag to the shared corpus would grow `event_tag` by 50% and
//! move every figure in `guard-cost.md` — invalidating a table for a variable
//! orthogonal to it. This test seeds its own store, and every other file's
//! corpus is byte-for-byte what the sibling experiment used.
//!
//! # The tie is the point, not a flaw
//!
//! `all:yes` and `shard:cold` have equal cardinality, so
//! `Selectivity::most_selective_first` — a stable sort by count — leaves the
//! input order deciding which is the seed. There is no ordering question to ask
//! when no tag is selective, and `tag_cardinality` has nothing to contribute.
//! That is what this corpus *means*.
//!
//! Run it with `cargo test --release --test unselective_pair -- --nocapture`.

mod support;

use correlated_exists_guard::Shape;
use correlated_exists_guard::chain::{Selectivity, arms_sql, guard_sql};
use correlated_exists_guard::seed::Corpus;
use happenstance_sqlite::connection::ConnectionSettings;
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::fixtures::{condition, condition_after, query_of};
use rusqlite::Connection;
use rusqlite::types::Value;

/// Events in the store.
///
/// 500,000 rather than 10^6: the shipped chain costs about a quarter of a second
/// per evaluation here and the adversarial arm is the one this file exists for,
/// so the size is chosen to keep the whole file inside `run.sh`'s budget while
/// still being two orders of magnitude past where the effect appears.
const SIZE: u64 = 500_000;

/// Rounds per shape, interleaved round-robin.
const ROUNDS: usize = 15;

/// The four shapes worth comparing here.
///
/// `chain-bounded-seed` is left out: `guard-cost.md` shows it indistinguishable
/// from the shipped chain in all nine cells, and a shape that has never moved is
/// a column a reader has to skip.
const SHAPES: [Shape; 4] = [
    Shape::Chain,
    Shape::ChainBoundedAllArms,
    Shape::Grouped,
    Shape::ChainExists,
];

fn median(samples: &mut [u128]) -> u128 {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

/// Runs one guard statement inside `BEGIN IMMEDIATE`, exactly as the shipped
/// `append` does, and rolls back.
///
/// Returns the **verdict** — whether the guard is violated — and not the raw
/// `max(position)`, because the two are not comparable across shapes. A shape
/// with `boundary_in_sql()` returns only positions already above the boundary,
/// so *any* row is a violation and `None` means none; a shape without it returns
/// the highest match and the comparison happens in Rust. The first version of
/// this file compared the raw positions and its own control caught it: at a
/// boundary of `SIZE`, `chain-bounded-all-arms` correctly returned `None` where
/// `chain-as-shipped` correctly returned `Some(500000)`, and both correctly mean
/// *not violated*.
fn timed_guard(
    connection: &Connection,
    shape: Shape,
    sql: &str,
    params: &[Value],
    boundary: i64,
) -> (u128, bool) {
    let started = std::time::Instant::now();
    connection
        .execute_batch("BEGIN IMMEDIATE")
        .expect("the write lock must be available");
    let highest: Option<i64> = connection
        .query_row(sql, rusqlite::params_from_iter(params.iter()), |row| {
            row.get(0)
        })
        .expect("the guard must run");
    connection.execute_batch("ROLLBACK").expect("rollback");
    let elapsed = started.elapsed().as_micros();
    let violated = if shape.boundary_in_sql() {
        highest.is_some()
    } else {
        highest.is_some_and(|value| value > boundary)
    };
    (elapsed, violated)
}

#[test]
fn neither_tag_selective() {
    let path = tempdb("unselective-pair");

    let store_id = {
        drop(SqliteEventStore::open(&path).expect("migrating must succeed"));
        let connection =
            happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
        connection
            .query_row(
                "SELECT CAST(v AS BLOB) FROM store_meta WHERE k = 'store_id'",
                [],
                |row| {
                    let raw: Vec<u8> = row.get(0)?;
                    let mut bytes = [0u8; 16];
                    bytes.copy_from_slice(&raw[..16]);
                    Ok(happenstance_core::StoreId::from_bytes(bytes))
                },
            )
            .expect("the adapter minted an incarnation")
    };
    {
        let mut connection =
            happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
        correlated_exists_guard::seed::seed_corpus(
            &mut connection,
            store_id,
            SIZE,
            Corpus::BothUnselective,
        )
        .expect("seeding must succeed");
    }

    let connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    let settings = ConnectionSettings::read_back(&connection).expect("pragmas must read back");
    let conditions = format!(
        "journal_mode={} synchronous={} busy_timeout_ms={} sqlite={}",
        settings.journal_mode(),
        settings.synchronous(),
        settings.busy_timeout_ms(),
        rusqlite::version(),
    );

    let [(ka, va), (kb, vb)] = Corpus::BothUnselective.guard_tags();
    let query = query_of(
        &[correlated_exists_guard::seed::SEED_TYPE],
        &[(ka, va), (kb, vb)],
    );
    let selectivity = Selectivity::read_for(&connection, &query).expect("the lookup must succeed");
    let items = query.items().expect("the query has items");

    // **The control that says this corpus is what it claims to be.** Both tags
    // must carry the whole log, or the arm below is measuring some other
    // selectivity contrast and the whole file is mislabelled.
    let a = selectivity.count_of("all:yes");
    let b = selectivity.count_of("shard:cold");
    let expected = i64::try_from(SIZE).unwrap_or(i64::MAX);
    assert_eq!(
        (a, b),
        (Some(expected), Some(expected)),
        "both tags must match every event — that is what makes this the \
         adversarial corpus. Got all:yes={a:?} shard:cold={b:?}"
    );
    println!("CARDINALITY\tall_yes={a:?}\tshard_cold={b:?}\tsize={SIZE}");

    for (label, guard) in [
        (
            "accepted-2tag-at-head",
            condition_after(query.clone(), SIZE),
        ),
        ("rejected-2tag-unbounded", condition(query.clone())),
    ] {
        let boundary = guard
            .guards()
            .first()
            .expect("the condition carries one guard")
            .after
            .map_or(0, |after| i64::try_from(after.get()).unwrap_or(i64::MAX));

        let mut built: Vec<(Shape, String, Vec<Value>)> = Vec::new();
        for shape in SHAPES {
            let mut params = Vec::new();
            let sql = guard_sql(&arms_sql(shape, items, &selectivity, boundary, &mut params));
            built.push((shape, sql, params));
        }

        let mut samples: Vec<Vec<u128>> = vec![Vec::new(); SHAPES.len()];
        let mut verdicts: Vec<Option<bool>> = vec![None; SHAPES.len()];

        // Round-robin, rotating the starting shape each round, so no shape is
        // permanently first and none of the differences is an artefact of
        // inheriting another's page-cache churn.
        for round in 0..ROUNDS {
            for offset in 0..SHAPES.len() {
                let index = (round + offset) % SHAPES.len();
                let (shape, sql, params) = &built[index];
                let (us, violated) = timed_guard(&connection, *shape, sql, params, boundary);
                samples[index].push(us);
                verdicts[index] = Some(violated);
            }
        }

        // The local correctness control: four spellings of one conjunction must
        // reach the same **verdict**. A shape that disagreed would be fast for
        // the wrong reason.
        let first = verdicts[0];
        for (index, verdict) in verdicts.iter().enumerate() {
            assert_eq!(
                *verdict,
                first,
                "{label}: shape `{}` disagreed with `{}` — a shape that decides \
                 less is not a faster shape",
                SHAPES[index].name(),
                SHAPES[0].name()
            );
        }

        for (index, shape) in SHAPES.iter().enumerate() {
            println!(
                "UNSELECTIVE\tshape={}\tsize={SIZE}\tscenario={label}\tboundary={boundary}\t\
                 us_median={}\trounds={ROUNDS}\t{conditions}",
                shape.name(),
                median(&mut samples[index]),
            );
        }
    }

    remove(&path);
}

fn tempdb(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "happenstance-exists-guard-{}-{name}.sqlite3",
        std::process::id()
    ));
    remove(&path);
    path
}

fn remove(path: &std::path::Path) {
    for suffix in ["", "-wal", "-shm"] {
        let mut candidate = path.to_path_buf().into_os_string();
        candidate.push(suffix);
        let _ = std::fs::remove_file(std::path::PathBuf::from(candidate));
    }
}
