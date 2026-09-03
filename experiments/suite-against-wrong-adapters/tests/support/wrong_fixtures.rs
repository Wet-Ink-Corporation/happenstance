//! The ten subjects: two controls, four wrong implementations, four injected
//! controls.
//!
//! A fixture is how a store is *opened*, and two of the ten carry their whole
//! defect here rather than in any `EventStore` impl — which is why the registry
//! is a map from a defect to the rules that catch it, and `NAME` names the thing
//! under measurement rather than the type this trait is implemented on.

use std::cell::Cell;
use std::rc::Rc;

use happenstance_testkit::{Capability, Fixture, fixtures::MemoryFixture};
use suite_against_wrong_adapters::correct::{Log as CorrectLog, LogStore, dense};
use suite_against_wrong_adapters::stores::{
    ForwardPagingBudgetStore, Log, PlainStore, StagedCommitStore, SwallowedReadFaultStore,
    empty_log,
};

use super::harness::{Kind, Subject};

/// The reason every volatile fixture here declines `REOPEN`.
const NO_MEDIUM: &str = "a Vec behind an Rc, with no durable medium to reopen over — this experiment's \
     axis is what the rule set certifies, not durability";

// =====================================================================
// Control 1 — the testkit's own reference fixture
// =====================================================================

/// `MemoryFixture` over `MemoryEventStore`, registered unchanged.
///
/// The first of the two conformant controls, and the stronger one: it is the
/// reference implementation the testkit *publishes*, not something this crate
/// wrote. If it fails a rule in this run, the run says nothing about the four.
impl Subject for MemoryFixture {
    const NAME: &'static str = "MemoryEventStore (reference)";
    const KIND: Kind = Kind::Control;

    fn open() -> Self {
        Self::new()
    }
}

// =====================================================================
// Control 2 — the correct core the four are one step from
// =====================================================================

/// `LogStore`, the completely correct store in [`suite_against_wrong_adapters::correct`].
///
/// The second control, and it is the one that matters for the reading: the four
/// stores below are this store with one step changed. If *it* fails something,
/// every "fails nothing" beneath is a comparison against a broken baseline.
#[derive(Debug)]
pub struct CorrectFixture(Rc<std::cell::RefCell<CorrectLog>>);

impl Fixture for CorrectFixture {
    type Store = LogStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(NO_MEDIUM);

    async fn connect(&self) -> Self::Store {
        LogStore::over(&self.0)
    }
}

impl Subject for CorrectFixture {
    const NAME: &'static str = "LogStore (correct core)";
    const KIND: Kind = Kind::Control;

    fn open() -> Self {
        Self(Rc::new(std::cell::RefCell::new(CorrectLog::new(dense))))
    }
}

// =====================================================================
// 1 — L1-1: the forward paging budget that is never spent
// =====================================================================

/// One log, and any number of forward-paging-budget handles onto it.
#[derive(Debug)]
pub struct ForwardPagingBudgetFixture<const INJECTED: bool>(Log);

impl<const INJECTED: bool> Fixture for ForwardPagingBudgetFixture<INJECTED> {
    type Store = ForwardPagingBudgetStore<INJECTED>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(NO_MEDIUM);

    async fn connect(&self) -> Self::Store {
        ForwardPagingBudgetStore::over(&self.0)
    }
}

impl<const INJECTED: bool> Subject for ForwardPagingBudgetFixture<INJECTED> {
    const NAME: &'static str = if INJECTED {
        "ForwardPagingBudgetStore + InnerJoin"
    } else {
        "ForwardPagingBudgetStore"
    };
    const KIND: Kind = if INJECTED {
        Kind::InjectedControl
    } else {
        Kind::Mutant
    };

    fn open() -> Self {
        Self(empty_log())
    }
}

// =====================================================================
// 2 — L1-2: REOPEN declared supported, honoured by an empty body
// =====================================================================

/// A fixture that declares `REOPEN` supported and overrides it with an **empty
/// body**, over a completely correct but entirely volatile store.
///
/// Not a forgotten override — that reaches the trait's provided body, which
/// panics and names this exact mistake, and is a different outcome. The author
/// who writes this one is the author of a real adapter whose pool "handles
/// reconnection", who reads `reopen`'s doc as being about *handles* rather than
/// about the *medium*, and who writes the honest-looking answer.
///
/// The three rules that then run against a live in-process `Vec` and pass —
/// `acknowledged_writes_survive_a_reopen`, `reopened_store_does_not_reissue_an_event_id`,
/// `recorded_time_survives_a_reopen` — are the whole of the suite's durability
/// certification.
#[derive(Debug)]
pub struct NoopReopenFixture<const INJECTED: bool>(Log);

impl<const INJECTED: bool> Fixture for NoopReopenFixture<INJECTED> {
    type Store = PlainStore<INJECTED>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        PlainStore::over(&self.0)
    }

    // THE DEFECT. Nothing is closed, nothing is reopened, and nothing in the
    // trait ties `REOPEN: Capability::SUPPORTED` to doing either.
    async fn reopen(&self) {}
}

