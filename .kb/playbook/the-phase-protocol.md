---
id: playbook-the-phase-protocol
title: "Playbook: the phase protocol, and what counts as proof"
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  A phase is done when its proof artefact exists and every exit criterion is ticked — not
  when the gate is green. `cargo xtask ci` being green is a precondition for LOOKING at the
  exit criteria, never one of them. Sequence by blast radius, write the ADRs before the code
  they constrain, and settle a design against something that compiles.
depends_on: []
related:
  - concept-proof-artefact-vs-green-gate
  - playbook-freezing-a-port
  - roadmap-the-runway
source_paths:
  - docs/RUNBOOK.md
last_reviewed: 2026-08-09
---

# The phase protocol

## The session loop

1. Read the status roll-up, then `git log --oneline -10`.
2. Run `cargo xtask ci`. Establish the baseline is green **before touching anything**.
3. Pick the first phase that is not done and whose dependencies are done.
4. Write the phase's ADRs first. Then the code they constrain.
5. Build the phase's proof artefact. If you cannot, the phase is not done — say so rather
   than ticking the box.
6. Re-run the gate. Tick the exit criteria. Add a dated session-log line.
7. Commit the code and the record together.

## The four principles that survived the rewrite

- Sequence by **blast radius**, not by artefact.
- A phase's proof is a named **artefact that would not exist if the design were wrong**.
- Settle a port against something that **compiles**, not against an argument.
- **A port frozen against one storage shape is shaped like that shape.**

## Where the authority sits

The specification is current truth; ADRs are history. Changing a `[FROZEN]` clause takes a
new ADR, not an edit. Where a phase body and the specification disagree, the specification
wins and the phase body is fixed in the same commit.
