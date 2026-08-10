---
id: governance-read-returns-the-stream-at-the-top-level
title: "`EventStore::read` returns the stream at the top level and is not `async`"
kind: governance
status: accepted
authority_tier: decision
summary: >-
  Nesting the stream inside a future silently drops `+ Send` from it on the `Send`
  flavour, defeating the two-trait design. TWO tests in `memory.rs` assert this and it
  takes both — one writes the bound at the definition, the other holds the stream across
  an await inside a real `tokio::spawn`. Deleting either re-opens the hole.
depends_on:
  - adr-0001-async-port-flavours
  - adr-0008-one-derivation-for-both-ports
related:
  - concept-two-port-flavours
source_paths:
  - CLAUDE.md
  - crates/happenstance-core/src/memory.rs
last_reviewed: 2026-08-09
---

# `read` returns the stream at the top level

## The rule

`EventStore::read` is **not** an `async fn`. It returns its stream as the outermost item
of its return type.

## Why

If the stream is nested inside a future, `trait_variant` marks the *future* `Send` on the
`Send` flavour and says nothing about the stream inside it. The bound a caller needs — a
stream they can hold across an await in a spawned task — is silently absent, and the
design's entire purpose is defeated without a single compiler error.

## Why it takes two tests, and not one

This is the part that is easy to get wrong while believing it is covered.

- `send_flavour_stream_is_send_in_generic_code` writes the bound **at the definition**, so
  the obligation is discharged before monomorphisation. That is the fix for an older test
  which asserted `Send` on a *concrete* stream and therefore passed by auto-trait leakage
  whatever the trait actually said.
- It is not sufficient alone. Under an `async fn read` refactor the outermost item is the
  future, `trait_variant` marks that future `Send`, and the assertion is satisfied by the
  wrong thing.
- `spawns_from_generic` is what rejects that refactor, because it holds the stream across
  an await inside a real `tokio::spawn`.

If you find yourself deleting either test, stop.
