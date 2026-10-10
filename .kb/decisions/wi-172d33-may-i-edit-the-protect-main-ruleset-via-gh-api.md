---
id: "kb-decision-wi-172d33"
title: "I edit it via gh api"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"May I edit the \"Protect main\" ruleset via gh api to drop the live Neon check from required status checks (wi-0f1291 decided B)?\", facing wi-0f1291 decided to make the live Neon check non-required until L8, which needs an edit to the 'Protect main' ruleset, we decided for I edit it via gh api and neglected You edit it in Settings, on the premise that You're fine with me changing repository settings, accepting that if wrong: Revert with one gh api call."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-172d33-may-i-edit-the-protect-main-ruleset-via-gh-api.md
last_reviewed: "2026-10-07"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-172d33"
question: "May I edit the \"Protect main\" ruleset via gh api to drop the live Neon check from required status checks (wi-0f1291 decided B)?"
door: two-way
blast_radius: system
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-07T03:57:31Z"
tree_hash: "cb53b0117974473d5baab3b551a15152f7b07678"
recommended: "A"
flip_condition: "If you keep repository settings changes to yourself"
---

# I edit it via gh api

## Context and problem statement

May I edit the "Protect main" ruleset via gh api to drop the live Neon check from required status checks (wi-0f1291 decided B)?

wi-0f1291 decided to make the live Neon check non-required until L8, which needs an edit to the 'Protect main' ruleset.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-172d33`. Anchor: `runbook/handover.md:112`.

## Decision drivers

- Effort for you

## Considered options

### A · I edit it via gh api (chosen)

One ruleset update removing only the Neon context; reversible the same way; restore recorded as part of L8's exit.

- Holds if You're fine with me changing repository settings.
- If wrong: Revert with one gh api call.

### B · You edit it in Settings

You remove the check in Settings, Rules; I add the handover note.

- Holds if You prefer to make settings changes yourself.
- If wrong: Nothing lost; slower.

## Evidence

- `gh api rulesets`: Protect main (id 22926481), required_status_checks includes 'conformance against a live Neon endpoint'
- `.kb/_intake/decisions/wi-0f1291-*.md`: decided B: non-required until L8

## Decision outcome

Chosen option: **I edit it via gh api**, the recommended option.

Decider's note: Edit the Protect main ruleset via gh api, removing only the Neon context.

### Consequences

- Good, because it holds if You're fine with me changing repository settings.
- Bad, because if wrong: Revert with one gh api call.

### Confirmation

Revisit when: If you keep repository settings changes to yourself

## Why this might be wrong

A ruleset edit by an agent is a settings change you might want to see happen

## Provenance

- Decided 2026-10-07T03:57:31Z by human:ryan (user-approved), via chat.
- Raised in session `f7fec0fb-352c-4bd4-92ac-d997606ea87e`.
- Verbatim: "kind: question"
