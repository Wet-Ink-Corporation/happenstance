---
id: kb-decision-0088
title: GitHub Issues is the tracker, and the runbook is the plan of record
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0088
reversibility: medium
phase: null
supersedes:
  - kb-decision-wi-016abe
superseded_by: null
summary: >-
  The owner's call, 2026-10-09. GitHub Issues becomes this repository's tracker, and runbook/
  stops being one. Authority is split. The repository keeps the plan of record that the offline
  lints read: the status table, each phase's goal, exit criteria and proof artefact, ledgers.md
  and roadmap.md. GitHub holds per-item progress: work items, assignment, blockers, bugs and
  findings, and the session narrative, in issue comments and PR bodies. CI never calls the GitHub
  API. Exit criteria stay in the phase files and are not issues. An epic closes only in the PR
  that ticks its exit criteria and sets its row to done. A leaf PR closes its issue with
  Closes #N. The backlog was seeded by PR #60 (c04cb913) as 154 issues, #61 to #214: one epic per
  live status row (#61 to #70), and features and tasks as sub-issues, under the milestones 0.4.0,
  1.0.0-rc.1 and 1.0.0. The link back is checked offline by cargo xtask lints. Each live,
  non-milestone status row names a unique #N in its Tracker column, and each unticked box in a
  phase file outside its exit criteria leads with #N. Struck and ticked boxes are exempt. .bklg/
  stays frozen, carried forward from wi-016abe, which this supersedes for its premise that the
  runbook tracks the remaining work. Unattended sessions work the queue by runbook/afk.md, under
  wi-1fde8c's merge policy. Reconsidered when redkiln-rs is live. Three alternatives lost: the
  runbook as tracker, GitHub as intake only, and a full migration that moves exit criteria too.
depends_on:
  - kb-decision-wi-1fde8c
related:
  - kb-decision-wi-016abe
  - kb-decision-wi-1fde8c
  - kb-decision-wi-ab0a5a
  - kb-decision-wi-38373d
source_paths:
  - .github/tracker.yml
  - runbook/afk.md
  - runbook/README.md
  - runbook/roadmap.md
  - runbook/handover.md
  - scripts/tracker-apply.sh
  - scripts/tracker-dependencies.sh
  - references/evaluation/backlog-2026-10-09/gap-report.md
  - references/evaluation/backlog-2026-10-09/manifest.md
  - references/evaluation/backlog-2026-10-09/issue-map.json
  - references/evaluation/backlog-2026-10-09/dependencies.tsv
  - CLAUDE.md
last_reviewed: 2026-10-09
---

# GitHub Issues is the tracker, and the runbook is the plan of record

**Accepted by the owner on 2026-10-09.** No long-form record exists; this atom is the record. It
supersedes [`wi-016abe`](wi-016abe-reconcile-hs-i0006-by-closing-it-as-is-and.md) only in that
atom's premise that the runbook tracks the remaining work. `.bklg/` stays frozen, which is the
half of that decision carried forward.

## Context

`wi-016abe` closed the redkiln backlog as it stood and re-planned what was left. It rested on a
premise: that *"git history and the runbook are an adequate record of what shipped"*
(`.kb/decisions/wi-016abe-reconcile-hs-i0006-by-closing-it-as-is-and.md:8`). The roadmap turned
that into D-2, *"`.bklg/` is frozen, and this runbook tracks the rest"*
(`runbook/roadmap.md:96-102`). From then on the runbook carried two kinds of content. One was the
plan: phases, exit criteria, proof artefacts and ledgers. The other was progress: open boxes,
session logs, and a handover rewritten whole at the end of each session (`runbook/README.md:71-74`).

The plan held, because `cargo xtask lints` checks the status table against the changelog, the
registry and the ledgers. The progress half did not hold, because nothing checks it:

- The 2026-10-06 status audit called it *"a record-keeping gap"*
  (`references/evaluation/status-audit-2026-10-06/assess-plan-progress.md:113`).
- The seeding pass found a phase row marked `in progress` while its session log was empty
  (`references/evaluation/backlog-2026-10-09/gap-report.md:49`).
- It also found that the handover still listed as untracked a file that had been tracked for
  weeks (`references/evaluation/backlog-2026-10-09/gap-report.md:54`).
- The runbook could not say who held an item, or what blocked it. And an unattended session had
  no queue to take from, so it worked from a hand-written prompt.

PR #60 (`c04cb913`) prepared the move:

- the tracker's vocabulary, `.github/tracker.yml:1-13`;
- the scripts that apply it;
- the seeding evidence, under `references/evaluation/backlog-2026-10-09/`;
- the AFK protocol, `runbook/afk.md`;
- a `Tracker` column on the status table (`runbook/README.md:78`);
- `#N · ` on every open box in the phase files.

GitHub now holds 154 issues, #61 to #214, against 154 proposed
(`references/evaluation/backlog-2026-10-09/gap-report.md:7-17`).

## Decision

1. **GitHub Issues is the tracker.** It holds per-item progress:
   - the work items, as Epic, Feature, Task and Bug;
   - assignment, and the blocked-by links between items;
   - the bugs and findings a session files;
   - the session narrative, in issue comments and PR bodies.

   The vocabulary is `.github/tracker.yml`. The kinds of item and when each closes are
   `runbook/afk.md:16-26`.
2. **The repository keeps the plan of record.** That is the status table, each phase's goal,
   work list, exit criteria and proof artefact, `runbook/ledgers.md` and `runbook/roadmap.md`.
   Where an issue and the plan disagree about what the plan is, the plan wins. Where an issue and
   the specification disagree, the specification wins, as it already does over the runbook.
3. **Exit criteria are not issues.** They stay in the phase files. An epic closes only in the PR
   that ticks its phase's exit criteria and sets its row to `done`, and an agent never closes one
   (`runbook/afk.md:20`, `:110-113`). A leaf PR closes its issue with `Closes #N`.
4. **The link back is checked offline, and CI never calls the GitHub API.** `cargo xtask lints`
   checks two things:
   - every live status row that is not a milestone names a well-formed `#N`, unique across the
     table. A `done` or milestone row may carry `—`.
   - every unticked `- [ ]` box in `runbook/phases/*.md` outside an exit-criteria section leads
     with `#N · `, or `#N, #M · `. Struck (`~~`) and ticked boxes are exempt.

   Both checks read only files (`runbook/README.md:117-120`). The gate therefore does not
   check that `#N` is open or that it means what the row says. That is the cost of a gate that
   runs with no network and no token.
5. **The session protocol moves with it** (`runbook/README.md:36-38`, `:56-68`):
   - a session takes an issue and claims it;
   - it narrates progress in the issue and the PR body;
   - it adds a dated line to `log.md` and the phase's session log only when the plan moves.

   An unattended session follows `runbook/afk.md` and merges under
   [`wi-1fde8c`](wi-1fde8c-should-unattended-sessions-be-allowed-to-self.md): self-merge on green,
   unless the issue carries `door:one-way` (`runbook/afk.md:71-80`).
6. **`.bklg/` stays frozen** and redkiln stays retired. Both are carried forward from
   `wi-016abe` unchanged.

## Consequences

- **Progress can no longer go stale without anyone noticing.** An issue's state, assignee and
  blockers are data that can be queried, so they are not prose that has to be re-read.
- **Two places, and one rule for which wins.** A session reads the handover, the status table
  and its issue (`runbook/README.md:5-7`). The plan never moves on an issue's say-so.
- **The gate checks less than it seems to.** A `#N` that points at a closed or unrelated issue
  passes the gate. Only a session or the owner reading GitHub catches it.
- **Dependencies are the owner's to apply.** An agent's GitHub tools cannot create blocked-by
  links (`scripts/tracker-dependencies.sh:10-11`). The owner runs that script once with their
  own `gh` login, against `references/evaluation/backlog-2026-10-09/dependencies.tsv`. Until
  then, the `status:blocked` labels are the only record of what blocks what.
- **The record now depends on GitHub.** The plan of record does not: it stays in the
  repository. A lost issue history loses the narrative, but not the plan.
- `CLAUDE.md`, `runbook/README.md`, `runbook/roadmap.md` (D-5) and `runbook/afk.md` are updated
  in the change that lands this record.

## What reopens it

- `redkiln-rs` goes live. That condition is carried from `wi-016abe` and the roadmap.
- The status table or a phase file's boxes drift from GitHub in a way the offline lint cannot
  see, and the drift misleads a session. The remedy would be a networked check outside the gate,
  not a change to the split.
- The owner needs exit criteria tracked as items. That is the full migration below, and it needs
  its own record.

## Alternatives that lost

- **The runbook as tracker (the status quo, D-2).** This kept everything in the repository and
  under the gate. It lost on the evidence above: per-item progress written as prose went stale,
  and nothing could detect it. It also had no assignment, no blocked-by and no queue, which an
  unattended session needs.
- **GitHub as intake only.** Issues would hold bugs and findings, and the runbook would keep the
  work items and progress. That makes two trackers with an unclear boundary, and every finding
  would need moving into a phase file by hand. It also leaves the queue and assignment problem
  where it was.
- **A full migration, exit criteria included.** Every exit criterion would become an issue, and
  the runbook would be reduced to prose. This loses the offline lints: the status lint and the
  ledger checks read exit criteria and proof artefacts from the files, and CI does not call
  GitHub. A phase's definition of done would then live in something the gate cannot see, and
  could be closed by a click.
