//! What the adapter actually emits, read off a running store.
//!
//! `crates/happenstance-sqlite/src/query_sql.rs`'s `item_sql` is private, so
//! [`Shape::Chain`] is a *transcription* of it — and a transcription is a claim,
//! not evidence. Reading it twice is not how a claim like that is checked. This
//! target installs a `sqlite3_trace_v2` callback on the adapter's own connection,
//! runs a real two-tag conditional `append` and a real paged `Query::all` replay
//! through the real [`SqliteEventStore`], and prints the statements SQLite was
//! actually handed.
//!
//! Two things come out of it, and both are load-bearing for everything else in
//! this experiment:
//!
//! 1. **The standing guard on the transcription.** The captured guard statement
//!    is asserted **equal** to the string `chain::guard_sql(chain::arms_sql(
//!    Shape::Chain, …))` builds. If `query_sql.rs` changes, this test goes red
//!    and `results/` stops being about SQL nobody runs.
//! 2. **The paged read statement, captured rather than written out.**
//!    `tests/query_plan.rs` explains and times the statement `fetch_page` emits
//!    at page 1,000; taking it from a trace rather than from a string literal is
//!    what makes the plan a plan *of the shipped read*.
//!
//!    **Every page is now one string, page 1 included.** It used not to be:
//!    `resume_from` was a clause the first page omitted, so the first statement
//!    differed from every later one and the steady state was page 2. The merge
//!    shape carries the window as a field with `lo = 0` when there is nothing to
//!    resume from — a position is a `NonZeroU64`, so the clause is vacuous
//!    rather than absent — and an arm that carried the window on some pages and
//!    not others would be two shapes wearing one name. This file asserts the
//!    stronger property that replaced the old one.
//!
//! It is a **debug** target on purpose. It reads SQL rather than a clock, so a
//! release build would buy nothing and the `run.sh` step that runs it is the one
//! that must never be skipped for time.
//!
//! Run it with `cargo test --manifest-path
//! experiments/shipped-append-condition-sql/Cargo.toml --test emitted_sql --
//! --nocapture`.

mod support;

use std::sync::{Mutex, OnceLock};

use correlated_exists_guard::chain::{Selectivity, Shape, arms_sql, guard_sql};
use correlated_exists_guard::{Conditions, seed};
use happenstance_core::AppendCondition;
use happenstance_core::{Event, EventStore, Query, ReadOptions};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::fixtures::{condition, query_of, tagged_event};
use rusqlite::trace::{TraceEvent, TraceEventCodes};

/// Events written before anything is traced.
///
/// Three pages of 512 plus a remainder, so that a `Query::all` replay emits a
/// first page, a steady-state page and a last page — which is the only way to see
/// that the steady-state statement is one string.
const EVENTS: usize = 1_100;

/// `PAGE_SIZE` — `crates/happenstance-sqlite/src/event_store.rs:141`.
const PAGE: usize = 512;

/// Every statement the traced connection has been handed.
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

