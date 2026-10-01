# `experiments/provided-method-spike`

Can a **provided** method be added to a `#[trait_variant::make(SendX: Send)]`
port after `0.3.2` without breaking anything? This question sits under phase 17's
ADR-0028 fork. The retention brief recommends that any future port-level report
of what a store has forgotten arrive as a provided method on `EventStore` whose
default is `Unknown`, so that it can land in any 1.x minor. It also says that
recommendation must rest on a compiling spike rather than on reasoning. This
directory is that spike.

It is not a crate. Nothing here is a workspace member or a gate step, and no
manifest or `Cargo.lock` changed. The spike ran on a scratch `git worktree` of
`52aa951`, so the main tree's crates were never edited. What survives is the
patch ([`spike.diff`](spike.diff)) and the raw transcripts
([`results/`](results/)).

## The question, and the pass criterion stated before anything ran

A method is written in the spelling ES-4 requires. That spelling is **not**
`async fn` with a body, because `trait_variant` 0.1.3's `transform_item` copies
the default block into the variant and only sets `asyncness: None`
(`trait-variant-0.1.3/src/variant.rs:129-168`):

```rust
fn name(&self, ..) -> impl Future<Output = Result<T, Self::Error>> {
    async { .. }
}
```

A use **passes** only if all three of these hold, and it **fails** if any one
does not:

1. **Implementors.** Every in-tree implementor still compiles unchanged. That
   includes Send and `!Send` implementors, adapters, testkit mutants, examples and
   benches, under `cargo check --workspace --all-features --all-targets` and the
   gate's wasm32 checks.
2. **Generic spawn.** Code generic over the trait can call the method, and code
   generic over the `Send` flavour can `tokio::spawn` it **without a
   `Self: Sync` bound**.
3. **Semver.** `cargo semver-checks check-release --baseline-version 0.3.2`
   reports no major finding that is not already present on the unmodified tree.

## The three uses

| # | Where | The provided method | Default |
| --- | --- | --- | --- |
| 1 | `EventStore` (core, trait_variant pair) | `fn history(&self) -> impl Future<Output = Result<HistoryReport, Self::Error>>` | `Ok(HistoryReport::Unknown)`; `HistoryReport` is a new `#[non_exhaustive]` enum `{ Unknown, Complete }` |
| 2 | `ProjectionStore` (core, trait_variant pair) | `fn commit_all(&self, batch: Self::Batch, checkpoints: &[(ProjectionId, SequencePosition)], authority: Authority) -> impl Future<Output = Result<(), CommitError<Self::Error>>>` | refuses with a new `CommitError::Unsupported` variant (`CommitError` is already `#[non_exhaustive]`) |
| 3 | `happenstance::Projection` (typed layer, behind `unstable-projection`) | `fn on_error(&mut self, at: SequencePosition) -> impl Future<Output = OnError>` | `OnError::Stop`; `OnError` is a new `#[non_exhaustive]` enum `{ Stop, Skip }` |

**`Projection` is not a trait_variant pair.** It is a plain trait
(`crates/happenstance/src/runner.rs:62-100`). Use 3 therefore tests the pattern
on a plain trait, as the brief asked. It can only meet criterion 2 in the weak
form, and the results show why.

The names are a spike's names and are not proposals. The spike also adds two
test files, one per crate, and both are in `spike.diff`.
`crates/happenstance-core/tests/provided_method_spike.rs` does three things:

- It implements both core ports on `!Send` and `!Sync` stores (`Rc<Cell<_>>`
  fields) using required methods only.
- It calls the defaults from `S: EventStore` and `S: ProjectionStore` code.
- It spawns them from functions bound only on `S: SendEventStore + 'static` and
  on `S: SendProjectionStore + 'static, S::Batch: Send`. Neither bound includes
  `Sync`.

## Conditions

| | |
| --- | --- |
| Tree | `52aa951` (branch `lane/p17-provided-method-spike`), as a scratch `git worktree` |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, host `x86_64-pc-windows-msvc` |
| `trait-variant` | 0.1.3 (the version `tests/trait_variant_pin.rs` pins) |
| `cargo-semver-checks` | 0.50.0, baseline = the published `0.3.2` from crates.io |
| Build | `CARGO_BUILD_JOBS=4`, `CARGO_TARGET_DIR` = the main tree's `target/` (shares registry artefacts; the worktree's crates are distinct package ids) |

