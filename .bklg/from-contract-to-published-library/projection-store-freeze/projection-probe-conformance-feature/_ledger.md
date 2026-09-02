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
  satisfied: true
  evidence: |
    `ProjectionProbe` at crates/happenstance-core/src/projection.rs:352-453 — `pub trait ProjectionProbe: ProjectionStore` (:403) in the bare flavour, five members and no more: `const READS_THROUGH_BATCH: bool` (:412), `probe_write(&self, &mut Self::Batch, &str, u64)` (:419), `probe_delete_all(&self, &mut Self::Batch)` (:427), `probe_read(&self, &str) -> impl Future<Output = Result<Option<u64>, Self::Error>>` (:447), `probe_read_through(&self, &Self::Batch, &str) -> Option<u64>` (:453). No `trait_variant::make`, and `rg -n "SendProjectionProbe" crates/` finds only two prose mentions forbidding one — no declaration. Verified by crates/happenstance-core/tests/projection_probe_round_trip.rs::probe_shape_matches_the_specification (passing), which coerces every member to an explicitly written signature through a generic `P: ProjectionProbe` so a rename is E0599 and a re-type is E0308, and by ::the_probe_is_a_supertrait_of_the_bare_flavour (passing), which calls `ProjectionStore::begin` from a bound of `ProjectionProbe` alone. RED: `error[E0432]: unresolved import happenstance_core::ProjectionProbe` — `no ProjectionProbe in the root`. DEVIATION, forced by house rule and recorded: `probe_read` is spelled `-> impl Future<...>` rather than the specification's `async fn`. RS-22-1 (standards/rust/22-rpitit-and-lifetime-capture.md:12) requires the desugaring in a public trait not under `#[trait_variant::make]`, because `async fn` there fires `async_fn_in_trait` under the gate's `-D warnings` — observed, not assumed. Semantics identical; the desugaring also puts the absence of `+ Send` at the declaration.
  mount_point: "crates/happenstance-core/src/projection.rs — item-level `#[cfg(feature = \"conformance\")]`, supertrait of `ProjectionStore`; re-exported from crates/happenstance-core/src/lib.rs:98-124"
  verifying_test: "crates/happenstance-core/tests/projection_probe_round_trip.rs::probe_shape_matches_the_specification"

- id: AC-002
  criterion: "**GIVEN** the same author, who is already paying for one dependency edge on `happenstance-core` and will not accept a second, **WHEN** they add `happenstance-core = { version = \"…\", features = [\"conformance\"] }` to the `[dependencies]` (not `[dev-dependencies]`) of the crate their store lives in, **THEN** the feature exists in `happenstance-core`'s `[features]` table as `conformance = []` — pulling in **no** dependency, **not** implying `std`, and **not** implying `memory` — so enabling it costs them exactly one flag on a dependency they already have and no new edge in their graph."
  satisfied: true
  evidence: |
    `conformance = []` at crates/happenstance-core/Cargo.toml:60-79 — literally an empty list: no `dep:` entry, no `std`, no `memory`. The manifest comment carries the orphan-rule argument that makes the empty list load-bearing and states that acquiring a `dep:` voids the placement decision. Verified by `cargo hack check -p happenstance-core --feature-powerset --no-dev-deps`: 16 combinations, all green, including `--no-default-features --features conformance` (3/16) — the crate's `no_std` arm with `conformance` and nothing else.
  mount_point: "crates/happenstance-core/Cargo.toml [features] — `conformance = []` beside `std`, `serde` and `memory` (Cargo.toml:34-52)"
  verifying_test: "cargo hack check --workspace --feature-powerset --no-dev-deps, via `cargo xtask ci` (xtask/src/main.rs:546-556) + reviewed diff of crates/happenstance-core/Cargo.toml"

