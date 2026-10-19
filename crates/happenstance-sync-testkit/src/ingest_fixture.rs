//! What a store adapter hands the ingest family, and the reference
//! implementation of it.

use alloc::format;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::future::Future;

use happenstance_testkit::Capability;

use crate::violation::{Violation, ensure};

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, MemoryStoreError, Query, ReadOptions,
    SendEventStore, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::{
    IngestError, IngestGroup, Ingested, MemoryIngestStore, SendIngestStore, Watermark,
};

/// One isolated backing store that both appends and ingests, and a way to
/// reach it.
///
/// One fixture instance is one isolated backing store, and each
/// [`connect`](Self::connect) is one handle onto it: the event-store family's
/// contract, one port over. A rule calls the opener it is given for a fresh
/// instance, so no rule sees another's events.
///
/// # No `Send`, on purpose
///
/// Not `trait_variant`-derived, and bounded on the bare `EventStore` and
/// `IngestStore`, so a `!Send` store, the Durable Object's, is a valid fixture.
/// This crate's `tests/local_ingest_conformance.rs` mounts one.
///
/// # Capabilities are defaulted items
///
/// Every item past [`connect`](Self::connect) has a default, so each one
/// added is additive to every implementation written before it, as
/// `happenstance_testkit::Fixture`'s capabilities are:
///
/// * [`MID_GROUP_FAULT`](Self::MID_GROUP_FAULT) and
///   [`arm_mid_group_fault`](Self::arm_mid_group_fault), which SY-30's rule
///   needs. Declined by default, in this crate's words, and
///   [`ingest_declensions_are_stated`] rejects a fixture that keeps them.
/// * Three per-event ceilings, which SY-18's ingest rule holds to every event
///   an ingest writes. `None` by default: facts, not trades.
pub trait IngestFixture {
    /// The store under test: an event store that also ingests.
    type Store: happenstance_core::EventStore + happenstance_sync::IngestStore;

    /// Whether this fixture can make its store fail between two rows of one
    /// ingest group.
    ///
    /// SY-30 says a group lands whole, and a conformant store decides every
    /// refusal before it writes, so the case left is a fault inside the write.
    /// Nothing in the port reaches between two rows of one `ingest`, so the
    /// injection is the adapter's: a trigger, a constraint armed for one write.
    ///
    /// A store that can absorb every fault its fixture could arm, and commit
    /// the group anyway, declines: the rule requires an armed ingest to fail,
    /// or it has not shown that a failure leaves nothing behind.
    const MID_GROUP_FAULT: Capability = Capability::declined(UNSTATED_MID_GROUP_FAULT);

    /// The largest `data` payload, in bytes, the store accepts in one event,
    /// or `None` for no ceiling.
    ///
    /// A fact about the store rather than a trade, so an `Option` rather than
    /// a [`Capability`]. It must mirror the adapter's own constant, never a
    /// literal restated here, or the rule checks the fixture and not the store.
    const MAX_EVENT_DATA_LEN: Option<usize> = None;

    /// The most tags the store accepts on one event, or `None` for no
    /// ceiling. Mirrored, as [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN)
    /// is.
    const MAX_TAGS_PER_EVENT: Option<usize> = None;

    /// The largest `metadata`, in bytes, the store accepts on one event, or
    /// `None` for no ceiling. Mirrored, as
    /// [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN) is. Zero is a legal
    /// declaration, because the contract guarantees no metadata capacity.
    const MAX_METADATA_LEN: Option<usize> = None;

    /// A handle onto this fixture's one backing store.
    ///
    /// An implementation in test code may panic when the environment cannot
    /// connect, as the event-store family's fixtures do: the suite has nothing
    /// to report about a store it never reached.
    fn connect(&self) -> impl Future<Output = Self::Store>;

