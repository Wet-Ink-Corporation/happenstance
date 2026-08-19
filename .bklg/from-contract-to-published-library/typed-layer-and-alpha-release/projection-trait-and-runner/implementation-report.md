---
item: "HS-S0026"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — The application-facing Projection trait and its streaming runner

**All twelve ACs are satisfied. Nothing is blocked and nothing is deferred.**

`happenstance::Projection`, `happenstance::run_projection`, `happenstance::Progressed` and
`happenstance::ProjectionError` are real, re-exported at the crate root behind a real
`unstable-projection` feature that is off by default. The runner reads once, pulls the
stream item by item, applies at most `chunk` decoded events into one open write set, and
hands that write set and the position of the last applied event to the port's **single**
`commit`. A failure discards the chunk whole through `rollback` and comes back as a typed
error carrying the position it stopped at and the progress it had already made durable.

The preflight EC-009 asks about first did **not** fire: HS-P0010's `MemoryProjectionStore`
had already landed at `crates/happenstance-core/src/projection_memory.rs:101`, behind
`all(feature = "memory", feature = "unstable-projection")`, together with
`MemoryProjectionBatch` (`:209`) and the uninhabited `MemoryProjectionStoreError` (`:268`).
Every transactional assertion in this story runs against it. No `ProjectionStore` was
written inside `crates/happenstance/`, and `doc_surface.rs::no_local_projection_store`
asserts the absence rather than leaving it to a reviewer's memory.

## TDD Evidence

The red was staged in the shape this slice's earlier stories used, because a Rust API that
does not exist yet fails to *compile* rather than fails an *assertion*, and a compile error
is not evidence about behaviour.

**Stage one — the surface, inert.** `crates/happenstance/src/runner.rs` landed first with
every signature the design fixes, the feature declared and forwarded, and the four items
re-exported at the crate root — but with a body that derived the query, ignored the store
and returned `Progressed::default()`. The crate root was untouched. Then:

```console
$ cargo test -p happenstance --features unstable-projection,memory --test projection_runner
test result: FAILED. 1 passed; 8 failed
```

Every one of the eight failed on the missing *behaviour*, not on a name:

| AC | test | the red it produced |
| --- | --- | --- |
| AC-001 | `derived_query_matches_event_types_and_scope` | `the runner read once, not once per chunk: left: 0 right: 1` |
| AC-002 | `apply_receives_decoded_domain_events` | `apply` was handed nothing at all |
| AC-003 | `read_model_and_checkpoint_commit_together` | no rows, no checkpoint |
| AC-004 | `first_run_starts_from_the_beginning` | `left: 0 right: 2` |
| AC-004 | `resume_advances_past_the_checkpoint` | `left: [] right: [Dispatched { .. }]` |
| AC-004 | `tolerates_gapped_positions` | `left: 0 right: 2` |
| AC-005 | `commits_before_the_stream_ends` | `every seeded event was pulled: left: 0 right: 5` |
| AC-006 | `decode_failure_names_its_position_and_rolls_back` | the poisoned event did not stop the runner |

The one test that **passed** in stage one is the one that must:
`checkpoint_without_rows_is_rejected`, which drives its own deliberately wrong runner and
never calls `run_projection` at all. That is AC-003's discriminator, and its passing at red
is what proves the atomicity oracle is not decorative — see below.

**Stage two — green.** The chunk loop replaced the inert body
(`crates/happenstance/src/runner.rs:401-513`), and all nine passed:

```console
$ cargo test -p happenstance --features unstable-projection,memory --test projection_runner
test result: ok. 9 passed; 0 failed
```

**The named wrong implementation.** AC-003 is not discharged by
`read_model_and_checkpoint_commit_together` passing. It is discharged by
`checkpoint_only_runner` (`crates/happenstance/tests/projection_runner.rs:412-441`) — a
runner that opens a batch, applies the decoded event into it, then *drops it* and carries
the checkpoint forward on a fresh empty one. `checkpoint_without_rows_is_rejected` asserts
that the same `rows_and_checkpoint_agree` oracle the real runner satisfies **fails** against
it. Remove the oracle's rigour and that test goes red; a rule no implementation can fail is
decorative, and this one can be failed.

**The surface's own red.** `crate_root_renders_the_projection_surface`,
`no_second_decode_path` and `runner_prints_nothing` were red against the untouched crate
root and went green when the fifth vocabulary bullet became a link, the Features table
gained its row, and the roadmap paragraph was deleted. An uncomposed render — four bare
`pub use`s under a surviving `*(planned)*` bullet — compiles, passes every runtime
assertion above, and fails all three.

## Commits

One checkpoint commit, carrying the whole story:

```
feat(typed-layer-and-alpha-release): Projection trait and runner
Story: typed-layer-and-alpha-release/projection-trait-and-runner
```

