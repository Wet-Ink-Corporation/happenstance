---
item: HS-S0076
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Swap the stand-in for the real `lbug` driver

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
  criterion: "GIVEN an adapter author who was told `happenstance-ladybug` targets LadybugDB and found a hand-written stand-in where the driver should be (`stand_in.rs:1-11`), WHEN they clone the tree and run `cargo build -p happenstance-ladybug --locked`, THEN the build resolves and compiles the real `lbug` crate — pinned once in the workspace manifest's `[workspace.dependencies]` beside `rusqlite` and `sqlx` with a stated version and a stated reason, consumed by the member as `lbug.workspace = true` — at a version that is neither a wildcard nor yanked and whose transitive licences fall inside the eight-entry allowlist, with the NOTE at `crates/happenstance-ladybug/Cargo.toml:18-22` gone."
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml [workspace.dependencies] -> crates/happenstance-ladybug/Cargo.toml:14-25"
  verifying_test: "cargo build -p happenstance-ladybug --locked; cargo deny check (xtask/src/main.rs:595-601, deny.toml:6,:10-19,:24)"

- id: AC-002
  criterion: "GIVEN the reviewer who is being asked to believe an \"unlike shape\" claim that rests on eight facts a human transcribed from docs.rs for `lbug` 0.16.1, WHEN they read this story's implementation report against the merged crate, THEN each of the eight rows at `stand_in.rs:20-30` is recorded as confirmed against the real crate with the real item cited, or as a divergence naming which argument it disturbs — specifically whether a `Transaction` type exists (PS-4 and the deferred write set, `projection_store.rs:1-22`), whether `Connection::query` takes `&self` (`stand_in.rs:203-212`), and whether `Database`/`Connection` are `Send + Sync` (`projection_store.rs:252-257`) — and no row is silently dropped with the module."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs:34-49 (the restated findings) + this story's implementation-report.md"
  verifying_test: "Review tier: eight-row reconciliation table in .bklg/from-contract-to-published-library/ladybug-projection-store/real-lbug-driver-swap/implementation-report.md; cargo doc --locked --workspace --all-features"

- id: AC-003
  criterion: "GIVEN the same reviewer, being asked to believe `impl SendProjectionStore for LadybugProjectionStore` (`projection_store.rs:252-258`) after the only compiler-checked basis for it — four assertions in a module this PR deletes (`stand_in.rs:332-360`) — has gone, WHEN they run `cargo test --locked --workspace --all-features`, THEN the same four assertions run against the real `lbug` types and pass; AND if any of them does not hold, the story halts and records a BR-12-shaped finding for the freeze verdict rather than quietly switching the adapter to the weak flavour."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/projection_store.rs:252-258 (the impl the assertions license)"
  verifying_test: "crates/happenstance-ladybug/src/projection_store.rs — #[cfg(test)] mod: database_and_connection_are_send_and_sync, query_result_is_send, the_driver_error_is_send_and_sync (re-homed from stand_in.rs:342-359)"

- id: AC-004
  criterion: "GIVEN an application author who already matches on `LadybugProjectionStoreError` and has read its doc comments to learn why `MalformedCheckpoint` refuses to collapse a corrupt checkpoint into `Ok(None)`, WHEN they read the enum after the swap, THEN exactly two things have changed — `Driver`'s `#[from]` field and `Commit`'s `#[source]` field now carry `lbug::Error` — while `MalformedCheckpoint`, `PositionOutOfRange` and `WriteTransactionInUse` stand untouched with their doc comments and every `# Errors` section intact, `#[non_exhaustive]` remains, and the enum still satisfies `core::error::Error + 'static` so ADR-0009's bound is met without the port ever demanding `Send`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/projection_store.rs:152-214, re-exported from crates/happenstance-ladybug/src/lib.rs:79-81"
  verifying_test: "crates/happenstance-ladybug/src/projection_store.rs — #[cfg(test)] mod: const-fn error-bound assertion over LadybugProjectionStoreError; cargo clippy --locked --workspace --all-targets --all-features -- -D warnings"

- id: AC-005
  criterion: "GIVEN a projection author who will write parameterised Cypher and was promised parameters travel beside the statement text so replay can `prepare` once and `execute` many times (`projection_store.rs:72-75`), WHEN `GraphStatement::parameters` becomes `Vec<(Box<str>, lbug::Value)>`, THEN `GraphStatement` and `GraphWriteSet` still compile with nothing interpolated into statement text; AND if the real `Value` does not implement what those types derive (`Debug`, `Clone`, and `Default` on the write set), the derive that has to go is recorded as a public-surface change in the report and in the rustdoc, never dropped in silence."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/projection_store.rs:70-150, re-exported from crates/happenstance-ladybug/src/lib.rs:79-81"
  verifying_test: "crates/happenstance-ladybug/src/projection_store.rs — #[cfg(test)] mod: GraphStatement-with-one-parameter construction pushed onto a GraphWriteSet; crates/happenstance-ladybug/tests/port_shape.rs:75-80 const _ block"

