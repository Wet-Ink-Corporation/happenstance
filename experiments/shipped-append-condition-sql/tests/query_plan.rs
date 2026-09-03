//! What SQLite actually does with these statements, at 10^6 rows.
//!
//! Timings say *how much*; a query plan says *why*, and without one a table is a
//! set of numbers nobody can act on. Two questions are answered here, both
//! against a seeded million-row store.
//!
//! # 1. The paged read — finding I-3
//!
//! `fetch_page` (`event_store.rs:1313-1381`) prepares one statement per 512-row
//! page and every one of them carries `position IN (<arm>)`. For `Query::all`
//! the arm is the bare `SELECT position FROM event`
//! (`query_sql.rs:160-161`), so the commonest replay in the library asks SQLite,
//! 1,954 times over a million-row log, which positions of `event` are in
//! `event`. Whether that is free or quadratic is a planner question:
//! `USE TEMP B-TREE` or a second `SCAN event` in the plan is the answer, and the
//! same statement with the `position IN (…)` clause removed is the control.
//! Both are also **executed** and timed, because a plan is an intention.
//!
//! The statement is not written out here. It is the string
//! `tests/emitted_sql.rs` captured off a running adapter, re-captured at page
//! 1,000 on this store and asserted identical to page 2's — so what is explained
//! is what runs.
//!
//! # 2. The guard shapes — findings I-1 and I-2
//!
//! The same for each shape in [`Shape::ALL`], which is what turns
//! `results/guard-cost.md`'s medians into a mechanism.
//!
//! Run it with `cargo test --release --manifest-path
//! experiments/shipped-append-condition-sql/Cargo.toml --test query_plan --
//! --nocapture`.

mod support;

use std::sync::{Mutex, OnceLock};

use happenstance_core::{EventStore, Query, ReadOptions};
use happenstance_sqlite::connection::ConnectionSettings;
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::fixtures::query_of;
use rusqlite::Connection;
use rusqlite::trace::{TraceEvent, TraceEventCodes};
use rusqlite::types::Value;
use shipped_append_condition_sql::Shape;
use shipped_append_condition_sql::chain::{Selectivity, arms_sql, guard_sql};

/// Events in the store every plan below is read against.
const SIZE: u64 = 1_000_000;

/// `PAGE_SIZE` — `crates/happenstance-sqlite/src/event_store.rs:141`.
const PAGE: i64 = 512;

/// Which page the read statement is explained at.
const PAGE_NUMBER: i64 = 1_000;

fn log() -> &'static Mutex<Vec<String>> {
    static LOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    LOG.get_or_init(|| Mutex::new(Vec::new()))
}

fn record(event: TraceEvent<'_>) {
    if let TraceEvent::Stmt(_, sql) = event
        && let Ok(mut entries) = log().lock()
    {
        entries.push(sql.to_owned());
    }
}

/// `EXPLAIN QUERY PLAN` for `sql`, as the rows SQLite returns.
fn explain(connection: &Connection, sql: &str, params: &[Value]) -> Vec<String> {
    let mut statement = connection
        .prepare(&format!("EXPLAIN QUERY PLAN {sql}"))
        .expect("the statement must prepare");
    let rows = statement
        .query_map(rusqlite::params_from_iter(params.iter()), |row| {
            let id: i64 = row.get(0)?;
            let parent: i64 = row.get(1)?;
            let detail: String = row.get(3)?;
            Ok(format!("id={id} parent={parent} {detail}"))
        })
        .expect("the plan must run")
        .map(|row| row.expect("a plan row"))
        .collect();
    rows
}

