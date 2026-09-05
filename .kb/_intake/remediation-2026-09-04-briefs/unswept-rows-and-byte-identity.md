# ES-18 says *byte-identical*. No adapter in this workspace restores byte-identity, and one now says so in its own documentation. Does the clause's second sentence get amended, or does the conformance reading become the clause?

Short answer up front: **amend ES-18's second sentence to say what its own
conformance rules already ask** — *a rejected append leaves the store holding none
of the batch* — and record the position counter and unswept refuse as permitted
residue. Held at **medium** confidence. The strongest argument against is that
"byte-identical" is a deliberately unforgiving word chosen to stop exactly this
kind of erosion, and that the lane proposing the softening is the lane that found
it inconvenient.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** It was written by the lane implementing `Q-01`, in the same session
as the change it describes. It carries its own strongest objection and answers it,
which is the form, but nobody independent argued the other side. Read it with that
discount applied.

---

## What happened, so the question is legible

`Q-01` of the pre-publication review
(`references/evaluation/review-pre-publication-2026-09-03.md:865-908`) found that
`happenstance-cloudflare` had a reachable, tested, *documented* state in which a
rejected `append` left rows of the batch in the log:
`CloudflareEventStoreError::PartialBatch`, produced when the compensating `DELETE`
that undoes a failed batch itself throws. **ES-18 [FROZEN]**
(`spec/SPECIFICATION.md:3440-3446`) admits no exception:

> Either every event in the batch lands or none does. A rejected append MUST leave
> the store byte-identical.

The lane took the audit's option (a) — *"restructure the write path so no
compensating statement is load-bearing"* — rather than option (b), amending the
clause, because the clause is `[FROZEN]` and a lane may not weaken one. The fix
uses the primitive the audit's own remediation paragraph named: **a single
statement is backed out whole**, and the batch's last statement covers the whole
batch. `origin_position` is now the commit marker; a row without it is not an
event, and `head`, the read plan and the append guard all filter on it. Two new
`wasm32` cases ask ES-18 through the port over the two faults that used to produce
a partial batch, and both were red before the change.

**That closes the clause's first sentence and leaves the second one open**, which
is what this brief is about.

## What is true today, quoted

Three facts, and the third is the one that makes this a clause question rather than
an adapter question.

**1. An unswept row survives a failed append.** When the discard throws,
`PartialBatch` still reports it and the rows are still there. They carry no
identity, so nothing that answers a question about events can see them — but they
occupy storage, and on a Durable Object storage is metered.

**2. The position counter moves either way.** This is in the adapter's own
documentation and predates the change
(`crates/happenstance-cloudflare/src/event_store.rs`, `discard_from`):

> What this deliberately does not do is reset the position counter. The schema's
> `AUTOINCREMENT` keeps its high-water mark across the delete, so a discarded batch
> leaves a **gap** rather than positions to be handed out twice — and it has to,
> because an `EventId` is `(store, position)` and a reused position is two
> different events wearing one identity. Gaps are permitted by the specification;
> reuse is not.

So *even a completely successful compensation* does not restore byte-identity, and
it must not: `sqlite_sequence` has to keep its high-water mark or VT-8 and VT-11
break. Byte-identity and position-uniqueness are in direct conflict for any store
that assigns positions from a monotone counter, which is every store in this
workspace.

**3. The clause's own rules ask the weaker question.** ES-18's `Rule:` block names
`append_is_atomic`, `condition_rejection_leaves_store_unchanged` and
`append_is_atomic_under_a_mid_batch_fault`, and describes the last of them as

> asserting the store afterwards holds all of the batch or none of it, with which
> of the two decided by what `append` answered.

*Holds*, through the port. Nothing in the suite compares bytes, and nothing could:
`Fixture` exposes `connect`, `reopen` and `arm_mid_batch_fault`, and no byte-level
snapshot at all. ES-22's rule does carry a `snapshot_of` comparison, but it is a
snapshot of what the *port* reports, not of the medium.

