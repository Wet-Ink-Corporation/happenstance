//! **R-1, head on.** What one page costs in memory when the rows are as large
//! as the adapter says it will accept.
//!
//! `crates/happenstance-sqlite/src/event_store.rs:245` declares
//! `MAX_EVENT_DATA_LEN = 1_048_576` and `:1293` sizes a page by row count alone:
//! `budget = min(remaining, PAGE_SIZE)`, with no byte budget anywhere. R-1's
//! claim follows arithmetically — a 512-row page of megabyte events is about
//! 512 MiB in one `Vec` before a single row reaches the caller — and
//! `instrument (b)`'s table cannot show it, because that table's rows carry a
//! 128-byte payload.
//!
//! So this arm seeds the other end: [`CEILING_EVENTS`] events at exactly the
//! declared ceiling, and reads **one** page at the shipped `PAGE_SIZE = 512`.
//!
//! # Why one page and not a replay
//!
//! The claim is about a single hop's residency. A second page would measure the
//! allocator's reuse of the first page's freed buffers, which is a property of
//! the allocator rather than of the adapter. One page, from a cold cursor, is
//! the whole of the question.
//!
//! # What the seed costs, stated because it is most of the run
//!
//! [`CEILING_EVENTS`] × 1 MiB is written through the real append path in batches
//! of [`CEILING_BATCH`], so the database file and its write-ahead log together
//! exceed a gigabyte for the duration. The `Scratch` removes all three files when
//! the test ends. On a machine short of disk this is the arm to skip, and it is
//! in its own test target so that skipping it does not cost the rest.
//!
//! Run with `cargo test --release --test page_residency_at_the_ceiling --
//! --nocapture`.

use std::time::Duration;

use futures_util::StreamExt;
use happenstance_core::{Event, Query, ReadOptions, SendEventStore, Tags};
use happenstance_sqlite::event_store::SqliteEventStore;
use one_connection_latency::probe;
use one_connection_latency::replica::{Replica, SHIPPED_PAGE_SIZE};
use one_connection_latency::workload::Scratch;

/// How many ceiling-sized events the log carries.
///
/// Eight more than the shipped page size, so that the first page is **full** and
/// the figure is a whole page rather than whatever happened to be there.
const CEILING_EVENTS: usize = SHIPPED_PAGE_SIZE + 8;

/// How many ceiling-sized events one seeding append carries.
///
/// Sixty-four, not the declared `MAX_EVENTS_PER_BATCH` of 256: at this payload
/// a 256-event batch is a 256 MiB transaction, and the seed would be measuring
/// the write-ahead log rather than filling the file.
const CEILING_BATCH: usize = 64;

/// One event at exactly `MAX_EVENT_DATA_LEN`.
fn ceiling_event(i: usize) -> Event {
    let tags = Tags::from_pairs([("all", "1")]).expect("a valid tag");
    #[allow(
        clippy::cast_possible_truncation,
        reason = "a payload byte pattern, not a value"
    )]
    let data = vec![(i % 251) as u8; SqliteEventStore::MAX_EVENT_DATA_LEN];
    Event::new("ceiling", data)
        .expect("a valid event type")
        .with_tags(tags)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn one_page_of_ceiling_sized_events_is_half_a_gigabyte() {
    let scratch = Scratch::new("ceiling-residency");
    let store = Replica::<SHIPPED_PAGE_SIZE>::open(scratch.path()).expect("opening the store");

    println!();
    println!("== R-1: one page at the declared data ceiling ==");
    println!(
        "   PAGE_SIZE = {SHIPPED_PAGE_SIZE} (shipped), MAX_EVENT_DATA_LEN = {} B",
        SqliteEventStore::MAX_EVENT_DATA_LEN
    );
    println!("   seeding {CEILING_EVENTS} events at the ceiling, {CEILING_BATCH} per batch");

    let started = std::time::Instant::now();
    let mut next = 0usize;
    while next < CEILING_EVENTS {
        let end = (next + CEILING_BATCH).min(CEILING_EVENTS);
        let events: Vec<Event> = (next..end).map(ceiling_event).collect();
        store
            .append(&events, None)
            .await
            .unwrap_or_else(|err| panic!("seeding at {next} failed: {err}"));
        next = end;
    }
    println!(
        "   seeded {CEILING_EVENTS} MiB in {:.1} s",
        started.elapsed().as_secs_f64()
    );

    // One page, from a cold cursor, with the residency region open.
    probe::reset();
    probe::set_want_peak(true);
    let query = Query::all();
    let stream = store.read(&query, ReadOptions::new());
    let mut stream = Box::pin(stream);
    let first = stream.next().await;
    probe::set_want_peak(false);
    assert!(
        matches!(first, Some(Ok(_))),
        "the first row of a seeded store should arrive"
    );

    let summary = probe::summarise();
    #[allow(clippy::cast_precision_loss)]
    let mib = summary.peak_bytes as f64 / (1024.0 * 1024.0);
    println!();
    println!(
        "   first page: rows={}  peak={} B ({mib:.1} MiB)  hold={:.1} ms",
        summary.peak_merged_rows, summary.peak_bytes, summary.hold_max_us / 1_000.0
    );
    println!(
        "   the whole read: ReadOptions::limit is None, which is what \
         `run_projection` uses for a rebuild"
    );
    println!();

    // A weak assertion, as everywhere in this crate: it fires only if the
    // instrument stopped working, never on a threshold.
    assert_eq!(
        summary.peak_merged_rows as usize, SHIPPED_PAGE_SIZE,
        "the first page did not fill; the seed is short and the figure is not a \
         whole page"
    );
    assert!(
        summary.pages >= 1,
        "no page was recorded, so nothing was measured"
    );
    // Nothing here waits on anything, but a deadline-free test that hung would
    // name no rule. The sleep is a yield point that lets the blocking pool
    // finish before `Scratch` unlinks the file underneath it.
    tokio::time::sleep(Duration::from_millis(50)).await;
}
