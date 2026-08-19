---
item: "HS-S0102"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — IngestStore and MemorySyncPeer get real bodies

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

The mount point on every row is `crates/happenstance-sync/src/lib.rs` — the crate root is this
library's composition root, per the spec's *Integration contract*. A body that exists but is not
reachable through that root is not mounted, and a `todo!()` that survives it is caught by the
`#![allow(clippy::todo)]` this story deletes.

```yaml
- id: AC-001
  criterion: "P2 stops holding two incompatible identities in their head. GIVEN an adapter author reading `happenstance-sync` to learn what a replicated event *is*, WHEN they follow `ReplicatedEvent` to the type of its `EventId`, THEN there is exactly one `EventId` in the workspace — `happenstance_core::EventId` — with `identity::{StoreId, EventId, RecordedAt}` deleted, `ReplicatedEvent` and `Watermark` retained and re-typed, every `identity::`-spelled site in `src/` and `tests/` revisited, and the crate-root note that explained the deliberate non-re-export (`crates/happenstance-sync/src/lib.rs:149-153`) resolved rather than left describing a state that no longer exists."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/real_peer_shapes.rs::replicated_event_carries_the_contract_crates_identity"

- id: AC-002
  criterion: "P4 watches an event leave one store and arrive in another with its authorship intact. GIVEN two independent `MemoryEventStore`s, each having already appended events of its own, WHEN a batch from the origin is pushed through `MemorySyncPeer`, pulled, and ingested into the receiver, THEN each arriving event is readable from the receiver carrying its **origin's** `EventId` unchanged, and is assigned a *local* position strictly above the head the receiver reported immediately before the ingest — never a position derived from, ordered by, or equal to the origin's."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::foreign_identity_survives_and_lands_above_the_local_head"

- id: AC-003
  criterion: "P2 learns that ingest cannot refuse them for a reason they cannot see. GIVEN an origin `EventGroup` whose `guard` carries an `AppendCondition` that the *receiving* store's state would falsify, WHEN that group is ingested, THEN it lands in full: `IngestStore::ingest`'s signature carries no condition parameter, the body reaches no conditional append path, `EventGroup::guard` is carried as evidence and never consulted, and the only `Err` the adapter can produce is a storage failure — a content or state disagreement is not an available outcome, and every fallible public function's rustdoc `# Errors` says so in those terms."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::a_guard_that_the_receiver_would_falsify_is_still_ingested"

- id: AC-004
  criterion: "P1/P4 never observe a state the origin never had. GIVEN a `PushBatch` of two or more `EventGroup`s where a decision in group *n+1* was only legal because group *n* had landed whole, WHEN a fault is injected mid-group during ingest, THEN no partial group is visible to any reader of the receiving store — the group either lands entire or not at all — and the receiver never re-infers the decomposition from event order. Whichever shape ADR-0026 authorises (group boundary carried into `ingest`, or atomicity composed above a flat `ingest` per `EventGroup`) is the one implemented, and its rustdoc states which."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::a_failed_group_publishes_nothing"

- id: AC-005
  criterion: "P3 reconnects after a dropped edge session and the replay costs nothing. GIVEN a batch already fully ingested by the receiver, WHEN the identical batch is delivered a second time — the normal case at the edge, not the exceptional one — THEN the outcome is `Ingested { appended: 0, skipped: n, last_local: None }`: no second copy, no compensating event, no `Err`; and the skip decision is reached through the store-assigned `EventId` via `contains_event_id`, with `Event::data` and `Event::metadata` untouched on that path. A partial re-delivery (some held, some new) appends exactly the new ones and skips exactly the held ones."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::replaying_the_same_batch_is_a_no_op (with ::a_partial_redelivery_appends_only_the_unheld)"

