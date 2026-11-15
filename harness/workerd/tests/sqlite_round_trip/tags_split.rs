//! VT-36 through a real push: the runner's edge offers the object a push over
//! Cloudflare's tags-per-call ceiling, the object's ingest refuses it as
//! `TagsPerBatch` (413, `Split`), and the edge halves and lands it in two.
//!
//! Three things make the observation possible, and each is pinned here:
//!
//! * the object's wide mount (`/sync-wide/`), because under `/sync/`'s
//!   `PeerLimits::minimum` a push carries at most 128 events, half the ceiling
//!   at 1,024 tags an event;
//! * an origin with no tag cap of its own: SQLite's 128 tags an event cannot
//!   reach the ceiling in any call, so the origin is the testkit's in-memory
//!   ingest store, reached through the harness's direct dependency;
//! * the refusal's text, not only its class: a push this size is also over
//!   `/sync/`'s body cap, which answers 413 `Split` too, for the wrong reason.
//!
//! Every size comes from `WorkerdFixture`'s declarations, the ones the
//! adapter's own ceilings are mirrored into, never from a second literal.
//!
//! The live test drives `Edge::step` by hand. `Edge::run` would retry an
//! object reset (a 500, `Transient`) on a push at or under the ceiling, and
//! that reset is ADR-0100's falsifier, which must stay red.

use core::cell::Cell;
use core::num::NonZeroUsize;

use happenstance_cloudflare::happenstance_sync::runner::{
    Edge, EdgeState, Failure, Phase, StepError, StepFailure, Stepped,
};
use happenstance_cloudflare::happenstance_sync::wire::{ResumeToken, Route};
use happenstance_cloudflare::happenstance_sync::{
    EventGroup, Exchange, FailureClass, IngestStore, PeerLimits, PushBatch, RemoteError,
    RemoteSyncPeer, SyncPeer, WriteGroupStore,
};
use happenstance_cloudflare::sync_http::max_request_bytes;
use happenstance_core::{Event, EventStore, Tag, Tags};
use happenstance_sync_testkit::{IngestFixture, MemoryIngestFixture, MemoryIngestHandle};
use happenstance_testkit::Fixture;
use happenstance_workerd_harness::fixture::WorkerdFixture;
use happenstance_workerd_harness::sync_mount::wide_limits;

use crate::http::{HttpError, RETRY_ATTEMPTS, RETRY_FIRST_PAUSE, refusal};
use crate::{drain, harness, reset};

/// The fixture whose declarations size the scenario.
type Declared = WorkerdFixture<'static>;

/// Tags on each event: the adapter's per-event ceiling, 1,024.
///
/// Evaluated at compile time, so a fixture that stops declaring it fails the
/// build by name rather than a test by arithmetic.
const PER_EVENT: usize = <Declared as Fixture>::MAX_TAGS_PER_EVENT
    .expect("the harness fixture declares the adapter's tags-per-event ceiling");

/// The adapter's tags-per-call ceiling, 262,144: what the refused push is over.
const CEILING: usize = <Declared as Fixture>::MAX_TAGS_PER_BATCH
    .expect("the harness fixture declares the adapter's tags-per-call ceiling");

/// Events in each of the two groups: one past half the ceiling's worth, 129,
/// so two groups are over the ceiling by one event and either alone is under
/// it. Const arithmetic, so a zero divisor is a build error, not a panic.
const GROUP_EVENTS: usize = CEILING / PER_EVENT / 2 + 1;

/// Events in the refused push: both groups.
const TOTAL_EVENTS: usize = 2 * GROUP_EVENTS;

/// Tags in the refused push, 264,192: the number the ingest's refusal names.
const TOTAL_TAGS: usize = TOTAL_EVENTS * PER_EVENT;

/// The most steps the live test takes after the halved push before it decides
/// the edge will never sync: the second group's push and the echo pulls need
/// three, so this is headroom (H-13).
const MAX_STEPS: usize = 8;

