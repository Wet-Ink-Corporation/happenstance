# Plan: `examples/two-devices-one-seat` — the flagship "your data stays true, everywhere it lives" demo

Base: `1cfa397` (Phase 13 → main, `0.6.0`). Planned 2026-11-15. Implemented 2026-11-15 (see Progress).

## Goal

One runnable program, `cargo run -p two-devices-one-seat`, that tells this story on three real
SQLite files and prints it in a form a README or landing page can quote:

1. A central store (standing in for Postgres or Neon) schedules a show with **3** seats and sells
   **2** at the box office. Two devices (a box-office **tablet** and a **phone**), each a
   `SqliteEventStore` on its own file, sync down. One seat is left.
2. The signal drops. Each device sells the last seat with a DCB decision (`happenstance::commit`,
   a conditional append). Both succeed locally, because each decision is correct for the history
   that device holds. A sync attempt while offline fails *transiently* and loses nothing.
3. The signal returns. Each device syncs with the central store through `happenstance-sync`'s
   runner, one `runner::Edge` per device, over `RemoteSyncPeer` → in-process `Exchange` →
   `serve_sync` → `StoreSyncPeer`. Ingest never refuses (SY-1), so the central log now holds
   **4 sales for 3 seats**: the oversell is visible, not lost.
4. The central side adjudicates (by convention, see D3). It runs a decision over the complete
   history and appends one `SaleRefunded` as an ordinary conditional append (SY-2). Both devices
   sync again and converge.
5. A convergent read model (`happenstance::run_projection`, declared `ConvergentProjection`) is
   built **on all three files** and shows the same thing on each: 4 sales, 1 refund, seats held
   == capacity, and an audit line that says why. Every store handle is then dropped, and the
   files are reopened and read back. The three logs hold identical event-id sets, and the three
   read models are identical.
6. Closing beat: the phone tries to sell again. Its local decision now refuses with *sold out*,
   offline and with no round trip, because the history it holds is the complete one.

The four pillars map to beats: true offline (2), true to your rules (2, 4, 6: one decision model
enforced locally and centrally), true to its history (3, 5: nothing overwritten, and the refund is
an event with a reason), true wherever it lives (5: three files, one answer).

## Findings that shape the design (checked against source at `1cfa397`)

