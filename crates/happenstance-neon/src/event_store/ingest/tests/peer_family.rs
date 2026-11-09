//! `NeonSyncPeer` against the live endpoint (#131): the sync suite, a replay,
//! and a round trip whose payloads come back byte for byte.
//!
//! Unit tests in the library target, beside the ingest family, for two
//! reasons: the endpoint client in `endpoint.rs` is reachable only from here,
//! and CI's `live-neon` job already runs every `#[ignore]`d test of this
//! target (`cargo test -p happenstance-neon --all-features --lib -- --ignored
//! --test-threads=1`) and fails unless each one it lists passes. So mounting
//! here needs no change to CI. `--test-threads=1` for the frontier's reason:
//! every rule reads right after it writes.
//!
//! The near side of the round trip is the sync testkit's in-memory ingest
//! store, through the generic [`round_trip_through_the_peer`], which names
//! only the ports a near store implements.

use happenstance_core::{Event, EventStore, Query, ReadOptions, SequencedEvent, Tags, collect};
use happenstance_sync::{
    EventGroup, IngestGroup, IngestStore, PushBatch, StoreResume, SyncPeer, Waiting,
    WriteGroupStore,
};
use happenstance_sync_testkit::{
    IngestFixture, MemoryIngestFixture, RefusalsOf, SyncFixture, oversized_push_refusal,
};
use tokio::sync::OnceCell;

use super::endpoint::{LiveTransport, fresh_schema};
use crate::config::NeonConfig;
use crate::event_store::{NeonEventStore, NeonSyncPeer};

/// One fresh schema per fixture, and a peer over a new handle onto it per
/// `connect`.
#[derive(Debug)]
struct NeonPeerFixture {
    schema: OnceCell<(NeonConfig, LiveTransport)>,
}

impl NeonPeerFixture {
    fn new() -> Self {
        Self {
            schema: OnceCell::new(),
        }
    }
}

impl SyncFixture for NeonPeerFixture {
    type Peer = NeonSyncPeer<LiveTransport>;

    async fn connect(&self) -> Self::Peer {
        let (config, transport) = self.schema.get_or_init(fresh_schema).await;
        // A cheap handle onto the one client and ledger, and a copy of a few
        // names, which the store keeps.
        NeonSyncPeer::new(NeonEventStore::new(transport.clone(), config.clone()))
    }

    /// The oversized push, the refusal every peer that honours its own
    /// limits can be driven into: `Split`, before the wire.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }
}

/// `happenstance_sync_testkit::emit_sync_tokio`, with this crate's `#[ignore]`
/// reason added, as every live test here carries.
macro_rules! emit_ignored_sync_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
            async fn $name() -> ::core::result::Result<(), ::happenstance_sync_testkit::Violation> {
                ::happenstance_sync_testkit::rules::$name(__conformance_fixture)
                    .await?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}
pub(crate) use emit_ignored_sync_tokio;

happenstance_sync_testkit::sync_peer_conformance!(
    mod_name = sync_conformance,
    emit = super::emit_ignored_sync_tokio,
    fixture = super::NeonPeerFixture::new()
);

/// A peer on a fresh schema of the live endpoint.
async fn live_peer() -> NeonSyncPeer<LiveTransport> {
    NeonPeerFixture::new().connect().await
}

/// Every event `store` holds beneath its frontier, in position order.
async fn log<S: EventStore>(store: &S) -> Vec<SequencedEvent>
where
    S::Error: core::fmt::Debug,
{
    collect(store.read(&Query::all(), ReadOptions::default()))
        .await
        .expect("the log of a reachable store reads")
}

/// Payloads no text codec would survive: every byte value, an empty
/// payload, a lone `NUL`, invalid UTF-8, and a base64 alphabet's padding;
/// metadata on one, tags on another.
fn awkward_events() -> [Vec<Event>; 2] {
    let tagged = Tags::from_pairs([("k", "v")]).expect("a valid tag");
    [
        vec![
            Event::new("EveryByte", (0..=u8::MAX).collect::<Vec<u8>>())
                .expect("a valid type")
                .with_metadata((0..=u8::MAX).rev().collect::<Vec<u8>>()),
            Event::new("Empty", Vec::new()).expect("a valid type"),
            Event::new("Nul", vec![0_u8]).expect("a valid type"),
        ],
        vec![
            Event::new("NotUtf8", vec![0xC3, 0x28, 0xFF, 0xFE])
                .expect("a valid type")
                .with_tags(tagged),
            Event::new("Padding", b"==".to_vec()).expect("a valid type"),
        ],
    ]
}

/// Pulls from `peer` until it says nothing is waiting, following its tokens.
async fn pull_all<P: SyncPeer>(peer: &P) -> PushBatch
where
    P::Error: core::fmt::Debug,
{
    let mut groups = Vec::new();
    let mut from: Option<P::Resume> = None;
    // A bound, so a peer that never says `Nothing` fails rather than spins.
    for _ in 0..64 {
        let pulled = peer.pull(from.as_ref()).await.expect("a pull answers");
        groups.extend(pulled.batch.groups);
        if pulled.waiting == Waiting::Nothing {
            return PushBatch::new(groups);
        }
        from = Some(pulled.resume);
    }
    panic!("the peer never said nothing was waiting");
}

