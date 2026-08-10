---
id: reference-adapter-shapes
title: "Measured: what the six skeletons told the type checker"
kind: reference
status: accepted
authority_tier: reference
summary: >-
  The compiler errors and compiling call sites the six skeleton crates produced when real
  associated types were written against the ports. This is the evidence phase 4 froze the
  contract on — the difference between settling a design against something that compiles and
  settling it against an argument.
depends_on: []
related:
  - map-the-instrument-portfolio
  - concept-the-batch-shape-axis
  - playbook-freezing-a-port
source_paths:
  - docs/adapter-shapes.md
last_reviewed: 2026-08-09
---

# What the skeletons told the type checker

## What this records

For each skeleton crate, the associated types it actually declares and what the compiler
said about them — including the two independent reasons `rusqlite::Transaction<'_>` fails
`SendProjectionStore`, and the confirming half from `sqlx`, whose
`Transaction<'static, Postgres>` owns its connection and is `Send`.

## Why it is a reference atom and not a decision

It records what the compiler answered, not what anyone chose. The choices it informs are
ADR-0012 through ADR-0019. An ADR that quotes these errors is doing what
[`playbook-freezing-a-port`](../playbook/freezing-a-port.md) asks for; an ADR that asserts a
port "survives" is not.
