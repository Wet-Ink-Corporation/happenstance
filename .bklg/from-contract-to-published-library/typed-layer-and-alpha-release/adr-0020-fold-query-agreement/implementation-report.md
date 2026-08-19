---
item: "HS-S0018"
stage: implement
created: "2026-08-15"
updated: "2026-08-15"
---

# Implementation Report — ADR-0020 — fold/query agreement, and DT-2's signature answer

> **STATUS: seven of eight.** AC-001 … AC-007 are satisfied by the two artefacts
> this story delivers. **AC-008 is not**, and cannot be from inside this PR: it
> asserts an accepted `.kb/decisions/0020-fold-query-agreement.md`, which only a
> **human-invoked `/redkiln:kb-ingest` wave** may author, on its own worktree
> branch. Hand-authoring the atom is the anti-pattern reverted at `0269720`. The
> spec says exactly this and calls leaving the row `satisfied: false` *"the correct
> outcome, not a blocker to work around"* (`spec.md`, *Clarifications resolved
> during spec*, item 2).

**No Rust was written, and that is the deliverable.** M1 exists so that the record
governing `DecisionModel` exists *before* the code it governs — project AC-016's
"written first" — and an accepted atom is immutable, so an ADR written afterwards
records what was built instead of deciding it.

## TDD Evidence

The deliverable is two markdown artefacts, so the tests that encode the ACs are the
existence-and-reachability checks the spec's merge-gate table names, plus the
mechanical negative that NF-001 and AC-004 turn on. Each was run **before** the
change and observed to fail for the right reason — the artefact absent, not a typo
or a broken command — and again after.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `test -f .kb/_intake/0020-fold-query-agreement.md` | **Red** `FAIL (missing)` — the mount point did not exist. **Green** `PASS` at 199 lines, composed as a decision record with the proposed frontmatter at `:26-86` |
| AC-001 | `cargo xtask affected --base main` (`.redkiln/config.yaml`, `affected_gate`) | **Green** — `affected gate passed`. **A correction to what the spec expects of this step:** `--base main` diffs the *whole initiative branch* (923 files, seven packages named), not this story's checkpoint, so it is not the "maps to no package" proof the spec's gate table predicts. It is still the configured story grain and it is green; the story-scoped negative is the `git diff --name-only` row below |
| AC-002, AC-003, AC-005, AC-006 | Side-by-side review against `_design.md` `## Shape decision` / `## Signatures` / `## Sign-off`, and re-opening every `crates/**` range cited | **Red** — no record existed to review. **Green** — five decision claims matching the design row for row, an eight-row rejected table, and two citation repairs recorded rather than silently applied |
| AC-004 | `git diff --name-only` shows no path under `crates/`, `examples/`, `xtask/`, `spec/` | **Green**, and this is the only *mechanical* enforcement available for the "no `unwrap`, no core edit" half of AC-004. It was vacuously green before the change and is meaningfully green after |
| AC-007 | `test -f references/adr/0020-fold-query-agreement.md` | **Red** `FAIL (missing)`. **Green** `PASS` at 411 lines |
| AC-007 | proposed `source_paths` resolves, and lists both artefacts | **Green** — six entries at `.kb/_intake/0020-fold-query-agreement.md:76-83`, each re-checked to exist, matching the pairing `.kb/decisions/0029-msrv-raised-to-1-97-1.md:45-47` already carries |
| — | `redkiln validate --kb` | **Green** before and after: `redkiln: validate passed.` The corpus is clean and this PR added **no** atom, which is the state AC-008 measures the wave against. `_intake` is skipped by design, so this command says nothing about the staged document itself |
| AC-008 | `git log --diff-filter=A -- .kb/decisions/0020-fold-query-agreement.md` | **Red, and still red.** No such path. Discharged by the wave, not by this PR |

**What a green gate proves here, and what it does not.** Nothing compiled changed,
so every compiled step is a *negative* proof: no clause moved, no citation rotted,
no package was touched. The substance of the record — that it states one shape,
names what lost, and puts the fallibility where the design put it — has no compiled
assertion and never will. The testing brief says so for this whole class: *"Record,
not a test."*

## Commits

One story checkpoint. Its own sha cannot be written into a file the commit
contains; it is recorded on the item through `redkiln record-links`.

| SHA | Subject |
| --- | ------- |
| *on the item's `links.commits`* | `feat(typed-layer-and-alpha-release): ADR-0020: fold/query agreement` |
| *pending* | the `/redkiln:kb-ingest` wave that mints `.kb/decisions/0020-fold-query-agreement.md` — AC-008's evidence, and a human's to run |

## Changes

