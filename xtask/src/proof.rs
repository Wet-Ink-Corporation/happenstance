//! Checks that each phase's proof artefact still holds the tests its clauses
//! name, then runs them.
//!
//! [`ARTEFACTS`] carries seven targets today: the conformance suite's own
//! `mutation_coverage` (CF-1 – CF-6, CF-18, and CF-22's model and concurrency
//! families; [ADR-0010]), the two `wire` targets the wire format was frozen
//! against ([ADR-0016]), the projection family's two — its mutant registry's
//! meta-tests and the harness parity guard — which are what phase 6's proof
//! artefact rests on and which nothing held until phase 6 closed, and the worked
//! example's two — `runs`, which is the only thing in the workspace that
//! *executes* a binary and reads what it printed, and `ui`, the `trybuild` pair
//! that pins the compiler's own diagnostic when a domain grows a variant.
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
//! [ADR-0010]: ../../.kb/decisions/0010-the-suite-must-prove-itself.md
//! [ADR-0016]: ../../.kb/decisions/0016-the-wire-format.md

use std::fs;
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
    /// The rules that must still be present, by the name `--list` prints them
    /// under, without the module prefix.
    pub(crate) rules: &'static [&'static str],
}

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

/// Every conformance target the gate executes on `wasm32-unknown-unknown`.
///
/// One row today, and the shape is the deliverable. `every-rule-under-workerd`
/// (HS-S0054) adds the Cloudflare conformance target here — a package, a target,
/// the module its emitter wraps and the rules worth naming — and inherits the
/// runner wiring, the version check, the exhaustive enumeration check and both
/// gate steps without writing any of them again.
pub(crate) const WASM_TARGETS: &[WasmTarget] = &[WasmTarget {
    package: REGISTRY_PACKAGE,
    target: "memory_conformance_wasm",
    module: "dcb_conformance_wasm",
    source: "crates/happenstance-testkit/tests/memory_conformance_wasm.rs",
    rules: MEMORY_WASM_RULES,
}];

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

/// The emitter a wasm32 conformance harness must go through.
///
/// `__emit_tokio` type-checks for this target and then cannot run on it, which is
/// the failure CF-23 is about; a target that emitted through it would compile
/// under the `cargo check` step above and fail here.
const WASM_EMITTER: &str = "__emit_wasm";

/// Where the one event-store rule enumeration lives.
const RULE_ENUMERATION: &str = "crates/happenstance-testkit/src/registry.rs";

/// The line the enumeration opens with.
const ENUMERATION_HEAD: &str = "macro_rules! for_each_event_store_rule {";

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

