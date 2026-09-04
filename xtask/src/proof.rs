//! Checks that each phase's proof artefact still holds the tests its clauses
//! name, then runs them.
//!
//! [`ARTEFACTS`] carries nine targets today: the conformance suite's own
//! `mutation_coverage` (CF-1 – CF-6, CF-18, and CF-22's model and concurrency
//! families; [ADR-0010]), the two `wire` targets the wire format was frozen
//! against ([ADR-0016]), the projection family's two — its mutant registry's
//! meta-tests and the harness parity guard — which are what phase 6's proof
//! artefact rests on and which nothing held until phase 6 closed, and the worked
//! example's two — `runs`, which was the first thing in the workspace to
//! *execute* a binary and read what it printed, and `ui`, the `trybuild` pair
//! that pins the compiler's own diagnostic when a domain grows a variant — and
//! `transfers-on-sqlite`'s two — `runs`, the second such binary and the only
//! one that puts the typed layer and a durable adapter in the same process, and
//! `contention`, the only target anywhere that races `happenstance::commit`
//! itself rather than the `append` beneath it.
//!
//! # Why naming the target was not enough
//!
//! The gate used to run `cargo test -p happenstance-testkit --test
//! mutation_coverage` and treat a green exit as proof the artefact exists.
//! Naming the target does catch its deletion — cargo answers `no test target
//! named 'mutation_coverage'` — and that is a real check. But `cargo test` exits
//! 0 on `running 0 tests`, so a file truncated to its `#![cfg(…)]` attributes
//! passes it, and so does one whose `#[test]`s have been renamed or
//! `#[ignore]`d. A step that a *deletion* fails and an *emptying* passes is
//! checking the filename.
//!
//! So the names are asserted, out of `--list`, before the tests run. That is the
//! same argument `package.rs` makes about `cargo package --list`: running a
//! command and discarding its output is the decorative-gate failure CLAUDE.md
//! names, one level up from the decorative *rule* ADR-0010 is about.
//!
//! # And a listing cannot see an `#[ignore]`
//!
//! That was the whole of the mechanism, and it was half of one. libtest prints
//! `name: test` for an ignored test exactly as it prints one that will run:
//! `experiments/gate-vacuity/results/raw/list-diff.txt` is **empty**, and it is
//! the diff of this file's own `--list` output for `projection_harness_parity`
//! with and without an `#[ignore = "…"]` on both of its tests. An assertion over
//! that listing cannot observe the attribute, and `cargo test` exits 0 over
//! `0 passed; 2 ignored` — so with that attribute on all thirty-one names these
//! nine targets carry, `cargo xtask ci` ran all of its steps and exited 0, four
//! of the nine artefacts having executed nothing while the step printed *"N
//! named tests present"* about each. The spelling that survives is not even an
//! obscure one: bare `#[ignore]` is refused by `clippy::pedantic`'s
//! `ignore_without_reason`, whose own diagnostic ends `help: add a reason with =
//! ".."`.
//!
//! So [`check`] reads the run's own output as well, and [`unexecuted`] is the
//! half that can disagree with the attribute: a name libtest did not report as
//! having *passed* fails the step, whatever the exit status said. The listing
//! assertion is kept rather than replaced — it is what fails a *renamed* test
//! before a build's worth of tests run, and it names the missing one.
//!
//! # The lists here are expectations, and they are meant to be edited
//!
//! Each entry's `tests` duplicate names that also live in the target's own
//! source, and that duplication is the mechanism rather than an oversight:
//! renaming one fails this step, which is exactly the moment to ask whether the
//! clause citing the old name in `SPECIFICATION.md` was updated too. A derived
//! list could not ask that question, because there would be nothing for it to
//! disagree with.
//!
//! It is deliberately a *subset* check, not an equality one. A ninth meta-test
//! is a good thing and must not need a gate edit to land. The seventh and
//! eighth — `the_model_rule_rejects_exactly_what_it_claims` and
//! `the_concurrency_rules_reject_exactly_what_they_claim` — are listed anyway,
//! because `SPECIFICATION.md` CF-22 cites them by name and this list is the
//! thing that notices when a cited name moves.
//!
//! That paragraph is the whole argument for the two `wire` entries as well, and
//! it is cited rather than re-made: a ninth wire test must not need a gate edit,
//! and the handful of wire tests a clause names must not be renamable in
//! silence. What differs is only *which* names earn a row, and that is argued at
//! [`WIRE_NEGATIVE_CONTROLS`] and [`SYNC_WIRE_TESTS`].
//!
//! # The wasm32 targets are a second shape, in the same file for one reason
//!
//! [`WASM_TARGETS`] carries the conformance targets the gate **executes** on
//! `wasm32-unknown-unknown`. It is a separate array rather than three optional
//! fields on [`Artefact`] because [`cargo_args`]'s `--all-features` is
//! load-bearing for the host rows and does not compile on that target, and
//! because the wasm flow has two entry points instead of one: [`wasm_run`]
//! enumerates and executes, and [`wasm_enumeration`] asserts the same targets
//! with no runner at all.
//!
//! They live here anyway, beside [`ARTEFACTS`], because they are the same
//! argument. `cargo test` exits 0 on `running 0 tests` whichever target it is
//! pointed at, and the wasm32 case is the one where nobody would have noticed:
//! before HS-S0048 the gate compiled that harness and never ran it, so an
//! emptied file passed a step whose name promised conformance.
//!
//! Each row names its [`RuleFamily`], and that field is what let the list hold
//! *every* `wasm32`-capable target in `happenstance-testkit` rather than the
//! event-store ones alone. The first cut of this file hard-coded the event-store
//! enumeration into both entry points, which made a projection row inexpressible
//! — and the projection harness was, at that moment, the only executed `wasm32`
//! target left with nowhere to run. A list that can hold only one family is a
//! list that decides coverage by omission.
//!
//! [ADR-0010]: ../../.kb/decisions/0010-the-suite-must-prove-itself.md
//! [ADR-0016]: ../../.kb/decisions/0016-the-wire-format.md

use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::spec_trace::workspace_root;

/// One test target, and the names inside it a clause cites.
pub(crate) struct Artefact {
    /// The package holding it.
    pub(crate) package: &'static str,
    /// Its test target, as `--test` names it.
    pub(crate) target: &'static str,
    /// The tests that must be present, fully qualified, by the name the
    /// specification cites.
    ///
    /// Fully qualified because that is what the clauses say and what a reviewer
    /// pastes into `cargo test`. Every target here wraps its tests in an inner
    /// `mod` of the target's own name, which is what makes the two halves match.
    pub(crate) tests: &'static [&'static str],
    /// The mutant registry this target declares, if it declares one.
    ///
    /// Per **artefact**, and that is the whole of the change. [`check`] used to
    /// select the count by *package* — `if package == REGISTRY_PACKAGE` — and
    /// [`registry_len`] read one hard-coded path, which was correct for exactly as
    /// long as `happenstance-testkit` had one row. A second row in the same
    /// package printed the event-store registry's count beside the projection
    /// target: a number that is not about that target at all, which is the same
    /// "a number quoted in two documents and computed nowhere" defect
    /// [`registry_len`]'s own docs were written about, one level up.
    pub(crate) registry: Option<&'static str>,
}

/// The package whose proof artefacts also carry the mutant registries.
///
/// Named once because the [`ARTEFACTS`] rows that own a registry have to agree
/// with it, and a second spelling is a second thing to get wrong. It no longer
/// *selects* the count — see [`Artefact::registry`] — but it is still the answer
/// to "whose registries are these", and both rows are its.
const REGISTRY_PACKAGE: &str = "happenstance-testkit";

/// Where the event-store mutant registry lives.
const EVENT_STORE_REGISTRY: &str = "crates/happenstance-testkit/tests/mutation_coverage.rs";

/// Where the projection mutant registry lives.
///
/// A second file with the same `REGISTRY` shape and a different subject. It is
/// what made the package-keyed selection above a defect rather than a
/// simplification.
const PROJECTION_REGISTRY: &str =
    "crates/happenstance-testkit/tests/projection_mutation_coverage.rs";

/// The line the registry opens with.
const REGISTRY_HEAD: &str = "const REGISTRY: &[Declared] = &[";

/// The meta-tests the conformance suite must still hold.
const META_TESTS: &[&str] = &[
    "mutation_coverage::every_rule_has_a_mutant",
    "mutation_coverage::mutant_registry_is_exhaustive",
    "mutation_coverage::mutants_fail_exactly_their_declared_rules",
    "mutation_coverage::every_mutant_states_its_provenance",
    "mutation_coverage::conformant_variants_pass_everything",
    "mutation_coverage::capability_skips_are_reported",
    // The seventh, landed at phase 3 stage 5 with the model family. It is behind
    // the testkit's `proptest` feature, which is why the subset check is run
    // with `--all-features` and would otherwise report it absent.
    "mutation_coverage::the_model_rule_rejects_exactly_what_it_claims",
    // The eighth, landed at phase 3 stage 5 with the concurrency family. It
    // needs no feature — the family needs threads rather than an optional
    // dependency — but it is listed for the same reason the seventh is: it is
    // the CF-1 obligation for a family whose rules are macro-emitted rather
    // than registry-keyed, and a rename should have to be noticed.
    "mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim",
];

/// The wire format's negative controls, in `happenstance-core`'s `wire` target.
///
/// These three are listed and the other wire tests are not, because they are the
/// only tests in the workspace whose deletion reads as *tidying up dead code*.
/// Each asserts that a plausible wrong encoding is caught: an unbounded event
/// wire accepting a 65 KiB payload, a `#[serde(skip)]`ped field surviving a
/// postcard round trip, an `Option`-shaped query becoming indistinguishable from
/// `None`. Every one of them constructs a type nothing else in the tree uses, so
/// a reader with a dead-code warning and a tidy instinct removes the instrument
/// and leaves the positive tests green. Nothing else asserts they exist.
const WIRE_NEGATIVE_CONTROLS: &[&str] = &[
    "wire::negative_controls::length_checked_event_wire_rejects_a_65_kib_payload",
    "wire::negative_controls::option_shaped_query_is_indistinguishable_from_none",
    "wire::negative_controls::skipped_event_wire_fails_the_postcard_round_trip",
];

/// WF-8's two envelope tests, in `happenstance-sync`'s `wire` target.
///
/// The version byte is readable before the message it prefixes, and an unknown
/// one is refused rather than guessed at. Nothing else in the workspace asserts
/// either by name: the envelope has exactly one producer and one consumer, both
/// of which agree with each other whatever the byte says, so a version check
/// deleted or a prefix moved behind the payload leaves every other test green.
const SYNC_WIRE_TESTS: &[&str] = &[
    "wire::rejects_an_unknown_format_version",
    "wire::version_is_readable_before_the_message",
];

/// The projection family's meta-tests, transcribed from `-- --list`.
///
/// **Read out of the listing, not derived from the source.** `Artefact::tests` is
/// what libtest prints, and the inner-`mod` shape the doc comment there describes
/// is a convention rather than a guarantee — guessing a name the listing did not
/// print is the one thing this row must not do. `cargo test --locked -p
/// happenstance-testkit --all-features --test projection_mutation_coverage --
/// --list` printed seven names and all seven are here; the target does wrap them
/// in a `mod` of its own name, so no deviation had to be reported.
///
/// Two of the seven are the ones the project's Definition-of-Done items 1 and 2
/// rest on, and neither could be renamed in silence before this row existed:
/// `projection_mutants_fail_exactly_their_declared_rules` is what makes a green
/// gate mean `CheckpointOnlyStore` failed **exactly** the rules it declares, and
/// `the_second_batch_shape_answers_every_rule_with_a_pass` is what makes the
/// second batch shape evidence rather than a second name for the oracle.
const PROJECTION_META_TESTS: &[&str] = &[
    "projection_mutation_coverage::every_projection_rule_has_a_mutant",
    "projection_mutation_coverage::projection_mutant_registry_is_exhaustive",
    "projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules",
    "projection_mutation_coverage::every_projection_mutant_states_its_provenance",
    "projection_mutation_coverage::projection_conformant_variants_pass_everything",
    "projection_mutation_coverage::a_batch_with_no_read_path_is_reported_as_a_skip",
    "projection_mutation_coverage::the_second_batch_shape_answers_every_rule_with_a_pass",
];

/// The harness parity guard's own two tests.
///
/// Listed for the reason the whole file exists: the guard is what stops a
/// projection rule from reaching two emitters instead of three, and a guard
/// nothing holds can be `#[ignore]`d by the same change that would have failed it.
/// It is also the answer to why the guard is a **test target** rather than a
/// `#[cfg(test)]` unit test beside `no_orphan_rules` — [`ARTEFACTS`] can only name
/// a target, so a unit test could not itself be held.
const PROJECTION_PARITY_TESTS: &[&str] = &[
    "projection_harness_parity::no_harness_lists_a_rule_by_hand",
    "projection_harness_parity::each_harness_invokes_the_suite_exactly_once",
];

/// The two tests that execute the worked example and read its transcript.
///
/// Listed for the reason the whole file exists, one level further out than
/// usual: nothing else in this workspace runs the example at all. `cargo test
/// --locked --workspace --all-features` *compiles* `course-subscriptions` and
/// never calls `main`, so before this target landed the initiative's flagship
/// scenario — *"the worked example runs end to end … with no `todo!()`
/// reached"* — was checked by a compile. A target holding the only execution of
/// a `publish = false` example is exactly the shape
/// [`WIRE_NEGATIVE_CONTROLS`] describes: nothing references it, a dead-code
/// instinct removes it, and every other test stays green.
///
/// Two names rather than the target's ten, and deliberately these two. The
/// first is *did it run*; the second is *is what it printed the surface the
/// design signed off* — and the second is the one an "it exits 0" rewrite
/// silently drops. The other eight are source-reading assertions whose subject
/// is the example's own text, and the subset check (`:34-41`) means a ninth
/// needs no edit here.
const WORKED_EXAMPLE_TESTS: &[&str] = &[
    "runs::the_binary_completes_the_dcb_cycle",
    "runs::the_transcript_is_the_designed_composition",
];

/// The tests that prove the typed layer and a durable adapter compose.
///
/// The same argument as [`WORKED_EXAMPLE_TESTS`], against a different absence.
/// That target runs the typed layer on `MemoryEventStore`; the adapter's own
/// tests run the conformance suite on `happenstance-core`. Both halves were
/// tested alone and **nothing in the workspace compiled them together**, so the
/// sentence a consumer actually cares about — *"`cargo add happenstance
/// happenstance-sqlite` and it works"* — was held up by neither.
///
/// Three names out of the target's ten, and deliberately these three.
/// `the_example_runs_end_to_end` is *did it run*. The other two are the claims
/// only a durable store can make and the ones an "it exits 0" rewrite silently
/// drops: that what was written is still there once every handle is dropped and
/// the file reopened, and that the projection's checkpoint came back with it so
/// a restart does not replay the log. The remaining seven are source-reading
/// assertions over the example's own text, and the subset check (`:34-41`)
/// means an eleventh needs no edit here.
const TYPED_LAYER_ON_SQLITE_TESTS: &[&str] = &[
    "runs::the_example_runs_end_to_end",
    "runs::the_balances_survive_the_reopen",
    "runs::the_checkpoint_survives_the_reopen",
];

