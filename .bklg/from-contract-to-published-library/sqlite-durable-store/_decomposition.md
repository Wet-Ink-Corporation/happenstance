# Briefs — The first adapter that is not an instrument (HS-P0012, sqlite-durable-store)

Companion file to `project.md`, `_intake-brief.md` and `_grounding.md`. Not an
item file: no frontmatter, no `redkiln` write-path constraints. It holds this
project's warranted briefs — **architecture** and **testing**, per the
decomposition's brief table. Each brief follows
`.redkiln/templates/briefs/brief.md` (Intent / Acceptance Criteria / Notes)
inside its own section.

---

## Architecture brief

### Intent

Turn `crates/happenstance-sqlite/` from a compiled instrument into a store that
has **passed** `happenstance_testkit::event_store_conformance!`, by settling —
before any SQL is written — the six architectural questions the skeleton left
open and the two the skeleton got wrong. This brief decides: **where the adapter
mounts** (which test targets are the composition root, and which sibling
contracts they wire to), **how a blocking driver acquires its runtime** (the one
seam that decides whether the concurrency family can run at all), **the read
snapshot ceiling** that ADR-0011 already requires and the skeleton does not
carry, **how a wide `Query` is decomposed** without minting new public API in a
frozen contract crate, **what the schema must hold** beyond the crate's own
(known-wrong) sketch, and **what stays out** — `publish`, the projection suite,
the clause homes.

It decides nothing that an Accepted decision atom already decided, and it writes
no production code. Where it takes a position that ADR-0022 must ratify, it says
so and names the alternative that lost.

### Acceptance Criteria

Architecture-grain, numbered `AC-A0n` so they cannot be confused with the
project's own `AC-001 – AC-016`. Each names the project AC it serves.

- **AC-A01 — the mount is a real test target, not a compiled module.** Every
  capability this project builds is reachable from
  `crates/happenstance-sqlite/tests/` through a testkit macro invocation, not
  from a `mod` that only `cargo check` ever sees. `shapes.rs` (the existing
  type-level target) still passes unchanged in intent after the store and stream
  gain fields. *(AC-001, AC-014)*
- **AC-A02 — the runtime seam is decided before the first `todo!()` is
  replaced.** The adapter's answer to "where does `spawn_blocking` get its
  runtime when the caller is a bare OS thread" is recorded in ADR-0022, and it is
  an answer that lets `event_store_concurrency_conformance!` run rather than
  return `NoRuntime` from every contender. *(AC-005, AC-007, project.md risk 1)*
- **AC-A03 — `ReadCursor` carries a snapshot ceiling.** A ceiling is captured no
  later than the first `poll_next` and bounds every subsequent page statement.
  This is discharge of an Accepted ADR, not a design choice. *(AC-001, ADR-0011)*
- **AC-A04 — no new public API lands in `happenstance-core` for this adapter's
  convenience without an ADR that says so.** Query pushdown decomposition is
  adapter-private unless ADR-0022 explicitly argues the opposite. *(AC-008)*
- **AC-A05 — `publish = false` is still in
  `crates/happenstance-sqlite/Cargo.toml` at the end of this project, and
  `PUBLISHABLE` in `xtask/src/package.rs` is unchanged.** *(AC-015)*
- **AC-A06 — nothing added to `happenstance-testkit` costs the `wasm32` or
  `!Send` flavour anything.** The `bench` surface is off by default *and*
  target-gated, and it adds no entry to `for_each_event_store_rule!`.
  *(AC-012, DR-08)*
- **AC-A07 — every capability the fixture declares is backed by a mechanism
  that could fail.** `SECOND_HANDLE` is a second `rusqlite::Connection` on one
  file; `REOPEN` drops and re-opens; the three limit constants are numbers the
  store actually enforces in `append`. *(AC-003, AC-004, AC-009)*
- **AC-A08 — the DoD 7 batch-shape question is answered in writing here, before
  code.** *(AC-011)*

### Notes

#### 1. The composition root: what mounts where

A library has no render tree, so the equivalent question is *which target
actually executes this code, against which sibling's contract*. An adapter that
compiles is not an adapter (`CLAUDE.md`, "The rule that matters"). The mount
points, and the real contracts each wires to:

| Mount (file) | New? | What it mounts | The sibling contract it wires to |
| --- | --- | --- | --- |
| `crates/happenstance-sqlite/tests/conformance.rs` | new | `SqliteFixture` + `happenstance_testkit::event_store_conformance!(SqliteFixture::new())` | `crates/happenstance-testkit/src/contract.rs` (`Fixture`), `crates/happenstance-testkit/src/registry.rs:94` (`for_each_event_store_rule!`) |
| same target (or a sibling target) | new | `event_store_model_conformance!` | `crates/happenstance-testkit/src/model.rs:782` |
| same target (or a sibling target) | new | `event_store_concurrency_conformance!` | `crates/happenstance-testkit/src/concurrency.rs:1129`, whose bound is `ConcurrentFixture` (`concurrency.rs:186`) |
| `crates/happenstance-sqlite/tests/shapes.rs` | exists | `SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync`, error bounds | the port's own shape; this is the guard that catches a field added carelessly |
| `crates/happenstance-sqlite/tests/projection.rs` | new | `SqliteProjectionStore` against the projection suite | **owned by `projection-store-freeze` (HS-P0010)** — this project consumes the macro that project ships and does not name it here, because it does not exist yet |
| `crates/happenstance-testkit/src/bench.rs` + a `bench` feature | new | `event_store_benchmarks!` and its own enumeration | inherited by adapters the way conformance is; **never** a rule (CF-34) |
| `crates/happenstance-testkit/src/concurrency.rs:206` | exists | `CONTENDERS` | every fixture in the workspace, which is why raising it is a workspace-wide cost |
| `.kb/decisions/` + `references/adr/` | new | ADR-0022 (atom + long record) | the runbook's ADR queue — never authored as a side effect |
| `spec/SPECIFICATION.md` | exists | ES-35, CF-14, CF-17, VT-21 – VT-24 markers | `cargo xtask spec-trace`, a gate step |
| `crates/happenstance-sqlite/{README.md,LICENSE-MIT,LICENSE-APACHE}` | new | the packaging facts | `xtask/src/package.rs` — which will **not** check them while `publish = false` stands (see §8) |

Three integration rules follow, and they are the ones an implementer building in
isolation gets wrong:

- **`SqliteFixture` lives in `crates/happenstance-sqlite/tests/`, not in
  `crates/happenstance-testkit/src/fixtures.rs`.** The testkit is published, is
  built for `wasm32` by `cargo xtask wasm`, and its `[dependencies]` today are
  `happenstance-core` and `futures-core` and nothing else
  (`crates/happenstance-testkit/Cargo.toml`). Putting a `rusqlite`-backed fixture
  in it would put a bundled C library into the dependency graph of the crate
  whose whole value is that it needs nothing. `happenstance-sqlite` already
  dev-depends on the testkit, so the fixture has everywhere it needs to be.
- **`MemoryFixture` is the shape to read and the shape *not* to copy.**
  `crates/happenstance-testkit/src/fixtures.rs:243-292` is the reference
  implementation, and its `connect` is an `Arc` clone. AC-003 rejects that shape
  explicitly: one `SqliteFixture` instance owns **one file path**, and each
  `connect()` calls `SqliteEventStore::open(path)` — a *new*
  `rusqlite::Connection`. Two instances get two paths.
- **`SqliteEventStore::open_in_memory` (`event_store.rs:132`) is the wrong
  constructor for the fixture, and it is the one an implementer reaches for
  first.** A private in-memory database is per-*connection*, so a second
  `connect()` would open a second, empty database and
  `two_handles_observe_each_others_appends` would fail — which is a MUST that
  panics rather than skips (`contract.rs:135-161`). It stays as a convenience
  constructor for single-handle callers; the fixture must not use it.

#### 2. Accepted decisions that bind this work

Cited by path. None of these is open, and none is this project's to amend.

- **`.kb/decisions/0011-read-laziness-and-isolation.md`** — the read snapshot.
  "An adapter issuing more than one statement per `read` must capture a position
  ceiling no later than the first poll and bound every later statement by it."
  `PAGE_SIZE = 512` (`crates/happenstance-sqlite/src/event_store.rs:81`) means
  more than one statement for any log over 512 events. **`ReadCursor`
  (`event_store.rs:307-328`) has no ceiling field. Adding one is required, not
  discretionary.** See §4.
- **`.kb/decisions/0012-append-shape-and-preconditions.md`** — `append` keeps
  `events: &[Event]`; an empty batch is refused (`AppendError::NoEvents`,
  `crates/happenstance-core/src/error.rs:219-225`) **before** any condition is
  evaluated; a condition is evaluated only against events the store already held
  when `append` began, never against the batch being written. The atom also names
  this project as the measurement that lifts its own `[PROVISIONAL]` marker, and
  fixes the successor question as binary: keep the borrow, or move to
  `Vec<Event>`. Nothing else.
- **`.kb/decisions/0013-position-assignment-and-visibility.md`** — the global
  visibility invariant. Its concrete corollary here is already written into the
  skeleton: `head` queries fresh every time and must never become a cached field
  `append` updates (`event_store.rs:226-235`), because one file backs several
  handles.
- **`.kb/decisions/0015-validated-identifiers-and-store-limits.md`** —
  `AppendError::ExceedsStoreLimit { limit, len }` with three `StoreLimit`
  variants, and "a capacity limit must never be enforced by a constructor and
  must never be enforced in `Deserialize`". For this adapter that means the three
  ceilings are checked inside `append`, before the transaction opens.
- **`.kb/decisions/0009-error-send-sync.md`** — `EventStore::Error` stays exactly
  `core::error::Error + 'static`. `SqliteEventStoreError` happens to be
  `Send + Sync` and `tests/shapes.rs` asserts it; that assertion is about what
  *callers* can do, and no `Send + Sync` bound may migrate onto the port.
- **ADR-0001 / ADR-0008** (`CLAUDE.md` constraints 1 and 3) — `read` returns the
  stream at the top level and is not `async`. The hand-written state machine at
  `event_store.rs:248-426` is the concrete instance, and its doc comment
  (`:248-271`) explains why no `Statement`, `Rows` or `Transaction` may appear in
  a field. Every field this project adds must keep that true;
  `tests/shapes.rs::read_stream_is_send_and_unpin` is the guard.

#### 3. The runtime seam — the decision that unblocks the concurrency family

This is the largest architectural risk in the project and the one most likely to
be discovered in a red run rather than decided. `project.md`'s first risk states
it for the read path; **it is wider than that.**

`event_store_concurrency_conformance!` expands to
`#[tokio::test(flavor = "multi_thread")]`, but the contenders are **bare OS
threads under `std::thread::scope`**, each driving its own future with the
testkit's own park-loop `block_on` (`concurrency.rs:45-68`). `tokio`'s runtime
context is thread-local, so inside a contender `Handle::try_current()` fails. The
skeleton turns that into `SqliteEventStoreError::NoRuntime` by design
(`event_store.rs:26-32`, `:174-178`) — a good design for a stray
`futures::executor::block_on`, and fatal here, because **every** store method
that reaches SQLite will want a blocking hop, not only `read`:

- writers call `store.append(...)` inside `crate::block_on` on a scoped thread
  (`concurrency.rs:884-895`, `:661`);
- the reader in `observe_while_writing` calls
  `happenstance_core::collect(store.read(...))` on its own scoped thread
  (`concurrency.rs:948-953`), and a failed read there is **reported as a
  sighting** — `"a concurrent read failed: {err}"` — so a `NoRuntime` does not
  even surface as an obviously-wrong failure; it surfaces as a conformance
  verdict about atomicity.

Two live options. **ADR-0022 must record which, and why.**

- **(a) Capture a `tokio::runtime::Handle` at construction, prefer it, and keep
  `try_current()` as the fallback.** `SqliteEventStore::open` / `::new` run on
  the harness's own thread, which *is* inside the tokio test, so
  `Handle::try_current().ok()` there yields a handle; a `Handle` is `Clone`,
  `Send`, `Sync` and `Unpin`, so carrying one in the store, in `ReadCursor` and
  in `SqliteReadStream` costs the shape assertions in `tests/shapes.rs` nothing.
  `NoRuntime` keeps a real meaning — a store both constructed and driven with no
  runtime anywhere — so the variant and its doc comment stay honest.
- **(b) Run the statement inline on the calling thread when no runtime is
  found.** Defensible for a synchronous driver: "no runtime" means there is no
  executor thread to starve. It makes `NoRuntime` unreachable, which means the
  variant, `event_store.rs:27-32`'s module-doc paragraph and
  `SqliteProjectionStoreError::NoRuntime` all have to be removed or rewritten in
  the same change rather than left as documentation of a state that cannot occur.

