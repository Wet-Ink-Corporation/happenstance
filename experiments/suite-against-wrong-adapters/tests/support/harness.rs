//! How a store is driven through every registered rule, and how a rule's failure
//! is turned back into data.
//!
//! Adapted from `crates/happenstance-testkit/tests/mutation_coverage/harness.rs`
//! at `56ef6c5`. It is **not** a copy: that file is inside the testkit's own test
//! binary and reaches nothing this crate cannot. Everything here goes through the
//! testkit's *published* surface — [`happenstance_testkit::for_each_event_store_rule!`],
//! [`happenstance_testkit::rules`], [`happenstance_testkit::block_on`],
//! [`happenstance_testkit::RuleOutcome`] — which is the point: an adapter author
//! outside this repository could write it, and the rule set it enumerates is the
//! same one `event_store_conformance!` expands.
//!
//! Three things it knows, and it knows nothing else:
//!
//! 1. **Dispatch is rule-by-name via the macro, store-by-type via one generic
//!    function.** A `&[(&dyn Fixture, ..)]` table does not compile — `EventStore`
//!    is not dyn-compatible, and boxing each rule's future would force the
//!    registry to pick `Send` or `!Send`, which is the choice ADR-0001 refuses.
//! 2. **A failing rule panics**, so observing "did this rule fail" means
//!    [`std::panic::catch_unwind`] plus a hook that does not spray expected
//!    backtraces across a passing run.
//! 3. **There are three outcomes, not two** — a skip is neither a pass nor a
//!    failure, and conflating it with either is the exact vacuity this experiment
//!    is measuring, reintroduced in the instrument that measures it.
//!
//! # No watchdog
//!
//! One store here suspends inside `append` on purpose. If a future never wakes,
//! this binary hangs rather than failing — and a wall-clock deadline inside the
//! instrument is a flake inside the instrument the first time a runner is loaded,
//! which is CF-33's own argument. The mitigation is structural: the suspending
//! store's `yield_once` signals the waker *before* returning `Pending`.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::panic;
use std::sync::Once;

use happenstance_testkit::{Capability, Fixture, RuleOutcome};

// =====================================================================
// What a store must supply to be driven
// =====================================================================

/// A [`Fixture`] this binary can open by name, with no arguments.
pub trait Subject: Fixture + Sized {
    /// The name this store is reported under.
    const NAME: &'static str;

    /// What this subject is: a control, or one of the four.
    const KIND: Kind;

    /// A fresh, isolated backing store.
    ///
    /// Called **inside** the caught closure, never hoisted out of it — see
    /// [`run_probe`].
    fn open() -> Self;
}

/// What a subject is registered as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A store believed correct. It must fail nothing.
    Control,
    /// One of the four wrong implementations under measurement.
    Mutant,
    /// The same store with a second, unrelated defect injected. It must fail
    /// something, or the "fails nothing" reading above it means nothing.
    InjectedControl,
    /// A variant registered to sharpen the reading rather than to be counted.
    /// Asserted in neither direction; its row is the whole of its contribution.
    Diagnostic,
}

/// The opener every rule is handed.
///
/// A free `async fn` rather than a closure, because a rule takes
/// `impl AsyncFn() -> F` and an `async fn` *item* satisfies that as a zero-sized
/// value.
async fn open_subject<S: Subject>() -> S {
    S::open()
}

// =====================================================================
// Three outcomes, not two
// =====================================================================

/// What running one rule against one store produced.
#[derive(Debug)]
pub enum Verdict {
    /// The rule ran and every assertion in it held.
    Passed,
    /// The rule required a capability the fixture declines, and did nothing.
    Skipped {
        /// The `Fixture` associated const, e.g. `"REOPEN"`.
        capability: &'static str,
        /// The fixture's own stated reason.
        reason: &'static str,
    },
    /// The rule, or the store under it, panicked.
    Panicked {
        /// The panic payload, as far as it can be recovered as a string.
        message: String,
        /// Where the panic was raised, when the hook could see it.
        origin: Option<Origin>,
    },
}

