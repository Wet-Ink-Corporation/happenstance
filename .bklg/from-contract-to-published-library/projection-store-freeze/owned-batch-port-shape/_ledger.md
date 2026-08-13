---
item: HS-S0004
stage: implement
created: 2026-08-12T13:45:58.717Z
updated: 2026-08-12T13:45:58.717Z
---

# Acceptance ledger — The owned-batch port shape, mounted and with the skeletons restated

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author opening crates/happenstance-core/src/projection.rs to write a store against a real transaction API, WHEN they read the trait, THEN it presents §4.0's shape item for item — `type Batch;` with no lifetime and no `where Self: 'a`, `fn begin(&self) -> Self::Batch`, `async fn checkpoint(&self, &ProjectionId) -> Result<Checkpoint, Self::Error>`, `async fn commit(&self, Self::Batch, &ProjectionId, SequencePosition, Authority) -> Result<(), CommitError<Self::Error>>`, `async fn reset(&self, Self::Batch, &ProjectionId) -> Result<(), ResetError<Self::Error>>`, `async fn rollback(&self, Self::Batch) -> Result<(), Self::Error>` — so the thing they bind is an owned value they already have, and the generic associated type nothing in the workspace could satisfy is gone"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs — the export block at :98-124 (line 115)"
  verifying_test: "crates/happenstance-core/src/projection.rs::tests::the_port_is_implementable_with_an_owned_batch (new #[cfg(test)] module), plus cargo check -p happenstance-core under cargo xtask affected --base main"

- id: AC-002
  criterion: "GIVEN a rule author asking a store \"how far has this projection got, and is what I am reading authoritative?\", WHEN they call `checkpoint`, THEN they receive a `Checkpoint` whose three variants (`NeverRun`, `Live { through }`, `Rebuilding { through }`) name every state the port can report and make `(None, true)` — \"authoritative, never run, which means nothing\" — unspellable; and the rustdoc they are reading at that moment says why the tuple lost, once, at `checkpoint`"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs — the export block at :98-124 (line 115 gains `Checkpoint`)"
  verifying_test: "crates/happenstance-core/src/projection.rs::tests::checkpoint_names_every_state_and_no_others, plus cargo clippy --workspace --all-targets --all-features -- -D warnings (missing_docs, root Cargo.toml:103)"

- id: AC-003
  criterion: "GIVEN an adapter author handling a failed `commit`, WHEN they `match` on the returned error, THEN they are offered only arms `commit` can actually produce — `ForeignBatch`, `CheckpointRegression { current, attempted }`, `Store(E)` — and never `Refused`, which belongs to `reset` alone; each enum is `#[non_exhaustive]` and carries the adapter's error as a type parameter, so nothing is stringified and no dead arm is forced"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs — the export block at :98-124 (line 115 gains `CommitError`, `ResetError`)"
  verifying_test: "crates/happenstance-core/src/projection.rs::tests::commit_and_reset_errors_stay_separate, plus the --no-default-features doc build of happenstance-core inside cargo xtask ci"

- id: AC-004
  criterion: "GIVEN an adapter on a one-shot HTTP transport with no connection and no cursor (Neon), WHEN it opens a batch, THEN `begin` costs it no round trip and no error path — it is neither `async` nor fallible — and the rustdoc at `begin` names the alternative that lost so a later reader does not \"restore symmetry\" with `EventStore`; the same obligation holds at `reset`, whose rustdoc says the port takes the caller's own deletes because \"the port has no idea what the read model is\""
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/projection.rs — the `ProjectionStore` trait at :87, re-exported from crates/happenstance-core/src/lib.rs:115"
  verifying_test: "crates/happenstance-core/src/projection.rs::tests::begin_is_neither_async_nor_fallible, plus cargo clippy --workspace --all-targets --all-features -- -D warnings"

- id: AC-005
  criterion: "GIVEN the edge developer's target — wasm32-unknown-unknown, where nothing is `Send` — WHEN the contract crate is built for it, THEN it builds, because `type Batch: Send;` is not written on the trait: trait_variant copies associated-type bounds verbatim into the !Send flavour, so the Send flavour's transitive requirement is documented at `Batch` rather than declared, and `#[trait_variant::make(SendProjectionStore: Send)]` still derives both flavours from one definition"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/projection.rs:87 — the `#[trait_variant::make(SendProjectionStore: Send)]` derivation; both flavours re-exported from lib.rs:115"
  verifying_test: "The mandatory wasm32 build of happenstance-core inside cargo xtask ci (xtask/src/main.rs:203-221), plus crates/happenstance-core/src/projection.rs::tests::a_non_send_batch_still_implements_the_bare_flavour"

