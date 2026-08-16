---
item: "HS-S0031"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — The typed layer's wasm32 claim, stated either way

**All seven ACs are satisfied. Nothing is blocked and nothing is deferred.**

The story split cleanly along the line D2 predicted. The gate step is the cheap half and
passed on the first try, with `Cargo.lock` unmoved — `_design.md:648`'s cost assumption
held and EC-006 did not fire. The `!Send` instantiation is the half that could fail, and
one of its two halves did: `run_projection` could not be spawned from generic code at all,
for a structural reason nobody had written down. That is EC-004 working, and it is the
finding this story was worth writing.

Two things landed that the spec did not anticipate, both recorded below rather than
absorbed: `cargo deny check bans` was **already red** on this tree before the ban was
added, and the ban's own first run found `async-trait` already resolved through
`wasm-bindgen-test`.

## TDD Evidence

Seven criteria, four instruments, and one of them found something. Every row below names
the red, what it asserted, and the green.

### AC-001 — the fifth step (red: the step did not exist)

```console
$ cargo test -p xtask --bins
---- tests::typed_layer_wasm_step_carries_the_designed_arguments stdout ----
panicked at xtask\src\main.rs:1016:32:
no step named `wasm32 build of the typed layer`
```

Green after `xtask/src/main.rs:319`. The assertion is not only the argument vector: it
reads the `//` comment block above the `name:` line out of the source, because a step's
reasoning lives in ordinary line comments and is unreachable at runtime. It requires that
block to name **Workers** (what the step buys), **Send** and **flavour** (what it does
not), and **flavours.rs** (where the other half lives). A step whose comment restates its
arguments fails this test.

### AC-002 — five by name (red: four)

```console
---- tests::wasm_steps_resolve_and_number_five stdout ----
assertion `left == right` failed: the typed layer is not in `cargo xtask wasm`
  left: 4
 right: 5
```

**Negative control, hand-run and observed.** With the `REQUIRED` entry deleted and the name
left in `wasm_steps()`:

```console
$ cargo run -q -p xtask -- wasm
thread 'main' panicked at xtask\src\main.rs:907:36:
REQUIRED must contain the `wasm32 build of the typed layer` step
```

Loud, by name, and at the build of the gate itself — not four green sections and a silent
narrowing. Reverted; `cargo test -p xtask` back to 52 passing.

### AC-006 — the powerset widening (red: three packages, not four)

```console
---- tests::the_wasm32_powerset_covers_the_typed_layer stdout ----
the typed layer's own feature combinations are never compiled for wasm32:
["happenstance-core", "happenstance-neon", "happenstance-testkit"]
```

The test also asserts the probe is **retained**, which is the half that is easy to get
backwards: a probed mandatory guard is unguarded, and a mandatory powerset breaks every
machine without `cargo hack`. Run by hand afterwards, 144/144 green.

### AC-003 / AC-004 — the instantiation and its controls

These two compile rather than assert, so "red" for them is the discriminator rather than a
first failing run. AC-003's instantiation passed on its first compile — which is the audit
outcome, not a weak test: it is a compile error at three named call sites the moment any
link in the chain binds `SendEventStore`.

AC-004's control is what makes AC-003 believable, and it was **run and observed**. With
`send_probe`'s `impl<T: Send> Probe<T>` inherent block deleted:

```console
running 7 tests
test the_weak_flavour_store_is_genuinely_not_send ... FAILED
test the_local_store_is_not_send ... FAILED
test every_entry_point_binds_the_weak_flavour ... ok
test run_projection_spawns_from_generic ... ok
test result: FAILED. 5 passed; 2 failed
```

Both **positive controls** fail; both negative assertions keep passing. That asymmetry is
the whole point of RS-61-3, and it is now something someone has watched happen. Reverted;
7 passed.

`the_local_store_is_not_send` is not a duplicate of `command-loop`'s probe test. It adds a
second positive control — `RefCell<Vec<SequencedEvent>>` **is** `Send` — which turns the
implementation note's *"`Rc`, not `RefCell`"* from a comment into an assertion. If someone
rebuilt the instrument on a bare `RefCell`, that row is what would notice.

### AC-005 — a real red, and a real finding

Written first exactly as the spec fixed it, `S: SendEventStore + Send + Sync + 'static`, it
did not compile:

```console
error[E0277]: `<S as SendEventStore>::Error` cannot be sent between threads safely
note: required because it appears within the type
      `happenstance::runner::Stopped<<S as SendEventStore>::Error, MemoryProjectionStoreError>`
   --> crates\happenstance\src\runner.rs:548:6
help: consider further restricting the associated type
    |     S: … , <S as SendEventStore>::Error: Send
```

