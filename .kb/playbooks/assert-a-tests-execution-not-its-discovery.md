---
id: kb-playbook-assert-execution-not-discovery-001
title: A gate that asserts a test exists has not asserted that it ran
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  An anti-vacuity check built on test discovery cannot see a silenced test, and the fix is to
  assert on the run's own output instead. Measured 2026-09-03 in experiments/gate-vacuity/:
  libtest's cargo test --list output is byte-identical with and without #[ignore] - the recorded
  diff is an empty file - so xtask/src/proof.rs, which rests its argument on the names being
  asserted out of --list before the tests run, cannot distinguish a suite that runs from one that
  does not. All thirty-one named proof tests were annotated #[ignore = "..."] and cargo xtask ci
  ran all thirty-three steps and exited 0, with four of nine artefacts then running zero tests.
  The bare #[ignore] spelling is caught, but by clippy::ignore_without_reason, a pedantic lint
  nothing in the gate's documentation mentions - and the spelling clippy's own help text
  recommends is exactly the one that survives it, so the lint that appears to close the hole
  instructs an author through it. The repair is to compare the count or the names in the run's
  result output against the expected set, not the discovery listing. The general shape: any check
  that asserts a test's existence as a proxy for its execution has this hole, and the two are
  different program behaviours rather than two ends of one reference - which is what separates
  this from a coverage-reporting problem. Where it stops holding: a harness whose discovery and
  execution listings are the same artefact has nothing to compare, and a suite small enough to
  read has no need of the check.
depends_on: []
related:
  - kb-decision-0010
  - kb-playbook-verify-referent-report-coverage-001
  - kb-decision-0037
  - kb-playbook-anchoring-citations-001
  - kb-playbook-control-fires-on-instrument-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/gate-vacuity/
  - xtask/src/proof.rs
last_reviewed: 2026-09-04
---

# A gate that asserts a test exists has not asserted that it ran

## The claim

A check built to prove a test suite is not vacuous — that the tests named in some contract
actually exist as compiled tests — is not the same check as one that proves those tests actually
*ran*. `xtask/src/proof.rs:29` rests its whole argument on the first: the names it expects are
asserted out of `cargo test --list`, before the tests run at all. That is a check over
**discovery**, and discovery cannot see whether a discovered test is then skipped.

## The measurement

`experiments/gate-vacuity/` puts the claim to the compiler rather than to argument. Libtest's
`cargo test --list` output is **byte-identical** with and without `#[ignore]` on a test —
`results/raw/list-diff.txt` is an empty file. `--list` enumerates tests; it does not run them, and
whether a test is marked to be skipped is not part of what discovery reports. So a check that
diffs `--list` output against an expected name set is answering "does a test with this name exist
in the binary," which `#[ignore]` does not change, rather than "did this test's body execute,"
which is exactly what `#[ignore]` changes.

The experiment then went further than a synthetic example: it annotated all **thirty-one** named
proof tests in the real `xtask/src/proof.rs` with `#[ignore = "…"]` and ran `cargo xtask ci`.
All thirty-three gate steps exited 0. Four of the nine artefacts the gate is meant to prove
non-vacuous then ran **zero** tests, silently, inside a fully green gate.

## The near-miss that makes this worth recording rather than obvious

The bare `#[ignore]` spelling — no reason string — is in fact caught by a lint,
`clippy::ignore_without_reason`. That looks, at first read, like the hole is already closed. It
is not: the lint is **pedantic**, off by default and unmentioned anywhere in the gate's own
documentation, and clippy's own help text for the lint recommends the `#[ignore = "…"]` spelling
as the fix — which is exactly the spelling the experiment used, and exactly the spelling that
survives the lint. The mechanism that looks like a safety net instructs an author through the gap
it appears to close.

## The repair

Assert on the **run's own output**, not on discovery. The result line `cargo test` and `cargo
nextest` both print — tests run, passed, ignored — carries the fact that discovery cannot:
whether execution actually happened. Comparing the expected test count or name set against that
line, rather than against `--list`, closes the hole because an ignored test is absent from "ran"
in a way it is never absent from "exists."

## Generalisation

Any check whose argument is "the name is asserted to exist" as a stand-in for "the behaviour is
asserted to have run" has this hole, and the fix is the same in shape every time: move the
assertion from the enumeration surface to the execution surface. This is not a restatement of
`kb-playbook-verify-referent-report-coverage-001` — that playbook's population is cross-references,
where an address and its referent are two ends of one relationship and the fix is to resolve the
referent as well as the address. A test suite is not a cross-reference: discovery and execution
are two different behaviours of one program, not two ends of one link, so there the fix is to
change which behaviour is asserted on, not to check a second property of the same one.

## Where it stops holding

A harness whose discovery listing and execution listing are the same artefact — one that cannot
enumerate a test without running it — has nothing to compare, and the distinction collapses. And a
suite small enough that a human reads every test name on every change has no structural need for
an automated check at all; the playbook is for the gate that stands in for that reading once the
suite has grown past it.
