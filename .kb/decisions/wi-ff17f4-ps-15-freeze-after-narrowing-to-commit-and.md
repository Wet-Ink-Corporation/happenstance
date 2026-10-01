---
id: "kb-decision-wi-ff17f4"
title: "Freeze narrowed; rollback non-normative"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"PS-15: freeze after narrowing to commit and reset, or give it another disposition?\", facing freezing PS-15 makes the run-time foreign-batch refusal on commit and reset a 1.0 promise, and its falsifier can only be fired by a major release, we decided for Freeze narrowed; rollback non-normative and neglected Freeze narrowed, and land a rollback rule now; Keep PROVISIONAL, freeze-by-18, on the premise that No one needs a guarantee about a foreign rollback before 1.0, accepting that if wrong: A later rollback promise is a new additive clause with its own rule."
depends_on: []
related: ["kb-decision-0075"]
source_paths: ["spec/SPECIFICATION.md", ".kb/decisions/0075-the-projection-ports-1-0-clauses.md"]
last_reviewed: "2026-09-30"
reversibility: low
phase: 17
supersedes: []
superseded_by: null
weighin_item: "wi-ff17f4"
question: "PS-15: freeze after narrowing to commit and reset, or give it another disposition?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-30T03:32:25Z"
tree_hash: "5c92af53e6f4cbe79c3137fb254a71da6706b7a0"
recommended: "A"
flip_condition: "an adopter or phase 13/18 consumer that relies on a foreign rollback's outcome"
---

# Freeze narrowed; rollback non-normative

## Context and problem statement

PS-15: freeze after narrowing to commit and reset, or give it another disposition?

Freezing PS-15 makes the run-time foreign-batch refusal on commit and reset a 1.0 promise, and its falsifier can only be fired by a major release.

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-ff17f4`. Anchor: `runbook/ledgers.md`.

## Decision drivers

- Checked by a rule today
- Work in L3
- 1.0 promise

## Considered options

### A · Freeze narrowed; rollback non-normative (chosen)

As drafted: MUST covers commit and reset, which the existing rule checks; rollback's behaviour described, not promised.

- Holds if No one needs a guarantee about a foreign rollback before 1.0.
- If wrong: A later rollback promise is a new additive clause with its own rule.

### B · Freeze narrowed, and land a rollback rule now

Add a conformance rule plus mutant (begin on one store, roll back on the other, both unchanged) and make that a second MUST.

- Holds if The rollback leg matters to adopters now.
- If wrong: One more testkit rule and mutant in L3; a testkit minor per CF-31 and a CHANGELOG entry.

### C · Keep PROVISIONAL, freeze-by-18

Leave the marker and move the row to phase 18.

- Holds if Phase 18 builds an instrument for the type-level construction.
- If wrong: Phase 18 has none, so the row fails the ledger lint when 18 closes; an evasion.

## Evidence

- `spec/SPECIFICATION.md:5828`: PS-15 — commit and reset MUST reject a batch begun on a different instance ... [FROZEN] — narrowed and frozen by ADR-0075
- `references/adr/0066-what-1-0-promises.md:164`: ADR-0066 assigned PS-15 freeze-by-17, with 'the MUST narrows to commit and reset' as a route
- `crates/happenstance-core/src/projection.rs:544`: rollback returns Result<(), Self::Error> and cannot carry the port-level ForeignBatch variant
- `.kb/decisions/0066-what-1-0-promises.md:148`: renew-past-1.0 allowed only where the falsifier firing would be additive or would relax an obligation

## Decision outcome

Chosen option: **Freeze narrowed; rollback non-normative**, the recommended option.

Decider's note: Chosen after the full walkthrough: freeze narrowed to commit and reset; rollback non-normative; a rollback promise later is an additive clause.

### Consequences

- Good, because it holds if No one needs a guarantee about a foreign rollback before 1.0.
- Bad, because if wrong: A later rollback promise is a new additive clause with its own rule.

### Confirmation

Revisit when: an adopter or phase 13/18 consumer that relies on a foreign rollback's outcome

## Why this might be wrong

If an adapter someday mutates a store on a foreign rollback, A does not forbid it; B would

## Provenance

- Decided 2026-09-30T03:32:25Z by human:ryan (user-approved), via chat.
- Raised in session `c8638052-7723-4bfd-8e86-255fbf4bff8b`.
- Verbatim: "PS-15: freeze after narrowing to commit and reset, or give it another disposition?"
