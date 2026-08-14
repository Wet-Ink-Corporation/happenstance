---
item: "HS-S0001"
stage: implement
created: "2026-08-13"
updated: "2026-08-13"
---

# Implementation Report — Sweep PS-1 – PS-37 for the pairing defect before any repair is scoped

## TDD Evidence

This story executes no library code, so no `#[test]` can observe it. The Testing
brief types project AC-008 as **Static** — *"a process gate on commit sequence as
much as a schema check"* — and the spec's *Tests and CI* table names the static
commands directly. Inventing a `#[test]` here would be exactly the decorative rule
`CLAUDE.md` forbids, so the red/green instrument is the static check set the spec
enumerates, run before the artefacts existed and again after.

The check set was written first, as one script over the four static ACs, and run
against an empty branch. Eight checks failed, each asserting a missing artefact
rather than a typo — no census document, no complete census to read, no README
registration, no resolvable pin, no staged intake file. The three census-content
checks (AC-001 verdict vocabulary, AC-003 exposing implementation, AC-006 owner)
are deliberately gated on `rows == 37` so that a missing census fails them instead
of passing them vacuously; that gate was added after the first red run showed them
reporting `PASS (got 0)` against nothing.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `grep -c '^| PS-' references/evaluation/ps-clause-pairing-sweep.md` = 37; unique `PS-\d+` ids = 37; every verdict cell ∈ {`sound`,`defective`,`undetermined`} | **red** `FAIL census rows (want 37, got 0)`, `FAIL distinct clause ids (want 37, got 0)`, `FAIL verdicts inside the closed vocabulary (no complete census to read)` → **green** `PASS (got 37)`, `PASS (got 37)`, `PASS (got 0)` bad cells. Tally: 7 `defective`, 29 `sound`, 1 `undetermined`. |
| AC-002 | Review of the Method section against `.kb/decisions/README.md:20-22` and `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:56-72`; every census row carries a `Src` field | **red** section absent → **green** Steps 0–4 present, classifier quoted verbatim, all 37 rows carry `Src`. Re-derivation by a second reader is the `_review.md` half. |
| AC-003 | No `defective` row with an empty / `TBD` exposing-implementation cell | **red** `FAIL (no complete census to read)` → **green** `PASS (got 0)`. Seven defective rows, each naming a buildable store or runner and grading it *plausible first cut*; PS-28 recorded `undetermined` rather than forced. |
| AC-004 | PS-1 and PS-19 rows cite `spec/SPECIFICATION.md:4733-4759` / `:5218-5223`, not the atoms | **red** rows absent → **green** both derived from clause text, and the headline records that one *supporting* sentence of the PS-1 atom does not survive re-derivation. |
| AC-005 | Document order: the threshold section precedes the tally section | **red** neither section existed → **green** *The threshold, declared before the count* precedes *The tally, against the threshold*; verdict `ISOLATED` against a pre-declared two-arm threshold. |
| AC-006 | Every `defective` row's owner cell ∈ {ADR-0017, ADR-0018, ADR-0019, new decision, routed} | **red** `FAIL (no complete census to read)` → **green** `PASS (got 0)` rows outside the vocabulary. |
| AC-007 | `rg 'ps-clause-pairing-sweep' references/evaluation/README.md` inside *Later additions*; the pinned sha resolves | **red** `FAIL README registration present`, `FAIL pinned commit resolves (got 'none')` → **green** `PASS`, `PASS pinned commit 2136dde resolves`. |
| AC-008 | Four empty-diff assertions + the staged intake file + four gate commands | **red** `FAIL KB amendment staged in _intake` (the four diff assertions pass by construction on a branch with no Rust change, and are non-regression guards rather than red-then-green evidence — stated here rather than dressed up) → **green** `PASS`, with `spec-trace`, `validate --kb`, `doctor`, `fmt --check`, `affected` and the full `ci` all green. |

Check script: `scratchpad/sweep-checks.sh` (kept out of the repository — the PR
boundary is four paths and a shell script is none of them). Its assertions are
reproduced verbatim in the table above and in the spec's *Tests and CI* table, so
nothing here depends on the script surviving.

## Commits

One checkpoint commit, no fixups. Its sha cannot be written into a file the
commit itself contains — this template says so in its own body — so it is
recorded on the item through `redkiln record-links` at the advance that files
this report, and returned in the slice digest. What is stable and citable here is
the subject and the shape:

| SHA | Subject |
| --- | ------- |
| *on the item's `links.commits`; see above* | `feat(projection-store-freeze): Sweep the PS clause range for the pairing defect` |

The commit carries the four paths listed under **Changes** and nothing else. Its
parent is `2136dde`, which is also the pin in the evidence document's header and
in the `references/evaluation/README.md` registration row — so the citations in
the document resolve against the tree the document was written from, which is the
whole of NF-001.

## Changes

Four paths, and not one byte outside them.

