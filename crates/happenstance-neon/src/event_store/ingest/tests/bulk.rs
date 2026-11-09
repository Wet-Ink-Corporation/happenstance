//! SY-14 on the peer it was deferred for: Kestrel Rotor's 1,840 events through
//! `NeonSyncPeer`, one request per push however many the far side holds
//! (#139).
//!
//! ADR-0027 reworded SY-14's bound to one ingest round trip per pulled batch
//! and held the marker for this measurement: 1,840 events over one-shot HTTP
//! inside a 34-minute window. The sync testkit's
//! `bulk_ingest_is_idempotent_in_bounded_round_trips` holds a store to one
//! `ingest` call per batch. What no port method sees is how many requests an
//! adapter spends inside one call, and CF-33 leaves that count to the adapter,
//! so it is pinned here twice:
//!
//! * **over the scripted transport**, which needs no endpoint and so runs in
//!   the gate. A round-trip count is a property of the requests the peer
//!   builds, not of the server, so this is the proof of the bound;
//! * **against the live endpoint**, `#[ignore]`d as every live test in this
//!   crate is, and run by CI's `live-neon` job through its library trio. It
//!   adds what a script cannot: the wall clock, the server's own skip of a
//!   held event, and the log the pushes leave. It prints its numbers on lines
//!   that start `SY14-MEASURE`, so they can be read back from the job's log.
//!
//! Both drive the push the way the runner reads its log: `read_groups` from
//! the near store at the peer's declared `max_batch_events`, whole write
//! groups, one push a page. Rotor's log is 46 appends of 40 events, the
//! largest indivisible unit Kestrel Cold Chain names being under forty, so a
//! page carries three groups, 120 events, and the log is 16 pushes.

use core::cell::Cell;
use core::num::NonZeroUsize;
use std::rc::Rc;
use std::time::{Duration, Instant};

use happenstance_core::{Event, EventStore};
use happenstance_sync::{EventGroup, PeerLimits, PushBatch, SyncPeer, Waiting, WriteGroupStore};
use happenstance_sync_testkit::{IngestFixture, MemoryIngestFixture};

use super::endpoint::{LiveTransport, fresh_schema};
use super::peer_requests::{identity, set};
use super::{OWN, Scripted, rows};
use crate::event_store::{NeonEventStore, NeonSyncPeer};
use crate::transport::{HttpResponse, SqlRequest, SqlTransport};

/// Kestrel Rotor's bulk ingest: 1,840 events.
const ROTOR_EVENTS: usize = 1_840;

/// Events in each of the near store's appends: one indivisible unit of work,
/// under Kestrel Cold Chain's largest of forty.
const GROUP_LEN: usize = 40;

/// The window Rotor's ingest has to finish in: 34 minutes (ADR-0027 §6).
const ROTOR_WINDOW: Duration = Duration::from_mins(34);

/// The statements one push sends: the ingest's three, then the watermark and
/// the identity. Fixed, whatever the push carries.
const STATEMENTS_PER_PUSH: usize = 5;

/// Pages past which a push loop is a near store that never ends, not a log.
const MAX_PAGES: usize = ROTOR_EVENTS;

/// The pushes Rotor's log takes at `max_batch_events`: whole groups to a page,
/// so the page holds as many groups as fit, and the last page what is left.
fn expected_pushes(max_batch_events: usize) -> usize {
    let groups = ROTOR_EVENTS.div_ceil(GROUP_LEN);
    groups.div_ceil((max_batch_events / GROUP_LEN).max(1))
}

/// A near store holding Rotor's log, written as the origin wrote it: one
/// append per unit of work, each a write group of its own.
async fn rotor_log() -> <MemoryIngestFixture as IngestFixture>::Store {
    let near = MemoryIngestFixture::new().connect().await;
    let events: Vec<Event> = (0..ROTOR_EVENTS)
        .map(|index| {
            Event::new("RotorReading", format!("rotor-{index:04}").into_bytes())
                .expect("a valid type")
        })
        .collect();
    for unit in events.chunks(GROUP_LEN) {
        EventStore::append(&near, unit, None)
            .await
            .expect("the near store's own append lands");
    }
    near
}

/// What pushing a near log through the peer took.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Pushed {
    /// Pushes made, one a page.
    pushes: usize,
    /// Requests the transport carried for them.
    requests: usize,
    /// Events the far side appended, by its acks.
    appended: u64,
    /// Events the far side already held, by its acks.
    skipped: u64,
}

