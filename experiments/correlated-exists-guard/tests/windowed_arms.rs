//! Does pushing the read's window into each arm dissolve the crossover?
//!
//! # The question this asks that `read_path.rs` does not
//!
//! [`results/read-path.md`](../results/read-path.md) priced three candidates for
//! I-3's outer wrapper and found a **crossover**: the `EXISTS` wrapper is
//! 1,089x better on a query matching every event and 2.3x worse on one matching
//! 1 in 97. Every one of those three argues about *which side of the join to
//! drive from*, and each is best at one end of the selectivity axis, so
//! choosing between them means the adapter changing its query plan on data it
//! samples — a decision, not an optimisation.
//!
//! None of them asks a different question: **why does the read's window stop at
//! the subquery boundary?** `fetch_page` applies `position >= ?`,
//! `position <= ?`, the direction and the page budget *outside* the membership
//! test, where nothing can push them into the arm that built it. The adapter
//! builds that arm. It could put them in.
//!
//! If that works the matched set is at most `budget x arms` **whatever the
//! corpus**, so the `IN` wrapper stops being bad on a broad query and the
//! crossover does not arise — which is a better outcome than navigating it,
//! because it needs no threshold, no sampling, and no plan that flips on data.
//!
//! # Soundness, which is prior to speed
//!
//! The claim is that bounding each arm by the page budget cannot lose a row the
//! page needed: an event in the merged top *b* is in some arm, and its rank
//! within that arm is no worse than its rank in the union. `fetch_page` already
//! relies on exactly this one level up, to bound each **chunk** and merge.
//!
//! It is asserted rather than argued here. **The control is the returned page**
//! — every column of every row, in order — compared across all three shapes
//! every round before any time is recorded. A shape that keeps the wrong
//! `budget` positions (an arm ordered `ASC` under a page ordered `DESC`, say)
//! returns a page that is *short*, not one that looks wrong, so counts would not
//! catch it.
//!
//! # Why backwards is measured and not assumed
//!
//! The window's direction decides *which* `budget` positions an arm keeps, so
//! backwards is the case where this shape is most likely to be quietly wrong.
//! It is one cell rather than a full grid: enough to fire the control, not
//! enough to claim a backwards throughput figure.
//!
//! Run it with `cargo test --release --test windowed_arms -- --nocapture`.

use correlated_exists_guard::Shape;
use correlated_exists_guard::chain::{
    PAGE_COLUMNS, Selectivity, Window, Wrapper, arms_sql, correlated_arms_sql, page_sql_directed,
    windowed_arms_sql,
};
use correlated_exists_guard::seed::Corpus;
use happenstance_sqlite::connection::ConnectionSettings;
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::fixtures::query_of;
use rusqlite::Connection;
use rusqlite::types::Value;

/// Events in each store. The same size `read_path.rs` uses, so the two tables
/// are about the same log even though they are not one run.
const SIZE: u64 = 500_000;

/// `PAGE_SIZE` from `crates/happenstance-sqlite/src/event_store.rs:141`.
const PAGE_SIZE: i64 = 512;

/// Rounds per cell, interleaved.
const ROUNDS: usize = 10;

/// One row of a returned page, every column, so the control compares what the
/// caller would actually receive rather than a position list.
type Row = (i64, String, Vec<u8>, Option<Vec<u8>>, Vec<u8>, i64);

