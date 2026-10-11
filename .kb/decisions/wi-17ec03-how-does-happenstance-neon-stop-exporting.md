---
id: "kb-decision-wi-17ec03"
title: "ProbeThenWriteStore becomes test-only"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"How does happenstance-neon stop exporting ProbeThenWriteStore (#78): hide with doc(hidden), gate behind a feature, move to the testkit, or make it test-only?\", facing a deliberately lost-update EventStore is pub and re-exported at the crate root of a published adapter, we decided for Test-only and neglected Feature gate; doc(hidden); Move to testkit, on the premise that nothing outside the crate builds it (true today), accepting that if wrong: a future live race test in tests/ would need it back behind a feature."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-17ec03-how-does-happenstance-neon-stop-exporting.md
last_reviewed: "2026-10-11"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-17ec03"
question: "How does happenstance-neon stop exporting ProbeThenWriteStore (#78): hide with doc(hidden), gate behind a feature, move to the testkit, or make it test-only?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-11T01:33:03Z"
tree_hash: "ebda45c0d57455f67f7b3790a2c7dad16d7361c8"
recommended: "A"
flip_condition: "a test outside the crate that must construct it"
---

# Test-only

## Context and problem statement

How does happenstance-neon stop exporting ProbeThenWriteStore (#78): hide with doc(hidden), gate behind a feature, move to the testkit, or make it test-only?

A deliberately lost-update EventStore is pub and re-exported at the crate root of a published adapter.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-17ec03`. Anchor: `crates/happenstance-neon/src/lib.rs:186`.

## Considered options

### A · Test-only (chosen)

#[cfg(test)], crate-private; drop the re-export; reword the two intra-doc links to code spans

- Holds if nothing outside the crate builds it (true today).
- If wrong: a future live race test in tests/ would need it back behind a feature.

### B · Feature gate

pub behind a non-default feature like instruments

- Holds if an external crate needs the mutant.
- If wrong: it is still semver surface under 1.x, just opt-in.

### C · doc(hidden)

keep pub, hide from docs, drop root re-export

- Holds if you accept hidden-but-reachable API.
- If wrong: cargo-semver-checks and users still see it as API.

### D · Move to testkit

relocate to happenstance-testkit's mutants

- Holds if testkit could depend on neon.
- If wrong: it cannot: neon dev-depends on testkit, and no adapter may be in testkit's graph.

## Evidence

- `crates/happenstance-neon/src/lib.rs:186`: pub use event_store::{NeonEventStore, NeonReadStream, ProbeThenWriteStore};
- `crates/happenstance-neon/src/event_store.rs:1606-1614`: only callers are two in-crate compile-shape unit tests

## Decision outcome

Chosen option: **Test-only**, the recommended option.

Decider's note: Ryan, 2026-10-11: go with your recommendations

### Consequences

- Good, because it holds if nothing outside the crate builds it (true today).
- Bad, because if wrong: a future live race test in tests/ would need it back behind a feature.

### Confirmation

Revisit when: a test outside the crate that must construct it

## Why this might be wrong

the crate docs argue from it as a pointable call site; test-only keeps the code but not docs.rs visibility

## Provenance

- Decided 2026-10-11T01:33:03Z by human:ryan (user-approved), via chat.
- Raised in session `b9226544-e5b2-5679-8317-bd20a3336161`.
- Verbatim: "How does happenstance-neon stop exporting ProbeThenWriteStore (#78): hide with doc(hidden), gate behind a feature, move to the testkit, or make it test-only?"
