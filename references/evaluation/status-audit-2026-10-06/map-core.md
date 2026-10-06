# Map: contract, typed layer and testkit (code quality and completeness)

Audit of `crates/happenstance-core`, `crates/happenstance`, `crates/happenstance-testkit`
in the read-only worktree at `1f92d08` (== origin/main, "Phase 17 L6a: the workerd job,
landed red on purpose (#34)"). Date: 2026-10-06. No cargo command was run (gate owned
elsewhere); everything below comes from grep/wc/sed/git over the tree. Paths are relative
to the worktree root.

Convention: **CODE SHOWS** = verified in source or tests. **DOCS CLAIM** = stated in prose
and not independently verified here. "Unverified" = no evidence found.

---

## 1. Size and shape

Commands: `find crates/<c>/src -name '*.rs' | xargs cat | wc -l` and the same over `tests/`.
The doc/comment/code split comes from line-prefix grep (`//[/!]` = doc, other `//` = comment).

| Crate | src lines | tests/ lines | doc lines | comment lines | blank | code (incl. in-src test mods) |
|---|---:|---:|---:|---:|---:|---:|
| happenstance-core | 8,190 | 4,519 | 3,404 | 474 | 627 | 3,685 |
| happenstance (typed layer) | 5,027 | 7,617 | 1,931 | 366 | 454 | 2,276 |
| happenstance-testkit | 16,652 | 22,498 | 7,094 | 1,155 | 1,220 | 7,183 |

Doc plus comment lines make up about 45 to 50% of each `src/`. The biggest files are
`happenstance-testkit/src/suite.rs` (6,578 lines, every event-store rule body in one file),
`tests/mutation_coverage/mutants.rs` (5,262), `tests/mutation_coverage.rs` (5,119),
`src/projection.rs` (2,267), and `happenstance-core/src/store.rs` (1,371).

Versions: workspace `version = "0.4.0"` (`Cargo.toml:24`), testkit `version = "0.4.0"`
(`crates/happenstance-testkit/Cargo.toml`), MSRV `1.97.1` (`Cargo.toml:26`). The CHANGELOG's
latest released heading is `## [0.3.2] — 2026-09-20` (`CHANGELOG.md:341`). 0.4.0 sits
under `[Unreleased]` (`CHANGELOG.md:35`), and phase 17 is "in progress"
(`runbook/README.md:102`). So the manifests already carry an **unreleased** 0.4.0.

---

## 2. happenstance-core (the contract)

### Public surface
- Modules: `pub mod store`, `pub mod projection`, plus private `append, error, event,
  identity, limits, query, tag, validate`, and `memory`/`projection_memory` behind `memory`
  (`src/lib.rs:105-130`).
- Re-exports (`src/lib.rs:132-170`): `AppendCondition, Guard, AppendError,
  ConditionViolated, InvalidEventType, InvalidQuery, InvalidTag, Event, EventParts, EventType,
  SequencePosition, SequencedEvent, EventId, RecordedAt, StoreId, limits consts, StoreLimit,
  Query, QueryItem, ReadOptions, EventStore, SendEventStore, collect, read_decision_model,
  Tag, Tags, Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, ResetError,
  SendProjectionStore`, `ProjectionProbe` (behind `conformance`), and the Memory* stores
  (behind `memory`). `bytes` and `futures_core` are re-exported for type identity.
- Three traits: `EventStore` (`src/store.rs:158-159`), `ProjectionStore`
  (`src/projection.rs:425-426`), and `ProjectionProbe` (`src/projection.rs:610`). Both ports
  are `#[trait_variant::make(Send…: Send)]`.
- Rough grep counts of `pub` items: fn 91, struct 19, enum 12, trait 3, const 33;
  `#[non_exhaustive]` 16; `#[must_use]` 42.
- Feature flags (`crates/happenstance-core/Cargo.toml`): `default = ["std","memory"]`,
  `std`, `serde = ["dep:serde","dep:base64","base64/alloc","bytes/serde","serde/alloc"]`,
  `memory = ["std"]`, `conformance = []`. The crate is `no_std` without `std`
  (`src/lib.rs:99`). `unstable-projection` was removed in 0.4.0 (CHANGELOG Unreleased
  "Removed", `CHANGELOG.md:304`).
- Non-workspace deps: bytes, futures-core, thiserror, trait-variant, plus optional serde and
  base64. **No in-workspace dependency** (CODE SHOWS: the manifest's `[dependencies]`), so the
  dependency rule holds.

### Binding constraints (CLAUDE.md) checked against code

| Constraint | Status | Evidence |
|---|---|---|
| No `#[async_trait]` | **Holds** | `grep -rn '^\s*#\[async_trait' --include='*.rs' crates examples harness` returns 0. All other hits are comments or tests (`crates/happenstance/src/runner.rs:56`, `crates/happenstance/tests/manifest_contract.rs:269` `fn async_trait_is_banned`). `deny.toml:37-60` bans the crate, with named wrapper exemptions (wasm-bindgen-test; `worker` via ADR-0035). `async-trait` is in `Cargo.lock:74`, reached only through those wrappers. |
| serde not in core defaults | **Holds** | `default = ["std", "memory"]` in `crates/happenstance-core/Cargo.toml`. serde is `optional = true`. |
| `EventStore::read` non-async, returns the stream at top level | **Holds** | `src/store.rs:184-188`: `fn read(&self, query: &Query, options: ReadOptions) -> impl Stream<Item = Result<SequencedEvent, Self::Error>>;` (`append`, `head`, `contains_event_id` are `async fn`, at `:283`, `:318`, `:341`). |
| Both Send tests exist | **Holds** | `send_flavour_stream_is_send_in_generic_code` at `src/memory.rs:628` (bound written at the definition, `:644`). `spawns_from_generic` at `src/memory.rs:657` (real `tokio::spawn` holding the stream across an await, `:680-690`). The same pattern is applied one layer up in `crates/happenstance/tests/flavours.rs:117` (`commit_spawns_from_generic`) and `:549` (`run_projection_spawns_from_generic`), and in the testkit at `tests/faulty_store_send_guard.rs:56`. |
| Bind `EventStore` in generic code | **Holds in typed layer** | `commit_with` is `where S: EventStore` (`crates/happenstance/src/command.rs:411-420`). `run_projection` is `S: EventStore` (`crates/happenstance/src/runner.rs:514-525`). |

### Panics, unwraps, lints, unsafe (non-test code)
I used a script that strips `#[cfg(test)]` items and comment lines
(`scratchpad/audit-scripts/prodcount.py`), then hand-checked each hit.
- `todo!`/`unimplemented!`: **0**. Workspace `todo = "deny"` (`Cargo.toml` `[workspace.lints.clippy]`).
- `.unwrap()`/`.expect(`: **0 in production code.** The script's 1 unwrap and 5 expects
  (`src/store.rs:714-1207`) are inside `#[cfg(test)] mod module_doc` (`src/store.rs:609`) and
  the second test module (`:1258`). The script misattributed them because of string braces.
  Workspace `unwrap_used = "deny"`.
- `panic!`: 8, all in the `const fn from_static` constructors (`src/tag.rs:115-121`,
  `src/event.rs:111-119`). Each one is documented under `# Panics` and works as a
  compile-time validation for literals. This is acceptable.
- Lock poisoning is absorbed rather than unwrapped: `self.events.read().unwrap_or_else(PoisonError::into_inner)` (`src/memory.rs:198`).
- `#[allow]` in production: 3, each with a reason. `clippy::struct_field_names` on `Event`
  (`src/event.rs:319`, DCB's own term), `clippy::ref_option` on a serde `with` fn
  (`src/event.rs:695`), and `items_after_test_module` (`src/event.rs:1092`). Plus two
  `#[expect(dead_code, reason = …)]` on doc carriers (`src/store.rs:415,489`). Test modules
  use scoped `#![allow(clippy::unwrap_used)]`.
- `unsafe`: **0** non-comment occurrences in all three crates. The workspace sets
  `unsafe_code = "forbid"`.
- Docs: workspace `missing_docs = "warn"`, `missing_errors_doc`/`missing_panics_doc = "warn"`,
  `rustdoc::broken_intra_doc_links = "deny"`. The gate runs clippy with `-D warnings` and
  rustdoc with `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:518,764`), so missing docs fail
  CI. There is no crate-level `#![deny(missing_docs)]`; the workspace lint plus `-D warnings`
  does that job.

### Tests
| Kind | Count | How measured |
|---|---:|---|
| `#[test]` in src | 84 | `grep -rn '#\[test\]' crates/happenstance-core/src` |
| `#[tokio::test]` in src | 13 | same |
| `#[test]` in tests/ | 35 (10 files) | same over tests/ |
| `#[tokio::test]` in tests/ | 33 | same |
| `proptest!` blocks in tests/ | 2 | `grep 'proptest! *{'` |
| doc fences | ~39 examples (78 fence lines) | `grep -E '^\s*//[/!] ?```'` |
| `compile_fail` doctests | 2 | |
| `#[ignore]` | 0 | |

Notable test files: `tests/frozen_signatures.rs` (318 lines; generic consumers that compile
only against the frozen signatures), `tests/wire.rs` (1,864; the serde wire format),
`tests/serialize_is_borrowing.rs`, `tests/probe_live_transaction_shape.rs`, and
`tests/trait_variant_pin.rs` (excluded from the package because it reads `Cargo.lock`;
manifest `exclude`).

---

## 3. happenstance (the typed layer)

### It is no longer a "five-line facade"
CLAUDE.md's "What this is" calls it "today a five-line facade over the contract". **CODE
SHOWS otherwise:** 5,027 src lines and 2,276 non-comment code lines, made up of:

| Module | Public items | Evidence |
|---|---|---|
| `codec` | `trait Codec { const TAG; encode; decode; reads_tag }`, `CodecError`, unit structs `Json` (feature `json`), `Postcard` (`postcard`), `Cbor` (`cbor`) | `src/codec.rs:80,199,276,302,328` |
| `domain` | `trait DomainEvent { const EVENT_TYPES; event_type; tags; encode; decode }`, `trait DecisionModel: Clone { type Event; scope; apply(&mut self, Self::Event) }` | `src/domain.rs:70,232-249` |
| `boundary` | sealed `trait Boundary { type Event; query(); absorb() }`, blanket impl for every `DecisionModel` | `src/boundary.rs:69,112` |
| `composition` | `Boundary` for tuples of arity 2 to 8, with every member forced to share the **first member's** `Event` type | `src/composition.rs:24-45,95-165` |
| `command` | `commit` (JSON), `commit_with` (any codec), `Retry`, `Committed`, `CommandOutcome {Committed, Nothing}`, `CommandError` | `src/command.rs:45,80,121,163,355,411` |
| `runner` (behind `unstable-projection`) | `trait Projection`, `run_projection`, `Progressed`, `ProjectionError` | `src/lib.rs:196-197,237-239`; `src/runner.rs:62,115,142,514` |
| `testing` (behind `memory` + `json`) | `given(..).event(..).when(..).then(..)/then_refused()`, `assert_domain_event` | `src/testing/mod.rs:100-404` |
| re-exports | the contract surface, listed explicitly item by item, plus `pub use happenstance_core;` | `src/lib.rs:240-319` |

Features (`crates/happenstance/Cargo.toml`): `default = ["std","memory","json"]`, `std`,
`serde` (forwards core's envelope serde), `memory`, `json`, `postcard`, `cbor`, and
`unstable-projection = ["dep:futures-core"]`. `serde` (the crate) is an unconditional dependency.
This is allowed because ADR-0003 constrains core only.

There is no derive macro. An application hand-writes `DomainEvent`: the `EVENT_TYPES` const,
`event_type`, `tags`, `encode`, and `decode` (`src/domain.rs:70-200`). `runbook/roadmap.md:148`
records `happenstance-macros` as out of scope by ADR-0033, and an accepted open question
records the hazard of the unchecked positional mapping
(`.kb/open-questions/event-type-positional-mapping-has-no-compiler-check.md`).

### What is behind `unstable-projection`
Only the runner: `Projection`, `run_projection`, `Progressed`, `ProjectionError`
(`src/lib.rs:237-239`). The port itself (`ProjectionStore`) is ungated and frozen (ADR-0063).

### Is `Projection::apply` still synchronous? **Yes. ADR-0074 is not built.**
- CODE SHOWS: `fn apply(&mut self, event: Self::Event, batch: &mut StoreBatch<Self>) -> Result<(), StoreError<Self>>;`
  (`src/runner.rs:95-99`). It is synchronous, takes a bare `Self::Event` rather than a
  `Delivered<E>`, and returns the store's error rather than the projection's own.
  `grep -rn Delivered crates/happenstance/src` returns nothing.
- Its rustdoc still argues **for** a synchronous apply ("Why `apply` is synchronous … An
  `async fn apply` was the alternative", `src/runner.rs:50-59`). That contradicts accepted
  decision ADR-0074 (`.kb/decisions/0074-projection-apply-is-async.md`, `status: accepted`),
  which makes apply async, handed a `Delivered`, and under
  `#[trait_variant::make(SendProjection: Send)]`. This is expected, because phase 18 ("The typed
  runner leaves its gate") is **not started** (`runbook/README.md:103`). But the shipped
  rustdoc presents the losing design's argument as current rationale.

### What the command loop and `Retry` offer
- `Retry` is a required argument. It is `#[non_exhaustive]` and wraps `NonZeroU32`, so an
  unbounded or zero retry cannot be written. `Retry::attempts(NonZeroU32)` and
  `Retry::once()` exist, and a `compile_fail` doctest proves `attempts(0)` is rejected
  (`src/command.rs:14-62`).
- On `ConditionViolated` **or** `Busy` (new in 0.4.0, ADR-0077), the loop re-derives the query,
  re-reads, folds a fresh clone of the boundary, and calls the decide closure again. It never
  resubmits stale events (`src/command.rs:300-316,485-488`). It runs up to the bound, then returns
  `CommandError::Exhausted`.
- **No backoff and no jitter**, by stated design ("There is no backoff between attempts",
  `src/command.rs:314`).
- An empty decision returns `CommandOutcome::Nothing` rather than an append error
  (`src/command.rs:121-140`). `CommandError::OutsideBoundary` refuses a decided event that is
  outside its own boundary, which prevents a lost update (`src/command.rs:~226-255`). The docs
  are honest about its composite-boundary residual.
- `Committed { position, attempts }` is `#[non_exhaustive]`, so callers can read it but not
  build it.

### Runner limitations the code itself documents (`src/runner.rs:300-495`)
It runs one projection per call with no fan-out. It halts on the first failure and has no
skip policy. It only claims `Authority::Live`. Chunk size has no default, and no measurement
backs one ("a gap rather than a position", `:445`). There is **no observability**: "no
callback, no channel and no `tracing` instrumentation anywhere in this workspace" (`:455-458`;
`grep tracing --include=Cargo.toml crates` returns 0). Nothing stops two runners on one
`(store, ProjectionId)` (no lease or fencing, `:470-494`). It is a catch-up call, not a
long-running subscription.

### Panics, lints, unsafe (non-test code)
- `todo!`/`unimplemented!`/`unwrap`/`expect`: **0**.
- `panic!`: 3, all in the public `testing` DSL (`src/testing/mod.rs:304,306,325`), where
  panicking is how a test assertion fails. Acceptable.
- `#[allow]`: 2. One is on the sealed module (`src/sealed.rs:13`). The other is
  `needless_pass_by_value` on `Given::event` (`src/testing/mod.rs:143`).
- Compile-time checks: `const CHECKED: () = assert!(…)` guards make an empty `EVENT_TYPES` a
  compile error (`src/boundary.rs:160`, `src/codec.rs:381`, used in `src/runner.rs:~529`).

### Tests
`#[test]` in src 33, `#[tokio::test]` in src 9. In tests/ (20 files): `#[test]` 80 and
`#[tokio::test]` 44. Doc fences about 15 examples, `compile_fail` 3, `#[ignore]` 0. Key files:
`tests/command_loop.rs` (700), `tests/projection_runner.rs` (733), `tests/flavours.rs` (604;
the `!Send` and spawned checks), `tests/retry_without_a_database.rs` (337, using the testkit's
`FaultyStore`), and `tests/composition.rs` (447). Several files are **source-text assertions**
(`include_str!` 21 times in tests/): `doc_budget.rs` holds rustdoc to an 80-column prose limit
and a 130-line crate-root budget, and `doc_surface.rs`, `manifest_contract.rs` and
`contract_surface.rs` parse manifests and source.

---

## 4. happenstance-testkit (the conformance suite)

### Structure
- Public modules: `bench`, `concurrency`, `fixtures`, `model`, `projection`, and the hidden
  `__private`. Private: `contract`, `faulty`, `gappy`, `registry`, `suite`
  (`src/lib.rs:440-487,802`).
- Public API: `Fixture`/`ProjectionFixture`/`ConcurrentFixture` traits
  (`src/contract.rs:131,727`; `src/concurrency.rs:219`), `Capability`, `RuleOutcome`
  (`src/contract.rs:1194`), `FaultyStore`/`SendFaultyStore`, `GappyMemoryStore`, `block_on`,
  `rules`. There are 23 `#[macro_export]` macros.
- Features: `default = ["memory"]`, `memory`, `proptest` (the property generators in
  `fixtures::strategies`), and `bench = []` (benchmarks only, no clock under `src/`).
  The testkit enables core's `std`, `memory` and `conformance` unconditionally.
- Its own version (CF-32), now 0.4.0. Adapter authors are told to pin it exactly.

### Rule families and counts
Each family is enumerated in exactly **one** callback macro (CF-22). The emitters
(tokio, blocking, wasm) are parameters. Counts come from parsing each macro body
(`scratchpad/audit-scripts/enum.py`):

| Family | Enumeration | Rules | Sections / notes |
|---|---|---:|---|
| Event store | `for_each_event_store_rule!` `src/registry.rs:94` | **95** | fixture contract (3), query semantics (12), read options (18), positions (2), head (3), identity/time/membership (8), append (14), value edges (12), append conditions (16), concurrency on one thread (2), read isolation (3), read-path error arm (1), position visibility (1) |
| Projection store | `for_each_projection_store_rule!` `src/projection.rs:1937` | **17** | baseline pair, the commit path differentially, reset, reading and rebuilding. This matches CLAUDE.md's "all seventeen rules". |
| Model (proptest, stateful) | `for_each_model_rule!` `src/model.rs:813` | 1 | `ops_agree_with_the_model` |
| Concurrency (real threads, Send flavour) | `for_each_concurrency_rule!` `src/concurrency.rs:1391` | 6 | the N-contender election, disjoint boundaries, unique positions, own last position, busy-left-nothing, no partial batch |
| Benchmarks (not the bar, CF-34) | `for_each_event_store_benchmark!` `src/bench.rs:813` | 3 | |

There are 95 + 1 + 6 = 102 conformance rules for an event store. CLAUDE.md's
"101 of 101" for Postgres predates `a_busy_append_left_nothing_behind` (CHANGELOG Unreleased
"Added", 0.4.0). The handover's "97 executed" for the workerd leg (`runbook/handover.md:47`) is
a different harness; I did not reconcile it (unverified).

**Capabilities (declared as consts, and a declined rule still runs and reports the skip):**
- `Fixture`: `SECOND_HANDLE`, `REOPEN` (required), plus `READ_YOUR_OWN_WRITES`
  (default SUPPORTED), `MID_BATCH_FAULT` and `READ_FAULT` (default declined)
  (`src/contract.rs:172,243,300,334,379`).
- `ProjectionFixture`: `SECOND_HANDLE`, `RESET_REFUSAL`, `COMMIT_FAULT` (default declined)
  (`src/contract.rs:789,857,903`).
- The reference `MemoryFixture` declines `REOPEN`, `MID_BATCH_FAULT` and `READ_FAULT`
  (`src/fixtures.rs:282-313`).

### Does the suite prove itself? (the mutation registry, ADR-0010)
- Event store: `tests/mutation_coverage.rs:408` `REGISTRY` holds **84 mutants + 2
  conformant variants** (`grep -o 'kind: Kind::…'`), plus **7 `RACERS`**
  (`:3000`), which are thread-level wrong stores for the concurrency family. Its meta-tests
  include `every_rule_has_a_mutant` (`:3275`), `mutant_registry_is_exhaustive` (`:3586`),
  `mutants_fail_exactly_their_declared_rules` (`:4061`),
  `conformant_variants_pass_everything` (`:4258`), and `capability_skips_are_reported` (`:4361`).
- Projection: `tests/projection_mutation_coverage.rs:300` `REGISTRY` holds **18 mutants + 3
  conformant variants**.
- `GappedPositionStore` is the conformant control that enforces the "no literal positions"
  rule (CF-6) (`tests/mutation_coverage.rs:411-420`).
- `FaultyStore` (`src/faulty.rs`) and `GappyMemoryStore` (`src/gappy.rs`) are published
  instruments.

### Where it is invoked (`grep` for the four suite macros, excluding comments)
sqlite (event + concurrency + projection), postgres (event×3, projection, live projection),
neon (event×3, projection), cloudflare (event store only. `grep -rln "projection_store_conformance\|ProjectionStore for" crates/happenstance-cloudflare/` returns nothing, so that crate has no projection store implementation at all), ladybug
(retired, projection), `examples/outside-projection-adapter`, and the testkit's own 15 harness
files.

### Panics, lints, unsafe (non-test code)
- `todo!`/`unwrap`: 0. `expect(`: 19, all in fixture/helper constructors over literal inputs
  (`src/fixtures.rs:27-694`, `src/suite.rs:569-572`). `panic!`: 41 and `assert!`: 289, which is
  correct for a test harness.
- `#[allow]`: 9. Four are `#![allow(clippy::missing_panics_doc)]` on rule modules (the rules
  panic by design, `src/suite.rs:93`, `src/projection.rs:245`, `src/model.rs:648`,
  `src/concurrency.rs:398`). Five are `#![allow(clippy::unwrap_used, unused_imports)]` inside
  macro bodies that expand into the **caller's** test crate (`src/lib.rs:616,732`, etc.).
- `unsafe`: 0.

### Tests
`#[test]` in src 32, `#[tokio::test]` in src 23, `proptest!` 4. In tests/ (28 top-level files
plus subdirs): `#[test]` 77, `#[tokio::test]` 39, `proptest!` 1, `#[should_panic]` 20,
`#[ignore]` 0. The macro-generated tests (95+17+… per harness) are not in these counts.

---

## 5. Quality judgement (candid)

### Good, with specifics
1. **The binding constraints are enforced by tests, not just stated.** The two Send tests
   have comments that record which bounds were removed and what the compiler said
   (`src/memory.rs:657-680`). `flavours.rs` instantiates the typed layer against a real
   `!Send` store. `deny.toml` bans async-trait and names its exemptions.
2. **Illegal states are unrepresentable, and that holds in the code.** `SequencePosition`
   wraps `NonZeroU64`, `Retry` wraps `NonZeroU32` with a `compile_fail` proof, and
   `CommandOutcome::Nothing` replaces a misleading `NoEvents` error. Empty `EVENT_TYPES` is a
   compile error through `const CHECKED`. Status structs are `#[non_exhaustive]`.
3. **The production code is clean.** Across about 30k src lines there are 0 `unsafe`, 0
   `todo!` and 0 production `unwrap`. The only production panics are documented `const fn`
   literal validators. Every `#[allow]` carries a reason.
4. **The testkit takes its own falsifiability seriously, which is rare.** It has 84 + 18
   named wrong implementations, a meta-test that fails if any rule has no mutant, conformant
   controls with gapped positions and paged streams, and emitters that make the suite
   runtime-agnostic (tokio, blocking, wasm).
5. **The documentation is unusually honest about limits.** Examples: the runner's
   "indistinguishable from a hang" (`src/runner.rs:458-461`), the at-most-once-under-reissue
   caveats on `append` (`src/store.rs:229-262`), and OutsideBoundary's composite residual.

### Weak, with specifics
1. **The prose volume is a maintenance liability.** About half of every src file is
   doc or comment, much of it project history ("this paragraph said *three* through the nine
   that landed after it", `crates/happenstance-testkit/src/lib.rs:64-69`) rather than API
   guidance. Manifest comments run for dozens of lines per feature. Readers outside the project
   pay for this.
2. **Line-number citations rot, and the code admits it.** `crates/happenstance/src/runner.rs:496-501`
   says SPECIFICATION.md cites `runner.rs:401`. The spec actually cites `runner.rs:524`
   (`spec/SPECIFICATION.md:6292,6393,6521`), so the comment is already stale. The same
   comment says `spec-trace` anchors only "80 of 401" citations. Much of the source layout is
   frozen to keep these line numbers valid (`event.rs:1088-1092` keeps items after the test
   module for this reason).
3. **Many "tests" are prose linters.** 53 `include_str!` uses across the three crates' tests
   and src parse their own source, manifests and READMEs (for example `doc_budget.rs` and the
   `module_doc` tests in `store.rs:609+`). This guards the documentation, but it is brittle and
   makes routine edits costly.
4. **Rustdoc contradicts an accepted ADR.** `Projection::apply` is synchronous and its
   rustdoc argues for that (`src/runner.rs:50-59`), while ADR-0074 is accepted the other way.
   Phase 18 is not started.
5. **The typed layer has real ergonomic gaps.** There is no derive for `DomainEvent`, so
   applications write the positional mapping by hand. Tuple boundaries require one shared event
   enum (`src/composition.rs:30-31`). The runner has no observability, no default chunk, no
   lease, and no fan-out. Retry has no backoff.
6. **Some rule names overstate what they test.** `racing_conditional_appends_elect_one_winner`
   (`src/suite.rs:5837-5882`) runs two appends one after the other on one handle. Real
   parallel contention is only in the opt-in concurrency family, which the `!Send` Cloudflare
   adapter cannot run.
7. **A skipped rule is invisible by default.** A declined capability passes silently unless
   `-- --show-output` is given (`src/lib.rs:50-57`). CF-18's residual is still an open question
   (`.kb/open-questions/cf-18-residuals-after-declension-by-inheritance.md`).
8. **Single-file concentration.** `suite.rs` is 6,578 lines, and the mutation registry plus
   mutants together run past 10k lines.

### Stale records found in passing
- CLAUDE.md calls `happenstance` "a five-line facade". It is 5,027 src lines with a command
  loop, codecs, a runner and a test DSL.
- `.kb/open-questions/happenstance-facade-does-not-match-adr-0006.md` (last touched 2026-09-30)
  says `lib.rs:243` is `pub use happenstance_core::*` and that there is "no
  happenstance::happenstance_core path". HEAD has explicit item re-exports and
  `pub use happenstance_core;` at `crates/happenstance/src/lib.rs:319`; line 245 is only a
  comment mentioning the glob. The "re-export the crate" half looks resolved in code. The
  adapter feature-gating half is still true, since there is no adapter dependency.
- `runbook/handover.md:39` says PR #34 is "Not merged", but HEAD `1f92d08` is the merge of #34.
- CLAUDE.md says Postgres ran "101 of 101". The event-store family now has 95 rules, plus 6
  concurrency rules and 1 model rule (102).

### Open items tied to these crates (accepted open questions, not settled)
- ES-38 `positions_are_not_reused_after_removal`: the rule is unwritten and owned by phase 14
  (`.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`). It is absent from the
  registry (verified).
- `ProjectionId::new` is infallible and unvalidated (`projection-id-is-unvalidated.md`).
- Tuple boundaries are bound to the first member's event type
  (`tuple-boundary-heterogeneous-event-type.md`).
- `Codec` is unsealed (`should-codec-be-sealed.md`).
- `trait-variant = "0.1.3"` is a caret pin on the proc macro that emits both port flavours
  (`Cargo.toml:151`; `trait-variant-caret-resolves-past-the-locked-gate.md`).
- No instrument detects an off-poll visibility defect
  (`off-poll-adapter-visibility-defect-undetected.md`).
