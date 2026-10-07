//! The ES-11 fence sweep: the two racing rules' shape, run many times over two
//! transports that differ only in the read-settlement wait.
//!
//! An **instrument**, not a rule. It prints one `ES11-SWEEP {json}` row per
//! trial and never fails on what it measures — a measurement does not gate
//! (CF-34). It fails only when the harness itself cannot run: a seed append or
//! the `before` read failing. The decision rule the rows are judged by was
//! written before the first run, in `experiments/es-11-fence/README.md`, and
//! `experiments/es-11-fence/run.sh` extracts and tallies them from a job log.
//!
//! # One trial
//!
//! Seed, read the scope back as `before`, build a stream over the same scope and
//! poll it **once** on this task, append a late event the scope matches, drain.
//! A trial is `red` when the drained set differs from `before` or carries the
//! late event — exactly the rule's assertion. Every trial is scoped by a tag of
//! its own, so the reads stay small and no trial sees another's events.
//!
//! # The arms
//!
//! `baseline` is [`HyperTransport::shared_unfenced`]: the adapter as it was
//! before the fence, since its `reads_settled` is ready at once. `fence` is
//! [`HyperTransport::shared`]. Same binary, same client, same endpoint, one
//! migrated schema; arms and shapes are interleaved per iteration with the order
//! rotated, so drift in the endpoint falls on both alike.

use core::pin::pin;
use core::task::Poll;
use std::time::Instant;

use futures_core::Stream;
use happenstance_core::{
    Event, EventStore, Query, QueryItem, ReadOptions, SequencePosition, SequencedEvent, Tags,
};
use happenstance_neon::event_store::NeonEventStore;
use happenstance_testkit::Fixture;
use happenstance_testkit::fixtures::tagged_event;

use super::NeonFixture;
use super::transport::{HyperTransport, Stage, Trace, TraceEvent};

/// Trials per arm per shape, per job attempt: 1,000 trials in all. Fixed by the
/// pre-registered decision rule, not tuned after a run.
pub(crate) const TRIALS_PER_CELL: usize = 250;

/// The prefix every row carries, so a job log can be filtered to rows alone.
pub(crate) const ROW_PREFIX: &str = "ES11-SWEEP";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Arm {
    Baseline,
    Fence,
}

impl Arm {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Fence => "fence",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    /// `read_result_is_stable_under_concurrent_append`: one scope, one late event.
    Es11,
    /// `query_items_share_one_snapshot`: two items, the late event matching the
    /// second.
    Es12,
}

impl Shape {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Es11 => "es11",
            Self::Es12 => "es12",
        }
    }
}

/// One arm's store and the trace its transport records into.
struct Instrument {
    arm: Arm,
    store: NeonEventStore<HyperTransport>,
    trace: Trace,
}

/// Where this run came from, for provenance in every row.
struct Provenance {
    run: String,
    attempt: String,
    sha: String,
}

impl Provenance {
    fn from_env() -> Self {
        let read = |name: &str| std::env::var(name).unwrap_or_else(|_| "local".to_owned());
        Self {
            run: read("GITHUB_RUN_ID"),
            attempt: read("GITHUB_RUN_ATTEMPT"),
            sha: read("GITHUB_SHA"),
        }
    }
}

/// Runs the whole sweep and prints its rows.
pub(crate) async fn run() {
    let fixture = NeonFixture::new();
    // Migrates the fixture's schema; the handle itself is not used.
    drop(fixture.connect().await);
    let provenance = Provenance::from_env();
    let instrument = |arm: Arm, transport: HyperTransport| {
        let trace = Trace::default();
        Instrument {
            arm,
            // The store's transport and the sweep each hold the trace, a cheap
            // `Arc` handle: one records, the other drains.
            // The config is cloned because each arm's store owns one.
            store: NeonEventStore::new(transport.traced(trace.clone()), fixture.config().clone()),
            trace,
        }
    };
    let baseline = instrument(Arm::Baseline, HyperTransport::shared_unfenced());
    let fence = instrument(Arm::Fence, HyperTransport::shared());

    println!(
        "{ROW_PREFIX}-META {}",
        serde_json::json!({
            "run": provenance.run,
            "attempt": provenance.attempt,
            "sha": provenance.sha,
            "trials_per_cell": TRIALS_PER_CELL,
        })
    );

    let mut cells = [
        (&baseline, Shape::Es11),
        (&fence, Shape::Es11),
        (&baseline, Shape::Es12),
        (&fence, Shape::Es12),
    ];
    for iter in 0..TRIALS_PER_CELL {
        for &(instrument, shape) in &cells {
            let row = trial(instrument, shape, iter, &provenance).await;
            println!("{ROW_PREFIX} {row}");
        }
        cells.rotate_left(1);
    }
}

