---
item: HS-S0010
stage: spec
created: 2026-08-12T13:46:04.770Z
updated: 2026-08-12T13:46:04.770Z
template_sig: 87bbf1d0
rendered_sig: bcea8d19
---

# Spec — Commit, rollback and dropped-batch rules, each with the store that fails it

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 7, AC-04, AC-05, BR-13 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG and scope seams |
| Project charter | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` — AC-003, AC-010, DR-04, DoD 1 |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/commit-rollback-and-drop-rules/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` — Architecture Notes 1 (seam map), 4 (probe + emitters), 5 (fixture contract), 6 (what one rule does), 7 (the foreign-batch hole), 9 (hostile stores, no literal positions); Testing brief rows AC-003 / AC-005 / AC-010; UX brief AC-U08 – AC-U10 |
| Signed-off design | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — `surfaces: []`, approved 2026-08-12. **No user-facing surface**; this story renders none |
| Story map | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` — slice 4, `commit-atomicity-and-mutants` |
| Roadmap pointer | `RUNBOOK.md:3848-3965` (phase 6), `RUNBOOK.md:3893-3897` (PS-7's Drop contract, and the probe that found the defect) |
| Normative source | `spec/SPECIFICATION.md` §4.0 (`:4632-4731`), §4.2 (`:4898-4925`), §4.5 (`:5099-5155`), §4.7 (`:5265-5345`), §4.11 (`:5652-5703`). **Where this spec and the specification disagree, the specification wins.** |

## One-line PR slice

The commit-side rules and the store that fails each: `failed_commit_leaves_both_unchanged`,
`rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable` (drop bare, then open and
commit a second batch — a pooled connection whose `Drop` returns to nothing answers `Busy` forever),
`commit_rejects_a_foreign_batch` via the per-instance stamp,
`commit_accepts_a_position_the_batch_did_not_write`, `commit_rejects_a_regressing_position` and
`distinct_projections_advance_independently`, each registered with its mutant and never asserting a
literal position.

## Executive summary

**What this PR lands.** Seven projection conformance rules and, in the same commit, seven wrong
stores that fail them — one per rule, registered as data in the projection mutant registry with the
exact rule set each fails and a pinned assertion substring.

**Pointer, not restatement.** The suite entry point, the `ProjectionFixture` contract, the
`Capability` / `RuleOutcome` reuse and the three emitters were landed by
`projection-suite-entry-point`; the registry shape, its three exactness meta-tests and
`CheckpointOnlyStore` were landed by `projection-mutant-registry` (HS-S0009, this story's declared
dependency and its slice-mate). Both already carry two rules — `commit_advances_the_checkpoint` and
`commit_is_atomic_with_the_read_model`. This story is the **delta**: it takes the rule set from two
to nine and takes the registry from one hostile store to eight.

**The delta that matters.** Before this PR the projection suite proves one thing — that a store which
drops the read-model write is caught. After it, the whole commit path is differential: a *failed*
commit, an explicit `rollback`, a batch dropped on the floor, a batch begun on the wrong store, a
position the batch never wrote, a position that goes backwards, and two projections that must not
share a checkpoint row. Project DoD 1 — *`CheckpointOnlyStore` fails by name* — becomes observable at
the end of this slice, and initiative DoD 7's first half moves with it.

**What it deliberately does not do.** It writes no `reset` rule and no read-through rule (slice 5),
no second batch shape (slice 6), no clause maturity edits and no freeze verdict (slice 8, and
HS-P0015 respectively).

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted anchor.

### 1. The port shape is a specification, not a suggestion

These rules are written against the trait at `spec/SPECIFICATION.md:4632-4731`, already landed by
`owned-batch-port-shape`: `type Batch;` with no lifetime, `fn begin(&self) -> Self::Batch` (neither
`async` nor fallible), `commit(batch, id, position, Authority) -> Result<(), CommitError<Self::Error>>`,
`rollback(batch) -> Result<(), Self::Error>`, and `checkpoint(id) -> Result<Checkpoint, Self::Error>`
returning `NeverRun` / `Live { through }` / `Rebuilding { through }`. `CommitError` carries
`ForeignBatch`, `CheckpointRegression { current, attempted }` and `Store(E)`. Do not reach for a
different spelling because a rule would be easier to write: if the shape blocks a rule, that is an
ADR-0017 finding to report, not a signature to adjust.

### 2. The suite may only observe the read model through `ProjectionProbe`

`probe_write` / `probe_read` / `probe_delete_all` / `probe_read_through` live in
**`happenstance-core`** behind `feature = "conformance"` (`spec/SPECIFICATION.md:4998-5031`), not in
the testkit, because an adapter's `tests/` is a different crate where neither a testkit trait nor the
adapter's type is local and the impl is rejected by the orphan rule. Every rule here writes through
`probe_write` and reads back through `probe_read` on a **fresh handle**; none reaches into an
adapter's internals. Without the probe the whole suite degenerates into a checkpoint test a broken
store passes (`RUNBOOK.md:3882-3892`).

### 3. A rule and the store that fails it land together, or neither lands

ADR-0010 (`.kb/decisions/0010-the-suite-must-prove-itself.md`) plus CF-1 – CF-4: every rule has a
registered wrong implementation; the registry is data, not prose; the assertion is made in **both**
directions so a mutant broken in more ways than it declares is caught; and provenance names the real
adapter shape that makes the defect plausible. `Declared` also carries `expect` — per-rule
`(rule, substring)` pins naming the exact assertion the mutant is expected to trip
(`crates/happenstance-testkit/tests/mutation_coverage.rs:141-186`). **No pass rate is ever quoted
over the mutant set.** The §4.11 *Rejects* column is where each mutant's provenance comes from; it is
not an invention slot (`spec/SPECIFICATION.md:5658-5671`).

### 4. Adding a rule re-opens every existing mutant's exactness claim

`mutants_fail_exactly_their_declared_rules` (`mutation_coverage.rs:2889`) runs **every** registered
store against **every** registered rule. Seven new rules therefore re-interrogate
`CheckpointOnlyStore`: where a new rule legitimately catches it — most plausibly
`dropped_batch_leaves_store_usable`, whose non-vacuity anchor asserts the *second* batch's row is
present — its `fails` list grows in this PR. That is the meta-test working, not a widening, and the
alternative (weakening the new rule so the old declaration survives) is the failure mode to refuse.

### 5. PS-7's rule has two halves and the second one is the rule

"Rolls back" alone certifies a store that has permanently lost its only writer. `dropped_batch_leaves_store_usable`
drops a batch **bare** — no `commit`, no `rollback` — and then opens and commits a *second* batch on
the same handle: the first probe row is absent, the second commit succeeds and its row is readable.
A reviewer's probe already found a pooled connection whose `Drop` returned it to nothing and a store
that answered `Busy` forever afterwards (`spec/SPECIFICATION.md:4898-4910`, `RUNBOOK.md:3893-3897`).
PS-7 is `[FROZEN]`; this rule is what makes it enforceable.

### 6. The foreign-batch check is run-time, by stamp, and the lifetime fix is refuted

Tying the batch to the receiver's lifetime was **compiled and refuted** — a lifetime names a region,
not an instance, and two `&Store` references unify — and the only construction that names an instance
is a generative brand, which fights `async` and forbids the batch escaping a closure
(`spec/SPECIFICATION.md:5099-5125`). So `begin` stamps an identity minted per store instance and
`commit` compares; with an owned batch the stamp is a field and the check is an integer comparison.
`commit_rejects_a_foreign_batch` therefore builds **two isolated stores from two `open()` calls** on
the `impl AsyncFn() -> F` every rule is handed (`crates/happenstance-testkit/src/registry.rs:45-49`)
and explicitly does **not** use `SECOND_HANDLE` (`spec/SPECIFICATION.md:5145-5155`). It asserts
`ForeignBatch` *and* that both stores are unchanged.

### 7. The checkpoint is a high-water mark of consideration, not of application

PS-21 is `[FROZEN]`: `commit` MAY name a position no applied event occupies and an adapter MUST NOT
validate `position` against what the batch wrote. Validating is a *reasonable* reading of the port
that would be equally conformant today and makes a narrow projection re-scan the same range forever
(`spec/SPECIFICATION.md:5271-5290`). `ValidatingCommitStore` is named by the specification itself as
the store that must fail this rule (`:5680-5685`). PS-22 is its dual: a position strictly below the
current checkpoint is rejected as `CheckpointRegression { current, attempted }`, **equal positions
MAY be accepted**, and both halves stay unchanged after the rejection (`:5292-5322`).

### 8. Nothing `[FROZEN]` is line-edited here

PS-1's MUST is a **coupling, not a progress obligation** — a `commit` returning `Ok` that makes
neither write durable satisfies the "or not at all" arm — and the repair is a decision atom owned by
`ps-clause-pairing-sweep` / `projection-decision-atoms`, never an edit
(`spec/SPECIFICATION.md:4742-4752`, `.kb/open-questions/ps-1-states-no-progress-obligation.md`).
`failed_commit_leaves_both_unchanged` is scoped to the **second conjunct on its own** — a partial
apply that reports failure — and claims nothing about progress. If writing it makes the gap feel
worth closing, that is a finding for the sweep, reported at this story's boundary.

### 9. Inducing a commit failure is a fixture capability, in `MID_BATCH_FAULT`'s mould

Nothing in the port lets an outside caller make a conformant store's `commit` fail; the injection has
to come from the adapter, which is exactly the argument that made `MID_BATCH_FAULT` a capability
rather than testkit machinery (`crates/happenstance-testkit/src/contract.rs:196-211`). So
`failed_commit_leaves_both_unchanged` is gated with `require!` and returns
`RuleOutcome::Skipped { capability, reason }` carrying the fixture's own words when the capability is
declined (`crates/happenstance-testkit/src/suite.rs:37-46`, CF-18 at `spec/SPECIFICATION.md:7597-7600`).
**Which** capability constant exists is `_design.md`'s call via Architecture brief Note 5 and
`projection-api-design-record`'s DT-3 resolution — if the projection fixture contract as landed
carries no commit-fault capability, this story **reports** rather than mints a second declension
policy (`project.md` Risks; `.kb/open-questions/cf-40-fixture-limits-ownership.md`).

### 10. Positions come from the store, never from a literal

CF-6 is `[FROZEN]`: no rule may assert on a literal sequence-position value, because the
specification permits gaps and a rule that assumes density passes the reference store and fails a
conformant adapter in the field (`spec/SPECIFICATION.md:7233-7244`). A projection rule has no event
log to take positions from, so it anchors on `SequencePosition::FIRST` and `.next()`
(`crates/happenstance-core/src/event.rs:244`, `:272-282`) — never `SequencePosition::new(<literal>)`
and never an integer list. A `cargo xtask ci` lint step scans the rule files for exactly those two
shapes (`xtask/src/lints.rs:640-700`). CF-33 applies too: no rule reads a clock.

### 11. The persona slice this realizes

P2, the adapter author, wants *"an executable definition of 'correct' they can run against their own
storage system, rather than a prose specification they have to interpret"*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-153`).
Seven of the port's most misreadable obligations stop being prose in this PR. Initiative AC-04 —
*told when they are finished* — is what a named red test is; AC-05 — *told, with a reason, where a
guarantee does not apply* — is why decision 9 is a reported skip and not an omitted test.

