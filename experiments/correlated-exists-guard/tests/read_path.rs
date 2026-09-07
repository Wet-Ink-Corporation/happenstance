//! The gap that could halve this crate's finding: **the read path enumerates.**
//!
//! # Why this file exists
//!
//! Every other table here measures the *guard*, which the adapter wraps in
//! `SELECT max(position) FROM (…)` (`event_store.rs:658-672`). `max()` is an
//! aggregate over a column the seed arm is already ordered by, so SQLite may
//! walk the seed descending and stop at the first row whose `EXISTS` holds.
//!
//! [`results/unselective-pair.md`](../results/unselective-pair.md) found
//! evidence that it does: at 500,000 events a **5,155-row** seed and a
//! **500,000-row** seed cost the same 130 µs and 128 µs. The seed grew 97x and
//! the cost did not move, which is not what one seek per seed row looks like.
//!
//! If that reading is right, every `chain-exists` figure in this crate may be a
//! figure about an early exit rather than about the join — and
//! `query_sql::chunks` serves **both** callers (`query_sql.rs:1-11`). `read`
//! enumerates: no `max()`, and a `LIMIT` that stops after 512 *output* rows
//! rather than at the first row satisfying a predicate. Whether the rewrite pays
//! there is the question, and no other file in this crate can answer it.
//!
//! # The four cells that discriminate
//!
//! Two corpora crossed with two page positions, on both shapes.
//!
//! * **selective** (`row:r7` ∩ `shard:cold`, 5,155 of 500,000) — the shape a
//!   decision-model read has, and the one `commit` performs before every write.
//! * **unselective** (`all:yes` ∩ `shard:cold`, all 500,000) — the shape a
//!   projection runner catching up over a broad query has, and the one where
//!   `chain-exists` must do the most work.
//! * **first page** — `resume_from = 1`, the start of a replay.
//! * **mid-replay page** — `resume_from` at the half-way position, which is the
//!   page finding I-3 measured at 49.8 ms through the real adapter.
//!
//! The prediction, stated before the run: on the selective corpus the rewrite
//! should still win, because the matched set is small either way. On the
//! **unselective** corpus it could lose — one scan plus 500,000 probes may beat
//! 500,000 correlated seeks, and there is no `max()` to stop either of them
//! early.
//!
//! # Two column sets, because the read path pays for two things
//!
//! `position` alone isolates *matching*; [`PAGE_COLUMNS`] is what `fetch_page`
//! actually selects and adds row materialisation. Materialisation is identical
//! across shapes, so including it can only dilute a ratio — reporting both is
//! what says by how much.
//!
//! Run it with `cargo test --release --test read_path -- --nocapture`.

mod support;

use correlated_exists_guard::Shape;
use correlated_exists_guard::chain::{
    PAGE_COLUMNS, Selectivity, Wrapper, arms_sql, correlated_arms_sql, page_sql_with,
};
use correlated_exists_guard::seed::Corpus;
use happenstance_sqlite::connection::ConnectionSettings;
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::fixtures::query_of;
use rusqlite::Connection;
use rusqlite::types::Value;

/// Events in each store.
const SIZE: u64 = 500_000;

/// `PAGE_SIZE` from `crates/happenstance-sqlite/src/event_store.rs:141`,
/// transcribed. The read path's budget is `min(remaining, PAGE_SIZE)`, and a
/// full page is what every hop but the last asks for.
const PAGE_SIZE: i64 = 512;

/// Rounds per cell, interleaved.
///
/// Ten rather than the guard tables' fifteen: the shipped chain on the
/// unselective corpus is the slowest statement in this crate, and eight cells of
/// it at fifteen rounds would take longer than the rest of `run.sh` put
/// together.
const ROUNDS: usize = 10;

/// The two shapes this file compares. The bounded variants are guard-only —
/// there is no `AppendCondition` boundary on a read.
const SHAPES: [Shape; 2] = [Shape::Chain, Shape::ChainExists];

