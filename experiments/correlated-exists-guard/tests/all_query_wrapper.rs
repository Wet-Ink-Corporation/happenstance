//! What omitting the wrapper for `Query::all` is worth, on the real statement.
//!
//! # Why this file exists
//!
//! [`results/read-path.md`](../results/read-path.md) §4 proposes three parts,
//! and only the first needs no cardinality estimate: when the chunk matches
//! **every** event, `fetch_page`'s `WHERE position IN (<matched>)` is a
//! tautology over the whole table. `chunks` short-circuits `Query::all` to
//! `SELECT position FROM event`, so the statement that ships asks SQLite to
//! materialise every position in the log in order to decide that every row
//! qualifies.
//!
//! Finding I-3 priced that at **846x** against a different store and a different
//! page, and this crate does not get to inherit someone else's ratio for a
//! change it makes. This file measures the pair directly, at the size and the
//! page budget the adapter actually uses.
//!
//! # The two statements
//!
//! Both are transcribed from `event_store.rs`'s `fetch_page`, with the
//! `resume_from`, ceiling, ordering and `LIMIT` clauses it appends. They differ
//! in one term:
//!
//! * `wrapper-in` — `WHERE position IN (SELECT position FROM event)`, what
//!   shipped until this crate's findings were adopted;
//! * `wrapper-omitted` — `WHERE 1`, a constant the planner folds away. The
//!   spelling keeps the clause list the caller builds unchanged, and its dual is
//!   `arms_sql`'s `WHERE 0` for the arm that can match nothing.
//!
//! **The control is the returned page**, asserted every round before either time
//! is recorded: same row count, same first and last position. A wrapper removed
//! from a statement that then returns a different page is not a faster statement,
//! it is a wrong one — and a tautology is exactly the kind of clause that is
//! only *almost* always removable.
//!
//! Run it with `cargo test --release --test all_query_wrapper -- --nocapture`.

use correlated_exists_guard::chain::PAGE_COLUMNS;
use correlated_exists_guard::seed::Corpus;
use happenstance_sqlite::connection::ConnectionSettings;
use happenstance_sqlite::event_store::SqliteEventStore;
use rusqlite::Connection;
use rusqlite::types::Value;

/// Events in the store. The same size the rest of this crate's read tables use.
const SIZE: u64 = 500_000;

/// `PAGE_SIZE` from `crates/happenstance-sqlite/src/event_store.rs:141`.
const PAGE_SIZE: i64 = 512;

/// Rounds per cell, interleaved.
const ROUNDS: usize = 10;

fn median(samples: &mut [u128]) -> u128 {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

/// One page of a `Query::all` read, with the membership term as the caller
/// spells it.
///
/// Everything after that term is `fetch_page`'s, in its order: `resume_from`,
/// the ceiling that discharges ES-12, the direction, and the page budget.
fn page_sql(membership: &str) -> String {
    format!(
        "SELECT {PAGE_COLUMNS} FROM event WHERE {membership} \
         AND position >= ? AND position <= ? ORDER BY position ASC LIMIT ?"
    )
}

/// Runs one page and returns the elapsed micros and the page it returned.
fn timed_page(connection: &Connection, sql: &str, params: &[Value]) -> (u128, Vec<i64>) {
    let started = std::time::Instant::now();
    let mut statement = connection.prepare(sql).expect("the statement must prepare");
    let mut rows = statement
        .query(rusqlite::params_from_iter(params.iter()))
        .expect("the page must run");
    let mut positions = Vec::with_capacity(512);
    while let Some(row) = rows.next().expect("a page row") {
        // Every column is read, so a shape cannot look fast by leaving the
        // payload on disk.
        let _ = std::hint::black_box(row.get::<_, Vec<u8>>(2).expect("the data column"));
        positions.push(row.get::<_, i64>(0).expect("position"));
    }
    (started.elapsed().as_micros(), positions)
}

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

#[test]
fn the_tautological_wrapper_is_what_it_costs() {
    let path = tempdb("all-query-wrapper");
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
            Corpus::SelectiveAndUnselective,
        )
        .expect("seeding must succeed");
    }
    let connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");

    let settings = ConnectionSettings::read_back(&connection).expect("pragmas must read back");
    println!(
        "CONDITIONS\tjournal_mode={}\tsynchronous={}\tbusy_timeout_ms={}\tsqlite={}\tsize={SIZE}",
        settings.journal_mode(),
        settings.synchronous(),
        settings.busy_timeout_ms(),
        rusqlite::version(),
    );

    let ceiling = i64::try_from(SIZE).unwrap_or(i64::MAX);
    let shapes = [
        ("wrapper-in", page_sql("position IN (SELECT position FROM event)")),
        ("wrapper-omitted", page_sql("1")),
    ];

    // Three depths, not two. The prediction before the run was that the
    // wrapper's cost is a materialisation and therefore flat in `resume_from`;
    // the first run falsified that, so the axis it *does* move on is measured
    // rather than asserted.
    for (page_name, resume_from) in [
        ("first-page", 1_i64),
        (
            "mid-replay-page",
            i64::try_from(SIZE / 2).unwrap_or(i64::MAX),
        ),
        (
            "late-replay-page",
            i64::try_from(SIZE - SIZE / 10).unwrap_or(i64::MAX),
        ),
    ] {
        let params = vec![
            Value::Integer(resume_from),
            Value::Integer(ceiling),
            Value::Integer(PAGE_SIZE),
        ];

        for (name, sql) in &shapes {
            for line in explain(&connection, sql, &params) {
                println!("PLAN\tpage={page_name}\tshape={name}\t{line}");
            }
        }

        let mut samples: Vec<Vec<u128>> = vec![Vec::new(); shapes.len()];
        let mut control: Option<Vec<i64>> = None;

        for round in 0..ROUNDS {
            // Alternate which shape goes first, so neither median is about
            // going second on a warm page cache.
            let order: Vec<usize> = if round % 2 == 0 {
                (0..shapes.len()).collect()
            } else {
                (0..shapes.len()).rev().collect()
            };
            for index in order {
                let (micros, page) = timed_page(&connection, &shapes[index].1, &params);
                match &control {
                    None => control = Some(page),
                    Some(expected) => assert_eq!(
                        &page, expected,
                        "shape {} returned a different page — the wrapper is not redundant",
                        shapes[index].0
                    ),
                }
                samples[index].push(micros);
            }
        }

        for (index, (name, _)) in shapes.iter().enumerate() {
            println!(
                "PAGE\tpage={page_name}\tshape={name}\trows={}\tus_median={}",
                control.as_ref().map_or(0, Vec::len),
                median(&mut samples[index]),
            );
        }
    }
}

fn tempdb(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "happenstance-exists-all-{}-{name}.sqlite3",
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
