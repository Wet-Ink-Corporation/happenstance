//! The wrong stores ES-38's rule rejects (#151): stores whose removal is
//! right and whose behaviour **after** it is not.
//!
//! `removal.rs` holds the fixtures whose removal itself is wrong, and the
//! guard rejects them. Every store here removes exactly the named events, so
//! the guard passes it, and gets one later step wrong: the next position, or
//! `head`. A module of its own for `removal.rs`'s reason, declared at the end
//! of `mutants.rs` so that no cited line there moves.
//!
//! One generic fixture over one [`AfterRemoval`] step, on
//! `mutants::Defect`'s pattern: each wrong store overrides exactly one of the
//! trait's provided methods, and every provided method is the correct step,
//! so the override is the only difference from a correct store.

use core::cell::RefCell;
use core::marker::PhantomData;
use std::collections::BTreeSet;
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent,
};
use happenstance_testkit::{Capability, Fixture, Removed};

use crate::correct::{Log, LogError, Snapshot, dense};
use crate::harness::Subject;
use crate::mutants::removal::NO_DURABLE_MEDIUM;

/// What a store here holds.
///
/// A struct of one field for now, so that a defect's signature names the
/// store's state rather than its log, and a later field does not change every
/// override.
#[derive(Debug)]
pub(crate) struct Removable {
    /// The events the store holds, and the high-water mark a correct removal
    /// keeps.
    pub(crate) log: Log,
}

impl Removable {
    fn new() -> Self {
        Self {
            log: Log::new(dense),
        }
    }

    /// The positions this store holds, ascending.
    fn held(&self) -> Vec<SequencePosition> {
        self.log
            .select(&Query::all(), ReadOptions::new())
            .iter()
            .map(|event| event.position)
            .collect()
    }
}

/// One step a store here takes after, or as part of, a removal, each with
/// the correct body provided.
pub(crate) trait AfterRemoval: 'static {
    /// The registry key.
    const NAME: &'static str;

    /// The removal `remove_outside_port` performs.
    fn remove(state: &mut Removable, positions: &[SequencePosition]) -> Removed {
        remove_correctly(state, positions)
    }

    /// The store's `append`.
    ///
    /// # Errors
    ///
    /// As `Log::append`.
    fn append(
        state: &mut Removable,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        state.log.append(events, condition)
    }

    /// The store's `head`.
    fn head(state: &Removable) -> Option<SequencePosition> {
        state.log.head()
    }
}

/// The correct removal: the high-water mark first, then the events.
fn remove_correctly(state: &mut Removable, positions: &[SequencePosition]) -> Removed {
    let removed = state.log.remove(positions);
    removed_every_named(positions, removed)
}

/// `Removed::Deleted`, once `removed` is the number of distinct positions
/// named.
///
/// # Panics
///
/// Otherwise, as a broken test environment: a rule names only positions the
/// store holds, which `remove_outside_port`'s contract lets an override
/// enforce by panicking.
fn removed_every_named(positions: &[SequencePosition], removed: usize) -> Removed {
    let named: BTreeSet<&SequencePosition> = positions.iter().collect();
    assert_eq!(
        removed,
        named.len(),
        "a rule names only positions this store holds, and it named {positions:?}"
    );
    Removed::Deleted
}

/// A fixture that declares `REMOVAL`, removes correctly, and takes `D`'s
/// step after it.
#[derive(Debug)]
pub(crate) struct AfterRemovalFixture<D> {
    state: Rc<RefCell<Removable>>,
    defect: PhantomData<D>,
}

impl<D: AfterRemoval> Fixture for AfterRemovalFixture<D> {
    type Store = AfterRemovalStore<D>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(NO_DURABLE_MEDIUM);
    // Granted honestly: the removal reaches the state every handle shares,
    // which is all the storage these stores have.
    const REMOVAL: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        AfterRemovalStore {
            state: Rc::clone(&self.state),
            defect: PhantomData,
        }
    }

    async fn remove_outside_port(&self, positions: &[SequencePosition]) -> Removed {
        D::remove(&mut self.state.borrow_mut(), positions)
    }
}

