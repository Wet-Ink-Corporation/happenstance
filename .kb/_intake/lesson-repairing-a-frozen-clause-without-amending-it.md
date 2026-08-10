# Repairing a frozen clause without amending it

## Where this expects to land

**Layer: none of the three scaffolded ones.** This is a governance practice for a normative
document — not a persona, not an interaction pattern, and not itself unsettled.
**KB root**, or a `playbooks/` layer if one is created. **Kind: `playbook`**, with
`authority_tier` chosen from the documented vocabulary rather than invented; it describes how
to work under a constraint rather than imposing a new one.

`source_paths`: `docs/architecture/SPECIFICATION.md`, `CLAUDE.md`, `docs/RUNBOOK.md`,
`xtask/src/spec_trace.rs`, `docs/evaluation/phase-4-5-reconciliation.md`, and this intake file.

## The constraint

`docs/architecture/SPECIFICATION.md` carries 200 numbered clauses, each with a maturity
marker. `CLAUDE.md` states the rule: **"Changing a `[FROZEN]` clause requires a new ADR, not an
edit."** 139 of the 200 clauses are `[FROZEN]` today.

A reconciliation pass therefore hits a wall on its first day. Sixteen clauses said things
about the code that were false, and most of them were frozen. If editing a frozen clause needs
an ADR and the pass is not authorised to write ADRs, does the pass just stop?

No — because **most of what was wrong was not the MUST.** A clause is a normative sentence
plus supporting prose: rationale, citations, the wrong implementation it rejects, worked
examples. The freeze protects the normative sentence. It does not license the prose around it
to describe an implementation that no longer exists.

## The test: repair or gap

> **A correction to a `[FROZEN]` clause is a *repair* if the set of implementations the clause
> admits is unchanged. Otherwise it is a *gap*, and a gap is an ADR's.**

The test is mechanical and can be applied honestly by someone who is not authorised to decide
anything. Ask: *is there an implementation that was conformant before this edit and is not
after, or vice versa?* If no, edit freely. If yes — or if you cannot tell — you have found a
gap, and the correct output is a recorded finding, not a smaller edit.

Repairs, all of which landed in this pass:

- a citation that points at the wrong line, or at the right line for the wrong reason;
- prose describing a superseded implementation — `AppendCondition` described as having public
  fields after VT-30 made `guards` private; `SequencePosition::next` described as
  `saturating_add` after it became `checked_add`;
- a named rule that turns out to exist under a different name, or in a different file;
- a self-referential count the document states about itself and gets wrong;
- tense: a clause written as a plan for something now shipped.

Gaps, none of which were closed:

- the MUST admits an implementation the named rule rejects (PS-1, PS-19 below);
- the MUST is silent about something a rule enforces (the two entries in
  `UNCLAIMED_PENDING_ADR`);
- a maturity marker whose stated falsifier can no longer falsify anything (ES-7, VT-9).

## The safe form for a discharged MUST: keep it, record the discharge

The most tempting edit in a normative document is deleting an obligation that has been met.
It is almost always wrong. **The record of what was once required, and what discharged it, is
the thing a future reader needs; the obligation's mere absence tells them nothing and reads as
an oversight.**

### PS-20, the worked example

`docs/architecture/SPECIFICATION.md:5250-5271`. The clause is `[FROZEN]`:

> **PS-20 — A runner MUST resume strictly after the checkpoint's position, and MUST start at
> the store's first position, inclusive, when the checkpoint is `NeverRun`.**

Its `Rejects:` field carried a second, dependent obligation:

> The runner MUST NOT be written against a `next()` that cannot signal overflow, because at
> `u64::MAX` the saturating version resumes at the position it just applied — an infinite
> reapply loop in the one place nobody will test.

That obligation is now met: `SequencePosition::next` is `NonZeroU64::checked_add` in `match`
form. The tempting edit is to strike the sentence, since nobody can write a runner against a
`saturating_add` that no longer exists. The clause instead says:

