//! A native SQLite store through the Neon and Postgres sync peers (#131):
//! payloads byte-identical both ways, and a replay that changes nothing.
//!
//! The cases are `#[ignore]`d with their reasons; `neon::` needs
//! `NEON_CONNECTION` and `postgres::` a server (see the crate root). The
//! helpers here name only the ports, so each far end is two lines.

#![cfg(not(target_arch = "wasm32"))]

mod groups;
mod neon;
mod postgres;

use core::num::NonZeroUsize;

use happenstance_core::{Event, EventStore, Query, ReadOptions, SequencedEvent, Tags, collect};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sync::{
    EventGroup, IngestGroup, IngestStore, PushBatch, SyncPeer, Waiting, WriteGroupStore,
};

/// The most pulls a drain makes before it decides the peer will never say it
/// is done: far above what these few groups need (H-13).
const MAX_PULLS: usize = 64;

/// A fresh in-memory SQLite store: the near end.
fn sqlite() -> SqliteEventStore {
    SqliteEventStore::open_in_memory().expect("an in-memory SQLite store opens")
}

/// Every event `store` holds, in position order.
async fn log<S: EventStore>(store: &S) -> Vec<SequencedEvent>
where
    S::Error: core::fmt::Debug,
{
    collect(store.read(&Query::all(), ReadOptions::default()))
        .await
        .expect("the log of a reachable store reads")
}

/// Payloads no text codec would survive, in two writes: every byte value,
/// an empty payload, a lone `NUL`, invalid UTF-8, a base64 alphabet's
/// padding; metadata on one, tags on another.
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

/// What a peer must carry untouched: the payload, the metadata, the tags.
type Carried<'e> = (&'e [u8], Option<&'e [u8]>, &'e Tags);

fn carried(event: &Event) -> Carried<'_> {
    (
        event.data().as_ref(),
        event.metadata().map(AsRef::as_ref),
        event.tags(),
    )
}

/// Appends the awkward events to `near`, two writes, and answers them.
async fn write_awkward<N: EventStore>(near: &N) -> [Vec<Event>; 2]
where
    N::Error: core::fmt::Debug,
{
    let written = awkward_events();
    for events in &written {
        near.append(events, None)
            .await
            .expect("the near store's own append lands");
    }
    written
}

/// The groups `near` wrote, as a push carries them.
async fn groups_of<N: WriteGroupStore>(near: &N) -> PushBatch
where
    N::Error: core::fmt::Debug,
{
    let page = near
        .read_groups(None, NonZeroUsize::new(128).expect("non-zero"))
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
    for _ in 0..MAX_PULLS {
        let pulled = peer.pull(from.as_ref()).await.expect("a pull answers");
        groups.extend(pulled.batch.groups);
        if pulled.waiting == Waiting::Nothing {
            return PushBatch::new(groups);
        }
        from = Some(pulled.resume);
    }
    panic!("the peer never said nothing was waiting in {MAX_PULLS} pulls");
}

/// Ingests `batch` into `near`, group for group, with no compensation.
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

/// SQLite to the far end and back, then the far end's own append to a second
/// SQLite store: every group kept, every payload, metadata and tag set byte
/// for byte, in order.
///
/// `far` is the store `peer` serves, written to directly for the far end's
/// own event.
async fn sqlite_round_trip<P, F>(peer: &P, far: &F)
where
    P: SyncPeer,
    P::Error: core::fmt::Debug,
    F: EventStore,
    F::Error: core::fmt::Debug,
{
    let near = sqlite();
    let back = sqlite();
    let [first, second] = write_awkward(&near).await;
    let sent = groups_of(&near).await;

    let ack = peer.push(&sent).await.expect("the push lands");
    let pulled = pull_all(peer).await;
    let own = [Event::new("FarOwn", vec![0x00, 0xFF, 0x80]).expect("a valid type")];
    far.append(&own, None)
        .await
        .expect("the far store's own append lands");
    let everything = pull_all(peer).await;
    ingest_into(&back, &everything).await;

    assert_eq!(ack.appended, 5);
    assert_eq!(pulled, sent, "the SQLite log came back unchanged");
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

/// A SQLite batch pushed twice: the second push is all skips, and the far
/// log and the next pull's page are what they were.
async fn replay_changes_nothing<P, F>(peer: &P, far: &F)
where
    P: SyncPeer,
    P::Error: core::fmt::Debug,
    P::Resume: PartialEq + core::fmt::Debug,
    F: EventStore,
    F::Error: core::fmt::Debug,
{
    let near = sqlite();
    write_awkward(&near).await;
    let batch = groups_of(&near).await;

    let once = peer.push(&batch).await.expect("the first push lands");
    let before = log(far).await;
    let page = peer.pull(None).await.expect("a pull answers");
    let again = peer.push(&batch).await.expect("the replay lands");
    let replayed = peer.pull(None).await.expect("a pull answers");

    assert_eq!((once.appended, once.skipped), (5, 0));
    assert_eq!((again.appended, again.skipped), (0, 5));
    assert_eq!(again.confirmed, once.confirmed);
    assert_eq!(log(far).await, before, "the far log is unchanged");
    assert_eq!(replayed.batch, page.batch);
    assert_eq!(replayed.resume, page.resume);
}
