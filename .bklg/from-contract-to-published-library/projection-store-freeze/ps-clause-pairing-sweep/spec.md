---
item: HS-S0001
stage: spec
created: 2026-08-12T13:45:56.341Z
updated: 2026-08-12T13:45:56.341Z
template_sig: 87bbf1d0
rendered_sig: e74a2bdd
---

# Spec — Sweep PS-1 – PS-37 for the pairing defect before any repair is scoped

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/ps-clause-pairing-sweep/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` — Architecture brief **AC-A08** and **Note 8** ("`[FROZEN]` clauses this work will want to widen — and must not"), plus "What would reshape this map" item 4 |
| Story map row | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` — milestone `decisions-and-design-record`, first row; merge order item 1 |
| Signed-off design | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — **`surfaces: []`**, approved 2026-08-12. No user-facing surface exists in this project, so this story renders none and re-decides none. |
| Roadmap pointer | `RUNBOOK.md:3848` — phase 6, "Freeze `ProjectionStore`"; `RUNBOOK.md:296-298` — ADR-0017 / 0018 / 0019 in the queue |

## One-line PR slice

Sweep PS-1 – PS-37 for the coupling-versus-progress pairing defect PS-1 and PS-19
both carry, and record the finding — systematic or isolated — as the input that
scopes the frozen-clause repair rather than a side effect of writing rules.

## Executive summary

Two `[FROZEN]` clauses in §4 are already known to be paired with conformance
rules their own `MUST` does not support: PS-1's sentence is a *coupling* and
`commit_advances_the_checkpoint` asks for *progress*
(`spec/SPECIFICATION.md:4744-4759`); PS-19's sentence is scoped *after a reset*
and `fresh_projection_has_no_checkpoint` asks about an id never seen
(`:5218-5249`). Both open-question atoms end with the same instruction, and
neither has been carried out: **before scoping either fix, check the other 35 PS
clauses for the same shape**
(`.kb/open-questions/ps-1-states-no-progress-obligation.md:77-81`;
`.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:70-75`).

This PR carries that instruction out and lands its result as evidence. It is the
first story in the project because the answer changes what comes next: a
*systematic* table-population defect and two *isolated* ones want different ADR
scope, and if it is systematic that is a re-plan reported at this story's
boundary rather than ten frozen clauses widened later under cover of a
rule-writing story
(`.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md`,
Architecture brief Note 8 and reshape item 4).

**Delta over what the tree already holds.** `cargo xtask spec-trace` runs on
every gate and cannot answer this question — it says so in its own header: *"It
cannot tell whether a rule is the right rule for a clause. That is judgement"*
(`xtask/src/spec_trace.rs:17-18`). The phase 4/5 reconciliation established the
method and the artefact shape but stopped at six recorded gaps without sweeping
the neighbourhood of any of them
(`.kb/reference/phase-4-5-specification-reconciliation-census.md:108-112`). What
is new here is a **census of soundness**, clause by clause, over one clause
family — and a stated verdict on whether §4.11's table was populated from a
systematic assumption that does not match the clause text beside it.

Nothing normative moves in this PR. No `[FROZEN]` clause is edited, no ADR is
written, no maturity marker changes, and no line of `crates/` is touched. The
repair is `unstable-projection-gate-and-clause-disposition`'s, the ADR scope is
`projection-decision-atoms`'s, and both depend on this row.

## Context pack

