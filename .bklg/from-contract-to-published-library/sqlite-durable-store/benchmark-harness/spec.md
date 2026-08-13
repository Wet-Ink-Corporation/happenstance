---
item: HS-S0034
stage: spec
created: 2026-08-12T13:46:33.284Z
updated: 2026-08-12T13:46:33.284Z
template_sig: 87bbf1d0
rendered_sig: 44ccf9f6
---

# Spec — event_store_benchmarks! in the testkit, provably not conformance

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/sqlite-durable-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/benchmark-harness/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` — architecture brief §1 (the mount table), §10 (*the testkit surface this project adds, and its blast radius*), §12 (what ADR-0022 must carry); testing brief §1 (the benchmark tier), §2 (AC-012's row), §3 (the emitter is the sole sanctioned seam) |
| Signed-off design | `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` — **no user-facing surface**, approved 2026-08-12. This story renders no surface and claims no `## Items` row. |
| Story map row | `.bklg/from-contract-to-published-library/sqlite-durable-store/_storymap.md` — slice `bench-harness-and-adr`, first row |
| Roadmap pointer | `RUNBOOK.md:4207-4213` — phase 8's benchmark work item, and the three scenarios it names |

Traces to project **AC-012**. Depends on nothing. Blocks `adr-0022-append-condition-strategy` (HS-S0035), which cannot quote a measured figure without it.

## One-line PR slice

Add `event_store_benchmarks!` to `happenstance-testkit` behind an off-by-default **and** target-gated `bench` feature, with its own enumeration outside `for_each_event_store_rule!` and a caller-supplied emitter, mounted against `MemoryFixture`.

## Executive summary

This PR lands `crates/happenstance-testkit/src/bench.rs` — a fourth macro family beside conformance, model and concurrency — and mounts it at a new test target, `crates/happenstance-testkit/tests/memory_benchmarks.rs`, so the harness runs against a real `Fixture` on the day it ships rather than on the day the SQLite fixture exists.

**Delta against the project charter.** The charter states the harness as a scope bullet (`project.md`, *In scope*, `event_store_benchmarks!(fixture)` in the testkit behind a `bench` feature) and AC-012 states its two halves: it runs, and *the conformance rule count is unchanged by its arrival*. What this story adds is the mechanism that makes the second half structurally true instead of asserted — a separate enumeration, a separate module, and no new entry in `xtask`'s `RULE_FILES` — plus the gate arithmetic the briefs only gesture at: a Cargo feature is not target-scoped, so `--all-features` switches `bench` on for `wasm32` too, and only the *optional* wasm32 feature-powerset step sees it.

Nothing about SQLite is in this PR. The harness's first paying customer is the slice-mate ADR, and the SQLite fixture inherits it two slices later exactly as an adapter inherits conformance.

## Context pack

Read this section and you can start. Everything below the Behavior table is signposted depth, not prerequisite.

**1. A benchmark is not a rule, and the enforcement is structural, not editorial.** CF-34 (`spec/SPECIFICATION.md:8263`) says performance MUST be measured by a separate harness and that harness MUST NOT be part of the conformance bar; its own `Rule:` line reads *none — the harness is not the bar, which is the clause's content*. So there is nothing to add to `suite.rs`, and adding one would violate the clause it claims to serve. Concretely: benchmarks get **their own enumeration macro in their own module**, exactly as `for_each_model_rule!` lives in `model.rs:712` and `for_each_concurrency_rule!` in `concurrency.rs:1044`, and for the reason `concurrency.rs` states — `cargo xtask spec-trace` scans `suite.rs` for `pub async fn`, so anything written there owes a clause. `for_each_event_store_rule!` (`registry.rs:94`) is not touched, `registry.rs::no_orphan_rules` sees the same set before and after, and `xtask/src/spec_trace.rs:85-89`'s `RULE_FILES` stays a three-element array. That triple is what makes AC-012's "rule count unchanged" a fact about the tree rather than a sentence in a report.

**2. Off by default is not enough; it must also be target-gated.** `crates/happenstance-testkit/src/fixtures.rs:298-305` records the exact trap, paid for once already with `proptest`: **a feature is not target-scoped**, so `--all-features` sets `feature = "bench"` on `wasm32-unknown-unknown` as well. The gate's `wasm32 feature powerset` step (`xtask/src/main.rs:564-591`) runs `cargo hack check -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps`, which is precisely the step that compiles that combination. The module therefore carries the same double condition the `model` module does (`lib.rs:182-183`): `#[cfg(all(feature = "bench", not(target_arch = "wasm32")))]`, plus `#[cfg_attr(docsrs, doc(cfg(feature = "bench")))]`. This is DR-08 and architecture-brief AC-A06 — *nothing added to the shared testkit costs the `!Send` flavour anything* — and it is the one obligation this story can fail silently, because the step that catches it is `OPTIONAL` and does **not** run under `cargo xtask ci --fast`.

**3. The measurement dependency belongs to the caller, not to the testkit.** CF-23 (`spec/SPECIFICATION.md:7920-7931`) is `[FROZEN]`: the testkit MUST NOT emit any runtime-specific attribute from its own expansion, and the per-test wrapper MUST be a parameter. The benchmark family extends that to the *timing* wrapper as well — the testing brief §3 names the emitter as the single sanctioned seam in this entire project. `happenstance-testkit`'s `[dependencies]` today are `happenstance-core` and `futures-core` and nothing else, and this PR leaves that line unchanged. A caller who wants `criterion`, `divan`, or a CSV row puts it in their own `dev-dependencies` and hands in an emitter; `.kb/decisions/0010-the-suite-must-prove-itself.md` is the decision this inherits.

**4. What the harness measures is fixed by two consumers, not chosen freely.** `RUNBOOK.md:4207-4213` names three scenarios — append throughput, conditional append under contention, and replay of N events with and without a tag filter. `.kb/decisions/0012-append-shape-and-preconditions.md:55-60` names a fourth requirement on the same instrument: the evidence that could lift the `&[Event]` marker is *"an adapter with a real write path and a benchmark harness, measured under a realistic batch and rejection mix"*. So the contended-append scenario must surface **rejections as a reported quantity**, not only elapsed time — a run in which every contender wins measures nothing about the rejection mix ADR-0012 asked for. Batch size, contender count and event count are caller-supplied parameters for the same reason: the numbers that make sense for a `Vec`-behind-an-`RwLock` and for a file under `BEGIN IMMEDIATE` are not the same numbers.

**5. Mounted means executed, and the mount is `MemoryFixture`.** `CLAUDE.md`'s *rule that matters* has a benchmark analogue the story map states directly: a harness reachable only by `cargo check` is the "compiles but never ran" failure this whole project exists to retire. `SqliteFixture` does not exist yet and is two slices away (`_storymap.md`, merge order 1 → 2), so this story mounts against `crates/happenstance-testkit/src/fixtures.rs`'s `MemoryFixture`, the reference implementation, in the testkit's own `tests/`. `tests/memory_concurrency_conformance.rs` is the shape to copy: a `#![cfg(...)]` at the top of the target and the family invoked twice, once through the default emitter and once through a second one, so that CF-23's parameter is demonstrated rather than described.

