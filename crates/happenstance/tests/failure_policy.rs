//! PS-27: a projection decides what happens to an event it cannot take.
//!
//! `Projection::on_error` is provided and halts by default. An override that
//! answers `Policy::Skip` writes its record into the batch it is handed, and
//! the runner commits that record with the checkpoint that moves past the
//! poisoned event. `skip_and_record_is_atomic` is the rule, and it is run here
//! against `run_projection` **and** against three wrong runners, one defect
//! each, written over the public API alone. A rule no runner can fail is
//! decorative; these three are the runners it fails.
//!
//! Positions always come from the store's own snapshot and never from a
//! literal: the specification permits gaps.
#![cfg(all(feature = "unstable-projection", feature = "memory", feature = "json"))]

use core::num::NonZeroUsize;

use happenstance::bytes::Bytes;
use happenstance::{
    ApplyFailure, Authority, Checkpoint, Chunk, Codec, CodecError, Delivered, DomainEvent, Event,
    EventId, EventStore, EventType, Json, MemoryEventStore, MemoryProjectionBatch,
    MemoryProjectionStore, MemoryProjectionStoreError, Policy, Projection, ProjectionError,
    ProjectionId, ProjectionStore, Query, ReadOptions, SequencePosition, Tags, checkpoint_id,
    collect, run_projection,
};
use happenstance_testkit::block_on;

// ---------------------------------------------------------------------------
// One domain, its events, and the projections over it
// ---------------------------------------------------------------------------

const SHREDDED: EventType = EventType::from_static("PartShredded");

/// The VIN `Recorder::apply` refuses while it has refusals left.
const REFUSE: &str = "REFUSE";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Part {
    Shredded { vin: String },
}

impl DomainEvent for Part {
    const EVENT_TYPES: &'static [EventType] = &[SHREDDED];

    fn event_type(&self) -> EventType {
        SHREDDED
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

fn shredded(vin: &str) -> Event {
    let payload = Json
        .encode(&Part::Shredded {
            vin: vin.to_owned(),
        })
        .expect("the fixture encodes");
    Event::new(SHREDDED, payload).expect("a valid event type")
}

/// An event of a declared type whose payload no codec here can read.
fn poisoned() -> Event {
    Event::new(SHREDDED, &b"shredded, and so is the payload"[..]).expect("a valid event type")
}

fn chunk(n: usize) -> Chunk {
    Chunk::of(NonZeroUsize::new(n).expect("a non-zero chunk"))
}

/// The key `Recorder` writes its skip record under.
fn skipped(id: EventId) -> String {
    format!("skipped:{id}")
}

/// A projection that does not override `on_error`.
struct Shreds {
    id: ProjectionId,
    scope: Tags,
}

impl Projection for Shreds {
    type Event = Part;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Part>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        let Part::Shredded { vin } = event.into_event();
        batch.write(vin, 1);
        Ok(())
    }
}

/// The projection's own refusals.
#[derive(Debug, thiserror::Error)]
enum Refusal {
    #[error("refused the part `{REFUSE}`")]
    Refused,
    #[error("the skip record could not be written")]
    Unrecorded,
    #[error(transparent)]
    Store(#[from] MemoryProjectionStoreError),
}

/// How `Recorder::on_error` answers. An enum rather than flags: each test
/// names the one policy it is about.
#[derive(Debug, Clone, Copy)]
enum Answers {
    /// Records a decode failure under `skipped:<id>` and skips it; halts on
    /// an apply failure. The subject of the rule.
    SkipDecodes,
    /// Records an apply failure under `refused:<id>` and skips it.
    SkipRefusals,
    /// Records any failure under `seen:<id>`, then halts.
    RecordThenHalt,
    /// Fails, as a policy whose record cannot be written does.
    Fails,
}

/// Writes `vin → 1` for every part, refusing [`REFUSE`] while it has
/// refusals left. It refuses before it writes, as `on_error`'s page advises.
struct Recorder {
    id: ProjectionId,
    scope: Tags,
    refusals_left: u32,
    answers: Answers,
}

impl Recorder {
    fn new(answers: Answers, refusals_left: u32) -> Self {
        Self {
            id: ProjectionId::from_static("recorder"),
            scope: Tags::empty(),
            refusals_left,
            answers,
        }
    }
}

impl Projection for Recorder {
    type Event = Part;
    type Store = MemoryProjectionStore;
    type Error = Refusal;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Part>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), Refusal> {
        let Part::Shredded { vin } = event.into_event();
        if vin == REFUSE && self.refusals_left > 0 {
            self.refusals_left -= 1;
            return Err(Refusal::Refused);
        }
        batch.write(vin, 1);
        Ok(())
    }