/// Runs `sql` for real and returns how long it took and how many rows came back.
fn time(connection: &Connection, sql: &str, params: &[Value]) -> (u128, usize) {
    let started = std::time::Instant::now();
    let mut statement = connection.prepare(sql).expect("the statement must prepare");
    let mut rows = statement
        .query(rusqlite::params_from_iter(params.iter()))
        .expect("the query must run");
    let mut count = 0usize;
    while rows.next().expect("a row").is_some() {
        count += 1;
    }
    (started.elapsed().as_micros(), count)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_plans_behind_the_numbers() {
    let path = tempdb("query-plan");

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
        shipped_append_condition_sql::seed::seed(&mut connection, store_id, SIZE)
            .expect("seeding must succeed");
    }

    let connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    let settings = ConnectionSettings::read_back(&connection).expect("pragmas must read back");
    let check =
        shipped_append_condition_sql::seed::verify(&connection, SIZE).expect("verify must succeed");
    assert!(
        check.is_sound(SIZE),
        "the seed must be sound before anything is explained: {}",
        check.line()
    );
    println!(
        "SEED\tsize={SIZE}\t{}\tdb_bytes={}\tjournal_mode={} synchronous={} \
         busy_timeout_ms={} sqlite={}",
        check.line(),
        std::fs::metadata(&path).map(|meta| meta.len()).unwrap_or(0),
        settings.journal_mode(),
        settings.synchronous(),
        settings.busy_timeout_ms(),
        rusqlite::version(),
    );

    // ---- 1. the paged read, at page 1,000 ---------------------------------

    // Capture the statement off a real replay rather than writing it out. The
    // trace callback is installed before the store is built, so it sees every
    // page.
    let traced =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    traced.trace_v2(TraceEventCodes::SQLITE_TRACE_STMT, Some(record));
    let store = SqliteEventStore::new(traced).expect("the file is migrated");

    let all = Query::all();
    let replay = std::time::Instant::now();
    let mut drained = 0usize;
    {
        use futures_core::Stream;
        let stream = store.read(&all, ReadOptions::default());
        let mut stream = core::pin::pin!(stream);
        let mut context = core::task::Context::from_waker(core::task::Waker::noop());
        loop {
            match Stream::poll_next(stream.as_mut(), &mut context) {
                core::task::Poll::Ready(Some(item)) => {
                    item.expect("every row must decode");
                    drained += 1;
                }
                core::task::Poll::Ready(None) => break,
                core::task::Poll::Pending => tokio::task::yield_now().await,
            }
        }
    }
    let replay_ms = replay.elapsed().as_millis();
    assert_eq!(drained, SIZE as usize, "the whole log must come back");

    let pages: Vec<String> = log()
        .lock()
        .map(|entries| {
            entries
                .iter()
                .filter(|sql| sql.contains("FROM event WHERE position IN"))
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    println!(
        "REPLAY\tsize={SIZE}\tquery=all\tstatements={}\tevents={drained}\t\
         wall_ms={replay_ms}\tus_per_event={:.2}",
        pages.len(),
        replay_ms as f64 * 1000.0 / drained as f64,
    );
    assert!(
        pages.len() > PAGE_NUMBER as usize,
        "a million events at 512 a page is 1,954 statements, not {}",
        pages.len()
    );
    assert_eq!(
        pages[1],
        pages[PAGE_NUMBER as usize - 1],
        "page 2 and page 1,000 must be one string; only the bound values move"
    );

    let page_sql = pages[PAGE_NUMBER as usize - 1].clone();
    // The bound values page 1,000 carries: `resume_from` is the first position
    // of the page, the ceiling is the head sampled when the read began, and the
    // budget is `PAGE_SIZE`.
    let resume_from = (PAGE_NUMBER - 1) * PAGE + 1;
    let page_params = vec![
        Value::Integer(resume_from),
        Value::Integer(SIZE as i64),
        Value::Integer(PAGE),
    ];
    // The control: the same statement with `position IN (…)` removed. It is a
    // string edit rather than a second builder, so nothing else can differ.
    let control_sql = page_sql.replace("WHERE position IN (SELECT position FROM event) AND ", "WHERE ");
    assert_ne!(control_sql, page_sql, "the control must actually differ");

    println!();
    println!("== the statement fetch_page emits at page {PAGE_NUMBER} of a Query::all replay ==");
    println!("{page_sql}");
    println!("-- bound: resume_from={resume_from} ceiling={SIZE} limit={PAGE}");
    for row in explain(&connection, &page_sql, &page_params) {
        println!("   {row}");
    }
    let (with_cold_us, with_rows) = time(&connection, &page_sql, &page_params);
    let (with_us, _) = time(&connection, &page_sql, &page_params);
    println!("   EXECUTED first_us={with_cold_us} second_us={with_us} rows={with_rows}");

    println!();
    println!("== the same statement with `position IN (…)` removed (the control) ==");
    println!("{control_sql}");
    for row in explain(&connection, &control_sql, &page_params) {
        println!("   {row}");
    }
    let (without_cold_us, without_rows) = time(&connection, &control_sql, &page_params);
    let (without_us, _) = time(&connection, &control_sql, &page_params);
    println!("   EXECUTED first_us={without_cold_us} second_us={without_us} rows={without_rows}");
    assert_eq!(
        with_rows, without_rows,
        "the control must return the same rows, or it is a different question"
    );
    println!(
        "PAGE\tsize={SIZE}\tpage={PAGE_NUMBER}\twith_in_clause_us={with_us}\t\
         without_in_clause_us={without_us}\trows={with_rows}"
    );

    // ---- 2. the guard shapes ---------------------------------------------

    let query = query_of(&["Seeded"], &[("shard", "cold"), ("row", "r7")]);
    let selectivity = Selectivity::read_for(&connection, &query).expect("the lookup must succeed");
    println!();
    println!(
        "== tag_cardinality, which orders the chain's seed: shard:cold={:?} row:r7={:?} ==",
        selectivity.count_of("shard:cold"),
        selectivity.count_of("row:r7"),
    );
    let items = query.items().expect("the query has items");
    let boundary = (SIZE / 2) as i64;

    for shape in Shape::ALL {
        let mut params = Vec::new();
        let sql = guard_sql(&arms_sql(shape, items, &selectivity, boundary, &mut params));
        println!();
        println!("== the guard statement for shape `{}` (boundary={boundary}) ==", shape.name());
        println!("{sql}");
        for row in explain(&connection, &sql, &params) {
            println!("   {row}");
        }
        // **Twice, and both are printed.** These are single executions in a
        // fixed order, immediately after a full replay has evicted the page
        // cache, so the first shape printed pays for warming pages the later
        // ones then find warm — the first column is order-dependent and must not
        // be read as a comparison between shapes. `results/guard-cost.md` is the
        // comparison: it interleaves, over tens of rounds, on one shared file.
        // What these two columns are good for is the *spread* between a cold
        // page cache and a warm one on a 184 MB database, which is a real
        // operating condition and is not visible anywhere else here.
        let (first_us, rows) = time(&connection, &sql, &params);
        let (second_us, again) = time(&connection, &sql, &params);
        assert_eq!(rows, again, "two executions must return the same rows");
        println!("   EXECUTED first_us={first_us} second_us={second_us} rows={rows}");
        println!(
            "GUARD-PLAN\tsize={SIZE}\tshape={}\tfirst_us={first_us}\tsecond_us={second_us}",
            shape.name()
        );
    }

    drop(store);
    remove(&path);
}

fn tempdb(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "happenstance-shipped-guard-{}-{name}.sqlite3",
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
