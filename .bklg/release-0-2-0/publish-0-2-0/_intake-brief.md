---
item: HS-P0004
stage: intake
created: 2026-08-10T02:59:45.239Z
updated: 2026-08-10T02:59:45.239Z
template_sig: 56ad54cb
rendered_sig: 0fc398f7
---

# Intake Brief — Phase 12: Publish `0.2.0`

## Problem

Nothing is on crates.io, so every decision so far has been reversible at no cost to
anyone. The MSRV, the public surface and the semver promise are private opinions.
Two failure modes only exist after a publish: a `--cfg docsrs` build that breaks on
docs.rs (which builds *after* the crate is published, and a release can be yanked but
never edited), and a semver break that is invisible to a branch-baseline check
because it merged two pull requests ago.

## Desired Outcome

`happenstance`, `happenstance-core` and `happenstance-testkit` published at 0.2.0,
with docs.rs green and a registry baseline to compare against from then on.

## Constraints

- **Phases 7 and 8 first.** Publishing a contract with no real adapter behind it
  publishes a shape nobody has used.
- **ADR-0004 loses its `provisional` marker here**, because first publish turns the
  MSRV into a promise to somebody else.
- **The `[PROVISIONAL]` clause audit happens here.** 49 of them; each is either
  frozen, re-marked with a reason, or explicitly carried, and the ledger is what
  makes that auditable.
- **Non-goal.** `happenstance-sqlite` is not published at 0.2.0 unless its own bar is
  met independently.

## Open Questions

- Does `cargo-semver-checks` against a **registry** baseline report anything the
  branch-baseline run has been missing? It cannot answer until a real version exists,
  which is part of what this phase buys.
- Is `0.2.0` still the right first number? The reasoning for not starting at `0.1.0`
  is recorded in the runbook and is not reopened without a reason.

## Proof artefact

**docs.rs green under `--all-features` and the `docsrs` cfg, and `cargo-semver-checks`
reporting against a registry baseline.**

The first is the one class of breakage that cannot be fixed after the fact. The
second is the first check in this repository's history that can see a break the
branch baseline is structurally blind to.

## Clauses

Audits every `[PROVISIONAL]` clause. ADR-0004 loses its provisional marker. No clause
is frozen *by* publication — publication only changes who pays when one moves.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` and will not leave `intake` until
every one is ticked.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
