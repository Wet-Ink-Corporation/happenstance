---
id: kb-decision-0094
title: PS-39 is frozen on a live Neon run of projection_ids_round_trip_by_bytes
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0094
reversibility: low
phase: 17b
supersedes: null
superseded_by: null
summary: >-
  The owner's call, 2026-10-12 ("all as recommended", wi-378078): schedule PS-39 in phase 17b and
  freeze it once live Neon has run projection_ids_round_trip_by_bytes green. ADR-0082 minted PS-39
  provisional at phase 17, green on memory, SQLite and both Postgres projection stores, with that
  one condition left. It is met. CI's live-neon job, on its own Neon branch, ran the rule green on
  main at 8b52528 (run 38271947822, job 114888156072): 20 of 20 gated projection tests executed
  and passed, the rule itself declining nothing, and 108 of 108 event-store tests besides. Neon is the last projection
  adapter in the workspace to run the rule, so every one has now passed it. PS-39 moves from
  PROVISIONAL to FROZEN. From 1.0.0, a store may not key a checkpoint lossily, and VT-35's bound
  and character set cannot narrow to suit one, without a major release. Reopened by a backing
  store whose key column cannot hold a MAX_PROJECTION_ID_LEN-byte UTF-8 key byte-faithfully and
  cannot be configured to. The proof is the run on main, not this change's pull-request run: a
  diff that touches only prose skips the live-neon job on a pull request, and the merge queue runs
  it again before main moves.
depends_on:
  - kb-decision-0082
  - kb-decision-wi-378078
related:
  - kb-decision-0066
  - kb-decision-0072
  - kb-decision-0082
  - kb-decision-0089
  - kb-decision-wi-378078
source_paths:
  - .github/workflows/ci.yml
  - crates/happenstance-neon/tests/neon_projection.rs
  - crates/happenstance-testkit/src/projection.rs
  - spec/SPECIFICATION.md
  - runbook/ledgers.md
  - runbook/phases/17b-after-the-window.md
last_reviewed: 2026-10-12
---

# PS-39 is frozen on a live Neon run of projection_ids_round_trip_by_bytes

**Accepted by the owner on 2026-10-12** ("all as recommended",
[`wi-378078`](wi-378078-do-93-option-a-on-both-questions-83-queryitem.md), issue #81). That call
scheduled PS-39 in phase 17b and set its freeze for when live Neon had run the rule green. The
freeze is one-way, so the owner merges the change that carries it. No long-form record exists;
this atom is the record.

## Context

PS-39 says a store MUST key a checkpoint on the `ProjectionId`'s exact bytes: any id
`ProjectionId::new` accepts round-trips through `commit`, `checkpoint` and `reset`, and two ids
that differ in any byte are two checkpoints. It is the store's half of VT-35, whose constructor
promises to fold, trim and normalise nothing.

[ADR-0082](0082-projection-id-is-validated-and-sync-is-reserved.md) minted it `[PROVISIONAL]`
at phase 17 (its D6). The rule, `projection_ids_round_trip_by_bytes`, was green then on memory,
SQLite and both Postgres projection stores against a live 17.10. It was not run on Neon locally,
because Neon needs a live endpoint and CI's `live-neon` job is where that endpoint is. ADR-0082
left one condition: freeze once live Neon has run it green. The 1.0 dispositions carried it as
`freeze-by-17b`, and phase 17b's work list never mentioned it. Issue #81 found that gap.

## Decision

1. **PS-39 is `[FROZEN]`.** The condition ADR-0082 set is met. CI's `live-neon` job ran the rule
   on `main` at `8b52528`, as run `38271947822`, job `114888156072`, on 2026-10-12:
   - the job was in branch-per-run mode, on its own Neon branch `ci-38271947822-1`, so no other
     run shared its frontier (#250);
   - `projection_conformance::projection_ids_round_trip_by_bytes ... ok`, with no `SKIP` line,
     so the fixture declined nothing the rule needs. The projection run's three `SKIP` lines, for
     `READS_THROUGH_BATCH` and `RESET_REFUSAL`, belong to three other rules;
   - the vacuity guard read `run-projection.txt: expected 20; executed 20 (passed 20, failed 0),
     ignored 0` and `run-events.txt: expected 108; executed 108 (passed 108, failed 0), ignored 0`,
     and the failure list was empty.
2. **Every projection adapter in the workspace has now run it green.** The adapters are
   memory, SQLite, the buffered and the live Postgres stores, and Neon. `happenstance-cloudflare`
   ships no projection store. Neon matters most because its storage is a server this workspace
   does not control, reached over one-shot HTTP. A key column that folds, pads or re-encodes is
   most likely to show up on a store like that.
3. **Why the run on `main` and not this change's pull-request run.** The plan was to make this
   change's own CI run the proof. That run cannot be: CI's `changes` job marks a pull request
   whose every changed path is prose as `code=false`, and `live-neon` then skips. This change
   touches only `spec/`, `runbook/`, `.kb/` and `CHANGELOG.md`. The run on `main` at this
   change's base is the same job, the same binary and the same rule, and it ran on the tree this
   change edits. A `merge_group` run always sets `code=true`, so the merge queue runs `live-neon`
   again before `main` moves, and as a required check it blocks the merge if the rule is red.
   That run's id is recorded in phase 17b's session log at merge.

## Consequences

- From `1.0.0`, a store that keys a checkpoint lossily breaks a frozen clause. So does narrowing
  VT-35's bound or character set to suit one. Either needs a major release and a superseding
  record.
- The rule stays a projection-family rule, so an adapter written later must pass it as written.
  The mutants ADR-0082 registered in the testkit's `tests/` keep it falsifiable.
- A store keyed on a cryptographic or wide hash of the id stays forbidden and stays unregistered
  as a mutant, as the clause says. No fixed set of ids can show it.
- `live-neon` is a required check again since #75 (ADR-0087's D8, `runbook/handover.md`). A
  later commit that turns the rule red on Neon is therefore refused at the merge queue rather
  than found after it.

## What reopens it

A backing store whose key column cannot hold a `MAX_PROJECTION_ID_LEN`-byte UTF-8 key
byte-faithfully and cannot be configured to. If one is real after `1.0.0`, VT-35 narrows only in a
major release, under its own record.

## Alternatives that lost

- **Wait for a pull-request run of `live-neon` on this change.** Forcing one would mean adding
  a non-prose path to the diff only to wake the job. That proves nothing the run on `main` did not,
  and it games the `changes` filter. Rejected.
- **Renew past 1.0.** This keeps a falsifier open with nothing left to measure on any adapter in
  the tree. It lost because the condition ADR-0082 set has been met.
- **Re-row the disposition only, and leave the freeze for later.** That was the fallback if no
  live run could be shown. A run can be shown, so the fallback is not needed.