    async fn on_error(
        &mut self,
        failure: &ApplyFailure<'_, Refusal>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<Policy, Refusal> {
        let id = failure.id();
        match (self.answers, failure) {
            (Answers::SkipDecodes, ApplyFailure::Decode { .. }) => {
                batch.write(skipped(id), 1);
                Ok(Policy::Skip)
            }
            (Answers::SkipRefusals, ApplyFailure::Apply { .. }) => {
                batch.write(format!("refused:{id}"), 1);
                Ok(Policy::Skip)
            }
            (Answers::RecordThenHalt, _) => {
                batch.write(format!("seen:{id}"), 1);
                Ok(Policy::Halt)
            }
            (Answers::Fails, _) => Err(Refusal::Unrecorded),
            _ => Ok(Policy::Halt),
        }
    }
}

/// Seeds `log` into a fresh event store, returning it and a fresh model store.
async fn seeded(log: &[Event]) -> (MemoryEventStore, MemoryProjectionStore) {
    let events = MemoryEventStore::new();
    EventStore::append(&events, log, None)
        .await
        .expect("the fixture seeds");
    (events, MemoryProjectionStore::new())
}

async fn checkpoint_of(models: &MemoryProjectionStore, projection: &Recorder) -> Checkpoint {
    models
        .checkpoint(&checkpoint_id(projection).expect("a derivable id"))
        .await
        .expect("the memory store reads its checkpoint")
}

// ---------------------------------------------------------------------------
// The default
// ---------------------------------------------------------------------------

#[test]
fn the_default_policy_is_halt() {
    block_on(async {
        let (events, models) = seeded(&[poisoned()]).await;
        let id = events.snapshot()[0].id;
        let refusal = Json.decode::<Part>(b"x").expect_err("`x` is not a Part");

        let mut shreds = Shreds {
            id: ProjectionId::from_static("shreds"),
            scope: Tags::empty(),
        };
        let mut batch = models.begin().await.expect("a buffer opens");
        let answer =
            Projection::on_error(&mut shreds, &ApplyFailure::decode(id, &refusal), &mut batch)
                .await;
        assert!(
            matches!(answer, Ok(Policy::Halt)),
            "a projection that says nothing halts: {answer:?}"
        );

        // Committed, so anything the default wrote would now be a row.
        let at = events.snapshot()[0].position;
        models
            .commit(
                batch,
                &ProjectionId::from_static("probe"),
                at,
                Authority::Live,
            )
            .await
            .expect("the probe batch commits");
        assert!(
            models.is_empty(),
            "the default wrote into the batch: {:?}",
            models.snapshot()
        );
    });
}

// ---------------------------------------------------------------------------
// Skip, halt and fail, and what each reports
// ---------------------------------------------------------------------------

#[test]
fn a_skipped_decode_failure_advances_past_it() {
    block_on(async {
        let (events, models) = seeded(&[shredded("VIN1"), poisoned(), shredded("VIN2")]).await;
        let log = events.snapshot();
        let mut recorder = Recorder::new(Answers::SkipDecodes, 0);

        let done = run_projection(&events, &models, &mut recorder, &Json, chunk(8))
            .await
            .expect("a skipped event does not stop the run");

        assert_eq!(done.through, Some(log[2].position));
        assert_eq!(done.applied, 2, "a skip is not an application");
        assert_eq!(done.skipped, 1, "the skip went unreported");
        assert_eq!(
            checkpoint_of(&models, &recorder).await,
            Checkpoint::Live {
                through: log[2].position
            }
        );
        assert_eq!(models.get("VIN1"), Some(1));
        assert_eq!(models.get("VIN2"), Some(1));
        assert_eq!(models.get(&skipped(log[1].id)), Some(1));
    });
}

#[test]
fn an_apply_failure_is_offered_as_apply_with_its_identity() {
    block_on(async {
        let (events, models) =
            seeded(&[shredded("VIN1"), shredded(REFUSE), shredded("VIN2")]).await;
        let log = events.snapshot();
        let mut recorder = Recorder::new(Answers::SkipRefusals, 1);

        let done = run_projection(&events, &models, &mut recorder, &Json, chunk(8))
            .await
            .expect("a skipped refusal does not stop the run");

        // Keyed on the identity the store read: the origin's, never one
        // re-minted from the local position (SY-21).
        assert_eq!(
            models.get(&format!("refused:{}", log[1].id)),
            Some(1),
            "the refusal was not offered as `Apply` under its own id: {:?}",
            models.snapshot()
        );
        assert_eq!(models.get(REFUSE), None, "the refused part was written");
        assert_eq!((done.applied, done.skipped), (2, 1));
    });
}

#[test]
fn a_halting_policy_discards_what_it_wrote() {
    block_on(async {
        let (events, models) = seeded(&[shredded("VIN1"), poisoned()]).await;
        let log = events.snapshot();
        let mut recorder = Recorder::new(Answers::RecordThenHalt, 0);

        let error = run_projection(&events, &models, &mut recorder, &Json, chunk(8))
            .await
            .expect_err("a halting policy stops the run");

        assert!(
            matches!(
                error,
                ProjectionError::Decode { position, rollback: None, .. }
                    if position == log[1].position
            ),
            "a halt is reported as the failure it halted on: {error:?}"
        );
        assert_eq!(models.get(&format!("seen:{}", log[1].id)), None);
        assert_eq!(models.get("VIN1"), None, "the good prefix was committed");
        assert_eq!(
            checkpoint_of(&models, &recorder).await,
            Checkpoint::NeverRun
        );
    });
}

#[test]
fn a_failing_policy_stops_with_the_policy_arm() {
    block_on(async {
        let (events, models) = seeded(&[shredded("VIN1"), poisoned()]).await;
        let poison = events.snapshot()[1].position;
        let mut recorder = Recorder::new(Answers::Fails, 0);

        let error = run_projection(&events, &models, &mut recorder, &Json, chunk(8))
            .await
            .expect_err("a failing policy stops the run");

        assert!(
            matches!(
                error,
                ProjectionError::Policy {
                    position,
                    source: Refusal::Unrecorded,
                    rollback: None,
                    ..
                } if position == poison
            ),
            "the policy's own error is reported in the policy arm: {error:?}"
        );
        assert_eq!(error.position(), Some(poison));
        let progress = error.progress();
        assert_eq!(
            (progress.through, progress.applied, progress.skipped),
            (None, 0, 0)
        );
        assert_eq!(
            checkpoint_of(&models, &recorder).await,
            Checkpoint::NeverRun
        );
    });
}

#[test]
fn a_skip_counts_toward_the_chunk() {
    block_on(async {
        let (events, models) = seeded(&[shredded("VIN1"), poisoned(), shredded(REFUSE)]).await;
        let log = events.snapshot();
        let mut recorder = Recorder::new(Answers::SkipDecodes, 1);

        let error = run_projection(&events, &models, &mut recorder, &Json, chunk(2))
            .await
            .expect_err("the refusal in the second chunk halts");

        // `[VIN1, poison]` is a full chunk of two and committed. A window that
        // counted only applied events would have held all three, and lost them.
        assert_eq!(error.position(), Some(log[2].position));
        let progress = error.progress();
        assert_eq!(
            (progress.through, progress.applied, progress.skipped),
            (Some(log[1].position), 1, 1)
        );
        assert_eq!(
            checkpoint_of(&models, &recorder).await,
            Checkpoint::Live {
                through: log[1].position
            }
        );
    });
}

#[test]
fn skip_and_record_holds_at_every_chunk_and_poison_position() {
    // Exhaustive and deterministic, in place of a property test: every chunk
    // boundary a four-event log has, against the poison at every index.
    for size in 1..=4 {
        for poison in 0..=3 {
            let mut vins = ["VIN_A", "VIN_B", "VIN_C"].into_iter();
            let log: Vec<Event> = (0..=3)
                .map(|at| {
                    if at == poison {
                        poisoned()
                    } else {
                        shredded(vins.next().expect("three good events"))
                    }
                })
                .collect();
            block_on(async {
                let (events, models) = seeded(&log).await;
                let read = events.snapshot();
                let mut recorder = Recorder::new(Answers::SkipDecodes, 0);
                let case = format!("chunk {size}, poison at {poison}");

                let done = run_projection(&events, &models, &mut recorder, &Json, chunk(size))
                    .await
                    .unwrap_or_else(|e| panic!("{case}: {e}"));

                assert_eq!((done.applied, done.skipped), (3, 1), "{case}");
                assert_eq!(
                    checkpoint_of(&models, &recorder).await,
                    Checkpoint::Live {
                        through: read[3].position
                    },
                    "{case}"
                );
                for vin in ["VIN_A", "VIN_B", "VIN_C"] {
                    assert_eq!(models.get(vin), Some(1), "{case}: {vin}");
                }
                assert_eq!(
                    models.get(&skipped(read[poison].id)),
                    Some(1),
                    "{case}: the record"
                );
            });
        }
    }
}

// ---------------------------------------------------------------------------
// The `Send` flavour's override is the one the runner calls
// ---------------------------------------------------------------------------

/// Implements the `Send` flavour, reached by path so only one of the two
/// trait names is in scope, and overrides `on_error` with a plain `async fn`.
struct SendRecorder {
    id: ProjectionId,
    scope: Tags,
}

impl happenstance::SendProjection for SendRecorder {
    type Event = Part;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Part>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        let Part::Shredded { vin } = event.into_event();
        batch.write(vin, 1);
        Ok(())
    }

