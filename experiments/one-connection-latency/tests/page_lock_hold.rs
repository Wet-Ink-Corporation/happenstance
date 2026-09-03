//! **Instrument (b).** How long does a read page hold the connection mutex, and
//! what does `PAGE_SIZE` actually control?
//!
//! # The construction
//!
//! One million events on one file, one handle, and a replay driven page by page
//! while a second task appends **through the same handle**. Twelve
//! configurations: `PAGE_SIZE ∈ {64, 128, 512, 2048}` crossed with query width
//! `∈ {1, 400, 1200}` items.
//!
//! The width axis is not a selectivity axis, and the seeded log is shaped to
//! keep it that way (see [`one_connection_latency::workload`]): the 1-item query
//! and the 1,200-item query both match every event, and the 400-item query
//! matches a third. What the width changes is the *plan* —
//! `ceil(arms / MAX_QUERY_ARMS_PER_STATEMENT)` statements per page, which is 1,
//! 1 and 3. That is the multiplier J-5's remediation names, and this file is
//! where it stops being arithmetic.
//!
//! # Two passes per configuration, because one number would be two
//!
//! * **residency, quiet.** The peak-live-bytes counter is process-global (it has
//!   to be — the page runs inside a `spawn_blocking` closure), so a peak taken
//!   while an appender is running is a number about the appender. This pass runs
//!   the replay alone.
//! * **latency, concurrent.** A second task appends on the same handle for the
//!   whole of the replay. This pass reports the appender's p50/p99 and the
//!   page's own lock-hold distribution, and takes no residency figure.
//!
//! Beside them sits the **quiet appender baseline** — the same appender, the
//! same batch, with no replay at all. An append p99 without it is a number with
//! no scale.
//!
//! # The page budget, stated because it bounds what these figures cover
//!
//! A configuration stops at [`LATENCY_PAGES`] pages or [`LATENCY_BUDGET`],
//! whichever comes first, and every row prints the page count it actually
//! reached. NF-003 requires the run to terminate unattended; a 1,200-item query
//! at `PAGE_SIZE = 64` is 15,625 pages over a million-event log, and running all
//! twelve to exhaustion would take hours. What the histogram needs is samples,
//! not completeness. The **full-replay** column extrapolates
//! `pages_total × hold_p50` and is labelled as extrapolation everywhere it
//! appears.
//!
//! Run with `cargo test --release --test page_lock_hold -- --nocapture`.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use futures_util::StreamExt;
use happenstance_core::{Event, Query, ReadOptions, SendEventStore, Tags};
use one_connection_latency::probe::{self, PageSummary};
use one_connection_latency::replica::Replica;
use one_connection_latency::workload::{self, Scratch, query_of_width};

/// How many events the log carries.
const SEED_EVENTS: usize = 1_000_000;

/// How many events one seeding append carries.
///
/// `SqliteEventStore::MAX_EVENTS_PER_BATCH` is 256 and this is it: seeding is
/// not the measurement, and a smaller batch would spend the run's wall clock on
/// transactions rather than on pages.
const SEED_BATCH: usize = 256;

/// Pages sampled in the quiet residency pass.
///
/// Small on purpose. Residency converges after the first page: the merge buffer
/// is `Vec::with_capacity(budget)` and every page fills it the same way, so a
/// longer pass adds samples of a number that does not move.
const RESIDENCY_PAGES: usize = 4;

/// Pages sampled in the concurrent latency pass, at most.
const LATENCY_PAGES: usize = 200;

/// Wall clock one configuration's latency pass may spend, at most.
const LATENCY_BUDGET: Duration = Duration::from_secs(15);

/// How long the quiet appender baseline runs.
const BASELINE: Duration = Duration::from_secs(3);

/// The query widths crossed against every page size.
const WIDTHS: [usize; 3] = [1, 400, 1_200];

/// One measured configuration.
#[derive(Debug)]
struct Row {
    page_size: usize,
    width: usize,
    statements: u64,
    merged_rows: u64,
    peak_bytes: i64,
    residency: PageSummary,
    concurrent: PageSummary,
    append_p50_ms: f64,
    append_p99_ms: f64,
    append_max_ms: f64,
    appends: usize,
}

