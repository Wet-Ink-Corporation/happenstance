---
item: HS-S0001
stage: discover
created: 2026-08-12T13:01:09.346Z
updated: 2026-08-12T13:01:09.346Z
template_sig: 86ce4036
rendered_sig: ce121609
---

# Discover — Sweep PS-1 – PS-37 for the pairing defect before any repair is scoped

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: sweep PS-1 – PS-37 for the coupling-versus-progress pairing defect PS-1 and PS-19 both carry, and record the finding — systematic or isolated — as the input that scopes the frozen-clause repair rather than a side effect of writing rules | `../_storymap.md:53` | The deliverable is a **recorded finding**, not a repair and not a rule. Scope discipline: this story writes no ADR and edits no clause |
| **AC-008** — ADR-0017/0018/0019 accepted before the port change lands, each naming the alternatives that lost, open questions resolved not deleted, `redkiln validate --kb` green | `../project.md:204-208` | The sweep is upstream of AC-008 rather than a second claim on it: it decides how wide the three ADRs have to be before anyone writes them |
| `dependsOn: []` — this is the project's first story and one of the initiative's two rank-0 roots | `../_storymap.md:107`; `../project.md:262-267` | Nothing supplies input to this story. It reads `spec/SPECIFICATION.md` §4 and two open-question atoms, all of which are already in the tree |
| Architecture brief Note 8, "`[FROZEN]` clauses this work will want to widen — and must not": the sweep **is a deliverable**, the repair is a new decision atom, and "if the sweep says systematic, that is ADR scope this project should *report* rather than silently absorb" | `../_decomposition.md:639-661` | Three obligations: a recorded finding; no line edit; a report path when the answer is "systematic" |
| PS-1 is `[FROZEN]` and its own clause text admits the defect: "**This clause's MUST is a coupling, not a progress obligation**" — a `commit` returning `Ok` that makes neither write durable satisfies the "or not at all" arm, passes `commit_is_atomic_with_the_read_model` and fails `commit_advances_the_checkpoint` | `spec/SPECIFICATION.md:4733-4759` | The defect shape is precise and mechanical: **a rule the §4.11 table assigns a clause, which the clause's MUST does not entail.** That is the predicate the sweep applies to all thirty-seven |
| PS-19 has the identical shape: the clause is scoped *after a successful reset*, while `fresh_projection_has_no_checkpoint` asks about an id never seen | `spec/SPECIFICATION.md:5218-5249`; `../_decomposition.md:644-647` | Two instances, not one, is what makes "systematic" a live hypothesis rather than a formality |
| Both open-question atoms ask the same thing before any fix is scoped — check the other 35 PS clauses for the same shape, because a systematic pairing defect and two isolated ones want different repairs | `.kb/open-questions/ps-1-states-no-progress-obligation.md`; `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`; `../_grounding.md:160-167,250-256` | The sweep's question is already written down by the two atoms that name phase 6 as owner. This story executes it, it does not invent it |
| The §4.11 rule table is the other half of the diff: seventeen rules, each with the clause it is assigned and the implementation it rejects | `spec/SPECIFICATION.md:5658-5676` | The sweep is a two-column comparison — clause MUST text against assigned rule — and the table is one of the two columns. It exists today; the rules do not |
| Testing brief tiers AC-008 as **Static**, and names `redkiln validate --kb` run *before* the port-shape commit plus `git diff --diff-filter=D` over `.kb/open-questions/` | `../_decomposition.md:778` | The sweep itself is proven by nothing executable. Its check is that a later reader can re-run its method and get the same answer — so the method must be written down, not just the verdict |
| Reshape trigger: "the sweep returns *systematic*. Then the frozen-clause repair is larger than one ADR, slice 1 grows, and this project's scope is wrong — a re-plan, not a wider ADR written quietly" | `../_storymap.md:126`; `../_decomposition.md:724-727` | The "systematic" branch has a defined escalation path. The spec stage must state it as an exit, not as a contingency to absorb |
| `CLAUDE.md`: "Changing a `[FROZEN]` clause requires a new ADR, not an edit" | `CLAUDE.md`, *Open questions, deliberately unresolved* | Binding on the repair, and therefore on how the finding is written: it must be readable as ADR input, not as a patch |

## Questions

Open questions to resolve before specifying.

1. **What exactly counts as an instance?** Answered here. An instance is a
   (clause, rule) pair where the rule asserts something the clause's `MUST`
   sentence does not entail — either because the rule is *wider* than the MUST
   (PS-1: the rule asserts progress, the MUST asserts only coupling) or because
   the rule is scoped to a *different situation* than the MUST (PS-19: the MUST
   is about the state after a reset, the rule is about an id never seen). Both
   shapes are recorded, and they are recorded separately, because they want
   different repair sentences.
