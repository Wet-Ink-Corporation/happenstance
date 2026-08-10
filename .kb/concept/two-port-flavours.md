---
id: concept-two-port-flavours
title: Why there are two traits per port, and what a provided body owes
kind: concept
status: accepted
authority_tier: note
summary: >-
  Each port is written once with no `Send` bound; `trait_variant` derives a second trait
  that adds it. The bare flavour serves wasm32 / Workers, the `Send` flavour serves
  multi-threaded runtimes, and a provided body is CLONED into the variant — so it must
  type-check under both sets of bounds at once.
depends_on:
  - adr-0001-async-port-flavours
  - adr-0008-one-derivation-for-both-ports
related:
  - governance-never-async-trait
  - governance-bind-eventstore-not-sendeventstore
  - governance-read-returns-the-stream-at-the-top-level
source_paths:
  - crates/happenstance-core/src/store.rs
last_reviewed: 2026-08-09
---

# Two flavours, one definition

## The shape

A port is declared once, without `Send`. `trait_variant` generates the second trait by
adding the bound. Two traits exist; one is authored.

## What each flavour is for

- **Bare** (`EventStore`) — the weaker requirement. A `!Send` store on
  `wasm32-unknown-unknown` inside a Durable Object implements this and only this. Generic
  code binds it.
- **`Send`** (`SendEventStore`) — for callers that spawn onto a multi-threaded runtime.

## The cost, stated

A provided method body is cloned into the variant, so it must type-check under **both**
flavours' bounds simultaneously. That is not a free abstraction: `ProjectionStore`'s GAT
admits a provided body that cannot be made to compile on either flavour, while `EventStore`
structurally cannot exhibit the same problem. The scheme is shared between the two ports;
the provided-method budget is not (ADR-0008).

## The trap

Nesting a returned stream inside a future moves the `Send` marker onto the future and
silently drops it from the stream. See
[`governance-read-returns-the-stream-at-the-top-level`](../governance/read-returns-the-stream-at-the-top-level.md).
