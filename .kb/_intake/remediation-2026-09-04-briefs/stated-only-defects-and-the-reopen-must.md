# L1-2 is closable only as far as *stating* it. Is `Kind::StatedOnlyDefect` the right place to put what cannot be checked, and who owns CF-17's marker now that its text carries a MUST?

Short answer up front, **revised — this brief's original answer was refuted**:
**no. The kind was withdrawn.** Its bar was satisfiable by two string literals,
and — the finding that settled it — the scenario it demanded is *unsatisfiable
in principle* for the only case the kind was invented for. What replaced it is a
named test that measures the two consequences that are measurable and **states**
the third instead of appearing to check it. Two things this touches are somebody
else's and are not taken here: CF-17's and CF-14's maturity markers, and two
`.kb/open-questions/` atoms whose line citations this change made stale.

**This brief did not get the two-critic pass the original thirteen had.** It was
written by the lane implementing `L1-2`, in the same session as the change it
describes. An adversarial review did read the *implementation*, and refuted the
original answer; the revision record at the foot says how. Read it with that
discount applied.

---

## What landed, so the question is legible

`references/evaluation/review-pre-publication-2026-09-03.md:1576-1620` (L1-2) is
explicit that a complete fix is structurally blocked: `MID_BATCH_FAULT` is
closable because arming it has a port-observable consequence — the append must
answer `Err` — and `REOPEN` has none. What was available, and what landed:

1. **CF-39's other half, on CF-17.** A fixture declaring `REOPEN` supported MUST
   make `reopen` discard state over a medium outside the process's hold, MUST
   state the mechanism, and MUST decline where its store has no such medium.
   Restated on the constant in `crates/happenstance-testkit/src/contract.rs`.
2. **`NoopReopenFixture`**, promoted out of
   `experiments/suite-against-wrong-adapters/` into the testkit's own
   `tests/mutation_coverage/mutants.rs`, so the obligation has a named wrong
   implementation that is *driven* rather than described.
3. ~~**`Kind::StatedOnlyDefect`**, because `Kind::Mutant` rejects an empty
   `fails` list outright — rightly, and the assertion was not relaxed.~~
   **[Withdrawn — revision 2.]** `Kind::Mutant`'s assertion is still right and
   still unrelaxed; what was wrong was giving the fixture a *kind* rather than a
   record. It is now
   `reopen_over_claiming_is_undetectable_and_this_is_the_record` in
   `tests/mutation_coverage.rs`, and `NoopReopenFixture` is deliberately in
   neither `for_each_mutant!` nor `REGISTRY`.

## The question this brief exists for

`Kind::ModelOnlyMutant`'s own documentation is the warning: a kind that borrows
another family for its bar can have that bar compiled out, and an adversarial
review walked a defect-free store straight through the gap. A kind that borrows
*nothing* — which is what "stated only" means — is a strictly larger hazard,
because there is no family to borrow.

The bar written for it is three obligations, none behind a `cfg`:

1. `fails` is empty, driven and measured.
2. A `STATED_ONLY_DEFECTS` row whose `observed` and `control` function pointers
   answer one fixed scenario **differently**. The scenario found is: append
   through a handle, reopen, append again through that same now-stale handle,
   read everything back through a fresh one. An honest fixture answers
   `["Before"]`; one whose `reopen` is empty answers `["Before", "After"]`.
3. The row's `certifies` list — the rules the store buys by lying — is asserted
   to have **passed**, so the day one of them starts rejecting it, the record
   goes red and the row is promoted to `Kind::Mutant`.

> **[Falsified — revision 2]** The paragraph below anticipated the objection and
> then dismissed it. The objection was correct and the dismissal was wrong, in
> both halves. Kept inline rather than deleted, because the *shape* of the error
> is the useful part: the brief named the third honest fixture as hypothetical
> ("could share differently") and did not go and look, and the tree already
> contained it.

