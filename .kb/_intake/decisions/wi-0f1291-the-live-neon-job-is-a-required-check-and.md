---
id: "kb-decision-wi-0f1291"
title: "Non-required until L8"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"The live Neon job is a required check and flaked on 2 of 3 runs today (the known ES-11/ES-12 race, ADR-0061). Keep it required and re-run until L8's fence lands, make it non-required until then, or pull L8 forward?\", facing the live Neon job is a required check and fails intermittently on a race ADR-0061 already records, so it can block PRs that touch no Neon code, we decided for Non-required until L8 and neglected Keep required, re-run; Pull L8 forward, on the premise that Re-runs cost more than a real Neon regression slipping through, accepting that if wrong: A real Neon break can merge unnoticed."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-06"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-0f1291"
question: "The live Neon job is a required check and flaked on 2 of 3 runs today (the known ES-11/ES-12 race, ADR-0061). Keep it required and re-run until L8's fence lands, make it non-required until then, or pull L8 forward?"
door: two-way
blast_radius: system
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-10-06T21:22:22Z"
tree_hash: "cb53b0117974473d5baab3b551a15152f7b07678"
recommended: "A"
flip_condition: "When L8's ES-11 record lands, re-add the check to the Protect main ruleset and confirm it is required; restore it sooner (back to A) if the race starts masking other Neon failures"
---

# Non-required until L8

## Context and problem statement

The live Neon job is a required check and flaked on 2 of 3 runs today (the known ES-11/ES-12 race, ADR-0061). Keep it required and re-run until L8's fence lands, make it non-required until then, or pull L8 forward?

The live Neon job is a required check and fails intermittently on a race ADR-0061 already records, so it can block PRs that touch no Neon code.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-0f1291`. Anchor: `runbook/handover.md:112`.

## Decision drivers

- Safety
- Cost now

## Considered options

### A · Keep required, re-run

Status quo until L8: re-run the job when it flakes.

- Holds if Flakes stay occasional (about 2 of 7 runs today).
- If wrong: Unrelated PRs keep stalling for re-runs.

### B · Non-required until L8 (chosen)

Drop it from the ruleset's required checks until L8 merges, then restore.

- Holds if Re-runs cost more than a real Neon regression slipping through.
- If wrong: A real Neon break can merge unnoticed.

### C · Pull L8 forward

Do L8's fence next so the race goes away.

- Holds if L8 is small and nothing more urgent is queued.
- If wrong: Phase-17 order changes for a CI nuisance.

## Evidence

- `runbook/handover.md:112`: Live Neon flakes on the ES-11/ES-12 race ... until L8 lands its fence. Re-run it; don't chase it.
- `PR #36 run 37504851570`: failed on read_result_is_stable..., then on query_items_share_one_snapshot, passed on the 3rd attempt
- `PRs #34, #35, #38`: the Neon job passed first time on each

## Decision outcome

Chosen option: **Non-required until L8**, overriding the recommendation (A).

Decider's note: After the L8 sizing (~2-3 days, fence undesigned, CI-only verification, overlap with L7): make the live Neon check non-required until L8 merges, then restore it.

### Consequences

- Good, because it holds if Re-runs cost more than a real Neon regression slipping through.
- Bad, because if wrong: A real Neon break can merge unnoticed.

### Confirmation

Revisit when: When L8's ES-11 record lands, re-add the check to the Protect main ruleset and confirm it is required; restore it sooner (back to A) if the race starts masking other Neon failures.

## Why this might be wrong

If the race is getting more frequent, re-runs become a tax on every PR

## Provenance

- Decided 2026-10-06T21:22:22Z by human:ryan (user-directed), via chat.
- Raised in session `f7fec0fb-352c-4bd4-92ac-d997606ea87e`.
- Verbatim: "kind: question"