Its short SHA is reported in this run's slice digest and is recoverable here with
`git log --grep "Story: typed-layer-and-alpha-release/projection-trait-and-runner" --oneline`
— quoting it in the body would be a hash the commit that carries the body cannot
contain.

## Changes

| file | shape of the change |
| --- | --- |
| `crates/happenstance/src/runner.rs` | **New, 616 lines.** `Projection` (`:62`), `Progressed` (`:115`), `ProjectionError` (`:142`), `run_projection` (`:401`), and three private helpers: `apply_one` (`:517`), `next_event` (`:594`) and `considered_through` (`:607`). |
| `crates/happenstance/src/lib.rs` | `mod runner;` gated at `:192`; the four re-exports with their `doc_cfg` badge at `:233`; the fifth vocabulary bullet rewritten in place at `:106`; the `unstable-projection` Features row at `:125`; the conditional reference-link target at `:169-176`; the `*(planned)*` paragraph deleted. |
| `crates/happenstance/Cargo.toml` | `unstable-projection` in `[features]`, forwarding to `happenstance-core/unstable-projection` and `dep:futures-core`; `futures-core` promoted from dev-only to an **optional** normal dependency. |
| `crates/happenstance/src/tests.rs` | `shadowing::same_projection_id`, the type-equality witness that `happenstance::projection` still names the contract's module. |
| `crates/happenstance/tests/projection_runner.rs` | **New.** One two-variant domain, one projection, two instrumented event stores, nine tests, and the wrong runner. |
| `crates/happenstance/tests/doc_surface.rs` | Four absence assertions and the crate-root composition assertion. |
| `crates/happenstance/tests/manifest_contract.rs` | `unstable_projection_is_declared_off_by_default`. |
| `xtask/src/main.rs` | `tests::the_typed_layer_makes_no_promise_it_does_not_keep` rewritten — see *Notes*. |

## Gates

Every command below was run from the worktree root on the committed tree.

| command | result |
| --- | --- |
| `cargo test -p happenstance --features unstable-projection,memory` | green — 42 lib, 9 `projection_runner`, 10 `doc_surface`, 8 `manifest_contract`, and the rest |
| `cargo test -p happenstance --doc --features unstable-projection` | green — 9 doctests including `runner::run_projection` |
| `cargo clippy -p happenstance --all-targets --all-features -- -D warnings` | green |
| `cargo hack check --feature-powerset -p happenstance` | green (**not** run by `--fast`) |
| `cargo doc -p happenstance --no-deps` | green |
| `cargo doc -p happenstance --no-deps --no-default-features` | green |
| `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc -p happenstance --no-deps --all-features` | green |
| `cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json,unstable-projection` | green |
| `cargo +1.97.1 check -p happenstance --all-features` | green |
| `cargo fmt --all --check` | green |
| `cargo xtask affected --base main` | **affected gate passed** |
| `cargo xtask ci --fast` | **all required checks passed (--fast: 4 optional step(s) not run)** |

A green `--fast` alone discharges neither AC-008 nor AC-010; the powerset and the three doc
builds above are what discharge them, and they were run by hand.

**Boundary, asserted rather than assumed.** `git status --porcelain -- spec/
crates/happenstance-core/src crates/happenstance-testkit/src experiments/` is empty. The one
path inside `xtask/src` that changed is explained under *Notes*.

## PS-33 / PS-27 / PS-30 evidence

Written as counts over the tree, for HS-S0027 to adjudicate. Nothing here is a verdict; no
maturity marker moved and `spec/SPECIFICATION.md` is untouched.

**PS-33 — ADR-0007's falsifier.** ADR-0007 allocates a checkpoint pump to
`happenstance-core`. **There is no such function in the tree, and this story did not write
one.** `grep -rn "^pub fn |^pub async fn " crates/happenstance-core/src/` returns exactly
two free functions in the whole contract crate:

```
crates/happenstance-core/src/store.rs:285:pub async fn collect<S, T, E>(stream: S) -> Result<Vec<T>, E>
crates/happenstance-core/src/store.rs:321:pub async fn read_decision_model<S>(
```

Neither is a checkpoint pump. `grep -rn "pump" crates/ --include=*.rs` returns exactly one
hit across the workspace, and it is a doc comment in this story's own
`crates/happenstance/src/runner.rs:134` explaining why a pump signature could not represent a
decode failure. So the independent-caller count for the core-side pump is **zero, over a
function that does not exist** — the count is unavailable because the subject is absent, not
because nobody looked. `happenstance::run_projection` drives the port's four methods
directly. The two admissible verdicts remain exactly the two the architecture brief names,
and both are HS-S0027's to write.

**PS-27 — a per-runner failure policy.** None was offered. `run_projection` takes five
arguments and none of them is a policy; the runner halts on the first failure and the
exclusion is stated on its own page at `crates/happenstance/src/runner.rs:305-313`, naming
`on_error: SkipPolicy` as the alternative that lost and why. `grep -rn "SkipPolicy|on_error"
crates/ --include=*.rs` finds that doc comment and nothing else. Failure policy is per
projection; skip-and-record is PS-27's subject and HS-P0010's suite rule, and neither is
pre-empted here.

