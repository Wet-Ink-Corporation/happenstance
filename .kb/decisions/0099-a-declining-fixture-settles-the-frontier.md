---
id: kb-decision-0099
title: A fixture that declines read-your-own-writes settles the frontier, and the rules that read back their own writes ask it to
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0099
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  Proposed on 2026-11-01 for #338, on the owner's option A (wi-87e964). The 83 named rules that read
  back their own appends read straight after appending, so a store whose reads trail its commits
  (happenstance-postgres, happenstance-neon) passed them only while nothing else on the server held
  pg_snapshot_xmin back. happenstance-testkit gains a provided Fixture::settle_frontier returning
  Settled { Visible, Nothing }. Each read-back rule calls it after its writes when the fixture
  declines READ_YOUR_OWN_WRITES, and reports a skip under that capability when the answer is
  Nothing. The waiting is the fixture's, so no rule waits (CF-33). The Postgres and Neon fixtures
  poll the read path's own predicate with a 30-second deadline. A fixture that supports the
  capability is never asked. No capability is added, and the change is additive. It mints CF-42
  [PROVISIONAL].
depends_on:
  - kb-decision-wi-87e964
  - kb-decision-0095
related:
  - kb-decision-wi-a4ab49
  - kb-decision-wi-fab349
  - kb-decision-0024
  - kb-decision-0042
source_paths:
  - crates/happenstance-testkit/src/contract.rs
  - crates/happenstance-testkit/src/settle.rs
  - crates/happenstance-testkit/src/suite.rs
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-testkit/tests/mutation_coverage/lagging.rs
  - crates/happenstance-postgres/tests/support/mod.rs
  - crates/happenstance-neon/tests/support/mod.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-11-01
---

# A fixture that declines read-your-own-writes settles the frontier, and the rules that read back their own writes ask it to

**Proposed on 2026-11-01 for #338.** The owner chose option A on 2026-11-01 (`wi-87e964`; its
duplicates `wi-a4ab49` and `wi-fab349` record the same answer). The session that wrote this record
built that option. Acceptance is the owner's, and it flips `status` alone. No long-form record
exists; this atom is the record.

## Context

`Fixture::READ_YOUR_OWN_WRITES` was read by the model family alone, which runs a trailing model
against a declining fixture (#79). The named rules did not read it. 83 of them append and then read,
count or exclude what they appended. `happenstance-postgres` and `happenstance-neon` decline the
capability, because a row is visible only once its `xact_id` is below
`pg_snapshot_xmin(pg_current_snapshot())`, and that frontier is the server's (ADR-0024). They
passed those 83 rules only because nothing else held the frontier back under `--test-threads=1`.
`runtime_seam.rs` had already met the lag by hand (#272). `[FROZEN]` CF-33 names a rule that
passes on a quiet machine and fails on a loaded one as worse than none.

The weigh-in assumed #349 had left a settled-frontier helper to reuse. It had not. The trailing
model weakens the oracle instead of moving the frontier, and it is `proptest`-gated.

## Options

- **A.** A provided `Fixture::settle_frontier(&self) -> impl Future<Output = Settled>`, with
  `pub enum Settled { Visible, Nothing }`. The rules settle; a declining fixture with no override
  skips under `READ_YOUR_OWN_WRITES`.
- **B.** A rule-side loop that re-reads until the append is visible. No API change, but the rule
  waits (CF-33), the check is circular, and a lost write hangs.
- **C.** Trailing-form assertions in each rule. No API change, but every presence anchor, count
  and "unchanged after a refusal" check becomes vacuous under lag, which weakens the suite on the
  two stores it exists for (H-12).
- **D.** `require!(F: READ_YOUR_OWN_WRITES)` on all 83. Postgres and Neon stop running them.

## Decision

A.

1. **The hook.** `settle_frontier` is a provided method at the end of `Fixture`, beside
   `arm_held_above_frontier`, with the same RPITIT shape, so the `!Send` and wasm32 paths are
   unchanged. Its provided body answers `Settled::Nothing` and does nothing. An override waits until
   every row the store holds is visible to every later `read` and `head` through any handle, then
   answers `Visible`.
2. **Who calls it.** A crate-private `settle!` in `src/settle.rs`. It branches on the `const`
   `F::READ_YOUR_OWN_WRITES` first, so a supporting fixture is never called and its run is
   bit-identical. On `Nothing` it returns the skip `require!` would produce for that capability,
   with the fixture's own reason, so no new constant is needed (D2).
3. **Where.** After the rule's writes and before it reads them back, never while an append of its
   own is suspended, and never after `arm_held_above_frontier`. A rule that reads during a lag on
   purpose (ES-10's interleaving, a concurrent reader's mid-flight reads) settles only before the
   read that anchors it.
4. **The adapters.** The Postgres and Neon fixtures poll `NOT EXISTS (… xact_id >= pg_snapshot_xmin(
   pg_current_snapshot()))` over their own schema's `event`, with a doubling pause from 1 ms to
   100 ms and a 30-second deadline that panics naming the probe (D3 a). A deadline answering
   `Nothing` was rejected: it would report a stuck server as a skip.
5. **The clause.** CF-42 `[PROVISIONAL]` in §6.3 states the obligation and names its rules.

## Evidence

In `crates/happenstance-testkit/tests/mutation_coverage/lagging/`, over `LaggingFixture`, which
wraps any registered fixture and hides every acknowledged append from `read` and `head` until a
settle:

- `a_conformant_store_behind_a_settling_lag_passes_every_rule`, and its held-row and concurrency
  siblings: red before the settle points (72 rules, then 6 in the concurrency family), green after.
- `an_unsettling_lag_skips_exactly_the_read_back_rules` and
  `the_read_back_inventory_partitions_the_named_rules`: the 83, and only they, skip on the provided
  body; the 19 that read back nothing run.
- `a_fixture_that_reads_its_own_writes_is_never_asked_to_settle`: `NeverAsked`'s `settle_frontier`
  panics, and nothing fails. Red with the `const` short-circuit removed (76 rules).
- `a_settle_that_reveals_nothing_is_caught_at_the_anchors`: an override that lies is caught by
  every read-back rule bar three whose read-back is absence, measured in `EMPTY_SETTLE_UNCAUGHT`.
- `settling_costs_no_mutant_coverage`: every racer, and every registered store a lag can reach, is
  rejected behind the lag as without it, bar three measured `LAG_MASKED` rows. In each the lag
  itself hides the defect: two are ES-10 interleavings the lag makes unobservable, and one is a
  held-row fixture whose missing arming the lag stands in for.
- `crates/happenstance-postgres/tests/frontier_settle.rs`, `#[ignore]`d: the #272 shape on purpose,
  a transaction holding the frontier while a read-back rule runs.

## Consequences

- An additive testkit change, by the `0.4.2` precedent for `arm_held_above_frontier` and `Armed`.
  A downstream fixture declining the capability goes from flaky-green to green with 83 stated skips
  until it overrides the method.
- Both live CI jobs fail on a `SKIP` line under `READ_YOUR_OWN_WRITES`, which libtest alone counts
  as a pass.
- A rule added later that reads back its own append must settle, and must be listed in
  `READS_BACK_ITS_OWN_WRITES`. The partition test refuses a rule on neither list.

## What would make this wrong

A frontier store that cannot state "everything held is visible" as a probe it can repeat, such as a
read replica with no view of the primary's `xmin`. Its fixture could only keep the provided body, and
83 rules would skip against it rather than run. That is CF-42's falsifier.
