# The apply-shape spike

The typed layer's `Projection::apply` is synchronous
(`crates/happenstance/src/runner.rs:95-99`), so a projection can push a row into a
buffer and cannot `.await` a statement into a live transaction.
`LivePostgresProjectionStore` exists and is published, and its own module
documentation says `run_projection` cannot drive it for a projection that writes
rows (`crates/happenstance-postgres/src/live_projection_store.rs:38-47`). Phase 18's
exit criterion asks for exactly that write, *"demonstrated against a real database
rather than a buffer"*. This spike is the evidence phase 17's apply record (lane
L3) needs, and it answers the question with compiled and executed code.

**The short answer: yes.** The proposed trait compiles under `trait_variant` 0.1.3,
including its provided `on_error`. It drives all three stores: a memory buffer, a
SQLite statement buffer, and a live `sqlx` transaction that `apply` awaits
statements through, against a real PostgreSQL 17.10. The runner spawns onto a
multi-thread runtime from code generic over the projection. The same trait and
runner type-check for `wasm32` with a `!Send` store and a `!Send` projection. The
envelope has no `position()` (`E0599`).

Five findings qualify that answer, and the record should carry all of them
(§4.2). The one with the widest reach: **a generic spawner must name the store as
its own type parameter** (`P: SendProjection<Store = St>`), or every store future
is `!Send`. The cause is the associated type's item bound,
`type Store: ProjectionStore`, and not `trait_variant`: the **shipped**
`happenstance::Projection`, a plain trait, fails the same way when
`run_projection` is spawned generically, and compiles with `Store = St`
(`tests/shipped_spawn.rs`, `results/spawn-shipped-store-as-projection.txt`).

This directory is **not** a workspace member. It is not a `cargo xtask ci` step
and adds no dependency to any `crates/**` manifest. `crates/happenstance` is not
edited: the proposed trait is declared in [`src/shape.rs`](src/shape.rs), inside
this crate.

---

## 1. The question

Can `Projection::apply` become

```rust
async fn apply(&mut self, event: Delivered<Self::Event>, batch: &mut StoreBatch<Self>)
    -> Result<(), Self::Error>;
```

when it is declared through `#[trait_variant::make(SendProjection: Send)]`, with:

- a `#[non_exhaustive]` `Delivered<E>` that exposes `id() -> EventId`, `event()` and
  `into_event()`, and has **no** accessor for the local `SequencedEvent::position`
  (SY-21);
