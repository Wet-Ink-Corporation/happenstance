---
item: "HS-S0058"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — ADR-0023 accepted, and CF-40 and WF-11 resolved rather than deleted

## Findings Ledger

**Outcome: BLOCKED. One of twelve ACs satisfied. Nothing faked, nothing stubbed,
and no atom hand-written into `.kb/decisions/` to make the story look finished.**

The blocker is the story's own defining mechanism. `/redkiln:kb-ingest` carries
`disable-model-invocation: true`, runs Stage A inline to create **its own** isolated
worktree, gates on a human before the workflow writes anything, and is merged by a
human afterwards. This story's spec says the same thing in its own words: *"the
ingest is a human's command, so this story is a staged handoff plus a verification,
not a write."* Everything assigned to the implementer is on disk; everything
assigned to the wave is not.

Running the `kb-ingest` workflow directly from this slice was available and was
**refused**: it would have run in the wrong tree, on the initiative branch, with no
human at either gate, and with this implementer taking EC-002's approval-gate
refusals unilaterally. AC-002's text is *gated by a human*, not *produced by a
wave*.

### AC by AC

| AC | result | what proves it, or what is missing |
| --- | --- | --- |
| AC-001 — coordinate CF-40 **first** | **satisfied** | `coordination-note.md`, Part 1. **Branch B**, with `ls .kb/decisions/` and `rg -n "CF-40" .kb/decisions .kb/maps` output quoted. `sqlite-durable-store`'s own ADR-0022 declined to mint it and said so twice (`:30`, `:93-94`), pointing at `kb-open-question-cf-40-ownership-001`; the index still carries the bullet as Open. One minting across both projects, and `git diff --name-only main -- .kb/decisions/` is empty for this story. |
| AC-002 — provenance: the wave, gated by a human | **blocked** | the wave. Four intake documents, the action plan, the wave id and the two refusals are staged and ready; the run is a human's. |
| AC-003 — one atom, on the decision map | **blocked** | the wave. Content staged, including the *one question, two consequences* paragraph and its citation of the playbook's own stated exception. |
| AC-004 — every rejected alternative with the reason it lost | **blocked** (atom half) | authored in full at `references/adr/0023-…` §2.5 and §3.3 and staged for the atom; the winning harness shape is recorded as `every-rule-under-workerd`'s finding rather than chosen here. |
| AC-005 — ADR-0001 cited, not lifted | **blocked** (atom half) | `references/adr/0023-…` §4 carries the discharge **and why the runbook's wording was read and not obeyed**. `git diff main -- .kb/decisions/0001-async-port-flavours.md` already produces no output. |
| AC-006 — ADR-0009's ES-6 prediction judged as new content | **blocked** (atom half) | `references/adr/0023-…` §5 and `.kb/_intake/es-6-verdict-…`. The verdict: **ADR-0009's decision holds, and this adapter is the evidence for it.** `git diff main -- .kb/decisions/0009-error-send-sync.md` already produces no output. |
| AC-007 — CF-40 resolved without losing its body | **blocked** | the wave owns the `status`/`related` flip and the index annotation. The answering content is staged and states explicitly that the body stays byte-identical and the file is never `git rm`ed. |
| AC-008 — WF-11 resolved on either outcome | **blocked** | staged with the *did not fire* branch and the measured numbers: verdict (c), no ceiling within 2,169 pages, 36,604,834 bytes as the firing payload at the platform's documented ceiling. No wire-format change anywhere in the diff. |
| AC-009 — the long-form record, new file only | **partly satisfied** | the record is on disk, is the only untracked addition under `references/adr/`, modifies no existing file there, and `cargo xtask spec-trace` is green. The half that is missing is this AC's own review gate: *the atom links the record*. |
| AC-010 — a hygienic wave | **blocked** | wave id proposed and suffixed (`2026-08-19-intake-phase-9`); the `README.md` drop specified; the fifth already-tracked intake file named so the gate expects it. |
| AC-011 — the runbook's queue row closes | **blocked, deliberately** | striking `RUNBOOK.md:302` before the atom exists would point at a path that does not resolve and claim an acceptance that has not happened. The edit and its exact shape are specified for the post-wave pass. |
| AC-012 — post-wave verification | **blocked** | three of the four commands are green now — `validate --kb` passed, `doctor` reports exactly six advisories and no seventh, `spec-trace` and `affected` green, and `adopt --templates` was never run — but this AC's WHEN is *when the wave is complete*. |

