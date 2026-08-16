---
item: "HS-S0026"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The application-facing Projection trait and its streaming runner

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Four things about this ledger are unusual and are unusual on purpose, so an implementer does not
"work around" them.

**A green `cargo xtask ci --fast` compiles none of this story's code.** `unstable-projection` is off
by default, so the default test run is green with the whole runner missing. Every row whose verifying
test lives in `projection_runner.rs` needs `cargo test -p happenstance --features
unstable-projection,memory` run explicitly, and AC-008 additionally needs `cargo hack check
--feature-powerset -p happenstance`, which `--fast` drops (`xtask/src/main.rs:830-836`). Evidence for
those rows is the output of the command run **by hand**.

**AC-007 and AC-012 are discharged partly by an absence.** AC-007's evidence includes an empty
`git diff --stat main -- spec/ crates/happenstance-core/src`; AC-012's includes the absence of any
`impl ProjectionStore for` under `crates/happenstance/`. An absence is not self-evidencing — cite the
command and its empty output, not a recollection that nothing was touched.

**AC-003 is satisfied only if a wrong implementation fails it.** The row is not discharged by the
atomicity test passing; it is discharged by `checkpoint_without_rows_is_rejected` demonstrating that a
runner committing the checkpoint without the batch **fails** the same assertion. A rule no
implementation can fail is decorative (`CLAUDE.md`, *The rule that matters*).

**AC-012 may halt the story rather than be worked around.** If HS-P0010's `MemoryProjectionStore` has
not landed, the correct outcome is to stop and report the missing dependency (EC-009) — not to write a
throwaway in-memory `ProjectionStore`, which would freeze a fixture shape this project does not own.

