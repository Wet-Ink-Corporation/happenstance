---
item: "HS-S0058"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — ADR-0023 accepted, and CF-40 and WF-11 resolved rather than deleted

## Findings Ledger

**Outcome: twelve of twelve ACs satisfied, in two passes with a human between
them. Nothing faked, nothing stubbed, and no atom hand-written into
`.kb/decisions/` to make the story look finished.**

This story was authored in two halves on purpose, because its defining mechanism is
a human's command. `/redkiln:kb-ingest` carries `disable-model-invocation: true`,
runs Stage A inline to create **its own** isolated worktree, gates on a human before
the workflow writes anything, and is merged by a human afterwards. Everything
assigned to the implementer went in at `d3030c6`; everything assigned to the wave
arrived at `6b0fe33` and merged at `3ac4bf1`. Running the `kb-ingest` workflow
directly from this slice was available and was **refused**: it would have run in the
wrong tree, on the initiative branch, with no human at either gate, and with this
implementer taking EC-002's approval-gate refusals unilaterally.

**The wave has now run, and this report is the post-wave pass.** The eleven rows
that read *BLOCKED — the missing dependency is the wave* were reconciled against the
tree the wave produced: nine of them were materially satisfiable the moment `3ac4bf1`
merged, and two — AC-011 and AC-012 — needed work that could only happen afterwards
and is done here. A ledger that stays red for reasons that have stopped holding is a
worse artefact than one that was never written, so the reconciliation is part of the
story rather than a follow-up.

### AC by AC

| AC | result | what proves it |
| --- | --- | --- |
| AC-001 — coordinate CF-40 **first** | **satisfied** | `coordination-note.md`, Part 1. **Branch B**, with `ls .kb/decisions/` and `rg -n "CF-40" .kb/decisions .kb/maps` output quoted, written before a single intake document |
| AC-002 — provenance: the wave, gated by a human | **satisfied** | the atom's only commit is `6b0fe33`, the wave's own; `d3030c6` wrote nothing under `.kb/decisions/`. Audit trail at `.kb/_governance/integration-waves/2026-08-20-intake-phase-9/`, four documents |
| AC-003 — one atom, on the decision map | **satisfied** | one path from `ls .kb/decisions/0023-*.md`; the row at `.kb/maps/decision-map.md:211`; the conjunction stated as one question at `:81` with the playbook's own exception cited; no second atom splits the pair |
| AC-004 — every rejected alternative with the reason it lost | **satisfied** | `references/adr/0023-…:153-160` (harness) and `:118-125` (mapping), short form in the atom at `:155-164`; the winning shape recorded as `every-rule-under-workerd`'s **finding** at `:150-151` |
| AC-005 — ADR-0001 cited, not lifted | **satisfied** | `git diff main -- .kb/decisions/0001-async-port-flavours.md` empty; the discharge-by-citation at atom `:131` and summary `:40-42`, saying why the runbook's wording was read and not obeyed |
| AC-006 — ADR-0009's ES-6 prediction judged as new content | **satisfied** | `git diff main -- .kb/decisions/0009-error-send-sync.md` empty; the verdict folded into ADR-0023 at `:106-112`, citing the four reconstruction tests. **ADR-0009's decision holds, and this adapter is the evidence for it rather than the exception to it** |
| AC-007 — CF-40 resolved without losing its body | **satisfied** | `status: accepted → superseded`, `related` extended, original body byte-identical with a dated section appended, index bullet annotated in place at `.kb/maps/open-questions-index.md:201-210`, nothing `git rm`ed. See *The append-not-modify shape* below |
| AC-008 — WF-11 resolved on either outcome | **satisfied** | the *did not fire* branch, on measurement: no ceiling within 2,169 pages, 36,604,834 bytes as the firing payload at the platform's documented ceiling, thirty-five times the declared 1 MiB. WF-11 stays `[PROVISIONAL]`; no wire-format change anywhere |
| AC-009 — the long-form record, new file only | **satisfied** | `git show --name-status d3030c6 -- references/adr/` is exactly one `A` row and zero `M` rows; the atom links the record from `source_paths` at `:66`; `cargo xtask spec-trace` green |
| AC-010 — a hygienic wave | **satisfied** | a seventh, distinctly-named wave directory with `main`'s intact; `ls .kb/_intake/` returns `README.md` and nothing else; all five staged documents ingested, including the fifth that predates this story |
| AC-011 — the runbook's queue row closes | **satisfied** | the queue row struck and rewritten in `RUNBOOK.md:295`'s ADR-0016 shape at `:304`, the phase 9 work box ticked at `:4401-4413` with the `vitest-pool-workers` parenthetical replaced by what landed, and no other `RUNBOOK.md` line touched by this story |
| AC-012 — post-wave verification | **satisfied** | all four commands re-run against the post-wave tree: `validate --kb` passed, `doctor` at exactly six advisories and no seventh, `spec-trace` green at 201 clauses and 401 citations, `affected --base main` green. `redkiln adopt --templates` never run |

### What was staged, and what the wave did with it

Four intake documents under `.kb/_intake/`, one per claim cluster: ADR-0023 proper;
the ES-6 verdict against ADR-0009's prediction; CF-40's resolution; WF-11's
resolution. Plus `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`
— new, eight sections — and `coordination-note.md`'s Part 2, which carried the wave
id, the intake set including the fifth already-tracked file, the two refusals EC-002
reserves for the gate, the post-wave verification list, and the two `RUNBOOK.md`
edits that belonged after the wave rather than before it.