/// The tests that race `happenstance::commit` on a real file.
///
/// A third absence, and the narrowest of the three. `src/main.rs` is
/// single-writer, so its `Retry` bound is never spent; the adapter's own
/// `tests/concurrency.rs` races the store properly but does it through the
/// **conformance suite**, against `happenstance-core`'s `append` and a
/// hand-built `AppendCondition`. Nothing raced the loop that owns the retry —
/// the one an application calls.
///
/// Both names, never one, and for the reason [`COMPILE_FAIL_PAIR`] gives.
/// `a_contended_commit_retries_rather_than_failing` is the claim, and it is the
/// one that goes quiet first: it is satisfied by *observing* a retry, and a
/// version of this file that hoped four threads would overlap rather than
/// forcing it with a barrier passes on a busy machine and asserts nothing on an
/// idle one. `no_update_is_lost_under_contention` is what stops a retry that
/// silently dropped a write from reading as success — it is the arithmetic, and
/// a loop can report `attempts > 1` and still lose money.
///
/// `every_contender_eventually_commits` is the target's third test and is not
/// listed, per the subset rule at `:34-41`.
const TYPED_LAYER_CONTENTION_TESTS: &[&str] = &[
    "a_contended_commit_retries_rather_than_failing",
    "no_update_is_lost_under_contention",
];

/// The compile-fail pair: the guarantee, and the control that gives it meaning.
///
/// Both names, never one. `an_unhandled_variant_fails_to_compile` is the claim —
/// a variant added to a domain enum stops the build at the fold that has not
/// been taught it — and `the_negative_control_compiles` is the only thing that
/// makes a red build *mean* that. Without the control, a typo, a renamed import
/// or an item moved behind a feature fails the first case and is banked as the
/// guarantee (RS-62-1: pair every compile-fail with a compiling one, and do not
/// trust the error code).
///
/// This row is [`WIRE_NEGATIVE_CONTROLS`]'s argument applied to the one place in
/// the tree where no mutant registry exists. The two fixtures under
/// `examples/course-subscriptions/tests/ui/` are referenced by nothing else, one
/// of them is *supposed* to be broken, and the pair's whole value is that it can
/// fail — so a reader six months out with a dead-code instinct is exactly who
/// this names them for.
const COMPILE_FAIL_PAIR: &[&str] = &[
    "ui::an_unhandled_variant_fails_to_compile",
    "ui::the_negative_control_compiles",
];

/// Every proof artefact the gate holds to its own names.
pub(crate) const ARTEFACTS: &[Artefact] = &[
    Artefact {
        package: REGISTRY_PACKAGE,
        target: "mutation_coverage",
        tests: META_TESTS,
        registry: Some(EVENT_STORE_REGISTRY),
    },
    Artefact {
        package: REGISTRY_PACKAGE,
        target: "projection_mutation_coverage",
        tests: PROJECTION_META_TESTS,
        registry: Some(PROJECTION_REGISTRY),
    },
    Artefact {
        package: REGISTRY_PACKAGE,
        target: "projection_harness_parity",
        tests: PROJECTION_PARITY_TESTS,
        registry: None,
    },
    Artefact {
        package: "happenstance-core",
        target: "wire",
        tests: WIRE_NEGATIVE_CONTROLS,
        registry: None,
    },
    Artefact {
        package: "happenstance-sync",
        target: "wire",
        tests: SYNC_WIRE_TESTS,
        registry: None,
    },
    Artefact {
        package: "course-subscriptions",
        target: "runs",
        tests: WORKED_EXAMPLE_TESTS,
        registry: None,
    },
    Artefact {
        package: "course-subscriptions",
        target: "ui",
        tests: COMPILE_FAIL_PAIR,
        registry: None,
    },
    Artefact {
        package: "transfers-on-sqlite",
        target: "runs",
        tests: TYPED_LAYER_ON_SQLITE_TESTS,
        registry: None,
    },
    Artefact {
        package: "transfers-on-sqlite",
        target: "contention",
        tests: TYPED_LAYER_CONTENTION_TESTS,
        registry: None,
    },
];

/// One conformance target that is **executed** on `wasm32-unknown-unknown`.
///
/// A separate shape beside [`Artefact`] rather than three optional fields on it,
/// and the reason is the one the risk register named: [`cargo_args`]'s
/// `--all-features` is load-bearing for the host rows' *fingerprint sharing*
/// (`:287-299`) and is outright wrong here — `happenstance-testkit`'s `proptest`
/// feature maps to a dependency declared only under
/// `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`, so a wasm row that
/// inherited the flag would not compile. Widening `Artefact` to make that flag
/// optional would have put a per-row conditional inside the args the host rows
/// depend on being identical. Two shapes, each internally uniform, is the cheaper
/// honesty.
///
/// It is a **list of rows** for the reason [`ARTEFACTS`] is: the Cloudflare
/// conformance target (HS-S0054) joins by adding a row here, carrying its own
/// package, target and module, and writes no second gate step, no second runner
/// wiring and no second environment variable.
pub(crate) struct WasmTarget {
    /// The package holding it.
    pub(crate) package: &'static str,
    /// Its test target, as `--test` names it.
    pub(crate) target: &'static str,
    /// The module the emitting macro wraps its rules in — `mod_name` at the
    /// invocation, and the prefix libtest prints in front of every name.
    pub(crate) module: &'static str,
    /// The target's own source, read by the runner-free guard.
    pub(crate) source: &'static str,
    /// Which suite this target runs, and therefore which enumeration it is held
    /// to.
    ///
    /// A field rather than a constant read by both entry points, and that is the
    /// whole reason `projection_conformance_wasm` has a row. See [`RuleFamily`].
    pub(crate) family: &'static RuleFamily,
    /// The rules that must still be present, by the name `--list` prints them
    /// under, without the module prefix.
    pub(crate) rules: &'static [&'static str],
}

/// One `wasm32` test target that is executed and runs no conformance rules.
///
/// See [`WASM_UNIT_TARGETS`] for why this is a second shape rather than three
/// more optional fields on [`WasmTarget`].
pub(crate) struct WasmUnitTarget {
    /// The package holding it.
    pub(crate) package: &'static str,
    /// How `cargo test` selects the target — `["--lib"]` today, and
    /// `["--test", "name"]` is the other spelling this field exists to allow
    /// without a second registry.
    pub(crate) selector: &'static [&'static str],
    /// The tree whose `wasm32`-gated test modules this row executes.
    ///
    /// Read by the runner-free guard, which is what makes an emptied row fail on
    /// a machine with no runner rather than pass quietly.
    pub(crate) source_dir: &'static str,
    /// The `cfg` spelling that tree's `wasm32`-only tests are written behind.
    ///
    /// A per-row field rather than one constant, because the rows this registry
    /// carries are gated in genuinely different places and a single spelling
    /// could only serve one of them. A `#[cfg(test)]` module inside a
    /// `src/` tree is written `cfg(all(test, target_arch = "wasm32"))`; an
    /// **integration** target is already `#[cfg(test)]` by virtue of being one,
    /// so its `wasm32`-only half is written `cfg(target_arch = "wasm32")` and a
    /// scan looking for the first spelling would find nothing and bail — telling
    /// a reader the row is aimed somewhere the tests are not, when it is aimed
    /// exactly at them.
    ///
    /// Widening [`WASM32_TEST_GATE`] to accept both spellings was the other
    /// option and it is worse: `cfg(target_arch = "wasm32")` appears in `src/`
    /// trees for reasons that have nothing to do with tests, so the *package*
    /// scan below — whose whole job is to notice a package with `wasm32` tests
    /// and no row — would start matching packages that have none.
    pub(crate) gate: &'static [&'static str],
    /// What this target needs from the host beyond the runner itself.
    ///
    /// Printed with the failure rather than left for the reader to infer: a
    /// target that needs a newer Node than the machine has fails inside a test
    /// body, and "the adapter is broken" is the wrong conclusion to hand
    /// someone.
    pub(crate) host: &'static str,
    /// The tests that must still be present, by the name `--list` prints.
    pub(crate) tests: &'static [&'static str],
}

/// One conformance suite: its rule enumeration, the macro that invokes it, and
/// the `wasm32` emitter it must go through.
///
/// The workspace has two, and before this type existed the wasm checks knew only
/// about the first. That was not a simplification: `projection_conformance_wasm`
/// could not be registered without failing the event-store enumeration check on
/// every one of its seventeen rules, so the only expressible answers were *drop
/// it* or *weaken the check*. Naming the family per row makes the third answer —
/// hold each target to its own enumeration, exhaustively — the cheap one.
///
/// It carries the *suite macro* and the *emitter* as well as the enumeration
/// because both are family-specific: a projection target that invoked
/// `event_store_conformance!`, or emitted through `__emit_wasm`, would not
/// compile, and a guard that looked for the event-store spellings in a
/// projection source would fail for a reason that is not the reader's.
pub(crate) struct RuleFamily {
    /// The enumeration macro's name, as a failure message should spell it.
    pub(crate) enumeration: &'static str,
    /// Where that enumeration lives.
    pub(crate) source: &'static str,
    /// The line the enumeration opens with.
    pub(crate) head: &'static str,
    /// The suite-invoking macro a target of this family must call.
    pub(crate) suite: &'static str,
    /// The `wasm32` emitter a target of this family must go through.
    ///
    /// `__emit_projection_tokio` type-checks for this target and then cannot run
    /// on it, exactly as `__emit_tokio` does for the event-store family — which
    /// is the failure CF-23 is about, and it has one spelling per family.
    pub(crate) emitter: &'static str,
}

/// The event-store suite: eighty-nine rules, one enumeration.
static EVENT_STORE_FAMILY: RuleFamily = RuleFamily {
    enumeration: "for_each_event_store_rule!",
    source: "crates/happenstance-testkit/src/registry.rs",
    head: "macro_rules! for_each_event_store_rule {",
    suite: "event_store_conformance!",
    emitter: "__emit_wasm",
};

/// The projection suite: a second enumeration, in a second file, with its own
/// emitter.
///
/// Registered for execution rather than dropped. The `wasm-conformance` CI job
/// this seam retired ran `cargo test -p happenstance-testkit --target
/// wasm32-unknown-unknown`, which is *every* `wasm32`-capable target in the
/// package — so retiring it without this row would have moved eighty-nine
/// executions into the gate and deleted seventeen, while the retirement note
/// claimed the gate was strictly more. A coverage decision taken by omission is
/// the shape the deployment brief's DEPLOY-AC-05 forecloses.
static PROJECTION_FAMILY: RuleFamily = RuleFamily {
    enumeration: "for_each_projection_store_rule!",
    source: "crates/happenstance-testkit/src/projection.rs",
    head: "macro_rules! for_each_projection_store_rule {",
    suite: "projection_store_conformance!",
    emitter: "__emit_projection_wasm",
};

/// The rules a wasm32-only subset would reach for first.
///
/// Transcribed rather than derived, and the two lists in this row do different
/// jobs on purpose. [`wasm_conformance`] already asserts the **whole**
/// enumeration against the listing, which no hand-list could keep up with; what
/// that derived check cannot do is notice a *rename*, because a name changed in
/// `registry.rs` is changed on both sides of the comparison at once. These nine
/// are the ones where a rename or a quiet removal would cost the most, and the
/// reasons are three:
///
/// The re-entrancy, concurrency and read-isolation rules are the reason this
/// target exists at all. `wasm32-unknown-unknown` is single-threaded and its
/// futures are `!Send`, so these are the rules that look hardest here and are
/// the first any narrowing would reach for — and they are the only place
/// [ADR-0001]'s bare flavour is observed under execution rather than under a
/// `cargo check`.
///
/// The two fixture-contract rules are the seam's consumer. The isolation rule is
/// the one genuinely new failure mode a Cloudflare fixture can have — a fixture
/// pointing every fresh instance at the same Durable Object storage passes
/// everything else — and it is what HS-S0054's row is being registered to run.
///
/// The three capability-declining rules pass while doing nothing visible unless
/// the runner is told not to capture. That makes them the ones a reader with a
/// dead-code instinct calls decorative, and they are exactly the rules whose
/// `SKIP <rule>: <reason>` lines the `--nocapture` argument exists to keep
/// legible.
///
/// [ADR-0001]: ../../.kb/decisions/0001-async-port-flavours.md
const MEMORY_WASM_RULES: &[&str] = &[
    "a_live_read_stream_does_not_block_an_append",
    "interleaved_appends_on_one_handle_elect_one_winner",
    "racing_conditional_appends_elect_one_winner",
    "read_result_is_stable_under_concurrent_append",
    "two_fixture_instances_observe_none_of_each_others_appends",
    "two_handles_observe_each_others_appends",
    "acknowledged_writes_survive_a_reopen",
    "append_is_atomic_under_a_mid_batch_fault",
    "append_reports_exceeded_store_limits",
];

/// The rules a `!Send` store is the only thing in the workspace that can lose.
///
/// The same argument [`MEMORY_WASM_RULES`] makes, aimed one axis over.
/// `LocalMemoryEventStore` is the workspace's only genuinely `!Send` event store
/// — an `Rc<RefCell<Vec<_>>>`, which is the shape a Durable Object has — and
/// this row is the only place the bare port flavour is *executed* on the target
/// it exists for. Every other wasm32 row drives a `Send` store that happens to
/// be running single-threaded.
///
/// The three re-entrancy and concurrency rules are why: they are the rules whose
/// pass depends on the store's borrow discipline rather than on a lock, and
/// `AwaitAcrossBorrowStore` in the testkit's own `tests/` is the registered
/// wrong implementation each of them rejects. The three fixture-contract rules
/// are `LocalFixture`'s own seam — including `acknowledged_writes_survive_a_reopen`,
/// which this fixture **declines**, and which is therefore the row's one visible
/// `SKIP <rule>: <reason>` line under `--nocapture`.
const LOCAL_WASM_RULES: &[&str] = &[
    "a_live_read_stream_does_not_block_an_append",
    "interleaved_appends_on_one_handle_elect_one_winner",
    "read_result_is_stable_under_concurrent_append",
    "two_fixture_instances_observe_none_of_each_others_appends",
    "two_handles_observe_each_others_appends",
    "acknowledged_writes_survive_a_reopen",
];