- **#450 (the runner adjudication hook) is OPEN** (checked through the REST API on 2026-11-15; no
  newer commit on `origin/main`). Phase 14 (#65) owns it, with #441 after it. `StoreSyncPeer`
  "never adjudicates": a push ingests with **no** compensation
  (`crates/happenstance-sync/src/store_peer.rs`, doc "It never adjudicates"). So no published API
  writes a compensation beside an ingest today. The example adjudicates **after** ingest, with an
  ordinary `commit` on the central store. That is SY-2's "compensated by an ordinary append", and
  SY-7 (provisional) gives authorship to one peer, here the central one. The example must say this
  is a convention pending #450 and must not invent a hook.
- `SqliteEventStore` is `Clone` (an `Arc<Mutex<Connection>>` handle, `event_store.rs:251`). Behind
  `happenstance-sqlite`'s **`sync`** feature it implements `SendIngestStore` and
  `SendWriteGroupStore` (`ingest_spike.rs:62`, `:90`; `type Error = SqliteEventStoreError` for
  both), and the trait_variant blanket gives the bare `IngestStore` and `WriteGroupStore`. So
  `StoreSyncPeer::new(central.clone(), central.clone())` and `Edge::step(&peer, &device_store)`
  both type-check with no adapter changes.
- `Edge<R>` (`runner.rs:180`): `Edge::new()`, `step(&mut self, peer: &P, store: &S) ->
  Result<Stepped, StepFailure<P, S>>` with `P: SyncPeer<Resume = R>, S: WriteGroupStore +
  IngestStore`; `status().is_synced()`. `run` needs a `Backoff` + `Sleep` (tokio `time`), which
  the example does not need: it steps in a bounded loop.
- `RemoteSyncPeer::new(exchange, limits)` (`remote.rs:85`), `Resume = wire::ResumeToken`.
  `Exchange` (bare) / `SendExchange` (`remote.rs:42`), `type Error: Error + Classify + 'static`,
  `async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, Self::Error>`.
  `serve_sync(&StoreSyncPeer<S, I>, Route, &[u8]) -> Result<Vec<u8>, ServeError<S::Error,
  I::Error>>` (`serve.rs:51`). The precedent for an in-process transport is `Loopback` in
  `crates/happenstance-sync-testkit/tests/remote_sync_conformance.rs:33-44`.
- `SequencedEvent` carries `id: EventId` (origin identity, preserved through ingest) and
  `position` (local). `EventId` is `Ord + Hash`. `Delivered<E>` (typed layer) has `id()` and
  `event()` only, with **no** arrival position and **no** `RecordedAt`. So a convergent fold can
  order only by `EventId` or by payload fields.
- `happenstance::{commit, Retry, CommandOutcome::Nothing, DecisionModel, DomainEvent, Projection,
  ConvergentProjection, Delivered, Chunk, run_projection, Json}` are all public at `0.6.0`.
  `impl<M: DecisionModel> Boundary for M` (`boundary.rs:112`).
- Example registration points: `examples/*` is a workspace glob (root `Cargo.toml:3`), so
  membership is automatic. `xtask/src/affected.rs` and `package.rs` need nothing (globs; examples
  are not publishable). Explicit registration: `xtask/src/proof.rs` (`ARTEFACTS` row plus a
  `const …_TESTS` list), `CLAUDE.md` repository map, `README.md` examples block (~`:115-140`),
  `CHANGELOG.md` `[Unreleased]`. No CI workflow names an example. `CLAUDE.md:94` says
  tickets-over-http is "the only lib target under examples/". A second lib target makes that
  sentence false, so it must change in the same commit.
- Repo convention vs this standard: sibling examples print with `println!`. H-06 requires
  `writeln!` on a locked handle. This plan writes through `W: io::Write`, which also lets tests
  capture the story without spawning (see Design). Sibling tests use
  `#![allow(clippy::unwrap_used, reason = …)]`. This plan uses `expect("why")` instead and needs
  no allow, because H-05 forbids `#[allow]`.

## Constraints and non-goals

- No new third-party dependency. Every dependency below is already in `Cargo.lock`:
  `happenstance`, `happenstance-sqlite` (+`sync`, `projection-store`), `happenstance-sync`,
  `rusqlite` (`workspace = true`, as transfers-on-sqlite argues), `serde`, `thiserror`, `anyhow`,
  `tokio` (`macros`, `rt`).
- No change to any published crate. No adjudication hook (that is #450). No adapter depends on
  another adapter: the example depends on one adapter plus the sync port.
- `#[async_trait]` is never used. Generic code binds the **bare** flavours (`EventStore`,
  `IngestStore`, `WriteGroupStore`, `Exchange`, `SyncPeer`). Import one flavour name per module
  (CLAUDE.md constraint 4).
- No `unsafe`, `unwrap`, `expect`, `panic!`, `as`, `#[allow]` or print macros in `src/`
  (examples follow production rules).
- Nothing spawned. `current_thread` runtime, every step awaited in sequence (H-13).
- Non-goals: an HTTP transport, Postgres or Neon in the gate (see D4), a peer set above one edge,
  an adjudicator that runs on its own schedule, RecordedAt-based ordering.

## Invariants (each is a test below)

- I1. After the story, the tablet, phone and central logs hold the **same set of `EventId`s**:
  6 domain events (1 scheduled, 4 sold, 1 refunded).
- I2. Exactly **one** `SaleRefunded` exists in each log, and its `EventId::store()` is the
  central store's id (SY-7: one author).
- I3. Seats held (sold minus refunded) == capacity (3) on every replica, in the decision model and
  in the read model.
- I4. Each offline sale was a successful `commit` (`CommandOutcome` committed), so nothing in the
  example bypasses the decision.
- I5. Without the adjudication step, the converged state has seats held == 4 > capacity, and no
  refund (the wrong implementation).
- I6. Adjudication is idempotent: a second `adjudicate()` returns `CommandOutcome::Nothing`.
- I7. A sync attempt while the link is down fails with class `Transient`. The edge stays
  `Running`, and the device's sale is delivered by the next step once the link is up.
- I8. The three read models are identical (same rows), as `ConvergentProjection` promises (SY-20).
- I9. All of the above hold after every handle is dropped and the files are reopened.

## Design

### Placement (ARC-02)

`examples/two-devices-one-seat/`, `publish = false`, a **lib + bin** package (D2). The lib holds
the domain, the transport, the read model and the scenario. The bin is a thin `main` that picks a
temp directory and writes the story to a locked stdout. Dependencies point inward:
`domain` ← `timeline` and `scenario` ← `story` ← `main`. `uplink` depends only on
`happenstance-sync` and the store type.

Files (ARC-06, no `mod.rs`):

| File | Holds | ~lines |
|---|---|---|
| `Cargo.toml` | manifest, long-hand happenstance lines with the why-comments siblings carry | 80 |
| `src/lib.rs` | `#![doc = include_str!("overview.md")]`, module list, re-exports | 40 |
| `src/overview.md` | the story, the four pillars, the #450 convention, the Postgres/Neon swap | 90 |
| `src/domain.rs` | `ShowId`, `SaleId`, `Device`, `BoxOffice` enum, `ShowLedger` model, `sell_seat`, `adjudicate`, `Refusal` | 230 |
| `src/uplink.rs` | `Uplink` exchange, `Link`, `UplinkError` + `Classify` | 110 |
| `src/timeline.rs` | `Timeline` projection, DDL, `TimelineView` read-back | 170 |
| `src/scenario.rs` | `Scenario` (three stores, two edges), `StoryError` | 240 |
| `src/story.rs` | `tell(dir, out)`: composes the beats and prints them | 170 |
| `src/readback.rs` | `Converged::read(dir)`, opened after every handle is dropped | 90 |
| `src/main.rs` | temp dir, `tell`, cleanup, `anyhow` at the edge | 40 |
| `tests/story.rs` | the test list, in `mod story` | 300 |

### Types (API-02, API-03)

```rust
// domain.rs
pub const CAPACITY: Seats = …;          // 3
pub struct Seats(NonZeroU32);           // capacity can't be zero; held counts are plain u32
pub struct ShowId { name: String, tag: Tag }   // tag "show:<name>", validated in TryFrom<&str>
pub struct SaleId { name: String, tag: Tag }   // "sale:<device>-<n>", minted by Device::next_sale
pub enum Device { Tablet, Phone, BoxOffice }   // closed set; BoxOffice = the central store's own sales
#[derive(Serialize, Deserialize)]
pub enum BoxOffice {
    ShowScheduled { show: String, capacity: u32 },
    SeatSold     { show: String, sale: String, buyer: String, sold_by: Device },
    SaleRefunded { show: String, sale: String, buyer: String, reason: RefundReason },
}
#[derive(Serialize, Deserialize)]
pub struct RefundReason { capacity: u32, sales_seen: u32, rule: Rule }
pub enum Rule { FirstToReachCentralKeepsTheSeat }   // D1. One variant, named, so the audit line is data
#[derive(Clone)]
pub struct ShowLedger { scope: Tags, capacity: Option<Seats>, held: Vec<Sale>, refunded: BTreeSet<String> }
#[derive(Debug, thiserror::Error)]
pub enum Refusal {
    #[error("show {show} is not scheduled on this device yet")] NotScheduled { show: String },
    #[error("sold out: {held} of {capacity} seats are held")]    SoldOut { capacity: u32, held: u32 },
}
```

- `DomainEvent for BoxOffice`: `EVENT_TYPES = ["ShowScheduled", "SeatSold", "SaleRefunded"]`
  via `EventType::from_static` (no `EVENT_TYPES[` indexing; the xtask lint rejects it). Tags:
  every event carries `show:<name>`. Sales and refunds also carry `sale:<id>`. Tags are built from
  the `Tag` held on `ShowId`/`SaleId`, so there is no fallible call in `tags()` (the
  carry-your-invariant shape).
- `ShowLedger::apply` folds in the store's own read order. `held` keeps sales in that order, and a
  refund removes its sale. This order is the **adjudicator's** arrival order on the central log,
  which D1 relies on. It is a decision over one log, not a convergent fold, so it is free to use
  log order.
- `pub fn sell_seat(ledger: &ShowLedger, sale: SaleId, buyer: &str, by: Device) ->
  Result<Vec<BoxOffice>, Refusal>`. The decision closure body. It refuses `NotScheduled` or
  `SoldOut`, otherwise returns one `SeatSold`. `buyer` is borrowed and copied into the owned event,
  because the event stores it (OWN-01/02). `sale` is moved, because the event stores it.
- `pub fn adjudicate(ledger: &ShowLedger) -> Result<Vec<BoxOffice>, Infallible>`. It returns
  one `SaleRefunded` for each held sale past capacity, refunding from the latest-arrived end, and
  **an empty vec when nothing is oversold**. `commit` maps an empty vec to
  `CommandOutcome::Nothing` (I6). `core::convert::Infallible` satisfies `commit`'s `D:
  core::error::Error + 'static`, and it says in the type that the adjudicator never refuses.
  `Refusal` stays `sell_seat`'s alone.

### Transport: `uplink.rs`

```rust
pub type Central = SqliteEventStore;                     // D4: the swap point, line 1
pub type CentralPeer = StoreSyncPeer<Central, Central>;
pub enum Link { Up, Down }
pub struct Uplink { far: CentralPeer, link: Cell<Link> } // bare Exchange: Cell is !Sync, fine on current_thread
impl Exchange for Uplink {                               // bare flavour, not SendExchange
    type Error = UplinkError;
    async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, UplinkError>;
}
#[derive(Debug, thiserror::Error)]
pub enum UplinkError {
    #[error("no signal: the request never left the device")] Offline,
    #[error(transparent)] Serve(#[from] ServeError<SqliteEventStoreError, SqliteEventStoreError>),
}
impl Classify for UplinkError { Offline => Transient, Serve(e) => e.class() }
```

- Why `Cell<Link>` and the bare flavour. The state is mutated through `&self` (the peer holds the
  exchange, and the edge borrows the peer), and nothing crosses a thread. `Cell` is the smallest
  tool for that. It costs `Send`/`Sync`, which nothing here needs, and it shows that the runner
  takes `!Send` peers (SY-17). The alternative was `AtomicBool` plus a `SendExchange` impl, which
  needs an ordering justification for no gain (ASY-15). Document this in the module doc, because
  the owner is learning Rust.
- Why `Uplink` keeps the `Exchange` promises trivially: one round trip is one function call. The
  reply is bounded by `StoreSyncPeer`'s `PeerLimits::minimum()`. There is no network, so no
  timeout is needed. Say so in the doc comment (H-13's "timeouts on external I/O": this is not
  external I/O).
- Each device gets its own `RemoteSyncPeer<Uplink>`, built as `RemoteSyncPeer::new(uplink,
  far.limits())` (limits read from the `StoreSyncPeer` before it moves into the `Uplink`).
- Why `RemoteSyncPeer` and not `StoreSyncPeer` directly: every exchange goes through the real
  postcard envelope and `serve_sync`, so swapping `Uplink` for an HTTP `Exchange` changes the
  transport and nothing else. That is the deployment story. (D5 offers the shorter alternative.)

### Read model: `timeline.rs`

```rust
pub struct Timeline { id: ProjectionId, scope: Tags }      // ProjectionId::from_static("seat_timeline")
impl Projection for Timeline { type Event = BoxOffice; type Store = SqliteProjectionStore;
                               type Error = SqliteProjectionStoreError; … }
impl ConvergentProjection for Timeline {}
pub const DDL: &str;  // seat_ledger(sale TEXT PK, buyer, sold_by, status CHECK IN ('held','refunded'), reason TEXT NULL) STRICT
                      // show_seats(show TEXT PK, capacity INTEGER NOT NULL) STRICT
pub struct TimelineView { pub capacity: u32, pub rows: Vec<Row> }  // read back with a fresh rusqlite::Connection
pub struct Row { pub sale: String, pub buyer: String, pub sold_by: String, pub status: Status, pub reason: Option<String> }
```

- `apply` is order-independent, so the convergence claim is true and not merely declared.
  `SeatSold` inserts the row with `status = 'held'` **ON CONFLICT DO NOTHING** on status, filling
  in buyer and sold_by. `SaleRefunded` upserts `status = 'refunded'` and `reason`, inserting a stub
  row if the sale has not arrived. It reads only `event()` (never position, and never `self` as
  history). Seats held are computed at read time as `COUNT(*) WHERE status = 'held'` and are never
  incremented, so arrival order cannot change them.
- The audit line is rendered from `RefundReason`, so the text is data on the event that
  replicates and not a string formatted at one site: *"phone-1 (Grace, phone) refunded: 4 sales
  reached central for 3 seats; rule: the first sale to reach central keeps the seat."*
- Print order is `ORDER BY status = 'refunded', sale`, a deterministic order that every replica
  shares. Chronology is not available to a convergent fold (no `RecordedAt` on `Delivered`, and
  positions are local). That is a teaching point, and the overview says so in two sentences.
- `Chunk::of(16)`. Each file gets `SqliteProjectionStore::open(path)` on the same file as its
  event store (the transfers-on-sqlite arrangement). The DDL is run by the application's own
  `rusqlite::Connection`, because the projection store migrates only checkpoints.

### Scenario: `scenario.rs`

```rust
pub struct Scenario {
    central: Central, tablet: SqliteEventStore, phone: SqliteEventStore,
    tablet_link: RemoteSyncPeer<Uplink>, phone_link: RemoteSyncPeer<Uplink>,
    tablet_edge: Edge<ResumeToken>, phone_edge: Edge<ResumeToken>,
    show: ShowId,
}
impl Scenario {
    pub fn open(dir: &Path) -> Result<Self, StoryError>;                 // three files: central.db, tablet.db, phone.db
    pub async fn schedule(&self, capacity: Seats) -> Result<(), StoryError>;            // on central
    pub async fn sell(&self, at: Device, buyer: &str) -> Result<SaleId, StoryError>;    // commit on that device's store
    pub fn set_link(&self, device: Device, link: Link);                                 // Cell, &self
    pub async fn sync(&mut self, device: Device) -> Result<Synced, StoryError>;         // bounded step loop
    pub async fn adjudicate(&self) -> Result<CommandOutcome, StoryError>;               // commit on central
    pub async fn project(&self) -> Result<(), StoryError>;                              // run_projection on all three
    pub async fn ledger(&self, at: Device) -> Result<ShowLedger, StoryError>;           // read_decision_model, for printing
}
```

- `sell(Device::BoxOffice, …)` sells on central. `Device::Tablet` and `Device::Phone` sell on the
  device file. `SaleId` is minted from a per-device counter derived from the device's own held
  sales, so ids are deterministic per run (`office-1`, `office-2`, `tablet-1`, `phone-1`) and the
  transcript is stable.
- `sync` steps the device's edge until `status().is_synced()`, at most `MAX_STEPS = 4` times, and
  then fails with `StoryError::NotSynced { device, steps }`. It is bounded, with no sleep and no
  backoff, because an in-process peer either answers or fails. A `Transient` failure is returned
  to the caller as `StoryError::Sync` (the story prints it as "no signal"). It is not retried
  here, because the offline beat wants to show it.
- `Retry::attempts(3)` for every `commit`.
- Ownership: `Scenario` owns every store handle and drops them all at once (the read-back must not
  see a live handle). `sync` takes `&mut self` because `Edge::step` takes `&mut Edge`. Everything
  else takes `&self`.

```rust
#[derive(Debug, thiserror::Error)]
pub enum StoryError {
    #[error("opening {file}")]                  Open { file: PathBuf, #[source] source: SqliteEventStoreError },
    #[error("opening the read model in {file}")] OpenModel { file: PathBuf, #[source] source: SqliteProjectionStoreError },
    #[error("{device} could not decide")]       Command { device: Device, #[source] source: CommandError<SqliteEventStoreError, Refusal> },
    #[error("central could not adjudicate")]    Adjudicate { #[source] source: CommandError<SqliteEventStoreError, Infallible> },
    #[error("{device} could not sync")]         Sync { device: Device, #[source] source: DeviceStep },
    #[error("{device} was not synced after {steps} steps")] NotSynced { device: Device, steps: usize },
    #[error("projecting {device}")]             Project { device: Device, #[source] source: ProjectionError<…> },
    #[error("reading the read model in {file}")] ReadModel { file: PathBuf, #[source] source: rusqlite::Error },
    #[error("writing the story")]               Write(#[from] io::Error),
}
pub type DeviceStep = StepFailure<RemoteSyncPeer<Uplink>, SqliteEventStore>;
```

There is no `#[non_exhaustive]`, because the crate is unpublished (ERR default table). Every
variant carries the device or file it concerns (ERR-03).

### Story and binary

- `pub async fn tell<W: io::Write>(dir: &Path, out: &mut W) -> Result<Converged, StoryError>`.
  It composes the beats inside one inner scope that owns the `Scenario`, so every handle is
  dropped when that scope ends. It then calls `Converged::read(dir)` and prints the read-back.
  Section markers are fixed strings (the transfers-on-sqlite convention), in this order:
  `== the show ==`, `== the signal drops ==`, `== both devices sell the last seat ==`,
  `== the signal returns ==`, `== central sees the whole history ==`,
  `== central keeps its rule ==`, `== everyone converges ==`, `== the same view, three places ==`,
  `== every handle dropped; reading the files back ==`, `== the phone tries again ==`.
  The last beat reopens the phone's file and runs `sell` once more, refused `SoldOut` locally.
- `main`: `#[tokio::main(flavor = "current_thread")]`, `-> anyhow::Result<()>`. The directory is
  `env::temp_dir().join(format!("happenstance-two-devices-{}", process::id()))`, created fresh and
  removed at the end (including `-wal` and `-shm`). Failures go through `.context()`. It calls
  `tell(&dir, &mut io::stdout().lock())`.
- `Converged { tablet: Replica, phone: Replica, central: Replica }`, where `Replica { ids:
  BTreeSet<EventId>, refunds: Vec<EventId>, view: TimelineView }`. Each is read by a fresh
  `SqliteEventStore::open` plus `collect(read(Query::all()))`, and by a fresh connection for the
  view. `Converged::read` also returns the central `StoreId` (from `IngestStore::store_id`) so
  tests can check I2.

### The Postgres and Neon swap (documented in `overview.md` and the README paragraph; see D4)

- **Postgres** (`happenstance-postgres`, feature `sync`): `type Central = PostgresEventStore;` and
  `PostgresEventStore::new(pool)` in `Scenario::open` in place of `SqliteEventStore::open(...)`.
  That is one type line and one constructor line, plus a `PgPool`. `StoreSyncPeer`, `serve_sync`
  and the adjudicator are unchanged, because they are generic over the store. The central **read
  model** does not swap for free: `Timeline`'s `type Store` is `SqliteProjectionStore`, and a
  Postgres central needs its own `Projection` impl over `PostgresProjectionStore`'s batch. Say so
  plainly rather than call it one line.
- **Neon**: not a one-line change, and the doc says so. `NeonSyncPeer` *is* the peer (the device
  talks to Neon directly over one-shot HTTP, with no server), so `RemoteSyncPeer<Uplink>` is
  replaced by `NeonSyncPeer::new(NeonEventStore::new(transport, config))`, and the application
  supplies an `SqlTransport` (the crate ships none for native). The adjudicator commits through a
  `NeonEventStore`.
- Neither is compiled by this example (D4).

### Alternatives considered

- `MemorySyncPeer` on central: rejected, because the user rules it test-only and it is not a real
  store.
- Compensation in the ingest (`IngestGroup::compensation`): no published runner path writes it,
  and `StoreSyncPeer` refuses to adjudicate. That path waits for #450.
- Adjudicate on EventId order (SY-23): it is deterministic across peers, but `StoreId`s are minted
  randomly per run, so which device wins would change between runs and the README transcript would
  not be stable. See D1.
- An `Adjudication::{On, Off}` parameter on `tell`: rejected, because it ships a "turn correctness
  off" switch in the flagship demo. The wrong-implementation test composes `Scenario`'s public
  steps without `adjudicate()` instead.
- `Edge::run` with a backoff: it pulls in tokio `time` and a `Sleep` impl for a peer that never
  waits. The bounded `step` loop is the honest shape here.

## Test list (drive in this order; TST-01)

All in `tests/story.rs`, inside `mod story` so `xtask proof` lists them qualified. Each test gets
its own fresh temp directory, named with the process id plus the test name and removed at the
end, so tests that run in parallel never share a file. Tests use `expect("why")`. Nothing sleeps.

1. [x] `story::a_device_sells_a_seat_offline` — schedule on central, sync the tablet, set the link
   `Down`, `sell(Tablet)` → committed. The tablet's ledger holds 1 sale and the central ledger
   holds 0.
2. [x] `story::a_sync_without_signal_is_transient_and_loses_nothing` (I7) — with the link down,
   `sync(Tablet)` → `Err(StoryError::Sync { source })` whose `failure().class ==
   FailureClass::Transient`, and the edge state is `EdgeState::Running`. Set the link up and
   `sync` → central holds the tablet's sale.
3. [x] `story::the_last_seat_is_refused_locally_once_sold` — on one device, with capacity 3 and
   3 held → `sell` fails `Refusal::SoldOut { capacity: 3, held: 3 }` and nothing is appended.
   This proves the local rule is real.
4. [x] `story::both_devices_sell_the_last_seat_and_central_holds_both` (SY-1) — the full offline
   double sale plus a sync of each. Central's ledger holds 4 sales for 3 seats. Check the outcome
   of each offline `commit` was committed (I4).
5. [x] `story::without_adjudication_the_show_stays_oversold` (I5, **the wrong implementation**) —
   the same steps as the converged story but with no `adjudicate()`, then sync both and project.
   Every replica's view has held == 4 > capacity and 0 refunds. This test must pass now. It
   documents what test 6 rejects.
6. [x] `story::the_story_converges` (I1, I2, I3, I8, I9) — `tell(dir, &mut Vec::new())`, then
   from the returned `Converged` read after drop: three equal `ids` sets of size 6, exactly one
   refund in each with `refund.store() == central_id`, held == 3 on every view, and three equal
   `TimelineView`s.
7. [x] `story::adjudication_is_idempotent` (I6) — after adjudicating once, `adjudicate()` →
   `CommandOutcome::Nothing`, and central's log length is unchanged.
8. [x] `story::the_refunded_sale_is_the_last_to_reach_central` (D1) — tablet syncs before phone,
   so the refund names `phone-1`. Then a mirrored run (phone first) refunds `tablet-1`. This
   rejects an adjudicator that always refunds a fixed device or refunds by sale id.
9. [x] `story::the_phone_is_refused_after_converging` — the last beat: the reopened phone file,
   with its link down, refuses `SoldOut` with no sync.
10. [x] `story::the_binary_tells_the_story` — spawn `CARGO_BIN_EXE_two-devices-one-seat`, exit 0,
    every section marker present **in order**, and the transcript contains "4 sales", "1 refund"
    and "3 of 3 seats held" exactly once per replica line. No store ids are printed (the transcript
    is stable across runs: run twice and compare).
11. [x] `story::the_program_uses_the_published_path` (source-reading, the transfers-on-sqlite
    precedent) — `src/` contains `happenstance::commit`/`commit(`, `run_projection(`,
    `serve_sync(`, `RemoteSyncPeer::new(`, `Edge::new(` and `impl ConvergentProjection`. It
    contains no `MemorySyncPeer` or `MemoryIngestStore`, and the manifest does not enable
    happenstance-sync's `memory` feature. This rejects a program that prints the transcript while
    hand-rolling the sync.

`xtask/src/proof.rs`: add `const OFFLINE_CONVERGENCE_TESTS: &[&str]` listing tests 2, 5, 6 and 8,
with a doc comment that names the wrong implementation each rejects (test 5 is the contrast,
listed so it cannot be silenced alone), plus an `Artefact { package: "two-devices-one-seat",
target: "story", … }` row.

Property test (TST-06): not planned. The convergence property across interleavings is already
`happenstance-sync-testkit`'s `convergent_projection_is_interleaving_independent` (#141). Test 8's
mirror is the one interleaving the example's claim depends on.

## Registration checklist (same change; PROC-11)

- [x] `examples/two-devices-one-seat/**` (new)
- [x] `xtask/src/proof.rs`: test list const + `ARTEFACTS` row
- [ ] (owner) `CLAUDE.md` map: a new entry after `tickets-over-http`, and the tickets-over-http line "the
      only lib target under examples/" is reworded (two lib targets now)
- [x] `README.md`: a `cargo run -p two-devices-one-seat` line in the examples block and a short
      paragraph. If D6 says flagship, it goes **above** the "Four more" block
- [x] `CHANGELOG.md` `[Unreleased]`: one "Added" entry
- [x] `Cargo.lock`: regenerated by cargo (PROC-13), path crate only
- [ ] Release habit: the example's `happenstance` line carries `version = "0.6.0"`, which the
      next release bump must move with the other six (handover lists "the six examples")

## Guide page: "Sync a device to a central store" (separate PR recommended; D7)

**File:** `docs/sync-a-device.md`. First line `# Sync a device to a central store`, then
`> **Answers:** \`how-to\` — How do I keep a device's event store in sync with a central one?`
(RP-00 one need, RP-10-1 exact token). Conflict theory stays out of scope: one paragraph plus a
link to the example covers "what if both sold the last seat".

**Content it needs:**
1. The shape: one `runner::Edge` per device, the far side a `StoreSyncPeer` behind `serve_sync`,
   and the device reaching it through `RemoteSyncPeer` over an `Exchange` the app supplies.
2. A compiled fence: two stores, an in-process `Exchange` calling `serve_sync`, one `Edge::step`
   in each direction, and an assert that both logs hold the same `EventId`s. This needs
   `MemoryIngestStore` (see the registration item below).
3. A compiled fence for offline: an `Exchange` returning a `Transient` error, the step fails, the
   edge stays `Running`, and the next step delivers. It should reject a runner that drops the
   batch on failure.
4. Persisting the watermark (SY-31): `Edge::watermark()` → `Watermark::commit` in the caller's
   projection batch, and `Edge::restored` on restart. Use a `text` fence if no projection store is
   linkable.
5. "Both sides wrote": ingest never refuses (cite SY-1), compensation is an ordinary append by the
   one adjudicator (cite SY-2, SY-7), and the hook is not built (#450). The example is the
   by-convention version and is linked.
6. SQLite lines (`features = ["sync"]`, `SqliteEventStore::open`) in `text` fences, which say they
   are not compiled here. Postgres and Neon swap notes as in Design.
7. Citations by clause id only (RP-30): SY-1, SY-2, SY-7, SY-17, SY-30, SY-31, SY-36. No `MUST`
   restated.

**Registration points for a new docs page:**
- `docs/sync-a-device.md` (new).
- `xtask/src/narrative.rs`: `#[cfg(doctest)] mod sync_a_device { #![doc =
  include_str!("../../docs/sync-a-device.md")] }`, with the comment above it that names each fence
  and the wrong implementation that turns it red (the house shape). `lint_narrative` (`HARNESS`)
  fails if a page has no `include_str!` line.
- `xtask/src/site.rs` `READING_ORDER`: insert after `"run-on-a-durable-object.md"` (the edge store
  and then syncing it), and update the order prose in the doc comment above it. `check_coverage`
  fails a page that is in neither `READING_ORDER` nor `OFF_SITE`.
- `docs/README.md` "Everything else" table: the row `| Syncing a device's store with a central one
  | [\`sync-a-device.md\`](sync-a-device.md) |` in the same position.
- `xtask/Cargo.toml` dev-dependency `happenstance-sync = { workspace = true, features =
  ["memory"] }`, so fences can name `MemoryIngestStore`. This is a feature on an existing dev-dep,
  not a new dependency, but see D7.
- Non-author review against `standards/pages/40-reviewing-a-page.md` (RP-40), and
  `cargo xtask lint-pages`, `cargo test -p xtask --doc`, `cargo xtask site --no-api`.
- Cross-link: the example's `overview.md` and the README paragraph link to the page.

## Decisions (H-16)

| # | Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|---|
| D1 | Which sale the adjudicator refunds | A) the last to reach central (central log order). B) lowest EventId (SY-23 order). C) the earliest device clock (RecordedAt). | **A**. It is deterministic in the demo, it reads as fair ("first to the box office wins"), and the decision is recorded as an event, so peers converge on the outcome whatever their own order. B makes the winner random per run (StoreIds are minted). C is ruled out: RecordedAt is not an ordering key (core docs). | owner | **A** (2026-11-15) |
| D2 | Package shape | A) lib + bin (tickets-over-http precedent). B) bin only, tests spawn the binary, and the wrong-impl contrast comes from a CLI flag or env var. | **A**. Tests drive `Scenario` steps directly, and the contrast composes them without `adjudicate`, so no "disable correctness" switch ships. Cost: `CLAUDE.md`'s "only lib target" sentence changes. | owner | **A**; CLAUDE.md left untouched, proposed text in Progress |
| D3 | Adjudication before #450 | A) central adjudicates by convention via `commit` after ingest, stated as such. B) wait for #450 and phase 14. C) add a hook to `happenstance-sync` here. | **A**. It is honest under SY-2 and SY-7 as they stand, and it needs no API change. When #450 lands, the example moves to the hook (open a follow-up issue linked from #450). C is a public API change in phase 14's lane. | owner | **A**, linking #450 |
| D4 | Postgres and Neon in CI | A) documented only. B) a `#[ignore = "needs a live Postgres"]` test in the example, run by the `live-postgres` leg. C) both legs. | **A**. B adds `happenstance-postgres` + `sqlx` + `testcontainers` to the example's dev graph and a second `Projection` impl for the central read model. That is not cheap, and `harness/sync-round-trip` already proves SQLite ↔ Postgres `StoreSyncPeer`. Neon is not a one-line swap at all (it needs an `SqlTransport`). | owner | **A**, documented only |
| D5 | Transport in the demo | A) `RemoteSyncPeer` over an in-process `Uplink` → `serve_sync`. B) `StoreSyncPeer` used directly as the peer. | **A**. It runs the real wire path, and "swap `Uplink` for your HTTP client" is the deployment sentence. B saves about 60 lines but hides the boundary the landing page is about. | owner | **A** |
| D6 | Crate name and billing | `two-devices-one-seat` / `offline-oversell` / `last-seat` | **`two-devices-one-seat`**. It names the situation, not the failure. If it is billed as *the* flagship, it gets the top README slot ahead of transfers-on-sqlite; otherwise it goes in the "Four more" block. | owner | `two-devices-one-seat`, top slot in the README examples block |
| D7 | Guide page scope and timing | A) separate follow-up PR. B) same PR as the example. Also: enable `memory` on xtask's `happenstance-sync` dev-dep, or keep every sync fence in `text`. | **A**, with `memory` enabled. A page with no compiled fence is the decoration the narrative harness exists to prevent. | owner | **B** (changed from the recommendation): page in this change, `memory` enabled |
| D8 | New example crate | Requested by the owner's brief. Recorded because creating a crate is H-16. | proceed | owner (brief) | requested |

