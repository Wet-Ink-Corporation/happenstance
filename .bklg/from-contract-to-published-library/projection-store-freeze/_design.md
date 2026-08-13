---
item: HS-P0010
stage: design
created: 2026-08-12T04:39:23.000Z
updated: 2026-08-12T04:39:23.000Z
---

# API surface design — Freeze ProjectionStore behind a suite that can fail

## Surfaces

**No user-facing surface.** This project ships nothing a person looks at or
clicks. That is confirmed two ways rather than assumed: the initiative charter
declares `userFacing: false` and states that `interaction-patterns.md` was
"deliberately not commissioned" at the intake gate
(`.bklg/from-contract-to-published-library/_decomposition.md:411-412,479-482`),
and this project's own UX brief says it outright — *"There is no screen"*
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:25`).
A repo-wide search of `_discovery/distillation/` confirms no
`interaction-patterns.md` was ever produced for this initiative; only
`opportunities.md` and `personas-and-journeys.md` exist.

What a human meets instead is two non-visual surfaces, both verified against
the current tree:

1. **A type surface.** `ProjectionStore`, its `Batch<'a>` associated type, and
   `ProjectionId` already exist at `crates/happenstance-core/src/projection.rs`;
   the `Checkpoint` / `Authority` / `CommitError` / `ResetError` types this
   project adds are the target shape recorded in this project's own brief
   (`_decomposition.md:17,121,149,348,556,589-590`), not yet in the file. A
   caller meets this surface by writing Rust code against a trait — there is
   no route, no DOM, nothing to capture.
2. **A text surface.** The one line a conformance run prints for a declined
   capability, verified at
   `crates/happenstance-testkit/src/contract.rs:500-507` (`skip_line`, called
   by `report` at line 532): `SKIP {rule}: fixture declines`
   `` `{capability}` `` `— {reason}`. It reaches a human through stdout / CI
   logs, never a screen.

Per `CLAUDE.md`'s *Where the work lives*: `design.capture` is undeclared in
`.redkiln/config.yaml`, so the perceptual review is a declared skip, not a
silent pass — there is no app to screenshot. The bundled `_design.md` template
itself exists for exactly this case, substituting a compiled doctest for a
capture-based review. That doctest is what the rest of this file is for; the
surface manifest below stays empty because a manifest built for routes and DOM
selectors has nothing to index in a medium with neither.

```yaml
surfaces: []
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