## Integration contract

- **Archetype**: `capability` — a user-observable slice, where the user is the adapter author and the
  observable surface is the suite's rule set and what a run prints.
- **Slice / milestone**: `commit-atomicity-and-mutants`. Slice-mate: `projection-mutant-registry`
  (HS-S0009), implemented in the same context and mounted as one integrated surface. This story lands
  second within the slice (`_storymap.md` Merge order, step 4).
- **Mount point**: `crates/happenstance-testkit/src/registry.rs` — the single
  `for_each_projection_store_rule!` enumeration. For a conformance rule this *is* the composition
  root: a rule not named there is compiled by nothing, emitted by no harness, and invisible to the
  orphan meta-test. A rule function that exists and is not enumerated is the library equivalent of a
  component that was built and never rendered.
- **Wires into** (real sibling contracts, by path, all pre-existing):
  - `crates/happenstance-core/src/projection.rs` — `ProjectionStore`, `Checkpoint`, `Authority`,
    `CommitError`, `ResetError` (landed by `owned-batch-port-shape`).
  - `happenstance-core`'s `ProjectionProbe` behind `feature = "conformance"` — the only read/write
    seam these rules may use (`spec/SPECIFICATION.md:4998-5031`).
  - `crates/happenstance-testkit/src/contract.rs` — `Capability` (`:355-433`) and `RuleOutcome`
    (`:458-537`), reused **unchanged** per Architecture brief AC-A06, plus the `ProjectionFixture`
    trait landed by `projection-suite-entry-point`.
  - `crates/happenstance-testkit/src/registry.rs:222-295` — the tokio / blocking / wasm emitters,
    driven through whatever parameterisation Note 4 settled upstream. Rules take
    `impl AsyncFn() -> F`, not a made fixture (`:45-49`).
  - `crates/happenstance-testkit/tests/mutation_coverage.rs` + `mutation_coverage/{harness,mutants}.rs`
    — the `Declared` shape (`:141-186`) and the three exactness meta-tests (`:2734`, `:2754`,
    `:2889`).
  - `crates/happenstance-core`'s `MemoryProjectionStore` and its fixture — the oracle every rule is
    run green against.
  - The three projection harness files under `crates/happenstance-testkit/tests/` (tokio, blocking,
    `#![cfg(target_arch = "wasm32")]`), and `CHANGELOG.md` for CF-29.
- **Renders surfaces**: **none.** `_design.md` declares `surfaces: []` and records the no-surface
  determination as the thing that was signed off. This story renders no screen and adds no public
  item to `happenstance-core`; its whole output is rules, mutants and one changelog entry.
- **Advances DoD scenario**: initiative **DoD 7**, first half — *"a deliberately wrong implementation
  that writes a checkpoint without its read model fails it, by name"* — and project **DoD 1**, which
  the story map places at the end of this slice. It does **not** advance DoD 7's second half (two
  unlike batch shapes, slice 6) and makes no freeze claim.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed
outside it.

```
crates/happenstance-testkit/src/**
crates/happenstance-testkit/tests/**
CHANGELOG.md
spec/SPECIFICATION.md
xtask/src/spec_trace.rs
.bklg/from-contract-to-published-library/projection-store-freeze/commit-rollback-and-drop-rules/**
```

**In this PR**

- Seven rule bodies in the projection rules module created by `projection-suite-entry-point`, and
  their seven names added to `for_each_projection_store_rule!`.
- Seven wrong stores in `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` and their
  `Declared` entries — `fails`, `provenance`, `mode`, `expect` pins — in the projection registry.
- Any `fails` growth on `CheckpointOnlyStore` that the exactness meta-tests demand (context pack §4).
- `CHANGELOG.md`: an entry naming each of the seven rules and the defect it detects (CF-29,
  `spec/SPECIFICATION.md:8141-8155`; the lint asserts presence and ≥120 characters of prose per rule
  named — `xtask/src/lints.rs:500-575`).
- **The generated region of `spec/SPECIFICATION.md` only.** §7.1–§7.2 sit between
  `<!-- BEGIN GENERATED: spec-trace §7.1–§7.2 -->` and `<!-- END GENERATED -->`
  (`xtask/src/spec_trace.rs:200-201`), and the `†` in each row means *the checker looked for this rule
  and did not find it* (`:1055`). Landing these rules makes six rows stale, so `cargo xtask spec-trace`
  goes red until the region is regenerated with `--write` and committed. That is a machine-written
  block, **not** a clause edit: no maturity marker, no clause prose and nothing `[FROZEN]` is touched
  by hand.
