//! Checks that the suite's own proof artefact still holds its meta-tests, then
//! runs them (CF-1 – CF-6, CF-18, and CF-22's model and concurrency families;
//! [ADR-0010]).
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
//! # The list here is an expectation, and it is meant to be edited
//!
//! [`META_TESTS`] duplicates eight names that also live in
//! `tests/mutation_coverage.rs`, and that duplication is the mechanism rather
//! than an oversight: renaming a meta-test fails this step, which is exactly the
//! moment to ask whether the clause citing the old name in `SPECIFICATION.md`
//! §6.1 was updated too. A derived list could not ask that question, because
//! there would be nothing for it to disagree with.
//!
//! It is deliberately a *subset* check, not an equality one. A ninth meta-test
//! is a good thing and must not need a gate edit to land. The seventh and
//! eighth — `the_model_rule_rejects_exactly_what_it_claims` and
//! `the_concurrency_rules_reject_exactly_what_they_claim` — are listed anyway,
//! because `SPECIFICATION.md` CF-22 cites them by name and this list is the
//! thing that notices when a cited name moves.
//!
//! [ADR-0010]: ../../docs/adr/0010-the-suite-must-prove-itself.md

use std::fs;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::spec_trace::workspace_root;

/// The package holding the proof artefact.
const PACKAGE: &str = "happenstance-testkit";

/// Where the mutant registry lives.
const REGISTRY_FILE: &str = "crates/happenstance-testkit/tests/mutation_coverage.rs";

/// The line the registry opens with.
const REGISTRY_HEAD: &str = "const REGISTRY: &[Declared] = &[";

/// Its test target.
const TARGET: &str = "mutation_coverage";

/// The meta-tests that must be present, by the name `SPECIFICATION.md` cites.
///
/// Fully qualified, because that is what the clauses say and what a reviewer
/// pastes into `cargo test`. The inner `mod mutation_coverage` inside the target
/// of the same name is what makes the two halves match.
pub(crate) const META_TESTS: &[&str] = &[
    "mutation_coverage::every_rule_has_a_mutant",
    "mutation_coverage::mutant_registry_is_exhaustive",
    "mutation_coverage::mutants_fail_exactly_their_declared_rules",
    "mutation_coverage::every_mutant_states_its_provenance",
    "mutation_coverage::conformant_variants_pass_everything",
    "mutation_coverage::capability_skips_are_reported",
    // The seventh, landed at phase 3 stage 5 with the model family. It is behind
    // the testkit's `proptest` feature, which is why the subset check above is
    // run with `--all-features` and would otherwise report it absent.
    "mutation_coverage::the_model_rule_rejects_exactly_what_it_claims",
    // The eighth, landed at phase 3 stage 5 with the concurrency family. It
    // needs no feature — the family needs threads rather than an optional
    // dependency — but it is listed for the same reason the seventh is: it is
    // the CF-1 obligation for a family whose rules are macro-emitted rather
    // than registry-keyed, and a rename should have to be noticed.
    "mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim",
];

/// The arguments common to both cargo invocations.
///
/// `--all-features` matters for a reason that is not about coverage: the `tests`
/// step above this one in the gate already builds the whole workspace with every
/// feature on, and a step that differs only in its feature set gets a different
/// fingerprint and rebuilds the crate and every one of its test targets from
/// scratch. Matching the feature set makes this step reuse those artifacts, which
/// is the difference between a few seconds and none.
const CARGO_ARGS: &[&str] = &[
    "test",
    "--locked",
    "-p",
    PACKAGE,
    "--all-features",
    "--test",
    TARGET,
];

/// Asserts the meta-tests exist, then runs them.
///
/// # Errors
///
/// Returns an error if the test target cannot be built or enumerated, if any of
/// [`META_TESTS`] is absent from its listing, or if the tests themselves fail.
pub(crate) fn run() -> Result<()> {
    let listed = list()?;

    let absent: Vec<&str> = META_TESTS
        .iter()
        .copied()
        .filter(|name| !listed.iter().any(|line| line == name))
        .collect();

    if !absent.is_empty() {
        let count = absent.len();
        bail!(
            "`{TARGET}` is missing {count} of its meta-tests: {absent:?}\n\n\
             The target exists and builds, so `cargo test` would have exited 0 \
             with nothing to say. These are the clauses' own names — if one was \
             renamed deliberately, update `xtask/src/proof.rs` and the `Rule:` \
             line of the clause in `SPECIFICATION.md` §6.1 that cites it, in the \
             same change. Listed: {listed:?}"
        );
    }

    println!(
        "{TARGET}: {} meta-tests present, {} registry rows",
        META_TESTS.len(),
        registry_len()?
    );

    let status = Command::new("cargo")
        .args(CARGO_ARGS)
        .status()
        .context("failed to launch `cargo test` for the proof artefact")?;

    if !status.success() {
        bail!("the suite's own proof artefact failed with {status}");
    }

    Ok(())
}

/// How many subjects the mutant registry declares.
///
/// Printed rather than asserted, and the distinction is the point: the count is
/// a fact about the tree that `CHANGELOG.md` and `docs/RUNBOOK.md` both quote in
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

/// Every test name libtest reports for the target.
///
/// libtest prints one `name: test` line per test, plus a trailing summary. The
/// suffix is stripped rather than the line split on `:`, because a test name
/// contains `::` and a split would take the wrong half; lines that do not carry
/// the suffix are the summary and are dropped.
fn list() -> Result<Vec<String>> {
    let output = Command::new("cargo")
        .args(CARGO_ARGS)
        .args(["--", "--list"])
        .output()
        .context("failed to launch `cargo test -- --list` for the proof artefact")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("could not enumerate `{TARGET}`'s tests:\n{}", stderr.trim());
    }

    let listing = String::from_utf8_lossy(&output.stdout).into_owned();
    Ok(listing
        .lines()
        .filter_map(|line| line.trim().strip_suffix(": test"))
        .map(str::to_owned)
        .collect())
}
