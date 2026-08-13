---
item: "HS-P0012"
stage: design
created: "2026-08-12"
updated: "2026-08-12"
---

# API surface design — The first adapter that is not an instrument

## Surfaces

**N/A — no user-facing surface.** This project ships no screen, CLI TUI, or
documentation site. It turns `crates/happenstance-sqlite` from a `todo!()`
skeleton into a real `EventStore` + `ProjectionStore` adapter: SQL bodies, a
real read stream, atomic append under `BEGIN IMMEDIATE`, a `SqliteFixture` for
the conformance/model/concurrency/reopen suites, and ADR-0022. What it touches
instead of a screen is a schema, a connection, and a decision record.

Verified directly rather than assumed: `project.md`'s scope, derived
requirements and AC-001–AC-016 are all conformance suites, SQL, schema,
connections and ADR content — none names a screen, route or view
(`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md`).
`initiative.md:196` states "Any screen, UI, or documentation site" is out of
scope for the initiative and `userFacing` is false. The initiative's
Warranted-briefs table assigns `ux` briefs only to `projection-store-freeze`,
`typed-layer-and-alpha-release` and `publication-and-positioning` —
`sqlite-durable-store` gets architecture and testing only, with an explicit
rationale that ux is "deliberately absent elsewhere rather than stubbed"
(`.bklg/from-contract-to-published-library/_decomposition.md`, Warranted-briefs
table and rationale). This project's own briefs file
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md`)
contains only an Architecture brief and a Testing brief — no UX section. The
initiative's Open design tensions table assigns DT-1–DT-8 to Publication &
positioning, Typed layer & worked example, and Conformance & projection
freeze only — none to `sqlite-durable-store`.

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
