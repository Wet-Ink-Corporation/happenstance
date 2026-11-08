# #130 — the Durable Object sync peer, and a native SQLite round trip

Branch `claude/p13-130` off `phase-13` at `45523312`. Plan of record:
`/mnt/project-files/notes/plan-0.6.0.md` item 3 and PR row 7; `runbook/phases/13-sync.md:134`.

Owner-agreed shape (delegation message, 2026-11-08):

- object side: `serve_sync` over the Durable Object's store, mounted on the object's fetch
  handler at `/sync/pull` and `/sync/push`; the reusable piece in `happenstance-cloudflare`
  behind `sync`, mounted in `harness/workerd`'s real object;
- client side: `RemoteSyncPeer` over HTTP `POST` carrying postcard envelopes;
- round trip: a native SQLite store against the Worker, in the `workerd` job's local and
  deployed legs; the test's HTTP client is `hyper`, already in the tree through Neon;
  `hyper` as a dev-dependency is approved;
- a Durable Object reset reaches the runner as a changed responder `StoreId`, which stops the
  edge; a transport failure is `Transient`.

## Goal

The Durable Object is a real sync peer: `sync_peer_conformance!`'s whole family passes against
it inside `workerd` (local and deployed), and a native SQLite store replicates through it over
real HTTP with payload bytes byte-identical and replays changing nothing.

## Constraints and non-goals

- No `#[async_trait]`; the bare flavour only on the object side (the object's store is
  `!Send`). No change to `happenstance-sync`'s port, wire or runner.
- Additive public API in `happenstance-cloudflare`, behind the existing `sync` feature (owner
  approved "the reusable piece"). Nothing outside `sync` changes.
- No new dependency in any `[dependencies]` table. Dev-dependencies of
  `happenstance-workerd-harness` (unpublished), host-only: `happenstance-sqlite` (workspace
  member), `tokio`, `bytes`, and the hyper client stack Neon already uses (`hyper`,
  `hyper-util`, `http-body-util`, `hyper-rustls`, `rustls`). Every one is already a node in
  `Cargo.lock`, so this adds edges and no node. See D-1.