/// A batch for the concurrent appender.
///
/// Its tag is its own, so that nothing it writes disturbs the `tag_cardinality`
/// rows the read path's queries are ordered by. Four events, because the
/// appender's subject is the *wait*, not the write.
fn appender_batch() -> Vec<Event> {
    (0..4)
        .map(|i| {
            let tags = Tags::from_pairs([("probe", "appender")]).expect("a valid tag");
            Event::new("probe.append", vec![i as u8; 64])
                .expect("a valid event type")
                .with_tags(tags)
        })
        .collect()
}

/// Appends on `store` until `stop`, and reports every append's latency in
/// nanoseconds.
async fn append_loop<const PAGE: usize>(
    store: Replica<PAGE>,
    stop: Arc<AtomicBool>,
    deadline: Instant,
) -> Vec<u64> {
    let mut latencies = Vec::new();
    let events = appender_batch();
    while !stop.load(Ordering::Relaxed) && Instant::now() < deadline {
        let started = Instant::now();
        store
            .append(&events, None)
            .await
            .expect("the concurrent appender should succeed");
        latencies.push(u64::try_from(started.elapsed().as_nanos()).unwrap_or(u64::MAX));
        // One yield between appends. Without it the appender monopolises its
        // worker and the figure becomes a throughput number rather than a
        // latency one.
        tokio::task::yield_now().await;
    }
    latencies
}

/// Drives a replay for at most `pages` pages or `budget` wall clock, and returns
/// how many rows it consumed.
async fn replay<const PAGE: usize>(
    store: &Replica<PAGE>,
    query: &Query,
    pages: usize,
    budget: Duration,
) -> usize {
    let stream = store.read(query, ReadOptions::new());
    let mut stream = Box::pin(stream);
    let want = pages * PAGE;
    let started = Instant::now();
    let mut rows = 0usize;

    while rows < want {
        match stream.next().await {
            Some(Ok(_event)) => rows += 1,
            Some(Err(err)) => panic!("the replay failed at row {rows}: {err}"),
            None => break,
        }
        if rows % PAGE == 0 && started.elapsed() > budget {
            break;
        }
    }
    rows
}

