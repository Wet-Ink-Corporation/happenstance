---
item: HS-S0081
stage: discover
created: 2026-08-12T13:02:50.772Z
updated: 2026-08-12T13:02:50.772Z
template_sig: 86ce4036
rendered_sig: e35fab83
---

# Discover — Measure the C++ build cost and implement the CI shape it buys

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — measure the cold and warm `lbug` build on each matrix runner, then implement and record the CI shape that number buys (excluded-plus-dedicated-job, or accepted in the three-OS gate) in `xtask/src/main.rs`, `.github/workflows/ci.yml` and `RUNBOOK.md`'s phase 11 body | `_storymap.md:47` | Measure, then decide, then implement, then record — in that order, and the order is the deliverable. |
| **AC-009** — the cold build time is measured and recorded, and the CI decision is recorded in `RUNBOOK.md`'s phase 11 body **with the number behind it** | `project.md:214-217`; `RUNBOOK.md:4420-4425`, `:4437` | A decision recorded without its number does not discharge the AC. |
| The gate's `clippy` and `tests` steps are `--workspace --all-features`, and the workspace is `members = ["crates/*", …]` — so the `lbug` dependency lands in **every** contributor's `cargo xtask ci` the moment it is added | `_decomposition.md:186-192`; `xtask/src/main.rs:116-129`, `:143-151` | This is the cost being measured, and it is workspace-wide by construction rather than by choice. |
| **A feature flag saves nothing**: `cargo hack check --workspace --feature-powerset` runs above the all-features steps when the tool resolves, and `cargo-hack` **does** resolve on this machine; `default-members` does not help either, because `--workspace` selects all members regardless | `_decomposition.md:381-387`; `xtask/src/main.rs:544-556`; `CLAUDE.md`, *Commands* | The most obvious remedy is refuted before it is proposed. Naming it is what stops it being re-proposed. |
| The levers that actually exist are `--exclude happenstance-ladybug` on the shared gate steps plus a dedicated step or CI job, **or** accepting the cost in the three-OS matrix | `_decomposition.md:388-393`; `.github/workflows/ci.yml:31-40` | Two options, and both are step-args edits in `xtask/src/main.rs` plus workflow edits — not configuration knobs. |
| The trade cuts both ways: left in the gate it taxes every contributor on every commit; split out, **it can silently stop running** — so whichever is chosen, the recorded rationale must address the *other* failure mode | `project.md:284-287`; `_decomposition.md:390-393` | The rationale is not "we chose X because it is faster"; it is "we chose X, and here is what we did about what X breaks". |
| Measurement protocol — a genuinely cold build (fresh `CARGO_TARGET_DIR`, `cargo build -p happenstance-ladybug --locked`) on each of the three matrix runners, plus one warm incremental figure, each stamped with toolchain and machine | `_decomposition.md:394-399`; `_decomposition.md:479` | The same discipline `references/evaluation/README.md` requires of any measurement kept as evidence: dated, pinned to a commit, superseded rather than edited. |
| **No precedent to copy** — nothing else in this workspace builds C++, so the design is from first principles while the *constraints* are checkable facts rather than estimates | `_decomposition.md:377-380`; `_grounding.md:193-198` | The absence of precedent is why this is its own story rather than a line in the packaging one. |
| `RUNBOOK.md` phase 11's work list already frames it — *"Give the crate its own CI job if the build cost is material, rather than slowing the three-OS gate everyone runs"* — and the exit criteria carry *"Build cost measured and the CI decision recorded here"* | `RUNBOOK.md:4420-4425`, `:4437` | The runbook states the conditional; this story supplies the antecedent and then acts on it. |
| **M4's consequence for any excluded configuration** — `cargo test` exits 0 on `running 0 tests`, which is why the gate runs `cargo xtask proof-artefact` and asserts the named tests out of `--list` first | `_decomposition.md:166-171`; `xtask/src/main.rs:170-178` | Any CI shape that stops compiling this crate also stops running the proof artefact, and the `ARTEFACTS` row is what makes that loud instead of silent. |
| `depends_on: ladybug-fixture-and-conformance-run` — supplies a gate that already runs the conformance target, so the CI shape is measured against the real workload rather than a build-only figure | `_storymap.md:47`, `:136-138` | Both packaging-slice stories sit after slice 3 for exactly this reason. |
| docs.rs is a separate, known failure and is `publication-and-positioning`'s call; this project's obligation is to hand that project the number and the `[package.metadata.docs.rs]` question | `_decomposition.md:401-406`; `crates/happenstance-core/Cargo.toml:54-56` | The number this story produces is one of the three things HS-S0083 hands over. |

## Questions

**Cold, warm, or both? — answered: both, and each stamped.** A genuinely cold
build with a fresh `CARGO_TARGET_DIR` and `--locked` on each of the three matrix
runners (`ubuntu-latest`, `windows-latest`, `macos-latest`, per
`.github/workflows/ci.yml:36-39`), plus one warm incremental figure. Cold is what a
CI runner and a first-time contributor pay; warm is what a contributor pays on
every subsequent commit, and the two arguments are different. Windows is not
optional in the sample: it is first-class in this workspace by explicit comment
(`.github/workflows/ci.yml:34-35`) and is the runner most likely to be worst for a
`cmake`-driven C++ build.

