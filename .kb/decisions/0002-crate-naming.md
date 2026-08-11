---
id: kb-decision-0002
title: Prefixed crate names, with a parallel claim on eventum
kind: decision
status: superseded
authority_tier: decision
adr_id: ADR-0002
reversibility: high
phase: 0
supersedes: null
superseded_by: kb-decision-0005
summary: >-
  Superseded by ADR-0005 and kept for the record. The bare eventum name on crates.io belonged to
  an unrelated crate dormant since 2020, so the decision was to publish under prefixed names
  immediately with eventum-core as the documented entry point, pursue a name-release request in
  parallel, and add a thin facade later if the name ever freed — nothing needing a rename for that
  to happen. Rejected: waiting for the name, which blocks all publishing on an unresponsive owner,
  and renaming the whole project, which spends the repository, organisation and branding to solve
  what a prefix already solves. It was superseded thirty-two minutes after being written, when the
  project was renamed to happenstance and the collision stopped existing. Its body is kept verbatim
  rather than corrected, because rewriting eventum to happenstance inside it would turn a true
  statement about a crates.io registration into a false one.
depends_on: []
related:
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/0002-crate-naming.md
  - docs/adr/0002-crate-naming.md
  - CONTRIBUTING.md
last_reviewed: 2026-08-10
---

# Prefixed crate names, with a parallel claim on `eventum`

## Context

At the time this decision was taken, the project's name was `eventum`. The bare `eventum` name on
crates.io was already registered: version `0.1.1`, described as "Asynchronous I/O Event manager",
first published 2020-07-03, last published 2020-08-27, repository `github.com/sjtakada/eventum-rs`,
dormant since. It is unrelated to this project and, by all appearances, abandoned. Prefixed names —
`eventum-core`, `eventum-sqlite`, and so on — were unaffected and free. A library crate had to ship
under *some* name, and the collision needed a decision rather than a wait.

## Decision

Publish under prefixed names immediately, and do not block anything on the bare name:

- `eventum-core` is the documented entry point.
- A name-release request goes to the crates.io team, and/or the owner is contacted directly, in
  parallel with everything else.
- If the name is ever released, add a thin `eventum` facade crate that re-exports `eventum-core`
  and feature-gates the adapters. Nothing has to be renamed for that to happen — the facade is
  purely additive on top of the prefixed layout.

The repository, the GitHub organisation and all branding kept the name `eventum`; only the
crates.io identity was affected by the collision.

## Consequences

**Good.** Ships without waiting on a third party. The prefixed layout — `eventum-core`,
`eventum-sqlite`, `eventum-testkit` — is what a multi-adapter ecosystem wants regardless of the
naming collision, so the workaround cost nothing beyond the missing single `cargo add eventum`.
A facade added later, if the name frees up, is purely additive and requires no rename.

**Bad.** No single `cargo add eventum` for as long as the collision stands, and readers would
reasonably wonder why. The README was to say so explicitly rather than leaving it a mystery.

## Alternatives rejected

- **Wait for the name.** Blocks all publishing on an unresponsive owner of a dormant crate —
  an open-ended dependency on someone who has not published since 2020.
- **Rename the whole project.** Costs the repository name, the GitHub organisation and all
  branding to solve a problem that a crate-name prefix already solves at zero cost.

## What superseded it

Superseded thirty-two minutes after being written: the project was renamed from `eventum` to
`happenstance` ([`kb-decision-0005`](0005-rename-to-happenstance.md)), whose crates.io endpoint was
free, and the collision this ADR was working around stopped existing. This atom's body is kept
verbatim rather than corrected in place — [`kb-decision-0005`](0005-rename-to-happenstance.md)'s
own record explains that rewriting `eventum` to `happenstance` here would invert a true statement
("the bare name is taken") into a false one, since the whole point of the rename was that the new
name is *not* taken. The corpus's immutability rule for accepted decisions extends by the same
logic to a superseded one: its body is history, not a place to reconcile with current names.
