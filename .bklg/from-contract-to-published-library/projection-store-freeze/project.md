---
id: HS-P0010
uid: 7be898
type: project
slug: projection-store-freeze
title: Freeze ProjectionStore behind a suite that can fail
parent: HS-I0006
initiative: from-contract-to-published-library
project: projection-store-freeze
status: in-review
process: project
stage: review
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-15T19:10:14.836Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# Freeze ProjectionStore behind a suite that can fail

## One-line objective

Give `ProjectionStore` a conformance suite that a deliberately wrong store
**fails by name**, and freeze the port on that evidence rather than on the
doc comment currently standing in for it.

## How this advances the initiative

`ProjectionStore` carries the largest provisional block in the specification —
roughly seventeen `PS-*` clauses, with **PS-2 a single gate under thirteen rows**
(`RUNBOOK.md:588-610`) — and the invariant it exists to protect is stated on the
trait (`crates/happenstance-core/src/projection.rs:13-30`) and enforced by
nothing. The module's own header says it: *"a port without a conformance suite is
a guess"* (`crates/happenstance-core/src/projection.rs:3-11`).

This is the initiative's **substrate root** (`_decomposition.md` Sequencing, rank
0). Four sibling projects build against a projection port; if it is frozen wrong,
each of them absorbs the cost. Freezing it first is the same blast-radius
sequencing the runbook has used throughout (`RUNBOOK.md:38-40`).

