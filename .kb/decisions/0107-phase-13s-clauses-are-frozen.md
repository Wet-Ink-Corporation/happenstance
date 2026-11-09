---
id: kb-decision-0107
title: Phase 13's clauses are frozen or re-dispositioned, and every deferred SY clause is settled or renewed
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0107
reversibility: low
phase: 13
supersedes: null
superseded_by: null
summary: >-
  Proposed on 2026-11-09 for #124 and #128; every freeze in it is the owner's call (Ryan), and
  nothing here binds until he accepts it. Fourteen clauses freeze on rules that run on phase-13
  and wrong implementations they fail: VT-6, VT-9, VT-21, VT-24, SY-14, SY-18, SY-20, SY-22,
  SY-23, SY-27, SY-28, SY-30, SY-31 and CF-40. SY-14 freezes on the live Neon measurement its
  marker was deferred for: Kestrel Rotor's 1,840 events in 16 one-request pushes, 1,583 ms. SY-7
  and SY-10 are renewed together until the runner's adjudication hook exists, SY-29 is confirmed
  renewed past 1.0, and SY-32 and WF-1 are renewed against their named experiments, so no
  deferred SY clause is left unowned. VT-36 moves to freeze-by-14, because the plan's
  freeze-by-21 is refused by the ledger lint. SY-35's Rule line is repaired so that the
  convergence family is outside the_sync_suite_never_decodes (#431). Four further freezes, SY-36,
  SY-37, SY-38 and CF-42, have complete evidence and are proposed in their own sections, each
  needing the owner's separate yes. CF-40 records the budget unit ADR-0043 left open: the field,
  not data plus metadata.
depends_on:
  - kb-decision-0026
  - kb-decision-0027
  - kb-decision-0066
  - kb-decision-0086
  - kb-decision-0098
  - kb-decision-0104
related:
  - kb-decision-0003
  - kb-decision-0043
  - kb-decision-0093
  - kb-decision-0099
  - kb-decision-0100
  - kb-decision-0102
  - kb-decision-0105
source_paths:
  - spec/SPECIFICATION.md
  - runbook/ledgers.md
  - runbook/phases/13-sync.md
  - crates/happenstance-sync/src/lib.rs
  - crates/happenstance-sync-testkit/src/ingest_suite/ingest_rules/topology.rs
  - crates/happenstance-sync-testkit/tests/wrong_ingest_stores.rs
  - crates/happenstance-sync-testkit/tests/wrong_peers.rs
  - crates/happenstance-sync-testkit/tests/wrong_scopes.rs
  - crates/happenstance-sync-testkit/tests/wrong_limits.rs
  - crates/happenstance-sync-testkit/tests/wrong_projections.rs
  - crates/happenstance-sync-testkit/tests/wrong_restores.rs
  - crates/happenstance-sync-testkit/tests/wrong_watermark_stores.rs
  - crates/happenstance-neon/src/event_store/sync_peer.rs
last_reviewed: 2026-11-09
---

# Phase 13's clauses are frozen or re-dispositioned, and every deferred SY clause is settled or renewed

**Proposed on 2026-11-09 for #124 and #128.** Every freeze is the owner's call, and this record
binds nothing until Ryan accepts it; on acceptance its `status` becomes `accepted` and the branch
that carries it merges. Sections 1 to 4 are the record as a whole. Section 5 holds four more
freezes whose evidence is complete, and **each of those needs its own yes**: they sit in the
branch's second commit, one hunk per marker, so any of them can be declined without touching the
rest. No long-form record exists; this atom is the record.

## Context

ADR-0066 §2 gave phase 13 the `freeze-by-13` rows of `runbook/ledgers.md`, and phase 13's exit
criteria require each one `[FROZEN]` or re-dispositioned by a record, and every `[DEFERRED]` SY
clause settled or renewed against a named experiment (CF-38). The ledger lint refuses a
`freeze-by-13` row once phase 13 reads `done`. Wave 3 of the 0.6.0 plan built a rule and its
wrong implementations for each clause (#133, #134, #136, #137, #139, #140, #141, #143), and each
implementer proposed the marker text this record adopts. All of that code is on `phase-13` at
`ead54869`. The live evidence is CI's: `live-neon`, `live-postgres` and `workerd`, all green on run
42547020310 (#446, head `d9a6ff28`).

## 1. Frozen

Each marker restates what reopens it. From `1.0.0`, loosening any of these is a major.

1. **VT-6.** `restored_peer_does_not_reissue_identities` (#133): write, back up, write, restore and
   run the adapter's documented post-restore procedure, write, and no `EventId` names two events
   across the two pulls. Green on `MemoryRestoreFixture` and on the Postgres `StoreSyncPeer` with
   `remint_identity` (ADR-0086); red on Postgres with the re-mint skipped, where the store reissued
   `<id>:2` to two events, which is the harm the marker named, measured. `tests/wrong_restores.rs`
   fails a device-keyed identity, a restore that restores nothing and a stamp-on-read incarnation.
   **Scope, stated:** Neon and the Durable Object are not mounted. Neon follows the same procedure
   with no restart; the Durable Object's point-in-time restore is ADR-0086's "not decided here".
   Mounting Neon in `live-neon` is a follow-up, not a condition.
2. **VT-9.** `ingest_preserves_the_recorded_time` (#134) fails six wrong ingest stores, one of
   them clamping to its own clock, and runs in every ingest family, the Durable Object's under
   `workerd` included. The frozen-clock falsifier stays retired.
3. **VT-21.** `store_accepts_the_guaranteed_minimum_payload` and the floor line of
   `peer_declares_its_own_limits`, which `BelowTheFloor` fails alone. The peer set compares
   65,536 (`MemorySyncPeer`, `NeonSyncPeer::new`) < 114,465 (`KvCappedFixture`) < 131,072 (the Neon
   store) ≤ 1,048,576 (SQLite, Postgres, Cloudflare). `NeonSyncPeer::new` declares exactly the
   floor by choice; `with_limits` may declare up to the store's ceilings.
4. **VT-24.** The ingest-path sentence (D1, approved under the standing AFK authority on
   2026-11-09) is already in the clause body: on the ingest path the batch is the call.
   `bulk_ingest_is_idempotent_in_bounded_round_trips` fails `CapsTheCallBelowTheFloor` under this
   clause alone. The refusal *beyond* a declared ingest ceiling is #379's rule, additive, and not a
   condition: the append side's refusal is held by `append_reports_exceeded_store_limits`.
5. **SY-14.** Measured, as ADR-0027 held the marker for. `live-neon` run 42547020310 printed:
   seed 614 events in 5 pushes, 446 ms; bulk, Kestrel Rotor's 1,840 events in 46 groups of 40 at
   128 events a push, **16 pushes, 16 requests**, 1,226 appended and 614 skipped, 1,583 ms (98 ms a
   request); replay 16 pushes, 16 requests, 1,840 skipped, 1,362 ms; `rows_held=1840`. Against the
   34-minute window (2,040,000 ms) that is about 0.08%. The scripted twin pins the count with no
   endpoint, and the port rule fails `LooksUpOneChunkOfIdentities`.
6. **SY-18.** Four rules: `peer_declares_its_own_limits` (#121), eight lying peers in
   `tests/wrong_limits.rs`; `peer_takes_a_batch_at_its_declared_limits` (#337);
   `ingest_validates_every_event_not_the_head` (#242); and
   `ingest_holds_the_call_to_its_tags_per_batch_ceiling` (#370). The Turnstile experiment ran on
   `KvCappedFixture`, a fixture peer, which the phase-13 work list sanctions ("need not be a real
   KV binding"): a 340,000-byte map against a 131,072-byte value, declared 114,465, is parked by
   the runner with no round trip and nothing written. That is prevention, not relocation. The
   shipped runner reads `limits()` before every push.
7. **SY-20.** `convergent_projection_is_interleaving_independent` (#141), with unit-ledger,
   first-arrival, arrival-rank and declared cost-layers folds failed beside an `EventId`-ordered
   control. See open point A on the clause's width.
8. **SY-22.** `non_convergent_projections_are_excluded_not_failed` (#141): cost layers diverges
   where stock-on-hand converges, and the `compile_fail` fence holds the exclusion.
9. **SY-23.** `event_id_order_is_identical_on_every_peer` (#136) and `happenstance-core`'s
   derived `Ord` on `StoreId` and `EventId`. Its mutant, a peer deduplicating by the highest
   origin position seen, breaches SY-12 too.
10. **SY-27 and SY-28.** `round_trip_preserves_the_whole_log` (#140): whole-log equality over
    `EventId` sets, byte for byte, no position compared; `tests/wrong_scopes.rs` withholds a slice
    by tag, by type, and by both, each failing that rule alone. A firing is answered by scoped
    replication as an additive opt-in with its own ADR.
11. **SY-30.** `push_envelope_preserves_group_boundaries` (#420) on the Durable Object over HTTP
    and on `NeonSyncPeer`; `ingest_group_is_atomic_under_a_mid_group_fault` (#242), armed on both;
    and the shipped runner pushing a SQLite log's write groups whole, with guards, through the
    Durable Object, `NeonSyncPeer` and the Postgres `StoreSyncPeer`, where the pre-#420 per-event
    push fails the same check (#143).
12. **SY-31.** `watermark_advances_transactionally_with_a_read_model` (#143) plays
    `runner::Watermark` over a projection fixture, fails four wrong projection stores, and is green
    on the reference store, SQLite, both Postgres stores and Neon. `Edge::watermark` is `None` until
    the far side names its store (ADR-0104); the pull token is not persisted, so a restored edge
    pulls from the start and ingest skips what it holds (SY-11). **Accepted gap:**
    `happenstance-cloudflare` has no `ProjectionStore`, so an edge hosted in a Durable Object has
    nowhere to keep its watermark through the port today. That is an adapter gap and not the
    falsifier (the object's SQLite could carry one); a Cloudflare projection store is a follow-up.
13. **CF-40.** The unit ADR-0043 left open is decided field-wise (#143): each byte ceiling bounds
    its own field and holds with the other at its own ceiling, so `ReplicatedEvent::payload_len`,
    data plus metadata, is the replication port's unit and never a store's. Alternatives:
    `EventDataLen` meaning data plus metadata, rejected because it changes a published variant's
    meaning and refuses bytes that land today when split differently (ADR-0043's own reason); and a
    sixth, shared ceiling now, rejected because nothing needs it, and kept as the falsifier's
    remedy. `append_reports_exceeded_store_limits` fails seventeen mutants,
    `SharedPayloadBudgetStore` among them. Every real adapter states constant ceilings.

## 2. Renewed

1. **SY-7 and SY-10, together, to `freeze-by-14`.** SY-7's rule holds only the store half (a
   compensation is minted under the identity the store holds). Its MUST, that a non-adjudicator
   writes none, binds a runner adjudication hook that does not exist, so no wrong runner can fail
   it yet; the owner decided on 2026-11-08 that SY-7 stays provisional. SY-10 meets the same hook:
   `runner::Edge`'s pull always accepts, and the hub's rule lives in the far side's push handler, so
   the per-direction rule holds today only when the spoke drives the edge. A hub that drives its
   own edge toward a spoke cannot adjudicate what it pulls. Both markers move when the hook mounts
   `only_the_adjudicator_compensates` and `directional_merge_rules_compose`. The hook is a
   follow-up to #244, to be filed. **#441 must be settled before SY-10 freezes**: its refused-push
   check can never fail, and the recommendation is to delete it and say in the rule's rustdoc that
   SY-34's refusal is the runner's half, held by `happenstance-sync/tests/runner/directions.rs`.
   Finding kept for the hook: a spoke's local append travels with no guard (a local condition is
   position-relative, SY-6), so a hub adjudicates by fact family (SY-35) from what it holds.
2. **SY-29, confirmed past 1.0.** SY-27 is frozen, so no 1.0 port method takes a peer-supplied
   `Query` and this MUST binds nothing on the 1.0 port. Reopened by the first scoped-replication
   proposal, whose named experiment is unchanged: a query that satisfies any tag-based policy over
   `Query`'s surface and still scans the log puts the seam in the adapter.
3. **SY-32, confirmed to phase 14 (#128).** Neither phase-13 peer built CF-27's instrument, every
   shipped peer declares `retention_floor: None`, and the runner reads none. The experiment stays
   the marker's: a peer offline 120 days against a 90-day window (#158, #163).
4. **WF-1, confirmed past 1.0 (#128).** The experiment is ADR-0026's: a `happenstance-dcb-interop`
   bridge reading a UmaDB server's log over its published `umadb.proto` gRPC service.

After this record, the only `[DEFERRED]` SY clause is SY-32, and it names its experiment.

## 3. Re-dispositioned

- **VT-36 to `freeze-by-14`.** Not sync work. The `workerd` job ran the 262,144-tag branch of
  `append_reports_exceeded_store_limits` green on the deployed object on run 42547020310 and on
  the runs before it, and `ingest_holds_the_call_to_its_tags_per_batch_ceiling` holds the call
  under `workerd`. What is not shown is a runner `Split` on `TagsPerBatch` through a real push.
- **Why 14 and not 21.** The plan's agreed default was `freeze-by-21`. `cargo xtask lints` refuses
  it: a `freeze-by-N` must name a phase 1.0 waits on, minus 21 itself, which is the audit. Phase 14
  is the last non-documentation phase before 1.0. SY-7 and SY-10 take the same row for the same
  reason.
- **SY-39 is not moved.** Its minting record, ADR-0105, is itself proposed. It stays
  `freeze-by-13` and freezes with ADR-0105's acceptance, or is re-dispositioned then.

## 4. SY-35's Rule line (#431)

SY-35 is `[FROZEN]`, and its `Rule:` line named an unwritten rule under which *every* rule of the
sync suite passes on undecodable payloads. The convergence family cannot: its fold is the
projection under test, which decodes through `run_projection` as any application projection may.
What SY-35's normative sentence lists, a convergence check, reads read-model bytes only, so the MUST
holds. The repair scopes `the_sync_suite_never_decodes` to the peer, ingest, restore and
write-group families and names the convergence family as outside it. The MUST sentence is
byte-identical. #431 closes on this record.

## 5. Proposed freezes that each need the owner's separate yes

These are the owner's explicitly (plan, "held for Ryan: every freeze, SY-36/37/38 explicitly", and
default 6 for CF-42). The evidence for each is complete. Each is one hunk of the branch's second
commit; declining one means reverting that hunk and giving the row a disposition, for which the
fallback is stated.

### 5.1 SY-36: every peer error states its class

`every_refusal_states_its_class` fails four wrong peers in `tests/wrong_peers.rs`: one class for
every failure, a fixture that declares no refusal, a peer that accepts the oversized push, and an
error whose class changes between reads. It reported no skip against `NeonSyncPeer` in `live-neon`
and the Postgres `StoreSyncPeer` in `live-postgres`, and passed under `workerd` against the Durable
Object, on run 42547020310. On each real peer the rule drives one refusal, the oversized push,
refused `Split` before the wire. The falsification test asks for every refusal each real peer can
produce; the rest are classified by each adapter's `Classify`, a method of the error alone
(`NeonPeerError`, the Durable Object's `class_for`, `StorePeerError`, `RemoteError`), and pinned by
unit tests, not by the rule. Walked over those: none needs its retry count or its batch. The
closest call is a platform 404 page from Cloudflare's edge, which the status alone reads as `Stop`
and the client reads as `Transient` by matching the page. That is still a function of the error.
The runner's escalation of a reset Durable Object to a stopped edge is the incarnation check
(ADR-0104), not the class. **Fallback:** `freeze-by-14`, with the rule extended to drive each real
peer's other classes.

### 5.2 SY-37: a pull says what is waiting

`nothing_waiting_means_the_next_pull_is_empty` fails a peer that always says `Nothing` and one that
reads a short page as the end, and reported no skip against `NeonSyncPeer` and the Postgres
`StoreSyncPeer` on run 42547020310, and passed under `workerd`. The falsification test ran by
construction: `NeonSyncPeer` computes `waiting` inside its pull's one request (#131), and
`StoreSyncPeer`, which the Durable Object serves, reads one event past the page in the same
`read_groups` call. Neither needs a second statement. **Fallback:** `freeze-by-14`.

### 5.3 SY-38: an unheld own identity stops ingest

`an_unheld_own_identity_stops_the_ingest` fails `DropsTheClaim`, `ChecksOnlyTheHead`,
`KeepsGroupsBeforeTheClaim`, `RefusesOnlyClaimsThatLand` and `RefusesEveryOwnIdentity`, and is
green on `MemoryIngestStore` and on every adapter implementing `IngestStore`: SQLite, Postgres,
Neon and the Durable Object. On Postgres and Neon its hidden-claim half reports a skip, because a
refused call consumes sequence values, and each crate's own unit test verifies it instead. The first
cost ADR-0098 named, no re-mint on Postgres and Neon, is gone since #104. The falsification test was
walked over the topologies the workspace supports, the runner's echo pull included: a store, neither
restored nor rewound, receives its own identity only for events it holds, which ingest skips. It
amends two frozen clauses (SY-1, SY-33), which is why it is held. **Fallback:** `freeze-by-14`.

### 5.4 CF-42: a declining fixture settles the frontier

Its freeze condition is met and CI holds it on every run: both live jobs fail on any `SKIP` under
`READ_YOUR_OWN_WRITES` (`.github/workflows/ci.yml`), and `frontier_settle` runs behind a held
frontier in `live-postgres`; both green on run 42547020310 and on every phase-13 merge before it.
The meta-tests fail `NeverAsked` and measure `EmptySettle`. Default 6 said the owner is asked when
this evidence is complete. **Fallback:** `freeze-by-14`.

## Open points for the owner

- **A. SY-20's width.** "Any two interleavings that agree on per-origin order and disagree
  everywhere else" admits only the two block orders, so a fold sensitive to alternation and not to
  block order passes. Recommended: freeze as worded. Such a fold is not commutative, which SY-20's
  falsification test already asks of every convergent fold, and widening the MUST is a tightening on
  user projections that would need its own record. Alternative: widen now, add an alternating
  interleaving to the rule in a small testkit change, and freeze after it lands.
- **B. VT-6 on two adapters.** Recommended: freeze on memory and Postgres, as section 1 says, and
  mount Neon as a follow-up. Alternative: mount Neon first.
- **C. SY-18 on a fixture peer.** Recommended: freeze, as the work list sanctions. Alternative:
  renew past 1.0 against a real Workers KV peer, safe because `PeerLimits` is `#[non_exhaustive]`.
  Unchecked here: whether `RemoteSyncPeer` refuses an inadmissible event before its HTTP send, as
  `NeonSyncPeer::admit` and `KvCappedPeer` do.
- **D. SY-31's Cloudflare gap.** Recommended: freeze, and file a Cloudflare projection store.
- **E. CF-25 and #125.** #125's portfolio check is parked on `claude/p13-125`. The `SyncPeer` port
  already carries frozen clauses, so these freezes do not change whether it is walked, but its
  freeze ledger has a row `Transport | SyncPeer | carried by SY-14`, which fails the moment SY-14 is
  frozen. When #125 rebases, that row becomes `far end`: `NeonSyncPeer` passes the sync suite over
  one-shot HTTP in `live-neon`.
- **F. Follow-ups to file:** the runner adjudication hook (SY-7, SY-10); #441's fix; the Neon
  restore mount (VT-6); a Cloudflare projection store (SY-31); #379 (VT-24's ceiling side).

## Consequences

- With section 5 accepted, no `freeze-by-13` row is left except SY-39, which waits on ADR-0105.
  Without it, SY-36, SY-37, SY-38 and CF-42 take their fallbacks before phase 13 reads `done`.
- §1.3 reads 182 `[FROZEN]`, 18 `[PROVISIONAL]` and 5 `[DEFERRED]` after sections 1 to 4, and 186,
  14 and 5 with section 5.
- `crates/happenstance-sync/src/lib.rs` and both sync READMEs list the provisional SY clauses that
  remain.
- The evidence for SY-14, SY-30, SY-36, SY-37, VT-6's live leg and CF-42 is in CI's live jobs, not
  the gate. A regression there turns `live-neon`, `live-postgres` or `workerd` red, not
  `cargo xtask ci`.
