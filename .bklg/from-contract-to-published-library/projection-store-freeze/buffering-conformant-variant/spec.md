---
item: HS-S0013
stage: spec
created: 2026-08-12T13:46:07.787Z
updated: 2026-08-12T13:46:07.787Z
template_sig: 87bbf1d0
rendered_sig: d3d25503
---

# Spec — A second, structurally unlike batch shape passes the whole suite

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — AC-04/AC-05/AC-06, **DoD 7** |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG and the scope seams |
| Project | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` — AC-004, DoD 2, DR-05 |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/buffering-conformant-variant/spec.md` |
| Key brief — architecture | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` **AC-A03** and **Note 3** ("The DoD-7 / AC-004 second shape — settled here") |
| Key brief — testing | same file, Testing brief, the **AC-004** row (Integration + E2E, both fixtures inside one gate run) |
| Signed-off design | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — **no user-facing surface**, approved 2026-08-12; `surfaces: []` |
| Story map | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` — slice 6, `second-batch-shape-and-evidence` |
| Roadmap pointer | `RUNBOOK.md:3848-3965` (phase 6 in full); `RUNBOOK.md:3935-3940` names the shape this story deliberately does **not** take |
| Specification | `spec/SPECIFICATION.md:5686-5691` (§4.11 assigns CF-5's conformant variant here), `:4849-4867` (PS-4), `:4760-4775` (PS-2's bar, which this does **not** clear) |

## One-line PR slice

The CF-5 conformant variant — a buffering, replay-at-commit projection store
legally unlike apply-on-write `MemoryProjectionStore`, built in
`crates/happenstance-testkit/tests/` per AC-A03 — passes the whole projection
suite, so two structurally unlike batch shapes are green inside one
`cargo xtask ci` run with both fixtures named.

## Executive summary

Slices 1–5 built the port, the probe, the reference store, the suite, the mutant
registry and every commit / reset / read-through rule. All of it has been checked
against exactly **one** batch shape: `MemoryProjectionStore`, which applies each
write as it arrives. This PR lands the second shape and runs the existing suite
against it unchanged.

The delta is three things and no more:

1. **One new store** — `BufferingProjectionStore`: `begin` allocates a write set
   and holds nothing; every probe write appends an op; `commit` is the only
   moment anything reaches the read model, and it replays the ops and writes the
   checkpoint in the same step. This is the adapter shape PS-4 explicitly permits
   (`spec/SPECIFICATION.md:4849-4856`) and the far end of §6's batch-shape axis
   (`:5686-5691`).
2. **One registry row** — registered as `Kind::ConformantVariant` with an **empty**
   `fails` slice and non-empty provenance, which is what turns the projection
   family's `conformant_variants_pass_everything` from a control asserting over
   nothing into a control (`crates/happenstance-testkit/tests/mutation_coverage.rs:3071-3120`,
   `:2856-2858`).
3. **One harness target** — a `projection_store_conformance!` invocation against
   the buffering fixture beside the existing `memory_conformance.rs`, so "passes
   the whole suite" is a set of named tests in the ordinary `tests` step rather
   than a claim in prose.

No rule is added, changed, weakened or gated. No port type changes. Nothing in
`crates/happenstance-core/` is touched. If the suite has to move for the second
shape to pass, that movement is the finding (CF-6) and it is reported, not
absorbed — which is exactly what the next story, `ps3-batch-shape-finding`, is
for.

## Context pack

Everything below is a decision already taken, restated here because an
implementer must honour it without opening anything. Deeper material sits behind
the anchors.

**The second shape is testkit-internal, and reaching for `happenstance-sqlite`
is the failure mode this story exists to prevent.** The architecture brief
settled it (AC-A03, Note 3): §4.11 already assigns CF-5's conformant variant to
`crates/happenstance-testkit/tests/` and names the buffering adapter as the
obvious one; the precedent (`GappedPositionStore`, `PagedStreamStore`) is already
in the tree; and the alternative inverts the DAG, because `sqlite-durable-store`
(HS-P0012) is downstream of this project and building a rusqlite instrument here
buys a `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]` entry and a
new `cargo deny` surface for a shape the buffering variant already provides. **No
edge from this project to `sqlite-durable-store` is added, and none is needed.**

**Structurally unlike means unlike on the batch-shape axis, and unlike on that
axis only.** `MemoryProjectionStore` applies on write; this store buffers and
replays at commit, holding no connection, no transaction and no lock between
`begin` and `commit` — the Workers `SqlStorage` / Neon-over-one-shot-HTTP shape
(`references/adapter-shapes.md`). One axis is deliberate: `variants.rs`'s own
doctrine is that each variant differs on *one* named axis
(`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:1-26`), because
a store that differs on two makes a rule failure ambiguous — you cannot tell which
difference convicted the rule. Concretely, this store **reads through its buffer**
(`READS_THROUGH_BATCH = true`): a pending op overlays committed state on
`probe_read_through`. That is not a softening of the shape — a buffered write set
is trivially readable — and it keeps the CF-5 positive control *total*. The
`READS_THROUGH_BATCH = false` instance is a different instrument and belongs to
`read-through-and-rebuild-rules`, which already owns it; minting a second one here
would put a skip-shaped hole in the only variant this family has.

**This story's job is to be *unable* to touch the suite.** CF-5's whole argument
is over-specification: a rule that asserts more than the specification requires,
passes against the reference store, and rejects a perfectly legal adapter in the
field (`variants.rs:1-15`). CF-6 then says the *rule* is wrong, not the store —
and `conformant_variants_pass_everything` says so in its own failure message
(`mutation_coverage.rs:3094-3108`). So a failing rule here is a finding to report
at the story boundary. What is forbidden is the shortcut: making the variant more
like `MemoryProjectionStore`, gating the rule behind a new capability, or
loosening the assertion, any of which converts the one instrument that can detect
over-specification into a second copy of the reference store.

**Two fixtures, one gate run — and that is structural, not procedural.** AC-004's
E2E half asks that both shapes be green inside the same `cargo xtask ci`
invocation, so a reviewer reads one run rather than reconciling two `cargo test`
invocations by hand. That is satisfied by the new harness being an ordinary
`tests/` target of `happenstance-testkit`: no `#[ignore]`, no opt-in feature, no
separate command. It is *not* satisfied by a README instruction.

**PS-2's bar is not cleared here, and must not be reported as cleared.** PS-2 is
`[FROZEN]` and asks for two *adapters* at opposite ends of the axis, and its
**Rejects** clause names the monoculture verbatim
(`spec/SPECIFICATION.md:4760-4775`). Two testkit instruments do not clear it.
AC-004/DoD 2 is a different, satisfiable bar — two structurally unlike batch
*shapes* pass the suite — and this story satisfies exactly that one. The project
already takes AC-014's second arm (`unstable-projection` with a stated reason,
AC-A04) precisely because of this gap; that disposition belongs to
`unstable-projection-gate-and-clause-disposition`, and no verdict on the 0.1
exposure is made here.

**The evidence is handed on, not judged.** Whether the two shapes disagreed
anywhere — including "they agreed everywhere", which is itself the finding and
not a silent success (`_decomposition.md` Note 10, item 2) — is written up by
`ps3-batch-shape-finding` (AC-015) from what this story's run showed. This story
owes that story an observation, in the story's own notes: which rules, if any,
needed different handling for the two shapes, and where.

