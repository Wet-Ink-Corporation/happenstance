# Reviewed diff — the owned-batch port shape

AC-013's real bar is a **reviewed** diff, because a compiler cannot tell a
minimal restatement from an unrelated edit that also compiles
(`_decomposition.md:312-316`, `:579-604`; `RUNBOOK.md:3942-3948`). This note is
that review. It records, per impl, the `Batch` type and the `Error` type before
and after; the disposition ADR-0017 recorded for `LiveHandleProjectionStore` and
the arm executed; for each shape-test the wrong shape it still rejects; and — at
the end, and it is the most important part — the two corpora this change broke
that the plan did not model.

---

## 1. The port, diffed against its authority

`_design.md`'s `## Signatures` block is populated (amendment of 2026-08-13, by
`projection-api-design-record`), so per this story's Integration contract it
**wins over** the spec's restatement of §4.0. It was followed. Where it and
`spec/SPECIFICATION.md:4642-4720` differ at all, they differ only by the design
record carrying derives the fenced block omits; every signature is identical.

| Item | Landed at | Matches |
| --- | --- | --- |
| `Checkpoint` | `crates/happenstance-core/src/projection.rs:133` | `_design.md:109-124`; `spec/SPECIFICATION.md:4645-4658` |
| `Authority` | `projection.rs:165` | `_design.md:126-129`; `:4660-4665` |
| `CommitError<E>` | `projection.rs:180` | `_design.md:131-152`; `:4667-4675` |
| `ResetError<E>` | `projection.rs:221` | `_design.md:154-166`; `:4676-4682` |
| `ProjectionStore` | `projection.rs:255` | `_design.md:144-180`; `:4684-4720` |
| `type Batch;` | `projection.rs:273` | no lifetime, no `where Self: 'a` |
| `fn begin(&self) -> Self::Batch` | `projection.rs:283` | neither `async` nor fallible |
| `async fn reset(…) -> Result<(), ResetError<Self::Error>>` | `projection.rs:334` | new method |

Derives are the design record's, item for item: `Debug, Clone, Copy, PartialEq,
Eq` on `Checkpoint` and `Authority`; `AppendError`'s set including
`thiserror::Error` on both error enums, with `#[error(transparent)]` on `Store`.
All four are `#[non_exhaustive]`.

**`ResetError::Refused` carries no payload.** That is not a coin toss taken here.
EC-002 required a stop-and-report if the question was unanswered; it is answered,
twice — ADR-0018 (`.kb/decisions/0018-…:108-110`, "it stays bare") and
`_design.md`'s *The states the API must express*, which records the cost out loud:
"an operator holding only the error value must go one place further to find out
why."

**One documentation placement resolved rather than duplicated.** This story's
AC-002 asks for the `(None, true)` sentence "once, at `checkpoint`"; `_design.md`
assigns its home to the `Checkpoint` **type**. Both are satisfied without writing
it twice: the sentence lives on `Checkpoint` (`projection.rs:126-132`), and
`checkpoint`'s own doc (`projection.rs:288-296`) says it returns a `Checkpoint`
"never an `Option<SequencePosition>`; the reason the tuple form lost is recorded
on that type". One home, one hop, no repetition — RS-70-5's density rule.

## 2. The mount

`crates/happenstance-core/src/lib.rs:115` — `pub use projection::{…}` gained
`Authority`, `Checkpoint`, `CommitError`, `ResetError`. **Unconditional**: no
`#[cfg(feature = …)]` on any of the four, because they are contract surface.
`crates/happenstance-core/Cargo.toml`'s `[features]` table is **untouched**
(NF-001) — `default`, `std`, `serde`, `memory`, exactly as before.

The mount is proven by compilation rather than inspection: the module-doc
doctest at `projection.rs:55-77` imports all four through the crate root
(`happenstance_core::…`) and doctests compile as an external crate, so a missing
re-export is `error[E0432]`. It runs as
`crates/happenstance-core/src/projection.rs - projection (line 55)`.

## 3. The five impls, before and after

The bar is **same `Batch` type, same `Error` type, same storage strategy, same
bodies**. Every row below holds it. EC-003 did not fire: not one impl needed an
edit beyond restating a signature.