**The diagnosis.** On a failure the runner holds the stop — which carries `S::Error` —
across the port's `rollback` await (`crates/happenstance/src/runner.rs:480`), because one
`ProjectionError` needs the error *and* what the rollback said. RS-25-4's *collapse before
the next await* is not available there: the value that must survive the await **is** the
error. The command loop escapes this only because it returns its error immediately, with no
await after it — which is why `commit_spawns_from_generic` has always compiled.

**Why this is not a defect to route.** ES-6 is `[FROZEN]` and leaves `Error` unbounded on
purpose (`happenstance-cloudflare`'s error holds an `Rc<str>`), and ADR-0009 settled where
the obligation is paid, in terms: *"a caller who needs the error itself across an await adds
that bound to its own signature and pays for it there; the port never grows one."* ADR-0009
also supplies the shape — a marker trait with a blanket impl, declared by the consumer, and
explicitly **not** shipped in `happenstance-core`. So the fix is
`crates/happenstance/tests/flavours.rs:503`'s local `ThreadSafeEventStore`, and the test's
bound reads `S: ThreadSafeEventStore + Send + Sync + 'static` — the spec's bound, one marker
further out. `Send`, `Sync` and `'static` keep the roles `memory.rs:651-680` records.

The extra marker does not blunt what AC-005 is for. An `async fn read` refactor makes the
*stream* `!Send`, and no bound on `Error` repairs that; this test still stops compiling.

No `[FROZEN]` clause was amended and `crates/happenstance-core/src/**` was not touched. What
changed instead is documentation: `crates/happenstance/src/runner.rs:297-310` now states the
cost on the item a caller meets, which is where they will meet it.

### AC-007 — the ban and the rustdoc

```console
---- async_trait_is_banned stdout ----
`async-trait` is not in deny.toml's `[bans]` deny list, so nothing refuses the one
attribute that makes the wasm32 target impossible

---- entry_points_state_their_flavour stdout ----
command.rs: `pub async fn commit<` does not say on its own page that it binds
`EventStore`, the weaker flavour that accepts both
```

The density half then failed a second time, on real prose, which is the reason it is a
criterion:

```console
runner.rs: a doc line on `pub async fn run_projection<` is 81 columns; the budget is 80:
because building one [`ProjectionError`] needs the error *and* the rollback's
```

The sentence yielded. Both green after.

## Commits

- `0a010c9` — `feat(typed-layer-and-alpha-release): Edge flavour and wasm claim`
- `7abff7d` — `fix(typed-layer-and-alpha-release): re-anchor twenty constitution citations`

The second is a separate commit on purpose, and it follows this branch's own precedent
(`c4e36c4`): `standards/rust/**` is outside this story's PR boundary, and re-anchoring a
`file:line` citation at the file it already named changes no claim, only whether a reader
can follow one. Inserting the fifth step shifted twenty of them.

## Changes

| File | Shape of the change |
| --- | --- |
| `xtask/src/main.rs` | The fifth `Step` in `REQUIRED` (`:296-333`) with its two-halves comment; `-p happenstance` added to the `OPTIONAL` `wasm32 feature powerset` (`:690-698`); `wasm_steps()` grown to five by name (`:905-912`); the `wasm` help text and the crate's own module docs updated to say five and to say what they do not prove; three new tests in the existing `mod tests` (`:1257`, `:1318`, `:1356`) plus two helpers |
| `deny.toml` | `[bans]` gains the `async-trait` deny entry with a named `wrappers` exemption and a comment citing ADR-0001 (`:38-60`), and `allow-wildcard-paths = true` (`:26-36`) — see *Notes* |
| `crates/happenstance/tests/flavours.rs` | Four additions: `the_local_store_is_not_send`, `every_entry_point_binds_the_weak_flavour`, `each_module_imports_one_flavour_name`, `run_projection_spawns_from_generic`, plus the local `ThreadSafeEventStore` marker and an `Enrolments` projection. `command-loop`'s existing tests are untouched |
| `crates/happenstance/tests/doc_surface.rs` | `entry_points_state_their_flavour` and `no_page_claims_an_executed_edge_test`, with `item_doc`/`first_sentence` helpers |
| `crates/happenstance/tests/manifest_contract.rs` | `async_trait_is_banned`, reading `deny.toml` through `include_str!` |
| `crates/happenstance/src/command.rs` | Documentation only: one paragraph each on `commit` and `commit_with` naming the flavour bound and what it buys. No signature, no body |
| `crates/happenstance/src/runner.rs` | Documentation only: a `# Which flavour this binds, and what spawning it costs` section on `run_projection`. No signature, no body |
| `standards/rust/*.md` | Twenty `xtask/src/main.rs:NNN` citations re-anchored. Separate commit |

**No public item was added** (NF-005), no `[FROZEN]` clause was edited, `spec/` and
`crates/happenstance-core/src/**` are untouched (NF-006), no `[dependencies]` entry moved
(NF-002), and the four existing `wasm32` steps keep their names, order and argument lists
byte for byte (NF-004).

## Gates

| Gate | Result |
| --- | --- |
| `cargo test -p xtask` | 52 passed, 0 failed |
| `cargo test -p happenstance --all-features --test flavours` | 7 passed |
| `cargo test -p happenstance --all-features --test doc_surface --test manifest_contract` | 10 + 6 passed |
| `cargo check --locked -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json` | green; `Cargo.lock` unmoved |
| `cargo xtask wasm` | five sections, the four originals first and in order |
| `cargo xtask affected --base main` | **affected gate passed** |
| `cargo xtask ci --fast` | **all required checks passed (--fast: 4 optional step(s) not run)**, printing five `=== wasm32 … ===` sections |
| `cargo hack check -p happenstance --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` | 144/144 green |
| `cargo deny check` | `advisories ok, bans ok, licenses ok, sources ok` |
| `cargo fmt --all --check` | clean |

The *Release bar* row of the spec's test table — `cargo xtask ci` whole — is run by
`publish-0-2-0-alpha-1`, this slice's last story, on the tree it publishes. The two
`OPTIONAL` steps this story leans on were each run standalone above.

## Notes

**Two deviations, both widenings of the diff, both stated rather than absorbed.**

1. **`deny.toml` gained `allow-wildcard-paths = true`, which this story did not plan.**
   `cargo deny check bans` was **already failing on this tree before any edit of mine**,
   verified by stashing my change and re-running: `error[wildcard]: found 1 wildcard
   dependency for crate 'happenstance'`, from `happenstance-testkit = { path =
   "../happenstance-testkit" }` at `crates/happenstance/Cargo.toml:70`. That spelling is
   deliberate and documented in the manifest — a versionless dev-dependency is stripped from
   the published manifest entirely, which is what keeps the testkit's CF-32 release order
   free. Leaving it red would have handed `publish-0-2-0-alpha-1` a gate it cannot pass at a
   point where the fix is a document edit under time pressure. The setting is scoped, not a
   switch-off: a wildcard against the *registry* still fails.

2. **The `async-trait` ban carries a named `wrappers` exemption.** Its first run exited 2:
   `async-trait v0.1.91` is already in the lock file, reached only from `wasm-bindgen-test`,
   itself a dev-dependency of the conformance harnesses and absent from every published
   artifact. A blanket allow would have made the ban decorative; naming the one wrapper keeps
   it real, because a second route into the graph is a wrapper this list does not carry.

**Two findings routed to the slice-mate's log rather than fixed here.**

- The `run_projection` spawn obligation above. It bears on **ES-6**, it is *consistent* with
  ES-6 and ADR-0009 rather than contradicting them, and so it is recorded as an accounted
  finding with the disposition *not a defect* — the alternative, inventing a defect entry for
  a promise that was kept, is the wrong-clause-citation failure EC-001 forbids.
- `cargo hack`'s wasm32 powerset emits `warning: method read_through is never used` from
  `crates/happenstance-core/src/projection_memory.rs:233` in eight feature combinations. It
  **pre-dates this story** — the existing three-package powerset emits it too — and under
  CI's ambient `RUSTFLAGS: -D warnings` it is a failure rather than a warning. It bears on no
  clause, and `crates/happenstance-core/src/**` is inadmissible here, so it is recorded as a
  **support**-bound finding at the moment it was found, per EC-001.

**Three things the spec expected that turned out not to hold, none of them load-bearing.**
`xtask/src/main.rs` already had a `mod tests` (Clarifications item 6 assumed it did not), so
the three new tests joined it rather than opening one. `tokio` was already in
`crates/happenstance/Cargo.toml`'s `[dev-dependencies]`, so Clarifications item 3's
contingency did not fire and the manifest stayed outside the diff. And `flavours.rs` already
carried an `Rc`-backed store and a probe from `command-loop`, so this story extended both
rather than writing a second of each — which is what the implementation notes asked for.

**The probe module is not `cfg(not(target_arch = "wasm32"))`-gated**, deliberately, and this
departs from an implementation note. That gate exists in `local_conformance.rs` because the
testkit's harnesses are type-checked for `wasm32` by a `--tests` step. Nothing compiles
`happenstance`'s test targets for that target: the mandatory step is a plain `cargo check`
and the powerset carries `--no-dev-deps`. Adding a `cfg` that no build ever evaluates is a
second thing to keep true for no check.