impl<D: AfterRemoval> Subject for AfterRemovalFixture<D> {
    const NAME: &'static str = D::NAME;

    fn open() -> Self {
        Self {
            state: Rc::new(RefCell::new(Removable::new())),
            defect: PhantomData,
        }
    }
}

/// One handle onto an [`AfterRemovalFixture`]'s state.
#[derive(Debug)]
pub(crate) struct AfterRemovalStore<D> {
    state: Rc<RefCell<Removable>>,
    defect: PhantomData<D>,
}

impl<D: AfterRemoval> EventStore for AfterRemovalStore<D> {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // Not `async`, and the stream at the top level (ADR-0008), as
        // `LogStore::read`: the borrow is taken and released before the
        // stream exists.
        Snapshot::new(
            self.state
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .map(|state| state.log.select(query, options)),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // No `.await` while the borrow is held: it is taken and released
        // inside one poll.
        let mut state = self
            .state
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
        D::append(&mut state, events, condition)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let state = self
            .state
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(D::head(&state))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let state = self
            .state
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(state.log.contains(id))
    }
}

// =====================================================================
// ES-38: the next position and `head` after a removal (#151)
// =====================================================================

/// A removal that forgets the highest position ever assigned: ES-38's named
/// wrong implementation.
///
/// `INTEGER PRIMARY KEY` without `AUTOINCREMENT`. SQLite keeps
/// `sqlite_sequence` only for an `AUTOINCREMENT` table, so nothing remembers
/// the largest rowid ever used, and the next insert takes one above the
/// largest rowid still held: the deleted tail's position.
/// `happenstance-sqlite`'s `tests/migration.rs` shows it on real SQLite.
#[derive(Debug)]
pub(crate) struct RowidReuse;

impl AfterRemoval for RowidReuse {
    const NAME: &'static str = "RowidReuseFixture";

    fn remove(state: &mut Removable, positions: &[SequencePosition]) -> Removed {
        // THE DEFECT: `discard` without `remember_high_water`.
        let removed = state.log.discard(positions);
        removed_every_named(positions, removed)
    }
}

/// A `head` read from the allocator rather than from the rows.
///
/// `SELECT seq FROM sqlite_sequence`, or a sequence's `last_value`: an O(1)
/// head that agrees with `max(position)` until something is removed, and then
/// names an event the store no longer holds.
#[derive(Debug)]
pub(crate) struct SequenceHead;

impl AfterRemoval for SequenceHead {
    const NAME: &'static str = "SequenceHeadFixture";

    fn head(state: &Removable) -> Option<SequencePosition> {
        // THE DEFECT: the allocator's mark wins over the last held row.
        state.log.high_water().max(state.log.head())
    }
}

/// A removal of everything that rewinds the allocator.
///
/// A retention job that purges a whole store with `TRUNCATE … RESTART
/// IDENTITY` because it is faster, or that resets the sequence on an empty
/// table. Positions restart under the same identity, which `REMOVAL`'s
/// documentation calls a different store. Any partial removal is correct.
#[derive(Debug)]
pub(crate) struct TruncatingWhenEmpty;

impl AfterRemoval for TruncatingWhenEmpty {
    const NAME: &'static str = "TruncatingWhenEmptyFixture";

    fn remove(state: &mut Removable, positions: &[SequencePosition]) -> Removed {
        let held = state.held();
        if held.is_empty() || !held.iter().all(|position| positions.contains(position)) {
            return remove_correctly(state, positions);
        }
        // THE DEFECT: every held event is named, so the store is replaced by
        // an empty one whose allocator starts again.
        state.log = Log::new(dense);
        removed_every_named(positions, held.len())
    }
}
