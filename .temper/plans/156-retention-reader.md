# #156: ES-39, the reader experiment (`experiments/retention-reader/`)

Planned against `origin/phase-14` at `5e08323c`, which has #467 (`WithholdingStore`, CF-27) merged.
Read-only planning. Nothing was built or run.

## Goal

Run the two readers ES-39 names against CF-27's instrument, a projection runner and an ingest
path. Record what each one produces over a holed log compared with the full log, and whether
anything at the port could have told it. Then give a verdict, judged against criteria fixed
before any run, on one question: **does either reader need a port-level report?** ADR-0028
decision 3 adds a constraint: any such report must default to `Unknown`, so a reader can never
rely on it.

The verdict picks one of two outcomes, both pre-authorised by ADR-0028 decision 9:
- **Refusal:** ES-39 freezes as "the port does not report completeness, and a caller MUST NOT
  infer it from any port value".
- **Method:** a provided `EventStore` method defaulting to `Unknown`, plus the rule
  `a_store_reports_the_history_it_does_not_hold`.

**Prior** (the 0.7.0 planner, and this plan's prediction): neither reader needs one. The
experiment is built so that this prior can lose. The criteria below are written so that a
reader which does need a report would show it.

## Constraints and non-goals

- **The experiment lives outside the workspace.** Its manifest has an empty `[workspace]`
  table and its own `Cargo.lock`, as in `experiments/apply-shape/Cargo.toml:1-14` and
  `experiments/polling-cost/Cargo.toml:1-16`. The root `members` list is unchanged, and so is
  the root `Cargo.lock`. It never runs under `cargo xtask ci`.
  - `xtask/src/affected.rs:386` already treats `experiments/` as inert.
  - The spec-trace filename walk skips `experiments/` (`spec_trace.rs:3326`).
- **No edit to any published crate** in the experiment PR. That includes core, the typed layer,
  sync, the testkit and sqlite.
- **No new third-party dependency.** Every dependency is a workspace path crate, plus
  `futures-core`, `serde` and `serde_json`, which are already in the graph. The experiment's
  own lockfile resolves them.
  - `rusqlite` comes from `happenstance_sqlite::rusqlite` (`crates/happenstance-sqlite/src/lib.rs:148`).
    That keeps the experiment off a second `libsqlite3-sys`, whose `links = "sqlite3"` key would
    refuse to resolve.
  - There is **no `tokio`** and **no `tempfile`**. Async code is driven by
    `happenstance_testkit::block_on`, and temporary files go in a hand-rolled directory guard.
- **No literal positions** in any assertion. Every comparison is against positions the store
  returned (CLAUDE.md, "the rule that matters").
- **Never `#[async_trait]`.** Port impls are written by hand. Each module imports only one
  flavour name (CLAUDE.md constraints 1 and 4).
- **Experiment code still follows the Iron rules.** No `unwrap` outside tests, no `#[allow]`,
  and no lossy `as`.
  - The crate takes `[lints]` from no workspace, because it has none. It sets
    `unsafe_code = "forbid"` and copies the root `[workspace.lints]` block **verbatim** into
    `[lints]`, following `experiments/apply-shape`'s practice. If apply-shape does not copy them,
    copy the root block anyway and say so in a comment.
- **No spec marker moves in the experiment PR.** Freezing ES-39 is #163's job, or D3's (see
  below). The method path, if the verdict calls for it, is a **separate, owner-gated PR**.