- id: AC-003
  criterion: "**GIVEN** an author reading the crate's docs to find out whether the seam exists at all, **WHEN** they open `happenstance-core`'s docs.rs page or type `happenstance_core::Projection…`, **THEN** the item is *reachable and announced* at both mount points and neither alone: a `pub use projection::ProjectionProbe;` in the export block (`crates/happenstance-core/src/lib.rs:98-124`) carrying the `#[cfg(feature = \"conformance\")]` + `#[cfg_attr(docsrs, doc(cfg(feature = \"conformance\")))]` pair the `memory` items already carry at `:120-122` — so the rendered page shows the feature badge rather than an unexplained absence — **and** a `conformance` bullet in the crate doc's `# Feature flags` list (`:75-82`) stating what it is for and that it is off by default and implies nothing. An item mounted at one and not the other is an item nobody finds."
  satisfied: true
  evidence: |
    Two mounts, and the third weaker one. Export block: crates/happenstance-core/src/lib.rs:132-134 — `#[cfg(feature = "conformance")] #[cfg_attr(docsrs, doc(cfg(feature = "conformance")))] pub use projection::ProjectionProbe;`, matching the `memory` pair at :136-138 exactly, so docs.rs renders the feature badge. Feature-flags list: lib.rs:83-92, stating what it is for, that it is off by default, and that it implies nothing — with the name spelled plainly and the reason for the non-link given inline. Proven by compilation rather than inspection: projection_probe_round_trip.rs:37-40 imports the trait by its crate-root path `happenstance_core::ProjectionProbe`, and an unexported item is error[E0432] — which is exactly the RED transcript this story started from. `cargo doc -p happenstance-core --all-features` green.
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) and crates/happenstance-core/src/lib.rs:75-82 (crate-doc `# Feature flags` list)"
  verifying_test: "crates/happenstance-core/tests/projection_probe_round_trip.rs — `use happenstance_core::ProjectionProbe;` (an unexported item is error[E0432]); plus the nightly --cfg docsrs rustdoc build in `cargo xtask ci`"

- id: AC-004
  criterion: "**GIVEN** an author who wants proof that the seam is *generic* — usable by suite code that has never heard of their store — and not merely present on one type, **WHEN** the round-trip test runs, **THEN** a function bound only on `P: ProjectionProbe` performs `begin` → `probe_write(&mut batch, \"k\", 7)` → `commit(batch, &id, position)` → `probe_read(\"k\") == Some(7)`, calling **no** inherent method of the concrete store anywhere on the assertion path, and the same helper also drives `probe_delete_all` and (under `P::READS_THROUGH_BATCH`) `probe_read_through`, so no member is dead on arrival. The instantiating store is defined in the test file itself, not `MemoryProjectionStore`, which does not exist yet at this story's boundary and whose use would prove only that the seam works for the oracle."
  satisfied: true
  evidence: |
    crates/happenstance-core/tests/projection_probe_round_trip.rs::writes_are_visible_through_the_trait_alone (passing) — the helper `round_trip<P: ProjectionProbe>` at :232-253 does `begin` → `probe_write(&mut batch, "k", 7)` → `commit(batch, &id, position, Authority::Live)` → `probe_read("k") == Some(7)`, and asserts en route that an open batch is invisible. `TestStore` is named only at the instantiation site; no inherent method appears anywhere on the assertion path. ::all_five_members_are_reachable_generically (passing) drives `probe_delete_all` through `reset` and `probe_read_through` under `P::READS_THROUGH_BATCH`, plus `rollback`, so no member is dead on arrival for `reset-rules` and `read-through-and-rebuild-rules`. The instantiating store is defined in the test file (:46-158, `TestStore` + its two impls), not `MemoryProjectionStore`, and the word `memory` does not appear in the file — EC-008's required response, and D6's independence made visible by inspection.
  mount_point: "crates/happenstance-core/tests/projection_probe_round_trip.rs — file-level `#![cfg(feature = \"conformance\")]`, exercising the export-block mount at crates/happenstance-core/src/lib.rs:98-124"
  verifying_test: "crates/happenstance-core/tests/projection_probe_round_trip.rs::writes_are_visible_through_the_trait_alone and ::all_five_members_are_reachable_generically"