**6. The harness runs in the existing test step — and still is not the bar.** `cargo xtask ci` runs `cargo test --locked --workspace --all-features` (`xtask/src/main.rs:143-153`), so once `bench` exists the mounted target compiles and runs there. That is deliberate and is *not* a contradiction of the testing brief's "not wired into any `verify:` command": no new gate step is added, `.redkiln/config.yaml`'s `verify:` block is untouched, and nothing added here asserts on a timing. The default emitter runs a deliberately smoke-sized budget and asserts only that each scenario completed and produced a finite measurement record. The forbidden thing — CF-34's own `Rejects:` line — is a *threshold*: no `assert!(elapsed < …)` anywhere, on any budget, ever. A harness nobody compiles rots, and a harness that can fail a merge on a loaded runner teaches people to re-run until green; the shape above is the only one that avoids both.

**7. This story writes no ADR and settles no append strategy.** It ships the instrument. `adr-0022-append-condition-strategy` (HS-S0035) runs it against real SQLite and records the winner, the losers and the number. Per `MEMORY.md` and `CLAUDE.md`'s two-places rule, an ADR is never a side effect of a code change — if this story discovers something ADR-0022 must carry, it goes in the story's own ledger and into the slice-mate's input, not into `.kb/decisions/`.

**8. The persona slice.** The adapter author (initiative *Who this is for*; AC-04/AC-05) inherits benchmarks the way they inherit conformance — one macro invocation against the fixture they already wrote, no measurement dependency imposed on them, and no risk that the new macro changed what "passing" means. That inheritance is what `RUNBOOK.md:4210-4211` calls out as something no other Rust DCB library does.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate, consumed inside this initiative by `adr-0022-append-condition-strategy` (which cannot quote a figure without it) and again by `sqlite-fixture-and-whole-suite`. Not a double, not a flag, not a fixme (`_storymap.md`, *Why the three foundations are foundations*).
- **Slice / milestone**: `bench-harness-and-adr`. Slice-mate: `adr-0022-append-condition-strategy` (HS-S0035, `blocked_by` this story).
- **Mount point**: `crates/happenstance-testkit/tests/memory_benchmarks.rs` — a new integration-test target that invokes `event_store_benchmarks!` against `MemoryFixture`. This is the composition root: the harness is *executed* here by `cargo test`, not merely compiled. Modelled on `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs`.
- **Wires into**:
  - `crates/happenstance-testkit/src/contract.rs` — `Fixture` (`connect`, the capability constants, the limits-are-facts distinction). The benchmark family binds `Fixture`, never a concrete store.
  - `crates/happenstance-testkit/src/fixtures.rs:243-292` — `MemoryFixture`, the fixture this story mounts against.
  - `crates/happenstance-testkit/src/registry.rs:228-300` — the `__emit_*` convention the benchmark emitters follow (`#[doc(hidden)] #[macro_export]`, one item per name, path substituted verbatim so a bare name resolves in the caller's scope).
  - `crates/happenstance-testkit/src/lib.rs:168-190` — the module list, the `pub use` surface, and the crate-doc paragraph that must gain a benchmark sentence **without an intra-doc link** (see the D13 note at `lib.rs:110-130`).
  - `crates/happenstance-testkit/Cargo.toml` — the `[features]` table gains `bench = []`; `default = []` is unchanged and `[dependencies]` is unchanged.
  - `happenstance-core`'s `Event`, `Query`, `AppendCondition`, `ReadOptions` and `collect` — the same imports `concurrency.rs:160-163` uses. No new public API in the contract crate (architecture brief AC-A04).
- **Renders surfaces**: **none.** `_design.md` records this project as having no user-facing surface, approved at the `/redkiln:plan` sign-off gate, so there is no `## Items` row for this story to claim and no `## Signatures` block to match.
- **Conformance rule(s)**: **none, deliberately, and that is the story's content.** No entry is added to `for_each_event_store_rule!` (`registry.rs:94`), no `pub async fn` is added to `suite.rs`, and `xtask/src/spec_trace.rs:85-89`'s `RULE_FILES` is unchanged. The behaviour is observed instead by `registry.rs::no_orphan_rules` and by the rule-name count being identical either side of the diff.
- **Clause(s)**: discharges **CF-34** (`spec/SPECIFICATION.md:8263-8275`) by building the separate harness the clause requires, and honours **CF-23** (`:7920-7931`, `[FROZEN]`) and **CF-32** (`:8200-8208`, `[FROZEN]`). CF-34 keeps its `[PROVISIONAL]` marker and its falsifier — a row-counting instrumented fixture would split the clause — and this story does **not** move that marker; no `[FROZEN]` clause is amended.
- **Advances DoD scenario**: initiative **DoD 3** — *the durable store passes the suite for real* — by supplying the measurement its blocking predecessor ADR-0022 must quote (`AC-013`, DR-06). The same harness is what initiative **DoD 5** (*"with the position-visibility cost measured rather than estimated"*) inherits at `postgres-and-neon-stores`.

## PR boundary

**In this PR**

- `crates/happenstance-testkit/src/bench.rs` — the module: three benchmark scenarios, `for_each_event_store_benchmark!`, the shipped emitters, `event_store_benchmarks!`, and the measurement record type.
- `crates/happenstance-testkit/src/lib.rs` — the gated `pub mod bench;` declaration, any `pub use` the record type needs, and the crate-doc paragraph placing benchmarks beside the three rule families.
- `crates/happenstance-testkit/Cargo.toml` — `bench = []` in `[features]`. No dependency change.
- `crates/happenstance-testkit/tests/memory_benchmarks.rs` — the mount.
- `crates/happenstance-testkit/README.md` — the family table gains a benchmark row saying it is not the bar.
- `CHANGELOG.md` — an `[Unreleased] / Added` entry naming the feature and the CF-34 boundary.
- The story's own backlog folder (`spec.md`, `_ledger.md`).

The implementer **may** also touch the wiring named in the Integration contract — `lib.rs`'s module list and `Cargo.toml`'s `[features]` are the composition root for a library, and editing them to mount this slice is not scope drift.

**Explicitly not in this PR**

- Anything under `crates/happenstance-sqlite/**`. No `todo!()` is replaced, no `SqliteFixture` is written, no schema is chosen.
- `.kb/decisions/**` and `references/adr/**`. ADR-0022 is HS-S0035's, authored through the runbook's ADR queue, never as a side effect of this change.
- `crates/happenstance-testkit/src/suite.rs`, `registry.rs`'s rule list, `model.rs`, `concurrency.rs` — including `concurrency::CONTENDERS`, which is `concurrency-family-and-contender-count`'s to move.
- `xtask/**` and `.redkiln/config.yaml`. No new gate step, no new `verify:` command, `RULE_FILES` untouched.
- Any measurement crate (`criterion`, `divan`, …) anywhere in the workspace manifest, including as a dev-dependency of the testkit.
- Tuning. `PAGE_SIZE`, chunk widths and ceilings belong to later stories; this ships the instrument, not a result.

```
crates/happenstance-testkit/src/bench.rs
crates/happenstance-testkit/src/lib.rs
crates/happenstance-testkit/Cargo.toml
crates/happenstance-testkit/README.md
crates/happenstance-testkit/tests/memory_benchmarks.rs
CHANGELOG.md
.bklg/from-contract-to-published-library/sqlite-durable-store/benchmark-harness/**
```

**Merge DoD.** `cargo xtask ci --fast` green, plus the wasm32 feature-powerset step run explicitly (it is `OPTIONAL` and `--fast` skips it, and it is the only step that compiles `bench` on `wasm32`); `cargo test -p happenstance-testkit --features bench` runs the mounted target; the rule-name count is identical either side of the diff; every AC-### has a cited row in `_ledger.md`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A benchmark family exists, in its own module | `pub mod bench`, declared `#[cfg(all(feature = "bench", not(target_arch = "wasm32")))]` with `#[cfg_attr(docsrs, doc(cfg(feature = "bench")))]` — the same double condition and docs attribute the `model` module carries. | `crates/happenstance-testkit/src/lib.rs:181-183`; `crates/happenstance-testkit/src/fixtures.rs:298-305` |
| It carries its own enumeration | `for_each_event_store_benchmark!` lives in `bench.rs`, beside the scenarios it names, and hands a comma-separated identifier list to a callback captured as raw token trees (`$($callback:tt)+`, not `$cb:path` — a parsed path cannot sit in callee position, which `registry.rs:94-105` explains and pays for). | `crates/happenstance-testkit/src/registry.rs:94-118`; `crates/happenstance-testkit/src/model.rs:712`; `crates/happenstance-testkit/src/concurrency.rs:1044` |
| Nothing is added to the conformance rule set | `for_each_event_store_rule!` gains no entry; `suite.rs` gains no `pub async fn`; `xtask/src/spec_trace.rs`'s `RULE_FILES` stays `[SUITE, model.rs, concurrency.rs]`. The rule-name count from `for_each_event_store_rule!(__emit_rule_names)` is identical before and after. | `crates/happenstance-testkit/src/registry.rs:94`, `:293-297`, `:411-430`; `xtask/src/spec_trace.rs:85-89` |
| The public entry point mirrors the other three families | `event_store_benchmarks!` takes the same three arms: `($fixture:expr)`, `(mod_name = …, fixture = …)`, and `(mod_name = …, emit = …, fixture = …)`; it hoists the fixture behind an `async fn __benchmark_fixture() -> impl Fixture` so an emitter never has to name the fixture type. It binds `Fixture`, not a concrete store, and imposes no `Send` bound. | `crates/happenstance-testkit/src/lib.rs:312`; `crates/happenstance-testkit/src/concurrency.rs:1129-1166` |
| The wrapper — and the timer — are caller-supplied | Emitters are `#[doc(hidden)] #[macro_export] macro_rules!` following the `__emit_*` naming, path-substituted verbatim so a bare name resolves in the caller's scope. A default smoke emitter ships so the macro is usable with one argument; a measurement emitter is the caller's, with its dependency in the caller's `dev-dependencies`. The testkit's `[dependencies]` stay `happenstance-core` + `futures-core`. | CF-23, `spec/SPECIFICATION.md:7920-7931`; `crates/happenstance-testkit/src/registry.rs:228-300`; `crates/happenstance-testkit/Cargo.toml` |
| Three scenarios, fixed by their consumers | (a) append throughput over a batch of *n*; (b) conditional append under *k* contenders, reporting the committed/rejected split as well as elapsed time; (c) replay of *N* events, once unfiltered and once behind a tag filter. Batch size, contender count and event count are caller-supplied parameters, not constants. | `RUNBOOK.md:4207-4213`; `.kb/decisions/0012-append-shape-and-preconditions.md:55-60` |
| The contended scenario reports a rejection mix | ADR-0012's lifting evidence is *"a realistic batch and rejection mix"*, so the measurement record carries the `ConditionViolated` count alongside the timing. A run whose contenders never collide is a valid measurement of the wrong thing, and the record makes that visible rather than averaging it away. | `.kb/decisions/0012-append-shape-and-preconditions.md:52-69`; `crates/happenstance-testkit/src/concurrency.rs:214-231` (`Attempt`, the shape a contender's result collapses to) |
| No assertion on a timing, at any budget | The default emitter asserts completion and a finite record. There is no threshold, no `assert!(elapsed < …)`, and no watchdog anywhere (CF-33 is why the concurrency family has none either). A benchmark result never fails a merge. | `spec/SPECIFICATION.md:8255-8275` (CF-34 and its `Rejects:`); `crates/happenstance-testkit/src/concurrency.rs` module docs |
| The harness is mounted and executed, not merely compiled | `crates/happenstance-testkit/tests/memory_benchmarks.rs` invokes the macro against `MemoryFixture` twice — once through the default emitter, once through a second one — under a target-level `#![cfg(not(target_arch = "wasm32"))]`. | `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs:26-36`; `crates/happenstance-testkit/src/fixtures.rs:243-292` |
| No new gate step, and no `verify:` change | `xtask/src/main.rs` is untouched; `.redkiln/config.yaml`'s `verify:` block is untouched. The existing `cargo test --locked --workspace --all-features` step exercises the mount for free, which is the whole of the wiring. | `xtask/src/main.rs:143-153`; `.redkiln/config.yaml:28-67` |
| Documented to the crate's own bar | Module docs state what a benchmark is *not*; every public item carries docs (`missing_docs = "warn"` + `-D warnings`); every fallible public fn carries `# Errors`. The crate-doc paragraph naming the family **must not** be an intra-doc link — `bench` is absent under default features and on `wasm32`, and `broken_intra_doc_links = "deny"` makes that a hard error under `cargo doc`. | `Cargo.toml:101-135` (workspace lints); `crates/happenstance-testkit/src/lib.rs:110-130` (the D13 failure already paid for once) |
| Versioning and changelog | `happenstance-testkit`'s own `version` key stays a literal (CF-32, `[FROZEN]`) and is never folded into the workspace by this change. A new off-by-default feature is additive; the `[Unreleased] / Added` entry names it and states that it is not part of the bar. | `spec/SPECIFICATION.md:8200-8208`; `crates/happenstance-testkit/Cargo.toml`; `CHANGELOG.md:24-30` |

