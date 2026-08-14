# Staged: the PS clause/rule pairing sweep answers two sub-questions and amends one atom

**Staged 2026-08-13** for the next `/redkiln:kb-ingest` wave. **Not an atom.**
Nothing here has been hand-written into `.kb/` — `CLAUDE.md` (*Where the work
lives*) reserves atom authorship to the ingest, and the first attempt at
hand-writing them was reverted at `0269720`.

**Source of every claim below:**
[`references/evaluation/ps-clause-pairing-sweep.md`](../../references/evaluation/ps-clause-pairing-sweep.md),
dated 2026-08-13 and pinned to `2136dde`, registered in
`references/evaluation/README.md` under *Later additions, which are neither*.

**Wave-id note for whoever runs the ingest.** Two waves already sit under
`.kb/_governance/integration-waves/`: `2026-08-10-intake` and
`2026-08-10-intake-2`. Give this one an id that collides with neither — the
second wave overwriting the first's audit trail is the failure mode
(`2026-08-13-ps-sweep` is free).

**Adjudication bias, stated up front:** every claim below is an **AMEND** to an
atom that already exists. **No new atom is proposed.** Two open questions gain an
answer to one of their ordered sub-questions and keep their bodies; one of them
additionally gains a correction to a sentence in its body that re-derivation
refutes. If the wave's adjudicator finds itself minting a `decision` atom from
this document, it has misread it: the sweep decides nothing, and the three
decisions phase 6 owes are `projection-decision-atoms`' (ADR-0017, ADR-0018,
ADR-0019).

---

## 1. AMEND `kb-open-question-ps-1-no-progress-obligation-001`

File: `.kb/open-questions/ps-1-states-no-progress-obligation.md`.
Status stays **`accepted`** (the question is *not* resolved — only one of its four
ordered sub-questions is answered).

### 1a. Sub-question 3 is answered: **isolated**

The atom's sub-question 3 reads (`:77-81`):

> **Before deciding phase 6's answer for PS-1 specifically, check the other 35 PS
> clauses for the same shape** — this defect and the PS-19 gap … may be systematic
> rather than two isolated incidents, and the scope of the ADR should reflect
> whichever is true.

**Answer: isolated.** The sweep covered all 37 clauses against a threshold
declared before the count. Result: 29 `sound`, 7 `defective`, 1 `undetermined`;
of the seven, **three** are *independent* (an exposing implementation that
satisfies every other `PS` `MUST`) and only **two** of those three — PS-1 and
PS-19 themselves — involve a rule §4.11's table introduced. The hypothesis that
§4.11's table was populated from a systematic assumption is **not supported**: the
prior cannot be confirmed by the observation that raised it, and no third case
was found inside the table.

Two qualifications that belong in the amendment because they change the repair's
shape:

- **The shape recurs outside the table.** PS-29 carries exactly PS-1's shape — a
  `[FROZEN]` sentence narrower than the rule written beside it — on
  `one_poisoned_projection_does_not_stall_the_others`, which exists only in
  PS-29's clause body and is not one of §4.11's seventeen. So the cause is a habit
  of writing the rule to the clause's *intent* rather than to its *sentence*,
  distributed across the family, not a defect of one table's population.
- **Three further rules rest on PS-1's missing obligation.**
  `rollback_leaves_both_unchanged` (PS-8),
  `commit_accepts_a_position_the_batch_did_not_write` (PS-21) and
  `commit_rejects_a_regressing_position` (PS-22) each enforce something no
  clause's `MUST` states. PS-1's repair should therefore be scoped as *which
  clause states that a successful `commit` advances the checkpoint, and which
  rules rest on it* rather than as *add a sentence to PS-1*.

### 1b. One sentence in the body is refuted — the correction

The atom's *What is true today* section says (`:48-51`):

> That a successful commit *advances* anything — as opposed to merely being atomic
> about whatever it does — is stated by no clause's `MUST` anywhere in the
> document. PS-22 presupposes progress happens; §4.1a's prose asserts it, but
> non-normatively.

Read literally against today's clause text, **PS-23 states it**:
*"One `commit` advances exactly one `ProjectionId`"*
(`spec/SPECIFICATION.md:5317-5318`). "Exactly one" excludes zero, so a `commit`
that advances nothing violates PS-23's `MUST`. The obligation is not absent from
the document — it is **misfiled**, and misfiled twice over:

1. **On the wrong clause.** PS-23 is about fan-out *scope*: its `Rejects` field
   names *"an adapter with a single-row checkpoint table"* (`:5327-5329`), which
   is the "not more than one" reading. Progress arrives through the word "exactly",
   almost certainly unintended.
2. **On a `[PROVISIONAL]` clause**, whose falsifier is *"a pair of read models in
   one store that must be mutually consistent at every observable instant"*. The
   only normative statement that a commit makes progress is scheduled to be
   rewritten by an unrelated question, and three other rules rest on it.

**This does not overturn the atom's finding.** PS-1's pairing defect reproduces
independently from `spec/SPECIFICATION.md:4733-4759`: the per-handle backing store
described in the sweep satisfies PS-1's `MUST` through the *"or not at all"* arm,
passes `commit_is_atomic_with_the_read_model`, and fails
`commit_advances_the_checkpoint`. What changes is the **repair**: *add a sentence
to PS-1* is now one of at least three candidates, alongside minting a clause and
splitting PS-23's two readings so progress lands where it is already half-written.

**Amendment form.** Per `.kb/open-questions/README.md`, the body describes what was
known on the day the question was filed and is not rewritten into its own answer.
The correction is therefore an **appended, dated amendment note** naming the
refuted sentence and citing `spec/SPECIFICATION.md:5317-5329` and the sweep — not
an edit to the original paragraph.

### 1c. Sub-questions 1, 2 and 4 stay open

