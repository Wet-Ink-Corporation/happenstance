---
id: kb-decision-0037
title: The MSRV becomes a promise at 0.2.0, and the number does not move
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0037
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  0.2.0 requires Rust 1.97.1, and that is a promise rather than a preference. The number is
  unchanged from ADR-0029 and its standing is not: until this release the floor was a self-imposed
  constraint raisable at zero cost because nothing was published, and 0.2.0 puts happenstance-core,
  happenstance, happenstance-testkit and happenstance-sqlite on a registry, from which moment the
  number binds someone who may never build the crate that forced it. This is an amendment lineage
  rather than a supersession: ADR-0004 and ADR-0029 stay accepted and byte-identical, and neither
  gets a superseded_by flip, on the precedent ADR-0029 itself set against ADR-0004. The floor was
  forced by a dependency's build script rather than by this workspace's code - rusqlite 0.40 pulls
  libsqlite3-sys 0.38.1, whose build script invokes cfg_select!, unavailable before 1.88 - and is
  deliberately set higher than that evidence requires, tied to the toolchain the project is
  developed and tested on, because rusqlite, libsqlite3-sys, sqlx, sqlx-core and sqlx-postgres
  declare no rust-version at all and the exact minimum would have to be bisected against a build
  script that the next silent dependency would invalidate again. An MSRV increase is a minor
  version bump named in CHANGELOG.md; a patch release cannot move it, and below 1.0 cargo already
  treats a minor bump as incompatible, so a floor rise cannot reach a consumer without a version
  change they chose. The promise is stated no larger than its evidence: the msrv CI job installs
  1.97.1, runs cargo hack check --workspace --no-dev-deps --rust-version and then the full test
  suite at that toolchain, and it is currently vacuous because rust-toolchain.toml pins the same
  number - it proves the workspace builds at 1.97.1, which was never in doubt, and not that 1.97.1
  is the minimum, which nothing in this repository proves. It starts proving something the day the
  pin and the floor diverge, which is why it is kept. Four alternatives lost: promising only
  "latest stable", which is the absence of a floor rather than a floor; pinning rusqlite back to
  0.37, which buys a number with a security and maintenance surface and would be re-fought every
  release; a per-package rust-version, which is the strongest of the four and loses on
  verification capacity rather than on merit, since four floors need per-package MSRV checking
  this workspace does not have; and a moving N-2-stable window, which breaks a consumer's build on
  a date rather than on an upgrade they chose.
depends_on:
  - kb-decision-0004
  - kb-decision-0029
related:
  - kb-governance-referent-not-reasoning-001
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-playbook-assert-execution-not-discovery-001
source_paths:
  - .kb/_intake/msrv-becomes-a-promise-at-0-2-0.md
  - Cargo.toml
  - rust-toolchain.toml
  - .github/workflows/ci.yml
  - CHANGELOG.md
  - RUNBOOK.md
last_reviewed: 2026-09-04
---

# The MSRV becomes a promise at 0.2.0, and the number does not move

## Decision

`0.2.0` requires Rust 1.97.1, and that is a promise rather than a preference. The number is
unchanged — `Cargo.toml`'s `rust-version` and `rust-toolchain.toml` both say `1.97.1` and keep
saying it. What changes is its standing. Until this release the floor was a self-imposed
constraint, raisable at zero cost because nothing was published and nobody was bound by it. `0.2.0`
puts four crates on a registry — **`happenstance-core`, `happenstance`, `happenstance-testkit` and
`happenstance-sqlite`**, named rather than counted, because a fifth crate is easy to mistake for a
member of that set: `xtask/src/package.rs`'s `PUBLISHABLE` constant holds five, adding
`happenstance-cloudflare`, and that constant answers a different question — what `cargo package
--list` and the licence/README check hold every crate to — from the one this atom answers, which is
what has actually been cut. `CHANGELOG.md`'s `[0.2.0-alpha.1] — 2026-08-16` entry is the only
release so far, `Cargo.toml`'s `version` field reads `0.2.0-alpha.1` today, and `RUNBOOK.md`'s phase
table has phase 12, "Publish `0.2.0`", at `not started`. This atom states what `0.2.0` **will**
bind when phase 12 lands, not a fact already true of the alpha.

## Which compiler, and why that one

The floor was forced by a dependency's build script, not by this workspace's code:
`happenstance-sqlite` depends on `rusqlite 0.40`, which pulls `libsqlite3-sys 0.38.1`, whose build
script invokes the `cfg_select!` macro, unavailable before Rust 1.88. The promised floor is set
higher than that evidence strictly requires, and deliberately: `rusqlite`, `libsqlite3-sys`,
`sqlx`, `sqlx-core` and `sqlx-postgres` declare no `rust-version` at all, five for five, so the
exact minimum is not discoverable from metadata and would have to be bisected against a build
script that the next silent dependency could invalidate again. The floor is tied to the toolchain
this project is developed and tested on instead.

## What a bump costs, and what is verified

An MSRV increase is a minor version bump named in `CHANGELOG.md`; a patch release cannot move it,
and below `1.0` cargo already treats a minor bump as incompatible, so a floor rise cannot reach a
consumer without a version change they chose. The `msrv` CI job installs `1.97.1`, runs `cargo hack
check --workspace --no-dev-deps --rust-version`, then the full suite with dev-dependencies included
— the second half exists because `--no-dev-deps` hides `proptest` and `tokio`, both pinned at
`1.85` with no headroom. The job is currently vacuous: `rust-toolchain.toml` pins the same `1.97.1`
it installs, so it proves the workspace builds at `1.97.1`, never in doubt, and not that `1.97.1` is
the minimum. It starts proving something the day the pin and the floor diverge, which is why it is
kept rather than deleted — the same reason ADR-0029 gave.

## Alternatives rejected

Promising only "latest stable" is the absence of a floor, not a floor, and moves the entire cost to
a consumer with a pinned toolchain. Lowering the published floor to `1.85` by pinning `rusqlite`
back to `0.37` buys a number at the cost of a real security and maintenance surface, re-fought at
every `rusqlite` release. A per-package `rust-version` is the strongest of the four and the one
with a real argument — a consumer of `happenstance` alone never links `libsqlite3-sys` — but is
rejected for now because it loses on verification capacity, not on merit: four floors need
per-package MSRV checking this workspace does not have, and one job checking the wrong one of four
is worse than one job checking the only one there is. A moving N-2-stable window breaks a build on
a date rather than on an upgrade chosen, which is the opposite of the property this policy exists
for.

## What this does not decide

The number — `1.97.1` is unchanged; this atom converts its status, not its value. `ADR-0004` and
`ADR-0029` are untouched: both stay `accepted` and byte-identical, amended rather than superseded,
on the precedent `ADR-0029` itself set against `ADR-0004`.
