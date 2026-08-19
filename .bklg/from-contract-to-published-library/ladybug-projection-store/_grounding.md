# Grounding — The unlike batch shape, and the freeze verdict (HS-P0015)

Companion file, not an item file. Cites paths for the briefs to build on; edits
nothing outside this project's planning artefacts.

## Accepted decision atoms that constrain this project

- **`.kb/decisions/0001-async-port-flavours.md`** (ADR-0001, accepted, phase 0) —
  the two-flavour `#[trait_variant::make]` derivation and the ban on
  `#[async_trait]`. `LadybugProjectionStore` implements `SendProjectionStore`
  (`crates/happenstance-ladybug/src/projection_store.rs:258`) on the evidence
  that `lbug`'s `Database`/`Connection` are `Send + Sync`
  (`…/projection_store.rs:252-257`) — a design decision the brief should cite
  rather than re-derive.
- **`.kb/decisions/0007-projection-runner-decodes.md`** (ADR-0007, accepted,
  phase 0) — where the runner splits; out of scope here (owned by
  `typed-layer-and-alpha-release`) but its indicative `Projection::apply`
  signature is the shape any future write-vocabulary decision must stay
  compatible with.
- **`.kb/decisions/0008-one-derivation-for-both-ports.md`** (ADR-0008,
  accepted, phase 1) — a provided body cannot hold the `Batch` GAT across a
  suspension point under any remedy tried. Binds any batch-vocabulary shape
  ADR-0025 or a future ADR-0017 might pick.
- **`.kb/decisions/0010-the-suite-must-prove-itself.md`** (ADR-0010, accepted,
  phase 3) — the mutant-registry discipline `projection-store-freeze`'s
  `CheckpointOnlyStore` obeys. This project consumes the suite; it does not
  extend the mutant registry.
- **`.kb/decisions/0029-msrv-raised-to-1-97-1.md`** (ADR-0029, accepted, phase
  2, amends ADR-0004) — the MSRV every crate in the workspace, including
  `happenstance-ladybug`, targets.

**ADR-0025 does not exist yet.** `.kb/decisions/` currently holds only
0001–0016 and 0029 (`ls .kb/decisions/`); ADR-0017–0024 are also unwritten,
each owned by a still-planning sibling project. RUNBOOK.md:304 reserves
**0025** for exactly this project — "Ladybug: checkpoint placement, how a
projection expresses graph mutations, and the blocking API" — matching
project.md's AC-002 verbatim. The number is free; nothing to disambiguate.

## The dependency this project cannot see past

`projection-store-freeze` (HS-P0010) is still `stage: storymap`, not merged —
its `project.md` is read below for what it commits to shipping, but nothing in
`crates/happenstance-core/src/projection.rs` reflects it yet. Two consequences
for whoever writes this project's briefs:

1. **The port's shape will change out from under the skeleton.**
   `crates/happenstance-core/src/projection.rs:97-99` today declares
   `type Batch<'a> where Self: 'a` — a GAT with a lifetime. `HS-P0010`'s own
   `project.md` (In scope, item 2) commits to dropping that lifetime to
   `type Batch;` and adding a write seam
   (`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`). Every
   `todo!()` body this project fills in
   (`crates/happenstance-ladybug/src/projection_store.rs:270-292`) is written
   against the **current**, pre-freeze signature; AC-003 cannot be satisfied
   independent of that upstream change landing first. project.md's own
   Dependencies section already states this as a hard `blocked_by`
   (`RUNBOOK.md:162`, `6 ─▶ 11`); the grounding note surfaces it again because
   an implementer skimming only the skeleton files could miss that the
   signature they're reading is stale by design.