- **`xtask/src/spec_trace.rs` `RULE_FILES` (`:85-89`) only if the upstream story did not already add
  the projection rules module.** That three-file list is what CF-6's lint, CF-29's changelog lint,
  CF-33's clock lint and spec-trace's check 6 all sweep; a rules file outside it means seven rules
  land invisible to four gate checks while every step still prints green. If it is already there,
  touch nothing; if it is not, add it and say so in the implementation report.

**Explicitly not in this PR**

- Any `reset` rule (`reset-rules`), `batch_reads_reflect_pending_writes`,
  `rebuild_is_chunk_size_invariant`, `rebuilding_is_distinguishable_from_live`
  (`read-through-and-rebuild-rules`).
- The CF-5 buffering conformant variant and the PS-3 finding (`buffering-conformant-variant`,
  `ps3-batch-shape-finding`).
- Clause maturity markers, the `unstable-projection` gate, the provisional block in `projection.rs`
  (`unstable-projection-gate-and-clause-disposition`).
- Any change to the `EventStore` suite's rules, fixtures or mutants; any change to `Capability` or
  `RuleOutcome`; any adapter skeleton; any new ADR.
- Repairing PS-1's or PS-19's scope gap (context pack §8) — report, do not absorb.

**Merge DoD (one line).** The nine-rule projection suite is green against `MemoryProjectionStore`
under all three emitters, each of the seven new rules is failed by exactly its declared mutant under
the three exactness meta-tests, and `cargo xtask ci` is green including `spec-trace` and the wasm32
conformance-harness check.

## Behavior and interfaces

Positions in every row are derived from `SequencePosition::FIRST` / `.next()`, never literals (context
pack §10). "Read back" always means a **fresh handle** and `probe_read`, never the batch.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `failed_commit_leaves_both_unchanged` | Commit at *P*. Arm the fixture's commit fault, `probe_write` a second row, commit at *P.next()* and require `Err`. Assert the second row is absent and the checkpoint still reads `Live { through: P }` — the second conjunct of PS-1 on its own, no progress claim. Gated with `require!`; a fixture that cannot inject a fault returns `Skipped` carrying its own reason. | `spec/SPECIFICATION.md:4732-4759` (PS-1, and why the coupling is not a progress obligation); `crates/happenstance-testkit/src/contract.rs:196-211` (the capability precedent); `crates/happenstance-testkit/src/suite.rs:37-46` (`require!`) |
| `rollback_leaves_both_unchanged` | Commit at *P*. `begin`, `probe_write`, `rollback(batch).await?`; assert the row is absent and the checkpoint is exactly as it was. `rollback` stays on the port because Rust has no async `Drop`. | `spec/SPECIFICATION.md:4911-4925` (PS-8, `[FROZEN]`) |
| `dropped_batch_leaves_store_usable` | `begin`, `probe_write`, `drop(batch)` — bare, no `commit`, no `rollback`. Then `begin` a **second** batch on the same handle, `probe_write` a different key, `commit` at the next position. Assert: first key absent, second key present, checkpoint advanced. Both halves; the second is the rule. | `spec/SPECIFICATION.md:4898-4910` (PS-7, `[FROZEN]`); `RUNBOOK.md:3893-3897`; Architecture brief Note 6 |
| `commit_rejects_a_foreign_batch` | Two `open()` calls → two isolated stores. `begin` on A, `commit` on B; assert `CommitError::ForeignBatch` and assert **both** stores unchanged (checkpoint and probe read). Deliberately does not use `SECOND_HANDLE`. | `spec/SPECIFICATION.md:5126-5155` (PS-15, `[PROVISIONAL]`); `crates/happenstance-testkit/src/registry.rs:45-49`; `spec/E2E-CASES.md:502` (E2E-19) |
| `commit_accepts_a_position_the_batch_did_not_write` | Commit an **empty** batch at a position no probe write touched; assert `Ok` and assert the checkpoint advanced to it. The checkpoint is a high-water mark of consideration. | `spec/SPECIFICATION.md:5277-5290` (PS-21, `[FROZEN]`); `spec/E2E-CASES.md:599` (E2E-23) |
| `commit_rejects_a_regressing_position` | Commit at *P*, then attempt *Q* < *P*; assert `CommitError::CheckpointRegression { current, attempted }` with both fields matching what the store was given, and assert the read model and checkpoint are both unchanged. Equal positions MAY be accepted and the rule must not assert otherwise. | `spec/SPECIFICATION.md:5297-5315` (PS-22, `[PROVISIONAL]`) |
| `distinct_projections_advance_independently` | Two `ProjectionId`s in one store, committed at different positions with distinct probe rows; assert each id reads back its own checkpoint and neither commit disturbed the other. | `spec/SPECIFICATION.md:5317-5333` (PS-23, `[PROVISIONAL]`); `spec/E2E-CASES.md:725` (E2E-28) |
| Every rule returns `RuleOutcome` | `#[must_use]`; the emitter calls `.report(name)`. A rule that ran returns `Ran`; only a rule whose **entire** content needed a declined capability returns `Skipped`. | `crates/happenstance-testkit/src/contract.rs:458-537`; `crates/happenstance-testkit/src/suite.rs:25-36` |
| One mutant per rule, registered as data | Seven entries in the projection `REGISTRY: &[Declared]`, each with a non-empty `provenance` taken from §4.11's *Rejects* column and an `expect` pin naming the exact assertion. `ValidatingCommitStore` is named by the specification; the other six names are the implementer's, and the *defect* each models is not. Candidates, defect-first: a partial apply that reports failure; a rollback that only discards the buffer; a `Drop` that returns a pooled connection to nothing and answers `Busy` afterwards; a batch carrying no per-instance stamp; a `commit` that validates the position; an unconditional `UPDATE checkpoint SET position = ?`; a single-row checkpoint table. | `spec/SPECIFICATION.md:5658-5685`; `crates/happenstance-testkit/tests/mutation_coverage.rs:141-186`; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| Exactness, both directions | `every_rule_has_a_mutant`, `mutant_registry_is_exhaustive` and `mutants_fail_exactly_their_declared_rules` stay green over the nine-rule set — including for `CheckpointOnlyStore`, whose declaration this PR may legitimately grow. No pass rate is quoted anywhere. | `crates/happenstance-testkit/tests/mutation_coverage.rs:2734`, `:2754`, `:2889` |
| Mounted under all three emitters | The seven names go in the one enumeration, so the tokio and blocking harnesses gain seven tests each and the `#![cfg(target_arch = "wasm32")]` harness is type-checked by the mandatory wasm32 conformance-harness step — no separately maintained subset. | `xtask/src/main.rs:231-243`; `_storymap.md` (AC-016 is proven by the existing mechanism) |
| Both flavours | Rules bind `ProjectionStore` (the weaker trait), never `SendProjectionStore`, and import only one of the two names per module. | `CLAUDE.md` binding constraint 4; `crates/happenstance-core/src/projection.rs:87-88` |
| No clock, no literal positions | CF-33 and CF-6, both enforced by `cargo xtask ci` lint steps over the rule files. | `spec/SPECIFICATION.md:7233-7244`, `:8236`; `xtask/src/lints.rs:33-42`, `:640-700` |

## Data and migrations

**N/A — no persisted data and no migration.** Nothing here reads or writes a database, a schema or a
file format. The two "data" artefacts this story touches are both in-repo source of record and both
are additive:

- The projection `REGISTRY: &[Declared]` — a `&'static [Declared]` compiled into the mutant-coverage
  test binary (`crates/happenstance-testkit/tests/mutation_coverage.rs:141-186`). Adding rows changes
  no shape; the three exactness meta-tests are its schema check.
