---
id: kb-decision-0038
title: async-trait is exempted where it is reached through testcontainers or tonic
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0038
reversibility: medium
phase: 10
supersedes: null
superseded_by: null
summary: >-
  deny.toml's async-trait ban gains testcontainers and tonic as wrappers, because phase 10's live
  Postgres fixture added testcontainers 0.28 as a dev-dependency and turned cargo deny check bans
  red on two new edges. Three distinct arguments sit behind one wrappers list and must not be
  collapsed; naming tonic admits async-trait from anywhere in the graph, which is the one real
  weakening and is recorded rather than discovered. A fourth route still fails the gate.
depends_on:
  - kb-decision-0001
  - kb-decision-0035
related:
  - kb-decision-0029
source_paths:
  - .kb/_intake/2026-09-06-adr-0038-async-trait-through-testcontainers.md
  - references/adr/0038-async-trait-through-testcontainers.md
  - deny.toml
last_reviewed: 2026-09-07
---

# async-trait is exempted where it is reached through testcontainers or tonic

## Decision

`deny.toml`'s `async-trait` ban gains two more wrappers — `testcontainers` and `tonic` — ratified
by ADR-0038. This amends `kb-decision-0001` and `kb-decision-0035` from outside; neither accepted
atom's body changes, only the `wrappers` list moves, the same shape `kb-decision-0035` used
against `kb-decision-0001` and `kb-decision-0029` used against `kb-decision-0004`. The full
transcript lives in `references/adr/0038-async-trait-through-testcontainers.md`; this atom cites
it rather than restating it.

The trigger: phase 10's story `postgres-schema-and-live-fixture` added `testcontainers 0.28` as a
dev-dependency of `happenstance-postgres` so the adapter has a live, pinned Postgres to be measured
against. `cargo deny check bans` went red on two new routes into the banned `async-trait`: one
direct from `testcontainers`, one through `tonic` under `bollard`. `deny.toml`'s own comment had
refused to pre-authorise this — *"a third route still fails until someone decides it should
not"* — so it was decided explicitly, following `kb-decision-0035`'s precedent, rather than
absorbed as a side effect of adding a dependency.

## Three arguments, one list — kept separate on purpose

`wrappers` now holds three exemptions, and collapsing them into "async-trait is fine for test and
platform crates" would license a fourth route the ban exists to keep failing:

1. **`wasm-bindgen-test`** — exempt because it ships in nothing. It is a dev-dependency present in
   no published artifact.
2. **`worker` / `worker-macros`** — ship, and are exempt on the narrower ground that the macro is
   used for `worker`'s own `DurableObject` trait, from which this workspace derives no
   `happenstance` port. `kb-decision-0035` opens by discarding argument 1 for this crate precisely
   because argument 1 does not apply to it.
3. **`testcontainers` / `tonic`** — argument 1 again, and more cleanly than the crate it was
   written for: `testcontainers` is a `[dev-dependencies]` entry of `happenstance-postgres`,
   stripped from the published manifest by Cargo and invisible to `cargo hack --no-dev-deps`.

Each argument is recorded on its own grounds because they are genuinely different claims, and
reusing one crate's reasoning for another crate's exemption is judging a different case with
borrowed words.

## The cost accepted knowingly

`deny.toml`'s `wrappers` mechanism matches only **direct** parents of a banned crate. `testcontainers`
alone reaches `async-trait` on one edge and `bollard`-via-`tonic` on another, so `testcontainers`
by itself would leave the check red; `tonic` has to be named too. Naming `tonic` admits
`async-trait` from **anywhere** in the dependency graph it appears in, including from a crate that
ships — not only from `testcontainers`' dev-only subgraph. That is the one real weakening this
decision makes, and it is recorded here rather than discovered later by someone auditing
`deny.toml`. A fourth route into `async-trait` not covered by any of these three arguments still
fails `cargo deny check bans` until someone names it and argues a fourth case.

## Why the load-bearing guard is unaffected

As with `kb-decision-0035`, `cargo deny` is an optional, probed gate step — it runs when the tool
resolves and is skipped otherwise. The guard that cannot be skipped is
`crates/happenstance/tests/flavours.rs`, which instantiates every typed-layer entry point against
a genuinely `!Send` store and stops compiling the moment a `Send` bound reaches the chain, on every
target, in every run. This exemption removes the legible, human-readable guard on two more edges
and leaves the unskippable compile-time one standing on all of them.

## What this does not decide

**ADR-0024's mechanism.** The migration this story shipped deliberately carries no `xid8` column
and no sequence default on `position`, so this story cannot answer the position-visibility question
by accident — that is `kb-decision-0024`'s, decided separately.

**Whether the default gate may grow a live-infrastructure step.** It may not: `testcontainers`'
tests are `#[ignore]`d, and the container starts only inside a dedicated, non-default CI job.

**Whether `cargo deny`'s probed-optional status is acceptable.** `kb-decision-0035` already names
that as a known weakness of the mechanism; this decision inherits it rather than re-litigating it.
It belongs to the gate's own design, not to any one adapter's exemption.

## Alternatives rejected

Editing `kb-decision-0001` or `kb-decision-0035` directly to add the new wrappers: rejected because
both are accepted and immutable, and the only legal shapes for changing an accepted decision's
content are a new atom or a superseding one — amendment from outside is that shape. Declining the
exemption and reworking `happenstance-postgres`'s live fixture to avoid `testcontainers` entirely:
not pursued, because `testcontainers` is the mechanism `RUNBOOK.md` names for giving the adapter a
real Postgres to measure against, and the crate's own use of `async-trait` never reaches a
`happenstance` port.