All three uses are in the tree at once, and every result below was taken against
that combined state. Absolute paths in `results/` are redacted to `<worktree>`,
`<target>`, `<repo>`, `<cargo-registry>` and `~`, for the reason `.gitignore`
gives for `*.log`. Nothing else in the transcripts was edited.
`git check-ignore -v` matches none of the files here. The names avoid
`*-output.txt` and `*.log` on purpose (see
`.kb/open-questions/experiment-raw-output-eaten-by-the-ignore-rule.md`).

## Results

### The semver baseline, and a control that proves the check can fail

| run | `happenstance-core` | `happenstance` |
| --- | --- | --- |
| unmodified `52aa951` vs `0.3.2` | 196 pass, 58 skip, **no finding** ([`semver-before-core.txt`](results/semver-before-core.txt)) | 196 pass, 58 skip, **no finding** ([`semver-before-happenstance.txt`](results/semver-before-happenstance.txt)) |
| spike applied vs `0.3.2` | 196 pass, 58 skip, **no finding** ([`semver-after-core.txt`](results/semver-after-core.txt)) | 196 pass, 58 skip, **no finding** ([`semver-after-happenstance.txt`](results/semver-after-happenstance.txt)) |
| **control:** spike + one *required* `async fn control_required` on `EventStore` | **1 major: `trait_method_added`** on `EventStore` and `SendEventStore` ([`semver-control-required-method.txt`](results/semver-control-required-method.txt)) | not run |

The baseline has no noise to separate out: `main` at `52aa951` is clean against
`0.3.2` in both crates. The clean after-run is a real result, because the control
shows that the same invocation on the same worktree fails on the rejected
alternative, which is a required method. The control ran and was then reverted,
so it is not in `spike.diff`.

### Per use

| criterion | use 1: `EventStore::history` | use 2: `ProjectionStore::commit_all` + `CommitError::Unsupported` | use 3: `Projection::on_error` (plain trait) |
| --- | --- | --- | --- |
| 1. implementors compile | **pass** | **pass**, only in the second spelling (see below). The one in-crate exhaustive match needed an arm | **pass**, for every `impl Projection` in the workspace (tests and examples). `benchmarks/` and `experiments/polling-cost/`, which also implement it, are not workspace members and were not checked |
| 2. generic call | **pass**, `S: EventStore` on a `!Send` store and on `MemoryEventStore` | **pass**, `S: ProjectionStore` on both. A refused `commit_all` leaves the checkpoint at `NeverRun` | **pass**, `P: Projection` |
| 2. generic spawn, no `Sync` | **pass**, `S: SendEventStore + 'static` | **pass**, `S: SendProjectionStore + 'static, S::Batch: Send` | **fail**, by construction. `P: Projection + Send + 'static` does not compile: *"future cannot be sent between threads safely … `impl Future<Output = OnError>` … is not `Send`"* ([`projection-generic-spawn-negative.txt`](results/projection-generic-spawn-negative.txt)). A *concrete* `P` spawns, but only through auto-trait leakage, and that proves nothing |
| 3. semver vs `0.3.2` | **pass** | **pass**. The new variant is not reported, because the enum is `#[non_exhaustive]` | exempt, since `unstable-projection` makes no semver promise (ADR-0066). The facade's new `HistoryReport` re-export is checked and passes |
| **verdict** | **PASS** | **PASS, with two conditions** | **not a trait_variant pair: callable, not generically spawnable** |

### Whole-tree checks (combined state)

