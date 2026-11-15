---
id: "kb-decision-773f57865d"
title: "Additive: Edge<R, A = Accept>, sibling of serve_sync"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"#450: how does the adjudication hook attach without breaking 0.6 callers: defaulted type parameter Edge<R, A = Accept> + with_adjudicator, push side through a new sibling of serve_sync rather than a new type parameter on it, additive only?\", facing #450's hook lands on happenstance-sync's published runner and serve_sync, we decided for Additive: Edge<R, A = Accept>, sibling of serve_sync and neglected Break serve_sync in 0.7.0, on the premise that semver-checks accepts the defaulted param, accepting that if wrong: fall back to step_with/run_with methods."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-c906860753"
question: "#450: how does the adjudication hook attach without breaking 0.6 callers: defaulted type parameter Edge<R, A = Accept> + with_adjudicator, push side through a new sibling of serve_sync rather than a new type parameter on it, additive only?"
door: one-way
blast_radius: external
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-15T06:27:41Z"
tree_hash: "739a27ce85f6661f3578d32f6a238bf7f2655c1e"
recommended: "A"
flip_condition: "semver-checks flags the default param"
---

# Additive: Edge<R, A = Accept>, sibling of serve_sync

## Context and problem statement

#450: how does the adjudication hook attach without breaking 0.6 callers: defaulted type parameter Edge<R, A = Accept> + with_adjudicator, push side through a new sibling of serve_sync rather than a new type parameter on it, additive only?

#450's hook lands on happenstance-sync's published runner and serve_sync.

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-c906860753`. Anchor: `crates/happenstance-sync/src/runner.rs:180`.

## Considered options

### A · Additive: Edge<R, A = Accept>, sibling of serve_sync (chosen)

Defaulted type param + with_adjudicator; new sibling fn for the push side; spike shows Edge<R> callers compile unchanged

- Holds if semver-checks accepts the defaulted param.
- If wrong: fall back to step_with/run_with methods.

### B · Break serve_sync in 0.7.0

serve_sync gains a type param, with a trace row

- Holds if one entry point matters more than compatibility.
- If wrong: turbofish callers break on a minor.

## Evidence

- `crates/happenstance-sync/src/runner.rs:180`: pub struct Edge<R> {

## Decision outcome

Chosen option: **Additive: Edge<R, A = Accept>, sibling of serve_sync**, the recommended option.

### Consequences

- Good, because it holds if semver-checks accepts the defaulted param.
- Bad, because if wrong: fall back to step_with/run_with methods.

### Confirmation

Revisit when: semver-checks flags the default param

## Why this might be wrong

two serve functions to document

## Provenance

- Decided 2026-11-15T06:27:41Z by human:claude (user-approved), via cli.
- Raised in session `456d08b0-9a08-5fef-92d6-6305f2ca971f`.
- Verbatim: "#450: how does the adjudication hook attach without breaking 0.6 callers: defaulted type parameter Edge<R, A = Accept> + with_adjudicator, push side through a new sibling of serve_sync rather than a new type parameter on it, additive only?"
