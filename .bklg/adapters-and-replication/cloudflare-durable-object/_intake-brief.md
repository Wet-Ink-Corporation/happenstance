---
item: HS-P0005
stage: intake
created: 2026-08-10T02:59:45.605Z
updated: 2026-08-10T02:59:45.605Z
template_sig: 56ad54cb
rendered_sig: 443ca9a1
---

# Intake Brief — Phase 9: Cloudflare Durable Object

## Problem

`happenstance-cloudflare` is the workspace's only `!Send` store and the entire reason
ADR-0001's two-flavour design exists — and it has never run a rule. Until it does,
the evidence base for the design is a `cargo check`: a compile of the *trait*, with
no implementation of the bare flavour anywhere and nothing ever executed on the
target the flavour exists for.

## Desired Outcome

Every conformance rule green under `workerd`, against a real Durable Object using
`SqlStorage`, with the off-tokio harness driving them.

## Constraints

- **`!Send` throughout.** A single `#[async_trait]` or a stray `Send` bound anywhere
  on the path deletes this target, which is what the gate's wasm32 build of the
  contract crate is standing guard over.
- **`#[tokio::test]` type-checks for wasm32 and then cannot run there.** That is
  precisely the failure CF-23 is about, and it is why type-checking the harnesses is
  not the same claim as executing them.
- **Non-goal.** Not on the 0.2.0 path; this branch never rejoins the trunk.

## Open Questions

- The `SqlStorage` mapping, and the off-tokio conformance harness. (ADR-0023)
- Whether a `worker::Error`-carrying error type loses information the caller needs.
  ES-6 was left genuinely open at phase 2, and ADR-0009's number was held for this.

## Proof artefact

**Every rule green under `workerd`, and a real `worker::Error`-carrying error type
that either loses information the caller needs or demonstrably does not.**

The second half is the part that carries information. "The error type compiles" is
satisfied by any type; the question ADR-0009 deferred is whether the caller can still
tell what went wrong, and only a real platform error answers it.

## Clauses

Discharges ES-6 if the error question resolves here. Exercises **CF-23** for real —
three harnesses in-tree, of which the wasm one is the only one no native `cargo test`
can reach.

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
