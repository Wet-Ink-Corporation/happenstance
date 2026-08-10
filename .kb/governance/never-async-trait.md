---
id: governance-never-async-trait
title: Never introduce `#[async_trait]`
kind: governance
status: accepted
authority_tier: decision
summary: >-
  `#[async_trait]` injects a `+ Send` bound, which makes the wasm32 / Cloudflare Workers
  target impossible. Ports are defined once with no `Send` bound and `trait_variant`
  derives the `Send` flavour. This is not a style preference; it is the constraint the
  whole two-trait design exists to satisfy.
depends_on:
  - adr-0001-async-port-flavours
related:
  - concept-two-port-flavours
  - governance-bind-eventstore-not-sendeventstore
source_paths:
  - CLAUDE.md
  - crates/happenstance-core/src/store.rs
last_reviewed: 2026-08-09
---

# Never introduce `#[async_trait]`

## The rule

No port, in any crate of this workspace, is declared with `#[async_trait]`.

## Why

The macro rewrites an async method into one returning `Pin<Box<dyn Future + Send>>`. The
`+ Send` is not optional and not configurable. `happenstance-cloudflare` targets
`wasm32-unknown-unknown` inside a Durable Object, where the futures are `!Send` — so a
single `#[async_trait]` anywhere on the port path deletes that target.

The alternative that lost: hand-writing both flavours. It was rejected because a provided
body would then exist twice and could drift; `trait_variant` clones one body into the
variant, which is also why a provided body must type-check under **both** flavours' bounds
at once (ADR-0008).

## How it is enforced

`cargo xtask ci` builds `happenstance-core` for `wasm32-unknown-unknown` on every run, and
that step names the contract crate on purpose. It is the standing guard: introducing the
macro fails the build rather than the review.

## What would change it

A target story that no longer includes Workers, or a Rust release that makes the derived
flavour unnecessary. Either is a new ADR, not an edit here.
