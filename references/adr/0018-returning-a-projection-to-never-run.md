# ADR-0018: Returning a projection to "never run" — scope, atomicity, and refusal

- **Status:** accepted
- **Date:** 2026-08-13
- **Settles:** PS-16 – PS-20 (`spec/SPECIFICATION.md:5172-5276`), and the
  three-variant `Checkpoint` at `:4643-4658`
- **Builds on:** ADR-0007 (checkpoints are per `(store, ProjectionId)`),
  ADR-0017 (`type Batch;` and `ProjectionProbe::probe_delete_all`)
- **Repairs:** nothing. PS-19 carries a known pairing defect inside this ADR's
  clause range; it is named, scoped out and attributed below, and the repair
  lands at `unstable-projection-gate-and-clause-disposition`.
- **Evidence:** `references/adapter-shapes.md`,
  `references/evaluation/ps-clause-pairing-sweep.md`

## The question, as the runbook posed it

> **0018** | 6 | How is a projection returned to "never run", what is that
> operation's transactional scope, and what may refuse it? (PS-16 – PS-20)
> — `RUNBOOK.md:297`

Three halves in the title and four decisions underneath them, **of three
different strengths**, and the point of this section is that the ADR must not
level them. `kb-playbook-one-decision-per-adr-title-001`'s discriminator is not
the conjunction — it is what each half rests on:

| Decision | Rests on | Strength |
|---|---|---|
| `reset(batch, id)` is one unit of work | a night-desk incident it models, and the absence of any adapter that can express clearing outside the batch | **provisional**, falsifier and phase named |
| scope is `(store, ProjectionId)` | **a prior decision**: ADR-0007 already fixed checkpoints there | **settled elsewhere — cited, not re-derived** |
| an adapter may refuse | a division of labour, plus a live count of implementers that is currently zero | **provisional**, falsifier and phase named |
| `Checkpoint` is a three-variant enum | a type that can spell a meaningless state | **settled by being made** |

Levelling these would be the exact failure the playbook records: a strong
decision and a weak one under one title, and the weak one reversed within days.

## Context

### The evidence, and the honest form of its absence

**No skeleton exercised `reset`.** There is no compiler transcript to quote here,
and inventing one would be a defect rather than a stronger record —
`references/adapter-shapes.md:29-33` argues exactly this in the document's own
voice: a table showing only `error[E….]` *"would rank it the most compatible
adapter in the workspace when it is the least."*

So the transcript this ADR quotes is a **stated absence**, from §5, *What a
skeleton does not prove* (`references/adapter-shapes.md:286-303`):

> | Batch shape | That an owned batch works for SQL, HTTP **and** a graph handle
> — and that a *borrowed* GAT also still works | **The projection conformance
> suite, which does not exist. Phase 6** |
> | Durability | **Nothing. Every body is `todo!()`** | Phase 8 |
>
> A skeleton falsifies a signature. A far end is a passing implementation, and
> phase 2 produced none.

Read against this ADR that is not a hole, it is the shape of the risk: every
decision below is a **signature-level** decision, taken with no durability
evidence at all, and the clauses say so through their maturity markers. PS-16 and
PS-18 are `[PROVISIONAL]`; PS-17, PS-19 and PS-20 are `[FROZEN]` and none of the
three rests on a durability measurement — PS-17 rests on a prior decision, PS-19
and PS-20 on a type-level distinction.

### The incident the atomicity decision models

`spec/SPECIFICATION.md:5181-5187` records what the two-statement runbook procedure
did on a real night desk:

> the truncate committed at 02:46:31, the pod died at 02:46:33, the runner
> restarted, read the old checkpoint, resumed past it, applied sixty-one events
> into an empty table and **reported healthy.**

That is the wrong implementation PS-16 rejects, and it is worth carrying in the
record rather than asserting atomicity abstractly: the failure is not data loss,
it is a read model that is silently wrong *and reports success*, which is the
failure mode the whole port exists to make impossible.