### What is staged, and what the human gate is being asked to approve

Four intake documents under `.kb/_intake/`, one per claim cluster: ADR-0023 proper;
the ES-6 verdict against ADR-0009's prediction; CF-40's resolution; WF-11's
resolution. Plus `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`
— new, `Status: proposed`, eight sections — and `coordination-note.md`'s Part 2,
which carries the wave id, the intake set including the fifth already-tracked file,
the two refusals EC-002 reserves for the gate, the post-wave verification list, and
the two `RUNBOOK.md` edits that belong after the wave rather than before it.

### Findings a reviewer should read

**1 — the CF-40 answer got better because phase 9 happened.** The open question's
sub-question 2 asks whether the fixture contract has one owning document. Before
this project it was two claims arguing; now it is three decisions observed:
ADR-0015 minted CF-40 and hedged, ADR-0012 owns CF-39 by adjacency, ADR-0022
explicitly declined, and ADR-0023 is the third to amend the contract without owning
it — with `CloudflareFixture` the first fixture in the workspace to declare **all
three** numeric ceilings *and* claim `MID_BATCH_FAULT`. The pattern has now worked
three times, and what it costs is nameable: a reader locating a fixture-contract
clause reads the specification by subject, not one ADR.

**2 — the long-form record carries a decision the gate must actually take.**
`cargo deny check bans` is **red**: `deny.toml` bans `async-trait` under ADR-0001 and
`worker` 0.8.5 depends on it unconditionally. §7 states both shapes — ratify a
`wrappers` entry with its reason written beside it, or refuse and accept a standing
red step — and says that leaving it undecided while the gate is red is not one of
them. This is the finding `worker-binding-layer` escalated against its AC-008, and
ADR-0023 is where it lands.

**3 — a slice-mate's finding is routed here rather than left in a report.**
`deferral-re-reads-and-es-32-verdict` found the deferred-clause equality statement
stale: the checker counts twelve `[DEFERRED]` clauses and names three the table does
not list, while the table names one the specification has demoted. That story
corrected the sentence and moved nothing; reconciling the table is a clause question
for ADR-0023's queue row and the projection passes, and it is written into
`RUNBOOK.md` itself so it survives without this report.

### What happens next, concretely

1. A human runs `/redkiln:kb-ingest` with no argument. Stage A resolves the intake
   set, creates the worktree, and gates.
2. At the gate: drop `.kb/_intake/README.md`; use the wave id
   `2026-08-19-intake-phase-9`; be ready to refuse a merge into ADR-0009's or
   ADR-0015's accepted body, and to refuse reading the ES-6 cluster as resolving
   `.kb/open-questions/es-6-names-an-unwritable-rule.md`.
3. After the wave: run the seven checks in `coordination-note.md`, Part 2; strike
   `RUNBOOK.md:302` in `RUNBOOK.md:295`'s ADR-0016 shape and tick phase 9's
   ADR-0023 work box; flip the remaining eleven ledger rows with the wave's own
   evidence.

### What was not touched

`.kb/decisions/`, `.kb/open-questions/`, `.kb/maps/`, `.kb/_governance/` — nothing.
`crates/**`, `spec/**`, `RUNBOOK.md` — nothing. No accepted atom's body moved, no
existing file under `references/adr/` was modified, no `pub` item changed anywhere.
