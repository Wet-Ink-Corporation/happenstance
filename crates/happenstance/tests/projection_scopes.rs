//! PS-41: a typed projection over several scopes reads their union.
//!
//! `Projection::scopes` names several tag sets, and an event matching any of
//! them is the projection's. The rule, `several_scopes_are_one_query`, is held
//! against both shipped runners and against the two wrong runners it exists
//! to reject: one that derives its query from `scope()` alone, which is every
//! runner before `scopes` existed and every wrapper that forwards `scope()`
//! but not `scopes()`; and one that ORs the scopes' tags with the event types
//! left out, which nominates a type the projection cannot decode.
//!
//! That a projection which does not override `scopes` keeps the checkpoint id
//! it had before is pinned by `the_runner_commits_under_the_derived_id`'s
//! golden in `projection_runner.rs`, and by `checkpoint_id`'s own example.
//!
//! Positions come from the store's own snapshot, never from a literal: the
//! specification permits gaps.
#![cfg(all(feature = "memory", feature = "json"))]

use core::num::NonZeroUsize;

use happenstance::bytes::Bytes;
use happenstance::{
    Authority, Checkpoint, Chunk, Codec, CodecError, Delivered, DomainEvent, Event, EventStore,
    EventType, InvalidCheckpointId, InvalidQuery, Json, MemoryEventStore, MemoryProjectionBatch,
    MemoryProjectionStore, MemoryProjectionStoreError, MemoryStoreError, Progressed, Projection,
    ProjectionError, ProjectionId, ProjectionStore, Query, QueryItem, ReadOptions, SendProjection,
    SequencePosition, Tags, ViewError, checkpoint_id, collect, run_fan_out, run_projection,
    source_checkpoint_id,
};

// ---------------------------------------------------------------------------
// One domain: a document filed against one or more cases
// ---------------------------------------------------------------------------

const FILED: EventType = EventType::from_static("DocumentFiled");

/// A type this projection does not declare, tagged like one it does.
const ARCHIVED: EventType = EventType::from_static("CaseArchived");

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct Filed {
    doc: String,
    cases: Vec<String>,
}

fn case(id: &str) -> Tags {
    Tags::from_pairs([("case", id)]).expect("a valid tag pair")
}

impl DomainEvent for Filed {
    const EVENT_TYPES: &'static [EventType] = &[FILED];

    fn event_type(&self) -> EventType {
        FILED
    }

    fn tags(&self) -> Tags {
        Tags::from_pairs(self.cases.iter().map(|c| ("case", c.as_str()))).expect("valid tags")
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        if *event_type != FILED {
            return Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            });
        }
        codec.decode(data)
    }
}

fn filed(doc: &str, cases: &[&str]) -> Filed {
    Filed {
        doc: doc.to_owned(),
        cases: cases.iter().map(|&c| c.to_owned()).collect(),
    }
}

fn landed(event: &Filed) -> Event {
    let payload = event.encode(&Json).expect("the fixture encodes");
    Event::new(event.event_type(), payload)
        .expect("a valid event type")
        .with_tags(event.tags())
}

// ---------------------------------------------------------------------------
// The projection: every document filed against a case I own
// ---------------------------------------------------------------------------

/// The documents of the cases one owner holds.
///
/// `common` is what `scope()` answers: the tags every owned case shares,
/// which is none. No runner reads it once `scopes` is overridden.
#[derive(Debug)]
struct Docket {
    id: ProjectionId,
    common: Tags,
    cases: Vec<Tags>,
    seen: Vec<Filed>,
}

impl Docket {
    fn owning(cases: &[&str]) -> Self {
        Self {
            id: ProjectionId::from_static("docket"),
            common: Tags::empty(),
            cases: cases.iter().map(|c| case(c)).collect(),
            seen: Vec::new(),
        }
    }
}

impl Projection for Docket {
    type Event = Filed;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.common
    }

    fn scopes(&self) -> &[Tags] {
        &self.cases
    }

    async fn apply(
        &mut self,
        event: Delivered<Filed>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        let event = event.into_event();
        batch.write(event.doc.clone(), 1);
        self.seen.push(event);
        Ok(())
    }
}

/// The wrong runner's view of a [`Docket`]: a wrapper that forwards `scope()`
/// and not `scopes()`, so the provided `scopes` wraps `scope()` alone.
struct ScopeOnly<'a>(&'a mut Docket);