    /// Arms the store so that its next `ingest` fails while writing the
    /// `after + 1`-th row it writes in that call.
    ///
    /// Foreign rows and compensation rows both count, in the store's own write
    /// order; a row it skips is not written and is not counted. Arming reaches
    /// handles already connected. The fault may stay armed for the fixture's
    /// life: no rule writes to an armed fixture except with the one ingest it
    /// armed for.
    ///
    /// # Errors
    ///
    /// The provided body returns a [`Violation`] of SY-30, naming the two ways
    /// to reach it: a fixture that declares
    /// [`MID_GROUP_FAULT`](Self::MID_GROUP_FAULT) supported and does not
    /// override this, or a rule that did not check the capability first. An
    /// override returns one when its environment cannot arm the fault.
    fn arm_mid_group_fault(&self, after: usize) -> impl Future<Output = Result<(), Violation>> {
        core::future::ready(Err(Violation::new(
            "SY-30",
            format!(
                "`IngestFixture::arm_mid_group_fault({after})` was called and not overridden: \
                 either this fixture declares MID_GROUP_FAULT supported and does not arm the \
                 fault, or a rule armed it without checking the capability"
            ),
        )))
    }
}

/// What [`IngestFixture::MID_GROUP_FAULT`] says when a fixture says nothing.
///
/// `pub` so an adapter author can read what their silence says. A fixture
/// cannot agree with it by naming it: [`ingest_declensions_are_stated`]
/// compares the text, so the constant named is the constant inherited.
pub const UNSTATED_MID_GROUP_FAULT: &str = "this fixture cannot make its store fail between two rows of one ingest \
     group; nothing in the port reaches inside an `ingest`, so the injection has \
     to come from the adapter and this one has none to offer";

/// CF-18's machine-checked half for the ingest family: no capability `F`
/// declines carries the reason it would inherit by saying nothing.
///
/// A declined capability's reason is printed on every run as the adapter's
/// account of its own store, and a default written by this crate says the same
/// thing about every store. `happenstance_testkit::assert_declensions_are_stated`
/// is the event-store family's copy. Emitted once per mounted suite by
/// [`ingest_store_conformance!`](crate::ingest_store_conformance), not by the
/// emitter, so a third-party emitter cannot drop it. It cannot see a reason
/// that is merely bad.
///
/// The opener is taken, and never called, so `F` is inferred from the
/// macro's hoisted fixture function.
///
/// # Errors
///
/// A [`Violation`] of CF-18 naming each capability `F` declines with the
/// inherited reason.
pub fn ingest_declensions_are_stated<F: IngestFixture>(
    _opener: impl AsyncFn() -> F,
) -> Result<(), Violation> {
    let declared = [(
        "MID_GROUP_FAULT",
        F::MID_GROUP_FAULT,
        UNSTATED_MID_GROUP_FAULT,
    )];
    let unstated: Vec<&str> = declared
        .iter()
        .filter(|(_, capability, default)| capability.reason() == Some(*default))
        .map(|(name, _, _)| *name)
        .collect();
    ensure(unstated.is_empty(), "CF-18", || {
        format!(
            "this fixture declines {} without saying why: the reason printed on every run is \
             this crate's words, not the adapter's account of its store. Write the constant on \
             the `impl`, with the reason this store cannot co-operate, in your own words",
            unstated.join(", ")
        )
    })
}

/// The reference [`IngestFixture`], over [`MemoryIngestStore`].
///
/// Every ingest rule is proven against this before it is allowed to fail an
/// adapter: a rule red here is a wrong rule, not a wrong store. Each instance
/// owns one fresh store, with an incarnation of its own, and each
/// [`connect`](IngestFixture::connect) is a new handle onto it.
#[derive(Debug, Clone, Default)]
pub struct MemoryIngestFixture {
    /// Shared by every handle, so two handles see one log.
    store: Arc<MemoryIngestStore>,
}

impl MemoryIngestFixture {
    /// A fixture over a fresh in-memory store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl IngestFixture for MemoryIngestFixture {
    type Store = MemoryIngestHandle;

