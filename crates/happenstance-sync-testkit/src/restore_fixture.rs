//! What an adapter hands the restore family, and the reference implementation
//! of it.

use alloc::format;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::future::Future;
use core::num::NonZeroUsize;

use happenstance_core::{Event, SendEventStore, SequencePosition};
use happenstance_sync::{
    EventGroup, IngestGroup, MemoryIngestStore, SendIngestStore, SendWriteGroupStore,
    StoreSyncPeer, Waiting,
};

use crate::ingest_fixture::MemoryIngestHandle;
use crate::violation::{Violation, ensure};

/// The clause every failure of a restore fixture is reported under: the one
/// rule that asked for it.
const VT_6: &str = "VT-6";

/// One isolated far side whose storage can be backed up and restored, and a
/// way to reach it.
///
/// One fixture instance is one isolated backing store, and each
/// [`connect`](Self::connect) is one peer onto it: the other families'
/// contract. What this family adds is the operator's half of the store's life:
/// a writer of the store's **own** events, a backup and a restore. A peer port
/// has none of the three, so the fixture supplies them.
///
/// # Why a trait of its own
///
/// Not a capability on [`SyncFixture`](crate::SyncFixture): a peer that issues
/// no identity of its own, such as `happenstance_sync::MemorySyncPeer`, which
/// holds only what it was pushed, has nothing a restore could reissue, and
/// every rule of that family is held against peers built on it. Here the three
/// methods are required, so mounting the family is the declaration that the
/// store can be restored.
///
/// # No `Send`, on purpose
///
/// Bounded on the bare [`SyncPeer`](happenstance_sync::SyncPeer), as
/// [`SyncFixture`](crate::SyncFixture) is, so a `!Send` peer is a valid
/// fixture.
///
/// # Errors
///
/// Every method but `connect` returns a [`Violation`] of VT-6 when the fixture
/// cannot do what it was asked. That is the fixture's own failure, reported
/// under the clause of the rule that asked, as SY-36's provocations are.
pub trait RestoreFixture {
    /// The peer adapter under test.
    type Peer: happenstance_sync::SyncPeer;

    /// A fresh peer onto this fixture's store, as a process started now would
    /// open one.
    ///
    /// An implementation in test code may panic when the environment cannot
    /// connect, as the other families' fixtures do.
    fn connect(&self) -> impl Future<Output = Self::Peer>;

    /// Appends `events` to the store as one local write, as the store's own
    /// writer would: each event takes an identity the store mints.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of VT-6 if the store refuses the write.
    fn append_local(&self, events: &[Event]) -> impl Future<Output = Result<(), Violation>>;

    /// Takes a backup of the store's storage as it stands now: everything a
    /// point-in-time restore to this moment would bring back, the store's
    /// identity among it.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of VT-6 if the backup cannot be taken.
    fn back_up(&self) -> impl Future<Output = Result<(), Violation>>;

    /// Puts the store's storage back to the last [`back_up`](Self::back_up),
    /// as an operator's restore would, and then takes **every step the adapter
    /// documents a deployment must take after a restore**.
    ///
    /// For an adapter that mints its identity once, that is VT-6's re-mint,
    /// run the way its documentation says to run it. An adapter that detects a
    /// restore, or mints on every open, documents no step, and does nothing
    /// here past the restore itself. The rule drops every peer it connected
    /// before it asks for the restore, and connects a fresh one after it, as a
    /// deployment stops its writers first and restarts what held the store.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of VT-6 if there is no backup to restore, or the
    /// restore or a documented step fails.
    fn restore(&self) -> impl Future<Output = Result<(), Violation>>;
}

/// The most groups one page of a [`MemoryRestoreFixture`] backup reads.
const BACKUP_PAGE: NonZeroUsize = NonZeroUsize::MIN.saturating_add(127);

/// The most pages a [`MemoryRestoreFixture`] backup reads before it reports
/// the store as one that never ends (H-13). Far above anything a rule writes.
const MAX_BACKUP_PAGES: usize = 1_024;