- The generated §7.1–§7.2 region of `spec/SPECIFICATION.md`, rewritten by
  `cargo xtask spec-trace --write` (`xtask/src/spec_trace.rs:200-201`, `:1248`). The checker is the
  authority; the region is regenerated, never hand-edited.

`MemoryProjectionStore` is in-process and volatile, so no fixture teardown, no schema and no ordering
constraint against another crate's data exists to manage.

## Acceptance criteria

Framed from **P2, the adapter author**, whose goal is *"an executable definition of 'correct' they
can run against their own storage system, rather than a prose specification they have to interpret"*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:125-127`).
Each row is a whole crossing of this library's stack — a name in the one enumeration, expanded into a
test by three emitters, driving the real port through the real probe against a real store, and read
back by the real gate. Nothing here is satisfied by a function that exists; a rule not enumerated is
compiled by nothing.

`redkiln verify --grain story` extracts these ids from the leading `| AC-### |` cell, so every
blocking invariant in this spec — including every interaction-quality invariant in the next
section — is a row here and nowhere else.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author whose store can make one write of a batch fail (a trigger raising on the third insert, a `CHECK` armed for one write, a connection killed mid-statement), **WHEN** they run the projection suite, **THEN** they are told whether a `commit` that *reports failure* left the read model and the checkpoint exactly as they were; **AND GIVEN** a fixture with no fault to inject, the rule still runs, reports the fixture's **own stated reason**, and is never mistaken for a pass. | `failed_commit_leaves_both_unchanged` is green against `MemoryProjectionStore` under the tokio and blocking projection harnesses, and red against the partial-apply mutant at the pinned `expect` substring. The declined arm is asserted on the returned **`RuleOutcome::Skipped { capability, reason }` value** (`crates/happenstance-testkit/src/contract.rs:458-537`), never on stdout. Scope check at review: the rule asserts only PS-1's second conjunct and makes no progress claim (`spec/SPECIFICATION.md:4742-4759`). |
| AC-002 | **GIVEN** an adapter author who must call `rollback` explicitly because Rust has no `async Drop`, **WHEN** they run the suite, **THEN** they learn whether an explicit rollback left **both halves** — read model and checkpoint — exactly as the last successful commit left them, rather than only whether it returned `Ok`. | `rollback_leaves_both_unchanged` green against the oracle, red against the mutant whose `rollback` discards only its in-memory buffer while the applied rows stay durable. Read-back is through a **fresh handle** and `probe_read`, never through the rolled-back batch. Clause: PS-8 `[FROZEN]` (`spec/SPECIFICATION.md:4911-4925`). |
| AC-003 | **GIVEN** an adapter author whose batch holds a pooled connection, **WHEN** a batch is dropped bare — no `commit`, no `rollback` — **THEN** they are told not merely that the work rolled back but that **the store is still usable afterwards**: a second batch opens on the same handle, commits, and its row reads back. *(Project AC-010; the second half is the rule.)* | `dropped_batch_leaves_store_usable` green against the oracle; red against the mutant whose `Drop` returns a pooled connection to nothing and answers `Busy` forever after — the defect a reviewer's probe actually found (`spec/SPECIFICATION.md:4898-4910`, `RUNBOOK.md:3893-3897`). Non-vacuity anchor: the assertion that the **second** batch's row is present, which is what a "rolls back" -only rule cannot make. |
| AC-004 | **GIVEN** an adapter author whose application holds more than one store instance, **WHEN** a batch begun on store A is committed on store B, **THEN** they are told it is rejected as `CommitError::ForeignBatch` **and** that neither store was disturbed — so the run-time stamp is proven to be a check and not a comment. | `commit_rejects_a_foreign_batch` builds **two isolated stores from two `open()` calls** on the `impl AsyncFn() -> F` every rule is handed (`crates/happenstance-testkit/src/registry.rs:45-49`) and deliberately does **not** use `SECOND_HANDLE`. Asserts the error variant and both stores' checkpoint + probe reads. Red against the mutant whose batch carries no per-instance stamp. Clause PS-15 (`spec/SPECIFICATION.md:5126-5155`), case E2E-19 (`spec/E2E-CASES.md:502`). |
| AC-005 | **GIVEN** a narrow projection that considered a range and applied nothing from it, **WHEN** its author commits an empty batch at that range's end, **THEN** the suite tells them the checkpoint advanced — a high-water mark of *consideration*, not of application — so their projection does not re-scan the same range forever. | `commit_accepts_a_position_the_batch_did_not_write` green against the oracle and **red against `ValidatingCommitStore`**, the store the specification itself names for this rule (`spec/SPECIFICATION.md:5680-5685`). Clause PS-21 `[FROZEN]` (`:5277-5290`), case E2E-23 (`spec/E2E-CASES.md:599`). The mutant is what makes it non-decorative: validating the position is a *reasonable* misreading that would otherwise be equally conformant. |
| AC-006 | **GIVEN** an adapter author whose runner restarts and replays, **WHEN** a commit names a position strictly below the current checkpoint, **THEN** they are told it is refused as `CheckpointRegression { current, attempted }` with both fields naming what the store was actually given, and that neither half moved — **AND** they are never told that an *equal* position must be refused, because the clause permits accepting it. | `commit_rejects_a_regressing_position` asserts the variant, both field values (derived from `SequencePosition::FIRST`/`.next()`, never literals) and the unchanged read model and checkpoint. Red against the mutant whose `commit` issues an unconditional `UPDATE checkpoint SET position = ?`. Review check: no assertion anywhere on the equal-position case (PS-22, `spec/SPECIFICATION.md:5292-5322`). |
| AC-007 | **GIVEN** an adapter author running several projections over one store, **WHEN** each advances at its own rate, **THEN** they are told whether one projection's commit disturbed another's checkpoint or rows — the failure a single-row checkpoint table produces silently. | `distinct_projections_advance_independently` commits two `ProjectionId`s at different positions with distinct probe rows and asserts each reads back its own checkpoint and its own row. Red against the single-row-checkpoint mutant. Clause PS-23 (`spec/SPECIFICATION.md:5317-5333`), case E2E-28 (`spec/E2E-CASES.md:725`). |
| AC-008 | **GIVEN** an adapter author who has just gone green, **WHEN** they ask what a green run is worth, **THEN** each of the seven new rules has a **registered wrong implementation that fails exactly it**, with provenance naming a real adapter shape and an `expect` pin naming the exact assertion — and **no pass rate is quoted anywhere over the mutant set**. *(Project AC-003.)* | The three exactness meta-tests stay green over the widened matrix: `every_rule_has_a_mutant`, `mutant_registry_is_exhaustive`, `mutants_fail_exactly_their_declared_rules` (`crates/happenstance-testkit/tests/mutation_coverage.rs:2734`, `:2754`, `:2889`, and their projection siblings landed by `projection-mutant-registry`). Seven `Declared` rows, each with non-empty `provenance` and an `expect` pin (`:141-186`). Where a new rule legitimately catches `CheckpointOnlyStore`, **its declaration grows in this PR** — weakening the new rule to preserve the old declaration is the refused repair (context pack §4). Review check: no fraction over the mutant set appears in code, doc or changelog (`.kb/decisions/0010-the-suite-must-prove-itself.md`). |
| AC-009 | **STATE invariants.** **GIVEN** an adapter author reading one run's output, **WHEN** the suite executes, **THEN** every one of the seven rules is *present in the run on every runtime* — enumerated once, emitted by the tokio, blocking and `wasm32` emitters alike, with none silently absent, none reachable only behind an extra feature flag, and a failure reported **at the failing rule's own name** rather than as an anonymous harness abort; a declined capability yields a reported skip that **occludes nothing** — every other rule's outcome still appears — and each rule leaves the store usable for the next, so no rule's effect and no run ordering can change another's verdict. | The seven names appear in the single `for_each_projection_store_rule!` enumeration (`crates/happenstance-testkit/src/registry.rs`) and the projection orphan meta-test (landed by `projection-suite-entry-point`) fails if a rule in the module is absent from it. Presence on the constrained target is proven by the mandatory `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` step (`xtask/src/main.rs:231-243`), not by a separately maintained subset. Independence is proven by construction — distinct `ProjectionId`s and distinct probe keys per rule, and each rule opening its own fixture through `impl AsyncFn() -> F` — and asserted by AC-008's `mutants_fail_exactly_their_declared_rules`, which runs every store against every rule in both directions. |
| AC-010 | **COMPOSITION invariants.** **GIVEN** the only surface this project has — the text a run prints and the record of what changed (`_design.md` declares `surfaces: []`) — **WHEN** the seven rules land, **THEN** every reported outcome carries **real composed presentation** through the one existing renderer rather than bare output: one skip vocabulary and one line shape, ``SKIP {rule}: fixture declines `{capability}` — {reason}``, naming the capability constant the author can actually change and carrying the fixture's own words; **AND** the density budget holds — **one** enumeration gains exactly seven names (2 → 9), **one** registry gains exactly seven rows (1 → 8 stores), **zero** new skip types, **zero** new harness files, **zero** new gate steps; **AND** the human-readable record exists — a `CHANGELOG.md` entry naming each rule and the defect it detects, and a regenerated §7.1–§7.2 so no rule is left marked `†` *not found*. | Skip rendering is `RuleOutcome::report`/`skip_line` reused **unchanged** (`crates/happenstance-testkit/src/contract.rs:500-537`; UX brief AC-U08, AC-U10, AC-U11) — a second format would compile, so review plus the value assertion is the check. Density is a diff review against the four counts, plus the orphan and exhaustiveness meta-tests, which fail on a second enumeration or a second registry. The record is machine-checked: `changelog_names_every_rule` with `MIN_CHARS_PER_RULE = 120` (`xtask/src/lints.rs:525`, `:446`) and `cargo xtask spec-trace` green over the regenerated region (`xtask/src/spec_trace.rs:200-201`). |