impl Verdict {
    /// Whether this counts as the store failing the rule.
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Panicked { .. })
    }

    /// Whether the panic came out of the suite's own rule bodies — i.e. whether
    /// the *rule rejected the store* rather than the store falling over.
    ///
    /// Every conformance rule and every helper that panics on its behalf lives in
    /// one file, `happenstance-testkit/src/suite.rs`, and `assert!` records the
    /// call site. So a store that falls over anywhere in this crate reports its
    /// own file and is distinguishable. This is a positive claim rather than a
    /// denylist, which is the mistake the testkit's own harness made first.
    pub fn is_a_rule_rejection(&self) -> bool {
        match self {
            Self::Panicked {
                origin: Some(origin),
                ..
            } => origin.is_in("suite.rs"),
            _ => false,
        }
    }

    /// A one-line rendering.
    pub fn describe(&self) -> String {
        match self {
            Self::Passed => "passed".to_owned(),
            Self::Skipped { capability, reason } => {
                format!("skipped ({capability} declined: {reason})")
            }
            Self::Panicked { message, origin } => match origin {
                Some(origin) => format!("FAILED at {}:{} — {message}", origin.file, origin.line),
                None => format!("FAILED — {message}"),
            },
        }
    }
}

/// Where a panic was raised.
#[derive(Debug, Clone)]
pub struct Origin {
    /// The source file, as rustc recorded it.
    pub file: String,
    /// The line within it.
    pub line: u32,
}

impl Origin {
    /// Whether the panic was raised in the named source file.
    ///
    /// The leaf only: the path rustc records is relative and platform-shaped
    /// (`crates\…` here, `crates/…` on CI), and only the file name is stable.
    fn is_in(&self, file: &str) -> bool {
        std::path::Path::new(&self.file)
            .file_name()
            .is_some_and(|name| name == file)
    }
}

// =====================================================================
// The panic hook
// =====================================================================

thread_local! {
    /// Whether a panic on *this* thread is one we are deliberately provoking.
    static SUPPRESSED: Cell<bool> = const { Cell::new(false) };

    /// Where the most recent panic on *this* thread was raised.
    static LAST_ORIGIN: RefCell<Option<Origin>> = const { RefCell::new(None) };
}

/// Installs a hook that stays silent for deliberately-provoked panics.
///
/// `set_hook` is process-global and libtest runs tests as concurrent threads, so
/// the hook is installed once and the *suppression* is thread-local, which is the
/// part that legitimately differs per test. The previous hook is captured and
/// called, so an unexpected panic still prints what it would have printed.
fn install_quiet_hook() {
    static INSTALLED: Once = Once::new();

    INSTALLED.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            LAST_ORIGIN.with_borrow_mut(|slot| {
                *slot = info.location().map(|location| Origin {
                    file: location.file().to_owned(),
                    line: location.line(),
                });
            });
            if SUPPRESSED.with(Cell::get) {
                return;
            }
            previous(info);
        }));
    });
}

/// Recovers a panic payload as a string, as far as `Any` allows.
fn panic_message(payload: &(dyn Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        return (*message).to_owned();
    }
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    "<panic payload was neither &str nor String>".to_owned()
}

// =====================================================================
// Probes
// =====================================================================

/// One rule, ready to run against one already-chosen store type.
///
/// `run` is a **function pointer**, not a boxed closure, which is what keeps the
/// harness free of `AssertUnwindSafe`: a non-capturing closure coerces to `fn()`,
/// and function pointers are unconditionally `UnwindSafe`.
#[derive(Debug, Clone, Copy)]
pub struct Probe {
    /// The rule's name, from `for_each_event_store_rule!`.
    pub name: &'static str,
    /// Runs the rule once against a freshly-opened fixture.
    pub run: fn() -> RuleOutcome,
}

/// Every registered rule, bound to `S`.
///
/// The emitter macro is defined **inside** this function so that it can mention
/// `S`: `macro_rules!` hygiene applies to local variables, not to type
/// parameters.
pub fn probes<S: Subject>() -> Vec<Probe> {
    macro_rules! probe {
        ($($rule:ident),* $(,)?) => {
            ::std::vec![ $(
                Probe {
                    name: ::core::stringify!($rule),
                    run: || happenstance_testkit::block_on(
                        happenstance_testkit::rules::$rule(open_subject::<S>)
                    ),
                }
            ),* ]
        };
    }

    happenstance_testkit::for_each_event_store_rule!(probe)
}