- id: AC-006
  criterion: "P4 gets the evidence ADR-0003's lift condition asks for. GIVEN an event whose `data` and `metadata` are arbitrary bytes that are not valid UTF-8 and not valid JSON, WHEN it crosses the store boundary and is read back from the receiver, THEN its `Bytes` compare equal to the origin's byte for byte, and no assertion on any path — implementation or test — inspects those bytes structurally."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::the_payload_bytes_are_identical_on_the_far_side"

- id: AC-007
  criterion: "P3 meets the round trip first as a compiled example, not as prose. GIVEN a reader opening `MemorySyncPeer`'s rustdoc, WHEN they read the first example, THEN they see one complete push/pull round trip in which the `MemoryResume` token is held in a **local binding outside the peer** and handed back on the next call — the shape SY-16 requires — and that example is compiled *and executed* by the gate rather than merely rendered, despite the crate's `publish = false` requiring the out-of-package doctest harness RS-62-5 names."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "doctest on `MemorySyncPeer` in crates/happenstance-sync/src/memory.rs, executed through the RS-62-5 out-of-package harness inside `cargo xtask ci --fast`"

- id: AC-008
  criterion: "P2 can trust the resume point they were handed. GIVEN a receiver that has ingested batches from two different origin stores, WHEN `watermark()` is asked what it has seen, THEN the answer is *derived from what was actually ingested* — the highest position observed per origin `StoreId` — so it cannot drift from the log, is monotonic under re-delivery and out-of-order arrival (a replay never lowers it), and `holds()` answers by delegating to `contains_event_id` rather than to any stored side counter."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::the_watermark_is_derived_and_monotonic"

- id: AC-009
  criterion: "P4's \"is it implemented, or is it a skeleton?\" is answered by the compiler. GIVEN the crate after this change, WHEN `cargo clippy --workspace --all-targets -- -D warnings` runs, THEN it is green *with* `#![allow(clippy::todo)]` deleted from `crates/happenstance-sync/src/lib.rs:135-139` — which means no `todo!()` survives anywhere in the crate, including the two stand-ins in `tests/real_peer_shapes.rs:16-20`, whose disposition is settled in this change (the `todo!()`s retired, the finding they recorded kept)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "`cargo clippy --workspace --all-targets -- -D warnings` (plus `rg -n \"todo!\\(\" crates/happenstance-sync` empty)"

- id: AC-010
  criterion: "P3's runtime survives the feature they came for. GIVEN the ingest path as this story leaves it, WHEN the crate is built for `wasm32-unknown-unknown` and when any generic helper this story writes is instantiated, THEN every *bound* names `IngestStore` / `SyncPeer` / `EventStore` and never a `Send` flavour, one name of each pair is in scope per module, no `#[async_trait]` is introduced, and the build is green — `impl SendIngestStore for MemoryEventStore` staying is correct and not a counter-example, because the type is `Send` and the bare flavour comes free."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "`cargo xtask wasm` (the existing wasm32 steps; the dedicated sync step is HS-S0104's)"

- id: AC-011
  criterion: "P4 finds the record still true after the sketch stops being one. GIVEN that implementing this crate falsifies most of `crates/happenstance-sync/src/ingest.rs:20-82` and the header of `tests/ingest_reaches_a_foreign_store.rs`, WHEN a paragraph of that prose is deleted, THEN the finding it held has already been moved into the ADR that consumed it; every `spec/SPECIFICATION.md` clause and test that cites a deleted line is listed in this story's companion record for `frozen-clause-repairs` (HS-S0112); the `memory`-feature divergence from `RUNBOOK.md:4581-4585` is recorded as a decision rather than taken silently; every changed public item carries rustdoc with `# Errors` naming conditions rather than error types; and a `CHANGELOG.md` entry names the change that makes the crate do something."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "`cargo xtask spec-trace` and `cargo doc --no-deps -p happenstance-sync` green, against a non-empty `.bklg/from-contract-to-published-library/replication-identity-and-ingest/ingest-store-and-memory-peer-round-trip/_invalidated-citations.md`"
```
