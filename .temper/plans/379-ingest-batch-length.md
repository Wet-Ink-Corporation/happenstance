# #379: an ingest call is held to its events-per-call ceiling

Cut from `origin/phase-21-prep` at `11d16bb`. Follow-up to #370 / PR #378 (its plan's D1).
The owner accepted the work as additive: a minor for `happenstance-sync-testkit`.

## Goal

A store that declares an events-per-call ceiling on its ingest fixture is held to it. A call
exactly at the ceiling lands. A call one event over is refused as
`IngestError::ExceedsStoreLimit { limit: EventsPerBatch, len: ceiling + 1 }` with nothing written,
whether it goes over across groups or only because of its compensation. VT-36's ingest rule
also checks, before it writes, that its over-ceiling call can be reached under that ceiling.

## Constraints and non-goals

- Additive only. One new defaulted associated const, one new rule, and two new `pub` skip
  constants. Nothing removed or re-typed.
- No new dependency. No change to `happenstance-core` or `happenstance-sync`.
- No clause minted, frozen or unfrozen. VT-24 is `[FROZEN]` and its marker already reads
  "The refusal beyond a declared ingest ceiling is #379's, additive". Only its `Rule:` and
  `Rejects:` lines grow, the way #378 grew VT-36's. The normative paragraph is not touched.
- No floor check in the new rule. VT-24's floor belongs to `bulk_ingest_is_idempotent_in_bounded_round_trips`,
  just as `store_accepts_the_guaranteed_minimum_batch_size` owns it on the append path and
  `append_reports_exceeded_store_limits` does not.
- No live Postgres, Neon or workerd runs. Those mounts get the const and run in their own CI legs.

## Invariants

- I-1. The rule counts every foreign event and every compensation event of one call (VT-24's
  ingest sentence), because all four adapters do: `len = Σ events + compensation`.
- I-2. A refusal must name `EventsPerBatch` and `len == ceiling + 1`. Each refused call carries
  exactly one event too many. This pins `len` the way VT-36's ingest rule does.
- I-3. A refused call leaves the log, head and watermark unchanged, and none of its events held.
- I-4. A declared ceiling at or above `MAX_PROVOKED_BATCH_EVENTS` (4,096) is not exercised (H-13).
- I-5. VT-36's reachable half: when both `MAX_TAGS_PER_BATCH` and `MAX_EVENTS_PER_BATCH` are
  declared, `ceiling + 1` tags must fit in `MAX_EVENTS_PER_BATCH` events of at most
  `MAX_TAGS_PER_EVENT` tags. If not, that is a VT-36 violation, raised before anything is written.
  If the declaration is reachable but the rule's own spread does not fit, the rule reports a skip
  with a stated reason. It does not fail a coherent store.

## Design

- Placement: `happenstance-sync-testkit::ingest_fixture` for the const.
  `ingest_suite::ingest_rules::ingest_holds_the_call_to_its_events_per_batch_ceiling` for the rule:
  it must be a top-level `pub async fn` there, because `no_orphan_ingest_rules` and spec-trace scan
  for that. The arrangements go in a new child module, `ingest_rules/batch_length.rs`, beside
  `batch_scope.rs`, reaching the shared helpers through `super` (ARC-05).
- `const MAX_EVENTS_PER_BATCH: Option<usize> = None;` on `IngestFixture`. It is `Option`, not
  `Capability`: a fact about the store, as its siblings are.
- New `pub const NO_EVENTS_PER_BATCH_LIMIT: &str = "MAX_EVENTS_PER_BATCH"` and
  `NO_EVENTS_PER_BATCH_LIMIT_REASON`, re-exported from `lib.rs`. These mirror
  `NO_TAGS_PER_BATCH_LIMIT{,_REASON}`.
- New `pub const TAGS_PER_BATCH_PAST_THE_CALL_LENGTH_REASON`, the tags rule's skip reason when
  its own spread does not fit under the call length.
- The rule makes three calls on one fresh fixture, after one local event, as VT-36's does:
  1. control: two groups, the second carrying two compensation events, totalling exactly
     `ceiling`;
  2. A: two groups without compensation, each at or under the ceiling, `ceiling + 1` together;
  3. B: one group of `ceiling - 1` foreign events plus two compensation events.
  A declared ceiling under 4 cannot hold the control's shape, and it is far under VT-24's
  floor of 128, so it is reported as a VT-24 violation before anything is written.
- Errors: `Violation` of `VT-24`, the clause whose ingest sentence says the call is the batch and
  that a capacity refusal is `IngestError::ExceedsStoreLimit`.
- Alternatives considered:
  - A class check (`Split`) as a separate assertion. Dropped: `IngestError::class` derives it from
    the limit, so after the limit check it can never fail, and `happenstance-sync`'s own test pins
    the mapping.
  - Failing the tags rule when its spread does not fit. Rejected: that would fail a coherent
    store for the rule's own margin (one tag under per event, plus two compensation events).

