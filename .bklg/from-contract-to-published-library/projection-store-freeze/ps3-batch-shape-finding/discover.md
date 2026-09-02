---
item: HS-S0014
stage: discover
created: 2026-08-12T13:01:21.625Z
updated: 2026-08-12T13:01:21.625Z
template_sig: 86ce4036
rendered_sig: 8b1ffe62
---

# Discover — The PS-3 evidence written as a finding, not a verdict

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: the PS-3 evidence written as a finding — *did the two batch shapes disagree, and where?* — **including the "they agreed everywhere" outcome, which is itself the finding and not a silent success** — handed to `publication-and-positioning` (HS-P0016) with no verdict on the `unstable-projection` exposure made here | `../_storymap.md:66` | A written artefact, a named recipient, and an explicit instruction that the null result is a result |
| **AC-015** — the PS-3 evidence is recorded as a written finding and handed on; **no verdict on the `unstable-projection` exposure is made here** | `../project.md:229-231` | Sole owner (`../_storymap.md:93`). The prohibition is half the criterion |
| `dependsOn: buffering-conformant-variant` — supplies the second shape and the run whose behaviour this story reports | `../_storymap.md:65-66,112` | There is nothing to report until both shapes have run the whole suite in one gate invocation |
| PS-3: "Until PS-2's bar is met the port SHOULD ship behind an off-by-default `unstable-projection` feature, with a documented exemption from semver" — `[PROVISIONAL — falsified the moment PS-2's bar is met before 0.1]` | `spec/SPECIFICATION.md:4776-4790` | PS-3 waits on a *schedule*, not on an answer: "it is resolved by PS-2 and by nothing of its own" (`:4798-4803`) |
| PS-2 is `[FROZEN]` and its bar is two **adapters** at opposite ends of the batch-shape axis, with its **Rejects** clause naming a two-instrument monoculture verbatim | `spec/SPECIFICATION.md:4760-4775` | Nothing this project built clears PS-2. The finding must say so rather than implying otherwise by silence |
| Architecture brief **AC-A04**: the design states in writing that PS-2's bar is **not** met by anything this project can build alone, takes AC-014's second arm, and "the PS-3 evidence hands `publication-and-positioning` a finding, not a verdict" | `../_decomposition.md:307-311,449-456` | The split between evidence and verdict is a recorded architectural decision, not a stylistic preference |
| The scope seam: the PS-3 verdict on whether the port ships behind `unstable-projection` at publish belongs to `publication-and-positioning` (HS-P0016) — "this project supplies the evidence; that project makes the call" | `../project.md:129-131` | The recipient is named in the charter's own out-of-scope list |
| The RUNBOOK frames the question the finding answers: ship frozen, or behind `unstable-projection` per the `tokio_unstable` idiom, "if the two batch shapes disagree" | `RUNBOOK.md:3924-3928`; `../_grounding.md:236-242` | "Did they disagree" is the literal question. The finding is its answer, with evidence |
| Architecture brief Note 10 item 2: "if the two shapes never disagree anywhere, either the rules are shape-blind in a way that hides the axis, or the axis is not where §4.2 says it is. PS-3's evidence is exactly this question and AC-015 is where the answer goes" | `../_decomposition.md:716-719` | The null result has two candidate explanations, and the finding must choose between them or say it cannot |
| Testing brief AC-015: **E2E (process, derived from AC-004)** — "not a new test: the finding is written from what AC-004's Integration run actually showed… If both fixtures pass identically, that itself is the finding and is recorded rather than silently treated as success" | `../_decomposition.md:785` | No test tier proves this story. Its only instrument is the review, and the wrong implementation below |
| The initiative risk this feeds: the freeze is "provisional in fact" until HS-P0015 writes its verdict, "which is exactly why the verdict is a separate project" | `../project.md:301-307` | Two separate downstream consumers — HS-P0015 for the freeze verdict, HS-P0016 for the exposure call — and this finding informs both |
| BR-15 / DR-06: every answer this project settles lands as a decision atom naming the alternatives that lost | `../project.md:161-163` | A *finding* is not a decision atom, and the distinction is the point: this story deliberately settles nothing |

## Questions

Open questions to resolve before specifying.

1. **What form does the finding take — a KB atom, a project artefact, or a
   section of the closeout?** Deferred to `spec`. The constraint recorded here:
   it must be citable by HS-P0016 without reading this project's implementation
   history, and it must not be a decision atom, because a decision atom would be
   the verdict this story is forbidden to make.
