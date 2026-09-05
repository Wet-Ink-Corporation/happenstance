# F2-5's actionable half is false: ES-22's `landed == 0` arm executes, twice, on every run

Short answer up front: **the finding is wrong and nothing is owed on its ground.**
`dropped_append_future_leaves_no_partial_batch`'s second arm — the byte-identical
`snapshot_of(&after) == snapshot_of(&before)` comparison — is reached by two
registered stores on every run of `tests/mutation_coverage`, in both feature
configurations, and has been since the rule's first commit. The lane that was to
build "a store that suspends before its first write" found two already in the tree
and stopped rather than adding a third.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** It was written by the lane that refuted the finding, in the same
session as the measurement. The measurement is mechanical and reproducible, which is
the part that does not need a critic; the *residual* it proposes at the end does,
and is held at low-to-medium confidence.

---

## What the finding claimed

F2-5, actionable half:

> Arm 2 has never run against anything. […] the only store in the tree that suspends
> inside `append` is `YieldingRowAtATimeStore`, whose `yield_once().await` sits at
> the **end** of the loop body — *after* the write. It therefore lands on
> `landed == 1`, fails arm 1, and arm 2 is never reached. Every other store has no
> suspension point at all and completes on the first poll.

The last sentence is the false one. Everything before it is correct about
`YieldingRowAtATimeStore`.

## The measurement

Arm 2 was mutated so that it must fail if anything reaches it — the right-hand side
replaced by an empty vector, which cannot equal the left because the rule seeds an
`Existing` row before it starts:

```rust
if landed == 0 {
    assert_eq!(
        snapshot_of(&after),
        Vec::new(),
        "MUTATED ARM 2 — cannot hold: `before` carries the seeded \
         `Existing` row, so any store reaching this line fails here"
    );
```

`cargo test -p happenstance-testkit --all-features`, verbatim:

```
test mutation_coverage::mutants_fail_exactly_their_declared_rules ... FAILED

failures:

---- mutation_coverage::mutants_fail_exactly_their_declared_rules stdout ----

thread 'mutation_coverage::mutants_fail_exactly_their_declared_rules' (99260) panicked at crates\happenstance-testkit\tests\mutation_coverage.rs:4024:41:
`PreCommitPositionStore` failed `dropped_append_future_leaves_no_partial_batch`, which it does not declare. Either the mutant is broken in more ways than it claims — the commonest way a mutant set decays — or the declaration is short a line. Saw: panicked at crates\happenstance-testkit\src\suite.rs:3605: assertion `left == right` failed: MUTATED ARM 2 — cannot hold: `before` carries the seeded `Existing` row, so any store reaching this line fails here
  left: [(1, Event { event_type: EventType("Existing"), data: <2 bytes>, tags: {"row:seed"}, metadata: None })]
 right: []

test result: FAILED. 15 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.00s
```

`cargo test -p happenstance-testkit --no-default-features --test mutation_coverage`
produces the identical failure. The finding was not a stale observation about one
feature configuration.

Note the `left` value. It is a one-row snapshot, not an empty one — so arm 2's real
comparison is between two populated snapshots and is not vacuously true. That is
exactly the anchor the rule's own prose demands ("without a row already in the
store, \"unchanged\" and \"empty\" are the same observation").

## How many stores reach it, and which

`mutants_fail_exactly_their_declared_rules` asserts eagerly, so the panic above
names only the first. Replacing the mutation with a print left every store running:

```
ARM2-REACHED after=["Existing"]
ARM2-REACHED after=["Existing"]
```

Two, and only two. They are identifiable without further instrumentation, because
the event-store family holds exactly three suspension points inside an `append`
body and no others:

| Store | Suspension point | `landed` after one poll |
|---|---|---|
| `YieldingRowAtATimeStore` (`mutants.rs:1960`) | end of the loop body, *after* the row is written | 1 — arm 1, which it is declared to fail |
| `PreCommitPositionStore` (`mutants.rs:4459`) | after allocating the position, *before* publishing to `committed` | 0 — **arm 2** |
| `AwaitAcrossBorrowStore` (`mutants.rs:4706`) | after `borrow_mut()`, *before* `correct::commit` | 0 — **arm 2** |

Both reach arm 2 and both **pass** it: neither has a defect on ES-22's axis, so the
snapshot really is byte-identical and the assertion really does hold.

## Why the finding looked true

Because the store that reaches arm 2 is not the store whose name advertises the
axis. `YieldingRowAtATimeStore` is the entry filed under *ES-22 — a suspension point
between two rows*, and its registry row and doc comment are the only prose in the
binary about dropped futures. The two stores that actually exercise arm 2 are filed
under other axes entirely — position visibility (ES-10) and a borrow held across an
await — and their suspension points are documented as *their* defects' mechanism
rather than as anything to do with cancellation. `PreCommitPositionStore`'s own
comment even says so:

> The transaction doing its work, and committing. One suspension, and it is the
> whole window: driven to completion in one go — which is what every other rule in
> the suite does — this store is indistinguishable from a correct one.