Every project AC this story traces to is covered: **AC-003** by AC-008 (and by the seven mutants the
rule rows each name), **AC-010** by AC-003 above.

## Interaction quality

RFC §6.7/D6, in the medium this story actually has. There is no screen: `_design.md` declares
`surfaces: []` and the no-surface determination is the thing that was signed off
(`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:48-50`, `:96-101`), and
its Anti-patterns section is `N/A` for the same reason. The binding presentation constraints
therefore come from the project's **UX brief**, which holds the text surface to named criteria
(`_decomposition.md` UX brief, AC-U08 – AC-U11), and from the project's own reader table: *"Whoever
reads the run … only the text surface; AC-U08 – AC-U11 are the whole of what they get"*
(`_decomposition.md:219`).

Every invariant below is **already an AC row above**. This section says only which id carries which
invariant and how it is verified — a bullet here would get no ledger row and would never be gated.

**STATE family — carried by AC-009.**

| Invariant | Reading in this medium | Carried by | Verified by |
| --- | --- | --- | --- |
| In place, not a context jump | A failure names the rule that failed, in the run the author already started; it does not abort the harness or send them to a different binary to find out which obligation broke | **AC-009** | one test per rule from the emitters; `mutants_fail_exactly_their_declared_rules` asserts the panic origin is *positively* inside the rules file |
| Non-occlusion | A skipped or failing rule never hides the other eight outcomes; the run reports all of them | **AC-009** | `RuleOutcome` values per rule; libtest reports every test |
| Preserved selection / continuity | Running one rule leaves the store usable for the next, and no rule depends on another's residue — distinct `ProjectionId`s, distinct probe keys, its own fixture instance | **AC-009** (and **AC-003**, which is this invariant *as* the rule) | the exactness meta-test drives every store through every rule in one process |
| Reversibility | The two ways out of an uncommitted batch — explicit `rollback` and a bare drop — both return the store to its prior state *and* leave it working | **AC-002**, **AC-003** | `rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable` |
| Reachability without a special path | Every rule reaches the author through the one documented entry point `projection_store_conformance!`, on every runtime including `wasm32`, with no extra feature flag and no separately maintained subset | **AC-009** | the single enumeration + orphan meta-test; the mandatory wasm32 conformance-harness step |

**COMPOSITION family — carried by AC-010.**

| Invariant | Reading in this medium | Carried by | Verified by |
| --- | --- | --- | --- |
| Presentation exists at all | Nothing prints bare: a declension is rendered by `RuleOutcome::skip_line`, not by an ad-hoc `println!` or a bare `assert!` message | **AC-010** | reuse of `contract.rs:500-537`; review — a fork compiles, so the type reuse plus the design constraint is the check (UX brief AC-U08) |
| Composition / placement | The seven rules are placed in the projection rules module and named in the one enumeration beside the existing two — not a second module with a second entry point | **AC-009**, **AC-010** | orphan meta-test; `RULE_FILES` sweep in `xtask/src/spec_trace.rs:85-89` |
| Transience | Persistent: the rule set and the changelog record. Revealed on demand: the skip reason, printed only when a capability is declined. Never transient: nothing this story lands disappears after a run — a declined rule stays *in the binary* and in the output | **AC-001**, **AC-009**, **AC-010** | `RuleOutcome::Skipped` value assertion; CF-18 (`spec/SPECIFICATION.md:7597-7600`) |
| Density budget, with numbers | +7 rule names (2 → 9), +7 registry rows (1 → 8 stores), +0 skip vocabularies, +0 harness files, +0 gate steps, +1 changelog entry with ≥ 120 characters of prose per rule named | **AC-010** | diff review against the five counts; `changelog_names_every_rule` (`xtask/src/lints.rs:525`, `MIN_CHARS_PER_RULE` at `:446`) |
| Hierarchy | The rule name leads, the reason follows, the capability constant is named so a reader is sent to the switch they can change — `NO_STORE_LIMITS`' precedent, which names all three constants because *"a skip naming only one of them would send an adapter author to look for the constant they did set"* | **AC-010** | `contract.rs:435-442`; UX brief AC-U10 |
| Named anti-patterns | `_design.md` names none (no surface). The three this story must not commit are inherited from the UX and architecture briefs: **a second skip vocabulary**, **a second rule enumeration**, and **a rule whose reason survives on the host and silently vanishes on `wasm32`** (`report` is a measured no-op there) | **AC-009**, **AC-010** | AC-A06 reuse; orphan meta-test; UX brief AC-U11 and the wasm harness routing `skip_line` through `console_log!` |

## Error conditions