## Risks

- **`is_synced()` after one step.** If `StoreSyncPeer` answers `Waiting::Unknown` on a page that
  happens to end exactly at the budget, `sync` needs a second step. `MAX_STEPS = 4` absorbs this.
  Tests 4 and 6 catch a regression as `NotSynced`.
- **A device's own sale echoed back.** A pull from central returns the device's own sale (as a
  foreign-origin group, its origin being the device). Ingest skips what the store holds, keyed by
  EventId. Test 6's size check (exactly 6 ids, exactly 1 refund) catches a duplicate.
- **ADR-0098, an unheld own identity.** Central pushing a device's own events back must not trip
  "unheld own identity stops ingest". It does not, because the device *holds* them. If it did,
  test 4 fails on the second device's sync. Read `ingest_spike.rs` if that happens and do not
  paper over it.
- **Read-model convergence.** If `apply` were order-dependent (for example an incremented counter,
  a sale that resets `status` to `held`, or a refund that is a plain `UPDATE`), test 6's
  equal-views assertion would still pass in this one interleaving, because every log the story
  builds holds a sale before its refund. *Corrected after review (F1):* an earlier version of this
  bullet named test 8's mirrored run as the guard, but test 8 never reads a view; it compares
  central's refunded ids. The guard is the unit test
  `timeline::tests::the_view_is_the_same_under_any_delivery_order`, which appends the same four
  events to scratch files in four orders, the refund ahead of its sale among them, projects each
  once and asserts every view equals the in-order one. Both mutants above were applied and turned
  it red while the eleven story tests stayed green. The testkit's interleaving rule is still the
  real proof of SY-20, and the example does not claim more.
