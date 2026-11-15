# VT-36: a runner `Split` on `TagsPerBatch` through a real push (wave 3, item 9)

Read at `origin/phase-14` = `5f5f8fea`. Planned for implementation on a branch off `phase-14`.
This plan writes no source. Architect's verdict: **build now**. It is a two-way door. It needs no
`Cargo.toml`, `package.json`, lockfile, `wrangler.toml` or `.github/workflows/ci.yml` edit. The
**freeze** itself is a one-way door and stays the owner's, in #163 / ADR-0108.

## Goal

Get the observation ADR-0107 §3 says is missing (`.kb/decisions/0107-…:180-183`; ledger row
`runbook/ledgers.md:309`): the runner's `Edge` pushes a batch over Cloudflare's 262,144 tags-per-call
ceiling to a real Durable Object over real HTTP. The object refuses it as
`IngestError::ExceedsStoreLimit { limit: TagsPerBatch, .. }`, which crosses HTTP as 413 and reads as
`Split`. The edge halves and keeps running. The halved pushes land whole, with nothing written by the
refused one. The test must run strictly on **both** `workerd` legs: local `wrangler dev` and the
deployed Worker.

## Why the observation has never happened (the finding that shapes the design)

Through the harness object as it is mounted today, **no push can reach the ceiling**:

- `harness/workerd/src/object/syncing.rs` serves `/sync/*` through
  `StoreSyncPeer::with_limits(.., PeerLimits::minimum())`, which allows 128 events per push and
  refuses more as `BatchTooLarge` before the ingest (`store_peer.rs:276`). At Cloudflare's 1,024 tags
  per event (`fixture.rs:135`) that is at most 131,072 tags, half the ceiling.
- The round trip's local store is SQLite, at `MAX_TAGS_PER_EVENT = 128` (`happenstance-sqlite
  event_store.rs:349`). Even against the adapter's 1,024 events per call, 128 × 1,024 = 131,072. A
  SQLite-origin edge can **never** trip Cloudflare's `TagsPerBatch`. `EventsPerBatch` always fires
  first.
- `sync_http::serve_sync_request` caps the body at `max_batch_bytes + ENVELOPE_ALLOWANCE` (1 MiB),
  and refuses past that as 413 `Split` **before decoding** (`sync_http.rs:186-193`). A push over
  262,144 tags is about 1.3 MB of envelope. Under `minimum` it would split on the body cap, which
  has the right class but is not `TagsPerBatch`. A test that only checked the class would pass for
  the wrong reason.

So the build needs three things:
1. a second mount on the object that declares wider limits;
2. a local store with no per-event tag cap;
3. an assertion on the refusal's **text**, not only on its class.

## Constraints and non-goals

- No edit to any `Cargo.toml`, `package.json`, `package-lock.json`, `wrangler.toml` or CI workflow.
  `round-trip.sh` already lists and runs every `#[ignore]`d test in `sqlite_round_trip` on both legs
  (`ci.yml` steps "The SQLite round trip against local workerd" and "… a deployed Durable Object").
  A new live test is picked up with no workflow change.
- No change to any published crate. Everything goes in `harness/workerd` (`publish = false`, never
  published) and its test target.
- The `/sync/*` mount, its `PeerLimits::minimum` and every existing test stay as they are.
- Not in scope: freezing VT-36. That needs the spec marker, the ledger row, the §7.1 census and
  ADR-0108, and all of it is #163, the owner's.
- Not in scope: `ENVELOPE_ALLOWANCE` being fixed at 1 MiB whatever `max_batch_events` a peer
  declares. See Risks: it is a follow-up issue, not this change.

## Invariants (testable)

- I-1. The refused push carries more than `MAX_TAGS_PER_BATCH` tags. Each of its groups carries no
  more than `MAX_TAGS_PER_BATCH`. Each event carries no more than `MAX_TAGS_PER_EVENT`. The push
  carries no more than the wide mount's `max_batch_events`. All four are derived from
  `WorkerdFixture`'s declarations, never from second literals.
- I-2. The refused push's encoded body is under the wide mount's `max_request_bytes`, so the refusal
  can only come from the ingest, not from the body cap.