| Impl | `Batch` before | `Batch` after | `Error` before | `Error` after | Bodies |
| --- | --- | --- | --- | --- | --- |
| `PostgresProjectionStore` (`crates/happenstance-postgres/src/projection_store.rs:97`) | `Transaction<'static, Postgres>` bound to `Batch<'a> where Self: 'a` | `Transaction<'static, Postgres>` | `PostgresProjectionStoreError` | unchanged | all `todo!()` before and after |
| `LadybugProjectionStore` (`crates/happenstance-ladybug/src/projection_store.rs:258`) | `GraphWriteSet` bound to the GAT | `GraphWriteSet` | `LadybugProjectionStoreError` | unchanged | all `todo!()` before and after |
| `SqliteProjectionStore` (`crates/happenstance-sqlite/src/projection_store.rs:210`) | `SqliteBatch` bound to `Batch<'a>` | `SqliteBatch` | `SqliteProjectionStoreError` | unchanged | `begin` and `rollback` real before and after; `checkpoint`, `commit` `todo!()`; new `reset` `todo!()` |
| `NeonProjectionStore<T>` (`crates/happenstance-neon/src/projection_store.rs:153`) | `NeonWriteBatch` bound to the GAT | `NeonWriteBatch` | `NeonError<T::Error>` | unchanged | real in every port method before and after; new `reset` real, with its request/decode helpers `todo!()` like `commit`'s |
| `LiveHandleProjectionStore` | `GraphWriteHandle<'a>` — **borrowed** | *n/a — moved, see §4* | `LadybugProjectionStoreError` | *n/a* | real bodies |

**Every changed line inside those four surviving impls is a signature line, an
`Authority`/`Checkpoint`/`CommitError`/`ResetError` import, a `let _ = (…)`
binding widened to name a new parameter, or a comment.** No `Batch` type, no
`Error` type, no storage strategy and no body semantics moved.

Three consequential details, each a *deletion* the port change bought rather
than an edit this story chose:

1. **Neon sheds `+ 'static`.** `impl<T: SqlTransport + 'static>` became
   `impl<T: SqlTransport>` (`projection_store.rs:153`). The bound existed only
   because the GAT's `where Self: 'a` had to be discharged for every `'a`; with
   no `'a` to quantify over it has nothing to discharge. The four `error[E0311]`
   this adapter recorded — two of them pointing into
   `happenstance-core/src/projection.rs` itself — are gone with it. That is the
   port constraint nobody had written down, retired.
2. **SQLite's `E0195` comment is discharged, not deleted.** Its own text said the
   literal `Self::Batch<'_>` "stays mandatory until the GAT leaves the port
   itself". It has left. `type Batch = SqliteBatch;` and
   `commit(&self, batch: Self::Batch, …)` now compile, which is PS-5's claim
   settled by a compiler. The transcript is preserved in the comment above the
   binding rather than dropped.
3. **`begin` became infallible everywhere at no cost to any adapter.** SQLite and
   Neon both already documented the port's `async` + `Result` as a round trip
   they did not need; both now return the batch directly. Postgres and Ladybug
   keep `todo!()`.

Two private helpers were restated because the port's types changed under them,
and both keep `todo!()` bodies: Neon's `decode_checkpoint` (return type
`Option<SequencePosition>` → `Checkpoint`) and `commit_request` (gained
`authority`). Two were added for the port's new method, in the same shape as
`commit`'s: `reset_request` and `decode_reset`.

## 4. `LiveHandleProjectionStore` — the arm ADR-0017 recorded, executed verbatim

**The atom is not silent, so EC-002 did not fire.** ADR-0017's disposition
sentence, quoted in full:

> `LiveHandleProjectionStore` is not deleted — it moves to
> `experiments/live-handle-projection-batch/`, because it is the only compiled
> evidence against this decision's own first claim, and `experiments/` is this
> workspace's home for a reproducible measurement kept outside the gate.

**The arm executed: moved.** `git mv crates/happenstance-ladybug/src/live_handle.rs
experiments/live-handle-projection-batch/live_handle.rs`, so `git log --follow`
reaches its history, plus a `README.md` recording what it refuted, why the
decision stands anyway, and what would bring it back. Nothing was chosen here;
neither the delete arm nor the keep arm was taken.

Consequential edits in `happenstance-ladybug`, all forced by the module leaving
the crate tree and none of them decisions: `pub mod live_handle;` and its
re-export removed from `src/lib.rs`; five intra-doc links to `crate::live_handle`
respelled as plain text naming the experiments path, because a link into a module
the crate no longer has is a **hard** `rustdoc::broken_intra_doc_links` error and
that lint is `deny` at the workspace root.

**Ordering, which AC-009 says is itself the check.** ADR-0017, ADR-0018 and
ADR-0019 are `status: accepted` atoms under `.kb/decisions/`, ingested by the
`2026-08-13-projection-adrs` wave at `493a194` — that is, in a commit **preceding**
this one — and `redkiln validate --kb` was run green before the first edit to
`projection.rs`. `git diff --diff-filter=D` over `.kb/open-questions/` is empty:
nothing was resolved by deletion.

## 5. The three shape-tests, and the wrong shape each still rejects

AC-010's bar is that none of them survives as a test that compiles and asserts
nothing. Each was **falsified by experiment** — the wrong shape was written into
the tree, the compiler's rejection recorded, and the tree restored.

