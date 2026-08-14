//! The mechanism: how a store is driven through every registered rule, and how a
//! rule's failure is turned back into data instead of a dead test binary.
//!
//! Nothing in this file knows what any rule means. It knows three things:
//!
//! 1. **Dispatch is rule-by-name via the macro, store-by-type via one generic
//!    function.** [`probes`] is that function. A `&[(&dyn Fixture, &[&str])]`
//!    table — the shape everyone reaches for first — does not compile:
//!    `EventStore` is not dyn-compatible, and boxing each rule's future to
//!    `Pin<Box<dyn Future>>` to store them uniformly forces the registry to pick
//!    `Send` or `!Send`, which is exactly the choice ADR-0001 refuses, one level
//!    up. `SPECIFICATION.md` §6.4 already writes that obituary for the rule
//!    registry; it applies unchanged to a store registry.
//! 2. **A failing rule panics**, so observing "did this rule fail" means
//!    [`std::panic::catch_unwind`] and a panic hook that does not spray dozens of
//!    expected backtraces across a passing run.
//! 3. **There are three outcomes, not two** — see [`Verdict`].
//!
//! # Hangs are the failure `catch_unwind` cannot rescue
//!
//! `happenstance_testkit::block_on` parks rather than spinning, so a store whose
//! future never wakes hangs this binary until the CI job timeout, with no
//! message and no indication of which rule was running.
//!
//! **The mitigation is structural, not temporal.** Every store in this binary is
//! built from `correct.rs`'s primitives with one step perturbed, and none of
//! those primitives blocks — there is no channel, no I/O and no custom `Future`
//! in the correct core, so a mutant can only hang by introducing one
//! deliberately.
//!
//! There is deliberately **no watchdog**. A wall-clock deadline inside the proof
//! artefact is a flake inside the proof artefact the first time a runner is
//! loaded, which is CF-33's own argument against timing-based conformance rules
//! applied to the meta-tests. Someone will propose one; this paragraph is the
//! answer.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::panic;
use std::sync::Once;

use happenstance_testkit::{Capability, Fixture, ProjectionFixture, RuleOutcome};

// =====================================================================
// What a store must supply to be driven
// =====================================================================

/// A [`Fixture`] this binary can open by name, with no arguments.
///
/// `Fixture` alone is not enough: the meta-tests need to *construct* the thing,
/// and they need a string to match against the registry table. Both are
/// associated items rather than constructor arguments because the whole point of
/// [`probes`] is that a store is supplied as a **type parameter** — there is no
/// value to pass an argument to.
///
/// Implemented on the fixture, while [`NAME`](Subject::NAME) names the *store*.
/// That is deliberate: the registry is a map from a defect to the rules that
/// catch it, and the defect lives in the store. The fixture is only how the
/// store is opened.
pub(crate) trait Subject: Fixture + Sized {
    /// The registry key. Matches the `name` field of this store's `Declared`
    /// entry in `mutation_coverage.rs`.
    const NAME: &'static str;

    /// A fresh, isolated backing store.
    ///
    /// Called **inside** the caught closure, never hoisted out of it — see
    /// [`run_probe`] for why that is load-bearing rather than incidental.
    fn open() -> Self;
}

/// The opener every rule is handed.
///
/// A free `async fn` rather than a closure, because a rule takes
/// `impl AsyncFn() -> F` and an `async fn` *item* satisfies that as a
/// zero-sized value: `open_subject::<S>` can therefore be passed by value into
/// every registered rule with no `&open` borrow, no `Copy` bound and no
/// higher-ranked obligation over the reference type.
async fn open_subject<S: Subject>() -> S {
    S::open()
}

// =====================================================================
// Three outcomes, not two
// =====================================================================

/// What running one rule against one store produced.
///
/// A rule can now `Ran`, `Skipped`, or panic, and **conflating a skip with
/// either of the others breaks CF-3's second direction**. A mutant whose fixture
/// declines `REOPEN` would otherwise read as "passes
/// `acknowledged_writes_survive_a_reopen`" when the rule never executed a line —
/// which is the exact vacuity this whole phase exists to remove, reintroduced in
/// the instrument that was supposed to detect it.
#[derive(Debug)]
pub(crate) enum Verdict {
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
    /// A one-line rendering for an assertion message.
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
    /// Whether this panic came out of the suite's own rule bodies.
    ///
    /// **This is what makes `FailureMode::Assertion` a positive claim rather
    /// than a denylist**, and the difference is not academic. Every conformance
    /// rule, and every helper that panics on the rule's behalf, lives in exactly
    /// one file: `happenstance-testkit/src/suite.rs`. `assert!` and `assert_eq!`
    /// record the *call site*, and so do the `#[track_caller]` panics of
    /// `Option::unwrap`, slice indexing and arithmetic overflow — so a store that
    /// falls over anywhere in `tests/mutation_coverage/` reports its own file and
    /// is rejected here, and so does the `Fixture::reopen` provided body in
    /// `contract.rs`, which a fixture reaches by declaring `REOPEN` supported and
    /// forgetting the override. That last one is measured: deleting
    /// `LosingFixture`'s override removes its modelled defect entirely, and under
    /// the old substring denylist all six meta-tests stayed green.
    pub(crate) fn is_a_rule_body(&self) -> bool {
        self.is_in("suite.rs")
    }

