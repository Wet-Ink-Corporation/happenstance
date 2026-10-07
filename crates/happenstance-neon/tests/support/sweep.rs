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
//! A trial is `anchor` when `before` does not hold exactly the seeded events at
//! the positions their appends returned — the rule's own anchor assertion, which
//! a frontier-lagged read can fail with no race at all. Otherwise it is `red`
//! when the drain carries the late event (`late_in_drained`) or differs from
//! `before` (`drained_ne_before`), the rule's two assertions. Every trial is
//! scoped by a tag of its own, so the reads stay small and no trial sees
//! another's events. What the trial does *not* share with the rules is listed
//! in `experiments/es-11-fence/README.md`.
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

/// The row layout `run.sh` pools. The pilot's rows, judged under the
/// pre-amendment rule, carry no `schema` field, and `run.sh` excludes every
/// attempt whose rows are not all of this one.
const ROW_SCHEMA: u32 = 2;

/// Why a trial is red: the rule's two assertions, kept apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reason {
    /// The event appended after the first poll was drained.
    LateInDrained,
    /// The drain differs from a `before` that held exactly the seeded events.
    DrainedNeBefore,
}

impl Reason {
    const fn as_str(self) -> &'static str {
        match self {
            Self::LateInDrained => "late_in_drained",
            Self::DrainedNeBefore => "drained_ne_before",
        }
    }
}

/// What one trial showed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Pass,
    Red(Reason),
    /// `before` did not hold exactly the seeded events, so nothing the window
    /// shows can be compared against it. Every read carries the visibility
    /// frontier, which has been seen to hide committed rows; a lagged `before`
    /// would otherwise score `drained != before` as a race.
    Anchor,
    /// The first poll, the append or the drain failed.
    Error,
}

impl Outcome {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Red(_) => "red",
            Self::Anchor => "anchor",
            Self::Error => "error",
        }
    }

    const fn reason(self) -> Option<Reason> {
        match self {
            Self::Red(reason) => Some(reason),
            Self::Pass | Self::Anchor | Self::Error => None,
        }
    }
}

/// What the window — first poll, append, drain — showed.
#[derive(Debug, Clone, Copy)]
enum Window {
    /// The first poll, the append or the drain returned an error.
    Failed,
    /// The stream drained to its end.
    Drained {
        late_in_drained: bool,
        matches_before: bool,
    },
}

/// What a trial observed, before it is judged.
#[derive(Debug, Clone, Copy)]
struct Observed {
    /// `before` held exactly the seeded events, at the positions their appends
    /// returned: the rule's own anchor assertion.
    before_complete: bool,
    window: Window,
}