**The strongest objection, in its own words.** *Obligation 2's scenario is not a
property of reopening; it is a property of how these two particular fixtures share
their logs. A third honest fixture could share differently and answer
`["Before", "After"]` too, at which point the bar admits a store with no defect.*
That is right, and it is why the scenario is a **witness** and never a rule: it is
a statement about two named fixtures, run against both, and `Fixture::reopen`'s
contract explicitly says a pre-reopen handle *may* stop working rather than that it
must. What the bar buys is narrower than it looks — that *this* row is not empty
prose — and it is exactly what obligation 2 buys one kind over. A reviewer who
wants more has to supply a port-observable consequence for reopening, and the
audit's finding is that there is not one.

### What the review measured, and why both halves of that are wrong

**The third honest fixture is not hypothetical; it is `happenstance-sqlite`.**
`SqliteFixture::reopen` closes the connections the *fixture* holds and
checkpoints the write-ahead log. The file is not replaced — reopening a file does
not replace it — so a handle the caller still owns is its own live
`rusqlite::Connection` onto that same file and keeps working. Run against the
scenario:

```
SqliteFixture        ["Before", "After"]   honest, durable, file-backed
ClosingFixture       ["Before"]            honest, replaces the live log
NoopReopenFixture    ["Before", "After"]   the liar
```

Two honest fixtures on opposite sides, the liar with one of them. The scenario
separates two *styles of `reopen` implementation*, not honest from defective, so
obligation 2 was measuring the wrong thing even when filled in in good faith. A
rule built from it rejects the workspace's only durable adapter. This is now
reproduced inside the testkit's own binary as `LiveHandleReopenFixture`, so the
falsifier stands whether or not anyone re-runs the sqlite crate.

**And the bar was untethered.** `observed` and `control` are `fn() -> String`
with no type-level tie to the store named in the row. The review satisfied the
whole kind with a subject that has no defect at all, twice: `ClosingFixture`
registered under the kind with `control` pointed at a defective third store, and
the degenerate form where both pointers return string literals and touch no
store, no fixture and no `Defect`. Exit 0 in both feature configurations. This
lane reproduced both.

**The repair that was asked for does not close it either**, which is what turned
"fix the bar" into "withdraw the kind". Adding the positive control
`Kind::ModelOnlyMutant` was forced to grow catches the *degenerate* spoof only
because a literal happens to differ from the honest answer. Point the spoof row's
`control` at the honest fixture and keep `observed` as a literal, and the row
passes obligation 2 **and** the positive control — measured. `Witness` escapes
this by holding `<T as Defect>::select`, the store's own function; there is no
analogue for a fixture-level defect, because `Fixture` is not dyn-compatible
(RPITIT) and cannot be held as data at all. So the tie the review asked for is
not available, and a kind whose soundness depends on it should not exist.

**What is left is what could honestly be checked.**
`reopen_over_claiming_is_undetectable_and_this_is_the_record` drives
`NoopReopenFixture` through every rule and asserts: it fails none; it converts
the three durability rules from reported skips into passes while an honest twin
one line apart reports them as skips; and it answers the sharpest proposed
observation identically to an honest durable-shaped fixture. The first two are
the hazard. The third is the standing falsifier for anyone who later proposes the
rule. Nothing claims the defect is demonstrated, because nothing can.

## What is not taken here

### CF-17's `[PROVISIONAL]` and CF-14's `[DEFERRED]` markers

