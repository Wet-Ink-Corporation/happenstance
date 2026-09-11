---
id: kb-playbook-repair-frozen-clause-001
title: Repairing a frozen clause without amending it
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  How a pass that is not authorised to decide anything corrects a normative document whose clauses
  are frozen. The test is mechanical: a correction to a FROZEN clause is a repair if the set of
  implementations the clause admits is unchanged, and otherwise it is a gap, which is an ADR's.
  Gives the taxonomy of each, the safe form for an obligation that has been met — keep the MUST
  verbatim, name the discharge as a discharge, cite the code and the test that assert it — and the
  form for a gap, which is to record the defect inside the clause it is about and change nothing
  normative. A 2026-09 case sharpens the test at its hardest edge, where every instinct says
  repair and the answer is amendment: ADR-0061 corrected a sufficiency condition inside ES-11
  without touching its MUST, its maturity marker, its Rule or its Cases, and without making
  anything an adapter must do any harder — and it was still an amendment, because the correction
  removed a route by which conformance could be claimed, so the set of implementations the clause
  admits moved. Nothing gets harder is not the test. Closes with why discovery and decision were
  kept in separate passes.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-open-question-disjoint-boundaries-no-clause-001
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-provisional-falsifiers-001
  - kb-decision-0010
  - kb-decision-0012
  - kb-decision-0015
  - kb-decision-0017
  - kb-decision-0018
  - kb-decision-0019
  - kb-governance-referent-not-reasoning-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-decision-0030
  - kb-open-question-ps-32-adr-0007-correction-owed-001
  - kb-open-question-cf-36-unperformed-cross-reference-001
  - kb-open-question-no-ps-rule-name-resolved-001
  - kb-decision-0051
  - kb-decision-0056
  - kb-decision-0062
  - kb-decision-0061
  - kb-open-question-one-shot-http-es-11-001
  - kb-open-question-cf-38-case-naming-no-clause-001
  - kb-open-question-dagger-convention-vs-maturity-markers-001
  - kb-open-question-es-18-byte-identical-conformance-001
source_paths:
  - .kb/_intake/lesson-repairing-a-frozen-clause-without-amending-it.md
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - spec/SPECIFICATION.md
  - CLAUDE.md
  - RUNBOOK.md
  - xtask/src/spec_trace.rs
  - references/evaluation/phase-4-5-reconciliation.md
last_reviewed: 2026-09-09
---

# Repairing a frozen clause without amending it

## The constraint

`CLAUDE.md` states the rule this playbook operates under: "Changing a `[FROZEN]` clause requires
a new ADR, not an edit." A large majority of `spec/SPECIFICATION.md`'s clauses carry that marker
(the exact count is in `kb-reference-phase-4-5-spec-reconciliation-001`). A reconciliation pass
therefore hits a wall on its first day: several clauses said things about the code that were
false, and most of them were frozen. If a pass is not authorised to write ADRs, does it just
stop?

No — because most of what was wrong was never the MUST. A clause is a normative sentence plus
supporting prose: rationale, citations, the wrong implementation it rejects, worked examples. The
freeze protects the normative sentence. It does not license the prose around it to keep
describing an implementation that no longer exists.

## The test: repair or gap

`.kb/decisions/README.md` already states this test in the corpus's own voice: **a correction to a
`[FROZEN]` clause is a repair if the set of implementations the clause admits is unchanged.
Otherwise it is a gap, and a gap is a decision's.** The test is mechanical and can be applied
honestly by someone not authorised to decide anything: is there an implementation that was
conformant before the edit and is not after, or vice versa? If no, edit freely. If yes — or if you
cannot tell — you have found a gap, and the correct output is a recorded finding, not a smaller
edit.

**Repairs:** a citation pointing at the wrong line, or the right line for the wrong reason; prose
describing a superseded implementation; a named rule that exists under a different name or file;
a self-referential count the document gets wrong; tense describing a plan as still future when it
has shipped.

**Gaps:** the MUST admits an implementation the named rule rejects; the MUST is silent about
something a rule enforces; a maturity marker whose stated falsifier can no longer falsify
anything.

## The hardest edge: a narrowing is an amendment