impl Projection for ScopeOnly<'_> {
    type Event = Filed;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        self.0.id()
    }

    fn scope(&self) -> &Tags {
        self.0.scope()
    }

    async fn apply(
        &mut self,
        event: Delivered<Filed>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.0.apply(event, batch).await
    }
}

// ---------------------------------------------------------------------------
// The runners the rule is held against
// ---------------------------------------------------------------------------

/// How many events a run applied, or its error rendered: the runners' error
/// types differ, and `Progressed` is not constructible outside the crate.
type Outcome = Result<usize, String>;

trait ScopedRunner {
    async fn run(
        &self,
        events: &MemoryEventStore,
        models: &MemoryProjectionStore,
        docket: &mut Docket,
    ) -> Outcome;
}

/// Two events a chunk, so the five-event log spans three chunks.
fn chunk() -> Chunk {
    Chunk::of(NonZeroUsize::new(2).expect("non-zero"))
}

/// `run_projection`, as shipped.
struct Single;

impl ScopedRunner for Single {
    async fn run(
        &self,
        events: &MemoryEventStore,
        models: &MemoryProjectionStore,
        docket: &mut Docket,
    ) -> Outcome {
        run_projection(events.into(), models, docket, &Json, chunk())
            .await
            .map(|progress| progress.applied)
            .map_err(|error| format!("{error:?}"))
    }
}

/// `run_fan_out`, as shipped, over one view.
struct Fanned;

impl ScopedRunner for Fanned {
    async fn run(
        &self,
        events: &MemoryEventStore,
        models: &MemoryProjectionStore,
        docket: &mut Docket,
    ) -> Outcome {
        let ran = run_fan_out(
            events.into(),
            models,
            core::slice::from_mut(docket),
            &Json,
            chunk(),
        )
        .await;
        if let Some(read) = ran.read {
            return Err(format!("the read failed: {read:?}"));
        }
        match <[_; 1]>::try_from(ran.views) {
            Ok([outcome]) => outcome
                .map(|progress| progress.applied)
                .map_err(|error| format!("{error:?}")),
            Err(views) => Err(format!("one view, {} outcomes", views.len())),
        }
    }
}

/// The named wrong runner: the query derived from `scope()` alone.
struct ReadsOnlyScope;

impl ScopedRunner for ReadsOnlyScope {
    async fn run(
        &self,
        events: &MemoryEventStore,
        models: &MemoryProjectionStore,
        docket: &mut Docket,
    ) -> Outcome {
        let mut narrowed = ScopeOnly(docket);
        run_projection(events.into(), models, &mut narrowed, &Json, chunk())
            .await
            .map(|progress| progress.applied)
            .map_err(|error| format!("{error:?}"))
    }
}

/// The second wrong runner: one item per scope, but tags alone, with the
/// event types left out (`QueryItem::tagged`). Otherwise as `run_projection`
/// over the public API: it decodes, applies, and commits once at the end
/// under `checkpoint_id`, stopping at the first event it cannot decode.
struct DropsTheTypes;

impl ScopedRunner for DropsTheTypes {
    async fn run(
        &self,
        events: &MemoryEventStore,
        models: &MemoryProjectionStore,
        docket: &mut Docket,
    ) -> Outcome {
        let items = docket.scopes().iter().cloned().map(QueryItem::tagged);
        let query = Query::from_items(items.collect::<Result<Vec<_>, _>>().expect("tagged"))
            .expect("at least one scope");
        let read = collect(EventStore::read(events, &query, ReadOptions::new()))
            .await
            .expect("the memory log reads");
        let mut batch = models.begin().await.expect("a buffered begin");
        let mut applied = 0;
        let mut through = None;
        for event in read {
            let decoded = Filed::decode(&Json, event.event.event_type(), event.event.data())
                .map_err(|error| format!("decode at {:?}: {error}", event.position))?;
            docket
                .apply(Delivered::new(event.id, decoded), &mut batch)
                .await
                .expect("the docket cannot fail");
            applied += 1;
            through = Some(event.position);
        }
        let Some(through) = through else {
            return Err("nothing was read".to_owned());
        };
        let key = checkpoint_id(docket).expect("a derivable id");
        models
            .commit(batch, &key, through, Authority::Live)
            .await
            .expect("the commit lands");
        Ok(applied)
    }
}

// ---------------------------------------------------------------------------
// PS-41 — `several_scopes_are_one_query`
// ---------------------------------------------------------------------------