- **SY-7 is provisional.** If phase 14 assigns authorship differently, D3's convention text has to
  change. That is the follow-up issue.
- **Stable transcript.** Any printed StoreId, path or timestamp breaks the README quote. Test 10
  runs the binary twice and compares, apart from the one line that prints the temp directory,
  which is masked or not printed. Decide: **do not print the directory**.

## Estimate (lower bound)

- Example: 11 files, about **1,560 lines** (`src/` about 1,180 including about 90 of overview
  prose, `tests/` about 300, manifest about 80).
- Registration: `xtask/src/proof.rs` +25, `CLAUDE.md` +10/−1, `README.md` +12, `CHANGELOG.md` +6.
  About **55 lines over 4 files**.
- Guide page (if D7 = same PR, otherwise the follow-up): `docs/sync-a-device.md` about 220,
  `narrative.rs` +12, `site.rs` +4, `docs/README.md` +1, `xtask/Cargo.toml` +1. About **240 lines
  over 5 files**.
- Total: about 16 to 20 files and **about 1,850 lines**. Implementation effort is about one focused
  session for the example and half a session for the page.

## Progress

- 2026-11-15: planned. Nothing implemented. #450 confirmed open on `origin/main` at `1cfa397`.

- 2026-11-15: implemented in worktree branch `worktree-agent-a15d1eb9aab5d8586`, base `1cfa397`
  (`origin/main` had not moved; the fast-forward was a no-op). Owner decisions as recorded in the
  table: D1 A, D2 A, D3 A, D4 A, D5 A, D6 as recommended, D7 **B**.

