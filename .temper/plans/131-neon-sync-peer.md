# #131 — `NeonSyncPeer`: the peer that cannot hold a transaction across a round trip

Branch `claude/p13-131` off `origin/phase-13` at `cad70864`. Implementer's plan.

## Goal

A `SyncPeer` whose far side is a Neon store reached over its HTTP endpoint, behind
`happenstance-neon`'s `sync` feature: **one SQL request per `pull` and one per `push`**, with
`waiting` decided inside the pull's own request (SY-37) and the responder identity
(ADR-0104) read inside the same request as the page or the write. It passes
`sync_peer_conformance!` against a live endpoint, mounted where `live-neon` already runs it.

## Constraints and non-goals

- No new dependencies (H-16). No new crates (plan-0.6.0, "No new crates").
- No change to `happenstance-sync`'s public API: `StoreSyncPeer`, `StorePeerError`,
  `StoreResume` untouched. The Neon peer reuses `StoreResume`, `PullBatchLimit`,
  `PeerLimits`, `EventGroup: From<WriteGroup>`.
- Additions in `happenstance-neon` go below every cited line: `lib.rs` is cited to `:220`,
  `event_store.rs` heavily, `error.rs` to `:156`. New module `event_store/sync_peer.rs`,
  declared at the end of `event_store.rs`; re-exports appended at the end of `lib.rs`.
  `ingest.rs` and `write_group.rs` are cited by no line, so their private builders and
  decoders are split and widened to `pub(super)` there.
- Don't touch `happenstance-cloudflare` or `harness/workerd` (#130's lane).
- CI: no change if the existing `--lib -- --ignored` step of `live-neon` picks the new
  live tests up. It does, if they are unit tests in the library target.
- Non-goals: a `Send` flavour (#421); `Waiting::Events` counting; bulk-ingest cost (#139).

## Invariants

- `pull` sends exactly one request: the group read and the identity read, two statements in
  one read-only `REPEATABLE READ` batch, so the page, `waiting` and the responder share one
  snapshot.
- `push` sends exactly one request: the ingest's three statements, then the watermark and
  the identity, in one `REPEATABLE READ` batch, so the ack's `confirmed` and `responder`
  describe the transaction that wrote the push. An empty push sends one read-only request
  of the last two only.
- A push the declared limits refuse sends nothing.
- Every page is whole groups; a group the budget cannot carry waits whole (D-2), and a page
  that stalls says `Waiting::Unknown`.

## Design

- Placement: `happenstance_neon::event_store::sync_peer` (private module, child of
  `event_store`, so it reaches the private SQL builders and decoders without widening
  them past the module tree; ARC-05). Re-exported as `happenstance_neon::{NeonSyncPeer,
  NeonPeerError}` behind `sync`.
- Types:
  - `pub struct NeonSyncPeer<T> { store: NeonEventStore<T>, limits: PeerLimits }`, no bounds
    on the struct (API-21). `new(store)` declares `PeerLimits::minimum()`, which sits
    inside the store's ceilings (128 events, 128 KiB per event); `with_limits(store,
    limits)`; `store(&self) -> &NeonEventStore<T>`.
  - `impl<T: SqlTransport> SyncPeer for NeonSyncPeer<T>`, bare flavour only (the store is
    bare-only; constraint 4). `type Resume = StoreResume` (the token is the far store's own
    position, as `StoreSyncPeer`'s is, and a `NeonSyncPeer` behind `serve_sync` speaks the
    same text form).
  - `pub enum NeonPeerError<E>` (`thiserror`, `#[non_exhaustive]`): `Read(NeonError<E>)`,
    `Ingest(IngestError<NeonError<E>>)`, `Answer(NeonError<E>)` (the request committed
    and its confirmation could not be decoded), `EventTooLarge { id, len, limit }`,
    `TooManyEvents { actual, limit }`, `BatchTooLarge { actual, limit }`. Own type,
    because `StorePeerError`'s limit variants are `#[non_exhaustive]` and cannot be
    built outside `happenstance-sync` (ERR-01). `Classify`: store arms ask the store's
    error, limit arms are fixed: Park, Split, Split (SY-36), as `StorePeerError`'s are.
- Splits in the private tree (no behaviour change, existing tests hold them):
  - `write_group.rs`: `groups_statement(after, max) -> SqlStatement` and
    `decode_group_page(rows, max_events) -> Result<GroupPage, NeonError<E>>`, both
    `pub(super)`; `read_write_groups` composes them.
  - `ingest.rs`: `ingest_write(&self, groups) -> Result<Option<Vec<SqlStatement>>,
    IngestError<..>>` (the ceilings, then the three statements; `None` for nothing
    foreign); `identity_statement`, `watermark_statement`, `ingest_body`,
    `ingested_from`, `identity_from(&ResultSet)`, `watermark_from(&ResultSet)`, all
    `pub(super)`; the port's methods compose them.
- Concurrency and resources: none new. One request per call; the response is bounded by
  `max_response_bytes` as every read here is (H-13).
- Postgres: **no dedicated type.** `StoreSyncPeer<PostgresEventStore, PostgresEventStore>`
  already is the Postgres peer: over a pool, the two or three queries a call makes are
  cheap, and `read_groups` already decides `waiting` in its own statement. A dedicated
  one-statement type would buy nothing SY-15 or SY-37 asks for on a pooled connection.
  Mounting the suite on it in `live-postgres` is cheap and is done if time allows.
