//! No ingest rule drops its fixture while a store it connected is alive.
//!
//! One fixture instance is one isolated backing store, and a real fixture may
//! own that backing: the SQLite fixture's temporary file is unlinked when the
//! fixture drops. A rule that writes `open().await.connect().await` drops the
//! fixture at the end of that statement and keeps the store, so the teardown
//! runs under a live handle (#500; on Windows, os error 32, which #499 had to
//! tolerate).
//!
//! The fixture here is faithful in everything the rules check, and counts the
//! handles it gave out that are still alive. Its `Drop` records, in a tally the
//! test reads, every drop with that count above zero. Recording rather than
//! panicking keeps a panic out of `Drop`, where it would abort if it ran during
//! another unwind, and lets the test name every rule that did it at once.

#![cfg(not(target_arch = "wasm32"))]

use std::cell::Cell;
use std::rc::Rc;

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, MemoryStoreError, Query, ReadOptions,
    SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::{IngestError, IngestGroup, IngestStore, Ingested, Watermark};
use happenstance_sync_testkit::{
    IngestFixture, MemoryIngestFixture, MemoryIngestHandle, Violation, ingest_rules,
};
use happenstance_testkit::{Capability, Removed, RuleOutcome, block_on};

/// A fixture over the in-memory oracle that notices being dropped under a
/// live handle.
///
/// `Rc<Cell<_>>` for both counts: each is shared by the fixture and its
/// handles (or by every fixture one rule opens), mutated through `&self`, and
/// the blocking emitter runs on one thread, so nothing needs to be `Send`.
struct WatchedFixture {
    inner: MemoryIngestFixture,
    /// Handles this fixture gave out that have not dropped yet.
    live: Rc<Cell<usize>>,
    /// Drops, over every fixture one rule opened, that found `live` above zero.
    dropped_under_a_live_handle: Rc<Cell<usize>>,
}

impl WatchedFixture {
    fn new(tally: &Rc<Cell<usize>>) -> Self {
        Self {
            inner: MemoryIngestFixture::new(),
            live: Rc::default(),
            dropped_under_a_live_handle: Rc::clone(tally),
        }
    }
}

impl Drop for WatchedFixture {
    fn drop(&mut self) {
        if self.live.get() > 0 {
            let tally = &self.dropped_under_a_live_handle;
            tally.set(tally.get() + 1);
        }
    }
}

impl IngestFixture for WatchedFixture {
    type Store = WatchedStore;

    const MID_GROUP_FAULT: Capability = Capability::declined(
        "a fixture over the in-memory oracle, which writes an ingest in one step and has no \
         fault seam; it exists to watch its own lifetime, not to fault a write",
    );

    /// Granted as the oracle grants it, so SY-40's rule runs here too.
    const REMOVAL: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        self.live.set(self.live.get() + 1);
        WatchedStore {
            inner: self.inner.connect().await,
            live: Rc::clone(&self.live),
        }
    }

    async fn remove_outside_port(&self, positions: &[SequencePosition]) -> Removed {
        self.inner.remove_outside_port(positions).await
    }
}

/// One handle onto a [`WatchedFixture`]'s store, counted while it lives.
#[derive(Debug)]
struct WatchedStore {
    inner: MemoryIngestHandle,
    live: Rc<Cell<usize>>,
}

impl Drop for WatchedStore {
    fn drop(&mut self) {
        self.live.set(self.live.get().saturating_sub(1));
    }
}

impl EventStore for WatchedStore {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        EventStore::read(&self.inner, query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        EventStore::append(&self.inner, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        EventStore::head(&self.inner).await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        EventStore::contains_event_id(&self.inner, id).await
    }
}

impl IngestStore for WatchedStore {
    type Error = MemoryStoreError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        IngestStore::store_id(&self.inner).await
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        IngestStore::ingest(&self.inner, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        IngestStore::watermark(&self.inner).await
    }
}

/// One rule's verdict, and how many of the fixtures it opened it dropped
/// under a live handle.
type Run = (&'static str, Result<RuleOutcome, Violation>, usize);

/// Every ingest rule against a fresh [`WatchedFixture`] opener, with its own
/// tally so a drop is attributed to the rule that did it.
fn runs() -> Vec<Run> {
    macro_rules! run_each {
        ($($name:ident),* $(,)?) => {
            vec![$({
                let tally = Rc::new(Cell::new(0));
                let opener = &tally;
                let verdict = block_on(ingest_rules::$name(async || WatchedFixture::new(opener)));
                (stringify!($name), verdict, tally.get())
            }),*]
        };
    }
    happenstance_sync_testkit::for_each_ingest_store_rule!(run_each)
}

/// The control: watching the lifetime changes no rule's verdict.
#[test]
fn the_watched_fixture_passes_every_ingest_rule() {
    let failed: Vec<(&str, Violation)> = runs()
        .into_iter()
        .filter_map(|(name, verdict, _)| verdict.err().map(|violation| (name, violation)))
        .collect();

    assert!(failed.is_empty(), "rules that failed: {failed:?}");
}

/// SY-7's and SY-10's rules are the ones #500 names; every other rule is held
/// to the same bar so a new rule cannot reintroduce the shape.
#[test]
fn no_rule_drops_its_fixture_under_a_live_handle() {
    let offenders: Vec<(&str, usize)> = runs()
        .into_iter()
        .filter(|(_, _, dropped)| *dropped > 0)
        .map(|(name, _, dropped)| (name, dropped))
        .collect();

    assert!(
        offenders.is_empty(),
        "rules that dropped a fixture under a live handle: {offenders:?}"
    );
}

/// The instrument's premise: a fixture kept alive past its handle records
/// nothing, and one dropped first records the drop.
#[test]
fn the_tally_counts_a_drop_under_a_live_handle_and_nothing_else() {
    let tally = Rc::new(Cell::new(0));

    let kept = WatchedFixture::new(&tally);
    drop(block_on(kept.connect()));
    drop(kept);
    assert_eq!(tally.get(), 0, "a fixture that outlived its handle");

    let early = WatchedFixture::new(&tally);
    let store = block_on(early.connect());
    drop(early);
    drop(store);
    assert_eq!(tally.get(), 1, "a fixture dropped under its handle");
}