Neither moved, and neither may. `CLAUDE.md` and CF-17's own text
(`spec/SPECIFICATION.md`, the marker's bracket) say moving a maturity marker is an
ADR's act. Adding a MUST to a `[PROVISIONAL]` clause's *text* is not moving it, and
that is what was done. Whether the declaration obligation is enough to move CF-17,
and whether CF-14 leaves `[DEFERRED]` now that its rejection has a named wrong
implementation, are for whoever owns the markers. The two adapters CF-14's deferral
was written against (HS-P0013, HS-P0014) still have not answered, and both are the
ones whose "reopen" is least like closing a file — which is where an empty `reopen`
is most tempting.

### Two `.kb/open-questions/` atoms now cite the wrong lines

CF-17's clause grew, and everything below it in `spec/SPECIFICATION.md` moved
with it. Two atoms cite past that point and are now stale by 37 (it was 27 at
revision 1; revision 2's rewrite of CF-17's `Rejects:` added ten more, and this
table is recomputed rather than adjusted):

| Atom | Cites | Anchor | Now at |
|---|---|---|---|
| `.kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md:43` | `SPECIFICATION.md:8611-8622` | `is recorded as what is still missing rather than as what was always meant` | `8648` |
| `.kb/open-questions/es-6-names-an-unwritable-rule.md:57` | `SPECIFICATION.md:8588` | `nothing. Batch shape's tick is the *one-sided* one` | `8625` |

Both anchors are **unique** in the file, so the repoint is mechanical and was
checked by grepping every occurrence rather than by trusting a tolerance. It was
not applied, because the lane is forbidden to edit `.kb/open-questions/`. Whoever
next runs a KB pass should apply it; nothing in the gate will catch it, which is
the point `citation-anchor-slack.md` in this directory is already making about a
tolerance that accepts a wrong occurrence.

### Whether the MUST wants a clause of its own

It was appended to CF-17 rather than minted as a new clause, on the grounds that
CF-39 sits inside the `MID_BATCH_FAULT` discussion rather than beside it and this
is the same shape. The audit's routing sentence is narrower than that — *"whoever
owns CF-17's `[PROVISIONAL]` marker decides whether `REOPEN` acquires CF-39's
declaration obligation"* — so a clause owner who thinks the obligation deserves a
CF-4x of its own has an argument the lane did not weigh.

## Cost of delay

None for the MUST, which is landed. The two stale citations cost nothing until
someone follows one, and then cost that reader ten minutes. The kind's shape is
free to revisit until a second row is written under it — at which point the
scenario/control shape is load-bearing for two entries instead of one, and
changing it means rewriting both.

---

## Revision record

**Revision 1** (the lane implementing L1-2) recommended `Kind::StatedOnlyDefect`
and shipped it, on the argument that its bar was data in the same binary and
therefore could not be compiled out the way `Kind::ModelOnlyMutant`'s third
obligation could. That argument was true and insufficient: a bar can be
unspoofable-by-`cfg` and still be spoofable by a pointer.

**Revision 2** (after an adversarial review of the implementation) withdraws the
kind. Two measurements did it, both reproduced by this lane rather than taken on
report:

1. The bar is satisfied by a defect-free subject whose two pointers are string
   literals. Adding the positive control the review asked for does **not** close
   this — measured, by pointing the spoof row's `control` at the honest fixture
   the control drives.
2. The scenario separates two honest styles of `reopen` from each other rather
   than honest from defective, with `happenstance-sqlite` on the liar's side. So
   obligation 2 is unsatisfiable in principle for `NoopReopenFixture`, which is
   the only case the kind existed for.

Revision 1's dismissal of the objection is kept inline above with a `[Falsified]`
marker rather than deleted. The error worth remembering is not the conclusion but
the method: the brief called the third honest fixture hypothetical instead of
looking for it, and it was already in the tree.

**What revision 2 does not change.** The bound on L1-2 itself — that no rule can
be written and none was — is unchanged and is now better evidenced than revision
1 argued it. The clause-level MUST on CF-17 stands. Nothing about the maturity
markers moved.

## A number in this lane's reports that is not a measurement

The `86 passed / 3 skipped` against `83 passed / 6 skipped` figures for the
over-claiming fixture are quoted from
`references/evaluation/review-pre-publication-2026-09-03.md` and
`experiments/suite-against-wrong-adapters/results/`. **This lane did not re-run
that experiment, and neither did the review.** Cite them as the audit's
measurement, not as this lane's. The in-tree replacement —
`reopen_over_claiming_is_undetectable_and_this_is_the_record`'s second assertion
— is the part that runs on every gate.