/// One trial, as one JSON row.
async fn trial(
    instrument: &Instrument,
    shape: Shape,
    iter: usize,
    provenance: &Provenance,
) -> serde_json::Value {
    let store = &instrument.store;
    let scope = format!(
        "{}-{}-{iter}-{}-{}",
        provenance.run,
        provenance.attempt,
        instrument.arm.as_str(),
        shape.as_str()
    );
    let tags = Tags::from_pairs([("trial", scope.as_str())]).expect("a trial tag is valid");
    let pairs = [("trial", scope.as_str())];
    let late_pairs = [("trial", scope.as_str()), ("item", "late")];

    let (query, late) = match shape {
        Shape::Es11 => {
            seed(store, tagged_event("Seeded", &pairs)).await;
            let query = Query::from_item(QueryItem::tagged(tags).expect("a tagged item is valid"));
            (query, tagged_event("Later", &late_pairs))
        }
        Shape::Es12 => {
            seed(store, tagged_event("Alpha", &pairs)).await;
            seed(store, tagged_event("Omega", &pairs)).await;
            // Each item owns its tag set, and both items carry the same one.
            let items = [
                QueryItem::new(["Alpha"], tags.clone()).expect("a valid item"),
                QueryItem::new(["Omega"], tags).expect("a valid item"),
            ];
            let query = Query::from_items(items).expect("two items are a valid query");
            (query, tagged_event("Omega", &late_pairs))
        }
    };

    let before = read_all(store, &query).await;
    // Discard the seeding and the `before` read: the row is about the window.
    drop(instrument.trace.take());

    let start = Instant::now();
    let mut stream = pin!(store.read(&query, ReadOptions::new()));
    let mut drained: Vec<SequencedEvent> = Vec::new();
    let mut error: Option<String> = None;
    // Once, on this task, exactly as the rule polls it.
    match core::future::poll_fn(|cx| Poll::Ready(stream.as_mut().poll_next(cx))).await {
        Poll::Ready(Some(Ok(event))) => drained.push(event),
        Poll::Ready(Some(Err(err))) => error = Some(format!("first poll: {err}")),
        Poll::Ready(None) | Poll::Pending => {}
    }

    let append_called = Instant::now();
    let late_position: Option<SequencePosition> =
        match store.append(core::slice::from_ref(&late), None).await {
            Ok(position) => Some(position),
            Err(err) => {
                error.get_or_insert_with(|| format!("append: {err}"));
                None
            }
        };

    while let Some(item) = core::future::poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
        match item {
            Ok(event) => drained.push(event),
            Err(err) => {
                error.get_or_insert_with(|| format!("drain: {err}"));
                break;
            }
        }
    }

    let trace = instrument.trace.take();
    let late_in_drained =
        late_position.is_some_and(|late| drained.iter().any(|event| event.position == late));
    let outcome = match error {
        Some(_) => "error",
        None if late_in_drained || snapshot(&drained) != snapshot(&before) => "red",
        None => "pass",
    };
    let at = |read_only: bool, stage: Stage| micros_since(start, &trace, read_only, stage);
    let append_dispatch = at(false, Stage::Dispatched);

    serde_json::json!({
        "run": provenance.run,
        "attempt": provenance.attempt,
        "iter": iter,
        "arm": instrument.arm.as_str(),
        "shape": shape.as_str(),
        "outcome": outcome,
        "error": error,
        "before_n": before.len(),
        "drained_n": drained.len(),
        "late_in_drained": late_in_drained,
        "t_read_dispatch_us": at(true, Stage::Dispatched),
        "t_read_send_us": at(true, Stage::Sent),
        "t_read_answer_us": at(true, Stage::Answered),
        "t_append_call_us": micros(start, append_called),
        "t_append_dispatch_us": append_dispatch,
        "t_append_send_us": at(false, Stage::Sent),
        "t_append_answer_us": at(false, Stage::Answered),
        "fence_wait_us": append_dispatch
            .map(|dispatch| dispatch.saturating_sub(micros(start, append_called))),
        "trace_events": trace.len(),
    })
}

/// A seed the trial cannot run without: a failure here is a harness error.
async fn seed(store: &NeonEventStore<HyperTransport>, event: Event) {
    if let Err(err) = store.append(&[event], None).await {
        panic!("a broken sweep, not a measurement: a seed append failed: {err:?}");
    }
}

/// Reads a whole scope: a failure here is a harness error.
async fn read_all(store: &NeonEventStore<HyperTransport>, query: &Query) -> Vec<SequencedEvent> {
    let mut stream = pin!(store.read(query, ReadOptions::new()));
    let mut events = Vec::new();
    while let Some(item) = core::future::poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
        match item {
            Ok(event) => events.push(event),
            Err(err) => panic!("a broken sweep, not a measurement: `before` failed: {err:?}"),
        }
    }
    events
}

/// What the rule compares: positions and events, in order.
fn snapshot(events: &[SequencedEvent]) -> Vec<(u64, &Event)> {
    events
        .iter()
        .map(|event| (event.position.get(), &event.event))
        .collect()
}

/// When the first trace event matching `read_only` and `stage` was taken,
/// relative to `start`. A trial has one read and one append in its window.
fn micros_since(
    start: Instant,
    trace: &[TraceEvent],
    read_only: bool,
    stage: Stage,
) -> Option<u64> {
    trace
        .iter()
        .find(|event| event.read_only == read_only && event.stage == stage)
        .map(|event| micros(start, event.at))
}

fn micros(start: Instant, at: Instant) -> u64 {
    u64::try_from(at.saturating_duration_since(start).as_micros()).unwrap_or(u64::MAX)
}