| id | Condition | Required handling |
| --- | --- | --- |
| **EC-001** | A fixture cannot inject a commit fault. | `failed_commit_leaves_both_unchanged` returns `RuleOutcome::Skipped { capability, reason }` carrying the **fixture's own words** via `require!`, and still appears in the run. It must not pass silently, must not be `#[ignore]`d, and must not be omitted from the enumeration (`crates/happenstance-testkit/src/suite.rs:37-46`; CF-18). |
| **EC-002** | The `ProjectionFixture` as landed carries **no** commit-fault capability constant. | **Report, do not invent.** Which constant exists is `_design.md`'s call via Architecture brief Note 5 and `projection-api-design-record`'s DT-3 resolution. Minting a second declension policy here is the failure `.kb/open-questions/cf-40-fixture-limits-ownership.md` names. Record the gap in the implementation report and at the story boundary. |
| **EC-003** | An exactness meta-test goes red because a new rule catches `CheckpointOnlyStore` (most plausibly `dropped_batch_leaves_store_usable`). | Grow that mutant's `fails` list **in this PR** and say so in the report. Weakening the new rule so the old declaration survives is the refused repair (context pack §4). |
| **EC-004** | `cargo xtask spec-trace` is red after the rules land, with `†` on §7.1–§7.2 rows. | Regenerate with `cargo xtask spec-trace --write` and commit the **generated region only**. No clause prose, no maturity marker and nothing `[FROZEN]` is hand-edited (`xtask/src/spec_trace.rs:200-201`). |
| **EC-005** | A CF-6/CF-33 lint fires, or fires on nothing because the projection rules module is outside `RULE_FILES`. | Fix the rule body (derive positions from `SequencePosition::FIRST`/`.next()`), or add the module to `RULE_FILES` — noting the array is typed `[&str; 3]` (`xtask/src/spec_trace.rs:85-89`) so the length changes with it — and state in the report which of the two it was. A rules file outside that list is swept by **four** gate checks that all still print green. |
| **EC-006** | The port shape as landed blocks a rule (for example, `commit` cannot be reached with an empty batch, or `CommitError` lacks a field the rule must assert on). | That is an **ADR-0017 finding to report**, not a signature to adjust. Do not re-spell the trait to make a rule easier to write (context pack §1). |
| **EC-007** | Writing `failed_commit_leaves_both_unchanged` makes PS-1's missing progress obligation feel worth closing. | Report it to `ps-clause-pairing-sweep`; do not absorb it. PS-1 is `[FROZEN]` and the repair is a new decision atom (`.kb/open-questions/ps-1-states-no-progress-obligation.md`). |
| **EC-008** | An `expect` pin names a rule the mutant does not declare, or a `fails` entry names a rule that does not exist. | `mutant_registry_is_exhaustive` fails by design — an unreachable pin reads as coverage and is not. Fix the row, never the meta-test. |
| **EC-009** | `require!` as written resolves `<$fixture as $crate::Fixture>::$capability` (`crates/happenstance-testkit/src/suite.rs:37-46`) and the projection fixture is a different trait. | Use whatever projection-aware gate `projection-suite-entry-point` / `projection-capability-skips` landed. If neither landed one, **report the gap** before writing a third skip mechanism — a second declension path is EC-002's failure by another route. |

## Non-functional

| id | Requirement | Enforced by |
| --- | --- | --- |
| **NF-001** | No rule asserts on a literal sequence position and no rule reads a clock. Positions are `SequencePosition::FIRST` and `.next()` only — note `.next()` returns `Option<Self>` by design, so the `None` arm is handled rather than assumed away (`crates/happenstance-core/src/event.rs:244`, `:272-282`). | `no_position_literals` and `no_clock` lint steps (`xtask/src/lints.rs:628`, `:231`), both sweeping `RULE_FILES` (CF-6, CF-33). |
| **NF-002** | Rules bind `ProjectionStore`, never `SendProjectionStore`, and import only one of the two names per module. No `#[async_trait]` anywhere. | `CLAUDE.md` binding constraints 1 and 4; `cargo clippy -D warnings`; the ambiguity is a compile error the moment both names are in scope. |
| **NF-003** | Every rule type-checks for `wasm32-unknown-unknown`: no threads, no `catch_unwind`, no host-only I/O in a rule body. | The mandatory `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` step (`xtask/src/main.rs:231-243`). |
| **NF-004** | No borrowing GAT is introduced anywhere in the rules or the seven mutants — `type Store<'a> where Self: 'a` on a foreign trait is one of five ingredients of a rustc ICE this repository already minimised and which still reproduces on 1.97.1. | `crates/happenstance-testkit/src/contract.rs:97-111`; `experiments/rustc-ice-gat-foreign-trait/`. Copy `MemoryFixture`'s owned-handle pattern (`crates/happenstance-testkit/src/fixtures.rs`). |
| **NF-005** | Deterministic: no sleeps, no timing assumptions, no dependence on rule execution order or on a shared static. Each rule opens its own fixture instance(s) and is O(a handful) of operations. | The three emitters run rules in unspecified order; `mutants_fail_exactly_their_declared_rules` re-runs every rule against every store in one process. |
| **NF-006** | **No new public item and no new dependency.** Nothing here is added to any crate's `lib.rs` export block or feature table, and nothing is a semver promise; the mutants live in `happenstance-testkit`'s own `tests/`, which is where CF-1 requires them (`spec/SPECIFICATION.md:7161-7163`). | `cargo package --list` and the `--no-default-features` doc build inside `cargo xtask ci`; diff review. |
| **NF-007** | The projection mutant binary stays gated off `wasm32` (`#![cfg(not(target_arch = "wasm32"))]`), and the inherited cost — the seven mutants are never type-checked for that target — is stated rather than discovered. | `crates/happenstance-testkit/tests/mutation_coverage.rs:33-46` sets the precedent; the wasm32 `--tests` check is what breaks if the gate is dropped. |

## Implementation notes (non-prescriptive)

Constraints, not a recipe. The order below is the order that keeps the tree green.

- **Land the registry rows and the rules together, rule by rule.** CF-1 makes a rule without a mutant
  unlandable and CF-2 makes a mutant without a rule unlandable, so the smallest green step is one
  rule + one mutant + one `Declared` row + one changelog sentence. Seven of those, not two big ones.
- **Write the mutant defect-first.** Name the *implementation mistake* (a pooled connection whose
  `Drop` returns to nothing; a `commit` that validates `position`; a single-row checkpoint table)
  before naming the type. `struct AlwaysWrong` satisfies CF-1 mechanically and proves nothing,
  because no author would have written it (CF-4, `spec/SPECIFICATION.md:7209-7220`).
- **`SequencePosition::next()` returns `Option<Self>`** — deliberately, so that a consumer at the top
  of the key space is told rather than looping (`crates/happenstance-core/src/event.rs:272-282`).
  A rule takes the `None` arm as a panic with a message, not a silent `unwrap`.
- **The rule signature is `impl AsyncFn() -> F`**, not a made fixture, precisely so a rule can ask for
  a second isolated store (`crates/happenstance-testkit/src/registry.rs:45-49`). AC-004 is the rule
  that spends that affordance; the other six call it once.
- **`require!` binds `$crate::Fixture`.** Check what the upstream slice landed for the projection
  family before reaching for it (EC-009), and do not write a third skip path.
- **`RULE_FILES` is `[&str; 3]`** (`xtask/src/spec_trace.rs:85-89`), so adding the projection rules
  module is a length change as well as an element. Confirm whether `projection-suite-entry-point`
  already did it; if not, do it here and say so in the report.
- **Regenerate, do not edit.** `cargo xtask spec-trace --write` owns §7.1–§7.2's generated region.
  Run it last, after all seven rules are named, so the region is rewritten once.
- **Changelog entries are prose, not a listing.** `MIN_CHARS_PER_RULE = 120` exists because a
  comma-separated list of rule names carries about fifty characters and says nothing about the defect
  (`xtask/src/lints.rs:437-446`). Write the defect each rule detects, one entry per rule name.
- **Read-back is always a fresh handle and `probe_read`.** Never assert through the batch, never reach
  into an adapter's internals — the whole suite degenerates into a checkpoint test a broken store
  passes if a rule cannot see the read model (`RUNBOOK.md:3882-3892`).
- **Whether the `Declared`/`Kind`/`FailureMode` types are duplicated or shared via `#[path]`** was left
  to the implementer by `projection-mutant-registry`; this story consumes whatever it chose and does
  not re-open it.

## Tests and CI (merge gate)

