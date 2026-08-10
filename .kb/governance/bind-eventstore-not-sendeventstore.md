---
id: governance-bind-eventstore-not-sendeventstore
title: Bind `EventStore`, not `SendEventStore`, in generic code
kind: governance
status: accepted
authority_tier: decision
summary: >-
  `EventStore` is the weaker requirement and accepts both flavours, so generic code binds
  it. Import only ONE of the two names per module — having both in scope makes method
  calls ambiguous, and the resulting error names neither the cause nor the fix.
depends_on:
  - adr-0001-async-port-flavours
related:
  - concept-two-port-flavours
  - governance-never-async-trait
source_paths:
  - CLAUDE.md
last_reviewed: 2026-08-09
---

# Bind `EventStore`, not `SendEventStore`

## The rule

Generic code takes `S: EventStore`. It takes `S: SendEventStore` only when it genuinely
requires the `Send` flavour — spawning onto a multi-threaded runtime, for instance.

## Why

`EventStore` is the weaker bound: every `SendEventStore` is one, and a `!Send` store is one
too. Binding the stronger trait by reflex excludes the wasm32 adapters from code that had
no reason to exclude them, and the exclusion shows up as a confusing trait-resolution
error in a downstream crate rather than as a decision anyone made.

## The import rule that goes with it

**Import only one of the two names per module.** Both in scope makes a method call
ambiguous, and the diagnostic points at the call site rather than at the imports, so the
reader looks for a problem in code that is correct.