### Why `reset` does not clear the rows

*"The design question is not whether to add `reset` but **how it clears the
rows**, because the port has no idea what the read model is. The answer is that
it does not clear them"* (`spec/SPECIFICATION.md:5165-5170`). `reset` is
`commit`'s dual: it takes a batch the caller has already filled with its own
deletes. Same transaction, same atomicity, no new knowledge required of the
adapter — and `ProjectionProbe::probe_delete_all` (ADR-0017 §2) exists so the
suite can exercise it without knowing either.

### The trap inside this ADR's clause range

**PS-19 carries a known pairing defect, and this ADR does not repair it.**

Its `MUST` is scoped *"After a successful `reset`"*; §4.11 additionally assigns it
`fresh_projection_has_no_checkpoint`, which asks about an id that has never been
seen. `references/evaluation/ps-clause-pairing-sweep.md` re-derived the defect
from the clause text at `spec/SPECIFICATION.md:5218-5223` rather than inheriting
it, and named a **plausible first cut** that exposes it: a store whose
`checkpoint` is `SELECT position, authority FROM checkpoints WHERE id = ?` and
whose Rust resolves the missing row with
`.unwrap_or(Checkpoint::Live { through: FIRST })` — the cheapest default, since
`SequencePosition` is `NonZeroU64` and `FIRST` is its minimum — and whose `reset`
writes an explicit `NeverRun` sentinel row. It answers `NeverRun` post-reset,
satisfying the `MUST` verbatim and distinguishable from a commit at `FIRST`
exactly as required, and answers `Live` for an unseen id. The sweep grades it
`independent`: it satisfies every other `PS` clause's `MUST`.

The mechanical test decides who owns the fix. *A correction to a `[FROZEN]` clause
is a repair if the set of implementations the clause admits is unchanged;
otherwise it is a gap, and a gap is a decision's* (`.kb/decisions/README.md:20-22`).
Widening PS-19 to cover the never-seen id makes the `unwrap_or(Live)` store
non-conformant, so the admitted set changes, so it is a **gap** — a new decision
atom's under `kb-playbook-repair-frozen-clause-001`, landing at
`unstable-projection-gate-and-clause-disposition` on the sweep's evidence.

**What this ADR owes is one sentence saying so, and it has now said it.** Nothing
`[FROZEN]` is line-edited here, no maturity marker moves, and
`spec/SPECIFICATION.md` is not in this decision's boundary.

The sweep's family verdict is **isolated** — three `independent` same-shape
defects against a threshold of five, two of them inside §4.11's table against a
threshold of three — so this ADR's scope is not widened and no re-plan is raised.
One further row inside PS-16 – PS-20 is worth naming even though it is not a
defect: PS-18's rule is described as running *"against a testkit fixture store
configured to protect one id"*, which in an adapter suite must mean the adapter's
own fixture. That wording ambiguity is `reset-rules`' and
`projection-capability-skips`' to close, not this ADR's.

## Decision

### 1. `reset(batch, id)` applies the batch and returns the checkpoint to `NeverRun`, as one unit of work

PS-16, `[PROVISIONAL]`.

The caller fills a batch with its own deletes and hands it to `reset`. The
adapter applies it and moves `id`'s checkpoint to `NeverRun` in the same
transaction. What it rejects is the two-statement runbook procedure on two
connections — and the record carries the failure it models (§Context) rather than
asserting atomicity in the abstract, because "atomic" is what everyone believes
their procedure already is.

**Falsifier and phase, stated:** falsified by an adapter whose read-model
clearing cannot be expressed through the same batch that carries ordinary writes.
*A store whose `TRUNCATE` cannot participate in the checkpoint transaction is the
shape to watch.* Owned by the projection-port phase. This is a claim about stores
that have not been built, so it is marked provisional rather than settled — and
the observation that would refute it is a `TRUNCATE`, not an opinion.

### 2. Reset is scoped to one `(store, ProjectionId)` pair — cited, not re-derived