/// Judges a trial. The anchor comes first: without a complete `before` the
/// window has nothing to be compared against, whatever else happened in it.
const fn judge(observed: Observed) -> Outcome {
    if !observed.before_complete {
        return Outcome::Anchor;
    }
    match observed.window {
        Window::Failed => Outcome::Error,
        Window::Drained {
            late_in_drained: true,
            ..
        } => Outcome::Red(Reason::LateInDrained),
        Window::Drained {
            matches_before: false,
            ..
        } => Outcome::Red(Reason::DrainedNeBefore),
        Window::Drained { .. } => Outcome::Pass,
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
    let Seeded {
        query,
        late,
        positions: seeded,
    } = seed_shape(store, shape, &scope).await;

    let before = read_all(store, &query).await;
    // The rule's anchor: `before` must hold exactly what was seeded before the
    // window means anything. A frontier lag can hide a committed seed here.
    let before_complete = before
        .iter()
        .map(|event| event.position)
        .eq(seeded.iter().copied());
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
    let window = match error {
        Some(_) => Window::Failed,
        None => Window::Drained {
            late_in_drained,
            matches_before: snapshot(&drained) == snapshot(&before),
        },
    };
    let outcome = judge(Observed {
        before_complete,
        window,
    });
    let at = |read_only: bool, stage: Stage| micros_since(start, &trace, read_only, stage);
    let append_dispatch = at(false, Stage::Dispatched);

    serde_json::json!({
        "schema": ROW_SCHEMA,
        "run": provenance.run,
        "attempt": provenance.attempt,
        "iter": iter,
        "arm": instrument.arm.as_str(),
        "shape": shape.as_str(),
        "outcome": outcome.as_str(),
        "reason": outcome.reason().map(Reason::as_str),
        "error": error,
        "seeded_n": seeded.len(),
        "before_complete": before_complete,
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

/// One trial's scope, seeded: the query over it, the late event it will
/// match, and the positions the seeds were appended at.
struct Seeded {
    query: Query,
    late: Event,
    positions: Vec<SequencePosition>,
}

/// Seeds `shape`'s events under the trial's own tag.
async fn seed_shape(store: &NeonEventStore<HyperTransport>, shape: Shape, scope: &str) -> Seeded {
    let tags = Tags::from_pairs([("trial", scope)]).expect("a trial tag is valid");
    let pairs = [("trial", scope)];
    let late_pairs = [("trial", scope), ("item", "late")];
    match shape {
        Shape::Es11 => Seeded {
            positions: vec![seed(store, tagged_event("Seeded", &pairs)).await],
            query: Query::from_item(QueryItem::tagged(tags).expect("a tagged item is valid")),
            late: tagged_event("Later", &late_pairs),
        },
        Shape::Es12 => {
            let positions = vec![
                seed(store, tagged_event("Alpha", &pairs)).await,
                seed(store, tagged_event("Omega", &pairs)).await,
            ];
            // Each item owns its tag set, and both items carry the same one.
            let items = [
                QueryItem::new(["Alpha"], tags.clone()).expect("a valid item"),
                QueryItem::new(["Omega"], tags).expect("a valid item"),
            ];
            Seeded {
                positions,
                query: Query::from_items(items).expect("two items are a valid query"),
                late: tagged_event("Omega", &late_pairs),
            }
        }
    }
}

/// A seed the trial cannot run without: a failure here is a harness error.
async fn seed(store: &NeonEventStore<HyperTransport>, event: Event) -> SequencePosition {
    match store.append(&[event], None).await {
        Ok(position) => position,
        Err(err) => panic!("a broken sweep, not a measurement: a seed append failed: {err:?}"),
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

#[cfg(test)]
mod tests {
    use super::{Observed, Outcome, Reason, Window, judge};

    const fn complete(window: Window) -> Observed {
        Observed {
            before_complete: true,
            window,
        }
    }

    const fn drained(late_in_drained: bool, matches_before: bool) -> Window {
        Window::Drained {
            late_in_drained,
            matches_before,
        }
    }

    #[test]
    fn a_clean_trial_passes() {
        assert_eq!(judge(complete(drained(false, true))), Outcome::Pass);
    }

    /// W1: a frontier-lagged `before` makes `drained != before` without any
    /// race in the window. That is an anchor failure, never a red — under the
    /// fence a red here would be scored a false C3 falsifier.
    #[test]
    fn an_incomplete_before_is_an_anchor_and_not_a_red() {
        let lagged = Observed {
            before_complete: false,
            window: drained(false, false),
        };

        assert_eq!(judge(lagged), Outcome::Anchor);
    }

    #[test]
    fn an_anchor_outranks_a_late_event_and_an_error() {
        for window in [drained(true, false), Window::Failed] {
            let lagged = Observed {
                before_complete: false,
                window,
            };

            assert_eq!(judge(lagged), Outcome::Anchor, "{window:?}");
        }
    }

    #[test]
    fn a_late_event_in_the_drain_is_red_for_that_reason() {
        assert_eq!(
            judge(complete(drained(true, false))),
            Outcome::Red(Reason::LateInDrained)
        );
    }

    #[test]
    fn a_drain_that_differs_from_a_complete_before_is_red_for_that_reason() {
        assert_eq!(
            judge(complete(drained(false, false))),
            Outcome::Red(Reason::DrainedNeBefore)
        );
    }

    #[test]
    fn a_failed_window_is_an_error() {
        assert_eq!(judge(complete(Window::Failed)), Outcome::Error);
    }

    /// The strings `run.sh` and the README match on.
    #[test]
    fn outcomes_and_reasons_print_as_the_tally_reads_them() {
        assert_eq!(Outcome::Pass.as_str(), "pass");
        assert_eq!(Outcome::Anchor.as_str(), "anchor");
        assert_eq!(Outcome::Error.as_str(), "error");
        assert_eq!(Outcome::Red(Reason::LateInDrained).as_str(), "red");
        assert_eq!(Reason::LateInDrained.as_str(), "late_in_drained");
        assert_eq!(Reason::DrainedNeBefore.as_str(), "drained_ne_before");
    }
}