- I-3. The first push that reaches the object is refused with status 413 and class `Split`, and the
  body contains `"tags per batch limit: {total}"`. That text is `IngestError::ExceedsStoreLimit`'s
  `#[error]` (`ingest/error.rs:95`) with `StoreLimit::TagsPerBatch`'s `Display` (`limits.rs:180`).
- I-4. After that refusal the edge is `Running`, with `last_failure == Failure::new(Phase::Push,
  FailureClass::Split)`, and the object holds nothing.
- I-5. The next push carries exactly one group. The edge then reaches `is_synced()`. The object holds
  both groups, each whole and equal to the local groups, tags included.
- I-6. No object reset (500) on a push at or under the ceiling is retried away. Such a reset is
  ADR-0100's falsifier and must stay red (`platform-miss.mjs` `OBJECT_RESETS` comment). `Edge::run`
  would retry it as `Transient`, so the test drives `Edge::step` by hand.

## Design

### Placement

All of it goes in `harness/workerd`, the instrument crate, which may depend on the adapter and both
testkits. Dependencies still point inward (ARC-02). Nothing goes in `happenstance-sync` or
`happenstance-cloudflare`: the runner already halves on any `Split` (`runner.rs:441-458`), and the
adapter already refuses before SQL (`event_store.rs:1089`).

### 1. `harness/workerd/src/sync_mount.rs` (new, `pub mod sync_mount;` in `lib.rs`)

This is the single definition the object and the native test both read, so "both ends configured
alike" (the header of `syncing.rs`) holds by construction rather than by two literals.

```rust
//! The object's two sync mounts and the limits each declares. ...
use happenstance_cloudflare::happenstance_sync::PeerLimits;

/// The mount whose limits reach Cloudflare's tags-per-call ceiling (VT-36): `/sync-wide/pull`
/// and `/sync-wide/push`. The `/sync/` mount declares `PeerLimits::minimum`, under which a push
/// carries at most 128 events, so at most 131,072 tags, and never meets the ceiling.
pub const WIDE_SYNC_PREFIX: &str = "/sync-wide/";

/// Payload bytes the wide mount declares per push: 4 MiB. Its only job is to lift
/// `max_request_bytes` (payload plus `ENVELOPE_ALLOWANCE`) above the ~1.3 MB envelope of a push
/// over the tags ceiling, so the body cap never answers for the ingest.
pub const WIDE_BATCH_BYTES: usize = 4 * 1024 * 1024;

/// What the wide mount declares: `minimum`, with the adapter's events-per-call ceiling and
/// `WIDE_BATCH_BYTES`.
#[must_use]
pub fn wide_limits() -> PeerLimits
```

- The body builds the value with `let mut limits = PeerLimits::minimum();` and then assigns
  `max_batch_events` and `max_batch_bytes`. `PeerLimits` is `#[non_exhaustive]` and `Clone` but not
  `Copy`, so a struct literal is refused outside its crate.
- `max_batch_events` is `match <WorkerdFixture<'static> as Fixture>::MAX_EVENTS_PER_BATCH { Some(n)
  => n, None => PeerLimits::minimum().max_batch_events }`. There is no `unwrap` (H-02). Test T-1
  pins that the `Some` arm is the one taken, so a fixture that loses its declaration fails by name
  instead of silently narrowing.
- Ownership: the function returns an owned `PeerLimits`, because `StoreSyncPeer::with_limits` and
  `RemoteSyncPeer::new` both store it (OWN-02).

### 2. `harness/workerd/src/object/syncing.rs` and `object.rs`

- `Syncing::serve(&self, request: Request)` becomes
  `serve(&self, request: Request, rest: &str, limits: PeerLimits)`.
  - `request` is taken by value: the body is consumed, as today.
  - `rest` is borrowed: it is the path after the mount.
  - `limits` is taken by value: `StoreSyncPeer` stores it.
- The route comes from `sync_route(&format!("{SYNC_PATH_PREFIX}{rest}"))`, so the
  `"pull"`/`"push"` table stays the published crate's only copy. Anything else answers 404 with
  `no sync route {path}`, as today.