/// The groups `near` wrote, as a push carries them.
async fn groups_of<N: WriteGroupStore>(near: &N) -> PushBatch
where
    N::Error: core::fmt::Debug,
{
    let page = near
        .read_groups(None, core::num::NonZeroUsize::new(128).expect("non-zero"))
        .await
        .expect("the near store's groups read");
    assert_eq!(page.waiting, Waiting::Nothing, "the test log fits one page");
    PushBatch::new(page.groups.into_iter().map(EventGroup::from).collect())
}

/// Ingests `batch` into `near` group by group, with no compensation.
async fn ingest_into<N: IngestStore>(near: &N, batch: &PushBatch)
where
    N::Error: core::fmt::Debug,
{
    let groups: Vec<IngestGroup<'_>> = batch
        .groups
        .iter()
        .map(|group| IngestGroup::new(&group.events, &[]).with_guard(group.guard.as_ref()))
        .collect();
    near.ingest(&groups)
        .await
        .expect("a well-formed ingest into the near store lands");
}

/// The near store's own log to the peer and back, and the peer's own log to
/// the near store: every payload, metadata and tag set byte for byte, every
/// group's boundary kept, every identity and recorded time kept.
///
/// Generic over the near store so any store with the three ports can stand
/// at the near end; the in-memory ingest store does here.
async fn round_trip_through_the_peer<N>(near: &N, back: &N, peer: &NeonSyncPeer<LiveTransport>)
where
    N: EventStore + WriteGroupStore + IngestStore,
    <N as EventStore>::Error: core::fmt::Debug,
    <N as WriteGroupStore>::Error: core::fmt::Debug,
    <N as IngestStore>::Error: core::fmt::Debug,
{
    let [first, second] = awkward_events();
    for events in [&first, &second] {
        near.append(events, None)
            .await
            .expect("the near store's own append lands");
    }
    let sent = groups_of(near).await;

    let ack = peer.push(&sent).await.expect("the push lands");
    let pulled = pull_all(peer).await;

    assert_eq!(ack.appended, 5);
    assert_eq!(pulled, sent, "the near log came back from Neon unchanged");

    // And the other way: Neon's own append, pulled and ingested at the near
    // end, beside what came back.
    let own = [Event::new("NeonOwn", vec![0x00, 0xFF, 0x80]).expect("a valid type")];
    peer.store()
        .append(&own, None)
        .await
        .expect("the Neon store's own append lands");
    let everything = pull_all(peer).await;
    ingest_into(back, &everything).await;

    let back_log = log(back).await;
    let held: Vec<Carried<'_>> = back_log
        .iter()
        .map(|stored| carried(&stored.event))
        .collect();
    let expected: Vec<Carried<'_>> = first
        .iter()
        .chain(&second)
        .chain(&own)
        .map(carried)
        .collect();
    assert_eq!(held, expected, "every payload byte for byte, in order");
}

/// What a peer must carry untouched: the payload, the metadata, the tags.
type Carried<'e> = (&'e [u8], Option<&'e [u8]>, &'e Tags);

fn carried(event: &Event) -> Carried<'_> {
    (
        event.data().as_ref(),
        event.metadata().map(AsRef::as_ref),
        event.tags(),
    )
}

/// The near round trip, with the in-memory ingest store at the near end.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_round_trip_through_the_neon_peer_is_byte_identical() {
    let near = MemoryIngestFixture::new();
    let back = MemoryIngestFixture::new();

    round_trip_through_the_peer(
        &near.connect().await,
        &back.connect().await,
        &live_peer().await,
    )
    .await;
}

/// Replaying a batch twice changes nothing: the second push is all skips,
/// the log and the next pull's page are what they were, and the ack still
/// confirms the batch.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn replaying_a_pushed_batch_changes_nothing() {
    let near = MemoryIngestFixture::new().connect().await;
    let [first, second] = awkward_events();
    for events in [&first, &second] {
        EventStore::append(&near, events, None)
            .await
            .expect("the near store's own append lands");
    }
    let batch = groups_of(&near).await;
    let peer = live_peer().await;

    let once = peer.push(&batch).await.expect("the first push lands");
    let before = log(peer.store()).await;
    let page = peer.pull(None).await.expect("a pull answers");
    let again = peer.push(&batch).await.expect("the replay lands");

    assert_eq!((once.appended, once.skipped), (5, 0));
    assert_eq!((again.appended, again.skipped), (0, 5));
    assert_eq!(again.confirmed, once.confirmed);
    assert_eq!(again.responder, once.responder);
    assert_eq!(log(peer.store()).await, before, "the log is unchanged");
    let replayed = peer.pull(None).await.expect("a pull answers");
    assert_eq!(replayed.batch, page.batch);
    assert_eq!(replayed.resume, page.resume);
    assert_ne!(replayed.resume, StoreResume::start());
}
