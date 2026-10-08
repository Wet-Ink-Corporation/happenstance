//! Case 1: `happenstance-core`'s `MemoryProjectionStore` — a buffered batch.
//!
//! What this file establishes about the proposed trait, one test each:
//!
//! * the runner over a buffer, with an `async fn apply`, is **ready at its first
//!   poll**, with no runtime — the ADR-0062/PS-6 finding carried to the typed
//!   layer, and the measured cost of the one keyword a buffered projection pays;
//! * `on_error`'s provided default halts: the chunk is rolled back and the
//!   runner reports the projection's **own** error type (PS-28);
//! * an override skips both a decode failure and an apply refusal, and writes
//!   its skip record into the batch that moves the checkpoint;
//! * what `apply` is handed is the `SequencedEvent`'s `id` — the origin's
//!   identity — and not the local position, demonstrated on a log whose local
//!   positions and origin positions disagree (SY-21).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::num::NonZeroUsize;
use core::pin::pin;
use core::task::{Context, Poll, Waker};
use std::sync::{Arc, Mutex};

use apply_shape::domain::{POISON, Ticked, scope, tick, undecodable};
use apply_shape::{ApplyFailure, Delivered, Policy, RunError, SendProjection, run};
use happenstance::{
    Checkpoint, EventId, Json, MemoryEventStore, MemoryProjectionBatch, MemoryProjectionStore,
    MemoryProjectionStoreError, ProjectionId, ProjectionStore, RecordedAt, SequencePosition,
    SequencedEvent, StoreId, Tags,
};

#[derive(Debug, thiserror::Error)]
enum TallyError {
    #[error(transparent)]
    Store(#[from] MemoryProjectionStoreError),
    #[error("refused a tick against `{0}`")]
    Refused(String),
}

/// Counts ticks per key. `skip` chooses whether it overrides `on_error`.
///
/// Implemented as [`SendProjection`] — the flavour a native application
/// writes — and driven by a runner bound on the bare `Projection`, through the
/// blanket impl.
struct Tally {
    id: ProjectionId,
    scope: Tags,
    /// Every id `apply` was handed, in order.
    seen: Arc<Mutex<Vec<EventId>>>,
    counts: std::collections::BTreeMap<String, u64>,
    skipped: u64,
}

impl Tally {
    fn new(name: &'static str) -> Self {
        Self {
            id: ProjectionId::from_static(name),
            scope: scope(),
            seen: Arc::default(),
            counts: std::collections::BTreeMap::new(),
            skipped: 0,
        }
    }
}

impl SendProjection for Tally {
    type Event = Ticked;
    type Store = MemoryProjectionStore;
    type Error = TallyError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Ticked>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), TallyError> {
        self.seen.lock().unwrap().push(event.id());
        let Ticked { key } = event.into_event();
        if key == POISON {
            return Err(TallyError::Refused(key));
        }
        let count = self.counts.entry(key.clone()).or_default();
        *count += 1;
        batch.write(key, *count);
        Ok(())
    }

    // No `on_error`: the provided default, `Policy::Halt`.
}

/// The same fold, with an `on_error` that skips and records the skip.
///
/// Written as `async fn` **in the impl**. The trait's default has to be spelled
/// `-> impl Future` with a block for `trait_variant`'s sake; an implementer
/// overriding it does not, because an `async fn` in an impl refines a
/// `-> impl Future` declaration. So the unusual spelling costs the library one
/// method, and costs an application nothing.
struct SkippingTally(Tally);

impl SendProjection for SkippingTally {
    type Event = Ticked;
    type Store = MemoryProjectionStore;
    type Error = TallyError;

    fn id(&self) -> &ProjectionId {
        &self.0.id
    }

    fn scope(&self) -> &Tags {
        &self.0.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Ticked>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), TallyError> {
        SendProjection::apply(&mut self.0, event, batch).await
    }

    async fn on_error(
        &mut self,
        failure: &ApplyFailure<'_, TallyError>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<Policy, TallyError> {
        // Both halves reach here: the shredded payload and the refused key.
        let _ = failure.id();
        self.0.skipped += 1;
        batch.write("skipped", self.0.skipped);
        Ok(Policy::Skip)
    }
}

fn chunk(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}

