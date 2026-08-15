# Two batch shapes against one projection suite — did they disagree?

**Date:** 2026-08-14.
**Pinned to:** `cfd9231` (`cfd92313cc59e059655fe130f3c8c31b19dfcf08`) on
`initiative/from-contract-to-published-library` — the commit whose
`cargo xtask ci` run this document reports, and the only run it reports.
**Lifecycle:** immutable evidence. **Superseded rather than edited**, per
[`README.md:9-13`](README.md) and `README.md:140-142`. A later comparison — a
third shape, a real adapter, a changed rule set — is a new dated document, not a
revision of this one, because two other files cite this one and a reader who
followed a citation must find what they were sent to.

Written for a planner standing at PS-3 or at the projection freeze verdict, who
has not read this project's implementation history and does not have to. It is a
**finding**: it records what one run showed. It decides nothing, and §7 says so
in terms.

---

## 1. What this was observed on

One `cargo xtask ci` invocation, clean tree, `all checks passed`. Not two
`cargo test` runs reconciled by hand — the two shapes are two ordinary `tests/`
targets of `happenstance-testkit`, so the gate runs both whether or not anyone
asks it to.

Inside that run's `tests` step, two conformance harnesses drove **the same
sixteen rules**, taken from the single enumeration `for_each_projection_store_rule!`
(`crates/happenstance-testkit/src/projection.rs:1843-1881` — the enumeration
lives beside the projection rules rather than in `registry.rs`, where the
event-store family's sits):

| Harness target | Fixture | Store | What a batch is |
|---|---|---|---|
| `crates/happenstance-testkit/tests/projection_conformance.rs` | `MemoryProjectionFixture` | `MemoryProjectionStore` (`crates/happenstance-core/src/projection_memory.rs:101`) | a materialised delta — `BTreeMap<String, u64>` plus a `clear_all` flag, collapsed at stage time (`projection_memory.rs:209-250`) |
| `crates/happenstance-testkit/tests/projection_conformance_buffering.rs` | `BufferingProjectionFixture` | `BufferingProjectionStore` (`crates/happenstance-testkit/tests/projection_mutation_coverage/buffering.rs:252`) | an ordered op journal, collapsed by nothing, replayed in order inside `commit` |

Both green in that one invocation: 16 rule tests each, plus one direct
visibility assertion in the buffering harness
(`projection_conformance_buffering.rs:73`).

A third store met the same sixteen rules in the same run, through the **mutant**
harness rather than through `projection_store_conformance!`: `MutantStore`
(`crates/happenstance-testkit/tests/projection_mutation_coverage/correct.rs:528`)
checks a connection out at `begin`, refuses a batch that does not hold it, and
returns it at `commit`, `rollback` or `Drop`. It is named here because §5 needs
it; the comparison below is between the two conformance-harness shapes only.

**Every store named above is in this process.** Nothing here crosses a
transaction manager, a connection pool under contention, or a server that can
refuse half a request.

## 2. The vocabulary, fixed before the ledger

Stated ahead of the evidence so that a reader can re-classify the raw
observations and reach the same labels. Six labels, and no seventh is minted
inside the ledger.

- **D1 — per-shape handling.** A rule whose body, fixture wiring or assertion had
  to branch on which shape it was running against.
- **D2 — asymmetric declension.** A rule one shape ran and the other reported as
  `RuleOutcome::Skipped`, because a capability constant — fixture-declared, or
  `ProjectionProbe::READS_THROUGH_BATCH` — differs between the two.
- **D3 — assertion loosened.** A rule whose assertion was generalised *during*
  the work that added the second shape, so that shape could pass it. Retrospective:
  read off the diff, never off the run.
- **D4 — divergent observable behaviour.** Both shapes conformant, both green,
  but the observable sequence differs in a way no clause currently describes.
- **agreed** — none of the four.
- **not comparable** — one shape's fixture never reached the rule, or both
  skipped it. A distinct verdict, never folded into **agreed**.

## 3. The ledger

One row per rule in `for_each_projection_store_rule!`, in enumeration order,
exactly once each. The row set was extracted from the macro rather than recalled;
the comparison is recorded in this story's implementation report.

| # | Rule | Verdict | The observation it rests on |
|---|---|---|---|
| 1 | `commit_advances_the_checkpoint` | agreed | Ran and passed against both. The checkpoint moves in the same step as the rows in each store; neither needed a different position, and neither assertion names a literal one. |
| 2 | `commit_is_atomic_with_the_read_model` | agreed | Ran and passed against both. PS-4's `Rule:` claims this rule is shape-blind by construction; the delta store couples the two halves under one `RwLock` write guard, the journal store couples them by replaying nothing until every refusal has been decided and then writing both inside one `RefCell` borrow. The rule did not notice. |
| 3 | `failed_commit_leaves_both_unchanged` | **D2** | Ran against the buffering fixture, which declares `COMMIT_FAULT` supported. Reported `RuleOutcome::Skipped` against `MemoryProjectionFixture`, whose declension is stated on the constant: *"MemoryProjectionStore applies the read-model writes and the checkpoint under one write lock, so it has no write that can be made to fail…"*. The `Capability` is `ProjectionFixture::COMMIT_FAULT` (`crates/happenstance-testkit/src/contract.rs:640`); the skip is a `RuleOutcome::Skipped` value collected by the harness (`contract.rs:473-537`), not a remembered line. |
| 4 | `rollback_leaves_both_unchanged` | agreed | Ran and passed against both, and this is one of the two rows where the two shapes hold the invariant for *different reasons*: the delta store discards a buffer, the journal store has nothing to issue a `ROLLBACK` to because nothing was ever sent. The rule asserts the state, not the mechanism, and neither shape needed handling. |
| 5 | `dropped_batch_leaves_store_usable` | agreed | Ran and passed against both, for row 4's reason and with the same consequence: PS-7's second half — *and the store remains usable* — is true by construction in both, which is why the mutant `PooledConnectionStore` exists in the same run to keep the rule from being decorative. |
| 6 | `commit_rejects_a_foreign_batch` | agreed | Ran and passed against both. Each store mints a per-instance `u64` stamp and compares it as a field; with `type Batch;` carrying no lifetime (PS-5) there is nothing else it could be, and the rule spends two isolated `open()` calls rather than a second handle. |
| 7 | `commit_accepts_a_position_the_batch_did_not_write` | agreed | Ran and passed against both. Neither store validates the committed position against its batch, which PS-21 forbids — notable only because the journal store is the one with the whole statement list in front of it and therefore the one most able to. |
| 8 | `commit_rejects_a_regressing_position` | agreed | Ran and passed against both, on the recorded checkpoint rather than on anything the batch holds, so the batch representation is not in the path. |
| 9 | `distinct_projections_advance_independently` | agreed | Ran and passed against both. Both key checkpoints by `ProjectionId`; neither shares a row between projections. |
| 10 | `reset_clears_rows_and_checkpoint_together` | agreed | Ran and passed against both. In the journal store `reset` replays the caller's staged `DeleteAll` and records the checkpoint inside the same borrow — a queued op rather than a truncate. |
| 11 | `reset_is_scoped_to_one_projection` | agreed | Ran and passed against both; one checkpoint key touched in each. |
| 12 | `refused_reset_changes_nothing` | **D2** | Ran against the buffering fixture, which declares `RESET_REFUSAL` supported and holds one protected id in a cell. Reported `RuleOutcome::Skipped` against `MemoryProjectionFixture`: *"MemoryProjectionStore holds no protection policy, so there is no projection it could decline to reset…"*. The `Capability` is `ProjectionFixture::RESET_REFUSAL` (`contract.rs:604`); the claim is the `RuleOutcome::Skipped` value, not the printed line. |
| 13 | `reset_is_not_commit_at_first` | agreed | Ran and passed against both. `MemoryProjectionStore` *removes* the checkpoint row and `BufferingProjectionStore` *records* an explicit `Checkpoint::NeverRun`; both read back as `NeverRun`, and the rule compares variants rather than positions, so the divergence in mechanism is invisible to it and is permitted by the clause. |
| 14 | `batch_reads_reflect_pending_writes` | agreed | Ran and passed against both, and this is the row that could most easily have gone otherwise. Both declare `READS_THROUGH_BATCH = true`; the delta store answers with a map lookup, the journal store with a backwards scan that stops at a queued `DeleteAll`. The rule asserts the value written, and got it from both. |
| 15 | `rebuild_is_chunk_size_invariant` | agreed | Ran and passed against both. Each chunk is a read-modify-write *through* the open batch at three different chunk sizes against three isolated stores; the journal store accumulates two writes for one key inside a chunk and lets replay order decide, the delta store decides at stage time. Same read model out. |
| 16 | `rebuilding_is_distinguishable_from_live` | agreed | Ran and passed against both. `Authority` is carried into the checkpoint variant by both; the batch representation is not in the path. |

**Tally: fourteen agreed, two D2, no D1, no D3, no D4, none *not comparable*.**
No rule was absent from either run.

**D3 is zero and it is provable rather than asserted.** The commit this document
is pinned to touches `crates/happenstance-testkit/tests/**` and nothing else:
`crates/happenstance-testkit/src/**` — where every rule body and every assertion
lives — has no diff in it, and neither does `crates/happenstance-core/**`. No
assertion was generalised, no rule was gated behind a newly minted capability,
and no rule was added.

### What the two D2 rows do and do not say

They are an asymmetry in what the two runs **assert**, not a disagreement between
the two stores: the buffering run asserts sixteen rules, the reference run asserts
fourteen and reports two honest declensions. The difference is in the two
*fixtures*' capability declarations, and its cause is that one store is shipped
and the other is an instrument — `MemoryProjectionStore` has no write that can be
made to fail and no protection policy, and building either into it to satisfy a
test is the trade CF-18 exists to let a fixture decline instead.

Recording these as **agreed** would have overstated the comparison, which is why
the vocabulary was fixed before the ledger was filled.

## 4. The null result, weighed

Fourteen of sixteen rules agreed, and the two that did not agreed about the
stores — they differ about the fixtures. So on the batch-representation axis the
answer is: **the two shapes did not disagree anywhere.**

That is the finding, not the absence of one, and Architecture brief Note 10 item
2 offers two readings of it. Both are addressed here rather than one chosen by
default.

**Reading A — the rules are shape-blind in a way that hides the axis.** Supported
in part. Three rows above show a rule accepting two genuinely different
mechanisms without noticing: rows 4 and 5 hold for opposite reasons (a discarded
buffer versus nothing ever sent), and row 13 accepts a removed checkpoint row and
a recorded `NeverRun` as the same answer because it compares variants rather than
positions. In each case the rule asserts the *state a caller can observe* and
declines to assert the mechanism — which is what a port-level rule is supposed to
do, and is also exactly what makes it unable to report that the mechanisms
differed.

**Reading B — the axis is not where §4.2 places it.** Also supported, and this is
the reading with the sharper consequence, because it is a fact about the tree
rather than an interpretation of a run. §4.11 assigns the projection family's
CF-5 conformant variant to *"the buffering adapter PS-4 permits"*, on the
understanding that `MemoryProjectionStore` sits at the other end. It does not.
`MemoryProjectionStore::begin` returns an owned `MemoryProjectionBatch` holding a
delta and `commit` applies it (`crates/happenstance-core/src/projection_memory.rs:280-330`);
it acquires no handle, opens no transaction and holds no lock between `begin` and
`commit`. It is already a deferred write set, and already satisfies PS-4's
reading in full.

So the axis this comparison actually spans is **narrower** than PS-2's. What
separates the two shapes is the batch's *representation* — an ordered journal
collapsed by nothing, versus a delta collapsed at stage time — together with the
port flavour each implements (`SendProjectionStore` over `Arc<RwLock<_>>`, and
the bare `ProjectionStore` over `Rc<RefCell<_>>`). What separates them is **not**
whether a handle is held across the batch, because neither holds one.

**Can this evidence distinguish the two readings?** Only partly, and the honest
answer is stated rather than implied: reading B is established as a fact about
what was compared, and it makes reading A untestable on this evidence. A rule
cannot be shown blind to an axis that neither of the two subjects sits on either
side of. The shape that *does* hold a resource across the batch —
`MutantStore`, §1 — met all sixteen rules in the same run and passed them, which
is a third data point and not a fourth shape: it is an in-process model of a
connection, written as the correct core the mutant registry mutates, and it is
driven by the mutant harness rather than by `projection_store_conformance!`.

What the run does establish, in one sentence a downstream reader can use: **no
projection rule assumed a lock was held, a transaction was open, or a connection
existed between `begin` and `commit`** — which is the sentence PS-4's `Rejects:`
clause asks for, and it is established against three in-process stores.

## 5. PS-2

**PS-2's bar — the suite green against two *adapters* at opposite ends of the
batch-shape axis — is not met by anything this project built alone.** PS-2 is
`[FROZEN]` (`spec/SPECIFICATION.md:4760-4775`), and its `Rejects:` clause names
the failure mode in terms: *"the schedule that freezes this port against
`MemoryProjectionStore` and an in-process rusqlite transaction. Both serialise
their writers, both hold a real handle, both are the same storage shape wearing
two hats — the exact monoculture CLAUDE.md's spread rule exists to catch."*

Two testkit instruments in process memory are fewer than two adapters, not more.
The shapes compared in §3 are `MemoryProjectionStore` and a store written for
this comparison; §4 records that they do not span PS-2's axis in any case. This
document makes no claim about when PS-2's bar will be met, or by what.

## 6. What is not in this document

No pass rate over the rule set or the mutant set, here or in the run's own output
([`.kb/decisions/0010-the-suite-must-prove-itself.md`](../../.kb/decisions/0010-the-suite-must-prove-itself.md)).
The denominator is an author's choice, so a fraction reports how representative
the author was while reading as though it reported how good the suite is.

No repair. Nothing above changes a rule, a fixture, a mutant or an adapter. Where
a rule's reach is described as narrower than a reader might assume — rows 4, 5 and
13 — that is a description of what the rule asserts, not a defect filed against
it; a rule that turns out to be wrong is fixed in its own story with the reason in
the same change.

No clause disposition. PS-3's maturity marker, the `unstable-projection` feature
gate and the rest of PS-1 – PS-37's disposition belong to
`unstable-projection-gate-and-clause-disposition` (HS-S0016). PS-2 is cited above
and not amended.

## 7. No verdict is made here

**This document makes no exposure verdict and no freeze verdict.** It contains no
recommendation, no preference, no leaning and no prediction, and a reader who
extracts one has extracted something that is not in it.

- Whether `ProjectionStore` ships behind an off-by-default `unstable-projection`
  feature at 0.1 is **`publication-and-positioning`'s (HS-P0016)** call, through
  `projection-port-ship-shape`. This document is one of the two reports that
  story settles PS-3 on rather than re-deriving.
- Whether the projection port's freeze **held** is **`ladybug-projection-store`'s
  (HS-P0015)** call, through `freeze-verdict-document`. This document supplies the
  *what it was checked against* for the two shapes that ran before Ladybug's, which
  that story cannot reconstruct afterwards.

Neither call is made here, and neither is recommended.