It owns the adapter author's half of the initiative's acceptance spine — AC-04
("told when they are finished"), AC-05 ("told, with a reason, where a guarantee
does not apply"), BR-13, DoD 7 — and the two design tensions that decide what the
suite *says to a human*: DT-3 and DT-8 (`../initiative.md` Open design tensions).

The freeze itself is **not called earned here.** That verdict is
`ladybug-projection-store`'s (HS-P0015), after a structurally unlike batch shape
has run it (`../initiative.md` Risks, row 1).

## In scope (this project)

Bounded by `RUNBOOK.md` phase 6 (`RUNBOOK.md:3848-3965`), which this project maps
onto one-for-one.

1. **ADR-0017, ADR-0018, ADR-0019, written first** (`RUNBOOK.md:296-298`):
   what a projection batch owns and what vocabulary writes into it (PS-4 – PS-15);
   how a projection is returned to "never run" and what may refuse it
   (PS-16 – PS-20); what happens when `apply` fails (PS-26 – PS-30). Each quotes
   the compiler transcripts in `references/adapter-shapes.md` rather than
   asserting the port "survives", per the phase's own exit criterion
   (`RUNBOOK.md:3952-3953`).
2. **The port shape.** Drop the batch lifetime to `type Batch;`, and grow the
   **write seam** the suite needs — today generic code can `begin` and `commit`
   and cannot write anything in between
   (`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`;
   `crates/happenstance-core/src/projection.rs:92-99`). Plus the stated Drop
   contract (PS-7): rolls back **and** leaves the store usable.
3. **`projection_store_conformance!`**, emitted through the existing registry so
   it inherits the tokio / blocking / wasm emitters unchanged
   (`crates/happenstance-testkit/src/registry.rs:93-103`,
   `crates/happenstance-testkit/src/lib.rs:62-82`), with the rule set enumerated
   in exactly one place and an orphan-rule meta-test over it
   (`crates/happenstance-testkit/src/lib.rs:84-92`).
4. **The projection fixture contract**: a `write_probe(&mut Batch)` and an
   out-of-band `read_probe(&Store)`, without which three of the six projection
   rules cannot observe the read model at all (`RUNBOOK.md:3882-3889`). Its
   capability surface follows the existing `Capability` / `RuleOutcome`
   machinery (`crates/happenstance-testkit/src/contract.rs:25-63, 355-433`).
5. **`CheckpointOnlyStore`** in the testkit's own `tests/`, registered in the
   mutant registry under ADR-0010's exactness discipline
   (`crates/happenstance-testkit/tests/mutation_coverage.rs:1-31`;
   `.kb/decisions/0010-the-suite-must-prove-itself.md`). This is the phase's
   whole bar: *if it passes, the port is not frozen* (`RUNBOOK.md:3890-3892`).
6. **`MemoryProjectionStore`** behind the `memory` feature — the oracle, the
   doctest target, and the cold-start fix — together with the documented `E0195`
   spelling trap that currently has nothing to copy from
   (PS-34, PS-36; `RUNBOOK.md:3898-3903`).
7. **`reset`**, `commit`'s position semantics (PS-21, PS-22), read-your-writes
   within a chunk (PS-12) and the per-projection failure policy
   (PS-26 – PS-30) (`RUNBOOK.md:3904-3921`).
8. **DT-3 and DT-8 resolved and recorded** in this project's `_design.md`,
   consuming `.kb/open-questions/cf-40-fixture-limits-ownership.md` rather than
   rediscovering it.
9. **Clause disposition for PS-1 – PS-37**, including the two clauses whose
   `MUST` is narrower than the rule table assigns them
   (`.kb/open-questions/ps-1-states-no-progress-obligation.md`,
   `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`) and the
   unvalidated `ProjectionId` this phase forces
   (`.kb/open-questions/projection-id-is-unvalidated.md`), plus **evidence** for
   PS-3 — did the two batch shapes disagree?
10. **Skeleton compatibility**: the Ladybug and Postgres skeletons compile with
    no change other than the lifetime parameter's removal — same `Batch` type,
    same error type, same bodies (`RUNBOOK.md:3942-3948, 3958-3960`).

## Out of scope (this project)

Each exclusion names the sibling that owns it, per `../_decomposition.md` Scope
seams.

- **The graph-shaped third batch shape, and the written freeze verdict** —
  `ladybug-projection-store` (HS-P0015). A verdict of "it did not hold" is a
  result, not a failure, and it is not this project's to pre-empt.
- **The application-facing `Projection` trait, the runner, ADR-0020/0021 and
  PS-33's falsifier** — `typed-layer-and-alpha-release` (HS-P0011)
  (`RUNBOOK.md:3971-3982`).
- **Any SQL projection adapter shipped as a product**, and ADR-0022 —
  `sqlite-durable-store` (HS-P0012). A rusqlite batch used here is an
  *instrument*, not that adapter.
- **The PS-3 verdict** on whether the port ships behind `unstable-projection` at
  publish — `publication-and-positioning` (HS-P0016). This project supplies the
  evidence; that project makes the call.
- **The first exercise of the fixture-limits policy against real numeric
  ceilings** — `cloudflare-durable-object-store` (HS-P0013). The *policy* is
  decided here (`_intake-brief.md` Clauses).
- **Amending anything `[FROZEN]`.** The `EventStore` contract, the values
  crossing it and the wire format are out of scope for amendment; a change there
  is a new decision atom and a re-plan, not a line edit (`../initiative.md`
  Out of scope). PS-1 is itself `[FROZEN]` — see Risks.
- **Any event-store rule, fixture or adapter change** beyond what the projection
  suite structurally requires.

## Derived requirements

Expanded from the initiative requirements this project owns
(`../_decomposition.md` Traceability matrix).

- **DR-01** *(from BR-04ᶠ, DoD 7)* — the port is frozen only once its own
  invariant is enforced by an executable rule that a wrong store fails.
- **DR-02** *(from AC-04)* — one entry point, invoked against a fixture, yields a
  pass or a **named** failure for every rule, with no rule silently absent from
  the run.
- **DR-03** *(from BR-13, AC-05)* — where a guarantee does not apply, the
  consumer is told so with the fixture's stated reason; a declined capability
  still appears in the output and never vanishes from the binary.
- **DR-04** *(from the house rule, `CLAUDE.md` "The rule that matters")* — no
  projection rule is added without a named wrong implementation that it rejects,
  and that implementation lives in the testkit's own `tests/`.
- **DR-05** *(from DoD 7)* — two **structurally unlike** batch shapes pass the
  suite; where the second shape comes from is settled in the architecture brief,
  not assumed.
- **DR-06** *(from BR-15)* — every answer this project settles lands as a
  decision atom naming the alternatives that lost, and the matching
  open-question atoms are **resolved rather than deleted**.
- **DR-07** *(from DT-3)* — how a consumer learns a guarantee is absent has one
  authoritative source, chosen deliberately between the suite's output and
  per-adapter documentation.
- **DR-08** *(from DT-8)* — whose adapter-author bar the suite holds is decided;
  if an outside author's, a documented extension surface exists and DR-02/DR-03
  hold against an implementation nobody here wrote.
- **DR-09** *(from BR-12)* — the projection suite runs on the constrained
  single-threaded target in the same gate run as everything else, not as a
  separately maintained subset.

## Acceptance criteria

Project-grain and observable. These are the spine the story map must cover.

- **AC-001** — `projection_store_conformance!` exists, expands to one test per
  rule, and its rule set is written in exactly one enumeration; a meta-test fails
  if a rule in the projection rules module is absent from that enumeration.
  *(Mirrors `for_each_event_store_rule!` and `no_orphan_rules`,
  `crates/happenstance-testkit/src/lib.rs:84-92`.)*
- **AC-002** — `CheckpointOnlyStore` — a store that commits the checkpoint and
  silently drops the read-model write — **fails at least one named rule**, and
  passes every rule it does not declare, under the mutant registry's exactness
  meta-tests (`crates/happenstance-testkit/tests/mutation_coverage.rs`).
- **AC-003** — every projection rule has a registered wrong implementation that
  fails it, with stated provenance naming the real implementation mistake it
  models; no pass rate is quoted over the mutant set
  (`.kb/decisions/0010-the-suite-must-prove-itself.md`).
- **AC-004** — two structurally unlike batch shapes pass the whole suite, each
  named in the proof artefact, and the second shape's provenance is a decision
  recorded in the architecture brief rather than an assumption.
- **AC-005** — a rule whose projection capability is declined is still emitted as
  a test, returns a skip carrying the fixture's stated reason, and is
  distinguishable from a pass in CI output; asserted on `RuleOutcome` values
  rather than on stdout (`crates/happenstance-testkit/src/contract.rs:458-537`).
- **AC-006** — DT-3 is resolved in `_design.md` with **one** source named
  authoritative, and the resolution cites
  `.kb/open-questions/cf-40-fixture-limits-ownership.md` rather than restating it.
- **AC-007** — DT-8 is resolved in `_design.md`. If the bar is held for an
  outside author, a documented extension surface exists and AC-002 and AC-005
  are demonstrated against a fixture written from the documentation alone.
- **AC-008** — ADR-0017, ADR-0018 and ADR-0019 are accepted **before** the port
  change lands, each quoting a compiler transcript from
  `references/adapter-shapes.md`, each naming the alternatives that lost, and the
  open-question atoms they answer are marked resolved, not deleted; `redkiln
  validate --kb` passes.
- **AC-009** — `type Batch` carries no lifetime parameter, and generic suite code
  can write into a batch and observe the resulting read model out of band
  (`write_probe` / `read_probe`) without reaching into an adapter's internals.
- **AC-010** — a batch dropped without `commit` or `rollback` rolls back **and**
  leaves the store usable, proved by a rule that rejects a store which loses its
  connection and answers `Busy` forever afterwards (PS-7, `RUNBOOK.md:3893-3897`).
- **AC-011** — `reset` clears rows and checkpoint in one unit of work, is scoped
  to one `(store, ProjectionId)`, and is refusable; a rule rejects
  `commit(empty, id, FIRST)` as a substitute, which silently skips event 1.
- **AC-012** — `MemoryProjectionStore` ships behind the `memory` feature, is the
  doctest target for the port, and the `E0195` spelling trap is documented where
  an implementer meets it (PS-34, PS-36).
- **AC-013** — the Ladybug and Postgres skeletons compile with **no change other
  than the removal of the batch's lifetime parameter** — same underlying `Batch`
  type, same error type, same bodies — and `cargo xtask ci` is green on the
  workspace.
- **AC-014** — `crates/happenstance-core/src/projection.rs` no longer describes
  itself as provisional, **or** the module is behind `unstable-projection` and
  says why; every `PS-1` – `PS-37` clause carries an accurate maturity marker and
  `cargo xtask spec-trace` is green.
- **AC-015** — the PS-3 evidence is recorded as a written finding — *did the two
  batch shapes disagree?* — and handed to `publication-and-positioning`; no
  verdict on the `unstable-projection` exposure is made here.
- **AC-016** — every projection rule runs and passes under the `wasm32` emitter
  in the same `cargo xtask ci` run as the host emitters, not as a separately
  maintained subset.

## Definition of done (boundary-level)

Observed at this project's boundary, on a clean checkout, before it is called
merged.

1. `CheckpointOnlyStore` **fails** the projection suite by name, and the failing
   test's name is quoted in the closeout — the initiative's DoD 7, first half.
2. Two structurally unlike batch shapes **pass** the projection suite — DoD 7,
   second half.
3. ADR-0017, ADR-0018 and ADR-0019 are accepted atoms under `.kb/decisions/`,
   and every open-question atom they answer is resolved rather than deleted.
4. `cargo xtask ci` is green on the whole workspace, including the mandatory
   `wasm32` steps and `cargo xtask spec-trace`.
5. Every `PS-*` clause disposition is recorded, and none is left provisional with
   an empty falsifier where this project was its owner.
6. DT-3 and DT-8 each carry a recorded resolution in `_design.md`.
7. The public projection API surface has passed the design-stage review under the
   repurposed `.redkiln/templates/_design.md` — signatures, visibility, what the
   shape costs a caller, and a doctest in place of a mock.
8. A green gate is a **precondition** for looking at items 1 and 2, never a
   substitute for them (`_intake-brief.md` Proof artefact).

## Dependencies

From the decomposition DAG (`../_decomposition.md` Sequencing).

**Depends on:** nothing. This is one of the initiative's two roots (rank 0) and
can start on day one. It assumes no substrate owned outside this initiative: the
registry, the `Fixture`/`Capability`/`RuleOutcome` machinery and the mutant
registry it extends are all already in the tree
(`crates/happenstance-testkit/src/`), and `RUNBOOK.md` phase 4 — the contract
freeze this phase builds on — is `done` (`RUNBOOK.md:154`).

**Unlocks:**

- `typed-layer-and-alpha-release` (HS-P0011) — consumes the frozen port and the
  write seam; the runbook's 6 → 7 edge.
- `postgres-and-neon-stores` (HS-P0014) — the runbook's 6 → 10 edge.
- `ladybug-projection-store` (HS-P0015) — the runbook's 6 → 11 edge, and the
  project that writes the freeze verdict.
- `sqlite-durable-store` (HS-P0012) — lists this project in its `blocked_by`
  alongside `typed-layer-and-alpha-release`. Recorded here because the
  decomposition's `blocks` column for this project omits it while sqlite's
  `blocked_by` names it; the edge is real either way and merge order (position 3)
  respects it.

## Risks and coupling notes

- **The DoD 7 watched edge.** Two unlike batch shapes are owed here, and the only
  SQL projection store in the tree ships from `sqlite-durable-store`, two
  positions later in merge order. Either the second shape is built inside the
  testkit — the `MemoryFixture` precedent — or this is a latent 6 → 8 inversion.
  `RUNBOOK.md:3935-3940` names the phase-2 rusqlite skeleton "fleshed out far
  enough to commit a real transaction" as the intended second shape, which is an
  instrument rather than the shipped adapter. **The architecture brief must
  settle this** (`../_decomposition.md`, Carried into the briefs).
- **PS-1 is `[FROZEN]` and admits an implementation its own rule table rejects.**
  A commit returning `Ok` that makes neither write durable satisfies the "or not
  at all" arm and fails `commit_advances_the_checkpoint`
  (`.kb/open-questions/ps-1-states-no-progress-obligation.md`). Adding the
  progress obligation changes the set of admitted implementations, so it is a new
  decision atom under `.kb/playbooks/` repair-frozen-clause discipline — **never
  a line edit**. PS-19 has the same shape, and that atom's sub-question 3 asks
  whether the defect is systematic across the other PS clauses; the ADR's scope
  should follow whichever is true.
- **Freezing against two shapes we both wrote.** `CLAUDE.md`'s spread rule says a
  port is only as well-designed as the spread of what implements it. The far end
  on this port's axis — batch shape and transactional seam — is Ladybug's graph
  batch and Neon's no-connection/no-cursor transport
  (`references/adapter-shapes.md:41-48`), and both are downstream. The freeze is
  therefore *provisional in fact* until HS-P0015 writes its verdict, which is
  exactly why the verdict is a separate project.
- **Removing the GAT touches six sites.** `grep`-visible `ProjectionStore for`
  impls are all `todo!()`-bodied skeletons (`_intake-brief.md` Clarifications).
  AC-013's bar is deliberately narrower than "compiles unchanged", which this
  phase's own decision makes unsatisfiable (`RUNBOOK.md:3942-3948`).
- **Do not reintroduce a borrowing GAT anywhere in the fixture.**
  `type Store<'a> where Self: 'a` on a foreign trait is one of five ingredients
  of a rustc ICE this repository already minimised, still reproducing on 1.97.1
  (`crates/happenstance-testkit/src/contract.rs:97-111`,
  `experiments/rustc-ice-gat-foreign-trait/`).
- **A `memory`-gated module is a rustdoc hazard.** An intra-doc link into a
  module absent on some documented configuration is a hard rustdoc error this
  workspace has already paid for once
  (`crates/happenstance-testkit/src/lib.rs:112-132`). The same discipline applies
  to `MemoryProjectionStore` behind its feature.
- **CF-40's ownership is claimed and disclaimed by one decision**
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md`). DT-3's answer here
  should not silently mint a second, divergent declension policy for the
  projection fixture — one policy, or a stated reason for two.
- **A boundary-scoped projection checkpoint would reopen ADR-0013's globally
  frozen visibility invariant**
  (`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`). If
  the design heads that way, stop and file the decision rather than absorbing it.
- **Nothing owns the post-phase reconciliation**, and this project's exit is what
  forces it (`.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`).
  Flagged, not settled here; AC-014 covers only this project's own clauses.
- **`ProjectionId` becomes unvalidated on a *frozen* port** the moment this lands
  (`.kb/open-questions/projection-id-is-unvalidated.md`). It is cheap to decide
  now and expensive afterwards.

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — BR/AC/DoD/DT spine
- [`../_decomposition.md`](../_decomposition.md) — the DAG, scope seams, traceability
- [`_intake-brief.md`](_intake-brief.md) — the approved intent and clause ledger

Knowledge base:

- `.kb/decisions/0010-the-suite-must-prove-itself.md` — the suite's own proof obligation
- `.kb/decisions/0007-projection-runner-decodes.md` — where the runner splits
- `.kb/decisions/0008-one-derivation-for-both-ports.md` — one derivation for `ProjectionStore` too (PS-35)
- `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`
- `.kb/open-questions/ps-1-states-no-progress-obligation.md`
- `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`
- `.kb/open-questions/projection-id-is-unvalidated.md`
- `.kb/open-questions/cf-40-fixture-limits-ownership.md`
- `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`
- `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`
- `.kb/maps/open-questions-index.md`

Specification, plan and code:

- `RUNBOOK.md:3848-3965` — phase 6 in full: work, proof artefact, exit criteria
- `RUNBOOK.md:588-610` — the provisional-clause groups and PS-2's thirteen rows
- `RUNBOOK.md:296-298` — ADR-0017 / 0018 / 0019 in the queue
- `spec/SPECIFICATION.md` — §4.11's PS rule table; the specification wins on conflict
- `spec/E2E-CASES.md` — E2E-15 – E2E-24, E2E-26 – E2E-29, E2E-50 become writable here
- `crates/happenstance-core/src/projection.rs` — the port, its invariant, its provisional marker
- `crates/happenstance-testkit/src/lib.rs` — the suite's shape and its one-enumeration rule
- `crates/happenstance-testkit/src/registry.rs` — `for_each_event_store_rule!` and the emitters
- `crates/happenstance-testkit/src/contract.rs` — `Fixture`, `Capability`, `RuleOutcome`
- `crates/happenstance-testkit/tests/mutation_coverage.rs` — the mutant registry to extend
- `crates/happenstance-testkit/src/fixtures.rs` — `MemoryFixture`, the reference implementation
- `references/adapter-shapes.md` — the six skeletons' transcripts the ADRs must quote
- `CLAUDE.md` — the rule that matters, and both conformance corollaries
- `.redkiln/templates/_design.md` — the public-API-surface review this project owes

## Companions

- [`_decomposition.md`](_decomposition.md) — this project's warranted briefs (architecture, ux, testing)
- [`_storymap.md`](_storymap.md) — the vertical-slice story map covering AC-001 – AC-016
- [`_intake-brief.md`](_intake-brief.md) — the approved project intake
- [`../initiative.md`](../initiative.md) — the parent initiative charter