/// C9. The **runner**, not only `apply`, polled once with a `Waker::noop`
/// context and no runtime. A projection whose `apply` never awaits is ready at
/// its first poll by async-fn semantics alone, so polling `apply` by itself
/// could not fail. Polling `run` over the two memory stores can: an await that
/// yields anywhere on the runner's path, around `apply` or in the store calls
/// it makes, returns `Pending` here.
#[test]
fn a_buffered_run_is_ready_at_its_first_poll_without_a_runtime() {
    let store = MemoryProjectionStore::new();
    let mut tally = Tally::new("first-poll");
    let events = MemoryEventStore::with_events([tick("a").unwrap(), tick("b").unwrap()]);
    let head = events.last_position();

    // No tokio here: a `Waker::noop` context is the whole executor.
    let mut context = Context::from_waker(Waker::noop());
    let future = pin!(run(&events, &store, &mut tally, &Json, chunk(1)));
    let outcome = future.poll(&mut context);

    let Poll::Ready(Ok(ran)) = outcome else {
        panic!("a buffered run must complete on the first poll: {outcome:?}");
    };
    assert_eq!((ran.applied, ran.through), (2, head));
}

#[tokio::test]
async fn the_provided_on_error_halts_and_reports_the_projections_own_error() {
    let events = MemoryEventStore::with_events([
        tick("a").unwrap(),
        tick(POISON).unwrap(),
        tick("b").unwrap(),
    ]);
    let poison_position = events.snapshot()[1].position;
    let models = MemoryProjectionStore::new();
    let mut tally = Tally::new("halting");

    let error = run(&events, &models, &mut tally, &Json, chunk(8))
        .await
        .unwrap_err();

    match error {
        RunError::Apply {
            position,
            source: TallyError::Refused(key),
            rollback: None,
            progress,
        } => {
            assert_eq!(position, poison_position);
            assert_eq!(key, POISON);
            assert_eq!(progress.through, None);
        }
        other => panic!("expected the projection's own refusal, got {other:?}"),
    }

    assert_eq!(
        models
            .checkpoint(&ProjectionId::from_static("halting"))
            .await
            .unwrap(),
        Checkpoint::NeverRun,
        "the chunk was rolled back whole"
    );
    assert!(
        models.is_empty(),
        "no row from the discarded chunk is visible"
    );
}

#[tokio::test]
async fn an_override_skips_both_failures_and_commits_its_record_with_the_checkpoint() {
    let events = MemoryEventStore::with_events([
        tick("a").unwrap(),
        tick(POISON).unwrap(),
        undecodable().unwrap(),
        tick("a").unwrap(),
    ]);
    let head = events.last_position().unwrap();
    let models = MemoryProjectionStore::new();
    let mut tally = SkippingTally(Tally::new("skipping"));

    // Two chunks of two, so a skip lands in each and both commit.
    let ran = run(&events, &models, &mut tally, &Json, chunk(2))
        .await
        .unwrap();

    assert_eq!((ran.applied, ran.skipped), (2, 2));
    assert_eq!(ran.through, Some(head));
    assert_eq!(models.get("a"), Some(2));
    assert_eq!(models.get("skipped"), Some(2), "the skip record committed");
    assert_eq!(
        models
            .checkpoint(&ProjectionId::from_static("skipping"))
            .await
            .unwrap(),
        Checkpoint::Live { through: head },
        "the checkpoint moved past both skipped events"
    );
}

#[tokio::test]
async fn apply_is_handed_the_origin_identity_not_the_local_position() {
    // An ingested log: the local order is 1, 2, 3; the origin store is another
    // one, and there the same events sat at 30, 10, 20.
    let origin = StoreId::from_bytes([7; 16]);
    let local = StoreId::from_bytes([1; 16]);
    let origin_positions = [30, 10, 20];
    let log: Vec<SequencedEvent> = origin_positions
        .iter()
        .enumerate()
        .map(|(index, &at)| {
            SequencedEvent::new(
                SequencePosition::new(u64::try_from(index).unwrap() + 1).unwrap(),
                EventId::new(origin, SequencePosition::new(at).unwrap()),
                RecordedAt::from_millis(0),
                tick("a").unwrap(),
            )
        })
        .collect();
    let expected: Vec<EventId> = log.iter().map(|event| event.id).collect();
    let events = MemoryEventStore::restore(local, log);
    let models = MemoryProjectionStore::new();
    let mut tally = Tally::new("identity");
    let seen = Arc::clone(&tally.seen);

    run(&events, &models, &mut tally, &Json, chunk(8))
        .await
        .unwrap();

    let seen = seen.lock().unwrap().clone();
    assert_eq!(
        seen, expected,
        "apply saw the origin identities, in local order"
    );
    assert!(
        seen.iter().all(|id| id.store() == origin),
        "no identity was re-minted under the local store"
    );
}