fn median(samples: &mut [u128]) -> u128 {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

fn timed_page(connection: &Connection, sql: &str, params: &[Value]) -> (u128, Vec<Row>) {
    let started = std::time::Instant::now();
    let mut statement = connection.prepare(sql).expect("the statement must prepare");
    let mut rows = statement
        .query(rusqlite::params_from_iter(params.iter()))
        .expect("the page must run");
    let mut page = Vec::with_capacity(512);
    while let Some(row) = rows.next().expect("a page row") {
        page.push((
            row.get(0).expect("position"),
            row.get(1).expect("event_type"),
            row.get(2).expect("data"),
            row.get(3).expect("metadata"),
            row.get(4).expect("tags"),
            row.get(6).expect("origin_position"),
        ));
    }
    (started.elapsed().as_micros(), page)
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

/// The three page shapes, built for one window.
///
/// Returned as `(name, sql, params)` with the parameters already in the order
/// each statement's `?`s appear — the matched set's, then `resume_from`,
/// `ceiling`, `budget`.
fn shapes(
    items: &[happenstance_core::QueryItem],
    selectivity: &Selectivity,
    window: Window,
    ceiling: i64,
) -> Vec<(&'static str, String, Vec<Value>)> {
    let tail = [
        Value::Integer(window.lo),
        Value::Integer(ceiling),
        Value::Integer(window.budget),
    ];

    let mut out = Vec::new();

    // What ships today: the correlated chain under the `IN` wrapper.
    let mut params = Vec::new();
    let matched = arms_sql(Shape::ChainExists, items, selectivity, 0, &mut params);
    params.extend_from_slice(&tail);
    out.push((
        "in-exists (ships)",
        page_sql_directed(
            Wrapper::InSubquery,
            &matched,
            PAGE_COLUMNS,
            window.backwards,
        ),
        params,
    ));

    // `read-path.md`'s best broad candidate, and its worst selective one.
    let mut params = Vec::new();
    let matched = correlated_arms_sql(items, selectivity, &mut params);
    params.extend_from_slice(&tail);
    out.push((
        "wrapper-exists",
        page_sql_directed(
            Wrapper::CorrelatedExists,
            &matched,
            PAGE_COLUMNS,
            window.backwards,
        ),
        params,
    ));

    // The proposal: the same `IN` wrapper, with the window inside each arm.
    let mut params = Vec::new();
    let matched = windowed_arms_sql(items, selectivity, window, &mut params);
    params.extend_from_slice(&tail);
    out.push((
        "windowed-in-exists",
        page_sql_directed(
            Wrapper::InSubquery,
            &matched,
            PAGE_COLUMNS,
            window.backwards,
        ),
        params,
    ));

    out
}

#[test]
fn the_window_belongs_inside_the_arm() {
    for (corpus_name, corpus) in [
        ("selective", Corpus::SelectiveAndUnselective),
        ("unselective", Corpus::BothUnselective),
    ] {
        let (path, connection) = seeded(corpus, corpus_name);
        let settings = ConnectionSettings::read_back(&connection).expect("pragmas must read back");
        println!(
            "CONDITIONS\tcorpus={corpus_name}\tjournal_mode={}\tsynchronous={}\t\
             busy_timeout_ms={}\tsqlite={}\tsize={SIZE}",
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

        let ceiling = i64::try_from(SIZE).unwrap_or(i64::MAX);
        let half = i64::try_from(SIZE / 2).unwrap_or(i64::MAX);
        let late = i64::try_from(SIZE - SIZE / 10).unwrap_or(i64::MAX);

        for (cell, window) in [
            (
                "first-page",
                Window {
                    lo: 1,
                    hi: ceiling,
                    budget: PAGE_SIZE,
                    backwards: false,
                },
            ),
            (
                "mid-replay-page",
                Window {
                    lo: half,
                    hi: ceiling,
                    budget: PAGE_SIZE,
                    backwards: false,
                },
            ),
            (
                "late-replay-page",
                Window {
                    lo: late,
                    hi: ceiling,
                    budget: PAGE_SIZE,
                    backwards: false,
                },
            ),
            // Backwards: the direction that decides *which* budget positions an
            // arm keeps, and therefore the one where a windowed arm is most
            // likely to be quietly short.
            (
                "backwards-from-head",
                Window {
                    lo: 1,
                    hi: ceiling,
                    budget: PAGE_SIZE,
                    backwards: true,
                },
            ),
        ] {
            let built = shapes(items, &selectivity, window, ceiling);

            for (name, sql, params) in &built {
                for line in explain(&connection, sql, params) {
                    println!("PLAN\tcorpus={corpus_name}\tcell={cell}\tshape={name}\t{line}");
                }
            }

            let mut samples: Vec<Vec<u128>> = vec![Vec::new(); built.len()];
            let mut control: Option<Vec<Row>> = None;

            for round in 0..ROUNDS {
                // Rotate which shape leads, so no median is about going second
                // on a warm page cache.
                for offset in 0..built.len() {
                    let index = (round + offset) % built.len();
                    let (micros, page) = timed_page(&connection, &built[index].1, &built[index].2);
                    match &control {
                        None => control = Some(page),
                        Some(expected) => assert_eq!(
                            &page, expected,
                            "shape {} returned a different page — the window is not \
                             sound where it was put",
                            built[index].0
                        ),
                    }
                    samples[index].push(micros);
                }
            }

            let rows = control.as_ref().map_or(0, Vec::len);
            for (index, (name, _, _)) in built.iter().enumerate() {
                println!(
                    "PAGE\tcorpus={corpus_name}\tcell={cell}\tshape={name}\trows={rows}\t\
                     us_median={}",
                    median(&mut samples[index]),
                );
            }
        }

        drop(connection);
        remove(&path);
    }
}

fn tempdb(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "happenstance-exists-window-{}-{name}.sqlite3",
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
