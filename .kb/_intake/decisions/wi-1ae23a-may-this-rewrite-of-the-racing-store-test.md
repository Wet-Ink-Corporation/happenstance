---
id: "kb-decision-wi-1ae23a"
title: "Land as-is"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"May this rewrite of the racing-store test instrument land (commit and PR)?\", facing rewriting the instrument that decides whether the concurrency rules can fail is H-16, so the owner decides whether it lands, we decided for Land as-is and neglected Land with a loud backstop; Don't land, on the premise that future rules keep to the precondition, accepting that if wrong: a future rule that holds an idle handle hangs the binary until the CI timeout, naming no rule."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-08"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-1ae23a"
question: "May this rewrite of the racing-store test instrument land (commit and PR)?"
door: one-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-08T04:52:15Z"
tree_hash: "9b8bbd90cc88369685f754a86cb4960bcfe4a6dc"
recommended: "A"
flip_condition: "You want no possible hang over a deterministic verdict"
---

# Land as-is

## Context and problem statement

May this rewrite of the racing-store test instrument land (commit and PR)?

Rewriting the instrument that decides whether the concurrency rules can fail is H-16, so the owner decides whether it lands.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-1ae23a`. Anchor: `crates/happenstance-testkit/tests/mutation_coverage/racers.rs:20`.

## Decision drivers

- Deterministic under load
- Hang risk

## Considered options

### A · Land as-is (chosen)

Census rendezvous with no bound; the precondition is documented.

- Holds if future rules keep to the precondition.
- If wrong: a future rule that holds an idle handle hangs the binary until the CI timeout, naming no rule.

### B · Land with a loud backstop

Add a generous bound whose expiry panics, so the row reads FellOver.

- Holds if a loud hang guard is worth adding a count back.
- If wrong: brings back a load-dependent red: louder than before, but still a flake.

### C · Don't land

Keep the yield-bounded version.

- Holds if the flake is tolerable.
- If wrong: gate runs stay red 3/3 under load.

## Evidence

- `crates/happenstance-testkit/tests/mutation_coverage/racers.rs:36`: module doc: the measurements and the injection experiment (old red 3/3, new green 10/10)
- `crates/happenstance-testkit/tests/mutation_coverage/racers.rs:90`: liveness precondition: every open never-read handle eventually appends or is dropped
- `scratchpad final4c.log`: final4c: 0 red of 20 (affinity F, build jobs 2)

## Decision outcome

Chosen option: **Land as-is**, the recommended option.

Decider's note: Land as-is

### Consequences

- Good, because it holds if future rules keep to the precondition.
- Bad, because if wrong: a future rule that holds an idle handle hangs the binary until the CI timeout, naming no rule.

### Confirmation

Revisit when: You want no possible hang over a deterministic verdict

## Why this might be wrong

a careless future rule hangs CI anonymously

## Provenance

- Decided 2026-10-08T04:52:15Z by human:ryan (user-approved), via chat.
- Raised in session `127a0aae-c5e8-45a1-b019-a198ad8718d2`.
- Verbatim: "kind: question"
