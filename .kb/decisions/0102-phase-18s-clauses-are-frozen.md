---
id: kb-decision-0102
title: Phase 18's eight clauses are frozen, and PS-9's and PS-24's evidence is repaired
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0102
reversibility: low
phase: 18
supersedes: null
superseded_by: null
summary: >-
  Accepted on 2026-11-07 for #127, the owner's call (Ryan, 06:32Z, "As recommended" on Q1-Q8).
  PS-16, PS-18, PS-25, PS-27, PS-30, PS-38 and SY-21, the seven freeze-by-18 rows ADR-0066 gave
  phase 18, are FROZEN, and so is CF-43, minted provisional at #118 and frozen with PS-18. Each
  freezes on a rule that runs and a wrong implementation it fails, and each marker restates what
  would reopen it. PS-16's evidence is corrected: no adapter clears with TRUNCATE, happenstance-
  cloudflare has no projection store, and the typed rebuild runs on SQLite only. PS-9's narrative
  now names the live savepoint test #103 ran, and PS-24's question is answered: neither typed
  runner emits Rebuilding. No MUST moves. From 1.0.0, loosening any of the eight is a major, and
  PS-25's digest, item encoding and separator are a promise.
depends_on:
  - kb-decision-0066
  - kb-decision-0074
  - kb-decision-0075
  - kb-decision-0101
related:
  - kb-decision-0078
  - kb-decision-0082
  - kb-decision-0094
source_paths:
  - spec/SPECIFICATION.md
  - runbook/ledgers.md
  - runbook/phases/18-typed-runner.md
  - crates/happenstance/tests/projection_clauses.rs
  - crates/happenstance/tests/projection_runner.rs
  - crates/happenstance/tests/failure_policy.rs
  - crates/happenstance/tests/fan_out/rules.rs
  - crates/happenstance/tests/fan_out/runs.rs
  - examples/rebuilding-read-models/tests/rebuild_through_reset.rs
last_reviewed: 2026-11-07
---

# Phase 18's eight clauses are frozen, and PS-9's and PS-24's evidence is repaired

**Accepted on 2026-11-07 for #127.** The owner answered the eight questions in #127's plan
himself (Ryan, 2026-11-07 06:32Z, "As recommended"): this record is written accepted (Q1), CF-43
freezes with PS-18 (Q2), ADR-0101 is accepted (Q3), PS-30 freezes without the polling-cost
passes (Q4), PS-16 freezes on corrected evidence (Q5), three tripwires turn round (Q6), #127
closes phase 18 (Q7), and SY-21's sync rule stays phase 13's (Q8). No long-form record exists;
this atom is the record.

## Context

ADR-0066 §2 gave seven clauses the disposition `freeze-by-18`: PS-16, PS-18, PS-25, PS-27, PS-30,
PS-38 and SY-21. #118 minted an eighth, CF-43, whose own marker says it is frozen with PS-18 by
#127 or renewed with it. The ledger lint refuses a `freeze-by-N` row once phase N reads `done`,
so phase 18 cannot close with any of the eight still open. Every past freeze cites a record, and
CF-43's mint is the only marker move that cited none. This is that record for all eight.

## Decision

Each clause moves to `[FROZEN]`. The evidence is in the tree, the rule runs, and a named wrong
implementation fails it. Each marker restates its falsifier so that what reopens it stays written.

