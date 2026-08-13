---
item: HS-S0011
stage: spec
created: 2026-08-12T13:46:05.705Z
updated: 2026-08-12T13:46:05.705Z
template_sig: 87bbf1d0
rendered_sig: 31ada6c2
---

# Spec — reset is one unit of work, scoped, refusable, and not commit-at-FIRST

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/reset-rules/spec.md` |
| Key brief — architecture | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` (Notes 4, 5, 6, 8, 9) |
| Key brief — testing | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` (Testing brief, **AC-011** row, `:781`) |
| Key brief — ux (text surface) | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` (AC-U07, AC-U08 – AC-U11) |
| Signed-off design | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — `surfaces: []`, `## Items` N/A |
| Story map row | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md:63` (slice `reset-and-rebuild-rules`, `:111`) |
| This story's discover | `.bklg/from-contract-to-published-library/projection-store-freeze/reset-rules/discover.md` |
| Roadmap pointer | `RUNBOOK.md:3904-3907` (phase 6, the `reset` bullet) |

## One-line PR slice

`reset` is provably one unit of work, scoped and refusable:
`reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`,
`refused_reset_changes_nothing`, `fresh_projection_has_no_checkpoint`, and
`reset_is_not_commit_at_first` rejecting `commit(empty, id, FIRST)` as a
substitute — with `TruncatingResetStore` and the commit-at-first substitute
registered as the stores that fail them.

## Executive summary

Five conformance rules land in the projection rule family, and every one of them
lands with a wrong store that it rejects.

The delta over what is already in the tree at the head of this slice: the
enumeration, the emitters, the fixture contract, the capability/skip machinery
and the projection mutant `REGISTRY` all exist by then (`projection-suite-entry-point`,
`projection-capability-skips`, `projection-mutant-registry`). The port has `reset`,
`Checkpoint` and `ResetError` (`owned-batch-port-shape`), and the probe has
`probe_delete_all` (`projection-probe-conformance-feature`). What does **not**
exist is any executable statement about `reset` at all — `crates/happenstance-core/src/projection.rs:110`
still returns `Option<SequencePosition>` from `checkpoint` and has no `reset`
method today, so *nothing* in the workspace can distinguish "returned to never
run" from "committed at the first position".

That distinction is the whole PR. `commit(empty, id, FIRST)` is the substitute
every deployment reaches for — **six scenarios out of six** reached for it and
all six got it wrong (`RUNBOOK.md:3904-3907`) — and it is wrong in the one way
nobody ever finds: the checkpoint reads back as `Some(1)`, the runner advances
past it, and event 1 is skipped permanently and silently (`spec/E2E-CASES.md:437-456`).
This PR converts that observation from a paragraph of prose into a rule that
fails **by name** against a registered mutant, and it does so alongside the four
rules that make `reset`'s other guarantees observable: one unit of work covering
rows and checkpoint together, scoping to a single `(store, ProjectionId)`,
refusability with no side effect, and a fresh projection reporting the
`Checkpoint::NeverRun` **variant** rather than a position.

It is delivered mounted: the five rule names go into the single
`for_each_projection_store_rule!` enumeration, so every harness — tokio, blocking
and `wasm32` — emits them in the same `cargo xtask ci` run, and their mutants go
into the projection `REGISTRY` under the three exactness meta-tests that already
police the event-store family.

## Context pack

The load-bearing decisions this story honours. Everything here is a decision
already taken elsewhere and binding here; the deeper artefacts sit behind the
anchors and are not repeated.

**1. `reset` is `commit`'s dual, and the port never learns what a read model is.**
The caller fills a batch with its own deletes and hands it over;
`reset(batch, id)` applies it and returns `id`'s checkpoint to `NeverRun` as one
unit (`spec/SPECIFICATION.md:5165-5170`, PS-16 at `:5172-5186`). The suite
expresses "clear the rows" through `ProjectionProbe::probe_delete_all`, which
exists for exactly this reason and is not optional garnish
(`spec/SPECIFICATION.md:5027`; architecture brief Note 4). No rule here may reach
into an adapter's storage to check a row.

**2. The assertion is on the `Checkpoint` variant, never on a position value.**
`Checkpoint` is a three-variant enum — `NeverRun` / `Live { through }` /
`Rebuilding { through }` — precisely because the tuple `(Option<SequencePosition>, bool)`
can spell `(None, true)`: *"authoritative, never run — which means nothing"*
(`spec/SPECIFICATION.md:4643-4658`). The natural way to write
`fresh_projection_has_no_checkpoint` and `reset_is_not_commit_at_first` is to
compare a checkpoint against `FIRST`, and that is **both** a CF-6 violation
(`spec/SPECIFICATION.md:7233-7245`) **and** the precise value the defective store
writes — a rule that compares positions cannot tell the two mutants apart and
both walk free. Every other position this slice compares is one the rule itself
supplied to `commit`.

**3. The scoping claim is only observable through a sibling id.** A rule that
resets `id`, asserts `id` is gone and passes is a rule `TruncatingResetStore` —
"a `reset` that clears every id" (`spec/SPECIFICATION.md:5680-5682`) — passes.
The testing brief specifies the sibling for this reason ("unaffected for a
sibling id", `_decomposition.md:781`). And the consequence of getting it wrong is
not a weak rule but a **red build**: `mutant_registry_is_exhaustive` rejects a row
whose `fails` list is empty (`crates/happenstance-testkit/tests/mutation_coverage/racers.rs:10-14`),
so a `TruncatingResetStore` that fails nothing breaks the registry. That is the
better outcome and it is stated here so nobody "fixes" it by trimming the row.

**4. "Changes nothing" is the operative half of refusal, not the error variant.**
PS-18 requires that a refusal leave both the read model and the checkpoint
unchanged and never be reported as success (`spec/SPECIFICATION.md:5200-5217`).
A rule that stops at `assert!(matches!(err, ResetError::Refused))` passes the
mirror-image defect: a correctly reported refusal taken *after* the rows have
gone. Both halves are observed through a **fresh handle**, the same way
`commit_is_atomic_with_the_read_model` observes its two (architecture brief
Note 6). Whether `ResetError::Refused` carries the store's stated reason is
`_design.md`'s recorded answer (AC-U07, `_decomposition.md:148-157`) and is
consumed here, not re-decided — the rule asserts that nothing changed, not why.

**5. Refusability is expressed through the fixture capability set, and a declined
capability is reported, never omitted.** The projection fixture needs "a way to
say *this store protects this id from reset*" (architecture brief Note 5,
`_decomposition.md:526-534`); which constants exist is `_design.md`'s call,
delivered by `projection-api-design-record` and wired by
`projection-capability-skips`. When a fixture declines it,
`refused_reset_changes_nothing` is still **emitted as a test** and returns
`RuleOutcome::Skipped` carrying the fixture's stated reason
(`crates/happenstance-testkit/src/contract.rs:473,500-507`), reusing that
vocabulary unchanged — one skip shape, one line format, no projection-local skip
type (AC-U08). The assertion is on the `RuleOutcome` **value**, not on stdout,
because `report` is a no-op on `wasm32` and libtest suppresses a passing test's
output anyway (AC-U09/AC-U11, `contract.rs:515-531`).

**6. PS-19 is `[FROZEN]` and its `MUST` is narrower than the rule assigned to
it — and this slice is where that gap becomes executable.** The clause is scoped
*after a successful reset*; `fresh_projection_has_no_checkpoint` asks about an id
never seen, and the store that satisfies one and fails the other is the natural
implementation rather than a contrivance (`spec/SPECIFICATION.md:5218-5249`;
`.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`). The repair is a
**new accepted decision atom** from `projection-decision-atoms` in slice 1 —
never a line edit to the clause (`CLAUDE.md`, "Changing a `[FROZEN]` clause
requires a new ADR, not an edit"). **Ordering is the check**: if that atom is not
accepted when this story starts, writing `fresh_projection_has_no_checkpoint` is
widening a frozen clause by test, and the story stops and reports rather than
proceeding. PS-17 and PS-20 are also `[FROZEN]` (`:5188-5190`, `:5251-5253`) and
are *implemented*, not amended — ADR-0007 already fixes checkpoints per
`(store, ProjectionId)` (`.kb/decisions/0007-projection-runner-decodes.md`).

**7. A mutant is only evidence when the defect is the only difference.** Every
wrong store in this repository's mutant binary delegates to one shared module of
correct steps and overrides exactly one of them, because a hand-written store
would differ in ways that make a red rule red for the wrong reason — which is the
failure `mutants_fail_exactly_their_declared_rules` exists to catch and the one it
would then be catching in the *instrument*
(`crates/happenstance-testkit/tests/mutation_coverage/correct.rs:1-18`;
RS-60-3 at `standards/rust/60-what-a-test-must-prove.md:169-176`). The two stores
this slice adds follow that shape, whatever module `projection-mutant-registry`
put the projection steps in. Provenance is never empty and no pass rate is ever
quoted over the mutant set (`.kb/decisions/0010-the-suite-must-prove-itself.md`;
CF-4 at `spec/SPECIFICATION.md:7209`).

**8. The persona slice.** The user here is the **adapter author**, and the
journey this story moves is initiative AC-04 — *"told when they are finished"* —
and AC-05 — *"told, with a reason, where a guarantee does not apply"*
(`.bklg/from-contract-to-published-library/initiative.md:316-322`). Concretely:
an adapter author who has implemented `reset` as a checkpoint-table truncate, or
as `commit(empty, id, FIRST)`, learns it from a **named failing test** in their
own harness rather than from a night-desk incident. The operator standing behind
that author is Norvant's 02:46 runbook procedure and Kestrel's `van_stock` /
`fgas_ledger` pair (`spec/E2E-CASES.md:414-497`, E2E-15 – E2E-18).

**9. What this story is not allowed to reach for.** No `[FROZEN]` clause text is
edited. No maturity marker is changed — that is
`unstable-projection-gate-and-clause-disposition`. No runner and no `Projection`
trait — PS-20's replay half is exercised *through the rules' own probe sequence*,
not by building the runner, which belongs to `typed-layer-and-alpha-release`
(`project.md` "Out of scope"). Nothing here makes a checkpoint boundary-scoped;
that would reopen ADR-0013's globally frozen visibility invariant
(`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`).

## Integration contract

- **Archetype**: `capability` — a user-observable slice through the whole stack.
  What the adapter author observes is five named tests in their own harness
  output, and a skip line with a reason where their store declines refusal.
- **Slice / milestone**: `reset-and-rebuild-rules`. Slice-mate:
  `read-through-and-rebuild-rules` (independent of this story; either order
  within the slice, `_storymap.md:111`). Both are implemented in one context and
  mounted as one integrated surface — the projection rule family after this slice
  is twelve of §4.11's seventeen adapter rules.
- **Mount point**: `crates/happenstance-testkit/src/registry.rs` — the
  `for_each_projection_store_rule!` enumeration landed by
  `projection-suite-entry-point`. This is the composition root for a conformance
  rule: a `pub async fn` in the projection rules module that is absent from that
  enumeration is a rule **nothing runs**, which is the failure the orphan
  meta-test exists for (`crates/happenstance-testkit/src/lib.rs:84-92`;
  `registry.rs:412-424`). A rule reachable only from a hand-written test is not
  mounted.
- **Wires into**:
  - the projection rules module and `ProjectionFixture` from
    `projection-suite-entry-point` (module path fixed there);
  - `Capability` and `RuleOutcome`, reused unchanged
    (`crates/happenstance-testkit/src/contract.rs:372,412,473,500-507`), and the
    declined-capability path from `projection-capability-skips`;
  - the projection mutant `REGISTRY: &[Declared]` and its three exactness
    meta-tests from `projection-mutant-registry`
    (`crates/happenstance-testkit/tests/mutation_coverage.rs:141-166` is the
    `Declared` shape being mirrored);
  - the shared correct-steps module the projection mutants delegate to
    (`crates/happenstance-testkit/tests/mutation_coverage/correct.rs` is the
    event-store precedent);
  - the port's `reset`, `Checkpoint`, `Authority` and `ResetError` from
    `owned-batch-port-shape`, and `ProjectionProbe::probe_write` /
    `probe_delete_all` / `probe_read` from `projection-probe-conformance-feature`
    (`spec/SPECIFICATION.md:4706-4715,4643-4682,5001-5015`);
  - the three emitters, unchanged, and the three harness files including
    `projection_conformance_wasm.rs` (`crates/happenstance-testkit/src/registry.rs`,
    `__emit_tokio` / `__emit_blocking` / `__emit_wasm`;
    `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` is the
    sibling's shape);
  - `CHANGELOG.md`, which CF-29's gate lint reads for every rule name
    (`xtask/src/lints.rs:525-531`).
- **Renders surfaces**: **none.** `_design.md` records `surfaces: []` and `##
  Items` N/A — this project ships nothing a person looks at. The one *text*
  surface it touches is the skip line — `SKIP <rule>: fixture declines
  <capability> — <reason>` (`_design.md` "Surfaces" item 2;
  `contract.rs:500-507`) — consumed unchanged: this story adds no second format.
- **Conformance rules**: `reset_clears_rows_and_checkpoint_together`,
  `reset_is_scoped_to_one_projection`, `reset_is_not_commit_at_first`,
  `refused_reset_changes_nothing`, `fresh_projection_has_no_checkpoint` — all
  five new, all five named by §4.11's table (`spec/SPECIFICATION.md:5658-5676`).
- **Clauses**: discharges PS-16, PS-17, PS-18, PS-19 and PS-20
  (`spec/SPECIFICATION.md:5172-5265`). **Amends none.** PS-17, PS-19 and PS-20
  are `[FROZEN]`; the PS-19 gap is repaired by the accepted atom from
  `projection-decision-atoms`, not by anything in this PR. PS-16 and PS-18 stay
  `[PROVISIONAL]` here; their disposition is
  `unstable-projection-gate-and-clause-disposition`'s.
- **Advances DoD scenario**: initiative **DoD 7** — *"The projection suite
  discriminates."* This story supplies two of the deliberately wrong
  implementations that make "discriminates" true beyond `CheckpointOnlyStore`,
  and it is a precondition for DoD 8's freeze verdict having anything to be a
  verdict about. Project DoD 1 (a mutant fails by name) gains two more names.

## PR boundary

**In this PR**

- Five projection conformance rules, added to the projection rules module and to
  the `for_each_projection_store_rule!` enumeration in the same change.
- `TruncatingResetStore` and the commit-at-first substitute store, in the
  testkit's own `tests/`, each built as a single override over the shared correct
  steps, each registered in the projection `REGISTRY` with non-empty provenance
  and per-rule pins.
- A mutant per remaining rule of this five, so the exhaustiveness meta-test stays
  green with the widened rule set (CF-1, CF-2).
- The sibling-id and fresh-handle observations that make the scoping and
  refusal claims real.
- `CHANGELOG.md` entries naming the defect each of the five rules detects
  (CF-29).
- This story's own backlog folder (`spec.md`, `_ledger.md`,
  `implementation-report.md`).
- **Wiring is in scope**: touching `registry.rs`'s enumeration and the three
  harness files to mount these five rules is this story's job, not scope drift.

**Explicitly not in this PR**

- **No edit to any `[FROZEN]` clause text and no maturity-marker change.**
  `spec/SPECIFICATION.md` is deliberately outside the write boundary below: the
  PS-16 – PS-20 clause rows already name these five rules, so nothing needs
  writing there, and the PS-19 repair is an accepted atom from slice 1.
  Marker disposition belongs to `unstable-projection-gate-and-clause-disposition`.
- The port itself. `reset`, `Checkpoint`, `ResetError` and the probe arrive from
  slice 2; if a signature is wrong, that is a finding reported against
  `owned-batch-port-shape`, not a fix made here.
- `batch_reads_reflect_pending_writes`, `rebuild_is_chunk_size_invariant`,
  `rebuilding_is_distinguishable_from_live` — the slice-mate's
  (`read-through-and-rebuild-rules`).
- The commit-side rules and their mutants —
  `commit-rollback-and-drop-rules`'s.
- The runner, the `Projection` trait, and the six integration-level rules §4.11
  moves to the e2e crate under CF-36 (`spec/SPECIFICATION.md:5694-5703`).
- Any new declension policy. DT-3's answer is consumed from `_design.md`;
  minting a second one here is the failure AC-006 names.

**Merge DoD (one line)**: the five rules are emitted by every harness including
`wasm32`, `TruncatingResetStore` and the commit-at-first substitute each fail
exactly the rule they declare and pass every other, and `cargo xtask ci` is green
with no `[FROZEN]` clause text changed.

```
crates/happenstance-testkit/src/**
crates/happenstance-testkit/tests/**
CHANGELOG.md
.bklg/from-contract-to-published-library/projection-store-freeze/reset-rules/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `reset_clears_rows_and_checkpoint_together` | Commit probe rows and a checkpoint; `begin` a batch, `probe_delete_all` into it, `reset(batch, id)`; through a **fresh handle** assert `probe_read` returns `None` for every written key *and* `checkpoint(id)` is `Checkpoint::NeverRun`. Both, or neither — the one-unit claim is the pairing, not either half. Paired with a failure-injecting variant: a `reset` that errors leaves both halves exactly as they were. | `spec/SPECIFICATION.md:5172-5186` (PS-16); `_decomposition.md:781` (Integration tier) |
| `reset_is_scoped_to_one_projection` | Two `ProjectionId`s in **one** store, both committed with probe rows under distinct keys; reset one; assert the reset id's rows and checkpoint are gone **and** the sibling's rows and checkpoint are unchanged. The sibling assertion is what makes the rule non-decorative; without it `TruncatingResetStore` fails nothing and the registry breaks. | `spec/SPECIFICATION.md:5188-5199` (PS-17), `:5681`; `spec/E2E-CASES.md:482-497` (E2E-18) |
| `refused_reset_changes_nothing` | Against a fixture configured to protect one id: assert the call returns `Err(ResetError::Refused)` — **not** `Ok` — and then, through a fresh handle, that the rows and the checkpoint are both exactly as they were before the attempt. The return-value check alone is insufficient and the spec names why. | `spec/SPECIFICATION.md:5200-5217` (PS-18); `_decomposition.md:148-157` (AC-U07) |
| …when the fixture declines the reset-refusal capability | The rule is still emitted as a test and returns `RuleOutcome::Skipped` carrying the fixture's stated reason; the assertion is made on the `RuleOutcome` **value**, not on stdout. Never `#[cfg]`-ed out of the binary. | `crates/happenstance-testkit/src/contract.rs:473,500-507,515-531`; `_decomposition.md:170-188` (AC-U09/AC-U10/AC-U11) |
| `fresh_projection_has_no_checkpoint` | For a `ProjectionId` the store has **never seen**, `checkpoint(id)` returns the `Checkpoint::NeverRun` variant. Asserted with a variant match; comparing against any `SequencePosition` is both the CF-6 violation and the exact value the mutant writes. | `spec/SPECIFICATION.md:4643-4658`, `:5660`; `discover.md:95-104` |
| `reset_is_not_commit_at_first` | Two ids in one store. On the first: commit rows and a checkpoint, then `reset`. On the second: `commit(empty_batch, id, SequencePosition::FIRST, Authority::Live)`. Assert the two checkpoints **differ** by variant (`NeverRun` vs `Live { .. }`). Then PS-20's half, without a runner: derive a resume point from each checkpoint under the port's own rule — strictly after a `Live` position, inclusive from the store's first position when `NeverRun` — and assert the event at the first position is included in the reset case and excluded in the commit-at-`FIRST` case. | `spec/SPECIFICATION.md:5218-5249` (PS-19), `:5251-5265` (PS-20); `spec/E2E-CASES.md:437-456` (E2E-16) |
| `TruncatingResetStore` | Correct in every step except one: its `reset` clears **every** id rather than the one it was given. Registered failing exactly `reset_is_scoped_to_one_projection`. Provenance: `SqliteProjectionStore::reset()` truncating the checkpoint table — cheap, obvious, and it destroys the append-only regulatory ledger sharing the file (Kestrel's `van_stock` resets several times a day; `fgas_ledger` must never). | `spec/SPECIFICATION.md:5680-5682`, `:5196-5199`; `_decomposition.md:684-688` |
| The commit-at-first substitute store | Correct in every step except one: `reset(batch, id)` is implemented as `commit(batch, id, SequencePosition::FIRST, Authority::Live)`. It compiles, it returns `Ok`, and the checkpoint moves — so every rule asserting only "the checkpoint changed" passes it. Registered failing exactly `reset_is_not_commit_at_first`. Whether it is a new store or an arm of `ValidatingCommitStore` is the implementer's call; the exactness meta-test constrains it more than taste does. Provenance: **all six deployment scenarios reached for this substitute and all six got it wrong.** | `RUNBOOK.md:3904-3907`; `_grounding.md:197-201`; `spec/SPECIFICATION.md:5244-5249`; `_decomposition.md:667-683` (Note 9) |
| A mutant for every rule of the five | CF-1 pairs each rule with at least one mutant; CF-2 registers them as data; CF-3 asserts both directions. The two named above cover two rules — the other three (`reset_clears_rows_and_checkpoint_together`, `refused_reset_changes_nothing`, `fresh_projection_has_no_checkpoint`) each need one, and §4.11 pre-specifies their shapes: the two-statement runbook procedure, a refusal reported as success, and a store answering `Live { through: FIRST }` for an id it has never seen. | `spec/SPECIFICATION.md:7161-7220`, `:5660-5676`; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| Mutant construction discipline | Each wrong store overrides exactly one step of the shared correct-steps module; it never reimplements the write path. A store that differs in two ways makes a red rule red for the wrong reason, and `mutants_fail_exactly_their_declared_rules` would then be catching a defect in the instrument. | `crates/happenstance-testkit/tests/mutation_coverage/correct.rs:1-18`; `standards/rust/60-what-a-test-must-prove.md:169-176` |
| Mounting | All five names appear in `for_each_projection_store_rule!`, so the tokio, blocking and `wasm32` harnesses each emit five more tests with no per-harness list to maintain. The orphan meta-test is what makes an unmounted rule a build failure rather than an omission. | `crates/happenstance-testkit/src/registry.rs` (`for_each_event_store_rule!`, `:94`; `no_orphan_rules`, `:412-424`); `crates/happenstance-testkit/src/lib.rs:84-92` |
| No literal positions, no clock | Every position these rules compare is one the rule itself passed to `commit`; the two checkpoint rules assert on the enum variant. No rule reads a clock or measures elapsed time. | CF-6 at `spec/SPECIFICATION.md:7233-7245`; CF-33 at `:8236`; `xtask/src/lints.rs` (the position lint) |
| Changelog | Each of the five rule names appears in `CHANGELOG.md` in an entry naming the **defect it detects** — the gate lint reads rule names out of the rule files and requires substance per rule, and it is explicitly the weakest check in the gate, so the prose has to carry it. | CF-29 at `spec/SPECIFICATION.md:8141-8150`; `xtask/src/lints.rs:511-531` |
| Precondition on the gate's file list | The three gate checks that police rules — spec-trace check 6, the CF-6 position lint and the CF-29 changelog lint — all read `RULE_FILES`, today a fixed `[&str; 3]` naming `suite.rs`, `model.rs` and `concurrency.rs` (`xtask/src/spec_trace.rs:85-89`). If the projection rules module is not in that list when this story starts, these five rules are invisible to all three. Widening it belongs to `projection-suite-entry-point`; finding it un-widened here is a **regression to report**, not to patch under cover of a rule-writing story. | `xtask/src/spec_trace.rs:85-89,623-631`; `_decomposition.md:709-727` (Note 10) |
| Precondition on the PS-19 repair | `fresh_projection_has_no_checkpoint` is only legitimate once the repair atom from `projection-decision-atoms` is accepted. If it is not, the story halts and reports; writing the rule anyway widens a `[FROZEN]` clause by test. If the slice-1 sweep returned "systematic", this story waits on the re-plan. | `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`; `discover.md:142-158`; `_storymap.md:107,111,126` |

## Data and migrations

**N/A — no persisted data and no migration.**

Three reasons, each specific rather than generic. First, nothing here ships a
storage adapter: the only stores this PR adds are wrong implementations living in
`crates/happenstance-testkit/tests/`, whose entire backing state is in-process for
the duration of one test, following `MemoryFixture`'s owned-handle-with-a-refcount
pattern (`crates/happenstance-testkit/src/fixtures.rs`). Second, nothing is
published, so there is no released schema and no consumer pinned to one — the
`ProjectionStore` port is behind the largest provisional block in the
specification and `happenstance-core` carries `publish = false` semantics for this
work until phase 12 (`project.md` "How this advances the initiative";
`CLAUDE.md` "Binding constraints" note 5). Third, `reset` is itself the operation
that replaces the migration people write by hand — the raw DDL against an
adapter's checkpoint table that E2E-15 rejects (`spec/E2E-CASES.md:414-433`) — so
introducing one here would be the defect this story exists to make unnecessary.

The only *state* transition this PR is about is the one the rules assert on: a
projection's checkpoint moving from `Live { through }` to `NeverRun` in the same
unit of work as the read model's rows going away. That is behaviour, checked by
five tests, not data with a shape on disk.

## Acceptance criteria

The persona is **P2, the adapter author** (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-153`),
and the criteria below are their fear crossing the full stack: *that the port
quietly assumed something their storage cannot provide, discovered late.* Their
journey's step 3 is the specific gap — they run the suite and get a green run
that is evidence about "one storage shape wearing four hats" (`:157-167`) — and
this story closes five rules' worth of it. Standing behind them is the operator
whose night desk actually paid: Norvant's 02:46:31 truncate and 02:46:33 pod
death, and Kestrel's `van_stock` (resets several times a day) beside
`fgas_ledger` (must never) — `spec/E2E-CASES.md:414-497`. Initiative **AC-04**
(*"told when they are finished"*) and **AC-05** (*"told, with a reason, where a
guarantee does not apply"*) are the two journeys moved
(`.bklg/from-contract-to-published-library/initiative.md:316-322`). Evidence is
secondary throughout; none of the four personas has been directly observed.

The tier vocabulary is the Testing brief's
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md`,
*Acceptance Criteria*): **Static** reads the tree without executing the code
under test; **Unit** is an in-process `#[test]`, including this suite's own
meta-tests over its registries; **Integration** is a conformance rule actually
driving a `ProjectionStore` through `begin` → probe-write → `commit`/`reset` →
probe-read; **E2E** is `cargo xtask ci` run whole. AC-011's row assigns this
story Integration **and** Unit, and both appear below.

Throughout: *the projection rules module* is whatever file
`projection-suite-entry-point` settled on under
`crates/happenstance-testkit/src/`, and *the projection mutant binary* is
`crates/happenstance-testkit/tests/projection_mutation_coverage.rs`, the mount
point `projection-mutant-registry` fixed. Naming a filename this story did not
choose is how a boundary fails for a reason that has nothing to do with scope.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author whose `reset` is the runbook procedure — two statements on two connections, the shape Norvant ran at 02:46:31 before the pod died at 02:46:33, leaving sixty-one events applied into an empty table and the runner *reporting healthy* (`spec/E2E-CASES.md:458-481`) — **WHEN** they run `projection_store_conformance!` against their own fixture, **THEN** `reset_clears_rows_and_checkpoint_together` fails **by name**, and a store that applies the caller's deletes and returns the checkpoint to `NeverRun` as one unit passes it: observed through a **fresh handle**, every key the rule wrote reads `None` *and* `checkpoint(id)` is `Checkpoint::NeverRun` — both, or neither, because the pairing is the claim and not either half. The interrupted half is asserted where the fixture can express a failing reset and is a reported skip where it cannot; it is never a silently absent assertion. | **Integration.** The rule in the projection rules module, emitted by all three projection harnesses; green against `MemoryProjectionStore`'s fixture, red against its registered mutant (the two-statement store) in the projection mutant binary. Clause: PS-16, `spec/SPECIFICATION.md:5172-5186`. Rows are observed only through `ProjectionProbe::probe_write` / `probe_delete_all` / `probe_read` — a rule that reads adapter storage directly fails review regardless of colour. |
| AC-002 | **GIVEN** Kestrel Cold Chain, whose one projection store holds `van_stock` (rebuilt several times a day across 138 devices) and `fgas_ledger` (a hash chain a regulator already holds), **WHEN** the author's `reset` is a `SqliteProjectionStore::reset()` that truncates the checkpoint table, **THEN** `reset_is_scoped_to_one_projection` fails by name — because the rule commits probe rows and a checkpoint under **two** `ProjectionId`s in one store, resets one, and asserts the sibling's rows *and* checkpoint are untouched as well as the target's being gone. The sibling assertion is the rule; without it the same code passes and the regulatory ledger is destroyed in the field. | **Integration.** Rule green against `MemoryProjectionStore`, red against `TruncatingResetStore`. Clause: PS-17 `[FROZEN]`, `spec/SPECIFICATION.md:5188-5199` — *implemented*, not amended; scope at `(store, ProjectionId)` is ADR-0007's decision applied to removal (`.kb/decisions/0007-projection-runner-decodes.md`). Case: E2E-18, `spec/E2E-CASES.md:482-497`. |
| AC-003 | **GIVEN** an adapter author whose store must protect one projection from reset (the `fgas_ledger` shape), **WHEN** they declare that capability and run the suite, **THEN** `refused_reset_changes_nothing` proves the refusal is real on **both** halves: the call returns `Err(ResetError::Refused)` and is never reported as success, **and** through a fresh handle the rows and the checkpoint are exactly as they were before the attempt. A store that reports the refusal correctly *after* deleting the rows fails, and so does one that reports success. | **Integration.** Rule green against a fixture that declares the refusal capability; red against both mutants of the pair (refusal-as-success, and refusal-after-the-fact). Clause: PS-18, `spec/SPECIFICATION.md:5200-5217`. Whether `ResetError::Refused` carries the store's reason is `_design.md`'s recorded answer (`_decomposition.md:148-157`, AC-U07) and is read, not re-decided — the rule asserts *nothing changed*, not why. |
| AC-004 | **GIVEN** an adapter author whose store cannot refuse a reset at all, **WHEN** they run the suite, **THEN** `refused_reset_changes_nothing` still appears in their output as a test and reports `RuleOutcome::Skipped` carrying **their fixture's own stated reason** — never `#[cfg]`-ed out, never a pass, never a second skip vocabulary invented for the projection family. This is initiative AC-05 in full for this story: a guarantee that does not apply says so, with a reason, in the same line shape every other declined capability uses. | **Unit** (the assertion is on the `RuleOutcome` **value**) **+ Integration** (the same rule inside a real harness run). `crates/happenstance-testkit/src/contract.rs:473,500-507`; the projection sibling of `capability_skips_are_reported`. Never asserted on stdout: `report` is a no-op on `wasm32` and libtest swallows a passing test's output (`contract.rs:515-531`; `_decomposition.md:170-188`, AC-U09/AC-U11). |
| AC-005 | **GIVEN** an operator asking the store about a projection it has **never seen**, **WHEN** the store answers, **THEN** `fresh_projection_has_no_checkpoint` requires the `Checkpoint::NeverRun` **variant** — and a store answering `Live { through: FIRST }` for that id fails by name. The assertion is a variant match; comparing against any `SequencePosition` is simultaneously a CF-6 violation and the exact value the defective store writes, so a rule written that way cannot tell the two mutants apart and both walk free. | **Integration** + **Static (ordering gate)**. `spec/SPECIFICATION.md:4643-4658` (why `Checkpoint` is three variants rather than `(Option<SequencePosition>, bool)`), `:5660`; CF-6 at `:7233-7245`. **Precondition, checked before the rule is written**: the PS-19 repair atom from `projection-decision-atoms` is `status: accepted` under `.kb/decisions/` and `redkiln validate --kb` is green on that tree. Absent it, the story halts and reports (EC-001) — writing the rule anyway widens a `[FROZEN]` clause by test. |
| AC-006 | **GIVEN** the operator at 03:18 who "resets properly" by writing `commit(empty_batch, id, SequencePosition::FIRST)` — the substitute **all six deployment scenarios reached for and all six got wrong** (`RUNBOOK.md:3904-3907`) — **WHEN** the runner resumes, **THEN** `reset_is_not_commit_at_first` rejects it: the rule resets one id and commits-at-`FIRST` on a sibling id in the same store, asserts the two checkpoints differ **by variant** (`NeverRun` vs `Live { .. }`), and then derives a resume point from each under the port's own rule — strictly after a `Live` position, inclusive from the store's first position when `NeverRun` — asserting the event at the first position is included in the reset case and **excluded** in the commit-at-`FIRST` case. Event 1 skipped permanently and silently is the defect; the variant check alone would not see the resume consequence, and the resume check alone would not see the state. | **Integration** + **Unit** (`commit(empty, id, FIRST)` as a registered mutant's declared failure mode, per the Testing brief's AC-011 row — *"what turns RUNBOOK's observation into an enforced rejection instead of a warning in prose"*). Clauses PS-19 `:5218-5249` and PS-20 `:5251-5265`; case E2E-16, `spec/E2E-CASES.md:437-456`. No runner is built: the resume point is derived inside the rule from the checkpoint the port returned. |
| AC-007 | **GIVEN** an adapter author who wants to know whether `reset_is_scoped_to_one_projection` can actually fail, **WHEN** they open the projection mutant binary, **THEN** `TruncatingResetStore` is registered there with a `fails` list naming exactly that rule and a **non-empty provenance** describing the real mistake it models — a `SqliteProjectionStore::reset()` truncating the checkpoint table, cheap and obvious, which destroys the append-only regulatory ledger sharing the file. It is correct in every other step, so the rule that goes red goes red for the defect and not for the store. | **Unit.** The three projection exactness meta-tests from `projection-mutant-registry`: every rule has a mutant (CF-1), the registry is exhaustive (CF-2), and each mutant fails **exactly** its declared rules and passes or accountably skips every other (CF-3) — `spec/SPECIFICATION.md:7161-7220`; shape at `crates/happenstance-testkit/tests/mutation_coverage.rs:141-166`; empty-`fails` rejection at `crates/happenstance-testkit/tests/mutation_coverage/racers.rs:10-14`. Provenance non-empty per CF-4 (`:7209`). |
| AC-008 | **GIVEN** that a rule nobody has checked is a rule that proves nothing, **WHEN** this story merges, **THEN** every one of its five rules has at least one registered wrong store that fails it — the commit-at-first substitute (declaring `reset_is_not_commit_at_first`, provenance: six scenarios out of six), `TruncatingResetStore`, and one each for the remaining three in the shapes §4.11 pre-specifies (the two-statement runbook procedure; a refusal reported as success; a store answering `Live { through: FIRST }` for an id it has never seen) — **and** each wrong store is a single overridden step over the shared correct-steps module, never a hand-written store. A store that differs in two ways makes a red rule red for the wrong reason, and the exactness meta-test would then be catching a defect in the instrument. | **Unit.** CF-1/CF-2/CF-3 over the widened rule set; construction discipline per RS-60-3 (`standards/rust/60-what-a-test-must-prove.md:169-176`) and the event-store precedent `crates/happenstance-testkit/tests/mutation_coverage/correct.rs:1-18`. §4.11's pre-specified shapes: `spec/SPECIFICATION.md:5660-5676`. **No pass rate is computed or quoted over the mutant set**, here or in the implementation report (`.kb/decisions/0010-the-suite-must-prove-itself.md`). |
| AC-009 | **GIVEN** initiative AC-04 — the adapter author is *told when they are finished*, "with no rule silently absent from the run" — **WHEN** they invoke the one entry point, **THEN** all five rule names are in the single `for_each_projection_store_rule!` enumeration, so the tokio, blocking and `wasm32` harnesses each emit five more tests with no per-harness list to maintain, and a rule present in the module but absent from the enumeration is a **build failure** rather than an omission. The edge developer's runtime is not a separately maintained subset: `projection_conformance_wasm.rs` type-checks the same five in the same `cargo xtask ci` run. | **Unit** (the projection sibling of `no_orphan_rules`, two directions — `crates/happenstance-testkit/src/registry.rs:412-424`, `crates/happenstance-testkit/src/lib.rs:84-92`) **+ E2E** (`cargo xtask ci`'s mandatory wasm32 conformance-harness step, `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown`, over `projection_conformance_wasm.rs` — the sibling shape is `crates/happenstance-testkit/tests/memory_conformance_wasm.rs`). |
| AC-010 | **GIVEN** an evaluator (initiative AC-08) reading the release notes to decide whether the suite is worth trusting, **WHEN** they look up any of these five rules, **THEN** `CHANGELOG.md` names **the defect it detects** rather than listing the rule, the gate's own rule lints see all five (they read `RULE_FILES`, so a projection rules module absent from that list makes them pass vacuously — reported, never patched here), and `git diff` over `spec/SPECIFICATION.md` is **empty**: no `[FROZEN]` clause text changed and no maturity marker moved to make anything pass. | **Static.** `cargo xtask spec-trace` (check 6 over `RULE_FILES`, `xtask/src/spec_trace.rs:85-89,623-631`); the CF-29 changelog lint, explicitly the weakest check in the gate and therefore carried by the prose (`xtask/src/lints.rs:511-531`; `spec/SPECIFICATION.md:8141-8150`); the CF-6 position lint over the same file set; `git diff --stat main...HEAD -- spec/` empty. |

**Coverage of the traced project ACs.** **AC-011** (*"`reset` clears rows and
checkpoint in one unit of work, is scoped to one `(store, ProjectionId)`, and is
refusable; a rule rejects `commit(empty, id, FIRST)` as a substitute, which
silently skips event 1"*) is covered clause-for-clause: one unit of work by
AC-001, scoped by AC-002, refusable by AC-003 and AC-004, the substitute
rejected by AC-006 — with AC-005 supplying the `NeverRun` variant without which
AC-006's assertion cannot be written honestly. **AC-003** (*"every projection
rule has a registered wrong implementation that fails it, with stated provenance
… no pass rate is quoted"*) is covered by AC-007 for the named store and AC-008
for the remaining four, both under the exactness meta-tests. AC-009 and AC-010
carry the two obligations every rule-bearing story in this project owes — being
mounted in the one enumeration, and being visible to the gate that polices rules.

## Interaction quality

**This story renders no surface, and that is a signed-off determination rather
than an omission.** `_design.md` records `surfaces: []`, `N/A — no user-facing
surface` under `## Items`, `## Signatures`, `## The states the API must express`
and `## Anti-patterns`, and its sign-off block approves *the no-surface
determination itself* (`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:48-54,84-101`).
The project's two non-visual surfaces are the type surface in
`crates/happenstance-core/src/projection.rs` (landed by `owned-batch-port-shape`,
outside this PR) and **the one text line a conformance run prints for a declined
capability** (`_design.md:33-38`) — which this story *consumes unchanged* and
does not extend.

So the COMPOSITION family is discharged by `_design.md` rather than re-decided
here. Every invariant that applies is an `AC-###` row in the table above, never a
bullet in this section: `redkiln verify` extracts ACs by matching a leading
`| AC-001 |` cell, so an invariant written here as prose would get no ledger row,
be gated by nothing, and be tested by nothing. This section says only *which id
carries which invariant, and how it is verified.*

**STATE family — translated into this medium (a conformance run's output and the
store it observes).**

| Invariant | Its form here | Carried by |
| --- | --- | --- |
| **In place, not a context jump** | A declined capability is reported *in the rule's own line, in the run the author already started* — never a separate mode, a documentation page, or a rule that must be enabled to be seen. | **AC-004** |
| **Non-occlusion** | A skip never hides the rule: it is emitted as a test and is distinguishable from a pass by value, not by absence. Symmetrically, a red rule names the defect rather than the store, which is what one-step mutant construction buys. | **AC-004**, **AC-008** |
| **Preserved state / selection** | The sibling projection survives the reset, and a refused reset leaves rows *and* checkpoint exactly as they were — the two places where "the operation acted on more than the author selected" is the whole bug. | **AC-002**, **AC-003** |
| **Reversibility** | `reset` is the port's own undo, and the rules assert it is complete: after it, `checkpoint(id)` is `NeverRun`, so a rebuild starts from the store's first position rather than one past it. The irreversible failure — event 1 skipped permanently and silently — is the thing rejected. | **AC-001**, **AC-006** |
| **Reachability without prior knowledge** | The suite's equivalent of keyboard reachability: one line, `projection_store_conformance!(fixture)`, reaches all five rules on every target, with no per-harness list and no `wasm32`-only subset to discover. | **AC-009** |

**COMPOSITION family — discharged by the signed-off design, with the three
obligations that survive translation, each already bound to an AC row.**

- **Presentation exists at all.** The skip line is composed, not bare markup:
  `SKIP {rule}: fixture declines \`{capability}\` — {reason}`, with the reason
  supplied by the fixture rather than written by the testkit
  (`crates/happenstance-testkit/src/contract.rs:500-507`; `_design.md:33-38`).
  A `RuleOutcome::Skipped` that reaches a human as silence fails **AC-004**.
- **Transience — persistent, not revealed on demand.** The five rules are
  persistent chrome of every run: enumerated once, emitted by all three
  harnesses, never behind a flag or a feature. **AC-009**.
- **Density budget, with its real numbers.** Five rule names added to one
  enumeration; the projection family reaches **twelve of §4.11's seventeen**
  adapter rules after this slice; **one** line of output per skipped rule and
  **one** changelog entry per rule naming its defect; **one** overridden step per
  wrong store. The budget's other side is the honest limitation already
  documented rather than re-litigated: `report` is a no-op on `wasm32`, which is
  exactly why assertions are on `RuleOutcome` values (`contract.rs:515-531`).
  **AC-004**, **AC-008**, **AC-010**.

**The design's named anti-patterns, in this medium.** `_design.md` records `N/A`
under `## Anti-patterns` for lack of a surface, so these come from the binding
briefs and the specification rather than being invented here — each is a
plausible wrong implementation an AC rejects: a rule that asserts a literal
position (**AC-005**, **AC-010**); a scoping rule run against one projection
(**AC-002**); a refusal rule that stops at the error variant (**AC-003**); a
declined capability that vanishes from the binary (**AC-004**); a mutant that
differs in two ways (**AC-008**); a rule that exists but is not enumerated
(**AC-009**); a `[FROZEN]` clause line-edited to make a rule legitimate
(**AC-005**, **AC-010**).

## Error conditions

| id | Condition | Required handling |
| --- | --- | --- |
| **EC-001** | The PS-19 repair atom from `projection-decision-atoms` is not `status: accepted` when this story starts. | **Halt and report at the story boundary.** `fresh_projection_has_no_checkpoint` asks about an id the store has never seen; PS-19's `MUST` is scoped to the state *after* a successful reset. Writing the rule anyway widens a `[FROZEN]` clause by test. Do not line-edit the clause and do not drop the rule quietly — both are decisions, and they are `unstable-projection-gate-and-clause-disposition`'s (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`). |
| **EC-002** | `xtask/src/spec_trace.rs`'s `RULE_FILES` still names only `suite.rs`, `model.rs` and `concurrency.rs` when this story starts. | Then spec-trace check 6, the CF-6 position lint and the CF-29 changelog lint are all blind to these five rules and pass **vacuously**. Report it as a regression against `projection-suite-entry-point`; do not widen the array under cover of a rule-writing story (`_decomposition.md:709-727`, Note 10). |
| **EC-003** | The fixture declines the reset-refusal capability. | Not an error: `refused_reset_changes_nothing` is emitted and returns `RuleOutcome::Skipped` with the fixture's stated reason. The mutant registry's undeclared-rules direction accepts `Skipped` **only** where the fixture itself declines that capability — a skip anywhere else is a hole in the map and never a pass. |
| **EC-004** | `TruncatingResetStore` is registered and fails nothing, because the scoping rule has no sibling assertion. | The registry breaks (`mutant_registry_is_exhaustive` rejects an empty `fails` list, `crates/happenstance-testkit/tests/mutation_coverage/racers.rs:10-14`). **Fix the rule, not the row.** Trimming the registry entry to make the build green converts a caught weak rule into a shipped one. |
| **EC-005** | A wrong store fails a rule it did not declare. | The defect is over-broad — the store differs in more than one step. Narrow the override; never widen `fails` to match observed behaviour, which is the registry-generated-from-outcomes failure CF-2 exists to forbid. |
| **EC-006** | The port's `reset` / `Checkpoint` / `ResetError` signatures cannot express one of these five rules. | Report as a finding against `owned-batch-port-shape`. Do **not** patch the port here: the PR boundary excludes `crates/happenstance-core/**` precisely so a signature disagreement surfaces as a port decision rather than as an edit made by a test author. |
| **EC-007** | `ProjectionProbe::probe_delete_all` cannot express "clear this projection's rows" for some store shape. | Report against `projection-probe-conformance-feature`. A rule must never reach into an adapter's storage to check or clear a row — that is the seam PS-11 exists for, and bypassing it makes the rule unrunnable for every other adapter. |
| **EC-008** | The new rules do not type-check for `wasm32`. | Fix the rules, not the harness. The mutant binary is `cfg(not(target_arch = "wasm32"))` by design (no unwinder, so `catch_unwind` cannot work), but the **rules** are cross-target and AC-009 is exactly the claim that they are. Nothing here may read a clock, spawn, or depend on `Send` (CF-33, `spec/SPECIFICATION.md:8236`; ADR-0001). |
| **EC-009** | A refusal is observed to leave the rows deleted and the checkpoint intact. | That is the mirror-image defect `refused_reset_changes_nothing` exists to catch, and it is a **real finding** about whichever store showed it — record it, do not weaken the rule to the error-variant check. |
| **EC-010** | `ps-clause-pairing-sweep` returned **"systematic"**. | This story waits on the re-plan (`_storymap.md`, *What would reshape this map*). A systematic pairing defect is larger than one atom, and proceeding would write five rules on top of an unrepaired frozen-clause set. |
| **EC-011** | The slice-mate (`read-through-and-rebuild-rules`) and this story both edit the enumeration and `CHANGELOG.md`. | Expected, and a textual conflict rather than a design one: both are implemented in one context and mounted as one surface. Resolve by union — never by dropping a rule name from the enumeration, which the orphan meta-test would then catch as a build failure. |

## Non-functional

| id | Requirement | Why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **No new dependency, no new feature flag, no `Cargo.toml` change.** The five rules and two stores use what slices 2 – 4 already landed. | The feature powerset `cargo hack` walks is unchanged, so this story adds nothing to the gate's combinatorial cost; a new feature here would be a scope error, and the publication project grades feature-surface growth. |
| **NF-002** | **Cross-target by construction.** No rule reads a clock, measures elapsed time, spawns, sleeps or requires `Send`. | CF-33 (`spec/SPECIFICATION.md:8236`) and ADR-0001's no-`Send`-bound rule. This is what makes AC-009's `wasm32` claim cheap rather than a second implementation; it is checked by the mandatory wasm32 harness step, not by inspection. |
| **NF-003** | **Determinism and isolation.** One fixture instance is one isolated backing store; each `connect()` is one handle onto it. Rules that need a fresh handle take one; no rule depends on test ordering or on a shared global. | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-testkit/src/fixtures.rs`'s `MemoryFixture` is the reference shape. Two ids in one store (AC-002, AC-006) is a **store-scoped** claim and must not be split across fixtures, which would prove nothing about scoping. |
| **NF-004** | **No pass rate, anywhere.** Neither the changelog, the implementation report nor the mutant registry's docs quote a fraction over the mutant set. | The denominator is an author's choice, so a fraction reports how representative the author was while reading as though it reported how good the suite is (`.kb/decisions/0010-the-suite-must-prove-itself.md`; CF-4). What is reported is which defects are covered and which axes are not. |
| **NF-005** | **Gate cost stays negligible.** Five rules × three harnesses × the registered fixtures, all in-process and in-memory. | Named so that a *measurable* slowdown is treated as a signal — a projection rule that costs real time is doing I/O or waiting, which NF-002 already forbids. |
| **NF-006** | **Rustdoc obligations hold on every new public item and rule.** Each rule's doc comment states the defect it rejects, in the house style the existing families already use. | `standards/rust/70-rustdoc-obligations.md`; `cargo xtask ci`'s docs step with `-D warnings`. The doc comment is also what CF-29's changelog prose is derived from, which keeps the two from drifting. |

## Implementation notes (non-prescriptive)

Sequencing that reduces rework, none of it binding.

- **Check the two preconditions before writing a line** — the PS-19 repair atom's
  status (EC-001) and `RULE_FILES` (EC-002). Both are cheap to check and
  expensive to discover after five rules exist.
- **Write `reset_is_not_commit_at_first` first.** It is the rule this story
  exists for, it is the one whose assertion is easiest to write wrongly, and its
  shape (two ids, variant comparison, resume-point derivation) is a superset of
  what AC-002 and AC-005 need — the other four tend to fall out of it.
- **Write each rule against its mutant immediately**, not after all five. A rule
  that has never been red is a rule nobody has checked, and the exactness
  meta-tests are cheaper to satisfy one store at a time than five at once.
- **Derive the resume point in one small helper** shared by the rule's two arms,
  so the `NeverRun`-is-inclusive / `Live`-is-exclusive rule is stated once. That
  helper is *not* the runner and must not grow toward one — the runner is
  `typed-layer-and-alpha-release`'s.
- **Reuse the correct-steps module rather than copying `MemoryProjectionStore`.**
  Whatever module `projection-mutant-registry` put the projection steps in is the
  delegation target; a copied store is a second implementation that drifts.
- **The commit-at-first substitute may be a new store or an arm of
  `ValidatingCommitStore`.** Either is legal; let the exactness meta-test decide
  it — if an arm makes the store fail two rules, it wanted to be its own store.
- **Write the changelog entries as you write each rule**, phrased as the defect
  detected. Writing five at the end produces five listings, which is exactly what
  CF-29's length check is a proxy against.
- **Keep the sibling-id keys distinct and obviously so** (`van_stock` /
  `fgas_ledger` read better than `a` / `b` and cost nothing) — the rule's failure
  message is read by someone who has never seen the file.
- **Report, do not absorb.** A wrong port signature, a missing probe method, an
  un-widened `RULE_FILES` and a systematic sweep verdict are each a boundary
  report; this story's PR boundary is drawn so that absorbing one would show up
  as an out-of-boundary file.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s Testing brief — the **AC-011** row (Integration
+ Unit) and the **AC-003** row (Unit + Static), plus the brief's merge-gate
command block, whose two grains are reproduced faithfully: story grain during
implementation, full `cargo xtask ci` at the project boundary.

| tier | command / path | proves |
| --- | --- | --- |
| **Static** | `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` | The five rules and two stores compile clean under the house lint set. Non-regression; evidence for no AC on its own. |
| **Static** | `cargo xtask spec-trace` | Check 6 over `RULE_FILES` resolves each of the five rule names and their PS-16 – PS-20 citations; **no clause text or maturity marker moved**. **AC-010** (and the EC-002 precondition, which this step is also how you detect). |
| **Static** | `cargo xtask ci`'s CF-29 changelog lint (`xtask/src/lints.rs:511-531`) | Every one of the five rule names appears in `CHANGELOG.md` in an entry with substance, naming the defect it detects. **AC-010.** |
| **Static** | The CF-6 position lint over the rule files | No rule asserts a literal position value; the two checkpoint rules assert variants. **AC-005**, **AC-010.** |
| **Static** | `git diff --stat main...HEAD -- spec/ crates/happenstance-core/` (empty) | The rules were made to pass by writing rules, not by editing the contract or the specification. **AC-010**, and the EC-006 boundary. |
| **Unit** | `cargo test -p happenstance-testkit` → the projection orphan meta-test (sibling of `no_orphan_rules`, `crates/happenstance-testkit/src/registry.rs:412-424`) | All five rules are in the single enumeration; a rule present in the module and absent from it is a build/test failure. **AC-009.** |
| **Unit** | The three projection exactness meta-tests in `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` (CF-1/CF-2/CF-3) | Every rule has a registered mutant; the registry is exhaustive and internally consistent; each mutant fails **exactly** its declared rules and passes or accountably skips the rest; provenance non-empty (CF-4). **AC-007**, **AC-008.** |
| **Unit** | The projection sibling of `capability_skips_are_reported` — assertions on `RuleOutcome` values (`crates/happenstance-testkit/src/contract.rs:473,500-507`) | A declined refusal capability yields `Skipped` with the fixture's reason, not a pass and not an absence. **AC-004.** |
| **Integration** | `projection_store_conformance!` driving `MemoryProjectionStore`'s fixture through `begin` → `probe_write` → `commit` → `probe_delete_all` → `reset` → fresh handle → `probe_read` + `checkpoint` | The five rules pass against a correct store: one unit of work, scoping, refusal without side effect, `NeverRun` for an unseen id, and the commit-at-`FIRST` rejection. **AC-001 – AC-003, AC-005, AC-006.** |
| **Integration** | The same rules driven against `TruncatingResetStore` and the commit-at-first substitute | Each fails by name — the discrimination claim, observed by execution rather than inspection. **AC-002**, **AC-006**, **AC-007**, **AC-008.** |
| **E2E** | `cargo xtask ci` (project boundary grain) | The mandatory `wasm32` conformance-harness step type-checks `projection_conformance_wasm.rs` with the five new rules **in the same run** as the host harnesses; `cargo hack`, `cargo deny` and the doc builds stay green over an unchanged feature surface. **AC-009**, **NF-001.** |
| **E2E** | `cargo xtask affected --base main` + `cargo xtask ci --fast` (story grain) | The bar a non-terminal story meets during implementation. **Not** evidence for AC-009 or AC-010 — `--fast` omits exactly the wasm32 steps and `spec-trace` those two rest on (`CLAUDE.md`, *Commands*). |
| **Static** | `redkiln validate --kb` | The PS-19 repair atom is accepted on this tree and no accepted atom's body moved. The **precondition** for AC-005, recorded with the sha it was run at. |

**Not run here, and why.** No new gate step is added: AC-009 costs a harness file
that an existing mandatory step already picks up, which is the point of citing
that pattern rather than inventing a wasm-specific job (`_decomposition.md:786`).
The whole-workspace proof artefact on a clean checkout is
`whole-gate-run-and-proof-artefact`'s, and a green gate is a **precondition** for
reading DoD 1 and DoD 7, never a substitute for them (`project.md` DoD 8).

## Risks and coupling (PR-scoped)

| Risk | Shape | Mitigation, in this PR |
| --- | --- | --- |
| **A frozen clause widened by test** | PS-19's `MUST` is narrower than the rule §4.11 assigns it, and writing the rule is the cheapest way to paper over that. | **EC-001** makes the repair atom a hard precondition and a halt-and-report otherwise; **AC-010** asserts `spec/` is untouched; the playbook names the repair-versus-gap test. |
| **A rule that no adapter can fail** | The scoping and refusal rules both have a shorter form that passes everything. | The sibling-id assertion (**AC-002**) and the fresh-handle both-halves assertion (**AC-003**) are written into the criteria, and their mutants exist to make the shorter form red. **EC-004** forbids fixing that by trimming the registry. |
| **A rule that compares positions** | The natural spelling of two of these five rules compares a checkpoint against `FIRST` — which is both a CF-6 violation and the exact value the defect writes. | Variant assertions in **AC-005** and **AC-006**; the CF-6 lint in the gate; every other position compared is one the rule itself supplied to `commit`. |
| **The gate cannot see the rules** | `RULE_FILES` is a fixed `[&str; 3]`; three gate checks read it. | **EC-002**: report against `projection-suite-entry-point`. Detected by running `cargo xtask spec-trace` early rather than at the end. |
| **Upstream port shape** | `reset`, `Checkpoint`, `ResetError` and the probe all arrive from slice 2; a signature that cannot express a rule blocks the story. | **EC-006/EC-007** route both to the owning story; the PR boundary excludes `crates/happenstance-core/**` so the coupling cannot be resolved silently. |
| **Mutant over-breadth** | A wrong store that differs in two ways makes a red rule red for the wrong reason — and the exactness meta-test then catches a defect in the instrument. | **AC-008**'s one-overridden-step discipline (RS-60-3) plus **EC-005**: narrow the override, never widen `fails`. |
| **Slice-mate collision** | `read-through-and-rebuild-rules` edits the same enumeration and the same changelog in the same slice. | **EC-011**: resolve by union; the orphan meta-test turns a dropped name into a build failure rather than a silent gap. Both stories are implemented in one context by design. |
| **`wasm32` divergence** | The mutant binary is host-only; it would be easy to let the rules drift host-only with it. | **AC-009** + **NF-002**: the rules are cross-target and the mandatory harness step checks it every run. **EC-008** forbids fixing a wasm failure by narrowing the harness. |
| **Scope creep into the runner** | PS-20's replay half invites building a runner to observe it. | **AC-006** derives the resume point inside the rule; the PR boundary and `project.md`'s *Out of scope* both exclude the runner and the `Projection` trait. |
| **A second declension policy** | It is easy to invent a projection-local skip type while writing the refusal rule. | **AC-004** consumes `Capability` / `RuleOutcome` and the `_design.md` capability set unchanged; a new public type here means AC-A06 was violated. |

## Dependencies

**Blocks on**

- **`projection-mutant-registry`** (slice 4) — supplies the projection
  `REGISTRY: &[Declared]`, the projection mutant binary, the three exactness
  meta-tests and the shared correct-steps module the two new wrong stores
  override exactly one step of. Without it, AC-007 and AC-008 have nowhere to
  register anything and every rule here would be unchecked.
- **`projection-capability-skips`** (slice 3) — supplies the declined-capability
  machinery and the recorded projection capability set. `refused_reset_changes_nothing`
  is the reason that edge exists: AC-003 needs a fixture that can say *this store
  protects this id from reset*, and AC-004 needs the declined path to be a
  reported skip rather than an omission.
- Transitively (not restated as edges): `projection-suite-entry-point` for the
  enumeration, the fixture and the harnesses; `owned-batch-port-shape` and
  `projection-probe-conformance-feature` for `reset` / `Checkpoint` /
  `ResetError` / `probe_delete_all`; `projection-decision-atoms` for the PS-19
  repair atom, which EC-001 turns into a checked precondition rather than an
  assumption.

**Unlocks**

- **`buffering-conformant-variant`** (slice 6) — names this story in its
  `depends_on` alongside the other two rule stories: the CF-5 conformant variant
  must pass *the whole suite*, and these five rules are part of it.
- **`ps3-batch-shape-finding`**, transitively — whether the two batch shapes
  disagreed is a question about the rule set this story helps complete.
- **`whole-gate-run-and-proof-artefact`** (slice 8) — the proof artefact names
  the tests the wrong stores fail; two of those names come from here.
- **`unstable-projection-gate-and-clause-disposition`** (slice 8) — PS-16 and
  PS-18's maturity disposition is decided against evidence that only exists once
  these rules run.

**Slice-mate, not a dependency**: `read-through-and-rebuild-rules` — independent
of this story, either order within `reset-and-rebuild-rules`
(`_storymap.md:111`), implemented in the same context and mounted as one surface.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Open each at the moment named;
do not read the corpus. Every path below was checked to exist.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` | The normative text these five rules discharge, and the only place their exact obligations are stated: PS-16 – PS-20 at `:5172-5265`, §4.11's rule table and its named rejected implementations at `:5658-5703`, `Checkpoint`'s three variants at `:4643-4658`, CF-1 – CF-6 at `:7161-7250`, CF-29 at `:8141-8150`. Where this spec and a clause disagree, the clause wins. | Open the specific range as each rule is written — never the file whole. | AC-001, AC-002, AC-003, AC-005, AC-006 |
| `spec/E2E-CASES.md` | The four cases these rules are the executable form of: E2E-15 `:414-433`, E2E-16 `:437-456` (event 1 skipped permanently and silently), E2E-17 `:458-481` (the 02:46:31 truncate), E2E-18 `:482-497` (`van_stock` vs `fgas_ledger`). They carry the *observable behaviour* phrasing each criterion is derived from. | Before writing each rule's assertion, to check the rule observes what the case says a user observes. | AC-001, AC-002, AC-006 |
| `crates/happenstance-testkit/src/contract.rs` | `RuleOutcome` at `:473`, `skip_line` at `:500-507` and `report`'s two honest per-target limitations at `:515-531` — the vocabulary AC-004 reuses **unchanged**, and the reason the assertion is on the value rather than on stdout. | While writing `refused_reset_changes_nothing`'s declined-capability arm. | AC-003, AC-004 |
| `crates/happenstance-testkit/src/registry.rs` | The mount point: the single enumeration and the three emitters. `no_orphan_rules` at `:412-424` is the meta-test that makes an unmounted rule a build failure. | The moment the first rule function exists — mount it in the same edit. | AC-009 |
| `crates/happenstance-testkit/src/lib.rs` | `:84-92` — the crate-doc statement of what the enumeration is for and why an orphan rule is a defect rather than an omission; also the rule-family table a new family must appear in. | When mounting, and again when updating the family table. | AC-009 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The `Declared` / `Kind` / `FailureMode` shape at `:141-166` that the projection registry mirrors, and the module doc's no-pass-rate discipline. The two new rows must read the same way a reviewer already knows how to read. | Before writing either registry row. | AC-007, AC-008 |
| `crates/happenstance-testkit/tests/mutation_coverage/correct.rs` | `:1-18` — the shared correct-steps precedent: one module of correct behaviour, one overridden step per wrong store. This is what stops a red rule being red for the wrong reason. | Before writing the first wrong store, and again if a store starts needing two overrides. | AC-008 |
| `crates/happenstance-testkit/tests/mutation_coverage/racers.rs` | `:10-14` — the empty-`fails` rejection. It is why a weak scoping rule *breaks the build* rather than merely under-checking, and why EC-004 forbids trimming the row. | When `TruncatingResetStore` is registered, and immediately if the registry goes red. | AC-002, AC-007 |
| `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` | The shape `projection_conformance_wasm.rs` copies — the `#![cfg(target_arch = "wasm32")]` harness the mandatory gate step type-checks. | When adding the five names changes what the wasm harness compiles. | AC-009 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` — the reference fixture: one instance is one isolated backing store, each `connect()` a handle onto it. The **fresh handle** every rule here observes through comes from this pattern. | Before writing the first fresh-handle assertion. | AC-001, AC-003 |
| `crates/happenstance-core/src/projection.rs` | The port as it stands **today** — `checkpoint` still returning `Option<SequencePosition>` at `:110` and no `reset` at all — which is what makes "returned to never run" indistinguishable from "committed at the first position" before slice 2. Read it to see the delta the rules depend on; the amended file is `owned-batch-port-shape`'s. | First, to confirm slice 2 actually landed the shape these rules need (EC-006). | AC-001, AC-005, AC-006 |
| `xtask/src/spec_trace.rs` | `RULE_FILES` at `:85-89` and check 6 at `:623-631` — the array three gate checks read. If the projection rules module is not in it, all three pass vacuously over these five rules. | Before writing any rule, as the EC-002 precondition check. | AC-010 |
| `xtask/src/lints.rs` | `:511-531` — CF-29's changelog lint, and its own documentation of why it is a length test rather than a vocabulary test. It is the weakest check in the gate, so the changelog prose has to carry the weight. | When writing the five changelog entries. | AC-010 |
| `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` | The recorded gap between PS-19's `MUST` and the rule assigned to it — the thing that makes `fresh_projection_has_no_checkpoint` illegitimate until the repair atom is accepted. | Before writing `fresh_projection_has_no_checkpoint`, as EC-001's check. | AC-005 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The repair-versus-gap test and the discipline any frozen-clause correction lands under. It is what keeps this story from "just fixing" PS-19 in passing. | The moment editing a `[FROZEN]` clause looks cheap. | AC-005, AC-010 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision behind the mutant registry: provenance names a real implementation mistake, and no pass rate is ever quoted over the mutant set. | While writing both provenance strings and the implementation report. | AC-007, AC-008 |
| `.kb/decisions/0007-projection-runner-decodes.md` | The accepted decision that already fixes checkpoints per `(store, ProjectionId)`. PS-17 is that decision applied to removal — cited, not re-derived. | While writing `reset_is_scoped_to_one_projection`'s doc comment. | AC-002 |
| `standards/rust/60-what-a-test-must-prove.md` | RS-60-3 at `:169-176` — one defect is one overridden default method, with the marker-type mechanism. The construction rule both new stores follow. | Before writing either wrong store. | AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The binding briefs: Architecture Notes 4, 5, 6, 9 and 10; the Testing brief's **AC-011** row (Integration + Unit, sibling id, named rejection) and **AC-003** row; the UX brief's AC-U07 – AC-U11. When an acceptance criterion's *instrument* is in doubt, this decides it. | Whenever a rule's tier or seam is unclear. | AC-001, AC-003, AC-004, AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off no-surface determination (`surfaces: []`) and the one text surface this story consumes unchanged — cite it rather than re-deciding it. | Only if a surface or declension-policy obligation appears to apply. | AC-004 |
| `RUNBOOK.md` | `:3904-3907` — phase 6's `reset` bullet, including the six-scenarios-out-of-six observation that is the commit-at-first mutant's provenance string. | When writing that store's provenance. | AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/reset-rules/discover.md` | This story's own discover stage: the signal ledger, and *The wrong implementation* section naming four ways to write these rules badly. It is the shortest statement of what this PR must not become. | Before starting, and again at self-review. | AC-002, AC-003, AC-005, AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated** — AC-001 … AC-010,
   none added, none dropped. The Behavior-and-interfaces table's thirteen rows
   fold into ten: the five rules become AC-001 – AC-003, AC-005 and AC-006; the
   declined-capability row becomes AC-004; the two named wrong stores become
   AC-007 and AC-008 (which also absorbs "a mutant for every rule of the five"
   and the construction-discipline row, because they are one obligation — a
   mutant is only evidence when the defect is the only difference); the mounting
   row becomes AC-009; and the no-literal-positions, changelog and gate-file-list
   rows fold into AC-010, which is the single claim *the gate can see these rules
   and nothing was moved to make them pass.*
2. **The two preconditions are error conditions, not acceptance criteria.** The
   PS-19 repair atom (EC-001) and `RULE_FILES` (EC-002) are things this story
   *checks and reports*, not things it delivers; making either an AC would claim
   credit for another story's work. Both appear in AC-005's and AC-010's
   verification cells as the checks that detect them.
3. **PS-20's replay half is discharged inside `reset_is_not_commit_at_first`,
   without a runner.** The rule derives a resume point from each checkpoint under
   the port's own rule — strictly after a `Live` position, inclusive from the
   store's first position when `NeverRun` — and asserts event 1 is included in
   one case and excluded in the other. Building the runner to observe it would
   reach into `typed-layer-and-alpha-release`; asserting only the variant would
   leave the *consequence* — event 1 skipped permanently — unchecked.
4. **The interrupted-reset half of PS-16 is asserted where the fixture can
   express a failing reset, and is a reported skip where it cannot.** This
   follows AC-004's vocabulary rather than inventing a second one, and it keeps
   the assertion from being silently absent for stores that cannot inject a
   failure. E2E-17's "one of the two consistent states, never the third" is the
   claim; the fresh-handle both-halves observation is how it is seen.
5. **The commit-at-first substitute's identity is left to the implementer** — a
   new store or an arm of `ValidatingCommitStore`. Both are legal; the exactness
   meta-test constrains the choice more than taste does, and if an arm makes the
   store fail two rules it wanted to be its own store (EC-005).
6. **Verification is Integration-first, with Unit carrying the registries.** That
   is the Testing brief's own split for AC-011 and AC-003, not a weakening: a
   claim about what a store does *after* a reset cannot be checked structurally,
   and a claim about whether a rule has ever been red cannot be checked by
   running it against a correct store.
7. **The composition family of the interaction-quality invariants is discharged
   by `_design.md`, not skipped.** `surfaces: []` is a signed-off determination
   and the sign-off block says the no-surface determination itself is what was
   approved. The three obligations that survive translation — presentation exists
   at all, transience, and the density budget with its real numbers — are bound
   to AC-004, AC-008, AC-009 and AC-010, so they are gated rather than asserted
   in prose.
8. **No pass rate, and no proof-artefact claim, is made here.** This story
   supplies two more names a wrong store fails; reading them as a verdict about
   the suite is `whole-gate-run-and-proof-artefact`'s and
   `ps3-batch-shape-finding`'s, and a green gate remains a precondition for that
   reading rather than a substitute for it.