/// Pushes `near`'s whole log through `peer`, a page of whole groups at the
/// peer's declared `max_batch_events` to each push, and asserts that each
/// push was exactly one request. `requests` reads the transport's count.
async fn push_the_log<N, T>(
    near: &N,
    peer: &NeonSyncPeer<T>,
    requests: impl Fn() -> usize,
) -> Pushed
where
    N: WriteGroupStore,
    N::Error: core::fmt::Debug,
    T: SqlTransport,
    T::Error: core::fmt::Debug,
{
    let budget =
        NonZeroUsize::new(peer.limits().max_batch_events).expect("a peer declares room for one");
    let mut pushed = Pushed::default();
    let mut after = None;
    for _ in 0..MAX_PAGES {
        let page = near
            .read_groups(after, budget)
            .await
            .expect("the near store's groups read");
        let Some(last) = page.groups.last() else {
            return pushed;
        };
        after = Some(last.first());
        let waiting = page.waiting;
        let batch = PushBatch::new(page.groups.into_iter().map(EventGroup::from).collect());

        let before = requests();
        let ack = peer
            .push(&batch)
            .await
            .expect("a push inside the declared limits lands");
        let spent = requests() - before;

        assert_eq!(spent, 1, "push {} cost {spent} requests", pushed.pushes);
        pushed.pushes += 1;
        pushed.requests += spent;
        pushed.appended += ack.appended;
        pushed.skipped += ack.skipped;
        if waiting == Waiting::Nothing {
            return pushed;
        }
    }
    panic!("the near store's log did not end within {MAX_PAGES} pages");
}

/// Queues one landed push's five answers: its counts, the refusal's `DO`, the
/// stamp's `UPDATE`, the watermark and the identity.
fn answer_push(transport: &Scripted, appended: usize, skipped: usize) {
    transport.answer_batch(&[
        set(&[serde_json::json!({
            "appended": appended.to_string(), "skipped": skipped.to_string(),
            "compensated": "0", "unheld": "",
        })]),
        serde_json::json!({ "command": "DO", "rowCount": null, "fields": [], "rows": [] }),
        serde_json::json!({ "command": "UPDATE", "rowCount": 1, "fields": [], "rows": [] }),
        set(&[]),
        identity(OWN),
    ]);
}

/// The length of each page of Rotor's log at `max_batch_events`, in order.
fn page_lengths(max_batch_events: usize) -> Vec<usize> {
    // A page carries one group at least: a group is never cut (SY-30).
    let per_page = (max_batch_events / GROUP_LEN).max(1) * GROUP_LEN;
    let mut left = ROTOR_EVENTS;
    let mut lengths = Vec::new();
    while left > 0 {
        let length = left.min(per_page);
        lengths.push(length);
        left -= length;
    }
    lengths
}

/// SY-14's bound, counted with no endpoint: Rotor's 1,840 events at the
/// minimum peer's 128 a push are 16 pushes and 16 requests, each the same
/// five statements carrying the same number of parameters, so the request a
/// push sends does not grow with its events. The replay, every event already
/// held at the far side, is 16 requests again: a held event costs no round
/// trip.
///
/// **Named wrong peers:** one that sends an `append` per event, 1,840
/// requests, the shape SY-14's first rejection names, fatal on this
/// transport; and `StoreSyncPeer` over this store, three requests a push.
#[tokio::test]
async fn rotors_ingest_is_one_request_per_push_however_much_is_held() {
    let limits = PeerLimits::minimum();
    let pages = page_lengths(limits.max_batch_events);
    let transport = Scripted::default();
    for length in &pages {
        answer_push(&transport, *length, 0);
    }
    for length in &pages {
        answer_push(&transport, 0, *length);
    }
    let near = rotor_log().await;
    let peer = NeonSyncPeer::new(transport.store());

    let first = push_the_log(&near, &peer, || transport.round_trips.get()).await;
    let replay = push_the_log(&near, &peer, || transport.round_trips.get()).await;

    let pushes = expected_pushes(limits.max_batch_events);
    assert_eq!(pushes, 16);
    assert_eq!(pages.len(), pushes);
    assert_eq!((first.pushes, first.requests), (pushes, pushes));
    assert_eq!((replay.pushes, replay.requests), (pushes, pushes));
    // The counts are the script's own answers: these check only that the
    // peer sums its acks, not anything a server decided.
    assert_eq!(first.appended, 1_840);
    assert_eq!(replay.skipped, 1_840);

    let sent = transport.sent.take();
    assert_eq!(sent.len(), 2 * pushes);
    let shape = |request: &SqlRequest| {
        let params: Vec<usize> = request
            .statements
            .iter()
            .map(|statement| statement.params.len())
            .collect();
        (request.read_only, request.statements.len(), params)
    };
    let first_shape = sent.first().map(shape);
    assert_eq!(
        first_shape.as_ref().map(|(_, statements, _)| *statements),
        Some(STATEMENTS_PER_PUSH)
    );
    for request in &sent {
        assert_eq!(
            Some(shape(request)),
            first_shape,
            "every push the same shape"
        );
    }
}

/// A transport that counts the round trips it carries, over another.
#[derive(Debug, Clone)]
struct Counted<T> {
    inner: T,
    round_trips: Rc<Cell<usize>>,
}

impl<T: SqlTransport> SqlTransport for Counted<T> {
    type Error = T::Error;