- `StoreSyncPeer::with_limits(store.clone(), store, limits)` replaces the hard-coded `minimum()`.
- In `object.rs`'s `fetch` match (lines 134-135):
  `(Some("sync"), Some(rest)) => self.syncing().serve(req, rest, PeerLimits::minimum()).await`
  and a new arm
  `(Some("sync-wide"), Some(rest)) => self.syncing().serve(req, rest, sync_mount::wide_limits()).await`.
  `/sync-reset` is unchanged. It empties the whole object, which serves both mounts over one log.
- `worker.mjs` needs no edit: it forwards any `/do/<object>/<route>` (`worker.mjs:278`).
- Update the module docs: the header says "The limits are `PeerLimits::minimum`". It should now
  name both mounts and why the second exists.

### 3. `harness/workerd/tests/sqlite_round_trip/http.rs`

- `HttpExchange` gains `sync_prefix: &'static str`. `new` sets it to `SYNC_PATH_PREFIX`, so every
  existing caller behaves the same.
- New method `pub(crate) fn at_prefix(self, prefix: &'static str) -> Self`. It is a consuming
  builder (API-13).
- `SendExchange::exchange` formats `"{}{route}"` with `self.sync_prefix`.
- `max_reply` is computed from the limits passed to `new`, as today, so a wide peer reads a wide
  reply.

### 4. `harness/workerd/tests/sqlite_round_trip/main.rs`

- Add `Harness::peer_wide(&self, name: &str) -> RemoteSyncPeer<HttpExchange>`: `wide_limits()` and
  `.at_prefix(WIDE_SYNC_PREFIX)`.
- The local store is `happenstance_sync_testkit::MemoryIngestFixture::new().connect().await`, which
  gives a `MemoryIngestHandle`. It has no tag caps and implements `EventStore`, `IngestStore` and
  `WriteGroupStore` (Send flavours, so the base traits hold too). It is reached through
  `happenstance-sync-testkit`, which is **already a direct normal dependency** of the harness, so
  integration tests can see it. That honours the `Cargo.toml` comment's "D12 lesson": no reliance
  on a feature inherited through another crate's edge. Naming
  `happenstance_cloudflare::happenstance_sync::MemoryIngestStore` would compile today only because
  the testkit turns on `happenstance-sync/memory`, and that is exactly the inheritance the lesson
  forbids.
- Scenario constants come from the fixture (`happenstance_workerd_harness::fixture::WorkerdFixture`
  through `happenstance_testkit::Fixture`):
  - `PER_EVENT` = `MAX_TAGS_PER_EVENT` (1,024)
  - `CEILING` = `MAX_TAGS_PER_BATCH` (262,144)
  - `GROUP_EVENTS` = `CEILING / PER_EVENT / 2 + 1` (129). Use `checked_div` or a `const` expression
    over the unwrapped `Option`s. These are tests, where `expect` is allowed.
  - Two groups of `GROUP_EVENTS` events, each event with `PER_EVENT` tags.
  - That gives `TOTAL` = 264,192 tags: over the ceiling by one event's worth. Each group holds
    132,096 tags, about a quarter of the lowest measured deployed wall (524,288; ADR-0100).
- `fn split_scenario() -> [Vec<Event>; 2]`:
  - Tags are `t0`..`t1023`, built once as `Tags` and cloned into each event. Each `Event` stores its
    own, so this is a real duplication (H-08).
  - The payload is one byte, and the type is `"Tagged"`.
  - The halved cap is `TOTAL_EVENTS / 2 = 129 = GROUP_EVENTS`, so the halved push takes exactly one
    group (`Run::gather`, `run.rs:36-66`).
- `async fn step_past_platform_misses(edge, peer, store) -> Result<Stepped, StepFailure<..>>`:
  - It retries a step only when the error is `StepError::Pull` or `StepError::Push` of
    `RemoteError::Exchange(HttpError::{PlatformMiss | Transport | Body | Timeout})`.
  - It makes at most `RETRY_ATTEMPTS` attempts, with `RETRY_FIRST_PAUSE` doubling, as `retrying`
    does.
  - It **never** retries `HttpError::Refused`, whatever its class (I-6). It is bounded (H-13).
  - A retry after a timed-out push that landed is safe, because ingest is idempotent
    (`bulk_ingest_is_idempotent_in_bounded_round_trips`).