The wave took all five, plus `0034-what-the-phase-8-reconciliation-cost.md`, and
produced two decision atoms rather than four: `kb-decision-0023`, which **merged the
ES-6 verdict into ADR-0023** as the coordination note said it should, and
`kb-decision-0034`, which answers CF-40 from outside rather than by amending any of
the three accepted decisions that argue about it. Both refusals the gate was asked
to be ready for were made: nothing was merged into ADR-0009's or ADR-0015's accepted
body, and `.kb/open-questions/es-6-names-an-unwritable-rule.md` was deliberately left
`accepted` rather than read as resolved.

### Findings a reviewer should read

**1 — the CF-40 answer got better because phase 9 happened.** The open question's
sub-question 2 asks whether the fixture contract has one owning document. Before
this project it was two claims arguing; now it is three decisions observed:
ADR-0015 minted CF-40 and hedged, ADR-0012 owns CF-39 by adjacency, ADR-0022
explicitly declined, and ADR-0023 is the third to amend the contract without owning
it — with `CloudflareFixture` the first fixture in the workspace to declare **all
three** numeric ceilings *and* claim `MID_BATCH_FAULT`. The pattern has now worked
three times, and what it costs is nameable: a reader locating a fixture-contract
clause reads the specification by subject, not one ADR. `kb-decision-0034` states
that as a position rather than an absence.

**2 — the long-form record carries a decision the gate must actually take.**
`cargo deny check bans` is **red**: `deny.toml` bans `async-trait` under ADR-0001 and
`worker` 0.8.5 depends on it unconditionally. §7 states both shapes — ratify a
`wrappers` entry with its reason written beside it, or refuse and accept a standing
red step — and says that leaving it undecided while the gate is red is not one of
them. The wave signed one of the three escalations and left this one open at
`kb-open-question-worker-async-trait-ban-001`, which is where a human overrules it in
one edit.

**3 — a slice-mate's finding is routed here rather than left in a report.**
`deferral-re-reads-and-es-32-verdict` found the deferred-clause equality statement
stale: the checker counts twelve `[DEFERRED]` clauses and names three the table does
not list, while the table names one the specification has demoted. That story
corrected the sentence and moved nothing; reconciling the table is a clause question,
and it is written into `RUNBOOK.md` itself so it survives without this report. **It
now lands on a queue row that says something**: before this pass, `RUNBOOK.md`'s
ADR-0023 row was still an open question, so a finding handed to *"ADR-0023's queue
row"* was handed to a row that had not been written.

### The append-not-modify shape, and a verification line that drifted

AC-007 and AC-008 verify with *"`git diff … shows frontmatter hunks only and zero
body hunks`"*. Run literally against either open question, that command reports body
hunks, and a later reviewer should not be misled by it. **What the wave did is
append, not modify.** In both files the original body is byte-identical — the 2026-08-10
text describing what was not known then — and a dated section is appended below it:
`## Resolved 2026-08-20 — status superseded; sub-question 3 moves out` on CF-40's,
and `## Answered 2026-08-20 — the condition is not constructible on this runtime` on
WF-11's. Both say so in their own first sentence (*"Everything above is the state of
knowledge on 2026-08-10 and is left exactly as it was written"*).

That is the shape `.kb/open-questions/README.md:42-45` requires — *"leave the body
describing what was not known at the time … do not rewrite a question into its own
answer"* — and it is what the criteria themselves ask for in prose: *"its body
byte-identical"*, *"the question is never rewritten into its own answer"*. So the
intent is met and the **verification wording** is the part that drifted: a
frontmatter-only diff cannot express an appended resolution without either editing
the body or splitting the answer into a file the question does not link. The
criterion text is not touched here — re-wording a verification line is the spec
path's job, and it is left for a later pass rather than silently reinterpreted.

### What happened next, concretely

1. A human ran `/redkiln:kb-ingest`. Stage A resolved the intake set, created the
   worktree, and gated.
2. At the gate: `.kb/_intake/README.md` dropped; wave id `2026-08-20-intake-phase-9`
   — the proposed id, dated the day it actually ran; both refusals held.
3. The wave committed at `6b0fe33` and was merged at `3ac4bf1`.
4. After the wave, in this pass: the queue row struck in `RUNBOOK.md:295`'s ADR-0016
   shape and phase 9's ADR-0023 work box ticked; the checks in
   `coordination-note.md`, Part 2 re-run; the eleven remaining ledger rows flipped
   with the wave's own evidence and no criterion text edited.

### What was not touched

No accepted atom's body moved — `git diff main` over `.kb/decisions/0001-async-port-flavours.md`
and `.kb/decisions/0009-error-send-sync.md` is empty, and `redkiln validate --kb`
enforces that against `HEAD`. No existing file under `references/adr/` was modified
by this story. No `crates/**`, no `spec/**`, no clause text and no maturity marker.
No `pub` item changed anywhere. In `RUNBOOK.md`, exactly two sites: the ADR-0023
queue row and phase 9's ADR-0023 work box — the CF-14 and CF-27 cells and phase 9's
session log are slice-mate `deferral-re-reads-and-es-32-verdict`'s and are edited in
its own commit.