fn median(samples: &mut [u128]) -> u128 {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

/// Runs one page statement and returns how long it took and how many rows came
/// back.
///
/// No transaction: a read on this adapter does not take the write lock, which is
/// the whole reason `read` hops to `spawn_blocking` while `append` does not.
/// Wrapping it in `BEGIN IMMEDIATE` would price a lock this path never acquires.
fn timed_page(connection: &Connection, sql: &str, params: &[Value]) -> (u128, usize) {
    let started = std::time::Instant::now();
    let mut statement = connection.prepare(sql).expect("the statement must prepare");
    let mut rows = statement
        .query(rusqlite::params_from_iter(params.iter()))
        .expect("the page must run");
    let mut seen = 0usize;
    while let Some(row) = rows.next().expect("a page row") {
        // Every column is read, so a shape cannot look fast by leaving the
        // payload on disk.
        let _ = std::hint::black_box(row.get::<_, i64>(0).expect("position"));
        seen += 1;
    }
    (started.elapsed().as_micros(), seen)
}

/// `EXPLAIN QUERY PLAN` for one page statement, one line per plan row.
///
/// The mechanism, and the reason this file has one at all: the read path wraps
/// the matched set in a **second** `position IN (…)` (`event_store.rs:1323`),
/// and that wrapper is uncorrelated whatever the inner chain does. Whether it
/// materialises is what decides how much of the guard path's win can transfer,
/// and only the plan says.
fn explain(connection: &Connection, sql: &str, params: &[Value]) -> Vec<String> {
    let mut statement = connection
        .prepare(&format!("EXPLAIN QUERY PLAN {sql}"))
        .expect("the plan must prepare");
    statement
        .query_map(rusqlite::params_from_iter(params.iter()), |row| {
            let id: i64 = row.get(0)?;
            let parent: i64 = row.get(1)?;
            let detail: String = row.get(3)?;
            Ok(format!("id={id} parent={parent} {detail}"))
        })
        .expect("the plan must run")
        .map(|row| row.expect("a plan row"))
        .collect()
}

/// Seeds one store to [`SIZE`] in `corpus` and returns a connection onto it.
fn seeded(corpus: Corpus, name: &str) -> (std::path::PathBuf, Connection) {
    let path = tempdb(name);
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
        correlated_exists_guard::seed::seed_corpus(&mut connection, store_id, SIZE, corpus)
            .expect("seeding must succeed");
    }
    let connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    (path, connection)
}

