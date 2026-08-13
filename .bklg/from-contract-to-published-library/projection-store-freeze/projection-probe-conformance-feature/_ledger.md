---
item: HS-S0005
stage: implement
created: 2026-08-12T13:45:59.653Z
updated: 2026-08-12T13:45:59.653Z
---

# Acceptance ledger — ProjectionProbe behind happenstance-core's conformance feature

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
  criterion: "**GIVEN** an adapter author with a working `ProjectionStore` impl who needs a bar that tells them when they are done, **WHEN** they enable `conformance` and write `impl ProjectionProbe for TheirStore`, **THEN** the trait they must satisfy is exactly the five members the specification publishes — `const READS_THROUGH_BATCH: bool`, `probe_write(&self, &mut Self::Batch, &str, u64)`, `probe_delete_all(&self, &mut Self::Batch)`, `async fn probe_read(&self, &str) -> Result<Option<u64>, Self::Error>`, `probe_read_through(&self, &Self::Batch, &str) -> Option<u64>` — declared as `pub trait ProjectionProbe: ProjectionStore` in the **bare** flavour with no `SendProjectionProbe` anywhere, so `Self::Batch` and `Self::Error` resolve through the port they already implement and a `Send` adapter is served by `trait_variant`'s blanket impl rather than by a second trait to implement. Nothing is added, renamed or re-typed relative to `spec/SPECIFICATION.md:4998-5031`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/projection.rs — item-level `#[cfg(feature = \"conformance\")]`, supertrait of `ProjectionStore`; re-exported from crates/happenstance-core/src/lib.rs:98-124"
  verifying_test: "crates/happenstance-core/tests/projection_probe_round_trip.rs::probe_shape_matches_the_specification"

- id: AC-002
  criterion: "**GIVEN** the same author, who is already paying for one dependency edge on `happenstance-core` and will not accept a second, **WHEN** they add `happenstance-core = { version = \"…\", features = [\"conformance\"] }` to the `[dependencies]` (not `[dev-dependencies]`) of the crate their store lives in, **THEN** the feature exists in `happenstance-core`'s `[features]` table as `conformance = []` — pulling in **no** dependency, **not** implying `std`, and **not** implying `memory` — so enabling it costs them exactly one flag on a dependency they already have and no new edge in their graph."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/Cargo.toml [features] — `conformance = []` beside `std`, `serde` and `memory` (Cargo.toml:34-52)"
  verifying_test: "cargo hack check --workspace --feature-powerset --no-dev-deps, via `cargo xtask ci` (xtask/src/main.rs:546-556) + reviewed diff of crates/happenstance-core/Cargo.toml"

- id: AC-003
  criterion: "**GIVEN** an author reading the crate's docs to find out whether the seam exists at all, **WHEN** they open `happenstance-core`'s docs.rs page or type `happenstance_core::Projection…`, **THEN** the item is *reachable and announced* at both mount points and neither alone: a `pub use projection::ProjectionProbe;` in the export block (`crates/happenstance-core/src/lib.rs:98-124`) carrying the `#[cfg(feature = \"conformance\")]` + `#[cfg_attr(docsrs, doc(cfg(feature = \"conformance\")))]` pair the `memory` items already carry at `:120-122` — so the rendered page shows the feature badge rather than an unexplained absence — **and** a `conformance` bullet in the crate doc's `# Feature flags` list (`:75-82`) stating what it is for and that it is off by default and implies nothing. An item mounted at one and not the other is an item nobody finds."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) and crates/happenstance-core/src/lib.rs:75-82 (crate-doc `# Feature flags` list)"
  verifying_test: "crates/happenstance-core/tests/projection_probe_round_trip.rs — `use happenstance_core::ProjectionProbe;` (an unexported item is error[E0432]); plus the nightly --cfg docsrs rustdoc build in `cargo xtask ci`"