- id: AC-006
  criterion: "GIVEN a caller spawning a projection runner on a multi-threaded executor, WHEN they write the bound their generic code needs, THEN it is `S::Batch: Send` — a bound \"a caller might write without help\" — rather than today's `for<'a> S::Batch<'a>: Send`, whose higher-ranked form rustc suggests in text that does not compile as printed (error[E0637]); and the test carrying that bound still fails when the shape regresses"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:115 — `ProjectionStore` as the bound generic code binds; exercised from crates/happenstance-ladybug/tests/port_shape.rs"
  verifying_test: "crates/happenstance-ladybug/tests/port_shape.rs:33-78 — the spawned-runner bound restated to `S::Batch: Send` with its `const _` instantiation"

- id: AC-007
  criterion: "GIVEN an adapter author in their own crate trying to name the types the port hands them, WHEN they write `use happenstance_core::{Authority, Checkpoint, CommitError, ResetError};`, THEN it resolves — the four types are mounted in the contract crate's export block beside `ProjectionId, ProjectionStore, SendProjectionStore`, unconditionally, with no `#[cfg(feature = …)]` on any of them, because an item that compiles and is not re-exported there is an item no adapter can name"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:115 — `pub use projection::{...}` gains `Authority`, `Checkpoint`, `CommitError`, `ResetError`; no new feature gate"
  verifying_test: "cargo test -p happenstance-core --doc — the module-doc example on crates/happenstance-core/src/projection.rs importing all four through the crate root"

- id: AC-008
  criterion: "GIVEN four downstream adapter crates whose authors already chose a batch representation, an error type and a storage strategy, WHEN the port's shape changes underneath them, THEN all five `ProjectionStore` impls compile again having changed nothing but method signatures — Postgres keeps `Transaction<'static, Postgres>`, Ladybug keeps `GraphWriteSet`, SQLite keeps `SqliteBatch`, Neon keeps `NeonWriteBatch`, every body stays `todo!()`, and Neon's `impl<T: SqlTransport + 'static>` sheds the `+ 'static` the GAT forced on it — and any edit that goes beyond restating a signature is reported as a defect in the port change, not absorbed"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:115 (the changed contract surface) consumed at crates/happenstance-postgres/src/projection_store.rs:94, crates/happenstance-ladybug/src/projection_store.rs:258, crates/happenstance-sqlite/src/projection_store.rs:208, crates/happenstance-neon/src/projection_store.rs:153"
  verifying_test: "cargo xtask affected --base main plus the wasm32 build of happenstance-neon (xtask/src/main.rs:203-221), and the reviewed diff .bklg/from-contract-to-published-library/projection-store-freeze/owned-batch-port-shape/_reviewed-diff.md recording Batch/Error type before and after per impl"

- id: AC-009
  criterion: "GIVEN `LiveHandleProjectionStore` — the only impl in the workspace that binds a genuinely borrowed batch with real bodies, and the compiled counter-example to the argument this story executes — WHEN the port stops admitting it, THEN its fate is the one ADR-0017 recorded by name (deleted, moved to experiments/, or kept with its transcripts and a note that the port no longer admits it), executed verbatim and never chosen here; and ADR-0017, ADR-0018 and ADR-0019 exist as accepted atoms with `redkiln validate --kb` green before the first edit to projection.rs, because that ordering is itself the check"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/live_handle.rs:174-219 and crates/happenstance-ladybug/src/lib.rs — the module's own export path; .kb/decisions/ for the three atoms"
  verifying_test: "redkiln validate --kb green in the commit preceding the port-shape commit, `git diff --diff-filter=D` over .kb/open-questions/ empty, and the executed-arm record in .bklg/from-contract-to-published-library/projection-store-freeze/owned-batch-port-shape/_reviewed-diff.md"

- id: AC-010
  criterion: "GIVEN three tests whose entire subject is the GAT — and one of which, by its own recorded history, was written first in a spelling that certified the shape it existed to reject — WHEN the GAT is removed, THEN none of them survives as a test that compiles and asserts nothing: each is restated into a form still capable of failing against a wrong shape, or removed with the reason stated in the diff note, and the removal case is argued rather than assumed"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:115 — the contract surface all three tests bind; asserted from the three adapter crates' own test paths"
  verifying_test: "crates/happenstance-postgres/src/projection_store.rs:143-178 (the_batch_does_not_borrow_the_store), crates/happenstance-ladybug/tests/port_shape.rs:33-78, crates/happenstance-sqlite/tests/shapes.rs:104-170 (batch_normalises_to_the_owned_type)"
```
