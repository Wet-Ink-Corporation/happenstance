---
id: "kb-decision-wi-0fbcac"
title: "Freeze all four with three edits"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"ADR-0027 (#106): ratify SY-7 (one adjudicator per fact family), SY-10 (directional merge rules), SY-14 (bulk ingest via the VT-8 unique EventId, measured on Neon) and SY-23 (EventId order) as the spec words them?\", facing ADR-0027 freezes four sync clauses and an accepted ADR is immutable, we decided for Freeze all four with three edits and neglected Freeze verbatim, on the premise that no clause is disputed, accepting that if wrong: a new superseding ADR."
depends_on: []
related:
  - kb-decision-0027
source_paths: []
last_reviewed: "2026-10-14"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-0fbcac"
question: "ADR-0027 (#106): ratify SY-7 (one adjudicator per fact family), SY-10 (directional merge rules), SY-14 (bulk ingest via the VT-8 unique EventId, measured on Neon) and SY-23 (EventId order) as the spec words them?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-14T02:39:26Z"
tree_hash: "2f25eccf8841e601a8ba820350af089070f5f579"
recommended: "A"
flip_condition: "Ryan disputes one clause"
---

# Freeze all four with three edits

## Context and problem statement

ADR-0027 (#106): ratify SY-7 (one adjudicator per fact family), SY-10 (directional merge rules), SY-14 (bulk ingest via the VT-8 unique EventId, measured on Neon) and SY-23 (EventId order) as the spec words them?

ADR-0027 freezes four sync clauses and an accepted ADR is immutable.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-0fbcac`. Anchor: `runbook/ledgers.md:310`.

## Considered options

### A · Freeze all four with three edits (chosen)

SY-14 bound = one ingest round trip per pulled batch, freeze after the Neon measurement; SY-10 rationale = adjudication; SY-23 gains not-fair-not-temporal caveat

- Holds if no clause is disputed.
- If wrong: a new superseding ADR.

### B · Freeze verbatim

as the spec words them

- Holds if wording is fine.
- If wrong: SY-14 stays untestable as worded.

## Evidence

- `crates/happenstance-core/src/identity.rs:96`: EventId derives Ord (SY-23 already published)
- `crates/happenstance-sqlite/src/ingest_spike.rs:12`: ON CONFLICT (origin_store, origin_position) DO NOTHING (SY-14 mechanism built)
- `references/scenarios/README.md:436`: first to the hub, not first to decide (SY-10/SY-7)

## Decision outcome

Chosen option: **Freeze all four with three edits**, the recommended option.

Decider's note: Ryan 2026-10-14: 'freeze all 4 with edits'. Freeze SY-7 as written; SY-10 with adjudication as the rationale; SY-14 reworded to one ingest round trip per pulled batch however many events are held, frozen once the Neon measurement passes; SY-23 with the not-fair, not-temporal caveat.

### Consequences

- Good, because it holds if no clause is disputed.
- Bad, because if wrong: a new superseding ADR.

### Confirmation

Revisit when: Ryan disputes one clause

## Why this might be wrong

the Neon measurement could fail, leaving SY-14 deferred

## Provenance

- Decided 2026-10-14T02:39:26Z by human:ryan (user-approved), via chat.
- Raised in session `2fe67e8e-8afc-5d1f-abc9-f48820905de5`.
- Verbatim: "ADR-0027 (#106): ratify SY-7 (one adjudicator per fact family), SY-10 (directional merge rules), SY-14 (bulk ingest via the VT-8 unique EventId, measured on Neon) and SY-23 (EventId order) as the spec words them?"