/// One `(PAGE_SIZE, width)` cell: the quiet residency pass, then the concurrent
/// latency pass.
async fn run_config<const PAGE: usize>(path: &Path, width: usize) -> Row {
    let store = Replica::<PAGE>::open(path).expect("opening a replica handle");
    let query = query_of_width(width);

    // --- pass A: residency, with nothing else running ------------------------
    probe::reset();
    probe::set_want_peak(true);
    replay(&store, &query, RESIDENCY_PAGES, Duration::from_secs(60)).await;
    probe::set_want_peak(false);
    let residency = probe::summarise();

    // --- pass B: latency, with an appender on the same handle ----------------
    probe::reset();
    let stop = Arc::new(AtomicBool::new(false));
    // The appender carries its own deadline as well as the flag, so that a
    // replay which ends early cannot leave it running into the next
    // configuration.
    let deadline = Instant::now() + LATENCY_BUDGET + Duration::from_secs(5);
    let appender = tokio::spawn(append_loop(store.clone(), Arc::clone(&stop), deadline));

    replay(&store, &query, LATENCY_PAGES, LATENCY_BUDGET).await;

    stop.store(true, Ordering::Relaxed);
    let mut appends = appender.await.expect("the appender task panicked");
    let concurrent = probe::summarise();

    Row {
        page_size: PAGE,
        width,
        statements: residency.statements_per_page,
        merged_rows: residency.peak_merged_rows,
        peak_bytes: residency.peak_bytes,
        residency,
        concurrent,
        append_p50_ms: workload::percentile_ms(&mut appends, 0.50),
        append_p99_ms: workload::percentile_ms(&mut appends, 0.99),
        append_max_ms: workload::percentile_ms(&mut appends, 1.0),
        appends: appends.len(),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn page_size_controls_the_lock_hold_and_the_residency() {
    let scratch = Scratch::new("page-lock-hold");
    let path = scratch.path().to_path_buf();

    println!();
    println!("== instrument (b): lock hold and residency on one shared handle ==");
    println!(
        "   seeding {SEED_EVENTS} events, {} B payload, 2 tags each, batches of {SEED_BATCH}",
        workload::PAYLOAD_BYTES
    );

    let seeder = Replica::<512>::open(&path).expect("opening the seeding handle");
    workload::seed(&seeder, SEED_EVENTS, SEED_BATCH).await;

    // --- the appender's own floor -------------------------------------------
    let stop = Arc::new(AtomicBool::new(false));
    let deadline = Instant::now() + BASELINE;
    let quiet = tokio::spawn(append_loop(seeder.clone(), Arc::clone(&stop), deadline));
    tokio::time::sleep(BASELINE).await;
    stop.store(true, Ordering::Relaxed);
    let mut baseline = quiet.await.expect("the baseline appender panicked");
    let baseline_n = baseline.len();
    let baseline_p50 = workload::percentile_ms(&mut baseline, 0.50);
    let baseline_p99 = workload::percentile_ms(&mut baseline, 0.99);
    let baseline_max = workload::percentile_ms(&mut baseline, 1.0);
    println!(
        "   quiet appender baseline (no replay): p50={baseline_p50:.3} ms  \
         p99={baseline_p99:.3} ms  max={baseline_max:.3} ms  over {baseline_n} appends"
    );
    drop(seeder);
    println!();

    let mut rows: Vec<Row> = Vec::new();
    for width in WIDTHS {
        rows.push(run_config::<64>(&path, width).await);
        rows.push(run_config::<128>(&path, width).await);
        rows.push(run_config::<512>(&path, width).await);
        rows.push(run_config::<2048>(&path, width).await);
    }

    println!();
    println!("-- shape: what PAGE_SIZE and width buy a page ---------------------------------");
    println!(
        "{:>5} {:>6} {:>6} {:>8} {:>14} {:>12}",
        "page", "width", "stmts", "rows/pg", "peak_bytes", "bytes/row"
    );
    for row in &rows {
        #[allow(clippy::cast_precision_loss)]
        let per_row = if row.merged_rows == 0 {
            0.0
        } else {
            row.peak_bytes as f64 / row.merged_rows as f64
        };
        println!(
            "{:>5} {:>6} {:>6} {:>8} {:>14} {:>12.0}",
            row.page_size, row.width, row.statements, row.merged_rows, row.peak_bytes, per_row
        );
    }

    println!();
    println!("-- hold: how long one page owns the connection mutex --------------------------");
    println!(
        "{:>5} {:>6} {:>7} {:>11} {:>11} {:>11} {:>11}",
        "page", "width", "pages", "quiet_p50", "conc_p50", "conc_p99", "conc_max"
    );
    for row in &rows {
        println!(
            "{:>5} {:>6} {:>7} {:>9.3}ms {:>9.3}ms {:>9.3}ms {:>9.3}ms",
            row.page_size,
            row.width,
            row.concurrent.pages,
            row.residency.hold_p50_us / 1_000.0,
            row.concurrent.hold_p50_us / 1_000.0,
            row.concurrent.hold_p99_us / 1_000.0,
            row.concurrent.hold_max_us / 1_000.0,
        );
    }

    println!();
    println!("-- cost: what a caller sharing the handle pays --------------------------------");
    println!(
        "   (quiet appender baseline: p50={baseline_p50:.3} ms  p99={baseline_p99:.3} ms)"
    );
    println!(
        "{:>5} {:>6} {:>9} {:>11} {:>11} {:>11} {:>12}",
        "page", "width", "appends", "app_p50", "app_p99", "app_max", "page_wait_p50"
    );
    for row in &rows {
        println!(
            "{:>5} {:>6} {:>9} {:>9.3}ms {:>9.3}ms {:>9.3}ms {:>10.3}ms",
            row.page_size,
            row.width,
            row.appends,
            row.append_p50_ms,
            row.append_p99_ms,
            row.append_max_ms,
            row.concurrent.wait_p50_us / 1_000.0,
        );
    }

    println!();
    println!("-- extrapolation: a full {SEED_EVENTS}-event replay at each configuration ------");
    println!("   (pages_total x quiet hold_p50; an extrapolation, not a measurement)");
    for row in &rows {
        let pages_total = SEED_EVENTS.div_ceil(row.page_size);
        #[allow(clippy::cast_precision_loss)]
        let seconds = pages_total as f64 * row.residency.hold_p50_us / 1_000_000.0;
        println!(
            "{:>5} {:>6} {:>10} pages  {:>10.1} s of held mutex",
            row.page_size, row.width, pages_total, seconds
        );
    }
    println!();

    // As in `reactor_stall.rs`: the only assertion is that the instrument
    // worked. A threshold on a figure would be this crate quietly becoming a
    // gate step (CF-34).
    assert!(
        rows.iter().all(|row| row.concurrent.pages > 0),
        "a configuration sampled no pages at all; its budget is too small to \
         measure anything and its row is not a result"
    );
}