    async fn on_error(
        &mut self,
        failure: &ApplyFailure<'_, MemoryProjectionStoreError>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<Policy, MemoryProjectionStoreError> {
        batch.write(skipped(failure.id()), 1);
        Ok(Policy::Skip)
    }
}

#[test]
fn a_send_projections_override_is_the_one_the_runner_calls() {
    block_on(async {
        let (events, models) = seeded(&[poisoned(), shredded("VIN1")]).await;
        let poison = events.snapshot()[0].id;
        let mut recorder = SendRecorder {
            id: ProjectionId::from_static("send_recorder"),
            scope: Tags::empty(),
        };

        // `run_projection` binds the bare flavour, so this goes through the
        // derived blanket impl. One that ran the default would halt here.
        let done = run_projection(&events, &models, &mut recorder, &Json, chunk(8))
            .await
            .expect("the override skipped the poison");

        assert_eq!(models.get(&skipped(poison)), Some(1));
        assert_eq!((done.applied, done.skipped), (1, 1));
    });
}

// ---------------------------------------------------------------------------
// PS-27 — `skip_and_record_is_atomic`, and three runners it rejects
// ---------------------------------------------------------------------------

/// The rule's chunk: larger than any log it seeds, so each leg is one chunk.
const EIGHT: NonZeroUsize = NonZeroUsize::new(8).expect("eight is not zero");

