# Two-shape observation — written from the run, not from memory

AC-007's deliverable. Written while `cargo xtask ci` was still on the screen,
because the alternative is reconstructing it a day later, which is how "they
agreed everywhere" becomes a silent success instead of the finding it is.

`ps3-batch-shape-finding` (HS-S0014) consumes this and composes
`references/evaluation/projection-batch-shape-evidence.md` from it. This file is
raw material; the finding is the artefact, and neither makes a verdict.

## What ran

One `cargo xtask ci`, clean tree, `all checks passed`. Inside its `tests` step,
five projection test targets, of which two are conformance harnesses driving a
fixture through **the same sixteen rules** from the one enumeration
`for_each_projection_store_rule!` (`crates/happenstance-testkit/src/projection.rs:1843-1888`):

| Target | Fixture | Store | Batch |
| --- | --- | --- | --- |
| `tests/projection_conformance.rs` | `MemoryProjectionFixture` | `MemoryProjectionStore` | a materialised delta: `BTreeMap<String, u64>` + a `clear_all` flag, collapsed at stage time (`crates/happenstance-core/src/projection_memory.rs:209-250`) |
| `tests/projection_conformance_buffering.rs` | `BufferingProjectionFixture` | `BufferingProjectionStore` | an ordered op journal, collapsed by nothing, replayed in order inside `commit` (`crates/happenstance-testkit/tests/projection_mutation_coverage/buffering.rs`) |

Both green in that one invocation. The mutant binary
(`tests/projection_mutation_coverage.rs`) drove the same sixteen rules against
nineteen further stores, including a third batch shape — see *the axis* below.

## Did the two shapes disagree?

**Fourteen rules agreed. Two differ, and they differ in the fixture's
capability declaration rather than in the rule.** Nothing needed per-shape
handling, and no rule was touched.

### The two that differ

`failed_commit_leaves_both_unchanged` and `refused_reset_changes_nothing` are
gated on `COMMIT_FAULT` and `RESET_REFUSAL` respectively. `MemoryProjectionFixture`
declines both and states why; `BufferingProjectionFixture` supports both. So the
same rule **runs** against one shape and reports `RuleOutcome::Skipped` against
the other. The two skip lines, read off `--show-output` on the reference run and
grounded in the `RuleOutcome` values the harness collected:

```
SKIP refused_reset_changes_nothing: fixture declines `RESET_REFUSAL` — MemoryProjectionStore holds no protection policy …
SKIP failed_commit_leaves_both_unchanged: fixture declines `COMMIT_FAULT` — MemoryProjectionStore applies the read-model writes and the checkpoint under one write lock …
```

This is a difference between two **fixtures**, not between two batch shapes: the
reference store declines because it is a shipped store with nothing to fault and
no policy to enforce, and the buffering fixture supports because it is an
instrument. It is nonetheless a real asymmetry in what the two runs assert, and
recording it as "agreed" would overstate the comparison — the buffering run
asserts sixteen rules and the reference run asserts fourteen.

It is also load-bearing rather than incidental: had the buffering fixture also
declined, those two rules would have been unexecuted against **every** conformant
variant this family has, and `projection_conformant_variants_pass_everything`'s
second assertion — added by this story — would have refused the registry.

### The fourteen that agreed

Every other rule ran against both shapes and passed against both, with no branch,
no per-shape wiring and no assertion generalised. Notably the ones with the most
room to assume: `commit_is_atomic_with_the_read_model` (PS-4's own `Rule:` claims
it is shape-blind — it is), `batch_reads_reflect_pending_writes` and
`rebuild_is_chunk_size_invariant` (a backwards scan over a journal answers them
exactly as a map lookup over a delta does), `dropped_batch_leaves_store_usable`
and `rollback_leaves_both_unchanged` (which hold in the buffering store for a
*different reason* — there is no resource to fail to return — and neither rule
noticed), and `commit_rejects_a_foreign_batch` (answered by a per-instance `u64`
stamp compared as a field, with no lifetime anywhere).

**No rule assumed a lock was held, a transaction was open, or a connection
existed between `begin` and `commit`.** That is the sentence PS-4's `Rejects:`
clause asks for, and it is the substantive positive result of this run.

## The axis — and where the framing was wrong

Stated plainly because the finding is worth less without it, and because a reader
who checks will find it in ten minutes.

