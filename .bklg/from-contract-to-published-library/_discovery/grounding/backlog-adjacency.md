---
title: Backlog adjacency — From Contract to Published Library
kind: grounding/summary
---

# Backlog adjacency

## What currently exists in `.bklg/`

Only two initiatives are on the board today:

- **This one** — `.bklg/from-contract-to-published-library/` (HS-I0006, stage
  `intake`), created from
  `.bklg/from-contract-to-published-library/_intake-brief.md`.
- **`support`** — `.bklg/support/initiative.md` (HS-I0005, `status: exploring`,
  `stage: intake`). Its body is the unfilled template (Summary/Outcomes/DoD/
  Projects/Notes all still carry placeholder prose) — nothing has been written
  into it yet.

There is **no `.bklg/_archive/`** anywhere in this tree (checked at the worktree
root: `ls .bklg/_archive` → "No such file or directory"). Nothing has ever
reached closeout, so there is no completed initiative whose scope this one
could re-open or duplicate.

## `support` is a boundary, not an overlap

`support` is not adjacent in subject matter — it is process scaffolding.
`.redkiln/config.yaml:5` sets `support_initiative: support`, and the
`redkiln:forge-fix` skill description says directly: "The story lives under
the support initiative." Nothing in `support`'s current body names a subject;
it exists only so `/redkiln:fix`'s preflight has a parent directory to file
into.

**Practical boundary this sets for `from-contract-to-published-library`:** any
incidental bug found while grounding, planning, or implementing this
initiative's own work is *not* this initiative's to fix inline as scope creep —
it is a reactive story that belongs under `support` (HS-I0005), per the
existing `support_initiative` wiring. This is already the tool's own boundary,
not a new one this initiative needs to invent.

## The load-bearing finding: this scope was already attempted once, by hand, and reverted

Commit `0269720` ("Revert the hand-authored backlog and knowledge base",
5 files listed below are a sample of 65 total) removed a complete hand-authored
decomposition of almost exactly the ground this new initiative's intake brief
covers. `CLAUDE.md`'s own "Where the work lives" section cites this same
commit as the cautionary precedent for why KB atoms are authored by
`/redkiln:kb-ingest` rather than by hand: "Atoms are authored by
`/redkiln:kb-ingest` from `.kb/_intake/`, not by hand: hand-writing them
produces the directory layout of the process without the process, which is
why the first attempt at this was reverted (`0269720`)." The same sentence
applies one level up, to initiatives.

The reverted tree (recoverable at `git show 0269720^:<path>`, not present in
the working tree) held four initiatives that partition the exact ground this
intake brief's "Desired Outcome" describes:

- **`alpha-0-2-0`** (HS-I0002) — *"0.2.0-alpha.1 — freeze the projection port
  and ship the typed layer."* Problem/outcome text at
  `git show 0269720^:.bklg/alpha-0-2-0/_intake-brief.md`. Covers RUNBOOK phase
  6 (freeze `ProjectionStore`, `RUNBOOK.md:3848`) and phase 7 (the typed layer
  and worked example, `RUNBOOK.md:3971`), decomposed into two projects:
  `freeze-projectionstore` and `typed-layer`.
- **`release-0-2-0`** (HS-I0003) — *"0.2.0 — the flagship adapter and first
  publish."* `git show 0269720^:.bklg/release-0-2-0/_intake-brief.md`. Covers
  phase 8 (`happenstance-sqlite`, `RUNBOOK.md:4166`) and phase 12 (publish
  `0.2.0`, `RUNBOOK.md:4448`), decomposed into `happenstance-sqlite` and
  `publish-0-2-0`. Its own Constraints section states the non-goal this new
  brief inherits verbatim: "No Cloudflare, no Postgres, no Neon, no
  replication. Those are the next initiative."
- **`adapters-and-replication`** (HS-I0004) — *"Adapters and replication — the
  post-0.2.0 spread."* `git show 0269720^:.bklg/adapters-and-replication/_intake-brief.md`.
  Covers phases 9–11 and 13–14 (Cloudflare Durable Object, Postgres/Neon,
  Ladybug projection store, `happenstance-sync`, retention/deletion/
  completeness — `RUNBOOK.md:4241`, `4311`, `4393`, `4516`, `4626`),
  decomposed into five projects: `cloudflare-durable-object`,
  `happenstance-sync`, `ladybug-projection-store`, `postgres-and-neon`,
  `retention-and-completeness`.
- **`support`** (HS-I0001 in that tree) — the same standing-scaffold initiative
  that exists today as HS-I0005; not overlapping in subject matter, as above.