2. **What counts as a "disagreement"?** Deferred to `spec`, and it needs
   defining before the run rather than after. Candidates: a rule that needed
   different handling per shape; a rule one shape passed only because of a
   capability declension; a rule whose assertion had to be loosened during
   `buffering-conformant-variant`. Fixing the vocabulary in advance is what stops
   the answer being chosen to fit the conclusion.
3. **If the shapes agreed everywhere, which of Note 10's two explanations is
   it?** Deferred to `spec` as a question the finding must *address*, with "we
   cannot tell from this evidence" as an acceptable answer — but not silence.
4. **Does the finding say anything about PS-2?** Answered: yes, and only one
   thing — that its bar is not met by anything this project built, per AC-A04
   (`../_decomposition.md:436-456`). It makes no recommendation about when it
   will be.
5. **None beyond what the project brief already carries.**

## Decision

Whether `ProjectionStore` ships behind an off-by-default feature at 0.1 turns on
whether the two batch shapes the suite was proved against actually disagreed —
and this project is the only one that will ever be in a position to observe that,
because it is the project that runs both. It is also explicitly not the project
that decides. This slice writes the evidence up: what the buffering
replay-at-commit shape and the apply-on-write oracle did differently under each
rule, where a rule needed different handling, where a capability was declined by
one and not the other, and — if nothing differed anywhere — that fact stated as a
finding with its two candidate explanations weighed. The spec will fix: the
artefact's location and form (citable by HS-P0016, and *not* a decision atom);
the disagreement vocabulary, fixed before the run rather than after; the
requirement that the null result be recorded explicitly; the single sentence about
PS-2's bar not being met; and the explicit non-verdict, so a downstream reader
cannot mistake evidence for a recommendation. No `[FROZEN]` clause is touched:
PS-3 is `[PROVISIONAL]` and PS-2 is frozen and merely cited.

## The wrong implementation

**A finding that reads "both batch shapes passed the whole suite; the port is
proven against the batch-shape axis."** Every sentence in it is true. It will
survive any review that checks facts rather than claims. And it is a **verdict**
wearing a finding's clothes: it tells HS-P0016 that the exposure question is
settled, when PS-2 — `[FROZEN]`, and the clause that actually gates the freeze —
requires two *adapters* at opposite ends of the axis and names a two-instrument
monoculture in its **Rejects** clause as the thing to refuse
(`spec/SPECIFICATION.md:4760-4775`). Nothing catches it: prose is not compiled,
`spec-trace` checks citations rather than inferences, and the downstream project
inherits the conclusion as though it were evidence. The guard is AC-A04's
requirement that the finding state in writing that PS-2's bar is not met by
anything this project can build alone (`../_decomposition.md:307-311`), and the
spec must make that sentence mandatory rather than advisable.

**The null result recorded as silence.** The likelier failure, and the cheaper
one: the two shapes agree on every rule, there is nothing to write, and the
finding says "no disagreements observed" — or is simply not written, because a
finding with no content feels like an absence of news. It is not. If two
structurally unlike shapes never diverge anywhere, either the rules are
shape-blind in a way that hides the axis, or the axis is not where §4.2 says it is
(`../_decomposition.md:716-719`) — and both of those are things HS-P0015 and
HS-P0016 need to know before they trust the suite. The storymap's own one-line
anticipates this in the story's title: "the 'they agreed everywhere' outcome,
which is itself the finding and not a silent success" (`../_storymap.md:66`).

**And the one that would be caught, but only by a reader who knows the rule:**
writing the finding as a `.kb/decisions/` atom. It would pass `redkiln validate
--kb` — the frontmatter is easy to make conformant — and it would make this
project the author of a decision the charter assigns to another
(`../project.md:129-131`). Worse, an accepted decision atom is immutable
(`CLAUDE.md`, *Where the work lives*), so HS-P0016 would have to supersede a
decision rather than make one. The finding is evidence and must be shaped as
evidence.

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

The two judgement boxes. **Literal positions**: this story adds no code at all —
its deliverable is a written finding — so the box is vacuously true.
**`[FROZEN]` clauses**: PS-2 is frozen and is *cited* here, in one direction only
(its bar is not met). Its text is not edited, and PS-3, the clause whose
disposition the evidence feeds, is `[PROVISIONAL]`
(`spec/SPECIFICATION.md:4778`). The eighth box is vacuous: if this story finds a
rule wrong, that is a disagreement between the shapes and therefore part of the
finding itself — but the fix belongs to whichever rule story owns it, with the
reason in the same change.
