# #139: SY-14, VT-24, VT-21 and SY-18, with bulk ingest measured against the Neon peer

## Goal

Land SY-14's named rule, `bulk_ingest_is_idempotent_in_bounded_round_trips`,
in the ingest family, with wrong stores that fail it and nothing else. It
batches by VT-24's floor: one call of `MIN_SUPPORTED_EVENTS_PER_BATCH` foreign
events. Pin the Neon peer's round-trip count for Kestrel Rotor's 1,840 events
deterministically over the scripted transport. Measure the same thing live on
CI's `live-neon` job, as an `#[ignore]`d library test that prints greppable
numbers. Establish what VT-21 and SY-18 already have. Write each clause's
proposed marker to the #124 notes file. **No marker flips.**

## What exists (established before designing)

- SY-12's rule already ingests "one held event and one new one" in one call
  (3 events). SY-14's rule must reject something SY-12's cannot see. That means
  scale and spread: held events past a chunk boundary, in several groups, at
  the floor.
- Every adapter enforces the unique origin index and decides skips inside the
  write (ADR-0027 §6). Neon holds a whole ingest call, compensation included,
  to `MAX_EVENTS_PER_BATCH` = 128 (`ingest.rs` `ingest_write`). Postgres and
  SQLite allow 256, Cloudflare 1,024, and memory has no ceiling.
- `NeonSyncPeer::push` is one request whatever the batch holds: the ingest's
  three statements, which carry two JSON parameters whatever the event count,
  then the watermark and the identity. Pinned for a two-group push by
  `a_push_is_one_request_that_writes_confirms_and_names_the_store`.
  `PeerLimits::minimum().max_batch_events` = 128.
