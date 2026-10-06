---
id: "kb-decision-wi-06b54f"
title: "Fix in the records commit"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Fix the pre-existing embedded-spaces assertion messages in query_sql.rs (reviewer F4) now, or leave them for a follow-up?\", facing two assertion strings embed runs of spaces from a lost line continuation, we decided for Fix in the records commit and neglected Leave for a follow-up, on the premise that A records commit goes into #35, accepting that if wrong: Rust diff grows; re-review needed."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-06"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-06b54f"
question: "Fix the pre-existing embedded-spaces assertion messages in query_sql.rs (reviewer F4) now, or leave them for a follow-up?"
door: two-way
blast_radius: file
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-06T13:23:37Z"
tree_hash: "cab814da9263e77984587bd7e7c699b6acf84eed"
recommended: "A"
flip_condition: "If you choose B on the records question"
---

# Fix in the records commit

## Context and problem statement

Fix the pre-existing embedded-spaces assertion messages in query_sql.rs (reviewer F4) now, or leave them for a follow-up?

Two assertion strings embed runs of spaces from a lost line continuation.

Raised by an agent (sweep) as a deferral and captured by Weigh-In as `wi-06b54f`. Anchor: `crates/happenstance-cloudflare/src/query_sql.rs:834`.

## Decision drivers

- Effort

## Considered options

### A · Fix in the records commit (chosen)

Restore the continuation at :834 and :1057.

- Holds if A records commit goes into #35.
- If wrong: Rust diff grows; re-review needed.

### B · Leave for a follow-up

Already on main; cosmetic.

- Holds if #35 stays as reviewed.
- If wrong: Ugly failure messages persist.

## Evidence

- `crates/happenstance-cloudflare/src/query_sql.rs:834`: carries                  {} arms

## Decision outcome

Chosen option: **Fix in the records commit**, the recommended option.

Decider's note: Fix the embedded-spaces messages in the records commit.

### Consequences

- Good, because it holds if A records commit goes into #35.
- Bad, because if wrong: Rust diff grows; re-review needed.

### Confirmation

Revisit when: If you choose B on the records question

## Why this might be wrong

It widens a reviewed PR for a cosmetic fix

## Outcome

The fix rode the records commit as decided, but that commit landed in **#36**, not #35: #35 was merged (`6a3adf6a`) before the records could be added to it (see `wi-d09adc`'s outcome). The two assertion messages in `crates/happenstance-cloudflare/src/query_sql.rs` are fixed there.

## Provenance

- Decided 2026-10-06T13:23:37Z by human:ryan (user-approved), via chat.
- Raised in session `f7fec0fb-352c-4bd4-92ac-d997606ea87e`.
- Verbatim: "Fix the pre-existing embedded-spaces assertion messages in query_sql.rs (reviewer F4) now, or leave them for a follow-up?"
