---
item: "HS-S0058"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — ADR-0023 accepted, and CF-40 and WF-11 resolved rather than deleted

> **This story is BLOCKED, and the blocker is the story's own defining mechanism
> rather than an obstacle around it.** One of twelve ACs is satisfied. Everything
> the spec assigns to *the implementer* is on disk; everything it assigns to *the
> wave* is not, because `/redkiln:kb-ingest` carries
> `disable-model-invocation: true`, creates its **own** dedicated worktree, and
> gates on a human before the workflow writes a byte. No atom was hand-written to
> get around it — which is the one thing this story exists to not do.

## What is blocked, exactly

`/redkiln:kb-ingest`'s command definition
(`~/.claude/plugins/marketplaces/redkiln-local/commands/kb-ingest.md`) is
unambiguous on all three points that matter here:

```
model: sonnet
effort: high
disable-model-invocation: true
```

> You run **Stage A inline** (resolve the intake set → create an isolated worktree →
> human gate), then launch the **`kb-ingest`** workflow once… The workflow works
> **entirely inside the worktree**… and **commits the whole wave on the worktree
> branch**. Your oversight is **reviewing and merging that branch**.

So the wave is (a) not model-invocable, (b) run in a worktree the command creates,
not this story's, and (c) gated by a human twice — once before the workflow and
once at the merge. The story's own spec says the same thing in its Context pack
decision 2: *"the ingest is a human's command, so this story is a staged handoff
plus a verification, not a write."*

Running the `kb-ingest` workflow directly from here was available and was
**refused**. It would have produced atoms through a real wave, but in the wrong
tree, on the initiative branch, with no human at either gate — and with this
implementer making the two refusals EC-002 reserves for the approval gate
unilaterally. AC-002's text is *"gated by a human"*, not *"produced by a wave"*.

## What is on disk

| artefact | what it is |
| --- | --- |
| `coordination-note.md`, Part 1 | AC-001's artefact: the CF-40 branch determination with the `ls` and `rg` output quoted, run at implement time |
| `coordination-note.md`, Part 2 | the proposed action plan the human gate is asked to approve: the intake set, the wave id, the two refusals, the post-wave verification list, and the two `RUNBOOK.md` edits that belong after the wave |
| `.kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` | claim cluster 1 — ADR-0023 proper |
| `.kb/_intake/es-6-verdict-against-adr-0009s-prediction.md` | claim cluster 2 — the ES-6 verdict against ADR-0009's prediction |
| `.kb/_intake/cf-40-fixture-contract-ownership-resolution.md` | claim cluster 3 — CF-40's resolution |
| `.kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md` | claim cluster 4 — WF-11's resolution |
| `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` | the long-form record, **new**, `Status: proposed` |

## The CF-40 coordination result

**Branch B.** No resolution exists, so this story stages one and HS-P0012 cites it.

The decisive evidence is that `sqlite-durable-store`'s own decision **declined** to
mint it and said so twice: `.kb/decisions/0022-append-condition-strategy.md:93-94`
records CF-40's clause home as a non-verdict with a named owner, pointing at
`kb-open-question-cf-40-ownership-001`, and the frontmatter summary repeats it at
`:30`. `ls .kb/decisions/` returns no `0023-*`;
`.kb/maps/open-questions-index.md:171-173` still carries the bullet as **Open**.

What this project newly knows is the useful part. Sub-question 2 asks whether the
fixture contract has one owning document or is amended piecemeal. Phase 9 supplies
the **third** data point rather than a second argument: `CloudflareFixture` is the
first fixture in the workspace to declare all three of CF-40's numeric ceilings
*and* claim CF-39's `MID_BATCH_FAULT`, and before it every fixture in the tree left
all three at `None`, so `append_reports_exceeded_store_limits` reported a skip
everywhere and certified nothing. Three ADRs have now amended the fixture contract,
none has claimed it, and none has collided.

## TDD Evidence

This story lands **no Rust**, so the instruments are the file-reading ones, and each
was run at implement time rather than assumed.

| AC | instrument | red (before) | green (after) |
| --- | --- | --- | --- |
| AC-001 | `ls .kb/decisions/`; `rg -n "CF-40" .kb/decisions .kb/maps` | no coordination note existed and the branch was undetermined; a second minting was possible | Branch B recorded in a committed note with the command output quoted |
| AC-009 (partly) | `git status --short -- references/adr/`; `cargo xtask spec-trace` | `references/adr/0023-*` absent | exactly one untracked addition, no existing file modified, spec-trace green |
| AC-012 (three of four commands) | `redkiln validate --kb`; `redkiln doctor`; `cargo xtask spec-trace`; `cargo xtask affected --base main` | — | `validate passed`; **exactly six** `template-drift` advisories and no seventh; spec-trace green; **`affected gate passed`** |
| AC-002, AC-003, AC-004, AC-005, AC-006, AC-007, AC-008, AC-010, AC-011, AC-012 | the wave | — | **blocked**; content staged, nothing faked |