### Red runs (TST-01)

- Red 1, all eleven `story::*` tests written first against an empty lib:
  `cargo test -p two-devices-one-seat --test story` →
  `error[E0432]: unresolved imports two_devices_one_seat::CAPACITY, ...::Converged, ...::Device,
  ...::Link, ...::Place, ...::Refusal, ...::Scenario, ...::StoryError, ...::tell`.
  (Before that, `include_str!` of the not-yet-written `src/domain.rs` and the rest failed with
  `couldn't read examples/two-devices-one-seat/tests/../src/domain.rs`.)
- Green: `domain`, `uplink`, `timeline`, `scenario`, `readback` and `story` written. All 11 passed
  on the first full run.
- Mutation check of test 8, standing in for its red: `adjudicate` changed to refund the
  *earliest* held sale (`.take(n)` for `.skip(keep)`) →
  `story::the_refunded_sale_is_the_last_to_reach_central` failed with
  `assertion left == right failed: tablet reached central first; left: ["central-1"],
  right: ["phone-1"]`. Reverted.
- Guide page: the three fences passed on their first run. Mutation check: the restored edge's
  `stepped.pushed` expectation changed from 0 to 1 → `narrative::sync_a_device (line 267) FAILED,
  assertion left == right failed: nothing above the watermark to push`. Reverted. This shows the
  fences execute and are not only compiled.