    /// Whether this panic came out of the **model** family's rule body.
    ///
    /// The same positive check one file over. `model.rs` holds the model, the
    /// runner and the single `panic!` that reports a shrunk counterexample, so a
    /// panic located there is the model rejecting the store — and a panic
    /// located anywhere else, including in the mutant's own source, is the store
    /// falling over. The distinction is the whole content of
    /// `the_model_rule_rejects_exactly_what_it_claims`: a store that panics on a
    /// borrow conflict would otherwise be counted as a store the *model* caught.
    pub(crate) fn is_a_model_rule_body(&self) -> bool {
        self.is_in("model.rs")
    }

    /// Whether this panic came out of the **concurrency** family's rule bodies.
    ///
    /// The same positive check one file further over. It is worth one extra
    /// sentence, because this is the family whose rules start threads: a
    /// contender that panics does so on *its own* thread, where the hook records
    /// the origin into that thread's local and nothing reads it back. What
    /// crosses back to the rule's thread is
    /// [`std::panic::resume_unwind`](std::panic::resume_unwind), carrying the
    /// payload and no location — so a store that falls over inside a contender
    /// arrives here with `origin: None` and is classified as
    /// [`RacerOutcome::FellOver`](crate::RacerOutcome::FellOver) rather than as
    /// a rejection. That is the conservative direction, and it is the one that
    /// matters: a store that panics must never read as a store a rule caught.
    pub(crate) fn is_a_concurrency_rule_body(&self) -> bool {
        self.is_in("concurrency.rs")
    }

    /// Whether the panic was raised in the named source file.
    ///
    /// Matching on the file name rather than the full path is deliberate — the
    /// path rustc records is relative and platform-shaped (`crates\…` here,
    /// `crates/…` on CI), and only the leaf is stable across both.
    fn is_in(&self, file: &str) -> bool {
        std::path::Path::new(&self.file)
            .file_name()
            .is_some_and(|name| name == file)
    }
}

/// Panic messages that mean *the store fell over*, not *the rule rejected it*.
///
/// # This is the second of two checks, and it is the weaker one
///
/// [`Origin::is_a_rule_body`] is the primary: a panic raised outside
/// `suite.rs` is not a rule's assertion, whatever it says. This list catches the
/// residue that check cannot see — a standard-library panic raised *inside* a
/// rule body, where the location is `suite.rs` and the rule nonetheless fell over
/// rather than asserting. A rule that slices `all[..2]` on a one-element result
/// is the shape: it panics at the rule's own line, and it is not the rule
/// rejecting anything.
///
/// **A denylist can never be the primary check**, which is the mistake this file
/// made first. It accepts any message not on it, including every custom
/// `panic!("…")` string a store or the testkit's own fixture contract can
/// produce, so a mutant whose modelled defect had been deleted outright still
/// read as proof. `Option::expect` is the neat illustration and used to have a
/// dead needle here: it panics with the caller's message *alone*, so
/// ``"called `Option::expect()`"`` never matches anything and its presence was
/// pure reassurance.
///
/// The list is substrings of the standard library's own panic messages. It is
/// not exhaustive and cannot be; it covers the families that a store made of
/// slices, `Option`s and arithmetic actually produces.
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
///
/// `panic::update_hook` would collapse the one- or two-instruction window inside
/// `call_once` where a concurrent panic could observe the default hook. It is
/// unstable at 1.97.1, so **do not build on it** — and the `Once` design is
/// chosen precisely so that the window stops mattering: the worst case is one
/// expected panic printing a backtrace, never a real one being hidden.
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

/// One rule, ready to run against one already-chosen store type.
///
/// `run` is a **function pointer**, not a boxed closure, and that is what keeps
/// the whole harness free of `AssertUnwindSafe`. A non-capturing closure coerces
/// to `fn()`, function pointers are unconditionally
/// [`UnwindSafe`](std::panic::UnwindSafe), and so [`run_probe`] can hand one
/// straight to `catch_unwind` with nothing asserted away.
///
/// There is no type parameter here even though the rule is generic over the
/// fixture: the fixture type is supplied by [`probes`] and then *closed over by
/// monomorphisation*, so by the time a `Probe` exists it is an ordinary value.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Probe {
    /// The rule's name, from `for_each_event_store_rule!`.
    pub(crate) name: &'static str,
    /// Runs the rule once against a freshly-opened fixture.
    pub(crate) run: fn() -> RuleOutcome,
}