**The persona slice.** The adapter author's fear, stated in
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-170`,
is discovering late that the port quietly assumed something their storage cannot
provide — because every implementation that has passed a suite here so far
"serialises its writers … four adapters, one storage shape wearing four hats".
This PR is the first moment a projection rule meets a store that holds nothing
open. What the adapter author gets is not a new API; it is the right to read a
green projection suite as evidence about the *port* rather than about
`MemoryProjectionStore`.

**Constraints that bite even though this is test code.** Both flavours: the store
implements the bare `ProjectionStore` (ADR-0001, `.kb/decisions/0001-async-port-flavours.md`)
and no `#[async_trait]` appears. No borrowing GAT anywhere on the fixture — that
is one of five ingredients of a rustc ICE this repository already minimised and
which still reproduces on 1.97.1 (`crates/happenstance-testkit/src/contract.rs:97-111`);
copy `MemoryFixture`'s owned-handle pattern (`crates/happenstance-testkit/src/fixtures.rs:243-292`).
Never assert a literal position value (`CLAUDE.md`, "The rule that matters") —
which applies to this store's own internals too: its checkpoint bookkeeping must
accept the positions the suite hands it rather than assuming density.

## Integration contract

- **Archetype**: `capability`. What the adapter author observes is a suite whose
  green is evidence about the port; the deliverable is user-observable as test
  names in the gate output.
