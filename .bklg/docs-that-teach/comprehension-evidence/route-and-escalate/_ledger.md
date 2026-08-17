---
item: "HS-S0167"
stage: implement
created: "2026-08-17T13:16:20.677Z"
updated: "2026-08-17T13:16:20.677Z"
---

# Acceptance ledger — Route to real destination ids, escalate anything that reopens a tension

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story adds no `pub` item and touches no crate, so every `verifying_test` below is a real,
runnable Static-tier command or a ledger-cited Artifact-evidence read against a real path — the two
tiers `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md`'s `## Testing brief` defines
for this project. Each was chosen because it rejects a named wrong implementation; the brief's
forbidden check ("a `Disposition:` line exists") appears nowhere below.

```yaml
- id: AC-001
  criterion: "GIVEN derived requirement 8 states that a log which is written and filed is a diary, WHEN a reviewer six months later asks who received this log and whether that person could actually do anything about it, THEN ## Hand-off carries a ### Submission record block of five one-line labelled fields — Owner (a named person, never a team), Authority (one line on why that person can act: the role, or the item they own), Date, Submitted (the log at a named commit), Channel (how, so a reviewer can check the claim rather than take it) — and deleting the whole of ## Dispositions index leaves that block untouched, because the submission is evidence and not a roll-up."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Hand-off (the persistent home of the submission record), reached in one hop from .bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions"
  verifying_test: "rg -n '^### Submission record' .bklg/docs-that-teach/comprehension-evidence/_friction-log.md returns exactly one hit falling after the ## Hand-off heading; no NOT-YET-RECORDED remains inside the block; git show <commit>:.bklg/docs-that-teach/comprehension-evidence/_friction-log.md resolves the commit named in Submitted; plus the IQ-2 deletion check against a scratch copy with ## Dispositions index removed"

- id: AC-002
  criterion: "GIVEN U3 opens the log to find the items that are theirs without reading the whole thing, WHEN they read a routed entry's Disposition: field, THEN it names an id from the closed three-class vocabulary — HS-I0005, the support initiative per .redkiln/config.yaml:5; a sibling project id (HS-P0020 … HS-P0023, or HS-P0025 for anything about the audience model); or a named staged deferral carrying the id of the hand-off that will carry it — and every one of those ids resolves to a real file on disk, so not one destination in the log is a description."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → each ### FL-### entry's Disposition: field in ## Chronological record"
  verifying_test: "test -f .bklg/support/initiative.md && test -f .bklg/docs-that-teach/application-author-path/project.md && test -f .bklg/docs-that-teach/reach-and-adapter-path/project.md, plus a test -f for every other id the routed sweep finds; and rg -n '^Disposition: routed:' .bklg/docs-that-teach/comprehension-evidence/_friction-log.md returns no payload without a resolving id"

- id: AC-003
  criterion: "GIVEN 'the sibling project that owns the requirement' is a lookup in the ownership tables rather than a judgement about who seems likely to care, WHEN the receiving owner asks why is this mine, THEN the same one-line Disposition: field carries, beside the id, the initiative id that owner owns which makes the item theirs — a BR, an AC or a DT from the ownership tables — so the correct-owner half of project AC-007 is falsifiable against a table instead of believed."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → each ### FL-### entry's Disposition: field in ## Chronological record"
  verifying_test: "rg '^\\| BR-' .bklg/docs-that-teach/_decomposition.md, rg '^\\| AC-' .bklg/docs-that-teach/_decomposition.md and rg '^\\| DT-' .bklg/docs-that-teach/_decomposition.md — every cited id present, and that row's owner being the same project the item was routed to (a two-sided check); Artifact-evidence: this ledger cites each routed entry by file:line"

- id: AC-004
  criterion: "GIVEN a library bug and a deliberate deferral are the two destinations with nothing on the other end able to object, WHEN U3 or HS-P0025 follows one of them, THEN a support-routed item states plainly that HS-I0005 is at stage: intake with zero projects — real, and structurally empty — and a deferral records the question together with the id of the hand-off that will carry it to /redkiln:kb-ingest or to HS-P0025's promotion pass, with the diff showing not one file added or changed anywhere under .kb/."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → the support-routed and deferral entries' Disposition: fields in ## Chronological record"
  verifying_test: "git diff --name-only for this PR contains no .kb/ path; rg -n 'HS-I0005' .bklg/docs-that-teach/comprehension-evidence/_friction-log.md shows the stage: intake / zero-projects statement alongside the routing; Artifact-evidence: this ledger cites the deferral line and its hand-off id by file:line"

- id: AC-005
  criterion: "GIVEN the initiative assumes the publication project's design gate stays closed and states that 'if it turns out that it does, that is escalated, not absorbed', WHEN the session finds a stumble whose remedy would change a tension already signed off in a sibling's design review, THEN its Disposition: field reads escalated: DT-<n> where DT-<n> is a row in .bklg/docs-that-teach/_decomposition.md's ## Design tension ownership table, names that row's owning project, and states in one line what would have to be re-decided — and DT-9 never appears, because DT-9 is HS-P0024's own tension and resolving it is not escalating it."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → each escalated ### FL-### entry's Disposition: field in ## Chronological record"
  verifying_test: "rg '^\\| DT-' .bklg/docs-that-teach/_decomposition.md yields the legal set; every escalated DT id is a member of it, the owning project named in the entry equals that row's owner, and DT-9 appears in no escalation; the escalation's substance stays reviewer-read with a file:line per escalation in this ledger"

- id: AC-006
  criterion: "GIVEN content-fixes-from-dispositions is landing fixes in the same slice and the project risk table rates re-deciding a signed-off tension Medium/High — 're-deciding one here would put half a decision in each' — WHEN that story picks up its inbox, THEN the escalated set and the fixed set share no FL-### id, ## Dispositions index carries the escalation roll-up derived from the entries so the two sets can be compared without re-reading the log, and if no stumble would reopen a tension the log says so in that roll-up rather than leaving the question unasked."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Dispositions index (the derived escalation roll-up), over the Disposition: fields in ## Chronological record"
  verifying_test: "the FL-### ids from rg -n '^Disposition: escalated:' .bklg/docs-that-teach/comprehension-evidence/_friction-log.md intersected with those from rg -n '^Disposition: fixed:' on the same file returns empty; where the escalated set is empty, an explicit zero-escalations sentence is rg-findable in the roll-up"

- id: AC-007
  criterion: "GIVEN a destination is declined by its owner ('not mine'), or an escalation later proves unnecessary, WHEN the correction is made, THEN the entry's Revisions: field gains a dated line carrying the earlier destination and the reason for the change, the original Disposition: text is not edited, and no FL-### id and no heading label anywhere in ## Chronological record moves — so a submission that already cited FL-004 outward still lands on the item it cited."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md → each ### FL-### entry's Revisions: field in ## Chronological record"
  verifying_test: "git diff on .bklg/docs-that-teach/comprehension-evidence/_friction-log.md confines every hunk to Disposition: lines, Revisions: lines, the ## Dispositions index block and the submission block; rg -n '^### FL-' on that file returns byte-identical output before and after this PR; read against .kb/governance/rewrite-the-referent-never-the-reasoning.md"

- id: AC-008
  criterion: "GIVEN U3 arrives at HS-P0024 from redkiln board needing only the items that are theirs, WHEN they open the project card, THEN one hop from project.md's ## Companions list reaches the log and at most one optional further hop through ## Dispositions index reaches the entry; every destination, ownership reason and escalation is readable at its own entry; the index is a two-column table in docs/README.md's existing shape whose first line states it is derived and that the chronological record is authoritative; the log still carries exactly the skeleton's eight ## sections; and everything this story added is static text — no fold, no widget, no script, nothing carrying meaning by colour or emoji alone, every checkbox on one line."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions, one hop to .bklg/docs-that-teach/comprehension-evidence/_friction-log.md → ## Dispositions index"
  verifying_test: "rg -n '^## ' .bklg/docs-that-teach/comprehension-evidence/_friction-log.md returns exactly the eight headings in order; rg -n '<details>|<summary>|<script>' on that file returns nothing; every checkbox line matches a single-line '- [ ] …'; rg -n '_friction-log' .bklg/docs-that-teach/comprehension-evidence/project.md returns the Companions row and git diff carries no frontmatter hunk; plus the IQ-2 deletion check — delete ## Dispositions index and no destination, reason or escalation is lost"
```
