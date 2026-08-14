//! The mechanism: how a projection store is driven through every registered
//! projection rule, and how a rule's failure is turned back into data instead of
//! a dead test binary.
//!
//! It is the event-store family's `tests/mutation_coverage/harness.rs` with one
//! port swapped, and it is a **second copy rather than a shared module** for the
//! reason the rest of the projection family is a second copy: the only thing
//! that differs is the trait a subject is looked up on
//! ([`ProjectionFixture`], not `Fixture`), and `Fixture` is a *supertrait* of the
//! event-store `Subject`, so no amount of parameterisation lets one trait serve
//! both. Sharing the file would also drag `probes`, `model_probes`,
//! `concurrency_probes` and the event-store `declines` into this binary, where
//! nothing calls them and `-D warnings` turns each one into a build failure.
//!
//! Nothing here knows what any rule means. It knows three things, and they are
//! the same three:
//!
//! 1. **Dispatch is rule-by-name via the macro, store-by-type via one generic
//!    function.** [`projection_probes`] is that function. A table of
//!    `&dyn ProjectionFixture` does not compile — `ProjectionStore` is not
//!    dyn-compatible — and boxing each rule's future to `Pin<Box<dyn Future>>`
//!    to store them uniformly would force this binary to pick `Send` or `!Send`,
//!    which is the choice ADR-0001 refuses.
//! 2. **A failing rule panics**, so observing "did this rule fail" means
//!    [`std::panic::catch_unwind`] and a panic hook that does not spray expected
//!    backtraces across a passing run.
//! 3. **There are three outcomes, not two** — see [`Verdict`].

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::panic;
use std::path::Path;
use std::sync::Once;

use happenstance_testkit::{ProjectionFixture, RuleOutcome};

// =====================================================================
// What a store must supply to be driven
// =====================================================================

/// A [`ProjectionFixture`] this binary can open by name, with no arguments.
///
/// `ProjectionFixture` alone is not enough: the meta-tests need to *construct*
/// the thing, and they need a string to match against [`REGISTRY`]. Both are
/// associated items rather than constructor arguments because the whole point of
/// [`projection_probes`] is that a store is supplied as a **type parameter** —
/// there is no value to pass an argument to.
///
/// Implemented on the fixture, while [`NAME`](ProjectionSubject::NAME) names the
/// *store*. That is deliberate and inherited: the registry is a map from a defect
/// to the rules that catch it, and the defect lives in the store. The fixture is
/// only how the store is opened.
///
/// [`REGISTRY`]: crate::REGISTRY
pub(crate) trait ProjectionSubject: ProjectionFixture + Sized {
    /// The registry key. Matches the `name` field of this store's `Declared`
    /// entry in `projection_mutation_coverage.rs`.
    const NAME: &'static str;

    /// A fresh, isolated backing projection store.
    ///
    /// Called **inside** the caught closure, never hoisted out of it — see
    /// [`run_probe`] for why that is load-bearing rather than incidental.
    fn open() -> Self;
}

/// The opener every projection rule is handed.
///
/// A free `async fn` rather than a closure, because a rule takes
/// `impl AsyncFn() -> F` and an `async fn` *item* satisfies that as a zero-sized
/// value: `open_subject::<S>` can therefore be passed by value into every
/// registered rule with no `&open` borrow, no `Copy` bound and no higher-ranked
/// obligation over the reference type.
///
/// It is also what makes a rule able to ask for **two isolated stores** — the
/// affordance `commit_rejects_a_foreign_batch` spends.
async fn open_subject<S: ProjectionSubject>() -> S {
    S::open()
}

// =====================================================================
// Three outcomes, not two
// =====================================================================