/// The reference [`RestoreFixture`]: a
/// [`StoreSyncPeer`] over a [`MemoryIngestStore`].
///
/// The in-memory peer that issues identities of its own. `MemorySyncPeer`
/// issues none, so it is not this family's reference; a store peer over the
/// in-memory oracle is the far side every networked peer in this workspace
/// stands in front of.
///
/// # How it restores
///
/// A backup reads the store's write groups whole: every event with its
/// identity, its recorded time and its group's guard. A restore builds a
/// **fresh** [`MemoryIngestStore`], with an incarnation of its own, and
/// ingests the backed-up groups into it. Ingest keeps an origin's identity and
/// time (VT-5, VT-9), so the restored log holds every backed-up event as it
/// was, at the positions it held; the fresh incarnation is the re-mint. The
/// in-memory store can do no other: it refuses a claim on its own identity it
/// does not hold (SY-38), so a restore under the old identity cannot be built
/// from its public API at all.
#[derive(Debug, Default)]
pub struct MemoryRestoreFixture {
    /// The store a peer connected now reaches. Swapped by a restore, so a
    /// `RefCell`: the fixture is used from one task, and no borrow outlives the
    /// statement that takes it.
    store: RefCell<Arc<MemoryIngestStore>>,
    /// The groups the last backup read, taken by the restore that uses them.
    backup: RefCell<Option<Vec<EventGroup>>>,
}

impl MemoryRestoreFixture {
    /// A fixture over a fresh in-memory store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// The store a peer connected now reaches.
    fn current(&self) -> Arc<MemoryIngestStore> {
        Arc::clone(&self.store.borrow())
    }
}

impl RestoreFixture for MemoryRestoreFixture {
    type Peer = StoreSyncPeer<MemoryIngestHandle, MemoryIngestHandle>;

    fn connect(&self) -> impl Future<Output = Self::Peer> {
        let store = self.current();
        core::future::ready(StoreSyncPeer::new(
            MemoryIngestHandle::over(Arc::clone(&store)),
            MemoryIngestHandle::over(store),
        ))
    }

    async fn append_local(&self, events: &[Event]) -> Result<(), Violation> {
        let store = self.current();
        SendEventStore::append(&*store, events, None)
            .await
            .map(drop)
            .map_err(|error| {
                Violation::new(
                    VT_6,
                    format!("the in-memory store refused a local append: {error}"),
                )
            })
    }

    async fn back_up(&self) -> Result<(), Violation> {
        let groups = every_group(&self.current()).await?;
        self.backup.replace(Some(groups));
        Ok(())
    }

    async fn restore(&self) -> Result<(), Violation> {
        let groups = self.backup.take().ok_or_else(|| {
            Violation::new(VT_6, "the fixture was asked to restore before any backup")
        })?;
        let restored = MemoryIngestStore::new();
        let ingest: Vec<IngestGroup<'_>> = groups
            .iter()
            .map(|group| IngestGroup::new(&group.events, &[]).with_guard(group.guard.as_ref()))
            .collect();
        let ingested = SendIngestStore::ingest(&restored, &ingest)
            .await
            .map_err(|error| {
                Violation::new(
                    VT_6,
                    format!("the restored store refused the backup: {error}"),
                )
            })?;
        let backed_up: usize = groups.iter().map(|group| group.events.len()).sum();
        ensure(ingested.appended == backed_up, VT_6, || {
            format!(
                "the restored store took {} of the {backed_up} backed-up events",
                ingested.appended
            )
        })?;
        self.store.replace(Arc::new(restored));
        Ok(())
    }
}

/// Every write group `store` holds, read page by page, as a push carries them.
async fn every_group(store: &MemoryIngestStore) -> Result<Vec<EventGroup>, Violation> {
    let mut groups = Vec::new();
    let mut after: Option<SequencePosition> = None;
    for _ in 0..MAX_BACKUP_PAGES {
        let page = SendWriteGroupStore::read_groups(store, after, BACKUP_PAGE)
            .await
            .map_err(|error| {
                Violation::new(
                    VT_6,
                    format!("the in-memory store's groups did not read: {error}"),
                )
            })?;
        let waiting = page.waiting;
        after = page.groups.last().map(happenstance_sync::WriteGroup::first);
        groups.extend(page.groups.into_iter().map(EventGroup::from));
        if waiting == Waiting::Nothing || after.is_none() {
            return Ok(groups);
        }
    }
    Err(Violation::new(
        VT_6,
        format!("the in-memory store's groups ran past {MAX_BACKUP_PAGES} pages"),
    ))
}