#[test]
fn the_read_path_enumerates() {
    for (corpus_name, corpus) in [
        ("selective", Corpus::SelectiveAndUnselective),
        ("unselective", Corpus::BothUnselective),
    ] {
        let (path, connection) = seeded(corpus, corpus_name);
        let settings = ConnectionSettings::read_back(&connection).expect("pragmas must read back");
        let conditions = format!(
            "journal_mode={} synchronous={} busy_timeout_ms={} sqlite={}",
            settings.journal_mode(),
            settings.synchronous(),
            settings.busy_timeout_ms(),
            rusqlite::version(),
        );

        let [(ka, va), (kb, vb)] = corpus.guard_tags();
        let query = query_of(
            &[correlated_exists_guard::seed::SEED_TYPE],
            &[(ka, va), (kb, vb)],
        );
        let selectivity =
            Selectivity::read_for(&connection, &query).expect("the lookup must succeed");
        let items = query.items().expect("the query has items");

        let matched: i64 = connection
            .query_row(
                "SELECT count(*) FROM event_tag a JOIN event_tag b USING (position) \
                 WHERE a.tag = ?1 AND b.tag = ?2",
                rusqlite::params![format!("{ka}:{va}"), format!("{kb}:{vb}")],
                |row| row.get(0),
            )
            .expect("the matched count must read");
        println!("MATCHED\tcorpus={corpus_name}\tsize={SIZE}\tevents={matched}");

        for (page_name, resume_from) in [
            ("first-page", 1_i64),
            (
                "mid-replay-page",
                i64::try_from(SIZE / 2).unwrap_or(i64::MAX),
            ),
        ] {
            for (columns_name, columns) in
                [("position-only", "position"), ("full-row", PAGE_COLUMNS)]
            {
                // Every (inner shape, outer wrapper) pair. The inner shape is
                // this crate's own fix; the outer wrapper is I-3's, and
                // `results/read-path.md` showed the second is what caps the
                // first. `wrapper-exists` reads the whole matched set through a
                // *correlated* body, so it takes a different builder — see
                // `correlated_arms_sql`.
                let mut built: Vec<(String, String, Vec<Value>)> = Vec::new();
                for wrapper in Wrapper::ALL {
                    for shape in SHAPES {
                        // The correlated wrapper subsumes the inner shape: its
                        // body is built from the item directly, so pairing it
                        // with both inner shapes would time one statement twice
                        // under two names.
                        if wrapper.needs_correlated_body() && shape != Shape::Chain {
                            continue;
                        }
                        let mut params = Vec::new();
                        let arms = if wrapper.needs_correlated_body() {
                            correlated_arms_sql(items, &selectivity, &mut params)
                        } else {
                            // The boundary argument is unused on a read —
                            // `arms_sql` consults it only for the bounded guard
                            // shapes, and neither shape here is one.
                            arms_sql(shape, items, &selectivity, 0, &mut params)
                        };
                        let sql = page_sql_with(wrapper, &arms, columns);
                        params.push(Value::Integer(resume_from));
                        params.push(Value::Integer(i64::try_from(SIZE).unwrap_or(i64::MAX)));
                        params.push(Value::Integer(PAGE_SIZE));
                        let name = if wrapper.needs_correlated_body() {
                            wrapper.name().to_owned()
                        } else {
                            format!("{}/{}", wrapper.name(), shape.name())
                        };
                        built.push((name, sql, params));
                    }
                }

                // The plan, once per cell, before any clock starts.
                if columns_name == "position-only" {
                    for (name, sql, params) in &built {
                        println!(
                            "== read plan: corpus={corpus_name} arm={name} page={page_name} =="
                        );
                        println!("   {sql}");
                        for line in explain(&connection, sql, params) {
                            println!("   {line}");
                        }
                    }
                }

                let mut samples: Vec<Vec<u128>> = vec![Vec::new(); built.len()];
                let mut returned: Vec<Option<usize>> = vec![None; built.len()];

                // Round-robin, rotating the starting shape, so neither inherits
                // the other's page-cache churn every round.
                for round in 0..ROUNDS {
                    for offset in 0..built.len() {
                        let index = (round + offset) % built.len();
                        let (_, sql, params) = &built[index];
                        let (us, rows) = timed_page(&connection, sql, params);
                        samples[index].push(us);
                        returned[index] = Some(rows);
                    }
                }

                // The correctness control: both spellings of one conjunction
                // must return the same page. A shape that returned fewer rows
                // would be faster for the wrong reason, and on the read path
                // that is a *silently truncated replay* rather than a wrong
                // verdict.
                let first = returned[0];
                for (index, rows) in returned.iter().enumerate() {
                    assert_eq!(
                        *rows, first,
                        "{corpus_name}/{page_name}/{columns_name}: arm `{}` returned a \
                         different page from `{}` — an arm that returns fewer rows is a \
                         truncated replay, not a faster one, and an arm that returns more \
                         has lost its correlation",
                        built[index].0, built[0].0,
                    );
                }

                for (index, (name, _, _)) in built.iter().enumerate() {
                    println!(
                        "READ\tcorpus={corpus_name}\tarm={name}\tpage={page_name}\t\
                         columns={columns_name}\tsize={SIZE}\trows={}\tus_median={}\t\
                         rounds={ROUNDS}\t{conditions}",
                        first.unwrap_or(0),
                        median(&mut samples[index]),
                    );
                }
            }
        }

        drop(connection);
        remove(&path);
    }
}

fn tempdb(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "happenstance-exists-read-{}-{name}.sqlite3",
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
