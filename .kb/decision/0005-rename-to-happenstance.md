---
id: adr-0005-rename-to-happenstance
title: "ADR-0005: Rename the project to `happenstance`, and make it the contract crate"
kind: decision
status: accepted
authority_tier: decision
summary: >-
  The project is renamed to `happenstance`, which is free on crates.io. PARTLY
  superseded by ADR-0006: the rename stands, the second half — that the bare name
  goes to the contract crate — does not. Status stays `accepted` rather than
  `superseded` because half of it is still binding.
depends_on: []
related:
  - adr-0006-bare-name-to-the-typed-layer
source_paths:
  - Cargo.toml
last_reviewed: 2026-08-09
adr_id: ADR-0005
supersedes:
  - adr-0002-crate-naming
superseded_by: null
---

# ADR-0005: Rename the project to `happenstance`, and make it the contract crate

- **Status:** partly superseded by [ADR-0006](0006-bare-name-to-the-typed-layer.md)
- **Date:** 2026-08-05
- **Supersedes:** [ADR-0002](0002-crate-naming.md)

> **Partly superseded.** This ADR bundles two independent decisions under one
> "and". The first — renaming the project from `eventum` to `happenstance` —
> **stands**, on exactly the reasoning below. The second — allocating the bare
> name to the contract crate rather than to a batteries-included facade — is
> reversed by [ADR-0006](0006-bare-name-to-the-typed-layer.md), which explains
> why the argument given for it does not hold.
>
> The body is kept verbatim, per the rule this ADR applied to ADR-0002.

## Context

[ADR-0002](0002-crate-naming.md) had to work around a naming collision: the bare
`eventum` name on crates.io belongs to an unrelated crate, dormant since 2020.
The workaround was to publish under prefixed names only, make `eventum-core` the
documented entry point, and file a name-release request in parallel — accepting
that there would be no single `cargo add eventum` and that the layout carried a
`-core` suffix which existed for no reason other than the collision.

That is a compromise imposed by an accident of registration, not by the design.
Renaming the project removes the cause rather than managing the symptom.

`happenstance` is free: the crates.io endpoint for it returns 404, and a prefix
search returns no `happenstance*` crates. The GitHub repository has already been
renamed to `Wet-Ink-Corporation/happenstance`.

## Decision

Rename the project to **happenstance**, and — because the bare name is now
available — make it the contract crate rather than a facade over a `-core`:

| Before | After |
|---|---|
| `eventum-core` | **`happenstance`** |
| `eventum-testkit` | `happenstance-testkit` |
| `eventum-sqlite` | `happenstance-sqlite` |
| `eventum-ladybug` | `happenstance-ladybug` |
| `eventum-sync` | `happenstance-sync` |
| `eventum-runtime` | `happenstance-runtime` |

So `cargo add happenstance` and `use happenstance::{Event, EventStore}` are the
entry point, and the `-core` suffix disappears — it only ever existed to dodge
the collision.

Adapter crates keep the prefix. The dependency rule is unchanged: everything
depends on `happenstance`; `happenstance` depends on nothing in this workspace.

### On the historical record

ADR-0002 is marked superseded and its body is left **verbatim**. It is a factual
record about the *other* `eventum` crate, and rewriting `eventum` to
`happenstance` inside it would invert the claim into a falsehood — asserting that
`happenstance` is taken, when the whole point of this ADR is that it is not.
`CONTRIBUTING.md` already requires superseding an ADR rather than rewriting one;
this is that rule applied to itself.

ADRs 0001, 0003 and 0004 *were* rewritten, because renaming a project changes an
identifier, not a decision. Those decisions stand exactly as they were taken.

## Consequences

**Good.** The entry point is the obvious name. One fewer crate. No dependence on
an unresponsive owner or on the crates.io team, and no pending request to track.

**Bad — and worth acting on.** A batteries-included facade now has no natural
name left. If one is ever wanted, it must be `happenstance-full` or similar, or
the contract crate has to be renamed at that point. That is the price of taking
the good name for the contract, and it is judged the right trade: a contract
crate is the thing users import constantly, while a facade is a convenience that
may never be built.

**Bad.** Anyone holding a link to the old name has to follow a redirect. GitHub
redirects the renamed repository; nothing was ever published to crates.io, so
there is no package-level breakage. The README carries a former-name note.

**Neutral.** `crates/happenstance/Cargo.toml` previously declared
`readme = "README.md"`, pointing at a file that does not exist in the package —
harmless for `cargo check`, fatal for `cargo publish`. The key is dropped. A
per-crate `README.md` should be written before the first publish, since that is
what crates.io renders on the package page.

## Follow-up

**Reserve the name.** `happenstance` is free *today*. Nothing prevents someone
else registering it before the first real release, which would force this entire
ADR to be reopened. Publishing even a placeholder version claims it.