Tier vocabulary is the project testing brief's: **Static** reads source without executing the code
under test; **Unit** is an in-process `#[test]`, including meta-tests over registries; **Integration**
is a rule actually driving a `ProjectionStore` through `begin` → probe-write → `commit`/`rollback` →
read-back; **E2E** is `cargo xtask ci` run whole
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:753-767`).

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static | `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` (inside `cargo xtask ci`) | The seven rules and seven mutants compile clean under the house lint set; NF-002's one-name-per-module rule is a compile error if broken |
| Static | `cargo xtask ci` → `no_position_literals` (`xtask/src/lints.rs:628`) over `RULE_FILES` | NF-001's CF-6 half, and — because the sweep is by file list — that the projection rules module is inside `RULE_FILES` at all (EC-005) |
| Static | `cargo xtask ci` → `no_clock` (`xtask/src/lints.rs:231`) | NF-001's CF-33 half |
| Static | `cargo xtask ci` → `changelog_names_every_rule` (`xtask/src/lints.rs:525`, `MIN_CHARS_PER_RULE` `:446`) | **AC-010**'s record half: each of the seven rule names has a changelog entry of ≥ 120 characters of prose naming a defect |
| Static | `cargo xtask spec-trace` | **AC-010**: the regenerated §7.1–§7.2 region resolves all seven rule names — no row left marked `†` *not found* (`xtask/src/spec_trace.rs:1055`) |
| Static | `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` (`xtask/src/main.rs:231-243`) | **AC-009**'s constrained-runtime arm and NF-003: the seven rules exist on `wasm32` as part of the same run, not as a maintained subset |
| Unit | `cargo test --workspace --all-features` → the projection orphan meta-test | **AC-009**: no rule in the module is missing from `for_each_projection_store_rule!` |
| Unit | `cargo test --workspace --all-features` → `every_rule_has_a_mutant`, `mutant_registry_is_exhaustive`, `mutants_fail_exactly_their_declared_rules` and their projection siblings (`crates/happenstance-testkit/tests/mutation_coverage.rs:2734`, `:2754`, `:2889`) | **AC-008** in both directions, including any `CheckpointOnlyStore` declaration growth (EC-003) and every `expect` pin (EC-008) |
| Unit | `RuleOutcome` value assertions on the declined-capability path | **AC-001**'s skip arm and **AC-010**'s presentation invariant — asserted on values, never on stdout (UX brief AC-U09) |
| Integration | `cargo test --workspace --all-features` → `projection_store_conformance!` against `MemoryProjectionStore` under the tokio and blocking harnesses | **AC-001** – **AC-007** green against the oracle: nine rules, both host emitters |
| Integration | the same seven rules driven against their seven mutants inside the projection mutant binary | **AC-008**: each mutant fails **exactly** its declared rules at the pinned assertion, and passes the rest |
| E2E | `cargo xtask affected --base main`; `cargo xtask ci --fast` | The story grain during implementation — the bar a non-terminal project meets (`_decomposition.md:832-845`) |
| E2E | `cargo xtask ci` (whole) | The merge DoD: nine-rule suite green under all three emitters, exactness meta-tests green, `spec-trace` and the mandatory wasm32 steps included — the two steps `--fast` omits and on which AC-009 and AC-010 partly rest |

## Risks and coupling (PR-scoped)

- **Slice ordering is load-bearing, not cosmetic.** `projection-mutant-registry` must land first: seven
  rules with nowhere to register a mutant are seven CF-1 violations. If the registry is not in the tree,
  stop rather than landing rules bare — the slice is implemented in one context precisely so this is a
  sequencing decision, not a merge race.
- **The exactness matrix is quadratic, so this PR re-opens an existing claim.** Adding seven rules re-runs
  `CheckpointOnlyStore` and the neither-write-durable store against all of them. Expect a `fails` growth,
  budget for it, and treat it as the meta-test working (EC-003).
- **A capability that does not exist.** AC-001 depends on a projection commit-fault capability whose
  existence is DT-3's and `_design.md`'s call, not this story's. If it is absent the rule cannot be
  gated honestly — report (EC-002) rather than fabricate a fault or drop the rule.
- **Two stories regenerating the same block.** If a slice-mate or a neighbouring story also runs
  `spec-trace --write`, the generated region conflicts textually. Regenerate once, last, after all seven
  names exist.
- **`RULE_FILES` is a four-check chokepoint.** CF-6, CF-29, CF-33 and spec-trace's check 6 all sweep it.
  A rules module outside it means seven rules land invisible to four gate checks while every step prints
  green — the most dangerous *silent* outcome available in this PR.
- **`wasm32` blind spot on the mutants.** The mutant binary is gated off that target (NF-007), so the seven
  hostile stores are never type-checked there. That is the inherited, stated cost — not a defect to fix
  here, but a fact the report should restate so nobody reads the wasm step as covering it.
- **Frozen-clause gravity.** `failed_commit_leaves_both_unchanged` sits next to PS-1's known scope gap and
  `commit_rejects_a_regressing_position` next to PS-22's equal-position permission. Both are places where a
  rule can quietly assert more than its clause says. The review check is in the AC rows: PS-1's second
  conjunct only, and no equal-position assertion at all.
- **Downstream coupling.** `buffering-conformant-variant` must pass all seven of these rules with a
  replay-at-commit batch. A rule that accidentally encodes apply-on-write semantics (for example, asserting
  a probe row is visible *before* commit) would pass here and fail there. Every rule reads back after
  commit, through a fresh handle, for exactly that reason.

## Dependencies

**Blocks on** — matches this story's `depends_on` exactly:

- `projection-mutant-registry` (HS-S0009, slice-mate) — supplies the projection `REGISTRY: &[Declared]`,
  the `Kind`/`FailureMode`/`expect` shape, the three (four) exactness meta-tests, the harness that drives
  one store through the enumeration, and `CheckpointOnlyStore`. Without it, every rule in this story is a
  CF-1 violation on arrival.

Transitively satisfied through that edge, and consumed here as-is — not re-decided, not re-landed:
`projection-suite-entry-point` (the enumeration, `ProjectionFixture`, the three harness files, the two
baseline rules), `projection-capability-skips` (the projection skip path), `owned-batch-port-shape` (the
trait, `Checkpoint`, `Authority`, `CommitError`), `projection-probe-conformance-feature` (`ProjectionProbe`),
`memory-projection-store` (the oracle).

**Unlocks:**

- `buffering-conformant-variant` — names this story in its own `depends_on`; the CF-5 variant must pass
  all seven of these rules for project AC-004 and initiative DoD 7's second half.
- `ps3-batch-shape-finding` — its finding is written from what those two shapes did against this rule set.
- `whole-gate-run-and-proof-artefact` — the proof artefact quotes the test `CheckpointOnlyStore` fails,
  read off the rule set this story completes.

Neither `reset-rules` nor `read-through-and-rebuild-rules` blocks on this story: both depend on
`projection-mutant-registry` directly and may proceed in parallel (`_storymap.md`, slices 4 – 5).

## Anchors (progressive disclosure)

Deferred depth, not optional depth. Each row says why the artifact is load-bearing and the moment to
open it; nothing below is pasted into this spec in bulk.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` | The normative text for every rule here: PS-1 (`:4742-4759`), PS-7 (`:4898-4910`), PS-8 (`:4911-4925`), PS-15 (`:5126-5155`), PS-21 (`:5277-5290`), PS-22 (`:5292-5322`), PS-23 (`:5317-5333`), and §4.11's *Rejects* column (`:5658-5685`) from which every mutant's provenance is taken. Where this spec and a clause disagree, the clause wins. | Before writing each rule body — read that rule's clause first, and read §4.11's row before naming its mutant | **AC-001** – **AC-008** |
| `spec/E2E-CASES.md` | The same obligations restated as observable behaviour: E2E-19 (`:502`), E2E-23 (`:599`), E2E-28 (`:725`). Useful when a clause reads ambiguously and the case does not. | When a rule's assertion set feels underdetermined by the clause | **AC-004**, **AC-005**, **AC-007** |
| `crates/happenstance-testkit/src/registry.rs` | **The mount point.** The single `for_each_projection_store_rule!` enumeration, the `impl AsyncFn() -> F` rule signature (`:45-49`) that makes **AC-004**'s two isolated stores possible, and the three emitters. A rule not named here is compiled by nothing. | First — before writing any rule, to see the exact signature and where the seven names go | **AC-009** |
| `crates/happenstance-testkit/src/contract.rs` | `Capability` and `RuleOutcome` reused unchanged (`:355-537`), `skip_line`'s single line shape (`:500-507`), the `MID_BATCH_FAULT` precedent that makes fault injection a capability rather than testkit machinery (`:196-211`), and the no-borrowing-GAT warning (`:97-111`). | Before writing **AC-001**'s `require!` gate, and before any fixture-shaped code | **AC-001**, **AC-010**, NF-004 |
| `crates/happenstance-testkit/src/suite.rs` | `require!` and `must!` as actually written — `require!` resolves `$crate::Fixture`, which is the trap EC-009 names — and the doc that says a rule uses `require!` only when its **entire** content needs the capability. | Immediately before gating **AC-001**'s rule; do not assume the macro applies to the projection fixture | **AC-001**, EC-009 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The `Declared` shape and `expect` pins (`:141-186`), the three exactness meta-tests (`:2734`, `:2754`, `:2889`), the no-pass-rate module doc (`:26-31`, `:203-206`) and the `wasm32` gate with its stated cost (`:33-46`). | When writing each registry row, and again when a meta-test goes red | **AC-008**, NF-007 |
| `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` | House style for a hostile store: how a defect is written, how provenance names a real adapter shape rather than a saboteur. Copy the shape, not the storage. | Before writing the first of the seven mutants | **AC-008** |
| `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` | `Subject` / `Verdict` / panic-origin machinery — how "failed at the rule's own assertion" is distinguished from "panicked inside the store". **AC-009**'s in-place-failure invariant is enforced here. | When a mutant fails for the wrong reason, or an `expect` pin will not match | **AC-008**, **AC-009** |
| `crates/happenstance-core/src/projection.rs` | The port under test: `ProjectionStore`, `Checkpoint`, `Authority`, `CommitError`, `ResetError`, and the invariant stated on the trait. Also the two-flavour naming that NF-002 constrains. | Continuously while writing rules; and immediately if a rule seems to need a signature that is not there (EC-006) | **AC-001** – **AC-007** |
| `crates/happenstance-core/src/event.rs` | `SequencePosition::FIRST` (`:244`) and `next() -> Option<Self>` with the reasoning for the `Option` (`:272-282`) — the only sanctioned way to obtain a position in a rule. | Before writing any position-carrying assertion | **AC-005**, **AC-006**, NF-001 |
| `xtask/src/lints.rs` | The three lints that gate this PR and what each actually catches: `no_clock` (`:231`), `changelog_names_every_rule` with `MIN_CHARS_PER_RULE = 120` (`:446`, `:525`), `no_position_literals` (`:628`) — including its own doc's warning that the grep is the *cheap second line*, not the enforcement. | When the changelog entry is written, and when a lint fires | **AC-010**, NF-001 |
| `xtask/src/spec_trace.rs` | `RULE_FILES` as a `[&str; 3]` (`:85-89`), the generated-region markers (`:200-201`) and the `†` semantics (`:1055`). The chokepoint four gate checks share. | Before assuming the rules module is swept; and last, when regenerating the region | **AC-010**, EC-005 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision behind **AC-008**: every rule has a registered wrong implementation, provenance names a real mistake, and **no pass rate is ever quoted**. | Before writing the registry rows and before writing the changelog | **AC-008** |
| `.kb/open-questions/ps-1-states-no-progress-obligation.md` | Why **AC-001** is deliberately scoped to PS-1's second conjunct, and why closing the gap here is out of bounds. | The moment writing **AC-001**'s rule makes the gap feel worth fixing | **AC-001**, EC-007 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The one declension policy. Prevents EC-002 from being "solved" by minting a second one for the projection fixture. | If the commit-fault capability turns out to be missing | **AC-001**, EC-002 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The briefs taken as given: Architecture Notes 1, 4, 5, 6, 7, 9; Testing brief rows **AC-003** / **AC-005** / **AC-010** and its tier definitions; UX brief AC-U08 – AC-U11, which are the whole of **AC-010**'s presentation constraints. | Before **AC-010**'s work, and whenever a decision feels like it is being made here that was already made there | **AC-008**, **AC-009**, **AC-010** |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off `surfaces: []` determination and its `N/A` anti-patterns — the reason **AC-010**'s composition invariants are sourced from the UX brief rather than invented. | Before writing anything that renders output | **AC-010** |
| `.bklg/from-contract-to-published-library/projection-store-freeze/projection-mutant-registry/spec.md` | The dependency's own contract: registry file path, meta-test names, `Kind`/`FailureMode` decisions, and the CF-5 deferral. This story consumes those choices rather than re-deciding them. | First, alongside the registry code, at the start of the slice | **AC-008** |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P2's goal, fear and journey (`:114-168`) — the intent every AC row above is framed from, including why "it compiles" is not evidence. | When an AC's phrasing needs to be checked against the user it claims to serve | **AC-001** – **AC-010** |
| `RUNBOOK.md` | Phase 6 in full (`:3848-3965`), the probe that found PS-7's `Busy`-forever defect (`:3893-3897`), and why a checkpoint-only suite is worthless without the probe (`:3882-3892`). | Before writing **AC-003**'s rule, and when justifying a mutant's provenance | **AC-003**, **AC-008** |
| `CHANGELOG.md` | The existing entry conventions CF-29's lint reads — one entry naming the rule and the defect, as prose. | When writing the seven entries, last | **AC-010** |