/// What running one projection rule against one store produced.
///
/// A rule can `Passed`, `Skipped`, or panic, and **conflating a skip with either
/// of the others breaks CF-3's second direction**. A store whose fixture
/// declines a capability would otherwise read as "passes the rule that needed
/// it" when the rule never executed a line — which is the exact vacuity this
/// binary exists to remove, reintroduced in the instrument that was supposed to
/// detect it.
#[derive(Debug)]
pub(crate) enum Verdict {
    /// The rule ran and every assertion in it held.
    Passed,
    /// The rule required a capability the fixture declines, and did nothing.
    Skipped {
        /// The `ProjectionFixture` associated const, e.g. `"RESET_REFUSAL"`.
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
    /// A one-line rendering for an assertion message.
    ///
    /// Every meta-test failure in this binary quotes it, so a reader is never
    /// left to re-derive which of the *N × M* `(store, rule)` cells went wrong.
    pub(crate) fn describe(&self) -> String {
        match self {
            Self::Passed => "passed".to_owned(),
            Self::Skipped { capability, reason } => {
                format!("skipped (`{capability}` declined: {reason})")
            }
            Self::Panicked { message, origin } => match origin {
                Some(origin) => format!("panicked at {}:{}: {message}", origin.file, origin.line),
                None => format!("panicked: {message}"),
            },
        }
    }
}

/// Where a panic was raised.
///
/// Captured from [`std::panic::PanicHookInfo::location`] rather than from the
/// payload, because `catch_unwind` hands back only the payload — the location
/// exists solely for the duration of the hook call, which is why the hook writes
/// it into a thread-local for [`run_probe`] to pick up afterwards.
#[derive(Debug, Clone)]
pub(crate) struct Origin {
    /// The source file, as rustc recorded it.
    pub(crate) file: String,
    /// The line within it.
    pub(crate) line: u32,
}

impl Origin {
    /// Whether this panic came out of the projection family's own rule bodies.
    ///
    /// **This is what makes `FailureMode::Assertion` a positive claim rather than
    /// a denylist**, and the difference is not academic. Every projection rule,
    /// and every helper that panics on a rule's behalf (`commit_ok`,
    /// `checkpoint_ok`, `probe_read_ok`, and the `must!` gate), lives in exactly
    /// one file: `happenstance-testkit/src/projection.rs`. `assert!` and
    /// `assert_eq!` record the *call site*, and so do the `#[track_caller]`
    /// panics of `Option::unwrap`, slice indexing and arithmetic overflow — so a
    /// store that falls over anywhere in `tests/projection_mutation_coverage/`
    /// reports its own file and is rejected here.
    ///
    /// # Why this matches three components and the event-store sibling matches
    /// one
    ///
    /// `Origin::is_in` over there compares the *leaf* only, because the path
    /// rustc records is relative and platform-shaped (`crates\…` here, `crates/…`
    /// on CI) and `suite.rs` is unique in the workspace. `projection.rs` is
    /// **not**: `happenstance-core` has one too, holding the port, the probe and
    /// a `#[cfg(test)]` module. A leaf-only match would classify a panic raised
    /// in the *contract crate* as a projection rule rejecting the store, which is
    /// precisely the substitution CF-2 forbids — so the tail of the path is
    /// compared instead, which is still separator-agnostic because
    /// [`Path::components`] splits on whichever separator the host uses to write
    /// them.
    pub(crate) fn is_a_projection_rule_body(&self) -> bool {
        /// The last three path components of the projection family's rules file.
        const TAIL: [&str; 3] = ["happenstance-testkit", "src", "projection.rs"];

        let mut tail: Vec<&str> = Path::new(&self.file)
            .components()
            .rev()
            .take(TAIL.len())
            .filter_map(|component| component.as_os_str().to_str())
            .collect();
        tail.reverse();
        tail == TAIL
    }
}

/// Panic messages that mean *the store fell over*, not *the rule rejected it*.
///
/// # This is the second of two checks, and it is the weaker one
///
/// [`Origin::is_a_projection_rule_body`] is the primary: a panic raised outside
/// `projection.rs` is not a rule's assertion, whatever it says. This list catches
/// the residue that check cannot see — a standard-library panic raised *inside* a
/// rule body, where the location is `projection.rs` and the rule nonetheless fell
/// over rather than asserting.
///
/// **A denylist can never be the primary check.** It accepts any message not on
/// it, including every custom `panic!("…")` string a store can produce, so a
/// mutant whose modelled defect had been deleted outright would still read as
/// proof. The event-store family measured exactly that and this file inherits the
/// conclusion rather than re-learning it
/// (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:206-246`).
pub(crate) const RUNTIME_PANICS: &[&str] = &[
    "index out of bounds",
    "already borrowed",
    "already mutably borrowed",
    "called `Option::unwrap()`",
    "called `Result::unwrap()`",
    "attempt to add with overflow",
    "attempt to subtract with overflow",
    "attempt to multiply with overflow",
    "attempt to divide by zero",
    "range end index",
    "range start index",
    "slice index starts at",
    "capacity overflow",
    "not yet implemented",
    "internal error: entered unreachable code",
];

// =====================================================================
// The panic hook
// =====================================================================

thread_local! {
    /// Whether a panic on *this* thread is one we are deliberately provoking.
    static SUPPRESSED: Cell<bool> = const { Cell::new(false) };

    /// Where the most recent panic on *this* thread was raised.
    ///
    /// The hook is the only place a panic's location is observable —
    /// `catch_unwind` returns the payload and nothing else — so it is stashed
    /// here and read back by [`run_probe`] a few instructions later, on the same
    /// thread. Thread-local rather than global for the same reason [`SUPPRESSED`]
    /// is: libtest runs these meta-tests concurrently.
    static LAST_ORIGIN: RefCell<Option<Origin>> = const { RefCell::new(None) };
}

/// Installs a hook that stays silent for deliberately-provoked panics.
///
/// # Why `Once`, and why the hook is never restored
///
/// `set_hook` / `take_hook` are **process-global**, and libtest runs this
/// binary's tests as concurrent threads. Bracketing each `catch_unwind` in a
/// take/set pair therefore races the other meta-tests: it swallows a genuine
/// failure's message on another thread, and if two brackets interleave it can
/// leave the wrong hook installed for the rest of the run. So the hook is
/// installed once and the *suppression* is thread-local, which is the part that
/// legitimately differs per test.
///
/// The previous hook is captured and called, so an unexpected panic anywhere
/// still prints exactly what it would have printed.
fn install_quiet_hook() {
    static INSTALLED: Once = Once::new();

    INSTALLED.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            // Recorded unconditionally, before the suppression branch: the
            // location is what `FailureMode::Assertion` is checked against, and a
            // hook that recorded it only for suppressed panics would be one
            // refactor away from recording nothing.
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

/// One projection rule, ready to run against one already-chosen store type.
///
/// `run` is a **function pointer**, not a boxed closure, and that is what keeps
/// the whole harness free of `AssertUnwindSafe`. A non-capturing closure coerces
/// to `fn()`, function pointers are unconditionally
/// [`UnwindSafe`](std::panic::UnwindSafe), and so [`run_probe`] can hand one
/// straight to `catch_unwind` with nothing asserted away.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Probe {
    /// The rule's name, from `for_each_projection_store_rule!`.
    pub(crate) name: &'static str,
    /// Runs the rule once against a freshly-opened fixture.
    pub(crate) run: fn() -> RuleOutcome,
}

/// Every registered projection rule, bound to `S`.
///
/// The emitter macro is defined **inside** this function so that it can mention
/// `S`. `macro_rules!` hygiene applies to local variables, not to type
/// parameters, so `S` in the expansion resolves to this function's `S` — the same
/// mechanism `projection_store_conformance!` uses to let one macro's expansion
/// define `__conformance_fixture` and another's call it.
///
/// The rule universe comes from `for_each_projection_store_rule!` and from
/// nowhere else. There is no second list of rule names in this binary, on
/// purpose: a hand-kept list would make CF-1 go green on the day a rule is added,
/// which is the day it must go red.
fn projection_probes<S: ProjectionSubject>() -> Vec<Probe> {
    macro_rules! probe {
        ($($rule:ident),* $(,)?) => {
            ::std::vec![ $(
                Probe {
                    name: ::core::stringify!($rule),
                    // Non-capturing: it mentions `S`, which is a generic
                    // parameter of the enclosing function rather than a capture,
                    // so the closure still coerces to `fn() -> RuleOutcome`.
                    run: || happenstance_testkit::block_on(
                        happenstance_testkit::projection::rules::$rule(open_subject::<S>)
                    ),
                }
            ),* ]
        };
    }

    happenstance_testkit::for_each_projection_store_rule!(probe)
}

/// Runs one probe, converting a panic into a [`Verdict`].
///
/// # The fixture is constructed inside the caught closure
///
/// `catch_unwind`'s bound is `F: FnOnce() -> R + UnwindSafe`, and it constrains
/// the closure's **captures only** — everything created inside it (the fixture,
/// the pinned future inside `block_on`, the store) is invisible to the check. A
/// [`Probe`]'s `run` captures nothing at all, so the check passes trivially and
/// still means something.
///
/// **The first refactor anyone attempts will break this.** Hoisting an
/// `Rc`-backed fixture out and passing it in makes the capture a `&Fixture`,
/// which is `!UnwindSafe`, and the compiler's suggestion is `AssertUnwindSafe` —
/// which asserts away the very check that made the harness safe to reuse a
/// fixture after a panic. Do not. Likewise `catch_unwind(|| results.push(…))`
/// captures `&mut Vec<_>` and needs the same assertion; this function returns the
/// verdict so the push happens outside.
fn run_probe(probe: Probe) -> Verdict {
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
            // Cleared above and written by the hook, so this is the location of
            // *this* panic or nothing at all — never a stale one from the
            // previous probe.
            origin: LAST_ORIGIN.with_borrow_mut(Option::take),
        },
    }
}

