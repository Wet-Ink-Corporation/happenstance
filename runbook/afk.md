# Working the backlog unattended

How an unattended (AFK) session takes work from GitHub Issues, does it, and reports
back. It is the general form of [`phase-15-afk-prompt.md`](phase-15-afk-prompt.md),
which was written for one phase and one night.

**Where things live.** GitHub Issues holds the work items: what is open, who has
it, what blocks it, and the bugs and findings a session files. This repository
holds the plan of record: the status table in [`README.md`](README.md), each phase
file's goal, exit criteria and proof artefact, [`ledgers.md`](ledgers.md) and
[`roadmap.md`](roadmap.md). The tracker's vocabulary — labels, milestones, issue
types — is [`.github/tracker.yml`](../.github/tracker.yml), and the evidence the
backlog was seeded from is
[`references/evaluation/backlog-2026-10-09/`](../references/evaluation/backlog-2026-10-09/gap-report.md).

## The hierarchy

| Kind | What it is | Closes when |
|---|---|---|
| **Epic** | One phase row in the status table. | The PR that ticks its exit criteria and sets the row to `done`. Never closed by an agent. |
| **Feature** | One observable outcome, 1–5 PRs. Usually one `## Work` box in a phase file. | Its last task closes, or its one PR merges. |
| **Task** | One PR. Exists only when a feature needs more than one. | Its PR merges with `Closes #N`. |
| **Bug** | Something does other than what it says it does. | Its fix merges. |

A feature that fits in one PR has no tasks: it is its own leaf. Split it into tasks
only when the second PR becomes real, not in advance.

## Labels: a state machine, not a tag cloud

Every open issue has an issue type (Epic, Feature, Task or Bug) and exactly one
`status:` and one `source:` label. Every label is defined in `tracker.yml`; the
`type:*` labels there are a fallback the seeded issues do not use.

| Status | Means | An unattended session… |
|---|---|---|
| `status:needs-triage` | New, not yet shaped or placed. | files here, and does not take from here. |
| `status:ready-for-agent` | The brief is complete, the door is two-way, and nothing blocks it. | **takes from here.** |
| `status:ready-for-human` | Shaped, but needs judgement, credentials or a person. | leaves it. |
| `status:needs-owner` | Waits on an owner decision or authority. | leaves it, and may add evidence in a comment. |
| `status:blocked` | Waits on another open issue, named by a blocked-by link. | leaves it. |

Two more labels decide what a session may do. `door:one-way` marks a change that
is hard to reverse: a published API, a `[FROZEN]` clause, an accepted ADR body, a
tag or a publish, or deleting a branch. A session never closes, merges or
completes one; it raises it and stops. `semver:breaking` work ships only in a
breaking release, and only the milestone that release names may carry it.

## Taking work

1. **Find the queue.** Search:

   ```
   repo:wet-ink-corporation/happenstance is:issue is:open label:status:ready-for-agent no:assignee -label:door:one-way
   ```

   Work the milestones in this order: `0.4.0`, then `1.0.0-rc.1`, then `1.0.0`, then
   issues with no milestone. Within a milestone, go by the Priority field, then by
   the order of sub-issues under their parent. Skip anything whose parent epic is
   blocked. The status table in `README.md` says which phases are live.
2. **Claim it before you start.** Assign the issue to yourself and comment with the
   branch name. One session holds one leaf at a time.
3. **Read the brief, then check it against the repository.** The body's
   *References* and the issue's `source_cite` point at the line it was drafted
   from. If the tree has moved past the brief — the work is already done, the
   cited line says something else — comment with the evidence, set
   `status:needs-triage`, and take the next issue.
4. **One PR per leaf.** The PR body says `Closes #N`. Follow the session protocol
   in [`README.md`](README.md) for everything the repository still owns: exit
   criteria, the status row, and `log.md`. Progress narrative goes in the issue
   and the PR, not in a phase file's session log.
5. **Merge policy (`wi-1fde8c`).** A session squash-merges its own PR once every
   check is green, **unless** the issue carries `door:one-way`. A one-way PR is
   opened and left for the owner. This extends `wi-ab0a5a` beyond phase 15, and
   its retry rule comes with it:
   - If a check fails in code the PR did not touch, re-run only the failed jobs,
     once.
   - A second failure, or any failure in code the PR touched, stops that leaf:
     leave the PR open, comment why, and take the next issue.
   - A `semver:` leaf may self-merge, because `cargo-semver-checks` runs on the PR.
     Its `CHANGELOG.md` entry is part of the leaf, not a follow-up.

## Filing what you find

Anything outside the leaf you are working on — a bug, a stale record, a citation
that no longer resolves, a check that cannot fail — becomes an issue. Do not widen
the PR.

- Use the **Agent finding** form. If you file through the API, it amounts to
  issue type Bug or Feature, `status:needs-triage` and `source:afk`.
- Search first (`is:issue in:title <clause or file>`) and comment on an existing
  issue rather than opening a duplicate.
- Give the evidence as `path:line` at a named commit, and name the wrong behaviour
  you saw, not only the right one.
- Set no parent and no milestone. Triage does that.
- Never close an issue you did not open, unless your merged PR closes it.

## Decisions

The Weigh-In protocol in `phase-15-afk-prompt.md` § *Decision protocol* still
applies.

- **A two-way, judgement-bound call:** take your recommendation, record it, and
  say so in a comment on the issue.
- **A one-way or authority-bound call:** raise it with `door: one-way` and
  `did: blocked`, set the issue to `status:needs-owner`, comment with the question
  and the fact that would flip it, and move on to independent work.

Never write an ADR as a side effect of a leaf.

## Never

- Flip a status-table row, tick an exit criterion, or close an epic. The owner
  does that, in the PR that finishes the phase.
- Take a `door:one-way` issue, or tag, publish or delete a remote branch.
- Run `redkiln` or edit `.bklg/`. Both are frozen records.
- Skip, ignore or weaken a check to get to green.
- Use `--no-verify`, or force-push to `main`.

## Final report

At the end of a session, post one comment on each issue you touched. Then print a
summary with:

- the PRs you opened, with their state;
- the issues you filed;
- every default you took, as id, question and choice;
- every item you left blocked, with what it is waiting on;
- anything you assumed and did not verify, said in those words.