- id: AC-005
  criterion: "**GIVEN** the local-first / edge developer (Persona 3) whose target is `wasm32` and whose build has no `std`, **WHEN** they enable `conformance` in any combination of the crate's other features — including `conformance` **without** `memory`, and `conformance` under `--no-default-features`, which is `no_std` — **THEN** it compiles on the host **and** on `wasm32-unknown-unknown`, because the trait body names only `core`/`alloc` and the port's own types. The widened combination set is 16 per target, up from 8."
  satisfied: true
  evidence: |
    Host powerset: `cargo hack check -p happenstance-core --feature-powerset --no-dev-deps` — 8 combinations became 16, all green, including `conformance` alone, `conformance` without `memory` (3/16), and `conformance` under `--no-default-features`, which is `no_std`. wasm32 powerset: `cargo hack check -p happenstance-core -p happenstance-neon -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` — 25 combinations, all green. Plus `cargo xtask wasm`, whose mandatory `cargo check -p happenstance-core --no-default-features --target wasm32-unknown-unknown` is the arm a powerset skips when cargo-hack is absent. The trait body names only `core::future::Future` and the port's own types, so nothing reaches for `std`: the named wrong implementation — a `conformance` surface that only compiles alongside `memory` — is rejected by combination 3 of 16.
  mount_point: "crates/happenstance-core/Cargo.toml [features] — `conformance = []`, implying neither `std` nor `memory`"
  verifying_test: "cargo hack check --workspace --feature-powerset --no-dev-deps (xtask/src/main.rs:546-556); cargo hack check -p happenstance-core … --target wasm32-unknown-unknown --feature-powerset (xtask/src/main.rs:564-593); cargo check -p happenstance-core --no-default-features --target wasm32-unknown-unknown (xtask/src/main.rs:205-215)"

- id: AC-006
  criterion: "**GIVEN** an author (or the evaluator, Persona 4) who meets this trait *only* through its rendered documentation, **WHEN** they read `ProjectionProbe`'s page, **THEN** every member carries a doc comment, `probe_read` carries `# Errors` naming the *conditions* rather than the error type, `probe_read_through`'s doc reproduces the specification's `unimplemented!()` guidance verbatim (the spelling is enforceable: `clippy::todo` is `deny` workspace-wide, `unimplemented` is not linted), and the trait's own doc restates why it lives in the contract crate rather than the testkit — the orphan-rule argument of D1 — naming `documented-extension-surface` as the story that can falsify it. **AND** no ungated doc comment anywhere in the crate links to `ProjectionProbe` or its members: the name is spelled plainly where mentioned, as `lib.rs:71-73` already does for `MemoryEventStore`, because a link into a `cfg`-gated item is a **hard error** under the gate's `--no-default-features` doc build."
  satisfied: true
  evidence: |
    WITHDRAWN 2026-08-13 and RE-ASSERTED 2026-08-14 against corrected text. The row was recorded
    `satisfied: true` on evidence that misdescribed the artefact in two places, and the slice review
    found both. It was flipped back to `satisfied: false`, the rustdoc was corrected, and the
    evidence below is written against what the file now says rather than against what the row
    previously claimed. What the withdrawn evidence asserted, and why each half was false:

    * *"with the `[dev-dependencies]` cost stated"* — the snippet published a `[dev-dependencies]`
      block carrying `happenstance-core = { features = ["conformance"] }`. That contradicts the
      paragraph three lines above it, which argues the impl **cannot** live in the adapter's
      `tests/` crate (orphan rule) and must therefore live in the adapter's `src/`: a dev-dependency
      does not exist for the lib build the impl compiles in, and Rust cannot `#[cfg]` on a
      dependency's feature. It also contradicted the snippet's own "one flag on a dependency they
      already have" by adding a second entry. The story spec anticipated it verbatim — AC-002 reads
      "`[dependencies]` (not `[dev-dependencies]`)".
    * *"names its own falsifier"* — the sentence read "the falsifier is the story that builds an
      outside author's fixture from the documentation alone, and it is named here", and no name
      followed. Self-refuting, and this criterion requires the name.

    NOW MET. Every member is documented at crates/happenstance-core/src/projection.rs:527-577, under
    `missing_docs = "warn"` with `-D warnings` (clippy green over all seven affected packages).
    `probe_read`'s `# Errors` names the condition — "if the read model cannot be read" — not the
    error type (:568-570). `probe_read_through`'s doc reproduces the specification's
    `unimplemented!()` spelling verbatim (:575-576); the spelling is enforceable rather than
    stylistic, because `clippy::todo` is `deny` workspace-wide and `unimplemented` is not linted.
    The trait's own doc carries D1's orphan-rule placement argument in full at :482-517, and the
    manifest snippet at :496-509 is now the shape that compiles and still demonstrates the argument:
    `[dependencies] happenstance-core = "…"`, `[features] conformance =
    ["happenstance-core/conformance"]`, `[dev-dependencies] happenstance-testkit = "…"` — with the
    comment stating why the `[dev-dependencies]` spelling of the first entry cannot work. The
    falsifier is named at :514-517: `documented-extension-surface` (HS-S0015). No inbound intra-doc
    link: `ProjectionProbe` is spelled plainly in lib.rs's feature-flags list with the reason inline,
    and all three doc steps are green — `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-core`
    at `--no-default-features` (where an inbound link into the gated item is a hard error), at
    default features, and at `--all-features` (where the new
    `[`# Implementing it`](ProjectionStore#implementing-it)` link resolves to
    `trait.ProjectionStore.html#implementing-it`, an anchor that exists in the rendered page).
  mount_point: "crates/happenstance-core/src/projection.rs (the trait's own rustdoc) and crates/happenstance-core/src/lib.rs:71-82 (plain-spelling comment + `# Feature flags` list)"
  verifying_test: "cargo doc -p happenstance-core --no-default-features with RUSTDOCFLAGS=-D warnings (xtask/src/main.rs:494-514); cargo clippy --workspace --all-targets --all-features -- -D warnings (missing_docs, missing_errors_doc, clippy::todo)"