"Every other rule in the suite" is the sentence that is one rule out of date: ES-22
does *not* drive to completion, which is precisely why it reaches through this
store's window and lands in arm 2. Reading the corpus by axis, as the finding did,
finds the wrong store. Only running it finds the right two.

`git log -S` puts both on the same day: the rule and `PreCommitPositionStore` landed
in the **same commit**, `d480446` (2026-08-08, *"Make every sentence phase 4 froze
checkable, and every rule capable of failing"*). Arm 2 has been executing since the
hour it was written.

## The neighbouring rule the finding asked about

`append_is_atomic_under_a_mid_batch_fault` (`suite.rs:3046-3063`) was checked for the
same shape by the same method — a print in each arm. It does **not** have it. Both
arms execute, three times between them:

```
MBF-ERR-ARM after=["Existing"]                 # GappedPositionStore — rolled back, passes
MBF-ERR-ARM after=["Existing", "X", "Y"]       # NoTransactionStore  — partial, fails (its declared pin)
MBF-OK-ARM  after=["Existing", "X", "Y", "Z"]  # NoopFaultFixture    — swallowed the fault, passes
```

The `Err` arm carries both a passing and a failing store, which is the strongest
form: it is not merely executed, it is *decisive*. No action is owed there and none
was taken.

## What is still true, and it is narrower than the finding

Two things survive the refutation, and they should not be conflated with it.

**1. No *conformant* store reaches arm 2.** Under the mutation,
`conformant_variants_pass_everything` passed — that test fails on any
`Verdict::Panicked` against a `Kind::ConformantVariant`, and the mutation panics
unconditionally when arm 2 is reached, so its silence is proof. `GappedPositionStore`
and `PagedStreamStore` both have `append` bodies with no suspension point, complete
on the first poll, and reach the rule through arm 1's `else` branch instead.

So arm 2 is exercised only by mutants. What that costs is CF-5's half of the
argument and not CF-1's: a rule is *under*-specified if no wrong store fails it, and
arm 2 has two stores passing it, which is coverage. A rule is *over*-specified if a
legal store fails it, and only a conformant control can catch that. Arm 2 has none.

The mitigation is that both stores reaching it are byte-for-byte legal *on ES-22's
axis* — their declared defects are elsewhere, and each is documented as
"indistinguishable from a correct one" when driven to completion. They are
functionally conformant controls for arm 2's content while being registered as
mutants for another rule's. Whether that is good enough, or whether the portfolio
owes a `Kind::ConformantVariant` whose `append` suspends before its first write, is
the one open question this lane leaves. It is a **small** question: such a store is
about forty lines, it would delegate everything to `crate::correct`, and the only
cost beyond that is the `MODEL_COVERAGE` heading arithmetic
(`the_model_coverage_heading_counts_the_table` pins "forty rows" and "thirty-eight
misses" as spelled words in prose, and a third control moves the first and not the
second). The argument against is CF-1 read strictly: name the plausible wrong
implementation such a control rejects, and the answer is *none* — a conformant
control rejects nothing by construction, which is the whole of CF-5's separate
justification, so the question is really "is CF-5's argument owed once per rule or
once per arm". Nobody has decided that, here or anywhere, and it is bigger than
ES-22.

**2. No real adapter has exercised the rule at all.** This is F2-5's other half and
it is untouched. Every store reaching either arm is an in-process `Rc`-backed
instrument in the testkit's own `tests/`; the rule's own doc comment says
`MemoryEventStore` "cannot answer this question and a real adapter must", and none
has. That is genuinely blocked on phase 10's `happenstance-postgres` or
`happenstance-neon` and stays blocked.

## The residual, stated precisely

Before this lane, F2-5 read as two things: *a rule with untested code inside it*,
**and** *a rule certified against nothing but a strawman*. The first is false and
should be struck. The residual is now exactly:

> **`dropped_append_future_leaves_no_partial_batch` has never been answered by a
> store with a real medium under it.** Both of its arms execute and both are
> decisive against the instruments in `tests/mutation_coverage`, but every store
> that has ever reached either arm is an `Rc<RefCell<…>>` in the testkit's own test
> target, where "the future was dropped" means a local was dropped rather than a
> connection was severed. Phase 10.

and, separately and much smaller:

> **Arm 2 is exercised by two mutants and by no `Kind::ConformantVariant`**, so
> nothing in the portfolio would catch arm 2 over-specifying. Whether CF-5's
> conformant-control obligation runs per rule or per *branch* is undecided, and
> ES-22 is the first place the difference has been visible.

## What this brief does not settle

- **Whether the second residual above is worth a store.** Recommended *no* at low
  confidence, on the CF-1 reading given, and explicitly handed to whoever owns the
  rule set rather than decided here.
- **Whether `PreCommitPositionStore`'s "every other rule in the suite" comment
  should be corrected.** It is prose in a mutant's doc comment, it is one rule out
  of date, and correcting it is a one-line change nobody has authorised. Noted so
  that the next reader who trips over it knows it was seen.
- **Nothing about ES-22's clause text, maturity marker, or rule body.** All three
  are untouched; the mutations above were measurements and were reverted, and
  `git status` is clean of them.
