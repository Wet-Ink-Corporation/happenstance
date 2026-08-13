---
item: "HS-P0019"
stage: design
created: "2026-08-11"
updated: "2026-08-11"
---

# API surface design — The whole gate on the assembled library, and a durable audience

**No public API surface.** This project adds, changes and removes no public item in
`happenstance-core`, `happenstance`, or any adapter crate.

The claim is deliberate and recorded three times upstream, not inferred here:

1. The initiative's own charter marks `userFacing: false` for the whole initiative and
   states "there is no screen anywhere in this initiative"
   (`.bklg/from-contract-to-published-library/_decomposition.md:266`).
2. The initiative's warranted-briefs table gives `closeout-and-durable-audience` zero
   architecture/ux/deployment briefs — only `testing`
   (`.bklg/from-contract-to-published-library/_decomposition.md:264`), with the explicit
   sentence: "`closeout-and-durable-audience` carries **one** brief deliberately. It
   verifies and records; it designs nothing. Forcing `architecture` or `ux` onto it would
   produce exactly the stub the right-sizing rule exists to prevent."
   (`_decomposition.md:286-288`).
3. This project's own `project.md` repeats it verbatim under *Out of scope*: "**Design of
   anything.** `_decomposition.md` *Warranted briefs* gives this project **one** brief
   (`testing`) deliberately" (`project.md:117-120`), and its own `_decomposition.md` brief
   header states again: "No `architecture`, `ux` or `deployment` section belongs here;
   forcing one would [produce a stub]" (`_decomposition.md:9-11`).

What this project actually touches instead: `cargo xtask ci` run to completion on a clean
checkout; a re-observation record of DoD 1-15; a decision-atom audit table under
`.kb/decisions/`; and persona/journey atoms promoted into `.kb/product/` via the
`/redkiln:kb-ingest` path (`project.md` DR-8, AC-007/008/009). All of these are backlog
markdown, knowledge-base atoms and CI/gate output — none of them is rendered to a screen.
The library this initiative assembles ships no application screen at all; the nearest thing
to one anywhere in the initiative is the crates.io/docs.rs presentation, and that belongs
entirely to the sibling project `publication-and-positioning` (HS-P0016), explicitly out of
this project's scope (`project.md` *Out of scope*).

## Items

N/A — no public surface. This project adds, changes or removes zero items in any crate's
public API; its deliverables are backlog records, a KB audit table and promoted product
atoms, none of which is a Rust API surface.

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

N/A — no user-facing surface. No API item is being introduced for a diff to be checked
against; the standing constraints in `CLAUDE.md` (no `#[async_trait]`, no `serde` in
`happenstance-core`'s defaults, `read` returns the stream at the top level, generic code
binds `EventStore` not `SendEventStore`, no `unwrap`/`expect` in library code) remain
binding on every other project but are not exercised by this one, which writes no library
code.

## The doctest

N/A — no user-facing surface. No example is authored because no API item exists to
demonstrate.

## Sign-off

N/A — no user-facing surface.

**Approved.** Ryan Britton (repository owner), 2026-08-12, at the `/redkiln:plan` design
sign-off gate. No mock was produced or owed: this project records no user-facing surface, and
that is stated explicitly here rather than left as a silent skip. What was approved is the
no-surface determination itself together with the anti-patterns recorded above — the design
stage's `design.capture` perceptual review remains a **declared** skip, per `CLAUDE.md`.
Conditions: none. Nothing here requires the human API-surface sign-off this
template exists to collect; the applicable sign-off is the project's own review gate
against `_testing.md` and the DoD checklist in `project.md`.