```yaml
- id: AC-001
  criterion: >-
    GIVEN an application author who has modelled a boundary and run a command (beats 1-3) and now
    wants a read model, WHEN they implement `happenstance::Projection` over their own domain enum —
    `type Event`, `type Store`, a `ProjectionId`, a `scope()` returning validated `Tags`, and one
    `apply` — THEN the events their projection is handed are nominated by a `Query` derived from
    `Self::Event::EVENT_TYPES` plus `scope()`, so there is no second filtering vocabulary to learn
    and no subscription to hand-maintain; and a projection spanning two stores is unrepresentable
    rather than merely undocumented, because `Store` is an associated type and no cross-store
    transaction exists.
  satisfied: true
  evidence: >-
    crates/happenstance/src/runner.rs:62 (`Projection` with `type Event`/`type
    Store`/`id`/`scope`/`apply`) and :420 (the query derived through `crate::boundary::derive_query`
    from `P::Event::EVENT_TYPES` plus `scope()`, the same derivation `Boundary::query` uses).
    `Store` is an associated type at :67, so a two-store projection is unrepresentable rather than
    merely discouraged. Test:
    crates/happenstance/tests/projection_runner.rs::derived_query_matches_event_types_and_scope —
    one read, the derived `Query` equal to an independently spelled oracle, and the out-of-scope
    `d9` event never handed to `apply`. PASS under `cargo test -p happenstance --features
    unstable-projection,memory`.
  mount_point: "crates/happenstance/src/lib.rs (crate root pub use, beside `pub use happenstance_core::*;`)"
  verifying_test: "crates/happenstance/tests/projection_runner.rs::derived_query_matches_event_types_and_scope (cargo test -p happenstance --features unstable-projection,memory)"

- id: AC-002
  criterion: >-
    GIVEN that same author, whose events are a Rust enum and not `Bytes`, WHEN the runner reads an
    event out of the store, THEN `apply` receives a decoded `Self::Event` produced by M3's `Codec` —
    the decode half ADR-0007 assigned to the typed layer — and the author writes no
    `serde_json::from_slice`, no manual match on `EventType`, and no second decode path anywhere in
    their projection.
  satisfied: true
  evidence: >-
    crates/happenstance/src/runner.rs:517 `apply_one` decodes through the crate's one path,
    `crate::codec::decode_event` (:531) — no second decoder anywhere. Tests:
    crates/happenstance/tests/projection_runner.rs::apply_receives_decoded_domain_events (the
    fixture `apply` at :154 takes `Stock`, so a `Bytes` signature would not compile against it) and
    crates/happenstance/tests/doc_surface.rs::no_second_decode_path (source assertion over
    runner.rs: `decode_event` present, `serde_json::from_slice` and its two siblings absent). Both
    PASS.
  mount_point: "crates/happenstance/src/lib.rs (run_projection's decode step, over M3's Codec)"
  verifying_test: "crates/happenstance/tests/projection_runner.rs::apply_receives_decoded_domain_events + crates/happenstance/tests/doc_surface.rs::no_second_decode_path"

- id: AC-003
  criterion: >-
    GIVEN an operator restarting a service after a crash, WHEN they compare the read model against
    the checkpoint, THEN they never find one ahead of the other: each chunk's read-model rows and its
    checkpoint move in the single `commit` (crates/happenstance-core/src/projection.rs:126), the
    runner never calls an `apply()`/`set_checkpoint()` pair because the port deliberately offers none
    (:20-26), and the position committed is the position of the last event actually applied, never a
    computed or anticipated one.
  satisfied: true
  evidence: >-
    crates/happenstance/src/runner.rs:484-495 — one `begin` (:452), the chunk applied into it, then
    the port's single `commit(batch, id, position, Authority::Live)`; the position is `last`, the
    position of the last event actually applied (:471). No `set_checkpoint` call, because the port
    offers none. Test:
    crates/happenstance/tests/projection_runner.rs::read_model_and_checkpoint_commit_together (rows
    and checkpoint both absent before, both present after, checkpoint equal to a position the store
    really assigned). Discriminated by ::checkpoint_without_rows_is_rejected
    (projection_runner.rs:412-457), a wrong runner that commits an empty batch and MUST fail the
    same agreement oracle — it does, and the test asserts that it does. Both PASS.
  mount_point: "crates/happenstance/src/lib.rs (run_projection's chunk loop, over crates/happenstance-core/src/projection.rs:117,:126,:138)"
  verifying_test: "crates/happenstance/tests/projection_runner.rs::read_model_and_checkpoint_commit_together, discriminated by ::checkpoint_without_rows_is_rejected"

- id: AC-004
  criterion: >-
    GIVEN an author whose projection has already run once, WHEN they run it again, THEN it resumes
    past the checkpoint and not at it — no event applied twice and none skipped, because
    `ReadOptions::from` is inclusive (projection.rs:104-106) — `None` from `checkpoint()` means never
    run and replays from the beginning rather than from position zero, `SequencePosition::next()`'s
    `None` arm (key-space exhaustion) is a typed refusal rather than an `unwrap`
    (event.rs:265-280), and positions may gap, so nothing computes `head − checkpoint` as a lag and
    no assertion names a literal position.
  satisfied: true
  evidence: >-
    crates/happenstance/src/runner.rs:427-435 — `considered_through` (:607) maps
    `Checkpoint::NeverRun` to no `from` anchor at all, and `Live`/`Rebuilding` to `through.next()`;
    the `None` arm of `next()` becomes `ProjectionError::KeySpaceExhausted` (:432), never an
    `unwrap`. No arithmetic on a position anywhere in the file beyond `next()`. Tests:
    projection_runner.rs::first_run_starts_from_the_beginning, ::resume_advances_past_the_checkpoint
    (a second run over an unchanged log applies 0 and reports `through: None`),
    ::tolerates_gapped_positions against `happenstance_testkit::GappyMemoryStore` at stride 7,
    comparing only against positions the store actually returned, and
    ::a_checkpoint_at_the_last_position_reports_exhausted_key_space, added after review found EC-005
    the only row of the EC table with a correct implementation and no executed path: the checkpoint
    is seeded at the last representable position through the **real** `MemoryProjectionStore`, and
    the arm, its `position()` and its empty `progress()` are all asserted. Under `saturating_add`
    the run would return `Ok` instead of that arm, so it is the wrong implementation this test
    rejects. All PASS.
  mount_point: "crates/happenstance/src/lib.rs (run_projection's resume arithmetic, over ProjectionStore::checkpoint and ReadOptions::from)"
  verifying_test: "crates/happenstance/tests/projection_runner.rs::resume_advances_past_the_checkpoint + ::first_run_starts_from_the_beginning + ::tolerates_gapped_positions (against happenstance_testkit::GappyMemoryStore) + ::a_checkpoint_at_the_last_position_reports_exhausted_key_space (EC-005)"

- id: AC-005
  criterion: >-
    GIVEN an author rebuilding a read model over a log far larger than memory, WHEN they call
    `run_projection` with a `chunk: NonZeroUsize`, THEN memory stays flat in the length of the
    replay: the runner never `collect`s the stream, commits happen while the stream is still being
    pulled, and the only buffer is the at-most-`chunk` events between one `begin` and its `commit` —
    which is the whole reason `EventStore::read` returns the stream at the top level and is not
    `async` (ADR-0001/ADR-0008), and the precondition for E2E-25's chunked rebuild.
  satisfied: true
  evidence: >-
    crates/happenstance/src/runner.rs:440-441 — one `events.read(...)` pinned with `core::pin::pin!`
    and pulled item by item by `next_event` (:594, a hand-written `poll_fn`); no `collect`, no `Vec`
    of the log. The only buffer is the at-most-`chunk` window at :455. Test:
    projection_runner.rs::commits_before_the_stream_ends — `PullWatching` (:245) records the
    committed row count at each pull and asserts rows[0] == 0 and rows.last() > 0 over five events
    at chunk 2, which a `collect`ing runner fails and a streaming one passes. PASS.
  mount_point: "crates/happenstance/src/lib.rs (run_projection's read + chunk loop, over crates/happenstance-core/src/store.rs:119)"
  verifying_test: "crates/happenstance/tests/projection_runner.rs::commits_before_the_stream_ends"

- id: AC-006
  criterion: >-
    GIVEN an author whose log contains one event their codec cannot decode, WHEN the runner reaches
    it, THEN they receive a typed `ProjectionError` naming the position it stopped at and carrying
    the concrete `CodecError` as a `#[source]` — the representable home E2E-26 says ADR-0007's pump
    signature cannot provide — the half-applied chunk is discarded through `rollback` so the
    checkpoint still sits at the last good position, the partial progress comes back as a returned
    `Progressed { through, applied }` value, and the runner renders nothing while it works: no
    spinner, no percentage, no line that rewrites in place (AC-U16).
  satisfied: true
  evidence: >-
    crates/happenstance/src/runner.rs:142 `ProjectionError`, whose `Decode` arm carries `position`
    and the concrete `CodecError` as `#[source]` (:186-201); the chunk is discarded through
    `rollback` at :480 and the partial progress rides on the error (`progress()` at :261). Test:
    projection_runner.rs::decode_failure_names_its_position_and_rolls_back — asserts the variant,
    the carried position, `error.position()` and `error.progress()` both at chunk 8 (nothing
    committed, checkpoint still `NeverRun`) and at chunk 2 (checkpoint at the last GOOD position,
    `applied == 2`), and walks `core::error::Error::source` down to `CodecError`. Plus
    doc_surface.rs::runner_prints_nothing. Both PASS.
  mount_point: "crates/happenstance/src/lib.rs (ProjectionError and Progressed, pub use'd at the crate root)"
  verifying_test: "crates/happenstance/tests/projection_runner.rs::decode_failure_names_its_position_and_rolls_back + crates/happenstance/tests/doc_surface.rs::runner_prints_nothing"

