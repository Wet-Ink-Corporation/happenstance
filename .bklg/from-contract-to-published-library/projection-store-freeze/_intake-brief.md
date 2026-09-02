---
item: HS-P0010
stage: intake
created: 2026-08-12T03:23:04.291Z
updated: 2026-08-12T03:23:04.291Z
template_sig: ab516678
rendered_sig: 663b0192
---

# Intake Brief — Freeze ProjectionStore behind a suite that can fail

## Problem

`ProjectionStore` carries the largest provisional block in the specification —
roughly seventeen PS-* clauses (`RUNBOOK.md:588-606`), with **PS-2 a single gate
standing under thirteen rows** (`RUNBOOK.md:608-610`). Its central invariant, that
the read-model write and the checkpoint write land in one transaction, is
documented on the trait and enforced by nothing. A port with no conformance suite
is a guess, and this one is a guess that four other projects are about to build
against.

## Desired Outcome

A `projection_store_conformance!` entry point exists with the same shape
obligations the event-store suite already carries: every rule registered, every
rule with a named wrong implementation, no rule silently absent from the run. We
know it worked when a deliberately wrong implementation that writes a checkpoint
without its read model **fails by name**, and two structurally unlike batch shapes
pass (DoD 7). The freeze itself is not called earned here — that verdict belongs to
`ladybug-projection-store` (HS-P0015), which tests it against a shape unlike the
ones that froze it.

## Constraints

- **Dependencies:** none upstream. This is a substrate root; `typed-layer-and-alpha-release`,
  `postgres-and-neon-stores` and `ladybug-projection-store` all wait on it.
- **A rule no adapter can fail is decorative.** Before adding one, name the wrong
  implementation it rejects and put that implementation in the testkit's own
  `tests/`. `CheckpointOnlyStore` is the first such implementation and is required,
  not optional.
- **Never assert on literal position values.** The specification permits gaps.
- The phase-3 amendment pass items that block a freeze must land first: §4's *"the
  port has zero implementers"* is false (five impls) and must be correct before
  anything is frozen.
- **Non-goals**, each naming its owner: the graph-shaped third batch shape and the
  written freeze verdict → `ladybug-projection-store`; the `Projection` trait, the
  runner and PS-33's falsifier → `typed-layer-and-alpha-release`; any SQL
  projection adapter shipped as a product → `sqlite-durable-store`; the PS-3
  verdict on whether the port ships behind `unstable-projection` →
  `publication-and-positioning` (this project supplies the evidence, not the
  verdict).

## Open Questions

- **DT-3 — how does a consumer learn a guarantee does not apply to them?** Reported
  in the suite's own output, documented per adapter, or both with one
  authoritative. A silent skip is the documented failure mode; two sources of truth
  is the other one. Consumes `.kb/open-questions/cf-40-fixture-limits-ownership.md`
  rather than rediscovering it.
- **DT-8 — whose adapter-author bar does the suite hold?** This project's own
  adapter authors, or an outside author too. If the latter, this project grows a
  documented testkit extension surface and AC-04/AC-05 must hold against
  implementations nobody in this repository wrote.
- **The DoD 7 watched edge.** Two unlike batch shapes are owed here, but the only
  SQL projection store in the tree ships from `sqlite-durable-store`, two positions
  later in merge order. Either the second shape is built inside the testkit — the
  `MemoryFixture` precedent — or this is a latent 6→8 inversion. It is not a DAG
  edge today because nothing in the runbook makes it one. **The architecture brief
  must settle this.**
- Carried in: `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`.

## Proof artefact

**`CheckpointOnlyStore` failing the suite by name**, alongside two structurally
unlike batch shapes passing it. This artefact would not exist if the design were
wrong: a suite that cannot reject a store which writes a checkpoint without its
read model has not enforced the one invariant the port exists to hold, and the
failing test is the only thing that distinguishes an enforced invariant from a doc
comment. A green `cargo xtask ci` is a precondition for looking at it, never the
evidence itself.

## Clauses

- **PS-4 – PS-15** `[PROVISIONAL]` — batch ownership, write vocabulary, drop
  semantics. Settled by **ADR-0017**.
- **PS-16 – PS-20** `[PROVISIONAL]` — reset, checkpoint scope, refusal. Settled by
  **ADR-0018**.
- **PS-26 – PS-30** `[PROVISIONAL]` — apply failure policy. Settled by **ADR-0019**.
- **PS-2** `[PROVISIONAL]` — the single gate under thirteen rows; the clause this
  project most has to get right.
- **PS-3** — evidence supplied here (did the two batch shapes disagree?); the
  verdict is `publication-and-positioning`'s.
- **PS-33, PS-34** — not this project's: PS-33 is `typed-layer-and-alpha-release`'s
  falsifier, PS-34's E0195 spelling trap is re-tested by `ladybug-projection-store`.
- **CF-40** `[PROVISIONAL]` — fixture-limits ownership; the *policy* is decided here
  and first exercised by `cloudflare-durable-object-store`.
- Nothing `[FROZEN]` is amended. If this work needs one changed, that is a new
  decision atom and a re-plan, not a line edit.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` — where each box's rationale is
written — and will not leave `intake` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
