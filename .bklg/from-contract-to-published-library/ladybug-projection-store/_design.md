---
item: HS-P0015
stage: design
created: "2026-08-11"
updated: "2026-08-11"
---

# API surface design — The unlike batch shape, and the freeze verdict

**This is the bundled design stage, repurposed.** Redkiln ships it because the pipeline had no stage
that decided what a screen looks like. This project has no screen. It fills in the four `todo!()`
bodies on `LadybugProjectionStore`
(`crates/happenstance-ladybug/src/projection_store.rs:270-292`), adds the real `lbug` driver
dependency, runs the projection conformance suite against a Ladybug fixture, authors ADR-0025 as a
`.kb/decisions/` atom, and writes two dated markdown evidence documents (the "structurally unlike"
definition and the freeze verdict) under `references/evaluation/`. It also edits xtask registries
(`xtask/src/proof.rs`, `xtask/src/package.rs`), `RUNBOOK.md` checkboxes, and CI config
(`.github/workflows/ci.yml`). None of this is rendered to a human as an application screen — every
artifact is Rust source, a knowledge-base atom, or a plain evidence document read directly, not a UI.

Confirmed via this project's own scope/out-of-scope lists and the architecture brief's *Modules and
seams touched* table (`.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md:65-88`),
which name only `crates/`, `xtask/`, `spec/`, `RUNBOOK.md`, `.kb/`, and `references/evaluation/` — no
web/app/ui path anywhere. The initiative itself is marked `userFacing: false` with no
`interaction-patterns.md` produced (`.bklg/from-contract-to-published-library/initiative.md:411`), and
this project owns none of the initiative's DT-1–DT-8 open design tensions
(`.bklg/from-contract-to-published-library/initiative.md:418-427`), which are all assigned to sibling
projects (Publication & positioning; Typed layer & worked example; Conformance & projection freeze;
Replication & retention answers).

**Public API surface note.** `_decomposition.md`'s seams table records that
`LadybugProjectionStoreError`'s `Driver`/`Commit` variants are re-pointed from wrapping
`stand_in::Error` to wrapping `lbug::Error` (`…/_decomposition.md:72-73`), and that
`live_handle.rs` is retired or re-pointed to match (T1/T3). That is a mechanical re-source of an
existing `#[from]`/`#[source]` field on an already-`pub` enum the phase-2 skeleton declared — not a
new, renamed or removed item — and it is scoped and reviewed under this project's own DR-1/DR-2 and
AC-003/AC-004, not designed here. This note records the obligation the template names; it is not a
claim that no `pub` item's field type changes.

## Surfaces

N/A — no user-facing surface. This project ships no screen, UI, or documentation site; see the framing
above for what it touches instead.

```yaml
```

## Items

N/A — no user-facing surface.

## Signatures

N/A — no user-facing surface.

## Shape decision

N/A — no user-facing surface.

## Placement and re-export

N/A — no user-facing surface.

## Visibility and stability

N/A — no user-facing surface.

## What it costs a caller

N/A — no user-facing surface.

## What a user meets first

N/A — no user-facing surface.

## The states the API must express

N/A — no user-facing surface.

## Anti-patterns

N/A — no user-facing surface.

## The doctest

N/A — no user-facing surface.

## Sign-off

N/A — no user-facing surface.

**Approved.** Ryan Britton (repository owner), 2026-08-12, at the `/redkiln:plan` design
sign-off gate. No mock was produced or owed: this project records no user-facing surface, and
that is stated explicitly here rather than left as a silent skip. What was approved is the
no-surface determination itself together with the anti-patterns recorded above — the design
stage's `design.capture` perceptual review remains a **declared** skip, per `CLAUDE.md`.
Conditions: none.
