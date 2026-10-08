---
id: "kb-decision-wi-863ab8"
title: "Docs PR now"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"ES-12's falsifier (query_items_share_one_snapshot red on Neon) has fired in CI but the spec marker, Neon's README and the open question still say it passes. Record that now in a small docs PR, or leave it for L8's ADR?\", facing ES-12's falsifier has fired, but the spec marker and Neon's README still say the rule passes, we decided for Docs PR now and neglected Leave for L8, on the premise that The spec should say what is true now, accepting that if wrong: L8's ADR rewrites the same lines."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-07"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-863ab8"
question: "ES-12's falsifier (query_items_share_one_snapshot red on Neon) has fired in CI but the spec marker, Neon's README and the open question still say it passes. Record that now in a small docs PR, or leave it for L8's ADR?"
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-07T03:57:31Z"
tree_hash: "cb53b0117974473d5baab3b551a15152f7b07678"
recommended: "A"
flip_condition: "If L8 starts this week"
---

# Docs PR now

## Context and problem statement

ES-12's falsifier (query_items_share_one_snapshot red on Neon) has fired in CI but the spec marker, Neon's README and the open question still say it passes. Record that now in a small docs PR, or leave it for L8's ADR?

ES-12's falsifier has fired, but the spec marker and Neon's README still say the rule passes.

Raised by an agent (sweep) as a deferral and captured by Weigh-In as `wi-863ab8`. Anchor: `spec/SPECIFICATION.md:3214`.

## Decision drivers

- Spec truth

## Considered options

### A · Docs PR now (chosen)

Record the firing: ES-12 marker note, Neon README count, open-question reopen trigger. No code, no marker change.

- Holds if The spec should say what is true now.
- If wrong: L8's ADR rewrites the same lines.

### B · Leave for L8

L8's superseding ADR records ES-11 and ES-12 together.

- Holds if L8 starts soon.
- If wrong: The spec stays wrong until then.

## Evidence

- `spec/SPECIFICATION.md:3214`: ES-12 [PROVISIONAL], says the rule passes
- `runbook/ledgers.md:191`: ES-11 and ES-12 frozen at 17 by ADR-0087 (the row this cited, ES-12's falsifier, is struck)
- `PR #36 run 37504851570 (attempt 2)`: query_items_share_one_snapshot ... FAILED

## Decision outcome

Chosen option: **Docs PR now**, the recommended option.

Decider's note: Small docs PR now recording ES-12's fired falsifier.

### Consequences

- Good, because it holds if The spec should say what is true now.
- Bad, because if wrong: L8's ADR rewrites the same lines.

### Confirmation

Revisit when: If L8 starts this week

## Why this might be wrong

L8 will touch the same lines again

## Provenance

- Decided 2026-10-07T03:57:31Z by human:ryan (user-approved), via chat.
- Raised in session `f7fec0fb-352c-4bd4-92ac-d997606ea87e`.
- Verbatim: "ES-12's falsifier (query_items_share_one_snapshot red on Neon) has fired in CI but the spec marker, Neon's README and the open question still say it passes. Record that now in a small docs PR, or leave it for L8's ADR?"