- The live test follows the observation sequence in the test list below. The step loop is bounded
  by `MAX_STEPS = 8`. That allows four steps, plus echo pulls, plus headroom.

### 5. `harness/workerd/round-trip.sh`

`FLOOR=4` becomes `FLOOR=6`. Five live tests exist today and this change adds the sixth. The FLOOR
is already stale by one. Raising it tightens the check and loosens nothing. It is a shell script,
not workflow config, but see decision D-4.

### 6. `harness/workerd/README.md` (PROC-11)

- The `syncing.rs` row (line 103): two mounts.
- The `tests/sqlite_round_trip/` row (line 105): "five live tests", and name the new one and the two
  new offline ones. It reads "four" today, which is also stale.

### Errors

There is no new error type. The test reads the existing chain
`StepError::Push(RemoteError::Exchange(HttpError::Refused { status, class, body }))`. The object
side keeps `Response::error(.., status_for(class))`.

### Concurrency and resources

- One object per test (`<run>-tags-split`), emptied first and last by `reset`, as every round-trip
  test is.
- `--test-threads=1` is already set by `round-trip.sh`.
- Every HTTP call is bounded by `ROUND_TRIP_TIMEOUT` (30 s), and the step loop by `MAX_STEPS`.
- Host memory: about 264k small `String`s, tens of MB, only on the live path.

### Alternatives considered

- **Raise `/sync/`'s declared limits for every test.** Rejected. The object fills pulls to its own
  declared limits, and existing clients cap replies at `max_request_bytes(minimum)`. That breaks
  "both ends configured alike", and a large pull would read as `ReplyTooLarge`, `Stop`.
- **Keep SQLite as the origin.** It cannot reach the ceiling at all, as shown above.
- **Add `happenstance-sync = { features = ["memory"] }` as a dev-dependency.** It works, but it is a
  `Cargo.toml` edit the owner must approve, and it gains nothing over the testkit's public handle.
- **Use `Edge::run` for the whole sequence.** It swallows the refusal text (I-3), and it retries an
  object reset 500 as `Transient` (I-6).
- **Add a new deployed-only `.mjs` script with its own CI step.** That needs a workflow edit and
  does not exercise the runner. The ask is a *runner* `Split`.

## Test list (drive in this order)

- [x] **T-1 (offline, in the gate)** `the_wide_mount_declares_the_adapters_events_per_call`.
  `wide_limits().max_batch_events == WorkerdFixture::MAX_EVENTS_PER_BATCH.expect(..)`, and every
  other field equals `PeerLimits::minimum()`'s except `max_batch_bytes == WIDE_BATCH_BYTES`.
  - Placement: in `src/sync_mount.rs` under `#[cfg(test)] mod tests`, host.
  - Red: does not compile, because `sync_mount` does not exist yet.
- [x] **T-2 (offline)** `the_split_scenario_crosses_the_ceiling_only_as_a_whole_push`. Arithmetic
  for I-1 from the fixture constants:
  - `PER_EVENT <= MAX_TAGS_PER_EVENT`;
  - group tags `<= CEILING`;
  - total tags `> CEILING`;
  - total events `<= wide_limits().max_batch_events`;
  - `TOTAL_EVENTS / 2 == GROUP_EVENTS` (the halved push is exactly one group);
  - and the negative control: `PeerLimits::minimum().max_batch_events * PER_EVENT < CEILING` (why
    `/sync/` cannot carry it).
  - Placement: `main.rs`, not ignored.
- [x] **T-3 (offline)** `the_split_push_fits_the_wide_mounts_body_cap`. I-2, measured rather than
  estimated:
  - Build a `RemoteSyncPeer` over a test-only `RecordingExchange`. It implements `SendExchange`,
    records `request.len()` in a `Cell`, and answers with an error whose `Classify` is `Stop`. Push
    `PushBatch::new` of the two groups' wire `EventGroup`s.
  - Assert `recorded > ENVELOPE_ALLOWANCE + PeerLimits::minimum().max_batch_bytes`. This is the
    negative control: under `/sync/` the body cap would answer first.
  - Assert `recorded <= max_request_bytes(&wide_limits())`.
  - No network. It runs in the gate. Build the events once here, about 264k tags, which costs well
    under a second on the host.
