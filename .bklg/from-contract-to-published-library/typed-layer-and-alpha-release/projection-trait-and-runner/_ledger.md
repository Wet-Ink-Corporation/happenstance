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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (crate root pub use, beside `pub use happenstance_core::*;`)"
  verifying_test: "crates/happenstance/tests/projection_runner.rs::derived_query_matches_event_types_and_scope (cargo test -p happenstance --features unstable-projection,memory)"

- id: AC-002
  criterion: >-
    GIVEN that same author, whose events are a Rust enum and not `Bytes`, WHEN the runner reads an
    event out of the store, THEN `apply` receives a decoded `Self::Event` produced by M3's `Codec` —
    the decode half ADR-0007 assigned to the typed layer — and the author writes no
    `serde_json::from_slice`, no manual match on `EventType`, and no second decode path anywhere in
    their projection.
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs (run_projection's resume arithmetic, over ProjectionStore::checkpoint and ReadOptions::from)"
  verifying_test: "crates/happenstance/tests/projection_runner.rs::resume_advances_past_the_checkpoint + ::first_run_starts_from_the_beginning + ::tolerates_gapped_positions (against happenstance_testkit::GappyMemoryStore)"

- id: AC-005
  criterion: >-
    GIVEN an author rebuilding a read model over a log far larger than memory, WHEN they call
    `run_projection` with a `chunk: NonZeroUsize`, THEN memory stays flat in the length of the
    replay: the runner never `collect`s the stream, commits happen while the stream is still being
    pulled, and the only buffer is the at-most-`chunk` events between one `begin` and its `commit` —
    which is the whole reason `EventStore::read` returns the stream at the top level and is not
    `async` (ADR-0001/ADR-0008), and the precondition for E2E-25's chunked rebuild.
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/tests/projection_runner.rs (the fixture import) — and, as an absence, crates/happenstance/ carrying no `impl ProjectionStore for`"
  verifying_test: "crates/happenstance/tests/projection_runner.rs importing MemoryProjectionStore from the crate that owns it + crates/happenstance/tests/doc_surface.rs::no_local_projection_store"
```