- **No runbook ticks** (`runbook/afk.md`). **No ADR** written as a side effect.
- **Out of scope:** ES-40's rule (#154), ES-38's rule (#151), the `Fixture` removal capability
  (#147) and SY-32's case (#158). This experiment feeds #158 with findings but does not build its
  case.

## Decision criteria (fixed before any run; copied verbatim into the README's first section)

The experiment examines four readers, R1 to R3 below, with the ingest path split into two:
- **R1**, the typed projection runner;
- **R2-src**, the ingest path pulling *from* a holed store;
- **R2-dst**, the ingest path writing *into* a holed store;
- **R3**, a decision model, recorded only and not part of the verdict.

A reader **needs a port-level report** if and only if it meets all four of C1 to C4 in at
least one scenario.

- **C1, harm.** Over the holed log, the reader does something its own contract calls wrong. A
  smaller output is not harm. ES-38 says `Query::all()` means "what this store holds", so a
  view built from fewer events is correct relative to its store.
  - Harm means a stated guarantee breaks: a clause (PS-, SY-, VT- or ES-), or the reader's own
    rustdoc promise. Examples: an event applied twice; an edge that stops with a false cause;
    a checkpoint that skips events the store holds.
  - The README names the clause or the doc line that each harm breaks.
- **C2, undetectable today.** No value the reader can already reach lets it detect or avoid
  the harm. That covers every `EventStore` method, every `IngestStore` / `WriteGroupStore` /
  `SyncPeer` value, and its own state. The instrument's inherent `withheld()` does not count,
  because it is not port.
- **C3, a report defaulting to `Unknown` would cure it, generically.** Both parts must hold.
  - **(a)** The reader keeps today's behaviour on `Unknown`. ADR-0028 forces this: all four
    published adapters, and `MemoryEventStore`, forget outside the port. They would answer
    `Unknown` for ever, so a reader that refuses on `Unknown` refuses everywhere.
  - **(b)** On a truthful answer, the reader does something materially better, which its caller
    **could not get from the store's concrete type**. The report must cross a boundary where
    the type is erased, such as code generic over `S: EventStore` in a library. A store that
    could answer truthfully has pruned through its own API, and its caller can already ask that
    API. If that holds for every store that could answer truthfully, (b) fails.
- **C4, `EventStore` is the seam.** The missing information belongs on `EventStore`. It does
  not belong on `IngestStore` or `WriteGroupStore` (the adapter's own ingest internals), on
  `SyncPeer` or `PeerLimits` (a remote peer, which is SY-32's seam), on the typed layer's
  checkpoint, or on the pruner's inherent API. A cure that belongs elsewhere is a finding for
  that seam's owner, not ES-39's method.

**Verdict rule:**
- **Method:** some reader meets C1, C2, C3 and C4.
- **Refusal:** no reader meets all four.
- **Escalate:** some reader meets C1, C2 and C4, and fails C3 *only* because `Unknown`-on-default
  is unacceptable. That is ADR-0028's falsifier: the report would have to be required. Stop and
  hand to the owner. Do not write a method.

**Findings are recorded whatever the verdict.** Any C1 harm that fails C3 or C4 goes into the
README with the seam that owns its cure, and becomes a candidate issue (D5).

## Predictions (written before running; the README keeps them beside the results)

| Reader | Scenario | Predicted C1 | Predicted verdict line |
|---|---|---|---|
| R1 | rebuild over holed vs full | no harm: smaller view, correct relative to the store (ES-38) | fails C1 |
| R1 | prune behind the checkpoint, then continue | no harm: view equals the full log's | fails C1 |
| R1 | prune ahead of the checkpoint, then continue | no harm under ES-38: pruned events never delivered | fails C1 |
| R1 | a floor reader over a scattered purge | the floor misdescribes holes above it (why ADR-0028 rejected the floor) | evidence for ES-39's *Rejects* line |
| R1 | source switch, store A then B, one checkpoint | **harm**: silent skip/replay (`runner.rs:257-259` warns; nothing enforces) | fails C2 (`store_id()` exists) and C4 (the typed checkpoint's seam) |
| R2-src | spoke pulls from a holed hub | no harm under ES-38/SY-27: spoke holds the hub's retained set; `SyncStatus` says synced | fails C1. If counted as harm, fails C4 (`PeerLimits`/SY-32) |
| R2-dst | spoke prunes foreign events, restored edge re-pulls from the start | **harm**: pruned events re-admitted at the tail, a projection applies them **twice** | fails C3(b) and C4 (dedupe is inside `IngestStore::ingest`, keyed on origin identity, not local position) |
| R2-dst | spoke prunes its **own** confirmed events, restored edge re-pulls | **harm**: `IngestError::UnheldOwnIdentity` → `FailureClass::Stop` (`ingest/error.rs:184`), a false "restored/rewound" (SY-38). Whole batch rolled back, so the edge is wedged | fails C4 (the check lives in the adapter's ingest; a cure is a tombstone there) |
| R3 | decision over a holed log | admits what the full log refuses (ES-40, vacuous) | recorded, not in the verdict. Refusing on `Unknown` would refuse on every adapter (C3a) |

**Predicted verdict: refusal**, with two harms found that ES-39's method cannot cure. Both
are owner-level findings for #158 and the replication seam. **R2-dst is new information:** the
0.7.0 prior did not mention it.
- `runner/watermark.rs:20-23` says *"a re-read is harmless, because ingest skips what it holds
  (SY-11)"*. That holds only while the store never forgets.
- `Edge::restored` (`runner.rs`, "pulls from the start of the far side's log") is the documented
  restart path.
- So deployment P2, where SQLite acts as a pruned buffer, breaks on its first restart.

## Design

### Placement and layout

```
experiments/retention-reader/
  Cargo.toml          outside the workspace; [workspace] empty table; publish = false; [lints] copied
  Cargo.lock          its own
  README.md           question, criteria (C1-C4, verbatim from this plan), predictions, conditions,
                      results tables, verdict, findings, what it does not license
  run.sh              cargo test --locked -- --test-threads=1 --nocapture, tee to results/, path scrub
  results/
    test-all.txt      the transcript of the run.sh test invocation
    environment.txt   rustc -Vv, commit, OS (redacted as provided-method-spike does)
  src/
    lib.rs            crate docs: what this is, a pointer to the README; `pub mod` list
    domain.rs         Claimed/Noted event enum (DomainEvent + serde), a counting Projection
    shared.rs         `Shared`: an Arc handle on MemoryIngestStore implementing the Send flavours
    holed.rs          `Holed`: WithholdingStore<Shared> + its inner Shared; WriteGroupStore + IngestStore
    prune.rs          `TempDb` guard and `prune_outside_port` (raw DELETE on a SQLite file)
    seams.rs          `Seams`: one value capturing every port answer for a store, for comparison
  tests/
    r1_runner.rs      R1 scenarios
    r2_ingest_source.rs   R2-src scenarios, through Edge + StoreSyncPeer<Holed>
    r2_ingest_destination.rs  R2-dst scenarios, on SQLite with raw DELETE, plus the instrument control
    r3_decision.rs    R3 (recorded)
    floor.rs          the floor reader over a scattered purge
```

The names follow `results/` precedent: avoid `*-output.txt` and `*.log`, which the root
`.gitignore` eats (`.kb/open-questions/experiment-raw-output-eaten-by-the-ignore-rule.md`).

**Why lib + integration tests**, not a binary that prints a report:
- each observation is an **assertion** about what was observed, so re-running the experiment
  on a later tree re-checks the record (the `suite-against-wrong-adapters` shape);
- the README cites test names;
- a binary would need `writeln!` plumbing and a schema for no reader.

An observation test asserts the *observed* hazard, for example
`assert_eq!(applied_twice, 1)`, and its doc comment says that this is a record, not a desired
behaviour.

### Dependencies (`Cargo.toml`)

```toml
[workspace]

[package]
name = "retention-reader"
version = "0.0.0"
edition = "2024"
rust-version = "1.97.1"
publish = false

[dependencies]
happenstance = { path = "../../crates/happenstance", default-features = false, features = ["std", "memory", "json"] }
happenstance-core = { path = "../../crates/happenstance-core", default-features = false, features = ["std", "memory"] }
happenstance-testkit = { path = "../../crates/happenstance-testkit" }          # WithholdingStore, block_on (default = memory)
happenstance-sync = { path = "../../crates/happenstance-sync", features = ["memory"] }  # Edge, StoreSyncPeer, MemorySyncPeer, MemoryIngestStore
happenstance-sqlite = { path = "../../crates/happenstance-sqlite", features = ["sync"] } # IngestStore + WriteGroupStore impls, re-exported rusqlite
futures-core = { version = "0.3", default-features = false }                 # naming Stream in Shared's hand-written impl
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

The testkit is a **normal** dependency, not a dev-dependency, because `src/holed.rs` names
`WithholdingStore`. Check that `happenstance`'s features include `std` before copying them:
apply-shape uses that set. Check that `happenstance-sync`'s `memory` feature and
`happenstance-sqlite`'s `sync` feature are spelled as at `5e08323c`
(`crates/happenstance-sync/Cargo.toml` `[features]`, `crates/happenstance-sqlite/Cargo.toml:171`).

### Types and signatures (ownership decided)

**`shared.rs`: an `Arc` handle on `MemoryIngestStore`.**

Why it exists:
- `MemoryIngestStore` is not `Clone` (`memory_ingest.rs:106`).
- `StoreSyncPeer::new(events: S, ingest: I)` takes both halves by value.
- There is no `impl EventStore for &S` or `for Arc<S>` anywhere. `fixtures.rs:175-190` explains
  why: coherence with `trait_variant`'s blanket impl.

So the experiment does what `MemoryHandle` does: a local newtype, implementing foreign traits,
which the orphan rule allows.

```rust
#[derive(Debug, Clone, Default)]
pub struct Shared(Arc<MemoryIngestStore>);   // clone = a second handle onto one log (Arc: cheap handle, H-08)
impl Shared {
    pub fn new() -> Self;
    pub fn with_store_id(id: StoreId) -> Self;
    pub fn store_id(&self) -> StoreId;        // the inherent one, for scenario setup
}
impl SendEventStore for Shared { type Error = MemoryStoreError; /* 4 methods, delegate via SendEventStore::.. on &*self.0 */ }
impl SendIngestStore for Shared { type Error = MemoryStoreError; /* store_id, ingest, watermark: delegate */ }
impl SendWriteGroupStore for Shared { type Error = MemoryStoreError; /* read_groups: delegate */ }
```

- Implement the **Send flavours**, as `MemoryIngestStore` does (`memory_ingest.rs:333,434`).
  The blanket impls then give the bare flavours that `WithholdingStore<S: EventStore>`, `Edge`
  and `StoreSyncPeer` bind.
- `read` returns the inner stream unchanged: `impl Stream<..> + Send`.
- Only `SendEventStore`, `SendIngestStore` and `SendWriteGroupStore` are imported in this
  module. No bare names (constraint 4).
- If `MemoryIngestStore` does not implement `SendWriteGroupStore` but the bare flavour, mirror
  whichever it does. Read `memory_ingest/groups.rs` first. The `Edge` doctest
  (`runner.rs:159-170`) proves it implements one of the two.

**`holed.rs`: the instrument, plus the two sync traits the testkit cannot supply.**

`WithholdingStore` lives in the testkit and `IngestStore` / `WriteGroupStore` live in sync.
The testkit cannot depend on sync, and a third crate cannot write
`impl IngestStore for WithholdingStore<_>` (orphan rule: E0117, the `compile_fail` on
`IngestStore`, `ingest.rs:118-126`). So the experiment wraps the instrument in a local type.

```rust
#[derive(Debug, Clone)]
pub struct Holed {
    view: WithholdingStore<Shared>,   // the port view: what a pruned store would answer
    inner: Shared,                    // the same Arc: what the store physically holds
}
impl Holed {
    pub fn new(inner: Shared) -> Self;                 // view = WithholdingStore::new(inner.clone())
    pub fn view(&self) -> &WithholdingStore<Shared>;   // EventStore reads and withholding go through here
    pub fn inner(&self) -> &Shared;                    // scenario setup only (appends, ingests)
}
impl WriteGroupStore for Holed { type Error = MemoryStoreError; async fn read_groups(..) }   // bare flavour
impl IngestStore for Holed { type Error = MemoryStoreError; /* delegate all three to inner */ }
```

- **Bare flavour.** `Holed` holds a `WithholdingStore`, which implements only `EventStore`, so
  futures that touch `view` are `Send` only by leakage. `Edge` and `StoreSyncPeer` bind the bare
  flavours (`runner.rs:81`), and the tests drive with `block_on`. Nothing here needs `Send`.
- **`read_groups(after, max_events)` hides withheld events.** It works like this:
  1. Loop: `page = inner.read_groups(cursor, max_events)`.
  2. For each group, keep the events for which
     `EventStore::contains_event_id(&self.view, event.id)` is `true`. Membership through the
     instrument is withheld-aware, and every event in an inner group is physically held, so
     `false` means withheld.
  3. Drop a group whose kept events are empty. Otherwise rebuild it with
     `WriteGroup::new(group.first(), group.guard().cloned(), first_kept)` plus `push` for the
     rest.
  4. If the filtered page is non-empty, or the inner page is empty, or `page.waiting()` is
     `Waiting::Nothing`, return `GroupPage::new(kept, page.waiting())`. Otherwise set
     `cursor = Some(last inner group's first)` and loop, so that a page made only of withheld
     groups does not end a pull early. The loop ends because `first` strictly increases
     (`write_group.rs:91-93`, the chaining guarantee). Check that `GroupPage` exposes
     `groups()` and `waiting()`, or the names it does use.
  - **Fidelity note (README).** A partially withheld group keeps its original `first`. A
    physically pruned store would name the group by its lowest remaining row. The spoke never
    sees `first`; only the hub's resume token does, and pagination stays monotone. Scenarios
    use **one event per append** for withheld events, so no group is cut. One scenario,
    R2-src-3, cuts a two-event group on purpose and records it.
  - The `Query` clone, the `guard().cloned()` and the event clones are real duplication into a
    new owned page (H-08).
- **`IngestStore` is a pure delegation to `inner`.** This is the instrument's stated fidelity
  gap (`withholding.rs`, "What it cannot model"). The test `instrument_control_*` uses it to
  show the gap is material.

**`prune.rs`: physical removal outside the port, on SQLite.**

```rust
/// A directory under std::env::temp_dir() unique to this process and scenario; removed on drop.
pub struct TempDb { dir: PathBuf }
impl TempDb {
    pub fn new(scenario: &str) -> io::Result<Self>;    // temp_dir/retention-reader-{pid}-{scenario}; create_dir_all
    pub fn path(&self) -> PathBuf;                      // dir/store.sqlite
}
impl Drop for TempDb { /* remove_dir_all; on Err write to a locked stderr. Drop never panics (ERR-13) */ }

/// Deletes the rows at `positions` with a raw DELETE, as an operator pruning outside the port would.
/// The store must not be open: the caller drops every handle first and reopens after.
pub fn prune_outside_port(path: &Path, positions: &[SequencePosition]) -> Result<usize, PruneError>;
```

- Body, in one `rusqlite` transaction on
  `happenstance_sqlite::rusqlite::Connection::open(path)`, for each position `p` converted with
  `i64::try_from(p.get())` (H-11):
  - `DELETE FROM event_tag WHERE position = ?1` (`event_tag.position REFERENCES event(position)`,
    `event_store.rs:312-317`, so tags go first);
  - `DELETE FROM event WHERE position = ?1`.
- Then refresh the planner hint:
  `UPDATE tag_cardinality SET events = (SELECT count(*) FROM event_tag WHERE event_tag.tag = tag_cardinality.tag)`.
- Return the number of `event` rows deleted.
- `PruneError` is a small hand-written enum, `Sqlite(rusqlite::Error)` and `Position(TryFromIntError)`,
  with `Display` and `Error`. This is a library-shaped module in an experiment, and
  `thiserror` is not added (no new dependency).
- The schema names are taken from `MIGRATION_1` (`event_store.rs:299-325`). Cite it in the doc
  comment. If the schema has a later migration at `5e08323c` that adds a table referencing
  `event(position)`, delete from it too.
- **Why SQLite and not the instrument:**
  - only a physically pruned store re-admits a re-delivered event, because ingest dedupe lives
    in the adapter's `UNIQUE (origin_store, origin_position)` (`event_store.rs:309`), and
    detects an unheld own identity only when the row *lands* (`ingest_spike.rs:16-24`);
  - `AUTOINCREMENT` (`event_store.rs:301`) means the raw DELETE leaves ES-38 intact, so the
    pruned store is a legal deleted-from store;
  - this is the out-of-port `DELETE` that ADR-0028 names, staged before #147 builds it as a
    `Fixture` capability.

**`seams.rs`: what the port says.**

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct Seams {
    pub store_id: StoreId,
    pub head: Option<SequencePosition>,
    pub all: Vec<(SequencePosition, EventId)>,       // read(Query::all(), forward)
    pub first_forward: Option<SequencePosition>,     // read(all, limit 1)
    pub member: Vec<(EventId, bool)>,                // contains_event_id for every id in `probe`
}
pub async fn seams<S: EventStore>(store: &S, probe: &[EventId]) -> Result<Seams, S::Error>;
```

This is used for the "could anything at the port have told it?" column. Each test compares
the `Seams` of the holed store with the `Seams` of a **young store** that holds the same
retained events. The young store is built by appending the retained events in order into a
fresh `MemoryEventStore`. Positions differ, so compare **shape**, not values:
- same count;
- same relative order of event types and tags;
- `contains_event_id(withheld id) == false` in both, where the young store never held it.

The recorded answer per seam is one of: "indistinguishable", or "differs, and here is how".
The only predicted difference is the position values themselves (holes vs dense). ES-38 forbids
reading meaning into gaps (gaps are always permitted), so a reader may not use them.

**`domain.rs`.**
- `enum ClaimEvent { Registered { claim }, Noted { claim, n } }`, implementing `DomainEvent`
  over `Json`. Two `EVENT_TYPES`, tagged `claim:<id>`. Copy the shape of
  `examples/transfers-on-sqlite` or polling-cost's domain.
- `struct Counter { id: ProjectionId, scope: Tags, applied: BTreeMap<EventId, u32> }`
  implementing `Projection<Store = MemoryProjectionStore>`.
- `apply` increments `applied[delivered.id()]` and writes a count row through the batch's
  public write API (`projection_memory.rs`; check its method names at `5e08323c`). Counting per
  `EventId` is what makes **double application observable**. A projection that only counts
  rows would hide it.

### Scenarios and what each records

All of them use `block_on`. "Full" means the same history with nothing withheld or pruned.

**R1, the typed runner** (`tests/r1_runner.rs`).
- **Setup.** `Holed` over `Shared`. Append 12 events, one per append, across claims `a`, `b`
  and `c`, interleaved. Run `run_projection(holed.view(), &models, &mut counter, &Json, chunk)`
  with `Chunk` 4.
- **R1-1 `rebuild_over_a_scattered_purge_is_smaller_and_says_nothing`.** Withhold claim `b`'s
  events plus one claim-`a` event in the middle, so survivors sit between the holes. Run fresh.
  Record:
  - `Progressed.applied` against the full count;
  - the checkpoint against the highest retained matching position;
  - that `counter.applied`'s keys equal the retained ids exactly;
  - that `Seams` matches a young store's shape.
  - C1: none. The runner did what PS-* promises relative to its store.
- **R1-2 `pruning_behind_the_checkpoint_changes_nothing`.** Run over the full log, withhold the
  first 6, append 2 more, run again. The view equals the full view plus 2, and nothing is
  re-applied.
- **R1-3 `pruning_ahead_of_the_checkpoint_is_never_delivered`.** Run through the first 6,
  withhold positions 8 and 10 (indexes into `assigned`), run again. The pruned ones are absent,
  and the checkpoint passes them.
- **R1-4 `a_source_switch_resumes_against_the_wrong_store`.**
  1. Run `counter` over store A, a `MemoryEventStore` with 8 events.
  2. Against the **same** `models`, run it over store B: a different `StoreId`, holding 5 of A's
     events ingested **plus** 4 B-local ones, so B's positions differ.
  3. Record the events skipped (B events at or below A's checkpoint) or re-applied.
  4. Record that `events.store_id()` would have told it (C2 fails), and that the cure is the
     typed checkpoint recording the source `StoreId` (C4: not `EventStore`, which already has
     `store_id()`). This is the deployment note's candidate issue 1.

  Build B as a `Shared` with an ingest of `ReplicatedEvent`s built from A's read
  (`ReplicatedEvent::new(e.id, e.recorded_at, e.event)`), so it is a realistic "rebuild from
  the hub" source.

**Floor** (`tests/floor.rs`).
- **`a_floor_misdescribes_a_scattered_purge`.** Withhold the S2 pattern: low survivor, holes
  above it. Compute what `earliest_position()` would answer, which is the lowest retained
  position.
- Show that "complete above the floor" is false: there are withheld positions above it.
- The instrument's `withheld()` is the oracle.
- This one-paragraph record backs ES-39's *Rejects* line and ADR-0028 decision 4 with a run,
  not prose.

**R2-src, ingest from a holed hub** (`tests/r2_ingest_source.rs`).
- **Setup.** `hub = Holed::new(Shared::new())`. Append 10 events, one per append, to
  `hub.inner()`. `peer = StoreSyncPeer::new(hub.clone(), hub.clone())`.
  `spoke = MemoryIngestStore::new()`. `edge = Edge::new()`.
- **R2-src-1 `a_spoke_pulling_a_scattered_hub_holds_the_retained_set_and_reports_synced`.**
  Withhold 3 scattered positions through `hub.view().withhold(..)`. Loop `edge.step` until
  `status().is_synced()`, at most 8 steps; if not synced by then, record that and fail.
  Record:
  - the spoke's ids equal the retained ids;
  - `status()` is synced, with pending `Exactly(0)` or as reported;
  - `spoke.watermark()` for the hub origin equals the max retained origin position, with no
    trace of the holes below it;
  - `Seams(spoke)` is shape-equal to a young store.
  - C1: under ES-38 and SY-27 the spoke holds what the hub holds. If the README classes that as
    harm, C4 points at `PeerLimits` (SY-32, `peer.rs:296-301`), not `EventStore`.
- **R2-src-2 `withholding_after_a_pull_leaves_the_spoke_with_more_than_the_hub`.** Pull
  everything, then withhold on the hub. The spoke keeps them; nothing at the port reconciles.
  This is expected under SY-1/SY-34 and recorded.
- **R2-src-3 `a_purge_that_cuts_a_write_group_ships_the_remainder`.** One two-event append on
  the hub. Withhold one of the two, then pull. The spoke receives a one-event group. Record it
  against SY-30, "group whole", which binds the groups as the store holds them; that is the
  README's reading. Name it as a question for #158 if the reviewer disagrees.

**R2-dst, ingest into a pruned spoke on SQLite** (`tests/r2_ingest_destination.rs`).
- **Setup.**
  - `db = TempDb::new(..)`, `spoke = SqliteEventStore::open(db.path())`.
  - `hub = MemorySyncPeer::new(StoreId::from_bytes([2; 16]))`, seeded with 4 single-event groups
    minted under a third origin `X` with `seed(EventGroup::new(None, vec![replicated]))`. The ids
    are `EventId::new(x, SequencePosition)`; take positions from `SequencePosition::FIRST` and
    `next()`, never literals. Check `seed`'s doc for what it requires of ids.
  - A `Counter` projection over the spoke into a `MemoryProjectionStore`.
- **R2-dst-1 `a_restored_edge_resurrects_pruned_foreign_events`.**
  1. `edge.step` until synced, so the spoke holds X1..X4. Run the projection.
  2. Keep `edge.watermark()` (it may be `None`, since nothing was pushed). Then drop the spoke.
  3. `prune_outside_port` X2 and X4, the latter being X's highest.
  4. Reopen. Assert ES-38 still holds.
  5. `Edge::new()`, or `Edge::restored(wm)` when there is a watermark. Step once.
  6. Run the projection again.
  7. Record:
     - `Ingested.appended`: the number of re-admitted events;
     - their new positions are above the pre-prune head;
     - `spoke.watermark()` regressed after the prune and recovered after the re-pull;
     - **the counter shows X2 and X4 applied twice** (`applied[id] == 2`).
  8. C1: harm. Delivered identity is applied twice. Name the PS-/VT- line the README cites:
     the runner doc's "nothing applied twice" (`runner.rs`) and VT-8's uniqueness, which held
     only among *held* rows.
  9. C3(b) fails, because a report about local positions cannot identify an origin-identity
     re-delivery. C4 fails, because the cure is a pruned-identity tombstone inside
     `IngestStore::ingest`, or a persisted pull token. Neither is `EventStore`.
- **R2-dst-2 `a_restored_edge_stops_on_a_pruned_own_event`.**
  1. Append L1..L3 locally on the spoke. Step until synced, so the push is confirmed. The pull
     returns L1..L3 back, and they are skipped as held. Keep `edge.watermark()`.
  2. Drop the spoke. Prune L1 (confirmed, so pruning it is exactly the deployment note's P2
     rule). Reopen.
  3. `Edge::restored(wm)`. Step.
  4. Record:
     - the step's error carries `IngestError::UnheldOwnIdentity { id }` with `id == L1's id`;
     - `status().state` is `EdgeState::Stopped(..)`;
     - nothing was written: the spoke's head is unchanged, and X events in the same pull did
       not land either;
     - a second `step` returns `StepError::Stopped` with no I/O.
  5. C1: harm. SY-38's remedy, "re-mint", is wrong advice here: the store was not restored, it
     was pruned. C3(a): on `Unknown` the edge must stop as today. C3(b) and C4: the own-identity
     check runs inside the adapter's ingest transaction (`ingest_spike.rs:16-24`). A cure is a
     record of pruned own positions consulted there, an adapter-internal and pruner-API concern.
     A generic `EventStore` report consulted by the edge *after* the error would be a
     probe-after-write that VT-8 already refuses to build on (`ingest.rs:97-101`).
- **Instrument controls**
  (`instrument_control_resurrection_is_masked`, `instrument_control_false_rewind_is_masked`).
  Stage the same two scenarios with the spoke as `Holed`, withholding instead of deleting.
  Record that ingest **skips** in both, so no harm is visible. This shows that #144's stated
  fidelity gap hides both harms, and that #151, #154 and #158 must use the #147 capability, not
  the decorator, for ingest-side claims. To run the edge over `Holed`, both traits are already
  implemented.

**R3, a decision over a holed log** (`tests/r3_decision.rs`). Recorded only, and not in the
verdict, because ES-40 and #154 own it.
- Over `Holed`, withhold claim `a`'s `Registered` event. Then `commit` a `DecisionModel` that
  refuses to note an unregistered claim.
- Record:
  - the full store refuses;
  - the holed store admits;
  - no seam differs.
- C3(a) is spelled out: a decision layer that refused on `Unknown` would refuse on every
  published adapter. Use the typed layer's `commit` or `commit_with`: read
  `crates/happenstance/src/command.rs` for its signature. If wiring a `DecisionModel` costs more
  than about 60 lines, fall back to a raw `append` with an `AppendCondition`. The record is the
  same, and the README says which was used.

### Errors

The experiment adds one error type, `PruneError` (above). Every test returns
`Result<(), Box<dyn Error>>` or uses `expect("why")`, which is allowed in tests. Library
functions in `src/` return typed errors: `S::Error`, `MemoryStoreError`, `PruneError`. There is
no `Box<dyn Error>` in a `pub` signature (H-09), even here.

### Concurrency and resources

- Single-threaded throughout, under `block_on`.
- No spawned task or thread.
- `TempDb` owns its directory and removes it on drop. The SQLite store is dropped before every
  out-of-port prune, so there is no second writer and no busy-timeout race.
- Edge loops are bounded at 8 steps (H-13).

### What lands in the repo

**PR A: this issue, two-way. Always.**
1. `experiments/retention-reader/`, all of the above. First commit: `README.md` with *only* the
   question, C1 to C4, the verdict rule and the predictions, before any test exists, so the
   criteria are visibly pre-registered in history. The provided-method-spike's "stated before
   anything ran" is the precedent.
2. Second commit: the code, `results/`, and the README's results, verdict and findings
   sections.
3. **Spec, line-neutral reconciliation.** In ES-39's `[DEFERRED …]` bracket, add one clause:
   *"The reader experiment ran at #156 (`experiments/retention-reader/README.md`): the verdict
   is <refusal|method>."* The marker stays `[DEFERRED]` unless D3 says otherwise. Run
   `cargo xtask spec-trace --write` and then `cargo xtask spec-trace`. Do not touch ES-38, ES-40
   or CF-27.
4. The PR body carries the verdict table and `Closes #156` only if D3 is A (freeze here) or C;
   with D3 = B it says `Part of #156; #163 freezes`. Nothing in `crates/`.

**If the verdict is refusal:** nothing in core, ever, from this issue. ES-39 freezes at #163,
or in PR A under D3, as "the port does not report completeness, and a caller MUST NOT infer it
from any port value", with ES-38's and ES-40's rules as its checkable consequences (ADR-0028
decision 9).

**If the verdict is method: PR B, separate, owner-gated, one-way.** Sketch only. It needs its
own plan.
- A provided `EventStore` method in ES-4's spelling,
  `fn <name>(&self) -> impl Future<Output = Result<<Report>, Self::Error>> { async { Ok(<Report>::Unknown) } }`.
  The body captures neither `&self` nor `Self::` (ES-3). `<Report>` is a `#[non_exhaustive]`
  enum.
- `WithholdingStore` overrides it truthfully.
- The rule `a_store_reports_the_history_it_does_not_hold` goes into the event-store family. A
  store answers `Unknown`, or answers accurately against the fixture's removal capability (#147).
- The PR also needs:
  - a mutant (`every_rule_has_a_mutant`);
  - a registry entry;
  - a `pass_list.txt` update (#144's D4);
  - a CHANGELOG minor entry;
  - `cargo-semver-checks` silent against `0.6.0`.
- **Name, report type and variants are owner calls.** ADR-0028 says "phase 14's".

## Test list (drive in this order)

These are experiment observations, not production TDD. Red is the compile error against the
not-yet-written support module, then the first observed assertion. Record each observation's
first red in Progress.

- [ ] 0. README commit: question, C1 to C4, the verdict rule, predictions. No code.
- [ ] 1. `shared_is_a_second_handle_onto_one_log`: append through one clone, read through
  another. *(First red: `Shared` does not exist.)*
- [ ] 2. `holed_read_groups_hides_withheld_events_and_pages_past_all_withheld_pages`. Unit
  test in `holed.rs`. 6 single-event groups; withhold groups 2 to 4 with `max_events` 1. The
  page chain yields groups 1, 5 and 6, each once.
- [ ] 3. `holed_read_groups_cuts_a_partially_withheld_group_keeping_first`.
- [ ] 4. `prune_outside_port_deletes_rows_and_the_store_reopens_conformant`. Reopen, and
  `read` lacks the rows. `contains_event_id` is false for them. The next append's position is
  above every pruned one (ES-38).
- [ ] 5. `seams_of_a_holed_store_match_a_young_store_in_shape`.
- [ ] 6. R1-1 to R1-4, in order.
- [ ] 7. `a_floor_misdescribes_a_scattered_purge`.
- [ ] 8. R2-src-1 to R2-src-3.
- [ ] 9. R2-dst-1, R2-dst-2, then the two instrument controls.
- [ ] 10. R3.
- [ ] 11. `run.sh`: `cargo test --locked -- --test-threads=1 --nocapture` into `results/`,
  paths scrubbed. `rustc -Vv` and `git rev-parse HEAD` into `environment.txt`.
- [ ] 12. README: a results table per reader (scenario, observed, C1, C2, C3a, C3b, C4, the
  seam that owns the cure), the verdict by the rule, findings, and "what this does not
  license". The last covers:
  - it says nothing about adapters other than SQLite for R2-dst, although the mechanism is
    shared: Postgres, Neon and Cloudflare all dedupe on the origin pair;
  - the instrument's ingest gap.
- [ ] 13. Spec reconciliation (ES-39 bracket), then `cargo xtask spec-trace --write` and
  `cargo xtask spec-trace`.
- [ ] 14. Gate: `cargo xtask ci --fast` for the spec edit. In the experiment, run
  `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check` and
  `cargo test --locked`. Its own target dir, so watch disk (Risks).

## Decisions (H-16)

| # | Decision | Options | Recommendation | Door | Decided by |
|---|---|---|---|---|---|
| D1 | The decision criteria | A) C1 to C4 and the verdict rule as written; B) a looser "would a report have helped at all" | **A.** B is satisfied by every scenario, since information always helps, and it makes the method inevitable. C3(b) and C4 are what make "defaults to `Unknown`" bite, as ADR-0028 requires. | Two-way until PR A merges; after that the record stands | owner (it shapes a published-surface outcome); delegation may confirm |
| D2 | Stage R2-dst on SQLite with a raw DELETE | A) SQLite and `happenstance_sqlite::rusqlite`, faithful; B) instrument only, recording the gap; C) wait for #147's capability | **A.** B cannot show either harm (the controls prove it). C serialises #156 behind #147 for no gain, since #147's SQLite arm will be this same DELETE. No new dependency: `rusqlite` is re-exported. | Two-way | architect |
| D3 | Who flips ES-39's marker | A) PR A freezes ES-39 as the refusal and closes #156; B) PR A records the verdict in the bracket, and #163 freezes ES-39 with ES-40, SY-32 and CF-27 in one freeze record (the ADR-0107 precedent); C) PR A freezes, #163 cites | **B.** The runbook lists ES-39 under #163, and plan-144 already deferred every marker move there. One freeze record for four clauses is easier to audit. Cost: #156 stays open, or closes with "Part of", until #163. | Two-way (marker text); the freeze itself is one-way-ish (`[FROZEN]` needs an ADR to change) | owner |
| D4 | Verdict, if it comes out **method** or **escalate** | Method: PR B, with its own plan, name and report type. Escalate: stop and hand ADR-0028's falsifier to the owner | Do not build PR B unattended. Both paths change a published trait or reopen an accepted ADR. | One-way | owner |
| D5 | The R2-dst harms and R1-4, if confirmed | A) file issues now: "replica pruning breaks re-pull: resurrection and false rewind (SY-31, SY-38, VT-8)", linked to #158, and "pin a projection checkpoint to its source `StoreId`", linked to the deployment note's candidate 1; B) record in the README only, leaving issue triage to the owner | **A for the first**, labelled `status:needs-triage`, because it breaks deployment P2 on its first restart and touches the frozen SY-38 and VT-8 wording. **A for the second as well.** Neither issue proposes a fix: the fixes include adapter tombstones, a persisted pull token, or the typed checkpoint shape, which are owner calls and possibly 1.0-sensitive (a checkpoint behaviour change). | Two-way (filing); the fixes are owner-level | owner for the fixes; filing may be delegated |
| D6 | R3 in this experiment | A) include, recorded only; B) leave it to #154 | **A.** About 80 lines, and it makes C3(a) concrete for the reader most at risk. It is excluded from the verdict, so #154's rule is not pre-empted. | Two-way | architect |
| D7 | Issue #156 carries `status:blocked` | It is unblocked now that #467 merged at `5e08323c` | Remove the label when work starts (triage) | Two-way | owner (triage), may be delegated |