/// The projection rules a single-threaded runtime would break first.
///
/// The atomicity pair is the projection port's whole invariant — read-model write
/// and checkpoint write in one transaction — and it is the pair a store that
/// buffers writes and replays them at commit gets wrong in a way no `cargo check`
/// sees. The two failure paths beside it are where an adapter that treats a
/// rollback as a no-op passes everything else. `rebuild_is_chunk_size_invariant`
/// is named because it is the one rule here that drives a loop long enough for a
/// stubbed `Instant::now()` to be reached, which RS-52-1 says is observable only
/// under execution.
const PROJECTION_WASM_RULES: &[&str] = &[
    "commit_advances_the_checkpoint",
    "commit_is_atomic_with_the_read_model",
    "failed_commit_leaves_both_unchanged",
    "rollback_leaves_both_unchanged",
    "rebuild_is_chunk_size_invariant",
    "rebuilding_is_distinguishable_from_live",
];

/// The rules a Durable Object is the only thing in the workspace that can lose.
///
/// The same argument [`MEMORY_WASM_RULES`] makes, aimed at an adapter rather
/// than at a fixture the testkit ships, and the reasons are four.
///
/// The two **fixture-contract** rules are the seam this row was registered to
/// run. `two_fixture_instances_observe_none_of_each_others_appends` is the one
/// genuinely new failure mode this project can introduce and that nothing
/// upstream can catch — a `CloudflareFixture` quietly pointing every fresh
/// instance at the same Durable Object storage passes every other rule in the
/// suite — and `two_handles_observe_each_others_appends` is CF-16's `must!`,
/// which fails rather than skips on a declined capability.
///
/// The three **`REOPEN`** rules are named because this fixture is the first in
/// the workspace to answer that capability `SUPPORTED`. Everything else declines
/// it, so until this row existed the three were skips everywhere and the reopen
/// path had never been executed by anything. A regression that turned the
/// constant back to a decline would leave them green as skips, and only a list
/// written down elsewhere notices.
///
/// The two **atomicity-under-fault** rules were named when this fixture still
/// *declined* `MID_BATCH_FAULT` and they were its visible
/// `SKIP <rule>: <reason>` lines under `--nocapture`. `measured-store-limits`
/// flipped it: the constant is `Capability::SUPPORTED`, the fault rests on a
/// real SQLite trigger, and both rules now **Ran**. They stay named for the
/// reason the flip created rather than removed — a regression turning the
/// constant back to a decline would leave them green as skips, and only a list
/// written down here notices.
///
/// `append_reports_exceeded_store_limits` is here for the same reason one
/// position over: it is the rule whose outcome `measured-store-limits` turned
/// from a `NO_STORE_LIMITS` skip into a `Ran`, and it is the workspace's one
/// capacity-capped runtime reporting on CF-40. The Cloudflare row prints no
/// `SKIP` line today.
///
/// The two **read-isolation and interleaving** rules are the ones whose pass
/// depends on this adapter's borrow discipline rather than on a lock — a Durable
/// Object is single-threaded and re-entrant, `SqlStorage` reaches its shared
/// state through a `RefCell` that is *tried*, and ADR-0011's ceiling-and-page is
/// what keeps a cursor off a suspension point. They are the rules that look
/// hardest here and the first any narrowing would reach for.
const CLOUDFLARE_WASM_RULES: &[&str] = &[
    "two_fixture_instances_observe_none_of_each_others_appends",
    "two_handles_observe_each_others_appends",
    "acknowledged_writes_survive_a_reopen",
    "reopened_store_does_not_reissue_an_event_id",
    "recorded_time_survives_a_reopen",
    "append_is_atomic_under_a_mid_batch_fault",
    "arming_a_mid_batch_fault_makes_the_append_fail",
    "append_reports_exceeded_store_limits",
    "read_result_is_stable_under_concurrent_append",
    "a_live_read_stream_does_not_block_an_append",
];

/// Every conformance target the gate executes on `wasm32-unknown-unknown`.
///
/// Three rows, and the shape is the deliverable. `every-rule-under-workerd`
/// (HS-S0054) adds the Cloudflare conformance target here — a package, a target,
/// the module its emitter wraps, its family and the rules worth naming — and
/// inherits the runner wiring, the version check, the exhaustive enumeration
/// check and both gate steps without writing any of them again.
///
/// The three are the whole of what `cargo test -p happenstance-testkit --target
/// wasm32-unknown-unknown` used to run in the retired `wasm-conformance` job,
/// and that is the bar this list is held to rather than a coincidence: the job
/// was retired *because* the gate subsumes it, and one row short of the package
/// the claim would have been false. `every_wasm32_capable_harness_has_a_row`
/// below is what keeps it true when a fourth harness lands.
pub(crate) const WASM_TARGETS: &[WasmTarget] = &[
    WasmTarget {
        package: REGISTRY_PACKAGE,
        target: "memory_conformance_wasm",
        module: "dcb_conformance_wasm",
        source: "crates/happenstance-testkit/tests/memory_conformance_wasm.rs",
        family: &EVENT_STORE_FAMILY,
        rules: MEMORY_WASM_RULES,
    },
    WasmTarget {
        package: REGISTRY_PACKAGE,
        target: "local_conformance",
        module: "local_wasm",
        source: "crates/happenstance-testkit/tests/local_conformance.rs",
        family: &EVENT_STORE_FAMILY,
        rules: LOCAL_WASM_RULES,
    },
    WasmTarget {
        package: REGISTRY_PACKAGE,
        target: "projection_conformance_wasm",
        module: "projection_conformance_wasm",
        source: "crates/happenstance-testkit/tests/projection_conformance_wasm.rs",
        family: &PROJECTION_FAMILY,
        rules: PROJECTION_WASM_RULES,
    },
    WasmTarget {
        // The fourth row, and the first that is not the testkit measuring
        // itself. Everything above drives a store the testkit ships; this drives
        // an **adapter**, on the target the two-flavour port design was paid
        // for, through a `Fixture` the adapter's own author wrote.
        //
        // It is the whole of this story's `xtask` delta, which is the contract
        // `wasm-execution-gate-step` took in its AC-006: a package, a target, the
        // module its emitter wraps, its family and the rules worth naming. No
        // second `Step`, no second runner wiring, no second `--target` plumbing.
        package: "happenstance-cloudflare",
        target: "durable_object_conformance",
        module: "dcb_conformance_wasm",
        source: "crates/happenstance-cloudflare/tests/durable_object_conformance.rs",
        family: &EVENT_STORE_FAMILY,
        rules: CLOUDFLARE_WASM_RULES,
    },
];

/// Every `wasm32` test target the gate executes that is **not** a conformance
/// harness.
///
/// A sibling registry rather than more [`WASM_TARGETS`] rows, and the split is
/// the same one [`WasmTarget`] makes against [`Artefact`]: every row of
/// `WASM_TARGETS` is held to a [`RuleFamily`]'s exhaustive enumeration, and a
/// target that runs no conformance rules has no enumeration to be held to. One
/// registry would have to make `family` optional, and an optional field on a
/// row is a conditional inside the check the other rows depend on.
///
/// **Why it exists at all.** `happenstance-cloudflare`'s adapter tests —
/// the write path, the read path, the four modelled `SqlStorage` properties, the
/// `!Send` probes' `wasm32` twins and the ES-6 reconstruction — are
/// `#[wasm_bindgen_test]` cases under `#[cfg(all(test, target_arch =
/// "wasm32"))]`, because that is the only target the auto-trait leak they exist
/// to catch can appear on. The gate *compiled* them from the day they merged
/// (`main.rs`'s `wasm32 build of the Cloudflare adapter` step carries `--tests`
/// for exactly that) and executed none of them: `WASM_TARGETS` holds three
/// testkit rows, and [`unregistered_wasm_harnesses`] scans only the testkit's
/// `tests/` directory, so nothing in this file could notice. A host
/// `cargo test -p happenstance-cloudflare` runs four probes and nothing about
/// whether the adapter works. That is the same *compiled, never executed* shape
/// the previous milestone built this machinery to end, one directory over.
pub(crate) const WASM_UNIT_TARGETS: &[WasmUnitTarget] = &[
    WasmUnitTarget {
        package: "happenstance-cloudflare",
        selector: &["--lib"],
        source_dir: "crates/happenstance-cloudflare/src",
        gate: WASM32_TEST_GATE,
        host: CLOUDFLARE_HOST,
        tests: CLOUDFLARE_UNIT_TESTS,
    },
    WasmUnitTarget {
        // The fixture's own contract, and the second row this registry has ever
        // carried — which is the field `selector`'s documentation was written
        // for: `["--test", name]` was always the other spelling, and this is the
        // first row to need it.
        //
        // It is *not* a `WASM_TARGETS` row, and the distinction is the one that
        // registry's own documentation draws: every row there is held to a
        // `RuleFamily`'s exhaustive enumeration, and this target runs no
        // conformance rule *of the suite's enumeration*. What it runs is the
        // nine assertions about the fixture that the suite cannot make about
        // itself — isolation between two instances alive at once, two handles
        // onto one object, the schema seam running per instance, the store id
        // surviving a second handle, the `REOPEN` override actually discarding
        // handle state rather than events, the armed fault being SQLite's rather
        // than the host shim's, and the one `SKIP` line the gate emits at all.
        // Every one of them needs a real Durable Object, so every one of them is
        // `#[wasm_bindgen_test]`, so without this row the gate would compile
        // them and run none — the exact shape the rest of this file exists to
        // end.
        //
        // The last two do name two shipped rules in their code, and that is not
        // the subset-list shape `the_executed_wasm_targets_name_no_rule_of_their_own`
        // rejects: they hand a *declining* fixture to two `require!`-gated rules
        // to observe the skip line, which is an assertion about reporting rather
        // than a hand-picked enumeration. The check that forbids naming rules
        // reads `WASM_TARGETS`, where the conformance target lives, and this row
        // is deliberately not there.
        //
        // The same target's *other* five cases are plain `#[test]`s about what
        // the fixture declares, and they need no runner at all: an ordinary
        // `cargo test -p happenstance-cloudflare` runs them. They are absent
        // from `tests` below because `--list` on this runner reports only what
        // the `wasm-bindgen-test` harness collects.
        package: "happenstance-cloudflare",
        selector: &["--test", "fixture_contract"],
        source_dir: "crates/happenstance-cloudflare/tests",
        gate: WASM32_TARGET_GATE,
        host: CLOUDFLARE_HOST,
        tests: CLOUDFLARE_FIXTURE_CONTRACT_TESTS,
    },
    WasmUnitTarget {
        // WF-11's falsifier, and the third row — which is the whole of
        // `wf-11-memory-ceiling-falsifier`'s (HS-S0056) `xtask` delta. That is
        // the contract `wasm-execution-gate-step` took in its AC-006 and
        // `every-rule-under-workerd` restated in its AC-003, cashed for the
        // second time: a package, a selector, the tree the runner-free guard
        // reads, and the tests a rename has to disagree with. No second `Step`,
        // no second runner variable, no second `--target` plumbing.
        //
        // It is deliberately **not** a `WASM_TARGETS` row. Every row there is
        // held to a `RuleFamily`'s exhaustive enumeration and this target runs
        // no conformance rule of either suite — WF-11 is a *wire* clause, and
        // nothing in an event-store suite can observe how much memory an encode
        // took. What it runs instead is a measurement: an isolate's real memory
        // ceiling, walked in-process with `core::arch::wasm32::memory_grow`, and
        // the peak cost of `happenstance-core`'s human-readable payload encode
        // against the binary one on identical bytes.
        //
        // It is a second target rather than two more cases in
        // `durable_object_conformance`, and that is forced twice over: that
        // harness is held to three lines and defines nothing, and a probe whose
        // job is to walk an isolate up to its ceiling must not share that
        // isolate with the conformance rules.
        package: "happenstance-cloudflare",
        selector: &["--test", "wf11_memory_ceiling"],
        source_dir: "crates/happenstance-cloudflare/tests",
        gate: WASM32_TARGET_GATE,
        host: CLOUDFLARE_HOST,
        tests: WF11_MEMORY_CEILING_TESTS,
    },
    WasmUnitTarget {
        // The query-ceiling target, and the fourth row. It exists because the
        // adapter's own partition is otherwise asserted only by arithmetic over
        // the *strings* it builds — how many compound terms, how many bound
        // parameters — and arithmetic about a string is not evidence that SQLite
        // would have refused the string or that the merge behind the partition
        // reassembles the right rows. The wall is reachable in this harness and
        // was simply never approached; this row approaches it.
        //
        // It is **not** a `WASM_TARGETS` row, for that registry's own stated
        // reason: every row there is held to a `RuleFamily`'s exhaustive
        // enumeration, and this target runs no conformance rule of either suite.
        // It could not: `MIN_SUPPORTED_QUERY_ITEMS` is 128 items at one tag
        // each, which is 128 arms and 128 parameters, inside both of SQLite's
        // pushdown limits by two orders of magnitude — so no rule the suite
        // enumerates can cross either wall, which is the whole finding.
        //
        // A second target rather than more cases in `durable_object_conformance`,
        // and forced by the same two things that forced WF-11's: that harness is
        // held to three lines and defines nothing of its own, and a target whose
        // first case deliberately drives the driver into refusing a statement
        // must not share an object with the rules.
        package: "happenstance-cloudflare",
        selector: &["--test", "wide_query_ceiling"],
        source_dir: "crates/happenstance-cloudflare/tests",
        gate: WASM32_TARGET_GATE,
        host: CLOUDFLARE_HOST,
        tests: WIDE_QUERY_CEILING_TESTS,
    },
];

/// The query-ceiling target's cases, by the name `--list` prints them.
///
/// Hand-written and **unprefixed**, for the reasons
/// [`WF11_MEMORY_CEILING_TESTS`] gives: there is no enumeration behind these
/// names, so the row is a citation and a rename should have to be noticed; and
/// there is no `mod $mod_name` wrapper, because `event_store_conformance!` is
/// not involved.
///
/// **The first name is load-bearing in a way the other six are not.**
/// `the_unpartitioned_statement_is_refused_by_this_runtime` is the control: it
/// hands this runtime the statement a translation with no partition would have
/// built and asserts the driver refuses it. Delete that and the six passing
/// cases below it degrade from *the wall is real and no longer hit* to *nothing
/// went wrong*, which is a suite that would stay green if
/// `SQLITE_MAX_COMPOUND_SELECT` were raised out from under it.
const WIDE_QUERY_CEILING_TESTS: &[&str] = &[
    "the_unpartitioned_statement_is_refused_by_this_runtime",
    "a_read_past_the_compound_select_ceiling_is_served_from_every_chunk",
    "a_read_past_the_bound_parameter_ceiling_is_served_from_every_chunk",
    "an_append_guard_past_the_compound_select_ceiling_is_not_refused",
    "an_append_guard_past_the_bound_parameter_ceiling_is_not_refused",
    "a_wide_guard_answers_from_every_chunk_not_the_first",
    "a_parameter_wide_guard_answers_from_every_chunk_not_the_first",
];