- CI: test invocations added to the `workerd` job only.
- Non-goals: a client `Exchange` that runs *inside* a Worker (a DO-to-DO stub transport);
  the `Send` flavour of the remote peer (#421); runner policy; persisting the edge.

## Design

### Placement (ARC-02)

- `crates/happenstance-cloudflare/src/sync_http.rs` (new, `#[cfg(feature = "sync")]`,
  declared at the end of `lib.rs` below every cited line): the HTTP binding of
  `happenstance_sync::serve_sync` for a Durable Object. Its tests:
  - `src/sync_http/tests.rs`: host-pure (route parsing, status/class table, request cap);
  - `src/event_store/ingest/tests/served.rs` (wasm32, Node shim): the request path end to end
    through `worker::Request`/`Response`, and `sync_peer_conformance!` mounted with
    `emit_sync_wasm`. A child of `ingest::tests` so it reaches `support` and the private
    `model` without widening either.
- `harness/workerd/src/sync_peer.rs` + `src/sync_peer/fixture.rs`: the family's names, its
  runner (through `ingest::dispatch::run_family` with a new `Family::SyncPeer`), and
  `WorkerdSyncFixture`. `src/object/syncing.rs`: the object's routes.
- `harness/workerd/tests/sqlite_round_trip.rs` + `tests/support/http.rs`: the native client
  and the live round trip.

### `happenstance_cloudflare::sync_http` (exact signatures)

```rust
pub type CloudflareSyncPeer = StoreSyncPeer<CloudflareIngestStore, CloudflareIngestStore>;
pub const SYNC_PATH_PREFIX: &str = "/sync/";
pub const ENVELOPE_ALLOWANCE: usize = 1_048_576;
pub fn sync_route(path: &str) -> Option<Route>;              // "/sync/pull" | "/sync/push"
pub fn max_request_bytes(limits: &PeerLimits) -> usize;      // max_batch_bytes + allowance, saturating
pub fn status_for(class: FailureClass) -> u16;               // T 503, Split 413, Park 422, Stop 400
pub fn class_for(status: u16) -> FailureClass;               // inverse; 408/429/5xx Transient, else Stop
pub async fn serve_sync_request(
    peer: &CloudflareSyncPeer,
    route: Route,
    request: Request,                                        // owned: the body is consumed
) -> worker::Result<Response>;
```

- Both halves are `CloudflareIngestStore`, not `CloudflareEventStore` + `CloudflareIngestStore`:
  `WriteGroupStore` is implemented on the wrapper only (`event_store/ingest.rs:575`), and one
  value cloned (two `Rc` bumps) cannot be two different stores, which `StoreSyncPeer`'s docs
  require. Recorded as a deviation under Declined.
- `serve_sync_request`: non-`POST` → 405 (`Allow: POST`); a declared `Content-Length` over
  the cap → 413 without reading; the body read as a stream and refused at 413 the moment it
  passes the cap (H-13); `serve_sync` → 200 `application/octet-stream`; a `ServeError` → its
  class's status, body = the error's text. A body-read failure propagates as `worker::Error`
  (the platform answers 500, which `class_for` reads as `Transient`).
- `class_for` / `status_for` are the one table both ends read; `FailureClass` is
  `#[non_exhaustive]`, so `status_for` maps an unknown class to 400 (`Stop`, the safe side).

### Error classes (SY-36)

| where | failure | class |
|---|---|---|
| object | `ServeError` | its own `Classify` → status |
| object | body over cap | 413 → `Split` |
| client | status ≠ 200 | `class_for(status)` |
| client | connect/TLS/IO error, timeout | `Transient` (owner default) |
| client | reply over its cap | `Stop` |
| client | URI or request build error | `Stop` |
| runner | responder `StoreId` changed after a reset | `StopCause::Incarnation` |

### Harness

- `WorkerdSyncFixture { ingest: WorkerdIngestFixture }`; `Peer = RemoteSyncPeer<ObjectExchange>`;
  `ObjectExchange` builds a real `worker::Request` (`POST https://harness/sync/<route>`),
  hands it to `serve_sync_request` over `StoreSyncPeer::new(store.clone(), store)`, and reads
  the `Response`'s status and bytes; a non-200 is `ObjectExchangeError::Refused { status,
  class, body }`, a `worker::Error` is `Transient`. Limits: `PeerLimits::minimum()` both ends.
  `refusals` = `oversized_push_refusal`.
- Routes: `/sync/pull`, `/sync/push` (the object's own unprefixed log, migrated on each
  request); `/sync-reset` (`delete_all`, the reset); `/sync-peer-rules`,
  `/sync-peer-rule/<name>`.
- `test/sync-peer.test.ts`; `deployed.mjs` gains the family with its floor; `ci.yml` gains the
  local step with the collected/passed/not-run guard.

### Round trip (live, `#[ignore = "..."]`, run with `--ignored` in the `workerd` job)

Env: `WORKERD_HARNESS_URL`, `HARNESS_TOKEN`, `WORKERD_HARNESS_RUN`. Client: `HttpExchange`
(`SendExchange`) over `hyper-util`'s legacy client with `hyper-rustls` (native roots),
`POST {url}/do/{object}/sync/{route}` with the bearer, a timeout on every round trip and a
capped reply body.

## Test list (drive in this order)

Crate, host:
1. [x] `sync_route_names_exactly_the_two_paths`
2. [x] `every_class_round_trips_through_its_status`
3. [x] `a_status_no_class_maps_to_reads_as_transient_or_stop`
4. [x] `the_request_cap_is_the_batch_plus_the_allowance_and_saturates`

Crate, wasm32 (Node shim):
5. [x] `a_push_and_a_pull_through_requests_round_trip_byte_identical`
6. [x] `a_request_that_is_not_a_post_is_refused_with_405`
7. [x] `a_body_over_the_cap_is_refused_as_split_before_the_store`
8. [x] `a_foreign_token_answers_400_and_reads_back_as_stop`
9. [x] `sync_conformance::*` (the whole family over the shim, through requests)

Harness, host:
10. [x] `the_names_are_the_sync_peer_enumeration_each_once`
11. [x] `a_round_trip_that_outlives_its_bound_is_a_transient_timeout` (paused time)
12. [x] `a_refused_connection_is_transient` (loopback port bound then released)

Harness, workerd (vitest):
13. [x] every sync-peer rule answers `ran`; an unknown rule answers 404