**What number makes the cost "material"? — deliberately open, with a procedural
recommendation this story makes rather than inherits.** Neither the project nor
the runbook fixes a threshold. The recommendation carried into `spec` is that the
threshold be **stated before the measurement is taken**, for exactly the reason
AC-001 fixes the axes before the suite runs: a threshold chosen after the number
is a justification, not a criterion. This is a proposal, flagged as one — the AC
requires the number and the decision, not the ordering — and if `spec` declines
it, the reason should be written down.

**Which lever, if the cost is material? — deferred to `spec` in choice, fixed in
menu and in obligation.** The menu is two items and a feature flag is not one of
them (`_decomposition.md:381-387`). The obligation is symmetrical: a decision to
exclude must say what makes the dedicated job unskippable, and a decision to keep
it in the matrix must say what makes the tax acceptable. Either rationale that
addresses only its own upside is incomplete under `project.md:284-287`.

**How does `lbug`'s blocking API meet a non-blocking port (ADR-0025)? — not this
story's, and it touches the measurement only as a confound.** If Q3's answer added
an optional runtime feature, the feature-powerset step compiles more
configurations than the plain build does, and the measured figure must say which
configuration it is a figure *for*. Named so the measurement is reproducible, not
to re-open the decision.

**What counts as "structurally unlike"? — not this story's.** The axes are on disk
from HS-S0074 and concern the batch shape, not the build. Named only to keep the
boundary explicit: a build-cost finding is not an axis and does not belong in that
document.

**Deferred to `spec`:** where the measurement lives — a file under
`references/evaluation/` on the `phase-4-reconciliation.md` genre, or a section of
`experiments/` — and whether the dedicated job, if chosen, runs on all three
runners or one.

## Decision

Adding `lbug` puts a `cmake`-driven C++ build into a workspace whose gate is
`--workspace --all-features` on three operating systems, and the cost of that
lands on every contributor on every commit from the moment the dependency merges —
but the obvious remedies are worse than they look: a feature flag saves nothing
because every gate step turns features on, and splitting the crate out of the
shared steps buys speed by creating a job that can stop running without anyone
noticing. This story measures the cost first — cold with a fresh
`CARGO_TARGET_DIR` and `--locked` on each matrix runner, plus a warm incremental
figure, each stamped with toolchain and machine — and only then chooses between
the two levers that exist, implements it in `xtask/src/main.rs` and
`.github/workflows/ci.yml`, and records the decision **with the number behind it**
in `RUNBOOK.md`'s phase 11 body. The spec will cover the measurement protocol and
its evidence file, the recommended pre-stated threshold, the implementation of
whichever lever is chosen, and a rationale that addresses the failure mode of the
option taken rather than only the one avoided — including, if the crate is
excluded, what makes the dedicated job unskippable and what keeps
`xtask/src/proof.rs`'s `ARTEFACTS` row still running the conformance target.
Nothing `[FROZEN]` is touched: this story edits build configuration and a runbook
body, and no conformance rule and no specification clause is in its blast radius.

## The wrong implementation

**The CI shape chosen first and the number measured afterwards to justify it.**
`--exclude happenstance-ladybug` added to the shared gate steps with a dedicated
job beside them, then a cold build timed once on the machine that happens to be
handy, the figure written into `RUNBOOK.md`'s phase 11 body under the decision it
did not inform. Every check passes: the gate is green, the number is recorded, the
exit criterion at `RUNBOOK.md:4437` is ticked, and AC-009's text is satisfied
word for word — because AC-009 asks that both exist, and only the *ordering* makes
the number a reason rather than a decoration. It is the same failure as a verdict
written to match a result, in a smaller register, in the same project. The guard
is the pre-stated threshold above and a measurement file committed ahead of the
`xtask`/workflow edits, on the commit-order discipline AC-001 already uses.

**`lbug` behind an off-by-default feature.** The remedy that feels obviously right
and is refuted by the gate's own definition: `clippy` and `tests` run
`--all-features` (`xtask/src/main.rs:116-129`, `:143-151`) and `cargo hack check
--workspace --feature-powerset` runs above them and resolves on this machine
(`:544-556`), so every step turns it on and the cost is paid in full. What the
flag *does* buy is a configuration in which the adapter does not compile at all —
and in that configuration `cargo test` exits 0 on `running 0 tests`
(`xtask/src/main.rs:170-178`), so a developer's local run reports success against
a crate that was never built. The proof-artefact row from
`ladybug-fixture-and-conformance-run` (HS-S0078) is the only thing that makes that
loud, which is why the row and this decision are coupled even though they sit in
different slices.

**The dedicated job that exists and never runs.** The split-out option's failure
mode, and it has three well-known spellings, all of which look green: a job with
`continue-on-error: true`; a job behind a `paths:` filter that does not match
`crates/happenstance-ladybug/**` (or matches it but not `Cargo.lock`, so a
dependency bump skips it); and a job that is not in the branch protection required
set, so a red run merges. In each case the CI page is green, the crate is
excluded from the gate everyone runs, and the adapter silently stops being
compiled — which is exactly `project.md:284-287`'s stated risk realised. The
rationale recorded in `RUNBOOK.md` must therefore name what makes the job
unskippable, and `spec` should treat "the job is required, unfiltered, and fails
the merge" as the acceptance condition rather than "the job exists".

**The number recorded without its configuration.** A single figure — "cold build:
6m41s" — with no toolchain, no runner, no target directory state and no feature
set. It satisfies "the build cost is a number", it is unreproducible, and it
cannot be compared against anything later when someone asks whether a `lbug`
upgrade made things worse. `references/evaluation/README.md:1-13`'s discipline is
the guard, and it applies here for the same reason it applies to the verdict.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
