# ADR-0030: The checkpoint reports the commits that happened

- **Status:** accepted
- **Date:** 2026-08-15
- **Phase:** 6 (unscheduled — the queue had no number for it, on ADR-0029's
  precedent: `RUNBOOK.md:287`)
- **Amends nothing, edits nothing.** PS-1, PS-19, PS-21 and PS-22 are
  byte-identical before and after. This decision mints **PS-38** and records four
  findings; not one existing `MUST` sentence moved.
- **Consumes:** [`references/evaluation/ps-clause-pairing-sweep.md`](../evaluation/ps-clause-pairing-sweep.md),
  the census that produced the finding and declared its threshold before counting.

## Context

`references/evaluation/ps-clause-pairing-sweep.md` swept all thirty-seven `PS`
clauses against the rules each is paired with, asking one question per pairing:
*is there an implementation that satisfies the clause's `MUST` verbatim and fails
that rule?* Seven rows came back `defective` and one `undetermined`. The verdict
on the hypothesis under test — that §4.11's table was populated from a systematic
assumption — was **ISOLATED**, with a qualification that matters more than the
headline: the shape recurs *outside* the table, so the cause is a habit of writing
the rule to a clause's **intent** rather than to its **sentence**
(`ps-clause-pairing-sweep.md:322-351`).

Four of the seven defective rows trace to one missing sentence, and this ADR is
about that sentence.

**Nothing in §4 obliged a `commit` to advance anything.** PS-1's `MUST` is a
*coupling* — the read-model write and the checkpoint write become durable together
or not at all — and a store that makes *neither* durable satisfies it through the
"or not at all" arm. Three rules nevertheless rest on progress:

| Rule | Clause it was hung on | What it additionally asserts |
|---|---|---|
| `commit_advances_the_checkpoint` | PS-1 | the checkpoint moved to *P* |
| `commit_accepts_a_position_the_batch_did_not_write` | PS-21 | "and assert the checkpoint advanced" |
| `commit_rejects_a_regressing_position` | PS-22 | that there *is* a current checkpoint to regress below |

And a fourth, `fresh_projection_has_no_checkpoint`, was hung on PS-19, whose
`MUST` is scoped *"After a successful `reset`"* and says nothing about an id that
has never been seen.

**The exposing implementation is a plausible first cut, and the workspace already
knows its shape.** A store whose backing state lives per **handle** rather than
per store — one whose `connect()` mints a fresh map instead of a fresh handle onto
a shared one. Nothing survives a handle, so `commit` returns `Ok`, both the row
and the checkpoint are absent through the fresh handles
`commit_is_atomic_with_the_read_model` reads with (the *"or not at all"* arm,
satisfied verbatim), and `commit_advances_the_checkpoint` fails. It is the fixture
bug CLAUDE.md's *"one fixture instance is one isolated backing store; each
`connect()` on it is one handle onto that store"* exists to forbid. Every other
`PS` `MUST` is satisfiable inside one handle, which is what makes the defect
**independent**: nothing else in §4 rejects that store
(`ps-clause-pairing-sweep.md:258`).

**One supporting sentence of the prior did not survive the sweep, and it changed
the repair.** `.kb/open-questions/ps-1-states-no-progress-obligation.md:48-51`
said progress is *"stated by no clause's `MUST` anywhere in the document"*. Read
literally, **PS-23 states it**: *"One `commit` advances exactly one
`ProjectionId`"* — "exactly one" excludes zero. But PS-23 is about fan-out
*scope*; its *Rejects* field names *"an adapter with a single-row checkpoint
table"*, which is the "not more than one" reading, and the progress half arrives
as a side effect of a word chosen for a different purpose. Worse, PS-23 is
`[PROVISIONAL]`, falsified by *"a pair of read models in one store that must be
mutually consistent at every observable instant"* — so the only normative
statement of progress in the document is scheduled to be rewritten by a question
about Norvant's control tower. **The obligation was misfiled, not absent**
(`ps-clause-pairing-sweep.md:355-395`).

## Decision

**Mint PS-38, `[PROVISIONAL]`, in §4.7:**

> A successful `commit(batch, id, position, authority)` MUST advance `id`'s
> checkpoint to `position`, and a `ProjectionId` no successful `commit` has named
> MUST read as `Checkpoint::NeverRun`.

