---
id: "kb-decision-9acb214f38"
title: "Rules use shipped hook; runner half via MiniEdge"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"#450: for SY-10's freeze, does 'mounts directional_merge_rules_compose' mean the testkit rules express their merge rules through the shipped Adjudicate/Accept with the runner half proven against a MiniEdge in happenstance-sync/tests, rather than moving the rule to drive a real Edge?\", facing sets the evidence bar for freezing SY-10, we decided for Rules use shipped hook; runner half via MiniEdge and neglected Rule drives a real Edge, on the premise that SY-8 keeps peers out of the ingest family, accepting that if wrong: a reviewer reads it as not mounted."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-907810c4bd"
question: "#450: for SY-10's freeze, does 'mounts directional_merge_rules_compose' mean the testkit rules express their merge rules through the shipped Adjudicate/Accept with the runner half proven against a MiniEdge in happenstance-sync/tests, rather than moving the rule to drive a real Edge?"
door: one-way
blast_radius: system
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-15T05:49:19Z"
tree_hash: "bd0c59e4174da9e30c0f385aa4f0e01a4154b6bb"
recommended: "A"
flip_condition: "you want the conformance rule to drive the runner"
---

# Rules use shipped hook; runner half via MiniEdge

## Context and problem statement

#450: for SY-10's freeze, does 'mounts directional_merge_rules_compose' mean the testkit rules express their merge rules through the shipped Adjudicate/Accept with the runner half proven against a MiniEdge in happenstance-sync/tests, rather than moving the rule to drive a real Edge?

Sets the evidence bar for freezing SY-10.

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-907810c4bd`. Anchor: `runbook/ledgers.md:326`.

## Considered options

### A · Rules use shipped hook; runner half via MiniEdge (chosen)

Testkit merge rules expressed through Adjudicate/Accept; runner half proven in happenstance-sync/tests against wrong runners

- Holds if SY-8 keeps peers out of the ingest family.
- If wrong: a reviewer reads it as not mounted.

### B · Rule drives a real Edge

Move directional_merge_rules_compose into the write-group family

- Holds if the rule itself must run the runner.
- If wrong: removes a rule from ingest_store_conformance! (one-way).

## Evidence

- `runbook/ledgers.md:326`: freezes when the adjudication hook mounts directional_merge_rules_compose

## Decision outcome

Chosen option: **Rules use shipped hook; runner half via MiniEdge**, the recommended option.

### Consequences

- Good, because it holds if SY-8 keeps peers out of the ingest family.
- Bad, because if wrong: a reviewer reads it as not mounted.

### Confirmation

Revisit when: you want the conformance rule to drive the runner

## Why this might be wrong

evidence split across two places

## Provenance

- Decided 2026-11-15T05:49:19Z by human:claude (user-approved), via cli.
- Raised in session `456d08b0-9a08-5fef-92d6-6305f2ca971f`.
- Verbatim: "#450: for SY-10's freeze, does 'mounts directional_merge_rules_compose' mean the testkit rules express their merge rules through the shipped Adjudicate/Accept with the runner half proven against a MiniEdge in happenstance-sync/tests, rather than moving the rule to drive a real Edge?"
