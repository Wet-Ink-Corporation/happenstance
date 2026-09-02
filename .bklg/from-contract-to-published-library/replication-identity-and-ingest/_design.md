---
item: "HS-P0017"
stage: design
created: "2026-08-11"
updated: "2026-08-11"
---

# API surface design — What a position means across a store boundary

## Surfaces

**N/A — no user-facing surface.** This project ships no screen, CLI TUI, or
documentation site. It turns `crates/happenstance-sync` from a stated "phase-2
sketch, not the protocol" (`crates/happenstance-sync/src/lib.rs:3`) into real
trait bodies for `SyncPeer` and `IngestStore`, a sync runner that exercises the
`!Send` path end to end, a new `happenstance-sync-testkit` crate
(`sync_peer_conformance!` plus a mutant registry under CF-1–CF-5), envelope
types on the wire format, and two peer adapters against transports already
owned by sibling projects. What it touches instead of a screen is Rust trait
implementations, an `xtask` gate wiring pass, and KB decision atoms (ADR-0026,
ADR-0027, an ADR-0003 amendment, two open-question resolutions) authored only
through `/redkiln:kb-ingest`.

Verified directly rather than assumed, by reading this project's own scope
rather than inferring it:

- `project.md`'s **In scope** list is entirely: `ADR-0026`/`ADR-0027`, the
  ingest re-check answer, `IngestStore` made real in
  `crates/happenstance-sync/src/ingest.rs`, `happenstance-sync-testkit` and its
  conformance macro, three peers (`MemorySyncPeer` plus two networked ones
  already owned elsewhere), the byte-identical round trip lifting ADR-0003's
  `provisional` marker, envelope types in `crates/happenstance-sync/src/wire.rs`,
  and two open-question resolutions
  (`.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md`,
  **In scope**, lines 66-111). None of these renders anything.
- Its **Out of scope** list is explicit that *building* the Durable Object and
  Postgres/Neon stores, and *publishing* `happenstance-sync`/
  `happenstance-sync-testkit` to the registry, both belong to sibling projects
  (`project.md`, **Out of scope**, lines 118-133) — this project only makes the
  port real and conformant, never a deployable surface.
- The initiative's own **Warranted briefs** table checks `architecture` and
  `testing` for `replication-identity-and-ingest` and leaves `ux` and
  `deployment` blank
  (`.bklg/from-contract-to-published-library/_decomposition.md:251-264`), and
  this project's own briefs file states it outright: "architecture / testing —
  `ux` and `deployment` are unwarranted for this project per
  `.bklg/from-contract-to-published-library/_decomposition.md`, *Warranted
  briefs*"
  (`.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md:5-6`).
- The initiative charter carries `userFacing: false` and states plainly that
  "no `interaction-patterns.md` was produced — deliberately, at the intake
  gate" (`.bklg/from-contract-to-published-library/initiative.md:411`), and
  that "the public API surface is the only 'surface' here, and it is reviewed
  at the design stage" (`initiative.md:196-197`) — the review this section
  performs. `.bklg/from-contract-to-published-library/_discovery/distillation/interaction-patterns.md`
  does not exist (verified with `test -f`).

**Public API surface note.** This project unambiguously adds and changes
public Rust items — `SyncPeer`, `IngestStore`, the sync runner, envelope
types, and the whole of `happenstance-sync-testkit`'s conformance surface.
That is real API-shape work, and it is repository-wide obligated to carry
rustdoc (`standards/rust/70-rustdoc-obligations.md`). It is not, however, a
*rendered* surface: nothing here produces a route, a DOM node, a TUI pane, or
a screenshot a design-system primitive could be checked against. This
project's own architecture and testing briefs
(`.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md`)
are where signatures, placement, visibility, and the states the port must
express are actually decided, cited, and defended trait-by-trait; this
section exists to record that the API-surface obligation the template names
was seen and is being discharged there, not silently dropped, and not
re-litigated here as a screen it is not.

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
