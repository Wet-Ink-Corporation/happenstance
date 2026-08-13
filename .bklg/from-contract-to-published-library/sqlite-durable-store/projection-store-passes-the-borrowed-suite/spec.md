---
item: HS-S0044
stage: spec
created: 2026-08-12T13:46:42.889Z
updated: 2026-08-12T13:46:42.889Z
template_sig: 87bbf1d0
rendered_sig: 25b5817b
---

# Spec — SqliteProjectionStore against the suite it did not write

## Scope lock

| Layer | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) |
| Project | [`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md`](../project.md) |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/projection-store-passes-the-borrowed-suite/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — **architecture brief §1** (the mount table, and the row that says this target's macro is owned elsewhere), **§3** (the runtime seam), **§7** (the projection store migrates independently, on its own connection), **§8** (what this project must not touch: the port, the suite, the declension policy), **§9** (DoD 7's second batch shape does *not* live here); **testing brief §1–§2** (the conformance tier and AC-011's row) |
| Upstream project (owns the suite) | [`.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md`](../../projection-store-freeze/_storymap.md) and [`../../projection-store-freeze/_decomposition.md`](../../projection-store-freeze/_decomposition.md) — the port shape, `ProjectionProbe`, `ProjectionFixture`, `projection_store_conformance!` and every rule this story runs |
| Signed-off design | [`../_design.md`](../_design.md) — **N/A, no user-facing surface**, approved 2026-08-12. The determination binds: this story renders no surface. It is not a licence to skip rustdoc on the public Rust items it changes |
| Story map row | [`../_storymap.md`](../_storymap.md) — slice `sqlite-projection-store` (sole member), `capability`, `depends_on: schema-migration-and-identity`, `traces_to: AC-011` |
| Grounding | [`../_grounding.md`](../_grounding.md) — the Accepted atoms and existing code patterns this project was grounded against |
| Roadmap pointer | `RUNBOOK.md:4166-4237` (phase 8 in full); `spec/SPECIFICATION.md:4632-4731` (the port shape this story implements against) |

## One-line PR slice

Give `SqliteProjectionStore` real bodies against its owned `Batch` on an independently-migrated connection, and mount it at `crates/happenstance-sqlite/tests/projection.rs` against the projection suite `projection-store-freeze` froze.

## Executive summary

`crates/happenstance-sqlite/src/projection_store.rs` has three `todo!()`s —
`migrate` (`:98-100`), `checkpoint` (`:227-233`) and `commit` (`:242-253`) — and
two bodies that are already honest: `begin` allocates a `Vec` and `rollback`
drops one. This PR replaces the three, adds the SQL the port's real rules need,
and mounts the result at a **new** `cargo test` target that invokes a macro this
project did not write.

**Delta, not restatement.** `project.md` AC-011 says the projection store passes
`projection-store-freeze`'s suite; the architecture brief's mount table
deliberately leaves the row's right-hand column as *"owned by HS-P0010 — this
project consumes the macro that project ships and does not name it here, because
it does not exist yet"*. That was correct when the brief was written and it is
not a specification an implementer can start from. This spec closes the gap by
naming, from HS-P0010's own planning artifacts, exactly what will be on disk when
this story starts and what this adapter therefore owes.

Three obligations follow that no earlier artifact states, and each is a place an
implementer building from the brief alone would land wrong:

1. **The port is not the port in the tree today.** `owned-batch-port-shape`
   (HS-P0010) lands `spec/SPECIFICATION.md:4632-4731` verbatim: `type Batch;`
   with no lifetime, a non-`async` infallible `begin`, `Checkpoint`, `Authority`,
   `CommitError`, `ResetError` and a `reset` method. This crate's skeleton is
   restated by *that* story with the same `Batch`, the same `Error` and the same
   `todo!()` bodies; this story is the first that has to make the new methods
   *work* — including `reset`, which has no skeleton body at all.
2. **The suite cannot see a read model without a probe, and the probe cannot be
   implemented in `tests/`.** `ProjectionProbe`
   (`spec/SPECIFICATION.md:4998-5031`) is a `happenstance-core` trait behind a
   `conformance` feature, and an adapter's `tests/` directory is a different
   crate where neither the trait nor the type is local. The impl lands in
   `src/projection_store.rs` behind a feature this crate does not have yet.
3. **This adapter is the buffering shape, not the live-transaction shape.**
   PS-2's *Rejects* clause names "an in-process rusqlite transaction" as half the
   monoculture it exists to refuse (`spec/SPECIFICATION.md:4772-4775`) — and this
   adapter is not even that, for two compiler reasons already recorded at
   `crates/happenstance-sqlite/src/projection_store.rs:9-31`. A green run here is
   a **third** instance of replay-at-commit, not a second end of the batch-shape
   axis. Claiming otherwise is the one way this story can do real damage.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is behind a
signposted anchor.

### The port this story implements is not the one in `projection.rs` today

`crates/happenstance-core/src/projection.rs:85-139` still carries
`type Batch<'a> where Self: 'a`, an `async fn begin` returning `Result`, a
`checkpoint` returning `Option<SequencePosition>`, a four-argument-less `commit`
and no `reset`. **Do not implement against it.** By the time this story runs,
`owned-batch-port-shape` (HS-P0010, slice `projection-port-and-probe`) has landed
the shape at `spec/SPECIFICATION.md:4632-4731`, which changes five things:

| Was | Is, when this story starts | Clause |
| --- | --- | --- |
| `type Batch<'a> where Self: 'a` | `type Batch;` | PS-5 |
| `async fn begin(&self) -> Result<Batch, E>` | `fn begin(&self) -> Self::Batch` | PS-6 |
| `checkpoint -> Option<SequencePosition>` | `checkpoint -> Checkpoint` (`NeverRun` / `Live { through }` / `Rebuilding { through }`) | PS-19, PS-24 |
| `commit(batch, id, position) -> Result<(), E>` | `commit(batch, id, position, Authority) -> Result<(), CommitError<E>>` | PS-15, PS-21, PS-22, PS-24 |
| — | `reset(batch, id) -> Result<(), ResetError<E>>` | PS-16 – PS-18 |

Two consequences the skeleton's own comments do not yet carry. `Authority` is not
decoration — a commit says whether the rows it leaves behind are authoritative,
so the checkpoint row must store it, or `Checkpoint::Rebuilding` cannot be
returned and `rebuilding_is_distinguishable_from_live` fails. And `reset` is *the
dual of commit*: the caller's batch carries the deletes, because only the caller
knows which rows are the read model. A `reset` implemented as `DELETE FROM`
anything this adapter chose would be inventing a read model it does not own.

**If the port on disk is not this shape when the story starts, stop.** That is a
blocking upstream finding about HS-P0010, not something to work around by
implementing both shapes.

### The batch is an owned buffer, and that is a compiler result rather than a preference

`projection_store.rs:9-31` records two independent rejections, each confirmed
against this crate: `rusqlite::Connection` is `Send` and not `Sync`, so a store
owning one directly is not `Sync`, so `&Self` is not `Send`, so *every* future in
the `SendProjectionStore` trait is rejected — including `checkpoint`, which never
touches a batch; and `rusqlite::Transaction<'_>` is itself `!Send`, so `commit`
is rejected on the batch **parameter** alone even when the store is wrapped to be
`Sync`. The first is fixed by the `Mutex` at `:60`; the second cannot be fixed at
all without giving up the live handle.

So `SqliteBatch` stays what it is: an owned, `Send`, replayable write set of
`PendingStatement`s (`:113-176`), opened into a real transaction inside `commit`.
That satisfies the port's *obligation* — read model and checkpoint move together
— without the port's *suggested mechanism*. This story does not revisit that
decision; it is the reason the crate compiles at all, and it is what makes the
port's own `type Batch;` change land here as a deletion rather than a redesign.

One consequence for the public surface, and it is a real one: once `begin` is
the only thing that may mint a batch that `commit` will accept (see the stamp,
below), `SqliteBatch: Default` and the public `SqliteBatch::new()` (`:142-152`)
are a way for a caller to build a batch no store owns. They go, or the stamp is
meaningless. `publish = false` is why that is a rename rather than a semver event
(`crates/happenstance-sqlite/Cargo.toml:12`).

### The suite writes through `ProjectionProbe`, and the orphan rule decides where the impl lives

The suite cannot know what a read model is, so every rule writes and reads
through `ProjectionProbe` (`spec/SPECIFICATION.md:4998-5031`):
`READS_THROUGH_BATCH`, `probe_write`, `probe_delete_all`, `probe_read`,
`probe_read_through`. It lives in `happenstance-core` behind `feature =
"conformance"` for a coherence reason the specification spells out at
`:5016-5028`: an adapter implementing a *testkit* trait for its own type in the
adapter's `tests/` directory is a different crate, where neither trait nor type
is local, and the impl is rejected — which would force a non-dev dependency on
`happenstance-testkit`.

**Therefore:** `impl ProjectionProbe for SqliteProjectionStore` lands in
`crates/happenstance-sqlite/src/projection_store.rs`, behind a new
`conformance` feature on this crate that forwards to
`happenstance-core/conformance`. It cannot live in `tests/projection.rs`. The
feature costs one flag on a dependency this crate already has and adds no edge to
the graph.

Two mechanical facts that follow and that a feature-powerset gate will find:

- `conformance` **without** `projection-store` must still compile, so the impl
  and its probe table are gated on both. `crates/happenstance-sqlite/Cargo.toml:28-33`
  has two features today; a third takes the powerset from four combinations to
  eight, and `cargo hack` runs it (`CLAUDE.md` — *Commands*).
- `projection-store` must forward `happenstance-core/unstable-projection` once
  `unstable-projection-gate-and-clause-disposition` (HS-P0010) puts the port
  behind that gate — PS-3, `spec/SPECIFICATION.md:4776-4790`. Without the forward
  this crate stops compiling the moment that story merges, and the failure looks
  like a missing module rather than a missing feature.

`ProjectionProbe` is declared on the **bare** flavour, and this adapter
implements `SendProjectionStore`; the two meet through `trait_variant`'s
derivation, which is the same route `CLAUDE.md` constraint 4 relies on
(`.kb/decisions/0001-async-port-flavours.md`,
`.kb/decisions/0008-one-derivation-for-both-ports.md`). Compile it early rather
than assume it: if it does not hold, that is a finding about where the probe
lives and belongs to HS-P0010, not a local workaround that re-declares the trait.

### The declensions this adapter must state, and the reason each one is honest

`ProjectionFixture` mirrors `Fixture` and reuses `Capability` and `RuleOutcome`
unchanged (`crates/happenstance-testkit/src/contract.rs:355-433`, `:458-537`);
its capability *set* is `projection-store-freeze`'s `_design.md` to fix, under
DT-3. Two declensions are this adapter's to state, with the fixture's own words:

- **`READS_THROUGH_BATCH = false`.** A buffered list of `PendingStatement`s has
  been sent to SQLite exactly never, so there is no open transaction to read
  through; answering from committed state is what PS-12 explicitly forbids
  (`spec/SPECIFICATION.md:5052-5058`). The alternative — a shadow map inside the
  batch — is rejected on the record, because it answers from a second source of
  truth SQLite never sees, and would buy a green `batch_reads_reflect_pending_writes`
  that says nothing about this adapter's real read path. CF-18 requires the
  `false` arm to be emitted as a **reported skip**, not omitted, and the
  machinery for that already exists.
- **Reset refusal.** This adapter protects no projection from reset, so whatever
  constant HS-P0010's design mints for `refused_reset_changes_nothing`
  (`spec/SPECIFICATION.md:5200-5206`) is declined here with that reason. This is
  not a shrug: PS-18's own falsifier says it is *"evaluated at the exit of the
  projection-port phase by asking whether the SQLite adapter implemented it"*
  (`:5203-5206`). This story is the answer to that question. Record the answer;
  do not move the marker.

A declined capability's reason is the adapter's account of a trade only the
adapter can write (`crates/happenstance-testkit/src/contract.rs:440-457`). Two
words and a shrug is a failed review, not a skip.

### The checkpoint schema is this crate's *second* connection, and it migrates itself

An event store and a projection store on one file are two connections, not one
(`../_decomposition.md`, architecture brief §7, last paragraph).
`schema-migration-and-identity` (HS-S0036) has already landed the shared
connection configuration — journal mode, `synchronous`, busy timeout — and
`SqliteProjectionStore::open` already adopts it; that story's spec says so
explicitly and leaves `migrate` as `todo!()` for this one
(`../schema-migration-and-identity/spec.md:356`). So this story lands the
checkpoint schema **only**, and it must be idempotent under a concurrent open for
the same reason the event store's is: `migrate` runs on every `open`
(`crates/happenstance-sqlite/src/projection_store.rs:79-83`).

Do not invent a second answer to the runtime seam. Whatever ADR-0022 settled for
the event store — the architecture brief recommends capturing a
`tokio::runtime::Handle` at construction and keeping `try_current()` as the
fallback (`../_decomposition.md` §3) — is what `SqliteProjectionStoreError::NoRuntime`
(`projection_store.rs:198-200`) must mean here too. One crate, one seam.

### What this story records and must not settle

Four findings leave this story written down and undecided, each with a named
owner. Settling any of them in passing is a scope violation, and the reason each
is here is that this adapter is the first evidence anyone has.

- **PS-2 is not cleared by a green run here.** Its bar wants one adapter holding
  a live transaction and one that cannot hold anything across an await
  (`spec/SPECIFICATION.md:4760-4771`), and its *Rejects* clause names the rusqlite
  pairing as the monoculture to refuse. This adapter is neither end: it is a
  third replay-at-commit shape beside `MemoryProjectionStore` and HS-P0010's
  buffering variant. The finding worth carrying is stronger than "not yet" —
  under `type Batch;` the live-transaction end may not be reachable by *any*
  rusqlite adapter, which is a fact about PS-2's stated rule that
  `publication-and-positioning` (HS-P0016) needs and this story is where it is
  first observable.
- **PS-18** — answered above, recorded, marker untouched.
- **PS-12's `false` arm** — this is the first non-testkit adapter to exercise it.
- **PS-6 and PS-7** are already marked "not this crate's to settle" in the
  skeleton (`projection_store.rs:235-260`); under the new port `begin` is no
  longer `async` or fallible, which resolves PS-6 upstream. The comment goes with
  the signature it annotates; the disposition does not become this crate's.

Hand-off route: this story's `_ledger.md`, then `spec-and-code-reconciliation`
(HS-S0047, same project) and HS-P0016's clause-ledger audit. **No
`[FROZEN]` clause is touched, no `[PROVISIONAL]` marker is moved, and no ADR is
authored in this PR** (`.kb/decisions/README.md`; `CLAUDE.md` — accepted decision
atoms are immutable).

### The persona-journey slice this realizes

The journey is *"Learn when you are finished"* — the adapter author's loop from a
signature that type-checks to a suite that says pass or fail and names why
(`../../initiative.md:245-246`). This story is that loop run against a port the
author did not design, with a suite the author did not write, in a crate that
until now has only ever told the type checker it *could* implement the trait. The
observable increment for both personas the story map names — the adapter author
and the library's consumer — is identical and is the whole point: a projection
store that has **passed**, against a real file, through a macro someone else
owns.

### Boundaries the implementer will be tempted to cross

- **Do not add, rename or reorder a projection conformance rule.** They are
  HS-P0010's. A rule that seems wrong is a finding written into the ledger and
  raised against that project, per `CLAUDE.md` ("fix the rule and explain why in
  the same change") — which is exactly what this project is forbidden from doing
  unilaterally (`../project.md`, *Out of scope*).
- **Do not take DoD 7's second unlike batch shape.** The architecture brief
  answers this before code: it belongs beside the suite, in the testkit
  (`../_decomposition.md` §9). Taking it here gives the DAG an edge it does not
  carry and inverts 6 → 8.
- **Do not touch `SqliteEventStore`.** Different module, different connection,
  different slice. The shared connection configuration is already landed and is
  consumed, not modified.
- **Do not delete `#![allow(clippy::todo)]`** (`crates/happenstance-sqlite/src/lib.rs:76-80`).
  DR-01 says it dies with the *last* `todo!()` in the crate, and after this PR
  the event-store paths may still carry theirs depending on merge order.
  `instrument-markers-removed-and-gate-green` (HS-S0045) owns that line and
  depends on this story.
- **Do not touch `publish = false`, `PUBLISHABLE`, or `Cargo.toml`'s stale
  `description`** (`../_decomposition.md` §8; `crates-io-name-and-packaging-facts`
  owns the description).
- **No new public API in `happenstance-core`.** If something the probe or the
  suite needs is missing there, that is an upstream finding, not a local
  addition (architecture brief AC-A04).

## Integration contract

- **Archetype**: `capability` — a user-observable slice through every layer this
  library has. Its observable outcome is a green macro invocation against durable
  storage, not a module that compiles.
- **Slice / milestone**: `sqlite-projection-store`. It is the **sole member** of
  its slice; there are no slice-mates to mount alongside. The story map records
  that it is independent of `race-model-and-durability` and may be scheduled
  either side of it, because it touches `projection_store.rs` and its own
  connection only (`../_storymap.md`, *Slice coherence notes*).
- **Mount point**: **`crates/happenstance-sqlite/tests/projection.rs`** — a new,
  real `cargo test` target carrying `SqliteProjectionFixture` and the single line
  `happenstance_testkit::projection_store_conformance!(SqliteProjectionFixture::new())`,
  run by `cargo test -p happenstance-sqlite` inside both `cargo xtask affected
  --base main` and `cargo xtask ci --fast`. Both invoke `--all-features`
  (`xtask/src/affected.rs:167-173`, `xtask/src/main.rs:143-153`), which is what
  makes a `conformance`-gated target actually run rather than quietly not
  compile. This is the mount the architecture brief's §1 table names, and
  AC-A01's rule is honoured literally: reachable from `tests/`, never from a
  `mod` only `cargo check` sees.
- **Wires into**:
  - `crates/happenstance-core/src/projection.rs` — the port as
    `owned-batch-port-shape` (HS-P0010) restates it: `Batch`, `begin`,
    `checkpoint`, `commit`, `reset`, `rollback`, plus `Checkpoint`, `Authority`,
    `CommitError`, `ResetError`. Target shape at `spec/SPECIFICATION.md:4632-4731`.
  - `happenstance_core::ProjectionProbe`, behind `feature = "conformance"` —
    `spec/SPECIFICATION.md:4998-5031`. The write/read seam every rule uses.
  - `crates/happenstance-testkit/src/contract.rs` — `ProjectionFixture` (mirrors
    `Fixture`, `:120-125`), `Capability` (`:355-433`) and `RuleOutcome`
    (`:458-537`), the last two reused unchanged so an adapter author learns one
    skip vocabulary.
  - `crates/happenstance-testkit/src/fixtures.rs:243-292` — `MemoryFixture`, the
    shape to read for a fixture and, per architecture brief §1, the `connect`
    shape **not** to copy: one fixture instance is one file, each `connect()` is
    a real `Connection`, not an `Arc` clone.
  - `crates/happenstance-sqlite/src/projection_store.rs` — the skeleton this
    story completes: the `Mutex` handle (`:102-110`), `SqliteBatch` (`:113-176`)
    and `SqliteProjectionStoreError` (`:178-206`), whose variants already
    enumerate the real failure modes.
  - `crates/happenstance-sqlite/src/event_store.rs` — the shared connection
    configuration and runtime seam landed by HS-S0036, consumed unchanged.
  - `crates/happenstance-sqlite/Cargo.toml:28-33` — the feature table, which is
    the second half of "mounted" for a library: an item behind no feature entry
    is invisible (`../../projection-store-freeze/_storymap.md`, preamble).
  - `crates/happenstance-sqlite/tests/shapes.rs` — the type-level guard. It must
    still pass after `SqliteProjectionStore` and `SqliteBatch` gain fields.
- **Renders surfaces**: **none.** `../_design.md` records N/A — no user-facing
  surface — for this entire project, approved 2026-08-12 at the design sign-off
  gate, with `design.capture` a declared skip. The public Rust items this story
  changes (`SqliteBatch`'s constructors, a probe impl, the store's methods) are
  library surface inside that determination and each carries rustdoc with an
  `# Errors` section naming conditions rather than types
  (`standards/rust/70-rustdoc-obligations.md`).
- **Conformance rule(s)** — every rule the suite enumerates for this adapter,
  named so a red run is diagnosable. From `projection-store-freeze`'s story map:
  `commit_advances_the_checkpoint`, `commit_is_atomic_with_the_read_model`,
  `failed_commit_leaves_both_unchanged`, `rollback_leaves_both_unchanged`,
  `dropped_batch_leaves_store_usable`, `commit_rejects_a_foreign_batch`,
  `commit_accepts_a_position_the_batch_did_not_write`,
  `commit_rejects_a_regressing_position`,
  `distinct_projections_advance_independently`,
  `reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`,
  `refused_reset_changes_nothing` (declined → reported skip),
  `fresh_projection_has_no_checkpoint`, `reset_is_not_commit_at_first`,
  `batch_reads_reflect_pending_writes` (declined → reported skip),
  `rebuild_is_chunk_size_invariant`, `rebuilding_is_distinguishable_from_live`.
  **This story adds no rule and changes no port** — it is the implementation side
  of rules that already exist, which is why naming them is the contract rather
  than a courtesy.
- **Clause(s)**: discharges no clause by itself and **amends none**. It supplies
  the first non-testkit evidence for **PS-1** (`spec/SPECIFICATION.md:4744-4759`,
  `[FROZEN]`), **PS-12**'s `false` arm (`:5052-5074`, `[PROVISIONAL]`), **PS-14**
  (`:5086-5098`, `[FROZEN]`), **PS-15**'s run-time stamp (`:5126-5155`,
  `[PROVISIONAL]`), **PS-16 – PS-18** (`:5200-5217`) and **PS-19** (`:5218-5249`,
  `[FROZEN]`), and it records — without acting on — that **PS-2** (`:4760-4775`,
  `[FROZEN]`) is not cleared here and that **PS-18**'s falsifier has now been
  asked. Clause dispositions belong to `projection-store-freeze` and
  `publication-and-positioning`.
- **Advances DoD scenario**: initiative **DoD 7** — *"The projection suite
  discriminates. Two structurally unlike batch shapes pass it, and a deliberately
  wrong implementation that writes a checkpoint without its read model fails it,
  by name"* (`../../initiative.md:377-379`). This story does not complete DoD 7
  (the two unlike shapes are HS-P0010's) and does not clear PS-2; it moves it
  toward green by being the first shipped, durable adapter the suite has ever
  run against, and it feeds **DoD 8**'s freeze verdict with the evidence that
  verdict is supposed to weigh. Project row: **AC-011**, the whole of it.

## PR boundary

```
crates/happenstance-sqlite/src/projection_store.rs
crates/happenstance-sqlite/src/lib.rs
crates/happenstance-sqlite/Cargo.toml
crates/happenstance-sqlite/tests/projection.rs
.bklg/from-contract-to-published-library/sqlite-durable-store/projection-store-passes-the-borrowed-suite/**
```

`Cargo.toml` is inside the boundary for **one reason only**: the `conformance`
feature entry and the `unstable-projection` forward, both of which are load
bearing for the mount. `publish`, `description`, dependencies and every other key
are out (`../_decomposition.md` §8). `lib.rs` is inside for the module-gate line
that a new feature needs and for nothing else — the status banner and the
`#![allow(clippy::todo)]` are HS-S0045's.

**In this PR**

- Real bodies for `SqliteProjectionStore::migrate`, `checkpoint` and `commit`,
  and a real `reset` against the port as restated by HS-P0010.
- The checkpoint schema, migrated idempotently on this store's own connection,
  carrying the projection id, the position and the authority.
- A per-store-instance stamp minted in the constructor, carried on `SqliteBatch`,
  and compared in `commit`, `reset` and `rollback` to produce
  `CommitError::ForeignBatch` / `ResetError::ForeignBatch`; `SqliteBatch::new`
  and its `Default` withdrawn so `begin` is the only way to mint one.
- `impl ProjectionProbe for SqliteProjectionStore` in `src/`, behind
  `all(feature = "projection-store", feature = "conformance")`, with the probe's
  own table created under the same gate.
- The two new `Cargo.toml` feature facts: `conformance =
  ["happenstance-core/conformance"]` and `projection-store` forwarding
  `happenstance-core/unstable-projection`.
- `crates/happenstance-sqlite/tests/projection.rs`: `SqliteProjectionFixture`
  (one instance is one fresh temp file, each `connect()` a real `Connection`),
  its declared and declined capabilities with stated reasons, and the
  `projection_store_conformance!` invocation.
- The module documentation at `projection_store.rs:1-44` corrected where it
  describes a port that no longer exists (`type Batch<'a>`, the `# Intended
  schema` block) — the moment `migrate` is real, a wrong SQL block above it is
  published documentation contradicting the code beneath it.

**Explicitly not in this PR**

- Any projection conformance rule, `ProjectionFixture`, `ProjectionProbe`,
  `MemoryProjectionStore`, the projection mutant registry, or the second unlike
  batch shape — all `projection-store-freeze` (HS-P0010).
- Any change to `happenstance-core`, including "just one more probe method".
- `SqliteEventStore`, `tests/conformance.rs`, `tests/migration.rs`, the
  concurrency family, the mutant registry rows — this project's other slices.
- Deleting `#![allow(clippy::todo)]` or rewriting `lib.rs`'s status banner —
  HS-S0045 (DR-01).
- `spec/SPECIFICATION.md` edits, marker moves, and ADR authoring — recorded as
  findings instead, per `CLAUDE.md` and the ADR-queue discipline.
- `publish`, `PUBLISHABLE`, the crate description, README and licence files —
  `crates-io-name-and-packaging-facts`.

**Merge DoD one-liner** — `cargo xtask affected --base main` green with
`projection_store_conformance!(SqliteProjectionFixture::new())` running against a
real temporary file, every enumerated projection rule reported as `Ran` or as
`Skipped` carrying this fixture's own stated reason, no `todo!()` left in
`projection_store.rs`, `cargo hack` clean over the widened feature powerset, and
the PS-2 / PS-18 / PS-12 findings written into `_ledger.md` rather than into
`spec/SPECIFICATION.md`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The suite runs whole, against a file** | `projection_store_conformance!(SqliteProjectionFixture::new())` in a new test target; every rule in `for_each_projection_store_rule!` appears in the binary as `Ran` or `Skipped { capability, reason }`. A rule absent from the run is the failure mode this contract exists to forbid. | `crates/happenstance-testkit/src/contract.rs:458-537`; `../_decomposition.md` architecture brief §1 (mount table) |
| **One fixture instance is one fresh file; each `connect()` is a real connection** | Not an `Arc` clone. Two instances share nothing; `commit_rejects_a_foreign_batch` needs two *isolated* stores, which is two `open()` calls, and explicitly not a second handle. | `crates/happenstance-testkit/src/fixtures.rs:243-292` (the shape to read, not copy); `spec/SPECIFICATION.md:5126-5155` |
| **`begin` is infallible, synchronous, and allocates** | Under the restated port `fn begin(&self) -> Self::Batch`. The existing body already is this in substance (`Ok(SqliteBatch::new())`); the change is deleting the round trip the port no longer asks for. | `spec/SPECIFICATION.md:4690-4694`; `crates/happenstance-sqlite/src/projection_store.rs:235-240` |
| **`commit` is one `BEGIN IMMEDIATE`: replay, checkpoint, commit** | Every `PendingStatement` in batch order, then the checkpoint upsert, then `COMMIT`. Read-model rows and the checkpoint are never two transactions and never two connections — the shape PS-1's *Rejects* clause names as the natural wrong one. | `spec/SPECIFICATION.md:4744-4759`; `crates/happenstance-sqlite/src/projection_store.rs:242-253` |
| **The regression check happens *inside* the transaction** | `commit` reads the current checkpoint under the same `BEGIN IMMEDIATE` that writes it, and returns `CommitError::CheckpointRegression { current, attempted }`. Checking before the transaction opens lets two commits interleave and both pass. | `spec/SPECIFICATION.md:4668-4675`; ADR-0012's precondition ordering, `.kb/decisions/0012-append-shape-and-preconditions.md` |
| **`CommitError` variants are not folded into `Store`** | `ForeignBatch` and `CheckpointRegression` are distinct arms and the rules assert on them. Mapping either into `CommitError::Store(SqliteProjectionStoreError::…)` passes a naive read of "it errored" and fails the rule. | `spec/SPECIFICATION.md:4668-4683` |
| **A foreign batch is rejected by an instance stamp, not by a lifetime** | Dropping the GAT does not close the hazard — a lifetime names a region, not an instance. `begin` stamps the batch with an identity minted per store instance; `commit`, `reset` and `rollback` compare it. With an owned batch the stamp is a field and the check is an integer comparison. Cloning the store (it is `Clone` over an `Arc`) must **not** change the stamp: a clone is the same store. | `spec/SPECIFICATION.md:5099-5125`; `crates/happenstance-sqlite/src/projection_store.rs:58-70` |
| **A rejected call leaves both stores unchanged** | The stamp is compared before any SQL is issued, so a foreign batch never opens a transaction. | `spec/SPECIFICATION.md:5126-5132` |
| **`SqliteBatch` can only be minted by `begin`** | `SqliteBatch::new()` and `#[derive(Default)]` are withdrawn; `push` and the accessors stay, because queueing the application's own read-model SQL is the caller's whole job. Free to do: `publish = false`. | `crates/happenstance-sqlite/src/projection_store.rs:142-176`; `crates/happenstance-sqlite/Cargo.toml:12` |
| **`checkpoint` returns the three-state enum, read back from the row** | `NeverRun` when no row exists; `Live { through }` / `Rebuilding { through }` from the stored authority. A stored position that is not a valid `SequencePosition` is `InvalidPosition`, never a silent zero — the variant already exists. | `spec/SPECIFICATION.md:4643-4660`; `crates/happenstance-sqlite/src/projection_store.rs:202-206` |
| **`Authority` is stored, so a rebuild is distinguishable from live** | `commit(..., Authority::Rebuilding)` leaves rows a reader must not trust, and `checkpoint` says so. Without the column, `rebuilding_is_distinguishable_from_live` cannot pass by any means. | `spec/SPECIFICATION.md:4658-4666` |
| **`reset` applies the caller's deletes and returns the id to `NeverRun`, as one unit** | The dual of `commit`: the batch carries the deletes because only the caller knows the read model. One transaction, scoped to one projection id, and distinguishable from `commit(empty, id, FIRST, Live)`. | `spec/SPECIFICATION.md:4706-4716`, `:5218-5230` |
| **This adapter refuses no reset, and says so** | `ResetError::Refused` is never returned; the fixture declines the corresponding capability with the real reason (no projection here is protected). That answer *is* PS-18's falsifier being asked for the first time — recorded in the ledger, marker untouched. | `spec/SPECIFICATION.md:5200-5206` |
| **`rollback` and a bare `drop` both leave the store usable** | Nothing was ever sent to SQLite, so both are a drop — and the second batch that the rule opens and commits afterwards must succeed. The rule exists because a pooled connection whose `Drop` returns to nothing answers `Busy` forever; this adapter is free of that by construction and the rule should still be run, not assumed. | `spec/SPECIFICATION.md:4898-4910`; `crates/happenstance-sqlite/src/projection_store.rs:255-260` |
| **`READS_THROUGH_BATCH = false`, and the rule reports a skip** | A buffer that has issued no statement has nothing to read through, and answering from committed state is forbidden. CF-18 requires the rule be emitted and skipped with a stated reason, never omitted. The shadow-map alternative is rejected on the record. | `spec/SPECIFICATION.md:5052-5074`; `crates/happenstance-testkit/src/contract.rs:458-537` |
| **`probe_write` / `probe_delete_all` are synchronous and infallible by signature** | They can only *queue* SQL — which is exactly what an owned buffer does. Worth stating because it is evidence for the port's own shape decisions, and because an implementer who reaches for a connection here will find the signature refuses. | `spec/SPECIFICATION.md:5007-5008` |
| **The probe's table exists only under `conformance`** | `probe_write` needs somewhere to write; that table is this crate's test read model and must not appear in a production schema. Created by `migrate` under the same `cfg` as the impl. | `spec/SPECIFICATION.md:5026-5027` (why `probe_delete_all` exists at all) |
| **`probe_read` reads committed state through the same blocking seam as every other method** | No `MutexGuard` held across an await — that would cost the adapter its `SendProjectionStore` impl, which is the whole reason `handle()` exists. | `crates/happenstance-sqlite/src/projection_store.rs:102-110` |
| **Migration is idempotent and safe under a concurrent open** | `migrate` runs on every `open`, and two opens of one file can race. `CREATE TABLE IF NOT EXISTS` inside one transaction, on the connection configuration HS-S0036 already landed. | `crates/happenstance-sqlite/src/projection_store.rs:79-100`; `../_decomposition.md` §7 |
| **One crate, one runtime seam** | The projection store adopts ADR-0022's answer for acquiring a runtime; `NoRuntime`'s doc comment stays true or is rewritten in the same change. A second, different answer inside one crate is the defect. | `../_decomposition.md` §3; `crates/happenstance-sqlite/src/projection_store.rs:198-200` |
| **The feature powerset compiles clean** | `conformance` without `projection-store` compiles (the impl and probe table are gated on both); `projection-store` forwards `happenstance-core/unstable-projection`; `--no-default-features` builds neither. Four combinations become eight and `cargo hack` runs them. | `crates/happenstance-sqlite/Cargo.toml:28-33`; `CLAUDE.md` — *Commands* |
| **The shape guard still holds** | `SqliteProjectionStore: Send + Sync`, `SqliteBatch: Send`, `SqliteProjectionStoreError: Error + Send + Sync + 'static` after the stamp and any new field. No `rusqlite` handle that borrows the connection may reach a field. | `crates/happenstance-sqlite/tests/shapes.rs`; `.kb/decisions/0009-error-send-sync.md` |
| **No `todo!()` remains in `projection_store.rs`** | Three go and none arrives. `probe_read_through` under `READS_THROUGH_BATCH = false` is `unimplemented!()` with a documented panic — the specification permits exactly that, and `clippy::todo` (denied workspace-wide) is not what it would trip. | `spec/SPECIFICATION.md:5011-5012`; `Cargo.toml:124-128` |
| **The module documentation stops describing a port that is gone** | The `# Why the batch is a buffer` argument survives verbatim — it is still true and still load-bearing — but references to `type Batch<'a>` and the `# Intended schema` block are corrected to what the code now is and does. | `crates/happenstance-sqlite/src/projection_store.rs:1-44` |
| **PS-2 is not claimed** | The run is recorded as a third replay-at-commit shape, not as the live-transaction end of the batch-shape axis. Reporting PS-2 as satisfied here is the one outcome that would make a downstream freeze verdict wrong. | `spec/SPECIFICATION.md:4760-4775`; `../../projection-store-freeze/_decomposition.md` §3 |

## Data and migrations

**Migration 2 of this crate, on the projection store's own connection.** There is
no existing data and no upgrade path: `SqliteProjectionStore::migrate` is
`todo!()` today (`crates/happenstance-sqlite/src/projection_store.rs:98-100`), so
this crate has never written a checkpoint row anywhere. It is a create, not an
alter. It is *not* part of the event store's migration 1 and must not be folded
into it: an event store and a projection store on one file are two connections,
and either may be opened without the other (both are separate crate features).

**Versioning.** Whatever marker `schema-migration-and-identity` chose for the
event store — `PRAGMA user_version` or a schema-version row — this schema carries
its own, on its own terms. If both roles are opened against one file, two
independent version markers in one database is a fact worth documenting in the
module header rather than a collision worth resolving by sharing one counter: the
two schemas advance on different stories' schedules.

**What the file holds after this migration.**

| Object | Purpose | The wrong shape it forbids |
| --- | --- | --- |
| `projection_checkpoint(projection_id TEXT PRIMARY KEY, position INTEGER NOT NULL, authority …)` `WITHOUT ROWID` | One row per projection: where it has been brought to, and whether its rows are authoritative | The skeleton's own sketch (`projection_store.rs:35-40`), which has **no authority column** — with it, `Checkpoint::Rebuilding` is unrepresentable and `rebuilding_is_distinguishable_from_live` cannot pass by any implementation |
| `authority`, stored as a discriminant (text or integer), never inferred | `Live` versus `Rebuilding`, read back into `Checkpoint` | Inferring "rebuilding" from the absence of a row, which collides with `NeverRun` — the exact `(None, true)` state the enum exists to make unrepresentable (`spec/SPECIFICATION.md:4643-4651`) |
| `position INTEGER NOT NULL`, validated on read | `SequencePosition` wraps a `NonZeroU64`; a stored `0` or a negative is `InvalidPosition`, which is already a variant | Widening a stored position silently, or returning `Checkpoint::Live { through: 1 }` for a corrupt row |
| Absence of a row | `Checkpoint::NeverRun` — and the state a successful `reset` returns an id to | A sentinel row with position `0`, which makes `fresh_projection_has_no_checkpoint` and `reset_is_not_commit_at_first` answer the same way for two different states |
| The probe table, under `feature = "conformance"` only | The suite's read model: `probe_write` and `probe_delete_all` queue against it, `probe_read` reads it back | Creating it unconditionally, which ships a conformance artefact into an application's database |
| No read-model tables | They are the application's business; this adapter owns the checkpoint and the transaction that carries it | An adapter that invents a read model in order to make `reset` implementable without the caller's batch |

**Read-model data belongs to the caller.** `reset` deletes through the batch the
caller supplies. Nothing in this migration creates, owns or truncates an
application table, and an implementation that does has re-answered a port
question that is not this crate's.

**Reversibility.** None is offered and none is owed: nothing outside this
workspace consumes this schema, the crate is `publish = false`, and whether
`happenstance-sqlite` is ever published is `publication-and-positioning`'s
decision (`../project.md`, *Out of scope*).

## Acceptance criteria

The persona is the one the story map names for every capability story in this
project — **the adapter author, and the library's consumer, whose observable
increment is identical**: *a store that has passed the bar, against durable
storage, through a suite it did not write* (`../_storymap.md`, preamble). The
journey is *"Learn when you are finished"* — from a signature that type-checks to
a suite that says pass or fail and names why
(`../../initiative.md:245-246`;
[`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md)).
Each criterion below is that loop crossing the whole stack — port, adapter, SQL
file, macro — not a capability restated.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an adapter author who has only ever been told by the type checker that `SqliteProjectionStore` *could* implement the port, **WHEN** they run `cargo test -p happenstance-sqlite --all-features`, **THEN** `projection_store_conformance!(SqliteProjectionFixture::new())` executes at `crates/happenstance-sqlite/tests/projection.rs` against a real temporary SQLite **file**, and **every** rule the suite enumerates appears in the run as `Ran` or as `Skipped { capability, reason }` — never absent, and never against an in-memory stand-in. | `crates/happenstance-sqlite/tests/projection.rs` (new target) — the whole-suite run; rule presence read from the run's own output, the way `crates/happenstance-testkit/src/registry.rs` already asserts no rule is orphaned from its macro. Command: `cargo test -p happenstance-sqlite --all-features`, inside `cargo xtask affected --base main`. |
| **AC-002** | **GIVEN** a consumer who must trust that two projection stores in one process do not silently share a database, **WHEN** the suite builds two fixture instances and opens handles from each, **THEN** one fixture instance is one fresh file and each `connect()` is a real `rusqlite::Connection` onto it — never an `Arc` clone — so `commit_rejects_a_foreign_batch` is exercised against two genuinely isolated stores rather than two names for one. | `crates/happenstance-sqlite/tests/projection.rs::SqliteProjectionFixture` — reviewed body (a `connect()` that clones a handle passes the rule and fails this criterion, the same trap `../_storymap.md` records for `SqliteFixture`), plus the green `commit_rejects_a_foreign_batch` and `distinct_projections_advance_independently` rules in the same run. |
| **AC-003** | **GIVEN** an author whose read model must never be ahead of, or behind, its checkpoint after a crash, **WHEN** `commit` is called with a batch of queued statements, **THEN** the replay, the checkpoint upsert and the `COMMIT` happen inside **one** `BEGIN IMMEDIATE` on one connection; and **WHEN** the commit fails, or the batch is rolled back, or the batch is simply dropped, **THEN** read model and checkpoint are both unchanged and the store still accepts and commits the next batch. | `commit_is_atomic_with_the_read_model`, `commit_advances_the_checkpoint`, `failed_commit_leaves_both_unchanged`, `rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable` — all in `crates/happenstance-sqlite/tests/projection.rs`'s suite run. |
| **AC-004** | **GIVEN** a consumer deciding whether the rows they are about to read can be trusted, **WHEN** they call `checkpoint(id)`, **THEN** they get `NeverRun` for a projection that has never committed, `Live { through }` after an authoritative commit and `Rebuilding { through }` after a rebuild commit — read back from a stored authority column, never inferred from the absence of a row — and a stored position that is not a valid `SequencePosition` surfaces as `InvalidPosition` rather than a silent zero. | `fresh_projection_has_no_checkpoint`, `rebuilding_is_distinguishable_from_live`, `commit_advances_the_checkpoint`; plus a targeted test in `crates/happenstance-sqlite/tests/projection.rs` that writes a corrupt row through the fixture's own connection and asserts `SqliteProjectionStoreError::InvalidPosition` (`crates/happenstance-sqlite/src/projection_store.rs:202-206`). |
| **AC-005** | **GIVEN** two runners that both believe they own a projection, **WHEN** each commits, **THEN** the loser is refused with a **distinct** `CommitError::CheckpointRegression { current, attempted }` — decided by a read taken *inside* the same `BEGIN IMMEDIATE` that writes, so the two cannot interleave and both pass — while a commit carrying a position **ahead of** anything the batch itself wrote is accepted, because a projection legitimately advances past events it filtered out. | `commit_rejects_a_regressing_position` and `commit_accepts_a_position_the_batch_did_not_write` in the suite run; reviewed `commit` body showing the checkpoint read is issued after `BEGIN IMMEDIATE`, not before. |
| **AC-006** | **GIVEN** an author holding batches from two stores, **WHEN** they hand store A's batch to store B, **THEN** B refuses it with `CommitError::ForeignBatch` **before any SQL is issued** — decided by a per-store-instance stamp minted in the constructor and compared in `commit`, `reset` and `rollback`, not by a lifetime — leaving both stores untouched; and **WHEN** they try to build a batch by hand, **THEN** they cannot: `SqliteBatch::new()` and its `Default` are withdrawn so `begin` is the only mint, while cloning the store (it is `Clone` over an `Arc`) does **not** change the stamp, because a clone is the same store. | `commit_rejects_a_foreign_batch` green; a test in `crates/happenstance-sqlite/tests/projection.rs` asserting a *cloned* store accepts its origin's batch; compile-fail evidence or reviewed absence of `SqliteBatch::new`/`Default` at `crates/happenstance-sqlite/src/projection_store.rs:142-176`. |
| **AC-007** | **GIVEN** an author who must rebuild a read model from scratch, **WHEN** they queue their own deletes into a batch and call `reset(batch, id)`, **THEN** those deletes and the removal of the checkpoint happen as one unit scoped to that one projection id, leaving every other projection's checkpoint and rows untouched, and afterwards `checkpoint(id)` reports `NeverRun` — distinguishable from `commit(empty, id, FIRST, Live)`. The adapter deletes nothing it chose itself: the read model is the caller's. | `reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`, `reset_is_not_commit_at_first` in the suite run; reviewed `reset` body containing no `DELETE FROM` against a table this adapter named. |
| **AC-008** | **GIVEN** an adapter author following the same route for their own crate, **WHEN** they look for where the suite's read/write seam is implemented, **THEN** they find `impl ProjectionProbe for SqliteProjectionStore` in `crates/happenstance-sqlite/src/projection_store.rs` behind `all(feature = "projection-store", feature = "conformance")` — not in `tests/`, where the orphan rule rejects it — with the probe's table created by `migrate` under that same gate and absent from any production schema; and **WHEN** the store is opened twice concurrently on one path, **THEN** `migrate` is idempotent on this store's **own** connection, carrying its own version marker, folded into neither the event store's migration nor its connection. | `crates/happenstance-sqlite/tests/projection.rs` — the whole suite (every rule writes through the probe, so a missing or wrong probe is a red suite, not a silent pass) plus a concurrent-open test asserting two `SqliteProjectionStore::open` calls on one path both succeed; `cargo build -p happenstance-sqlite --no-default-features --features conformance` compiling clean. |
| **AC-009** | **GIVEN** an evaluator reading the run to decide what this adapter actually proved, **WHEN** a capability is declined, **THEN** the rule still appears in the binary as a reported skip carrying this fixture's **own stated reason** — `READS_THROUGH_BATCH = false` because a buffer that has issued no statement has nothing to read through and answering from committed state is forbidden; the reset-refusal capability declined because no projection here is protected — and neither reason is a shrug: each names the trade and the alternative that lost (the shadow map inside the batch, rejected on the record). | `batch_reads_reflect_pending_writes` and `refused_reset_changes_nothing` present in the run as `Skipped { capability, reason }` (`crates/happenstance-testkit/src/contract.rs:440-457`, `:458-537`); reviewed reason strings in `crates/happenstance-sqlite/tests/projection.rs`. |
| **AC-010** | **GIVEN** a consumer who will `cargo add` this crate with an arbitrary feature selection and read its docs, **WHEN** the gate runs, **THEN** `crates/happenstance-sqlite/src/projection_store.rs` contains **no `todo!()`**, `cargo hack` is clean over the widened powerset (`conformance` without `projection-store` compiles; `projection-store` forwards `happenstance-core/unstable-projection`; `--no-default-features` builds neither), `tests/shapes.rs` still holds after the stamp and any new field, and the module documentation no longer describes a port that is gone — every public item this story changes carrying rustdoc with an `# Errors` section naming conditions rather than types. | `rg -n "todo!" crates/happenstance-sqlite/src/projection_store.rs` empty; `cargo hack check -p happenstance-sqlite --feature-powerset`; `cargo test -p happenstance-sqlite --test shapes`; `cargo doc -p happenstance-sqlite --all-features` under `-D warnings` inside `cargo xtask ci --fast`. |
| **AC-011** | **GIVEN** a reader of the eventual freeze verdict, **WHEN** they open this story's `_ledger.md`, **THEN** they find PS-2 recorded as **not cleared** — this run is a *third* replay-at-commit shape, not the live-transaction end of the batch-shape axis, and the stronger finding that under `type Batch;` that end may be unreachable by *any* rusqlite adapter — PS-18's falsifier recorded as now asked and answered, and PS-12's `false` arm recorded as first exercised by a non-testkit adapter; and **THEN** `spec/SPECIFICATION.md` is unedited, no maturity marker has moved, no projection conformance rule was added, renamed or reordered, and no ADR was authored in this PR. | `_ledger.md` rows carrying the three findings with cited `file:line`; `git diff --stat` showing no change under `spec/`, `.kb/decisions/`, `crates/happenstance-core/` or `crates/happenstance-testkit/`; `cargo xtask spec-trace` citation count not fallen. |

Project AC coverage: **AC-011** of `../project.md` — *"the projection store passes
the suite it did not write"* — is discharged by AC-001 (the whole-run claim)
resting on AC-002 – AC-009, guarded by AC-010, and bounded by AC-011's
non-claims. Its second half — *whether the second unlike batch shape lives here
or in the testkit* — was answered before code, in the architecture brief §9, and
is deliberately **not** a criterion of this story.

## Interaction quality

**Composition family — N/A, and that is a signed-off determination, not a
skip.** [`../_design.md`](../_design.md) records *"N/A — no user-facing surface"*
for this entire project, approved by the repository owner on 2026-08-12 at the
`/redkiln:plan` design sign-off gate, with `design.capture` a declared skip and
**no named anti-patterns** (its Anti-patterns section is N/A). This story renders
no surface: no screen, no route, no CLI view. There is therefore no presentation,
placement, transience, density-budget or hierarchy invariant to carry, and
inventing one here would contradict an approved design.

One obligation survives that determination and is carried as an AC rather than
prose: the **library's presentation layer is its rustdoc**, and the scope lock
above states plainly that a no-surface finding "is not a licence to skip rustdoc
on the public Rust items it changes"
([`standards/rust/70-rustdoc-obligations.md`](../../../../standards/rust/70-rustdoc-obligations.md)).
That invariant is **AC-010**.

**State family — the library analogue, and every applicable invariant is an
AC-### row above.** For an adapter under a borrowed suite, "state" is the store's
observable state across a call and the run's own legibility. Which row carries
which:

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Non-occlusion** — a declined capability must still be *visible* in the run, reported with a reason, never silently absent from the binary | **AC-001**, **AC-009** | Every enumerated rule present as `Ran` or `Skipped { capability, reason }`; the two declensions read from the run's output |
| **In-place, not a context jump** — a rejected call must not move the store: the stamp is compared before any SQL is issued, so a foreign batch never opens a transaction | **AC-006**, **AC-003** | `commit_rejects_a_foreign_batch`, `failed_commit_leaves_both_unchanged` |
| **Preserved selection** — the caller's read model stays the caller's; `reset` applies only the deletes the caller queued, and the adapter truncates nothing it chose | **AC-007** | `reset_is_scoped_to_one_projection`; reviewed `reset` body |
| **Reversibility** — `rollback` and a bare `drop` are both a no-op that leaves the store usable, and `reset` returns a projection to `NeverRun` rather than to a sentinel | **AC-003**, **AC-007**, **AC-004** | `rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable`, `reset_is_not_commit_at_first`, `fresh_projection_has_no_checkpoint` |
| **Reachability** — the "keyboard reachability" analogue: the capability must be reachable through the repository's own merge-gate command, from a real `tests/` target, never from a `mod` only `cargo check` sees (architecture brief AC-A01) | **AC-001**, **AC-010** | `cargo xtask affected --base main` and `cargo xtask ci --fast` both running `cargo test -p happenstance-sqlite --all-features` |
| **Legible failure** — a red run must name *which* rule failed and a rejection must name *why*: `ForeignBatch` and `CheckpointRegression` stay distinct arms rather than folding into `CommitError::Store` | **AC-005**, **AC-006** | The two rules asserting on the specific variants; `# Errors` rustdoc naming conditions (AC-010) |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A batch minted by a different store instance reaches `commit`, `reset` or `rollback` | `CommitError::ForeignBatch` / `ResetError::ForeignBatch`, decided by stamp comparison **before** any statement is prepared; both stores unchanged. Never folded into `CommitError::Store` (AC-006) |
| **EC-002** | The attempted position is not ahead of the stored checkpoint | `CommitError::CheckpointRegression { current, attempted }`, decided by a read taken inside the same `BEGIN IMMEDIATE`; the transaction rolls back and the read model is unchanged (AC-005) |
| **EC-003** | The stored `position` column is `0`, negative, or otherwise not a `SequencePosition` | `SqliteProjectionStoreError::InvalidPosition` (`crates/happenstance-sqlite/src/projection_store.rs:202-206`) — never a silent zero, never `Live { through: 1 }` (AC-004) |
| **EC-004** | Two opens of one file race `migrate`, or a commit meets a writer holding the write lock | `CREATE TABLE IF NOT EXISTS` inside one transaction makes migration idempotent; contention is resolved by the finite busy timeout `schema-migration-and-identity` already landed. **No retry loop, watchdog or `#[timeout]` may be added around a rule** — a hang is a finding about the timeout value, not something a test papers over (`../_decomposition.md` testing brief §4) |
| **EC-005** | A blocking call is issued with no reachable Tokio runtime | `SqliteProjectionStoreError::NoRuntime` (`:198-200`), meaning exactly what ADR-0022's runtime seam settled for the event store. One crate, one seam; a second, different answer inside this crate is the defect |
| **EC-006** | The port on disk when this story starts is still `type Batch<'a>` / `async fn begin` / `Option<SequencePosition>` — i.e. `owned-batch-port-shape` (HS-P0010) has not landed | **Stop and raise it as a blocking upstream finding.** Do not implement both shapes, do not shim, do not vendor a local copy of the port. Recorded in `_ledger.md` against AC-011 |
| **EC-007** | `probe_read_through` is called while `READS_THROUGH_BATCH = false` | A documented `unimplemented!()` panic — permitted by `spec/SPECIFICATION.md:5011-5012`, and not what workspace-denied `clippy::todo` trips. It must be documented on the item, not left to a reader to discover from a backtrace (AC-009, AC-010) |
| **EC-008** | Something the probe or a rule needs is missing from `happenstance-core` | An upstream finding against HS-P0010, never a local addition to `happenstance-core` (architecture brief AC-A04). The PR boundary above does not include that crate |
| **EC-009** | A projection conformance rule appears wrong while implementing against it | Written into `_ledger.md` as a finding and raised against `projection-store-freeze`. This project may not add, rename or reorder a rule (`../project.md`, *Out of scope*), which is the one exception to `CLAUDE.md`'s "fix the rule in the same change" |

## Non-functional

| id | requirement | why it is load-bearing here |
| --- | --- | --- |
| **NF-001** | **No `MutexGuard` is held across an `await`** anywhere in the new bodies, and no `rusqlite` handle that borrows the connection reaches a struct field | It is the whole reason `handle()` exists (`crates/happenstance-sqlite/src/projection_store.rs:102-110`). Losing it costs the adapter its `SendProjectionStore` impl, which `tests/shapes.rs` catches — but only if the field is what changed, so review the awaits too ([`standards/rust/24-the-blocking-bridge.md`](../../../../standards/rust/24-the-blocking-bridge.md), [`standards/rust/25-what-removes-send-and-sync.md`](../../../../standards/rust/25-what-removes-send-and-sync.md)) |
| **NF-002** | **No clock, timeout, watchdog or retry loop is introduced around a conformance rule** | The projection suite's analogue of CF-33. A rule that reads a clock is non-deterministic; a rule wrapped in a timeout converts a real deadlock into a flake (`../_decomposition.md` testing brief §4) |
| **NF-003** | **No new dependency and no new edge in the workspace graph.** The `conformance` feature forwards a flag on a dependency this crate already has; the fixture's temp file is a process-local ordinal plus `Drop` cleanup, not a new crate | The dependency rule in `CLAUDE.md` and architecture brief §11. A crate added for a fixture is a crate a publisher has to justify later ([`standards/rust/50-dependency-hygiene.md`](../../../../standards/rust/50-dependency-hygiene.md)) |
| **NF-004** | **The feature powerset stays coherent as it doubles.** Three features means eight combinations; each must compile, and `--no-default-features` must build neither role | `cargo hack` runs it in the full gate, so an incoherent `cfg` fails somebody else's story rather than this one ([`standards/rust/51-features-and-no-std.md`](../../../../standards/rust/51-features-and-no-std.md)) |
| **NF-005** | **The conformance artefact never reaches a production schema.** The probe table is created only under `feature = "conformance"` | A test read model shipped inside an application's database is a defect that no test in this repository would catch, because every test enables the feature (AC-008) |
| **NF-006** | **`clippy -D warnings`, `cargo fmt`, and the rustdoc obligations hold on every touched item** — `# Errors` sections naming conditions, not types | The gate denies warnings workspace-wide; rustdoc is this library's only presentation surface (`../_design.md` is N/A on everything else) |
| **NF-007** | **The run is deterministic and leaves no files behind.** One fixture instance owns one temp file and removes it on `Drop`; nothing is shared between instances | Otherwise `commit_rejects_a_foreign_batch` and the isolation rules become order-dependent, and a green run stops meaning anything (AC-002) |

## Implementation notes (non-prescriptive)

Directions, not instructions — the implementer owns the code.

- **Read the port on disk first, then this spec.** The Context pack's five-row
  table is what *should* be there when you start. If it is not, EC-006 applies
  and the story stops; if it is there but differs in a detail, the code on disk
  wins over this spec's paraphrase, and the difference is a ledger note.
- **Compile the probe impl early, before writing any SQL.** The `trait_variant`
  route from a bare-flavour `ProjectionProbe` to a `SendProjectionStore` adapter
  is asserted in the Context pack, not proved. Finding out on day three that the
  probe cannot be implemented from `src/` is the expensive version of the same
  discovery.
- **The stamp is a small decision with a large blast radius.** An `AtomicU64`
  ordinal minted in the constructor and copied into the `Arc`ed inner state is
  enough; the constraint that decides it is that `Clone` must not change it
  (AC-006). A `usize` derived from a pointer address is the shape to avoid — an
  `Arc` can be freed and a new allocation land at the same address.
- **Write `migrate` and the probe table in the same transaction** so a
  half-migrated file cannot exist, and keep the probe table's `CREATE` under the
  same `cfg` as the impl rather than under a runtime `if`.
- **Do the checkpoint read and write with one prepared statement each, inside the
  transaction**, and let SQLite's `INSERT … ON CONFLICT DO UPDATE` carry the
  upsert. The regression check is a comparison on the row you just read under the
  lock, not a second round trip.
- **`Authority` wants an explicit stored discriminant.** Text is easier to read in
  a debugger and costs a byte or two; an integer is smaller and needs a documented
  mapping. Either is fine; inferring it is not (AC-004).
- **Correct the module documentation as you replace each body**, not at the end.
  The `# Why the batch is a buffer` argument survives verbatim; the `type
  Batch<'a>` references and the `# Intended schema` sketch do not, and the sketch
  is *known wrong* the moment `migrate` is real — it has no authority column.
- **Read `MemoryFixture` for shape and then do the opposite on `connect()`**
  (`crates/happenstance-testkit/src/fixtures.rs:243-292`): one instance is one
  file, each `connect()` is a real `Connection`.
- **Write the declension reasons as prose a reviewer will read**, in the fixture,
  where the run prints them. They are this adapter's account of a trade only this
  adapter can write (`crates/happenstance-testkit/src/contract.rs:440-457`).
- **Keep the three findings in a scratch note as you hit them** and move them into
  `_ledger.md` at the end. PS-2's stronger form — that the live-transaction end
  may be unreachable by any rusqlite adapter under `type Batch;` — is worth more
  to HS-P0016 than "not cleared", and it is only observable from inside this work.

## Tests and CI (merge gate)

Tiers, commands and targets are the repository's own, wired from
`.redkiln/config.yaml` rather than invented here (`../_decomposition.md` testing
brief §1 and §6; AC-T02).

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static** | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | `spec/SPECIFICATION.md` untouched and its citation count not fallen (**AC-011**); the file-reading lints still clean after the module-doc rewrite (**AC-010**) |
| **Static** | `rg -n "todo!" crates/happenstance-sqlite/src/projection_store.rs` → empty; `git diff --stat` over `spec/`, `.kb/`, `crates/happenstance-core/`, `crates/happenstance-testkit/` → empty | Three `todo!()`s gone and none arrived; the PR boundary held (**AC-010**, **AC-011**) |
| **Unit / type-level** | `cargo test -p happenstance-sqlite --test shapes` (`crates/happenstance-sqlite/tests/shapes.rs`) | `SqliteProjectionStore: Send + Sync`, `SqliteBatch: Send`, the error type's bounds — after the stamp field and any schema state land (**AC-010**, NF-001) |
| **Conformance (this story's real proof)** | `cargo test -p happenstance-sqlite --all-features --test projection` → `crates/happenstance-sqlite/tests/projection.rs` | The whole borrowed suite against a real file: **AC-001** – **AC-007**, **AC-009**. Every rule `Ran` or `Skipped { reason }`, never absent |
| **Conformance (targeted, same target)** | `crates/happenstance-sqlite/tests/projection.rs` — the corrupt-row test, the cloned-store-accepts-its-batch test, the concurrent-`open` test | The three behaviours the borrowed suite cannot see because they are adapter-private: **AC-004** (`InvalidPosition`), **AC-006** (clone keeps the stamp), **AC-008** (idempotent migrate) |
| **Feature matrix** | `cargo hack check -p happenstance-sqlite --feature-powerset`; `cargo build -p happenstance-sqlite --no-default-features --features conformance` | Eight combinations compile; `conformance` without `projection-store` compiles; the `unstable-projection` forward is present (**AC-008**, **AC-010**, NF-004) |
| **Docs** | `cargo doc -p happenstance-sqlite --all-features` under the gate's `-D warnings` | The module doc no longer describes a port that is gone; `# Errors` sections present on changed public items (**AC-010**, NF-006) |
| **Story grain (merge gate)** | `cargo xtask affected --base main` (`affected_gate`, `.redkiln/config.yaml:40`) — `--all-features`, so the `conformance`-gated target actually runs | The gate this story is merged against; the mount is *reachable from the repository's own command*, not just from a hand-typed one (**AC-001**) |
| **Project grain (not this story's bar)** | `cargo xtask ci --fast` (`integration_scoped`, `:55`) | Owned by `instrument-markers-removed-and-gate-green` (HS-S0045). Run it locally if convenient; a red `--fast` caused by an *event-store* `todo!()` is not this story's failure |
| **Ledger** | `_ledger.md` with `require_ledger: true` (`.redkiln/config.yaml:67`) | Every AC-### above carries cited evidence; a green gate is a precondition for reading the criteria, never a substitute (`RUNBOOK.md:38-42`; testing brief AC-T05) |

**Not this story's tiers.** The benchmark tier (`bench` feature) and the mutation
registry are the event-store slices'; the projection suite's own falsifiability —
a deliberately wrong implementation that writes a checkpoint without its read
model, failing *by name* — is DoD 7 and belongs to `projection-store-freeze`
(architecture brief §9). This story must not add a projection mutant to make its
run look more falsifiable than it is.

## Risks and coupling (PR-scoped)

| Risk | Blast radius | Handling |
| --- | --- | --- |
| **The upstream suite has not landed.** `projection-store-freeze` (HS-P0010) owns `projection_store_conformance!`, `ProjectionFixture`, `ProjectionProbe` and the restated port. If it has not merged, there is literally nothing to mount against | Blocks the story entirely | Named as a **project-level** dependency in `../project.md` and `../_storymap.md`, deliberately not a story `depends_on`. Confirm the macro exists before starting; if it does not, this is a scheduling escalation, not a reason to write a local stand-in suite |
| **The port lands in a shape this spec paraphrased wrongly.** Five signatures are described here from HS-P0010's planning artifacts, not from code | The Context pack table and the Behavior table go stale mid-story | EC-006: the code on disk wins, the delta goes in the ledger. Read the port first, in the same sitting as reading this spec |
| **The probe cannot be implemented from `src/` after all** — e.g. the bare/`Send` flavour derivation does not line up as the Context pack assumes | The mount strategy changes shape; the `conformance` feature may be wrong | Compile a stub impl on day one. A failure here is an upstream finding about where `ProjectionProbe` lives, never a local re-declaration of the trait |
| **A green run gets over-claimed as PS-2 cleared.** The most damaging outcome available to this story, because it makes a downstream freeze verdict wrong on evidence that looks strong | `projection-store-freeze`'s DoD 8 verdict; HS-P0016's clause ledger | **AC-011** makes the non-claim a checkable criterion, and the ledger carries the stronger finding about reachability under `type Batch;` |
| **The stamp is implemented as identity-per-`Arc`-clone.** Then a cloned store rejects its own batches and half the rules break in confusing ways | `commit`, `reset`, `rollback` | AC-006 carries an explicit cloned-store test; the Implementation notes name the pointer-address shape to avoid |
| **`Cargo.toml` scope creep.** The file is inside the PR boundary for two feature facts only, and it also carries a stale `description` and `publish = false`, both of which are other stories' | `crates-io-name-and-packaging-facts`, `publication-and-positioning` | PR boundary is explicit; review the `Cargo.toml` diff line by line — it should be two feature lines |
| **Merge-order coupling with HS-S0045.** `#![allow(clippy::todo)]` dies with the *last* `todo!()` in the crate, which may be an event-store one | `crates/happenstance-sqlite/src/lib.rs` | DR-01: this story leaves the allow in place even though its own `todo!()`s are gone. HS-S0045 depends on this story and owns that line |
| **Two version markers in one file** if both roles are opened against one path | Confusing to a future reader, not a correctness bug | Documented in the module header rather than resolved by sharing a counter — the two schemas advance on different stories' schedules (Data and migrations, above) |
| **`schema-migration-and-identity`'s connection configuration changes shape after this lands** | The projection store's `open` consumes it | Consumed, never modified. If the shared configuration is not yet in the form this story expects, that is a real predecessor failure, not something to fork |

## Dependencies

**Blocks on (story slugs, this project):**

- **`schema-migration-and-identity`** — the sole `depends_on`. It lands the shared
  connection configuration (journal mode, `synchronous`, busy timeout) that
  `SqliteProjectionStore::open` already adopts, and it explicitly leaves this
  store's `migrate` as `todo!()` for this story
  (`../schema-migration-and-identity/spec.md:356`). Without it there is no agreed
  answer to how this crate opens a file.

**Blocks on (project-level, outside this project — not a story edge):**

- **`projection-store-freeze` (HS-P0010)** ships the port restatement, the
  `unstable-projection` gate, `ProjectionProbe`, `ProjectionFixture` and every
  rule this story runs. Recorded in `../project.md` *Dependencies* and
  `../_storymap.md`; it is a *project* edge because the whole of this project
  except this one story is independent of it.

**Unlocks:**

- **`instrument-markers-removed-and-gate-green`** (HS-S0045) — names this story in
  its own `depends_on`; it cannot delete `#![allow(clippy::todo)]` until the last
  `todo!()` on a SQLite path, including these three, is gone (`../_storymap.md`,
  merge order 4 → 5).
- **`spec-and-code-reconciliation`** (HS-S0047) — reads this story's recorded
  PS-2 / PS-12 / PS-18 findings when it reconciles the clause range against the
  code.
- Downstream of the project: **`projection-store-freeze`**'s DoD 8 freeze verdict
  and **`publication-and-positioning`** (HS-P0016)'s clause-ledger audit both
  weigh this run's evidence.

**Independent of:** the `race-model-and-durability` slice. This story touches
`projection_store.rs` and its own connection only and may be scheduled either
side of it (`../_storymap.md`, *Slice coherence notes*). It has no slice-mates:
`sqlite-projection-store` has exactly one member.

## Anchors (progressive disclosure)

Open these at the moment named, not before. Everything needed to *start* is
above; these carry the depth that would drown the context pack.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` (PS-1 – PS-24, `:4632-5249`) | The normative port, probe and rule contract in full — the exact clause text behind every row of the Behavior table, including the *Rejects* clauses that name the wrong implementations | Before writing the first body; re-read `:4632-4731` against the code on disk to discharge EC-006 | AC-003, AC-004, AC-005, AC-006, AC-007 |
| `crates/happenstance-sqlite/src/projection_store.rs` | The skeleton being completed, and the recorded compiler argument at `:9-31` for why the batch is an owned buffer — the reasoning is not repeated anywhere else and re-deriving it costs a day | First, before any design choice about `commit`; again at `:1-44` when correcting the module doc | AC-003, AC-006, AC-008, AC-010 |
| `crates/happenstance-core/src/projection.rs` | The port **as it is today** — deliberately the shape *not* to implement against. Reading it is how you tell whether HS-P0010 has landed | Day one, as the first check; then never again | AC-011 (EC-006) |
| `crates/happenstance-testkit/src/contract.rs` | `Fixture` (`:120-125`) — the shape `ProjectionFixture` mirrors — plus `Capability` (`:355-433`), `RuleOutcome` (`:458-537`) and the declension-reason contract at `:440-457` | When writing `SqliteProjectionFixture` and its two declension reasons | AC-002, AC-009 |
| `crates/happenstance-testkit/src/fixtures.rs` (`:243-292`) | `MemoryFixture` — the reference fixture to read for shape, and the one whose `connect()` must **not** be copied here | Alongside the fixture, in the same sitting | AC-002 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` | The upstream story map: the exact rule names, `owned-batch-port-shape`, `unstable-projection-gate-and-clause-disposition`, and which capability constant the reset-refusal rule is gated on | Before declaring capabilities; again if a rule name in this spec does not match the macro | AC-001, AC-009 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The upstream briefs — §3 carries the batch-shape axis argument that PS-2 rests on, which is what makes this story's non-claim precise rather than cautious | When writing the PS-2 finding into `_ledger.md` | AC-011 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` | This project's architecture brief §1 (mount table), §3 (runtime seam), §7 (two connections, independent migration), §8 (what must not be touched), §9 (DoD 7 does not live here); testing brief §1–§2, §4, §6 | §1 and §7 before the mount and the schema; §3 when `NoRuntime` first appears; §4 the moment a test looks like it wants a timeout | AC-001, AC-008, AC-010, EC-004, EC-005 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_storymap.md` | The slice's independence from `race-model-and-durability`, the merge order, and the coverage row that makes this story the sole owner of project AC-011 | When sequencing, and when tempted to absorb a neighbouring story's work | AC-011 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` | The signed-off no-surface determination and its date — the authority for this spec having no composition invariants | Only if someone asks why the Interaction quality section has no composition family | AC-010 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/schema-migration-and-identity/spec.md` | The predecessor's connection configuration and its explicit hand-off leaving this store's `migrate` as `todo!()` (`:356`) | Before writing `open`/`migrate`, to consume rather than re-decide | AC-008 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_grounding.md` | The Accepted atoms and in-tree patterns this project was grounded against — the fastest route from "is this decided?" to the atom that decided it | When a design choice feels like it might already be settled | AC-003, AC-010 |
| `.kb/decisions/0001-async-port-flavours.md` | Why no `#[async_trait]` and why the two flavours exist — the constraint the probe/`SendProjectionStore` meeting point rests on | Before assuming the bare-flavour probe reaches this adapter | AC-008 |
| `.kb/decisions/0008-one-derivation-for-both-ports.md` | One `trait_variant` derivation serving both flavours — the mechanism CLAUDE.md constraint 4 relies on | Same sitting as the probe stub | AC-008 |
| `.kb/decisions/0009-error-send-sync.md` | The error-bound obligation `tests/shapes.rs` enforces on `SqliteProjectionStoreError` | When adding an error variant or a field to the store | AC-010 |
| `.kb/decisions/0012-append-shape-and-preconditions.md` | Preconditions-then-one-transaction ordering — the precedent for reading the checkpoint *inside* `BEGIN IMMEDIATE` rather than before it | While writing `commit` | AC-005 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | Why a rule that no adapter can fail is decorative — the reason this story records findings against the suite instead of quietly satisfying it | When a rule looks wrong, before touching it | AC-009, AC-011 |
| `standards/rust/91-adapter-authoring-recipe.md` | The house recipe for exactly this shape of work, with the named wrong implementations | Before the first body, as the style pass | AC-003, AC-007 |
| `standards/rust/24-the-blocking-bridge.md` | The blocking-bridge pattern: how a `MutexGuard` and an `await` are kept apart | While writing every method that touches `handle()` | NF-001, AC-010 |
| `standards/rust/51-features-and-no-std.md` | Feature-gate hygiene — what a third feature costs and how a `cfg` stays coherent across eight combinations | When adding `conformance` to `Cargo.toml` | AC-008, AC-010 |
| `standards/rust/70-rustdoc-obligations.md` | The `# Errors`-names-conditions rule and the rest of the doc bar — this library's only presentation surface | While writing each public item's docs, not after | AC-010 |
| `.redkiln/config.yaml` | The merge-gate commands by grain, verbatim (`:40`, `:48`, `:55`, `:67`) | When running the gate, and when writing ledger evidence | AC-001, AC-011 |
| `RUNBOOK.md` (`:4166-4237`) | Phase 8 in full — where this work sits in the plan of record and what it is expected to leave behind | For orientation, or when a scope question is not answered by the project artifacts | AC-011 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The *"Learn when you are finished"* journey and the adapter-author persona the acceptance criteria are framed from | If an AC's framing needs to be defended or restated | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the eleven the first pass decided** — AC-001 through
   AC-011, none added and none dropped. The coincidence that this story's own
   AC-011 shares an id with the project AC-011 it traces to is noted so a reader
   does not mistake one for the other: the project's AC-011 is discharged by
   *all eleven* of this spec's criteria, and this spec's AC-011 is the non-claim
   row.
2. **The Interaction quality section has no composition family, and that is
   sourced, not skipped.** `../_design.md` records N/A — no user-facing surface —
   for the whole project, approved 2026-08-12, with `design.capture` a declared
   skip and no named anti-patterns. The one obligation that survives is rustdoc,
   which is carried as an AC row (AC-010) rather than a prose bullet, because
   `redkiln verify` only extracts criteria from table rows and `- AC-###:`
   bullets.
3. **The state-family invariants were translated, not invented.** For an adapter
   under a borrowed suite there is no focus or scroll; the honest analogues are
   run legibility (a skip is reported, never absent), non-movement on rejection,
   reversibility, and reachability through the repository's own gate command.
   Each is mapped to an existing AC row rather than given a row of its own, so
   no criterion is duplicated in the ledger.
4. **Project AC-011's second half is not this story's.** *"…and this project's
   architecture brief records — before code — whether the second unlike batch
   shape DoD 7 owes lives here or in the testkit"* was discharged by the brief
   itself (§9: the testkit), before this story existed. This spec records the
   answer and adds no criterion for it.
5. **`cargo xtask ci --fast` is named in the test table but is not this story's
   bar.** It is the project's integration grain and HS-S0045's to take green. A
   red `--fast` caused by an event-store `todo!()` is not a failure of this
   story, and treating it as one would make this story wait on a slice it is
   explicitly independent of.
6. **Three targeted tests were added beside the borrowed suite**, for behaviours
   the suite cannot see: the corrupt-position row, a cloned store accepting its
   origin's batch, and two concurrent opens. They are adapter-private assertions
   in this crate's own target — **not** additions to the conformance suite, which
   this project may not touch.
7. **No ADR is authored here.** The runtime-seam question is ADR-0022's and is
   consumed; PS-2, PS-12 and PS-18 leave as recorded findings. That follows the
   repository's ADR-queue discipline: a decision record is never a side effect of
   an implementation change.
