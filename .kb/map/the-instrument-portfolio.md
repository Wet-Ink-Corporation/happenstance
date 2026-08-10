---
id: map-the-instrument-portfolio
title: "Map: the instrument portfolio — which crate exists to measure what"
kind: map
status: accepted
authority_tier: note
summary: >-
  Six skeleton crates exist to be disagreed with by a type checker. Each is an INSTRUMENT
  first and a target second, and each was chosen for the axis it sits at the far end of —
  `happenstance-postgres` assigns positions outside the transaction, `happenstance-neon` has
  no connection and no cursor, `happenstance-cloudflare` is the only `!Send` store.
depends_on: []
related:
  - playbook-freezing-a-port
  - governance-an-adapter-must-pass-the-suite
  - map-the-crate-dependency-rule
source_paths:
  - CLAUDE.md
  - docs/adapter-shapes.md
last_reviewed: 2026-08-09
---

# The instrument portfolio

## What a skeleton is

Real associated types and `todo!()` bodies, `publish = false`, and a scoped
`#![allow(clippy::todo)]` naming the phase that removes it. A skeleton exists to be
disagreed with by a type checker. **It is not an adapter until it has run the conformance
suite, and none of them has.**

## What each one measures

| Crate | The axis it sits at the end of |
| --- | --- |
| `happenstance-sqlite` | the ordinary case: rusqlite, borrowed transaction, serialised writers |
| `happenstance-cloudflare` | the workspace's only `!Send` store; wasm32; the whole reason ADR-0001 exists |
| `happenstance-postgres` | positions assigned **outside** the transaction — the visibility invariant is not free |
| `happenstance-neon` | no connection, no interactive transaction, no cursor; host **and** wasm32 |
| `happenstance-ladybug` | a graph projection store — a non-SQL batch |
| `happenstance-sync` | a port crate, not an adapter; peers depend on it as stores depend on the contract |

## Why the spread is the point

Four implementations that all serialise their writers and assign positions under a lock are
one storage shape wearing four hats. Postgres and Neon are in the tree so that a freeze has
something to be wrong about.