## Test list (drive in this order)

- [x] T1. A fixture that declares no events ceiling makes the new rule report a skip with
  `NO_EVENTS_PER_BATCH_LIMIT` (unit, `ingest_suite::tests`).
- [x] T2. A ceiling at `MAX_PROVOKED_BATCH_EVENTS` is not exercised: skip (unit).
- [x] T3. A ceiling under 4 is a VT-24 violation (unit).
- [x] T4. The faithful wrong store declares and enforces 256 and passes every rule, the new one
  included (`tests/wrong_ingest_stores.rs`).
- [x] T5. `CountsEachGroupsLengthApart` fails exactly the new rule, under VT-24.
- [x] T6. `LeavesCompensationOutOfTheLength` fails exactly the new rule.
- [x] T7. `RefusesAtTheLengthLimit` fails exactly the new rule.
- [x] T8. `RefusesTheLengthAfterWriting` fails exactly the new rule.
- [x] T9. `CapsTheCallBelowTheFloor`, under a fixture that honestly declares its 64, still fails
  exactly the bulk rule under VT-24.
- [x] T10. Tags rule: an unreachable declaration (64 tags per call, 8 per event, 7 events per call)
  is a VT-36 violation (unit).
- [x] T11. Tags rule: a reachable declaration whose spread does not fit (64 / 8 / 9) is a skip with
  the new reason (unit).
- [x] T12. `every_rule_reports_a_store_that_cannot_read_its_identity` lists the new rule (VT-24).
- [x] Adapter fixtures: SQLite spike (256), Postgres (256), Neon (128), Cloudflare shim (1,024),
  workerd (mirrored).
- [x] Spec: VT-24 `Rule:` / `Rejects:`. CHANGELOG under 0.8.0 Unreleased. `lib.rs` table.
  `CLOUDFLARE_UNIT_TESTS` in `xtask/src/proof.rs`.

## Decisions (H-16)

| Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|
| Public API: new defaulted const and three new `pub` consts in sync-testkit | additive minor | additive | owner (issue #379, "accepted as additive") | done |
| `CapsTheCallBelowTheFloor` now meets a fixture that declares its cap of 64 | (a) keep `WrongFixture` and widen the test to `fails_exactly_these` with the new rule; (b) give the mutant an honest wrapper fixture, keeping the assertion `fails_exactly(bulk, VT-24)` unchanged | (b): the assertion is untouched, and the mutant isolates the floor mistake from a declaration mistake | implementer, recorded for review | (b) |
| Adding the new rule to `[FROZEN]` VT-24's `Rule:` line | edit the Rule line only / write an ADR | Rule line only: the frozen marker names #379 as additive, and no normative sentence changes | implementer, flagged for review | Rule line only |

## Risks

- Neon's ceiling is 128, exactly VT-24's floor. The control is a 128-event call with two
  compensation events. Neon counts compensation, so this matches its declaration. It runs only on
  `live-neon`, and is not verified here.
- Cloudflare at 1,024 events: the control call takes about a second on the shim. Acceptable.

## Progress

- Red 1 (T1-T3, T10, T11): `cargo test -p happenstance-sync-testkit --lib events_per_batch` gave
  `error[E0425]: cannot find function ingest_holds_the_call_to_its_events_per_batch_ceiling` and
  `error[E0438]: const MAX_EVENTS_PER_BATCH is not a member of trait IngestFixture`.
- Red 2 (T10, T11, checks disabled, then restored): with `ensure_reachable` ignored and
  `fits_the_call` short-circuited, `an_unreachable_tags_per_batch_ceiling_is_a_violation` and
  `a_reachable_tags_ceiling_the_rule_cannot_spread_under_is_a_skip` both FAILED
  (ingest_suite.rs:2382, :2400).
- Red 3 (T4): `--test wrong_ingest_stores` before the wrong store declared a call length. Every
  `fails_exactly` test panicked with `ingest_holds_the_call_to_its_events_per_batch_ceiling failed
  ...'s mutant`, `left: Ok(Skipped { capability: "MAX_EVENTS_PER_BATCH", .. }) right: Ok(Ran)`.
- Red, oracle: `an_unenforced_events_per_batch_ceiling_is_a_violation`. `MemoryIngestFixture`
  declaring 128 is a VT-24 violation ("accepted"), so the rule is not decorative against a store
  that enforces nothing.
- Green: sync-testkit lib 48 + wrong_ingest_stores 45. The SQLite spike runs the new rule (Ran).
- Not run locally: the Cloudflare shim (needs Node 24; this host has 22), Postgres, Neon, workerd.