/// PS-41's rule, against any runner. `Err` names every leg that failed.
///
/// The owner holds `c1` and `c2`, not `c3`. The log is a `c1` filing, a `c3`
/// filing, a `CaseArchived` tagged `c1`, a `c2` filing and one filing against
/// both `c1` and `c2`. The run must apply exactly the three owned filings,
/// once each and in log order (leg A); commit under `checkpoint_id` through
/// the last of them (leg B); and return `Ok`, reporting those three (leg C).
/// It cannot return `Ok` if it was handed the `CaseArchived`, whose decode
/// refuses it.
async fn several_scopes_hold(runner: &impl ScopedRunner) -> Result<(), String> {
    let events = MemoryEventStore::new();
    let c1 = filed("d1", &["c1"]);
    let c3 = filed("d2", &["c3"]);
    let c2 = filed("d4", &["c2"]);
    let both = filed("d5", &["c1", "c2"]);
    EventStore::append(&events, &[landed(&c1), landed(&c3)], None)
        .await
        .expect("the fixture seeds");
    let archived = Event::new(ARCHIVED, Bytes::from_static(b"not json"))
        .expect("a valid event type")
        .with_tags(case("c1"));
    EventStore::append(&events, &[archived], None)
        .await
        .expect("the fixture seeds");
    EventStore::append(&events, &[landed(&c2), landed(&both)], None)
        .await
        .expect("the fixture seeds");
    let last: SequencePosition = events.snapshot().last().expect("seeded").position;

    let models = MemoryProjectionStore::new();
    let mut docket = Docket::owning(&["c1", "c2"]);
    let ran = runner.run(&events, &models, &mut docket).await;

    let mut failed = Vec::new();
    let owned = vec![c1, c2, both];
    if docket.seen != owned {
        failed.push(format!("leg A: the run applied {:?}", docket.seen));
    }
    let key = checkpoint_id(&docket).expect("a derivable id");
    let at = models.checkpoint(&key).await.expect("readable");
    if at != (Checkpoint::Live { through: last }) {
        failed.push(format!("leg B: the checkpoint under {key} is {at:?}"));
    }
    match ran {
        Ok(applied) if applied == owned.len() => {}
        other => failed.push(format!("leg C: the run returned {other:?}")),
    }
    if failed.is_empty() {
        Ok(())
    } else {
        Err(failed.join("\n"))
    }
}

/// The legs a failure names, in order.
fn legs(outcome: Result<(), String>) -> Vec<char> {
    let Err(failed) = outcome else {
        return Vec::new();
    };
    failed
        .lines()
        .filter_map(|line| line.strip_prefix("leg ")?.chars().next())
        .collect()
}

#[tokio::test]
async fn several_scopes_are_one_query() {
    assert_eq!(several_scopes_hold(&Single).await, Ok(()));
}

#[tokio::test]
async fn several_scopes_are_one_query_in_a_fan_out() {
    assert_eq!(several_scopes_hold(&Fanned).await, Ok(()));
}

/// The wrong runner reads `common`, the empty scope, so it is handed every
/// filing (the types still keep the `CaseArchived` out): it applies `c3`'s
/// filing (A), commits under the key its own one-scope query derives, which
/// leaves the scopes' key at `NeverRun` (B), and reports four applied (C).
#[tokio::test]
async fn the_scopes_rule_rejects_a_runner_reading_only_scope() {
    assert_eq!(
        legs(several_scopes_hold(&ReadsOnlyScope).await),
        ['A', 'B', 'C']
    );
}

/// The runner that drops the types is handed the `CaseArchived` tagged `c1`
/// and stops on it: it has applied `c1`'s filing alone (A), committed
/// nothing (B), and returns the decode failure (C).
#[tokio::test]
async fn the_scopes_rule_rejects_a_runner_that_drops_the_types() {
    assert_eq!(
        legs(several_scopes_hold(&DropsTheTypes).await),
        ['A', 'B', 'C']
    );
}

// ---------------------------------------------------------------------------
// The derived ids follow the set, not its order
// ---------------------------------------------------------------------------

/// A projection with one scope, which does not override `scopes`.
struct OneCase {
    id: ProjectionId,
    scope: Tags,
}

impl Projection for OneCase {
    type Event = Filed;
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
        _: Delivered<Filed>,
        _: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        Ok(())
    }
}