2. **PS-34's contingency may already be moot by the time this project
   starts.** `spec/SPECIFICATION.md:5546-5559` marks PS-34
   `[PROVISIONAL — contingent on PS-5; dead the moment `type Batch;` lands]`.
   `HS-P0010` plans to land exactly that. If it does, AC-006's "re-test PS-34's
   E0195 trap by a third implementer" is testing a clause the specification
   itself calls dead — the honest re-test outcome may be "the trap no longer
   exists because the lifetime is gone," not a reproduction of the original
   `error[E0195]`. The architecture brief should decide up front which of
   those two outcomes AC-006 is actually asking for, rather than assuming the
   trap is still live. (`crates/happenstance-ladybug/src/live_handle.rs:68-83`
   carries the only in-tree transcript of the trap, produced against the
   **current** lifetime'd GAT.)

## Existing code patterns this project must follow or replace

- **`crates/happenstance-ladybug/src/projection_store.rs`** — the skeleton to
  fill in. `GraphStatement`/`GraphWriteSet` (deferred, owned, `Send + 'static`
  write set), `LadybugProjectionStoreError` (five variants including
  `MalformedCheckpoint`/`PositionOutOfRange` for the `INT64`/`NonZeroU64`
  narrowing, and `WriteTransactionInUse` for the single-writer rule), and the
  four `todo!()` bodies at lines 270-292. The module doc (lines 1-64) already
  states the two-condition PS-4 falsifier and records that the "no live handle
  before a traversal" half is **not settled by the skeleton** — that is
  AC-005's read-your-own-writes projection.
- **`crates/happenstance-ladybug/src/live_handle.rs`** — a deliberate
  **instrument, not a shipping adapter** (module doc line 156: "Not a
  shipping adapter"). Holds the only in-tree ICE transcript (lines 38-66,
  "DefId::expect_local" panic when a non-`'static` store implements the port
  on rustc 1.97.1) and the E0195 transcript (lines 68-83). Both are evidence
  for the risk in project.md's "The GAT and the ICE" row. Note for the
  architecture brief: `LiveHandleProjectionStore` still imports
  `crate::stand_in::{Connection, Database}` (line 92) and would, on a literal
  reading of AC-003 ("no `stand_in` type appears on any path the suite
  exercises"), be exempt from retirement since the conformance suite runs
  against `LadybugProjectionStore`, not `LiveHandleProjectionStore` — but that
  reading should be confirmed explicitly rather than assumed, since DR-2's
  broader phrasing ("must be retired when the real `lbug` types land, not
  left beside them") could be read either way.
- **`crates/happenstance-ladybug/src/stand_in.rs`** — the dependency-free
  stand-in for `lbug` 0.16.1 (docs.rs's last buildable version), calibrated
  fact-by-fact against published docs (table at lines 17-25). DR-2 requires
  retiring it from every suite-exercised path when the real driver lands.
- **`crates/happenstance-ladybug/tests/port_shape.rs`** — an existing
  compile-only test (nothing runs; every body is `todo!()`) that instantiates
  generic code at both batch shapes (`weak_flavour`, `send_flavour` modules).
  This is the pattern to extend, not replace, once the suite can actually run:
  it already demonstrates CLAUDE.md constraint 4 (bind `ProjectionStore`, not
  `SendProjectionStore`, in generic code; the two flavour names must stay in
  separate modules or `E0034` ambiguity results).
- **`crates/happenstance-core/src/projection.rs`** — the port itself, `#[status:
  provisional]` (module doc lines 3-11: "a port without a conformance suite is
  a guess"). This project amends nothing here (out of scope, owned by
  `projection-store-freeze`); it only consumes whatever shape lands.
- **`crates/happenstance-testkit/src/lib.rs`, `src/contract.rs`,
  `src/registry.rs`** — the `Fixture`/`Capability`/`RuleOutcome` machinery and
  `event_store_conformance!` macro shape
  (`crates/happenstance-testkit/src/lib.rs:311-357`) that
  `projection_store_conformance!` will mirror once `projection-store-freeze`
  ships it — that macro **does not exist yet** in this tree (confirmed:
  `rg "projection_store_conformance"` over `crates/` returns nothing). The
  fixture this project writes for Ladybug must follow the same "one fixture
  instance is one isolated backing store; `connect()` returns one handle"
  contract (`crates/happenstance-testkit/src/contract.rs:1-63`), and any rule
  the fixture cannot satisfy must decline with a stated reason
  (`contract.rs:25-63`) rather than `#[cfg]`-out — this is DR-5, and it
  consumes `.kb/open-questions/cf-40-fixture-limits-ownership.md` rather than
  re-deciding fixture-constant ownership.
- **`crates/happenstance-core/src/store.rs`'s sibling pattern** and
  `crates/happenstance-testkit/src/fixtures.rs`'s `MemoryFixture` — the
  reference implementation the event-store suite is built against; the
  closest existing precedent for what a Ladybug fixture's shape should look
  like once the projection equivalent exists.
- **`crates/happenstance-ladybug/Cargo.toml`** — currently `publish = false`,
  `lbug` deliberately commented out (not a dependency). AC-003/AC-010 and
  RUNBOOK.md:4432-4438's exit criteria require: adding the real `lbug`
  dependency, removing `publish = false`, and adding `LICENSE-APACHE`,
  `LICENSE-MIT` and `README.md` — the pattern already present in
  `crates/happenstance-core/` (`ls crates/happenstance-core/` shows all three
  plus `Cargo.toml`/`src`/`tests`). `xtask/src/main.rs:519` (`"packaged
  artifacts carry their licences and README"`) is the `cargo package --list`
  assertion AC-010 must satisfy.

## Specification anchors (the clause wins on any conflict)

- **PS-2** (`spec/SPECIFICATION.md:4760-4774`, `[FROZEN]`) — the single gate:
  `CheckpointOnlyStore` must fail `commit_is_atomic_with_the_read_model`, and
  two adapters at opposite ends of the batch-shape axis must pass. This
  project supplies one end of that axis; it does not touch the `[FROZEN]`
  marker (AC-008).
- **PS-3** (`spec/SPECIFICATION.md:4776-4787`, `[PROVISIONAL]`) — whether the
  port ships behind `unstable-projection`. This project's run is a data
  point (`RUNBOOK.md:601`, "6, decided at 12"); the verdict belongs to
  `publication-and-positioning`.
- **PS-34** (`spec/SPECIFICATION.md:5546-5559`, `[PROVISIONAL — contingent on
  PS-5; dead the moment `type Batch;` lands]`) — see the dependency-tension
  note above; AC-006 must be read against whichever state `type Batch` is in
  when this project actually runs.
- **§4.1a's six-question table** (`spec/SPECIFICATION.md:4791-4822`) —
  question 1 ("does any adapter need a live handle acquired before the first
  write") names "The Ladybug skeleton" by name as the confirming instrument
  at phase 11, gating PS-4, PS-5, PS-6, PS-12 and PS-34 together. AC-005's
  read-your-own-writes projection is this project's answer to that question.
- **E2E-19** (`spec/E2E-CASES.md:502-520`) and **E2E-24**
  (`spec/E2E-CASES.md:622-640`) — the two cases RUNBOOK.md:4440 says this
  project's third shape makes writable. E2E-24 in particular is framed around
  exactly this adapter's shape: "an adapter whose storage API offers
  `transactionSync(callback)` and no handle that can be held across an
  `await`... a buffer of pending statements replayed in one call at commit."

## Tensions and risks for the briefs to carry forward

1. **The port-shape dependency above** — every brief must treat
   `projection-store-freeze` landing its `type Batch;` change as a hard
   precondition, not a soft one; the skeleton's current signatures are a
   snapshot of the pre-freeze port.
2. **PS-34's contingent-dead status** — AC-006 needs an explicit reading
   before work starts on it: is the "third implementer" test re-running the
   original E0195 trap (only possible if `type Batch<'a>` somehow survives),
   or confirming the trap is retired? The architecture brief should settle
   this rather than the story map discovering it mid-implementation.
3. **`live_handle.rs`'s continued use of `crate::stand_in`** — flagged above;
   confirm whether AC-003's "no `stand_in` on any suite-exercised path" is
   read to exempt the instrument module or not, since the module is kept
   deliberately (module doc: "It exists so that phase 6 has two unlike batch
   shapes... and so that the claim... has a counter-example that compiles")
   and may need updating in step with `projection_store.rs` even though the
   suite itself never runs against it.
4. **Single-writer concurrency is routine, not exceptional** — LadybugDB
   permits many readers and exactly one writer
   (`crates/happenstance-ladybug/src/projection_store.rs:204-213`); any
   concurrency-shaped projection rule will meet `WriteTransactionInUse`, and
   project.md's own risk register (already correct) states the answer must
   not be "retry until green."
5. **Build cost / CI-shape decision (AC-009)** has no existing precedent in
   this workspace to cite — no other crate here builds C++ through `cxx`/
   `cmake` yet — so the architecture or deployment brief will need to design
   the measurement and the CI-job-vs-three-OS-gate decision from first
   principles, informed only by the module doc's note that "docs.rs itself
   fails to build `lbug` 0.19.1" (`crates/happenstance-ladybug/src/lib.rs:32`).