/// Runs one probe, converting a panic into a [`Verdict`].
///
/// The fixture is constructed **inside** the caught closure. `catch_unwind`'s
/// bound constrains the closure's captures only, and a `Probe`'s `run` captures
/// nothing at all — so the check passes trivially and still means something.
/// Hoisting the fixture out would make the capture a `&Fixture`, which is
/// `!UnwindSafe`, and the compiler's suggestion is `AssertUnwindSafe`, which
/// asserts away the very check that made this safe.
pub fn run_probe(probe: Probe) -> Verdict {
    install_quiet_hook();

    SUPPRESSED.with(|flag| flag.set(true));
    LAST_ORIGIN.with_borrow_mut(|slot| *slot = None);
    let caught = panic::catch_unwind(probe.run);
    SUPPRESSED.with(|flag| flag.set(false));

    match caught {
        Ok(RuleOutcome::Ran) => Verdict::Passed,
        Ok(RuleOutcome::Skipped { capability, reason }) => Verdict::Skipped { capability, reason },
        Err(payload) => Verdict::Panicked {
            message: panic_message(payload.as_ref()),
            origin: LAST_ORIGIN.with_borrow_mut(Option::take),
        },
    }
}

/// Every rule's verdict against one store.
#[derive(Debug)]
pub struct SubjectReport {
    /// The store's [`Subject::NAME`].
    pub name: &'static str,
    /// What it was registered as.
    pub kind: Kind,
    /// One entry per registered rule, in registry order.
    pub outcomes: Vec<(&'static str, Verdict)>,
    /// `(capability, reason)` for every capability this fixture declines.
    pub declines: Vec<(&'static str, &'static str)>,
}

impl SubjectReport {
    /// The names of every rule this store failed.
    pub fn failed(&self) -> Vec<&'static str> {
        self.outcomes
            .iter()
            .filter(|(_, verdict)| verdict.is_failure())
            .map(|(name, _)| *name)
            .collect()
    }

    /// The names of every rule that reported a skip.
    pub fn skipped(&self) -> Vec<&'static str> {
        self.outcomes
            .iter()
            .filter(|(_, verdict)| matches!(verdict, Verdict::Skipped { .. }))
            .map(|(name, _)| *name)
            .collect()
    }

    /// How many rules ran and passed.
    pub fn passed(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|(_, verdict)| matches!(verdict, Verdict::Passed))
            .count()
    }
}

/// Drives every rule against `S` and collects the verdicts.
pub fn run_subject<S: Subject>() -> SubjectReport {
    let outcomes = probes::<S>()
        .into_iter()
        // The push happens here, outside `catch_unwind`, so no `&mut Vec<_>` is
        // ever captured by the caught closure.
        .map(|probe| (probe.name, run_probe(probe)))
        .collect();

    SubjectReport {
        name: S::NAME,
        kind: S::KIND,
        outcomes,
        declines: declines::<S>(),
    }
}

/// Every capability `S` declines, with the reason it gave.
///
/// The names are written out because a trait's associated items cannot be
/// enumerated from outside it. A capability that arrives on `Fixture` without a
/// line here shows up as a skip nothing accounts for — which is a loud failure
/// rather than a silent one.
fn declines<S: Subject>() -> Vec<(&'static str, &'static str)> {
    fn declined(
        name: &'static str,
        capability: Capability,
    ) -> Option<(&'static str, &'static str)> {
        capability.reason().map(|reason| (name, reason))
    }

    let mut declined_items: Vec<(&'static str, &'static str)> = [
        declined("SECOND_HANDLE", S::SECOND_HANDLE),
        declined("REOPEN", S::REOPEN),
        declined("MID_BATCH_FAULT", S::MID_BATCH_FAULT),
    ]
    .into_iter()
    .flatten()
    .collect();

    if S::MAX_EVENT_DATA_LEN.is_none()
        && S::MAX_TAGS_PER_EVENT.is_none()
        && S::MAX_EVENTS_PER_BATCH.is_none()
    {
        declined_items.push((
            happenstance_testkit::NO_STORE_LIMITS,
            happenstance_testkit::NO_CEILING_REASON,
        ));
    }
    declined_items
}