- **Slice / milestone**: `second-batch-shape-and-evidence`. Slice-mate:
  `ps3-batch-shape-finding` (implemented in the same context, mounted as one
  surface; it consumes this story's run and writes the PS-3 finding).
- **Mount point**: `crates/happenstance-testkit/tests/` — specifically (a) the
  projection mutant registry's `REGISTRY: &[Declared]` and its
  `for_each_…_mutant!` enumeration, in whichever file `projection-mutant-registry`
  landed them (one binary at
  `crates/happenstance-testkit/tests/mutation_coverage.rs`, or the sibling binary
  Note 9 leaves open — this story does **not** re-decide it), and (b) a new
  `projection_store_conformance!` harness target beside
  `crates/happenstance-testkit/tests/memory_conformance.rs`. An item at neither is
  an instrument nothing runs; a store registered but never driven through the
  macro leaves AC-004's Integration half unproven, and a store driven but never
  registered leaves the projection `conformant_variants_pass_everything` asserting
  over nothing (`mutation_coverage.rs:2856-2858`).
- **Wires into**:
  - `ProjectionStore` and its associated types in
    `crates/happenstance-core/src/projection.rs` (as reshaped by
    `owned-batch-port-shape`) — consumed, never modified.
  - `ProjectionProbe` behind `happenstance-core`'s `conformance` feature
    (`projection-probe-conformance-feature`) — the only write path this store
    exposes to a rule.
  - `Capability` and `RuleOutcome`, reused unchanged
    (`crates/happenstance-testkit/src/contract.rs:355-537`), and the projection
    fixture trait from `projection-suite-entry-point`.
  - `projection_store_conformance!` and the three emitters
    (`crates/happenstance-testkit/src/registry.rs:222-295`) — invoked, not changed.
  - The `Declared` / `Kind` / `FailureMode` registry shape
    (`crates/happenstance-testkit/tests/mutation_coverage.rs:73-186`) and the
    `Subject: Fixture + Sized` harness convention
    (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:60-63`).
  - Precedent to copy rather than invent:
    `crates/happenstance-testkit/tests/mutation_coverage/variants.rs` — a
    conformant variant's module doc, its one-axis argument and its fixture pair.
- **Renders surfaces**: **none.** `_design.md` records `surfaces: []` and no
  `## Items` — this project ships nothing a person looks at, and this story adds
  no public API item to `happenstance-core` or `happenstance-testkit`'s `src/`.
  What reaches a human is the suite's stdout, whose one text surface
  (`RuleOutcome::skip_line`, `crates/happenstance-testkit/src/contract.rs:500-507`)
  this story consumes unchanged.
- **Conformance rule(s)**: **adds none**, and that is the point. It is observed
  by the projection family's existing meta-tests —
  `conformant_variants_pass_everything` (CF-5/CF-6),
  `mutant_registry_is_exhaustive` (CF-2), `every_mutant_states_its_provenance`
  (CF-4) — plus every projection rule, each of which now runs against a second
  store. A rule this story fails is a rule finding (CF-6), reported at the
  boundary.
- **Clause(s)**: discharges §4.11's CF-5 obligation for the projection family
  (`spec/SPECIFICATION.md:5686-5691`) and supplies PS-4's evidence — "the rule is
  shape-blind by construction, which is the point" (`:4849-4867`). It **amends
  nothing**, and explicitly does not discharge PS-2 (`[FROZEN]`, `:4760-4775`) or
  PS-3 (`:4776-4790`). Maturity markers and the `unstable-projection` disposition
  belong to `unstable-projection-gate-and-clause-disposition`; no `[FROZEN]`
  clause is edited here.
- **Advances DoD scenario**: initiative **DoD 7** — "The projection suite
  discriminates" — second half ("two structurally unlike batch shapes pass it").
  The first half (`CheckpointOnlyStore` fails by name) was landed by
  `projection-mutant-registry` and `commit-rollback-and-drop-rules`. Project DoD 2
  goes green at the end of this slice.

## PR boundary

**In this PR**

- The buffering store and its fixture, defined **once** under
  `crates/happenstance-testkit/tests/`, beside the projection mutants, with a
  module doc that states the axis it differs on and why that difference is legal.
- Its `Declared` row in the projection registry: `Kind::ConformantVariant`,
  `fails: &[]`, non-empty `provenance` naming the real adapter shape (no
  connection held across an await), `expect: &[]`.
- Its entry in the projection `for_each_…_mutant!` enumeration, so the mutant
  harness drives it and the exactness meta-tests see it.
- A new `projection_store_conformance!` harness target driving the buffering
  fixture, with the same target discipline as its sibling
  (`crates/happenstance-testkit/tests/memory_conformance.rs:23`).
- The story's own backlog folder: ledger, and the two-shape observation
  `ps3-batch-shape-finding` consumes.
- Mounting is in scope: the registry file, the enumeration and the harness list
  are the composition root here, and touching them is not scope drift.

**Explicitly not in this PR**

- Any new conformance rule, or any edit to an existing one. Adding a rule would
  also incur CF-29's mutant-plus-changelog obligation
  (`spec/SPECIFICATION.md:8141-8143`), which is a signal this story has drifted.
- Anything under `crates/happenstance-core/src/` — no port change, no probe
  change, no feature added. If the documented pair
  (`projection_store_conformance!` + `ProjectionProbe`) is insufficient to build
  this store, that is `documented-extension-surface`'s finding and a boundary
  report, not a quiet contract edit.
- Any `happenstance-sqlite` / `happenstance-postgres` / `happenstance-ladybug`
  change, any new dev-dependency (`rusqlite` above all), any `Cargo.toml` feature.
- The PS-3 finding itself (`ps3-batch-shape-finding`), the maturity markers and
  the `unstable-projection` gate
  (`unstable-projection-gate-and-clause-disposition`), the whole-gate proof
  artefact (`whole-gate-run-and-proof-artefact`), and any verdict on the 0.1
  exposure (`publication-and-positioning`, HS-P0016).
- `spec/SPECIFICATION.md`. This story is evidence *for* clauses, not an edit to
  them.

**Merge DoD (one line)**: the projection suite is green against the buffering
fixture and against `MemoryProjectionStore` in one `cargo xtask ci` run, both
fixtures named, with the projection `conformant_variants_pass_everything` control
non-vacuous and no rule touched.

```
crates/happenstance-testkit/tests/**
.bklg/from-contract-to-published-library/projection-store-freeze/buffering-conformant-variant/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| Buffering batch: nothing reaches the read model before `commit` | `begin` returns an owned write set (`type Batch;`, PS-5) and acquires **no** handle, transaction or lock. Probe writes append ops. The store's committed state is observably unchanged until `commit` returns `Ok`. | `spec/SPECIFICATION.md:4849-4856` (PS-4: PS-1 "is satisfiable by opening the transaction inside `commit` around a buffered write set"); `crates/happenstance-core/src/projection.rs` |
| `commit` replays the write set and the checkpoint as one unit | The ops are applied and the checkpoint advanced together, or neither — PS-1's coupling, reached by a different mechanism than `MemoryProjectionStore`'s. A partial replay that leaves rows applied and no checkpoint is what `commit_is_atomic_with_the_read_model` exists to reject; this store must not be the thing it rejects. | `_decomposition.md` Architecture brief Note 6 (the shape every projection rule shares); `spec/SPECIFICATION.md:5662` |
| `rollback` and bare `drop` cost nothing and leave the store usable | Dropping the write set is the rollback: no connection to return, so PS-7's "and the store remains usable" holds by construction. The story's value is that it holds *for a different reason* than the reference store's — a pooled-connection store answering `Busy` forever is the mutant this pair keeps honest. | `spec/SPECIFICATION.md:4898-4910`; the rule and its mutant land in `commit-rollback-and-drop-rules` (dependency) |
| `reset` under buffering | `reset(batch, id)` is expressed as an op in the same write set, applied at `commit` — one unit of work, scoped to one `(store, ProjectionId)`, and refusable. It is not a truncate and it is not `commit(empty, id, FIRST)`. | `reset-rules` (dependency); `spec/SPECIFICATION.md:5200-5217` |
| Reads through the pending write set | `READS_THROUGH_BATCH = true`; `probe_read_through` overlays pending ops on committed state, so `batch_reads_reflect_pending_writes` (PS-12) **executes** rather than skipping. Deliberate: with one variant in the family, a skip is a hole in CF-5's positive control, which asserts every rule ran against a variant. | `crates/happenstance-testkit/tests/mutation_coverage.rs:3110-3119`; `spec/SPECIFICATION.md:5086-5098`; `read-through-and-rebuild-rules` owns the `false` instrument |
| Per-instance batch identity survives buffering | `begin` stamps the batch with the store instance's identity and `commit` compares it, so `commit_rejects_a_foreign_batch` is answered by an integer comparison rather than a lifetime. With an owned batch the stamp is a field. | `_decomposition.md` Architecture brief Note 7; `spec/SPECIFICATION.md:5099-5125`; `commit-rollback-and-drop-rules` (dependency) |
| Passes the whole suite, as named tests | One `projection_store_conformance!` invocation against the buffering fixture yields one test per projection rule; zero failures; any skip carries the fixture's stated reason and is distinguishable from a pass. | `crates/happenstance-testkit/tests/memory_conformance.rs:27` (the invocation shape); `crates/happenstance-testkit/src/contract.rs:458-537` |
| Registered as data, not asserted in prose | One `Declared` row: `kind: Kind::ConformantVariant`, `fails: &[]` (empty **iff** conformant variant), `provenance` naming the adapter shape that makes it plausible, `expect: &[]`. Hand-written; never generated from observed outcomes. | `crates/happenstance-testkit/tests/mutation_coverage.rs:125-186`, `:325-351` (`GappedPositionStore` / `PagedStreamStore` rows) |
| CF-5's positive control becomes non-vacuous | With this row present, the projection sibling of `conformant_variants_pass_everything` asserts two things that previously asserted nothing: no variant was rejected by any rule, **and** every projection rule executed against a variant. | `crates/happenstance-testkit/tests/mutation_coverage.rs:3071-3120`, `:2856-2858` |
| A rule that rejects it is a finding about the rule | CF-6: the response is to report at the story boundary and record the disagreement for `ps3-batch-shape-finding` — never to loosen the assertion, gate the rule behind a new capability, or make the variant resemble `MemoryProjectionStore`. | `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:1-15`; `mutation_coverage.rs:3094-3108`; `_decomposition.md` Note 10 item 2 |
| One definition, two consumers | The store and fixture are defined in one source file and reached by both the mutant binary and the conformance harness via the existing `#[path = "…"] mod` include convention. A second copy is two stores that drift, and the registry would then describe one of them. | `crates/happenstance-testkit/tests/mutation_coverage.rs:64-65` |
| Both shapes inside one gate run | The harness is an ordinary `tests/` target — no `#[ignore]`, no opt-in feature, no separate command — so `cargo xtask ci`'s `tests` step runs both fixtures and the proof artefact is one run naming both. | `CLAUDE.md` "Commands"; `_decomposition.md` Testing brief, AC-004 row |
| The wasm32 `--tests` check stays green | `cargo xtask ci` type-checks every testkit test target for `wasm32-unknown-unknown`. The new harness declares its target discipline explicitly, mirroring its sibling's `#![cfg(not(target_arch = "wasm32"))]`, and the store itself pulls in nothing host-only. | `xtask/src/main.rs:231-243`; `crates/happenstance-testkit/tests/memory_conformance.rs:23` |
| No `#[async_trait]`, no borrowing GAT | Bare-flavour `ProjectionStore` impl; the fixture hands back an owned handle holding a refcount. Both are repository-level constraints that apply to test code exactly as they do to `src/`. | `.kb/decisions/0001-async-port-flavours.md`; `crates/happenstance-testkit/src/contract.rs:97-111`; `crates/happenstance-testkit/src/fixtures.rs:243-292` |
| No literal positions, anywhere | Including inside the store: checkpoint bookkeeping accepts whatever positions the suite assigns and never assumes density or a starting value. | `CLAUDE.md` "The rule that matters"; `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:49-60` |

## Data and migrations

**N/A — no persisted data and no migration.** This story adds a store whose
entire medium is process memory, in a test target that is rebuilt from nothing on
every run. There is no schema, no on-disk format, no wire format and nothing to
migrate; `happenstance-core` is not touched, so no published surface changes
either.

Two things look like data and are not, named so nobody treats them as such:

- **The projection `REGISTRY: &[Declared]`** gains one row. It is data-as-code
  and deliberately hand-written — generating it from observed outcomes turns the
  proof artefact into a snapshot test in which every future regression is
  "expected" (`crates/happenstance-testkit/tests/mutation_coverage.rs:125-141`).
  Its shape is fixed by `Declared` and is not migrated here.
- **The buffering batch's write set** is an in-memory op list whose
  representation is private to this store. It is deliberately *not* a public type
  and must not become one: a shared "write set" type would make the two batch
  shapes structurally alike again, which is the one property this story exists to
  avoid.

## Acceptance criteria

The persona is **P2, the adapter author**
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-170`).
Their journey today stops at step 3 — *"a green run today is evidence about one
storage shape wearing four hats, not evidence the port itself is sound"*
(`:157-165`) — and every criterion below is written as that step being crossed,
not as a capability being present. All seven together discharge project
**AC-004** (`.bklg/from-contract-to-published-library/projection-store-freeze/project.md:191-194`):
AC-002 and AC-006 are its **Integration** half, AC-005 its **E2E** half, and
AC-001 / AC-003 / AC-004 / AC-007 are what stop that half being satisfied
vacuously.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author whose storage holds no connection, transaction or lock between opening a write set and committing it (the Workers `SqlStorage` / Neon-over-one-shot-HTTP shape, `references/adapter-shapes.md`), **WHEN** a `BufferingProjectionStore` written in exactly that shape is driven through `begin` → probe writes → `commit`, **THEN** the store's committed read model and checkpoint are observably unchanged at every point before `commit` returns `Ok`, and change only at that moment — so the second shape is unlike `MemoryProjectionStore` on the batch-shape axis *by assertion*, not by claim in a doc comment. | A dedicated test in the store's own file under `crates/happenstance-testkit/tests/`: open a batch, write through `ProjectionProbe`, read committed state out of band and assert unchanged; then `commit` and assert it changed. Fails if the store degenerates into apply-on-write. Positions read back are whatever the suite assigned, never literals (`CLAUDE.md`, "The rule that matters"). |
| AC-002 | **GIVEN** an adapter author who wants "correct" to be executable rather than interpreted (`personas-and-journeys.md:125-127`), **WHEN** they read the gate output for the buffering fixture, **THEN** they see **one named test per projection rule**, zero failures, and any rule whose capability the fixture declines still present as a test reporting `RuleOutcome::Skipped` with the fixture's stated reason — never absent from the binary and never indistinguishable from a pass. | One `projection_store_conformance!` invocation against the buffering fixture in a new harness target beside `crates/happenstance-testkit/tests/memory_conformance.rs`, whose emitted test names are the projection rule names (`crates/happenstance-testkit/src/registry.rs:222-295`). Skip text is `RuleOutcome::skip_line` unchanged (`crates/happenstance-testkit/src/contract.rs:500-507`); skip-versus-pass is asserted on `RuleOutcome` values, not on stdout (project AC-005's convention, `contract.rs:458-537`). |
| AC-003 | **GIVEN** an adapter author who must trust that "this store is legal" is a claim someone committed to rather than a run that happened to be green, **WHEN** they open the projection mutant registry, **THEN** they find exactly one new hand-written `Declared` row — `kind: Kind::ConformantVariant`, `fails: &[]`, `expect: &[]`, and a non-empty `provenance` naming the real adapter shape that makes buffering plausible — and no row generated from observed outcomes. | The projection siblings of `mutant_registry_is_exhaustive` (CF-2) and `every_mutant_states_its_provenance` (CF-4), over the row's presence in both `REGISTRY: &[Declared]` and the `for_each_…_mutant!` enumeration (`crates/happenstance-testkit/tests/mutation_coverage.rs:73-186`; `:325-351` for the `GappedPositionStore` / `PagedStreamStore` precedent). Review check: the row is hand-written, per `mutation_coverage.rs:125-141`. |
| AC-004 | **GIVEN** an adapter author whose fear is a rule that asserts more than the specification requires and rejects their legal store in the field (`personas-and-journeys.md:150-165`), **WHEN** the projection family's CF-5 positive control runs, **THEN** it asserts two things it could not assert before this story — that **no variant was rejected by any rule**, and that **every projection rule executed against a variant** — so the control stops being a test over an empty set. | The projection sibling of `conformant_variants_pass_everything` (`crates/happenstance-testkit/tests/mutation_coverage.rs:3071-3120`; vacuity guard at `:2856-2858`; CF-6 failure message at `:3094-3108`). Non-vacuity is demonstrable by deleting the row locally and watching the control's own guard fail; that deletion is not committed. |
| AC-005 | **GIVEN** a reviewer, and the adapter author reading over their shoulder, who must not have to reconcile two `cargo test` invocations by hand, **WHEN** they read **one** `cargo xtask ci` run, **THEN** both batch shapes are green inside it with **both fixtures named** in the output — because the new harness is an ordinary `tests/` target with no `#[ignore]`, no opt-in feature and no separate command — and the run's mandatory `wasm32` `--tests` check stays green over the widened target set. | `cargo xtask ci` (`tests` step) as the single command; target discipline mirrors its sibling's `#![cfg(not(target_arch = "wasm32"))]` (`crates/happenstance-testkit/tests/memory_conformance.rs:23`), and the wasm32 half is the existing "wasm32 check of the conformance harnesses" step (`xtask/src/main.rs:231-243`), which needs no new gate step. A README instruction does not satisfy this row. |
| AC-006 | **GIVEN** an adapter author reading the registry to learn what the suite has actually been proven against, **WHEN** they follow the row to the store, **THEN** the store and its fixture are found **once**, in one source file, reached by both the mutant binary and the conformance harness — so the registry describes the same code the harness ran, and a second drifting copy cannot exist. | The existing `#[path = "…"] mod` include convention (`crates/happenstance-testkit/tests/mutation_coverage.rs:64-65`) plus the `Subject: Fixture + Sized` harness convention (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:60-63`). Review check on the diff: exactly one definition of the store type across `crates/happenstance-testkit/tests/**` — `cargo clippy -D warnings` catches a dead duplicate but not a used one, so this row is a named diff assertion rather than an inferred one. |
| AC-007 | **GIVEN** `ps3-batch-shape-finding` (this story's slice-mate), which must write PS-3's evidence from what *this* run showed rather than from memory, **WHEN** this story completes, **THEN** it hands over a written two-shape observation — which projection rules, if any, needed different handling for the two shapes and where, **including the "they agreed everywhere" outcome, which is the finding and not a silent success** — and the diff proves the CF-6 discipline was kept: zero rules added, zero rules edited, zero new capabilities minted to gate a rule, and nothing under `crates/happenstance-core/src/`. | A companion note in this story's own backlog folder (`.bklg/from-contract-to-published-library/projection-store-freeze/buffering-conformant-variant/`) is the artefact; the discipline half is a diff-scope assertion over the PR-boundary globs, cross-checked by CF-29's absence — no rule added means no mutant-plus-changelog obligation incurred (`spec/SPECIFICATION.md:8141-8143`), so an incurred one is the drift signal (`_decomposition.md` Architecture brief Note 10, item 2). |

## Interaction quality

This story **renders no visual surface**: the project's signed-off `_design.md`
records `surfaces: []` and `## Items` N/A, and its sign-off approved *the
no-surface determination itself*
(`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:48-50`,
`:92-101`). The composition family below is therefore **not** discarded as N/A —
it is taken in the medium this project actually has, which `_design.md` names
explicitly: **one text surface, the lines a conformance run emits**
(`_design.md:33-38`), reaching a human through stdout and CI logs. An unstyled
render satisfies every structural assertion; the analogue here is a run that
exits 0 while telling the adapter author nothing, and these invariants are what
make that fail.

Every invariant below is carried by an `AC-###` **row in the table above** — none
is introduced here, because `redkiln verify` extracts ACs from that table and a
bullet in this section would never be gated.

**STATE invariants** (in the medium: the gate run, and the tree it runs over)

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not context-jump** — the second shape appears inside the run the reviewer is already reading; it demands no second command, second terminal or reconciliation step. | **AC-005** | One `cargo xtask ci`; no `#[ignore]`, no opt-in feature, no separate invocation. |
| **Non-occlusion** — nothing is hidden by the new fixture's arrival. Every projection rule still emits its own named test for **both** fixtures, and a declined capability still occupies a line rather than vanishing. | **AC-002**, **AC-005** | One test per rule per fixture; `RuleOutcome::Skipped` asserted as a value (`crates/happenstance-testkit/src/contract.rs:458-537`). |
| **Preserved focus / selection** — the existing `MemoryProjectionStore` run is untouched: same test names, same emitters, same target discipline, so a reviewer's existing reading of the suite still resolves. | **AC-005**, **AC-007** | Diff-scope assertion: no edit to `memory_conformance.rs`'s invocation, no rule renamed. |
| **Reversibility** — a rule that rejects this store is *reported*, never absorbed. The store may be discussed; the rule may not be quietly loosened, gated behind a new capability, or made to resemble the reference store. | **AC-007** (with **EC-001**) | CF-6's own failure message (`mutation_coverage.rs:3094-3108`); the diff shows zero rule edits. |
| **Reachability without a special invocation** (the keyboard-reachability analogue) — every emitted test is individually addressable by its rule name from an ordinary `cargo test` filter, exactly as the event-store harnesses already are. | **AC-002** | Test names are rule names, per the three emitters (`crates/happenstance-testkit/src/registry.rs:222-295`). |

**COMPOSITION invariants** (from `_design.md`'s text surface, taken in its own medium)

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the deliverable reaches a human as *composed output*, not bare structure. A skip prints the composed `skip_line` — `` SKIP {rule}: fixture declines `{capability}` — {reason} `` (`_design.md:33-38`, `crates/happenstance-testkit/src/contract.rs:500-507`) — consumed unchanged, never re-spelled locally. | **AC-002** | Assertion on the `RuleOutcome` value plus the emitter's existing report path; a locally re-implemented skip string is a review reject. |
| **Composition / placement** — store and fixture sit beside the projection mutants they are registered with; the harness sits beside `memory_conformance.rs`. Neither lands in `src/`, neither lands in another crate. | **AC-006**, **AC-005** | The mount point in the Integration contract; the PR-boundary globs. |
| **Transience — persistent chrome, not opened on demand** — the second shape is present in every run, for everyone, forever. It is not revealed by a feature flag, an env var or a nightly job. | **AC-005** | No `#[ignore]`, no `required-features`, no new `Cargo.toml` feature (NF-002). |
| **Density budget, with the real numbers** — the delta this story is allowed: **+1** store definition, **+1** fixture, **+1** `Declared` row, **+1** enumeration entry, **+1** harness target, **+1** backlog note. And: **0** new conformance rules, **0** edits to existing rules, **0** new capabilities, **0** new dependencies or features, **0** files under `crates/happenstance-core/src/`, **0** files outside the two PR-boundary globs. Output density: **exactly one** test per projection rule per fixture — no rule emitted twice. | **AC-003**, **AC-006**, **AC-007** | Diff review against the PR boundary; the exactness meta-tests (CF-2, CF-4) catch a duplicated or missing enumeration entry. |
| **Hierarchy** — what a reader meets first is the *control* (the projection `conformant_variants_pass_everything`) making a claim over a non-empty set; the individual green rules are supporting detail. Before this story the hierarchy was inverted: many green rules beneath a control asserting over nothing. | **AC-004** | The control's own vacuity guard (`mutation_coverage.rs:2856-2858`). |
| **Named anti-pattern — the second copy.** Two definitions of the buffering store means the registry describes one and the harness runs the other. | **AC-006** | One definition, two consumers, via `#[path = "…"] mod` (`mutation_coverage.rs:64-65`). |
| **Named anti-pattern — the two-axis variant.** A variant differing on more than one named axis makes a rule failure unattributable — you cannot tell which difference convicted the rule (`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:1-26`). This store differs on the batch-shape axis only, and reads through its buffer (`READS_THROUGH_BATCH = true`) precisely so it does not also differ on read-through. | **AC-001** | The store's module doc names its one axis (NF-003); `READS_THROUGH_BATCH = true` is a compile-time constant a reviewer can read. |
| **Named anti-pattern — the vacuous green.** A variant that passes everything because it quietly became the reference store. | **AC-001** | AC-001's direct before/after-`commit` assertion, made independently of the suite; a green suite alone does not satisfy it. |

## Error conditions

| id | condition | required response |
| --- | --- | --- |
| **EC-001** | A projection conformance rule **rejects** the buffering store. | This is **CF-6**: the finding is about the *rule*, not the store. Stop, record which rule and what it assumed, and report at the story boundary for `ps3-batch-shape-finding` (AC-007). Forbidden responses, each of which converts the only over-specification instrument into a second copy of the reference store: loosening the assertion, gating the rule behind a newly minted capability, or reshaping the variant toward `MemoryProjectionStore` (`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:1-15`; `mutation_coverage.rs:3094-3108`). |
| **EC-002** | The documented pair — `projection_store_conformance!` + `ProjectionProbe` — proves **insufficient** to build this store without touching `crates/happenstance-core/src/`. | Boundary report, not a quiet contract edit. That insufficiency is `documented-extension-surface`'s finding (project AC-007, DT-8's outside-author arm) and is worth more reported than patched around. Nothing under `crates/happenstance-core/src/` is edited in this PR under any circumstance. |
| **EC-003** | A projection exactness meta-test fails on the new row — the variant declared with a non-empty `fails`, or `REGISTRY` and the enumeration disagreeing. | The registry is the claim and the run is the check: fix the row, or fix the store so the claim is true. Never edit the meta-test, and never regenerate the row from observed outcomes — that turns the proof artefact into a snapshot in which every future regression is "expected" (`mutation_coverage.rs:125-141`). |
| **EC-004** | The mandatory `wasm32` `--tests` check fails because the new harness or store pulls in something host-only. | Fix the store, not the step. The harness declares its target discipline explicitly (`crates/happenstance-testkit/tests/memory_conformance.rs:23`), and a buffering in-memory store has no legitimate host-only dependency — so this failure means a `std`-only or runtime-only reach crept in. `cfg`-ing the whole harness out of `wasm32` without a stated reason is not a fix. |
| **EC-005** | The store passes every rule, but AC-001's direct assertion shows committed state changing before `commit`. | The variant is not the second shape at all — it is `MemoryProjectionStore` in different clothing, and its green run is vacuous. AC-002 and AC-005 are **not** reportable while AC-001 fails; the ledger rows are independent, the story is not. |
| **EC-006** | `cargo hack`'s feature powerset, or `cargo deny`, changes verdict because of this PR. | Something outside the boundary moved: a new dev-dependency or a new feature. Revert it. This story's whole cost model is "no new edge in the graph" (project UX brief AC-U02, `_decomposition.md`), and a new `cargo deny` surface is exactly what AC-A03 rejected `happenstance-sqlite` to avoid. |
| **EC-007** | A dependency story's substrate is missing — the projection registry, the suite entry point, `reset`, or the read-through rule is not in the tree. | Halt loudly and report the missing dependency. Do **not** stub it and do not build a private substitute inside this story: the entire claim is that the *existing* suite, unchanged, is what the second shape passed. |

## Non-functional

| id | requirement | basis |
| --- | --- | --- |
| **NF-001** | The second fixture roughly doubles projection-suite execution and must stay **proportional** — no sleeps, no wall-clock waits, no timeout-based assertions. A buffering in-memory store has no I/O to wait on, so any added wall-clock time is a defect. | `cargo xtask ci` is run before every "done" (`CLAUDE.md`, "Commands"); a gate people avoid running is a gate that stops running. |
| **NF-002** | **Zero** new dependencies, dev-dependencies, `Cargo.toml` features or target-specific dependency tables. `rusqlite` above all. | Architecture brief **AC-A03** / Note 3: the buffering variant was chosen over a downstream SQLite instrument precisely to avoid a `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]` entry and a new `cargo deny` surface. |
| **NF-003** | The store's module doc states **the one axis it differs on, and why that difference is legal under the specification**, citing the PS clause that permits it — the same discipline `variants.rs` holds itself to, in the same voice. A variant whose legality is unargued is a variant a future reader will "fix". | `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:1-26`; `standards/rust/70-rustdoc-obligations.md`. |
| **NF-004** | Test code is held to the same bar as `src/`: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and the MSRV floor of **1.97.1**. No `#[async_trait]` anywhere; no borrowing GAT on the fixture. | `CLAUDE.md` "Binding constraints" 1 and the MSRV paragraph; `.kb/decisions/0001-async-port-flavours.md`; the ICE-minimisation note at `crates/happenstance-testkit/src/contract.rs:97-111`. |
| **NF-005** | Deterministic and single-threaded-friendly: no thread spawning, no scheduler-order dependence, no `std::time`. The same code type-checks under the `wasm32` emitter. | `xtask/src/main.rs:231-243`; project AC-016's harness belongs to `projection-suite-entry-point` and must not be made unbuildable here. |
| **NF-006** | No literal position values anywhere — including in the store's own checkpoint bookkeeping, which accepts whatever positions the suite assigns and assumes neither density nor a starting value. | `CLAUDE.md` "The rule that matters"; the precedent's deliberately hostile constants (`variants.rs:49-60`). |
| **NF-007** | The store's write-set representation stays **private**. It must not become a shared public "write set" type, which would make the two batch shapes structurally alike again — the one property this story exists to avoid. | The "Data and migrations" section above; project DR-05. |
| **NF-008** | The run's result is never summarised as a **pass rate** over the rule or mutant set. "N of M rules green" is the forbidden framing, in the observation note as much as in code. | `.kb/decisions/0010-the-suite-must-prove-itself.md` (Accepted); project AC-003's no-pass-rate clause. |

## Implementation notes (non-prescriptive)

Direction, not instruction. The implementer may reach any of this differently and
say why.

- **Copy the precedent rather than invent one.**
  `crates/happenstance-testkit/tests/mutation_coverage/variants.rs` already holds
  two conformant variants with the module doc, the one-axis argument (`:1-26`),
  the CF-5/CF-6 framing (`:1-15`) and the fixture pairing this story needs.
  `GappedPositionStore`'s hostile constants (`FIRST_GAPPED = 4096`, `GAP = 7`,
  `:49-60`) are worth studying for their *reasoning*: chosen so a rule assuming
  density fails loudly rather than by one. The buffering store's analogue is that
  **nothing whatsoever** is observable before `commit` — the most hostile legal
  reading of PS-4.
- **The shape is small.** `begin` returns an owned write set — an op list plus the
  store's identity stamp — and takes no lock. Probe writes push. `commit` drains,
  applies, advances the checkpoint and returns. `rollback` and `Drop` do nothing
  at all, which is exactly why PS-7 holds here for a different reason than it does
  in the reference store, and that difference is the story's value.
- **Read-through is an overlay, not a second store.** `probe_read_through` walks
  the pending ops over committed state. Keeping `READS_THROUGH_BATCH = true` keeps
  CF-5's control total; the `false` instrument already belongs to
  `read-through-and-rebuild-rules`, and duplicating it here would put a
  skip-shaped hole in the only variant the projection family has.
- **The identity stamp is a field, not a lifetime.** With an owned `Batch` (no
  lifetime parameter, project AC-009), `commit_rejects_a_foreign_batch` is
  answered by comparing a value the store handed out at `begin`. Resist reaching
  for a lifetime to express it — that is precisely the shape
  `owned-batch-port-shape` removed.
- **Owned handles on the fixture.** Copy `MemoryFixture`'s pattern
  (`crates/happenstance-testkit/src/fixtures.rs:243-292`): one fixture instance is
  one backing store, each `connect()` a refcount clone. A borrowing GAT here is
  one of five ingredients of a rustc ICE this repository already minimised and
  which still reproduces on 1.97.1 (`crates/happenstance-testkit/src/contract.rs:97-111`).
- **Mount in both places or it is invisible.** Registry row **and** enumeration
  entry **and** harness invocation. Registered-but-not-driven leaves AC-004's
  Integration half unproven; driven-but-not-registered leaves the control
  asserting over nothing. Both mount sites are named in the Integration contract.
- **Write the observation while the run is in front of you.** AC-007's note is
  cheapest during the first green run and nearly impossible to reconstruct
  afterwards. "They agreed everywhere" is a complete and acceptable answer —
  written down, with which rules were exercised, not left as silence.
- **If a rule fails, read it before touching anything.** Nine times in ten the
  interesting question is *what did this rule assume about who holds a lock*. That
  sentence is the PS-3 finding.

## Tests and CI (merge gate)

Tier vocabulary is the project testing brief's
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md`,
Testing brief preamble): **Static** reads source or config without executing the
code under test; **Unit** is an in-process `#[test]`, including meta-tests over
the registries; **Integration** is a rule actually driving a `ProjectionStore`
through `begin` → write → `commit` → read-back; **E2E** is `cargo xtask ci` run
whole, or a claim provable only by inspecting that run's combined output.

| tier | command / path | proves |
| --- | --- | --- |
| **Integration** | `cargo test -p happenstance-testkit --test <new projection conformance harness>` → `crates/happenstance-testkit/tests/` (beside `memory_conformance.rs`) | **AC-002.** Every projection rule drives the buffering store to completion with zero failures; skips carry the fixture's reason and are distinguishable from passes. Project AC-004's Integration half, second shape. |
| **Integration** | `cargo test -p happenstance-testkit --test memory_conformance` (unchanged) | The **first** shape is still green and untouched — "two shapes" needs both, and this run must not be edited by this PR (**AC-005**, **AC-007**). |
| **Unit** | `crates/happenstance-testkit/tests/` — the store's own before/after-`commit` visibility test | **AC-001.** Committed state unchanged before `commit`, changed after. The one assertion a green suite cannot make on its own; without it **EC-005** is undetectable. |
| **Unit** | `cargo test -p happenstance-testkit --test mutation_coverage` → the projection siblings of `mutant_registry_is_exhaustive` (CF-2, `mutation_coverage.rs:2751-2860`) and `every_mutant_states_its_provenance` (CF-4) | **AC-003.** The row exists in both `REGISTRY` and the enumeration, is `Kind::ConformantVariant` with `fails: &[]`, and states non-empty provenance. |
| **Unit** | same target → the projection sibling of `conformant_variants_pass_everything` (`mutation_coverage.rs:3071-3120`; guard `:2856-2858`) | **AC-004.** CF-5's positive control asserts over a non-empty set: no variant rejected by any rule, and every projection rule executed against a variant. |
| **Static** | `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo fmt --check` | **NF-004.** Test targets held to the `src/` bar; also catches a *dead* second copy of the store (**AC-006**'s cheap half). |
| **Static** | `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` — the existing step at `xtask/src/main.rs:231-243` | **AC-005**'s wasm32 half, **NF-005**, **EC-004.** The new harness type-checks for the constrained target inside the same run, at the cost of no new gate step. |
| **Static** | `cargo hack check --workspace --feature-powerset --no-dev-deps`; `cargo deny` (both resolve on this machine, per `CLAUDE.md` "Commands") | **NF-002**, **EC-006.** No new feature, no new dependency edge, no changed licence/advisory surface. |
| **Static** | reviewed diff against the PR-boundary globs | **AC-006**, **AC-007**, and the density budget: one store definition; zero rules added or edited; nothing under `crates/happenstance-core/src/`. Not compiler-checkable — a compiler cannot distinguish "the minimal change" from "an unrelated change that also compiles", which is why the Testing brief already uses a reviewed diff for project AC-013. |
| **E2E** | `cargo xtask ci` — one invocation, clean tree | **AC-005.** Both fixtures green, both named, inside one run. Project AC-004's E2E half, and project DoD 2's observable moment. |
| **E2E (process)** | the story's companion note under `.bklg/from-contract-to-published-library/projection-store-freeze/buffering-conformant-variant/` | **AC-007.** The two-shape observation exists and is specific, including the "agreed everywhere" outcome. Consumed by `ps3-batch-shape-finding` (project AC-015), which the Testing brief already types *E2E (process, derived from AC-004)*. |

Story grain during implementation is `cargo xtask affected --base main` plus
`cargo xtask ci --fast`; the merge gate for this story is the **full**
`cargo xtask ci`, because AC-005 is a claim about a whole run and `--fast` omits
the wasm32 conformance-harness check AC-005's second half needs
(`_storymap.md`, "Merge order", closing paragraph).

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| --- | --- | --- |
| **The suite moves to accommodate the store.** The highest-value failure mode: a rule is loosened, gated or reshaped so the variant passes, and the one instrument that could detect over-specification becomes a second copy of the reference store. | The pressure arrives at the worst moment — one red rule with everything else green, at the end of a slice. | **EC-001** names the forbidden responses; **AC-007**'s diff assertion makes "zero rules edited" a ledger row rather than an intention; CF-6 says it in the run output (`mutation_coverage.rs:3094-3108`). |
| **Vacuous conformance** — the store drifts toward apply-on-write during debugging and passes everything for the wrong reason. | The suite cannot see it: a store that applies early passes every rule that only observes after `commit`. | **AC-001** asserts the axis directly, before and after `commit`, independently of the suite. |
| **Two-axis drift** — an incidental second difference (read-through, error mapping, checkpoint density) creeps in and a later rule failure becomes unattributable. | `variants.rs:1-26` learned this the expensive way for the event-store family. | `READS_THROUGH_BATCH = true` is fixed by this spec; **NF-003** forces the module doc to name the single axis, which is where a second one becomes visible to a reviewer. |
| **Scope leak into `happenstance-core`.** A missing probe method or an awkward port shape tempts a one-line contract edit. | The port was frozen upstream by `owned-batch-port-shape` and `projection-probe-conformance-feature`; editing it here invalidates their reviewed diffs and silently widens a soon-to-be-published surface. | The PR boundary forbids `crates/happenstance-core/src/` outright; **EC-002** routes the need to `documented-extension-surface` as a finding, where it is worth more. |
| **Coupling to the three dependency stories.** This story runs *their* rules; if any of them lands late or partially, the "whole suite" claim is unprovable. | They *are* the suite — there is nothing to pass without them. | **EC-007**: halt and report, never stub. All three sit in slices 4–5 of this project, so the merge order already enforces it (`_storymap.md`). |
| **Coupling to the slice-mate.** `ps3-batch-shape-finding` is implemented in the same context and consumes this run. | If the observation is not written while the run is live, the finding gets reconstructed from memory — which is how "they agreed everywhere" becomes a silent success instead of the finding it is. | **AC-007** makes the note a deliverable of *this* story, not the next one. |
| **The registry's file location is still open.** Note 9 leaves the one-binary-versus-sibling-binary question unsettled. | Guessing wrong lands the row where nothing reads it. | This story does not re-decide it: the Integration contract names the *symbols* (`REGISTRY: &[Declared]`, the `for_each_…_mutant!` enumeration), not a file path, which makes the spec robust to either outcome. |
| **PS-2 mis-reporting** — reporting this story as having cleared PS-2's `[FROZEN]` bar. | PS-2 asks for two *adapters* at opposite ends of the axis and its **Rejects** clause names the monoculture verbatim (`spec/SPECIFICATION.md:4760-4775`). Two testkit instruments are not two adapters. | Stated in the Context pack and repeated here; the disposition belongs to `unstable-projection-gate-and-clause-disposition`, and no verdict on the 0.1 exposure is made in this PR. |

## Dependencies

**Blocks-on** — all three sit in earlier slices of this same project, and every
one must be in the tree before this story can make its claim (**EC-007**):

| story slug | what this story consumes from it |
| --- | --- |
| `commit-rollback-and-drop-rules` | The commit-side rules the buffering store must pass — `failed_commit_leaves_both_unchanged`, `rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable`, `commit_rejects_a_foreign_batch` (the per-instance stamp), and the position rules — plus the projection `REGISTRY` / enumeration shape this story adds one row to. |
| `reset-rules` | `reset`'s rules — one unit of work, scoped to one `(store, ProjectionId)`, refusable, and not `commit(empty, id, FIRST)` — which the buffering store answers by expressing `reset` as an op in the same write set. |
| `read-through-and-rebuild-rules` | `batch_reads_reflect_pending_writes` (PS-12) gated on `READS_THROUGH_BATCH`, and the rebuild rules. It also **owns** the `READS_THROUGH_BATCH = false` instrument, which is why this story sets `true` rather than minting a second one. |

Transitively, through those three: `projection-mutant-registry` (the `Declared` /
`Kind` shape and the exactness meta-tests), `projection-suite-entry-point`
(`projection_store_conformance!` and the projection fixture trait),
`projection-capability-skips` (the reported-skip machinery),
`owned-batch-port-shape` and `projection-probe-conformance-feature` (the port and
the probe), and `memory-projection-store` (the *first* batch shape, without which
"two shapes" is one).

**Unlocks:**

| story slug | what it takes from this one |
| --- | --- |
| `ps3-batch-shape-finding` | Slice-mate, same context. Takes AC-007's two-shape observation and writes the PS-3 finding (project AC-015). Project **DoD 2** is observable at the end of that pair. |
| `documented-extension-surface` | Declares `buffering-conformant-variant` in its own `depends_on` (`_storymap.md`). Takes this store as the worked example of what the documented pair can build — and takes EC-002's report, if one is made, as direct evidence about that extension surface. |
| `whole-gate-run-and-proof-artefact` | Takes "both passing batch shapes" as half of the proof artefact it records; the other half is the test name `CheckpointOnlyStore` fails. |
| `unstable-projection-gate-and-clause-disposition` | Downstream of `ps3-batch-shape-finding`, so downstream of this. Takes the evidence — **not** a PS-2 verdict, which this story explicitly does not make. |

No dependency on anything outside this project. In particular **no edge to
`sqlite-durable-store` (HS-P0012) is added**: that would invert the initiative
DAG, and AC-A03 settled the second shape as testkit-internal precisely to avoid
it.

## Anchors (progressive disclosure)

Deferred, not optional. The Context pack above is sufficient to *start*; each row
below is the artefact to open at the moment named, for the AC it serves. Link and
read — do not paste in bulk.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/tests/mutation_coverage/variants.rs` | The precedent to copy rather than invent: two existing conformant variants, each with the module doc, the one-axis argument (`:1-26`), the CF-5/CF-6 framing (`:1-15`) and the deliberately hostile constants with their reasoning (`:49-60`). Writing the buffering store without reading this produces a variant that is legal but unargued. | **First** — before writing a line of the store. Its shape, its doc and its fixture pairing all come from here. | AC-001, AC-003, AC-004 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The registry itself: `Declared` / `Kind` / `FailureMode` (`:73-186`), the hand-written-not-generated warning (`:125-141`), the two existing conformant-variant rows (`:325-351`), the `#[path = …] mod` include convention (`:64-65`), the vacuity guard (`:2856-2858`) and `conformant_variants_pass_everything` with its CF-6 message (`:3071-3120`, `:3094-3108`). | When writing the `Declared` row and the enumeration entry — and again the moment a meta-test fails (EC-003). | AC-003, AC-004, AC-006 |
| `crates/happenstance-testkit/tests/memory_conformance.rs` | The harness target to mirror exactly: target discipline at `:23`, the single conformance invocation at `:27`, and a module doc that states what the harness is *for*. The new target is this file with one fixture swapped. | When creating the new harness target. | AC-002, AC-005 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability` / `RuleOutcome` and the skip machinery (`:355-537`), the composed `skip_line` text a human actually reads (`:500-507`), and — critically — the borrowing-GAT rustc ICE note (`:97-111`) that constrains how the fixture may be typed. | Before designing the fixture type (the ICE note), and again when asserting skip-versus-pass on `RuleOutcome` values. | AC-002 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture`'s owned-handle pattern (`:243-292`) — one fixture instance is one backing store, each `connect()` a refcount clone. The buffering fixture is this, with a different store behind it. | While writing the fixture, immediately after reading the ICE note. | AC-001, AC-002 |
| `crates/happenstance-testkit/src/registry.rs` | The three emitters and how a rule name becomes a test name (`:222-295`) — which is what makes AC-002's "one named test per rule" and the reachability invariant structural rather than aspirational. Invoked, never changed. | When the emitted test names must be predicted or debugged. | AC-002, AC-005 |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | The `Subject: Fixture + Sized` convention (`:60-63`) the mutant harness drives every registered store through — the interface the new fixture must satisfy to be enumerable at all. | When wiring the enumeration entry, once the fixture compiles. | AC-003, AC-006 |
| `spec/SPECIFICATION.md` | The clause text this story is evidence for and must not contradict: PS-4 permitting exactly this shape — *"satisfiable by opening the transaction inside `commit` around a buffered write set"* (`:4849-4867`); §4.11 assigning CF-5's projection conformant variant here (`:5686-5691`); PS-12 read-through (`:5086-5098`); the foreign-batch clause (`:5099-5125`); PS-7 (`:4898-4910`); reset (`:5200-5217`); and PS-2's `[FROZEN]` bar with its **Rejects** clause, which this story does **not** clear (`:4760-4775`). | Open the specific range when a rule's intent is in question — above all at EC-001, where the question is what the specification actually requires versus what the rule asserts. | AC-001, AC-002, AC-004, AC-007 |
| `crates/happenstance-core/src/projection.rs` | The port as reshaped by `owned-batch-port-shape` — `type Batch;` with no lifetime, `begin` / `commit` / `rollback` / `reset`, the `Checkpoint` / `Authority` / `CommitError` / `ResetError` types, and `ProjectionProbe` behind the `conformance` feature. Consumed, never modified. | Immediately before writing the `impl` — the signatures are authoritative there, not in this spec. | AC-001, AC-002 |
| `references/adapter-shapes.md` | What the type checker actually told the six skeletons, including the no-connection-held shape (Workers `SqlStorage`, Neon over one-shot HTTP) this variant models. It is what makes the row's `provenance` a real claim rather than a plausible sentence. | When writing the `Declared` row's `provenance` and the module doc's legality argument. | AC-003 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | Architecture brief **AC-A03** (`:303`) and **Note 3** (`:406`) — why the second shape is testkit-internal rather than `happenstance-sqlite`, with the cost argument; **Note 10** item 2 — "they agreed everywhere" is the finding; and the Testing brief's **AC-004** row (`:774`) — the exact Integration + E2E split this spec's tests table implements. | When the temptation to reach downstream appears, and when writing AC-007's observation. | AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P2's journey (`:114-170`) — step 3, *"a green run today is evidence about one storage shape wearing four hats"*, is the sentence this story exists to make false. Carries its own qualification: none of the four personas was directly observed (`:361-363`). | If an AC's user intent needs re-grounding, and when framing the observation note. | AC-001, AC-002, AC-007 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | Accepted decision atom: the suite must prove itself, and **no pass rate is quoted** over the mutant set. Directly constrains how this story's result may be reported — "N of M rules green" is exactly the forbidden framing. | Before writing AC-007's observation note, or any summary of the run. | AC-004, AC-007 |
| `.kb/decisions/0001-async-port-flavours.md` | Accepted decision atom: ports are defined once with no `Send` bound and `trait_variant` derives the `Send` flavour; `#[async_trait]` is never introduced. Applies to this test-only store exactly as it does to `src/`. | Before writing the `impl` block. | AC-001 |
| `xtask/src/main.rs` | The gate, defined once — in particular the "wasm32 check of the conformance harnesses" step (`:231-243`), `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown`, which is what makes AC-005's wasm32 half free and EC-004 possible. | When the new harness target is created, and at EC-004. | AC-005 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off design (2026-08-12): `surfaces: []` and the explicit no-surface determination (`:48-50`, `:92-101`), plus the **one text surface** — `skip_line`'s composed output (`:33-38`) — that the composition invariants are taken in. Binding on what "presentation" means here. | Before implementing anything that prints, and when checking the composition invariants. | AC-002 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` | Slice 6's row and the merge order — the ordering guarantee that the three dependency stories are already in the tree, and the closing note on why this story's merge gate is the full `cargo xtask ci` rather than `--fast`. | At EC-007, and when choosing the merge-gate command. | AC-005, AC-007 |
| `standards/rust/70-rustdoc-obligations.md` | The house doc obligations the store's module doc must meet — what a doc comment owes a reader, in the voice `variants.rs` already uses. | While writing the module doc, once the store compiles. | AC-003 |

## Clarifications resolved during spec

1. **The seven AC ids the first pass fixed are exactly the seven enumerated
   here.** None added, none dropped. The behaviour table in *Behavior and
   interfaces* carries more rows than there are ACs because several rows are
   consequences of one criterion — the buffering `commit` semantics,
   `rollback`/`Drop` costing nothing, and `reset`-as-an-op are all facets of
   AC-001's single assertion that nothing is observable before `commit`. The
   ledger matches this enumeration exactly.
2. **Which AC carries project AC-004's two halves.** The Testing brief splits
   AC-004 into Integration (both fixtures drive the suite to completion) and E2E
   (both inside one `cargo xtask ci`). Those are **AC-002** and **AC-005**
   respectively. AC-001, AC-003, AC-004 and AC-006 exist because both halves are
   satisfiable vacuously — by a store that is not actually unlike, by a registry
   row that claims nothing, by a control asserting over an empty set, or by two
   copies of one store that have drifted apart.
3. **The Interaction-quality section is written, not waived.** `_design.md`
   records `surfaces: []`, so the *visual* composition family has nothing to
   index — but the same file names a **text surface** (`_design.md:33-38`), and
   the composition invariants are taken in that medium: presentation exists
   (`skip_line` consumed unchanged), placement, transience (persistent gate
   chrome, not feature-gated), a numeric density budget, hierarchy (the control
   above its supporting detail), and three named anti-patterns. Every one is
   carried by a row in the acceptance table, per the extraction rule; that section
   only says which.
4. **`READS_THROUGH_BATCH = true` is settled here, and is not a softening.** A
   buffered write set is trivially readable, so `true` is the honest value for
   this shape; `false` would both misdescribe the store and put a skip-shaped hole
   in the only conformant variant the projection family has, weakening CF-5's
   positive control (AC-004). The `false` instrument is owned by
   `read-through-and-rebuild-rules`.
5. **Where the registry row goes is deliberately left as "wherever
   `projection-mutant-registry` landed it".** Note 9 leaves the one-binary versus
   sibling-binary question open, and re-deciding it here would be scope drift into
   an upstream story. The Integration contract names the *symbols*
   (`REGISTRY: &[Declared]`, the `for_each_…_mutant!` enumeration) rather than a
   file path, which makes this spec robust to either outcome.
6. **A failing rule is not a story failure.** EC-001 makes CF-6 the required
   response: report, do not absorb. This is the one place where "the story is
   blocked" and "the story found what it was built to find" are the same state,
   and the spec says so rather than leaving an implementer to choose under
   end-of-slice pressure.
7. **NF-008 was added late, from ADR-0010.** The no-pass-rate constraint is a
   project AC-003 obligation on the *registry*, but this story's most likely
   breach is the observation note ("34 of 34 rules green"), which no earlier
   artefact guarded. It constrains reporting, not code, and adds no scope.
8. **PS-2 stays uncleared and unclaimed.** Repeated in three places on purpose —
   Context pack, Integration contract, Risks — because the failure mode is one of
   *reporting*, not of building: two testkit instruments passing is a true and
   valuable claim, and calling it PS-2 would make it a false one.
9. **No new companion artefacts beyond the observation note and the ledger.** The
   story's backlog folder gains AC-007's note and `_ledger.md`; the PR boundary
   forbids everything else outside `crates/happenstance-testkit/tests/**`.