PS-17, `[FROZEN]`.

**This half is settled by a prior decision.** ADR-0007 already fixed checkpoints
per `(store, ProjectionId)`; PS-17 is *"the same decision applied to the operation
that removes one"* (`spec/SPECIFICATION.md:5190-5191`). This ADR cites
`.kb/decisions/0007-projection-runner-decodes.md` and stops. Re-arguing it here
would produce a second, independently-worded statement of one commitment, which is
how two decisions come to disagree without anyone reversing either.

What the clause rejects is worth carrying: a `reset()` that truncates the
checkpoint table. *"Cheap, obvious, and it destroys the append-only regulatory
ledger sharing the file — Kestrel Cold Chain's `van_stock` must reset several
times a day and `fgas_ledger` must never"* (`spec/SPECIFICATION.md:5196-5199`).

### 3. Refusal is a port mechanism; the policy lives in the domain

PS-18, `[PROVISIONAL]`. An adapter MUST be *able* to refuse a reset through
`ResetError::Refused`; a refusal MUST leave both the read model and the checkpoint
unchanged, and MUST NOT be reported as success.

The alternative that lost is named in the clause and is named again here because
it is genuinely attractive: **refusal as purely a typed-layer concern.** That is
where the *policy* belongs — the projection knows that `audit_trail_export` is a
hash chain a regulator already holds, and the store does not. But *"a policy with
no port-level mechanism is bypassed by anyone holding the store, which is every
operator with a runbook"* (`spec/SPECIFICATION.md:5211-5215`). The port supplies
the mechanism; the domain decides what to protect. It is the same division of
labour the specification takes for sync compensation.

**Falsifier and phase, stated:** falsified if no adapter ever implements
protection, in which case the variant is dead weight and refusal belongs solely to
the typed layer. *Evaluated at the exit of the projection-port phase by asking
whether the SQLite adapter implemented it.* This is question 4 of §4.1a's six —
*"Does anyone **call** these?"* — and it is the one most at risk of never being
asked, because *"'nobody ever needed it' is not something you can observe by
waiting"* (`spec/SPECIFICATION.md:4812`). The phase exit that counts callers
is the only thing that makes this marker mean anything.

**What `ResetError::Refused` carries is not decided here.** Whether the variant
carries the store's stated reason or stays bare is a public-API shape question,
and it belongs to the design record (`projection-api-design-record`, project
AC-007) where it is decided out loud rather than defaulted to silence. This ADR
fixes the mechanism and its atomicity guarantee; it does not fix the payload.

### 4. `Checkpoint` is a three-variant enum

PS-19 and PS-20 rest on it, and it is **settled by being made**:

```rust
#[non_exhaustive]
pub enum Checkpoint {
    NeverRun,
    Live { through: SequencePosition },
    Rebuilding { through: SequencePosition },
}
```

The alternative is `(Option<SequencePosition>, bool)`, and it loses on one
sentence: it can spell `(None, true)` — *"authoritative, never run — which means
nothing"* (`spec/SPECIFICATION.md:4643-4658`). This is the crate's own *illegal
states are unrepresentable* line applied where a reader will otherwise write
`if let Some(p) = checkpoint` and get it wrong.

Two consequences the enum buys, both normative:

- **PS-19**: after a successful `reset`, `checkpoint(id)` returns
  `Checkpoint::NeverRun`, and that is distinguishable from
  `commit(empty_batch, id, SequencePosition::FIRST, Live)`. Under the tuple it is
  not: `commit(empty, id, FIRST)` reads back as `Some(1)`, the runner advances
  past it, *"and event 1 is skipped permanently, silently, and nobody will ever
  find it"* (`spec/SPECIFICATION.md:5245-5247`).
