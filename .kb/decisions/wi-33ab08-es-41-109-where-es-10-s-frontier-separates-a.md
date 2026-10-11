---
id: "kb-decision-wi-33ab08"
title: "contains_event_id means held, not visible"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"ES-41 (#109): where ES-10's frontier separates a committed row from a visible one, does contains_event_id mean held or visible?\", facing ES-41 is frozen but leaves held-versus-visible open for frontier stores, we decided for Held and neglected Visible; Leave unsettled, on the premise that membership should agree with the append condition and VT-8 uniqueness, accepting that if wrong: a caller sees true and a read that does not yet yield it."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-33ab08-es-41-109-where-es-10-s-frontier-separates-a.md
last_reviewed: "2026-10-11"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-33ab08"
question: "ES-41 (#109): where ES-10's frontier separates a committed row from a visible one, does contains_event_id mean held or visible?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-11T01:33:03Z"
tree_hash: "ebda45c0d57455f67f7b3790a2c7dad16d7361c8"
recommended: "A"
flip_condition: "a caller that needs true to mean read yields it now"
---

# Held

## Context and problem statement

ES-41 (#109): where ES-10's frontier separates a committed row from a visible one, does contains_event_id mean held or visible?

ES-41 is frozen but leaves held-versus-visible open for frontier stores.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-33ab08`. Anchor: `crates/happenstance-postgres/src/event_store.rs:641`.

## Considered options

### A · Held (chosen)

Codify today's Postgres/Neon answer; new ADR adds the sentence; rule behind a defaulted-declined fixture capability; mutant carries the frontier predicate

- Holds if membership should agree with the append condition and VT-8 uniqueness.
- If wrong: a caller sees true and a read that does not yet yield it.

### B · Visible

Carry the frontier predicate into the probe on Postgres and Neon

- Holds if membership must match read exactly.
- If wrong: probe disagrees with ingest dedup and the append condition; behaviour break on two adapters.

### C · Leave unsettled

Keep the non-normative prose to 1.0

- Holds if nobody relies on it.
- If wrong: 1.0 ships an unpinned reading on a frozen method.

## Evidence

- `spec/SPECIFICATION.md:4958`: whether holds means held in the committed log or visible to read is not settled here
- `crates/happenstance-testkit/src/contract.rs:291`: ES-25 evaluates a condition over what the store holds

## Decision outcome

Chosen option: **Held**, the recommended option.

Decider's note: Ryan, 2026-10-11: go with your recommendations

### Consequences

- Good, because it holds if membership should agree with the append condition and VT-8 uniqueness.
- Bad, because if wrong: a caller sees true and a read that does not yet yield it.

### Confirmation

Revisit when: a caller that needs true to mean read yields it now

## Why this might be wrong

the issue's own acceptance names a frontier-carrying mutant, so it already assumes A; check that is the intent not an artefact

## Provenance

- Decided 2026-10-11T01:33:03Z by human:ryan (user-approved), via chat.
- Raised in session `b9226544-e5b2-5679-8317-bd20a3336161`.
- Verbatim: "ES-41 (#109): where ES-10's frontier separates a committed row from a visible one, does contains_event_id mean held or visible?"