- id: AC-007
  criterion: "**GIVEN** everyone already depending on this workspace's green gate, **WHEN** this PR is merged, **THEN** `cargo xtask ci` is green whole — the event-store conformance suite, the three existing harnesses and all four `wasm32` steps unchanged and passing — **and** the story's own boundary held: `spec/SPECIFICATION.md` is not edited (PS-11 and PS-12 stay `[PROVISIONAL]` at `:4980` and `:5055`, and `cargo xtask spec-trace` is therefore unaffected), no conformance rule, fixture, mutant or `MemoryProjectionStore` is added, no adapter skeleton is touched, and no ADR is written as a side effect of this story. If the reshape trigger of D9 is met — a projection rule that can observe the read model *without* the probe — it is **reported at the slice boundary**, not absorbed by shrinking the seam here."
  satisfied: true
  evidence: |
    SPLIT RESULT — the boundary half is met, the whole-gate half is not, and the reason is inherited rather than this story's.

    MET, the boundary half: `spec/SPECIFICATION.md` is not edited (PS-11 and PS-12 remain `[PROVISIONAL]`, and no clause text, marker or rule citation moved); no conformance rule, fixture, mutant, registry entry or `MemoryProjectionStore` was added; `crates/happenstance-testkit/` is untouched; no adapter skeleton was touched; no ADR was written. `git diff --stat` for this story's commit contains only crates/happenstance-core/src/projection.rs, crates/happenstance-core/src/lib.rs, crates/happenstance-core/Cargo.toml, crates/happenstance-core/tests/projection_probe_round_trip.rs and this story's backlog folder. EC-007's reshape trigger was not met: no projection rule that can observe the read model without the probe was found.

    NOT MET, the whole-gate half: `cargo xtask ci` is not green, and neither is `cargo xtask spec-trace` or `cargo xtask lint-constitution`. Nothing in this story causes it. The cause is the slice-mate `owned-batch-port-shape`, whose ADR-0017-mandated move of `crates/happenstance-ladybug/src/live_handle.rs` and whose growth of `projection.rs` invalidated 7 `file:line` citations in `spec/SPECIFICATION.md` and 16 in `standards/rust/`, and broke 3 compiled constitution examples that implement the port with the GAT. Both corpora sit outside both stories' PR boundaries; the full inventory with the exact re-points is .bklg/from-contract-to-published-library/projection-store-freeze/owned-batch-port-shape/_reviewed-diff.md §7. Every gate step this story owns is green: fmt, clippy `-D warnings` over all seven affected packages, `cargo test` over the six code packages (0 failures), both feature powersets over the widened combination set, the `--no-default-features` doc build, the `--all-features` doc build, and all four `cargo xtask wasm` steps. Left `satisfied: false` deliberately: the criterion says `cargo xtask ci` is green whole, and it is not.

    RESOLVED 2026-08-13 (slice review, authorised boundary widening). The remedy this note named
    was taken, arm (a): the re-pointing was pulled forward into the slice rather than deferred to
    `unstable-projection-gate-and-clause-disposition`, because `xtask/src/affected.rs:117-125`
    runs `spec-trace` first and unconditionally and `.redkiln/config.yaml:40` wires
    `cargo xtask affected --base {{base}}` as the story gate, so no story in this project could
    pass its own declared gate while the seven citations were stale.

    What was done. spec/SPECIFICATION.md: the five moved-file citations re-point to
    `experiments/live-handle-projection-batch/live_handle.rs` at identical line numbers (the file
    moved verbatim under ADR-0017), and the two anchors move to `ReadOptions::from`'s new line
    (`projection.rs:406-407`) and `rollback`'s (`projection.rs:456-465`). No clause text, marker
    or maturity changed — only `file:line` targets. standards/rust: nineteen citation targets
    re-pointed, and the three rules whose subject this slice deleted were dispositioned at their
    own stated triggers rather than by fresh editorial judgement — RS-22-4 and RS-92-1 **retired**
    into dated `## Retired` sections in their own atoms (both carried an explicit PS-5 settlement
    condition, and PS-5 landed with ADR-0017), and RS-21-1 **amended**: the obligation survives
    (`trait_variant` still delegates associated types verbatim), the higher-ranked spelling does
    not, so the bound is now `S::Batch: Send` and the dead `E0637` fence is replaced by a bare
    `compile_fail` whose real diagnostic — *future cannot be sent between threads safely*, with
    `code: None`, measured — is recorded in the prose.

    Green, re-run 2026-08-13 on the fixed tree:
    `cargo xtask spec-trace` — 0 problems, 358 citations checked (up from 353), §7.1-§7.2 match;
    `cargo test -p xtask --doc` — 227 passed, 0 failed, 2 ignored (was 227/3/3);
    `cargo xtask lint-constitution` — 27 atoms, all consistent;
    `cargo xtask affected --base main` — affected gate passed;
    `cargo xtask ci --fast` — all required checks passed;
    `cargo xtask ci` — **all checks passed**, whole, including both feature powersets,
    `cargo deny` and the nightly `--cfg docsrs` build.

    Two further gate failures this story's own run had not surfaced were fixed in the same change,
    both in `crates/happenstance-core/src/projection_memory.rs`: a `broken_intra_doc_links` hard
    error on the **default** feature set (`ProjectionProbe::READS_THROUGH_BATCH`, a `conformance`
    item, linked from a `memory` page — invisible to both doc steps the gate ran), and a
    pre-existing `redundant_explicit_links` hard error in the `--all-features`
    `--document-private-items` step. A third doc step, `documentation (default features)`
    (xtask/src/main.rs:515-540), now covers the blind spot.
  mount_point: "the whole workspace gate — xtask/src/main.rs (`cargo xtask ci` step list), with this story's diff confined to the PR boundary block in spec.md"
  verifying_test: "cargo xtask ci run whole on a clean tree (includes cargo test --workspace --all-features and cargo xtask spec-trace); git diff --stat against the merge base"
```