So the second sentence has never been checked by anything, is unsatisfiable by any
counter-assigning store, and disagrees with the first sentence's own rules about
what the clause means.

## The question

Does ES-18's second sentence stay as written, get amended, or get demoted?

### Option A — amend the second sentence to the conformance reading

Replace *"A rejected append MUST leave the store byte-identical"* with something
that says what the rules ask: **a rejected append MUST leave the store holding none
of the batch — no event of it readable, countable by `head`, or visible to an
append condition.** Add non-normative commentary naming the two permitted
residues: a consumed stretch of the position space, and medium-level refuse an
adapter may not have been able to sweep.

- **Costs a caller**: nothing they could previously rely on. No caller can observe
  bytes through the port.
- **Costs an adapter author**: a weaker bar, and that is the objection below.
- **Semver**: none. It relaxes an obligation nothing enforced.
- **Buys**: the clause stops asserting something no adapter can meet, and stops
  being the thing an adapter author has to quietly decide to read loosely. It is
  the honest form of what phase 8 and phase 9 both shipped.
- **Requires**: an ADR. ES-18 is `[FROZEN]`; changing its text is exactly what a
  new decision record is for, and no lane may do it in passing.

### Option B — leave the sentence and add commentary

Keep the normative text; add a non-normative paragraph recording that byte-identity
is unattainable for a counter-assigning store, that the rules ask the weaker
question, and that this is a known gap.

- **Buys**: no ADR, no clause change, and the tension is at least written down
  where the next reader meets it.
- **Costs**: a `[FROZEN]` clause whose commentary contradicts its own MUST, which
  is a worse artefact than either half alone. It is also the shape `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`
  already records as costing: a marker that binds the design without anything
  checking it.

### Option C — make the second sentence true by making the store byte-identical

Reset the position counter on a discard, and require the compensation to succeed.

- **Buys**: literal compliance.
- **Costs**: it is unsound. Resetting the counter reissues positions a batch was
  already given, and an `EventId` is `(store, position)`, so two different events
  end up wearing one identity — VT-8 and VT-11, both `[FROZEN]`, both broken to
  satisfy ES-18. And "require the compensation to succeed" is not implementable
  without transaction control, which a Durable Object does not offer through
  `sql.exec()`. This option is listed because it is the one a reader reaches for
  first, and it should be visibly closed rather than silently absent.

## Recommendation

**Option A**, at medium confidence, sequenced with whoever owns ES-18 at phase 12's
release gate rather than now.

The argument is that the clause is currently three things at once — a MUST, a set
of rules that ask something weaker, and a body of adapter documentation that
follows the rules — and only one of the three can be the specification. The rules
are what adapters are actually held to, they are what `cargo xtask ci` runs, and
they are what both shipping adapters were written against. Amending the sentence
changes no behaviour anywhere; it changes which of the three a future reader
believes.

## The strongest argument against, in its own words

> *"Byte-identical" was chosen because every softer phrasing is one an adapter
> author can talk themselves past, and this brief is the first attempt to talk past
> it — filed by the lane that found the word inconvenient, in the same session,
> proposing that the specification move to where its code now is. That is the
> direction of erosion the whole `[FROZEN]` mechanism exists to resist. The
> position-counter argument proves the sentence is imprecise, not that it should be
> weakened: the honest amendment is "byte-identical except for the position
> counter", which keeps the bar and fixes the flaw. And leaving unswept rows on the
> medium is a real cost the weaker sentence stops anyone having to justify.*

That is a real argument and it is why this is medium rather than high. Two
concessions to it. First, *"byte-identical except for the position counter"* is a
genuine fourth option and may well be the right one — this brief does not close it,
and whoever writes the ADR should cost it against Option A directly; the reason it
is not the recommendation is that it is still unenforceable by any instrument the
testkit has, so it would restore the "asserted but unchecked" shape rather than
remove it. Second, the objection is right that unswept refuse should not become
invisible: whatever the sentence says, the adapter's `PartialBatch` and its
documentation are what tell an operator the sweep failed, and neither should be
softened.

## Cost of delay

