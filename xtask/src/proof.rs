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

    let listed = list(artefact)?;

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
fn list(artefact: &Artefact) -> Result<Vec<String>> {
    let Artefact {
        package, target, ..
    } = *artefact;

    let output = Command::new("cargo")
        .args(cargo_args(artefact))
        .args(["--", "--list"])
        .output()
        .with_context(|| {
            format!("failed to launch `cargo test -- --list` for `{package}`'s `{target}`")
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "could not enumerate `{package}`'s `{target}` tests:\n{}",
            stderr.trim()
        );
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

    /// What the run does **not** cover, stated rather than left to be assumed.
    ///
    /// The local gate never checks the MSRV and never *executes* a wasm test; both
    /// are CI jobs. An artefact that claims coverage it does not have is worse
    /// than none, because its whole value to a reader is that they do not have to
    /// trust a summary.
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
