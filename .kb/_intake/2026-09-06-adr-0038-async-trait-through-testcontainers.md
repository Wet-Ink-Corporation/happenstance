# Intake — ADR-0038, `async-trait` through `testcontainers`

Staged for `/redkiln:kb-ingest`. **Do not hand-author the atom from this file** —
hand-writing atoms is what commit `0269720` reverted. The long form is
`references/adr/0038-async-trait-through-testcontainers.md` and the atom should
cite it by line rather than restate it.

## What happened

Phase 10, story `postgres-schema-and-live-fixture` (HS-S0061), added
`testcontainers 0.28` as a dev-dependency of `happenstance-postgres` so the
adapter has a live pinned Postgres to be measured against — the mechanism
`RUNBOOK.md:4350-4362` names. It turned `cargo deny check bans` red with two new
routes to the banned `async-trait`: one direct from `testcontainers`, one through
`tonic` under `bollard`.

`deny.toml`'s comment had refused to pre-authorise the case — *"A third route
still fails until someone decides it should not."* It was decided rather than
absorbed, following ADR-0035's precedent exactly.

## The shape the atom should carry

- **kind:** decision · **authority_tier:** decision · **status:** accepted
- **adr_id:** ADR-0038 · **phase:** 10 · **reversibility:** medium
- **amends:** ADR-0001 (the ban) and ADR-0035 (the first widening). Neither body
  changes; only the exemption set moves. This is the same shape ADR-0029 used
  against ADR-0004 and ADR-0035 used against ADR-0001 — an accepted decision atom
  is immutable, so an amendment is a new atom, never an edit.

## The load-bearing distinction, which a summary will lose if it is not told to keep it

There are now **three different arguments** behind one `wrappers` list, and
collapsing them into "async-trait is allowed for test and platform crates" throws
away the reasoning:

1. `wasm-bindgen-test` — exempt because it **ships in nothing**.
2. `worker` / `worker-macros` — **ships**, and is exempt on the narrower ground
   that it uses the macro for its *own* traits, from which this workspace derives
   no `happenstance` port (ADR-0035 opens by discarding argument 1 for exactly
   this reason).
3. `testcontainers` / `tonic` — argument 1 again, and **more cleanly than the
   crate it was written for**: a `[dev-dependencies]` of one crate, stripped from
   the published manifest by Cargo, invisible to `cargo hack --no-dev-deps`.

## The cost this record accepts knowingly

`tonic` is named as its own wrapper because `wrappers` matches **direct** parents,
so `testcontainers` alone leaves the check red. That admits `tonic` from anywhere
in the graph, including from a crate that ships. It is the one real weakening, it
is recorded rather than discovered, and a fourth route still fails.

## What it must say it does not decide

- ADR-0024, the position-visibility mechanism. Migration 1 deliberately carries
  no `xid8` column and no sequence default on `position` so this story cannot
  answer that question by accident.
- Whether the default gate may grow a live-infrastructure step. It may not (DR-9,
  AC-011); `testcontainers`' tests are `#[ignore]`d and the container starts only
  in the new CI job.
- Whether `cargo deny`'s probed-optional status is acceptable. ADR-0035 named it
  as a known weakness; so does this. It belongs to `the-gate`, not to an adapter.

## Links the atom should carry

- `[[0001-async-port-flavours]]` — amended
- `[[0035-async-trait-through-worker]]` — amended, and the precedent for the shape
- `[[0029-msrv-raised-to-1-97-1]]` — the amend-don't-edit pattern
- The `deny.toml` entry itself, which carries the argument at the point of use