/// The WF-11 probe's two cases, by the name `--list` prints them.
///
/// Hand-written here, and that is the **opposite** of the choice
/// [`WASM_TARGETS`]' Cloudflare row makes — on purpose. That row derives its
/// expectation from `for_each_event_store_rule!` because the thing being guarded
/// is a harness against a rule enumeration. There is no enumeration behind these
/// two names, so the row is a *citation*, which is precisely the case this
/// file's own opening argument says duplication is for: a rename should have to
/// be noticed.
///
/// **Unprefixed**, unlike every other list in this file, because there is no
/// `mod $mod_name` wrapper around them — `event_store_conformance!` is not
/// involved here and the two cases sit at the top level of the target.
///
/// **Two, and the count is forced by the instrument.** `wasm32` linear memory is
/// one global resource shared by every test in a target and `memory_grow` is
/// one-way, so a ceiling staircase in its own case would leave a grown heap
/// behind and zero every later peak delta. One case therefore owns the ordering
/// — anchored encodes first, ceiling second, the computed size last — and the
/// other is written to be order-independent, comparing encoded *lengths* rather
/// than memory, so it is immune to whatever the first did.
///
/// What this list cannot catch is stated rather than left to be discovered: a
/// body emptied while both names survive. A name-based artefact cannot see an
/// assertion that was deleted, and there is no rule enumeration here to derive a
/// second expectation from. That residual gap is named in the story's
/// implementation report.
const WF11_MEMORY_CEILING_TESTS: &[&str] = &[
    "records_the_ceiling_and_the_encode_peaks",
    "the_two_encode_paths_produce_the_published_size_ratio",
];

/// What both `happenstance-cloudflare` rows need from the machine.
///
/// Named once because both rows execute the same shim against the same engine,
/// and a host requirement stated twice is a host requirement that will disagree
/// with itself the first time Node's floor moves.
const CLOUDFLARE_HOST: &str = "Node 22.5 or newer: `crates/happenstance-cloudflare/src/host.rs` \
     reaches `node:sqlite` through `process.getBuiltinModule`, and the runner's \
     default host is Node";

/// The fixture-contract cases that need a real object under them.
///
/// Transcribed for [`MEMORY_WASM_RULES`]' reason — a rename needs something to
/// disagree with — and every one is a fact no other check in this repository can
/// reach. The isolation case in particular is the one genuinely new failure mode
/// this adapter can have: a `CloudflareFixture` that quietly pointed every fresh
/// instance at the same Durable Object storage would pass every conformance rule
/// that does not construct two fixture instances, and it constructs both here
/// **before** either appends, which is what tells isolation apart from clearing.
const CLOUDFLARE_FIXTURE_CONTRACT_TESTS: &[&str] = &[
    "on_the_object::the_host_is_reachable_from_an_integration_test",
    "on_the_object::two_instances_alive_at_once_observe_none_of_each_others_appends",
    "on_the_object::two_handles_from_one_instance_observe_each_others_appends",
    "on_the_object::migrate_runs_once_per_instance_not_per_connect",
    "on_the_object::a_second_handle_does_not_re_mint_the_store_id",
    "on_the_object::a_supported_capability_has_its_method_overridden",
    "on_the_object::the_hosts_arming_hook_fires_once_and_disarms",
    // The two that carry the claims this crate would otherwise be making on
    // trust. The first reads the trigger's own `RAISE(ABORT, …)` text back out
    // of a caller-visible error, which is the only assertion that can tell
    // CF-39's fault apart from one armed on the JavaScript host — a fault the
    // double owns rather than the store. The second is the *only* place in the
    // gate where a `SKIP <rule>: fixture declines …` line is actually produced
    // and read, because `CloudflareFixture` declines nothing: without it,
    // project AC-003's "emits" half has no live instance anywhere.
    "on_the_object::the_armed_fault_is_a_real_trigger_inside_the_store",
    "on_the_object::a_declined_capability_reaches_the_gate_as_a_skip_line",
];

/// The `happenstance-cloudflare` cases worth naming, by the name `--list` prints.
///
/// Transcribed rather than derived, for the reason [`MEMORY_WASM_RULES`] gives:
/// a rename needs something to disagree with. Chosen so that each is a fact no
/// other check in this repository can reach — a deleted or `#[cfg]`-ed-out
/// module takes its own detector with it, and only a list written down
/// elsewhere notices.
///
/// The four groups, and what each is the last guard on:
///
/// * the `!Send` twins, including the positive control, which are the only
///   assertions that run on a target where `wasm-bindgen`'s
///   `cfg(not(target_feature = "atomics"))` `unsafe impl` is live;
/// * the three write-path all-or-none guards, which reject the shape the adapter
///   itself had — N inserts, a throw converted to `Err(…)`, and a turn that
///   commits the rows written before it;
/// * the read path's ceiling detectors and their three committed negative
///   controls, which are the whole of ADR-0011 as this adapter implements it;
/// * the ES-6 reconstruction, which is project AC-005's artefact.
const CLOUDFLARE_UNIT_TESTS: &[&str] = &[
    "wasm_tests::the_probe_is_not_vacuous",
    "wasm_tests::the_js_boundary_types_are_not_send",
    "wasm_tests::the_error_type_is_not_send",
    "wasm_tests::the_send_flavour_does_not_imply_a_send_error",
    "event_store::write_path_tests::a_batch_that_throws_after_its_first_row_leaves_nothing_behind",
    "event_store::write_path_tests::a_batch_that_throws_while_stamping_identity_leaves_nothing_behind",
    "event_store::write_path_tests::a_batch_whose_discard_also_fails_reports_both_failures",
    "event_store::read_path_tests::read_is_stable_under_an_interleaved_append",
    "event_store::read_path_tests::a_ceilingless_paging_read_is_rejected",
    "event_store::read_path_tests::a_cursor_held_across_a_poll_is_rejected",
    "event_store::read_path_tests::a_null_head_ceiling_is_rejected_on_the_empty_store",
    "event_store::read_path_tests::all_items_of_one_query_share_one_ceiling",
    "es6_reconstruction::constraint_violation_reaches_the_caller_as_condition_violated",
    "es6_reconstruction::transport_fault_reaches_the_caller_distinguishably",
    "es6_reconstruction::an_evidence_discarding_classifier_is_rejected",
    "es6_reconstruction::the_distinction_is_reachable_from_outside_the_crate",
];

/// The runner that executes a `wasm-bindgen-test` harness.
///
/// Spelled once and shared with the gate step's probe in `main.rs`: a step that
/// probes for one tool and runs another is a step that skips for the wrong
/// reason.
pub(crate) const WASM_RUNNER: &str = "wasm-bindgen-test-runner";

/// Cargo's per-target runner variable.
///
/// Set on the inner `cargo test` here rather than on the gate step's `env`, and
/// the choice is deliberate. `Step.env` would set it on the `cargo run -p xtask`
/// process and reach the real invocation only by inheritance — which works
/// inside the gate and leaves `cargo xtask wasm-conformance` broken as a
/// standalone command. Setting it per-`Command` honours the reason `Step.env`
/// exists (`main.rs:89-97`: a per-target variable must not become a
/// process-wide one) more strictly than `Step.env` itself would, because the
/// variable never touches this process's own environment at all.
const WASM_RUNNER_VAR: &str = "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER";

/// The target triple the conformance rules are executed on.
const WASM_TRIPLE: &str = "wasm32-unknown-unknown";

/// The crate whose version the installed runner must match exactly.
const WASM_BINDGEN: &str = "wasm-bindgen";

/// A workspace file, read through the root rather than `include_str!`.
///
/// The same reasoning [`registry_len`] is reached through: this file is
/// `xtask`'s and the sources it reads are the testkit's, so a compile-time
/// include would make a `publish = false` tool's build depend on a crate it has
/// no other relationship with — and would freeze the read at `xtask`'s own
/// compilation rather than the gate's run.
fn read_source(file: &str) -> Result<String> {
    let root = workspace_root()?;
    fs::read_to_string(root.join(file)).with_context(|| format!("reading {file}"))
}

/// The parts of a source that are not comments.
///
/// Written because the single-sourcing guard in [`wasm_enumeration`] was a bare
/// `source.contains(rule)` over the whole file, and that is a substring search
/// which cannot tell a rule *list* from a sentence about a rule.
/// `local_conformance.rs` names rules in its documentation and in a capability
/// comment — it is explaining where they moved to, and which one its fixture
/// declines — and every `wasm32` harness worth registering is one whose prose
/// discusses its own rules. A guard that a correct file fails is not a stricter
/// guard; it is one whose next reader deletes it. Deliberately no count in that
/// sentence: it is a second copy of a list, and it drifts the first time the
/// list moves.
///
/// The truncation is at the first `//` on each line, so a trailing comment is
/// cut as well as a whole-line one. Two limits, both stated rather than left to
/// be discovered: a `//` inside a string literal truncates that line early, which
/// can only ever make this scan miss a rule name it should have seen — and block
/// comments it cannot read at all, which is why [`wasm_enumeration`] refuses a
/// source containing one rather than scanning past it.
fn code_only(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split("//").next().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Where the `wasm32`-capable conformance harnesses live.
///
/// A directory rather than a list, and that is the point of it. See
/// [`unregistered_wasm_harnesses`].
const HARNESS_DIR: &str = "crates/happenstance-testkit/tests";

/// Test targets that drive a suite through a `wasm32` emitter and have no row in
/// [`WASM_TARGETS`].
///
/// The retired `wasm-conformance` CI job ran `cargo test -p happenstance-testkit
/// --target wasm32-unknown-unknown` — *every* `wasm32`-capable target in the
/// package, named individually nowhere, so it could not fall behind the tree.
/// [`WASM_TARGETS`] names each one, so it can, and the first cut of this seam
/// did: the job was retired with one row registered and two harnesses left
/// executing nowhere, while the note replacing it said the gate was strictly
/// more. This is the check that makes that sentence true rather than hopeful,
/// and it is why the claim is allowed to be written down at all.
///
/// A harness is `wasm32`-capable if its **code** names an emitter spelled
/// `__emit…wasm`. Derived from the spelling rather than from the registered
/// families, because a check that looked only for emitters already in
/// [`WASM_TARGETS`] could never notice a third family arriving unregistered —
/// which is the exact shape of the miss it exists to prevent.
///
/// # Errors
///
/// Returns an error if the directory cannot be read, if a source in it cannot be
/// read, or if it holds fewer capable harnesses than there are registered rows —
/// which means this scan is reading somewhere the rows are not.
fn unregistered_wasm_harnesses() -> Result<Vec<String>> {
    let dir = workspace_root()?.join(HARNESS_DIR);
    let entries = fs::read_dir(&dir).with_context(|| format!("reading {HARNESS_DIR}"))?;

    let mut capable = Vec::new();
    for entry in entries {
        let path = entry
            .with_context(|| format!("reading an entry of {HARNESS_DIR}"))?
            .path();
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }

        let source =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let code = code_only(&source);
        let emits_to_wasm = code
            .match_indices("__emit")
            .map(|(at, _)| {
                code[at..]
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect::<String>()
            })
            .any(|name| name.ends_with("wasm"));

        if emits_to_wasm {
            capable.push(
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or_default()
                    .to_owned(),
            );
        }
    }
    capable.sort();

    // Compared against the rows that live in *this* directory, not against every
    // row in the registry. The distinction did not exist while every row was the
    // testkit's own and it became load-bearing the moment an adapter registered
    // one: this scan reads a single directory by design (`HARNESS_DIR`), so a
    // Cloudflare row counted here would make the guard fire on a registry that
    // is exactly right, with a message accusing the scan of reading nowhere.
    //
    // The property being guarded is unchanged and is still the important one:
    // every harness the scan *can* see must be registered, and the scan must be
    // able to see at least as many as claim to be there.
    let rows_here = WASM_TARGETS
        .iter()
        .filter(|wasm| wasm.source.starts_with(HARNESS_DIR))
        .count();
    if capable.len() < rows_here {
        bail!(
            "{HARNESS_DIR} holds {} wasm32-capable harness(es) — {capable:?} — and \
             {rows_here} row(s) claim to live there. This scan is reading somewhere \
             the rows are not, so its silence would mean nothing.",
            capable.len()
        );
    }

    Ok(capable
        .into_iter()
        .filter(|target| !WASM_TARGETS.iter().any(|wasm| wasm.target == *target))
        .collect())
}

/// Where a crate says "these tests exist only on `wasm32`".
///
/// The module gate, normalised for whitespace and accepted in both orderings.
/// It is the spelling `happenstance-cloudflare` uses in six files, and the one
/// that makes a test module invisible to every host `cargo test` — which is
/// exactly the condition under which "compiled by the gate, executed by nothing"
/// can happen without anybody noticing.
const WASM32_TEST_GATE: &[&str] = &[
    r#"cfg(all(test,target_arch="wasm32"))"#,
    r#"cfg(all(target_arch="wasm32",test))"#,
];

