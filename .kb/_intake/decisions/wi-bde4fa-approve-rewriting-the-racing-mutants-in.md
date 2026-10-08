---
id: "kb-decision-wi-bde4fa"
title: "Approve rewriting the racing mutants in happenstance-testkit's mutation_coverage (RacingProbeStore, GlobalVersionStore) to use a deterministic rendezvous (TST-10), so the_concurrency_rules_reject_exactly_what_they_claim stops flaking? It has failed the full gate on main, CI's MSRV job, and several local gates this session.: A: rewrite RacingProbeStore/GlobalVersionStore to a deterministic rendezvous"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Approve rewriting the racing mutants in happenstance-testkit's mutation_coverage (RacingProbeStore, GlobalVersionStore) to use a deterministic rendezvous (TST-10), so the_concurrency_rules_reject_exactly_what_they_claim stops flaking? It has failed the full gate on main, CI's MSRV job, and several local gates this session.\", facing the question an agent raised, we decided for A: rewrite RacingProbeStore/GlobalVersionStore to a deterministic rendezvous."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-08"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-bde4fa"
question: "Approve rewriting the racing mutants in happenstance-testkit's mutation_coverage (RacingProbeStore, GlobalVersionStore) to use a deterministic rendezvous (TST-10), so the_concurrency_rules_reject_exactly_what_they_claim stops flaking? It has failed the full gate on main, CI's MSRV job, and several local gates this session."
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-10-08T04:51:51Z"
tree_hash: "62904db838806d6ee58397ad1f39d294acf908da"
---

# Approve rewriting the racing mutants in happenstance-testkit's mutation_coverage (RacingProbeStore, GlobalVersionStore) to use a deterministic rendezvous (TST-10), so the_concurrency_rules_reject_exactly_what_they_claim stops flaking? It has failed the full gate on main, CI's MSRV job, and several local gates this session.: A: rewrite RacingProbeStore/GlobalVersionStore to a deterministic rendezvous

## Context and problem statement

Approve rewriting the racing mutants in happenstance-testkit's mutation_coverage (RacingProbeStore, GlobalVersionStore) to use a deterministic rendezvous (TST-10), so the_concurrency_rules_reject_exactly_what_they_claim stops flaking? It has failed the full gate on main, CI's MSRV job, and several local gates this session.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-bde4fa`. Anchor: `crates/happenstance-testkit/tests/mutation_coverage.rs:5073`.

## Decision outcome

Chosen option: **A: rewrite RacingProbeStore/GlobalVersionStore to a deterministic rendezvous**.

Decider's note: approved in the sitting

## Provenance

- Decided 2026-10-08T04:51:51Z by human:ryan (user-directed), via chat.
- Raised in session `6ec241b3-3771-51a8-b5a3-78697abb6042`.
- Verbatim: "kind: question"