2. **What is the threshold between "isolated" and "systematic"?** Deferred to
   `spec`, deliberately. Setting a number here would be a guess made before
   looking; setting it in the spec, before the sweep runs, is the same
   discipline the mutant registry uses. The spec must fix the threshold and the
   escalation route *in advance of the count*, so the answer cannot be chosen to
   fit the budget.
3. **Does the sweep cover PS-31 – PS-37, which are about derivation and
   documentation rather than store behaviour?** Deferred to `spec`. AC-008 and
   the two atoms say "PS-1 – PS-37" without qualification; if a subrange is
   excluded, the exclusion is a stated finding with a reason, not a silent
   narrowing of the range.
4. **Does the sweep have licence to fix anything?** Answered: no. Every repair,
   including a one-word one, is a new decision atom belonging to
   `projection-decision-atoms` (for the clause text) or to
   `unstable-projection-gate-and-clause-disposition` (for maturity markers).
5. **None beyond these** — the project brief already carries the risk register
   (`../project.md:292-300`) and the sweep does not re-open it.

## Decision

`ProjectionStore`'s specification contains at least two clauses whose `[FROZEN]`
`MUST` text is narrower than the conformance rule the §4.11 table assigns them,
and nobody has yet asked whether that is a pair of accidents or a habit. Until
somebody does, ADR-0017/0018/0019 cannot be scoped: a repair written for two
isolated clauses and a repair written for a systematic pairing defect are
different documents, and the second one is a re-plan rather than a wider ADR.
This slice therefore produces one artefact — a per-clause finding over PS-1 –
PS-37, recording for each clause the rule(s) assigned to it, whether the rule's
assertion is entailed by the clause's `MUST`, and if not, which of the two defect
shapes it is. The spec will fix the method (the two-column diff, applied to every
clause including the ones nobody suspects), the classification vocabulary
(*wider-than-MUST* versus *differently-scoped*), the isolated/systematic
threshold **before** the count is known, and the escalation route if the
threshold is crossed. It will also state what the sweep is forbidden to do:
change a clause, add a rule, or reword a maturity marker. No ADR is required by
this story — this story is what tells the ADR pass how much to cover.

## The wrong implementation

**A sweep that reports "isolated — PS-1 and PS-19 only" because it looked at
PS-1 and PS-19.** It satisfies every check this repository can bring to bear:
nothing is compiled, `cargo xtask ci` is untouched, `cargo xtask spec-trace` is
green because no citation moved, `redkiln validate --kb` is green because no atom
changed, and the two open-question atoms can be marked resolved with a
straight face. It is also exactly the outcome both atoms were written to prevent,
and it silently ratifies the ADR budget the RUNBOOK already assumed.

The falsifier is the method, not the verdict: the finding must carry a **row per
clause, thirty-seven rows**, each naming the clause's `MUST` sentence, the
rule(s) §4.11 assigns it, and the entailment verdict. A finding with two rows is
convicted by its own shape. A reader must be able to pick any clause at random —
PS-13 and PS-14 both sit under `rebuild_is_chunk_size_invariant`
(`spec/SPECIFICATION.md:5675`), PS-21 and PS-22 both sit under a `commit` call
(`:5667-5668`) — re-run the diff, and land on the same verdict the sweep
recorded.

The concrete admitting implementation the sweep must be able to *name* for every
clause it clears, and which is already named for PS-1, is a store whose `commit`
returns `Ok(())` and makes neither the read-model row nor the checkpoint durable
(`spec/SPECIFICATION.md:4744-4753`). If the sweep cannot describe that store for
a given clause, it has not actually checked the clause — it has read it. Whether
that store becomes a registered mutant in
`crates/happenstance-testkit/tests/mutation_coverage/` is
`commit-rollback-and-drop-rules`' business and not this story's; naming it is.

**Second wrong implementation, cheaper and likelier:** a sweep that finds a third
instance and repairs it in passing, inside the same commit, because the fix is
one clause and obviously right. Nothing catches it — a `[FROZEN]` clause's text
is not checksummed and `spec-trace` checks citations rather than immutability.
This is precisely the line edit `CLAUDE.md` forbids, and the sweep's own spec
must forbid it in writing.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

Two boxes needed judgement rather than a reflex tick. **Literal positions**: this
story adds no conformance rule at all — it reads clause text and the §4.11 table
— so the box is vacuously true, and the sweep's own spec forbids adding one.
**`[FROZEN]` clauses**: the sweep *reads* PS-1, PS-19 and every other frozen `PS`
clause and changes none of them; its output is the input that scopes the ADR,
which `projection-decision-atoms` writes before any clause moves and before the
port change lands (`../_storymap.md:107-108`). The eighth box is true at the rule
grain: §4.11's seventeen rules do not exist yet, so no rule this story could find
wrong is in the tree — what it finds wrong is a *clause-to-rule assignment*, and
that is recorded as the finding rather than patched.