- VT-21: the store side has `store_accepts_the_guaranteed_minimum_payload`,
  rejecting `PayloadCeilingStore` and `TruncatingPayloadStore`
  (happenstance-testkit `mutation_coverage.rs`). The peer side has
  `peer_declares_its_own_limits` line 2, rejecting `BelowTheFloor`
  (`tests/wrong_limits.rs`). It is compared across every mounted peer
  (#121).
- VT-24's append side: `store_accepts_the_guaranteed_minimum_batch_size`
  rejects `BatchParameterCeilingStore` and `ChunkLosingBatchStore`. No ingest
  rule holds the floor to a call.
- SY-18: four rules (`peer_declares_its_own_limits` with the Turnstile
  experiment on `KvCappedFixture`, `ingest_validates_every_event_not_the_head`,
  `ingest_holds_the_call_to_its_tags_per_batch_ceiling` and
  `peer_takes_a_batch_at_its_declared_limits`), each with mutants. #121 and
  #337 built them.
- #379 (`IngestFixture::MAX_EVENTS_PER_BATCH` plus a batch-length rule): read,
  and **not folded**. See D2.

## Constraints and non-goals

- No marker flips. No new dependency. Public API change is additive only: one
  new rule `pub async fn` in a published testkit, which is a minor of its
  number (ADR-0066 §4) and is how every phase-13 rule landed.
- CI: only new `#[ignore]`d `--lib` tests, which the existing live-neon trio
  picks up. No workflow edit.
- Never assert a literal local position. Origin positions are facts the rule
  supplies.
- Existing mutants keep their pinned sets. Only the every-rule identity table
  gains a row.

## Design

- `ingest_suite.rs`: `pub async fn bulk_ingest_is_idempotent_in_bounded_round_trips<F: IngestFixture>(open: impl AsyncFn() -> F) -> Result<RuleOutcome, Violation>`.
  It sits last among the rules and is enumerated last. The arrangement goes in
  a child module `ingest_rules/bulk.rs`, the shape `batch_scope.rs` set.
- Arrangement (origin `0x7D`): `MIN_SUPPORTED_EVENTS_PER_BATCH` foreign events
  at origin positions 1..=128, each with distinct bytes and no tags. They go in
  seven groups of 1, 3, 4, 8, 16, 32 and 64 events. The **first delivery** is
  every third event plus the last, 44 events in one call that keeps each
  event's group. So the first group is wholly held and every other group is
  partly held, with held events past index 64. Then **the bulk call**: all 128
  in one `ingest`. It must be `Ok`, counted `(84, 44, 0)`. Every event is held
  once (found by bytes), every first-delivered row is unchanged (identity,
  time, bytes, position) and the watermark is origin position 128. Then **the
  re-delivery**: the same call is `(0, 128, 0)` and leaves log, head and
  watermark as they were.
- Clauses: a capacity refusal (`IngestError::ExceedsStoreLimit`) of the
  at-floor call is reported as **VT-24**. Everything else is **SY-14**,
  including a failed identity read.
- Mutants (`tests/wrong_ingest_stores.rs`):
  - `LooksUpOneChunkOfIdentities`: the held set is consulted only for the
    first `LOOKUP_CHUNK` = 64 foreign events of a call, as a store does that
    sends one `IN (…)` chunk and forgets to loop. A held event past that is
    written again. Expected set: {bulk → SY-14}.
  - `CapsTheCallBelowTheFloor`: refuses a call of more than 64 events as
    `ExceedsStoreLimit { EventsPerBatch }`. Expected set: {bulk → VT-24}.
- Neon: a new test module, `ingest/tests/bulk.rs`.
  - `rotors_ingest_is_one_request_per_push_however_much_is_held`, scripted.
    The near side is the in-memory ingest store holding Rotor's 1,840 events
    as 46 appends of 40. It is paged by `read_groups(after, 128)`, the
    runner's read. Each page goes through one `NeonSyncPeer::push`: 16 pushes,
    16 requests, each request five statements and constant parameters. The
    replay gives 16 again.
  - `rotors_bulk_ingest_through_the_live_neon_peer_is_bounded`,
    `#[ignore = "needs a live Neon endpoint…"]`. Same loop over a counting
    `LiveTransport`. It seeds every third event first, then pushes the log,
    then replays. It asserts one request per push, the counts, and the 34
    minute bound, and prints `SY14-MEASURE …` lines.

## Test list (drive in this order)

- [x] bulk rule green on `MemoryIngestFixture` (and on the faithful store)
- [x] bulk rule fails `LooksUpOneChunkOfIdentities` exactly, as SY-14
- [x] bulk rule fails `CapsTheCallBelowTheFloor` exactly, as VT-24
- [x] every-rule identity table includes the new rule as SY-14
- [x] every other mutant's pinned set unchanged (whole file green)
- [x] Neon scripted: 1,840 events, 16 pushes, 16 requests, replay 16
- [x] Neon live test compiles, is listed under `--ignored` (CI runs it)
- [x] Postgres live ingest family runs the new rule green (local)
- [x] SQLite spike runs it green (`cargo test -p happenstance-sqlite --lib bulk_ingest`); Cloudflare's mount is wasm32-only and runs in the gate's wasm step where the toolchain is present

## Decisions (H-16 and design)

- D1. Where the floor is read on the ingest path. Options: (a) the call is
  the batch, as VT-36 already says for tags, so a store must take 128 foreign
  events in one call; (b) the group is the batch, so only a 128-event group is
  owed; (c) no floor on ingest. **(a) built.** `PeerLimits::minimum` already
  makes a peer take a 128-event push, and a push is one ingest call. Neon's
  ceiling is exactly 128 per call. (b) lets a store refuse a page of small
  groups that the minimum peer must take. (c) leaves SY-14's pulled batch
  unbounded below.
- D2. #379 is not folded. It holds a *declared* ceiling from both sides plus
  VT-36's reachability. This rule holds the *floor*, with no fixture constant.
  They share an axis, not an instrument. #379 stays open as a follow-up.
- D3. The live measurement is a direct page-and-push loop, not `Edge::run`.
  It counts exactly the round trips SY-14 bounds (ingest per pulled batch),
  without the runner's pulls in the count. The runner path through the Neon
  peer is already exercised in harness/sync-round-trip.
- D4. The rule reports two clauses (VT-24 for a capacity refusal at the floor,
  SY-14 otherwise). Precedent: SY-18's rule cites VT-21 in its floor message.
  Two clause strings make the VT-24 evidence trail exact in `fails_exactly`.

## Risks

- A store with a per-call ceiling below 128 would go red. None ships: Neon is
  128, and the rule's call is exactly 128 with no compensation.
- Live Neon is CI-only. The scripted test pins the count, and the live test
  adds wall clock.

## Progress

- Red 1 (2026-11-09): `cargo test -p happenstance-sync-testkit --test wrong_ingest_stores`
  — 3 failed. `..._fails_a_store_that_looks_up_one_chunk_of_identities` and
  `..._fails_a_store_that_caps_the_call_below_the_floor` panicked at
  wrong_ingest_stores.rs:957 ("`bulk_ingest_is_idempotent_in_bounded_round_trips`
  is not in the enumeration"); `every_rule_reports_a_store_that_cannot_read_its_identity`
  panicked at :1006 (`a rule was added`: 10 rules for 11 rows). The 31 others,
  every existing mutant, green with the two new mistakes compiled in.
- Red 2: with the rule written, `wrong_ingest_stores` — 2 failed.
  `..._fails_a_store_that_looks_up_one_chunk_of_identities`: "`bulk_ingest_is_idempotent_in_bounded_round_trips`
  must reject its mutant, left: Ok(Ran)". The mutant's fallback still deduped
  against the call's working copy, which starts as the whole log; fixed in the
  mutant, which now sees past its chunk only what this call wrote.
  `ingested_events_land_above_the_local_head_fails_a_store_that_places_at_origin_position`:
  the new rule failed SY-5's mutant (Err("SY-14")) because it compared the
  first delivery's rows by position. Fixed in the rule: identity, time and
  bytes, as SY-12's rule compares, leaving position to SY-5.
- Green: `wrong_ingest_stores` 34 passed, both new mutants failing exactly
  the new rule (SY-14 and VT-24), every existing pinned set unchanged.
- Neon scripted `rotors_ingest_is_one_request_per_push_however_much_is_held`:
  green on its first run. It pins behaviour #131 built (one request per push);
  the discriminating red for that claim is #131's `a_push_is_one_request_...`
  against `StoreSyncPeer`. Its own first compile failed on an `as` cast the
  hook flagged (fixed to `try_from`), not on behaviour. The live twin is
  listed under `--lib -- --ignored --list`, as is the Neon family's mount of
  the new rule.
- Postgres: `cargo test -p happenstance-postgres --all-features --lib -- --ignored --test-threads=1 ingest`
  against `HAPPENSTANCE_POSTGRES_URL` — 54 passed, the new rule among them.
- Checks: `RUSTFLAGS=-Dwarnings cargo check` on both touched crates at default
  and `--no-default-features`, wasm32 for neon `--features sync` and the
  testkit; `cargo doc --workspace --all-features --no-deps
  --document-private-items` with `-D warnings`; `cargo xtask spec-trace`
  (after `--write`, two §7.2 rows); `cargo xtask lints`: all clean.
- Gate: first run RED on `No space left on device` (environmental; another
  agent's build filled the disk). After merging origin/phase-13 (7b9c7f80):
  temper gate GREEN, fingerprint 0cceb650bb5541d4 — fmt, scan, manifests,
  clippy, test, doc all ok. spec-trace and lints re-run clean after the merge.
  The gate's advisories are all on lines older than this branch (base
  f6f7e147): none is in code this change wrote.