- [x] **T-4 (offline)** `step_past_platform_misses_never_retries_a_refusal`.
  - A 500 `Refused` carrying `OBJECT_RESETS["cpu-limit"]`'s text is returned after one attempt.
  - A `PlatformMiss` is retried up to `RETRY_ATTEMPTS`.
  - Use `start_paused = true` and a scripted fake `SyncPeer`, or a scripted `Exchange` behind
    `RemoteSyncPeer` with a memory local store. Paused time means no wall clock (TST-10).
  - This pins I-6 where it can fail by name.
- [x] **T-5 (live, `#[ignore = ".."]` with the file's existing reason string)**
  `the_runner_splits_a_push_on_tags_per_batch_through_the_object`. Steps:
  1. `peer_wide("tags-split")`, `reset`. Local `MemoryIngestHandle`. Append group A and then group
     B as two `append` calls, so there are two write groups. Precondition:
     `local.read_groups(None, ..)` shows `[(129), (129)]`.
  2. Build `Edge::new()`. Call `step_past_platform_misses` once, which must answer
     `Err(StepError::Push(RemoteError::Exchange(HttpError::Refused { status: 413, class:
     FailureClass::Split, body })))` with `body.contains(&format!("tags per batch limit:
     {TOTAL}"))`. The assertion message prints `body`, so a body-cap 413 explains itself. (I-3)
  3. `edge.status().state == EdgeState::Running` and `last_failure == Some(Failure::new(Phase::Push,
     FailureClass::Split))`. `drain(&peer)` is empty: nothing written. (I-4)
  4. The next step is `Ok(stepped)` with `stepped.pushed == GROUP_EVENTS`: one group landed alone.
  5. Step until `edge.status().is_synced()`, at most `MAX_STEPS`, with every step `Ok`.
  6. `drain(&peer)` equals the local groups as `EventGroup`s: two groups of 129, each event with
     1,024 tags, identity and bytes equal. (I-5)
  7. `reset`.
  - Red: does not compile, because `peer_wide` and `at_prefix` do not exist. Then, after the client
    half only, a live run against `wrangler dev` without the object arm answers 404 `no route` on
    `/sync-wide/push`, read as `Stop`, at step 2. Record whichever red is reachable. If `wrangler dev`
    cannot run on the implementer's machine, record the compile red and the first CI run.
- [x] **T-6** (FLOOR raised; the CI half, both legs listing and passing 6, is not verified here) `round-trip.sh` `FLOOR=6`. Verify both legs list 6 and pass 6 on the PR's CI run.
- [ ] Gate: `cargo xtask ci`, and `cargo test -p happenstance-workerd-harness --test
  sqlite_round_trip` (offline tests). Also `cargo xtask wasm`: the harness lib compiles for wasm32
  through `worker-build` in CI, and `sync_mount.rs` must not pull in a host-only item.

Optional, not needed for VT-36 and recommended only if there is spare time: a hermetic copy of T-5
in `crates/happenstance-sync/tests/runner/`. The runner would run against a `StoreSyncPeer` over a
`MemoryIngestStore` wrapper that refuses `TagsPerBatch` above a small ceiling. That would pin the
classification chain, from `IngestError::ExceedsStoreLimit { TagsPerBatch }` through
`StorePeerError` to `Split` and halving, in the gate rather than only in the `workerd` job. The
existing `a_halvable_split_is_retried_at_once_and_the_run_syncs` covers halving on a generic
`Split`. About 100 lines.

## What discharges VT-36's open condition, and what then freezes it

- **The evidence:** on a CI run of the `workerd` job, the step "The SQLite round trip against a
  deployed Durable Object" lists and passes
  `the_runner_splits_a_push_on_tags_per_batch_through_the_object`, with `round-trip.sh` strict
  (listed = passed, no failures, floor 6). The local leg passes the same test in the same run.
  - Record the run id and job id in the 0.7.0 log for #163, as ADR-0107 cited 42547020310.
  - Recommend **two** green deployed runs, to match the "green across runs" wording of the ledger's
    other condition. The PR's run and the `phase-14` merge run are enough.
- **The freeze:** ADR-0108 (#163), which is the owner's. It changes the marker to `[FROZEN]`, adds
  the new test to the `Rule:` line, sets the ledger row to frozen with this run id, and updates the
  §7.1 census. The falsifier ("a store whose per-call wall is not a function of the tag count")
  stays as the clause's own text. Freezing it is the owner's judgment that the 262,144 margin has
  held across runs.

## Decisions (H-16)

| # | Decision | Options | Recommendation | Door | Decided by | Outcome |
|---|---|---|---|---|---|---|
| D-1 | Build the observation in 0.7.0, or re-disposition VT-36 to `renew-past-1.0` | build now / build and hold / re-disposition | **Build now.** It is cheap, needs no protected edits, and fits inside an existing strict step | two-way | overnight authority (plan-0.7.0 decision 5) | build now (delegation, 2026-11-15) |
| D-2 | Freeze VT-36 in ADR-0108 on this evidence | freeze / renew-past-1.0 with the record below | Freeze, if T-5 is green on two deployed runs | **one-way** | owner | owner, not part of this change |
| D-3 | A second mount `/sync-wide/` on the harness object (an internal surface of an unpublished crate) | second mount / widen `/sync/` / new route with a query flag | Second mount | two-way | implementer, within authority | second mount (delegation, 2026-11-15) |
| D-4 | Raise `round-trip.sh` `FLOOR` from 4 to 6 (tightens, and it is a shell script, not workflow config) | raise / leave | Raise, and say so in the PR body (H-05: this is not a loosening) | two-way | implementer; owner can veto at review | raise (delegation, 2026-11-15) |
| D-5 | Local origin store | `MemoryIngestHandle` (direct dependency) / `MemoryIngestStore` via an inherited feature / `Cargo.toml` dev-dep | `MemoryIngestHandle` | two-way (the dev-dep option would need owner approval) | implementer | MemoryIngestHandle (delegation, 2026-11-15) |

### Fallback record text for #163, if D-1 or D-2 goes the other way or T-5 never goes green deployed

> **VT-36 to `renew-past-1.0`.** ADR-0107 moved VT-36 to `freeze-by-14` to wait for a runner
> `Split` on `TagsPerBatch` through a real push on the deployed `workerd` leg. Phase 14 did not
> produce it [reason: e.g. "the deployed run reset the object under the ceiling" / "the owner
> deferred it"]. Through a `StoreSyncPeer` declaring `PeerLimits::minimum`, a push carries at most
> 128 events, so at most 131,072 tags at Cloudflare's 1,024 per event: half the 262,144 ceiling. A
> SQLite origin, at 128 tags per event, cannot exceed 131,072 tags in any call the adapter accepts.
> No minimum-declaring edge can reach the ceiling. The clause's ingest-path sentence is held by
> `ingest_holds_the_call_to_its_tags_per_batch_ceiling` under `workerd`, and its floor and ceiling by
> the 262,144-tag branch of `append_reports_exceeded_store_limits`, green on the strict deployed leg
> across runs (ADR-0107 §3, run 42547020310 and before). Renewed past 1.0 against the first push
> over the ceiling through a peer declaring more than 256 events a push. The falsifier is unchanged.
> Plan: `notes/plan-vt36.md`.

## Risks

- **A deployed object reset on a landing push (132,096 tag rows in one `transactionSync`).** That
  is a quarter of the lowest measured wall, and less than the 262,144-row append the strict leg
  already passes on every run. T-5 fails red rather than retrying (I-6), which is the right outcome:
  it would be ADR-0100's falsifier, and the evidence for D-2's "renew".
- **CPU on the echo pulls and the final drain.** The object fills a pull to its declared limits,
  which do not budget tags. The final drain reads about 264k tag rows into one response of about
  1.3 MB. Reads have not been measured on the deployed object. The CPU limit is 30 s; this is
  estimated at seconds. If it resets, the class is `Refused` 500. T-5 fails, and the step names the
  drain. The fix would be to drain with a peer declaring `max_batch_events = GROUP_EVENTS`. That
  does not help, because the object fills to *its own* limits, so the real fix is a third mount at
  129 events. Note it, do not pre-build it.
- **"Worker does not exist [code: 10007]".** This code is not in the repository. It is the
  Cloudflare API's "script not found", which I expect at `wrangler deploy`/`delete` or in a race at
  the edge. The new test does not touch the deploy or delete steps. It lengthens the deployed leg by
  about 30–60 s. If 10007 appeared as an HTTP answer mid-test, it is not one of `is_platform_miss`'s
  signatures, so it would fail as `Stop`. That is the same exposure as the five existing
  round-trip tests. Not verified: where 10007 is emitted.
- **Body size.** T-3 measures it, so an encoding change that grew it past 5 MiB fails offline and by
  name, not as a mysterious deployed 413.
- **Latent design gap (follow-up, not this change).** `ENVELOPE_ALLOWANCE` is a fixed 1 MiB, sized
  as 8 KiB × 128 events. A peer declaring 1,024 events gets 1 KiB per event of allowance, and a
  1,024-tag event needs about 5 KiB. An application mounting `serve_sync_request` with wide limits
  would split tag-heavy pushes on the body cap. The class is correct (`Split`) and halving
  converges, but it splits earlier than declared. Worth an issue against `happenstance-cloudflare`.
- **Cost.** Cloudflare bills DO SQLite by rows written. One deployed run of T-5 writes about 264k
  tag rows plus index rows, roughly 0.5–1M row writes. The leg's existing record-only ingest-ceiling
  bisection writes several times that on every run. The paid plan includes 50M rows written a month
  and charges $1 per million beyond, so the marginal cost is cents a run. The local leg is free.

## Size (lower bound)

| File | Change |
|---|---|
| `harness/workerd/src/sync_mount.rs` (new) | ~45 lines including T-1 |
| `harness/workerd/src/lib.rs` | +1 |
| `harness/workerd/src/object.rs` | +3 / −1 |
| `harness/workerd/src/object/syncing.rs` | ~+15 / −8 |
| `harness/workerd/tests/sqlite_round_trip/http.rs` | ~+15 |
| `harness/workerd/tests/sqlite_round_trip/main.rs` | ~+190 (T-2..T-5, the helpers, `RecordingExchange`) |
| `harness/workerd/round-trip.sh` | 1 |
| `harness/workerd/README.md` | ~4 |

Total: 7 files changed and 1 new, about 270 lines. No protected file is touched.

## Progress

- 2026-11-15: planned (architect). Not started.
- 2026-11-15: implemented (implementer).
  - Placement deviation: T-2..T-5 and their helpers (`split_scenario`, `origin`, `written`,
    `step_past_platform_misses`, `RecordingExchange`, `ScriptedExchange`) live in a new module
    `tests/sqlite_round_trip/tags_split.rs` rather than `main.rs`, which was already 606 lines
    (ARC-06/CRF-02). `Harness::peer_wide` is in `main.rs` as planned.
  - Red T-1: `cargo test -p happenstance-workerd-harness --lib sync_mount` ->
    `error[E0432]: unresolved imports super::WIDE_BATCH_BYTES, super::wide_limits`. Green after
    `sync_mount.rs`.
  - Red T-2..T-5: `cargo test ... --test sqlite_round_trip` -> `E0603 RETRY_FIRST_PAUSE is
    private`, `E0599 no method is_unanswered`, `E0599 no method peer_wide`. Green after the
    `http.rs` client half and `peer_wide`: 8 offline passed, 6 ignored.
  - T-4 mutation: a helper that also retried `class() == Transient` failed with
    `an object reset is never retried: left 3, right 1`. Reverted.
  - T-5 live red (404 on `/sync-wide/push` without the object arm) not reachable here: no
    `wrangler dev` was run (the delegation forbids live legs). The first CI run is its evidence.
  - Wasm32: `cargo clippy -p happenstance-workerd-harness --target wasm32-unknown-unknown -- -D
    warnings` clean; `cargo xtask wasm` exit 0.