| command | result | transcript |
| --- | --- | --- |
| `cargo check --workspace --all-features --all-targets` | exit 0. All 17 members checked (7 published crates, sync, ladybug, 7 examples, xtask), no warnings | [`workspace-check.txt`](results/workspace-check.txt) |
| `cargo check --locked -p happenstance-core --target wasm32-unknown-unknown --no-default-features --features std` | exit 0 | [`wasm-checks.txt`](results/wasm-checks.txt) |
| `cargo check --locked -p happenstance-core --target wasm32-unknown-unknown` | exit 0 | ″ |
| `cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown` | exit 0 | ″ |
| `cargo check --locked -p happenstance-cloudflare --tests --target wasm32-unknown-unknown` (the workspace's real `!Send` adapter, on the target where it is `!Send`) | exit 0 | ″ |
| `cargo check --locked -p happenstance-neon --target wasm32-unknown-unknown` | exit 0 | ″ |
| `cargo clippy -p happenstance-core -p happenstance --all-features --all-targets -- -D warnings` | exit 0. Neither `clippy::manual_async_fn` nor `async_fn_in_trait` fires on the spelling | [`clippy.txt`](results/clippy.txt) |
| `cargo fmt -p happenstance-core -p happenstance -- --check` | clean after `cargo fmt` | none |
| `cargo test -p happenstance-core --all-features` | exit 0. 201 passed, 0 failed, across 13 binaries, including the spike's 3 and ES-3/ES-4's `provided_method_future_is_send_in_generic_code` | [`core-tests.txt`](results/core-tests.txt) |
| `cargo test -p happenstance --all-features --no-fail-fast` | exit 101. 179 passed, **1 failed**: `no_skip_and_record_path_is_offered` (see below) | [`happenstance-tests.txt`](results/happenstance-tests.txt) |

The `!Send` implementors this covers are the two in the spike's own test file,
`crates/happenstance/tests/flavours.rs`'s `LocalStore` (a bare `impl EventStore`
with no `+ Send` on its stream), and `happenstance-cloudflare` on wasm32. The
same workspace check also compiles the bare-flavour generic wrapper
`FaultyStore<S: EventStore>` and every other implementor in the workspace:
`MemoryEventStore`, `MemoryProjectionStore`, sqlite, postgres (both projection
stores), neon, ladybug, `SendFaultyStore`, the testkit fixtures and the
mutation-coverage mutants.

## What the spike found beyond pass/fail

**1. A default body must not move an owned associated-type value into its
future.** The first spelling of `commit_all` was `async move { drop(batch);
Err(CommitError::Unsupported) }`. It fails to compile in the *derived* flavour,
because `Self::Batch` carries no `Send` bound (projection.rs states this on
purpose). See [`commit-all-captures-batch.txt`](results/commit-all-captures-batch.txt):

```text
error: future cannot be sent between threads safely
  = help: within `{async block@…}`, the trait `Send` is not implemented for `<Self as SendProjectionStore>::Batch`
help: consider further restricting the associated type
429 | #[trait_variant::make(SendProjectionStore: Send where <Self as SendProjectionStore>::Batch: Send)]
```

rustc's suggested fix is an **attribute-level** bound. That is the same
one-keystroke trap ES-3 names for `Self: Sync`: it would push `Batch: Send` onto
the bare flavour's wasm32 implementors. The spelling that compiles drops the
batch *before* the future exists and returns `async { Err(..) }`. That is sound
because the port already says that dropping a batch bare is a rollback. The
general rule has the same shape as ES-3. **A provided body's future must capture
nothing whose auto-traits the trait does not bound.** That means no `&self`
without `where Self: Sync`, and no `Self::Batch` at all. ES-3 states only the
first half.

**2. `CommitError::Unsupported` is additive to semver-checks, but not free.**

- **It breaks an exhaustive match inside the defining crate.** `#[non_exhaustive]`
  does not apply within `happenstance-core`, and AC-003's test
  `commit_and_reset_errors_stay_separate` (projection.rs:848) matches
  `CommitError` exhaustively. It needed one new arm. That is expected and costs
  one line, but it is a change.
- **It contradicts the reason `CommitError` and `ResetError` are separate.** They
  were split so that "a caller matching `commit`'s result never has to consider
  `Refused`, which `commit` cannot produce" (projection.rs:265-270). Adding
  `Unsupported` to `CommitError` gives `commit` callers a variant `commit` cannot
  produce. By the port's own reasoning, a refusing `commit_all` wants its own
  error type rather than a variant on `CommitError`. **This is recorded, not
  decided.**

**3. A new public type in core costs an edit in the facade.** `happenstance`'s
`contract_surface.rs:266` requires every unconditional core item to be
re-exported, so `HistoryReport` failed it until `happenstance/src/lib.rs` gained
`pub use happenstance_core::HistoryReport;`. `doc_surface.rs:139` asserts a
literal `use` line, so the re-export has to go on its own line. A provided method
that introduces a type is therefore a change to two crates. Neither is a semver
break.