## Clarifications resolved during spec

1. **Ten acceptance criteria, exactly as the front half enumerated.** None added, none dropped. The
   seven rules take AC-001 – AC-007 one apiece; AC-008 carries the mutant obligation (project AC-003);
   AC-009 and AC-010 carry the two interaction-quality families as gated rows rather than prose.
2. **Project AC-010 is AC-003 here**, and it is carried by the rule's *second* half — the second batch
   opening and committing — because "rolls back" alone certifies a store that has permanently lost its
   only writer.
3. **The composition invariants are sourced from the UX brief, not from `_design.md`.** `_design.md`
   declares `surfaces: []` and records `N/A` under Anti-patterns; the signed-off content is the
   no-surface determination itself. Rather than treat AC-010 as vacuous, this spec binds it to the
   project's UX brief AC-U08 – AC-U11 and the reader table at `_decomposition.md:219`, which state what
   the text surface owes a human. This is a reading of the design, not a change to it.
4. **`require!` binds `Fixture`, not a projection trait.** Verified in `crates/happenstance-testkit/src/suite.rs:37-46`.
   Which gate the projection family uses is upstream's; EC-009 records the check and forbids a third path.
5. **`RULE_FILES` today lists three files and includes no projection module**
   (`xtask/src/spec_trace.rs:85-89`, `[&str; 3]`). Whether this story adds it depends on what
   `projection-suite-entry-point` landed; the PR boundary already permits the edit, and EC-005 requires
   the report to say which case obtained.
6. **The CF-29 threshold is 120 characters per rule named**, confirmed at `xtask/src/lints.rs:446`
   (`MIN_CHARS_PER_RULE`), and the match is a **whole-identifier** test (`:482-501`) — so a longer rule
   name's entry cannot discharge a shorter one's obligation. None of the seven names nests inside
   another, but the entries are still written one per rule.
7. **`SequencePosition::next()` returns `Option<Self>`**, not `Self` (`crates/happenstance-core/src/event.rs:272-282`).
   Rules handle the `None` arm explicitly; NF-001 records it so an implementer does not discover it as a
   compile error and reach for a literal.
8. **No test tier is claimed for anything this story cannot prove by running.** Every AC row above has a
   real command; the project's untestable ACs (AC-006, AC-007, AC-015) belong to other stories and are
   not smuggled in here (`_decomposition.md:893-900`).