/// The scenario's two writes: [`GROUP_EVENTS`] events each, every event
/// carrying [`PER_EVENT`] tags and a one-byte payload.
fn split_scenario() -> [Vec<Event>; 2] {
    let tags: Tags = (0..PER_EVENT)
        .map(|n| Tag::new(format!("t{n}")).expect("a short ASCII tag is valid"))
        .collect();
    let group = || -> Vec<Event> {
        (0..GROUP_EVENTS)
            .map(|_| {
                Event::new("Tagged", vec![0x01])
                    .expect("a static event type is valid")
                    // Each event owns its tags: the data really is duplicated.
                    .with_tags(tags.clone())
            })
            .collect()
    };
    [group(), group()]
}

/// A fresh in-memory ingest store holding the scenario, one append per group,
/// so its log is two write groups.
async fn origin() -> MemoryIngestHandle {
    let local = MemoryIngestFixture::new().connect().await;
    for group in split_scenario() {
        local
            .append(&group, None)
            .await
            .expect("the in-memory store has no tag ceiling");
    }
    local
}

/// `local`'s log in the writes that made it, as the wire carries them.
async fn written(local: &MemoryIngestHandle) -> Vec<EventGroup> {
    local
        .read_groups(
            None,
            NonZeroUsize::new(TOTAL_EVENTS).expect("the scenario has events"),
        )
        .await
        .expect("the local groups read")
        .groups
        .into_iter()
        .map(EventGroup::from)
        .collect()
}

/// One [`Edge::step`], tried again only when the object never answered: a
/// platform miss, no connection, a broken-off body or a timeout, up to
/// [`RETRY_ATTEMPTS`] with a doubling pause, as `retrying` pauses.
///
/// Never a refusal, whatever its class. A 500 is `Transient`, and on a push at
/// or under the ceiling it is an object reset, ADR-0100's falsifier: retried,
/// a deployed reset would be hidden behind the next attempt. A retry after a
/// push that landed and then timed out is safe, because ingest is idempotent.
async fn step_past_platform_misses<X, S>(
    edge: &mut Edge<ResumeToken>,
    peer: &RemoteSyncPeer<X>,
    store: &S,
) -> Result<Stepped, StepFailure<RemoteSyncPeer<X>, S>>
where
    X: Exchange<Error = HttpError>,
    S: WriteGroupStore + IngestStore,
{
    let mut pause = RETRY_FIRST_PAUSE;
    let mut outcome = edge.step(peer, store).await;
    for _ in 1..RETRY_ATTEMPTS {
        let unanswered = match &outcome {
            Err(
                StepError::Pull(RemoteError::Exchange(error))
                | StepError::Push(RemoteError::Exchange(error)),
            ) => error.is_unanswered(),
            _ => false,
        };
        if !unanswered {
            break;
        }
        tokio::time::sleep(pause).await;
        pause = pause.saturating_mul(2);
        outcome = edge.step(peer, store).await;
    }
    outcome
}

/// An exchange that records the length of the one request it is handed and
/// answers none, so a push's encoded size is measured without a network.
#[derive(Debug, Default)]
struct RecordingExchange {
    sent: Cell<Option<usize>>,
}

impl Exchange for RecordingExchange {
    type Error = HttpError;

    async fn exchange(&self, _route: Route, request: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        self.sent.set(Some(request.len()));
        Err(refusal(400, "recorded, never sent".to_owned()))
    }
}

/// An exchange that answers every request with the failure `answer` builds,
/// and counts the requests.
#[derive(Debug)]
struct ScriptedExchange {
    answer: fn() -> HttpError,
    calls: Cell<usize>,
}

impl ScriptedExchange {
    fn answering(answer: fn() -> HttpError) -> Self {
        Self {
            answer,
            calls: Cell::new(0),
        }
    }
}

impl Exchange for ScriptedExchange {
    type Error = HttpError;

    async fn exchange(&self, _route: Route, _request: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        self.calls.set(self.calls.get() + 1);
        Err((self.answer)())
    }
}

