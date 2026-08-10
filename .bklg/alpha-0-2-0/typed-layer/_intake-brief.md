---
item: HS-P0002
stage: intake
created: 2026-08-10T02:59:44.439Z
updated: 2026-08-10T02:59:44.439Z
template_sig: 56ad54cb
rendered_sig: c2b0ad21
---

# Intake Brief — Phase 7: The typed layer and the worked example

## Problem

`happenstance` — the crate most people will `cargo add` — is a five-line facade over
the contract. Nothing in the workspace uses the contract the way an application
author would, and a consumer is what discovers contract defects. Discovering them
after the flagship adapter is written is the sequence the whole plan exists to avoid,
which is why this phase comes before phase 8 rather than after it.

## Desired Outcome

`DomainEvent`, `DecisionModel`, `Codec`, the command loop, the application-facing
`Projection` trait and runner, and a testing DSL — with the canonical DCB worked
example rewritten on top of them, so the example demonstrates the library rather
than the contract.

## Constraints

- **Phase 6 first.** The projection runner is written against the frozen port.
- **This is the crate that depends on `serde`.** ADR-0003 forbids it in
  `happenstance-core`; forbidding it here would forbid the thing the split exists to
  allow. Read the crate names carefully.
- **ADR-0007 decided the runner splits across the seam** because it decodes. That
  decision is executed here, and its falsifier (PS-33) is evaluated at this phase's
  exit.
- **Non-goal.** No adapter, no publication.

## Open Questions

- How does a decision model guarantee that its query and its fold cannot disagree?
  (ADR-0020)
- How does a payload's shape evolve — codec tag, versioned event types, upcasting —
  and does the read path need a hook it does not have? (ADR-0021)

## Proof artefact

**A `trybuild` compile-fail case: add an event variant, and the crate stops compiling
until the fold handles it.**

It would not exist if the design were wrong, and that is the whole test — a typed
layer whose domain can grow without the compiler noticing is a facade with more
words. The failure it forbids is silent: a new event variant that the fold ignores
produces a decision model that is quietly, permanently behind.

## Clauses

Discharges PS-33 (ADR-0007's falsifier) and the application-facing half of §4.9's
runner split. Adds no ES or VT clause: this layer consumes the contract and does not
alter it — if it turns out to need an alteration, that is a new ADR against a
`[FROZEN]` clause and this phase has found the defect it exists to find.

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
