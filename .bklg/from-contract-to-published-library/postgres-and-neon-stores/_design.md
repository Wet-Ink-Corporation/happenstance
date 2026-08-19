---
item: "HS-P0014"
stage: design
created: "2026-08-11"
updated: "2026-08-11"
---

# API surface design — The two stores that disagree with the port

## Surfaces

**N/A — no user-facing surface.** This project ships no screen, CLI TUI, or
documentation site. It turns `crates/happenstance-postgres` and
`crates/happenstance-neon` from `todo!()` skeletons into two `EventStore` (and,
sequenced last, `ProjectionStore`) adapters against the frozen
`happenstance-core` contract: SQL append/read/head bodies under whichever
ES-10 mechanism ADR-0024 settles, a Neon transport over one-shot HTTP, two new
live-infrastructure CI jobs, and the decision record itself. What it touches
instead of a screen is a schema, a connection (or, for Neon, the absence of
one), and a decision record.

Verified directly rather than assumed: this project's own briefs file states
it explicitly — `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md:11-13`,
"No `ux` section belongs here — there is no surface to perceive, and
`design.capture` is deliberately absent from `.redkiln/config.yaml`." Confirmed
independently in `.redkiln/config.yaml` (~lines 75-80): `design:` capture is
deliberately undeclared, which "makes the perceptual review a skip rather than
a silent pass... there is no app to screenshot." At the initiative level,
`.bklg/from-contract-to-published-library/initiative.md:411` states
`userFacing` is false, and no `interaction-patterns.md` distillation exists at
`.bklg/from-contract-to-published-library/_discovery/distillation/` (missing).
The initiative's "## Open design tensions" table (`initiative.md:409-427`,
DT-1–DT-8) assigns every tension to one of three other projects —
Publication & positioning, Typed layer & worked example, or Conformance &
projection freeze — none to this one. This project's warranted briefs are
architecture, testing and deployment only
(`postgres-and-neon-stores/_decomposition.md:9-13`), matching `project.md`'s
scope: Postgres/Neon trait bodies, conformance runs against live
infrastructure, ADR-0024 on the position-visibility mechanism, CI job
topology, and crates.io publish-readiness — none of it renders anything a
human looks at.

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
