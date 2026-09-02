---
item: HS-S0077
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Fill the four bodies, delete the allow, record PS-34's disposition

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
  criterion: "The four bodies do something when called. GIVEN the adapter author has the real `lbug` driver merged and a temporary LadybugDB directory, WHEN they call `begin()`, append one parameterised `GraphStatement`, `commit()` it at a position and then call `checkpoint()`, THEN that position comes back and no method panics — this is the first `ProjectionStore` implementation in the workspace whose methods execute rather than type-check."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs"
  verifying_test: "crates/happenstance-ladybug/tests/round_trip.rs::checkpoint_round_trips_through_a_real_commit"

- id: AC-002
  criterion: "The checkpoint and the read-model write land in one transaction, checkpoint last. GIVEN a batch carrying two statements, WHEN the author commits it, THEN exactly one connection is opened, one `BEGIN TRANSACTION` … `COMMIT` pair wraps the replay, the buffered statements execute in the buffer's order, and the checkpoint statement is the last statement before `COMMIT` — so a reviewer reading `commit` sees PS-1 held rather than inferred."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs"
  verifying_test: "crates/happenstance-ladybug/src/projection_store.rs::tests::commit_writes_the_checkpoint_last_in_one_transaction; crates/happenstance-ladybug/tests/round_trip.rs::a_failed_statement_leaves_the_store_unchanged"

- id: AC-003
  criterion: "An empty batch still advances the checkpoint. GIVEN events that produced no graph mutation, WHEN the runner commits an empty `GraphWriteSet` at their position, THEN `commit` does not short-circuit on `is_empty()` and a subsequent `checkpoint()` returns the new position — so a restart does not replay those events forever."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs"
  verifying_test: "crates/happenstance-ladybug/tests/round_trip.rs::an_empty_write_set_still_advances_the_checkpoint"

- id: AC-004
  criterion: "A corrupt checkpoint is reported, never mistaken for a missing one. GIVEN a checkpoint property holding `0` or a negative value on the read side, or a position above `i64::MAX` on the write side, WHEN the author reads or writes it, THEN they get `MalformedCheckpoint { projection, value }` or `PositionOutOfRange { position }` — never `Ok(None)`, never an `unwrap`, never a variant simplified away — and both directions are proven without a LadybugDB instance."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs"
  verifying_test: "crates/happenstance-ladybug/src/projection_store.rs::tests::zero_and_negative_are_malformed_not_absent; ::positions_above_i64_max_are_out_of_range; ::every_valid_position_round_trips"

- id: AC-005
  criterion: "Single-writer contention is handed back, not hidden. GIVEN a second commit racing an open write transaction, WHEN the driver refuses, THEN the caller receives `LadybugProjectionStoreError::WriteTransactionInUse` rather than an opaque `Driver` string, and the crate contains no retry loop, backoff or sleep — so the freeze verdict inherits the finding instead of a hidden timing behaviour."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs"
  verifying_test: "crates/happenstance-ladybug/tests/round_trip.rs::a_second_concurrent_commit_reports_write_transaction_in_use (declared unit-tier fallback per spec Implementation notes)"

- id: AC-006
  criterion: "Rollback is free, and it is reversible in the only sense that matters. GIVEN a batch with statements appended, WHEN the author calls `rollback()`, THEN the write set is dropped without opening a connection or issuing `ROLLBACK`, and a subsequent `checkpoint()` returns exactly what it returned before `begin()` — the store's observable state is where the author left it."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs"
  verifying_test: "crates/happenstance-ladybug/tests/round_trip.rs::rollback_leaves_the_checkpoint_where_it_was"

- id: AC-007
  criterion: "The crate stops claiming to be a skeleton, and clippy can catch it if it starts again. GIVEN a reviewer who trusts the gate over prose, WHEN they run the workspace clippy step and read the crate's own docs, THEN `#![allow(clippy::todo)]` is gone from `lib.rs` (deleted, not narrowed), both `# Status: skeleton` headings are corrected, `Cargo.toml`'s \"Not yet implemented.\" description is corrected, and every fallible public function still carries an `# Errors` section naming conditions rather than error types."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs"
  verifying_test: "cargo clippy --workspace --all-targets --all-features -- -D warnings (xtask/src/main.rs clippy step); grep -n \"allow(clippy::todo)\" crates/happenstance-ladybug/src/lib.rs empty"

- id: AC-008
  criterion: "Generic code still instantiates at both flavours, with real bodies behind it. GIVEN whatever `Batch` shape the frozen port merged, WHEN `port_shape.rs` compiles, THEN `weak_flavour::advance` and `send_flavour::spawn_a_batch_across_an_await` still instantiate at `LadybugProjectionStore` with the two flavour modules kept separate, and any bound that had to change — notably `for<'a> S::Batch<'a>: Send` collapsing to `S::Batch: Send` — is recorded as a finding, in the PR description and in the PS-34 record, never silently weakened."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs"
  verifying_test: "crates/happenstance-ladybug/tests/port_shape.rs::both_batch_shapes_satisfy_the_same_generic_code (and the `const _` block at :75-80)"

- id: AC-009
  criterion: "PS-34's disposition is on disk, dated, and honest about the context that wrote it. GIVEN the third pair of hands has just written the four bodies without having read `live_handle.rs:68-83`, WHEN they record the outcome, THEN `references/evaluation/ps-34-third-implementer-disposition.md` exists, is dated and commit-pinned, names the documentation the author did have, states exactly one of the two mutually exclusive outcomes (trap retired — no `error[E0195]`, no `where Self: 'a` in any impl; or the literal re-test with the diagnostic captured and the documented remedy judged sufficient or not), and moves no marker in `spec/SPECIFICATION.md`."
  satisfied: false
  evidence: ""
  mount_point: "references/evaluation/ps-34-third-implementer-disposition.md"
  verifying_test: "Process check: file present, dated and commit-pinned per references/evaluation/README.md:1-13; git diff <slice-base>..HEAD -- spec/SPECIFICATION.md shows no marker changed"

- id: AC-010
  criterion: "The specification's cross-references still point at the truth after the bodies land. GIVEN six citations naming lines this PR moves, WHEN the reviewer runs the trace step and reads the diff, THEN `cargo xtask spec-trace` is green and the diff shows only non-normative prose changed: `:4590`, `:4592`, `:4605` and `:4612` renumbered, `:372` and `:8095` minimally corrected because this PR falsifies their \"Two of the five are `todo!()` throughout\" claim, and no `[FROZEN]` marker, clause sentence or maturity-table row differs."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md"
  verifying_test: "cargo xtask spec-trace (xtask/src/spec_trace.rs:291-378); hand-reviewed git diff <slice-base>..HEAD -- spec/SPECIFICATION.md recorded in the implementation report"
```