/// Every registered rule, bound to `S`.
///
/// The emitter macro is defined **inside** this function so that it can mention
/// `S`. `macro_rules!` hygiene applies to local variables, not to type
/// parameters, so `S` in the expansion resolves to this function's `S` — the
/// same mechanism `event_store_conformance!` uses to let one macro's expansion
/// define `__conformance_fixture` and another's call it.
pub(crate) fn probes<S: Subject>() -> Vec<Probe> {
    macro_rules! probe {
        ($($rule:ident),* $(,)?) => {
            ::std::vec![ $(
                Probe {
                    name: ::core::stringify!($rule),
                    // Non-capturing: it mentions `S`, which is a generic
                    // parameter of the enclosing function rather than a capture,
                    // so the closure still coerces to `fn() -> RuleOutcome`.
                    run: || happenstance_testkit::block_on(
                        happenstance_testkit::rules::$rule(open_subject::<S>)
                    ),
                }
            ),* ]
        };
    }

    happenstance_testkit::for_each_event_store_rule!(probe)
}

/// Every rule of the **model** family, bound to `S`.
///
/// [`probes`] one enumeration over, and it is a second function rather than a
/// parameter of the first because the two families' rules live in two modules
/// and a `macro_rules!` expansion cannot take a module path from a `$rule:ident`
/// fragment. `happenstance_testkit::model` explains why the paths are not
/// unified.
///
/// Gated on the feature because the module is: `model` is behind the testkit's
/// off-by-default `proptest` feature, and the gate runs `--all-features`.
#[cfg(feature = "proptest")]
pub(crate) fn model_probes<S: Subject>() -> Vec<Probe> {
    macro_rules! probe {
        ($($rule:ident),* $(,)?) => {
            ::std::vec![ $(
                Probe {
                    name: ::core::stringify!($rule),
                    run: || happenstance_testkit::block_on(
                        happenstance_testkit::model::rules::$rule(open_subject::<S>)
                    ),
                }
            ),* ]
        };
    }

    happenstance_testkit::for_each_model_rule!(probe)
}

/// Every rule of the **concurrency** family, bound to `S`.
///
/// [`probes`] one enumeration further over, and the extra `where` clause is the
/// whole reason it cannot ride the same macro: this family needs the fixture's
/// *handle* to be `Send`, because each contender's handle is moved onto its own
/// thread. Note what it does **not** need — the `Send` flavour of the port. The
/// future never crosses a thread; it is created on the contender's thread by
/// `block_on` and finishes there. `concurrency.rs` records the measurement.
///
/// Not feature-gated — the family needs no optional dependency, only threads —
/// but it does not exist on `wasm32`, and neither does this binary.
pub(crate) fn concurrency_probes<S>() -> Vec<Probe>
where
    S: Subject,
    S::Store: Send,
{
    macro_rules! probe {
        ($($rule:ident),* $(,)?) => {
            ::std::vec![ $(
                Probe {
                    name: ::core::stringify!($rule),
                    run: || happenstance_testkit::block_on(
                        happenstance_testkit::concurrency::rules::$rule(open_subject::<S>)
                    ),
                }
            ),* ]
        };
    }

    happenstance_testkit::for_each_concurrency_rule!(probe)
}