> **That obligation is discharged, not withdrawn:** `next()` is `NonZeroU64::checked_add` in
> `match` form (`event.rs:272-282`) and `position_next_signals_overflow`
> (`event.rs:842-873`) asserts it, so a runner may now be written against it.

Note the three components, and that all three are needed:

1. **The MUST stays, verbatim.** No implementation gains or loses conformance, so the freeze is
   not touched. This is a repair by the test above.
2. **The discharge is named as a discharge** — "discharged, not withdrawn" — so a reader
   cannot mistake a satisfied obligation for a relaxed one.
3. **The evidence is cited: the code that satisfies it and the test that asserts it.** Without
   the test citation, the discharge is a claim about a moment in time; with it, the discharge
   is guarded, because deleting `position_next_signals_overflow` now breaks the citation check
   in `cargo xtask spec-trace`.

The third component is what makes this form durable rather than merely polite. **A discharge
recorded without a named test is a discharge that can silently regress.**

### The same form applied to a gap

Where the MUST *is* the problem, the form changes: state the defect inside the clause, name
the shape of implementation that exposes it, and name the phase that owns the repair — and
change nothing normative. Two examples, both from this pass:

**PS-1** (`SPECIFICATION.md:4733-4757`) — "The read-model write and the checkpoint write MUST
become durable together or not at all." §4.11's table assigns it three rules, one of which is
`commit_advances_the_checkpoint`. The clause now records that its MUST **is a coupling, not a
progress obligation**: a `commit` returning `Ok` that makes *neither* the row nor the
checkpoint durable satisfies the "or not at all" arm, passes
`commit_is_atomic_with_the_read_model` (both-absent is one of the two states that rule
permits), and fails `commit_advances_the_checkpoint`. That a successful commit advances
anything is stated by no clause's MUST. Phase 6 owns the repair, "and it is an ADR's rather
than an edit's because this clause is `[FROZEN]`."

**PS-19** (`SPECIFICATION.md:5218-5244`) — the MUST is scoped *after a successful `reset`*,
while `fresh_projection_has_no_checkpoint` asks about an id never seen. The clause now records
the natural implementation that satisfies the MUST verbatim and fails the rule: `reset` writes
an explicit `NeverRun` sentinel row, and `checkpoint(id)` resolves a missing row with
`.unwrap_or(Checkpoint::Live { through: FIRST })`. Phase 6 owns whether the clause widens or a
new one says it.

In both cases the finding is **inside the clause it is about**, not in a separate defects list.
A finding filed elsewhere is one the next reader of the clause will not see, and the next
reader of the clause is exactly the person who needs it.

## Why ADR authorship stayed outside this pass

The pass wrote no ADR, and this was a scope decision rather than a shortage of time. Three
reasons, in order of weight:

1. **A pass that both discovers and decides cannot be audited.** Its findings and its
   resolutions arrive in one artifact, and a reviewer has no baseline against which to judge
   whether a decision was forced by the evidence or convenient for closing the pass. Keeping
   discovery separate is what makes the seven recorded findings reviewable as findings.
2. **A decision taken to unblock a checker is a decision taken for the wrong reason.** The two
   entries in `UNCLAIMED_PENDING_ADR` could each have been "closed" by attaching the rule to a
   nearby clause. Both attachments would have asserted that a `[FROZEN]` clause contains a
   proposition it does not contain — a documentation defect strictly worse than the missing
   attribution, because it would be invisible to every check.
3. **ADRs in this repository have a described process and a human sign-off.** `docs/RUNBOOK.md`
   assigns each open question to a phase and an owner; ADR-0029 is the standing example of a
   constraint moved deliberately, in an ADR, rather than in silence. A pass that shortcuts that
   is not saving work, it is moving work to a place where it is not recorded.

The cost is real and should be stated: **seven findings are open and none is scheduled by this
pass.** They are recorded in `gaps-owed-a-decision.md` and
`open-question-nothing-owns-the-post-phase-reconciliation.md`, and two of them are additionally
held in `UNCLAIMED_PENDING_ADR` where the gate prints them on every green run. Only those two
have a mechanism attached; the other five rely on the KB and the runbook being read.