/// The scenario, measured from the events it builds: each event within the
/// per-event ceiling, each group within the per-call ceiling, both together
/// over it, the push within the wide mount's event count, and half the push
/// exactly one group, which is what the edge's halving offers next.
///
/// And the negative control: `/sync/`'s 128 events a push at 1,024 tags an
/// event cannot reach the ceiling, which is why the wide mount exists.
///
/// Rejects a scenario sized by typed-in numbers that drift from the adapter
/// (a group over the ceiling alone is refused with nothing to split), and one
/// whose push the wide mount refuses as `BatchTooLarge` before any ingest.
#[test]
fn the_split_scenario_crosses_the_ceiling_only_as_a_whole_push() {
    let per_event_cap = <Declared as Fixture>::MAX_TAGS_PER_EVENT
        .expect("the harness fixture declares the adapter's tags-per-event ceiling");
    let groups = split_scenario();
    let tags_in = |group: &[Event]| -> usize { group.iter().map(|event| event.tags().len()).sum() };

    for group in &groups {
        assert_eq!(group.len(), GROUP_EVENTS);
        assert!(
            group
                .iter()
                .all(|event| event.tags().len() <= per_event_cap),
            "every event is within the per-event ceiling"
        );
        assert!(tags_in(group) <= CEILING, "a group alone lands");
    }
    let total: usize = groups.iter().map(|group| tags_in(group)).sum();
    assert_eq!(total, TOTAL_TAGS);
    assert!(total > CEILING, "the whole push is over the ceiling");
    assert!(TOTAL_EVENTS <= wide_limits().max_batch_events);
    assert_eq!(
        TOTAL_EVENTS / 2,
        GROUP_EVENTS,
        "the halved push is one group"
    );
    assert!(
        PeerLimits::minimum().max_batch_events * PER_EVENT < CEILING,
        "no push /sync/ admits can meet the ceiling"
    );
}

/// The refused push's envelope, as `RemoteSyncPeer` encodes it, is over
/// `/sync/`'s body cap and within the wide mount's: so through the wide mount
/// only the ingest can refuse it.
///
/// Rejects a wide mount whose body cap still answers first, a deployed 413
/// that would read as the right class for the wrong reason; and an encoding
/// that grows the push past the cap, caught here by name rather than on the
/// deployed leg.
#[tokio::test]
async fn the_split_push_fits_the_wide_mounts_body_cap() {
    let local = origin().await;
    let batch = PushBatch::new(written(&local).await);
    let peer = RemoteSyncPeer::new(RecordingExchange::default(), wide_limits());

    let refused = peer
        .push(&batch)
        .await
        .expect_err("the recorder answers nothing");

    assert!(
        matches!(
            refused,
            RemoteError::Exchange(HttpError::Refused { status: 400, .. })
        ),
        "{refused:?}"
    );
    let sent = peer
        .exchange()
        .sent
        .get()
        .expect("the push reached the exchange");
    let narrow = max_request_bytes(&PeerLimits::minimum());
    let wide = max_request_bytes(&wide_limits());
    assert!(
        sent > narrow,
        "{sent} bytes would split on /sync/'s {narrow}-byte cap"
    );
    assert!(
        sent <= wide,
        "{sent} bytes are over the wide mount's {wide}-byte cap"
    );
}