| Test | Restated to | Wrong shape it still rejects | Falsification transcript |
| --- | --- | --- | --- |
| `crates/happenstance-postgres/src/projection_store.rs::the_batch_is_owned` (with `the_batch_does_not_borrow_the_store`) | `assert_static::<<PostgresProjectionStore as ProjectionStore>::Batch>()`, plus the identity function returning `Transaction<'static, Postgres>` | (a) a batch rebound to any type other than this adapter's own; (b) a batch that borrows, which `type Batch;` still admits from a store carrying its own lifetime | rebound `type Batch = sqlx::PgConnection;` → `error[E0308]: expected Transaction<'static, Postgres>, found PgConnection` at `projection_store.rs:183` |
| `crates/happenstance-ladybug/tests/port_shape.rs::send_flavour::spawn_a_batch_across_an_await` | `S::Batch: Send` in place of `for<'a> S::Batch<'a>: Send` | a runner bound-set that omits the batch's `Send` obligation, which `SendProjectionStore` does not imply | deleted the bound → `error: future cannot be sent between threads safely` at `port_shape.rs:97` |
| `crates/happenstance-sqlite/tests/shapes.rs::send_impl_satisfies_the_bare_bound` (`batch_normalises_to_the_owned_type`) | `P: ProjectionStore<Batch = SqliteBatch>`, and the `+ 'static` the GAT forced is deleted | this adapter's `Batch` rebound away from `SqliteBatch` — including "improving" it to a `rusqlite::Transaction`, the shape its own module doc rejects twice | rebound to `Vec<PendingStatement>` → `error[E0271]: type mismatch resolving <SqliteProjectionStore as ProjectionStore>::Batch == SqliteBatch` at `shapes.rs:188` |

**None was removed**, so EC-007 did not fire. The Postgres test's recorded
history — its first spelling certified the shape it existed to reject, because
`'_` in a turbofish is *inferred* while `'_` in argument position elides to a
fresh universally-quantified lifetime — is preserved in its doc comment, together
with the note that the hazard cannot recur now that there is no lifetime to
elide.

**The measurement AC-006 asks for, quoted before and after:**

```text
before:  for<'a> S::Batch<'a>: Send
after:   S::Batch: Send
```

and the reason the before is not something a caller writes unaided is preserved
in the test's own doc: rustc's printed suggestion was
`<S as SendProjectionStore>::Batch<'_>: Send`, which does not compile as printed
because a `where` clause is not an elision context — ``error[E0637]``.

## 6. What this story did **not** touch

- `ProjectionId::new` is still infallible; `.kb/open-questions/projection-id-is-unvalidated.md` still owns it.
- `projection.rs`'s `# Status: provisional` header block survives at `:1-11` (NF-007).
- No `[FROZEN]` clause is line-edited; `spec/SPECIFICATION.md` is not edited at all.
- `crates/happenstance-core/Cargo.toml` `[features]` is byte-identical (NF-001).
- No `ProjectionProbe`, no `MemoryProjectionStore`, no conformance rule, no fixture, no mutant.
- No ADR was written.

---

## 7. FINDING — the blast radius the plan did not model

**This is the headline of the review and it is a re-plan, reported at the story
boundary rather than absorbed** (`_design.md`, closing; `_decomposition.md`
Architecture Note 10).

This spec analysed `spec-trace` and concluded (Behavior table, last row) that
"check 7 stays green and stale citations pass silently", making re-pointing a
deferred tidy-up for `unstable-projection-gate-and-clause-disposition`
(HS-S0016). **That premise is false.** Two corpora cite this story's code by
`file:line` *with text anchors*, and both are now red:

### 7a. `spec/SPECIFICATION.md` — 7 problems, `cargo xtask spec-trace`

Five are the file ADR-0017 required to move; two are anchors that moved because
`projection.rs` grew from 139 to 548 lines.

```text
spec/SPECIFICATION.md:372  — crates/happenstance-ladybug/src/live_handle.rs:187-223 — file does not exist
spec/SPECIFICATION.md:4592 — crates/happenstance-ladybug/src/live_handle.rs:174     — file does not exist
spec/SPECIFICATION.md:4605 — crates/happenstance-ladybug/src/live_handle.rs:68-83   — file does not exist
spec/SPECIFICATION.md:4612 — crates/happenstance-ladybug/src/live_handle.rs:38-66   — file does not exist
spec/SPECIFICATION.md:8095 — crates/happenstance-ladybug/src/live_handle.rs:174-223 — file does not exist
spec/SPECIFICATION.md:5967 — projection.rs:104-105 is evidence for `from`, now out of anchor range
spec/SPECIFICATION.md:8409 — projection.rs:126-138 is evidence for `rollback`, now out of anchor range
```