/// Naming one scope through `scopes` derives the id that scope derives
/// through `scope()`, so moving a projection onto `scopes` does not rebuild
/// it. Order and a repeat do not move the id; adding a scope moves it and
/// the pin beside it (PS-25, PS-40), so a newly owned case's history is
/// replayed.
#[test]
fn the_ids_follow_the_set_of_scopes() {
    let one = OneCase {
        id: ProjectionId::from_static("docket"),
        scope: case("c1"),
    };
    let named = Docket::owning(&["c1"]);
    assert_eq!(
        checkpoint_id(&one).expect("a derivable id"),
        checkpoint_id(&named).expect("a derivable id"),
    );
    assert_eq!(
        source_checkpoint_id(&one).expect("a derivable id"),
        source_checkpoint_id(&named).expect("a derivable id"),
    );

    let forwards = Docket::owning(&["c1", "c2"]);
    let backwards = Docket::owning(&["c2", "c1", "c2"]);
    assert_eq!(
        checkpoint_id(&forwards).expect("a derivable id"),
        checkpoint_id(&backwards).expect("a derivable id"),
    );
    let more = Docket::owning(&["c1", "c2", "c3"]);
    assert_ne!(
        checkpoint_id(&forwards).expect("a derivable id"),
        checkpoint_id(&more).expect("a derivable id"),
    );
    assert_ne!(
        source_checkpoint_id(&forwards).expect("a derivable id"),
        source_checkpoint_id(&more).expect("a derivable id"),
    );
    assert_ne!(
        checkpoint_id(&forwards).expect("a derivable id"),
        checkpoint_id(&named).expect("a derivable id"),
    );
}

// ---------------------------------------------------------------------------
// No scopes at all
// ---------------------------------------------------------------------------

/// An empty `scopes` is refused before anything is read: each runner returns
/// `Boundary(NoItems)`, applies nothing and writes no row, checkpoint or pin,
/// and `checkpoint_id` refuses it the same way. A runner that read
/// everything would apply both filings; one that read nothing would return
/// `Ok`.
#[tokio::test]
async fn no_scopes_is_refused_before_anything_is_read() {
    let events = MemoryEventStore::new();
    EventStore::append(
        &events,
        &[landed(&filed("d1", &["c1"])), landed(&filed("d2", &["c2"]))],
        None,
    )
    .await
    .expect("the fixture seeds");
    let models = MemoryProjectionStore::new();
    let mut docket = Docket::owning(&[]);

    let ran: Result<Progressed, ProjectionError<MemoryStoreError, _, _>> =
        run_projection((&events).into(), &models, &mut docket, &Json, chunk()).await;
    assert!(
        matches!(ran, Err(ProjectionError::Boundary(InvalidQuery::NoItems))),
        "{ran:?}"
    );
    assert!(docket.seen.is_empty());
    assert_eq!(
        checkpoint_id(&docket),
        Err(InvalidCheckpointId::Query(InvalidQuery::NoItems))
    );
    assert_eq!(
        source_checkpoint_id(&docket),
        Err(InvalidCheckpointId::Query(InvalidQuery::NoItems))
    );

    let fanned = run_fan_out(
        (&events).into(),
        &models,
        core::slice::from_mut(&mut docket),
        &Json,
        chunk(),
    )
    .await;
    assert!(fanned.read.is_none(), "{:?}", fanned.read);
    assert!(
        matches!(
            fanned.views.as_slice(),
            [Err(ViewError::Stopped(ProjectionError::Boundary(
                InvalidQuery::NoItems
            )))]
        ),
        "{:?}",
        fanned.views
    );
    assert!(docket.seen.is_empty());
    assert!(models.snapshot().is_empty(), "{:?}", models.snapshot());
}

// ---------------------------------------------------------------------------
// The Send flavour inherits the provided method
// ---------------------------------------------------------------------------

/// A `Send` projection that overrides `scope` and not `scopes`.
struct SendCase {
    id: ProjectionId,
    scope: Tags,
}

impl SendProjection for SendCase {
    type Event = Filed;
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
        _: Delivered<Filed>,
        _: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        Ok(())
    }
}

/// `trait_variant` copies `scopes`' default body into `SendProjection`, and
/// the blanket `Projection` impl forwards to it: a `Send` projection that
/// overrides only `scope` is read as that one scope through either trait.
#[test]
fn the_send_flavour_provides_scopes() {
    let one = SendCase {
        id: ProjectionId::from_static("one"),
        scope: case("c1"),
    };
    assert_eq!(SendProjection::scopes(&one), [case("c1")]);
    assert_eq!(Projection::scopes(&one), [case("c1")]);
}