The test's hard cases are not the obvious widenings. They are corrections where every visible
signal says *repair*. In 2026-09, ES-11's asynchronous-driver paragraph was found to state a
sufficiency condition that is false as written for a driver with no shared ordering primitive
between its operations: *a read spawned at its first poll and an append spawned afterwards land in
the same queue in that order*. That is a fact about pooled drivers presented as a fact about
asynchronous ones — spawning at the first poll orders the two operations at the *client*, and the
clause needs the snapshot to precede the commit at the *store*. The correction
(`kb-decision-0061`) requires them also to be ordered against a later append by something the
store itself honours.

Every signal pointed at repair. The MUST was untouched; the maturity marker was untouched; the
`Rule` and the `Cases` were untouched; and nothing an adapter must do got any harder, because the
correction only removed a route by which conformance could be *claimed*. It was still an
amendment, and it was correctly taken as an ADR: an adapter that could previously have argued
conformance from *spawned at the first poll* alone no longer can, so the set of implementations
the clause admits moved even though no obligation did.

**Keep the discrimination: "nothing gets harder" is not the test.** A narrowing changes the
admitted set as surely as a widening does, and a correction that removes a route to a conformance
claim is an amendment even when every adapter's obligations are byte-identical afterwards. That
matters here rather than as a technicality, because ES-11 and ES-12 reduce to ES-10 plus a
ceiling — a careless move in either direction reaches append-condition correctness.

## The safe form for a discharged MUST

The most tempting edit is deleting an obligation that has been met — almost always wrong. The
record of what was once required, and what discharged it, is what a future reader needs; the
obligation's bare absence reads as an oversight.

**Worked example.** A `[FROZEN]` clause required a runner to resume strictly after a checkpoint,
and its `Rejects:` field additionally forbade writing that runner against a position type that
could not signal overflow — because a saturating increment at its maximum resumes at the position
it just applied, an infinite reapply loop nobody would test. That dependent obligation is now met:
the position type's increment became a checked, matched operation. The tempting edit is to strike
the sentence. The clause instead states that the obligation is **discharged, not withdrawn**,
names the code that satisfies it and the test that asserts it, and says a runner may now be
written against it.

Three components, all required: **the MUST stays verbatim** — no implementation gains or loses
conformance, so the freeze is untouched, which is what makes this a repair; **the discharge is
named as a discharge**, so a reader cannot mistake a satisfied obligation for a relaxed one; and
**the evidence is cited** — the code and the test — so the discharge is guarded rather than a
claim about a moment in time. Deleting the cited test now breaks the citation check in the gate.
**A discharge recorded without a named test is a discharge that can silently regress.**

## The form for a gap

Where the MUST is the problem: state the defect inside the clause it is about, name the shape of
implementation that exposes it, and name what an ADR would have to decide — and change nothing
normative. Two examples from this pass: a durability clause whose MUST, read literally, is a
coupling rather than a progress obligation, so a commit that durably writes neither the read
model nor the checkpoint satisfies its "or not at all" arm while still failing the rule that
expects a successful commit to advance something; and a checkpoint-reset clause scoped only to the
case *after* a successful reset, silent about an id never seen at all, where the natural
implementation satisfies the MUST verbatim and still fails the rule. In both, the finding sits
inside the clause it is about, not in a separate list — a finding filed elsewhere is one the next
reader of the clause will not see, and that reader is exactly who needs it.

## Why discovery and decision stayed in separate passes

Three reasons, in order of weight: a pass that both discovers and decides cannot be audited,
because its findings and its resolutions arrive in one artifact with no baseline to judge whether
the decision was forced by the evidence or convenient for closing the pass; a decision taken to
unblock a checker is a decision taken for the wrong reason — attaching an unclaimed rule to a
nearby clause "to close it" would assert a `[FROZEN]` clause contains a proposition it does not,
a documentation defect invisible to every check; and ADRs in this repository carry a described
process and a human sign-off that a pass shortcutting past them does not save work so much as move
it somewhere unrecorded. The cost is real: several findings from this pass are open and
unscheduled, held as open-question atoms and, where a mechanism exists, inside
`kb-playbook-ratchet-gate-landing-001`'s exemption list.
