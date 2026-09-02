---
id: kb-decision-0036
title: The projection port is not frozen at 0.2.0 and ships behind unstable-projection
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0036
reversibility: medium
phase: 6
supersedes: null
superseded_by: null
summary: >-
  ProjectionStore is not frozen at 0.2.0. It ships behind the off-by-default
  unstable-projection feature in happenstance-core and happenstance, forwarded as
  projection-store on adapters, with its semver exemption documented on the module.
  This is PS-3's SHOULD evaluated against PS-2's [FROZEN] two-part bar, part by part,
  and it is the first time anything in the tree has applied that bar to a real
  adapter set. Part 1 is met: CheckpointOnlyStore exists as a registered Defect and
  demonstrably fails the suite, so the rule rejects something. Part 2 is not: exactly
  one storage adapter has run projection_store_conformance!, SqliteProjectionStore
  against a real temporary file, and it fails both halves — SQLite's Batch is an
  owned write set under ADR-0017, so it sits at the buffered end rather than the
  live-transaction end, and no cannot-hold-across-await adapter has passed at all.
  Testkit fixtures and the outside-projection-adapter example are worth having and
  are not adapters; counting them is the monoculture PS-2's Rejects clause names. The
  rejected arm is freezing now, and its cost is asymmetric and unrecoverable: a
  frozen port is a semver promise, a published version can be yanked but never
  removed, and the axis the port is most likely to be wrong about is the one no
  passing adapter occupies. The cost of the arm taken is accepted and stated: a
  consumer must name a feature to get a projection store at all, and
  cargo-semver-checks will not police the surface. PS-3 keeps [PROVISIONAL] — a
  satisfied SHOULD is not a moved marker — and PS-2 is applied, not amended. Carries
  the routed finding that happenstance-sqlite's default set forwarded the gate on,
  defeating off-by-default for the one crate a consumer installs; resolved
  2026-09-02, default is now ["event-store"] alone, and the flag keeps its name
  because event-store/projection-store names roles across three adapters while
  unstable-projection names maturity.
depends_on:
  - kb-decision-0017
related:
  - kb-decision-0030
  - kb-open-question-adapter-default-projection-feature-001
source_paths:
  - .kb/_intake/ps-3-projection-port-ships-gated.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs
  - crates/happenstance-sqlite/tests/projection.rs
  - crates/happenstance-sqlite/Cargo.toml
  - crates/happenstance-core/Cargo.toml
  - references/evaluation/projection-batch-shape-evidence.md
last_reviewed: 2026-09-02
---

# The projection port is not frozen at 0.2.0 and ships behind unstable-projection

## Decision

`ProjectionStore` is **not** frozen at `0.2.0`. It ships behind the off-by-default
`unstable-projection` feature in `happenstance-core` and `happenstance`, forwarded
as `projection-store` on adapters, with the semver exemption documented on the
module itself. PS-3 (`spec/SPECIFICATION.md`:4880-4881) is a SHOULD conditioned on
PS-2's bar being met; this decision is that evaluation taken part by part, applied
rather than argued, and it is the first time anything in the tree has run PS-2's bar
against a real adapter set.

## PS-2's bar, evaluated

PS-2 (`spec/SPECIFICATION.md`:4864-4867) is `[FROZEN]` and is a conjunction of two
parts. Both must hold for freezing to be the right call.

**Part 1 — a hostile store must fail the suite. Met.** `CheckpointOnlyStore` exists
at `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs`:44,
registered as a `Defect` with `NAME = "CheckpointOnlyStore"`, and the mutant
registry asserts it fails exactly the rules its row claims. This is a rule that
demonstrably rejects a wrong implementation, not one nothing can fail.

**Part 2 — two adapters at opposite ends of the batch-shape axis must pass. Not
met.** The named ends are a live-transaction adapter (rusqlite or `sqlx`) and one
that cannot hold anything across an await (Workers `SqlStorage` or Neon over
one-shot HTTP). Exactly one storage adapter has run `projection_store_conformance!`:
`SqliteProjectionStore`, at `crates/happenstance-sqlite/tests/projection.rs`:226,
against a real temporary file — and `spec/SPECIFICATION.md`:392 states plainly that
the other four impls still run against nothing. It fails both halves at once:
SQLite's `Batch` is an *owned* write set under `kb-decision-0017`, statements pushed
and replayed at commit, so it sits at the buffered end rather than the
live-transaction end; and no cannot-hold-across-await adapter has passed either.
`NeonProjectionStore` carries real bodies in all four port methods but runs against
nothing, and there is no Workers projection store at all.

What has passed besides SQLite — `MemoryProjectionFixture`,
`BufferingProjectionFixture`, the blocking and `wasm32` test harnesses, and
`examples/outside-projection-adapter` — is worth having and is not an adapter.
Counting it would be the precise error PS-2's Rejects clause names: freezing the
port against a monoculture of in-process, non-adapter implementations.

## The arm that lost, and what it costs

**Freezing at `0.2.0`.** Rejected because part 2 is unmet, and the cost of being
wrong here is asymmetric and unrecoverable: a frozen port is a semver promise, a
published version can be yanked but never removed, and the axis the port is most
likely to be wrong about — whether a batch can be a live borrowed handle rather than
an owned set — is exactly the one no passing adapter currently occupies.

The arm taken costs something real too, and it is accepted rather than hidden: a
consumer must name a feature to get a projection store at all, the surface makes no
semver promise, and `cargo-semver-checks` will not police it.

## What this does not decide

PS-3 keeps `[PROVISIONAL]` — a satisfied SHOULD is not a moved maturity marker; its
falsifier remains PS-2's bar being met before `0.1`, which has not happened. PS-2
itself is untouched: it is applied here, not amended, which is the point of writing
a verdict against a frozen clause instead of editing it.

## The routed finding, and its resolution

`cargo add happenstance-sqlite` turned the unfrozen port on without the consumer
naming it: the crate's `default` carried `projection-store`, which forwards
`happenstance-core/unstable-projection`, walking around the gate the other two
crates hold off-by-default. The objection that had held back fixing this — that
`crates/happenstance-sqlite/tests/projection.rs` would stop running bare — was
measured and found false: that file is gated on `all(feature = "projection-store",
feature = "conformance")`, two features, and `conformance` was never in `default`;
the family already ran 0 tests bare and 24 under `--all-features` either way.
Resolved 2026-09-02: `happenstance-sqlite`'s `default` is now `["event-store"]`
alone. The flag keeps its name rather than being renamed to echo
`unstable-projection`, because `event-store`/`projection-store` names a role shared
by three adapters and `unstable-projection` names maturity — a signal that already
lives on `happenstance-core`'s own gate and that this flag forwards rather than
duplicates.

Still owed and routed rather than resolved here: `happenstance-neon` and
`happenstance-postgres` carry the identical `default = ["event-store",
"projection-store"]`. Both are stubs whose projection bodies are `todo!()`, so
nothing ships from them today; the fix belongs to the `postgres-and-neon-stores`
project, before either crate is published. See
`kb-open-question-adapter-default-projection-feature-001`.

## Alternatives rejected

Freezing now and accepting the monoculture, rejected on PS-2's own Rejects clause.
Waiting for both axis ends to have a passing adapter before writing any verdict,
rejected because PS-3's SHOULD needed evaluating on its own terms now — a decision
that a bar is unmet is still a decision, and leaving it silently unstated would have
left `0.2.0` to freeze the port by default rather than by argument.