- Alternatives considered:
  - *`type NeonSyncPeer<T> = StoreSyncPeer<NeonEventStore<T>, NeonEventStore<T>>`.* Composes
    with zero new SQL and meets SY-37 (the group read decides `waiting` in its statement),
    but costs **two** HTTP round trips per pull (page, identity) and **three** per push
    (ingest, watermark, identity), against the owner's one-and-one, and the identity is
    read in a different snapshot from the page. Lost on the round-trip count, the budget
    E2E-36 states.
  - *A façade store that stashes the identity read with the page for `StoreSyncPeer`'s
    next `store_id` call.* A cache ADR-0104 forbids, and racy across concurrent calls.

## Test list (drive in this order)

Scripted (no endpoint), in `ingest/tests/peer.rs`, beside the scripted transport they reuse:
- [x] `a_pull_is_one_read_only_request_of_the_page_and_the_identity`: one round trip, two
      statements, read-only, the page decoded whole, `responder` from the second set.
- [x] `a_pull_that_reaches_the_end_of_the_log_says_nothing_is_waiting` and its converse,
      a scan that filled its probe says `Unknown`.
- [x] `a_push_is_one_request_that_writes_confirms_and_names_the_store`: one round trip,
      five statements, not read-only, counts, watermark and responder from sets 0, 3, 4.
- [x] `an_empty_push_is_one_read_only_request_for_the_watermark_and_the_identity`.
- [x] `a_push_over_the_declared_limits_sends_nothing` (each limit, its class).
- [x] `a_refused_push_is_the_ingest_refusal` (HS038 answer maps to `UnheldOwnIdentity`).

Live (`#[ignore]`, `live-neon`'s `--lib -- --ignored`), in `ingest/tests/peer_family.rs`:
- [x] `sync_peer_conformance!` over a fresh schema per fixture.
- [x] `replaying_a_pushed_batch_changes_nothing`: second push is all skips, log identical.
- [x] `a_round_trip_through_the_neon_peer_is_byte_identical`: a near store's groups pushed
      through the peer and pulled back, payload `Bytes` equal byte for byte.

## Decisions (H-16)

| Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|
| Where the **SQLite**↔Neon round trip lives | (a) path-only dev-dep `happenstance-sqlite` in `happenstance-neon`; (b) a non-published harness crate; (c) an existing example | (a) | coordinator, under the owner's standing AFK authority | (b), shaped like #130's `harness/workerd`: (a) would rewrite the binding adapter-dependency rule |

## Risks

- The Neon `/sql` endpoint renders one result set per statement in a batch, including a
  `DO` and an `UPDATE`: the push decoder indexes sets 3 and 4. Checked by hand against
  local Postgres for the SQL; the indexing is held by the scripted test and the live leg.
- `REPEATABLE READ` read-only batch on the endpoint: measured for writes
  (`the_endpoint_honours_repeatable_read_on_a_two_statement_batch`); the read form is the
  same header path.

## Progress

- Red 1 (compile): `cargo test -p happenstance-neon --features sync --lib peer` with the seven
  scripted tests written first: `error[E0432]: unresolved imports
  crate::event_store::NeonPeerError, crate::event_store::NeonSyncPeer`, plus
  `no method named groups_statement / identity_statement / watermark_statement`.
- Green: the splits in `write_group.rs` and `ingest.rs`, then `event_store/sync_peer.rs`;
  7 passed. Mutant: the empty push's request without `.read_only()` fails
  `an_empty_push_is_one_read_only_request_for_the_watermark_and_the_identity`
  (`peer_requests.rs:234`), restored.
- Scripted file renamed `peer.rs` -> `peer_requests.rs`: `spec-trace` refused a second
  bare `peer.rs` (the spec cites `peer.rs:N` meaning `happenstance-sync`'s).
- Neon SQL checked by hand against local Postgres 17.10 (the container on :32768): the
  rendered migrations 1+3, the push request (three ingest statements, watermark, identity)
  in one `REPEATABLE READ` transaction twice, then the pull (group read, identity) in one
  `REPEATABLE READ READ ONLY` transaction. First push: appended 3, watermark p=7, identity
  row; replay: appended 0 skipped 3, same watermark and identity; pull: two groups in
  group order, `scanned` 3; zero negative `write_group` tags left.
- Live Neon leg (`peer_family.rs`: the suite's 10 rules, replay, round trip) compiles and
  lists under `--ignored`; not run here (no endpoint). `live-neon`'s `--lib --ignored`
  step picks it up: CI unchanged.
- Postgres: no type; `StoreSyncPeer<PostgresEventStore, PostgresEventStore>` mounted in
  `ingest/tests/peer_family.rs`. Run locally against the container:
  `HAPPENSTANCE_POSTGRES_URL=... cargo test -p happenstance-postgres --features sync --lib
  peer_family -- --ignored --test-threads=1`: 12 passed (10 rules, no SKIP lines; replay;
  round trip). `live-postgres`'s `--lib --ignored` step picks it up: CI unchanged.
- Test list: all ticked except the SQLite end of the round trip (BLOCKED, see Decisions):
  the round trip's near end is the sync testkit's in-memory ingest store for now.
- [x] scripted: pull one request; budget-ended page; pull from a token; push one request;
      empty push; limits before the wire; refused push.
- [x] live: conformance mount; replay; byte-identical round trip (memory near end).
- [x] SQLite near end: `harness/sync-round-trip` (`happenstance-sync-round-trip-harness`,
      `publish = false`), one test binary `tests/round_trip/` with `neon::` and `postgres::`
      cases (round trip both ways, replay). Postgres cases: 2 passed locally against the
      container. Neon cases without `NEON_CONNECTION` fail naming the variable, as the
      workerd harness's do. CI: a strict list/run/assert trio in `live-neon` (`neon::`) and in
      `live-postgres` (`postgres::`). `Cargo.lock`: one new node, the harness itself; every
      dependency of it was already a node. Two `standards/rust` citations of `ci.yml` lines
      repointed (the inserted steps moved them).
