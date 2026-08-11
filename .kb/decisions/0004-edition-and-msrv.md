---
id: kb-decision-0004
title: Rust 2024 edition, MSRV 1.85
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0004
reversibility: medium
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Provisional, and amended rather than superseded by ADR-0029 — the number in this decision is no
  longer the MSRV, and its reasoning is what ADR-0029 acted on. Edition 2024 with rust-version
  1.85 and the toolchain pinned at 1.97.1 in rust-toolchain.toml, deliberately two different
  facts: the floor a consumer may build with, and the compiler contributors use. Edition 2024 is
  not incidental — its rule that return-position impl Trait captures all in-scope lifetimes by
  default is what lets EventStore::read return a stream borrowing &self with no explicit + '_.
  Let-chains were avoided as post-1.85, which ADR-0029 reverses. The MSRV is verified only in CI,
  by a job running cargo hack check --no-dev-deps --rust-version on a pinned toolchain, and the
  standing instruction if that job fails is to raise the MSRV rather than work around it — which
  is what happened. happenstance-core is no_std plus alloc under --no-default-features. Policy: an
  MSRV bump is a minor version bump and is called out in the changelog; it becomes a promise
  rather than a preference at first publish, phase 12.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0004-edition-and-msrv.md
  - references/adr/0004-edition-and-msrv.md
  - rust-toolchain.toml
  - Cargo.toml
  - CLAUDE.md
last_reviewed: 2026-08-10
---

# Rust 2024 edition, MSRV 1.85

## Context

A library crate has to state which compilers it supports, and that choice interacts directly with
the language features the design depends on. At the time this decision was taken, the features the
project actually needed were: `async fn` / return-position `impl Trait` in traits (stabilised
1.75, load-bearing for the async-port-flavours decision, ADR-0001), `core::error::Error` for
`no_std` support (1.81), `Option::is_none_or` (1.82), and edition 2024 itself (1.85).

## Decision

Edition 2024, `rust-version = "1.85"`, toolchain pinned to 1.97.1 in `rust-toolchain.toml` — two
deliberately different numbers. The `rust-version` is the floor a downstream consumer's own
toolchain must clear; the `rust-toolchain.toml` pin is the compiler contributors actually build
with, and is free to run ahead of the floor.

Edition 2024 is not incidental to this design. Its rule that return-position `impl Trait` captures
all in-scope lifetimes by default is what lets `EventStore::read` return a stream borrowing
`&self` with no explicit `+ '_` written anywhere in the signature — a signature that constraint 3
in `CLAUDE.md` treats as load-bearing for the whole two-trait (`Send` / non-`Send`) design.

**Let-chains were avoided**, despite being stable in edition 2024 generally, because they landed
in the language at 1.88 specifically — after this ADR's 1.85 floor. Where one would have read
naturally, a `match` and a comment explained why, so the workaround was visible rather than silent.

## Consequences

**Good.** 1.85 was old enough to be undemanding for anyone building against the crate, and new
enough to carry everything the design needed at the time. Edition 2024 supplies the lifetime-
capture rule the ports rely on.

**Bad.** The MSRV is verified only in CI, by a dedicated job running
`cargo hack check --workspace --no-dev-deps --rust-version` on a pinned toolchain — not checked
against a local install of the floor version. The standing instruction, if that job ever fails, is
to raise the MSRV rather than work around the failure. That is exactly what happened: five of the
five database crates in the workspace declare no `rust-version` at all, so `cargo hack
--rust-version` could not protect the floor against them, and only running the compiler found the
problem — which is the reasoning the later decision to raise the MSRV (ADR-0029) acted on to move
the floor to 1.97.1.

**Neutral.** `happenstance-core` is `no_std` + `alloc` under `--no-default-features`, verified in
the feature matrix. Nothing needed that at the time; it cost a handful of `alloc::` imports and
kept the option open for embedded or constrained targets.

## Policy

An MSRV bump is a minor version bump, and is called out in the changelog. This is a preference
until first publish (phase 12) and a promise to downstream consumers afterward — nothing is
published yet, so `rust-version = "1.85"` was, at the time, a self-imposed constraint raisable at
zero cost to anyone, not a hard limit to weigh a dependency against.

## Status and what amended it

Provisional at authorship (2026-08-05, alongside the initial scaffold, before the code it
constrains existed) and amended — not superseded — by ADR-0029, raised at phase 2. The number in
this decision's title is no longer the MSRV; it is 1.97.1. The body stays verbatim because its
*reasoning* is what ADR-0029 acted on rather than overturned: this is the decision that establishes
the floor is a preference until first publish and invites exactly the kind of trade ADR-0029 made.
The let-chains avoidance is rescinded in effect (1.97.1 is well past 1.88) but the constraint stays
written here as the historical record of why it existed at the time.