Unchanged and unanswered: whether the obligation is PS-1's to carry or a new
clause's (1); whether resolving it re-attributes `commit_advances_the_checkpoint`
in §4.11's table (2); and whether the answer interacts with ADR-0017/0018/0019 (4).
Owner unchanged: phase 6, and the repair itself is
`unstable-projection-gate-and-clause-disposition`'s under
`kb-playbook-repair-frozen-clause-001`'s discipline.

### 1d. `related` gains one edge

Add `kb-reference-ps-clause-pairing-sweep-001` **if and only if** the wave decides
this sweep warrants a `reference` atom of its own. It is not proposed here: the
document is registered evidence under `references/evaluation/` and citable by
path, exactly as `references/evaluation/phase-4-5-reconciliation.md` was before
`kb-reference-phase-4-5-spec-reconciliation-001` was minted for it. If the wave
does mint one, both open questions should gain the edge and the pointer atom
should carry the verdict, the threshold and the tally — not the census.

---

## 2. AMEND `kb-open-question-ps-19-scope-narrower-001`

File: `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`.
Status stays **`accepted`**.

### 2a. Sub-question 2 is answered: the systematic hypothesis is not supported

The atom's sub-question 2 (`:69-75`) is the same instruction plus the hypothesis:

> that pairing at a similarity score of 40, the closest of the six gaps in this
> batch, suggests §4.11's table may have been populated with a systematic
> assumption that does not match the clause text it sits beside, rather than two
> isolated defects. Whoever takes either gap should scan the remainder before
> scoping the ADR, since a single pass fixing the table's assumption may be
> cheaper than two point fixes.

**Answer: the scan was done and the hypothesis is not supported.** A single pass
over §4.11's table is **not** cheaper than point fixes, because there is nothing
table-wide to fix: two defects inside the table, both already known. Three point
repairs — PS-1, PS-19, PS-29 — plus one recorded lesson is the right shape, and
that is the shape phase 6's budget already assumes.

### 2b. The atom's own finding reproduces, with a sharper exposing implementation

Re-derived from `spec/SPECIFICATION.md:5218-5223` before the atom was re-read.
The sweep's version names the store concretely: `checkpoint` is
`SELECT position, authority FROM checkpoints WHERE id = ?` and the Rust resolves
a missing row with `.unwrap_or(Checkpoint::Live { through: FIRST })` — the
cheapest default, `SequencePosition` being `NonZeroU64` with `FIRST` as its
minimum — while `reset` writes an explicit `NeverRun` sentinel row. It satisfies
every other `PS` `MUST`, including PS-22, since nothing can regress below `FIRST`.
The atom's own bar — *"the natural implementation, not a contrivance"* (`:41-48`)
— is met and the atom is **confirmed**, not corrected.

### 2c. Sub-questions 1 and 3 stay open

Whether the obligation belongs on a widened PS-19 or a new clause (1), and where a
new clause would sit (3). Owner unchanged.

---

## 3. AMEND `kb-map-open-questions-index-001`

File: `.kb/maps/open-questions-index.md`, *Specification governance & conformance*.
Both bullets stay **Open** and stay listed; each gains one annotating sentence, in
the style the ES-6 bullet already uses (*"Amended 2026-08-10: …"*).

- The `ps-1-states-no-progress-obligation.md` bullet (`:48-51`) gains:
  *Amended 2026-08-13: the 37-clause sweep answers sub-question 3 —* isolated *—
  and refutes the body's claim that no clause's `MUST` states progress; PS-23's
  "exactly one" does, on a `[PROVISIONAL]` clause about fan-out scope.*
- The `ps-19-scope-narrower-than-its-rule.md` bullet (`:52-55`) gains:
  *Amended 2026-08-13: the sweep answers sub-question 2 — the systematic
  §4.11-table hypothesis is not supported; the finding itself reproduces.*

Both annotations should cite
`references/evaluation/ps-clause-pairing-sweep.md` by path, since the index states
what a question is and not its evidence.

---

## 4. What this document explicitly does **not** propose

Named so the omissions read as decisions rather than oversights:

- **No `decision` atom, and no ADR.** Not ADR-0017, ADR-0018 or ADR-0019 — those
  are `projection-decision-atoms`' — and not a "small" one recording what the
  sweep found. An ADR written as a side effect of a discovery pass is the
  un-auditable artefact `kb-playbook-repair-frozen-clause-001:108-119` names.
- **No change to any `[FROZEN]` clause, maturity marker or rule citation** in
  `spec/SPECIFICATION.md`. That is project AC-014, at
  `unstable-projection-gate-and-clause-disposition`.
- **No resolution of either open question.** One sub-question each is answered;
  both questions stay `accepted` and open.
- **No answer to the adjacent traps.** `kb-open-question-projection-id-unvalidated-001`
  and `kb-open-question-global-vs-boundary-visibility-001` were not reached by any
  defective row — no row's repair touches `ProjectionId`'s validation or a
  boundary-scoped checkpoint — so neither is amended, and the sweep records that it
  stopped short of both.
- **No `reference` atom.** See §1d: proposed conditionally, not asserted.
- **No new open question**, though two candidates were recorded in the sweep and
  are named here so the wave can see them and still decline: (a) three clauses —
  PS-3, PS-31, PS-36 — carry a *documentation* obligation with no instrument, which
  is one shape and may deserve one question; (b) `rebuild_is_chunk_size_invariant`
  is ungated while `batch_reads_reflect_pending_writes` carries
  `READS_THROUGH_BATCH`, so a write-behind adapter blessed by PS-4 and PS-12's
  second arm cannot pass it. Both are routed inside this project — (a) to the
  rustdoc obligations, (b) to ADR-0017 and the suite stories — and neither needs an
  atom to be actionable.