**The red that matters most is one that was deliberately not turned green.**
`.kb/decisions/0023-*.md` does not exist. Writing it by hand would satisfy
`redkiln validate --kb` perfectly — validation checks conformance and immutability,
never provenance — and would be missing the adjudication, the map sync and the
wave's audit trail. That is the failure `CLAUDE.md` records as reverted at
`0269720`, and it is the single easiest way to make this story look finished.

## Changes

| file | shape of the change |
| --- | --- |
| `.kb/_intake/*.md` (four new) | staged raw material, one per claim cluster. Not atoms, not held to `KbFrontmatter`, and each says so in its first line. Each names its intended layer and the merges it must **not** be adjudicated into. |
| `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` | **new**. Eight sections: the conjunction and its playbook exception; the mapping's four properties as the real bindings left them; the harness and the three shapes that lost; ADR-0001 cited not lifted; ADR-0009's ES-6 prediction judged; the `workerd` blocking finding; the dependency reversal with its measured cost **and the `cargo deny check bans` price it has not paid**; and what it does not touch. |
| `…/adr-0023-and-atom-resolutions/coordination-note.md` | AC-001's artefact plus the action plan for the gate. |
| `…/adr-0023-and-atom-resolutions/_ledger.md` | AC-001 flipped with cited evidence; the other eleven left `satisfied: false` with the exact missing dependency written into each `evidence` field. |
| this report and `report.md` | the stage artefacts. |

**No path under `.kb/decisions/`, `.kb/open-questions/`, `.kb/maps/` or
`.kb/_governance/` is touched. No `crates/**`. No `spec/**`. No `RUNBOOK.md`.**

## Gates

| command | result |
| --- | --- |
| `redkiln validate --kb` | `redkiln: validate passed.` |
| `redkiln doctor` | exactly **six** `template-drift` advisories — `discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md`, `_design.md`, `_intake-brief.md` — and no seventh. `redkiln adopt --templates` was never run. |
| `cargo xtask spec-trace` | green — *"traceability: no problems found; §7.1–§7.2 matches the checker"* |
| `cargo xtask affected --base main` | **`affected gate passed`** |
| `git diff --name-only -- .kb/decisions/ .kb/open-questions/ .kb/maps/ crates/ spec/ RUNBOOK.md` | empty for this story's commit |

## Notes

**Why `RUNBOOK.md:302` was not struck through.** AC-011 asks for the queue row to be
struck and rewritten *pointing at the atom by path*. The atom does not exist, so the
path would not resolve and the row would claim an acceptance that has not happened
— the same inversion phase 9's own proof-artefact section was rewritten to prevent,
where a box is ticked because the work feels done rather than because the artefact
is there. The edit and the exact shape it must take are specified at the end of the
coordination note so the post-wave pass is a transcription rather than a
re-derivation.

**Why the long-form record says `Status: proposed`.** Every other file under
`references/adr/` says `accepted`, and this one may not until the atom exists. It is
staged now rather than after the wave for the reason the spec gives — the atom
summarises the record, so the record has to be there for the atom to link — and the
status line is what stops a reader taking a staged record for a settled decision.

**The wave will pick up a fifth file, and that is correct.**
`.kb/_intake/0034-what-the-phase-8-reconciliation-cost.md` has been tracked since
`4ad58d0` and is still in `_intake`, which by that directory's own contract means no
wave has ingested it. It is named in the action plan so the gate expects it.

**One finding handed to this story by a slice-mate, recorded so it reaches the ADR
queue.** `deferral-re-reads-and-es-32-verdict` found `RUNBOOK.md`'s deferred-clause
equality statement stale on arrival: `spec-trace` counts **twelve** `[DEFERRED]`
clauses and names PS-18, PS-27 and PS-30, which the table does not list, while the
table names PS-33, which §7.2 now carries as `NON-NORMATIVE`. That story corrected
the *sentence* and moved no marker, row or census figure; reconciling the table is a
clause question and belongs to ADR-0023's queue row and to whichever pass owns the
projection deferrals. It is written into `RUNBOOK.md` itself so it cannot be lost
with a report.

**What a reviewer should check first.** That nothing under `.kb/decisions/` moved.
The whole story is the discipline rather than the prose, and every one of its writes
has an easier version that passes every automated check in this repository and
destroys the record.

## Commits

One checkpoint on `initiative/from-contract-to-published-library`, carrying the
staged handoff and the long-form record. The post-wave half — the atoms, the map
rows, the two open-question flips, the `RUNBOOK.md` queue row and the verification —
is a second pass that begins with a human running `/redkiln:kb-ingest`.

| SHA | subject |
| --- | --- |
| `PENDING` | `feat(cloudflare-durable-object-store): stage ADR-0023 and the CF-40/WF-11 resolutions for the ingest wave` |
