//! Checks that each phase's proof artefact still holds the tests its clauses
//! name, then runs them.
//!
//! [`ARTEFACTS`] carries three targets today: the conformance suite's own
//! `mutation_coverage` (CF-1 – CF-6, CF-18, and CF-22's model and concurrency
//! families; [ADR-0010]), and the two `wire` targets the wire format was frozen
//! against ([ADR-0016]).
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
//! [ADR-0010]: ../../docs/adr/0010-the-suite-must-prove-itself.md
//! [ADR-0016]: ../../docs/adr/0016-the-wire-format.md

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
}

/// The package whose proof artefact also carries the mutant registry.
///
/// Named once because [`registry_len`] and the [`ARTEFACTS`] row that owns it
/// have to agree, and a second spelling is a second thing to get wrong.
const REGISTRY_PACKAGE: &str = "happenstance-testkit";

/// Where the mutant registry lives.
const REGISTRY_FILE: &str = "crates/happenstance-testkit/tests/mutation_coverage.rs";

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

/// Every proof artefact the gate holds to its own names.
pub(crate) const ARTEFACTS: &[Artefact] = &[
    Artefact {
        package: REGISTRY_PACKAGE,
        target: "mutation_coverage",
        tests: META_TESTS,
    },
    Artefact {
        package: "happenstance-core",
        target: "wire",
        tests: WIRE_NEGATIVE_CONTROLS,
    },
    Artefact {
        package: "happenstance-sync",
        target: "wire",
        tests: SYNC_WIRE_TESTS,
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
    if package == REGISTRY_PACKAGE {
        // Reported beside its own entry rather than made a column every entry
        // has to answer: the count comes from a file only this package has.
        println!(
            "{package}/{target}: {present} named tests present, {} registry rows",
            registry_len()?
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
fn registry_len() -> Result<usize> {
    let root = workspace_root()?;
    let body = fs::read_to_string(root.join(REGISTRY_FILE))
        .with_context(|| format!("reading {REGISTRY_FILE}"))?;

    let mut lines = body.lines().skip_while(|l| l.trim() != REGISTRY_HEAD);
    if lines.next().is_none() {
        bail!("{REGISTRY_FILE} has no `{REGISTRY_HEAD}` — the mutant registry has moved or gone");
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

    bail!("{REGISTRY_FILE}'s `REGISTRY` declaration is never closed by a `];` at column zero")
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
