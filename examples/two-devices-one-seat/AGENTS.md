# two-devices-one-seat

## Purpose

The flagship example. Two devices sell the last seat while offline; sync brings
both sales to a central store, which keeps its rule with one ordinary append;
three SQLite files converge on one log and one read model, read back after every
handle is dropped. Central's rule is a merge rule on its sync peer (#450,
#472). `src/overview.md` is the story and the crate-root doc.

## Layer

An example, `publish = false`: a library (the domain, the transport, the read
model, the scenario) and a thin binary (`src/main.rs`, the `anyhow` edge that
tells the story into a locked stdout). It sits above `happenstance` (the typed
layer), `happenstance-sqlite` (features `sync`, `projection-store`) and
`happenstance-sync` (no features). Inside: `domain` <- `timeline`, `scenario`
<- `readback`, `story` <- `main`; `rule` (central's merge rule) depends on
`domain`, the sync port and the store type; `uplink` on the sync port, the
store type and `rule`; `error` sits beside `scenario`, naming types from
`domain`, `rule` and `uplink`.

## Public surface

Every module is private; `src/lib.rs` re-exports each public item once, so an
item has one path. The surface is what `tests/story.rs` and `src/main.rs` drive:

- `tell(dir, &mut W) -> Result<Converged, StoryError>`, the whole story.
- `Scenario` (generic over central's rule, defaulting to `CentralRule`) and
  its steps (`open`, `judged_by`, `schedule`, `sell`, `set_link`, `sync`,
  `judge_again`, `project`, `view`, `ledger`, `log`, `status`), with `Synced`,
  `CAPACITY` and `SHOW`.
- The domain: `BoxOffice` (its JSON shape pinned by
  `domain::tests`), `ShowLedger`, `Sale`, `SaleId`, `ShowId`, `Place`,
  `Device`, `Refusal`, `RefundReason`, `Rule`, `UnknownPlace`, and the
  decisions `schedule`, `sell_seat`, `adjudicate` (in `domain/decide.rs`).
- Central's rule: `CentralRule` (a `runner::Adjudicate`) and `RuleError`.
- The transport: `Uplink`, `Link` (`Up`, `Down`, `PullOnly`), `UplinkError`,
  and `Central`, `CentralError`, `CentralPeer`, the aliases a Postgres
  central changes on the sync path.
- The read model: `Timeline`, `TimelineView`, `Row`, `Status`; read back:
  `Converged`, `Replica`.

`publish = false`, so this is not semver-bound, but `docs/sync-a-device.md`
mirrors `Uplink` and `UplinkError` by name, and `src/overview.md` names the
swap's items: renaming one means updating both.

## Error types

- `StoryError` (`src/error.rs`) is the one error the library returns. Every
  variant names the place or file it concerns. The large library errors it
  carries (`CommandError`, `StepFailure` as `DeviceStep`, `ProjectionError`)
  are **boxed**, so every `Result<_, StoryError>` stays small on its error
  side (clippy's `result_large_err`); the box costs one allocation on a path
  that is already failing. `refusal()` and `failure()` reach into the boxes
  for the two cases a caller branches on. `Contradicted` is the story
  refusing to print a claim that did not happen.
- `UplinkError` is the transport's: `Offline` (classified `Transient`) or
  central's own `ServeError`, its rule's failure included, keeping central's
  class (SY-36).
- `RuleError` is central's rule's: a failed read keeps the store's class;
  a query, decode or encode failure is `Stop`. Every rule `Scenario` mounts
  fails as a `RuleError`, so `DeviceStep` is one type whichever rule judged.
- `UnknownPlace` is `Place`'s `FromStr` error. `Refusal` is the sale
  decision's domain refusal, carried inside `StoryError::Sell`.
- `anyhow` appears only in `src/main.rs`.

## Allowed dependencies

`happenstance`, `happenstance-sqlite`, `happenstance-sync`, `rusqlite`
(workspace), `serde`, `thiserror`, `anyhow` (binary edge only), `tokio`
(`macros`, `rt`). Adding any other is an owner decision.

## Invariants

- Every decision a place makes goes through `happenstance::commit`; nothing
  appends around it. Central's refund is the one event that does not: the
  runner lands it with the push it answers (SY-2), as `CentralRule` decides.
- Every sync goes through `RemoteSyncPeer` -> `Exchange` ->
  `serve_sync_adjudicated` -> `StoreSyncPeer::with_adjudicator`, stepped by
  `runner::Edge`. No memory peer, no `memory` feature.
- Only central writes a `SaleRefunded` (SY-7): only central's peer mounts the
  rule; each device's edge keeps `Accept` (SY-10).
- `Timeline` gives the same table under any delivery order; it reads only the
  event, never a position, and counts seats held at read time.
- The transcript prints no store id, path or time, so it is stable across runs.
- The story returns `StoryError::Contradicted` rather than print a claim that
  did not happen.

## Test strategy

`tests/story.rs`, inside `mod story`, drives `Scenario`'s public steps against
real files in a per-test directory; five of its tests are registered in
`xtask/src/proof.rs`. The wrong implementation (no adjudication) is a test that
composes the same steps with central judged by a rule that answers nothing
(`Scenario::judged_by`), not a switch in the binary. One
test spawns the binary twice and compares the transcripts; one reads `src/` to
pin the published call path. Two unit tests need crate internals:
`timeline::tests` projects the same events in four orders, the refund ahead of
its sale among them, which no story run can produce; `domain::tests` pins the
payload's bytes. A claim about the read model or the wire needs a test that
some plausible wrong implementation fails (TST-05); name the mutant in the
plan.

## Do not

- Add a flag or environment variable that turns the central rule off.
- Write a compensation anywhere but through the runner's merge rule, or let a
  device's edge judge what it pulls (SY-7).
- Print anything run-dependent, or use `println!` (write through `W: Write`).
- Use `#[allow]`, `unwrap`, `expect` or panics in `src/`.

## Decisions

From `.temper/plans/two-devices-one-seat.md`, decided by the owner on
2026-11-15:

- D1 A: refund the sale that reached central last, by central's log order.
- D2 A: lib + bin, like `tickets-over-http`.
- D3 A: central adjudicates by convention via `commit` after ingest, linking #450.
  Done with: #472 moved it onto the hook #450 shipped in `0.7.0`.
- D4 A: the Postgres and Neon swaps are documented, not compiled.
- D5 A: `RemoteSyncPeer` over an in-process `Exchange` into `serve_sync`.
- D6: the crate is named `two-devices-one-seat` and takes the top README slot.
- D7 B: the guide page `docs/sync-a-device.md` ships in the same change.
