//! The other direction of CF-42: a fixture that supports read-your-own-writes
//! is never asked to settle.
//!
//! [`NeverAsked`] forwards everything to the fixture it wraps, and its
//! `settle_frontier` panics. A rule that called the hook on a fixture declaring
//! `READ_YOUR_OWN_WRITES` supported would fail here, which is what holds such a
//! fixture's run bit-identical to the run before the hook existed (H-12).

use happenstance_testkit::{Armed, Capability, Fixture, Settled};

use crate::harness::Subject;

/// Any fixture `F`, whose `settle_frontier` refuses to be called.
#[derive(Debug)]
pub(crate) struct NeverAsked<F>(F);

impl<F: Fixture> Fixture for NeverAsked<F> {
    type Store = F::Store;

    const SECOND_HANDLE: Capability = F::SECOND_HANDLE;
    const REOPEN: Capability = F::REOPEN;
    const READ_YOUR_OWN_WRITES: Capability = F::READ_YOUR_OWN_WRITES;
    const MID_BATCH_FAULT: Capability = F::MID_BATCH_FAULT;
    const READ_FAULT: Capability = F::READ_FAULT;
    const MAX_EVENT_DATA_LEN: Option<usize> = F::MAX_EVENT_DATA_LEN;
    const MAX_TAGS_PER_EVENT: Option<usize> = F::MAX_TAGS_PER_EVENT;
    const MAX_EVENTS_PER_BATCH: Option<usize> = F::MAX_EVENTS_PER_BATCH;
    const MAX_METADATA_LEN: Option<usize> = F::MAX_METADATA_LEN;
    const HELD_ABOVE_FRONTIER: Capability = F::HELD_ABOVE_FRONTIER;

    fn connect(&self) -> impl Future<Output = Self::Store> {
        self.0.connect()
    }

    fn reopen(&self) -> impl Future<Output = ()> {
        self.0.reopen()
    }

    fn arm_mid_batch_fault(&self, after: usize) -> impl Future<Output = ()> {
        self.0.arm_mid_batch_fault(after)
    }

    fn arm_read_fault(&self) -> impl Future<Output = ()> {
        self.0.arm_read_fault()
    }

    fn arm_held_above_frontier(&self) -> impl Future<Output = Armed> {
        self.0.arm_held_above_frontier()
    }

    async fn settle_frontier(&self) -> Settled {
        panic!(
            "a rule asked a fixture that supports READ_YOUR_OWN_WRITES to settle its \
             frontier; CF-42 settles only on a fixture that declines it"
        )
    }
}

impl<F: Subject> Subject for NeverAsked<F> {
    const NAME: &'static str = F::NAME;

    fn open() -> Self {
        Self(F::open())
    }
}