- id: AC-004
  criterion: "**GIVEN** an author who wants proof that the seam is *generic* — usable by suite code that has never heard of their store — and not merely present on one type, **WHEN** the round-trip test runs, **THEN** a function bound only on `P: ProjectionProbe` performs `begin` → `probe_write(&mut batch, \"k\", 7)` → `commit(batch, &id, position)` → `probe_read(\"k\") == Some(7)`, calling **no** inherent method of the concrete store anywhere on the assertion path, and the same helper also drives `probe_delete_all` and (under `P::READS_THROUGH_BATCH`) `probe_read_through`, so no member is dead on arrival. The instantiating store is defined in the test file itself, not `MemoryProjectionStore`, which does not exist yet at this story's boundary and whose use would prove only that the seam works for the oracle."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/tests/projection_probe_round_trip.rs — file-level `#![cfg(feature = \"conformance\")]`, exercising the export-block mount at crates/happenstance-core/src/lib.rs:98-124"
  verifying_test: "crates/happenstance-core/tests/projection_probe_round_trip.rs::writes_are_visible_through_the_trait_alone and ::all_five_members_are_reachable_generically"

- id: AC-005
  criterion: "**GIVEN** the local-first / edge developer (Persona 3) whose target is `wasm32` and whose build has no `std`, **WHEN** they enable `conformance` in any combination of the crate's other features — including `conformance` **without** `memory`, and `conformance` under `--no-default-features`, which is `no_std` — **THEN** it compiles on the host **and** on `wasm32-unknown-unknown`, because the trait body names only `core`/`alloc` and the port's own types. The widened combination set is 16 per target, up from 8."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/Cargo.toml [features] — `conformance = []`, implying neither `std` nor `memory`"
  verifying_test: "cargo hack check --workspace --feature-powerset --no-dev-deps (xtask/src/main.rs:546-556); cargo hack check -p happenstance-core … --target wasm32-unknown-unknown --feature-powerset (xtask/src/main.rs:564-593); cargo check -p happenstance-core --no-default-features --target wasm32-unknown-unknown (xtask/src/main.rs:205-215)"

- id: AC-006
  criterion: "**GIVEN** an author (or the evaluator, Persona 4) who meets this trait *only* through its rendered documentation, **WHEN** they read `ProjectionProbe`'s page, **THEN** every member carries a doc comment, `probe_read` carries `# Errors` naming the *conditions* rather than the error type, `probe_read_through`'s doc reproduces the specification's `unimplemented!()` guidance verbatim (the spelling is enforceable: `clippy::todo` is `deny` workspace-wide, `unimplemented` is not linted), and the trait's own doc restates why it lives in the contract crate rather than the testkit — the orphan-rule argument of D1 — naming `documented-extension-surface` as the story that can falsify it. **AND** no ungated doc comment anywhere in the crate links to `ProjectionProbe` or its members: the name is spelled plainly where mentioned, as `lib.rs:71-73` already does for `MemoryEventStore`, because a link into a `cfg`-gated item is a **hard error** under the gate's `--no-default-features` doc build."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/projection.rs (the trait's own rustdoc) and crates/happenstance-core/src/lib.rs:71-82 (plain-spelling comment + `# Feature flags` list)"
  verifying_test: "cargo doc -p happenstance-core --no-default-features with RUSTDOCFLAGS=-D warnings (xtask/src/main.rs:494-514); cargo clippy --workspace --all-targets --all-features -- -D warnings (missing_docs, missing_errors_doc, clippy::todo)"

- id: AC-007
  criterion: "**GIVEN** everyone already depending on this workspace's green gate, **WHEN** this PR is merged, **THEN** `cargo xtask ci` is green whole — the event-store conformance suite, the three existing harnesses and all four `wasm32` steps unchanged and passing — **and** the story's own boundary held: `spec/SPECIFICATION.md` is not edited (PS-11 and PS-12 stay `[PROVISIONAL]` at `:4980` and `:5055`, and `cargo xtask spec-trace` is therefore unaffected), no conformance rule, fixture, mutant or `MemoryProjectionStore` is added, no adapter skeleton is touched, and no ADR is written as a side effect of this story. If the reshape trigger of D9 is met — a projection rule that can observe the read model *without* the probe — it is **reported at the slice boundary**, not absorbed by shrinking the seam here."
  satisfied: false
  evidence: ""
  mount_point: "the whole workspace gate — xtask/src/main.rs (`cargo xtask ci` step list), with this story's diff confined to the PR boundary block in spec.md"
  verifying_test: "cargo xtask ci run whole on a clean tree (includes cargo test --workspace --all-features and cargo xtask spec-trace); git diff --stat against the merge base"
```