Its falsifier is a store that answers `checkpoint` from a replica that may lag its
own `commit` — the shape a projection store over an eventually-consistent read
model has. If that is real the obligation narrows to *"a subsequent read through
the same handle"* and every rule downstream gains a handle constraint. It is owned
by the first projection adapter over storage this workspace does not control.

Its rules are `commit_advances_the_checkpoint`, which exists, and the new
`fresh_projection_has_no_checkpoint`, which does not. It rejects the per-handle
store above.

**Both sentences are one proposition**, which is what keeps this one decision
rather than two: *the checkpoint reports the commits that happened*. The first
sentence says a commit is visible in it; the second says nothing else is.

**Four clauses gain a recorded finding and lose nothing.** PS-1, PS-19, PS-21 and
PS-22 each carry a paragraph naming what its rule asserts beyond its own sentence
and pointing at PS-38. PS-1's and PS-19's paragraphs also answer the question each
clause explicitly assigned to phase 6 — *"either a sentence in this clause or a
clause of its own"*, and *"whether this clause widens or a new one says it"* — with
the same answer: a clause of its own.

**PS-8, PS-13, PS-28 and PS-29 are recorded and routed, not repaired here.** Each
carries its own finding paragraph in the specification naming its exposing
implementation, its strength and its owner. None of them is this decision's:
PS-8's and PS-13's are `dependent` and inside ADR-0017's clause range; PS-29's is
`independent` but its rule does not exist and its observability design is the
typed-layer phase's, deferred there by ADR-0019; PS-28's is `undetermined` and
resolves by defining *"the last good position"* when the rule is written.

## Consequences

**§1.3's hand count moves by one clause and one `[PROVISIONAL]`**: 200 → 201
clause IDs, 198 → 199 normative, 49 → 50 `[PROVISIONAL]`. Corrected by hand,
outside the generated markers, because §1.3 is the only count in the document a
person computed and generating it would destroy the property it is used to prove
(`xtask/src/spec_trace.rs:37-57`).

**Three defective pairings become sound and one stays daggered.**
`commit_advances_the_checkpoint`, `commit_accepts_a_position_the_batch_did_not_write`
and `commit_rejects_a_regressing_position` now have a clause whose `MUST` entails
what they assert. `fresh_projection_has_no_checkpoint` still does not exist; it is
marked new wherever it is named and renders `†` in §7.2, which is what the legend
there means.

**No adapter gains or loses conformance today.** The per-handle store PS-38
rejects already failed `commit_advances_the_checkpoint`; what changes is that the
failure is now a clause violation rather than a rule reaching past one. Every
adapter that passed the suite before this decision passes it after.

**PS-38 is `[PROVISIONAL]` and that is not a hedge.** Its falsifier is a real
storage shape this workspace intends to build against — Neon over one-shot HTTP
and a Durable Object are both candidates — and PS-38 is written where a `[FROZEN]`
neighbour could not have absorbed it, which is the whole reason it is a new clause.

## Alternatives rejected

**Add a sentence to PS-1.** PS-1 is `[FROZEN]`, and adding progress to it changes
the set of implementations it admits — which is a **gap** rather than a repair by
the classifier at `.kb/decisions/README.md:20-22`, and a gap is a decision's. It
would also conflate two propositions in one clause: coupling and progress are
independently falsifiable and an adapter can satisfy either without the other.
Rejected on both counts.

**Split PS-23's two readings and put progress where it is already half-written.**
Tempting, because the word is already there. Rejected because PS-23 is
`[PROVISIONAL]` on a question about fan-out that has nothing to do with progress:
an obligation three other rules rest on cannot live on a clause scheduled to be
rewritten by an unrelated falsifier. Splitting it would also mean editing a clause
to mean something its *Rejects* field says it does not.

**Leave it recorded and route it to phase 7.** This is what PS-1's own paragraph
forbade: *"Phase 6 owns the repair."* Routing an obligation four rules already
enforce would leave the specification asserting, through §7.2's *Conformance rule*
column, that four clauses are falsified by tests they do not entail — for another
phase.

**Repair all seven defective rows in one decision.** Rejected under
`.kb/playbooks/one-decision-per-adr-title.md`'s bar: an ADR that cannot be stated
as one question is two ADRs. Four of the seven are one question and are answered
here; the other three are three different questions with three different owners
and are recorded in the clauses they are about.