## Risks

- **Disk.** The experiment's own target dir builds `rusqlite` bundled, `happenstance-sqlite`
  and `sync`: probably 1 to 2 GB. Mitigations:
  - `CARGO_TARGET_DIR` pointed at the main tree's `target/`, the provided-method-spike's
    practice;
  - `CARGO_INCREMENTAL=0`;
  - `cargo clean -p retention-reader` afterwards.
- **A prediction is wrong.** That is the point. Report what was observed and apply the verdict
  rule mechanically. In particular:
  - If `MemorySyncPeer` or `Edge` filters the requester's own events on pull, R2-dst-2 does not
    reproduce. Record that, and stage it through a `StoreSyncPeer` hub instead.
  - If SQLite's ingest does not raise `UnheldOwnIdentity` for a landed own row after a DELETE,
    record the actual behaviour. A silent re-admission of one's own event is the same harm
    class as R2-dst-1.
- **Over-reading the instrument.** The controls make the ingest gap explicit. The README must
  say that R2-src runs through the decorator, which is faithful for a *source*, since a source
  only reads, and that R2-dst does not.
- **`WriteGroupStore` for `Holed` misbehaving under pagination.** Covered by tests 2 and 3
  before any edge scenario.
- **Criteria judged too strict.** C3(b) could be read as defining the method out of existence.
  Answer, which goes in the README: C3(b) is ADR-0028 decision 3 restated. A default of
  `Unknown` is useful only where generic code acts on a non-default answer that it could not
  get otherwise. D1 puts this in front of the owner.