    fn round_trip(
        &self,
        request: SqlRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> {
        self.round_trips.set(self.round_trips.get() + 1);
        self.inner.round_trip(request)
    }

    fn reads_settled(&self) -> impl Future<Output = ()> {
        self.inner.reads_settled()
    }
}

/// Every third event of `near`'s log, each a group of its own, in pushes of
/// at most `max_batch_events`: the part of Rotor's log the far side already
/// holds when the bulk push arrives.
async fn every_third<N>(near: &N, max_batch_events: usize) -> Vec<PushBatch>
where
    N: WriteGroupStore,
    N::Error: core::fmt::Debug,
{
    let whole = NonZeroUsize::new(ROTOR_EVENTS).expect("non-zero");
    let page = near
        .read_groups(None, whole)
        .await
        .expect("the near store's groups read");
    assert_eq!(page.waiting, Waiting::Nothing, "the log is one page here");
    let picked: Vec<EventGroup> = page
        .groups
        .into_iter()
        .flat_map(|group| EventGroup::from(group).events)
        .step_by(3)
        .map(|event| EventGroup::new(None, vec![event]))
        .collect();
    picked
        .chunks(max_batch_events)
        .map(|groups| PushBatch::new(groups.to_vec()))
        .collect()
}

/// SY-14's measurement, against the live endpoint: Rotor's 1,840 events
/// through `NeonSyncPeer`, a third of them already held, in 16 pushes of one
/// request each, each push on average inside one event's share of the
/// 34-minute window; and the replay, every event held, in 16 requests again,
/// which writes nothing. Prints the numbers on
/// `SY14-MEASURE` lines for the job's log.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn rotors_bulk_ingest_through_the_live_neon_peer_is_bounded() {
    let (config, transport) = fresh_schema().await;
    let round_trips = Rc::new(Cell::new(0));
    let counted: Counted<LiveTransport> = Counted {
        inner: transport,
        round_trips: Rc::clone(&round_trips),
    };
    let peer = NeonSyncPeer::new(NeonEventStore::new(counted, config));
    let limits = peer.limits();
    let near = rotor_log().await;
    let requests = || round_trips.get();

    let seeded_at = Instant::now();
    let mut seeded = 0_u64;
    let seeds = every_third(&near, limits.max_batch_events).await;
    for batch in &seeds {
        seeded += peer.push(batch).await.expect("a seed push lands").appended;
    }
    let seeding = seeded_at.elapsed();

    let started = Instant::now();
    let first = push_the_log(&near, &peer, requests).await;
    let bulk = started.elapsed();
    let replayed_at = Instant::now();
    let replay = push_the_log(&near, &peer, requests).await;
    let replaying = replayed_at.elapsed();
    let held = rows(peer.store()).await;

    println!(
        "SY14-MEASURE seed events={seeded} pushes={} elapsed_ms={}",
        seeds.len(),
        seeding.as_millis()
    );
    for (leg, pushed, elapsed) in [("bulk", first, bulk), ("replay", replay, replaying)] {
        println!(
            "SY14-MEASURE {leg} events={ROTOR_EVENTS} group_len={GROUP_LEN} max_batch_events={} \
             pushes={} requests={} appended={} skipped={} elapsed_ms={} per_request_ms={} \
             window_ms={}",
            limits.max_batch_events,
            pushed.pushes,
            pushed.requests,
            pushed.appended,
            pushed.skipped,
            elapsed.as_millis(),
            elapsed.as_millis() / u128::try_from(pushed.requests.max(1)).expect("small"),
            ROTOR_WINDOW.as_millis(),
        );
    }
    println!("SY14-MEASURE rows_held={held}");

    let pushes = expected_pushes(limits.max_batch_events);
    let total = u64::try_from(ROTOR_EVENTS).expect("small");
    assert_eq!(seeded, total.div_ceil(3), "every third event, seeded once");
    assert_eq!((first.pushes, first.requests), (pushes, pushes));
    assert_eq!((first.appended, first.skipped), (total - seeded, seeded));
    assert_eq!((replay.pushes, replay.requests), (pushes, pushes));
    assert_eq!((replay.appended, replay.skipped), (0, total));
    assert_eq!(held, total, "each of Rotor's events held once");
    // The whole window cannot be the assertion: the job's own timeout is
    // shorter, so a run past 34 minutes is cancelled, never red. The budget
    // that can fail here is one event's share of the window, the pace at
    // which SY-14's rejected one-request-per-event peer would exactly fill
    // it: each push of up to 120 events must, on average, clear it.
    let budget = ROTOR_WINDOW / u32::try_from(ROTOR_EVENTS).expect("small");
    for (leg, pushed, elapsed) in [("bulk", first, bulk), ("replay", replay, replaying)] {
        let per_push = elapsed / u32::try_from(pushed.pushes.max(1)).expect("small");
        assert!(
            per_push < budget,
            "the {leg} leg averaged {per_push:?} a push, past one event's share of Rotor's \
             {ROTOR_WINDOW:?} window, {budget:?}"
        );
    }
}