### Review round 1 (changes-requested at fingerprint 75f3476b)

- [x] F1 (TST-05): `timeline::tests::the_view_is_the_same_under_any_delivery_order`, a unit test
  so it can build the events with the real decisions and open a `Site`. Passes on the shipped
  `apply`. Red by mutation, each applied and reverted: (a) `SeatSold`'s `DO UPDATE` also setting
  `status = 'held'`, and (b) `SaleRefunded` as `UPDATE seat_ledger SET ... WHERE sale = ?1` with
  no stub. Both → `assertion left == right failed: refund-first: a different table`, while
  `--test story` stayed `11 passed` under each. The Risks bullet above is corrected.
- [x] F2: `Link::PullOnly` loses only the push route (and any route a later version adds, since
  `Route` is non-exhaustive). `story::a_lost_push_is_transient_and_the_next_step_delivers_it`
  asserts `failure.phase == Phase::Push`, `Transient`, edge `Running`, the pull landed, central
  short, then `landed == 1` on the next sync. Registered in `proof.rs`; `narrative.rs`'s claim
  about the guide fence narrowed to what a pull-phase failure shows.
- [x] F3: `CentralError = <Central as EventStore>::Error`; `Handset::linked_to`,
  `UplinkError::Serve`, `StoryError::{Schedule, Adjudicate}` go through `Central`. `Site` and the
  per-place `StoryError` variants stay SQLite (making `Site` generic over central's store is
  not clean), so `uplink.rs` and `overview.md` now list exactly what a Postgres central changes.