- **A stale schema in `prune_outside_port`.** Read `MIGRATION_*` at the implementation commit,
  not this plan's citation.

## Size estimate (lower bound)

| File | ≈ lines |
|---|---|
| `Cargo.toml` (with the lint block) | 70 |
| `README.md` | 260 |
| `run.sh` | 35 |
| `src/lib.rs` | 40 |
| `src/domain.rs` | 120 |
| `src/shared.rs` | 110 |
| `src/holed.rs` (with unit tests 2 and 3) | 190 |
| `src/prune.rs` (with test 4) | 120 |
| `src/seams.rs` (with test 5) | 90 |
| `tests/r1_runner.rs` | 230 |
| `tests/floor.rs` | 50 |
| `tests/r2_ingest_source.rs` | 170 |
| `tests/r2_ingest_destination.rs` | 260 |
| `tests/r3_decision.rs` | 80 |
| `results/*.txt` | generated |
| `spec/SPECIFICATION.md` (ES-39 bracket, line-neutral) | +1 / −1 |

That is **about 15 files and 1,800 lines** at the lower bound, about 670 of them in `src/`.
`Cargo.lock` is generated. PR B, if it happens at all, is separate: about 6 files and 300
lines, plus a mutant.

## Progress

- [ ] Plan written (this file).