/// Where an **integration** target says "this half only exists on `wasm32`".
///
/// The same job as [`WASM32_TEST_GATE`] one directory over, and it is a separate
/// constant rather than a third entry there for the reason
/// [`WasmUnitTarget::gate`] gives: this spelling is common in `src/` trees for
/// reasons unrelated to tests, so folding it into the package scan would make
/// that scan match packages with no `wasm32` tests at all.
const WASM32_TARGET_GATE: &[&str] = &[r#"cfg(target_arch="wasm32")"#];

/// Packages whose sources declare `wasm32`-only test modules and that have no
/// row in [`WASM_UNIT_TARGETS`].
///
/// The sibling of [`unregistered_wasm_harnesses`], and it exists for the miss
/// that one could not see: it scans a single directory —
/// `crates/happenstance-testkit/tests` — so a `wasm32` test module *anywhere
/// else in the workspace* executed nowhere and nothing said so. That is not
/// hypothetical; it is what happened to `happenstance-cloudflare`'s seventy-four
/// adapter tests, which the gate compiled and never ran.
///
/// **What it looks for, and what that costs.** A `.rs` file under any
/// `crates/*/src` tree whose *code* carries a [`WASM32_TEST_GATE`] spelling.
/// Comments are stripped first, so a file discussing the gate in prose does not
/// count — the same reason [`code_only`] exists.
///
/// **Its limits, stated so they are not read as guarantees.** It sees `src`
/// trees, so a `wasm32`-only integration test outside the testkit's `tests/`
/// directory is caught by neither scan; it matches a spelling rather than
/// parsing `cfg`, so a gate written some third way evades it; and it says
/// nothing about whether a registered row's tests pass, which needs the runner.
/// What it does guarantee is that the two spellings this workspace actually uses
/// cannot appear in a package the execution registry has never heard of.
///
/// # Errors
///
/// Returns an error if a crate directory or source cannot be read, or if a
/// registered row's `source_dir` holds no gated module at all — which means the
/// row is aimed somewhere the tests are not.
fn unregistered_wasm_unit_packages() -> Result<Vec<String>> {
    let root = workspace_root()?;
    let mut gated: Vec<String> = Vec::new();

    for entry in fs::read_dir(root.join("crates")).context("reading crates/")? {
        let crate_dir = entry.context("reading an entry of crates/")?.path();
        let manifest = crate_dir.join("Cargo.toml");
        if !manifest.is_file() {
            continue;
        }
        if !tree_has_wasm32_test_gate(&crate_dir.join("src"), WASM32_TEST_GATE)? {
            continue;
        }
        let manifest = fs::read_to_string(&manifest)
            .with_context(|| format!("reading {}", manifest.display()))?;
        gated.push(package_name(&manifest)?);
    }
    gated.sort();

    // The registry must be aimed at something. A row whose tree carries no gated
    // module is a row that would keep passing while the tests it names moved
    // away — the same failure the scan above exists to catch, one level in.
    for unit in WASM_UNIT_TARGETS {
        if !tree_has_wasm32_test_gate(&root.join(unit.source_dir), unit.gate)? {
            bail!(
                "{}'s row points at {}, which carries no `{}` module. The row is \
                 aimed somewhere the wasm32 tests are not, so its silence would \
                 mean nothing.",
                unit.package,
                unit.source_dir,
                unit.gate[0]
            );
        }
    }

    Ok(gated
        .into_iter()
        .filter(|package| {
            !WASM_UNIT_TARGETS
                .iter()
                .any(|unit| unit.package == *package)
        })
        .collect())
}

/// Whether any `.rs` file under `dir` carries one of `gate`'s spellings.
fn tree_has_wasm32_test_gate(dir: &Path, gate: &[&str]) -> Result<bool> {
    if !dir.is_dir() {
        return Ok(false);
    }
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry
            .with_context(|| format!("reading an entry of {}", dir.display()))?
            .path();
        if path.is_dir() {
            if tree_has_wasm32_test_gate(&path, gate)? {
                return Ok(true);
            }
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let source =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let code: String = code_only(&source)
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        if gate.iter().any(|spelling| code.contains(spelling)) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// The `name` of a package, read out of its manifest.
///
/// A four-line parse rather than `cargo metadata`, for the reason
/// [`locked_wasm_bindgen_version`] reads `Cargo.lock` directly: this check costs
/// no dependency resolution, and it runs on every machine.
///
/// # Errors
///
/// Returns an error if the manifest declares no `name`, which would mean it is
/// not a package manifest at all.
fn package_name(manifest: &str) -> Result<String> {
    manifest
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("name = "))
        .map(|name| name.trim_matches('"').to_owned())
        .context("a crate manifest declares no `name`")
}

/// Every rule a [`RuleFamily`]'s enumeration declares.
///
/// Derived rather than transcribed, and that is the opposite of the choice
/// [`META_TESTS`] makes one screen up — deliberately, because the two lists are
/// answering different questions. `META_TESTS` is transcribed so that a rename
/// has something to disagree with. This is derived so that **adding** a rule
/// needs no edit here while a rule that exists on the host and never reaches
/// `wasm32` still fails: an exhaustive check that a hand-list could only
/// approximate, over a set that grows.
///
/// Parameterised by family rather than reading one fixed path, for the reason
/// [`RuleFamily`] gives: the parser is identical for both enumerations — the two
/// macros have the same body shape — and it was only the hard-coded path that
/// made the projection harness unregisterable.
///
/// # Errors
///
/// Returns an error if the enumeration cannot be read, if it has moved, or if it
/// parses to fewer than two names — all three of which would otherwise silently
/// weaken every check built on it into a check of nothing.
fn enumerated_rules(family: &RuleFamily) -> Result<Vec<String>> {
    let RuleFamily {
        source: file, head, ..
    } = *family;
    let body = read_source(file)?;

    let mut lines = body.lines().skip_while(|l| l.trim() != head);
    if lines.next().is_none() {
        bail!("{file} has no `{head}` — the enumeration has moved or gone");
    }

    let mut rules = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        // The macro body ends at the first `}` in column zero.
        if line.starts_with('}') {
            break;
        }
        if trimmed.starts_with("//") {
            continue;
        }
        let Some(name) = trimmed.strip_suffix(',') else {
            continue;
        };

        // Refused rather than skipped, and the difference is the whole value of
        // this parser. A name it cannot read is silently *absent* from every
        // check built on this list — so a rule renamed into a spelling the scan
        // does not recognise would shrink the enumeration and pass, which is the
        // vacuity failure one level in from the one this file exists for. The
        // suite's rules are `snake_case` without exception; anything else here
        // is a shape change that has to be looked at.
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            bail!(
                "{file} lists `{name}`, which is not a snake_case rule \
                 name. The enumeration's shape has changed; a name this scan cannot \
                 read is a rule silently missing from every check built on it."
            );
        }

        rules.push(name.to_owned());
    }

    if rules.len() < 2 {
        bail!(
            "{file}'s `{}` parsed to {} rule(s); the enumeration's shape has \
             changed and every check built on it is now checking nothing",
            family.enumeration,
            rules.len()
        );
    }

    Ok(rules)
}

/// The arguments both cargo invocations for one artefact share.
///
/// `--all-features` matters for a reason that is not about coverage: the `tests`
/// step above this one in the gate already builds the whole workspace with every
/// feature on, and a step that differs only in its feature set gets a different
/// fingerprint and rebuilds the crate and every one of its test targets from
/// scratch. Matching the feature set makes this step reuse those artifacts, which
/// is the difference between a few seconds and none — per entry, because each
/// entry is a separate pair of invocations and one that dropped the flag would
/// pay the whole rebuild on its own.
///
/// Borrowed rather than owned: every field is `&'static str`, so the arguments
/// need no allocation even though they are built per entry.
fn cargo_args(artefact: &Artefact) -> [&'static str; 7] {
    [
        "test",
        "--locked",
        "-p",
        artefact.package,
        "--all-features",
        "--test",
        artefact.target,
    ]
}

/// Asserts every registered artefact's named tests exist, then runs them.
///
/// # Errors
///
/// Returns an error if a test target cannot be built or enumerated, if any name
/// an [`Artefact`] lists is absent from its listing, or if the tests themselves
/// fail.
pub(crate) fn run() -> Result<()> {
    for artefact in ARTEFACTS {
        check(artefact)?;
    }

    Ok(())
}

/// One artefact: assert its names out of `--list`, report, then run it.
fn check(artefact: &Artefact) -> Result<()> {
    let Artefact {
        package,
        target,
        tests,
        registry,
    } = *artefact;

    let listed = list(
        &cargo_args(artefact),
        &[],
        &format!("`{package}`'s `{target}`"),
    )?;

    let absent: Vec<&str> = tests
        .iter()
        .copied()
        .filter(|name| !listed.iter().any(|line| line == name))
        .collect();

    if !absent.is_empty() {
        let count = absent.len();
        bail!(
            "`{package}`'s `{target}` is missing {count} of the tests the gate names: {absent:?}\n\n\
             The target exists and builds, so `cargo test` would have exited 0 \
             with nothing to say. These are the clauses' own names — if one was \
             renamed deliberately, update `xtask/src/proof.rs` and the clause in \
             `SPECIFICATION.md` that cites it, in the same change. Listed: {listed:?}"
        );
    }

    let present = tests.len();
    if let Some(registry) = registry {
        // Reported beside its own entry rather than made a column every entry has
        // to answer, and keyed to *this* artefact rather than to its package: the
        // count is about one registry, and two targets in one package have two.
        println!(
            "{package}/{target}: {present} named tests present, {} registry rows",
            registry_len(registry)?
        );
    } else {
        println!("{package}/{target}: {present} named tests present");
    }

    // `.output()` rather than `.status()`, and the transcript forwarded
    // verbatim, so the step's log is unchanged for a reader. What changes is
    // that the run's own report is now *read* on the way past: the exit status
    // cannot tell `2 passed` from `0 passed; 2 ignored`, and neither can the
    // `--list` assertion above it — libtest prints an ignored test's name in
    // that listing exactly as it prints a running one's.
    let run = Command::new("cargo")
        .args(cargo_args(artefact))
        .output()
        .with_context(|| format!("failed to launch `cargo test` for `{package}`'s `{target}`"))?;

    let transcript = String::from_utf8_lossy(&run.stdout);
    print!("{transcript}");
    eprint!("{}", String::from_utf8_lossy(&run.stderr));

    if !run.status.success() {
        bail!(
            "`{package}`'s `{target}` proof artefact failed with {}",
            run.status
        );
    }

    let silent = unexecuted(tests, &transcript);
    if !silent.is_empty() {
        let count = silent.len();
        bail!(
            "`{package}`'s `{target}` exited 0 without running {count} of the tests \
             the gate names: {silent:?}\n\n\
             The target built and libtest was happy — an `#[ignore = \"…\"]` costs \
             nothing but a zero in the `passed` column, and `clippy::pedantic`'s \
             `ignore_without_reason` hands an author that exact spelling in its own \
             `help:` line. These are the clauses' own names. If one was silenced \
             deliberately, the clause in `SPECIFICATION.md` that cites it is now \
             checked by nothing, and that is the change to make first."
        );
    }

    Ok(())
}

/// The arguments one wasm32 target's two cargo invocations share.
///
/// **No `--all-features`**, and this is the one flag whose absence is worth a
/// comment. [`cargo_args`] carries it for a fingerprint-sharing reason that is
/// entirely about the host, and here it does not merely fail to help — it does
/// not compile. `happenstance-testkit`'s `proptest` feature maps to a dependency
/// declared only under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`,
/// and a Cargo feature is not target-scoped, so `--all-features` sets it on this
/// target too. The two existing `wasm32` steps pass no feature flags for exactly
/// this reason (`main.rs:231-243`), and this matches them — which is also what
/// keeps their build artifacts shared rather than rebuilt (NF-001).
fn wasm_cargo_args(wasm: &WasmTarget) -> [&'static str; 8] {
    [
        "test",
        "--locked",
        "-p",
        wasm.package,
        "--test",
        wasm.target,
        "--target",
        WASM_TRIPLE,
    ]
}

/// The `wasm-bindgen` version the committed lock file resolves.
///
/// Derived, never hard-coded. `wasm-bindgen-cli` must match the `wasm-bindgen`
/// in the dependency graph *exactly* — the runner refuses a mismatched schema,
/// and is right to — so a pinned number here would mean a `cargo update` leaves
/// a runner that no longer matches, failing as a confusing runtime error instead
/// of as a version bump. `.github/workflows/ci.yml` reads it out of
/// `cargo metadata` for the same reason; this reads `Cargo.lock` directly so the
/// check costs no dependency resolution.
///
/// # Errors
///
/// Returns an error if `Cargo.lock` cannot be read or if `wasm-bindgen` is not
/// in it — which would mean the harness this step executes no longer exists.
fn locked_wasm_bindgen_version() -> Result<String> {
    let lock = read_source("Cargo.lock")?;

    let mut lines = lock
        .lines()
        .skip_while(|l| l.trim() != format!("name = \"{WASM_BINDGEN}\""));
    if lines.next().is_none() {
        bail!(
            "`{WASM_BINDGEN}` is absent from Cargo.lock, so there is no wasm32 test \
             harness for the gate to execute"
        );
    }

    for line in lines.take(4) {
        if let Some(version) = line.trim().strip_prefix("version = ") {
            return Ok(version.trim_matches('"').to_owned());
        }
    }

    bail!("Cargo.lock's `{WASM_BINDGEN}` package declares no version")
}

/// Refuses a runner whose schema does not match the graph (EC-002).
///
/// The mismatch is otherwise reported by the runner as an opaque failure part
/// way through a test binary, which reads as *the conformance suite broke* when
/// it means *reinstall one tool*. Naming the two versions and the command that
/// reconciles them is the difference.
fn check_wasm_runner_version() -> Result<()> {
    let expected = locked_wasm_bindgen_version()?;

    let output = Command::new(WASM_RUNNER)
        .arg("--version")
        .output()
        .with_context(|| format!("failed to launch `{WASM_RUNNER} --version`"))?;

    let reported = String::from_utf8_lossy(&output.stdout).into_owned();
    let installed = reported.split_whitespace().last().unwrap_or_default();

    if installed != expected {
        bail!(
            "`{WASM_RUNNER}` is {installed} and Cargo.lock resolves `{WASM_BINDGEN}` \
             {expected}. The runner refuses a mismatched schema, so this would fail \
             part way through a test binary as though the suite had broken.\n\n\
             \tcargo install wasm-bindgen-cli --version {expected} --locked"
        );
    }

    println!("{WASM_RUNNER} {installed} matches Cargo.lock's {WASM_BINDGEN} {expected}");
    Ok(())
}

/// The head of a list of missing names, with the rest counted rather than
/// printed.
///
/// An emptied target is missing *every* rule, and a failure message carrying 89
/// fully-qualified names is one a reader scrolls past rather than reads —
/// which costs the message the one property a build failure has to have, that
/// the reader can act on it without re-running anything. Ten names and a count
/// says both *what* is missing and *how much*.
fn first_few(names: &[String]) -> String {
    const SHOWN: usize = 10;

    let head = names
        .iter()
        .take(SHOWN)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ");

    match names.len().checked_sub(SHOWN) {
        Some(rest) if rest > 0 => format!("{head}, and {rest} more"),
        _ => head,
    }
}

