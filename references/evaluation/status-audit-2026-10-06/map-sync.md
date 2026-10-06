# Map: what is unbuilt or retired

Audit of the worktree at `1f92d08` (== origin/main), 2026-10-06. Read-only. No cargo commands were run.
Paths are relative to the worktree root. Where a statement comes only from a document, it says "docs CLAIM".

---

## 1. `crates/happenstance-sync` (replication)

### 1.1 Size and publish status

| Fact | Evidence |
|---|---|
| 2,900 lines in total: 1,573 in `src/` (6 files), 1,273 in `tests/` (4 files), 54 in the manifest | `wc -l` over `crates/happenstance-sync` |
| `publish = false`, and the manifest description reads "Not yet implemented." | `crates/happenstance-sync/Cargo.toml:3,12` |
| It is the only `publish = false` crate inside the workspace. The other one is the retired ladybug crate. | `grep -n '^publish' crates/*/Cargo.toml` returns 2 lines |
| The crates.io names `happenstance-sync` and `happenstance-sync-testkit` are not claimed. Docs CLAIM a 404 was returned on 2026-09-29. | `runbook/phases/13-sync.md` work item 1; `xtask/src/reserve.rs:120,126` lists both |
| Nothing outside the crate depends on it except `happenstance-sqlite`, as a **dev-dependency** | `crates/happenstance-sqlite/Cargo.toml:115` |

### 1.2 Public items

| Module | Item | Kind | Real or stub |
|---|---|---|---|
| `peer.rs` | `SyncPeer` (bare) / `SendSyncPeer` (through `trait_variant`): `pull`, `push`, `limits` | trait | port definition, `peer.rs:82-158` |
| `peer.rs` | `Pulled<R>`, `PushBatch`, `EventGroup`, `Ack`, `PeerLimits` (`retention_floor: Option<EventId>` at `:301`), `PullBatchLimit`, `SyncError` | types | real data types, `peer.rs:168-393` |
| `ingest.rs` | `IngestStore` / `SendIngestStore`: `store_id`, `ingest`, `watermark` | trait | port definition, `ingest.rs:140-200` |
| `ingest.rs` | `IngestGroup<'a>`, `Ingested` | types | real, `ingest.rs:216,248` |
| `identity.rs` | `ReplicatedEvent`, `Watermark` (a version vector over `StoreId`) | types | real, `identity.rs:46,93` |
| `memory.rs` | `MemorySyncPeer`, `MemoryResume`, `MemoryPeerError` | in-memory peer | **real bodies** for `pull`, `push` and `limits` (`memory.rs:164-232`). Not behind the `memory` feature: `lib.rs:143` declares `pub mod memory;` with no `cfg`. |
| `wire.rs` | `FORMAT_VERSION = 1`, `Envelope<T>` (derived `Serialize`, hand-written `Deserialize`), `check_format_version`, `WireError` | wire envelope | real, `wire.rs:66-359` |
| — | `SyncRunner` | runner | **does not exist**. `grep -rn SyncRunner crates --include=*.rs` returns nothing. |

`PushBatch`, `EventGroup` and `ReplicatedEvent` carry **no** serde derives. Docs CLAIM the message set is phase 13's work (`lib.rs:86-97`).

### 1.3 `todo!()` bodies: count and locations

`grep -rn 'todo!' crates/happenstance-sync` finds:

- **`src/`: 0 `todo!()` bodies.** `lib.rs:135-137` says the last one, the `MemoryEventStore` ingest impl, was deleted at phase 17 rather than finished. The crate has no `#![allow(clippy::todo)]`.
- **`tests/real_peer_shapes.rs`: 5 `todo!()` bodies**, all in deliberate stand-ins:
  - `:110` and `:114`: `DurableObjectPeer` pull and push, in `impl SyncPeer` at `:99`
  - `:194`, `:209` and `:218`: `OneShotHttpPeer`, in `impl SendSyncPeer` at `:198`

They are stand-ins on purpose. They exist so the type checker can test the port's shape, and the tests never run them. The comments at `real_peer_shapes.rs:28` and `:322` say so.