Harness, live (`--ignored`):
14. [x] `sqlite_round_trips_through_the_object_byte_identical` (A → DO → B, and B → DO → A)
15. [x] `replaying_a_batch_twice_changes_nothing` (a push replayed at the object; a pull
        replayed into SQLite)
16. [x] `a_reset_object_stops_the_edge_and_leaves_the_log_alone`
17. [x] `a_wrong_token_is_stop`

## Decisions (H-16)

- D-1 dev-dependencies: the hyper stack as Neon uses it (`hyper`, `hyper-util`,
  `http-body-util`, `hyper-rustls`, `rustls`), `bytes`, `tokio`, `happenstance-sqlite`; treated
  as covered by "hyper as a dev-dependency where needed is owner-approved", because `hyper`
  alone cannot make a request (no IO adapter without `hyper-util`, no HTTPS for the deployed
  leg without `hyper-rustls`). No node is added to `Cargo.lock`. Reported, not assumed silent.
- D-2 additive public API in `happenstance-cloudflare` (owner approved the reusable piece).

## Risks

- `workerd` job time: native `rusqlite` + `ring` builds on top of `worker-build`; the job's
  30-minute timeout is unchanged (not mine to raise).
- `wrangler dev` in CI for the local leg: needs no account; readiness is polled, bounded.
- Node shim fidelity for `Request`/`Response`: Node 24 has both globals.

## Progress

Honest ordering note: the four pure functions of `sync_http` (items 1-4) and
`serve_sync_request` were written before their tests, in one pass from the signatures
above; their tests therefore never ran red against missing code. The evidence that the
tests bite is the mutation runs below, each run against the real code with one line
changed and then reverted.

- Items 1-4 (host): green on first run (`cargo test -p happenstance-cloudflare --features
  sync --lib sync_http`, 4 passed). Not red-first; see the note.
- Items 5-9 (Node shim, `wasm-bindgen-test-runner`): 14 then 15 passed. **Mutation red:**
  `let cap = max_request_bytes(..).max(usize::MAX)` (cap disabled) turned
  `a_body_over_the_cap_is_refused_as_split_before_the_store` red with
  `served.rs:183: left: 400, right: 413` (the garbage body reached `serve_sync` and was
  refused as malformed); reverted, green.
- Item 10 (harness host): green, 1 passed.
- Item 13 (vitest under local workerd, Node 24): `test/sync-peer.test.ts` 13/13 passed,
  every rule `ran`. First run was after the routes existed; not red-first.
- Items 11-12 (harness host, not live): 2 passed.
- Items 14-17 (live, `wrangler dev` on 127.0.0.1:8787, `--ignored --test-threads=1`):
  4 passed. **Mutation red:** `/sync-reset` with its `delete_all` removed turned
  `a_reset_object_stops_the_edge_and_leaves_the_log_alone` red:
  `main.rs:316: expected the incarnation check to stop the edge, got Ok(Stepped {
  ingested: Ingested { appended: 0, skipped: 1, .. }, pushed: 0 })`; reverted, rebuilt,
  4 passed.
- `round-trip.sh` checked both ways locally: with the right token, `listed 4; passed 4`,
  exit 0; with a wrong token, `passed 1, failed 3`, exit 1 with the `::error::` line.
- The CI local step simulated end to end (build, `setsid npx wrangler dev`, readiness
  poll, `round-trip.sh`): green, and its trap left no `wrangler`/`workerd` process behind.
- Self-review fixes: `read_capped` no longer pre-sizes from the client's `Content-Length`
  (H-13); `carried` borrows instead of cloning each `Event` (H-08); the reply body is
  moved into the `Vec` rather than copied. Re-run after: shim 190 passed; vitest
  sync-peer + ingest + write-group + conformance 132/132; local round trip 4/4.
- Process slip, recorded: one `.rs` line (the `SqliteEventStore` import path in
  `tests/sqlite_round_trip/main.rs`) was changed with `sed` rather than Edit. Content
  is the one-token path fix; nothing else was edited outside Edit/Write (`cargo fmt`
  aside).
- `cargo xtask spec-trace`, `lint-constitution`, `lints`: green. Two `standards/`
  citations into `ci.yml` (`:2168`, `:2233`) repointed by referent to `:2242`, `:2307`.
- temper gate: GREEN (fingerprint a382cf9cfd9424c7).
