---
item: "HS-S0001"
stage: report
created: "2026-08-13"
updated: "2026-08-13"
---

# Report — Sweep PS-1 – PS-37 for the pairing defect before any repair is scoped

## Findings Ledger

The story's outcome as the review gate reads it. Eight ACs, all satisfied; nothing
blocked, nothing deferred, nothing stubbed. The deliverable is one registered,
dated, commit-pinned evidence document plus one staged intake file, and the only
mount this medium has — the `references/evaluation/README.md` enumeration — is
made.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **The census is complete and the verdicts are closed.** 37 rows, PS-1 – PS-37, no omissions and no "not applicable" escape; the seven clauses whose §7.2 rule cell reads `*(none — see clause)*` are verdicted on their no-rule pairing rather than excused. **7 `defective`, 29 `sound`, 1 `undetermined`.** | `references/evaluation/ps-clause-pairing-sweep.md`, *The census*; static check `grep -c '^\| PS-'` = 37, unique ids = 37, zero cells outside the vocabulary (red: 0 rows) | None. `projection-decision-atoms` and `unstable-projection-gate-and-clause-disposition` consume the table by row. |
| **The verdict is `ISOLATED`, against a threshold declared before the count.** Three `independent` same-shape defects (arm (a) threshold: five); two of them inside §4.11's seventeen-rule table (arm (b) threshold: three, and two is the prior itself). The hypothesis that §4.11's table was populated from a systematic assumption is **not supported**. | *The threshold, declared before the count* precedes *The tally, against the threshold* in the document; the similarity-40 PS-1/PS-19 pairing is named as the hypothesis under test | **No re-plan.** Phase 6's budget is the right size. `projection-decision-atoms` scopes three ADRs narrowly rather than widening them. |
| **A third independent defect exists, and it is outside the table.** PS-29's `MUST` says a poisoned projection's terminal state must be *"observable through the API"*; its rule additionally demands the supervisor report *"without being polled for it"*. A supervisor exposing `fn failures(&self) -> Vec<Poisoned>` satisfies the sentence and fails the rule. | Census row PS-29; `spec/SPECIFICATION.md:5446-5453` | **ADR-0019 names it** (PS-26 – PS-30) and defers the observability design to HS-P0011; the clause repair is a new decision at `unstable-projection-gate-and-clause-disposition`. |
| **A prior is partly refuted.** Both PS-1 and PS-19 reproduce as pairing defects, re-derived from clause text rather than inherited. But `.kb/open-questions/ps-1-states-no-progress-obligation.md:48-51`'s claim that progress is *"stated by no clause's `MUST` anywhere in the document"* is wrong: PS-23's *"One `commit` advances exactly one `ProjectionId`"* states it — on a clause about fan-out scope, and a `[PROVISIONAL]` one. | *The headline finding: a prior is partly refuted*; `spec/SPECIFICATION.md:5317-5329` | The repair now has **three** candidate shapes, not one foregone edit. Amendment staged at `.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md` §1b — appended and dated, body left verbatim, per `.kb/open-questions/README.md`. |
| **Four dependent defects, three of them one cause.** `rollback_leaves_both_unchanged` (PS-8), `commit_accepts_a_position_the_batch_did_not_write` (PS-21) and `commit_rejects_a_regressing_position` (PS-22) each enforce an obligation no clause's `MUST` states. PS-13's rule is falsified by a store while its `MUST` binds a projection. | Census rows PS-8, PS-13, PS-21, PS-22, each with an exposing implementation and a named strength | PS-1's repair should be scoped as *which clause states progress, and which rules rest on it*. PS-8 and PS-13 are ADR-0017's to name. |
| **One `undetermined`, not rounded.** PS-28's rule asserts the checkpoint *"sits at the last good position"*; under *last applied event* every chunked runner fails while satisfying the `MUST`, under *last committed position* the pairing is sound. The text supports both and the sweep declines to pick. | Census row PS-28 | **What resolves it:** defining the phrase when the rule is written. Integration-level; HS-P0011's. |
| **Two cross-clause findings the classifier does not catch.** (1) `rebuild_is_chunk_size_invariant` is ungated while `batch_reads_reflect_pending_writes` carries `READS_THROUGH_BATCH`, so a write-behind adapter blessed by PS-4 and PS-12's second arm cannot pass it. (2) PS-15's `MUST` binds `commit`, `reset` **and** `rollback`; its rule exercises `commit` only. | *Cross-clause findings* | (1) routed to ADR-0017 (the probe's design) and the suite stories; (2) routed to the suite stories. |
| **Five inverse-shape observations, three of one shape.** PS-3, PS-31 and PS-36 each carry a documentation obligation with no instrument. | *Inverse-shape observations* | Candidate open question, named in the intake file and deliberately **not** opened. This project already carries the instrument (`standards/rust/70-rustdoc-obligations.md`, and the design record's `## Visibility and stability`). |
| **Three tooling observations, reported not repaired.** `parse_clauses` lifts a non-rule identifier out of a `**Rule:**` field at PS-10 (`compile_fail`), PS-16 (`probe_delete_all`) and PS-27 (`on_error`). | *Tooling observations*; `xtask/src/spec_trace.rs:22-29` | Out of scope per EC-001 — §7.1/§7.2 are derived, so this is a tooling bug with an existing owner. None changes a verdict. |
| **Nothing normative moved, and nothing was decided.** Zero diff under `spec/`, `crates/`, `xtask/`, `examples/`, and under `.kb/` outside `_intake/`. No `[FROZEN]` clause line-edited, no ADR written, no maturity marker moved, no atom hand-authored. | Four empty `diff --stat` assertions; `cargo xtask spec-trace` reports the identical census to `main` | This is the baseline the next two stories are judged against. It was the cheapest thing in the story to destroy and it survived. |

**Mount point:** `references/evaluation/README.md`, *Later additions, which are
neither* — one entry, beside `review-citation-drift.md`, carrying the same
immutable dated-and-pinned lifecycle. A document in that directory without a
README row is reachable only by `ls`, and the README claims to enumerate the
directory; the registration is the mount, not decoration.

**Deferred:** nothing. **Blocked:** nothing. The only handoff was the one the spec
requires — the KB-side amendments staged for `/redkiln:kb-ingest`, a human-invoked
wave, because `CLAUDE.md` reserves atom authorship to it. **That wave has since
run**: `2026-08-13-projection-adrs` at `493a194` consumed
`.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md` and amended
`ps-1-states-no-progress-obligation.md` and `ps-19-scope-narrower-than-its-rule.md`,
each keeping its prior body byte-for-byte under one appended dated section, each
keeping `status: accepted` and its owner. So the answer is now findable from
`.kb/` directly, and the README row has been updated to say so rather than to
say it is staged.

**One erratum, recorded rather than applied silently.** Slice review found the
census's **Src** column inconsistent exactly where the verdict turns: PS-26, PS-27,
PS-28 and PS-30 carried `T` — *"§4.11's table"* — for rules that live in §4.11's
six-rule **prose** list (`spec/SPECIFICATION.md:5694-5700`) rather than in its
seventeen-rule table (`:5658-5675`), while PS-29, whose rule sits in the same
paragraph, correctly carried `C X`. Since PS-29 is the third `independent` S1
defect, a reader re-deriving the verdict from that column alone would have fired
arm (b) and reached `systematic` where *The tally* derives `isolated`. The four
rows now read `C X`, PS-20 (which the table *does* name) now reads `C T X`, the
Src legend defines `T` as the seventeen-rule table specifically, and an **Erratum
2026-08-13** section at the foot of the document records all of it. **No verdict,
tally, shape or strength changed** — still 37 rows, 29/7/1, three `independent` S1
defects, two inside the table, still **ISOLATED**. The document's *immutable,
superseded rather than edited* lifecycle is why the correction is written down in
the document instead of being made quietly.

## Acceptance

| AC | Status | Verification as run |
| --- | --- | --- |
| AC-001 | satisfied | Static: 37 rows, 37 unique ids, zero verdict cells outside `sound`/`defective`/`undetermined`. Red: 0 rows. |
| AC-002 | satisfied | Method Steps 0–4 present; classifier quoted verbatim from `.kb/decisions/README.md:20-22` and the playbook `:56-72`; all three attribution sources named and the union swept; every row carries its `Src`. Second-reader re-derivation is the `_review.md` half. |
| AC-003 | satisfied | Static: zero `defective` rows with an empty or `TBD` exposing-implementation cell; each names a buildable shape and grades it. PS-28 recorded `undetermined` rather than forced to a binary. |
| AC-004 | satisfied | PS-1 and PS-19 rows cite `spec/SPECIFICATION.md:4733-4759` and `:5218-5223`; the atoms appear only as priors under test. Non-reproduction of a supporting sentence stated as the headline. |
| AC-005 | satisfied | Threshold section precedes tally section in file order; two arms with reasoning; shapes S1/S2/S3 distinguished and explicitly non-aggregating; verdict `ISOLATED`; no re-plan raised. |
| AC-006 | satisfied | Static: every `defective` row's owner is drawn from {ADR-0017, ADR-0018, ADR-0019, new decision}. Adjacent traps routed and recorded as not-settled in the intake file §4. |
| AC-007 | satisfied | `references/evaluation/README.md` entry inside *Later additions*, with date `2026-08-13`, pin `2136dde` (resolves), and the supersede-not-edit lifecycle; the document header repeats both. |
| AC-008 | satisfied | Four empty diff assertions; the staged intake file present; `spec-trace`, `validate --kb`, `validate`, `doctor` (the six expected template-drift advisories, unchanged), `fmt --check`, `affected`, `ci --fast` and the full `ci` all green. |

## Knowledge Harvest

Candidates for `.kb/` at closeout. **None was promoted by this story** — the
KB-side material it produced went to `.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md`
for the ingest wave, which `CLAUDE.md` reserves atom authorship to; that wave ran
at `493a194` and took the two open-question amendments, leaving the candidates
below for closeout to adjudicate.

- **A `reference` pointer atom for this sweep**, if the closeout wave wants one —
  carrying the verdict, the threshold and the tally, not the census. Exactly the
  relationship `kb-reference-phase-4-5-spec-reconciliation-001` has to
  `references/evaluation/phase-4-5-reconciliation.md`. Proposed conditionally in
  the intake file §1d, not asserted.
- **A `playbook` candidate: sweep the neighbourhood before scoping the repair.**
  Two atoms carried the same unexecuted instruction for three days; executing it
  changed the ADR scope (a third defect found, one prior sentence refuted) and cost
  one story. The transferable rule is that a defect found twice is a hypothesis,
  and a hypothesis is confirmed by the *third* case, never by the two that raised
  it — which is arm (b) of this sweep's threshold, generalised.
- **A `concept` candidate: what a rule assigned to a clause actually claims.**
  Step 0 (name what the `MUST` binds) and Step 2 (a borrowed instrument is sound
  only if the clause says so) are reusable over the `CF`, `ES` and `SY` families,
  none of which was swept. Whether they should be is itself a candidate question.
- **A candidate open question, named and not opened:** three clauses carry a
  documentation obligation with no instrument (PS-3, PS-31, PS-36). Recorded in the
  intake file §4 so the wave can see it and still decline.
