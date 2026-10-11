---
id: "kb-decision-wi-a1d32d"
title: "File all six now"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"File the six drafted phase-13 issues (P-1 to P-6) from the LiveStore audit on GitHub under epic #64?\", facing six audit findings become tracked phase-13 work only if they are issues; filing writes to the project's GitHub tracker, we decided for File all six now and neglected File P-1 and P-5 only; Fold into #105/#106; Leave as drafts, on the premise that the findings are wanted as tracked work, accepting that if wrong: close six issues as not planned."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-11"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-a1d32d"
question: "File the six drafted phase-13 issues (P-1 to P-6) from the LiveStore audit on GitHub under epic #64?"
door: two-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-11T06:03:47Z"
tree_hash: "88b6230e559136570669b53f461a018c19cb1f8b"
recommended: "A"
flip_condition: "Ryan wanting phase 13 kept out of the tracker until 0.5.0"
---

# File all six now

## Context and problem statement

File the six drafted phase-13 issues (P-1 to P-6) from the LiveStore audit on GitHub under epic #64?

Six audit findings become tracked phase-13 work only if they are issues; filing writes to the project's GitHub tracker.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-a1d32d`. Anchor: `/mnt/project-files/notes/livestore-audit.html`.

## Decision drivers

- tracker noise
- chance findings are lost

## Considered options

### A · File all six now (chosen)

P-2 as status:needs-owner, the rest status:blocked, linked to #64

- Holds if the findings are wanted as tracked work.
- If wrong: close six issues as not planned.

### B · File P-1 and P-5 only

the testkit falsifiers and the simulation; keep the port and runner ideas in the report

- Holds if you want tests before port changes.
- If wrong: file the other four later.

### C · Fold into #105/#106

comment on the two ADR issues linking the report, no new issues

- Holds if you prefer fewer issues.
- If wrong: findings stay untracked.

### D · Leave as drafts

status quo

- Holds if sync work is far off.
- If wrong: findings get rediscovered.

## Evidence

- `/mnt/project-files/notes/livestore-audit.html`: Proposed issues P-1 to P-6, drafts only, none filed, all under epic #64
- `runbook/phases/13-sync.md`: #105 ADR-0026 and #106 ADR-0027 are owner ADRs that gate most of phase 13
- `.github/tracker.yml`: status:needs-owner waits on an owner decision: semver, a frozen clause, an ADR

## Decision outcome

Chosen option: **File all six now**, the recommended option.

Decider's note: Ryan chose 'File all six' on the decision card, 2026-10-11

### Consequences

- Good, because it holds if the findings are wanted as tracked work.
- Bad, because if wrong: close six issues as not planned.

### Confirmation

Revisit when: Ryan wanting phase 13 kept out of the tracker until 0.5.0

## Why this might be wrong

if ADR-0026/0027 reshape the port, P-2 and P-4 may need rewriting

## Provenance

- Decided 2026-10-11T06:03:47Z by human:ryan (user-approved), via chat.
- Raised in session `d2eac15f-b38c-5eeb-93ee-a15e38cd28e3`.
- Verbatim: "kind: question"
