---
id: kb-decision-0006
title: The bare name goes to the typed layer; the contract becomes happenstance-core
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0006
reversibility: medium
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Partly superseded by ADR-0007 — the naming decision stands and is the rule in force today; the
  projection-runner relocation is corrected. ADR-0005's allocation is inverted: happenstance-core
  holds the ports, types, errors and the in-memory reference store, carries no serde, is no_std
  plus alloc, and is what an adapter author pins; happenstance holds Codec, DomainEvent,
  DecisionModel and the command loop, re-exports the contract, feature-gates the adapters, and is
  what an application cargo adds. happenstance-runtime ceases to exist. The workspace rule that
  follows is that everything depends on happenstance-core and happenstance-core depends on nothing
  in this workspace. The serde boundary moves with the contract rather than with the name: the
  contract crate's documentation must say so in its first paragraph and happenstance must point at
  it. Grounded in who imports what, and in serde_core, futures-core and tracing-core all giving the
  bare name to what applications import. Rejected: happenstance-domain, deferring the allocation,
  and ADR-0005's own happenstance-full. Not marked provisional, because a naming decision is
  settled by being made rather than by being tested.
depends_on:
  - kb-decision-0005
related:
  - kb-decision-0003
source_paths:
  - .kb/_intake/0006-bare-name-to-the-typed-layer.md
  - docs/adr/0006-bare-name-to-the-typed-layer.md
  - CLAUDE.md
  - RUNBOOK.md
last_reviewed: 2026-08-10
---

# The bare name goes to the typed layer; the contract becomes `happenstance-core`

## Context

[`kb-decision-0005`](0005-rename-to-happenstance.md) (ADR-0005) made two decisions under one
"and": rename `eventum` to `happenstance`, and make `happenstance` the **contract** crate rather
than a facade over a `-core`. The first was unforced and well evidenced, and stands. The second
rode in on the first's momentum, and its stated argument — "a contract crate is the thing users
import constantly, while a facade is a convenience that may never be built" — does not survive
inspection: it is an empirical claim about users who did not exist, asserted without evidence, and
the facade it called "may never be built" was already scheduled in `RUNBOOK.md` with dependencies
and an exit criterion, contradicting the same afternoon's other document. ADR-0005 had also
recorded its own cost under "Bad": a batteries-included facade left with no natural name — and
that bill had arrived, with no good candidate among `-runtime`, `-domain`, `-typed`, `-model`,
`-app`.

Three major crates converged independently on the opposite allocation, for the same shape: the
bare name goes to what applications import, `-core` to the low-churn trait crate that implementers
pin — `serde_core`/`serde`, `futures-core`/`futures`, `tracing-core`/`tracing`. `futures-core` is
already a dependency of this workspace. ADR-0005 had rejected `-core` as "a suffix which existed
for no reason other than the collision" — true of the old `eventum-core`, false here.

## Decision

Invert the allocation. `happenstance-core` holds ports, types, errors, the in-memory reference
store, and (at the time of writing) the projection runner; it carries no `serde`, is `no_std` +
`alloc`, and is what an adapter author pins. `happenstance` holds `Codec`, `DomainEvent`,
`DecisionModel` and the command loop; it re-exports the contract and feature-gates the adapters,
and is what an application `cargo add`s. `happenstance-testkit`, `happenstance-sqlite`,
`happenstance-ladybug` and `happenstance-sync` keep their names and depend on `happenstance-core`.
`happenstance-runtime` ceases to exist; its contents become `happenstance`.

The dependency rule is unchanged in substance and clearer in form: everything depends on
`happenstance-core`; `happenstance-core` depends on nothing in this workspace.

The `serde` boundary — [`kb-decision-0003`](0003-opaque-payloads.md)'s "never in the contract
crate's default features" — moves with the contract, not with the name: it attaches to whichever
crate defines the ports, which is now `happenstance-core`.

At the time this decision was taken, the projection runner was also moved into the contract crate,
on the grounds that it pumps an `EventStore` into a `ProjectionStore` without ever decoding a
payload. That half is corrected by ADR-0007, below.

## Consequences

**Good.** `cargo add happenstance` gives an application what it actually wants. ADR-0005's unnamed
facade problem disappears rather than being answered badly, and the layout matches three
independent ecosystem precedents.

**Good.** `happenstance-core` gets exactly the suffix that signals a tiny dependency graph,
`no_std` support, and a semver surface that should almost never move.

**Bad.** A second rename four hours after the first — free, since nothing was published, but
churn nonetheless. Recording why the first allocation was wrong is the mitigation.

**Bad.** Someone learning how `happenstance` maps to the DCB specification now starts at
`happenstance-core` rather than at the obvious name; the contract crate's documentation must say
so in its first paragraph, and `happenstance` must point at it.

**Neutral.** The crates.io reservation follow-up from [`kb-decision-0005`](0005-rename-to-happenstance.md)
now covers two names, `happenstance` and `happenstance-core`, both free and neither reserved at the
time of writing.

## Alternatives rejected

- **Keep `happenstance` as the contract, rename the typed layer to `happenstance-domain`.**
  Cheapest fix for the immediate `-runtime` collision, but leaves the bare name on the crate fewer
  people import and leaves the facade problem unsolved.
- **Defer the allocation until the typed layer's own phase.** Rejected: no future phase produces
  the missing information, since there would still be no users to observe — only a later, costlier
  version of the same judgment call, made after docs, doctests and the worked example had all been
  written against the deferred names.
- **`happenstance-full` as the facade name**, as ADR-0005 contemplated. Concedes the good name to
  the smaller audience and reads as an afterthought.

## What this decision is not, and what partly superseded it

This decision is not marked provisional — a naming decision is settled by being made, not by being
tested, unlike ADR-0001/0003/0004's technical provisionality. It is partly superseded by ADR-0007:
the naming half above stands and is the rule `CLAUDE.md` enforces today; the projection-runner
relocation is corrected there, because a runner that never decodes withholds the half applications
actually need.