/// A projection runner over the memory stores, as the rule sees one.
///
/// `Err` carries the position the run stopped at, where it names one. The
/// window is a bare size, because `Chunk` hands its number only to this
/// crate's own runner.
trait Runner {
    async fn run(
        &self,
        events: &MemoryEventStore,
        models: &MemoryProjectionStore,
        projection: &mut Recorder,
        size: NonZeroUsize,
    ) -> Result<(), Option<SequencePosition>>;
}

/// The runner this crate ships.
struct Shipped;

impl Runner for Shipped {
    async fn run(
        &self,
        events: &MemoryEventStore,
        models: &MemoryProjectionStore,
        projection: &mut Recorder,
        size: NonZeroUsize,
    ) -> Result<(), Option<SequencePosition>> {
        run_projection(events, models, projection, &Json, Chunk::of(size))
            .await
            .map(|_| ())
            .map_err(|error| error.position())
    }
}

/// PS-27's rule, against any runner. `Err` names every leg that failed.
///
/// It reads *"after a crash injected between them"* as: after a skip, the
/// record and the checkpoint past the poison are **both present**; after a
/// failure injected between the record and the commit, both are **absent**;
/// and a restart writes both. Never one without the other.
async fn skip_and_record_holds(runner: &impl Runner) -> Result<(), String> {
    let mut failed = Vec::new();

    // A: the poison is the chunk's last event, so the record and the move
    // are the last two things the chunk does.
    let (events, models) = seeded(&[shredded("VIN1"), poisoned()]).await;
    let log = events.snapshot();
    let mut recorder = Recorder::new(Answers::SkipDecodes, 0);
    let run = runner.run(&events, &models, &mut recorder, EIGHT).await;
    let record = models.get(&skipped(log[1].id));
    let at = checkpoint_of(&models, &recorder).await;
    let moved = Checkpoint::Live {
        through: log[1].position,
    };
    if run.is_err() || record.is_none() || at != moved {
        failed.push(format!(
            "leg A: after a skip, run {run:?}, record {record:?}, checkpoint {at:?}; \
             expected Ok, the record, and the checkpoint at the poison"
        ));
    }

    // B: a refusal after the skip halts the chunk the record is in.
    let (events, models) = seeded(&[shredded("VIN1"), poisoned(), shredded(REFUSE)]).await;
    let log = events.snapshot();
    let mut recorder = Recorder::new(Answers::SkipDecodes, 1);
    let run = runner.run(&events, &models, &mut recorder, EIGHT).await;
    let record = models.get(&skipped(log[1].id));
    let vin1 = models.get("VIN1");
    let at = checkpoint_of(&models, &recorder).await;
    if run != Err(Some(log[2].position))
        || record.is_some()
        || vin1.is_some()
        || at != Checkpoint::NeverRun
    {
        failed.push(format!(
            "leg B: after a failure between the record and the commit, run {run:?}, \
             record {record:?}, VIN1 {vin1:?}, checkpoint {at:?}; expected the stop \
             at the refusal and nothing durable"
        ));
    }

    // C: the restart, its refusal spent, offers the poison again.
    let run = runner.run(&events, &models, &mut recorder, EIGHT).await;
    let record = models.get(&skipped(log[1].id));
    let vin1 = models.get("VIN1");
    let at = checkpoint_of(&models, &recorder).await;
    let moved = Checkpoint::Live {
        through: log[2].position,
    };
    if run.is_err() || record.is_none() || vin1.is_none() || at != moved {
        failed.push(format!(
            "leg C: after the restart, run {run:?}, record {record:?}, VIN1 {vin1:?}, \
             checkpoint {at:?}; expected Ok, every row, and the checkpoint at the end"
        ));
    }

    if failed.is_empty() {
        Ok(())
    } else {
        Err(failed.join("\n"))
    }
}

