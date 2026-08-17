---
item: "HS-S0163"
stage: implement
created: "2026-08-17T13:16:18.427Z"
updated: "2026-08-17T13:16:18.427Z"
---

# Acceptance ledger — The auditable-shape friction-log scaffold

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
  criterion: "GIVEN U2 opens the repository at session start with a stranger already in the room, WHEN they look for where the session gets recorded, THEN the scaffold exists carrying all eight sections in the fixed order (## Status, ## Scenario, ## Session record, ## Severity scale, ## Chronological record, ## Dispositions index, ## Scope of the claim, ## Hand-off), zero findings, a ## Status banner stating that no session has been run and naming the token once, and every unfilled slot carrying that token — so nothing is left for U2 to invent, and no later reader can mistake the scaffold for evidence."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions (the project card's drill-down index)"
  verifying_test: "rg -n '^## ' .bklg/docs-that-teach/comprehension-evidence/_friction-log.md returns exactly the eight headings in order; rg -c 'NOT-YET-RECORDED' on the same file is non-zero; rg -n '^### FL-' on the same file returns nothing"

- id: AC-002
  criterion: "GIVEN the protocol was fixed before anyone was recruited, WHEN U3 six months later asks whether the severity scale was tuned to the findings, THEN the scaffold's ## Scenario, ## Severity scale and the narration-mode field of ## Session record are verbatim transcriptions of _design.md's protocol section, each naming that file inline as the source of record — and the scaffold names no scenario, no scale, no narration mode and no disqualifying criterion of its own."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_design.md → the protocol section added by dt9-and-fixed-protocol (the second mount), reached from project.md → ## Companions"
  verifying_test: "git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md predates this story's commit, and the transcribed blocks in .bklg/docs-that-teach/comprehension-evidence/_friction-log.md match _design.md character for character"

- id: AC-003
  criterion: "GIVEN U3 reads the log as a git diff with every style stripped, or through a screen reader, WHEN they try to rank what stopped the reader, THEN every severity mark is a text token drawn from the token-to-meaning legend reproduced in ## Severity scale, and nothing anywhere in the file carries meaning by colour or emoji alone."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions (the project card's drill-down index)"
  verifying_test: "git show HEAD:.bklg/docs-that-teach/comprehension-evidence/_friction-log.md read as raw bytes — every Severity position is a word or number from the legend, and a scan for emoji or colour-swatch glyphs in a Severity position returns nothing"

- id: AC-004
  criterion: "GIVEN U2 is watching a stranger and typing at minute 40, WHEN a stumble happens, THEN the entry template already sitting in ## Chronological record offers six one-line labelled fields — Time, Kind, Severity, What happened, Disposition, Revisions — so the only thing U2 decides is what just occurred; and intervention and abandonment are first-class values of Kind alongside observation and reaction, so a log with zero interventions asserts that none occurred (IQ-5) and a session that stops at the first blocker produces a finding rather than a void (IQ-6)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions (the project card's drill-down index)"
  verifying_test: "Artifact-evidence: ledger-cited file:line at the entry-template block in .bklg/docs-that-teach/comprehension-evidence/_friction-log.md showing all six labels and all four Kind values, read against .bklg/docs-that-teach/comprehension-evidence/_decomposition.md IQ-5/IQ-6 and UX-AC-009"

- id: AC-005
  criterion: "GIVEN a sibling project has already cited FL-004 in its own risk table, WHEN it follows that anchor after the log is finalised, THEN it lands on the item it cited — because the scaffold fixes, in ## Chronological record's own preamble, that ids are FL-### assigned in occurrence order, never reassigned, that the section is append-only, and that the heading line including its label is frozen at write time."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions (the project card's drill-down index)"
  verifying_test: "Artifact-evidence: ledger-cited file:line at the ## Chronological record preamble in .bklg/docs-that-teach/comprehension-evidence/_friction-log.md, checked against .bklg/docs-that-teach/comprehension-evidence/_decomposition.md IQ-3 / UX-AC-007 — the label freeze must be stated, not implied by 'ids are stable'"

- id: AC-006
  criterion: "GIVEN U3 opens the log to find the items that are theirs, WHEN they read one stumble entry, THEN its disposition is legible at that entry without navigating anywhere (IQ-1), exactly one of five arms is expressible — not yet dispositioned / fixed: <ref> / accepted: <reason> / routed: <id> / escalated: DT-<n>, with zero arms and two arms both failures — and a Revisions: slot is present from the moment the entry is first written, defaulting to none, so a later change appends beside the original instead of erasing it (IQ-4)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions (the project card's drill-down index)"
  verifying_test: "Artifact-evidence: ledger-cited file:line at both the Disposition and Revisions fields of the entry template and at the five-arm list in .bklg/docs-that-teach/comprehension-evidence/_friction-log.md, read against .kb/governance/rewrite-the-referent-never-the-reasoning.md and IQ-1 / IQ-4"

- id: AC-007
  criterion: "GIVEN a routed stumble arrives at an owner, WHEN that owner asks whether it is theirs, THEN the shape has already answered — the routed: arm is typed to an item id that resolves to a real file (HS-P0022, HS-P0023, the support initiative per .redkiln/config.yaml:5, or a named staged deferral) and the escalated: arm to a DT id from .bklg/docs-that-teach/_decomposition.md's ownership table, so a prose destination is not expressible in the shape at all."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions (the project card's drill-down index)"
  verifying_test: "test -f .bklg/support/initiative.md && test -f .bklg/docs-that-teach/application-author-path/project.md && test -f .bklg/docs-that-teach/reach-and-adapter-path/project.md; rg '^\\| DT-' .bklg/docs-that-teach/_decomposition.md yields the DT vocabulary the escalated arm names"

- id: AC-008
  criterion: "GIVEN a reviewer deletes the entire ## Dispositions index, WHEN they re-read what is left, THEN not one stumble and not one disposition has been lost — the index says in its own first line that it is derived and that the chronological record is authoritative (IQ-2) — and the whole file is still readable and navigable with no script, no widget and no rendering step, browser find and stable heading anchors only, with every checkbox on a single line."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions (the project card's drill-down index)"
  verifying_test: "rg -n '<details>|<summary>|<script>' .bklg/docs-that-teach/comprehension-evidence/_friction-log.md returns nothing; every checkbox line matches a single-line '- [ ] …'; plus the IQ-2 deletion check performed against the file and its result ledger-cited"

- id: AC-009
  criterion: "GIVEN a person who has never seen this story reaches HS-P0024 from redkiln board, WHEN they open the project card, THEN one hop from project.md's ## Companions list reaches the scaffold, _design.md's protocol section names it by path and heading vocabulary, and what they find is composed from this repository's existing document primitives — the brief spine, the ## Shape decision table, the risk-table shape, docs/README.md's two-column routing table, the one-line checkbox — rather than a hand-rolled format or a new navigation widget."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/project.md → ## Companions, plus the second mount at .bklg/docs-that-teach/comprehension-evidence/_design.md → the protocol section"
  verifying_test: "rg -n '_friction-log' .bklg/docs-that-teach/comprehension-evidence/project.md .bklg/docs-that-teach/comprehension-evidence/_design.md returns both the Companions row and the protocol line, and git diff carries no frontmatter hunk in either file"
```