**Stale claim:** `runbook/roadmap.md:19-22`, a snapshot dated 2026-09-28 at `f89e184`, says sync has "four `todo!()` bodies … blocked on `happenstance-core` having no write path that preserves a foreign `EventId`". The code no longer matches:
- `src/` has 0 `todo!()` bodies.
- That blocker was removed by ADR-0073, which froze VT-10 with the rule that the adapter owns its write path (commit `52aa951`).
- `crates/happenstance-sqlite/src/ingest_spike.rs:50` implements `SendIngestStore` for `SqliteEventStore`.

### 1.4 Tests that exist (count of test functions)

| File | What it shows | Evidence |
|---|---|---|
| `tests/real_peer_shapes.rs` | Both stand-in peer shapes satisfy one generic runner signature (a compile-level check). `memory_peer_round_trips_and_dedupes` runs for real. | `:328`, `:350`, `:388` |
| `tests/cursor_shape_probe.rs` | A port shaped like a cursor admits a peer that cannot hold a cursor. This is the probe behind `pull` returning a batch plus an owned token. | `:234` |
| `tests/ingest_reaches_a_foreign_store.rs` | Coherence experiment: a third crate cannot write an `IngestStore` impl for a foreign store | `:191` |
| `tests/wire.rs` | WF-8 envelope version-refusal tests, in postcard and in `serde_json` | `:119`, `:295`; registered as a proof artefact in `xtask/src/proof.rs:433-437` |
| `happenstance-sqlite/src/ingest_spike.rs` | 11 `#[tokio::test]` rules for SQLite ingest. They cover: foreign id and `RecordedAt` preserved, redelivery skipped, events land above the local head, compensation atomic, append conditions ignored, watermark, and a pinned VT-6 breach. | `ingest_spike.rs:189-610` |

The SQLite ingest is **test-only**: `#[cfg(all(test, feature = "event-store"))] mod ingest_spike;` (`crates/happenstance-sqlite/src/lib.rs:121-122`). No shipped code path performs ingest.

### 1.5 What blocks the rest

- **Ordering in the dependency graph.** Phase 13 depends on 5, 8, 9, 10a, 10b, 12, 17 and 18 (`runbook/README.md` status table). Phase 17 is "in progress" and phase 18 is "not started". Phase 18 comes first because SY-20's rule needs the convergence declaration that 18 builds (`runbook/roadmap.md:70-71`).
- **The Postgres/Neon `StoreId` restore gap (VT-6).** Neither store detects a restore, so a restored or branched store can issue an identity a second time. This must be closed before `restored_peer_does_not_reissue_identities` can pass (`13-sync.md`, work item 2; `crates/happenstance-postgres/migrations/0001_event_log.sql:111-118`, `crates/happenstance-neon/migrations/0001_neon_log.sql:139-141` as cited there; not re-verified).
- **ADR-0026 (what a peer is) and ADR-0027 (merge and compensation) are not written.** No file in `.kb/decisions/` or `references/adr/` matches `*0026*` or `*0027*`. They exist only as frozen backlog folders: `.bklg/.../adr-0026-peer-ingest-and-transport` and `.bklg/.../adr-0027-merge-compensation-and-message-set`.
- **CF-25's portfolio check is not built.** It gates the peer-port freeze (`13-sync.md`). Docs CLAIM `xtask/src/spec_trace.rs` has no portfolio logic. Not re-verified.

### 1.6 What phase 13 says must be built (`runbook/phases/13-sync.md`)

Status in the runbook: **not started** (`runbook/README.md`, row 13). Estimate: **12 days** (`13-sync.md`, "Estimate").