## Data and migrations

**N/A.** This story adds no schema, no persisted format and no wire type. It touches no database: the only store it drives is `MemoryEventStore` through `MemoryFixture`, which is a `Vec` behind an `RwLock` created and dropped inside the test process. Migration 1 and the `event_tag` schema belong to `schema-migration-and-identity` in the next slice.

Two manifest-shaped facts that are *not* migrations but sit in the same blast radius, recorded so they are not mistaken for absent:

- **A new Cargo feature is a public, semver-visible surface.** `bench = []` is additive and off by default, so it breaks nothing — but `--all-features` reaches it on every target, which is why the target gate in the Context pack §2 is a correctness requirement and not tidiness.
- **No dependency is added anywhere in the workspace**, so `Cargo.lock` is unchanged and `cargo deny check` has nothing new to read. Any measurement crate enters through a *caller's* `dev-dependencies`, never through this crate.

## Acceptance criteria

Eight criteria. Each is written from the intent of a persona the initiative names
(`.bklg/from-contract-to-published-library/initiative.md`, *Who this is for*; journeys
*Learn when you are finished*, *Event-source at the edge without hand-rolling it*,
*Decide in one sitting*), and each crosses the whole stack this story has — macro
definition → feature and target gate → a real `Fixture` → an executed `cargo test`
target. All eight together discharge project **AC-012**
(`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md:256-259`), whose two
halves are *it runs* and *the conformance rule count is unchanged by its arrival*.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **The adapter author inherits benchmarks the way they inherit conformance.** GIVEN an adapter author who already has a `Fixture` and has never measured anything, WHEN they add one line — `happenstance_testkit::event_store_benchmarks!(MyFixture::new());` — to a test target and run `cargo test --features bench`, THEN all three scenarios execute against their real store through `Fixture::connect`, each reporting a completed measurement record, with no measurement crate added to their own manifest and no conformance rule re-run or re-defined. The mount in this PR is `MemoryFixture`, and it is *executed* by `cargo test`, not merely compiled. | `crates/happenstance-testkit/tests/memory_benchmarks.rs` run by `cargo test -p happenstance-testkit --features bench -- --show-output`; the generated per-scenario tests appear by name in the run's output. |
| AC-002 | **The evaluator can check "benchmarks are not the bar" instead of trusting it.** GIVEN an evaluator reading this diff (initiative *Decide in one sitting*), WHEN they compare the conformance rule set either side of it, THEN the rule-name list produced by `for_each_event_store_rule!(__emit_rule_names)` is **identical**, `suite.rs` has gained no `pub async fn`, `xtask/src/spec_trace.rs`'s `RULE_FILES` is still a three-element array, and no clause's `Rule:` line has changed — so CF-34's claim is a property of the tree rather than a sentence in a report. | `cargo test -p happenstance-testkit no_orphan_rules` (`crates/happenstance-testkit/src/registry.rs:412`) plus `cargo xtask spec-trace` and `cargo xtask lints`, both green; the before/after rule-name count recorded as cited evidence in `_ledger.md`. |
| AC-003 | **The edge developer keeps their runtime, including under `--all-features`.** GIVEN the constrained-runtime developer whose target is `wasm32-unknown-unknown`, WHEN the testkit is checked on that target across the *whole* feature powerset — the combination `--all-features` actually produces, because a Cargo feature is not target-scoped — THEN `bench` compiles away entirely, the four mandatory wasm32 steps are unchanged, and nothing in the `!Send` flavour has gained a bound, a dependency or an item (DR-08, architecture brief AC-A06). | `cargo hack check -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` green (the gate's `wasm32 feature powerset` step, `xtask/src/main.rs:564-591`) **run explicitly**, since it is `OPTIONAL` and `--fast` skips it; plus `cargo xtask wasm`. |
| AC-004 | **No measurement dependency is imposed on anybody, and the timer is the caller's.** GIVEN an adapter author who wants `criterion`, `divan` or a CSV row, WHEN they invoke the third arm with `emit = <their emitter>`, THEN their own wrapper surrounds each scenario and their dependency stays in *their* `dev-dependencies` — while `happenstance-testkit`'s `[dependencies]` remain exactly `happenstance-core` + `futures-core` and `Cargo.lock` is unchanged. Two distinct emitters are demonstrated in the mount, not described (CF-23, `[FROZEN]`). | `crates/happenstance-testkit/tests/memory_benchmarks.rs` invokes the family twice, the second time through a second emitter; `git diff -- crates/happenstance-testkit/Cargo.toml Cargo.lock` shows a `[features]` line added and nothing else. |
| AC-005 | **The decision-maker measures the three things the decision needs, at their own numbers.** GIVEN ADR-0022's author (the slice-mate, HS-S0035) about to choose an append-condition strategy, WHEN they run the harness against real SQLite, THEN they get (a) append throughput over a batch of *n*, (b) conditional append under *k* contenders, and (c) replay of *N* events run **both** unfiltered and behind a tag filter — with *n*, *k* and *N* supplied by them at the call site, because no constant in the testkit can be right for both a `Vec` behind an `RwLock` and a file under `BEGIN IMMEDIATE`. | Parameterised invocations in `crates/happenstance-testkit/tests/memory_benchmarks.rs` at two different parameter sets; the replay scenario reports a filtered and an unfiltered record. |
| AC-006 | **A contended run in which nobody collides is visible as such, not averaged away.** GIVEN the same author, whose evidence obligation is a *"realistic batch and rejection mix"* (`.kb/decisions/0012-append-shape-and-preconditions.md:52-60`), WHEN the contended scenario runs with *k* contenders over one append condition, THEN the record reports the committed and the rejected counts **separately**, distinguishes a `ConditionViolated` rejection from a transport-level failure the way `concurrency::Attempt` does, and accounts for every contender (`committed + rejected + failed == k`) — so a run in which every contender wins is legible as a measurement of the wrong thing. | An assertion in `crates/happenstance-testkit/tests/memory_benchmarks.rs` on the record's own fields against `MemoryFixture` (which serialises writers, so exactly one contender commits); no timing is asserted. |
| AC-007 | **A benchmark result can never turn a merge red, and the testkit never reads a clock.** GIVEN any contributor on a loaded CI runner, WHEN the mounted target runs inside the gate's existing `cargo test --locked --workspace --all-features -- --show-output` step, THEN it passes on completion alone: there is no threshold, no `assert!(… < …)` on a duration, no watchdog and no sleep anywhere in the family, `cargo run -p xtask -- lint-clock` stays green over `crates/happenstance-testkit/src` — which is where `bench.rs` lands — and the default emitter prints at most one line per scenario so the CF-18 `SKIP <rule>: …` lines that step exists to surface are not buried under benchmark output. | `cargo run -p xtask -- lint-clock` (a REQUIRED gate step, `xtask/src/main.rs:361`, implemented at `xtask/src/lints.rs:223-281`) green; `cargo xtask ci --fast` green; a reviewed `rg` over `crates/happenstance-testkit/src/bench.rs` for threshold-shaped assertions returning nothing, cited in `_ledger.md`. |
| AC-008 | **A reader meets the boundary before they meet the macro.** GIVEN an adapter author or evaluator arriving at docs.rs or the crate README, WHEN they read the benchmark family's entry, THEN the module docs and the crate-doc paragraph state what a benchmark is *not* (CF-34) and how an emitter is supplied (CF-23), the README's family table carries a benchmark row saying it is not the bar, `CHANGELOG.md` has an `[Unreleased] / Added` entry naming the feature and the boundary, every public item carries docs and every fallible public fn an `# Errors` section — and the crate-doc mention of `bench` is **not** an intra-doc link, so `cargo doc` still succeeds under default features and on `wasm32`, which is the D13 failure this workspace has already paid for once (`crates/happenstance-testkit/src/lib.rs:112-131`). | `cargo doc -p happenstance-testkit --no-deps` with default features **and** the gate's `documentation` step (`cargo doc --locked --workspace --all-features --no-deps --document-private-items`, `RUSTDOCFLAGS=-D warnings`) both green; README and `CHANGELOG.md` diffs read at review. |

**Coverage of the traced project AC.** AC-012's first half (*it runs, behind a `bench`
feature*) is AC-001 + AC-005 + AC-006, executed rather than compiled. Its second half
(*the conformance rule count is unchanged by its arrival*) is AC-002, with AC-003 and
AC-007 guarding the two ways that half can be true on paper and false in the gate — a
feature that reaches `wasm32`, and a harness that fails a merge on a timing. AC-004 and
AC-008 are what make the inheritance usable by someone who is not this project.

