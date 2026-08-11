---
id: kb-decision-0029
title: The MSRV is 1.97.1
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0029
reversibility: medium
phase: 2
supersedes: null
superseded_by: null
summary: >-
  The MSRV is raised from 1.85 to 1.97.1 and rusqlite stays at 0.40. This amends ADR-0004 rather
  than superseding it - that decision's body stays verbatim, because its reasoning is what this
  one acted on: it is ADR-0004 that says the floor is a preference until first publish and invites
  the trade. rust-version and rust-toolchain.toml remain two different facts and are not collapsed
  into one. The forcing evidence is that libsqlite3-sys 0.38.1's build script uses cfg_select!, and
  that it, rusqlite, sqlx, sqlx-core and sqlx-postgres declare no rust-version at all, so neither
  cargo hack --rust-version nor resolver = "3" could see the break coming; only running the
  compiler found it. Let-chains, stable since 1.88, are now available, and CLAUDE.md's constraint
  forbidding them is rewritten. The msrv CI job is kept with a comment although it is now vacuous,
  because it runs the same compiler the gate runs and stops being vacuous the day the pin and the
  floor diverge. Phase 12 must revisit the floor at first publish. Rejected: pinning rusqlite back
  to 0.37, a per-package rust-version, and raising only as far as cfg_select! requires.
depends_on:
  - kb-decision-0004
related: []
source_paths:
  - .kb/_intake/0029-msrv-raised-to-1-97-1.md
  - references/adr/0029-msrv-raised-to-1-97-1.md
  - rust-toolchain.toml
  - Cargo.toml
  - CLAUDE.md
last_reviewed: 2026-08-10
---

# The MSRV is 1.97.1

## Context

Phase 2 built six adapter skeletons. Five compile at the 1.85 floor ADR-0004 set. The sixth,
`happenstance-sqlite`, does not: its dependency `rusqlite 0.40` pulls `libsqlite3-sys 0.38.1`,
whose **build script** invokes the `cfg_select!` macro, which is unavailable before 1.88.
`cargo hack check --no-dev-deps --rust-version` and `resolver = "3"`'s MSRV-aware resolution both
depend on `rust-version` metadata to catch a break before it happens, and `libsqlite3-sys`,
`rusqlite`, `sqlx`, `sqlx-core` and `sqlx-postgres` declare none at all — five packages out of
five. Only running the compiler surfaced the failure.

Two facts bounded the choice. The break sits in a dependency's build script, not in this
project's code: `happenstance-sqlite` is a `publish = false` skeleton whose every body is
`todo!()`. And it is recoverable — `rusqlite 0.37` / `libsqlite3-sys 0.35` was verified to build
at both 1.85 and 1.97.1, so staying at the old floor was available, at the cost of an older
SQLite binding pinned for a reason unrelated to SQLite.

## Decision

The MSRV moves to **1.97.1**, and `rusqlite` stays at 0.40. This amends ADR-0004 rather than
superseding it: ADR-0004's body stays verbatim because its reasoning is what licensed this move —
it marked the 1.85 floor `provisional`, and CLAUDE.md's fifth constraint states the corollary
outright: weigh the floor, do not obey it, until first publish turns it into a promise. Nothing
is published yet, so nothing downstream is pinned to the old number.

`rust-version` and `rust-toolchain.toml` now carry the same number, and they stay two different
facts rather than collapsing into one: the toolchain pin is what contributors build with and
moves whenever someone wants a newer compiler; the MSRV is what consumers may build with and
moves only by ADR. The `msrv` CI job — `cargo hack check --no-dev-deps --rust-version` on a
pinned toolchain, then a full `cargo test --workspace --all-features` at that pin — is kept
rather than deleted, with a comment naming why it is currently vacuous: it now runs the same
compiler the gate runs, so it proves nothing until the pin and the floor next diverge.

One immediate, unforced consequence: let-chains stabilised in 1.88 and are now available.
CLAUDE.md's constraint that forbade them for the MSRV's sake is rewritten in the same change; no
code is rewritten to use them yet, because a decision and a refactor should not land in one
commit.

## Rejected

**Pin `rusqlite` back to 0.37.** Verified working at both toolchains, and the narrowest fix.
Rejected because it buys the floor by freezing a dependency version for a reason unrelated to
what that dependency does, and would hand phase 8's driver decision (ADR-0022) a version already
chosen by an MSRV constraint rather than by anything about SQLite.

**Give `happenstance-sqlite` its own, higher `rust-version`.** Honest per-package metadata, since
the crate is `publish = false` and the three published crates could have kept 1.85 truthfully.
Rejected on moving parts: CI would need a second toolchain, `cargo test --workspace` at 1.85
would still fail without an exclusion list to maintain, and "the MSRV" would become ambiguous in
conversation — worse than raising it.

**Raise only as far as `cfg_select!` requires.** Would have preserved more headroom, but the
exact floor is not discoverable from any metadata; it would have to be found by bisecting
toolchains against a build script, and the next dependency declaring nothing would invalidate it
again. A number tied to the pin is at least a number with a reason.

Phase 12 must revisit the floor at first publish, when it stops being a preference and starts
being a promise to a consumer who may not even build `happenstance-sqlite`.
