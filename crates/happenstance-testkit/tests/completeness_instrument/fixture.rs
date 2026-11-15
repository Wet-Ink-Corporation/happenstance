//! `PrunedMemoryFixture`: a store whose whole prior history is withheld.
//!
//! Why the whole history and not a scattered survivor set: every rule assumes
//! it starts on a fresh store, and many read `Query::all()`
//! (`reading_an_empty_store_yields_nothing`, `head_of_an_empty_store_is_none`,
//! `condition_against_an_empty_store_admits_the_append`). A survivor from before
//! the rule would be visible, and would fail those rules for a reason that has
//! nothing to do with pruning. So the strongest staging the suite admits is a
//! store that has forgotten everything it held, and that is CF-27's claim
//! exactly: a pruned store and a young store are the same value at every seam.
//! Holes inside a rule's own history need the rule's cooperation, which is
//! ES-38's and ES-40's rules' to supply.

use core::future::{Future, ready};
use std::collections::BTreeSet;

use happenstance_core::EventStore;
use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle, tagged_event};
use happenstance_testkit::{Capability, Fixture, WithholdingStore};

/// The seeded history: three batches of the event types the suite's rules
/// put in their append conditions.
///
/// Chosen so that the history is *dangerous*: were a withheld event still seen
/// by a condition, rules that expect an append to land would be refused. A
/// history no rule's condition matches would leave the condition rewrite
/// untested by every family but the model.
const SEEDED: [&[&str]; 3] = [
    &["Blocker"],
    &["Existing", "Anything", "First", "A"],
    &[
        "Enrolled",
        "StudentSubscribed",
        "CourseCapacityChanged",
        "Written",
        "Hit",
        "TariffPublished",
        "MeterReadingTaken",
    ],
];

/// The tags those conditions name. Every seeded event carries all of them, so
/// a tag query matches it whatever its type (a superset matches).
const TAGS: &[(&str, &str)] = &[
    ("course", "c1"),
    ("course", "c0"),
    ("student", "s1"),
    ("enrolment", "c1-s1"),
    ("writer", "b0"),
    ("k", "v"),
    ("tariff", "t1"),
    ("meter", "m1"),
];

/// One isolated memory store behind a `WithholdingStore` that withholds
/// everything appended before the rule began.
pub(crate) struct PrunedMemoryFixture {
    store: WithholdingStore<MemoryHandle>,
}

impl PrunedMemoryFixture {
    /// Seeds twelve events in three batches, then withholds every one of them.
    ///
    /// # Panics
    ///
    /// If the instrument reports anything but the positions it was asked to
    /// withhold: the suite would then be measuring a broken fixture.
    pub(crate) async fn pruned() -> Self {
        let memory = MemoryFixture::new();
        let store = WithholdingStore::new(memory.connect().await);
        for batch in SEEDED {
            let events: Vec<_> = batch
                .iter()
                .map(|event_type| tagged_event(event_type, TAGS))
                .collect();
            store
                .append(&events, None)
                .await
                .expect("an unconditional append to a memory store lands");
        }

        let withheld = store
            .withhold_where(|_| true)
            .await
            .expect("a memory store's reads do not fail");

        let seeded: usize = SEEDED.iter().map(|batch| batch.len()).sum();
        assert_eq!(withheld.len(), seeded, "every seeded event is withheld");
        assert!(
            withheld.windows(2).all(|pair| pair[0] < pair[1]),
            "withhold_where returns ascending positions: {withheld:?}"
        );
        assert_eq!(
            store.withheld(),
            withheld.iter().copied().collect::<BTreeSet<_>>(),
            "the report is what was withheld"
        );
        Self { store }
    }
}

impl Fixture for PrunedMemoryFixture {
    type Store = WithholdingStore<MemoryHandle>;

    // Each `connect` is a clone of the instrument, and a clone shares both the
    // inner store and the withheld set.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    const REOPEN: Capability = Capability::declined(
        "MemoryEventStore has no durable medium to reopen over, and the withheld \
         set is process state besides",
    );

    const MID_BATCH_FAULT: Capability = Capability::declined(
        "the inner MemoryEventStore extends one Vec under one lock, so there is no \
         moment between two rows of a batch at which a fault could fire",
    );

    const READ_FAULT: Capability = Capability::declined(
        "the inner MemoryEventStore iterates a Vec it already holds; a read fault \
         through the instrument is shown with FaultyStore as the inner store in \
         tests/withholding_store_instruments.rs",
    );

    // Declined, though `withhold` could answer it: the instrument hides events
    // from a store that still holds them, and `REMOVAL` promises a deletion
    // from the store's own storage (#147). Granting it here would let the
    // guard rule certify a removal nothing performed.
    const REMOVAL: Capability = Capability::declined(
        "the instrument withholds events from a MemoryEventStore that still holds \
         them, which is hiding rather than deleting, and the inner store has no \
         delete of its own",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        // A second handle onto the same instrument: a cheap clone of two `Arc`s.
        ready(self.store.clone())
    }
}