/// A step the object refused is returned at once, even a 500 that reads as
/// `Transient`; a platform miss is tried again, up to [`RETRY_ATTEMPTS`].
///
/// Rejects a helper that retries by class, as `retrying` and `Edge::run` do,
/// which would hide a deployed object reset on a push under the ceiling
/// (ADR-0100's falsifier); and one that never retries, which would fail the
/// deployed leg on a Worker still landing. On paused time, so the pauses read
/// no wall clock.
#[tokio::test(start_paused = true)]
async fn step_past_platform_misses_never_retries_a_refusal() {
    let local = MemoryIngestFixture::new().connect().await;
    // `platform-miss.mjs`'s `OBJECT_RESETS["cpu-limit"]`, verbatim.
    let reset = RemoteSyncPeer::new(
        ScriptedExchange::answering(|| {
            refusal(
                500,
                "Durable Object exceeded its CPU time limit and was reset.".to_owned(),
            )
        }),
        wide_limits(),
    );

    let outcome = step_past_platform_misses(&mut Edge::new(), &reset, &local).await;

    assert!(
        matches!(
            outcome,
            Err(StepError::Pull(RemoteError::Exchange(HttpError::Refused {
                status: 500,
                class: FailureClass::Transient,
                ..
            })))
        ),
        "{outcome:?}"
    );
    assert_eq!(
        reset.exchange().calls.get(),
        1,
        "an object reset is never retried"
    );

    let miss = RemoteSyncPeer::new(
        ScriptedExchange::answering(|| refusal(500, "Worker not found.".to_owned())),
        wide_limits(),
    );

    let outcome = step_past_platform_misses(&mut Edge::new(), &miss, &local).await;

    assert!(
        matches!(
            outcome,
            Err(StepError::Pull(RemoteError::Exchange(
                HttpError::PlatformMiss { .. }
            )))
        ),
        "{outcome:?}"
    );
    assert_eq!(miss.exchange().calls.get(), RETRY_ATTEMPTS, "bounded");
}

/// VT-36 through a real push. The edge offers the object both groups, 264,192
/// tags, through the wide mount; the object's ingest refuses the call as over
/// its 262,144 tags (413, `Split`) and writes nothing; the edge halves, pushes
/// one group alone, then the other, and syncs. The object then holds both
/// groups, each whole, tags included.
///
/// Rejects an object whose ingest does not hold the whole call to the ceiling
/// (the first push lands), one that writes part of a refused call, a runner
/// that parks or stops on a halvable split, and one that halves by cutting a
/// group. The refusal's text is checked, so a 413 from the body cap, the same
/// class for another reason, fails here and says so.
#[tokio::test]
#[ignore = "needs a running harness Worker (WORKERD_HARNESS_URL, HARNESS_TOKEN, WORKERD_HARNESS_RUN); the `workerd` CI job runs these with --ignored"]
async fn the_runner_splits_a_push_on_tags_per_batch_through_the_object() {
    let harness = harness();
    let peer = harness.peer_wide("tags-split");
    reset(&peer).await;
    let local = origin().await;
    let written = written(&local).await;
    let shape: Vec<usize> = written.iter().map(|group| group.events.len()).collect();
    assert_eq!(shape, [GROUP_EVENTS, GROUP_EVENTS], "two writes, whole");

    let mut edge = Edge::new();
    let refused = step_past_platform_misses(&mut edge, &peer, &local).await;

    let Err(StepError::Push(RemoteError::Exchange(HttpError::Refused {
        status: 413,
        class: FailureClass::Split,
        body,
    }))) = &refused
    else {
        panic!("expected the object to refuse the whole push as 413 Split, got {refused:?}");
    };
    assert!(
        body.contains(&format!("tags per batch limit: {TOTAL_TAGS}")),
        "the split must come from the ingest's tags-per-call ceiling, not the body cap: {body}"
    );
    assert_eq!(edge.status().state, EdgeState::Running);
    assert_eq!(
        edge.status().last_failure,
        Some(Failure::new(Phase::Push, FailureClass::Split))
    );
    assert_eq!(drain(&peer).await, [], "the refused push wrote nothing");

    let halved = step_past_platform_misses(&mut edge, &peer, &local)
        .await
        .expect("the halved push lands");
    assert_eq!(halved.pushed, GROUP_EVENTS, "one group, alone");

    for _ in 0..MAX_STEPS {
        if edge.status().is_synced() {
            break;
        }
        step_past_platform_misses(&mut edge, &peer, &local)
            .await
            .expect("every step after the split lands");
    }

    assert!(edge.status().is_synced(), "{:?}", edge.status());
    assert_eq!(
        drain(&peer).await,
        written,
        "the object holds both writes, each whole"
    );
    reset(&peer).await;
}