## Interaction quality

**No rendered surface, and that is a signed-off finding rather than an omission.**
`.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md:12-17,84-89` records this
project as having **no user-facing surface** — no screen, no route, no view — approved
by the repository owner at the `/redkiln:plan` sign-off gate, with `design.capture`
deliberately absent from `.redkiln/config.yaml` so the perceptual review is a declared
skip and not a silent pass (`CLAUDE.md`, *Where the work lives*). There is therefore no
`## Items` row for this story to claim and no `## Signatures` block to match, and the
**composition family is formally N/A** — no placement, transience, density budget,
hierarchy or named visual anti-pattern applies, because nothing is composed.

What is *not* N/A is the family this repository substitutes for it. Per `CLAUDE.md`,
`_design.md` is the bundled design stage repurposed to the **public API surface**,
"because every other check in this repository is satisfied by an API that is correct
and unusable". The benchmark family is exactly such an API. So the state-invariant
family is read at that grain, and — per RFC §6.7/D6 — **every invariant that applies is
carried by an AC-### row in the table above**, never by a bullet here. This section only
says which row carries which invariant and how it is verified.

| Invariant (state family, read at API grain) | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — an existing caller's world does not move. No rule is renamed, retired or re-ordered; no existing macro arm changes; `suite.rs`, `registry.rs`'s rule list, `model.rs` and `concurrency.rs` are untouched, so an adapter that passed yesterday passes today with the same names in the same output. | AC-002 | `no_orphan_rules` + `cargo xtask spec-trace` + `cargo xtask lints`; identical rule-name list either side of the diff. |
| **Non-occlusion** — the new output does not bury the old. The gate's test step runs `-- --show-output` precisely so a declined capability's `SKIP <rule>: …` line reaches a human (`xtask/src/main.rs:143-153`); a chatty benchmark emitter would make that step's reason for existing unreadable. The default emitter is held to at most one line per scenario. | AC-007 | `cargo test -p happenstance-testkit --features bench -- --show-output` read at review; line count per scenario. |
| **Preserved state across the change** — the default-feature surface is byte-for-byte what it was. `default = []` is unchanged, no public item appears without `bench`, `[dependencies]` and `Cargo.lock` are unchanged, and the `wasm32` flavour gains nothing at all. | AC-003, AC-004 | Host and `wasm32` feature powersets; `git diff` of the two manifests. |
| **Reversibility** — the caller can decline, and declining costs nothing. The feature is off by default and target-gated, so not enabling it removes the family entirely rather than leaving a disabled shell; and no measurement dependency was installed that would have to be removed. | AC-003, AC-004 | `cargo hack check -p happenstance-testkit --feature-powerset --no-dev-deps`, which compiles the `bench`-off combinations. |
| **Reachability** — the analogue of keyboard reachability for a library: the capability is reachable by a documented command from a cold start, not only by reading the source. `cargo test --features bench` is stated in the module docs and the README row, and the mounted target proves the path is real. | AC-001, AC-008 | The mounted target runs; docs and README diffs read at review. |
| **Presentation exists at all** — the API-surface analogue of "every control carries real composed presentation". A macro with no module docs, no `# Errors` sections and no README row is the library equivalent of an unstyled render: every structural assertion passes and nobody can use it. The workspace lints (`missing_docs = "warn"` with `-D warnings`) enforce the floor; the README row and the CF-34 sentence are what carry the *meaning*. | AC-008 | Both `cargo doc` runs green under `-D warnings`; README/`CHANGELOG.md` reviewed. |
| **Consistent composition with its siblings** — the family sits beside the other three rather than inventing a fourth idiom: the same three macro arms, the same `__emit_*` emitter convention, the same fixture-hoisting so an emitter never names the fixture type, the same `#[cfg_attr(docsrs, doc(cfg(…)))]` marker. A caller who has invoked one family has invoked this one. | AC-001, AC-004 | Reviewed against `crates/happenstance-testkit/src/concurrency.rs:1129-1166` and `registry.rs:228-300`; the mount mirrors `tests/memory_concurrency_conformance.rs`. |

