---
id: kb-decision-0005
title: Rename the project to happenstance, and make it the contract crate
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0005
reversibility: high
phase: 0
supersedes:
  - kb-decision-0002
superseded_by: null
summary: >-
  Partly superseded by ADR-0006 — the rename stands, the crate allocation is reversed. Two
  decisions were taken under one 'and'. First, rename the project from eventum to happenstance,
  whose crates.io endpoint is free and whose GitHub repository was already renamed; that half is
  untouched. Second, because the bare name was now available, allocate it to the contract crate
  rather than to a batteries-included facade, with adapters keeping the happenstance- prefix; that
  half ADR-0006 reverses, on the ground that the argument given for it was unevidenced and
  contradicted by the runbook's own facade plan. Recorded consequences: one fewer crate and no
  dependency on the crates.io team, but the facade is left with no natural name, old links redirect
  through GitHub with no package-level breakage, and a per-crate README is owed before first
  publish. Its outstanding follow-up is that nothing yet reserves the name: publishing even a
  placeholder claims it, and someone else registering it first would reopen this decision.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0005-rename-to-happenstance.md
  - docs/adr/0005-rename-to-happenstance.md
  - CONTRIBUTING.md
  - CLAUDE.md
last_reviewed: 2026-08-10
---

# Rename the project to `happenstance`, and make it the contract crate

## Context

[`kb-decision-0002`](0002-crate-naming.md) had worked around a naming collision: the bare
`eventum` name on crates.io belonged to an unrelated crate, dormant since 2020. The workaround was
to publish under prefixed names only, make `eventum-core` the documented entry point, and file a
name-release request in parallel — accepting no single `cargo add eventum` and a `-core` suffix
that existed for no reason other than the collision.

That was a compromise imposed by an accident of registration, not by the design. Renaming the
project removes the cause rather than continuing to manage the symptom. `happenstance` was free:
the crates.io endpoint for it returned 404, and a prefix search returned no `happenstance*` crates.
The GitHub repository had already been renamed to `Wet-Ink-Corporation/happenstance`.

## Decision

Two decisions were taken under one "and."

**First:** rename the project to `happenstance`. Every `eventum-*` crate became `happenstance-*`
(`eventum-testkit` → `happenstance-testkit`, `eventum-sqlite` → `happenstance-sqlite`, and so on).

**Second:** because the bare name was now available, allocate it to the contract crate rather than
to a facade — `eventum-core` became simply `happenstance`, dropping the `-core` suffix, so
`cargo add happenstance` and `use happenstance::{Event, EventStore}` were the entry point. Adapter
crates kept the prefix. The dependency rule stayed unchanged: everything depends on `happenstance`;
`happenstance` depends on nothing in this workspace.

**On the historical record:** [`kb-decision-0002`](0002-crate-naming.md) was marked superseded and
its body left verbatim, since it is a factual record about the *other* `eventum` crate and
rewriting `eventum` to `happenstance` inside it would invert the claim into a falsehood. ADR-0001,
ADR-0003 and ADR-0004 *were* rewritten to the new crate names, because renaming a project changes
an identifier, not a decision — those decisions stand exactly as taken.

## Consequences

**Good.** The entry point became the obvious name. One fewer crate than the `-core` layout. No
dependence on an unresponsive owner or on the crates.io team, and no pending name-release request
to track.

**Bad — and worth acting on.** A batteries-included facade was left with no natural name. If one
is ever wanted, it needs `happenstance-full` or similar, or the contract crate has to be renamed
again — judged the right trade at the time, since a contract crate is imported constantly and a
facade is a convenience that may never be built.

**Bad.** Anyone holding a link to the old name has to follow a redirect; GitHub redirects the
renamed repository, and nothing had been published to crates.io, so there was no package-level
breakage.

**Neutral.** `crates/happenstance/Cargo.toml` had declared `readme = "README.md"` pointing at a
file absent from the package — harmless for `cargo check`, fatal for `cargo publish`. The key was
dropped; a per-crate `README.md` is owed before first publish.

## Follow-up

**Reserve the name.** `happenstance` was free at the time of writing. Nothing prevented someone
else registering it before a first real release, which would force this decision to be reopened.
Publishing even a placeholder version claims it. This follow-up remains outstanding.

## What partly superseded it

[`kb-decision-0006`](0006-bare-name-to-the-typed-layer.md) (ADR-0006) reverses the second half of
this decision: the bare name moves to the typed layer instead, and the contract crate becomes
`happenstance-core`. The rename half — `eventum` to `happenstance` — is untouched and stands
exactly as recorded here.
