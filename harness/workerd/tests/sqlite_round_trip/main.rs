//! A native SQLite store replicating through the Durable Object over real HTTP
//! (#130): the object's `/sync/pull` and `/sync/push`, reached through the
//! harness Worker's router from this process, with [`http::HttpExchange`] as
//! the client and the runner's [`Edge`] driving it.
//!
//! # Live, and gated
//!
//! The round trips need a running harness Worker and are `#[ignore]`d with the
//! reason, so a machine without one keeps a green `cargo xtask ci`. The
//! `workerd` CI job runs them with `-- --ignored`, twice: against
//! `wrangler dev` (local `workerd`) and against the Worker it deploys. They
//! read three variables, and a run reached with `--ignored` and without them
//! fails rather than passing over nothing:
//!
//! * `WORKERD_HARNESS_URL`: the Worker, `http://127.0.0.1:8787` or the
//!   deployed `https://...workers.dev`;
//! * `HARNESS_TOKEN`: the bearer token the Worker's router requires;
//! * `WORKERD_HARNESS_RUN`: a prefix for the object names, so two runs never
//!   share an object.
//!
//! Each test empties its object first and last (`/sync-reset`), so a deployed
//! object holds nothing once the test has answered.
//!
//! The two tests at the bottom are not live: they pin what the client does
//! when no answer comes, and run in the gate.

#![cfg(not(target_arch = "wasm32"))]

mod http;

use core::num::NonZeroUsize;
use core::time::Duration;

use happenstance_cloudflare::happenstance_sync::runner::{
    Backoff, Edge, EdgeState, Sleep, StepError, StopCause,
};
use happenstance_cloudflare::happenstance_sync::wire::ResumeToken;
use happenstance_cloudflare::happenstance_sync::{
    Classify, EventGroup, FailureClass, IngestGroup, IngestStore, PeerLimits, PushBatch,
    RemoteError, RemoteSyncPeer, ReplicatedEvent, SyncPeer, Waiting, WriteGroupStore,
};
use happenstance_core::{
    AppendCondition, Event, EventId, EventStore, Query, QueryItem, ReadOptions, RecordedAt,
    SequencePosition, SequencedEvent, StoreId, Tag, Tags, collect,
};
use happenstance_sqlite::event_store::SqliteEventStore;
use hyper::Method;

use crate::http::{HttpError, HttpExchange, RETRY_ATTEMPTS, bounded, refusal, retrying};

/// The most pulls a drain makes before it decides the object will never say
/// it is done: far above what these few groups need (H-13).
const MAX_PULLS: usize = 64;

/// The Worker under test, from the environment.
struct Harness {
    url: String,
    token: String,
    run: String,
}

/// The three variables, or a panic naming the missing one: a test reached
/// with `--ignored` and nothing to run against is a broken environment, never
/// a pass.
fn harness() -> Harness {
    let read = |name: &str| {
        std::env::var(name)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| {
                panic!(
                    "a broken test environment, not a failing peer: `{name}` is not set. \
                     These tests are #[ignore]d so a machine without a harness Worker \
                     stays green; they are reached with -- --ignored."
                )
            })
    };
    Harness {
        url: read("WORKERD_HARNESS_URL"),
        token: read("HARNESS_TOKEN"),
        run: read("WORKERD_HARNESS_RUN"),
    }
}

impl Harness {
    /// The object called `<run>-<name>`, as a remote peer declaring
    /// [`PeerLimits::minimum`], which is what the object's routes declare.
    fn peer(&self, name: &str) -> RemoteSyncPeer<HttpExchange> {
        self.peer_with_token(name, &self.token)
    }

    /// [`peer`](Self::peer), presenting `token` instead.
    fn peer_with_token(&self, name: &str, token: &str) -> RemoteSyncPeer<HttpExchange> {
        self.peer_declaring(name, token, PeerLimits::minimum())
    }

    /// The object called `<run>-<name>`, declaring `limits` to the runner.
    /// Tighter than the object's own is still true of it: a batch these
    /// admit, the object admits.
    fn peer_declaring(
        &self,
        name: &str,
        token: &str,
        limits: PeerLimits,
    ) -> RemoteSyncPeer<HttpExchange> {
        let exchange =
            HttpExchange::new(&self.url, &format!("{}-{name}", self.run), token, &limits)
                .expect("the platform's CA store reads");
        RemoteSyncPeer::new(exchange, limits)
    }
}