- id: AC-006
  criterion: "GIVEN the adapter author whom `projection_store.rs:261-264` promised that dropping the port's batch lifetime would be a deletion here rather than a redesign, WHEN the merged `ProjectionStore` meets this crate, THEN the promise is kept and observed: under `type Batch;` the impl's `type Batch<'a> = GraphWriteSet where Self: 'a` collapses to `type Batch = GraphWriteSet` and `port_shape.rs:61`'s `for<'a> S::Batch<'a>: Send` collapses to `S::Batch: Send` — that collapse being PS-5's predicted ergonomic relief observed, written down where the freeze verdict can cite it; under a surviving GAT nothing collapses and that is recorded too. Either way `port_shape.rs` is extended, never weakened: both flavour modules stay separate, the `const _` block still instantiates generic code at every store the crate still has, and a bound that has to change is surfaced as a finding rather than loosened."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/projection_store.rs:258-292 (the impl header and associated type) + crates/happenstance-ladybug/tests/port_shape.rs:75-80"
  verifying_test: "crates/happenstance-ladybug/tests/port_shape.rs::both_batch_shapes_satisfy_the_same_generic_code (cargo test -p happenstance-ladybug --test port_shape), with the const _ instantiation at :75-80"

- id: AC-007
  criterion: "GIVEN the sibling implementer who must record PS-34's disposition from a context that has NOT read `live_handle.rs:68-83` first (`_storymap.md`, PS-34, and who is allowed to write it), WHEN this story retires or re-points `live_handle.rs` as one decision with `stand_in`'s removal (T1/T3), THEN neither the `error[E0195]` transcript nor the rustc 1.97.1 ICE transcript is re-pasted into `lib.rs`, a commit message, this story's report, or anywhere else the sibling's context will meet it — both are referred to by citation into `references/adapter-shapes.md:169-195` and `:363-367` — AND the retirement itself is recorded as a verdict finding (the freeze deletes the counter-example that was evidence for freezing), so the evidence survives the file."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs:74,:78 (the module declaration and re-export) + crates/happenstance-ladybug/tests/port_shape.rs:9,:77,:79"
  verifying_test: "rg -n \"E0195|internal compiler error\" crates/happenstance-ladybug .bklg/from-contract-to-published-library/ladybug-projection-store/real-lbug-driver-swap returns citations only; cargo build --locked --workspace --all-features"

- id: AC-008
  criterion: "GIVEN an adapter author who opens `lib.rs` first and currently reads The driver is deliberately absent — a section that will be false the moment this merges — WHEN they read the crate root after the swap, THEN there is exactly one type universe and the documentation describes the crate that exists: `pub mod stand_in;` is gone from `lib.rs:76`, `rg stand_in crates/happenstance-ladybug` returns nothing, the re-export list at `:78-81` names only types that exist, `:24-32` is rewritten to state what the build now costs and that docs.rs fails on `lbug` 0.19.1 as a live consequence, `:34-49` is restated against AC-002's reconciliation rather than left as a claim about a stand-in, and `:51-66`'s Open decisions is replaced by a link to ADR-0025's atom per CLAUDE.md's link-the-atom / cite-the-record rule."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-ladybug/src/lib.rs:24-81 (the crate root and composition root)"
  verifying_test: "rg -n stand_in crates/happenstance-ladybug returns nothing (merge-gate grep); cargo doc --locked --workspace --all-features and cargo clippy --locked --workspace --all-targets --all-features -- -D warnings"

- id: AC-009
  criterion: "GIVEN the reviewer who must be able to trust that `spec/SPECIFICATION.md`'s `file:line` citations resolve — the property `cargo xtask spec-trace` exists to keep, and the one a deleted file breaks in a way no later merge repairs — WHEN the gate runs on this diff, THEN `spec-trace` is green: the four citations naming `live_handle.rs` (`:4592`, `:4605`, `:4612`, `:8095`) and the status-table narration at `:372` are repaired together with any count a repaired sentence states, every edit sits in non-normative framing prose, and no maturity marker, clause sentence or falsifier differs from its state at this story's base commit."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:372,:4592,:4605,:4612,:8095 (citations only)"
  verifying_test: "cargo xtask spec-trace (xtask/src/spec_trace.rs:291-372, tolerance :374-378); git diff <base>..HEAD -- spec/SPECIFICATION.md reviewed by hand"

- id: AC-010
  criterion: "GIVEN every contributor whose `cargo xtask ci` will, from this commit onward, compile LadybugDB's C++ through `cxx` and `cmake`, WHEN the whole gate runs on the swapped tree, THEN it is green — `cargo xtask ci`, not `--fast`, because this is the first time `cargo deny`, the workspace feature powerset and `spec-trace` meet a new dependency graph — with the four package-selected `wasm32` steps unchanged and unaffected; AND the wall-clock cost this adds to a cold gate is measured and handed on to `cold-build-cost-and-ci-shape` rather than acted on here; AND if `lbug` or its build script will not build at MSRV 1.97.1, the story halts and records an ADR-0004/ADR-0029-shaped finding rather than moving the floor in silence."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:115-155,:546-556,:595-601 (the gate, defined once) over crates/happenstance-ladybug/**"
  verifying_test: "cargo xtask ci (whole gate, not --fast); cargo hack check --workspace --feature-powerset --no-dev-deps; timed cold cargo build -p happenstance-ladybug --locked with a fresh CARGO_TARGET_DIR recorded in the implementation report"
```