| Work item | State in the code |
|---|---|
| Claim `happenstance-sync` and `-sync-testkit` on crates.io | not done |
| Close the Postgres/Neon `StoreId` restore gap | not done |
| ADR-0026, ADR-0027 | not written |
| Real `IngestStore`: turn the SQLite spike into a `sync` feature, ingest for the real peers, a sync-owned in-memory oracle, ingest's own error type | spike only, under `cfg(test)` |
| Settle ES-41's held-versus-visible reading | open |
| `sync_peer_conformance!` in `happenstance-sync-testkit` | **the crate does not exist** (`ls crates` shows no such directory) |
| `MemorySyncPeer` behind a `memory` feature | exists with real bodies, but not feature-gated |
| Use core's `EventId`, `StoreId` and `RecordedAt` | **done** at phase 17 (ticked; ADR-0073) |
| Envelope types on the tested wire format | the envelope is done (phase 5); the message types are not |
| Ingest bound on `EventStore`, not `SendEventStore` | port is bare-plus-Send; runner unbuilt |
| **Two real peers** (Durable Object, Postgres-over-HTTP) plus a SQLite round trip | **not built**; only `todo!()` stand-ins in tests |
| WF-1 interop decision recorded | not done |
| KV-capped peer for SY-18 | not built |
| Filtered-subset store for SY-27/28 | not built |
| Freeze each `freeze-by-13` clause (VT-6, VT-9, VT-21, VT-24, SY-7, SY-10, SY-14, SY-18, SY-20, SY-22, SY-23, SY-27–31, CF-40) | none frozen by this phase |
| Proof artefact: one suite green against three peers, two of them unlike, plus a byte-identical round trip | not started |

### 1.7 SY clause maturity (`spec/SPECIFICATION.md:6856-7912`; index table `:9955-9989`)

| Marker | Count | Clauses |
|---|---|---|
| `[FROZEN]` | 21 | SY-1–6, 8, 9, 11–13, 15–17, 19, 24–26, 33–35 |
| `[PROVISIONAL]` | 9 | SY-7, 10, 20, 21, 22, 23, 29, 30, 31 |
| `[DEFERRED]` | 5 | SY-14, 18, 27, 28, 32 |

**No conformance rule for any SY clause has been written.** In the index table, 32 of the 35 SY rows carry `†` (`grep -c '^| SY-.*†' spec/SPECIFICATION.md` → 32). `†` means the rule name "was looked for … and not found. It must be written" (`spec/SPECIFICATION.md:9753-9757`). The other three rows have no rule name to dagger: SY-8 ("the whole of `happenstance-sync-testkit`"), SY-15 (`peer_conformance!` against a fixture) and SY-17 (the sync testkit compiling). All three point at a testkit that does not exist.

So all 21 frozen SY clauses are frozen on the page, with **no executable check** in the repository. `xtask/src/spec_trace.rs:1012-1013` records why: "`happenstance-sync` is a skeleton and `happenstance-sync-testkit` does not exist".

Note: the SQLite spike's tests exercise the behaviour behind SY-1, SY-2 and SY-5 (ingest ignores conditions, compensation is atomic, ingested events land above the head). They are not the named rules and are not in a suite.

### 1.8 Other claims checked

- **"The whole crate builds for wasm32" (`lib.rs:122-124`): unverified.** None of the explicit wasm32 package steps in `xtask/src/main.rs` names `happenstance-sync`. `grep -n happenstance-sync xtask/src/main.rs` finds only `:1539`, a feature-forwarding lint list. A wasm32 feature-powerset step over the workspace might cover it; that was not checked.
- `README.md:154` lists sync as "🔲 stub, open questions written down", which matches the code.

---

## 2. Retention and forgetting (ES-38/39/40, CF-27, SY-32, ADR-0028)

### 2.1 What is decided

ADR-0028 (`.kb/decisions/0028-what-a-store-may-forget.md`, status accepted, phase 17, commit `004413a`) decides **"the refusal, with an additive reservation. Nothing published changes in `0.4.0`"** (`:62-64`):

1. Deletion, truncation, compaction, redaction and tombstoning stay outside `EventStore` through all of 1.x (`:66-70`).
2. ES-38 says what a store that has been deleted from may look like: no reused positions, and survivors stay unique and monotonic. ES-40's MAY stands: a condition over removed history passes vacuously (`:71-80`).
3. The reservation: if a reader needs a report, it arrives as a **provided** `EventStore` method whose default answers `Unknown`. That is additive. A compiling spike (`experiments/provided-method-spike`) is cited as evidence that cargo-semver-checks treats it as additive (`:81-87`, summary).
4. A floor primitive (`earliest_position`) is rejected (`:88-91`).
5. SY-32's `retention_floor` is kept and means resumability, not completeness (`:234-241`).
6. CF-27's report is local to the instrument: a testkit decorator over an **arbitrary retained set** (`:244-262`).
7. Redaction (E2E-49): a tag cannot be redacted through the port (`14-retention.md`, ticked).