- **PS-20**: a runner resumes strictly *after* the checkpoint's position, and
  starts at the store's first position, **inclusive**, when the checkpoint is
  `NeverRun`. The off-by-one runs in both directions, and the clause names its
  dependency: `ReadOptions::from` is an inclusive lower bound while `Guard::after`
  is exclusive. That obligation is **discharged, not withdrawn** —
  `SequencePosition::next()` is `NonZeroU64::checked_add` in `match` form and
  `position_next_signals_overflow` asserts it — so a runner may now be written
  against it, and `checkpoint.next()` is a correct resume point even on a store
  with gaps.

## Consequences

**For an operator with a runbook.** Rebuilding one projection stops being raw DDL
against the adapter's checkpoint table. The procedure is: fill a batch with the
deletes you want, call `reset`, and either it all happened or none of it did.
There is no window in which the rows are gone and the checkpoint is not.

**For an adapter author.** `reset` costs one method whose body is `commit`'s with
a different checkpoint write, and `Refused` costs nothing at all until the adapter
chooses to protect something. The one genuinely new obligation is that a refusal
must be *observably* inert — both halves unchanged — which is
`refused_reset_changes_nothing`'s to check and needs a fixture that can configure
a protected id.

**For the suite.** Two rules become writable that were not:
`reset_clears_rows_and_checkpoint_together` (through `probe_delete_all`) and
`reset_is_scoped_to_one_projection`. `refused_reset_changes_nothing` needs a
capability the fixture may decline, and by `CLAUDE.md`'s rule a declined
capability still runs and reports its stated reason rather than vanishing from the
binary.

**What stays open, and where.** PS-16 and PS-18 are provisional on observations
nobody has made yet — a `TRUNCATE` that cannot join the transaction, and an
adapter that actually implements protection. PS-19's pairing defect is *known and
not repaired here*. And the rebuild question this ADR touches but does not settle
— in-place versus rebuild-into-a-second-`ProjectionId`-and-swap — is §4.1a's
question 3, discriminated by storage rather than by preference: *"a store that
cannot hold two copies of a 4.1M-event read model has no swap available"*
(`spec/SPECIFICATION.md:5334-5338`). This ADR assumes neither.

**One thing deliberately not touched.** A *boundary-scoped* projection checkpoint
would reopen ADR-0013's globally frozen visibility invariant. It surfaced while
writing §2 and was filed rather than absorbed:
`kb-open-question-global-vs-boundary-visibility-001` owns it, and settling a
frozen invariant in passing inside a reset ADR is precisely the move that has to
be refused.

## Alternatives rejected

**The two-statement runbook procedure** — truncate the read model, then reset the
checkpoint, on two connections. Rejected by the incident it caused, quoted in
§Context: sixty-one events applied into an empty table, reported healthy. It is
what the port replaces, and it is not a straw man — it is what was actually run.

**`reset` clearing the rows itself.** Rejected because the port has no idea what
the read model is. Any version of this requires the adapter to know which tables,
labels or keys belong to a `ProjectionId`, which is precisely the knowledge PS-9
keeps out of the port. The batch-of-caller-deletes design needs no new knowledge
and reuses `commit`'s transaction.

**Refusal as purely a typed-layer concern.** Rejected in §3. The policy belongs
there; the mechanism cannot, because a policy with no port-level mechanism is
bypassed by anyone holding the store.

**`(Option<SequencePosition>, bool)` for the checkpoint.** Rejected in §4: it can
spell `(None, true)`. Also rejected in its weaker form — a bare
`Option<SequencePosition>` — which cannot distinguish a rebuild in flight from an
authoritative read model at all and is what makes PS-24 necessary.

**Re-deriving PS-17's scope from first principles.** Rejected on process grounds
rather than substance: ADR-0007 already decided it, and a second independent
derivation of one commitment is two things to keep in agreement.

**Repairing PS-19 here.** Rejected in §Context. It is a gap, not a repair, and a
gap is a new decision atom's — written under
`kb-playbook-repair-frozen-clause-001`'s discipline at a story whose whole purpose
is clause disposition, with the sweep as its baseline. An ADR that both discovers
and decides cannot be audited.
