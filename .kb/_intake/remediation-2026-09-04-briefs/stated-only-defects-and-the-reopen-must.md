# L1-2 is closable only as far as *stating* it. Is `Kind::StatedOnlyDefect` the right place to put what cannot be checked, and who owns CF-17's marker now that its text carries a MUST?

Short answer up front: **the kind earns its place because its bar is data in the
same binary and cannot be compiled out — but it is one row wide, and a kind with
one row is a shape nobody has argued against yet.** Two things it touches are
somebody else's and are not taken here: CF-17's and CF-14's maturity markers, and
two `.kb/open-questions/` atoms whose line citations this change made stale.

**This brief did not get the two-critic pass the original thirteen had.** It was
written by the lane implementing `L1-2`, in the same session as the change it
describes. Read it with that discount applied.

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
3. **`Kind::StatedOnlyDefect`**, because `Kind::Mutant` rejects an empty `fails`
   list outright — rightly, and the assertion was not relaxed.

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

CF-17's clause grew by 27 lines, and everything below it in
`spec/SPECIFICATION.md` moved with it. Two atoms cite past that point and are now
stale by exactly 27:

| Atom | Cites | Anchor | Now at |
|---|---|---|---|
| `.kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md:43` | `SPECIFICATION.md:8611-8622` | `is recorded as what is still missing rather than as what was always meant` | `8638` |
| `.kb/open-questions/es-6-names-an-unwritable-rule.md:57` | `SPECIFICATION.md:8588` | `nothing. Batch shape's tick is the *one-sided* one` | `8615` |

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
