---
id: playbook-freezing-a-port
title: "Playbook: freezing a port"
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  A port is only as well designed as the SPREAD of what implements it. Before freezing one,
  name the axis it is most likely to be wrong about and check that something in the
  workspace sits at the other end of it. Four adapters that all serialise their writers and
  assign positions under a lock are one storage shape wearing four hats.
depends_on: []
related:
  - map-the-instrument-portfolio
  - concept-the-batch-shape-axis
  - question-projection-store-port-is-provisional
source_paths:
  - CLAUDE.md
  - docs/adapter-shapes.md
last_reviewed: 2026-08-09
---

# Freezing a port

## The bar

**A port frozen against one storage shape is shaped like that shape.**

`MemoryEventStore`, a `RefCell` store, rusqlite and a Durable Object all serialise their
writers and assign positions under a lock — four adapters, one storage shape. A port frozen
against them is frozen against SQLite wearing four hats.

## The steps

1. **Name the axis** the port is most likely to be wrong about. For `ProjectionStore` it is
   the batch shape: borrowed-from-the-store versus owned-and-`Send`.
2. **Find the thing at the other end of it.** `happenstance-postgres` assigns positions
   *outside* the transaction; `happenstance-neon` has no connection, no interactive
   transaction and no cursor. They are instruments first and targets second.
3. **Settle it against something that compiles**, never against an argument. A skeleton
   exists to be disagreed with by a type checker.
4. **Write the ADR before the code it constrains**, quoting the compiler errors rather than
   asserting that the port "survives".
5. **Build a hostile implementation and require the suite to fail it.** For phase 6 that is
   `CheckpointOnlyStore`, which commits the checkpoint and silently drops the read-model
   write. If it passes, the port is not frozen.

## The bar that carries no information

"The skeletons compile unchanged." A phase that deliberately changes a signature makes that
unsatisfiable, and a phase that changes nothing makes it free. The bar that carries
information is narrower: the skeletons' underlying types, error types and bodies are
unchanged, and only the named parameter differs.