- `type Error` (PS-28);
- a **provided** `on_error` that defaults to `Halt`, written as
  `-> impl Future` with a block because `trait_variant` 0.1.3 copies the block
  verbatim (the PS-27 seam, and PS-9/PS-11's evidence)?

Each part must hold over a buffered batch and a live one, on a native
multi-thread runtime and on `wasm32`.

## 2. The criteria, stated before the run

These are the bar the task and the projection-apply brief set. Each one names what
would count as failure.

| # | Criterion | Fails if |
|---|---|---|
| C1 | The trait, with `async fn apply` and a provided `on_error` in `-> impl Future` form, compiles under `trait_variant` 0.1.3 | any error in the macro expansion or in the `Send` copy of the default body |
| C2 | A memory-store projection runs through a runner bound on the bare `Projection` | wrong rows or checkpoint |
| C3 | A `SqliteBatch` projection runs, with `apply` pushing statements | wrong rows or checkpoint in the file, read back through a fresh connection |
| C4 | A `LivePostgresBatch` projection runs, with `apply` **awaiting** `execute` through the live transaction | does not compile, or wrong rows against a real server |
| C5 | The runner future spawns onto multi-thread tokio from code generic over `P: SendProjection` | no set of caller-side bounds makes it `Send` |
| C6 | The trait and the runner type-check for `wasm32-unknown-unknown` with a `!Send` store and a `!Send` projection | a `Send` bound leaks into the bare flavour |
| C7 | `Delivered` has no `position()`: `E0599` | the call compiles |
| C8 | `on_error` can write a skip record into the batch through the store's **inherent** API, with no bound on `Batch` in library code (PS-9, PS-11) | a library-owned write vocabulary or a `Batch` bound is needed |
| C9 | A buffered `apply` is ready at its first poll, with no runtime (PS-6 carried up) | `Pending` on the first poll |
| C10 | `ProjectionStore` does not move (ADR-0063's falsifier) | any port signature has to change |

Two more observations were in scope from the brief, and neither was a pass/fail
criterion: whether return-type notation is usable on 1.97.1 (§4.2, F6), and what
`Skip` means over a live batch after the **server** has refused a statement (F5).

## 3. What is here, and how to run it

| Path | What it is |
|---|---|
| [`src/shape.rs`](src/shape.rs) | The proposed trait: `Projection`/`SendProjection`, `Delivered`, `ApplyFailure`, `Policy`. Carries the `E0599` doctest pair. |
| [`src/runner.rs`](src/runner.rs) | A minimal runner: the shipped loop with `apply` awaited, `Delivered` built from `SequencedEvent::id`, and failures offered to `on_error`. `RunError<R, W, A>` has three type parameters. |
| [`src/edge.rs`](src/edge.rs) | The `!Send` end: an `Rc` store and an `Rc` projection. It sits in the library so the wasm32 check sees it. Carries the `E0277` doctest pair. |
| [`src/domain.rs`](src/domain.rs) | The one domain every case folds. It has a key `apply` refuses and a payload that will not decode. |
| [`tests/memory.rs`](tests/memory.rs) | C2, C7 (behaviourally), C8, C9 (the runner polled once) |
| [`tests/sqlite.rs`](tests/sqlite.rs) | C3, C5, C8 |
| [`tests/postgres.rs`](tests/postgres.rs) | C4, C5, C8, the server-side-failure limit, and the server's `SELECT version()`. `#[ignore]`d; needs `APPLY_SHAPE_DATABASE_URL`. |
| [`tests/spawn.rs`](tests/spawn.rs), [`tests/common/mod.rs`](tests/common/mod.rs) | C5: the generic spawner and its bounds, and the two weakened spawners behind `demonstrate-spawn-*` features |
| [`tests/shipped_spawn.rs`](tests/shipped_spawn.rs) | F1 on the **shipped** `happenstance::Projection`: `run_projection` spawned generically with `Store = St`, and, behind `demonstrate-shipped-f1`, with `P::Store` |
| [`tests/edge.rs`](tests/edge.rs) | C6, run on the host |
| [`src/demonstrate.rs`](src/demonstrate.rs) | Built only under `--features demonstrate-refusals`, and expected to fail. It produces the real `E0599`/`E0277` transcripts. |
| [`probes/rtn.rs`](probes/rtn.rs) | The return-type-notation probe |
| [`run.sh`](run.sh), [`scrub.py`](scrub.py) | The reproducer, which writes `results/*.txt` with the home directory scrubbed to `~` and the repository root to `<repo>` |

```console
bash experiments/apply-shape/run.sh

# with the Postgres case (it is otherwise reported as ignored):
docker run -d --rm --name apply-shape-pg -p 55432:5432 \
    -e POSTGRES_PASSWORD=postgres postgres:17.10
APPLY_SHAPE_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/postgres \
    bash experiments/apply-shape/run.sh
docker stop apply-shape-pg
```

Each step, if you want to run it alone (`CARGO_BUILD_JOBS=4` throughout, from
this directory):

```console
cargo test                                                       # C2 C3 C5 C6 C7 C8 C9, F1 (shipped), doctests
APPLY_SHAPE_DATABASE_URL=… cargo test --test postgres -- --ignored --test-threads=1 --nocapture   # C4, and the server version
cargo check --lib --target wasm32-unknown-unknown --no-default-features               # C6
cargo clippy --all-targets -- -W clippy::pedantic
cargo check --lib --features demonstrate-refusals                # expected to FAIL: E0599, E0277
rustc --edition 2024 --crate-type lib --out-dir target/rtn-probe probes/rtn.rs       # expected to FAIL: E0658
cargo test --features demonstrate-spawn-minimal-bounds --test spawn --no-run         # expected to FAIL: 9 E0277s
cargo test --features demonstrate-spawn-store-as-projection --test spawn --no-run    # expected to FAIL: 5 E0277s (F1)
cargo test --features demonstrate-shipped-f1 --test shipped_spawn --no-run           # expected to FAIL: 6 E0277s (F1, shipped)
```

`native` is the default feature and carries the two database adapters. The wasm32
check turns it off, so it builds the trait, the runner and the edge store, and
nothing that links C or opens a socket.

Three transcripts come from **deliberately weakened** spawners, each behind its
own feature so `run.sh` reproduces them against the source as it stands:

- `results/spawn-minimal-bounds.txt` (`demonstrate-spawn-minimal-bounds`): the
  spawner with only `S: SendEventStore + Sync + 'static`,
  `P: SendProjection + Send + 'static` and
  `<P as SendProjection>::Store: SendProjectionStore + Sync + 'static`.
- `results/spawn-store-as-projection.txt` (`demonstrate-spawn-store-as-projection`):
  every bound in §4.1, but with the store written as `<P as SendProjection>::Store`
  rather than a type parameter `St`.
- `results/spawn-shipped-store-as-projection.txt` (`demonstrate-shipped-f1`): the
  shipped `run_projection`, spawned with the store written as `P::Store`.

The first two were hand edits of `tests/common/mod.rs` in this directory's first
version, and their transcripts predated a reformat of `src/runner.rs`. They are now
features, and every transcript in `results/` was re-run against the final source.

## 4. Results

First measured at `52aa951` with `crates/` clean. The transcripts in `results/`
are a re-run on `52aa951` plus lane L2's uncommitted diff, whose changes under
`crates/` are comments and `spec/SPECIFICATION.md:N` citations inside string
literals only (`results/environment.txt` lists the files). On `rustc 1.97.1
(8bab26f4f 2026-07-14)`, `x86_64-pc-windows-msvc`. Postgres was `postgres:17.10`
in a container started by hand, and the server's own `SELECT version()`,
`PostgreSQL 17.10 (Debian 17.10-1.pgdg13+1)`, is in `results/test-postgres.txt`.
`results/environment.txt` has the rest of the record.

When this work began the Docker daemon was **not** running (`docker info`: *"failed
to connect to the docker API at npipe:////./pipe/dockerDesktopLinuxEngine"*), and
the Postgres case was built to compile and be reported as skipped. The daemon
became available later in the session, so the case was **run**. It is not
skipped, and `results/test-postgres.txt` is the transcript.

### 4.1 Against the criteria

| # | Result | Evidence |
|---|---|---|
| C1 | **Held.** First compile, no expansion errors. | `src/shape.rs:174-232`; `results/clippy.txt` |
| C2 | **Held.** | `tests/memory.rs:169`, `:212` |
| C3 | **Held.** 3 applied, 2 skipped. Rows and skip records are read back from the file, and the checkpoint is at the head. | `tests/sqlite.rs:130`; `results/test-default.txt` |
| C4 | **Held, against a live server.** `apply` awaits `batch.execute(..).await?` inside the live transaction. 3 applied, 2 skipped, rows and skip records committed, and the checkpoint read back through a fresh store is `Live` at the head. | `tests/postgres.rs:98-109`, `:175`, `:232`; `results/test-postgres.txt` |
| C5 | **Held, with the caller-side bounds below.** The one spawner drives all three stores. | `tests/common/mod.rs:22-25`; `tests/spawn.rs:114` |
| C6 | **Held.** | `results/wasm32-check.txt`; `src/edge.rs:257`; `tests/edge.rs:15` |
| C7 | **Held.** `error[E0599]: no method named position found for reference &Delivered<u8>`. | `results/refusals.txt`; `src/shape.rs:63` |
| C8 | **Held on all three batch shapes.** `MemoryProjectionBatch::write`, `SqliteBatch::push`, `LivePostgresBatch::execute`. The runner names `Batch` only as `StoreBatch<P>`, with no bound. | `tests/memory.rs`; `tests/sqlite.rs:83`; `tests/postgres.rs:113-130`; `src/runner.rs` |
| C9 | **Held.** The runner, `run` over the two memory stores, returns `Poll::Ready(Ok(..))` on the first poll of a `Waker::noop` context. Polling `apply` alone could not fail, because an `apply` with no await is ready by async-fn semantics; the runner can, and a yield inserted before the `apply` await turned it red. | `tests/memory.rs:151` |
| C10 | **Held.** No port signature was touched. `LivePostgresBatch::execute` is the published inherent `async fn`, used as it is. | `crates/happenstance-postgres/src/live_projection_store.rs:176` |

**The bounds a generic spawner writes** (`tests/common/mod.rs:22-25`). Each one
was demanded by the compiler, and each was removed again to confirm it is
load-bearing. The two `Sync`s count:

```rust
S:  SendEventStore<Error: Send> + Sync + 'static,
P:  SendProjection<Store = St, Event: Send, Error: Send + Sync> + Send + 'static,
St: SendProjectionStore<Batch: Send> + Sync + 'static,
```

| Bound | Why it is there |
|---|---|
| `Store = St`, with `St` a type parameter | **F1** below. Without it, all four store futures are `!Send`. |
| `St: SendProjectionStore<Batch: Send>` | The batch is held across every `apply` await. ADR-0017 keeps `Send` off `type Batch`, so the caller supplies it, as with the port. |
| `S: SendEventStore<Error: Send>` | The runner holds a read error across the rollback await. This is ADR-0009's obligation, unchanged from `flavours.rs`. |
| `P: SendProjection<Error: Send + Sync>` | `Send` for the same reason. `Sync` because `ApplyFailure` holds `&'a A` across the `on_error` await, and `&A: Send` needs `A: Sync`. |
| `S: Sync` | The runner holds `&S` across its awaits, and `&S: Send` needs `S: Sync`. Removing it fails with *future cannot be sent between threads safely*. |
| `St: Sync` | The same, for `&St`. Removing it fails the same way. |
| `P: SendProjection<Event: Send>` | The `match` on `decode(..)` keeps the scrutinee, a `Result<P::Event, CodecError>`, alive across the `on_error` await in its `Err` arm. One restructuring was tried, a two-step `match`, and it did not shed the bound. A real `SendProjection` needs a `Send` event anyway. |

### 4.2 Findings the record should carry

**F1. A generic spawner must name the store as a type parameter.** The bound
`<P as SendProjection>::Store: SendProjectionStore` does not make the store's
futures `Send`. `results/spawn-store-as-projection.txt` shows five `E0277`s:
`checkpoint`, `begin`, `rollback` and `commit`'s futures, plus `Batch` even with
`Batch: Send` written on the `SendProjectionStore` side. Here is the mechanism in
Rust terms. The associated type is declared `type Store: ProjectionStore`, so
inside generic code `P::Store` is known to implement the **bare**
`ProjectionStore` through that item bound. The trait solver prefers an item
(where-clause) bound over an impl, so the store's `ProjectionStore` methods are
resolved through the bound, whose futures are opaque, rather than through the
blanket `impl<T: SendProjectionStore> ProjectionStore for T`, whose futures are
known to be `Send`. Writing `P: SendProjection<Store = St>` with `St` a fresh type
parameter removes the item bound from the path, and the blanket impl is used.
**`trait_variant` is not the cause.** It copies the item bound into
`SendProjection`, but a trait it never touches has the same bound and the same
failure.

The **shipped** trait is such a trait: `happenstance::Projection` is a plain trait
with `type Store: ProjectionStore` (`crates/happenstance/src/runner.rs:62-67`).
This is compiled, not inferred. `tests/shipped_spawn.rs` spawns the shipped
`run_projection` from code generic over `P` with `Store = St`, and it runs. The
same spawner with the store written as `P::Store` fails with six `E0277`s: the
`checkpoint`, `begin`, `rollback` and `commit` futures, plus `Batch` and `Error`
(`results/spawn-shipped-store-as-projection.txt`).
`crates/happenstance/tests/flavours.rs` never hit it because its projection is
concrete. Phase 18's rewrite of the runner's
"what spawning it costs" section (`runner.rs`, the `run_projection` rustdoc) must
say this, or the first generic spawner written from the docs will not compile.
The alternative that would remove it, `type Store: SendProjectionStore` on the
`Send` flavour only, cannot be written: `trait_variant` copies bounds verbatim
(ES-5, RS-20-1).

**F2. `trait_variant`'s blanket impl forwards parameters by name, and pedantic
clippy rejects an `_`-prefixed one.** Declaring the default as
`fn on_error(&mut self, _failure: …, _batch: …)` fails
`clippy::used_underscore_binding`, because the generated impl *uses* `_failure`.
The workspace sets `pedantic = warn` and `.cargo/config.toml` makes warnings
errors, so the same declaration in `crates/happenstance` would fail the gate. The
fix is at `src/shape.rs:223-231`: named parameters, discarded with `let _ = x;`
in the body.

**F3. The `-> impl Future` spelling costs the library one method and costs
applications nothing.** The trait must spell the provided default as
`-> impl Future<Output = …> { async { … } }`. An implementer overriding it may
write a plain `async fn on_error(..)`, because an `async fn` in an impl refines an
RPITIT declaration (`tests/memory.rs`'s `SkippingTally`, `tests/sqlite.rs`,
`tests/postgres.rs`). The edge case shows the `-> impl Future` form in an impl
compiles too (`src/edge.rs`).

**F4. SY-21's wording has to say "local".** `delivered.position()` is `E0599`, but
`delivered.id().position()` compiles and returns a `SequencePosition`: the
**origin's**, which SY-23 relies on. `tests/memory.rs:243` runs a restored log
whose local positions are 1, 2, 3 and whose origin positions are 30, 10, 20.
`apply` sees the origin identities in local order, and no identity is re-minted.
The MUST NOT is true of the local position and false of "a `SequencePosition`",
which is what the brief predicted.

**F5. `Skip` over a live batch is only as good as the transaction.** An
**application** refusal raised before any statement leaves the transaction
healthy, and `on_error`'s skip write commits with the checkpoint
(`apply_awaits_statements_through_a_live_transaction`). A **server** refusal
aborts the transaction, so the skip write is refused too, and the run stops with
`RunError::Policy` carrying the adapter's error. The chunk is rolled back and no
rows are written (`a_server_side_failure_leaves_nothing_for_on_error_to_write_into`,
which was stated before it was run and passed). A buffered batch has no such
limit. The record should not let `Policy::Skip` read as universal. Skipping a
server-side failure on a live batch needs a `SAVEPOINT` around each `apply`, and
whether the adapter offers that or the runner issues it is open.

**F6. Return-type notation is not available, so the `trait_variant` pair stays.**
`P: Projection<apply(..): Send>` is `error[E0658]: return type notation is
experimental` on 1.97.1 (`results/rtn-probe.txt`). One trait with RTN at the
spawn site is therefore not an option at this MSRV. The brief asked the session
log to record which `Send` mechanism the spike chose: **`trait_variant`**.

**F7. `Delivered::new` should be public.** An application unit-testing its own
`apply` has to be able to build the argument. A runner-only constructor would
force every such test through a store. `Delivered::new(id, event)` takes an
`EventId`, so the local position still has nowhere to go (`src/shape.rs:97`).

**F8. PS-28's `From` bound pays for itself.** `type Error:
From<<Self::Store as ProjectionStore>::Error>` is what lets
`batch.execute(..).await?` work inside `apply` with no `map_err`
(`tests/postgres.rs:98`, `:109`, `:130`). The halting test also shows the runner
returning the **projection's own** error type in `RunError::Apply`
(`tests/memory.rs:169`), which is the three-parameter
`ProjectionError<R, W, A>` in miniature.

## 5. Verdict

**The proposed shape holds.** All ten criteria passed. The provided method under
`trait_variant` 0.1.3 compiled on the first attempt, and it is exercised with a
`Send` implementor (memory, SQLite, live Postgres, all spawned from generic code)
and a `!Send` one (the edge store, run on the host and checked for wasm32). The
brief's fallback, a second `LiveProjection` trait with a second runner, is not
needed.

This is **not** a verdict that the shape is free. F1 puts one non-obvious bound
on every generic spawner, and it applies to the shipped trait too. F5 narrows what
`Policy::Skip` can promise over a live batch. Both belong in the record as
statements rather than being left for phase 18 to rediscover.

## 6. What L3's record may cite

- **Async `apply` over a live transaction works:** `tests/postgres.rs:98-109`
  (the awaited `execute`), `:230` (the committed checkpoint, read back) and
  `results/test-postgres.txt` (3 passed, the third printing the server's
  `SELECT version()`: PostgreSQL 17.10).
- **Provided `on_error` under `trait_variant` 0.1.3:** `src/shape.rs:223-231`
  (declaration), plus a `Send` override (`tests/sqlite.rs`, `tests/postgres.rs:113`)
  and a `!Send` override (`src/edge.rs`).
- **PS-9/PS-11, a skip record through inherent APIs with no `Batch` bound:**
  `tests/memory.rs:212`, `tests/sqlite.rs:83`, `tests/postgres.rs:113-130`.
- **SY-21:** `results/refusals.txt` (`E0599`) and `tests/memory.rs:243` (origin
  identity delivered), plus F4's wording.
- **Generic-`P` spawn and its bounds:** `tests/common/mod.rs:22-25`,
  `results/spawn-minimal-bounds.txt`, `results/spawn-store-as-projection.txt` (F1),
  and on the shipped trait `tests/shipped_spawn.rs` and
  `results/spawn-shipped-store-as-projection.txt`.
- **wasm32 and `!Send`:** `results/wasm32-check.txt`, `src/edge.rs`,
  `results/refusals.txt` (`E0277`, `EdgeTally: SendProjection`).
- **First-poll readiness (PS-6 carried up):** `tests/memory.rs:151`, the runner
  polled once.
- **ADR-0063's falsifier did not fire:** the spike changed no file under `crates/`.
  The re-run's `results/environment.txt` lists files under `crates/` because lane
  L2's other work touched them, in comments and citation strings only; no port
  signature changed.
- **The `Send` mechanism is `trait_variant`, not RTN:** `results/rtn-probe.txt`.
- **Limits:** F2 (lint), F5 (`Skip` after a server-side failure).

Cite by `experiments/apply-shape/<path>:<line>`. The line numbers above are those
of the source the transcripts in `results/` were re-run against; re-check them
against the tree the record cites.

## 7. What this does not settle

- **Codec tag resolution.** The runner calls `DomainEvent::decode` with the codec
  in hand, because the shipped `decode_event` is crate-private. The events here
  carry no framing, so this is not what was asked.
- **Panicking `apply`.** PS-30's `panicking_apply_rolls_back` over an async
  `apply` needs `catch_unwind` per poll, and a hand-written `poll_fn` wrapper
  (the runner avoids `futures-util`). This is not built here.
- **The event side.** All three cases read from `MemoryEventStore`. The question
  is about the projection side, and the event store's `Send` story is ADR-0008's.
- **`cargo semver-checks`.** It was not run: nothing published changed, and the
  typed runner is behind `unstable-projection`.
- **The cost of `ProjectionError<R, W, A>`'s third parameter** on downstream
  signatures is shown here only as `RunErrorFor<S, P>`, an alias. Whether the
  published type ships a similar alias is phase 18's decision.
