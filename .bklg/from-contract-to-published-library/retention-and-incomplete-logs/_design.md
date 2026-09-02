---
item: HS-P0018
stage: design
created: "2026-08-11"
updated: "2026-08-11"
---

# API surface design — What a store may forget, and how a reader finds out

**This is the bundled design stage, repurposed.** Redkiln ships it because the pipeline had no stage
that decided what a screen looks like. This project has no screen: it ships no crate, no adapter and no
application surface. Per this project's own briefs artifact
(`.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:6-13`),
it carries **two** briefs — architecture and testing — and states explicitly *"No ux or deployment
section belongs here: there is no screen, and this project deploys nothing."* The initiative charter
(`.bklg/from-contract-to-published-library/initiative.md:196`) lists this class of work under its own
non-goals with `userFacing: false`, and the initiative's *Warranted briefs* table
(`.bklg/from-contract-to-published-library/_decomposition.md:251-264`) checks only `architecture` and
`testing` for this project, not `ux`.

What this project touches instead: a **completeness instrument** (a decorator over `EventStore` that
deliberately hides part of its own log, built in `crates/happenstance-testkit/tests/`), two new
conformance rule bodies wired into `for_each_event_store_rule!`, spec-marker moves in
`spec/SPECIFICATION.md`, and a KB decision atom (ADR-0028) authored through `.kb/_intake/` and
`/redkiln:kb-ingest`. DT-7 — the design tension this project owns, *"how a reader is told the log it is
reading is incomplete, and why"* — is resolved as a vocabulary/documentation decision inside this
project's own prose and ADR-0028 (composition roots 150–213 above, DA-1 through DA-3 of the
architecture brief), not as any rendered UI: the "readers" involved are `read_decision_model`, a
projection runner, and `IngestStore::holds` — Rust code paths, not a screen a human looks at. No route,
no DOM selector, no component exists to enumerate.

**Public API surface note.** This project may add a defaulted `Fixture` capability and/or method
(DA-4, `crates/happenstance-testkit/src/contract.rs`) and two `pub fn` rule bodies re-exported from
`suite.rs`. Both are testkit-internal conformance-suite surface, not application-facing UI, and are
scoped and reviewed under this project's own AC-A05/AC-A06/AC-011, not here. This section exists to
record the API-surface obligation the template names; it is not a claim that no `pub` item changes.

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