/// Executes every registered conformance target on `wasm32-unknown-unknown`.
///
/// Three assertions before anything runs, because the failure this guards
/// against is a green exit over nothing:
///
/// 1. The installed runner matches the lock file.
/// 2. Every rule the row's own [`RuleFamily`] declares appears in the target's
///    own `--list`. Exhaustive and derived, so a rule added upstream needs no
///    edit here, and a rule `#[cfg]`-ed out of the wasm harness — the shape
///    project AC-002 forbids — fails. **Per row**, not once for the array: a
///    projection target held to the event-store enumeration fails on all
///    seventeen of its rules, which is what made the earlier one-enumeration
///    version of this function unable to hold a projection row at all.
/// 3. Every rule the row *names* appears too. Transcribed, so a rename has
///    something to disagree with; see [`MEMORY_WASM_RULES`].
///
/// Then the rules run under `--nocapture`. That flag is not noise: `println!` is
/// a silent discard on this target, so `__emit_wasm` reports a declined
/// capability's `SKIP <rule>: <reason>` through `console_log!`, and without the
/// flag the runner swallows it. It is the target's own idiom for the
/// `--show-output` the gate's host `tests` step carries — and it is not a
/// synonym: `--show-output` is rejected outright by this runner.
///
/// # Errors
///
/// Returns an error if the runner is absent or mismatched, if a target cannot be
/// built or enumerated, if any rule the enumeration declares is missing from a
/// target's listing, or if the rules themselves fail.
pub(crate) fn wasm_run() -> Result<()> {
    check_wasm_runner_version()?;

    let env = [(WASM_RUNNER_VAR, WASM_RUNNER)];

    for wasm in WASM_TARGETS {
        let enumerated = enumerated_rules(wasm.family)?;
        let label = format!("`{}`'s `{}` on {WASM_TRIPLE}", wasm.package, wasm.target);
        let args = wasm_cargo_args(wasm);
        let listed = list(&args, &env, &label)?;

        let qualified = |rule: &str| format!("{}::{rule}", wasm.module);
        let absent: Vec<String> = enumerated
            .iter()
            .map(|rule| qualified(rule))
            .filter(|name| !listed.iter().any(|line| line == name))
            .collect();

        if !absent.is_empty() {
            bail!(
                "{label} is missing {} of the {} rules `{}` declares: {}\n\n\
                 The target builds, so `cargo test` would have exited 0 with nothing \
                 to say. The rule set is single-sourced through that one enumeration \
                 — a rule that reaches the host harnesses and not this one is a \
                 wasm32-only subset, which is the outcome this step exists to \
                 forbid. Listed: {} name(s).",
                absent.len(),
                enumerated.len(),
                wasm.family.enumeration,
                first_few(&absent),
                listed.len()
            );
        }

        let unnamed: Vec<&str> = wasm
            .rules
            .iter()
            .copied()
            .filter(|rule| !listed.iter().any(|line| *line == qualified(rule)))
            .collect();

        if !unnamed.is_empty() {
            bail!(
                "{label} is missing {} of the rules the gate names: {unnamed:?}\n\n\
                 These are named in `xtask/src/proof.rs` rather than derived, so \
                 that a rename has something to disagree with. If one moved \
                 deliberately, update this row's `rules` list in the same change.",
                unnamed.len()
            );
        }

        println!(
            "{}/{}: {} rules enumerated, {} named, executing on {WASM_TRIPLE}",
            wasm.package,
            wasm.target,
            enumerated.len(),
            wasm.rules.len()
        );

        let status = Command::new("cargo")
            .args(args)
            // `--nocapture` and not `--show-output`: the host flavour of this
            // flag is rejected by `wasm-bindgen-test-runner` outright.
            .args(["--", "--nocapture"])
            .envs(env)
            .status()
            .with_context(|| format!("failed to launch `cargo test` for {label}"))?;

        if !status.success() {
            bail!("{label} failed with {status}");
        }
    }

    wasm_unit_run(&env)
}

/// Executes every registered non-conformance `wasm32` target.
///
/// The same two assertions the loop above makes, minus the one that cannot
/// apply: there is no [`RuleFamily`] enumeration to be exhaustive against, so
/// what is checked is that every test the row *names* is still in the target's
/// own `--list`. That is what a rename or a deleted module has to disagree with.
///
/// The host requirement is printed before the run rather than after the failure.
/// These targets talk to a real SQLite through `node:sqlite`, so on a machine
/// whose Node is too old they fail inside a test body with a JS exception, and
/// the reader's first conclusion would be that the adapter is broken.
///
/// # Errors
///
/// Returns an error if a target cannot be built or enumerated, if a named test
/// is absent from its listing, or if the tests themselves fail.
fn wasm_unit_run(env: &[(&str, &str); 1]) -> Result<()> {
    for unit in WASM_UNIT_TARGETS {
        let label = format!(
            "`{}`'s {} on {WASM_TRIPLE}",
            unit.package,
            unit.selector.join(" ")
        );
        let args = wasm_unit_cargo_args(unit);
        let listed = list(&args, env, &label)?;

        let absent: Vec<&str> = unit
            .tests
            .iter()
            .copied()
            .filter(|test| !listed.iter().any(|line| line == test))
            .collect();

        if !absent.is_empty() {
            bail!(
                "{label} is missing {} of the tests the gate names: {absent:?}\n\n\
                 The target builds, so `cargo test` would have exited 0 with \
                 nothing to say about them. These are named in \
                 `xtask/src/proof.rs` rather than derived, so that a rename — or \
                 a `#[cfg]` that quietly stops compiling a module — has something \
                 to disagree with. Listed: {} test(s).",
                absent.len(),
                listed.len()
            );
        }

        println!(
            "{}/{}: {} tests listed, {} named, executing on {WASM_TRIPLE}\n  host: {}",
            unit.package,
            unit.selector.join(" "),
            listed.len(),
            unit.tests.len(),
            unit.host
        );

        let status = Command::new("cargo")
            .args(&args)
            .args(["--", "--nocapture"])
            .envs(env.iter().copied())
            .status()
            .with_context(|| format!("failed to launch `cargo test` for {label}"))?;

        if !status.success() {
            bail!(
                "{label} failed with {status}\n\n\
                 If every case failed at once, check the host before the code: {}",
                unit.host
            );
        }
    }

    Ok(())
}

/// The arguments one non-conformance `wasm32` target's two cargo invocations
/// share.
///
/// A `Vec` rather than [`wasm_cargo_args`]'s fixed array because the selector is
/// one token for `--lib` and two for `--test <name>`, and a row that could only
/// ever be a `--lib` would need this function rewritten the first time an
/// integration test needed executing.
fn wasm_unit_cargo_args(unit: &WasmUnitTarget) -> Vec<&'static str> {
    let mut args = vec!["test", "--locked", "-p", unit.package];
    args.extend_from_slice(unit.selector);
    args.extend_from_slice(&["--target", WASM_TRIPLE]);
    args
}

/// Asserts the executed targets exist and are wired to their own rule set — with
/// no runner, on every machine.
///
/// This is the compensating half of the shape `main.rs` chose for the execution
/// step, and the reason that step is allowed a probe at all. `wasm-bindgen-cli`
/// is a `cargo install`ed binary pinned to a schema version;
/// `rust-toolchain.toml` can pin a *target* and cannot install a *binary*, so
/// the runner is genuinely absent on some machines and a mandatory step there
/// fails for a reason that is not about the code. What may **not** happen is the
/// skip taking the guard with it: everything below runs whether or not the
/// runner exists, and an emptied, rewired or hand-subsetted target fails here.
///
/// What it deliberately does not claim: that the rules *passed*, or that any
/// individual rule is not `#[ignore]`d. Those need the runner, and
/// [`wasm_run`] is where they are checked. A guard whose limits are undocumented
/// is read as a guarantee.
///
/// # Errors
///
/// Returns an error if no target is registered, if a target's source is missing,
/// carries a block comment the code scan cannot read, or no longer invokes its
/// family's suite through that family's `wasm32` emitter, if a row names no
/// rules, if a named rule is absent from the enumeration, or if a target writes
/// a rule list of its own.
pub(crate) fn wasm_enumeration() -> Result<()> {
    if WASM_TARGETS.is_empty() {
        bail!(
            "no conformance target is registered for execution on {WASM_TRIPLE}. \
             Removing the last row silently reduces the gate's wasm32 claim to a \
             `cargo check`, which is what this check exists to refuse."
        );
    }

    // Before the rows are checked, the *set* of rows is. Everything below asks
    // whether a registered target is honest; this asks whether the registration
    // is complete, which is the question a per-row loop cannot reach and the one
    // that was got wrong. See `unregistered_wasm_harnesses`.
    // The same question for the targets that run no conformance rules. Asked
    // separately because the scan above reads one directory and this one reads
    // every crate's `src` tree — and the gap between those two sentences is
    // where seventy-four executed-by-nothing adapter tests lived.
    let unregistered = unregistered_wasm_unit_packages()?;
    if !unregistered.is_empty() {
        bail!(
            "{} package(s) declare wasm32-only test modules with no row in \
             WASM_UNIT_TARGETS: {unregistered:?}\n\n\
             Each compiles for that target and executes nowhere. A `#[cfg(all(test, \
             target_arch = \"wasm32\"))]` module is invisible to every host \
             `cargo test`, so the only thing that can run one is a row here.",
            unregistered.len()
        );
    }
    for unit in WASM_UNIT_TARGETS {
        if unit.tests.is_empty() {
            bail!(
                "{}'s row names no tests, so a rename would have nothing to \
                 disagree with",
                unit.package
            );
        }
    }

    let unregistered = unregistered_wasm_harnesses()?;
    if !unregistered.is_empty() {
        bail!(
            "{HARNESS_DIR} holds {} wasm32 conformance harness(es) with no row in \
             WASM_TARGETS: {unregistered:?}\n\n\
             Each drives a suite through a wasm32 emitter, so each compiles for \
             that target — and nothing executes it. The retired `wasm-conformance` \
             CI job ran the whole package and named no target individually; the \
             note that retired it claims this gate is strictly more, and a row \
             short that is false rather than approximate.",
            unregistered.len()
        );
    }

    for wasm in WASM_TARGETS {
        let enumerated = enumerated_rules(wasm.family)?;
        let source = read_source(wasm.source)?;

        // The needles are looked for in the *code*, not in the file. A harness
        // whose module documentation quotes `__emit_wasm` while its expansion
        // goes through `__emit_tokio` is precisely the target this guard exists
        // to fail, and a whole-file `contains` passes it on the strength of the
        // prose.
        if source.contains("/*") {
            bail!(
                "{} carries a block comment, which `code_only` cannot read — so \
                 every check below it would be scanning text this guard believes \
                 is code. Use line comments here, or teach `code_only` to strip \
                 block ones before this file needs them.",
                wasm.source
            );
        }
        let code = code_only(&source);

        for (needle, complaint) in [
            (
                wasm.family.suite,
                "no longer invokes the conformance suite; an emptied target exits 0 \
                 on `running 0 tests`",
            ),
            (
                wasm.family.emitter,
                "does not emit through its family's wasm32 emitter, so whatever it \
                 runs is not the wasm32 harness — the tokio emitter type-checks \
                 here and cannot run here",
            ),
            (
                wasm.module,
                "no longer declares the module the gate expects its rules under",
            ),
        ] {
            if !code.contains(needle) {
                bail!("{} {complaint} (looked for `{needle}`)", wasm.source);
            }
        }

        if wasm.rules.is_empty() {
            bail!(
                "{}'s row names no rules, so a rename would have nothing to \
                 disagree with",
                wasm.target
            );
        }

        for rule in wasm.rules {
            if !enumerated.iter().any(|name| name == rule) {
                bail!(
                    "{}'s row names `{rule}`, which `{}` does not declare. Either \
                     the rule was renamed and this list was not, or this list names \
                     a rule that never existed.",
                    wasm.target,
                    wasm.family.enumeration
                );
            }
        }

        // Single-sourcing, checked at the one place a subset would be written —
        // and over the code alone, because a harness is *expected* to discuss its
        // own rules in prose. `local_conformance.rs` names some by hand — where
        // they moved to when they became suite rules, and which one `LocalFixture`
        // declines — and a whole-file scan reads that prose as a subset list and
        // refuses a correct file.
        let hand_written: Vec<&String> = enumerated
            .iter()
            .filter(|rule| code.contains(rule.as_str()))
            .collect();

        if !hand_written.is_empty() {
            bail!(
                "{} names {} conformance rule(s) in its own code: {hand_written:?}\n\n\
                 The rule set is single-sourced through `{}`. A list here is a \
                 wasm32-only subset — a second enumeration that can drift from the \
                 first, which is the outcome the project's AC-002 forbids by \
                 construction.",
                wasm.source,
                hand_written.len(),
                wasm.family.enumeration
            );
        }

        println!(
            "{}/{}: {} named rules, all declared by `{}`'s enumeration of {}",
            wasm.package,
            wasm.target,
            wasm.rules.len(),
            wasm.family.enumeration,
            enumerated.len()
        );
    }

    Ok(())
}

/// How many subjects the mutant registry declares.
///
/// Printed rather than asserted, and the distinction is the point: the count is
/// a fact about the tree that `CHANGELOG.md` and `RUNBOOK.md` both quote in
/// prose ("fifty wrong implementations", "fifty-two stores drive the suite"), and
/// a number quoted in two documents and computed nowhere goes stale the first
/// time a mutant lands — which it did: the runbook's stage-6 session log read
/// "41 → 50 registered subjects" against a registry of 52, and this line is what
/// the correction was checked with. Printing it beside
/// the meta-test count — which `main.rs` already prints for the same reason —
/// puts the current answer in the gate's own output, where the person editing
/// those sentences is looking.
///
/// A *pinned* count would be worse than useless here. `mutant_registry_is_exhaustive`
/// already holds `REGISTRY` to the stores that exist, so an expected number in
/// this file would add nothing but an edit every time a mutant lands — and
/// CLAUDE.md forbids quoting a pass rate over the mutant set precisely because
/// the denominator is a choice. This reports the denominator; it does not
/// legislate it.
///
/// # Errors
///
/// Returns an error if the registry file cannot be read, if it declares no
/// `REGISTRY`, or if the declaration is not terminated — all three of which
/// would otherwise print a plausible `0`.
fn registry_len(file: &str) -> Result<usize> {
    let root = workspace_root()?;
    let body = fs::read_to_string(root.join(file)).with_context(|| format!("reading {file}"))?;

    let mut lines = body.lines().skip_while(|l| l.trim() != REGISTRY_HEAD);
    if lines.next().is_none() {
        bail!("{file} has no `{REGISTRY_HEAD}` — the mutant registry has moved or gone");
    }

    let mut rows = 0usize;
    for line in lines {
        // The declaration ends at the first `];` in column zero. Rows are one
        // indentation level in, so nothing inside a row can end the scan.
        if line.starts_with("];") {
            return Ok(rows);
        }
        if line.trim() == "Declared {" {
            rows += 1;
        }
    }

    bail!("{file}'s `REGISTRY` declaration is never closed by a `];` at column zero")
}