/// Every traced statement containing `needle`, in order.
fn traced(needle: &str) -> Vec<String> {
    log()
        .lock()
        .map(|entries| {
            entries
                .iter()
                .filter(|sql| sql.contains(needle))
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_adapter_emits_the_chain_this_crate_transcribes() {
    let path = tempdb();

    // Migrate through the adapter, then re-open the *same file* with a traced
    // connection and wrap it. Tracing from the first statement would fill the log
    // with migration DDL and say nothing.
    drop(SqliteEventStore::open(&path).expect("migrating must succeed"));

    let events: Vec<Event> = (1..=EVENTS)
        .map(|position| {
            let [row, shard] =
                seed::seed_tags(seed::Corpus::SelectiveAndUnselective, position as u64);
            tagged_event(
                seed::SEED_TYPE,
                &[
                    (
                        row.split_once(':').expect("a key:value tag").0,
                        row.split_once(':').expect("a key:value tag").1,
                    ),
                    (
                        shard.split_once(':').expect("a key:value tag").0,
                        shard.split_once(':').expect("a key:value tag").1,
                    ),
                ],
            )
        })
        .collect();

    {
        // Untraced, and in batches of `MAX_EVENTS_PER_BATCH`, because the adapter
        // refuses a wider one and because the write path is not what is being
        // read here.
        let store = SqliteEventStore::open(&path).expect("opening must succeed");
        for batch in events.chunks(SqliteEventStore::MAX_EVENTS_PER_BATCH) {
            EventStore::append(&store, batch, None)
                .await
                .expect("seeding must succeed");
        }
    }

    let connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    let conditions = Conditions::require(&connection);
    println!("== conditions ==");
    println!(
        "{} page_size_rows={PAGE} max_arms={}",
        conditions.line(),
        SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT,
    );

    let traced_connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    traced_connection.trace_v2(TraceEventCodes::SQLITE_TRACE_STMT, Some(record));
    let store = SqliteEventStore::new(traced_connection).expect("the file is migrated");

    // ---- 1. the guard --------------------------------------------------------

    let query = query_of(&[seed::SEED_TYPE], &[("shard", "cold"), ("row", "r7")]);
    let doomed = condition(query.clone());
    let outcome = EventStore::append(&store, &events[..1], Some(&doomed)).await;
    assert!(
        outcome.is_err(),
        "an unbounded guard over a query the log satisfies must be violated: \
         {outcome:?}"
    );

    let emitted = traced("SELECT max(position) FROM");
    assert_eq!(
        emitted.len(),
        1,
        "one guard, one statement — got {emitted:?}"
    );
    println!();
    println!("== the statement a two-tag append-condition guard emits ==");
    println!("{}", emitted[0]);

    let lookup = traced("FROM tag_cardinality");
    assert_eq!(lookup.len(), 1, "one selectivity lookup — got {lookup:?}");
    println!();
    println!("== the tag_cardinality lookup that precedes it (Selectivity::read_for) ==");
    println!("{}", lookup[0]);

    let selectivity = Selectivity::read_for(&connection, &query).expect("the lookup must succeed");
    let items = query.items().expect("the query has items");
    // **The shipped shape moved, and this line is where that is recorded.**
    //
    // Until this crate's findings were adopted, the adapter emitted
    // `Shape::Chain` — an uncorrelated intersection chain with the guard's
    // boundary compared only in Rust — and this assertion named it. It now
    // emits the correlated chain with the boundary bound into the seed arm,
    // which is `Shape::ChainExistsBoundedSeed`.
    //
    // The *other* shapes in `chain.rs` are unchanged and remain what they were:
    // `Shape::Chain` is now a historical shape rather than the shipped one, and
    // every "vs `chain-as-shipped`" ratio in `results/` is a ratio against the
    // adapter as it stood on 2026-09-05, before the change this crate motivated.
    // Those tables are a dated record and are correct as such; this assertion is
    // the standing guard and has to track the present.
    //
    // `experiments/shipped-append-condition-sql/` has the same assertion against
    // `Shape::Chain` and it no longer holds there. That crate is a closed record
    // of a shape that stopped shipping, and its README says so.
    let boundary = guard_boundary(&doomed);
    let mut params = Vec::new();
    let transcribed = guard_sql(&arms_sql(
        Shape::ChainExistsBoundedSeed,
        items,
        &selectivity,
        boundary,
        &mut params,
    ));
    println!();
    println!("== this crate's Shape::ChainExistsBoundedSeed, for comparison ==");
    println!("{transcribed}");
    println!(
        "TRANSCRIPTION\tchain-exists-bounded-seed\tmatches_adapter={}",
        transcribed == emitted[0]
    );
    assert_eq!(
        transcribed, emitted[0],
        "the transcription has drifted from `query_sql::item_sql`; every figure \
         in results/ would be about SQL the adapter does not emit"
    );

    // ---- 2. the paged read ---------------------------------------------------

    log().lock().expect("the log").clear();
    let all = Query::all();
    let mut drained = 0usize;
    {
        use futures_core::Stream;
        let stream = EventStore::read(&store, &all, ReadOptions::default());
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
    assert_eq!(drained, EVENTS, "the whole log must come back");

    let pages = traced("FROM event WHERE position >=");
    println!();
    println!(
        "== the statements a paged Query::all replay emits ({} pages) ==",
        pages.len()
    );
    for (index, page) in pages.iter().enumerate() {
        println!("page {}: {page}", index + 1);
    }
    assert!(
        pages.len() >= 3,
        "{EVENTS} events at {PAGE} a page is three statements, not {}",
        pages.len()
    );
    assert_eq!(
        pages[0], pages[1],
        "page 1 must be the same string as page 2: the window is a field with a          vacuous lower bound, not a clause the first page omits"
    );
    assert_eq!(
        pages[1], pages[2],
        "every page must be one string; only the bound values move"
    );
    println!();
    println!("== the page statement (page 1 == page 2 == page 1,000) ==");
    println!("{}", pages[1]);


    // ---- 3. the paged read of a *tagged* query --------------------------------
    //
    // Section 2 traces `Query::all`, which the adapter serves with a bounded
    // scan and no join at all — so it says nothing about the shape everything
    // in `results/merge-join.md` is about. This traces a two-tag read and
    // asserts the three properties that make SQLite's co-routine merge
    // available, on the statement the running store actually prepared.
    //
    // The unit tests in `query_sql.rs` assert the same three on
    // `page_statements`' output. What they cannot see is `page_window` — the
    // step that resolves `resume_from`, `to`, the ceiling and the direction
    // into the window — so a bug there would leave every unit test green and
    // every figure in `results/` about SQL the adapter does not emit. That gap
    // is the only reason this file exists.

    log().lock().expect("the log").clear();
    let tagged = query_of(&[seed::SEED_TYPE], &[("shard", "cold"), ("row", "r7")]);
    {
        use futures_core::Stream;
        let stream = EventStore::read(&store, &tagged, ReadOptions::default());
        let mut stream = core::pin::pin!(stream);
        let mut context = core::task::Context::from_waker(core::task::Waker::noop());
        loop {
            match Stream::poll_next(stream.as_mut(), &mut context) {
                core::task::Poll::Ready(Some(item)) => {
                    item.expect("every row must decode");
                }
                core::task::Poll::Ready(None) => break,
                core::task::Poll::Pending => tokio::task::yield_now().await,
            }
        }
    }

    let tagged_pages = traced("FROM event JOIN (");
    println!();
    println!("== the statement a paged two-tag read emits ==");
    let page = tagged_pages
        .first()
        .expect("a tagged read must emit at least one page statement");
    println!("{page}");

    // The window is on the arm, where `(tag, position)` can use both columns.
    assert!(
        page.contains("seed.position >= ? AND seed.position <= ?"),
        "the arm lost its window, so every co-routine rewinds to the start of \
         its tag range: {page}"
    );
    // The budget is on the compound, once. A per-arm limit bounds the work at
    // `budget x arms` and costs the merge — 17x-48x on a broad 128-item query.
    assert_eq!(
        page.matches(" LIMIT ?").count(),
        1,
        "the budget must sit on the compound and nowhere else: {page}"
    );
    // A join, not a membership test: `IN` materialises the compound before
    // emitting a row, which is finding I-3.
    assert!(
        !page.contains("position IN ("),
        "the outer wrapper is back: {page}"
    );

    drop(store);
    remove(&path);
}

fn tempdb() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "happenstance-shipped-guard-{}-emitted-sql.sqlite3",
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

/// The boundary the adapter binds into the seed arm, from the condition under
/// trace.
///
/// `after: None` is a boundary of zero, because positions start at one — which
/// is `evaluate`'s own rule, restated here rather than assumed so the two cannot
/// disagree silently.
fn guard_boundary(condition: &AppendCondition) -> i64 {
    condition
        .guards()
        .first()
        .expect("the condition carries one guard")
        .after
        .map_or(0, |after| i64::try_from(after.get()).unwrap_or(i64::MAX))
}