**Is it in the 0.4.0 breaking window?** The decision was **taken** inside the window (phase 17). It was moved there from phase 14 because one possible answer would have added a required method (`14-retention.md`, header notes). The answer chosen is a refusal plus a reservation that is additive only. The window is therefore **not needed** for retention any more, and phase 14's remaining work is additive. ADR-0028 `:93-99` says that if phase 14 slips past 1.0, ES-39 may be renewed past 1.0.

### 2.2 What is open

| Clause | Marker | Rule | Evidence |
|---|---|---|---|
| ES-38 | FROZEN | `positions_are_not_reused_after_removal` † (unwritten) | `spec/SPECIFICATION.md:9901`; `xtask/src/spec_trace.rs:1186-1189` lists it as `Unresolvable::Scheduled` |
| ES-39 | DEFERRED | `a_store_reports_the_history_it_does_not_hold` † | `:9902`; `spec_trace.rs:1191-1194` |
| ES-40 | PROVISIONAL | `condition_over_removed_history_does_not_reject` † | `:9903`; `spec_trace.rs:1196-1199` |
| CF-27 | DEFERRED | `instrument_report_is_accurate_and_the_suite_cannot_tell` † | `:10021` |
| SY-32 | DEFERRED | `retention_gap_is_reported_not_silent` † | `:9986` |

Phase 14 (`runbook/phases/14-retention.md`) is **not started**. Estimate: 5 days. It depends on 13 and 17. Remaining work:
- the completeness instrument
- a removal capability on `Fixture`
- the ES-38 and ES-40 rules
- the reader experiment that decides ES-39
- the 120-day-offline vs 90-day-window case for SY-32

One question stays open deliberately: whether a wiped Durable Object counts as the same store (ADR-0028 `:270-276`). None of the five rules above exists in code. `grep` for the rule names finds them only in `xtask/src/spec_trace.rs`.

**Stale claim:** `runbook/roadmap.md:23-24`, the 2026-09-28 snapshot, still says "Retention has no answer … one of ADR-0028's two answers adds a method to `EventStore`". ADR-0028 has since been decided as the refusal.

---

## 3. SQLite on wasm32 (phase 19)

| Fact | Evidence |
|---|---|
| 19a (skeleton, 3 days) and 19b (adapter, 8–10 days) are both **not started** | `runbook/README.md` rows 19a and 19b; `runbook/phases/19-sqlite-on-wasm.md` has no ticked items and an empty session log |
| No such crate exists | `ls crates` shows 9 directories, none of them for SQLite on wasm |
| `happenstance-sqlite` does not build for `wasm32-unknown-unknown`: `rusqlite`/`libsqlite3_sys` fails, and every read goes through `spawn_blocking`. Docs CLAIM the error was observed on 2026-09-28 at `3916f29`; not re-run here. | `references/seeds/sqlite-on-wasm.md:12-27` |
| The gate does not check this crate on wasm32, and its docs never say "host only" | same, `:23-27` |
| **Outside the 1.0 path.** It adds little to the port's design, but "it may be the offline spoke phase 13 wants" | `19-sqlite-on-wasm.md` "Why here"; `runbook/roadmap.md:58,73-74` |
| The open decisions: driver (`sqlite-wasm-rs` vs `rusqlite`), host and storage (memory / IndexedDB / OPFS), whether it is a new crate, a shared SQL home, and the CI approach (headless browser or not) | `references/seeds/sqlite-on-wasm.md:161-191` |
| The only SQLite on wasm32 that ships is Cloudflare's, inside a Durable Object, and it cannot leave one | seed `:28-35` |

Consequence for the vision: the seed says four of the six scenarios put SQLite on a device (seed `:44-53`; `19-sqlite-on-wasm.md`). Today no store in the workspace can run in a browser or on a device-like wasm target outside Cloudflare.

---

## 4. `crates/happenstance-ladybug` (retired, ADR-0078)