**4. Use 3 trips PS-27's tripwire, by design.**
`projection_clauses.rs:284`'s `no_skip_and_record_path_is_offered` fails with
*"the runner grew `on_error`; PS-27's verdict is now wrong."* That is the guard
doing its job, not a mechanism failure. Any real `on_error` has to move PS-27's
verdict in the same change. The spike's first doc sentence on `on_error` also
exceeded the typed layer's 80-character first-sentence budget (`tests.rs:633`),
and it was shortened. That was about the spike's own prose.

**5. On a plain trait, a provided `-> impl Future` has no `Send` flavour to
reach for.** Generic `P: Projection` code can await `on_error`, but it cannot
spawn it, and no bound a caller can write fixes that on 1.97.1. This does not
reduce what `run_projection` offers today, because its generic future is already
not `Send` for a bare `S: EventStore`, and flavours.rs spawns it only with a
concrete `P`. It is still direct evidence for the projection-apply brief's
recommendation. If the typed `Projection` gains any async member, provided or
required, it wants the same `#[trait_variant::make(SendProjection: Send)]`
derivation the ports use.

## Verdict

**Use 1 passes all three criteria.** A provided `EventStore` method defaulting to
`Unknown` compiles for every Send and `!Send` implementor in the tree. That
includes wasm32. It can be called from `S: EventStore` code and spawned from
`S: SendEventStore + 'static` code with no `Sync`. `cargo-semver-checks` 0.50.0
reports nothing against `0.3.2`, and the positive control shows that it would
have reported a required method.

**Use 2 passes, with two conditions.** The default must drop the batch before the
future exists. Adding a variant to an existing `#[non_exhaustive]` error is
semver-additive, but it costs an in-crate match arm and contradicts the
`CommitError`/`ResetError` separation. A separate error type is the likely
better shape.

**Use 3 is the plain-trait case and shows the limit.** The provided method
compiles and is callable, but it is not spawnable from generic code, and a real
`on_error` also has to move PS-27.

## What it licenses, and what it does not

**It licenses:**

- ADR-0028 can choose "no required method in 0.4.0; a later report arrives as a
  provided method defaulting to `Unknown`" on compiled evidence. The mechanism
  does not force the fork to collapse to "required in 0.4.0" or "only in 2.0".
- The same holds for an additive, refusing provided method on
  `ProjectionStore`, under the spelling in finding 1.

**It does not license:**

- **Any of the spike's names or signatures.** `HistoryReport`, `commit_all`'s
  parameter shape and `OnError` were written to be compiled, not to be adopted.
  No adapter overrode any of the methods, so the override path is exercised only
  by trait_variant's blanket impl forwarding to the `Send` flavour's copy of the
  default.
- **"semver-checks is silent" as "no consumer can break".** 58 lints were
  skipped. The tool cannot see a default that lies; a `Complete` default would
  pass it. Cargo's semver reference also classes a new defaulted trait item as a
  *possibly-breaking* minor change. A downstream crate with another trait in
  scope that has a method of the same name gets `E0034` ambiguity, the collision
  `EventStore` already carries a `doc(alias = "E0034")` for. The spike did not
  measure that. Choosing a method name is choosing that risk.
- **Anything about `happenstance-testkit`'s semver.** It was not modified and not
  checked.
- **Anything about the MSRV job or the powerset.** The pin equals the floor, so
  the same compiler ran. `cargo hack` and the docs builds were not run: memory on
  this machine is tight, and the full gate was deliberately not run.

## Reproducing

```console
git worktree add <scratch>/spike-wt 52aa951      # -c core.longpaths=true on Windows
cd <scratch>/spike-wt
git apply <repo>/experiments/provided-method-spike/spike.diff
export CARGO_BUILD_JOBS=4
cargo semver-checks check-release -p happenstance-core --baseline-version 0.3.2
cargo semver-checks check-release -p happenstance      --baseline-version 0.3.2
cargo check --workspace --all-features --all-targets
cargo test  -p happenstance-core --all-features --test provided_method_spike
cargo test  -p happenstance --features unstable-projection,json --test provided_method_spike
cd - && git worktree remove --force <scratch>/spike-wt
```

The `before` runs are the same two `semver-checks` commands without the
`git apply`. The two negative transcripts come from variants that are not in
`spike.diff`. `commit-all-captures-batch.txt` is `commit_all` written as `async
move { drop(batch); … }`. `projection-generic-spawn-negative.txt` comes from a
test file holding only a `fn spawn_generic<P: Projection + Send + 'static>`, and
the file was deleted after the run.