The five file-move citations re-point to
`experiments/live-handle-projection-batch/live_handle.rs` at the same line
numbers — the file moved verbatim, so every line is where it was. The two anchor
citations need new lines in `projection.rs`: `ReadOptions::from` is now at `:293`
and `rollback` at `:352`.

**Not fixed here, deliberately.** `spec/SPECIFICATION.md` is excluded from this
story's PR boundary and EC-004's required response is explicit: *"Stop and
report — do not edit `spec/SPECIFICATION.md` inside this story. Re-pointing
citations is `unstable-projection-gate-and-clause-disposition`'s work."*

**But the deferral no longer works, and that is the re-plan.** `cargo xtask
affected --base main` runs `spec-trace` **first and unconditionally**
(`xtask/src/affected.rs:117-125`), before it compiles anything, and
`.redkiln/config.yaml`'s `affected_gate` is what `redkiln verify --grain story`
runs. So **no story in this project can pass its own gate until these seven are
re-pointed** — including HS-S0016, the story that owns them. The work cannot
stay where the plan put it. It needs pulling forward, or explicitly authorising
inside this slice.

### 7b. `standards/rust/` — 16 citation problems and 3 failing compiled examples

The Rust constitution cites this code with line+text anchors and **compiles three
examples that implement `ProjectionStore` with the GAT**. `cargo xtask
lint-constitution` reports 16 problems; `cargo test -p xtask --doc` fails 3 of
230 — and `xtask` is one of this project's affected packages, so this is inside
the story gate too.

Three of those are not citation drift but **rules whose subject this story
deleted**, and each names its own settlement condition:

- **RS-22-4** is marked `[PROVISIONAL — settles at SPECIFICATION PS-5, which
  retires the GAT and with it this trap]`. PS-5 has now landed. The rule —
  *"Write the literal `Self::Batch<'_>` in every impl, even when the batch is
  owned"* — is now false, and its `Do` example no longer compiles.
- **RS-21-1** — *"Write the associated-type `Send` bounds by hand, higher-ranked
  over the GAT"* — is half true: the bounds are still not inherited, but they are
  no longer higher-ranked, and its `Not` example demonstrates an `E0637` that can
  no longer be reached.
- **RS-92-1**'s *"What would open it up"* says PS-5 landing "deletes ingredient
  two and the whole exposure with it". It has.

**Not fixed here, deliberately, and this is a judgement worth arguing with.**
Restating an adapter skeleton is mechanical. **Retiring a house-style rule is
not** — it is an editorial decision of ADR weight, the corpus has no retirement
convention to follow, inventing one inside an unrelated story is exactly the
side-effect authorship this project's sequencing exists to prevent, and
`standards/rust/**` is not in this story's PR boundary. The three rules should be
disposed of the way this repository disposes of things: deliberately, in a change
whose subject they are.

The full inventory, so whoever takes it does not have to rediscover it:

```text
20-two-flavour-ports.md:147,197        port_shape.rs anchors moved
21-send-is-not-inherited.md:63,64      port_shape.rs anchors gone (the E0637 text no longer exists)
22-rpitit-and-lifetime-capture.md:308  sqlite projection_store.rs anchor moved
22-rpitit-and-lifetime-capture.md:309  live_handle.rs — file moved
30-error-taxonomy.md:353               projection.rs:90 `type Error` — now at :257
61-compile-time-assertions.md:164,165  postgres projection_store.rs anchors moved
61-compile-time-assertions.md:328,329  port_shape.rs anchors moved
90-skeletons-and-todo.md:83            sqlite projection_store.rs anchor moved
90-skeletons-and-todo.md:174,175       live_handle.rs — file moved
92-toolchain-limits-and-dead-ends.md:109,110  live_handle.rs — file moved

failing doctests (cargo test -p xtask --doc):
  21-send-is-not-inherited.md line 21     — for<'a> S::Batch<'a>: Send
  22-rpitit-and-lifetime-capture.md 206   — type Batch<'a>, async fn begin -> Result
  92-toolchain-limits-and-dead-ends.md 29 — type Batch<'a> = WriteSet where Self: 'a
```

### 7c. What *is* green

Everything the story's own acceptance criteria are measured by:

- `cargo fmt --all --check`
- `cargo clippy --locked -p happenstance-core -p happenstance-testkit -p happenstance-postgres -p happenstance-ladybug -p happenstance-sqlite -p happenstance-neon -p xtask --all-targets --all-features -- -D warnings`
- `cargo test --locked` over the six code packages — 0 failures
- `cargo xtask wasm` — all four steps, including the mandatory `happenstance-core` build (AC-005) and `happenstance-neon` (AC-008)
- `cargo test -p happenstance-core --doc` — including the new module-doc example (AC-007)
- the five new unit tests in `projection.rs` (AC-001 – AC-005)

`projection.rs` is 548 lines, well past the 138 EC-004 watches for, so nothing
was deleted that should not have been.