impl<const INJECTED: bool> Subject for NoopReopenFixture<INJECTED> {
    const NAME: &'static str = if INJECTED {
        "NoopReopenFixture + InnerJoin"
    } else {
        "NoopReopenFixture"
    };
    const KIND: Kind = if INJECTED {
        Kind::InjectedControl
    } else {
        Kind::Mutant
    };

    fn open() -> Self {
        Self(empty_log())
    }
}

// =====================================================================
// 3 — L3-01: a fetch failure reported as end-of-stream
// =====================================================================

/// One log, and any number of swallowing paged handles onto it.
///
/// [`arm_read_fault`](Self::arm_read_fault) is an **inherent** method, not a
/// `Fixture` one, and that is the finding rather than an omission: `Fixture` has
/// `MID_BATCH_FAULT` / `arm_mid_batch_fault` for the write path and no read-path
/// analogue, so there is no seam through which any conformance rule could arm
/// this. `tests/defect_is_real.rs` arms it by hand.
#[derive(Debug)]
pub struct SwallowedReadFaultFixture<const INJECTED: bool> {
    log: Log,
    fault_after: Rc<Cell<Option<usize>>>,
}

impl<const INJECTED: bool> SwallowedReadFaultFixture<INJECTED> {
    /// Makes the `pages`-th page fetch of every subsequent read fail.
    ///
    /// No conformance rule can call this. That is the point.
    pub fn arm_read_fault(&self, pages: usize) {
        self.fault_after.set(Some(pages));
    }
}

impl<const INJECTED: bool> Fixture for SwallowedReadFaultFixture<INJECTED> {
    type Store = SwallowedReadFaultStore<INJECTED>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(NO_MEDIUM);

    async fn connect(&self) -> Self::Store {
        SwallowedReadFaultStore::over(&self.log, &self.fault_after)
    }
}

impl<const INJECTED: bool> Subject for SwallowedReadFaultFixture<INJECTED> {
    const NAME: &'static str = if INJECTED {
        "SwallowedReadFaultStore + InnerJoin"
    } else {
        "SwallowedReadFaultStore"
    };
    const KIND: Kind = if INJECTED {
        Kind::InjectedControl
    } else {
        Kind::Mutant
    };

    fn open() -> Self {
        Self {
            log: empty_log(),
            fault_after: Rc::new(Cell::new(None)),
        }
    }
}

// =====================================================================
// 4 — F2-5: a store that genuinely suspends inside `append`
// =====================================================================

/// One log, and any number of staged-commit handles onto it.
#[derive(Debug)]
pub struct StagedCommitFixture<const INJECTED: bool>(Log);

impl<const INJECTED: bool> Fixture for StagedCommitFixture<INJECTED> {
    type Store = StagedCommitStore<INJECTED>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(NO_MEDIUM);

    async fn connect(&self) -> Self::Store {
        StagedCommitStore::over(&self.0)
    }
}

impl<const INJECTED: bool> Subject for StagedCommitFixture<INJECTED> {
    const NAME: &'static str = if INJECTED {
        "StagedCommitStore + InnerJoin"
    } else {
        "StagedCommitStore"
    };
    const KIND: Kind = if INJECTED {
        Kind::InjectedControl
    } else {
        Kind::Mutant
    };

    fn open() -> Self {
        Self(empty_log())
    }
}

// =====================================================================
// A diagnostic, not one of the four
// =====================================================================

/// [`SwallowedReadFaultFixture`] with the fault **already armed** at `open`.
///
/// Registered to separate two readings that "fails nothing" would otherwise
/// merge:
///
/// * *the suite cannot see this class of defect at all* — every consequence of a
///   swallowed read fault is invisible through the port; and
/// * *the suite has no way to make it happen* — the consequence is plainly
///   visible, and nothing in `Fixture` can trigger the fault that produces it.
///
/// Only the second is true, and this row is the proof. It is not counted among
/// the four, because no adapter author ships a store whose every read fails: it
/// is the same defect with the trigger pulled by hand, which is exactly what the
/// missing capability would let a rule do.
#[derive(Debug)]
pub struct ArmedReadFaultFixture {
    log: Log,
    fault_after: Rc<Cell<Option<usize>>>,
}

impl Fixture for ArmedReadFaultFixture {
    type Store = SwallowedReadFaultStore<false>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(NO_MEDIUM);

    async fn connect(&self) -> Self::Store {
        SwallowedReadFaultStore::over(&self.log, &self.fault_after)
    }
}

impl Subject for ArmedReadFaultFixture {
    const NAME: &'static str = "SwallowedReadFaultStore (fault armed by hand)";
    const KIND: Kind = Kind::Diagnostic;

    fn open() -> Self {
        Self {
            log: empty_log(),
            // The second page fetch of every read fails, and is reported as the
            // end of the stream.
            fault_after: Rc::new(Cell::new(Some(1))),
        }
    }
}