| Fact | Evidence |
|---|---|
| Excluded from the workspace | `Cargo.toml:4` `exclude = ["crates/happenstance-ladybug"]` |
| Kept as a frozen record of 2,180 lines of `.rs`. The first line of the crate root says it is retired. | `crates/happenstance-ladybug/src/lib.rs:1`; `Cargo.toml:12` |
| `lbug` has left the lockfile | `grep -c lbug Cargo.lock` → 0 |
| Retired at commit `9d99339` (PR #33) | `git log -- .kb/decisions/0078-…` |

**Why:** the owner decided on Weigh-In `wi-630032`: "there are just too many issues with it" (`.kb/decisions/0078-happenstance-ladybug-is-retired.md:52-58`). It was already unpublishable, because `lbug`'s build script does not render on docs.rs, and ADR-0066 had already put it outside 1.0 (`:93-101`).

**What the vision loses** (ADR-0078 `:103-150`):
- **A graph read model as a shipped adapter.** It was the only projection store that is not SQL (a Cypher `GraphWriteSet`), and the only evidence that the projection port is not shaped around SQL.
- **The only observation of PS-16's falsifier on an engine that is not SQL.** The ADR says the spread behind PS-16 "is narrower than it was".
- **The only real adapter run under the runtime-free blocking emitter.** The testkit's own `tests/projection_conformance_blocking.rs` now covers it against the reference store only.

No clause changed marker, and the ADR says no clause lost its only instrument (`:151-162`). The `port_shape.rs` evidence was moved to `crates/happenstance-postgres/tests/port_shape.rs`.

**Does anything replace it?** **No.** The ADR's answer is to "write a projection adapter against the frozen port, as `examples/outside-projection-adapter` shows" (`:165-168`). No graph projection store is planned in any runbook phase. Its README row says "⛔ retired" (`README.md:153`).

---

## 5. How much of the replication/offline vision exists in working code

The vision (`crates/happenstance-sync/src/lib.rs:17-70`) has three parts:
- local-first devices that sync with a shared instance, in both peer-to-peer and hub-and-spoke shapes
- a port with adapters, settled against two unlike networked peers
- a conformance suite in `happenstance-sync-testkit`

| Vision component | Exists and runs? | Evidence |
|---|---|---|
| Port traits (`SyncPeer`, `IngestStore`) | yes, as definitions | §1.2 |
| Cross-store event identity (`EventId = (StoreId, position)`) | yes, in core | ADR-0073; `lib.rs:106-109` |
| Versioned wire envelope | yes, tested | `tests/wire.rs`; `proof.rs:433-437` |
| Message-set serialisation (`PushBatch` etc.) | **no** | `lib.rs:86-97` |
| In-memory peer | yes, with one round-trip/dedupe test | `memory.rs:164-232`; `real_peer_shapes.rs:388` |
| Ingest into a real store | **test-only spike** (SQLite, 11 tests) | `happenstance-sqlite/src/lib.rs:121-122` |
| Sync runner (fan-out, pull/push loop, watermark persistence) | **no** | no `SyncRunner` anywhere |
| Sync conformance testkit | **no** | crate absent |
| Networked peers (Durable Object, Postgres/Neon over HTTP) | **no**; type-level stand-ins with `todo!()` | `real_peer_shapes.rs:110-218` |
| Offline on-device store (SQLite on wasm32 or in a browser) | **no** | §3 |
| Retention/compaction across peers | **no**; field only (`PeerLimits::retention_floor`) | `peer.rs:301` |
| Executable SY rules | **0 of 35** | §1.7 |
| Published | **no** | `Cargo.toml:12` |

**Rough estimate: about 10–15% of the replication/offline vision exists as working code.** That share is the data types, the envelope, an in-memory peer and a test-only SQLite ingest. The design side is much further along: 21 of 35 SY clauses are frozen, and the identity and ingest-write-path ADRs are taken. The remaining runtime work has not started: the runner, the testkit, two networked peers, a device store and any executable conformance rules. The runbook estimates it at phase 13 (12 days) + phase 14 (5 days) + phase 19a/19b (11–13 days, optional for 1.0). Phase 18 (5–8 days) comes before 13 (`runbook/roadmap.md:70-74`). The 10–15% figure is a judgment weighted by component, not a measurement.
