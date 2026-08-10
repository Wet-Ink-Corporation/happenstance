---
id: governance-no-serde-in-the-contract-crate
title: Never put `serde` in `happenstance-core`'s default features
kind: governance
status: accepted
authority_tier: decision
summary: >-
  Payloads crossing the contract are opaque `Bytes`. The `serde` feature of
  `happenstance-core` covers envelope types only, for replication, and is never on by
  default. Read the crate name carefully — this constrains the CONTRACT crate, and says
  the opposite of what it used to before the rename.
depends_on:
  - adr-0003-opaque-payloads
related:
  - adr-0006-bare-name-to-the-typed-layer
  - adr-0016-the-wire-format
source_paths:
  - CLAUDE.md
  - crates/happenstance-core/Cargo.toml
last_reviewed: 2026-08-09
---

# Never put `serde` in `happenstance-core`'s default features

## The rule

`happenstance-core` treats payloads as opaque `Bytes` and carries no `serde` dependency by
default. Its `serde` feature exists for the replication envelope types and nothing else.

## The trap in the crate name

ADR-0006 gave the bare name `happenstance` to the **typed layer** and renamed the contract
crate to `happenstance-core`. So this constraint now says the opposite of what the same
sentence said before that rename:

- `happenstance-core` — the contract. No `serde` by default. This rule.
- `happenstance` — the typed layer, whose entire job is encoding. It is the crate that
  *will* depend on `serde`, and forbidding it there would forbid the thing the split
  exists to allow.

A reader who applies the rule by crate name without checking which layer they are in gets
it exactly backwards, which is why the distinction is written down rather than assumed.

## How it is enforced

`cargo xtask ci` builds the contract crate's documentation with `--no-default-features`,
and `cargo xtask lint-core-alloc-features` (D12) checks that the crate names `serde/alloc`
and `base64/alloc` inside its own `serde` feature rather than relying on someone else to
enable them.

## What would change it

Nothing short of abandoning `no_std` support, which is a new ADR.