**This story produces a finding, not a fix, and the separation is load-bearing.**
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:108-119` records
why discovery and decision were deliberately kept in separate passes: *"a pass
that both discovers and decides cannot be audited, because its findings and its
resolutions arrive in one artifact with no baseline to judge whether the decision
was forced by the evidence or convenient for closing the pass."* Producing an ADR
here, or a "small" clause edit, destroys exactly the baseline the next two stories
need. If the sweep's output feels incomplete without a fix attached, that feeling
is the design working.

**The repair/gap test is mechanical and is the sweep's only classifier.** From
`.kb/decisions/README.md:20-22` and restated in the playbook
(`repairing-a-frozen-clause-without-amending-it.md:56-72`): *a correction to a
`[FROZEN]` clause is a **repair** if the set of implementations the clause admits
is unchanged; otherwise it is a **gap**, and a gap is a decision's.* Applied to a
clause/rule pairing this becomes one question with a yes/no answer: **is there an
implementation that satisfies the clause's `MUST` verbatim and fails a rule the
tables assign to that clause?** If yes, the pairing is defective and the fix is an
ADR's. If no, the pairing is sound. "I cannot tell" is a third recorded verdict,
not a coin flip — the playbook's own instruction is that inability to tell *is* a
gap finding.

**A finding is only a finding if it names the implementation that exposes it.**
Both existing atoms clear this bar and neither does it with a contrivance: PS-19's
exposing store writes an explicit `NeverRun` sentinel on reset and resolves a
missing row with `.unwrap_or(Checkpoint::Live { through: FIRST })` — *"the natural
implementation, not a contrivance"*
(`.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:41-48`). The same bar
holds every row this sweep files. This is `CLAUDE.md`'s conformance corollary
one level up: a finding no implementation can exhibit is decorative in exactly
the way a rule no adapter can fail is.

**Three tables must be read, not one, and they disagree in different ways.**
A PS clause's rule attribution is stated in three places: the `**Rule:**` field
inside the clause body itself (e.g. `spec/SPECIFICATION.md:4735-4743`), §4.11's
rule → clause table (`:5653-5676`, seventeen adapter-level rules), and §7.2's
clause → rule index (`:8628-8666`). `spec-trace` already proves §7.1/§7.2 are
derived from `parse_clauses` rather than hand-maintained
(`xtask/src/spec_trace.rs:22-29`), so an index/clause disagreement is a tooling
bug and out of scope; what is in scope is the **semantic** disagreement none of
the three can detect. Where a clause body already records its own defect — PS-1
does, at `:4744-4759` — the sweep reproduces the finding independently rather
than copying it forward, because a census that trusts its two priors cannot
report that a prior was wrong.

**The verdict threshold is decided before the count, not after it.** "Systematic"
versus "isolated" is the output that changes the project's shape, so it must not
be a post-hoc reading of whatever number appears. State the rule that separates
the two — and the reasoning behind it — in the finding, then count. The
qualitative arm matters as much as the arithmetic one: PS-1 and PS-19 pair at a
similarity score of 40, *"the closest of the six gaps in this batch"*, which is
what raised the suspicion that §4.11's table *"may have been populated with a
systematic assumption that does not match the clause text it sits beside"*
(`.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:70-75`). A third
defect of a *different* shape is weaker evidence of a systematic cause than a
third of the *same* shape; say which was found.

**The escalation arm is a first-class outcome, not a failure.** If the sweep
returns "systematic", the frozen-clause repair is larger than one ADR and this
project's scope is wrong — that is reported at this story's boundary as a
re-plan, never absorbed
(`_decomposition.md`, Architecture brief Note 8 third bullet, and reshape item
4). Equally, "the two known gaps are the only two" is a real result that scopes
`projection-decision-atoms` narrowly and must be recorded with the same evidence
as any other verdict.

**The mount is `references/`, not `.kb/`, and that is a deliberate constraint.**
`CLAUDE.md` ("Where the work lives") states that KB atoms are authored by
`/redkiln:kb-ingest` from `.kb/_intake/` and **not by hand** — the first attempt
at hand-writing them was reverted (`0269720`). So this story writes its evidence
document under `references/evaluation/`, following the precedent its own method
comes from (`references/evaluation/phase-4-5-reconciliation.md`, whose citable
pointer atom `.kb/reference/phase-4-5-specification-reconciliation-census.md` was
produced by an ingest wave afterwards), and stages the KB-side amendments as an
intake document for the next wave rather than editing `.kb/` by hand. The two
open-question atoms are amended by that wave, not by this PR.

**An evaluation document that is not registered is an orphan.**
`references/evaluation/README.md` classifies every document in that directory by
lifecycle, and its "Later additions, which are neither" section exists precisely
for a dated, commit-pinned, immutable byproduct that is *not* one of the original
fourteen (`references/evaluation/README.md:40-55`). This story's document belongs
in that section, dated and pinned, and registering it there is the mount — an
unregistered file in that directory contradicts the README's own claim to
enumerate the directory.

**The persona slice.** The initiative's user here is the adapter author and the
four sibling projects that build against this port; the surface is the public API,
the feature table and what the suite prints
(`_storymap.md`, Backbone preamble). This story serves the *reader of a frozen
clause* — the person who will implement PS-16 or PS-22 next month and needs to
know whether the rule beside it actually follows from the sentence. Their humane
outcome is that no clause in this family silently asks for more than it says, and
where one does, they find it recorded rather than discovering it in a failing test.

**What this story must not do**, each with the owner it belongs to:
line-edit any `[FROZEN]` clause (`unstable-projection-gate-and-clause-disposition`
lands the repair as a new decision atom); write ADR-0017 / 0018 / 0019
(`projection-decision-atoms`); change any maturity marker or rule citation in
`spec/SPECIFICATION.md` (`unstable-projection-gate-and-clause-disposition`,
AC-014); touch `crates/happenstance-core/src/projection.rs` or the testkit
(`owned-batch-port-shape` onward); or settle the adjacent traps —
`ProjectionId`'s validation (`.kb/open-questions/projection-id-is-unvalidated.md`)
and the boundary-scoped checkpoint that would reopen ADR-0013
(`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`). A PS
clause whose defect turns out to be one of those two is *recorded and routed*,
not decided here.

## Integration contract

- **Archetype**: `foundation` — real in-tree evidence consumed by two capability
  stories in this project. Not a double, not a fixme: the artefact it produces is
  cited by path from the ADRs and from the clause-disposition story.
- **Slice / milestone**: `decisions-and-design-record`. Slice-mates:
  `projection-decision-atoms`, `projection-api-design-record`. All three are
  implemented in one context; this row lands **first** within the slice
  (`_storymap.md`, Merge order item 1).
- **Mount point**: **`references/evaluation/README.md`** — the "Later additions,
  which are neither" section (`:40-55`), where the sweep's evidence document is
  registered with its date, its pinned commit and its lifecycle. This is the
  composition root for this medium: `references/evaluation/` is an enumerated,
  lifecycle-classified corpus, and a document dropped into it without a row in the
  README is reachable only by `ls`.
- **Wires into**:
  - `references/evaluation/phase-4-5-reconciliation.md` — the precedent artefact
    whose shape and finding-by-finding form this document follows.
  - `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` — supplies
    the repair/gap classifier the sweep applies verbatim.
  - `.kb/open-questions/ps-1-states-no-progress-obligation.md`,
    `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` — the two priors
    the sweep answers (sub-question 3 and sub-question 2 respectively); amended
    via `.kb/_intake/`, never by hand.
  - `.kb/maps/open-questions-index.md` — the index whose two PS bullets gain the
    sweep's answer in the same ingest wave; staged, not hand-edited.
  - `spec/SPECIFICATION.md` §4 (`:4733-5657`), §4.11 (`:5653-5676`) and §7.2's PS
    block (`:8628-8666`) — read-only inputs.
  - `xtask/src/spec_trace.rs` — read-only; establishes what the gate already
    proves so the sweep does not re-derive it.
- **Renders surfaces**: **none.** `_design.md` records `surfaces: []` for this
  project, approved 2026-08-12. No screen, no public item, no printed line
  changes here.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** This story
  changes no port, adds no rule and admits no implementation, so there is nothing
  a fixture could fail. Its check is documentary and is asserted in this spec's
  acceptance criteria and by the two gate steps that must stay green over an
  unchanged normative tree (`cargo xtask spec-trace`, `redkiln validate --kb`).
- **Clause(s)**: **PS-1 – PS-37, read and dispositioned as evidence; none
  discharged, amended or re-marked.** Every one of the 37 is `[FROZEN]`,
  `[PROVISIONAL]` or `[DEFERRED]` exactly as it is at merge as at branch. The two
  `[FROZEN]` clauses whose repair this evidence scopes are PS-1 (`:4733`) and
  PS-19 (`:5218`); that repair is a new decision atom in
  `unstable-projection-gate-and-clause-disposition`, never an edit.
- **Advances DoD scenario**: initiative **DoD 7** ("The projection suite
  discriminates", `initiative.md:377-379`) — the precondition half. The port
  cannot be frozen honestly while a clause in its family admits an implementation
  its own rule table rejects, so this is the evidence that makes DoD 7's freeze
  readable rather than a mismatch frozen along with the port
  (`.kb/open-questions/ps-1-states-no-progress-obligation.md:64-68`). It also
  feeds initiative exit criterion 3 (`initiative.md:569-570`) and project DoD
  items 3 and 5 (`project.md:245-250`).

## PR boundary

**In this PR**

1. `references/evaluation/ps-clause-pairing-sweep.md` — the evidence document:
   the method, the classifier, the pre-declared systematic/isolated threshold, a
   row per clause PS-1 – PS-37 with its verdict, the exposing implementation for
   every defective row, and the ADR-scope handoff.
2. `references/evaluation/README.md` — one registration entry in "Later
   additions, which are neither", carrying the date and the pinned commit
   (the mount).
3. `.kb/_intake/<dated>-ps-clause-pairing-sweep.md` — the staged KB amendment for
   the next `/redkiln:kb-ingest` wave: the answers to PS-1 sub-question 3 and
   PS-19 sub-question 2, plus the index bullets they change. Staged only; this PR
   does not run the ingest.
4. `.bklg/from-contract-to-published-library/projection-store-freeze/ps-clause-pairing-sweep/**`
   — this story's ledger and implementation report.

**Explicitly not in this PR**

- Any edit to `spec/SPECIFICATION.md` — no clause text, no `**Rule:**` field, no
  maturity marker, no §4.11 or §7.2 row. Owner:
  `unstable-projection-gate-and-clause-disposition` (project AC-014).
- Any `.kb/decisions/` atom, including ADR-0017 / 0018 / 0019. Owner:
  `projection-decision-atoms` (project AC-008).
- Any hand-edit to `.kb/**` outside `_intake/`. Owner: `/redkiln:kb-ingest`
  (`CLAUDE.md`, "Where the work lives").
- Any change under `crates/**`, `xtask/**` or `examples/**`.
- A verdict on `unstable-projection` exposure at publish — that is
  `publication-and-positioning` (HS-P0016)'s, per `project.md:129-131`.

**Merge DoD one-liner.** The sweep is merged when a reader can open one
registered, dated document, see a verdict for every clause PS-1 – PS-37 reached by
one stated test, read the systematic-or-isolated call with the threshold that was
declared before the count, and confirm from `git diff` that not one normative
byte moved — with `cargo xtask ci` and `redkiln validate --kb` green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The sweep covers all 37 PS clauses, none skipped** | Every clause PS-1 through PS-37 gets a row. A clause whose §7.2 rule cell is `*(none — see clause)*` — PS-9, PS-31, PS-32, PS-33, PS-35, PS-36, PS-37 — is not exempt: "no rule assigned" is itself a pairing to verdict on, and a clause with no rule and no falsifier is the defect `spec-trace`'s header names as indistinguishable from a decision nobody wanted to make. | `spec/SPECIFICATION.md:8628-8666`; `xtask/src/spec_trace.rs:11-13` |
| **One classifier, applied per (clause, rule) pair** | For each rule the tables assign to a clause, answer: does an implementation exist that satisfies the clause's `MUST` verbatim and fails that rule? Yes → **defective** (a gap; an ADR's). No → **sound**. Undetermined → **undetermined**, recorded as a gap finding, never rounded to sound. | `.kb/decisions/README.md:20-22`; `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:56-72` |
| **Three attribution sources are read for every clause** | The clause body's own `**Rule:**` field, §4.11's rule → clause table, and §7.2's clause → rule index. A pair present in any one of them is swept. Index-versus-clause *mechanical* disagreement is out of scope — `spec-trace` derives §7.1/§7.2 from `parse_clauses` — and is reported as a tooling observation if seen, not repaired here. | `spec/SPECIFICATION.md:4735-4743`, `:5653-5676`, `:8628-8666`; `xtask/src/spec_trace.rs:22-29` |
| **Every defective row names its exposing implementation** | A defective verdict carries the shape of store that satisfies the `MUST` and fails the rule, stated concretely enough to be built, and says whether it is a plausible first cut or a contrivance. A row that cannot name one is recorded **undetermined**, not defective. | `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:41-48`; `CLAUDE.md`, "The rule that matters" corollaries |
| **PS-1 and PS-19 are re-derived, not inherited** | Both priors are checked independently against today's clause text. If either reproduces, say so and cite it; if either does **not** reproduce, that is the headline finding and the two open-question atoms are wrong — report it rather than quietly agreeing with them. | `spec/SPECIFICATION.md:4744-4759`, `:5218-5249`; `.kb/open-questions/ps-1-states-no-progress-obligation.md:30-51` |
| **Systematic-versus-isolated is declared before it is counted** | The document states the threshold and the reasoning that separates the two verdicts *before* presenting the tally, and distinguishes same-shape defects (evidence of a systematic table-population assumption) from different-shape ones (evidence of isolated incidents). The similarity-40 pairing of PS-1 and PS-19 is the hypothesis under test, named as such. | `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:70-75` |
| **A "systematic" verdict escalates at the story boundary** | If systematic, the document states the re-plan trigger explicitly — which clauses, what the wider repair would have to decide, and that phase 6's budget assumes it is not a re-plan — and the implementation report raises it rather than widening ADR scope in place. | `_decomposition.md`, Architecture brief Note 8 (third bullet) and "What would reshape this map" item 4; `RUNBOOK.md:3848` |
| **The finding hands ADR scope forward, per clause group** | Each defective row names which of ADR-0017 (batch ownership / write vocabulary, PS-4 – PS-15), ADR-0018 (reset and checkpoint scope, PS-16 – PS-20), ADR-0019 (apply-failure policy, PS-26 – PS-30) or a new decision would own it, so `projection-decision-atoms` consumes scope rather than re-reading 37 clauses. | `project.md:65-71`; `_storymap.md`, `projection-decision-atoms` row |
| **Adjacent traps are routed, not settled** | A defective row whose repair would touch `ProjectionId` validation or a boundary-scoped checkpoint is recorded and routed to its existing open question; the sweep states that it stopped there and why. | `.kb/open-questions/projection-id-is-unvalidated.md`; `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`; `project.md:326-329` |
| **The document is immutable-lifecycle evidence** | Dated, pinned to the commit it was written against, superseded rather than edited — the lifecycle `references/evaluation/README.md` assigns to `review-citation-drift.md`, the closest precedent for a byproduct that is not one of the original fourteen. Its registration entry states that lifecycle. | `references/evaluation/README.md:40-55`, `:75-86` |
| **Nothing normative changes** | `git diff --stat` over the merge shows zero changes under `spec/`, `crates/`, `xtask/` and `.kb/` outside `_intake/`. `cargo xtask spec-trace` reports the same clause census as before the branch, and `redkiln validate --kb` and `redkiln doctor` stay green. | `CLAUDE.md`, "Changing a `[FROZEN]` clause requires a new ADR, not an edit"; `_decomposition.md` **AC-A08** |
| **The KB amendment is staged, not written** | The `.kb/_intake/` document is the wave input; the open-question atoms and `.kb/maps/open-questions-index.md` are amended by `/redkiln:kb-ingest`, which prefers AMEND over new atoms. This PR hand-authors no atom. | `CLAUDE.md`, "Where the work lives"; `.kb/maps/open-questions-index.md:8-13` |

**Interfaces.** None in the compiled sense: this story adds no type, no trait, no
function and no feature. Its interface is a document with a fixed contract — one
row per clause, one verdict per row from a three-value vocabulary
(`sound` / `defective` / `undetermined`), a named exposing implementation on every
`defective` row, and one stated verdict for the family — so the two consuming
stories can cite it by `file:line` without reading it end to end.

## Data and migrations

**N/A.** This story touches no schema, no persisted state, no serialised format
and no stored artefact. It adds no table, no column and no on-disk representation:
`ProjectionStore`, its `Batch` and its checkpoint types are untouched here
(`crates/happenstance-core/src/projection.rs` is out of the PR boundary), and the
only files it writes are Markdown under `references/`, `.kb/_intake/` and this
story's own backlog folder.

The one migration-shaped concern in the neighbourhood is deliberately **not**
this story's: `owned-batch-port-shape` removes the batch lifetime parameter, which
is a source-level change to six skeleton `impl` sites and is bounded by project
AC-013's "no change other than the removal of the batch's lifetime parameter"
(`project.md:221-224`). Nothing here anticipates it.

## Acceptance criteria

The persona is the **adapter author** (`_discovery/distillation/personas-and-journeys.md:114-127`)
in the moment they are most exposed: reading a `[FROZEN]` clause in order to
implement against it, afraid of "discovering — late, expensively — that the port
they implemented against quietly assumed something their storage system cannot
provide" (`:146-150`). Every criterion below is that person's goal crossing the
whole stack of this medium — clause text, rule attribution table, conformance
rule, and the document they will actually open. All eight trace to project
**AC-008** (`project.md:204-209`), whose sweep half `_storymap.md:86` assigns to
this story.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author about to implement PS-16, who cannot tell from `spec/SPECIFICATION.md` alone whether any clause other than PS-1 and PS-19 asks for more than it says, **WHEN** they open `references/evaluation/ps-clause-pairing-sweep.md`, **THEN** they find a row for every clause PS-1 through PS-37 with no omissions and no "not applicable" escapes — including the seven whose §7.2 rule cell reads `*(none — see clause)*` (PS-9, PS-31, PS-32, PS-33, PS-35, PS-36, PS-37), for which "no rule assigned" is itself the pairing under verdict — and each row carries exactly one verdict from the closed vocabulary `sound` / `defective` / `undetermined`. | Static: `rg -c '^\| PS-' references/evaluation/ps-clause-pairing-sweep.md` returns `37`, and `rg -o 'PS-[0-9]+' … \| sort -u \| wc -l` returns `37`; every verdict cell matches `^(sound\|defective\|undetermined)$`. Review check in `_review.md` confirms the seven no-rule clauses are verdicted rather than excused. Ground: `spec/SPECIFICATION.md:8628-8666`. |
| AC-002 | **GIVEN** that same author asking "is this rule the right rule for this clause?" — the exact question `cargo xtask spec-trace` states it cannot answer (`xtask/src/spec_trace.rs:17-18`) — **WHEN** they read the document's method section, **THEN** they find **one** stated classifier, applied identically to every `(clause, rule)` pair drawn from all three attribution sources (the clause body's own `**Rule:**` field, §4.11's rule → clause table, §7.2's clause → rule index): *does an implementation exist that satisfies the clause's `MUST` verbatim and fails that rule?* — yes → `defective` (a gap, an ADR's), no → `sound`, cannot tell → `undetermined`, and **THEN** they can re-apply it to a clause themselves and reach the same verdict. | Review: the method section states the classifier verbatim against `.kb/decisions/README.md:20-22` and `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:56-72`, and each row's rationale names which of the three sources supplied the pair. Spot-re-derivation of three rows by the slice reviewer must reproduce the verdict; a disagreement is recorded, not overwritten. |
| AC-003 | **GIVEN** the corollary that a finding no implementation can exhibit is decorative in exactly the way a rule no adapter can fail is (`CLAUDE.md`, "The rule that matters"), **WHEN** the author reads any row verdicted `defective`, **THEN** it names the shape of store that satisfies the clause's `MUST` and fails the assigned rule — concretely enough to be built — and says whether that store is a plausible first cut or a contrivance; **AND** a row for which no such store can be named is recorded `undetermined`, never rounded to `sound` and never left as an unsupported `defective`. | Review against the bar the priors already clear: `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:41-48` ("the natural implementation, not a contrivance"). Static: every row whose verdict is `defective` has a non-empty exposing-implementation cell; `rg` over the census table shows no `defective` row with an empty or `TBD` cell. |
| AC-004 | **GIVEN** two open-question atoms that already assert a defect and could simply be believed, **WHEN** the author reads the PS-1 and PS-19 rows, **THEN** each verdict is derived independently from today's clause text and cited to it (`spec/SPECIFICATION.md:4744-4759`, `:5218-5249`), not copied from the atom; **AND** if either fails to reproduce, that non-reproduction is stated as the document's headline finding — the atoms are reported wrong rather than quietly agreed with. | Review: the PS-1 and PS-19 rows cite `spec/SPECIFICATION.md` line ranges as their evidence, with the open-question atom cited only as the prior under test. `_review.md` records that the reviewer checked the derivation is not a paraphrase of `.kb/open-questions/ps-1-states-no-progress-obligation.md:30-51`. |
| AC-005 | **GIVEN** that "systematic" and "isolated" send this project down different roads, **WHEN** the author reaches the verdict section, **THEN** the threshold separating the two — and the reasoning behind it — appears **before** the tally, the count is presented against it, and same-shape defects (evidence of a systematic §4.11 table-population assumption) are distinguished from different-shape ones (evidence of isolated incidents), with the similarity-40 PS-1/PS-19 pairing named as the hypothesis under test; **AND** if the verdict is systematic, the document states the re-plan trigger explicitly — which clauses, what a wider repair would have to decide — rather than widening ADR scope in place. | Review: document order is checked literally (threshold section precedes tally section). Ground: `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:70-75`; escalation path per `_decomposition.md` Architecture brief Note 8 and "What would reshape this map" item 4. A systematic verdict additionally requires a re-plan note in the implementation report, raised at the story boundary. |
| AC-006 | **GIVEN** that `projection-decision-atoms` must scope three ADRs without re-reading 37 clauses, **WHEN** its implementer opens the sweep, **THEN** every `defective` row names which of ADR-0017 (batch ownership and write vocabulary, PS-4 – PS-15), ADR-0018 (reset and checkpoint scope, PS-16 – PS-20), ADR-0019 (apply-failure policy, PS-26 – PS-30) or a new decision would own it; **AND** any row whose repair would touch `ProjectionId` validation or a boundary-scoped checkpoint is **routed** to its existing open question with a statement that the sweep stopped there and why, never settled in passing. | Review against `project.md:65-71` (the three ADRs' clause groups) and `_storymap.md:54`. Static: every `defective` row has a non-empty owner cell drawn from `{ADR-0017, ADR-0018, ADR-0019, new decision, routed}`; every `routed` cell cites `.kb/open-questions/projection-id-is-unvalidated.md` or `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`. |
| AC-007 | **GIVEN** that a document dropped into `references/evaluation/` without a README row is reachable only by `ls`, and that the README claims to enumerate the directory, **WHEN** anyone opens `references/evaluation/README.md`, **THEN** the sweep appears in "Later additions, which are neither" (`:40-55`) with its date, the commit it is pinned to, and the immutable *supersede-rather-than-edit* lifecycle stated — the same lifecycle that section already assigns to `review-citation-drift.md` — **AND** the document itself carries that date and pin in its own header. | Review of the `references/evaluation/README.md` diff: exactly one entry added, in that section, carrying a date and a real commit sha resolvable by `git cat-file -e <sha>`. Static: `rg -n 'ps-clause-pairing-sweep' references/evaluation/README.md` returns a hit inside that section's line range. |
| AC-008 | **GIVEN** that `CLAUDE.md` forbids editing a `[FROZEN]` clause and forbids hand-writing `.kb/` atoms, and that this story's whole value is being a baseline the next two stories are judged against, **WHEN** a reviewer runs `git diff` over the merge, **THEN** not one normative byte has moved — zero changes under `spec/`, `crates/`, `xtask/`, `examples/`, and under `.kb/` outside `_intake/` — the KB-side amendments to the two open-question atoms and `.kb/maps/open-questions-index.md` exist only as a staged `.kb/_intake/` document for the next `/redkiln:kb-ingest` wave, and `cargo xtask spec-trace`, `redkiln validate --kb` and `redkiln doctor` are green with the same clause census and the same six `template-drift` advisories as on `main`. | Static: `git diff --stat main...HEAD -- spec/ crates/ xtask/ examples/` is empty; `git diff --stat main...HEAD -- .kb/ ':!.kb/_intake/'` is empty; `git diff --diff-filter=D main...HEAD -- .kb/open-questions/` is empty (the testing brief's own AC-008 check, `_decomposition.md` Testing brief AC-008 row). Gate: `cargo xtask spec-trace`, `redkiln validate --kb`, `redkiln doctor`, then `cargo xtask ci`. |

## Interaction quality

**Composition family — declared not applicable, on the design's authority.**
`_design.md` records `surfaces: []`, approved 2026-08-12
(`_design.md:48-50`, `:96-101`), and its Items / Signatures / Placement /
States / Anti-patterns sections all read `N/A — no user-facing surface`
(`:52-90`). This story renders no screen, adds no public item, changes no
printed line and introduces no control, so *presentation exists at all*,
*transience*, *hierarchy* and the design's named anti-patterns have no referent
here. This is a declared skip with a signed-off basis, not a silent pass, and
this story does not re-decide it.

One composition invariant does survive the translation into this medium, and it
is the reason the mount is what it is: **an artifact with no registration is the
documentary equivalent of bare markup.** `references/evaluation/` is an
enumerated, lifecycle-classified corpus; a file in it with no README row renders
as an orphan. That invariant is carried by **AC-007**, as a table row, not as a
bullet here.

**State family — the invariants that do apply, and the AC that carries each.**
Every one is an `AC-###` row above; this section only says which.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Non-occlusion** — no clause is hidden behind a judgement call; the seven clauses with no assigned rule are not silently dropped, and `undetermined` is a visible third state rather than a rounding to `sound` | **AC-001**, **AC-003** | `rg` census count of 37 plus the closed three-value verdict vocabulary; empty exposing-implementation cell forces `undetermined` |
| **In-place, not context-jump** — the reader gets one document with one row per clause and does not have to reconstruct the finding by hopping between three specification tables | **AC-001**, **AC-002** | each row names which of the three attribution sources supplied its pair, so the hop is recorded once and not repeated per reader |
| **Preserved selection / no silent state change** — the normative tree is exactly as it was; the reader's existing citations into `spec/SPECIFICATION.md` still resolve, and no maturity marker moved under them | **AC-008** | empty `git diff` over `spec/`, `crates/`, `xtask/`; `cargo xtask spec-trace` green with an unchanged clause census |
| **Reversibility** — the document is immutable-lifecycle evidence: superseded rather than edited, so a later correction never destroys the baseline the next two stories are judged against | **AC-007** | the registration entry states the supersede-not-edit lifecycle, matching `references/evaluation/README.md:40-55` |
| **Reachability** (this medium's *keyboard reachability*) — every reader route into the finding is a real one: the README row, the `file:line` citations the two consuming stories will use, and the staged intake document that will carry it into `.kb/` | **AC-007**, **AC-006**, **AC-008** | README `rg` hit; per-row ADR owner cells; the `.kb/_intake/` document present and the `.kb/` tree otherwise untouched |
| **Reversible escalation** — a "systematic" verdict surfaces as a re-plan at the story boundary instead of silently expanding the next story's scope | **AC-005** | the threshold-before-tally ordering check plus the implementation report's re-plan note |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The three attribution sources disagree *mechanically* for some clause — the clause body's `**Rule:**` field names a rule §7.2 does not, or vice versa. | Out of scope for repair: `spec-trace` derives §7.1/§7.2 from `parse_clauses` (`xtask/src/spec_trace.rs:22-29`), so a mechanical disagreement is a tooling bug. Record it as a tooling observation in the document, sweep the union of the pairs, and do not edit the specification. |
| **EC-002** | A clause looks wrong but no store can be named that satisfies its `MUST` and fails the rule. | Verdict `undetermined`, with the reasoning that stalled written out. Per the playbook, inability to tell **is** a gap finding — it is never rounded up to `defective` for narrative tidiness nor down to `sound` to clear the row (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:56-72`). |
| **EC-003** | PS-1 or PS-19 does **not** reproduce against today's clause text. | That is the headline finding. State it first, cite the clause lines, and record that the corresponding open-question atom's premise is wrong — staged for the ingest wave as a correction to the atom, never as a hand-edit and never as a quiet agreement with the atom instead. |
| **EC-004** | The count crosses the pre-declared systematic threshold. | Escalate at this story's boundary as a re-plan: name the affected clauses, say what a wider repair would have to decide, and state that phase 6's budget assumed otherwise (`_decomposition.md`, Architecture brief Note 8 third bullet; "What would reshape this map" item 1). Do **not** widen `projection-decision-atoms`' ADR scope in place. |
| **EC-005** | A `defective` row's repair would require settling `ProjectionId` validation or a boundary-scoped checkpoint (which would reopen ADR-0013). | Route it: record the row, cite `.kb/open-questions/projection-id-is-unvalidated.md` or `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`, and state that the sweep stopped at the boundary and why (`project.md:326-329`). |
| **EC-006** | `redkiln validate --kb` fails, or `git diff` shows a change under `.kb/` outside `_intake/`. | A hand-authored atom has landed. Revert it and move the content into the `.kb/_intake/` document — `/redkiln:kb-ingest` owns atom authorship, and the first attempt at hand-writing atoms was reverted for exactly this reason (`CLAUDE.md`, "Where the work lives"; commit `0269720`). |
| **EC-007** | The evidence document exists but `references/evaluation/README.md` carries no row for it. | The mount is incomplete and the story is not done: the README claims to enumerate the directory, so an unregistered file contradicts it. AC-007 is unsatisfied regardless of how good the census is. |
| **EC-008** | The sweep's own `file:line` citations do not resolve at the pinned commit. | Fix before merge. This directory's named failure mode is precisely citation drift that still passes `spec-trace` (`references/evaluation/README.md:50-55`), and a census whose citations rot is a census nobody can re-derive. |

## Non-functional

| id | requirement | why it is load-bearing here |
| --- | --- | --- |
| **NF-001** | **Citation integrity.** Every `file:line` in the document resolves at the commit it is pinned to, and the pin is stated in the document header and the README row. | `review-citation-drift.md` §1 records six citations that resolve, pass `spec-trace`, and point at the wrong line (`references/evaluation/README.md:50-55`). This document's whole value is being re-derivable. |
| **NF-002** | **Citable without being read end to end.** The two consuming stories must be able to cite a single row by `file:line`; the census table is the index and no finding lives only in prose outside it. | `projection-decision-atoms` consumes scope, not narrative; `unstable-projection-gate-and-clause-disposition` consumes per-clause dispositions. |
| **NF-003** | **Immutability.** Dated, pinned, superseded rather than edited. | The separation of discovery from decision only holds if the discovery artifact cannot be retro-fitted to the decision (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:108-119`). |
| **NF-004** | **Zero gate cost.** No new CI step, no new dependency, no compile-time or runtime change; `cargo xtask ci` takes the same shape and the same steps as on `main`. | This story adds documentation only; a gate that grew here would be a signal the PR boundary was crossed. |
| **NF-005** | **Reproducibility by a second reader.** The method is stated well enough that someone re-applying the classifier to a sampled clause reaches the same verdict — or their disagreement is itself recorded as a finding rather than resolved by authority. | A census that only its author can reproduce cannot scope an ADR; and disagreement about a pairing is exactly the signal the sweep exists to surface. |
| **NF-006** | **Vocabulary discipline.** Three verdicts, one owner vocabulary, one exposing-implementation field — no per-row prose schema. | Uniform rows are what let the next story consume the table mechanically instead of re-reading 37 arguments. |

## Implementation notes (non-prescriptive)

- **Read the classifier before the clauses.** `.kb/decisions/README.md:20-22` and
  `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:56-72` supply
  the repair/gap test verbatim; adopting it as-is is cheaper and more defensible
  than deriving a new one, and the playbook is already the authority the next two
  stories will be judged against.
- **Copy the shape, not the content, of the precedent.**
  `references/evaluation/phase-4-5-reconciliation.md` is the closest artefact in
  the tree — finding-by-finding, dated, pinned — and following its form makes the
  README registration and the eventual ingest wave routine rather than novel.
- **Declare the threshold in writing before opening clause PS-2.** The ordering
  is the integrity mechanism; writing it after the count is indistinguishable from
  writing it to fit the count, and AC-005 checks document order literally.
- **Sweep the union, not the intersection.** A pair present in any one of the
  three attribution sources is swept. §4.11 and §7.2 are large tables; the clause
  body's `**Rule:**` field is easy to skip and is where PS-1's own recorded defect
  sits (`spec/SPECIFICATION.md:4735-4759`).
- **Batch the seven no-rule clauses deliberately.** PS-9, PS-31, PS-32, PS-33,
  PS-35, PS-36 and PS-37 have `*(none — see clause)*` in §7.2. They are the rows
  most likely to be skimmed and the ones where "no rule and no falsifier" is a
  finding in its own right; give them their own pass rather than trailing them.
- **Write the `.kb/_intake/` document as the wave's input, not as an atom.** Date
  its filename, and — per the ingest wave convention already learned in this repo
  — do not assume a single wave: name it so a second wave cannot overwrite the
  first's audit trail. The wave, not this PR, amends
  `.kb/open-questions/ps-1-states-no-progress-obligation.md`,
  `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` and
  `.kb/maps/open-questions-index.md`.
- **Run the diff checks as you go, not at the end.** `git diff --stat` over
  `spec/`, `crates/`, `xtask/` is a two-second check and the cheapest way to catch
  a "while I was in there" edit before it becomes an argument at review.
- **If the sweep starts to feel like a specification rewrite, stop.** That feeling
  is the boundary being crossed: the repair belongs to
  `unstable-projection-gate-and-clause-disposition` and the decisions to
  `projection-decision-atoms`.

## Tests and CI (merge gate)

Tier vocabulary is the project Testing brief's (`_decomposition.md` Testing
brief, "Intent"): **Static** reads source without executing the code under test;
**Unit** is `cargo test` in-process; **Integration** drives a real store;
**E2E** is `cargo xtask ci` run whole. This story is Static and review-tier only
— it executes no library code, which is why the Testing brief already types the
project's AC-008 as **Static** and as *"a process gate on commit sequence as much
as a schema check"*.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `rg -c '^\| PS-' references/evaluation/ps-clause-pairing-sweep.md` → `37`; unique-id count also `37` | AC-001 — the census is complete and no clause is omitted or duplicated |
| Static | `rg -n 'sound\|defective\|undetermined' references/evaluation/ps-clause-pairing-sweep.md` — every verdict cell inside the closed vocabulary | AC-001 — one verdict per row, no free-text verdicts |
| Static | `rg -n 'ps-clause-pairing-sweep' references/evaluation/README.md` inside the "Later additions" section (`:40-55`); `git cat-file -e <pinned-sha>` | AC-007 — the mount exists and the pin resolves |
| Static | `git diff --stat main...HEAD -- spec/ crates/ xtask/ examples/` → empty | AC-008 — nothing normative moved |
| Static | `git diff --stat main...HEAD -- .kb/ ':!.kb/_intake/'` → empty; `git diff --diff-filter=D main...HEAD -- .kb/open-questions/` → empty | AC-008 — no hand-authored atom, no deleted open question (the Testing brief's own AC-008 check) |
| Static | `cargo xtask spec-trace` (`xtask/src/spec_trace.rs`) | AC-008 — the clause census and every cross-reference is identical to `main`'s |
| Static | `redkiln validate --kb` and `redkiln doctor` | AC-008 — KbFrontmatter conformance, accepted-decision immutability, and exactly the six expected `template-drift` advisories |
| Review (`_review.md`) | adversarial slice review re-derives three sampled rows with the document's own classifier | AC-002, AC-003 — the classifier is stated, uniform, and reproducible; every `defective` row names a buildable store |
| Review (`_review.md`) | PS-1 and PS-19 rows checked for independent derivation against `spec/SPECIFICATION.md:4744-4759`, `:5218-5249` | AC-004 — the priors were tested, not inherited |
| Review (`_review.md`) | document order check: threshold section precedes tally section; shape reasoning present | AC-005 — the verdict was not fitted to the count |
| Review (`_review.md`) | every `defective` row's owner cell against `project.md:65-71`; every `routed` cell against its open-question atom | AC-006 — ADR scope is handed forward and adjacent traps are routed, not settled |
| E2E | `cargo xtask ci` on a clean checkout | project DoD 4's precondition — the gate is green over a tree whose normative content is unchanged (`project.md` DoD 4) |

Story-grain iteration uses `cargo xtask affected --base main` and
`cargo xtask ci --fast`; the two steps `--fast` omits — `spec-trace` and the
mandatory wasm32 conformance-harness check — are exactly the ones AC-008 leans
on, so the full `cargo xtask ci` is the merge bar for this row even though the
project is non-terminal (`_decomposition.md` Testing brief, "Merge-gate
commands").

## Risks and coupling (PR-scoped)

| Risk | Why it bites here | Mitigation in this PR |
| --- | --- | --- |
| **The sweep becomes a specification rewrite.** Reading 37 clauses closely makes the urge to fix one nearly irresistible, and a one-word `[FROZEN]` edit looks harmless. | It would destroy the baseline the next two stories are judged against and violate `CLAUDE.md`'s "a new ADR, not an edit". | The PR boundary names it, EC-006 and AC-008 make it a failing diff check, and `cargo xtask spec-trace` runs on every gate. |
| **The verdict is shaded to "isolated" to keep the plan intact.** A systematic verdict costs a re-plan; nobody wants one at slice 1. | It would ship a scoped ADR set against evidence that does not support it — the exact failure the discovery/decision split exists to prevent. | AC-005's threshold-before-tally ordering, and EC-004's explicit escalation path as a *first-class outcome*, not a failure. |
| **PS-1's clause body already records its defect, so the row gets copied forward.** | A census that trusts its priors cannot report that a prior was wrong, and PS-1's prior is the one most likely to be uncritically inherited. | AC-004 requires independent derivation with clause-line citations; the review samples it. |
| **`undetermined` becomes a dumping ground** — or, worse, disappears because every row was forced to a binary. | Either way the count is fiction and the systematic/isolated verdict rests on it. | AC-003 makes `undetermined` the required outcome when no store can be named, and NF-005 asks a second reader to reproduce sampled rows. |
| **Slice coupling: the ADRs are drafted before the sweep is final.** All three `decisions-and-design-record` stories are implemented in one context. | `projection-decision-atoms` consumes this story's scope; a late change to the sweep invalidates drafted ADRs. | Merge order fixes this row **first** within the slice (`_storymap.md:107`), and `projection-decision-atoms` lists `ps-clause-pairing-sweep` in its `depends_on` (`:54`). |
| **Cross-slice coupling: slice 8 is waiting.** `unstable-projection-gate-and-clause-disposition` lists this story in its `depends_on` (`_storymap.md:68`) and lands the frozen-clause repair. | An `undetermined`-heavy sweep leaves slice 8 without the disposition it needs, seven slices later. | Every `undetermined` row states what would resolve it, so slice 8 inherits a question with a shape rather than a shrug. |
| **The staged intake document is never consumed.** `/redkiln:kb-ingest` is a user-invoked command; the wave may not run for some time. | The two open-question atoms and `.kb/maps/open-questions-index.md` keep pointing at unanswered sub-questions while the answer sits in `references/`. | The document and the README row both state that the KB-side amendment is staged and name the intake file, so the answer is findable from `.kb/` even before the wave runs. |
| **Citation drift inside the sweep itself.** 37 rows of `file:line` citations into a 9,000-line specification. | This directory's own recorded failure mode (`references/evaluation/README.md:50-55`). | NF-001: the document is pinned to a commit, and the review spot-checks citations against that commit. |

## Dependencies

**Blocks on: nothing.** `depends_on: []` — this is the first row in the
`decisions-and-design-record` slice and the first story in the project
(`_storymap.md:53`, merge order item 1 at `:107`). Everything it reads is already
in the tree: `spec/SPECIFICATION.md`, the two open-question atoms, the playbook,
`references/evaluation/`, and `xtask/src/spec_trace.rs`.

**Unlocks:**

| Story slug | Relationship |
| --- | --- |
| `projection-decision-atoms` | Direct `depends_on` (`_storymap.md:54`). Consumes the per-row ADR-owner cells as scope for ADR-0017 / 0018 / 0019 instead of re-reading 37 clauses. |
| `projection-api-design-record` | Transitive, through `projection-decision-atoms` (`_storymap.md:55`). |
| `unstable-projection-gate-and-clause-disposition` | Direct `depends_on` across slices (`_storymap.md:68`). Lands the frozen-clause repair as a new decision atom, and re-marks maturity for PS-1 – PS-37 using this sweep's dispositions (project AC-014). |
| `/redkiln:kb-ingest` (a human-invoked wave, not a story) | Consumes the `.kb/_intake/` document to amend the two open-question atoms and `.kb/maps/open-questions-index.md`. |

No dependency reaches outside this initiative, and this story owns no substrate
another project maintains.

## Anchors (progressive disclosure)

The Context pack above is the must-read core and is self-sufficient. These are the
deeper artifacts — open each at the moment named, and link rather than paste.

| anchor (real path) | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` | The subject matter. §4's clause bodies (`:4733-5657`), §4.11's rule → clause table (`:5653-5676`) and §7.2's PS index (`:8628-8666`) are the three attribution sources the sweep reads; it is read-only input and nothing in it is edited. | First — the census cannot begin without the clause list; §7.2 supplies the row skeleton. | AC-001, AC-002 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | Supplies the repair-vs-gap classifier verbatim (`:56-72`) and, at `:108-119`, the reason discovery and decision are separate passes — the sentence that makes "no fix in this PR" a design property rather than an omission. | Before writing the method section, and again if the urge to attach a fix appears. | AC-002 |
| `.kb/decisions/README.md` | States the repair/gap distinction at its source (`:20-22`); the playbook restates it, and citing both is what makes the classifier defensible rather than invented here. | Alongside the playbook, when fixing the classifier's wording. | AC-002 |
| `.kb/open-questions/ps-1-states-no-progress-obligation.md` | Prior #1 under test. Its `:30-51` is the argument to re-derive rather than inherit; `:64-68` is why a mismatch frozen alongside the port is the failure mode; `:77-81` is the instruction this story carries out. | When writing the PS-1 row — after independently deriving the verdict, to compare. | AC-004 |
| `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` | Prior #2 under test, and the source of the exposing-implementation bar (`:41-48`, "the natural implementation, not a contrivance") and of the systematic-assumption hypothesis (`:70-75`, similarity 40). | When writing the PS-19 row, and before declaring the systematic/isolated threshold. | AC-003, AC-005 |
| `xtask/src/spec_trace.rs` | Fixes the boundary between what the gate already proves and what judgement owes: `:17-18` states it cannot tell whether a rule is the right rule; `:11-13` names the no-rule-no-falsifier defect; `:22-29` shows §7.1/§7.2 are derived, which is why mechanical disagreement is out of scope. | Before deciding which disagreements to sweep and which to report as tooling observations. | AC-002 |
| `references/evaluation/README.md` | The mount. `:40-55` is the "Later additions, which are neither" section and the immutable dated-and-pinned lifecycle the registration must state; `:50-55` names citation drift as this directory's own failure mode. | When registering the document — the last edit of the PR, once the pin is known. | AC-007 |
| `references/evaluation/phase-4-5-reconciliation.md` | The precedent artefact: a dated, pinned, finding-by-finding evaluation document. Its form is what this document copies, which keeps the README row and the later ingest wave routine. | Before drafting the document's structure. | AC-001, AC-003 |
| `.kb/reference/phase-4-5-specification-reconciliation-census.md` | Establishes the delta this story owes: `:108-112` records that the earlier reconciliation stopped at six recorded gaps without sweeping any neighbourhood. Reading it prevents re-running a census that already exists. | Before writing the "what is new here" framing and the threshold reasoning. | AC-005 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | Architecture brief **Note 8** ("`[FROZEN]` clauses this work will want to widen — and must not") and "What would reshape this map" item 4 define the escalation contract; the Testing brief's AC-008 row defines the Static checks this story is graded by. | Before choosing between escalation and absorption, and when assembling the CI table. | AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` | `:65-71` fixes which clause groups ADR-0017 / 0018 / 0019 own — the vocabulary the owner cells must use; `:326-329` fixes the adjacent traps that are routed rather than settled. | When filling every `defective` row's owner cell. | AC-006 |
| `.kb/open-questions/projection-id-is-unvalidated.md` | One of the two adjacent traps. A row whose repair would harden `ProjectionId` is routed here, not decided. | Only when a row's repair reaches the identifier. | AC-006 |
| `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` | The second adjacent trap: a boundary-scoped checkpoint would reopen ADR-0013. Routing, not settling. | Only when a row's repair reaches checkpoint scope. | AC-006 |
| `.kb/maps/open-questions-index.md` | The index whose two PS bullets gain the sweep's answer — in the ingest wave, not in this PR. Read it to write the staged intake document against the real current wording. | When drafting the `.kb/_intake/` document. | AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off design records `surfaces: []` (`:48-50`) and its sign-off (`:96-101`) makes the no-surface determination itself the approved decision. It is why the composition family of interaction quality is a declared skip here. | Before writing anything that would render or print. | AC-007 |
| `RUNBOOK.md` | `:3848` places this work in phase 6 ("Freeze `ProjectionStore`") and `:296-298` puts ADR-0017 / 0018 / 0019 in the queue — the budget a "systematic" verdict would break. | When stating the re-plan trigger, if the verdict is systematic. | AC-005 |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's eight**; none added, none dropped.
   AC-001 – AC-008 consolidate the twelve behaviour rows of "Behavior and
   interfaces": the three-attribution-source obligation folds into AC-002 (it is
   the classifier's input set, not a separate deliverable), the escalation
   obligation folds into AC-005 (it is what the verdict *does*, and separating
   them would let a spec pass with a verdict nobody acted on), and the immutable
   lifecycle folds into AC-007 (the registration entry is where the lifecycle is
   stated, so one row checks both).
2. **The verdict vocabulary is closed at three values** — `sound`, `defective`,
   `undetermined`. A fourth ("out of scope", "N/A") was considered and rejected:
   it is where the seven no-rule clauses would have gone, and their pairing is
   precisely what the sweep exists to judge.
3. **The systematic/isolated threshold is deliberately *not* fixed by this spec.**
   Naming a number here would be the planning stage deciding the finding. What the
   spec fixes is the *ordering* — declared before counted — and the requirement
   that shape, not only arithmetic, carries the argument.
4. **Scope is the PS family only.** CF, ES and SY clauses are not swept. The
   hypothesis under test is about §4.11's table population
   (`.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:70-75`), and widening
   to other families would turn a bounded first story into a specification-wide
   audit that nothing in this project's plan has budget for. If a PS finding
   suggests the same shape elsewhere, that is a recorded observation and a
   candidate open question, not extra rows.
5. **Mechanical index/clause disagreement is out of scope** and is reported, not
   repaired — `spec-trace` derives §7.1/§7.2 (`xtask/src/spec_trace.rs:22-29`), so
   a mechanical mismatch is a tooling bug with an existing owner. The sweep's
   subject is semantic disagreement no tool can see.
6. **The evidence lands in `references/`, not `.kb/`**, and the KB-side amendment
   is staged in `.kb/_intake/`. Hand-writing atoms produces the directory layout of
   the process without the process, which is why the first attempt was reverted
   (`CLAUDE.md`, "Where the work lives"; commit `0269720`).
7. **No ADR is written here**, including one that would "just record what the
   sweep found". Authorship belongs to `projection-decision-atoms`, and an ADR
   written as a side effect of a discovery pass is the un-auditable artifact the
   playbook's `:108-119` names.
8. **The composition family of interaction quality is a declared skip**, on
   `_design.md`'s approved `surfaces: []` (`:48-50`, `:96-101`) — recorded
   explicitly rather than omitted, so a reader can tell a decision from an
   oversight. The one surviving composition invariant (registration, or the
   document is an orphan) is carried as AC-007, a table row, not a bullet.
9. **All eight ACs trace to project AC-008**, per `_storymap.md:86`. The story
   owns the *sweep* half of that AC; the ADR-acceptance half is
   `projection-decision-atoms`'.