**Low, and it does not compound.** Nothing blocks: the adapter meets the clause's
rules, the gate is green, and the residue is documented at the site that produces
it. What delay costs is that the *next* adapter author reads a MUST their store
cannot meet and has to decide alone how to read it — which is what happened here,
twice, and neither time was recorded.

## What this does not settle

- **Whether the testkit should be able to arm a fault in the compensation.** The
  audit's own separable half (`:908`): `Fixture::arm_mid_batch_fault` arms a
  trigger `BEFORE INSERT ON event`, which a `DELETE` does not fire, so no
  conformance rule can reach the branch this lane fixed. Both of this lane's new
  cases are crate-local `wasm32` tests for exactly that reason. That belongs to
  `happenstance-testkit`, and it is what would make the choice observable to every
  adapter rather than to this one.
- **Whether the unswept rows should be swept opportunistically.** A one-statement
  `DELETE FROM event WHERE origin_position IS NULL` at the top of `append`, or at
  `migrate`, would bound the accumulation. It was deliberately not added: it puts a
  write on the hot path to bound a cost that only materialises after a failure the
  adapter already reports, and the trade is a measurement nobody has taken.
- **`PartialBatch`'s name.** It now reports unswept refuse rather than a partial
  batch, and the name says the older thing. It was kept because
  `xtask/src/proof.rs`'s `CLOUDFLARE_UNIT_TESTS` pins
  `a_batch_whose_discard_also_fails_reports_both_failures` by name, `xtask` was
  held by another lane in this wave, and renaming the variant without renaming that
  test would have been the worse half of the change. A rename is free — the crate
  is not on the registry — and belongs with whoever next opens `xtask`'s registry.
- **The `# Cancellation` half of the same crate.** That is `Q-02`, and it is a
  different clause.

---

## Premise correction — the generalisation is false, refuted by execution

**Added by the wave integrator, 2026-09-04, after adversarial review.** This
section corrects a stated reason and takes no decision; the recommendation above
is the author's and stands or falls on its own merits.

The brief argues:

> Byte-identity and position-uniqueness are in direct conflict for any store that
> assigns positions from a monotone counter, which is every store in this
> workspace.

**That is false for `happenstance-sqlite`, which is the store the sentence most
needs to be true of.** SQLite's `AUTOINCREMENT` high-water mark lives in
`sqlite_sequence`, an ordinary table under ordinary transaction control, and a
rollback restores it. Executed against the crate's own driver:

```
test autoincrement_high_water_is_restored_by_a_rollback ... ok
test result: ok. 1 passed; 0 failed
```

with the counter observed advanced *inside* the transaction, `seq_after ==
seq_before` after rollback, and the next committed row landing at the position
that was next before the rolled-back batch. No `EventId` can collide, because the
rolled-back positions were never handed to a committed event.

`SqliteEventStore::append_locked` is exactly that shape — `transaction_with_
behavior(TransactionBehavior::Immediate)` wrapping condition evaluation and the
whole batch, committed only on success (`crates/happenstance-sqlite/src/event_store.rs:595`,
`:608`). So the conflict the brief generalises to "every store in this workspace"
is specific to a store that **cannot** use transaction control and must
compensate with `DELETE` — the Durable Object, and only it.

**What this does and does not do to the recommendation.** It does not refute the
conclusion: SQLite makes no literal byte-level promise about page or freelist
state after a rollback, so *byte-identical* may still be the wrong word in a
clause, and the amendment may still be owed. What it removes is the argument
offered for it. A brief proposing an amendment to a **`[FROZEN]`** clause on the
ground that no store can satisfy it should not rest that ground on a claim about
"every store in this workspace" that is true of one of them.

The brief's own opening records that it did not get the author → two-critic →
revision pass the original thirteen had. This is the correction that pass would
have produced, arriving late. **The premise needs rewriting before this reaches
`.kb/` or an ADR** — the honest form is narrower and, on the evidence, still
sufficient: the conflict is real for a store whose atomicity comes from
compensation rather than from a transaction, and `happenstance-cloudflare` is
that store.