- [x] F4: modules private, one path per item through `lib.rs`. Items in public signatures that
  were reachable only by module path are re-exported (`Row`, `Status`, `RefundReason`, `Rule`,
  `Sale`, `UnknownPlace`, `DeviceStep`, `Central`, `CentralError`, `CentralPeer`, and the three
  decisions); `DDL` and `SALE_REFUNDED` became `pub(crate)`.
- [x] F5: `#[serde(tag = "type", rename_all = "snake_case")]` on `BoxOffice`, the v2 upcast
  described on the type, and `domain::tests::the_payload_names_its_type_in_snake_case` pins the
  bytes.
- [x] F6: AGENTS.md gained "Public surface" and "Error types". Root `CLAUDE.md` not touched.
- [x] F7: `StoryError` and `DeviceStep` moved to `src/error.rs`.
- [x] F8: `seat_ledger` keyed by `(show, sale)`.

### Deviations from the design above

- `Device { Tablet, Phone }` and `Place { Central, Tablet, Phone }` instead of one `Device` with a
  `BoxOffice` variant: `sync` and `set_link` take a `Device`, so central cannot be asked to sync
  with itself (API-02).
- `Seats(NonZeroU32)` dropped. Capacity is a `u32` on the event; a zero-seat show refuses every
  sale, which is correct, so the newtype bought nothing.
