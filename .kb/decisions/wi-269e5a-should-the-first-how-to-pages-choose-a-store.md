---
id: "kb-decision-wi-269e5a"
title: "Follow-up branch"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Should the first how-to pages (choose-a-store, rebuild-a-read-model, retry-a-command, pass-the-conformance-suite, run-on-a-durable-object) be written on this branch before it merges, or in a follow-up?\", facing you asked for user documentation; I stopped at the infrastructure and five existing pages, we decided for Follow-up branch and neglected This branch, on the premise that a smaller reviewable merge matters, accepting that if wrong: first deploy is thin on how-tos."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-269e5a-should-the-first-how-to-pages-choose-a-store.md
last_reviewed: "2026-09-30"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-269e5a"
question: "Should the first how-to pages (choose-a-store, rebuild-a-read-model, retry-a-command, pass-the-conformance-suite, run-on-a-durable-object) be written on this branch before it merges, or in a follow-up?"
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-30T02:54:01Z"
tree_hash: "196f0e0f336a42fd143c6a5c88bbbe30c0f559b4"
recommended: "A"
flip_condition: "wanting how-tos in the first deploy"
---

# Follow-up branch

## Context and problem statement

Should the first how-to pages (choose-a-store, rebuild-a-read-model, retry-a-command, pass-the-conformance-suite, run-on-a-durable-object) be written on this branch before it merges, or in a follow-up?

You asked for user documentation; I stopped at the infrastructure and five existing pages.

Raised by an agent (sweep) as a deferral and captured by Weigh-In as `wi-269e5a`. Anchor: `runbook/phases/22-docs-site.md`.

## Decision drivers

- PR size

## Considered options

### A · Follow-up branch (chosen)

Merge the site now; write how-tos next, each reviewed.

- Holds if a smaller reviewable merge matters.
- If wrong: first deploy is thin on how-tos.

### B · This branch

Write the five how-tos before merging.

- Holds if the first deploy should be complete.
- If wrong: a much bigger PR to review.

## Evidence

- `runbook/phases/22-docs-site.md`: [ ] The first how-to pages in docs/ ...

## Decision outcome

Chosen option: **Follow-up branch**, the recommended option.

Decider's note: Merge the site first; how-tos on a follow-up branch.

### Consequences

- Good, because it holds if a smaller reviewable merge matters.
- Bad, because if wrong: first deploy is thin on how-tos.

### Confirmation

Revisit when: wanting how-tos in the first deploy

## Why this might be wrong

if the site should not go live without how-tos

## Provenance

- Decided 2026-09-30T02:54:01Z by human:ryan (user-approved), via chat.
- Raised in session `f0834c2a-1fe5-44d1-9d05-77d50ee6b39b`.
- Verbatim: "Should the first how-to pages (choose-a-store, rebuild-a-read-model, retry-a-command, pass-the-conformance-suite, run-on-a-durable-object) be written on this branch before it merges, or in a follow-up?"