/// Runs one probe, converting a panic into a [`Verdict`].
///
/// # The fixture is constructed inside the caught closure
///
/// `catch_unwind`'s bound is `F: FnOnce() -> R + UnwindSafe`, and it constrains
/// the closure's **captures only** — everything created inside it (the fixture,
/// the pinned future inside `block_on`, the store) is invisible to the check. A
/// `Probe`'s `run` captures nothing at all, so the check passes trivially and
/// still means something.
///
/// **The first refactor anyone attempts will break this.** Hoisting an
/// `Rc`-backed fixture out and passing it in makes the capture a `&Fixture`,
/// which is `!UnwindSafe`, and the compiler's suggestion is `AssertUnwindSafe` —
/// which asserts away the very check that made the harness safe to reuse a
/// fixture after a panic. Do not. Likewise `catch_unwind(|| results.push(…))`
/// captures `&mut Vec<_>` and needs the same assertion; this function returns the
/// verdict so the push happens outside.
pub(crate) fn run_probe(probe: Probe) -> Verdict {
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

/// Every rule's verdict against one store.
#[derive(Debug)]
pub(crate) struct SubjectReport {
    /// The store's [`Subject::NAME`].
    pub(crate) name: &'static str,
    /// One entry per registered rule, in registry order.
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

impl SubjectReport {
    /// The verdict for `rule`, or `None` if no such rule ran.
    pub(crate) fn verdict(&self, rule: &str) -> Option<&Verdict> {
        self.outcomes
            .iter()
            .find(|(name, _)| *name == rule)
            .map(|(_, verdict)| verdict)
    }

    /// The names of every rule that reported a skip.
    pub(crate) fn skipped(&self) -> Vec<&'static str> {
        self.outcomes
            .iter()
            .filter(|(_, verdict)| matches!(verdict, Verdict::Skipped { .. }))
            .map(|(name, _)| *name)
            .collect()
    }
}

/// Drives every rule against `S` and collects the verdicts.
pub(crate) fn run_subject<S: Subject>() -> SubjectReport {
    let outcomes = probes::<S>()
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

// =====================================================================
// The same three things, for the projection family
// =====================================================================

/// A [`ProjectionFixture`] this binary can open by name, with no arguments.
///
/// [`Subject`]'s sibling, and a second trait for the reason
/// [`ProjectionFixture`] is a second trait: `Subject: Fixture` binds the
/// event-store port in its own supertrait, so a projection instrument cannot
/// satisfy it and inventing an `EventStore` for one would be a fixture written
/// to satisfy a bound no projection rule reads.
pub(crate) trait ProjectionSubject: ProjectionFixture + Sized {
    /// The name this instrument is reported under.
    const NAME: &'static str;

    /// A fresh, isolated backing projection store.
    ///
    /// Called **inside** the caught closure, never hoisted out of it — see
    /// [`run_probe`].
    fn open() -> Self;
}

/// The opener every projection rule is handed.
///
/// A free `async fn` for [`open_subject`]'s reason: an `async fn` *item*
/// satisfies `impl AsyncFn() -> F` as a zero-sized value, so it passes by value
/// into every rule with no borrow and no higher-ranked obligation.
async fn open_projection_subject<S: ProjectionSubject>() -> S {
    S::open()
}

/// Every registered projection rule, bound to `S`.
///
/// [`probes`] one enumeration over, and a second function rather than a
/// parameter of the first for the reason [`model_probes`] and
/// [`concurrency_probes`] are: the two families' rules live in two modules and a
/// `macro_rules!` expansion cannot take a module path from a `$rule:ident`
/// fragment.
pub(crate) fn projection_probes<S: ProjectionSubject>() -> Vec<Probe> {
    macro_rules! probe {
        ($($rule:ident),* $(,)?) => {
            ::std::vec![ $(
                Probe {
                    name: ::core::stringify!($rule),
                    run: || happenstance_testkit::block_on(
                        happenstance_testkit::projection::rules::$rule(
                            open_projection_subject::<S>
                        )
                    ),
                }
            ),* ]
        };
    }

    happenstance_testkit::for_each_projection_store_rule!(probe)
}

/// Drives every projection rule against `S` and collects the verdicts.
pub(crate) fn run_projection_subject<S: ProjectionSubject>() -> SubjectReport {
    let outcomes = projection_probes::<S>()
        .into_iter()
        .map(|probe| (probe.name, run_probe(probe)))
        .collect();

    SubjectReport {
        name: S::NAME,
        outcomes,
        declines: projection_declines::<S>(),
    }
}

/// Every projection capability `S` declines, with the reason it gave.
///
/// [`declines`]'s sibling, and it carries the same warning: the names are
/// written out because a trait's associated items cannot be enumerated from
/// outside it, so a constant that arrives on `ProjectionFixture` without a line
/// here shows up as a skip the meta-tests cannot account for.
///
/// There is no projection counterpart to the `NO_STORE_LIMITS` branch below,
/// and that is a property of the port rather than an omission: `CommitError` and
/// `ResetError` carry no capacity variant, so the projection family declares no
/// numeric-limit constants and never reaches the surface CF-40 is about.
fn projection_declines<S: ProjectionSubject>() -> Vec<(&'static str, &'static str)> {
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

/// Every capability `S` declines, with the reason it gave.
///
/// The names are written out because a trait's associated items cannot be
/// enumerated by a macro from outside it. That makes this the one list in the
/// binary with no mechanical backstop — so it is kept beside the trait it
/// mirrors rather than in the registry, and a capability that arrives without a
/// line here shows up as a skip the meta-tests cannot account for, which is a
/// loud failure rather than a silent one.
///
/// The third one arrived, which is the evidence that the arrangement works:
/// `MID_BATCH_FAULT` landed with `append_is_atomic_under_a_mid_batch_fault`, and
/// omitting it here would have made every mutant's skip of that rule
/// unaccountable rather than invisible.
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

    // CF-40's limits are facts rather than trades, so they are not `Capability`
    // and cannot go through `declined`. They still produce a reported skip, and a
    // skip nothing here accounts for is a rule that stopped running.
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