**This brief recommends (a)** and asks ADR-0022 to record (b) as the alternative
that lost, because (a) keeps every existing comment in the crate true, keeps
blocking work off an async executor's thread whenever there *is* one, and is the
smaller diff to the state machine. The implementer may argue for (b) in the ADR;
what is not available is discovering the problem in a red concurrency run.

Consequence worth stating: whichever wins, this is the same seam
`concurrency.rs:61-68` predicted ("phase 10's adapter will need the *spawn* to
become a parameter the harness supplies"). This project solves it **inside the
adapter**, not by changing the testkit's bound — tightening
`ConcurrentFixture`'s bound is a breaking change to a published crate for the
benefit of one adapter, and `postgres-and-neon-stores` (HS-P0014) is the project
that will have the evidence for whether the seam generalises.

#### 4. The read path

- **The ceiling (required by ADR-0011).** `ReadCursor` gains one field — the
  highest position this read may ever yield — captured on the *first*
  `fetch_page`, under the same connection lock, before the first page's rows are
  selected. Every subsequent page statement carries it as an upper bound. Without
  it, a replay that pages 512 rows at a time silently absorbs writes committed by
  a second connection between hops, which is precisely the "one state sampled no
  later than the first poll" promise ADR-0011 makes.
- **It composes with, and does not replace, `ReadOptions::to`.** The effective
  upper bound is the tighter of the caller's `to` and the ceiling; under
  `backwards`, the ceiling bounds the *starting* end. `read_to_is_inclusive`,
  `read_from_and_to_bound_a_closed_window` and
  `read_to_under_backwards_bounds_the_older_end` are the rules that will notice a
  confusion here (`crates/happenstance-testkit/src/registry.rs`, Read options).
- **`resume_from` stays inclusive.** `event_store.rs:313-323` records a real
  historical bug — the field was `resume_after`, seeded from an inclusive
  `ReadOptions::from` and advanced to `last.position`, so every page boundary
  re-read a row. `AppendCondition`'s `after` is the exclusive one in this
  contract. Do not "tidy" the two into one sense.
- **`advance()` (`event_store.rs:351-368`) is already written and already
  correct** for budget and direction; the ceiling is an extra `WHERE` term, not a
  change to that function's arithmetic.
- **Lazy means lazy.** `read` still executes nothing —
  `event_store.rs:203-215`'s comment is load-bearing, and CF-33's no-clock
  discipline plus ADR-0001's non-`async` `read` are what force it.

#### 5. Query pushdown, chunking, and the `index_arms()` question

`RUNBOOK.md:4203-4206` and `references/evaluation/ARCHITECTURAL-EVALUATION.md:827`
both write the work item as "handle `Query::index_arms()` exceeding SQLite's
pushdown limits". **That API does not exist.**
`crates/happenstance-core/src/query.rs` has `Query::items() -> Option<&[QueryItem]>`
(`:204`), `QueryItem::types()` (`:102`) and `QueryItem::tags()` (`:107`), and no
`index_arms`, `arm_count` or `IndexArm`. No decision atom mints them; the
evaluation *proposes* them.

**Decision: option (b) — decompose privately inside `happenstance-sqlite`.**

- AC-008 requires the *behaviour* (chunked, not refused; VT-23's 128-item floor
  passing via `store_evaluates_a_query_at_the_guaranteed_minimum_item_count`),
  and never the public name.
- `happenstance-core`'s `EventStore` is `[FROZEN]` (`spec/SPECIFICATION.md:371`).
  `Query` is not the same item, and the addition would be purely additive — but
  adding public surface to the contract crate between the alpha and `0.2.0` for
  the benefit of *one* adapter is exactly the move `CLAUDE.md` warns about: "a
  port is only as well-designed as the spread of what implements it." One
  implementor is not a spread.
- The re-open trigger is named rather than left to memory: if
  `postgres-and-neon-stores` independently needs the same decomposition, that is
  two unlike storage shapes agreeing, and *that* is the evidence to mint
  `index_arms()` in `happenstance-core` — as its own ADR, then, not as a side
  effect of this one.

ADR-0022 must record this explicitly, because the runbook's own work item names
the API and a reader will otherwise think it was forgotten.

Shape of the private decomposition (non-prescriptive; the implementer owns the
details):

- an *arm* is one prepared-statement branch derived from a `QueryItem` after the
  most-selective-tag choice — `Query::All` is one arm (a bare scan over `event`),
  and an item's arm count is a function of its type set and tag set;
- arms are chunked into `ceil(arms / N)` statements and their cursors merged in
  Rust. `RUNBOOK.md:4204` proposes `N = 400`; the real ceiling is SQLite's
  compiled-in compound-select and bound-parameter limits. Probe them at open if
  the driver exposes them (note `rusqlite` is pulled with
  `default-features = false, features = ["bundled"]` in the workspace manifest,
  so a limits API may need a feature added — weigh that against documenting the
  bundled defaults in ADR-0022);
- **the merge is where the query rules actually get decided**, not the SQL. Each
  chunk statement must be ordered by position and bounded by the same ceiling and
  `resume_from`; the merge is a k-way merge that **de-duplicates on position**
  (`duplicate_items_do_not_duplicate_events`, `query_items_are_or`,
  `query_union_is_item_concatenation`) and applies the page budget and
  `ReadOptions::limit` to the **merged** output, never per chunk
  (`limit_applies_across_items_not_per_item`,
  `read_limit_applies_after_filtering`).

#### 6. The write path

- **Preconditions, in order, before the transaction opens.** Empty batch →
  `AppendError::NoEvents` (this ordering is a rule:
  `empty_batch_is_refused_before_the_condition_is_evaluated`). Then the three
  store limits → `AppendError::ExceedsStoreLimit`, never `AppendError::Store`,
  never a truncation (`error.rs:227-244`; ADR-0015).
- **One `BEGIN IMMEDIATE` transaction, probe then insert.** The condition is a
  non-empty sequence of `Guard { query, after }`; each guard is an `EXISTS` probe
  for a match strictly above its `after`, evaluated *before* any row of this
  batch is inserted (ADR-0012). `BEGIN IMMEDIATE` is what takes the write lock at
  the top, and it is the whole reason the probe and the insert are one decision.
- **The named wrong implementation, and why `BEGIN DEFERRED` is it.** AC-007
  names read-then-write. Concretely in SQLite that is `BEGIN DEFERRED` (or no
  transaction at all): a lost race then arrives as a driver error rather than as
  `AppendError::ConditionViolated`, and the concurrency family distinguishes
  those two by construction — `Attempt::Rejected` versus `Attempt::Failed`
  (`concurrency.rs:214-231`). A store that reports `Failed` where the suite
  expects `Rejected` fails the racing rules; a store that reports neither because
  it probed outside the lock fails "exactly one winner" within a handful of
  iterations.
- **`SQLITE_BUSY` is an architecture problem, not a tuning knob.** With
  `CONTENDERS` connections on one file, `BEGIN IMMEDIATE` on a busy database
  returns `SQLITE_BUSY` *immediately* unless a busy handler is configured, and
  that error becomes `AppendError::Store` → `Attempt::Failed` → a red rule that
  is not about the adapter's logic. The adapter therefore configures a busy
  timeout and records the value in ADR-0022. Two things to hold together: CF-33
  forbids a **rule** reading a clock and says nothing about an adapter's own
  retry policy; and there is **no watchdog anywhere in the suite** (CF-33,
  `concurrency.rs:24-43`), so an *unbounded* busy handler converts a livelock
  into a hung CI job that names no rule. Finite and generous, not infinite.
- **Journal and durability pragmas.** WAL for two-connection concurrency, and
  `synchronous` at a setting that supports a durability claim: CF-14
  (`spec/SPECIFICATION.md:7482-7485`) names `PRAGMA synchronous = OFF` **by
  name** as a wrong implementation the reopen rule exists to reject. Whatever is
  chosen is a documented property of the adapter, in ADR-0022 and in the crate's
  README.
- **The return value is the caller's own last position**, not the store head
  (`append_returns_last_written_position`, `batch_positions_follow_slice_order`).
  `AUTOINCREMENT` is load-bearing: positions are never reused after a delete, and
  gaps are permitted — no code and no rule may assume `+1`
  (`CLAUDE.md`, "never assert on literal position values"; the testkit's own
  `GappedPositionStore` is what enforces it).

#### 7. Schema, migration and identity

The sketch in `event_store.rs:36-54` is **known to be wrong** in a way that
serialises every writer, and the correction lives in `RUNBOOK.md:4178-4187`
rather than in the code. Implementing the doc comment as written is the most
likely way to ship a slow, conformant adapter. The module doc is *published
documentation*; correcting it is part of this work, not a follow-up.

What migration 1 must hold:

- `event(position INTEGER PRIMARY KEY AUTOINCREMENT, event_type, data, metadata,
  tags, …)`;
- `event_tag(tag, position)` `WITHOUT ROWID`, key left `(tag, position)` so the
  range stays sorted by position, **with `event_type` carried as a covering
  column** — otherwise a `QueryItem`'s type constraint becomes a join back to
  `event` walked under the `BEGIN IMMEDIATE` write lock;
- a `tag_cardinality` table, because multi-tag arms must be probed
  most-selective-tag-first and SQLite cannot supply per-value cardinality
  (`ANALYZE` stores only an average);
- the `EventId` and `recorded_at` columns settled at phase 4. `EventId` is
  `EventId::new(StoreId, SequencePosition)`
  (`crates/happenstance-core/src/identity.rs:97`), so `contains_event_id`'s probe
  needs the origin store and origin position stored and `UNIQUE` **together** —
  which is both the index the probe seeks and the constraint that keeps ingest
  from storing one event twice (`event_store.rs:237-244`);
- **the store's own `StoreId`, minted once at schema creation and persisted.**
  This is what makes `reopened_store_does_not_reissue_an_event_id` and
  `recorded_time_survives_a_reopen` askable at all. VT-6 permits either mechanism
  (mint once, or mint per open); the testkit's own `DurableFixture`
  (`crates/happenstance-testkit/tests/fixture_instruments.rs:75-90`) switched to
  mint-once for exactly this reason and says so. Mint-per-open is the wrong
  choice for a file-backed store.

**Migration must be idempotent and safe under a concurrent open.**
`SqliteEventStore::open` calls `migrate` on *every* connect
(`event_store.rs:120-124`), and the fixture connects twice onto one file, so two
`CREATE TABLE`s can race. `IF NOT EXISTS` inside a `BEGIN IMMEDIATE`, and
`INSERT OR IGNORE` then read-back for the `StoreId` row. The projection store's
checkpoint schema (`projection_store.rs:33-43`) migrates independently, on its
own connection, and must be equally idempotent — an event store and a projection
store on one file are two connections, not one.

#### 8. What this project must **not** touch

- **`publish = false` stays** (`crates/happenstance-sqlite/Cargo.toml:12`).
  `RUNBOOK.md:4230`'s exit checkbox says "publish = false removed" and it is
  **stale**: AC-015 says publishing "stays someone else's decision" and
  `.bklg/from-contract-to-published-library/_decomposition.md:43` gives that
  decision to `publication-and-positioning`. Deleting the line here makes
  `xtask/src/package.rs`'s `reconcile()` fail CI the moment the crate is
  promoted while `PUBLISHABLE` (`package.rs:86`) has not been updated — the
  "promoted-but-unreconciled" failure the module's own doc comment names. Record
  the runbook checkbox as superseded by the AC; do not silently diverge from
  either.
  *Consequence to carry into the ledger:* with `publish = false` standing, the
  packaging gate **does not** check this crate. AC-015's evidence is therefore a
  recorded `cargo package -p happenstance-sqlite --list --allow-dirty` run
  showing `README.md`, `LICENSE-MIT` and `LICENSE-APACHE`, not a green gate step.
  The gate step is `publication-and-positioning`'s to earn. The stale
  `description = "... Not yet implemented."` (`Cargo.toml:3`) is this project's to
  replace; `publish` is not.
- **`PUBLISHABLE` / `REQUIRED_FILES` in `xtask/src/package.rs`** — unchanged.
- **The projection port, its `Batch` shape, the projection suite, the
  capability-declension policy (DT-3) and the adapter-author bar (DT-8)** —
  `projection-store-freeze` (HS-P0010). This project *consumes* them. PS-6
  (`begin`'s needless `async` + `Result`) and PS-7 are already marked "not this
  crate's to settle" in `projection_store.rs:235-260`; that stays true.
- **Anything `[FROZEN]`.** If contact with a real database says a frozen clause
  is wrong, that is a new decision atom and a re-plan.

#### 9. Tensions recorded, not settled

- **CF-40's clause home** (ADR-0012 vs ADR-0015) is self-contradicted inside
  ADR-0015 and is an accepted open question the KB says it has no standing to
  resolve (`.kb/open-questions/cf-40-fixture-limits-ownership.md`). This project
  needs the **capability** — a fixture stating numeric ceilings — and gets no say
  in the clause's home. Record and escalate to the ADR queue (AC-009).
- **`CONTENDERS = 8` versus the two proof artefacts' 64**
  (`crates/happenstance-testkit/src/concurrency.rs:206`; `RUNBOOK.md:159`,
  `:4217-4222`; the discrepancy is named at `RUNBOOK.md:2680-2700`). AC-005
  permits exactly two outcomes. This brief's recommendation is **raise the
  constant with a stated reason**, because (i) the constant's own doc says it is
  not a tuning knob and the rules assert set properties that hold at any size
  above one, so no rule is rewritten; (ii) 64 is the number both artefacts
  already claim. The cost the ADR must weigh, and it is not small: the constant
  is workspace-wide, so **every** fixture — `MemoryFixture`, the mutant fixtures,
  the racers — re-runs at 64, and 64 `rusqlite::Connection`s onto one file is a
  real file-descriptor and busy-contention claim this project has to verify
  rather than assume. If verification says otherwise, amend **both** artefacts.
  Leaving it is a failure of AC-005, not a deferral.
- **DoD 7's second unlike batch shape (AC-011).** *Answer, recorded here before
  code: it does **not** live in this project.* `SqliteProjectionStore` supplies
  the owned-buffer SQL shape as a *consumer* of the suite
  `projection-store-freeze` freezes; taking the second shape here would give the
  DAG an edge it does not carry and invert the 6 → 8 order
  (`.bklg/from-contract-to-published-library/_decomposition.md:295-300`;
  `project.md`, risk 3). The second unlike shape belongs with the testkit, beside
  the suite it validates. If `projection-store-freeze`'s own architecture brief
  assigns it here instead, that is a **blocking re-plan** — a new dependency edge
  before implementation starts — not something this project absorbs quietly.
- **ES-35 / CF-14 / CF-17 (AC-010).** This adapter supplies the *reopen* far end
  and does **not** supply the *fault* far end: `MID_BATCH_FAULT` defaults to
  declined (`contract.rs:207-211`) and nothing in this project's ACs asks for it.
  The fixture should decline it **explicitly, with the real reason**, in the same
  shape `MemoryFixture` uses for `REOPEN` (`fixtures.rs:275-284`), so the skip is
  stated rather than inherited. That is also the honest verdict for ES-35's
  `[PROVISIONAL]` marker: this project can freeze the *reopen* half and must
  restate the falsifier that remains — "a store that loses a write to a fault
  rather than to an instruction" — rather than declare the axis closed.
- **`recorded_time_survives_a_reopen` has had no negative control reaching its
  headline assertion since phase 4** (`RUNBOOK.md:3205-3207`). If it passes on
  the first try, that is a claim to check, not a result to accept
  (`CLAUDE.md`: "a rule that no adapter can fail is decorative"). The testing
  brief owns how; the architectural precondition is §7's persisted `StoreId` and
  a `recorded_at` that is read back rather than re-stamped.

#### 10. The testkit surface this project adds, and its blast radius

`event_store_benchmarks!` goes **in the testkit** (AC-012, CF-34,
`RUNBOOK.md:4207-4213`), and three structural constraints decide its shape:

- **Its own enumeration, in its own module.** `model.rs` and `concurrency.rs`
  each carry their own `for_each_*_rule!` beside their rules, and
  `concurrency.rs:138-144` states why: `cargo xtask spec-trace` scans `suite.rs`
  for `pub async fn`, so anything written there needs a clause. Benchmarks must
  not be in `suite.rs` and must not appear in `for_each_event_store_rule!` — that
  is what makes AC-012's "the conformance rule count is unchanged by its arrival"
  structurally true rather than asserted.
- **Feature-gated *and* target-gated.** A `bench` feature off by default is not
  sufficient: `fixtures.rs:298-305` records that **a feature is not
  target-scoped**, so `--all-features` sets it on `wasm32` too, and the gate runs
  a `wasm32` feature-powerset check. Gate the module the way `concurrency` is
  gated — it does not exist on `wasm32` — or make it compile there. Either
  answer, deliberately.
- **The emitter stays a parameter** (CF-23). Any measurement dependency belongs
  in the *adapter's* dev-dependencies, reached through a caller-supplied emitter,
  not in the testkit's `[dependencies]`.

**On AC-006 ("the phase-3 mutant harness re-run with `SqliteEventStore` in the
pass column").** The pass-column meta-test is
`mutation_coverage::conformant_variants_pass_everything`
(`xtask/src/proof.rs`, `META_TESTS`), and its stores are built from
`tests/mutation_coverage/correct.rs`'s `Rc`/`RefCell` primitives inside the
testkit's own test binary. Registering `SqliteEventStore` *there* would make the
published testkit dev-depend on an adapter that dev-depends on it, and would drag
`rusqlite` plus a tokio runtime into a harness deliberately built to need
neither (`harness.rs:19-35`) — and it hard-requires §3's option (a) to work at
all. **Recommended reading: discharge AC-006 from the adapter's own test target**
— every registered rule driven against a conformant `SqliteFixture` *is* what
`conformant_variants_pass_everything` asserts, and that is exactly what
`event_store_conformance!(SqliteFixture::new())` does. If a reviewer requires
testkit-side registration instead, it must be `cfg(not(target_arch = "wasm32"))`
dev-dependencies (the pattern the testkit's `Cargo.toml` already uses for tokio),
and §3(a) becomes mandatory rather than recommended.

Separately: this project adds **no** conformance rule as such — but AC-010
settles **CF-17's rule shape**, and if that reshapes or adds a rule,
`mutation_coverage::every_rule_has_a_mutant` fails until a `REGISTRY` row exists
in `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`
(`crates/happenstance-testkit/README.md:108-113`). The row is not optional and
not a follow-up.

#### 11. Deliberately left to the implementer

Not under-specified — these are calls the compiler, a measurement or a review is
better placed to make than a planning document:

- the exact SQL for the probe and the page query, and whether the guard probes
  are one compound statement or one per guard;
- `PAGE_SIZE`'s final value (`event_store.rs:81` calls the current 512 "a
  placeholder until it is measured") and the chunk width `N`;
- the busy-timeout value, the `synchronous` setting, and whether WAL is set per
  connection or persisted in the file;
- the three declared ceilings' numbers, subject to one hard constraint the
  planning document *can* see: `append_reports_exceeded_store_limits` appends a
  payload of exactly `MAX_EVENT_DATA_LEN` bytes and then one byte more
  (`contract.rs:239-247`), so a ceiling of SQLite's own ~1 GB makes the suite
  unrunnable. An adapter-enforced policy ceiling the suite can afford, above
  VT-21's 65,536-byte floor (`crates/happenstance-core/src/limits.rs:22`), is the
  shape that works — and the number is a **fact about this adapter** that must be
  documented in the README and enforced in `append`, not a trade;
- how the fixture owns its temporary directory. `tempfile` is **not** in
  `[workspace.dependencies]` today, so adding it is a workspace manifest change
  that `cargo deny` will see (`deny.toml`; the gate runs it). A process-local
  ordinal plus a `Drop` cleanup needs no dependency and has precedent in
  `fixture_instruments.rs:95-102`. Either is fine; the manifest consequence is
  not optional to think about.

#### 12. What ADR-0022 must carry

Written **before** the implementation (AC-013, DR-06), answering one question,
naming what lost, and quoting a measured number rather than a preference
(`RUNBOOK.md:4227-4228`). Authored through the runbook's ADR queue as an atom
under `.kb/decisions/` plus the long record under `references/adr/`, per
`CLAUDE.md`'s two-places rule — never as a side effect of a code change.

At minimum it records: the append-condition strategy and the two candidates from
`crates/happenstance-sqlite/src/lib.rs:56-62` that lost, with the benchmark
figure; tag storage, and why the join table beat the canonical blob and JSON1
(`lib.rs:63-65`); the amended schema and why the crate's own sketch was wrong;
the runtime seam (§3) and its rejected alternative; the `index_arms()` rejection
and its re-open trigger (§5); the busy timeout, `synchronous` and journal mode;
the `CONTENDERS` resolution (§9); and — since ADR-0012 names this project as its
own lifting measurement — the binary verdict on `&[Event]` versus `Vec<Event>`,
which is a *finding this project supplies*, not a change it makes.

---

## Testing brief

### Intent

Prove — not assert — that `crates/happenstance-sqlite/` has stopped being an
instrument. Every tier below answers the same question the architecture brief
asked in §1: *which target actually executes this code, against which sibling's
contract*, plus the two questions a conformance suite cannot ask of itself —
whether a rule can fail (`CLAUDE.md`, "a rule that no adapter can fail is
decorative") and whether the merge-gate command that is supposed to run it,
does. This brief maps every `project.md` `AC-001` – `AC-016` to the tier and
command that proves it, names the fixtures/seams a test double is legitimate
for (deliberately narrow — a store adapter's whole job is to *not* be doubled),
and states the mutation-registry and ledger obligations that make a green run
evidence rather than a claim.

It writes no test code. It decides which macro, which target, which command,
and which rows a reviewer checks for.

### Acceptance Criteria

Testing-grain, numbered `AC-T0n` so they cannot be confused with the project's
own `AC-001 – AC-016`. Each names the project AC(s) it discharges.

- **AC-T01 — every project AC has a named tier and a named command**, tabulated
  below, with no `AC-001` – `AC-016` left to "the suite, generally." *(all)*
- **AC-T02 — the merge-gate commands are the repository's own**, not invented
  for this project: `cargo xtask ci --fast` for this non-terminal project's
  integration grain, `cargo xtask affected --base {{base}}` for the story
  grain, both wired from `.redkiln/config.yaml:40,55` rather than typed fresh
  per story. *(AC-014)*
- **AC-T03 — no test in this project doubles `rusqlite` or the filesystem.**
  The only legitimate seam is the emitter parameter for `event_store_benchmarks!`
  (CF-23); everything else runs against a real SQLite file, because a doubled
  driver is exactly the "compiles but never ran" failure mode this project
  exists to retire. *(AC-001 – AC-011, by construction)*
- **AC-T04 — the mutation registry gains a row before AC-006 is claimed done**,
  and the row names a real defect this schema can produce, not a placeholder.
  *(AC-006)*
- **AC-T05 — every AC-001 – AC-016 has cited evidence in its story's
  `_ledger.md`**, per `require_ledger: true`
  (`.redkiln/config.yaml:67`) — a green gate is a precondition for looking at
  the criteria, never a substitute (`RUNBOOK.md:38-42`, restated in
  `project.md` DoD 4). *(all)*
- **AC-T06 — `recorded_time_survives_a_reopen` gets a negative control before
  this project claims AC-004.** *(AC-004)*

### Notes

#### 1. The test mix, and where each tier runs

Four tiers, in ascending cost, each pinned to a real `cargo` invocation and a
real target — no tier here is a description of intent with no command behind
it.

| Tier | What it is | Where it runs | Command |
| --- | --- | --- | --- |
| **Static** | `fmt`, `clippy -D warnings`, `spec-trace`, the five file-reading lints, the mutation-registry shape check | workspace-wide, package-scoped by diff | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`); `cargo xtask affected --base {{base}}` (`affected_gate`, `:40`) |
| **Unit / type-level** | `SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync`, error bounds — the shape guard that catches a field added carelessly before any SQL runs | `crates/happenstance-sqlite/tests/shapes.rs` (exists; architecture brief §1) | `cargo test -p happenstance-sqlite --test shapes` (subsumed by `cargo xtask ci --fast`) |
| **Conformance / property (this project's real proof)** | `event_store_conformance!`, `event_store_model_conformance!`, `event_store_concurrency_conformance!`, the reopen rules, and `SqliteProjectionStore` against the projection suite | `crates/happenstance-sqlite/tests/conformance.rs` (new), a sibling target for concurrency if the macro's own doc recommends process isolation, `crates/happenstance-sqlite/tests/projection.rs` (new) — the same mounts the architecture brief's §1 table names | `cargo test -p happenstance-sqlite` inside `cargo xtask ci --fast` |
| **Mutation coverage (the check on the checker)** | Confirms a rule can fail: the phase-3 mutant harness re-run with `SqliteEventStore` reachable, per architecture brief §10's "discharge AC-006 from the adapter's own test target" reading | `crates/happenstance-testkit/tests/mutation_coverage.rs`'s `REGISTRY` (`xtask/src/proof.rs:79-90`), rows in `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` | `cargo xtask ci --fast` runs the meta-tests named at `xtask/src/proof.rs:86-90`, including `mutation_coverage::every_rule_has_a_mutant` and `mutation_coverage::conformant_variants_pass_everything` |
| **Benchmark (never conformance)** | `event_store_benchmarks!` behind the `bench` feature — ADR-0022's evidence, not a pass/fail gate | `crates/happenstance-testkit/src/bench.rs` + adapter dev-dependency emitter | run manually / in ADR authoring, **not** wired into any `verify:` command — CF-34 forbids it appearing in `for_each_event_store_rule!` and this project must not add it to any gate script either |
| **Whole-gate (out of scope for this project)** | the full `cargo xtask ci`, feature powersets, `cargo deny`, docsrs, MSRV | `closeout-and-durable-audience` | `e2e: "cargo xtask ci"` (`.redkiln/config.yaml:60`) — this project's own bar stops at `integration_scoped` |

There is no separate "integration test" tier beyond conformance/property:
the conformance suite *is* this adapter's integration test, because its
target is a real file on disk through a real `rusqlite::Connection` — the
architecture brief's AC-A07 requirement that every declared capability be
backed by a mechanism that could fail is what makes a conformance run here
integration-grade rather than unit-grade. There is likewise no product-level
"e2e" tier for a library with no UI; `project.md`'s DoD item 5 names
`cargo xtask ci --fast` as this project's terminal command, and `closeout-and-
durable-audience` (HS-P0019) owns the initiative-wide `cargo xtask ci`.

#### 2. AC-to-tier traceability

Every `project.md` acceptance criterion, its tier, and the concrete rule or
command that proves it. Where an AC is a *decision* rather than a *behaviour*
(ADR content, a recorded verdict), the "test" is the reviewer checking the
artifact exists and is cited — named here so no AC is silently left without a
proof mechanism.

| Project AC | Tier | Proof |
| --- | --- | --- |
| AC-001 (suite runs, whole) | Conformance | `event_store_conformance!(SqliteFixture::new())` green; every entry in `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`) reports `Ran` or `Skipped{reason}` — never absent. Checked by reading the run's own output, since `registry.rs:413-423` already asserts no rule is orphaned from the macro at the workspace level. |
| AC-002 (instances share nothing) | Conformance | The isolation rule inside `event_store_conformance!`, run against `SqliteFixture` opening a fresh temp file per `new()` (architecture brief §1, "two instances get two paths"). |
| AC-003 (second handle is a second connection) | Conformance + code review | `two_handles_observe_each_others_appends` (a MUST, `contract.rs:135-161`) green through two distinct `rusqlite::Connection`s; the fixture's `connect()` body is the evidence, not just the green rule — a `Clone`-shaped fixture can pass the rule and fail this AC (`project.md` AC-003's own text). Review checks `SqliteFixture::connect` calls `SqliteEventStore::open(path)`, not an `Arc` clone. |
| AC-004 (reopen survives) | Conformance | `REOPEN` declared available; `acknowledged_writes_survive_a_reopen` and `recorded_time_survives_a_reopen` green across a genuine close-and-reopen. AC-T06 below is the negative-control obligation this AC inherits. |
| AC-005 (contender count is a decision) | Conformance/concurrency + ADR | `event_store_concurrency_conformance!` green at whatever `CONTENDERS` (`crates/happenstance-testkit/src/concurrency.rs:206`) ADR-0022 settles; the ADR text itself is the proof that the 8-vs-64 discrepancy (architecture brief §9) was resolved, not silently left. A reviewer checks both the green run and the ADR paragraph. |
| AC-006 (model + mutant harness) | Conformance + mutation coverage | `event_store_model_conformance!` green (`crates/happenstance-testkit/src/model.rs:782`); `mutation_coverage::conformant_variants_pass_everything` (`xtask/src/proof.rs:90`) passing with `SqliteFixture` reachable per architecture brief §10 — discharged from the adapter's own conformance run, not from a new testkit-side registration. AC-T04 is the registry-row obligation. |
| AC-007 (atomic append, probe-then-insert rejected) | Conformance/concurrency + mutation coverage | The racing rules in `event_store_concurrency_conformance!` distinguishing `Attempt::Rejected` from `Attempt::Failed` (`concurrency.rs:214-231`); a `BEGIN DEFERRED` mutant added to `mutation_coverage/mutants.rs`'s `REGISTRY` (the architecture brief names this exact defect in §6) is the mechanism that proves the rule *can* fail. |
| AC-008 (wide query chunked, not refused) | Conformance | `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (VT-23, 128-item floor) plus a query wide enough to force the chunk boundary the architecture brief's §5 names (`ceil(arms/N)` > 1). No new public API, so no unit test of `index_arms()` exists to write — the pushdown decomposition is adapter-private and its behavior is what the conformance rule checks. |
| AC-009 (limits are facts, VT-21–24 run) | Conformance | `append_reports_exceeded_store_limits` (`contract.rs:239-247`) running rather than skipping, against the fixture's stated real ceilings; VT-21 – VT-24 rules in `for_each_event_store_rule!`. CF-40's clause-home escalation (§9) is a *recorded open question*, not a test — the KB atom's existence is the proof, checked by review. |
| AC-010 (durability clauses get verdicts) | Conformance + spec-trace | CF-17's rule shape and ES-35's frozen/provisional-with-falsifier verdict are read against `cargo xtask spec-trace`'s citation count (static tier); the *behaviour* backing ES-35 (a real reopen surviving) is the same conformance rule as AC-004. CF-14's confirm/withdraw verdict is a reviewed ADR-0022 paragraph, not a runnable test — named here so it is not silently skipped. |
| AC-011 (projection store passes the suite) | Conformance | `SqliteProjectionStore` against `projection-store-freeze`'s (HS-P0010) projection suite, mounted at `crates/happenstance-sqlite/tests/projection.rs` (architecture brief §1 table). This project does not author the suite; it only runs it. |
| AC-012 (benchmarks exist, are not conformance) | Benchmark tier + static | `event_store_benchmarks!` runs manually behind `bench`; the *static* proof that it cost conformance nothing is `for_each_event_store_rule!`'s rule count unchanged — a diff check a reviewer runs, not a new test. |
| AC-013 (ADR carries a number) | Review, not a test tier | ADR-0022 exists under `.kb/decisions/` and `references/adr/`, cites a measured benchmark figure. Proven by reading the atom, per `CLAUDE.md`'s two-places rule. Named here so "testing" does not silently exclude the project's largest single piece of required evidence. |
| AC-014 (instrument markers gone, gate green) | Static + integration | No `todo!()` (`rg` over `crates/happenstance-sqlite/src`), `#![allow(clippy::todo)]` deleted from `lib.rs`; `cargo xtask ci --fast` green. This is AC-T02's command directly. |
| AC-015 (publishable, publishing not decided) | Static + manual packaging check | `publish = false` unchanged (architecture brief §8); `cargo package -p happenstance-sqlite --list --allow-dirty` run and its output (README, both licence files present) recorded as evidence — **not** a gate step, because `xtask/src/package.rs`'s `PUBLISHABLE` (`:86`) and `reconcile()` (`:172`) do not check an unpublished crate. |
| AC-016 (spec and code agree) | Static | `cargo xtask spec-trace` (part of `reachability_static`, `.redkiln/config.yaml:48`); the citation-count-not-fallen check is exactly what that command performs. |

#### 3. Fixtures and seams — what may be doubled, and what may not

The architecture brief's §1 already names the one fixture this project builds
(`SqliteFixture`, in `crates/happenstance-sqlite/tests/`, not in the testkit).
This section adds the testing-specific corollary: **a fixture is not a mock.**
`Fixture` (`crates/happenstance-testkit/src/contract.rs`) exists precisely so
conformance tests run against the real store through a real connection; the
only components legitimately replaced for a test are:

- **The benchmark emitter** (CF-23) — the sole sanctioned seam, a parameter
  supplied by the caller so measurement dependencies stay out of the testkit's
  `[dependencies]`. It is not used by any conformance or mutation-coverage
  test.
- **Mutant stores** in `mutation_coverage/mutants.rs` — these are not test
  doubles of `happenstance-sqlite`, they are alternate *real* `EventStore`
  implementations (`Rc`/`RefCell`-backed, per `harness.rs:19-35`) built to
  fail a named rule. `SqliteEventStore` itself is never mutated in place; a
  SQL-shaped defect (e.g. the `BEGIN DEFERRED` probe-then-insert from AC-007,
  or a two-table tag join with the wrong separator, per
  `mutants.rs`'s own module doc) is written as its own small store or,
  per architecture brief §10's recommended reading, discharged by the
  conformance run itself rather than added to this registry redundantly.
- **The fixture's temporary file.** Per architecture brief §11, a process-
  local ordinal plus `Drop` cleanup (no new dependency) — a real file, never
  an in-memory stand-in, because `open_in_memory` is explicitly the wrong
  constructor for this fixture (architecture brief §1's third bullet).

What must **not** be doubled: `rusqlite::Connection`, the SQLite file itself,
or the `tokio` runtime under the concurrency family. Doubling any of these
turns a conformance run back into an instrument — a `todo!()`-shaped failure
mode wearing a mock, which is the exact thing this project exists to retire.

#### 4. The runtime seam and CF-33's no-watchdog rule, restated for testing

Architecture brief §3 and `project.md`'s risks name the largest testing risk
directly: whichever ADR-0022 option wins, a test that only exercises the
*sequential* rules will not exercise it, because `NoRuntime` and a genuine
`SQLITE_BUSY` deadlock are both concurrency-only failure modes. Two testing
consequences follow:

- **`event_store_concurrency_conformance!` is not optional coverage for this
  project — it is the coverage that finds §3's seam.** A green sequential
  suite with a red or hanging concurrency suite is not partial credit; per
  `project.md`'s first risk, discovering this in a red run rather than in the
  architecture brief is exactly the failure mode being planned against, and
  the same is true of discovering it in a *green* run that quietly skipped
  the family. No fixture in this project may decline `SECOND_HANDLE`.
- **No test in this project may add a timeout, watchdog, or retry loop
  around a conformance rule.** CF-33 (`concurrency.rs:24-43`) forbids a rule
  reading a clock; a hang is a finding about the busy-timeout value (§6),
  not something a test should paper over. If a rule hangs locally, that is
  evidence for ADR-0022's busy-timeout paragraph, not a reason to add
  `#[timeout]`.

#### 5. `recorded_time_survives_a_reopen`'s missing negative control

`RUNBOOK.md:3205-3207` and architecture brief §9 both flag this by name: the
rule has had nothing in the workspace able to fail it since phase 4. This
project is what changes that, and AC-T06 makes it a checkable obligation
rather than a hope:

- **Before AC-004 is marked done in the story's `_ledger.md`**, confirm this
  rule can fail against this adapter — either by a deliberate temporary
  mutation during development (re-stamping `recorded_at` on reopen instead of
  reading it back, per architecture brief §7) run once and reverted, or by a
  registry row in `mutation_coverage/mutants.rs` that encodes the same defect
  permanently. The architecture brief's §7 precondition (persisted `StoreId`,
  `recorded_at` read back not re-stamped) is what makes the second option
  possible; prefer it, because a permanent registry row is evidence a reviewer
  can re-run forever, while a reverted local mutation is not.
- The ledger entry for AC-004 cites which of the two was used and where.

#### 6. Merge-gate commands, by grain

Restated in one place so an implementer never has to reconstruct them from
`.redkiln/config.yaml` mid-story:

| Grain | Command | Source |
| --- | --- | --- |
| Story (affected) | `cargo xtask affected --base {{base}}` | `affected_gate`, `.redkiln/config.yaml:40` |
| Story (reachability tripwire) | `cargo xtask lints && cargo xtask spec-trace` | `reachability_static`, `:48` |
| This project's integration grain | `cargo xtask ci --fast` | `integration_scoped`, `:55`; also `project.md` DoD 5 |
| Initiative terminal grain (not this project's) | `cargo xtask ci` | `e2e`, `:60`; owned by `closeout-and-durable-audience` |

`cargo xtask ci --fast` is the command this project must keep green
end-to-end; it is what discharges AC-014 directly and is a precondition
(never a substitute, `RUNBOOK.md:38-42`) for every other AC's cited evidence.

#### 7. What this brief deliberately leaves to the implementer

- The exact split of conformance macros across one `tests/conformance.rs`
  target versus several — architecture brief §1 allows either, and this
  brief does not add a preference beyond "reachable from `tests/`, not from
  a `mod`."
  the mutation-registry row's exact `Defect` step overridden for AC-007's
  `BEGIN DEFERRED` mutant, and whether it is written longhand (per the eight
  named exceptions in `mutants.rs`'s module doc) or via the `Defect` trait's
  default-step composition.
- Whether the reopen story and the concurrency story in the story map are one
  slice or two — a scheduling question, not a coverage question; both ACs are
  independently tabulated above regardless.
