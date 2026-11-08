//! The Postgres sync peer (#131): `StoreSyncPeer` over this store, under the
//! sync suite, with a replay and a byte-identical round trip.
//!
//! # Why no `PostgresSyncPeer`
//!
//! `happenstance-neon` carries a `NeonSyncPeer` because a round trip is the
//! expensive unit there, and `StoreSyncPeer` asks a store one question per
//! port method: two HTTP round trips a pull and three a push. Over a pool
//! those are two or three queries on a held connection, and `read_groups`
//! already decides `waiting` inside its own statement (SY-37), so a type that
//! fused them would buy nothing SY-15 or SY-37 asks for. The two crates share
//! no SQL either: no adapter depends on another. So the Postgres peer is
//! `StoreSyncPeer<PostgresEventStore, PostgresEventStore>`, and this file is
//! the evidence that it clears the bar.
//!
//! Unit tests in the library target, beside the ingest family, because the
//! server harness in `server.rs` is reachable only from here; CI's
//! `live-postgres` job already runs and counts every `#[ignore]`d test of this
//! target, so mounting here needs no change to CI.

use happenstance_core::{Event, EventStore, Query, ReadOptions, SequencedEvent, Tags, collect};
use happenstance_sync::{
    EventGroup, IngestGroup, IngestStore, PushBatch, StoreSyncPeer, SyncPeer, Waiting,
    WriteGroupStore,
};
use happenstance_sync_testkit::{
    IngestFixture, MemoryIngestFixture, RefusalsOf, SyncFixture, oversized_push_refusal,
};
use sqlx::PgPool;
use tokio::sync::OnceCell;

use super::server;
use crate::event_store::PostgresEventStore;

/// The Postgres sync peer: the store is both halves.
type PostgresPeer = StoreSyncPeer<PostgresEventStore, PostgresEventStore>;

/// One fresh schema per fixture, and a peer over new handles onto it per
/// `connect`.
#[derive(Debug)]
struct PostgresPeerFixture {
    pool: OnceCell<PgPool>,
}

impl PostgresPeerFixture {
    fn new() -> Self {
        Self {
            pool: OnceCell::new(),
        }
    }
}

impl SyncFixture for PostgresPeerFixture {
    type Peer = PostgresPeer;

    async fn connect(&self) -> Self::Peer {
        // Refcounted handles onto the one pool, which every handle shares.
        let pool = self.pool.get_or_init(server::fresh_pool).await;
        StoreSyncPeer::new(
            PostgresEventStore::new(pool.clone()),
            PostgresEventStore::new(pool.clone()),
        )
    }

    /// The oversized push, the refusal every peer that honours its own
    /// limits can be driven into: `Split`, before the store is asked.
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
            #[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
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
    fixture = super::PostgresPeerFixture::new()
);

/// Every event `store` holds beneath its frontier, in position order.
async fn log<S: EventStore>(store: &S) -> Vec<SequencedEvent>
where
    S::Error: core::fmt::Debug,
{
    collect(store.read(&Query::all(), ReadOptions::default()))
        .await
        .expect("the log of a reachable store reads")
}

/// Payloads no text codec would survive, in two writes; metadata on one,
/// tags on another.
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

/// A near store's log to the Postgres peer and back, and the Postgres
/// store's own append to a second near store: every payload, metadata and
/// tag set byte for byte.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_round_trip_through_the_postgres_peer_is_byte_identical() {
    let near = MemoryIngestFixture::new().connect().await;
    let back = MemoryIngestFixture::new().connect().await;
    let fixture = PostgresPeerFixture::new();
    let peer = fixture.connect().await;
    let far = PostgresEventStore::new(fixture.pool.get_or_init(server::fresh_pool).await.clone());
    let [first, second] = awkward_events();
    for events in [&first, &second] {
        EventStore::append(&near, events, None)
            .await
            .expect("the near store's own append lands");
    }
    let sent = groups_of(&near).await;

    let ack = peer.push(&sent).await.expect("the push lands");
    let pulled = pull_all(&peer).await;
    let own = [Event::new("PostgresOwn", vec![0x00, 0xFF, 0x80]).expect("a valid type")];
    far.append(&own, None)
        .await
        .expect("the Postgres store's own append lands");
    let everything = pull_all(&peer).await;
    let groups: Vec<IngestGroup<'_>> = everything
        .groups
        .iter()
        .map(|group| IngestGroup::new(&group.events, &[]).with_guard(group.guard.as_ref()))
        .collect();
    IngestStore::ingest(&back, &groups)
        .await
        .expect("a well-formed ingest into the near store lands");

    assert_eq!(ack.appended, 5);
    assert_eq!(
        pulled, sent,
        "the near log came back from Postgres unchanged"
    );
    let back_log = log(&back).await;
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

/// Replaying a batch twice changes nothing: the second push is all skips,
/// and the log and the next pull's page are what they were.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn replaying_a_pushed_batch_changes_nothing() {
    let near = MemoryIngestFixture::new().connect().await;
    let [first, second] = awkward_events();
    for events in [&first, &second] {
        EventStore::append(&near, events, None)
            .await
            .expect("the near store's own append lands");
    }
    let batch = groups_of(&near).await;
    let fixture = PostgresPeerFixture::new();
    let peer = fixture.connect().await;
    let far = PostgresEventStore::new(fixture.pool.get_or_init(server::fresh_pool).await.clone());

    let once = peer.push(&batch).await.expect("the first push lands");
    let before = log(&far).await;
    let page = peer.pull(None).await.expect("a pull answers");
    let again = peer.push(&batch).await.expect("the replay lands");

    assert_eq!((once.appended, once.skipped), (5, 0));
    assert_eq!((again.appended, again.skipped), (0, 5));
    assert_eq!(again.confirmed, once.confirmed);
    assert_eq!(log(&far).await, before, "the log is unchanged");
    let replayed = peer.pull(None).await.expect("a pull answers");
    assert_eq!(replayed.batch, page.batch);
    assert_eq!(replayed.resume, page.resume);
}
