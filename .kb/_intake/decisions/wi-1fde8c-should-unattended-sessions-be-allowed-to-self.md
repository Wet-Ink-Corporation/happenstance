---
id: "kb-decision-wi-1fde8c"
title: "Self-merge any green leaf except door:one-way"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should unattended sessions be allowed to self-merge on green, as wi-ab0a5a allowed for phase 15 only?\", facing whether an unattended session may merge its own green PR decides how much it can finish without you, we decided for Self-merge any green leaf except door:one-way and neglected Review required; Self-merge two-way, non-semver leaves, on the premise that semver-checks and the gate are trusted, accepting that if wrong: an unreviewed API addition reaches a release."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-09"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-1fde8c"
question: "Should unattended sessions be allowed to self-merge on green, as wi-ab0a5a allowed for phase 15 only?"
door: one-way
blast_radius: system
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-10-09T04:40:10Z"
tree_hash: "ea04f06c710bf10da999320ff25db70416b299a3"
recommended: "B"
flip_condition: "a self-merged PR needs a revert"
---

# Self-merge any green leaf except door:one-way

## Context and problem statement

Should unattended sessions be allowed to self-merge on green, as wi-ab0a5a allowed for phase 15 only?

Whether an unattended session may merge its own green PR decides how much it can finish without you.

Raised by an agent (marker) as a assumption and captured by Weigh-In as `wi-1fde8c`. Anchor: `runbook/afk.md:84`.

## Decision drivers

- throughput unattended
- what lands unseen

## Considered options

### A · Review required

AFK PRs wait for you

- Holds if you review within a day or two.
- If wrong: a queue of open PRs and stacked conflicts.

### B · Self-merge two-way, non-semver leaves

merge on green unless door:one-way or semver:*

- Holds if the gate catches what matters.
- If wrong: a bad internal change lands; revert is cheap.

### C · Self-merge any green leaf except door:one-way (chosen)

widest grant

- Holds if semver-checks and the gate are trusted.
- If wrong: an unreviewed API addition reaches a release.

## Evidence

- `runbook/phase-15-afk-prompt.md:16`: wi-ab0a5a: self-merge on green — one PR per work item, squash-merged once every check passes
- `runbook/afk.md`: A session opens its PR and leaves it for review. wi-ab0a5a granted self-merge on green to phase 15 only

## Decision outcome

Chosen option: **Self-merge any green leaf except door:one-way**, overriding the recommendation (B).

Decider's note: Self-merge all but one-way

### Consequences

- Good, because it holds if semver-checks and the gate are trusted.
- Bad, because if wrong: an unreviewed API addition reaches a release.

### Confirmation

Revisit when: a self-merged PR needs a revert

## Why this might be wrong

the gate has known vacuous checks (audit V6, A7)

## Provenance

- Decided 2026-10-09T04:40:10Z by human:ryan (user-directed), via chat.
- Raised in session `bc46557d-5fc1-52e1-8e6a-88a74e68a653`.
- Verbatim: "kind: assumption"
