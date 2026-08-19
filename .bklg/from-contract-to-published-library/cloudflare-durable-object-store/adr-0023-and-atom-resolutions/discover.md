---
item: HS-S0058
stage: discover
created: 2026-08-12T13:02:20.139Z
updated: 2026-08-12T13:02:20.139Z
template_sig: 86ce4036
rendered_sig: 1d2ab6ca
---

# Discover — ADR-0023 accepted, and CF-40 and WF-11 resolved rather than deleted

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: stage `.kb/_intake/` and run the ingest path so ADR-0023 lands as an accepted atom stating the `SqlStorage` mapping and the off-tokio harness as one question with the alternatives that lost, and so CF-40's ownership and WF-11's atom move to *resolved rather than deleted* | `_storymap.md`, **Slices**, `adr-0023-and-atom-resolutions` row | The story is a staging-and-handoff, not an authoring-in-place |
| This story owns **every** KB write in the project, and nothing else does | `_storymap.md`, **Why these milestones and not others**, last bullet | Single writer. Any `.kb/` change in another story's diff is a defect in that story |
| AC-005 (the ES-6 outcome recorded as a decision atom, not as prose), AC-006 (ADR-0023 accepted, authored through the ingest path), AC-008 (CF-40's ownership resolved, not deleted), AC-011 (WF-11's atom reflects the evidence) | `project.md`, **Acceptance criteria** | Four ACs, one mechanism |
| `dependsOn` five stories — the read path (the ceiling-and-page resolution, or the compiled reason it did not fit), the error verdict (the ES-6 artefact and what it showed), the conformance run (the harness shape actually built and the concurrency family's stated non-invocation), the measured limits (three numbers and how each was measured), and the WF-11 falsifier (whether it bit) | `_storymap.md`, **Slices**, `depends_on` column | It is last because it records what the other four found. Every edge is a piece of the record's content |
| Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, **not by hand** — hand-writing produces the directory layout of the process without the process, which is why the first attempt was reverted (`0269720`) | `CLAUDE.md`, **Where the work lives** | The failure has already happened once in this repository and is on the record |
| An accepted decision atom is **immutable**: `redkiln validate --kb` checks each one against `HEAD`, so correcting one means writing a new atom that supersedes it, never editing the body | `CLAUDE.md`, **Where the work lives** | The enforcement exists for `.kb/`. It does not exist for `references/adr/` |
| A decision lives in two places on purpose: the atom in `.kb/decisions/` carrying the frontmatter and supersession graph, and the long-form record in `references/adr/` carrying the transcripts and rejected alternatives. Link the atom; cite the record by `file:line`, because `spec/SPECIFICATION.md` cites line ranges that exist only in the long form | `CLAUDE.md`, **Where the work lives** | An edit to the long form moves lines a clause points at, and `spec-trace` catches it only if a citation falls out of range |
| **ADR-0001 must not be edited.** Its `provisional` marker was already lifted at phase 1 by `LocalMemoryEventStore`, ADR-0008 records the lift, and `RUNBOOK.md:4280-4282`'s "formally retired" is satisfied by *citing this adapter* from ADR-0023 and the long-form record | `_grounding.md` §1; `_decomposition.md`, Architecture brief Notes §2; `.kb/decisions/0008-one-derivation-for-both-ports.md` | A standing hazard the plan names twice, because the runbook's wording invites exactly the wrong action |
| **ADR-0009 must not be edited.** Confirming or refuting its ES-6 prediction is a new record | `project.md`, DR-7; `.kb/decisions/0009-error-send-sync.md` | ES-6 was settled, not deferred. The artefact judges the prediction; it does not amend the atom |
| **CF-40 must not be minted twice.** The atom's contradiction is between two accepted, unedited ADRs; it names phase 8 as what forces it, and `sqlite-durable-store` (HS-P0012) merges one position ahead of this project | `.kb/open-questions/cf-40-fixture-limits-ownership.md`, *What is not decided* and *What forces it*; `_decomposition.md`, Architecture brief §6 | Whichever project reaches the answer first owns it and the other cites it. Coordinate before authoring |
| CF-40's deeper sub-question is not about CF-40: it is whether the fixture contract has one owning document or is amended piecemeal by whichever ADR needs the next capability | same atom, *Ordered sub-questions* 2 | The evidence `measured-store-limits` produces narrows it to this, which is the more useful answer |
| WF-11's atom asks three ordered sub-questions, the first of which is whether phase 9 actually needs to forward a large payload through JSON at all | `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`, *Ordered sub-questions* | "Resolved" here may legitimately mean "the condition is not constructible on this runtime, and here is what would construct it" |
| ADR-0023 is **one** question — the `SqlStorage` mapping and the off-tokio harness together — with the alternatives that lost | `project.md`, AC-006; `RUNBOOK.md:302`, `:4267-4268` | Splitting it into two atoms would satisfy validation and miss the point of the record |
| Every answer this project settles lands as a decision atom naming the alternatives that lost, written through the runbook's ADR queue rather than as a side effect, and the matching open-question atoms are resolved rather than deleted | `project.md`, DR-7 | The queue, not the commit, is the authoring path |

## Questions

**CF-40's ownership — the live one.** Answered in *procedure*, deferred in
*substance*. Procedure: this story is the only place in HS-P0013 permitted to write
it, and its first action is to check whether `sqlite-durable-store` (HS-P0012,
merging one position ahead) has already resolved it. If it has, this story cites;
if it has not, this story resolves and HS-P0012 cites. Substance — whether CF-40 is
ADR-0015's or moves beside CF-39 in ADR-0012, and the second, larger sub-question of
whether the fixture contract has one owning document — is settled at **spec**, on
the evidence `measured-store-limits` produced. The one outcome that is forbidden is
two independent mintings, which is what the decomposition's non-goals list exists to
prevent.

**The off-tokio harness shape — the other live one.** ADR-0023 states it as one
question with the `SqlStorage` mapping, per AC-006, and must name the alternatives
that lost with the reason each lost: `vitest-pool-workers` as its own CI job
(`RUNBOOK.md:4267-4268`), which loses to AC-004's "same run as the rest of the gate"
if an in-gate step can exist; `wasm-bindgen-test-runner` under node with no Durable
Object, which loses if the `SqlStorage` binding cannot be satisfied without
`workerd`; and a probe-gated step with no compensating mandatory assertion, which
loses to AC-004's own wording. Which one *won* is `every-rule-under-workerd`'s
finding, and this story records it rather than choosing it.

**Does ADR-0023 lift ADR-0001's marker?** Answered: **no**, and the record must say
why, because the runbook's wording invites the opposite. The marker was lifted at
phase 1, ADR-0008 records the lift, and the atom is accepted and immutable. What
this project supplies is the *real-runtime evidence* behind an already-accepted
decision, cited from ADR-0023 and from the long-form record.

**Does WF-11's atom close, or move to resolved-with-a-condition?** Deferred to
**spec**; it depends entirely on `wf-11-memory-ceiling-falsifier`'s finding, and
"resolved" is satisfied by a stated, evidenced condition as well as by a closure.
Either way the atom is **resolved, not deleted**.

**Ingest hygiene.** Two mechanics settled now so they are not improvised: the
wave's id is suffixed so it does not overwrite an earlier wave's audit trail, and
the intake staging directory's own README is not an atom and is dropped at the
approval gate rather than ingested.

**Who runs the ingest?** Answered: a human. `/redkiln:kb-ingest` is a user-invoked
command, so this story plans a handoff — staged intake files, a proposed action
plan, and the coordination check with HS-P0012 — rather than performing the write
itself.

## Decision

Four questions this project answers are worth nothing to anyone outside it unless
they are written down in the one place the repository treats as settled, and the
repository is unusually strict about *how* they get there: atoms are minted by the
ingest path from staged intake files, accepted atoms are immutable and checked
against `HEAD` forever, and open questions are resolved rather than deleted. This
slice is the single writer. It stages the `SqlStorage` mapping and the off-tokio
harness as one ADR-0023 with the alternatives that lost, records the ES-6 outcome
as a new atom rather than as an edit to ADR-0009, and moves CF-40's ownership and
WF-11's falsifier from open to resolved — CF-40 only after checking that
`sqlite-durable-store` has not already minted it. It is sequenced last because its
content is the other four milestones' findings. The spec will cover: the intake
files to stage and what each carries; the exact ADR-0023 outline including the
alternatives that lost and the ADR-0001 citation that is a citation and not a
marker lift; the CF-40 coordination check and both branches of its outcome; the
WF-11 resolution wording for either finding; the ingest wave id and the approval
gate; and the post-ingest verification — `redkiln validate --kb`, `redkiln doctor`
with exactly six `template-drift` advisories and no seventh, and `cargo xtask
spec-trace`. Nothing `[FROZEN]` is amended anywhere in this project, and this is the
story that would have had to author the atom first if anything had needed to be.

## The wrong implementation

**The atom written by hand into `.kb/decisions/`.** It has the right filename, a
valid `KbFrontmatter`, a supersession graph that resolves, outbound-only links, and
reciprocal backlinks added by hand for good measure. `redkiln validate --kb` is
green, because validation checks *conformance and immutability*, not *provenance*.
What is missing is everything the ingest path exists to produce: the cross-file
adjudication that prefers a merge or an amendment over a new atom, the ADR-conflict
adjudication, the map and index sync, and the audit trail of the wave itself. This
repository has already paid for this exact mistake and reverted it — `CLAUDE.md`
names the commit (`0269720`) and the diagnosis: "hand-writing them produces the
directory layout of the process without the process."

**And the one that is worse, because it validates *and* looks conscientious: the
correction applied as an edit.** This project's artefact confirms ADR-0009's ES-6
prediction, so a clarifying line is added to
`.kb/decisions/0009-error-send-sync.md`; or `RUNBOOK.md:4280-4282`'s "ADR-0001's
`provisional` marker formally retired" is read literally and ADR-0001's frontmatter
is touched. Both are caught — `redkiln validate --kb` checks accepted atoms against
`HEAD` — and that is the *good* case. The bad case is the same instinct applied one
directory over, to `references/adr/`, which validation does **not** check and where
`spec/SPECIFICATION.md` cites line ranges that exist only in the long form
(`CLAUDE.md`, **Where the work lives**). An edit there silently moves the lines a
frozen clause points at, and `cargo xtask spec-trace` notices only if a citation
happens to fall out of range — so the failure mode is a clause that still resolves
and now points at the wrong paragraph.

**Where the detectors live, and the one gap none of them covers.** Nothing here is
a conformance rule, so nothing belongs in
`crates/happenstance-testkit/tests/mutation_coverage.rs` — its rows are stores that
fail named rules, and an atom is not a store. The detectors already exist and are
wired: `redkiln validate --kb` for frontmatter conformance and accepted-atom
immutability; `redkiln doctor` for the backlog and for the six expected
`template-drift` advisories, where a seventh means a template was changed without
deciding to; and `cargo xtask spec-trace` for the clause citations, including the
long-form line ranges. The gap none of them covers is **provenance** — no check can
tell an ingested atom from a hand-written one — and the only guard for it is the
workflow itself: stage into `.kb/_intake/`, mint through `/redkiln:kb-ingest`, and
treat the CF-40 coordination with HS-P0012 as a precondition of the wave rather than
as a step inside it.

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

**Box 6.** No conformance rule is added here, and no Rust at all. The story's
output is staged intake files and an ingest handoff; nothing in it asserts a
position.

**Box 7.** This is the story where a frozen-clause change would have had to land,
and **none is needed**: CF-23, ES-6 and ES-9 are each honoured by the work rather
than amended by it — the third harness is built, ES-6 is confirmed against a real
`worker::Error`, and ES-9's neighbourhood is satisfied through ADR-0011's accepted
ceiling-and-page mechanism. Had `durable-object-read-path` failed to honour ES-9,
the required order is the one this box states: a new decision atom, written first,
and a re-plan — which is precisely why that story's Decision names the escalation
instead of assuming the adapter is excused. CF-39, CF-40 and WF-11 are
`[PROVISIONAL]` and are discharged or resolved rather than amended.

**Box 8.** No conformance rule here seems wrong. Two pieces of *plan* text do read
as wrong against the knowledge base and are corrected in this story's record rather
than obeyed, each with its reason: `RUNBOOK.md:4280-4282`'s instruction to formally
retire ADR-0001's marker, which was lifted at phase 1 and whose atom is immutable;
and AC-005's "bound added, or ADR-0009's deferral confirmed", which predates
ADR-0009's acceptance. Both corrections are stated in ADR-0023 in the same change
that records the evidence.