#[test]
fn skip_and_record_is_atomic() {
    assert_eq!(block_on(skip_and_record_holds(&Shipped)), Ok(()));
}

/// One wrong runner per defect, otherwise the shipped loop (CF-3).
#[derive(Debug, Clone, Copy)]
enum Defect {
    /// On `Skip`, rolls back the batch `on_error` wrote into and moves the
    /// checkpoint with an empty commit: the skip *primitive* PS-27's
    /// *Rejects* names, used as the implementation.
    Swallows,
    /// On `Skip`, leaves the checkpoint at the last *applied* event, as a
    /// reading of "the position the last applied event carries" would.
    StopsShortOfTheSkip,
    /// Hands `on_error` a batch of its own and commits it at once, so that
    /// "a skip record survives a later failure".
    RecordsInItsOwnBatch,
}

/// A runner over the public API alone, carrying one [`Defect`].
struct Mutant(Defect);

impl Mutant {
    /// Offers a failure to `on_error`, into the batch this defect chooses.
    async fn offer(
        &self,
        projection: &mut Recorder,
        failure: &ApplyFailure<'_, Refusal>,
        batch: &mut MemoryProjectionBatch,
        models: &MemoryProjectionStore,
        at: SequencePosition,
    ) -> Result<Policy, Refusal> {
        let Defect::RecordsInItsOwnBatch = self.0 else {
            return projection.on_error(failure, batch).await;
        };
        let mut own = models.begin().await.expect("a buffer opens");
        let answer = projection.on_error(failure, &mut own).await;
        if matches!(answer, Ok(Policy::Skip)) {
            let key = checkpoint_id(projection).expect("a derivable id");
            models
                .commit(own, &key, at, Authority::Live)
                .await
                .expect("the record's own batch commits");
        } else {
            models.rollback(own).await.expect("a buffer drops");
        }
        answer
    }