- `SaleId` is not a tag. Nothing queries by sale, and dropping the tag removed the only fallible
  call from the decision. The decision mints it from the fold; `Scenario::sell` reads it back
  through the `FnMut` closure. `StoryError::NothingSold` covers the committed-nothing case rather
  than assuming it away.
- `SaleRefunded` carries no `buyer`; the read model takes buyer and place from the sale row.
- `Synced` counts what *landed* (new events at the destination), not what was sent, because a
  device pushes its whole log including central's own events (SY-27) and "sent 3" said nothing.
- `Converged` has no separate `central_id`; each `Replica` carries its own `store`.
- `StoryError::Contradicted { claim }`: the story fails rather than print a claim that did not
  happen (offline sync succeeding, a second refund, files that disagree, the phone selling again).
  The binary therefore checks itself, as `transfers-on-sqlite`'s `bail!` does.
- `StoryError::Schedule` added beside `Adjudicate` (both `Infallible` decisions).
- `tickets-over-http`'s `src/lib.rs` and `Cargo.toml` each claimed to be the only example with a
  library target; both reworded in this change, since this change makes them false.

### Proposed CLAUDE.md text (owner to apply; CLAUDE.md is protected)

New repository-map entry, placed directly after the `examples/tickets-over-http/` entry:

```
examples/two-devices-one-seat/   the flagship. two devices sell the last seat offline; sync
                                 brings both sales to a central store (SY-1), which keeps its
                                 rule with one ordinary append (SY-2) by convention until #450;
                                 three SQLite files converge on one log and one read model,
                                 read back after every handle is dropped. RemoteSyncPeer over
                                 an in-process Exchange into serve_sync. lib + bin, like
                                 tickets-over-http, so its tests drive the story step by step.
```

Replacement for the tickets-over-http entry's last line. Current:

```
                                 roles, and the only lib target under examples/.
```

Proposed:

```
                                 roles, and one of the two lib targets under examples/.
```

### Gate

- `cargo xtask affected --base origin/main`: see the report; recorded here once run.