Between them, `alpha-0-2-0` + `release-0-2-0` + `adapters-and-replication`
map almost one-to-one onto this new intake brief's four "Desired Outcome"
bullets: (1) the contract used by a typed layer/worked example ↔
`alpha-0-2-0`; (2) adapters passing the suite across disagreeing storage
shapes ↔ `release-0-2-0` (SQLite) + `adapters-and-replication` (Cloudflare,
Postgres/Neon, Ladybug); (3) a published release with provisional clauses
audited ↔ `release-0-2-0`'s `publish-0-2-0` project; (4) honest replication
and retention answers ↔ `adapters-and-replication`'s `happenstance-sync` and
`retention-and-completeness` projects.

**Reading this correctly matters for scope, not just history.** This is not a
separate initiative whose boundary needs negotiating — it is (most of) this
initiative's own subject matter, previously typed in by hand instead of
produced by `/redkiln:initiative` + `/redkiln:plan`, and reverted for that
reason rather than for being wrong about the ground. The reverted intake
briefs are useful prior art for what a plausible decomposition looks like
(four candidate seams: freeze-the-port, publish-the-first-adapter,
spread-across-disagreeing-shapes, replication/retention) — but per this
seed's own "Deliberately not decided here" section
(`references/seeds/remaining-runway.md:125-131`) and per `0269720`'s own
lesson, that decomposition is `/redkiln:plan`'s to re-derive from real
grounding, not to copy from the reverted files. The reverted `_intake-brief.md`
bodies are evidence of a shape that was considered once, not a decision that
binds this pass.

## Where the current `.kb/` already tracks pieces of this ground

The ADR corpus import (`.kb/decisions/0001`–`0016`, `0029`) and the phase 4/5
specification-reconciliation pass are both already in `.kb/`, per
`.kb/maps/domain-map.md:82-142`. Two specific open-question atoms sit
directly on ground this initiative's clause ledger names:

- `.kb/open-questions/cf-40-fixture-limits-ownership.md`
  (`kb-open-question-cf-40-ownership-001`) is the same CF-40-ownership
  question the intake brief's Clauses section flags ("CF-40's ownership is
  itself an open question owned by phase 8" — matches the atom's own "Forced
  by phase 8, the first adapter with real limits" line). This atom already
  exists and is `status: accepted`; this initiative's distillation stage
  should link and consume it rather than re-derive it as new.
- `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` and
  `.kb/open-questions/postgres-arm-c-structural-cost.md` sit on the same
  ES-10 / Postgres-position-visibility ground the intake brief's Open
  Questions section restates ("Is the ES-10 visibility invariant global or
  per-boundary?", "How does a Postgres adapter buy position visibility?").

None of the other 20 `.kb/open-questions/` atoms duplicate this brief's
higher-level questions (replication append-condition re-check, what a store
may forget) — those remain genuinely open, matching the brief's own framing
that they are "carried for distillation," not settled.

## No persona/product atoms exist yet for this brief's audience framing

`.kb/product/` and `.kb/design/` each contain only their `README.md`
template — no persona or journey atoms exist for "application author" or
"adapter author," the two audiences the intake brief (Who it is for,
inherited from `references/seeds/remaining-runway.md:79-91`) and this initiative
name. There is nothing here to conflict with; this initiative would be first
to give those audiences a durable atom, should distillation produce one.

## Summary of adjacency risk

- **Duplication risk: high, but against reverted (non-live) artifacts, not a
  live initiative.** The overlapping content no longer exists in the working
  tree; the risk is re-deriving a decomposition instead of treating the
  reverted `alpha-0-2-0` / `release-0-2-0` / `adapters-and-replication`
  intake briefs as prior art for `/redkiln:plan` to check its own output
  against.
- **Sequencing dependency, inherited from the reverted briefs and consistent
  with `CLAUDE.md`'s own "Ordering that must not be reordered"
  (`RUNBOOK.md:244-258`) and this brief's own Constraints:** phase 7 (typed
  layer) before phase 8 (flagship adapter); phase 4 before 8/9/10 for
  `EventId`/`recorded_at`; phase 8–12 (alpha, SQLite, publish) before the
  post-0.2.0 spread (Cloudflare/Postgres/Neon/Ladybug/sync/retention), because
  the reverted `adapters-and-replication` brief's own phase-13 dependency was
  `blocked_by: [8, 9, 10, 12]`.
- **No live boundary conflict** with `support` (HS-I0005) beyond the standing
  reactive-work routing already wired in `.redkiln/config.yaml:5`.
- **No `.bklg/_archive/` entries** to reconcile against; nothing has closed.