This story's spec describes `MemoryProjectionStore` as **apply-on-write** and the
new store as the buffering one at the far end of the axis. The code does not say
that. `MemoryProjectionStore::begin` returns an owned `MemoryProjectionBatch`
holding a delta, and `commit` applies it (`projection_memory.rs:280-330`) — it
already sits at the *deferred write set* end, holds no handle across the batch,
and satisfies PS-4's reading in full. §4.11's own sentence ("the obvious one is
the buffering adapter PS-4 permits") carries the same assumption.

So what this run actually compared is **narrower** than "buffered versus live
transaction":

* **collapsed delta versus replayed journal** — whether a batch decides
  last-write-wins at stage time or at replay time, and whether a queued
  `delete_all` is a flag or an op with a position in a sequence. Real, and the
  one axis this variant differs on.
* **`Send` versus bare flavour** — `MemoryProjectionStore` is `Arc<RwLock<_>>`
  and implements `SendProjectionStore`; `BufferingProjectionStore` is
  `Rc<RefCell<_>>` and implements the bare `ProjectionStore`. A consequence of
  holding nothing across the batch rather than a second deliberate axis, but it
  does mean the projection rules have now been driven against a `!Send` store
  through the tokio harness.

What was **not** compared here is the other end of PS-2's axis: a store that
acquires a handle at `begin` and holds it to `commit`. That shape is in the
workspace and *is* driven through all sixteen rules — `MutantStore`
(`tests/projection_mutation_coverage/correct.rs:528-546, :629-641`) checks a connection out
at `begin`, refuses a batch that does not hold it, and returns it at `commit`,
`rollback` or `Drop` — but through the **mutant harness**, not through
`projection_store_conformance!`, and it is an in-process model of a connection
rather than one. Three shapes have met the rules; none of them crosses a process
boundary.

Both readings Note 10 item 2 offers therefore apply in part, and this evidence
distinguishes them only weakly: the rules *are* shape-blind on this axis, and the
axis as §4.2 places it is not where the two conformance-harness shapes actually
sit. `ps3-batch-shape-finding` owns saying so; it is written here so that story
does not have to re-derive it.

## What the suite alone can and cannot see

Recorded because it is the AC-001 justification and it was measured rather than
assumed. With `probe_write` temporarily made apply-on-write — one line, reverted
before commit — **five of the sixteen rules went red**:
`rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable`,
`failed_commit_leaves_both_unchanged`, `commit_rejects_a_foreign_batch` and
`commit_rejects_a_regressing_position`. So the suite is not blind to a crude
degeneration.

What it would not see is the careful one: a store that applies on write and
compensates on every path a rule observes — rollback, bare drop, fault, refusal.
That store passes all sixteen and is not the second batch shape at all.
`the_read_model_changes_only_at_commit`
(`crates/happenstance-testkit/tests/projection_conformance_buffering.rs:73`) is
what rejects it, and it asserts on committed state read out of band while a batch
is open, independently of any rule.

## The CF-6 discipline, as a diff claim

* **0** conformance rules added. `for_each_projection_store_rule!` is byte-identical.
* **0** conformance rules edited. No rule body, gate or assertion changed.
* **0** new capabilities minted. `SECOND_HANDLE`, `RESET_REFUSAL`, `COMMIT_FAULT`
  and `READS_THROUGH_BATCH` are the same four switches.
* **0** files under `crates/happenstance-core/src/`.
* **0** new dependencies, dev-dependencies, features or target tables.
* **0** edits to `spec/SPECIFICATION.md`.
* No CF-29 obligation incurred, which is the cross-check: adding a rule would owe
  a mutant plus a changelog entry, and an incurred one is the drift signal.

Two meta-tests in the projection mutant binary did change, and neither is a
conformance rule: `projection_conformant_variants_pass_everything` gained the
"every rule executed against a variant" assertion AC-004 requires, and
`the_second_batch_shape_answers_every_rule_with_a_pass` is new.

## What is not claimed

PS-2's bar is not met by anything this story built. PS-2 is `[FROZEN]`, asks for
two **adapters** at opposite ends of the batch-shape axis, and its `Rejects:`
clause names the two-instrument monoculture verbatim
(`spec/SPECIFICATION.md:4760-4775`). Two testkit instruments in process memory are
not two adapters. No verdict on the `unstable-projection` exposure is made here or
in the finding; that is `publication-and-positioning`'s (HS-P0016).

No pass rate is quoted, here or anywhere in the run's output (ADR-0010).