/// Empties the object behind `peer`: the next sync request migrates a fresh
/// log under a fresh identity. Retried past a platform miss, and idempotent,
/// so a retry after a cut request empties the same object again.
async fn reset(peer: &RemoteSyncPeer<HttpExchange>) {
    let answer = retrying(async || {
        peer.exchange()
            .call(Method::GET, "/sync-reset", Vec::new())
            .await
    })
    .await
    .expect("the object resets");
    assert_eq!(answer, b"reset");
}

/// The runner's clock: the backoff's delays, on tokio's timer.
struct TokioSleep;

impl Sleep for TokioSleep {
    async fn sleep(&self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}

/// A backoff that rides out a deployed edge's blips without hiding a refusal.
fn backoff() -> Backoff {
    Backoff::new(Duration::from_millis(200), Duration::from_secs(5), 130)
        .expect("the base is below the cap")
}

/// A fresh in-memory SQLite store.
fn sqlite() -> SqliteEventStore {
    SqliteEventStore::open_in_memory().expect("an in-memory SQLite store opens")
}

/// Every event `store` holds, in log order.
async fn log<S: EventStore>(store: &S) -> Vec<SequencedEvent> {
    collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .expect("the log reads")
}

/// What a replica must carry of an event, byte for byte: its identity, its
/// recorded time and the event, payload and metadata bytes included. The
/// position is the holding store's own and is left out.
fn carried(events: &[SequencedEvent]) -> Vec<(EventId, RecordedAt, &Event)> {
    events
        .iter()
        .map(|event| (event.id, event.recorded_at, &event.event))
        .collect()
}

/// An event no text codec would leave alone: `data` and `metadata` verbatim,
/// with two tags.
fn awkward(event_type: &'static str, data: Vec<u8>, metadata: Vec<u8>) -> Event {
    let tags: Tags = ["course:awkward", "byte:\u{1F980}"]
        .into_iter()
        .map(|tag| Tag::new(tag.to_owned()).expect("a valid tag"))
        .collect();
    Event::new(event_type, data)
        .expect("a static event type is valid")
        .with_tags(tags)
        .with_metadata(metadata)
}

/// Pulls `peer` until it says nothing is waiting, and returns every group.
async fn drain(peer: &RemoteSyncPeer<HttpExchange>) -> Vec<EventGroup> {
    let mut groups = Vec::new();
    let mut from: Option<ResumeToken> = None;
    for _ in 0..MAX_PULLS {
        let pulled = retrying(async || peer.pull(from.as_ref()).await)
            .await
            .expect("a pull");
        groups.extend(pulled.batch.groups);
        if pulled.waiting == Waiting::Nothing {
            return groups;
        }
        from = Some(pulled.resume);
    }
    panic!("the object never said nothing was waiting in {MAX_PULLS} pulls");
}

/// SQLite A appends, an edge pushes it to the object, and SQLite B pulls it
/// from there: B holds A's events with the same identity, recorded time and
/// bytes. Then the other way, B to A. Both directions cross real HTTP twice.
///
/// Rejects a transport that reads a body as text (the invalid UTF-8 and the
/// NUL would not survive), and an object that re-encodes a payload.
#[tokio::test]
#[ignore = "needs a running harness Worker (WORKERD_HARNESS_URL, HARNESS_TOKEN, WORKERD_HARNESS_RUN); the `workerd` CI job runs these with --ignored"]
async fn sqlite_round_trips_through_the_object_byte_identical() {
    let harness = harness();
    let peer = harness.peer("round-trip");
    reset(&peer).await;
    let (a, b) = (sqlite(), sqlite());

    let every_byte: Vec<u8> = (0..=u8::MAX).collect();
    a.append(
        &[
            awkward(
                "Opaque",
                vec![0xC3, 0x28, 0x00, 0xFF],
                vec![0xFE, 0xFF, 0x00],
            ),
            awkward("Opaque", every_byte, Vec::new()),
            awkward("Opaque", Vec::new(), vec![0x00]),
        ],
        None,
    )
    .await
    .expect("a's append");
    a.append(&[awkward("Alone", b"{\"k\":1}".to_vec(), Vec::new())], None)
        .await
        .expect("a's second append");

    let mut edge_a = Edge::new();
    edge_a
        .run(&peer, &a, &mut backoff(), &TokioSleep)
        .await
        .expect("a syncs with the object");
    let mut edge_b = Edge::new();
    edge_b
        .run(&peer, &b, &mut backoff(), &TokioSleep)
        .await
        .expect("b syncs with the object");

    assert_eq!(carried(&log(&b).await), carried(&log(&a).await), "a to b");

    b.append(&[awkward("FromB", vec![0x80, 0x81], vec![0xFF])], None)
        .await
        .expect("b's append");
    edge_b
        .run(&peer, &b, &mut backoff(), &TokioSleep)
        .await
        .expect("b pushes");
    edge_a
        .run(&peer, &a, &mut backoff(), &TokioSleep)
        .await
        .expect("a pulls");

    assert_eq!(carried(&log(&a).await), carried(&log(&b).await), "b to a");
    assert_eq!(log(&a).await.len(), 5);
    assert!(edge_a.status().is_synced(), "{:?}", edge_a.status());
    reset(&peer).await;
}

/// A batch pushed twice lands once, and the object's log is what it was; a
/// batch pulled from it and ingested twice lands once, and SQLite's log is
/// what it was.
///
/// Rejects an object that appends a re-delivery instead of skipping it, and
/// an ack that counts a skipped event as appended.
#[tokio::test]
#[ignore = "needs a running harness Worker (WORKERD_HARNESS_URL, HARNESS_TOKEN, WORKERD_HARNESS_RUN); the `workerd` CI job runs these with --ignored"]
async fn replaying_a_batch_twice_changes_nothing() {
    let harness = harness();
    let peer = harness.peer("replay");
    reset(&peer).await;
    let origin = StoreId::from_bytes([0x13; 16]);
    let events = (1..=3)
        .map(|position| {
            ReplicatedEvent::new(
                EventId::new(
                    origin,
                    SequencePosition::new(position).expect("test positions are not zero"),
                ),
                RecordedAt::from_millis(1_800_000_000_000),
                awkward("Replayed", vec![0xFF, 0x00, 0xFE], Vec::new()),
            )
        })
        .collect();
    let batch = PushBatch::new(vec![EventGroup::new(None, events)]);

    // Retried past a platform miss. A miss raised before the object runs
    // leaves nothing behind; one that cut a push after it landed would turn
    // the first ack into the replay's, and this test red, which is a loud
    // failure rather than a silent pass.
    let first = retrying(async || peer.push(&batch).await)
        .await
        .expect("the first push");
    let held = drain(&peer).await;
    let replay = retrying(async || peer.push(&batch).await)
        .await
        .expect("the replay");

    assert_eq!((first.appended, first.skipped), (3, 0));
    assert_eq!((replay.appended, replay.skipped), (0, 3));
    assert_eq!(drain(&peer).await, held, "the object's log is unchanged");

    let local = sqlite();
    let pulled: Vec<IngestGroup<'_>> = held
        .iter()
        .map(|group| IngestGroup::new(&group.events, &[]))
        .collect();
    let landed = local.ingest(&pulled).await.expect("the first ingest");
    let before = log(&local).await;
    let again = local.ingest(&pulled).await.expect("the second ingest");

    assert_eq!((landed.appended, landed.skipped), (3, 0));
    assert_eq!((again.appended, again.skipped), (0, 3));
    assert_eq!(log(&local).await, before, "SQLite's log is unchanged");
    reset(&peer).await;
}

/// A reset object comes back under a new identity, and the edge that synced
/// with the old one stops at the next answer, before acting on it, with the
/// local log untouched; and does no I/O after.
///
/// Rejects a runner that treats the empty object as the same peer, and so
/// re-pushes nothing (its watermark says the object has it all) while the
/// object holds none of it.
#[tokio::test]
#[ignore = "needs a running harness Worker (WORKERD_HARNESS_URL, HARNESS_TOKEN, WORKERD_HARNESS_RUN); the `workerd` CI job runs these with --ignored"]
async fn a_reset_object_stops_the_edge_and_leaves_the_log_alone() {
    let harness = harness();
    let peer = harness.peer("reset");
    reset(&peer).await;
    let local = sqlite();
    local
        .append(&[awkward("BeforeReset", vec![0x01], Vec::new())], None)
        .await
        .expect("the append");
    let mut edge = Edge::new();
    edge.run(&peer, &local, &mut backoff(), &TokioSleep)
        .await
        .expect("the first sync");
    let known = edge.status().responder.expect("the object named itself");

    reset(&peer).await;
    let before = log(&local).await;
    let stopped = edge.step(&peer, &local).await;

    let Err(StepError::Incarnation(change)) = stopped else {
        panic!("expected the incarnation check to stop the edge, got {stopped:?}");
    };
    assert_eq!(change.known, known);
    assert_ne!(change.answered, known, "the reset object is a new store");
    assert!(matches!(
        edge.status().state,
        EdgeState::Stopped(StopCause::Incarnation(_))
    ));
    assert_eq!(log(&local).await, before, "nothing was ingested");
    assert!(matches!(
        edge.step(&peer, &local).await,
        Err(StepError::Stopped(_))
    ));
    reset(&peer).await;
}

/// The router's refusal of a token is a status, and a status is a class:
/// 403 is `Stop`, so an edge with the wrong credentials halts rather than
/// retrying forever.
#[tokio::test]
#[ignore = "needs a running harness Worker (WORKERD_HARNESS_URL, HARNESS_TOKEN, WORKERD_HARNESS_RUN); the `workerd` CI job runs these with --ignored"]
async fn a_wrong_token_is_stop() {
    let harness = harness();
    let peer = harness.peer_with_token("wrong-token", "not-the-token");

    let refused = peer.pull(None).await.expect_err("the router refuses it");

    assert!(
        matches!(
            refused,
            RemoteError::Exchange(HttpError::Refused { status: 403, .. })
        ),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Stop);
}

/// The runner pushes a SQLite log to the object in the writes that made it
/// (SY-30, #143): appends of 2, 3 and 1 events and a foreign group of 2
/// ingested under a guard, with the object declared at 4 events a batch, so
/// the edge needs several pushes. Pulled back over HTTP, the object holds the
/// four groups, each whole, the ingested one with its guard.
///
/// Rejects a runner that cuts its push at the batch limit, which tears the
/// append of 3 across two pushes and merges what it cuts, and a far side that
/// drops a relayed group's guard on the way in or the way out.
#[tokio::test]
#[ignore = "needs a running harness Worker (WORKERD_HARNESS_URL, HARNESS_TOKEN, WORKERD_HARNESS_RUN); the `workerd` CI job runs these with --ignored"]
async fn the_runner_keeps_write_groups_through_the_object() {
    let harness = harness();
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = 4;
    let peer = harness.peer_declaring("groups", &harness.token, limits);
    reset(&peer).await;
    let local = sqlite();
    let opaque = |n: u8| awkward("Written", vec![n, 0x00, 0xFF], Vec::new());
    local
        .append(&[opaque(1), opaque(2)], None)
        .await
        .expect("an append");
    local
        .append(&[opaque(3), opaque(4), opaque(5)], None)
        .await
        .expect("an append");
    let origin = StoreId::from_bytes([0x30; 16]);
    let relayed: Vec<ReplicatedEvent> = [7, 9]
        .into_iter()
        .map(|position| {
            ReplicatedEvent::new(
                EventId::new(
                    origin,
                    SequencePosition::new(position).expect("test positions are not zero"),
                ),
                RecordedAt::from_millis(1_800_000_000_000),
                awkward("Relayed", vec![0xFE, 0x01], vec![0x00]),
            )
        })
        .collect();
    let guard = AppendCondition::new(Query::from_item(
        QueryItem::of_types(["Contested"]).expect("a valid item"),
    ));
    local
        .ingest(&[IngestGroup::new(&relayed, &[]).with_guard(Some(&guard))])
        .await
        .expect("the ingest");
    local.append(&[opaque(6)], None).await.expect("an append");

    let mut edge = Edge::new();
    edge.run(&peer, &local, &mut backoff(), &TokioSleep)
        .await
        .expect("the edge syncs with the object");
    let written: Vec<EventGroup> = local
        .read_groups(None, NonZeroUsize::new(64).expect("non-zero"))
        .await
        .expect("the local groups read")
        .groups
        .into_iter()
        .map(EventGroup::from)
        .collect();
    let held = drain(&peer).await;

    let shape = |groups: &[EventGroup]| -> Vec<(usize, bool)> {
        groups
            .iter()
            .map(|group| (group.events.len(), group.guard.is_some()))
            .collect()
    };
    assert_eq!(
        shape(&written),
        [(2, false), (3, false), (2, true), (1, false)]
    );
    assert_eq!(
        held, written,
        "the object holds the four writes, each whole"
    );
    assert!(edge.status().is_synced(), "{:?}", edge.status());
    reset(&peer).await;
}

/// No answer within the bound is a timeout, and a timeout is `Transient`.
///
/// Rejects an unbounded round trip (this test would never return) and a
/// timeout reported as `Stop`, which would halt an edge on one slow answer.
/// On paused time, so no wall clock is read.
#[tokio::test(start_paused = true)]
async fn a_round_trip_that_outlives_its_bound_is_a_transient_timeout() {
    let limit = Duration::from_millis(1);

    let outcome: Result<(), HttpError> = bounded(core::future::pending(), limit).await;

    let error = outcome.expect_err("the bound fires");
    assert!(
        matches!(error, HttpError::Timeout { after } if after == limit),
        "{error:?}"
    );
    assert_eq!(error.class(), FailureClass::Transient);
}

/// A refused connection is a transport failure, and a transport failure is
/// `Transient` (#130's default): the object may be restarting.
///
/// The port is one this process bound and released, so nothing listens on it.
#[tokio::test]
async fn a_refused_connection_is_transient() {
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .and_then(|listener| listener.local_addr())
        .expect("a loopback port binds")
        .port();
    let exchange = HttpExchange::new(
        &format!("http://127.0.0.1:{port}"),
        "nobody",
        "token",
        &PeerLimits::minimum(),
    )
    .expect("the platform's CA store reads");
    let peer = RemoteSyncPeer::new(exchange, PeerLimits::minimum());

    let refused = peer.pull(None).await.expect_err("nothing listens");

    assert!(
        matches!(refused, RemoteError::Exchange(HttpError::Transport(_))),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Transient);
}

/// Each answer `platform-miss.mjs` records Cloudflare giving while a fresh
/// Worker lands is a platform miss, and a platform miss is `Transient`.
///
/// Rejects a client that reads the edge's 404 page through `class_for` alone,
/// which would stop the deployed leg's edge on a propagation delay.
#[test]
fn a_platform_miss_reads_as_transient() {
    let misses = [
        (500, "Worker not found.\n"),
        (
            404,
            "<html><body><h1>There is nothing here yet</h1></body></html>",
        ),
        (500, " Durable Object reset because its code was updated. "),
        (
            500,
            "<html><span class=\"cf-error-code\">1104</span></html>",
        ),
    ];

    for (status, body) in misses {
        let error = refusal(status, body.to_owned());
        assert!(matches!(error, HttpError::PlatformMiss { .. }), "{error:?}");
        assert_eq!(error.class(), FailureClass::Transient, "{error:?}");
    }
}

/// The object's own 404s are plain text, never one of the platform's pages,
/// and stay `Stop`; so does a 404 that only mentions the page's words.
///
/// Rejects a recogniser that matches loosely (any 404, or the heading's text
/// without its element), which would retry a missing route as if the Worker
/// were still landing.
#[test]
fn the_objects_own_404_stays_stop() {
    for body in [
        "no sync route /sync/pul",
        "no such rule",
        "expected /do/<object>/<route>",
        "There is nothing here yet",
    ] {
        let error = refusal(404, body.to_owned());
        assert!(
            matches!(error, HttpError::Refused { status: 404, .. }),
            "{error:?}"
        );
        assert_eq!(error.class(), FailureClass::Stop, "{error:?}");
    }
}

/// A `Transient` failure is tried again after a pause, up to
/// [`RETRY_ATTEMPTS`]; the first answer that is not `Transient` ends it.
///
/// Rejects a retry that never retries, one that retries a `Stop` (a refused
/// token would be sent three times), and one without a bound. On paused time,
/// so the pauses read no wall clock.
#[tokio::test(start_paused = true)]
async fn retrying_retries_a_transient_failure_and_nothing_else() {
    let attempts = core::cell::Cell::new(0_usize);
    let transient = || HttpError::Timeout {
        after: Duration::from_millis(1),
    };

    let recovered = retrying(async || {
        attempts.set(attempts.get() + 1);
        if attempts.get() < RETRY_ATTEMPTS {
            Err(transient())
        } else {
            Ok("answered")
        }
    })
    .await;
    assert_eq!(recovered.expect("the last attempt answers"), "answered");
    assert_eq!(attempts.get(), RETRY_ATTEMPTS);

    attempts.set(0);
    let gave_up: Result<(), HttpError> = retrying(async || {
        attempts.set(attempts.get() + 1);
        Err(transient())
    })
    .await;
    assert!(
        matches!(gave_up, Err(HttpError::Timeout { .. })),
        "{gave_up:?}"
    );
    assert_eq!(attempts.get(), RETRY_ATTEMPTS, "bounded");

    attempts.set(0);
    let stopped: Result<(), HttpError> = retrying(async || {
        attempts.set(attempts.get() + 1);
        Err(refusal(403, "forbidden".to_owned()))
    })
    .await;
    assert_eq!(stopped.expect_err("a refusal").class(), FailureClass::Stop);
    assert_eq!(attempts.get(), 1, "a Stop is not retried");
}