/// Every projection rule's verdict against one store.
#[derive(Debug)]
pub(crate) struct SubjectReport {
    /// The store's [`ProjectionSubject::NAME`].
    pub(crate) name: &'static str,
    /// One entry per registered projection rule, in enumeration order.
    pub(crate) outcomes: Vec<(&'static str, Verdict)>,
    /// `(capability, reason)` for every capability this fixture declines.
    ///
    /// Collected so that CF-3's undeclared half can *check* a skip instead of
    /// waving it through. A [`Verdict::Skipped`] is neither a pass nor a failure,
    /// and the only thing that makes it honest is that the fixture said in
    /// advance it could not do the thing; a skip whose `(capability, reason)`
    /// pair is not on this list came from somewhere else, which means a rule
    /// quietly stopped running.
    pub(crate) declines: Vec<(&'static str, &'static str)>,
}

/// Drives every projection rule against `S` and collects the verdicts.
pub(crate) fn run_subject<S: ProjectionSubject>() -> SubjectReport {
    let outcomes = projection_probes::<S>()
        .into_iter()
        // The push happens here, outside `catch_unwind`, so no `&mut Vec<_>` is
        // ever captured by the caught closure.
        .map(|probe| (probe.name, run_probe(probe)))
        .collect();

    SubjectReport {
        name: S::NAME,
        outcomes,
        declines: declines::<S>(),
    }
}

/// Every projection capability `S` declines, with the reason it gave.
///
/// The names are written out because a trait's associated items cannot be
/// enumerated by a macro from outside it. That makes this the one list in the
/// binary with no mechanical backstop — so it is kept beside the trait it mirrors
/// rather than in the registry, and a capability that arrives on
/// [`ProjectionFixture`] without a line here shows up as a skip the meta-tests
/// cannot account for, which is a loud failure rather than a silent one.
///
/// There is no projection counterpart to the event-store family's
/// `NO_STORE_LIMITS` branch, and that is a property of the port rather than an
/// omission: `CommitError` and `ResetError` carry no capacity variant, so the
/// projection family declares no numeric-limit constants and never reaches the
/// surface CF-40 is about.
fn declines<S: ProjectionSubject>() -> Vec<(&'static str, &'static str)> {
    [
        S::SECOND_HANDLE
            .reason()
            .map(|reason| ("SECOND_HANDLE", reason)),
        S::RESET_REFUSAL
            .reason()
            .map(|reason| ("RESET_REFUSAL", reason)),
        S::COMMIT_FAULT
            .reason()
            .map(|reason| ("COMMIT_FAULT", reason)),
    ]
    .into_iter()
    .flatten()
    .collect()
}