    /// Decodes and applies one event, offering a failure. `None` is applied.
    async fn take(
        &self,
        projection: &mut Recorder,
        event: &happenstance::SequencedEvent,
        batch: &mut MemoryProjectionBatch,
        models: &MemoryProjectionStore,
    ) -> Option<Result<Policy, Refusal>> {
        let (id, at) = (event.id, event.position);
        match Part::decode(&Json, event.event.event_type(), event.event.data()) {
            Ok(part) => match projection.apply(Delivered::new(id, part), batch).await {
                Ok(()) => None,
                Err(source) => {
                    let failure = ApplyFailure::apply(id, &source);
                    Some(self.offer(projection, &failure, batch, models, at).await)
                }
            },
            Err(source) => {
                let failure = ApplyFailure::decode(id, &source);
                Some(self.offer(projection, &failure, batch, models, at).await)
            }
        }
    }
}

impl Runner for Mutant {
    async fn run(
        &self,
        events: &MemoryEventStore,
        models: &MemoryProjectionStore,
        projection: &mut Recorder,
        size: NonZeroUsize,
    ) -> Result<(), Option<SequencePosition>> {
        let key = checkpoint_id(projection).expect("a derivable id");
        let mut options = ReadOptions::new();
        if let Checkpoint::Live { through } = checkpoint_of(models, projection).await {
            options = options.from(through.next().expect("not the last position"));
        }
        // The fixture log holds only the projection's type, so all of it is
        // the projection's query here.
        let log = collect(EventStore::read(events, &Query::all(), options))
            .await
            .expect("the memory log reads");

        for window in log.chunks(size.get()) {
            let mut batch = models.begin().await.expect("a buffer opens");
            let mut last = None;
            for event in window {
                match self.take(projection, event, &mut batch, models).await {
                    None => last = Some(event.position),
                    Some(Ok(Policy::Skip)) => match self.0 {
                        Defect::Swallows => {
                            models.rollback(batch).await.expect("a buffer drops");
                            let empty = models.begin().await.expect("a buffer opens");
                            models
                                .commit(empty, &key, event.position, Authority::Live)
                                .await
                                .expect("the empty skip commits");
                            batch = models.begin().await.expect("a buffer opens");
                            last = None;
                        }
                        Defect::StopsShortOfTheSkip => {}
                        Defect::RecordsInItsOwnBatch => last = Some(event.position),
                    },
                    Some(_) => {
                        models.rollback(batch).await.expect("a buffer drops");
                        return Err(Some(event.position));
                    }
                }
            }
            match last {
                Some(at) => models
                    .commit(batch, &key, at, Authority::Live)
                    .await
                    .expect("the chunk commits"),
                None => models.rollback(batch).await.expect("a buffer drops"),
            }
        }
        Ok(())
    }
}

/// Runs the rule against one mutant and returns the legs it failed, in order.
fn failed_legs(defect: Defect) -> Vec<char> {
    let Err(message) = block_on(skip_and_record_holds(&Mutant(defect))) else {
        return Vec::new();
    };
    let legs: Vec<char> = message
        .lines()
        .filter_map(|line| line.strip_prefix("leg "))
        .filter_map(|rest| rest.chars().next())
        .collect();
    assert!(!legs.is_empty(), "{defect:?} failed unparseably: {message}");
    legs
}

#[test]
fn skip_and_record_is_atomic_rejects_a_swallowed_skip() {
    // The record never becomes durable, so every leg that looks for it fails;
    // B too, because the empty commit moved the checkpoint without the rows.
    assert_eq!(failed_legs(Defect::Swallows), ['A', 'B', 'C']);
}

#[test]
fn skip_and_record_is_atomic_rejects_a_checkpoint_short_of_the_skip() {
    // Only A ends a chunk on a skip; in B and C the skip is followed by an
    // event that carries the checkpoint past it.
    assert_eq!(failed_legs(Defect::StopsShortOfTheSkip), ['A']);
}

#[test]
fn skip_and_record_is_atomic_rejects_a_record_in_its_own_batch() {
    // A passes: nothing fails after the record. B's refusal leaves the record
    // and its checkpoint durable while `VIN1` went down with the chunk, and
    // C resumes past `VIN1` for good.
    assert_eq!(failed_legs(Defect::RecordsInItsOwnBatch), ['B', 'C']);
}
