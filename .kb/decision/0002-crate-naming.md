---
id: adr-0002-crate-naming
title: "ADR-0002: Prefixed crate names, with a parallel claim on `eventum`"
kind: decision
status: superseded
authority_tier: decision
summary: >-
  Prefixed crate names under `eventum-`, because the bare name was taken on crates.io
  by an unrelated dormant crate. Wholly superseded by the rename to `happenstance`,
  which was free; kept unaltered as the record of why the original naming was chosen.
depends_on: []
related: []
source_paths:
  - Cargo.toml
last_reviewed: 2026-08-09
adr_id: ADR-0002
supersedes: []
superseded_by: adr-0005-rename-to-happenstance
---

# ADR-0002: Prefixed crate names, with a parallel claim on `eventum`

- **Status:** superseded by [ADR-0005](0005-rename-to-happenstance.md)
- **Date:** 2026-08-05

> **Superseded.** The project was renamed to `happenstance`, which is free on
> crates.io, so the compromise below is moot. This ADR is kept unaltered as the
> record of why the original naming was chosen. Everything it says about the
> `eventum` name refers to the unrelated crate that still holds it.

## Context

The bare name `eventum` on crates.io is taken:

- version `0.1.1`, described as "Asynchronous I/O Event manager"
- first published 2020-07-03, last published 2020-08-27
- repository `github.com/sjtakada/eventum-rs`, dormant since

It is unrelated to this project and, by all appearances, abandoned. Prefixed
names — `eventum-core`, `eventum-sqlite`, and so on — are unaffected and free.

## Decision

Publish under prefixed names now. Do not block anything on the bare name.

- `eventum-core` is the documented entry point.
- Open a name-release request with the crates.io team, and/or contact the owner,
  in parallel.
- If the name is released, add a thin `eventum` facade crate that re-exports
  `eventum-core` and feature-gates the adapters. Nothing has to be renamed for
  that to happen.

The repository, the GitHub organisation and all branding keep the name
`eventum`; only the crates.io identity is affected.

## Consequences

**Good.** Ships today. No dependence on a third party's response. The prefixed
layout is what a multi-adapter ecosystem wants anyway, and a facade added later
is purely additive.

**Bad.** There is no single `cargo add eventum` for now, and readers will
reasonably wonder why. The README says so explicitly rather than leaving it
mysterious.

## Alternatives rejected

- **Wait for the name** — blocks all publishing on an unresponsive owner.
- **Rename the whole project** — costs the repository name, the organisation and
  the branding to solve a problem that a prefix already solves.