1. **PS-16.** `reset_clears_rows_and_checkpoint_together` runs in every projection suite: memory,
   SQLite, both Postgres stores, Neon in its live job, `RecordingProjectionStore` and
   `examples/outside-projection-adapter`. The typed half (#129) is
   `rebuild_through_reset_restores_every_table` and
   `protected_reset_is_refused_and_the_view_keeps_running`, on two SQLite tables. The graph arm
   left with Ladybug (ADR-0078). The runbook's two-statement truncate-then-reset fails the same
   predicate in #129's file. Reopened by a store whose only bulk clear cannot join the checkpoint's
   transaction and whose batch cannot carry a `DELETE` that clears instead. The remedy is
   additive: the adapter declines, or a provided method is added.
2. **PS-18.** HS-P0010's count is taken, at one. `happenstance-sqlite` refuses through
   `SqliteProjectionStore::protect_from_reset`, reading `projection_protection` inside its
   `BEGIN IMMEDIATE`, and its fixture declares `RESET_REFUSAL`, so
   `refused_reset_changes_nothing` runs against a real file. `NoopProtectFixture` fails it. The
   memory, Postgres and Neon stores decline with a stated reason. Reopened by an adapter that must
   refuse and cannot read its mark inside the reset's own transaction. It declines the capability,
   and the port is unchanged.
3. **PS-25.** ADR-0074's derived id, as built at #107: `checkpoint_id`, `name@` and 16 lowercase
   hex digits of a hand-written 64-bit FNV-1a over `Cc`-terminated items (US, RS, GS, FS), goldens
   pinned, a name over 238 bytes refused. `changed_query_starts_a_new_checkpoint` holds it.
   Reopened by a narrowed query common enough that adoption through `begin` and an empty `commit`
   is a burden. An additive adoption helper answers that, and the digest stays where it is.
4. **PS-27.** The seam was built at #101: a provided `on_error`, defaulting to `Policy::Halt`,
   handed the `ApplyFailure` and the batch. `skip_and_record_is_atomic` passes against
   `run_projection` and fails three wrong runners: a swallowed skip, a checkpoint short of the
   skip, and a record in its own batch. It runs through a one-view fan-out too (#112). Reopened by
   a skip whose record cannot ride the batch that advances the checkpoint. On a live batch that is
   a server-refused statement with no application savepoint, which `Policy::Skip` documents and
   #103 ran as its control.
5. **PS-30.** The fan-out runner was built at #112. `panicking_apply_rolls_back` passes against
   `run_fan_out` and fails three catching runners. The shipped runner catches no panic (ADR-0101,
   accepted with this record), so the MUST binds a catching runner, which 1.x may add.
   ADR-0066's `outside-1.0` fallback is conditional on no fan-out runner being built, so it is not
   available. Reopened by a target on which a caught unwind leaves a write set that cannot be
   rolled back before another view commits.
6. **PS-38.** `commit_advances_the_checkpoint` and `fresh_projection_has_no_checkpoint` run in
   every projection suite. `fan_out_resumes_each_view_from_its_own_checkpoint` runs N ids on one
   handle through the typed runner, which is the settlement the ledger asked for. The
   no-lagging-replica obligation is documented on `ProjectionStore::checkpoint` and in
   `happenstance-neon` (ADR-0075). Reopened by a deployment that must answer `checkpoint` from a
   replica that may lag its own `commit`. The obligation would then narrow to the same handle.
7. **SY-21.** `apply` is handed `Delivered<E>`, with `id()` and no position accessor (#98).
   `compile_fail,E0599` fences on `Delivered` and `ApplyFailure`, each beside a compiling control,
   hold the absence. `apply_is_handed_the_origin_identity_not_the_local_position` runs a restored
   log through the shipped runner. Reopened if a convergent projection needs the local position
   for a reason other than ordering. `convergent_projection_cannot_observe_local_position`, the
   same surface through replication, stays phase 13's and is declared scheduled.
8. **CF-43.** As PS-18: the same instrument and the same wrong fixture, and a protection held per
   handle is rejected because the rule resets through a handle opened before the mark. Reopened by
   an adapter whose mark cannot be read inside the reset's transaction. Such an adapter declines
   `RESET_REFUSAL`.

## Evidence repairs under unchanged MUSTs

Changing a `[FROZEN]` clause's text needs a record, and these two changes are recorded here. Each
MUST sentence is byte-identical before and after.

- **PS-9.** Its narrative said the application savepoint pattern had not been run. #103 ran it:
  `a_savepoint_in_apply_lets_skip_record_a_server_refusal`, through `run_projection` against
  PostgreSQL 17.10. The narrative now names that test.
- **PS-24.** Its narrative left open whether the typed runner ever emits `Rebuilding`. Neither
  runner does: `run_projection` and `run_fan_out` both commit `Authority::Live`, and
  `run_projection`'s `# Rebuilding` page points to a second name and a swap.

## The PS-16 correction

The phase file's PS-16 line, written at #129, said the `TRUNCATE` axis is held by the port rule's
runs on Postgres, Neon and Cloudflare. That is wrong on two counts. Every SQL store's
`probe_delete_all` issues `DELETE` (SQLite, both Postgres stores, Neon), and the memory store
clears its map. `happenstance-cloudflare` has no projection store. So nothing in the workspace sits
at the far end of that axis. This is stated as accepted risk, not as a CF-25 portfolio acceptance,
because the axis is not one of CF-25's. A live-Postgres test that resets through a `TRUNCATE` on
the live batch would build that end. It is an optional follow-up and not a freeze condition.

## What 1.0 now promises

- From `1.0.0`, loosening any of the eight is a major.
- PS-25's digest, its item encoding and its `@` separator are part of the promise. Changing any of
  them moves every derived checkpoint id, so it is a major too.
- PS-30 binds any catching runner a 1.x minor adds.

## Alternatives

- **Renew PS-18 past 1.0**, ADR-0066's fallback. Not needed: #118 delivered an adapter.
- **PS-30 `outside-1.0`.** Not available on ADR-0066's own terms, because a fan-out runner ships.
  It would also leave a shipped runner's panic behaviour unpromised.
- **Wait for the polling-cost fan-out passes before freezing PS-30.** They measure ES-32's renewal
  falsifier, reads per poll, not PS-30's MUST. They become a follow-up issue.
- **Renew CF-43 past 1.0.** Allowed, because firing it relaxes an obligation, but it would split
  the clause from PS-18, whose instrument and wrong fixture it shares.

## Consequences

- No `freeze-by-18` row is left in the ledger, and phase 18 can read `done`.
- The three tripwires in `projection_clauses.rs` that pinned `[DEFERRED` now pin `[FROZEN` and this
  record, as PS-33's pins ADR-0031.
- `experiments/polling-cost` stays outside the gate under CF-34. Its fan-out passes are owed to
  ES-32, not to PS-30.