/// Every rule `for_each_event_store_rule!` declares.
///
/// Derived rather than transcribed, and that is the opposite of the choice
/// [`META_TESTS`] makes one screen up — deliberately, because the two lists are
/// answering different questions. `META_TESTS` is transcribed so that a rename
/// has something to disagree with. This is derived so that **adding** a rule
/// needs no edit here while a rule that exists on the host and never reaches
/// `wasm32` still fails: an exhaustive check that a hand-list could only
/// approximate, over a set that grows.
///
/// # Errors
///
/// Returns an error if the enumeration cannot be read, if it has moved, or if it
/// parses to fewer than two names — all three of which would otherwise silently
/// weaken every check built on it into a check of nothing.
fn enumerated_rules() -> Result<Vec<String>> {
    let body = read_source(RULE_ENUMERATION)?;

    let mut lines = body.lines().skip_while(|l| l.trim() != ENUMERATION_HEAD);
    if lines.next().is_none() {
        bail!("{RULE_ENUMERATION} has no `{ENUMERATION_HEAD}` — the enumeration has moved or gone");
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
                "{RULE_ENUMERATION} lists `{name}`, which is not a snake_case rule \
                 name. The enumeration's shape has changed; a name this scan cannot \
                 read is a rule silently missing from every check built on it."
            );
        }

        rules.push(name.to_owned());
    }

    if rules.len() < 2 {
        bail!(
            "{RULE_ENUMERATION}'s `for_each_event_store_rule!` parsed to {} rule(s); \
             the enumeration's shape has changed and every check built on it is now \
             checking nothing",
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

    let status = Command::new("cargo")
        .args(cargo_args(artefact))
        .status()
        .with_context(|| format!("failed to launch `cargo test` for `{package}`'s `{target}`"))?;

    if !status.success() {
        bail!("`{package}`'s `{target}` proof artefact failed with {status}");
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
/// 2. Every rule `for_each_event_store_rule!` declares appears in the target's
///    own `--list`. Exhaustive and derived, so a rule added upstream needs no
///    edit here, and a rule `#[cfg]`-ed out of the wasm harness — the shape
///    project AC-002 forbids — fails.
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

    let enumerated = enumerated_rules()?;
    let env = [(WASM_RUNNER_VAR, WASM_RUNNER)];

    for wasm in WASM_TARGETS {
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
                "{label} is missing {} of the {} rules `for_each_event_store_rule!` \
                 declares: {}\n\n\
                 The target builds, so `cargo test` would have exited 0 with nothing \
                 to say. The rule set is single-sourced through the one enumeration \
                 — a rule that reaches the host harnesses and not this one is a \
                 wasm32-only subset, which is the outcome this step exists to \
                 forbid. Listed: {} name(s).",
                absent.len(),
                enumerated.len(),
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
                 deliberately, update `MEMORY_WASM_RULES` in the same change.",
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

    Ok(())
}

/// Asserts the executed targets exist and are wired to the one rule set — with
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
/// Returns an error if no target is registered, if a target's source is missing
/// or no longer invokes the suite through [`WASM_EMITTER`], if a row names no
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

    let enumerated = enumerated_rules()?;

    for wasm in WASM_TARGETS {
        let source = read_source(wasm.source)?;

        for (needle, complaint) in [
            (
                "event_store_conformance!",
                "no longer invokes the conformance suite; an emptied target exits 0 \
                 on `running 0 tests`",
            ),
            (
                WASM_EMITTER,
                "does not emit through `__emit_wasm`, so whatever it runs is not the \
                 wasm32 harness — `__emit_tokio` type-checks here and cannot run here",
            ),
            (
                wasm.module,
                "no longer declares the module the gate expects its rules under",
            ),
        ] {
            if !source.contains(needle) {
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
                    "{}'s row names `{rule}`, which `for_each_event_store_rule!` \
                     does not declare. Either the rule was renamed and this list \
                     was not, or this list names a rule that never existed.",
                    wasm.target
                );
            }
        }

        // Single-sourcing, checked at the one place a subset would be written.
        let hand_written: Vec<&String> = enumerated
            .iter()
            .filter(|rule| source.contains(rule.as_str()))
            .collect();

        if !hand_written.is_empty() {
            bail!(
                "{} names {} conformance rule(s) itself: {hand_written:?}\n\n\
                 The rule set is single-sourced through \
                 `for_each_event_store_rule!`. A list here is a wasm32-only subset \
                 — a second enumeration that can drift from the first, which is \
                 the outcome the project's AC-002 forbids by construction.",
                wasm.source,
                hand_written.len()
            );
        }

        println!(
            "{}/{}: {} named rules, all declared by the one enumeration of {}",
            wasm.package,
            wasm.target,
            wasm.rules.len(),
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

    /// AC-003. The names the gate asserts are names the one enumeration emits.
    ///
    /// The duplication is the mechanism, exactly as it is for [`META_TESTS`]: a
    /// rule renamed in `registry.rs` and not here fails at this test rather than
    /// drifting into an expectation list that quietly matches nothing.
    #[test]
    fn every_named_wasm_rule_is_one_the_enumeration_declares() {
        let enumerated = enumerated_rules().unwrap();
        assert!(
            enumerated.len() > 1,
            "the rule enumeration parsed to {} names, which is a parser failure \
             rather than a rule set",
            enumerated.len()
        );

        assert!(
            !WASM_TARGETS.is_empty(),
            "no executed wasm32 target is registered, so this check has nothing \
             to disagree with"
        );
        for wasm in WASM_TARGETS {
            assert!(
                !wasm.rules.is_empty(),
                "`{}` names no rules, so an emptied target would pass its own \
                 anti-vacuity guard",
                wasm.target
            );
            for rule in wasm.rules {
                assert!(
                    enumerated.iter().any(|name| name == rule),
                    "`{}` names `{rule}`, which `for_each_event_store_rule!` does \
                     not declare — a rename that reached the enumeration and not \
                     this list",
                    wasm.target
                );
            }
        }
    }

    /// AC-005. The executed target delegates wholly to the one enumeration.
    ///
    /// A wasm-only subset list would fail project AC-002 by construction, and
    /// the place it would be written is the target's own source — a
    /// `for_each_event_store_rule!`-free hand-rolled list of the rules someone
    /// judged safe on a single-threaded runtime. So the target may name the
    /// emitter, the fixture and the module, and no individual rule at all.
    #[test]
    fn the_executed_wasm_targets_name_no_rule_of_their_own() {
        let enumerated = enumerated_rules().unwrap();
        assert!(
            !WASM_TARGETS.is_empty(),
            "no executed wasm32 target is registered, so this check reads nothing"
        );

        for wasm in WASM_TARGETS {
            let source = read_source(wasm.source).unwrap();
            assert!(
                source.contains("event_store_conformance!"),
                "`{}` no longer invokes the conformance suite; an emptied target \
                 exits 0 on `running 0 tests`",
                wasm.source
            );
            assert!(
                source.contains(WASM_EMITTER),
                "`{}` does not emit through `{WASM_EMITTER}`, so whatever it runs \
                 is not the wasm32 harness",
                wasm.source
            );
            for rule in &enumerated {
                assert!(
                    !source.contains(rule.as_str()),
                    "`{}` names the rule `{rule}` itself. The rule set is \
                     single-sourced through `for_each_event_store_rule!`; a \
                     hand-written list here is a wasm-only subset",
                    wasm.source
                );
            }
        }
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