    /// Declined, by incapacity. The oracle's store error is uninhabited and
    /// its ingest stages every row before one `extend`, so there is no write
    /// between two rows for a fault to land in, and adding one would put a
    /// mistake switch in the reference (TST-09).
    const MID_GROUP_FAULT: Capability = Capability::declined(
        "the in-memory oracle stages every row of a call and then extends its log once, and its \
         store error cannot be constructed, so no fault can land between two rows of a group; \
         SY-30's rule is proven against the faithful wrong store and the SQLite spike instead",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        // Ready rather than `async`: a handle is a refcount bump, and this
        // says so where a real fixture's `connect` does I/O.
        core::future::ready(MemoryIngestHandle {
            store: Arc::clone(&self.store),
        })
    }
}

/// One handle onto a [`MemoryIngestFixture`]'s store.
///
/// A type of its own because the orphan rule refuses `impl EventStore for
/// Arc<MemoryIngestStore>` in this crate: both the trait and `Arc` are
/// foreign, and `Arc` is not `#[fundamental]`. `Arc` because the store is
/// `Send + Sync` and the handle keeps it so, which is what lets it run under
/// the tokio emitter.
#[derive(Debug, Clone)]
pub struct MemoryIngestHandle {
    store: Arc<MemoryIngestStore>,
}

impl SendEventStore for MemoryIngestHandle {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        SendEventStore::read(&*self.store, query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        SendEventStore::append(&*self.store, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        SendEventStore::head(&*self.store).await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        SendEventStore::contains_event_id(&*self.store, id).await
    }
}

impl SendIngestStore for MemoryIngestHandle {
    type Error = MemoryStoreError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        SendIngestStore::store_id(&*self.store).await
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        SendIngestStore::ingest(&*self.store, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        SendIngestStore::watermark(&*self.store).await
    }
}

#[cfg(test)]
mod tests {
    use core::future::Future;

    use happenstance_testkit::Capability;

    use super::{IngestFixture, MemoryIngestHandle, ingest_declensions_are_stated};
    use crate::MemoryIngestFixture;

    /// A fixture that declares nothing, so every item is the trait's default.
    struct Silent(MemoryIngestFixture);

    impl IngestFixture for Silent {
        type Store = MemoryIngestHandle;

        fn connect(&self) -> impl Future<Output = Self::Store> {
            self.0.connect()
        }
    }

    #[test]
    fn the_default_arm_reports_a_violation_of_sy_30() {
        let armed = happenstance_testkit::block_on(
            Silent(MemoryIngestFixture::new()).arm_mid_group_fault(2),
        );

        assert_eq!(armed.map_err(|violation| violation.clause()), Err("SY-30"));
    }

    #[test]
    fn a_fixture_that_inherits_the_default_declension_is_rejected() {
        let checked = ingest_declensions_are_stated(async || Silent(MemoryIngestFixture::new()));

        assert_eq!(
            checked.map_err(|violation| violation.clause()),
            Err("CF-18")
        );
    }

    /// One that supports the fault, and one that declines it in its own words.
    struct Supports(MemoryIngestFixture);
    struct States(MemoryIngestFixture);

    impl IngestFixture for Supports {
        type Store = MemoryIngestHandle;
        const MID_GROUP_FAULT: Capability = Capability::SUPPORTED;

        fn connect(&self) -> impl Future<Output = Self::Store> {
            self.0.connect()
        }
    }

    impl IngestFixture for States {
        type Store = MemoryIngestHandle;
        const MID_GROUP_FAULT: Capability =
            Capability::declined("a test fixture over the oracle, which has no fault seam");

        fn connect(&self) -> impl Future<Output = Self::Store> {
            self.0.connect()
        }
    }

    #[test]
    fn a_supported_or_stated_capability_passes() {
        assert_eq!(
            ingest_declensions_are_stated(async || Supports(MemoryIngestFixture::new())),
            Ok(())
        );
        assert_eq!(
            ingest_declensions_are_stated(async || States(MemoryIngestFixture::new())),
            Ok(())
        );
    }
}