| Path | Shape of the change |
| --- | --- |
| `references/evaluation/ps-clause-pairing-sweep.md` | **New.** The evidence document: header with date `2026-08-13` and pin `2136dde`; a five-step method (name what the `MUST` binds → the classifier → borrowed instruments → strength → the exposing-implementation bar); a two-arm threshold declared before the count; the 37-row census; the tally and verdict; the headline finding; two cross-clause findings; five inverse-shape observations; three tooling observations; and what is staged for `.kb/`. |
| `references/evaluation/README.md` | **The mount.** One entry added to *Later additions, which are neither*, beside `review-citation-drift.md` and carrying the same lifecycle — dated, pinned, immutable, supersede rather than edit — plus the verdict and the statement that the KB side is staged, not written. Nothing else in the file changed. |
| `.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md` | **New, staged only.** Three AMENDs for the next `/redkiln:kb-ingest` wave — the two open-question atoms and the open-questions index — plus an explicit list of what it does *not* propose. It states its own adjudication bias (amend, never mint) and warns the wave off the two existing wave ids. |
| `.bklg/.../ps-clause-pairing-sweep/**` | `_ledger.md` (eight rows flipped `false` → `true` with evidence; no criterion touched), this report, and `report.md`. |

## Gates

Run from the worktree root at the commit above.

| Command | Result |
| --- | --- |
| `scratchpad/sweep-checks.sh` (the eight static AC assertions) | 14/14 `PASS`, exit 0 |
| `cargo xtask spec-trace` | exit 0 — 200 clauses (139 FROZEN, 49 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE), 95 rules, 58 cases, 358 citations; *traceability: no problems found; §7.1–§7.2 matches the checker*. Identical to `main`. |
| `redkiln validate --kb` | `validate passed` |
| `redkiln validate` | `validate passed` |
| `redkiln doctor` | exactly the six expected `template-drift` advisories, plus the nine pre-existing foundation-story-not-consumed advisories present on the base branch; exit status unchanged from `main` (`discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md`, `_design.md`, `_intake-brief.md`) |
| `cargo fmt --all --check` | exit 0 |
| `cargo xtask affected --base main` | *762 file(s) changed against `main`; no package affected — nothing to compile*; **affected gate passed** |
| `cargo xtask ci --fast` | *all required checks passed (--fast: 4 optional step(s) not run)* |
| `cargo xtask ci` | **all checks passed** — the merge bar the spec names for this row, run whole because the two steps `--fast` omits (`spec-trace` and the mandatory wasm32 conformance-harness check) are exactly the ones AC-008 leans on |

The affected-package gate is empty by design: the affected set for this story is
`[]`, because nothing under `crates/`, `xtask/` or `examples/` moved. That is
AC-008's assertion, not a gap in coverage — a non-empty affected set here would
mean the PR boundary had been crossed.

## Notes

**No re-plan is raised.** The escalation arm of the Architecture brief's Note 8
does not fire. The verdict is `isolated` against a threshold declared before the
count: three `independent` same-shape defects (threshold five), of which two fall
inside §4.11's seventeen-rule table (threshold three, and two is the prior itself).
Phase 6's budget — three ADRs plus one clause-disposition story — is the right size
for three point repairs and one recorded lesson.

**Three deviations from the spec's expectations, all in the direction of more
evidence rather than less.**

1. **A third independent defect was found, and it is outside §4.11's table.**
   PS-29 carries exactly PS-1's and PS-19's shape — a `[FROZEN]` sentence narrower
   than the rule written beside it — on `one_poisoned_projection_does_not_stall_the_others`,
   which exists only in the clause body. That sharpens the verdict rather than
   changing it: the cause is a habit of writing the rule to the clause's *intent*
   rather than to its *sentence*, distributed across the family, not a defect of one
   table's population. `projection-decision-atoms` consumes it as ADR-0019 scope.

2. **A supporting sentence of a prior does not survive re-derivation, and it was
   reported rather than agreed with.** `.kb/open-questions/ps-1-states-no-progress-obligation.md:48-51`
   says progress is *"stated by no clause's `MUST` anywhere in the document"*.
   PS-23's *"One `commit` advances exactly one `ProjectionId`"* states it — on the
   wrong clause (PS-23's own `Rejects` field shows the intent was *not more than
   one*) and on a `[PROVISIONAL]` one. The finding itself reproduces; what changes
   is the repair, which now has at least three candidate shapes instead of one
   foregone edit. Staged as an amendment to the atom's body, not a rewrite of it.

3. **Two shapes the classifier does not call defects were recorded anyway**, in
   their own sections, so a later reader does not mistake silence for absence:
   *inverse-shape* rows where the rule is **weaker** than the `MUST` (PS-3, PS-12,
   PS-15, PS-31, PS-36 — three of them the same documentation-obligation-with-no-
   instrument shape), and three *tooling observations* where `parse_clauses` lifts
   a non-rule identifier out of a `**Rule:**` field (`compile_fail` at PS-10,
   `probe_delete_all` at PS-16, `on_error` at PS-27). The tooling ones are reported
   and not repaired, per EC-001: §7.1/§7.2 are derived, so a mechanical mismatch is
   a tooling bug with an existing owner.

**Nothing was decided.** No `[FROZEN]` clause was line-edited, no ADR written, no
maturity marker moved, no `.kb/` atom hand-authored. The separation of discovery
from decision is the reason the next two stories have a baseline to be judged
against, and it is the one thing this story could have destroyed cheaply.