| Path | Shape of the change |
| --- | --- |
| `references/adr/0020-fold-query-agreement.md` | **New, 411 lines.** The long-form record: `RUNBOOK.md:300`'s question quoted and answered in one sentence; the hazard quoted from the worked example on both sides with a table of the two divergence directions; the four frozen constructors the answer is built out of; the audience constraint and DT-2's concrete form; five numbered decisions; where the fallibility went, in four parts, ending in defect candidate D-1 with `VT-18` as its nearest clause subject and three "fixes" refused by name; an eight-row rejected table whose second column is the wrong implementation each admits; consequences carrying the 2.4:1 measurement as a falsifiable prediction; what the decision does **not** decide; and a citation-repair record |
| `.kb/_intake/0020-fold-query-agreement.md` | **New, 199 lines, staged.** The wave's input: the op, a proposed frontmatter block (`adr_id: ADR-0020`, `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 7`, `reversibility: medium`, `depends_on`/`related` naming four real atom ids, six resolving `source_paths`), the hazard, the five claims, the residual and D-1, the eight rejected shapes, the consequences, the repaired citations, the map rows, and an explicit *Not proposed* list. Carries the one-wave and wave-id-suffix instructions, and the `README.md`-in-the-glob warning (EC-002) |
| `.bklg/.../adr-0020-fold-query-agreement/_ledger.md` | Seven rows flipped `false → true` with cited evidence. **AC-008 left `false`.** No criterion, `mount_point` or `verifying_test` touched |
| `.bklg/.../adr-0020-fold-query-agreement/implementation-report.md`, `report.md` | **New.** This report and the story's findings ledger |

**Not touched, and asserted by the gate:** `crates/**`, `examples/**`, `xtask/**`,
`spec/SPECIFICATION.md`, `RUNBOOK.md`, `.kb/decisions/**`, `.kb/maps/**`,
`.kb/open-questions/**`.

## Gates

| Command | Result |
| --- | --- |
| `test -f` × 2 (red baseline) | both `FAIL (missing)` before the change |
| `test -f` × 2 (after) | both `PASS` |
| `cargo xtask affected --base main` | **passed** — `affected gate passed`. `--base main` names seven packages because the branch has 923 files changed against `main`, not because this story touched code |
| `redkiln validate --kb` | **passed** — `redkiln: validate passed.` |
| `redkiln doctor` | unchanged: the **six** expected `template-drift` advisories and the pre-existing foundation-story advisories, none of them this story's |
| `git diff --name-only` | only `.kb/_intake/0020-…`, `references/adr/0020-…` and this story's own `.bklg` folder |
| `cargo xtask ci --fast` | run once for the slice, after the slice-mate — see that story's report; green and unchanged by construction, because nothing compiled moved (NF-001) |

## Notes

**Two citations in the inputs did not say what they were cited for, and both were
repaired rather than quietly used.** This is EC-008's required handling and the
precedent `_design.md` `## Sign-off` set on itself.

1. `crates/happenstance-core/src/projection.rs:47-61` is cited by
   `_decomposition.md:544-546` and by `_design.md` `## Shape decision` as the source
   of *"two constructors enforcing different rules is the defect that makes an
   invalid value reachable through the weaker one."* That range is the module's
   *"The invariant that drives the design"* prose about read-model/checkpoint
   atomicity. The sentence is at `projection.rs:152-154`, in `ProjectionId::new`'s
   doc comment. The record cites the corrected range and says so.
2. `RUNBOOK.md:524` is cited for AC-013's falsifiable prediction; `:524` is the
   testkit-instrument row about a store holding only a suffix of its own log. The
   `happenstance-macros` row is `:525`.

Both are **repairs** by `.kb/decisions/README.md`'s mechanical test — the set of
implementations admitted is unchanged — so neither is an amendment and neither
needed a decision of its own.

**One input claim was checked and found imprecise, and the record does not repeat
it.** `_design.md` and this story's own spec describe `EventStore` as offering
*"`read`, `append`, `head` and `read_decision_model`"*. The trait
(`crates/happenstance-core/src/store.rs:93-269`) has four required methods —
`read` `:119`, `append` `:213`, `head` `:248`, `contains_event_id` `:268` — and
`read_decision_model` is a **free function** at `:321`, not a trait method. It does
not change any conclusion here (this record makes no claim about the read surface),
and it is noted because the slice-mate's AC-005 turns on that surface having **no
hook**, which is true of both spellings.

**Deviation from the spec's implementation notes: none of substance.** The long
record was written first and the staged document distilled from it, as advised, and
the `0029` pair was used for the frontmatter fields and `summary` density.

**A judgement call worth flagging to review.** The staged document proposes
`kb-open-question-projection-id-unvalidated-001` as a `related` edge, because the
sentence the one-shape claim rests on lives in the doc comment that open question
owns. It is cited only, not resolved, edited or deleted — but if the wave judges
the edge to imply otherwise, drop it: nothing in AC-005 depends on it, and the
quotation stands on the `file:line` alone.