**The two anti-patterns this project does name**, both from its briefs rather than a
visual design: a **doubled driver or filesystem** in place of the real thing (testing
brief §3 — the emitter parameter is the *sole* sanctioned seam in this project), and a
**benchmark result gating a merge** (CF-34's own `Rejects:` line). AC-004 and AC-007 are
the rows that make each falsifiable.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | `--all-features` is resolved for `wasm32-unknown-unknown`, so `feature = "bench"` is on for a target with no threads and no host clock. | The module does not exist there: `#[cfg(all(feature = "bench", not(target_arch = "wasm32")))]`, exactly as `model` is gated (`crates/happenstance-testkit/src/lib.rs:175-184`) and for the reason `fixtures.rs:298-305` records. Compiles cleanly; the family is simply absent. Failing this is a compile error in the `wasm32 feature powerset` step — which is `OPTIONAL`, so it is the one condition here that can reach `main` unnoticed. |
| EC-002 | `bench.rs` spells `Instant`, `elapsed`, `std::time` or `sleep` in a code line. | The gate step *no conformance rule reads a clock* fails — `xtask/src/lints.rs:223-281` scans **every** `.rs` file under `crates/happenstance-testkit/src`, which is where this module lands, and its four needles are exactly those strings. The remedy is the one the lint's own doc comment names: move the clock into the caller's emitter (which is CF-23's shape anyway), **never** widen the lint's exclusions (`xtask/src/lints.rs:39-42`). A `core::time::Duration` handed *in* by an emitter trips nothing; reading a clock inside the testkit does. |
| EC-003 | A caller supplies a degenerate parameter — zero contenders, an empty batch, zero events to replay. | Refuse at the call with a message naming the parameter, before any store call. An empty batch is *already* refused by the contract (`append` rejects it, and before evaluating the condition), so letting it through turns a caller's typo into a benchmark of the error path reported as throughput. |
| EC-004 | The fixture under test declares a `MAX_BATCH`/payload/tag ceiling below the caller's chosen *n*, so the append scenario is refused with `ExceedsStoreLimit`. | Report the refusal in the record as a refusal — never count a rejected append as completed work, and never panic. Limits are declared facts about a fixture (`crates/happenstance-testkit/src/contract.rs`), so a benchmark meeting one is a legitimate outcome that must be legible in the output. |
| EC-005 | A contender's append fails for a transport/store reason rather than losing the race. | Distinguished from `ConditionViolated` in the record, the way `concurrency::Attempt` distinguishes `Rejected` from `Failed` (`crates/happenstance-testkit/src/concurrency.rs:214-231`). Collapsing the two would let a broken store report a perfect rejection mix. |
| EC-006 | The caller names an emitter that is not in scope, or misspells it. | The failure is a compile error in the *caller's* crate naming the missing item — a consequence of the `__emit_*` convention substituting the path verbatim (`registry.rs:228-300`). Nothing is caught and re-reported; a benchmark family that silently fell back to the default emitter would make CF-23's parameter decorative. |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| NF-001 | **Zero new dependencies, anywhere in the workspace** — including dev-dependencies of the testkit. `Cargo.lock` is unchanged; `cargo deny check` reads nothing new. | CF-23 and testing brief §3. Observed by the manifest and lockfile diff (AC-004). |
| NF-002 | **No new public surface under default features**, on any target. | Keeps the `bench` feature purely additive and semver-safe for a crate that already carries its own independent version (CF-32). Observed by the host feature powerset (AC-003). |
| NF-003 | **The mounted smoke budget stays small enough to live in the gate's test step.** The mount runs on every `cargo test --locked --workspace --all-features`; a benchmark sized for a real measurement does not belong there. Target: the new target adds no more than a couple of seconds on a developer machine, with the *real* numbers driven by the caller's parameters at ADR-0022 time. | `xtask/src/main.rs:143-153`. Observed by the wall-clock delta of the test step, recorded as evidence — not asserted in code (that would be the threshold AC-007 forbids). |
| NF-004 | **No `unsafe`, and the crate's documented bar applies unchanged**: workspace lints, `missing_docs`, `-D warnings`, `# Errors` on every fallible public fn. | `standards/rust/` and the workspace `[lints]` table. Observed by clippy and the two doc builds (AC-008). |
| NF-005 | **The family's entry point imposes no `Send` bound.** It binds `Fixture`, not a concrete store and not `SendEventStore`, so the shape stays honest for the flavour the whole two-trait design exists for — even though the module itself is absent on `wasm32`. | ADR-0001; `CLAUDE.md` binding constraint 4. Observed by review of the generated bounds and by the family compiling against `MemoryFixture` without a `Send` annotation being added anywhere. |
| NF-006 | **The feature powerset stays cheap.** `bench` is the testkit's second feature, so the host powerset goes from 2 combinations to 4. That is the ceiling this story is allowed to add — no sub-features, no `bench-criterion`, no `default`-implied variant. | `xtask/src/main.rs:546-563`. Observed by `cargo hack check -p happenstance-testkit --feature-powerset --no-dev-deps`. |

## Implementation notes (non-prescriptive)

These are constraints the spec has already accepted, plus the traps found while writing
it. Where a note reads like a design, it is a design the front half fixed — deviate and
say so in the ledger, do not deviate silently.

- **Start from `concurrency.rs`, not from `suite.rs`.** The concurrency family is the
  nearest structural precedent: its own module, its own enumeration macro
  (`concurrency.rs:1044`), its own three-arm entry point (`:1129-1166`), its own mount in
  `tests/`, and a `#[cfg]` that is load-bearing rather than tidy. Copying its shape is
  most of this story; the differences are the second `cfg` condition (a feature as well
  as a target) and the fact that nothing here is a rule.
- **The clock lives in the emitter, and this is not a preference.** EC-002 makes it
  mechanical: `xtask`'s CF-33 lint fails the gate on `Instant`, `elapsed`, `std::time`
  or `sleep` appearing in code anywhere under `crates/happenstance-testkit/src`. This
  lands CF-23 and CF-33 in the same place — the testkit *defines and drives* the
  workload, the caller's emitter *observes* it. A record field typed
  `core::time::Duration` and filled by the emitter is fine; a `let t = Instant::now();`
  inside `bench.rs` is a red gate. Plan the record around counts the harness can produce
  without a clock (events appended, batches acknowledged, committed, rejected, failed,
  matched-on-replay) and let duration arrive from outside.
- **Contention without threads.** The front half fixed the family's bound at `Fixture`
  with no `Send` (Behavior table, entry-point row). The concurrency family reaches
  contention with real threads and pays for it with `F::Store: EventStore + Send`
  (`concurrency.rs:186`). To keep the weaker bound, drive *k* append futures interleaved
  on one thread — the shape the position-visibility and re-entrancy rules already use —
  and let an adapter that wants thread-level contention supply that through its own
  emitter. If interleaving turns out not to produce a real rejection mix against a
  serialising store, that is a finding for AC-006 and belongs in the ledger and in
  HS-S0035's input, not in a quiet widening of the bound.
- **`$($callback:tt)+`, not `$cb:path`.** The enumeration macro hands its identifier list
  to a callback captured as raw token trees; a parsed `path` fragment cannot sit in
  callee position. `registry.rs:94-118` explains this and has already paid for it.
- **Hoist the fixture.** Expand to an `async fn __benchmark_fixture() -> impl Fixture` (or
  the family's equivalent) so an emitter never has to name the fixture's type —
  `concurrency.rs:1129-1166` is the pattern, and it is what makes a third-party emitter
  writable at all.
- **Two emitters ship, and the second one is the proof.** One default smoke emitter so the
  one-argument arm works, and one alternative in the same shape, invoked from the mount —
  mirroring `tests/memory_concurrency_conformance.rs:30-36`. A single emitter makes CF-23
  a claim; two make it an observation.
- **The crate-doc paragraph is prose, not a link.** `lib.rs:112-131` records the exact
  failure and its cost. Spell `bench` in backticks; do not write `[`bench`]`.
- **Order of operations that keeps the gate legible.** Land the module and the feature
  first, then the mount, then the docs/README/CHANGELOG; run `cargo run -p xtask --
  lint-clock` early rather than discovering EC-002 at the end, and run the `wasm32`
  powerset explicitly before opening the PR, because `--fast` will not.

## Tests and CI (merge gate)

Grounded in the project testing brief
(`.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md`, *Test
strategy* table and the AC-to-tier matrix at AC-012's row) and in the repository's own
wired commands (`.redkiln/config.yaml`, `verify:` block) — nothing here is invented for
this story, and **no new gate step is added**.

| tier | command / path | proves |
| --- | --- | --- |
| Story grain (wired) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | fmt, clippy `-D warnings` and tests for `happenstance-testkit` and its dependents, plus the five file-reading lints and `spec-trace` unconditionally. The story's own gate. |
| Benchmark tier (mounted, executed) | `cargo test -p happenstance-testkit --features bench -- --show-output` → `crates/happenstance-testkit/tests/memory_benchmarks.rs` | AC-001, AC-004, AC-005, AC-006, AC-007. The harness ran against a real `Fixture` — the difference between an adapter and a crate that compiles. |
| Meta-test (exists) | `cargo test -p happenstance-testkit no_orphan_rules` → `crates/happenstance-testkit/src/registry.rs:412-435` | AC-002: the registered rule set and the rules declared in `suite.rs` still agree, and neither grew. |
| Static lint (REQUIRED gate step) | `cargo run -p xtask -- lint-clock` → `xtask/src/lints.rs:223-281` | AC-007: nothing under `crates/happenstance-testkit/src` — `bench.rs` included — reads a clock. |
| Static (REQUIRED gate step) | `cargo xtask spec-trace` + `cargo xtask lints` (`reachability_static`, `.redkiln/config.yaml:48`) | AC-002: `RULE_FILES` still parses to the same rule set, no clause's citations moved, no retired rule is live. |
| Docs | `cargo doc -p happenstance-testkit --no-deps` (default features) **and** the gate's `documentation` step, `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:290-301`) | AC-008: the crate documents under both configurations, and the D13 intra-doc-link trap was avoided. |
| Feature powerset, host (OPTIONAL step) | `cargo hack check -p happenstance-testkit --feature-powerset --no-dev-deps` (`xtask/src/main.rs:546-563`) | AC-003, AC-004, NF-002, NF-006: all four `{bench} × {proptest}` combinations build, including the ones with `bench` off. |
| Feature powerset, `wasm32` (OPTIONAL step — **run it explicitly**) | `cargo hack check -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` (`xtask/src/main.rs:564-591`) | AC-003: the double `cfg` holds. This is the only step that compiles `bench` on `wasm32`, and `cargo xtask ci --fast` skips it. |
| `wasm32` mandatory | `cargo xtask wasm` | AC-003: the four required wasm32 steps, including the conformance-harness check, are unchanged by this story. |
| Integration grain (wired) | `cargo xtask ci --fast` (`integration_scoped`, `.redkiln/config.yaml:55`) | The non-terminal project's bar: everything above except the two powersets, `cargo deny` and the nightly docs.rs build. Includes the test step that executes the mount. |
| Manifest / lockfile | `git diff -- crates/happenstance-testkit/Cargo.toml Cargo.lock` | AC-004, NF-001: one `[features]` line added; `[dependencies]` and `Cargo.lock` untouched. |
| Ledger (wired) | `require_ledger: true` (`.redkiln/config.yaml:67`) → `_ledger.md` | AC-T05: every AC-### carries cited evidence. A green gate is a precondition for reading the criteria, never a substitute. |

**Not in the gate, deliberately.** No benchmark result gates anything, `.redkiln/config.yaml`'s
`verify:` block is untouched, and the testing brief's own row for this tier reads *"not
wired into any `verify:` command"*. What *is* in the gate is that the harness still
compiles, still runs, and still costs conformance nothing.

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation, in this PR |
| --- | --- | --- |
| **The one silent failure: `bench` reaching `wasm32`.** | The only step that compiles that combination is `OPTIONAL` and is skipped by `--fast`, which is this project's own integration command. A missing `not(target_arch = "wasm32")` therefore passes every gate a story normally runs and breaks the target the library's two-flavour design exists for. | AC-003 makes running the step explicitly a merge obligation, restated in the Merge DoD. The `cfg` is copied from `model` (`lib.rs:175-184`) rather than written fresh. |
| **CF-33's lint scope is wider than its name.** | *"No conformance rule reads a clock"* actually scans all of `crates/happenstance-testkit/src`, so the benchmark module — whose entire subject is timing — is inside a directory where four timing constructs are forbidden strings. Discovering this after the module is written is a rewrite, not a fix. | EC-002 and the implementation notes state it up front, with the resolution that CF-23 wanted anyway: the testkit produces counts, the caller's emitter produces durations. |
| **A default emitter that grows into a measurement library.** | The easy next step from "the default emitter reports something" is "the default emitter reports a duration", which needs a clock, which needs a dependency — and the testkit's two-dependency manifest is a load-bearing property (`.kb/decisions/0010-the-suite-must-prove-itself.md`). | NF-001 and AC-004 pin the manifest; the default emitter's job is bounded to completion + record well-formedness in AC-007. |
| **Coupling to the slice-mate (HS-S0035, `blocked_by` this story).** | ADR-0022 cannot quote a measured figure the harness cannot produce. If the scenarios are shaped for `MemoryFixture` — no rejections, no ceilings, no transaction — the ADR inherits an instrument that cannot answer its question. | AC-005 (caller-supplied *n*, *k*, *N*) and AC-006 (the rejection mix ADR-0012 asked for) are written from the *consumer's* need, and `MemoryFixture` is the mount, not the design target. |
| **Coupling to `concurrency-family-and-contender-count`.** | That story owns `concurrency::CONTENDERS` and the 8-vs-64 discrepancy. A benchmark that reads or re-uses `CONTENDERS` would silently give this story a vote on that decision. | The PR boundary excludes `concurrency.rs` including `CONTENDERS`; contender count is a benchmark *parameter*, unrelated to the conformance constant. |
| **Coupling to `sqlite-fixture-and-whole-suite` (two slices later).** | That fixture inherits this harness. Anything assumed about an in-process `Vec` — instant appends, no `ExceedsStoreLimit`, no busy timeout — becomes a bug the day a real file is behind it. | EC-004 and EC-005 require refusals and failures to be first-class in the record rather than panics or silent zeros. |
| **Gate runtime creep.** | The mount runs in every `cargo test --workspace --all-features`, on every story in this initiative from here on. | NF-003 bounds the smoke budget and makes the real numbers a caller-time parameter. |
| **Feature-surface creep.** | `bench` is public and semver-visible the moment the crate publishes; a sub-feature added later is another combination in two powersets. | NF-006 caps this story at exactly one feature with no sub-features. |

## Dependencies

**Blocks on: nothing.** `depends_on: []`. This is the first story of the first slice
(`_storymap.md`, *Merge order* §1) and the initiative's merge order puts it before every
other story in this project — nothing else may start, because AC-013 puts the decision
record before the implementation and the record needs a number this harness produces.

**Unlocks:**

| story slug | how it consumes this |
| --- | --- |
| `adr-0022-append-condition-strategy` (HS-S0035, same slice `bench-harness-and-adr`) | Runs this harness against real SQLite to measure the three append-condition candidates and the tag-storage options, and quotes the figure in ADR-0022. It is `blocked_by` this story and cannot be started with a fixme in its place. |
| `sqlite-fixture-and-whole-suite` (slice `durable-event-store`) | `SqliteFixture` inherits the family the way it inherits conformance — one macro invocation, no new dependency. |
| `postgres-and-neon-stores` (later project, initiative **DoD 5**) | The same harness is what makes *"with the position-visibility cost measured rather than estimated"* answerable at all. |

No dependency on substrate owned outside this initiative, and no project-level edge:
`projection-store-freeze` (HS-P0010) and `typed-layer-and-alpha-release` (HS-P0011) are
project dependencies recorded in `project.md`, and neither is a story dependency here.

## Anchors (progressive disclosure)

Linked, not pasted. Open each at the moment named — the Context pack above is what you
need to *start*; these are what you need to be *right*.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` (CF-34 at `:8263-8275`, CF-23 at `:7920-7931`, CF-32 at `:8200-8208`) | CF-34 is the clause this story discharges, and its `Rule: none` line plus its `Rejects:` line are the two sentences that decide the whole design. CF-23 `[FROZEN]` is why the emitter is a parameter. | Before writing a line of `bench.rs`, and again before the first commit message. | AC-002, AC-004, AC-007 |
| `xtask/src/lints.rs:223-281` (`no_clock`, `CLOCK_CONSTRUCTS`, and the scope note at `:39-42`) | The lint scans **all** of `crates/happenstance-testkit/src` for `Instant`, `elapsed`, `std::time`, `sleep`. It decides where the clock is allowed to live and its own docs forbid widening the exclusions. | Before choosing the measurement record's fields — this is the one that causes a rewrite if found late. | AC-007 |
| `crates/happenstance-testkit/src/concurrency.rs` (module docs; `:186`, `:214-231`, `:1044`, `:1129-1166`) | The nearest structural precedent: own module, own enumeration, three-arm entry point, `Attempt`'s Rejected-vs-Failed split, and the `Send` bound this story must *not* copy. | Immediately, as the shape to work from; `:214-231` again when designing the record. | AC-001, AC-005, AC-006 |
| `crates/happenstance-testkit/src/registry.rs:94-118`, `:228-300`, `:405-435` | The `$($callback:tt)+` capture and why a `path` fragment fails; the `__emit_*` emitter convention verbatim-substituted into the caller's scope; and `no_orphan_rules`, which is AC-002's machine check. | While writing the enumeration macro and the emitters. | AC-001, AC-002, AC-004 |
| `crates/happenstance-testkit/src/lib.rs:112-131` and `:168-190` | The recorded D13 failure (an intra-doc link to a `cfg`-absent module is a hard rustdoc error) and the `model` module's exact double-`cfg` + `doc(cfg(...))` declaration to copy. | When adding `pub mod bench;` and the crate-doc paragraph. | AC-003, AC-008 |
| `crates/happenstance-testkit/src/fixtures.rs:294-306` (and `MemoryFixture` above it) | The comment that states *a feature is not target-scoped* and names the step that catches it — the reasoning EC-001 rests on — plus the fixture this story mounts against. | Before writing the `cfg`, and when writing the mount. | AC-001, AC-003 |
| `crates/happenstance-testkit/tests/memory_concurrency_conformance.rs` | The mount to mirror: a target-level `#![cfg(not(target_arch = "wasm32"))]` and the family invoked twice, once per emitter — CF-23 demonstrated rather than described. | When writing `tests/memory_benchmarks.rs`. | AC-001, AC-004 |
| `crates/happenstance-testkit/src/contract.rs` | `Fixture`, `connect`, `Capability`, `NO_STORE_LIMITS` and the limits-are-declared-facts distinction the benchmark family binds against and must report rather than panic on. | When deciding what the scenarios may assume about a fixture. | AC-001, AC-004, EC-004 |
| `crates/happenstance-testkit/Cargo.toml` | The `[features]` table to extend, the two-line `[dependencies]` that must not grow, the per-target dependency tables, and the CF-32 comment on the crate's independent version. | When adding `bench = []`. | AC-004, AC-008 |
| `xtask/src/main.rs:143-153`, `:290-301`, `:361-374`, `:546-591` | The four gate steps this story interacts with: the test step that executes the mount with `--show-output`, the docs step, the REQUIRED clock lint, and the two feature powersets — one of which is the only place `bench` meets `wasm32`. | Before claiming the merge DoD; the powerset entry before opening the PR. | AC-003, AC-007, AC-008 |
| `.kb/decisions/0012-append-shape-and-preconditions.md:50-70` | Names this harness as the evidence that could lift the `&[Event]` marker, and specifies *"a realistic batch and rejection mix"* — the sentence AC-006 exists to satisfy. | When designing the contended scenario's record. | AC-005, AC-006 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision the testkit's dependency discipline and its "a rule no adapter can fail is decorative" habit descend from — the authority behind keeping measurement out of this crate. | Before adding anything to the testkit's manifest, if you are tempted. | AC-004, NF-001 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` (architecture §10 at `:451-471`; testing brief §3 at `:650-656`; the AC-to-tier row at `:638`) | The three structural constraints on this surface stated by the project itself, the sanctioned-seam rule, and the tier AC-012 is verified at. | Before finalising the scenario list, and when filling the ledger's `verifying_test` column. | AC-002, AC-004, AC-005 |
| `RUNBOOK.md:4207-4213` | The roadmap item that fixes the three scenarios and states, in the repository's own words, that adapters inherit benchmarks the way they inherit conformance — "which no other Rust DCB library does". | When deciding what the three scenarios are; do not invent a fourth here. | AC-001, AC-005 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_storymap.md` (Slices table; *Why the three foundations are foundations*; *Merge order* §1) | Fixes this story as a foundation with a named in-initiative consumer, and fixes the sequencing that makes HS-S0035 blocked on it. | When scoping — if a change feels like it belongs to a later story, this is where it is already assigned. | AC-005 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md:12-35`, `:84-89` | The signed-off no-user-facing-surface determination and its `design.capture` skip — the authority for the Interaction quality section's composition-family N/A. | If anyone asks why this story renders nothing. | AC-008 |
| `.redkiln/config.yaml` (`verify:` block, `:40`, `:48`, `:55`, `:67`) | The wired story-grain, static and integration-grain commands, and `require_ledger`. This story adds nothing to it — reading it is how you confirm that. | Before the merge DoD, and when filling `_ledger.md`. | AC-002, AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half decided** — AC-001 … AC-008, none
   added and none dropped. `_ledger.md` carries the same eight ids and the same criterion
   text verbatim.
2. **The CF-33 lint reaches the benchmark module, and that reshapes the record type.**
   The gate step named *"no conformance rule reads a clock"* is implemented as a scan of
   every `.rs` file under `crates/happenstance-testkit/src` for `Instant`, `elapsed`,
   `std::time` and `sleep` (`xtask/src/lints.rs:223-281`, `:42`). `bench.rs` lands in that
   directory. The front half's Behavior table says the contended scenario reports "the
   committed/rejected split **as well as** elapsed time"; the resolution that keeps both
   that sentence and a green REQUIRED gate step is: **the testkit never reads a clock —
   the caller's emitter does.** The harness produces counts; a duration reaches the record
   only when an emitter hands one in (a `core::time::Duration` field trips no needle). This
   is not a weakening of the front half, it is CF-23 and CF-33 landing on the same seam,
   and it is why the default emitter reports completion and counts rather than a time.
3. **Contention is interleaved, not threaded.** The front half fixed the entry point's
   bound at `Fixture` with no `Send`. The concurrency family buys thread-level contention
   with `F::Store: EventStore + Send` (`concurrency.rs:186`), which this story may not
   copy without breaking its own stated bound and DR-08. So the shipped contended scenario
   interleaves *k* append futures on one thread. If that turns out not to produce a real
   rejection mix, it is an AC-006 finding for the ledger and for HS-S0035's input — not a
   quiet widening of the bound.
4. **"Not wired into any `verify:` command" and "runs in the gate's test step" are both
   true and are not in tension.** The testing brief's row means no *new* gate step and no
   `verify:` change; the mounted target is nevertheless executed by the existing
   `cargo test --locked --workspace --all-features` step. Reconciled by NF-003 (a smoke
   budget) and AC-007 (no threshold, ever), which is what keeps an executed benchmark from
   becoming a benchmark that gates.
5. **Non-occlusion is a real invariant here, not a UI metaphor.** The gate's test step
   passes `-- --show-output` specifically so CF-18's `SKIP <rule>: …` lines reach a human
   (`xtask/src/main.rs:143-153`). A verbose default emitter would bury them. Bound into
   AC-007 as a table row rather than left as prose, so it is extracted, gated and tested.
6. **The composition family of RFC §6.7/D6 is formally N/A**, on the authority of the
   signed-off `_design.md` (no user-facing surface, approved 2026-08-12), and the state
   family is read at the public-API grain per `CLAUDE.md`'s account of what `_design.md`
   is for in this repository. Every applicable invariant is carried by an AC-### row in
   the acceptance table; the Interaction quality section only maps them.
7. **No ADR is written, amended or numbered by this story**, and no `[FROZEN]` clause
   moves. CF-34 keeps its `[PROVISIONAL]` marker and its falsifier. ADR-0022 is HS-S0035's,
   authored through the runbook's ADR queue as an atom plus a long record — never as a side
   effect of a code change (`CLAUDE.md`, *Where the work lives*).