- id: AC-007
  criterion: >-
    GIVEN the author of `projection-clause-verdicts` (HS-S0027), who must write the PS-33 / PS-27 /
    PS-30 verdicts from evidence rather than intention, WHEN they read this story's implementation
    report, THEN they find each count stated as a fact about the tree — whether a checkpoint pump
    exists in `happenstance-core` at all and who calls it (PS-33), that no per-runner
    `on_error: SkipPolicy` was offered because failure policy is per projection (PS-27's Rejects),
    and that no fan-out runner was built because `Batch<'a>` borrows its store and cannot cross a
    `tokio::spawn` (PS-30) — with no line of `spec/SPECIFICATION.md` edited, no maturity marker
    moved, and no pump written into the contract crate to make ADR-0007's text match the tree.
  satisfied: true
  evidence: >-
    The implementation-report body, section "PS-33 / PS-27 / PS-30 evidence". Counted rather than
    asserted: `grep -rn "^pub fn |^pub async fn " crates/happenstance-core/src/` returns exactly two
    free functions — `collect` (store.rs:285) and `read_decision_model` (store.rs:321) — and `grep
    -rn "pump" crates/ --include=*.rs` returns exactly one hit, a doc comment in this story own
    runner.rs:134. There is no checkpoint pump in the contract crate, so its independent-caller
    count is zero, and none was written to make ADR-0007 read correctly. `grep -rn
    "SkipPolicy|on_error" crates/ --include=*.rs` finds only runner.rs:310, the doc comment stating
    the exclusion; `grep -rn "spawn" crates/happenstance/src` finds only runner.rs:319, the same.
    Boundary held: `git status --porcelain -- spec/ crates/happenstance-core/src` empty at commit
    time.
  mount_point: "the story's implementation-report body (the evidence HS-S0027 consumes); no mount in spec/ or crates/happenstance-core/"
  verifying_test: "empty `git diff --stat main -- spec/ crates/happenstance-core/src` asserted at review, plus the implementation report's PS-33/PS-27/PS-30 evidence section"

- id: AC-008
  criterion: >-
    GIVEN an evaluator with one bounded sitting reading the manifest to decide whether the projection
    surface is something they can depend on, WHEN they look for it, THEN they find a real
    `unstable-projection` feature that is off by default — making CHANGELOG.md:19-22's existing claim
    true rather than aspirational — whose forwarding to `happenstance-core/unstable-projection` (or
    its deliberate absence, if HS-P0010 has not landed that feature) is stated in a manifest comment
    beside the AC-A04 comment already there; and turning it on only adds: every item that compiled
    before still compiles, and `default-features = false` means exactly what it meant before this PR
    (RS-51-1).
  satisfied: true
  evidence: >-
    crates/happenstance/Cargo.toml — `unstable-projection =
    ["happenstance-core/unstable-projection", "dep:futures-core"]` under a comment stating the
    forwarding and why the port beneath is unfrozen; `default = ["std", "memory", "json"]`
    unchanged. Tests:
    crates/happenstance/tests/manifest_contract.rs::unstable_projection_is_declared_off_by_default
    (declared, absent from `default`, forwards, stated in a comment, and gates a real item at the
    crate root) PASS; xtask/src/main.rs tests::the_typed_layer_makes_no_promise_it_does_not_keep,
    rewritten to assert the passthrough and the surface agree in BOTH directions, PASS. `cargo hack
    check --feature-powerset -p happenstance` exits 0, run by hand because `--fast` drops it.
  mount_point: "crates/happenstance/Cargo.toml [features] (the unstable-projection row and its forwarding comment)"
  verifying_test: "crates/happenstance/tests/manifest_contract.rs::unstable_projection_is_declared_off_by_default + `cargo hack check --feature-powerset -p happenstance` (not run by --fast)"

- id: AC-009
  criterion: >-
    GIVEN an evaluator landing on the crate-root page after the alpha, WHEN they scan it for how this
    crate reads events back into a read model, THEN they meet the real surface and not a roadmap: the
    fifth vocabulary bullet at crates/happenstance/src/lib.rs:49-51 has become an intra-doc link to
    the real item in place — same position, same order, same discriminator prose — the `# Features`
    region gains an `unstable-projection` row of one clause, below the vocabulary and recessive,
    region 7 still last; the "Planned, and specified in spec/SPECIFICATION.md" heading is gone,
    because this is the last of the five bullets; and the density budget holds: first doc sentence
    ≤ 80 characters, identifier ≤ 24 characters, code inside a doc fence ≤ 72 columns, doc prose
    ≤ 80 columns, module doc ≤ 130 lines.
  satisfied: true
  evidence: >-
    crates/happenstance/src/lib.rs:106-110 — the fifth vocabulary bullet is now `[**The typed
    projection runner**][projection-runner]`, in place, still fifth and last; :125 adds the
    one-clause `unstable-projection` row to the Features table, below the vocabulary and above the
    adapter pointer. The `(planned)` marker and the paragraph that explained it are gone, because
    this was the last of the five. Tests: doc_surface.rs::crate_root_renders_the_projection_surface
    (order, link form, no roadmap, feature row, root re-exports, the 130-line and 80/72-column
    budgets), docs_composition.rs::roadmap_bullet_became_a_link, ::no_planned_heading_survives,
    ::landing_page_names_only_the_persistent_four, doc_budget.rs over the whole crate,
    src/tests.rs::doc_density_budget_holds and ::the_crate_root_page_fits_above_the_fold. All PASS.
    `cargo doc -p happenstance --no-deps` green.
  mount_point: "crates/happenstance/src/lib.rs — module doc regions 4 and 5 of `crate-root-rustdoc`, plus the root pub use block"
  verifying_test: "crates/happenstance/tests/doc_surface.rs::crate_root_renders_the_projection_surface + `cargo doc -p happenstance --no-deps`"

- id: AC-010
  criterion: >-
    GIVEN the same evaluator reading the published docs.rs page rather than the source, WHEN they
    open `Projection` or `run_projection`, THEN each carries a `doc_cfg` badge naming
    `unstable-projection` (the design's anti-pattern 15 is the failure this forbids) —
    `[package.metadata.docs.rs]` and `#![cfg_attr(docsrs, feature(doc_cfg))]` present, added by this
    story if M3 did not add them — and the crate-root page renders complete and warning-free with the
    feature off, which is its default state: no intra-doc link resolves in only some configurations,
    and rustdoc treats a broken one as a hard error rather than a warning
    (RS-70-2/RS-70-4/RS-51-5).
  satisfied: true
  evidence: >-
    crates/happenstance/src/lib.rs:233-235 — `#[cfg(feature = "unstable-projection")]` plus
    `#[cfg_attr(docsrs, doc(cfg(feature = "unstable-projection")))]` on the four re-exports, and
    :192-193 on the module itself. `[package.metadata.docs.rs]` (all-features, `--cfg docsrs`) and
    `#![cfg_attr(docsrs, feature(doc_cfg))]` present. The intra-doc link is reference-style with the
    target spelled conditionally at :169-176, so it resolves with the feature on AND off — RS-70-2,
    and the reason an inline `](run_projection)` is impossible from an ungated page. Verified:
    `cargo doc -p happenstance --no-deps` green; the same `--no-default-features` green;
    `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc -p happenstance --no-deps
    --all-features` green. Tests: manifest_contract.rs::docs_rs_metadata_is_declared,
    docs_composition.rs::docsrs_metadata_is_present. PASS.
  mount_point: "crates/happenstance/Cargo.toml [package.metadata.docs.rs] + crates/happenstance/src/lib.rs (#![cfg_attr(docsrs, feature(doc_cfg))] and the gated doc lines)"
  verifying_test: "crates/happenstance/tests/manifest_contract.rs::docs_rs_metadata_is_declared + `cargo doc -p happenstance --no-deps` (default and --no-default-features) + `cargo +nightly doc -p happenstance --no-deps --all-features` under RUSTDOCFLAGS=\"--cfg docsrs -D warnings\""

- id: AC-011
  criterion: >-
    GIVEN the edge developer (P3) whose target is `wasm32-unknown-unknown` and whose futures are not
    `Send`, WHEN this crate's first port-binding generic code lands, THEN it binds `EventStore` and
    never `SendEventStore` — the weaker requirement, which accepts both flavours — imports one
    flavour name per module and reaches the other by full path, introduces no `#[async_trait]`,
    leaves `EventStore::read` non-`async` with both memory.rs tests that pin its shape untouched, and
    leaves `pub use happenstance_core::*;` (lib.rs:75) surviving with nothing shadowing a contract
    name: `ProjectionId`, `ProjectionStore`, `Query`, `ReadOptions` and `SequencePosition` all arrive
    through the glob and are used, not redefined.
  satisfied: true
  evidence: >-
    crates/happenstance/src/runner.rs:404-410 — `S: EventStore`, the bare flavour; `SendEventStore`
    and `SendProjectionStore` are named nowhere in the file, there is no `#[async_trait]`, and
    `EventStore::read` is untouched. The private module is `runner`, not `projection`, precisely so
    `happenstance::projection` still resolves to the contract's module through the glob at
    lib.rs:237 — witnessed by crates/happenstance/src/tests.rs shadowing::same_projection_id, a
    type-equality body that stops compiling if it moves back. Verified: `cargo check -p happenstance
    --target wasm32-unknown-unknown --no-default-features --features std,json,unstable-projection`
    green; doc_surface.rs::no_contract_name_is_shadowed,
    doc_budget.rs::the_glob_reexport_survives_and_nothing_shadows_it and
    src/tests.rs::contract_names_are_not_shadowed PASS; `git status --porcelain --
    crates/happenstance-core/src` empty.
  mount_point: "crates/happenstance/src/lib.rs (run_projection's generic signature and the surviving glob re-export at :75)"
  verifying_test: "`cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json,unstable-projection` + crates/happenstance/tests/doc_surface.rs::no_contract_name_is_shadowed + empty `git diff --stat main -- crates/happenstance-core/src`"

- id: AC-012
  criterion: >-
    GIVEN a maintainer who must be able to believe the atomicity claim rather than take it on trust,
    WHEN the transactional test runs, THEN it runs against HS-P0010's real `MemoryProjectionStore`
    behind the `memory` feature — never a throwaway `ProjectionStore` written inside this crate,
    which would freeze a fixture shape this project explicitly does not own — and if that
    cross-project dependency has not landed when this story is picked up, the story halts loudly,
    naming the missing artefact, rather than substituting one.
  satisfied: true
  evidence: >-
    crates/happenstance/tests/projection_runner.rs:26-32 imports `MemoryProjectionStore`,
    `MemoryProjectionBatch` and `MemoryProjectionStoreError` from HS-P0010's crate through the
    facade glob — the preflight found them already landed at
    crates/happenstance-core/src/projection_memory.rs:101, :209 and :268 behind `all(feature =
    "memory", feature = "unstable-projection")`, so EC-009 did not fire and nothing was substituted.
    The absence half: doc_surface.rs::no_local_projection_store scans every `.rs` under
    crates/happenstance/src and tests/ and finds no `impl ProjectionStore for` and no `impl
    SendProjectionStore for`. PASS.
  mount_point: "crates/happenstance/tests/projection_runner.rs (the fixture import) — and, as an absence, crates/happenstance/ carrying no `impl ProjectionStore for`"
  verifying_test: "crates/happenstance/tests/projection_runner.rs importing MemoryProjectionStore from the crate that owns it + crates/happenstance/tests/doc_surface.rs::no_local_projection_store"
```