**PS-30 — the fan-out runner.** **Not built.** One `run_projection` call drives one
projection. The reason is now stronger than the brief anticipated: the port's `Batch` stopped
being a GAT and became a plain **owned** associated type
(`crates/happenstance-core/src/projection.rs:436`), so the obstacle is no longer only a
lifetime — an owned write set cannot be shared between tasks at all, and moving one into a
`tokio::spawn` would additionally require `Send`, which the flavour that exists for `wasm32`
cannot promise. `grep -rn "spawn" crates/happenstance/src` finds one hit, the doc comment at
`runner.rs:319` saying so. N read models cost N independent reads, which is the number
`polling-cost-measurement` records.

**PS-18** is not evaluated here and is not evaluable from this story: its subject *does* now
exist — `reset` and `ResetError::Refused` landed with HS-P0010
(`crates/happenstance-core/src/projection.rs:497`, `:270`) — which is a change from what this
spec's clarification 7 assumed, and HS-S0027 should re-read the tree before recording the
exclusion this story's spec expected.

## Notes

**One file outside the declared PR boundary changed, and it had to.**
`xtask/src/main.rs`'s `tests::the_typed_layer_makes_no_promise_it_does_not_keep` asserted
`!manifest.contains("unstable-projection")` for `crates/happenstance/Cargo.toml`, under a doc
comment whose stated premise was *"`happenstance` re-exports no projection item, so a public
feature there would promise a surface the crate does not expose."* This story makes that
premise false. The assertion was therefore **rewritten, not weakened**: it now asserts the
passthrough and the surface agree in *both* directions (`assert_eq!(passthrough, surface)`),
and, when the passthrough is present, that it is off by default, that it forwards to the
contract crate, and that the module is gated. It still fails on a feature table advertising a
runner nobody can name, and it now also fails on four `pub use`s a consumer cannot turn on —
which the old form did not catch. The spec's exclusion of `xtask/src/main.rs` is about
*registering the fifth `wasm32` step*, which is HS-S0031's and was not touched.

**`futures-core` became an optional dependency, and NF-005 says it should not have.** Naming
`Stream` is unavoidable for a runner that pulls a replay instead of buffering it: the trait
has to be in scope to call `poll_next`, `happenstance-core` does not re-export it, and
re-exporting it from there would be an edit under `crates/happenstance-core/src/**`, which
AC-A02 forbids outright. So the feature turns on `dep:futures-core`. **It adds no node to any
dependency graph** — `happenstance-core` already depends on `futures-core` unconditionally
and this crate already depends on `happenstance-core` unconditionally, so `cargo tree` is
byte-identical — which is the test RS-50-1 actually states. It is optional rather than
unconditional so that the feature adds exactly what it needs and a build without
`unstable-projection` links nothing new.

**The private module is `runner`, not `projection`, and that was a near-miss worth recording.**
`mod projection;` compiled and produced a `hidden_glob_reexports` warning:
`happenstance_core::projection` is a public module, the crate root's
`pub use happenstance_core::*;` re-exports it, and a private module of the same name shadows
it — so `happenstance::projection::ProjectionStore` would have silently stopped resolving.
That is anti-pattern 14 arriving through a *module* name rather than a type name, which
nothing in the spec anticipated. The module was renamed and a type-equality witness added at
`crates/happenstance/src/tests.rs` so the mistake cannot come back quietly.

**The design's signature is honoured with one mechanical difference.** `_design.md:553`
writes `batch: &mut <Self::Store as ProjectionStore>::Batch<'_>`. The port's `Batch` is no
longer a generic associated type — HS-P0010 made it a plain owned associated type
(`crates/happenstance-core/src/projection.rs:436`, with the reason on the module doc at
`:74-96`) — so the lifetime has nowhere to go and the signature is
`&mut <Self::Store as ProjectionStore>::Batch`. Nothing else about the shape moved.

**One intra-doc link could not be inline, and the spec's own AC-010 is why.** AC-009's
verification suggested an inline `](run_projection)` on the vocabulary bullet. That is a
*hard* rustdoc error in the default configuration, where `unstable-projection` is off and the
item does not exist — `broken_intra_doc_links = "deny"` at the workspace root, RS-70-2, and
exactly the failure `_design.md`'s mock caught. The bullet is therefore a reference-style
link with its target spelled conditionally at `lib.rs:169-176`, the shape the command-loop
bullet already uses; the test asserts both spellings so neither configuration can rot.

**No experiment, no spec edit, no conformance rule.** `experiments/` is untouched
(HS-S0028's), `spec/SPECIFICATION.md` is untouched (HS-S0027's), and
`crates/happenstance-testkit/src/suite.rs` is untouched — a rule about a layer *above* the
port is one no adapter can fail.