/// Every test name libtest reports for one artefact's target.
///
/// libtest prints one `name: test` line per test, plus a trailing summary. The
/// suffix is stripped rather than the line split on `:`, because a test name
/// contains `::` and a split would take the wrong half; lines that do not carry
/// the suffix are the summary and are dropped.
/// Takes the invocation as arguments rather than as an [`Artefact`] because the
/// wasm32 rows share the parsing and share none of the flags: a different
/// target, no `--all-features`, and a runner variable the host rows must never
/// see. Generalising the parameter is what let the wasm entry reuse this without
/// touching what the host rows pass.
fn list(args: &[&str], env: &[(&str, &str)], label: &str) -> Result<Vec<String>> {
    let output = Command::new("cargo")
        .args(args)
        .args(["--", "--list"])
        .envs(env.iter().copied())
        .output()
        .with_context(|| format!("failed to launch `cargo test -- --list` for {label}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("could not enumerate {label}'s tests:\n{}", stderr.trim());
    }

    let listing = String::from_utf8_lossy(&output.stdout).into_owned();
    Ok(listing
        .lines()
        .filter_map(|line| line.trim().strip_suffix(": test"))
        .map(str::to_owned)
        .collect())
}

/// The named tests one run's own output does not report as having **passed**.
///
/// # Why the run's output, and not a second listing
///
/// The obvious route is closed. `cargo test -- --list --ignored` would name the
/// silenced tests outright, but the locked `wasm-bindgen-test 0.3.76`
/// (`Cargo.lock:1985-1986`) offers `--include-ignored` and no run-only-ignored
/// mode, so it is a mechanism the two arms of this file could never share. A
/// run's own stdout is the one surface every libtest-shaped runner here prints
/// in the same shape — and this file was already producing it and throwing it
/// away, which is the decorative-gate failure one level up from the one the
/// module documentation opens with.
///
/// # Why each name, and not the reported `passed` count
///
/// Comparing `tests.len()` against the run's `passed` is the cheaper
/// comparison and it answers a different question: `2 passed` is also what a
/// target prints whose two *named* tests were `#[ignore]`d and two others
/// added. The names are what the clauses cite and what [`ARTEFACTS`] exists to
/// hold, so the outcome is read per name.
///
/// The parse is libtest's per-test outcome line, `test <name> ... ok`, trimmed
/// and matched whole. A name that appears only on an `ignored` or a `FAILED`
/// line is *not* reported as having passed — which is the distinction a
/// substring search over the same text cannot make, because an ignored test
/// prints its name too.
///
/// # What this does not observe
///
/// Only the names an [`Artefact`] carries, and only for the targets [`check`]
/// runs. A tenth test inside one of those targets may still be `#[ignore]`d
/// without failing here, deliberately and for the reason the module
/// documentation gives for the subset check — what may not happen in silence is
/// a name a clause cites going quiet.
pub(crate) fn unexecuted<'a>(named: &[&'a str], run_output: &str) -> Vec<&'a str> {
    let passed: Vec<&str> = run_output
        .lines()
        .map(str::trim)
        .filter_map(|line| line.strip_prefix("test "))
        .filter_map(|line| line.strip_suffix(" ... ok"))
        .collect();

    named
        .iter()
        .copied()
        .filter(|name| !passed.contains(name))
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    /// The phase-6 evidence document, read by the assertions below.
    ///
    /// It lives here rather than beside the gate's own items because its only
    /// readers are in this module, and a `#[cfg(test)]` item in the middle of the
    /// file is one an editor can drop a doc comment onto by accident — which is
    /// exactly what happened to [`list`] when it first landed there.
    ///
    /// Reached through [`workspace_root`] rather than `include_str!` on purpose:
    /// this file is `xtask`'s, the document is `references/`'s, and a
    /// compile-time include would make a `publish = false` tool's build depend on
    /// a markdown file it has no other relationship with. [`registry_len`] already
    /// reaches the tree the same way.
    ///
    /// **Repointed at every supersession, and that is the whole discipline.**
    /// `references/evaluation/` documents are immutable and superseded rather
    /// than edited, so the artefact this names is replaced by a later dated one
    /// rather than corrected in place — and a constant left on the earlier file
    /// keeps passing while holding a document nobody will read next. It named
    /// `phase-6-projection-proof.md` until `fresh_projection_has_no_checkpoint`
    /// landed and that document stopped describing the tree.
    const PHASE_6_PROOF: &str = "references/evaluation/phase-6-projection-proof-at-closeout.md";

    fn proof_document() -> String {
        let root = workspace_root().unwrap();
        fs::read_to_string(root.join(PHASE_6_PROOF))
            .unwrap_or_else(|e| panic!("reading {PHASE_6_PROOF}: {e}"))
    }

    /// Every registry count is printed beside the target it is about.
    ///
    /// `check()` used to select the count by **package** alone, and `registry_len`
    /// read one hard-coded path. A second `happenstance-testkit` row therefore
    /// printed the *event-store* registry's row count against the projection
    /// target — a number that is not about that target at all, which is the very
    /// defect `registry_len`'s own documentation was written about, one level up.
    #[test]
    fn a_registry_count_belongs_to_the_target_it_is_printed_beside() {
        let mut seen: Vec<&str> = Vec::new();
        for artefact in ARTEFACTS {
            let Some(registry) = artefact.registry else {
                continue;
            };
            assert!(
                !seen.contains(&registry),
                "two artefacts name `{registry}`; a count shared by two rows is a \
                 count about neither"
            );
            seen.push(registry);
            assert!(
                registry.contains(artefact.target),
                "`{}`'s registry is `{registry}`, which does not name that target",
                artefact.target
            );
        }
        assert!(
            seen.len() >= 2,
            "the defect this test exists for is only reachable with two registries; \
             found {}",
            seen.len()
        );
    }

    /// The projection family is held by the gate at all.
    ///
    /// A test target that runs but whose names nothing asserts is the library
    /// equivalent of a component that renders nowhere: `cargo test --workspace`
    /// passes just as happily with one fewer target as with one more.
    #[test]
    fn the_phase_six_targets_are_held() {
        for target in ["projection_mutation_coverage", "projection_harness_parity"] {
            assert!(
                ARTEFACTS
                    .iter()
                    .any(|a| a.package == REGISTRY_PACKAGE && a.target == target),
                "`{target}` is in no `ARTEFACTS` row, so its test names can be \
                 renamed, `#[ignore]`d or emptied in silence"
            );
        }
    }
    /// The project's first Definition-of-Done item asks for a **test name**, and
    /// there are two of them.
    ///
    /// The conformance rule `CheckpointOnlyStore` fails, and the meta-test that
    /// asserts it fails exactly there. Quoting one and letting the reader assume
    /// the other is the single easiest way to make the artefact useless, because
    /// the gate exits zero *because* the mutant failed where it was declared to.
    #[test]
    fn the_artefact_names_both_tests_and_the_mutant() {
        let doc = proof_document();
        for name in [
            "CheckpointOnlyStore",
            "commit_is_atomic_with_the_read_model",
            "projection_mutants_fail_exactly_their_declared_rules",
        ] {
            assert!(
                doc.contains(name),
                "the phase-6 proof artefact never names `{name}`"
            );
        }
    }

    /// The second Definition-of-Done item asks for **two fixture names**, and a
    /// green run is consistent with a second batch shape that is the oracle
    /// wearing a hat.
    #[test]
    fn the_artefact_names_both_batch_shapes() {
        let doc = proof_document();
        for fixture in ["MemoryProjectionFixture", "BufferingProjectionFixture"] {
            assert!(
                doc.contains(fixture),
                "the phase-6 proof artefact never names `{fixture}`"
            );
        }
    }

    /// A ratio over a mutant set, in any spelling — or `None`.
    ///
    /// Shape-shaped rather than denominator-shaped, and that is the whole point.
    /// The check this replaced named `19`, which is today's registry size: the
    /// sentence `48 of 50 mutants caught` passed it, and a twentieth mutant would
    /// have retired the guard without anything going red. Regex-free because
    /// `xtask` carries no regex dependency and a digit scan is all the shape needs.
    ///
    /// Expects a lowercased line, because the two phrase forms are matched
    /// literally. A count of *rules* is a statement about the enumeration and
    /// stays allowed — only a fraction whose denominator is a choice is caught.
    fn forbidden_ratio(line: &str) -> Option<String> {
        for phrase in ["pass rate", "caught out of"] {
            if line.contains(phrase) {
                return Some(format!("the phrase `{phrase}`"));
            }
        }
        let mut from = 0usize;
        while let Some(offset) = line[from..].find(|c: char| c.is_ascii_digit()) {
            let start = from + offset;
            let end = line[start..]
                .find(|c: char| !c.is_ascii_digit())
                .map_or(line.len(), |o| start + o);
            for sep in ["/", " of "] {
                if let Some(rest) = line[end..].strip_prefix(sep)
                    && rest.starts_with(|c: char| c.is_ascii_digit())
                {
                    let digits = rest
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(rest.len());
                    return Some(format!("`{}`", &line[start..end + sep.len() + digits]));
                }
            }
            from = end;
        }
        None
    }

    /// The guard reads the shape, not the registry's current size.
    ///
    /// This is the test that fails against the check this one replaced: `48 of 50`
    /// names neither `19` nor `/19`, so the old spelling-by-denominator let the
    /// forbidden sentence straight through, and would have stopped seeing even
    /// `19 of 19` on the day a twentieth mutant landed.
    #[test]
    fn the_ratio_guard_reads_the_shape_rather_than_the_registry_size() {
        for forbidden in [
            "48 of 50 mutants caught",
            "19 of 19 mutants fail exactly where declared",
            "the mutant set: 19/19",
            "20/20 mutants",
            "mutant pass rate: 100%",
            "18 caught out of nineteen mutants",
        ] {
            assert!(
                forbidden_ratio(forbidden).is_some(),
                "`{forbidden}` is a ratio over the mutant set and the guard missed it"
            );
        }
        for allowed in [
            "nineteen mutants, each declared against the rules it fails",
            "19 mutants are registered, and 19 rules name one",
            "crates/happenstance-testkit/tests/mutation_coverage.rs:141-186 registers the mutant",
        ] {
            assert!(
                forbidden_ratio(allowed).is_none(),
                "`{allowed}` states an enumeration rather than a ratio, and the guard \
                 flagged it: {:?}",
                forbidden_ratio(allowed)
            );
        }
    }

    /// ADR-0010's prohibition, enforced rather than remembered.
    ///
    /// The denominator over a mutant set is a choice, so a fraction reports how
    /// representative the author was while reading as though it said how good the
    /// suite is. A count of *rules* is a statement about the enumeration and is
    /// allowed; a ratio over mutants is not, in any spelling.
    #[test]
    fn the_artefact_quotes_no_pass_rate_over_the_mutant_set() {
        let doc = proof_document().to_ascii_lowercase();
        for line in doc.lines() {
            if !line.contains("mutant") {
                continue;
            }
            assert!(
                forbidden_ratio(line).is_none(),
                "a ratio over the mutant set — {} — in: {line}",
                forbidden_ratio(line).unwrap_or_default()
            );
        }
    }

    // -----------------------------------------------------------------------
    // HS-S0048 AC-003, AC-005, AC-006 — the executed wasm32 conformance targets
    // -----------------------------------------------------------------------

    /// AC-006. The executed targets are rows, not arguments.
    ///
    /// The named wrong implementation this rejects is a `Step` carrying
    /// `-p happenstance-testkit --test memory_conformance_wasm` in its own
    /// `args`: it satisfies every other criterion here and forces HS-S0054 to
    /// duplicate the whole design — a second step, a second runner variable and
    /// a second anti-vacuity guard — to run one more target.
    #[test]
    fn every_executed_wasm_target_carries_its_own_package_and_target() {
        assert!(
            !WASM_TARGETS.is_empty(),
            "no conformance target is registered for execution on wasm32, so the \
             gate's wasm32 story is still a `cargo check`"
        );

        let mut seen: Vec<(&str, &str)> = Vec::new();
        for wasm in WASM_TARGETS {
            for (label, value) in [
                ("package", wasm.package),
                ("target", wasm.target),
                ("module", wasm.module),
                ("source", wasm.source),
            ] {
                assert!(
                    !value.is_empty(),
                    "`{}` carries an empty {label}, so the row cannot address a \
                     target on its own",
                    wasm.target
                );
            }
            assert!(
                !seen.contains(&(wasm.package, wasm.target)),
                "`{}`'s `{}` is registered twice",
                wasm.package,
                wasm.target
            );
            seen.push((wasm.package, wasm.target));
        }
    }

    /// AC-003. The names the gate asserts are names the row's own enumeration
    /// emits.
    ///
    /// The duplication is the mechanism, exactly as it is for [`META_TESTS`]: a
    /// rule renamed in `registry.rs` and not here fails at this test rather than
    /// drifting into an expectation list that quietly matches nothing.
    ///
    /// Resolved per row rather than once. Held to a single enumeration this
    /// assertion would fail every projection row on every one of its rules — an
    /// answer that reads as *the projection harness must not be registered* when
    /// it means *this test read the wrong file*.
    #[test]
    fn every_named_wasm_rule_is_one_the_enumeration_declares() {
        assert!(
            !WASM_TARGETS.is_empty(),
            "no executed wasm32 target is registered, so this check has nothing \
             to disagree with"
        );
        for wasm in WASM_TARGETS {
            let enumerated = enumerated_rules(wasm.family).unwrap();
            assert!(
                enumerated.len() > 1,
                "`{}` parsed to {} names, which is a parser failure rather than a \
                 rule set",
                wasm.family.enumeration,
                enumerated.len()
            );
            assert!(
                !wasm.rules.is_empty(),
                "`{}` names no rules, so an emptied target would pass its own \
                 anti-vacuity guard",
                wasm.target
            );
            for rule in wasm.rules {
                assert!(
                    enumerated.iter().any(|name| name == rule),
                    "`{}` names `{rule}`, which `{}` does not declare — a rename \
                     that reached the enumeration and not this list",
                    wasm.target,
                    wasm.family.enumeration
                );
            }
        }
    }

    /// The Cloudflare conformance target is mounted by a **row**, and nothing
    /// else.
    ///
    /// `wasm-execution-gate-step` committed in its AC-006 that the next executed
    /// target arrives by registering a row rather than by writing a second
    /// execution step, and this is that contract cashed. It is not a restatement
    /// of the row's own fields for their own sake: what would satisfy every
    /// *other* check in this file, and fail the contract, is a second `Step` in
    /// `main.rs`'s `REQUIRED` with `-p happenstance-cloudflare --test
    /// durable_object_conformance` hard-coded into its `args` — a shape under
    /// which the registry stays three rows long, `wasm_conformance` keeps
    /// passing, and the seam has quietly grown a parallel path with its own
    /// runner wiring to drift.
    ///
    /// So the row's identity is asserted here *and* the step count is asserted
    /// beside it. The `module` in particular is load-bearing: libtest prints
    /// `<mod_name>::<rule>`, so a row whose `module` disagrees with the target's
    /// `mod_name` fails every one of the eighty-nine derived expectations at
    /// once, with a message about missing rules rather than about a typo.
    #[test]
    fn the_cloudflare_conformance_target_is_a_row_and_not_a_second_step() {
        let row = WASM_TARGETS
            .iter()
            .find(|wasm| wasm.package == "happenstance-cloudflare")
            .expect(
                "the Cloudflare conformance target has no row in WASM_TARGETS, so \
                 `cargo xtask ci` compiles it and executes nothing — which is the \
                 exact position this project started in",
            );

        assert_eq!(
            row.target, "durable_object_conformance",
            "the row must name the target `cargo test --test` selects"
        );
        assert_eq!(
            row.module, "dcb_conformance_wasm",
            "the row's module is the prefix libtest prints in front of every \
             rule, so it must equal the target's `mod_name`"
        );
        assert_eq!(
            row.family.enumeration, EVENT_STORE_FAMILY.enumeration,
            "an adapter's store conformance target is held to the event-store \
             enumeration, exhaustively"
        );
        assert_eq!(
            row.source, "crates/happenstance-cloudflare/tests/durable_object_conformance.rs",
            "the runner-free guard reads this path; a row pointing anywhere else \
             would pass on a machine with no runner while proving nothing"
        );

        let execution_steps = crate::REQUIRED
            .iter()
            .filter(|step| step.name.contains("wasm32 run of"))
            .count();
        assert_eq!(
            execution_steps, 1,
            "registering a target is adding a row, never adding a step. Two \
             execution steps means two runner wirings, two `--target` plumbings \
             and two places for the next target to be forgotten."
        );
    }

    /// HS-S0056 AC-008. The WF-11 memory-ceiling probe is mounted by a **row**.
    ///
    /// The second caller of the contract [`wasm-execution-gate-step`] wrote in
    /// its AC-006 and [`every-rule-under-workerd`] restated in its AC-003: a
    /// further executed `wasm32` target arrives as a row, never as a second
    /// `Step`. So this test asserts the row's identity *and*, beside it, the
    /// three things a parallel step would have needed — a second execution step,
    /// a second runner variable, and `--target` plumbing naming this target
    /// directly. Each of those would satisfy every other check in this file
    /// while leaving the seam with two paths to drift apart.
    ///
    /// The probe is **not** a [`WASM_TARGETS`] row: that registry holds every
    /// row to a [`RuleFamily`]'s exhaustive enumeration, and this target runs no
    /// conformance rule at all. It measures an isolate's memory ceiling and the
    /// cost of one encode, which no rule in either suite can observe.
    #[test]
    fn the_wf11_probe_is_a_row_and_not_a_second_step() {
        let row = WASM_UNIT_TARGETS
            .iter()
            .find(|unit| unit.selector == ["--test", "wf11_memory_ceiling"])
            .expect(
                "the WF-11 memory-ceiling probe has no row in WASM_UNIT_TARGETS, \
                 so `cargo xtask ci` compiles it under the existing `--tests` \
                 wasm32 check and executes it nowhere — which is the discover \
                 stage's named wrong implementation wearing a different hat",
            );

        assert_eq!(
            row.package, "happenstance-cloudflare",
            "the probe lives in the adapter's own crate, because the isolate \
             whose ceiling it measures is that adapter's runtime"
        );
        assert_eq!(
            row.source_dir, "crates/happenstance-cloudflare/tests",
            "the runner-free guard reads this tree; a row pointing anywhere else \
             would pass on a machine with no runner while proving nothing"
        );
        assert_eq!(
            row.gate, WASM32_TARGET_GATE,
            "an integration target is already `#[cfg(test)]` by being one, so its \
             wasm32-only half is written `cfg(target_arch = \"wasm32\")`"
        );

        let execution_steps = crate::REQUIRED
            .iter()
            .filter(|step| step.name.contains("wasm32 run of"))
            .count();
        assert_eq!(
            execution_steps, 1,
            "registering a target is adding a row, never adding a step"
        );
        assert!(
            crate::REQUIRED
                .iter()
                .all(|step| step.env.iter().all(|(key, _)| *key != WASM_RUNNER_VAR)),
            "`{WASM_RUNNER_VAR}` is set per-`Command` in this file and must never \
             become a second, step-level runner wiring"
        );
        assert!(
            crate::REQUIRED
                .iter()
                .all(|step| !step.args.contains(&"wf11_memory_ceiling")),
            "no gate step may name this target in its own args: that is the \
             hard-coded `--target` plumbing the row exists to make unnecessary"
        );
    }

    /// HS-S0056 AC-009. The probe's row names the tests a rename has to
    /// disagree with.
    ///
    /// `cargo test` exits 0 on `running 0 tests`, so naming the target is
    /// checking the filename. The row's `tests` are what
    /// [`wasm_unit_run`] asserts out of `--list` *before* anything runs, which
    /// is what makes a deleted, renamed, `#[ignore]`d or `cfg`-ed-away probe
    /// fail the gate rather than pass it.
    ///
    /// The names are **unprefixed**, and that is asserted rather than assumed:
    /// unlike the conformance target there is no `mod $mod_name` wrapper here,
    /// because `event_store_conformance!` is not involved — so a `::` in either
    /// name would mean the row is written against a shape the target does not
    /// have, and every `--list` comparison would fail with a message about a
    /// missing test rather than about a wrong expectation.
    #[test]
    fn the_wf11_probe_row_names_both_of_its_tests_unprefixed() {
        let row = WASM_UNIT_TARGETS
            .iter()
            .find(|unit| unit.selector == ["--test", "wf11_memory_ceiling"])
            .expect("the WF-11 memory-ceiling probe has no row in WASM_UNIT_TARGETS");

        assert!(
            !row.tests.is_empty(),
            "a row that names no test has nothing for a rename to disagree with, \
             and an emptied probe would exit 0 on `running 0 tests`"
        );
        for name in [
            "records_the_ceiling_and_the_encode_peaks",
            "the_two_encode_paths_produce_the_published_size_ratio",
        ] {
            assert!(
                row.tests.contains(&name),
                "the row does not name `{name}`, which the spec fixes as one of \
                 the probe's two cases"
            );
        }
        for name in row.tests {
            assert!(
                !name.contains("::"),
                "`{name}` is written as though a module wrapped it; the probe's \
                 two cases sit at the top level of the target and libtest prints \
                 them unprefixed"
            );
        }
    }

    /// AC-005. The executed target delegates wholly to its family's enumeration.
    ///
    /// A wasm-only subset list would fail project AC-002 by construction, and
    /// the place it would be written is the target's own source — an
    /// enumeration-free hand-rolled list of the rules someone judged safe on a
    /// single-threaded runtime. So the target may name the emitter, the fixture
    /// and the module, and no individual rule in its **code** at all.
    ///
    /// In its code, and that qualifier is load-bearing rather than a softening.
    /// The scan this test and [`wasm_enumeration`] share reads
    /// [`code_only`] — see [`prose_naming_a_rule_is_not_a_subset_list`] for the
    /// wrong implementation the qualifier rejects, and for what would go on
    /// passing if the scan were widened back to the whole file.
    #[test]
    fn the_executed_wasm_targets_name_no_rule_of_their_own() {
        assert!(
            !WASM_TARGETS.is_empty(),
            "no executed wasm32 target is registered, so this check reads nothing"
        );

        for wasm in WASM_TARGETS {
            let enumerated = enumerated_rules(wasm.family).unwrap();
            let source = read_source(wasm.source).unwrap();
            assert!(
                !source.contains("/*"),
                "`{}` carries a block comment, which `code_only` cannot read",
                wasm.source
            );
            let code = code_only(&source);
            assert!(
                code.contains(wasm.family.suite),
                "`{}` no longer invokes `{}`; an emptied target exits 0 on \
                 `running 0 tests`",
                wasm.source,
                wasm.family.suite
            );
            assert!(
                code.contains(wasm.family.emitter),
                "`{}` does not emit through `{}`, so whatever it runs is not the \
                 wasm32 harness",
                wasm.source,
                wasm.family.emitter
            );
            for rule in &enumerated {
                assert!(
                    !code.contains(rule.as_str()),
                    "`{}` names the rule `{rule}` in its own code. The rule set is \
                     single-sourced through `{}`; a hand-written list here is a \
                     wasm-only subset",
                    wasm.source,
                    wasm.family.enumeration
                );
            }
        }
    }

    /// The wrong implementation [`code_only`] exists to stop being written.
    ///
    /// Two of them, and they fail in opposite directions. A whole-file
    /// `contains` refuses `local_conformance.rs`, whose prose names rules while
    /// explaining where they moved to — so the guard reds on a correct file and
    /// its next reader deletes the guard. A scan that stripped *nothing but*
    /// whole-line comments still reads a trailing one, so
    /// `let f = fixture(); // drives two_handles_observe_each_others_appends`
    /// would red as well.
    ///
    /// What must keep failing is the real subset: a rule name in code. Both
    /// halves are asserted, because a `code_only` that returned the empty string
    /// would satisfy the first on its own.
    #[test]
    fn prose_naming_a_rule_is_not_a_subset_list() {
        let rule = "two_handles_observe_each_others_appends";

        for prose in [
            format!("//! `{rule}` used to live here as a `reentrancy` module."),
            format!("    // Moved into the suite: {rule}."),
            format!("let fixture = LocalFixture::new(); // drives {rule}"),
        ] {
            assert!(
                !code_only(&prose).contains(rule),
                "the scan read a rule name out of a comment: {prose}"
            );
        }

        for code in [
            format!("const WASM_RULES: &[&str] = &[\"{rule}\"];"),
            format!("#[wasm_bindgen_test] async fn {rule}() {{}}"),
        ] {
            assert!(
                code_only(&code).contains(rule),
                "the scan missed a hand-written rule list, which is the whole \
                 subject of the guard: {code}"
            );
        }
    }

    /// AC-007. Every `wasm32`-capable harness in the package has a row.
    ///
    /// The retired `wasm-conformance` CI job ran `cargo test -p
    /// happenstance-testkit --target wasm32-unknown-unknown`, which is *every*
    /// target in the package that compiles for `wasm32` — it named none of them
    /// individually and therefore could not fall behind. [`WASM_TARGETS`] names
    /// each one, so it can, and the first version of this seam did: it retired
    /// the job with one row registered and two harnesses left executing nowhere,
    /// while the retirement note claimed the gate was strictly more.
    ///
    /// So the list is checked against the directory rather than against itself,
    /// by [`unregistered_wasm_harnesses`] — in `cargo test -p xtask` here, and
    /// in the gate's own mandatory compensator row, which is what makes the
    /// claim in `ci.yml` a checked one rather than a hopeful one.
    ///
    /// The three rows this currently holds are exactly the three targets
    /// `cargo test -p happenstance-testkit --target wasm32-unknown-unknown`
    /// built: `memory_conformance_wasm`, `local_conformance` and
    /// `projection_conformance_wasm`. Adding a fourth harness without a row
    /// fails here rather than at review.
    #[test]
    fn every_wasm32_capable_harness_has_a_row() {
        let unregistered = unregistered_wasm_harnesses().unwrap();
        assert!(
            unregistered.is_empty(),
            "{unregistered:?} drive a conformance suite through a wasm32 emitter \
             and have no row in WASM_TARGETS, so nothing executes them"
        );
    }

    /// The same question one directory wider, and the miss the test above could
    /// not see.
    ///
    /// [`unregistered_wasm_harnesses`] reads exactly one directory —
    /// `crates/happenstance-testkit/tests` — so a `wasm32`-only test module
    /// anywhere else in the workspace executed nowhere and no check said so.
    /// That is not hypothetical: it is what happened to
    /// `happenstance-cloudflare`'s adapter tests, which the gate compiled from
    /// the day they merged and ran on no machine, while the previous
    /// milestone's stated purpose was to end precisely that shape.
    #[test]
    fn every_package_with_wasm32_only_tests_has_a_row() {
        let unregistered = unregistered_wasm_unit_packages().unwrap();
        assert!(
            unregistered.is_empty(),
            "{unregistered:?} carry `#[cfg(all(test, target_arch = \"wasm32\"))]` \
             modules and have no row in WASM_UNIT_TARGETS, so nothing executes them"
        );
    }

    /// The scan is a detector rather than a decoration: it finds the package it
    /// currently exempts.
    ///
    /// Without this, `unregistered_wasm_unit_packages` returning an empty list
    /// would be indistinguishable from it reading nowhere — which is the exact
    /// failure `unregistered_wasm_harnesses` guards against with its own
    /// `capable.len() < WASM_TARGETS.len()` bail, and the reason that bail
    /// exists at all.
    #[test]
    fn the_wasm32_unit_scan_can_see_the_package_it_exempts() {
        let root = workspace_root().unwrap();
        assert!(
            tree_has_wasm32_test_gate(
                &root.join("crates/happenstance-cloudflare/src"),
                WASM32_TEST_GATE
            )
            .unwrap(),
            "the scan cannot see the one tree it is registered against, so its \
             silence about every other tree would mean nothing"
        );
        assert!(
            !tree_has_wasm32_test_gate(
                &root.join("crates/happenstance-core/src"),
                WASM32_TEST_GATE
            )
            .unwrap(),
            "and it does not fire on a crate that has no wasm32-only test module, \
             which would make every row a false positive"
        );
    }

    /// The two gate spellings are genuinely different, and each row's is the one
    /// its own tree uses.
    ///
    /// Without this, `WASM32_TARGET_GATE` could be a copy of `WASM32_TEST_GATE`
    /// — or either could drift to a spelling neither tree writes — and every
    /// `source_dir` guard would keep passing on the strength of the *other*
    /// row's tree. The negative half is what makes it a detector: the module
    /// spelling must **not** be what an integration target is written behind,
    /// because that is the whole reason the field exists.
    #[test]
    fn each_unit_row_is_gated_by_the_spelling_its_own_tree_uses() {
        let root = workspace_root().unwrap();
        for unit in WASM_UNIT_TARGETS {
            assert!(
                tree_has_wasm32_test_gate(&root.join(unit.source_dir), unit.gate).unwrap(),
                "{}'s row names `{}`, which {} does not use",
                unit.package,
                unit.gate[0],
                unit.source_dir
            );
        }
        assert!(
            !tree_has_wasm32_test_gate(
                &root.join("crates/happenstance-cloudflare/tests"),
                WASM32_TEST_GATE
            )
            .unwrap(),
            "an integration target is already `#[cfg(test)]` by being one, so it \
             cannot be written behind the module spelling. If this ever passes, \
             `WasmUnitTarget::gate` has stopped distinguishing anything and one \
             constant would do."
        );
    }

    /// What the run does **not** cover, stated rather than left to be assumed.
    ///
    /// An artefact that claims coverage it does not have is worse than none,
    /// because its whole value to a reader is that they do not have to trust a
    /// summary.
    ///
    /// One of the three limits it names has since been closed and the document
    /// is **not** edited to say so: `references/evaluation/` artefacts are
    /// immutable and superseded rather than corrected, and this one is a true
    /// record of the tree at phase 6. The local gate did not execute a
    /// conformance rule on `wasm32` then; since HS-S0048 it does, through
    /// [`wasm_run`]. The MSRV limit still holds.
    #[test]
    fn the_artefact_states_what_the_run_does_not_cover() {
        let doc = proof_document();
        for claim in [
            "minimum supported Rust version",
            "conformance on wasm32",
            "PS-2",
        ] {
            assert!(
                doc.contains(claim),
                "the artefact's limits section never names `{claim}`"
            );
        }
    }
}
