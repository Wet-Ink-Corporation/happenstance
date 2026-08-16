---
item: HS-S0024
stage: spec
created: 2026-08-12T13:46:21.090Z
updated: 2026-08-12T13:46:21.090Z
template_sig: 87bbf1d0
rendered_sig: 9890d291
---

# Spec — FaultyStore<S> and GappyMemoryStore in the testkit

## Scope lock

| Anchor | Path |
| ------ | ---- |
| Initiative | `.bklg/from-contract-to-published-library/initiative.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` (AC-009, DR-07) |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/misbehaving-testkit-stores/spec.md` |
| Story map row | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:55` (M4 `testing-surface`) |
| Signed-off design (**binding**) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` — items `:350-366`, signatures `:606-626`, shape `:650`, placement `:696-700`, visibility `:749-750`, states `:960-962` |
| Architecture / testing briefs | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md:424`, `:437-441`, `:481-486`, `:503-519`, `:572-584`, `:787`, `:803-805` |
| This story's discovery | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/misbehaving-testkit-stores/discover.md` — the signal ledger, the three named mutants, and the two questions it deferred here |
| Roadmap pointer | `RUNBOOK.md:4031-4041` — phase 7's testing surface, as cited by `project.md`'s DR-07 |
| House rules | `CLAUDE.md` — *The rule that matters* and its two corollaries; binding constraints 1, 3, 4 |

## One-line PR slice

Land `FaultyStore<S>` / `SendFaultyStore<S>` and `GappyMemoryStore` in
`crates/happenstance-testkit/src/`, reachable from the testkit root — two wrapper types
per flavour rather than one impl, each shipped with the wrong caller it rejects: a retry
loop that branches on `Some(conflicting_position)`, and a handler that assumes
`position + 1`.

## Executive summary

Every store a consumer of this library can currently test against behaves perfectly.
`MemoryEventStore` names the conflicting event on every violation
(`crates/happenstance-core/src/memory.rs:380-382` returns `ConditionViolated::at(..)`) and
assigns dense positions from 1 (`:277`, `:389`), so two whole classes of caller bug are not
merely hard to test — they are **untestable**, because the input that separates a correct
caller from an incorrect one never occurs in-process. This PR ships the two stores that
produce those inputs on demand, in the crate whose job is instruments for other people's
tests.

The delta over what exists: two new public wrapper types and one new public store in
`happenstance-testkit`, a `memory` feature the testkit does not declare today, and — in the
testkit's own `tests/` — the two named wrong callers, each asserted by the *specific*
disagreement it produces rather than by a bare `#[should_panic]`. Both new stores are put
through `event_store_conformance!`, because CLAUDE.md's rule that matters admits no
exception for a store that lives in a testkit.

No conformance rule is added, no `[FROZEN]` clause is amended, and nothing under
`crates/happenstance-core/src/**` is touched. `given-when-then-dsl` (HS-S0025) is the
slice-mate that consumes `FaultyStore` to demonstrate a retry loop with no database; it is
blocked on this.

## Context pack

The decisions this story must honour, stated as decisions. Everything deeper is a
signposted anchor in the second half of this spec.

**1. `FaultyStore` is two types, and that is coherence, not taste.** `trait_variant` emits a
blanket `impl<T: SendEventStore> EventStore for T`, so one type cannot carry a hand-written
impl of each flavour — the diagnostic is `error[E0119]`, naming the macro-generated impl.
So: `FaultyStore<S: EventStore>` implements `EventStore`, `SendFaultyStore<S: SendEventStore>`
implements `SendEventStore`, and the two are siblings that share a private core
(RS-20-4, `standards/rust/20-two-flavour-ports.md:202-215`; `_design.md:650`). Read that atom
and `21-send-is-not-inherited.md` **before** writing either wrapper; the alternative is an
afternoon spent arguing with the trait solver. Everything generic in this story binds
`EventStore`, the weaker requirement that accepts both, and imports exactly one flavour name
per module (CLAUDE.md constraint 4).

**2. `violate_next` reports `conflicting_position: None`, and that is the entire point of the
fixture.** `ConditionViolated::conflicting_position` is `Option` because an adapter with no
interactive transaction — `happenstance-neon` — reports `None` legitimately
(`crates/happenstance-core/src/error.rs:135-147`). A retry loop that branches on the field
being `Some` works in-process and stops working against a remote store, which is to say it
passes every test this project could otherwise write. `ConditionViolated::unspecified()`
(`:153`) is the constructor. A `FaultyStore` that reported `Some` because it is "more useful
in tests" would have exactly the same behaviour as `MemoryEventStore` on the one axis it was
built to vary, and would prove nothing while looking like proof (discover.md, *The wrong
implementation*).

**3. The wrapper needs its own error type, and the compiler forces it.** `EventStore::Error`
is an associated type the wrapper cannot fabricate a value of: `MemoryEventStore`'s error is
`pub enum MemoryStoreError {}` (`crates/happenstance-core/src/memory.rs:291`) — uninhabited,
so `fail_next_read` has literally no `S::Error` to yield. The wrapper therefore projects
`type Error = FaultyStoreError<S::Error>` with an `Injected` arm and a `Store(#[source] E)`
arm. This is a **design completion, not a re-decision**: `_design.md:606-617` lists the three
constructors and is silent on the associated type, and the port requires one
(`crates/happenstance-core/src/store.rs:100-101`). It is recorded as **DG-1** in this story's
report so the design's item list can be amended rather than diverged from silently.

**4. `read` stays non-`async` and returns the stream at the top level.** This is binding
constraint 3 and it is exactly the constraint a wrapper is most likely to break, because
`async fn read(&self) -> impl Stream` reads more naturally and compiles. The two guards are
in `crates/happenstance-core/src/memory.rs:614` and `:643-680`; the Send flavour's wrapper
owes the same obligation, so an injected read failure is a **lazy** first item on the stream,
never an eager error at call time (ADR-0011, `.kb/decisions/0011-read-laziness-and-isolation.md`).

**5. `GappyMemoryStore` cannot be built on `MemoryEventStore`, and the reason is worth
knowing before trying.** `MemoryEventStore` allocates from its own **length** —
`position_at(first_index + offset)` where `first_index = stored.len()`
(`crates/happenstance-core/src/memory.rs:388-389`) — not from its head. Seeding it through
`restore` with strided positions therefore makes the *next* append land **below** the head,
violating VT-11's strict monotonicity. `GappyMemoryStore` is a standalone store with its own
allocator, reusing the frozen crate's public matching surface (`Query::matches`,
`AppendCondition::is_violated_by`, `ReadOptions`) exactly as `memory.rs:293-400` does.

**6. Gaps are legal, and something in this repository already depends on that.** VT-11 is
`[FROZEN]`: positions are unique and strictly increasing, and gaps are permitted
(`spec/SPECIFICATION.md:1020-1027`). ES-9's rule `read_from_a_gap_position` carries an
*interior-gap* branch that runs **only** where a fixture's own allocator left a hole
(`crates/happenstance-testkit/src/suite.rs:1533-1553`). Do not claim this story turns that
branch on for the first time: `GappedPositionStore` — `pub(crate)`, `!Send`, fixed at 4096
in steps of 7 — already does, from the mutant registry
(`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:44-116`). What this story adds
is **reach**: an equivalent instrument that is published, `Send`, and takes the caller's own
stride, so an application author outside this workspace can aim it at their own handler. The
registry's variant is CF-5's conformant control and is **not** refactored, replaced or
re-pointed by this story.

**7. Both new stores run the conformance suite.** *"An adapter that compiles but has not run
the suite is not an adapter"* (CLAUDE.md) does not exempt a store because it ships in a
testkit. `GappyMemoryStore` runs it to prove its gaps are the permitted freedom rather than a
defect; `SendFaultyStore` runs it **unarmed**, which is the strong claim — a wrapper that
mangles ordering, positions or errors when nothing is injected is exactly the failure a
delegating `read`/`append` invites. The fixtures live in `tests/`, following
`MemoryFixture`/`MemoryHandle` (`crates/happenstance-testkit/src/fixtures.rs:173-189`, `:243`,
`:270-285`) including the newtype-not-blanket-`impl` reasoning and the honest declension of a
capability with a stated reason.

**8. No conformance rule is added, and the mutant discipline still applies.** These are
fixtures, not rules: no adapter's CI can turn red because of them, and `lint-changelog`
(CF-29, `xtask/src/main.rs:406-419`) is untouched because it counts *rules*. What does apply
is ADR-0010's shape: a mutant asserted by `#[should_panic]` alone records only that
*something* failed, so a wrong caller that fails for an unrelated reason reads as proof
(`crates/happenstance-testkit/tests/fixture_instruments.rs:24-39`). Each wrong caller here is
written as a named function and asserted on the **specific disagreement** it produces — a
value comparison, not a panic.

**9. Never assert a literal position.** The one test in this story that observes positions at
all is `GappyMemoryStore`'s, and `assert_eq!(positions, [1, 3, 5])` is forbidden even though
it would be deterministic and green: it asserts the store's private allocation policy, is the
exact habit `CLAUDE.md` forbids, and would be planted in the crate that teaches adapter
authors how to test. Capture what the store assigned and assert the *relation* — strictly
increasing, and some consecutive pair separated by more than one.

**10. The `memory` feature the design names does not exist yet.** `_design.md:619-626` writes
`#[cfg(feature = "memory")]` over `GappyMemoryStore` and `:750` gates it on `memory`, but
`crates/happenstance-testkit/Cargo.toml:35-41` declares only `default = []` and `proptest`.
Implemented literally, that `cfg` is always false and the type silently does not exist. This
story declares the feature — `memory = ["happenstance-core/memory"]`, **in `default`** — so
the item the design signed off is the item a `cargo add happenstance-testkit` user meets. The
crate's unconditional dependency feature (`Cargo.toml:29`) stays: `fixtures::MemoryFixture`
needs it whatever this feature says.

**11. The testkit's version is not bumped here.** CF-32 gives it an independent number
(`xtask/src/main.rs:421-437` asserts it), and `_design.md:649` settles its alpha value;
executing that is `publish-0-2-0-alpha-1`'s. This story adds a `## [Unreleased]` entry to
`CHANGELOG.md` and nothing more.

**12. The journey slice.** This is Beat 2 of *"Choose a contract before a database"* — the
application author's first hour, at the point where *"pick a database"* and *"does the domain
model work"* stop being one decision (`_storymap.md:26`). The persona is the application
author writing their first retry loop and their first read-model handler; the humane outcome
is that both of their plausible mistakes fail **in a test they can run in a second**, rather
than in production against a store they have not adopted yet.

## Integration contract

- **Archetype**: `capability` — the instruments are the user-observable slice; there is no
  layer of this story that is a double or a `todo!()`.
- **Slice / milestone**: **M4 `testing-surface`**. Slice-mate: `given-when-then-dsl`
  (HS-S0025), which `blocks` on this story and consumes `FaultyStore` to drive a caller's
  retry loop with no database (`_storymap.md:56`). The two are implemented in one context and
  mounted as one surface; this one merges first because it depends on nothing in the project.
- **Mount point**: **`crates/happenstance-testkit/src/lib.rs`** — the testkit crate root, the
  composition root the architecture brief names for exactly these two types
  (`_decomposition.md:481-486`). The new types are `pub use`d there beside
  `pub use contract::{Capability, Fixture, …}` (`:187`), in the same act that declares their
  modules. A type constructed in a module and not re-exported at the root is not mounted:
  `_design.md:696-700` makes crate-root reachability the surface.
- **Wires into**: `happenstance_core::{EventStore, SendEventStore}` and their associated
  `Error` projection (`crates/happenstance-core/src/store.rs:96-260`);
  `AppendError` / `ConditionViolated::unspecified()` (`error.rs:135-161`); `Query::matches`,
  `AppendCondition::is_violated_by`, `ReadOptions`, `SequencePosition`, `SequencedEvent`,
  `EventId`, `StoreId`; `MemoryEventStore` + `MemoryStoreError` behind the `memory` feature;
  and, in `tests/`, the `Fixture` contract (`crates/happenstance-testkit/src/contract.rs:120-173`),
  `Capability` (`:372-430`), and `event_store_conformance!`
  (`crates/happenstance-testkit/src/lib.rs:342-357`). The design-system primitive equivalents
  here are `MemoryFixture` / `MemoryHandle` (`fixtures.rs:173-285`) — the shape every new
  fixture copies rather than reinvents.
- **Renders surfaces**: **none of the six** surface ids in `_design.md` (`crate-root-rustdoc`,
  `crate-readme`, `first-program-doctest`, `worked-example-transcript`, `dsl-failure-message`,
  `compile-fail-diagnostic`) — every one of them belongs to `happenstance`, and this story
  must not add an item to that crate's root page. What it does claim from the design's
  `## Items` block is three entries: `happenstance_testkit::FaultyStore`,
  `happenstance_testkit::SendFaultyStore`, `happenstance_testkit::GappyMemoryStore`
  (`_design.md:350-366`).
- **Public items**: the three above, plus **`happenstance_testkit::FaultyStoreError<E>`**,
  which the port's associated type forces and which the design's list does not carry — DG-1,
  raised as a design amendment rather than absorbed (context pack, decision 3). Nothing else
  becomes `pub`; the fixtures that run the suite stay in `tests/`.
- **Conformance rule(s)**: **none added.** The story is adapter-observable in the other
  direction — the existing suite is run against two new stores, and
  `read_from_a_gap_position`'s interior-gap branch (`suite.rs:1533-1553`) is the rule whose
  behaviour `GappyMemoryStore` exercises. Because no rule is added, no adapter's CI can turn
  red, and CF-29's changelog lint has nothing new to require.
- **Clause(s)**: **VT-11** (`spec/SPECIFICATION.md:1020-1027`, `[FROZEN]`) and **ES-9**
  (`:2748-2820`, `[FROZEN]`) are *consumed as written* — `GappyMemoryStore` is an instrument
  for them, not an amendment to them. No clause is edited, and no ADR is required.
- **Advances DoD scenario**: no initiative DoD scenario turns green on this story alone, and
  saying otherwise would be the overclaim this repository keeps catching. Proximately it moves
  **project DoD 6** (`cargo xtask ci --fast` green, `project.md:236-237`) and satisfies the
  testkit half of **project AC-009**; it is the hard precondition for `given-when-then-dsl`,
  which carries AC-009's user-observable half, and it feeds **initiative DoD 13** (*the gate is
  green on the assembled whole*).

## PR boundary

**In this PR**

- `crates/happenstance-testkit/src/` — the two wrapper types, `FaultyStoreError<E>`,
  `GappyMemoryStore`, their rustdoc and doctests, and the crate-root `pub use` that mounts
  them.
- `crates/happenstance-testkit/Cargo.toml` — the `memory` feature and its addition to
  `default`. No new dependency, so no `Cargo.lock` change.
- `crates/happenstance-testkit/tests/` — the two suite fixtures, the two named wrong callers
  and their conformant siblings, and the `Send`-flavour spawn guard.
- `crates/happenstance-testkit/README.md` — only if an example changes; its fences are
  compiled as doctests (`src/lib.rs:1-7`).
- `CHANGELOG.md` — one `## [Unreleased] → Added` entry naming both instruments and the defect
  each detects.
- This story's own backlog folder, for the ledger and the implementation report.

**Explicitly not in this PR**

- Anything under `crates/happenstance-core/src/**`. The frozen contract is consumed, and a
  defect found in it is recorded and routed (AC-012), never patched here.
- Any new or altered rule in `crates/happenstance-testkit/src/suite.rs`, and any change to the
  mutant registry under `tests/mutation_coverage/` — including `GappedPositionStore`, which
  stays exactly as it is (context pack, decision 6).
- Anything in `crates/happenstance/` — the given/when/then DSL is `given-when-then-dsl`'s, and
  a test double must not enter an application's dependency graph (`_design.md:696-700`).
- The testkit's version number and any publish step (`publish-0-2-0-alpha-1`).
- New public items beyond the four named above.

**Merge DoD**: `cargo xtask ci --fast` is green with `event_store_conformance!` passing
against both new stores and both wrong callers demonstrably failing; the implementer MAY also
touch `crates/happenstance-testkit/src/lib.rs`'s module declarations and re-exports to mount
this slice — that is the mount, not scope drift.

```
crates/happenstance-testkit/src/**
crates/happenstance-testkit/tests/**
crates/happenstance-testkit/Cargo.toml
crates/happenstance-testkit/README.md
CHANGELOG.md
standards/rust/**
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/misbehaving-testkit-stores/**
```

**`standards/rust/**` was added on 2026-08-16, and it admits citation re-anchoring ONLY.**
The constitution cites `crates/happenstance-testkit/src/lib.rs` and the mutation-coverage
harness by `file:line`; this story adds the "Stores that misbehave on purpose" region,
which moved every anchor below it by 41 lines, and `cargo xtask lint-constitution` is a
gate step. The story is therefore forced across its boundary or into a red gate, with no
third option. This entry permits **line-number repair to existing citations and nothing
else**: rule text, evidence selection, rule retirement and new atoms all stay outside, so
the widening cannot later be cited to justify editing a rule. This story's actual use is
eleven citations across `41-declarative-macros.md`, `62-doctests-and-harnesses.md`,
`91-adapter-authoring-recipe.md` and `60-what-a-test-must-prove.md`, every hunk a balanced
insertion/deletion. Fourth instance of this class in the initiative; HS-P0010 settled the
first three the same way and predicted the recurrence.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| Two wrapper types, one per flavour | `pub struct FaultyStore<S: EventStore>` implements `EventStore`; `pub struct SendFaultyStore<S: SendEventStore>` implements `SendEventStore` and is `Send + Sync`. One type carrying both hand-written impls is `error[E0119]`; do not attempt it | `_design.md:608-609`, `:650`; `standards/rust/20-two-flavour-ports.md:202-215`; `_decomposition.md:572-584` |
| Construction and arming | `new(inner) -> Self`, `violate_next(self, n: u32) -> Self`, `fail_next_read(self, n: u32) -> Self` — all `#[must_use]`, all by value, private fields. Counters are interior-mutable (`core::sync::atomic::AtomicU32`, not `Cell`) because `append`/`read` take `&self` and the `Send` flavour must stay `Send + Sync`; `n = 0` arms nothing and is legal | `_design.md:611-617`, `:749`; `crates/happenstance-core/src/store.rs:213-217` (`&self`) |
| The injected append failure | Each armed append returns `Err(AppendError::ConditionViolated(ConditionViolated::unspecified()))` — `conflicting_position` is **`None`**, which a remote store legitimately reports — and leaves the inner store **byte-identical** (the inner `append` is never called). The counter decrements once per armed call; when it reaches zero the call delegates | `crates/happenstance-core/src/error.rs:135-157`; `_design.md:613-615`; ES atomicity, `store.rs:141-147` |
| The injected read failure is lazy | `read` stays non-`async` and returns the stream at the top level. An armed read returns a stream whose **first polled item** is `Err(FaultyStoreError::Injected)`; nothing fails at call time, and on the `Send` flavour the returned stream is still `Send` | `CLAUDE.md` constraint 3; `crates/happenstance-core/src/store.rs:103-124`; `.kb/decisions/0011-read-laziness-and-isolation.md`; `crates/happenstance-core/src/memory.rs:614`, `:643-680` |
| `FaultyStoreError<E>` | `#[non_exhaustive] pub enum FaultyStoreError<E> { Injected, Store(#[source] E) }`, `thiserror`-derived, satisfying `core::error::Error + 'static`. Required because `S::Error` may be uninhabited — `pub enum MemoryStoreError {}` — so no `S::Error` value can be produced. Never a `String` (RS-30-2) | `crates/happenstance-core/src/memory.rs:291`; `crates/happenstance-core/src/store.rs:100-101`; `standards/rust/30-error-taxonomy.md` |
| Unarmed, the wrapper is transparent | With nothing armed, every method delegates and the wrapper is indistinguishable from its inner store: `head`, `contains_event_id`, ordering, `ReadOptions` handling and assigned positions all come from the inner store unchanged. Proved by running the full conformance suite against it | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-testkit/src/fixtures.rs:189-241` (delegation shape) |
| `GappyMemoryStore` | `#[cfg(feature = "memory")] pub struct GappyMemoryStore` with `#[must_use] pub fn with_stride(stride: core::num::NonZeroU64) -> Self`. A standalone `SendEventStore` — implementing the `Send` flavour gets the bare one free and keeps the instrument usable inside `tokio::spawn`; its own allocator assigns `previous + stride` so `position + 1` is never the next position. `type Error = MemoryStoreError` (uninhabited: it cannot fail on its own) | `_design.md:619-626`, `:750`; `crates/happenstance-core/src/store.rs:14-20`; `crates/happenstance-core/src/memory.rs:291-293` |
| Why it is not built on `MemoryEventStore` | That store allocates from its **length**, not its head (`position_at(first_index + offset)`, `first_index = stored.len()`), so a store seeded with strided positions via `restore` allocates its next position *below* its own head — a VT-11 violation. The gappy store reuses `Query::matches`, `AppendCondition::is_violated_by` and `ReadOptions` directly, modelled on the reference store's append/read bodies | `crates/happenstance-core/src/memory.rs:277`, `:388-389`, `:293-400`; `spec/SPECIFICATION.md:1020-1027` |
| It is a *conformant* store | Positions unique and strictly increasing, `ReadOptions::from` evaluated as a range predicate in both directions, `head()` the highest visible position, append atomic and condition-checked before the write. Asserted by `event_store_conformance!` against a fixture in `tests/`, which also runs ES-9's interior-gap branch | `spec/SPECIFICATION.md:1020-1027`, `:2750-2775`; `crates/happenstance-testkit/src/suite.rs:1490-1555` |
| The two suite fixtures | `tests/`-local, not published: one over `SendFaultyStore<MemoryHandle>` with nothing armed, one over an `Arc`-shared `GappyMemoryStore` handle. Both follow `MemoryFixture` — a handle **newtype** rather than a blanket `impl EventStore for Arc<S>` (coherence), `SECOND_HANDLE` supported, `REOPEN` declined with a stated non-empty reason | `crates/happenstance-testkit/src/fixtures.rs:173-189`, `:243-290`; `crates/happenstance-testkit/src/contract.rs:120-173`, `:372-430` |
| Wrong caller #1 — the retry loop that branches on `Some` | A named function in `tests/` that retries only when `conflicting_position.is_some()`. Driven against `FaultyStore::…violate_next(1)` it never retries and surfaces the violation as a failure; its conformant sibling — retry on `is_condition_violated()` — succeeds on the second attempt. The test additionally asserts the reported `conflicting_position` **is `None`**, so a fixture that "helpfully" reported `Some` cannot pass it | `crates/happenstance-core/src/error.rs:135-147`, `:253`; discover.md, *The wrong implementation*; `_decomposition.md:503-508` |
| Wrong caller #2 — the handler that assumes `position + 1` | A named function computing its next expected position as `previous + 1`. Against `GappyMemoryStore` its computed value **disagrees with the position the store assigned**, asserted as that comparison; the conformant sibling resumes with `ReadOptions::from(checkpoint.next())` as a threshold and answers "caught up?" by comparing with `head()`, never by subtraction | `crates/happenstance-core/src/store.rs:240-247`; `crates/happenstance-core/src/projection.rs:104-106`; `.kb/decisions/0013-position-assignment-and-visibility.md` |
| Mutant assertion discipline | Neither wrong caller is asserted by a bare `#[should_panic]`: that records only that *something* failed, which is the shape CF-2/ADR-0010 rejects by name. Each asserts the specific value-level disagreement, and each ships beside its conformant sibling so "and the right one works" is asserted rather than assumed | `crates/happenstance-testkit/tests/fixture_instruments.rs:24-39`; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| No literal positions, anywhere | Every assertion that touches positions compares against what the store returned from `append`. `assert_eq!(positions, [1, 3, 5])` is forbidden even where it would be deterministic | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-testkit/src/suite.rs:1562-1567` (CF-6 note) |
| Flavour guard | A generic function in `tests/` bounded `S: SendEventStore + Send + Sync + 'static` holds a `SendFaultyStore`-wrapped read stream across an `await` inside a real `tokio::spawn`, modelled on the core test that exists to reject the `async fn read` refactor. Native-only; the wasm32 harness check compiles the rest | `crates/happenstance-core/src/memory.rs:643-680`; `CLAUDE.md` constraints 1 and 3; `xtask/src/main.rs:231-245` |
| Feature and reachability | `crates/happenstance-testkit/Cargo.toml` gains `memory = ["happenstance-core/memory"]` and `default = ["memory"]`; the unconditional `happenstance-core` dependency feature stays. `GappyMemoryStore`'s test file carries the matching `#![cfg(feature = "memory")]` so the feature powerset still compiles. All four items are `pub use`d at the crate root; none is `#[doc(hidden)]` | `crates/happenstance-testkit/Cargo.toml:29`, `:35-41`; `_design.md:696-700`, `:751`; `xtask/src/main.rs:546-556`, `:564-592` |
| Documentation obligations | Each public item carries a first sentence that is a complete claim, a runnable doctest driven by `happenstance_testkit::block_on` (no runtime dependency added), an `# Errors` section on every fallible function naming conditions rather than types, and — on each store — the named wrong implementation it exists to reject | `crates/happenstance-testkit/src/registry.rs:310-330`; `standards/rust/70-rustdoc-obligations.md`; `CLAUDE.md`, *The rule that matters*, first corollary |
| What does not change | No rule in `suite.rs`; no entry in `tests/mutation_coverage/`; no file under `crates/happenstance-core/src/`; no clause in `spec/SPECIFICATION.md`; no testkit version bump. `cargo xtask spec-trace` and `lint-changelog` see no new obligations | `xtask/src/main.rs:406-437`; `_storymap.md:152-154` |

## Data and migrations

**N/A — no persisted data, no schema, no wire format, and no dependency change.** Stated
rather than left blank, because three things in this story look like data and are not:

- **The gappy allocator's state** is a `Vec<SequencedEvent>` behind an `RwLock` plus a
  next-position cursor, per process and per store instance. It never leaves memory, is not
  serialised, and is not `Snapshot`/`restore`-compatible with `MemoryEventStore` — this story
  adds no counterpart to those methods.
- **The arming counters** on `FaultyStore` are process-local `AtomicU32`s, shared by clones of
  a handle so that a second handle onto the same fixture observes the same remaining count.
  This is behaviour a fixture author must be told about in the rustdoc, not a migration.
- **The manifest change is additive.** `memory` is a new feature added to `default`, forwarding
  to a `happenstance-core` feature the testkit already enables unconditionally; a feature only
  ever *adds* (RS-51-1). No new third-party dependency is introduced, so `Cargo.lock` and
  `cargo deny`'s licence set are unchanged, and `[workspace.dependencies]` is untouched.

The only externally visible "migration" is the testkit's public surface growing by four items,
which is a semver-minor event on that crate's own CF-32 number — recorded in `CHANGELOG.md`
here and *executed* by `publish-0-2-0-alpha-1`.

## Acceptance criteria

The persona throughout is the **application author in their first hour** — Beat 2 of *"Choose
a contract before a database"* (`_storymap.md:26`), the point at which *"pick a database"* and
*"does the domain model work"* stop being one decision. A second reader appears in AC-005 and
AC-008: the **adapter author**, for whom this crate is the bar, and who must not be taught a
habit by an instrument that does not hold itself to it.

Every criterion below is a goal crossing the full stack — from what the author types to what
the compiler, the suite or the rendered page does about it. Together they carry project
**AC-009**'s testkit half (`project.md:192-196`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an application author who has just run `cargo add --dev happenstance-testkit` and wants to test a retry loop without adopting a database, **WHEN** they write `use happenstance_testkit::{FaultyStore, SendFaultyStore, GappyMemoryStore, FaultyStoreError};` and construct `SendFaultyStore::new(MemoryEventStore::new()).violate_next(1)`, **THEN** all four names resolve at the **crate root** with no private module in the path, no `#[doc(hidden)]`, and no second crate to add — the instruments are opened-on-demand as *one* act (adding the testkit), never two | The doctest on each of the four items in `crates/happenstance-testkit/src/faulty.rs` / `src/gappy.rs`, compiled by `cargo test --doc -p happenstance-testkit`, imports **only** from `happenstance_testkit::` root; plus `crates/happenstance-testkit/tests/faulty_store_instruments.rs::instruments_are_reachable_from_the_crate_root`, which imports all four by root path and constructs each |
| AC-002 | **GIVEN** an author whose command loop must survive a store that refuses an append without naming the conflict — the case `happenstance-neon` produces routinely and no in-process store produces at all, **WHEN** they arm `violate_next(1)` and append, **THEN** the call returns `AppendError::ConditionViolated` whose `conflicting_position` is **`None`**, and the inner store is left exactly as it was — the same events, the same head, the same assigned positions — because the inner `append` was never called | `crates/happenstance-testkit/tests/faulty_store_instruments.rs::injected_violation_reports_no_conflicting_position` asserts the variant *and* `conflicting_position.is_none()`; `::injected_violation_does_not_touch_the_inner_store` snapshots `head()` and the full read before and after the armed append and asserts equality against values the store itself returned |
| AC-003 | **GIVEN** an author who wrote the retry loop everyone writes first — retry only when the store tells me *which* event conflicted, **WHEN** that loop runs against `FaultyStore::new(..).violate_next(1)`, **THEN** it fails **in their own test suite, in under a second, with no database**, while the sibling loop that branches on `is_condition_violated()` succeeds on its second attempt — and the counter, having reached zero, delegates from then on, so the fixture is reversible rather than permanently poisoned | `crates/happenstance-testkit/tests/faulty_store_instruments.rs::retry_gated_on_some_conflicting_position_never_retries` (asserts the loop's returned error, not a bare panic — the shape CF-2/ADR-0010 rejects) beside `::retry_gated_on_is_condition_violated_succeeds_on_the_second_attempt`, which asserts the committed position came from the store's own `append` return and that a third append with nothing armed still delegates |
| AC-004 | **GIVEN** an author testing the read path of a handler that must not fall over when the store dies mid-stream, **WHEN** they arm `fail_next_read(1)` and call `read`, **THEN** nothing fails at call time — `read` is still not `async` and still hands back the stream at the top level — the failure arrives as the **first polled item** (`Err(FaultyStoreError::Injected)`), and on the `Send` flavour that stream is still `Send`, provably: it survives being held across an `await` inside a real `tokio::spawn` | `crates/happenstance-testkit/tests/faulty_store_instruments.rs::injected_read_failure_is_the_first_polled_item_not_a_call_time_error`; and `crates/happenstance-testkit/tests/faulty_store_send_guard.rs::spawns_from_generic_over_send_faulty_store`, a generic fn bounded `S: SendEventStore + Send + Sync + 'static` modelled on `crates/happenstance-core/src/memory.rs:643-680`, native-only, which fails to compile if `read` is ever refactored to `async fn` |
| AC-005 | **GIVEN** an adapter author who will copy whatever this crate does, and an application author who must be able to trust that a *disarmed* fixture is not quietly lying about ordering or positions, **WHEN** `SendFaultyStore` is put through the full conformance suite with nothing armed, **THEN** it passes every rule — ordering, `ReadOptions` in both directions, `head()`, atomicity, identity — because the wrapper delegates rather than reimplements, and its own fixture declines any capability it cannot offer **with a stated reason** rather than vanishing from the binary | `crates/happenstance-testkit/tests/faulty_store_conformance.rs` invokes `happenstance_testkit::event_store_conformance!(FaultyFixture::new())` over an unarmed `SendFaultyStore<MemoryHandle>`; the fixture follows `MemoryFixture` (`crates/happenstance-testkit/src/fixtures.rs:173-189`, `:243-290`) — handle **newtype**, `SECOND_HANDLE` supported, `REOPEN` declined with a non-empty reason that the suite reports |
| AC-006 | **GIVEN** an author who read the design's item list and expects `GappyMemoryStore` to exist in a default `cargo add happenstance-testkit`, **WHEN** they build with default features and again with `--all-features`, **THEN** the type is present in both — because the `memory` feature the `#[cfg]` names is **declared** and is **in `default`** — and on docs.rs it renders with a `doc_cfg` gate badge naming that feature rather than appearing ungated or not at all | `crates/happenstance-testkit/Cargo.toml` declares `memory = ["happenstance-core/memory"]` and `default = ["memory"]`; `crates/happenstance-testkit/tests/gappy_store_instruments.rs` carries `#![cfg(feature = "memory")]` and is collected under both default and `--all-features` runs of `cargo test -p happenstance-testkit`; the badge is proved by the gate's nightly `--cfg docsrs` rustdoc step (`xtask/src/main.rs`, `OPTIONAL`) against the crate's existing `[package.metadata.docs.rs]` block (`Cargo.toml:56-58`) |
| AC-007 | **GIVEN** an author writing their first read-model handler, who computed the next position as `previous + 1` because that is what every store they have ever used did, **WHEN** that handler runs against `GappyMemoryStore::with_stride(..)`, **THEN** its computed position **disagrees with the position the store actually assigned** and the test says so as a value comparison — while the sibling handler that resumes with `ReadOptions::from(checkpoint.next())` and answers *am I caught up?* by comparing against `head()` processes every event exactly once | `crates/happenstance-testkit/tests/gappy_store_instruments.rs::handler_assuming_position_plus_one_disagrees_with_the_store` (asserts `computed != assigned`, both captured from the store) beside `::handler_resuming_past_an_inclusive_checkpoint_sees_every_event_once`, which drives the resume through `SequencePosition::next` and `ReadOptions::from` per `crates/happenstance-core/src/projection.rs:104-106` |
| AC-008 | **GIVEN** an adapter author who will read this crate to learn what a legitimate store may do, **WHEN** `GappyMemoryStore` is put through the conformance suite, **THEN** it **passes** — proving its gaps are the freedom VT-11 grants and not a defect — ES-9's interior-gap branch runs against it, and no assertion anywhere in this story names a literal position value: every position claim is a *relation* (strictly increasing; some consecutive pair separated by more than one) measured against what the store returned | `crates/happenstance-testkit/tests/gappy_memory_conformance.rs` invokes `event_store_conformance!(GappyFixture::new())` over an `Arc`-shared handle, exercising `read_from_a_gap_position`'s interior-gap branch (`crates/happenstance-testkit/src/suite.rs:1533-1553`); `crates/happenstance-testkit/tests/gappy_store_instruments.rs::assigned_positions_are_strictly_increasing_with_at_least_one_gap` states the relation; reviewer check: `rg -n "assert_eq!\(.*\[\s*[0-9]" crates/happenstance-testkit/tests/gappy_store_instruments.rs` returns nothing |
| AC-009 | **GIVEN** an author whose test must distinguish *the fixture broke this on purpose* from *my store failed*, **WHEN** either failure reaches them, **THEN** they match on `FaultyStoreError::Injected` versus `FaultyStoreError::Store(e)` — a typed, `#[non_exhaustive]`, `core::error::Error` value whose `Store` arm keeps the inner error reachable through `source()` rather than occluding it behind a rendered string — and the wrapper compiles over a store whose `Error` is **uninhabited** (`pub enum MemoryStoreError {}`) with no `unwrap`, `expect` or `unreachable!` anywhere in the path | `crates/happenstance-testkit/tests/faulty_store_instruments.rs::injected_and_store_errors_are_distinguishable_and_the_source_chain_survives` asserts the discriminant and walks `source()`; the uninhabited case is proved by the crate compiling at all over `MemoryEventStore` (`crates/happenstance-core/src/memory.rs:291`); reviewer check: `rg -n "unwrap\(\)|expect\(|unreachable!" crates/happenstance-testkit/src/faulty.rs crates/happenstance-testkit/src/gappy.rs` returns nothing |
| AC-010 | **GIVEN** a reader who meets these four items on the testkit's rendered crate-root page and has not read this spec, **WHEN** they scan the item table and open one item, **THEN** each row shows a **complete first-sentence claim of ≤ 80 characters that is not truncated with an ellipsis**, each identifier is ≤ 24 characters, each item page carries a runnable doctest driven by `happenstance_testkit::block_on` (no runtime dependency added), an `# Errors` section on every fallible function naming conditions rather than types, and — on each store — the **named wrong implementation it exists to reject**, so the instrument teaches what it is for without this document | `cargo test --doc -p happenstance-testkit` compiles every doctest; `cargo doc` under the gate's `-D warnings` and the crate's existing `missing_docs` posture catches an undocumented `pub` item; reviewer checks against the design's density budget (`_design.md:845-857`) and anti-patterns 5 and 6 (`_design.md:983-987`): first sentences ≤ 80 chars, identifiers ≤ 24 (`FaultyStore` 11, `SendFaultyStore` 15, `GappyMemoryStore` 16, `FaultyStoreError` 16), doc prose ≤ 80 columns, code inside doc fences ≤ 72 columns |

## Interaction quality

This story renders **none of the six surfaces** `_design.md` enumerates — all six belong to
`happenstance` (Integration contract, *Renders surfaces*). It nevertheless renders a surface in
the same medium: the **testkit's own rustdoc crate-root page and four item pages**, which is
where an application author actually meets these instruments. The design's composition rules
are written for that medium and apply here unchanged; the state rules translate to the
library's own vocabulary, where *"the control did not move the user's place"* reads as *"the
instrument did not move the store's state"*.

Every invariant below is carried by an **AC row in the table above** — none is a loose bullet,
because a bullet here would get no ledger row and would never be gated.

**STATE invariants**

| Invariant (library reading) | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — an injected failure perturbs exactly the thing it claims to and nothing else; the inner store is byte-identical afterwards because its `append` was never called | **AC-002** | `injected_violation_does_not_touch_the_inner_store` — before/after snapshot of `head()` and the full read, compared against store-returned values |
| **Non-occlusion** — a real store failure is never hidden behind the fixture's own error; `FaultyStoreError::Store(e)` keeps `e` reachable through `source()`, and neither arm is flattened to a `String` (RS-30-2) | **AC-009** | `injected_and_store_errors_are_distinguishable_and_the_source_chain_survives` |
| **Preserved "selection"** — the analogue of not losing focus/scroll: an *unarmed* wrapper preserves the inner store's ordering, `ReadOptions` handling, `head()`, identity and assigned positions exactly | **AC-005** | the full `event_store_conformance!` suite against an unarmed `SendFaultyStore` |
| **Reversibility** — arming is bounded and self-undoing: the counter decrements once per armed call, reaches zero, and the wrapper delegates again; `n = 0` arms nothing and is legal, so no fixture is permanently poisoned | **AC-003**, **EC-001** | `retry_gated_on_is_condition_violated_succeeds_on_the_second_attempt`, which appends a third time with nothing armed and asserts delegation |
| **Reachability** — the analogue of keyboard reachability: every shipped instrument is reachable by its documented public path in one act, with no private module, no `#[doc(hidden)]`, and no second crate beyond the testkit itself | **AC-001** | root-only imports in every doctest, plus `instruments_are_reachable_from_the_crate_root` |
| **Laziness is a state promise, not a detail** — an armed read fails where a real store fails, at the first poll, never at call time; and the stream stays `Send` on the `Send` flavour | **AC-004** | `injected_read_failure_is_the_first_polled_item_not_a_call_time_error` + `spawns_from_generic_over_send_faulty_store` |

**COMPOSITION invariants** — from `_design.md`, binding

| Invariant | Design source | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Presentation exists at all** — every public item carries real composed rustdoc: a complete first-sentence claim, a runnable doctest, `# Errors` where fallible, and the named wrong implementation it rejects. A `pub` item with a bare one-line summary is the library equivalent of unstyled markup | `_design.md:751` (*"an item worth shipping is worth documenting"*); `standards/rust/70-rustdoc-obligations.md` | **AC-010** | `cargo test --doc -p happenstance-testkit`; `cargo doc` under `-D warnings` |
| **Placement / composition** — the four items are `pub use`d at `crates/happenstance-testkit/src/lib.rs` so they appear in the crate's item table; hierarchy is controlled by the three levers the design allows (module-doc position, root re-export, `doc_cfg` badges) and by nothing else — no `#[doc(hidden)]`, no inline HTML | `_design.md:696-700`, `:903-906`, `:751` | **AC-001**, **AC-010** | root-path imports in tests and doctests; reviewer check that no new item is `doc(hidden)` |
| **Transience — opened on demand** | `_design.md:832` assigns `FaultyStore`/`GappyMemoryStore` **opened on demand**: *"a second crate; an application should not carry a test double in its graph"* | **AC-001** (they live in the testkit and are reached by adding it), and the PR boundary's prohibition on touching `crates/happenstance/` | the PR file allowlist; `crates/happenstance/Cargo.toml` is untouched, so no application acquires a test double transitively |
| **Density budget, with its real numbers** — first sentence **≤ 80 characters** and never ellipsis-truncated in the item table; public identifier **≤ 24 characters**; doc prose **≤ 80 columns**; code inside a doc fence **≤ 72 columns**; code line width **100 columns** (rustfmt) | `_design.md:845-857` | **AC-010** | rustfmt + reviewer measurement against the four names, all of which comply today (11 / 15 / 16 / 16) |
| **Hierarchy** — a gated item is marked, not demoted: `GappyMemoryStore` renders with a `doc_cfg` badge naming `memory`, in a crate that already declares the docs.rs metadata | `_design.md:903-906`; anti-pattern 6 (`_design.md:985-986`) | **AC-006** | the nightly `--cfg docsrs` rustdoc step; `Cargo.toml:56-58` |
| **Named anti-pattern 5** — *an item table row whose first-sentence column ends in an ellipsis* | `_design.md:983-984` | **AC-010** | first sentences held to ≤ 80 characters and reviewed as rendered claims |
| **Named anti-pattern 6** — *a feature-gated item's page shows no gate badge while `Cargo.toml` gates it* | `_design.md:985-986` | **AC-006** | the feature is really declared and really in `default`; the badge step runs |
| **The standing set, not re-litigated** — no `#[async_trait]`; `read` returns the stream at the top level; generic code binds `EventStore`, never `SendEventStore`; no `unwrap`/`expect` in library code | `_design.md:990-993`; `CLAUDE.md` constraints 1, 3, 4 | **AC-004**, **AC-009** | the spawn guard; `rg` for `unwrap(`/`expect(`/`unreachable!` over the two new source files returning nothing |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | `violate_next(0)` or `fail_next_read(0)` | Legal and inert. Arms nothing, panics nothing, and the wrapper stays transparent. Documented on both constructors so a caller writing `violate_next(attempts - 1)` is not surprised |
| EC-002 | An armed append fires | The inner `append` is **not** called. No event is written, no position is allocated, `head()` is unchanged. A wrapper that appends and then reports failure would be a fixture that lies about atomicity — the very property ES asserts (`crates/happenstance-core/src/store.rs:141-147`) |
| EC-003 | Two handles onto one `FaultyStore` fire armed calls concurrently | The counters are shared `AtomicU32`s, so the **total** number of injected failures across handles is at most `n` and never wraps below zero. Which handle receives which failure is unspecified and must be *documented as unspecified* rather than implied by a single-threaded test |
| EC-004 | The inner store returns its own error while nothing is armed | Surfaced as `FaultyStoreError::Store(e)` with `e` intact under `source()`. Never re-labelled `Injected`, never swallowed, never stringified |
| EC-005 | `S::Error` is uninhabited (`pub enum MemoryStoreError {}`) | The wrapper still compiles and still has a usable `Error`, because it projects its own `FaultyStoreError<S::Error>` rather than trying to produce an `S::Error`. No `unreachable!()` is written to "handle" the impossible arm |
| EC-006 | `GappyMemoryStore` exhausts its position space (`previous + stride` overflows) | Checked arithmetic, never a silent wrap — a wrapped position would break VT-11's strict monotonicity, which is the one property this store exists to demonstrate it *keeps*. The disposition (a documented panic named in a `# Panics` section, or a refusal) is the implementer's call under `standards/rust/11-const-construction-and-panics.md`, and must be **stated in the rustdoc**. It is documented rather than asserted: reaching it requires ~2⁶⁴/stride appends, and a test that cannot run is not evidence |
| EC-007 | A new fixture declines a capability | It declines with a **non-empty stated reason** that the suite reports, per the `Capability` constructor (`crates/happenstance-testkit/src/contract.rs:372-430`). A rule whose capability is declined still runs and still reports; it never vanishes from the binary (RS-40-5) |
| EC-008 | An armed read is created but never polled | Nothing happens, and the counter is decremented at the point the *stream is produced*, not at first poll — so `read`'s arming semantics are stated once in the rustdoc and are the same whether or not the caller drains. Whichever the implementer picks, the choice is documented and asserted by AC-004's test |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| NF-001 | **No new third-party dependency.** `Cargo.lock` is unchanged, `[workspace.dependencies]` is untouched, and `cargo deny`'s licence set is unchanged | The testkit is a dev-dependency for everyone who uses it; every crate it pulls in is a crate their build carries. `git diff --stat Cargo.lock` is empty; `cargo deny` (gate `OPTIONAL`, resolves on this machine) sees nothing new |
| NF-002 | **The edge flavour survives.** `SendFaultyStore` is `Send + Sync`; no `#[async_trait]` appears; the testkit's `wasm32` harness check still compiles | CLAUDE.md constraints 1 and 3; `cargo xtask wasm`'s conformance-harness step, and the native-only `tokio::spawn` guard for the `Send` half |
| NF-003 | **Feature independence.** `cargo hack` feature-powerset over the testkit compiles at every combination, including `--no-default-features` — which must exclude `GappyMemoryStore` *and* its test file cleanly, not fail to build | A feature only ever adds (RS-51-1, `standards/rust/51-features-and-no-std.md`). The `#![cfg(feature = "memory")]` at the top of the gappy test file is what makes the powerset honest |
| NF-004 | **Gate hygiene.** `clippy -D warnings` clean; no `unwrap`/`expect`/`unreachable!` in the two new library modules; no `#[doc(hidden)]`; no `#[allow]` added without a named phase | `standards/rust/00-prime-directives.md`; `cargo xtask affected --base main` runs fmt + clippy `-D warnings` for the affected package set |
| NF-005 | **The suite runs twice more and the gate stays cheap.** Two additional `event_store_conformance!` expansions are added to a crate that already runs several; the added wall-clock on `cargo xtask ci --fast` is expected to be seconds, and is **observed, not gated** (CF-34: performance is measured by a separate harness and is not the bar) | If either new conformance target materially dominates the test step, the fixture — not the rule — is what gets cheaper, and the observation is recorded in the implementation report |
| NF-006 | **Determinism.** No test in this story depends on wall-clock timing, thread interleaving or iteration order of a hash container. EC-003's concurrent case asserts a bound (*at most `n` injected failures*), never a schedule | `standards/rust/60-what-a-test-must-prove.md`; a flaky instrument in the crate that teaches testing is worse than no instrument |

## Implementation notes (non-prescriptive)

These are observations that save time, not instructions. Where one conflicts with something
the compiler says, the compiler wins and the report records it.

- **Read RS-20-4 and RS-21 before the first line of either wrapper.** The two-types shape is
  not a preference and the diagnostic (`error[E0119]`, naming a macro-generated impl) is
  confusing enough that rediscovering it costs an afternoon. `standards/rust/20-two-flavour-ports.md:202-215`
  states it with the compiled example.
- **A shared private core is the obvious way to keep the siblings honest.** A `struct
  FaultyCore { violate: AtomicU32, fail_read: AtomicU32 }` (or equivalent) held by both
  wrappers means the arming logic exists once; only the trait impls differ. `AtomicU32` rather
  than `Cell` because `append`/`read` take `&self` (`crates/happenstance-core/src/store.rs:213-217`)
  and the `Send` flavour must stay `Sync`.
- **Delegating `read` is where the flavour bugs live.** Write the signature by hand from the
  port (`crates/happenstance-core/src/store.rs:103-124`) rather than letting an IDE infer it,
  and resist `async fn read` — the guard test in AC-004 exists because that refactor compiles.
- **Model the gappy allocator on the reference store's bodies, not on its data structure.**
  `crates/happenstance-core/src/memory.rs:293-400` shows how `Query::matches`,
  `AppendCondition::is_violated_by` and `ReadOptions` are used together; copy the *use*, and
  give the store its own cursor rather than deriving positions from a length (context pack,
  decision 5, and `memory.rs:388-389` for why).
- **Copy `MemoryFixture` rather than inventing a fixture shape.** `crates/happenstance-testkit/src/fixtures.rs:173-189`
  and `:243-290` carry the handle-newtype reasoning (coherence forbids a blanket
  `impl EventStore for Arc<S>`) and the capability-declension idiom.
- **Name each mutant test after the wrong belief it holds**, not after the type under test —
  `retry_gated_on_some_conflicting_position_never_retries` says what a reader needs; `faulty_store_test_2`
  does not. `crates/happenstance-testkit/tests/fixture_instruments.rs:24-39` is the in-tree
  precedent for asserting the specific disagreement instead of `#[should_panic]`.
- **Write the `CHANGELOG.md` entry as the last edit**, naming both instruments *and the defect
  each detects* — that sentence is the one an adapter author reads when deciding whether to
  upgrade, and it is much easier to write once the tests exist.
- **If the frozen contract fights back, record it — do not patch it.** A missing constructor,
  an unhelpful `Option`, an associated type that cannot be satisfied: each is a defect-log
  entry naming its clause ID, routed to `defect-log-and-macros-verdict` (project AC-012), never
  an edit under `crates/happenstance-core/src/**`.

## Tests and CI (merge gate)

Grounded in the project testing brief's AC-009 row (`_decomposition.md:787`), which places
`FaultyStore<S>` and `GappyMemoryStore` at the **integration-fixture** tier in
`happenstance-testkit` and names the two mutant tests as the instruments.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo fmt --check`, `cargo clippy --all-targets -D warnings` (inside `cargo xtask affected`) | 100-column code lines, no new `#[allow]`, no `unwrap`/`expect` in the new modules — NF-004 |
| Doctest | `cargo test --doc -p happenstance-testkit`; `crates/happenstance-testkit/src/faulty.rs`, `src/gappy.rs` | Every public item's example compiles **and runs** through `happenstance_testkit::block_on` (`src/lib.rs:188`), imported by crate-root path only — AC-001, AC-010 |
| Unit / integration (mutant) | `crates/happenstance-testkit/tests/faulty_store_instruments.rs` | AC-002, AC-003, AC-009, EC-001, EC-002, EC-004 — the retry loop that branches on `Some` fails; its conformant sibling succeeds on attempt two; the inner store is untouched; the error arms are distinguishable |
| Unit / integration (mutant) | `crates/happenstance-testkit/tests/gappy_store_instruments.rs` | AC-007, AC-008 — the `position + 1` handler disagrees with the store; the `ReadOptions::from(next())` sibling sees every event once; positions asserted as a *relation*, never as literals |
| Integration (conformance) | `crates/happenstance-testkit/tests/faulty_store_conformance.rs` → `event_store_conformance!(FaultyFixture::new())` | AC-005 — an unarmed wrapper is indistinguishable from its inner store across every rule in the suite, and the fixture's declined capability reports its reason |
| Integration (conformance) | `crates/happenstance-testkit/tests/gappy_memory_conformance.rs` → `event_store_conformance!(GappyFixture::new())` | AC-008 — gaps are the permitted freedom, not a defect; ES-9's interior-gap branch (`src/suite.rs:1533-1553`) runs against a second, published, `Send` instrument |
| Integration (flavour guard) | `crates/happenstance-testkit/tests/faulty_store_send_guard.rs` | AC-004, NF-002 — the wrapped read stream is `Send` at the *definition*'s bound and survives `tokio::spawn`; rejects an `async fn read` refactor |
| Feature powerset | `cargo hack --feature-powerset check -p happenstance-testkit` (gate `OPTIONAL`; resolves on this machine) | NF-003 — `--no-default-features` still builds, `memory` only ever adds |
| wasm32 | `cargo xtask wasm` (the conformance-harness step) | NF-002 — the testkit's harnesses still compile for `wasm32`, so nothing here introduced a `Send` bound the edge target cannot carry |
| Docs | `cargo doc` under `-D warnings`; the nightly `--cfg docsrs` build (gate `OPTIONAL`) | AC-006, AC-010 — no undocumented `pub` item, no broken intra-doc link, and the `memory` gate badge renders |
| **Story gate** | `cargo xtask affected --base main` (`.redkiln/config.yaml:41`) | The story-grain bar: fmt, clippy `-D warnings` and tests for `happenstance-testkit` and its dependents, plus the five file-reading lints and `spec-trace` unconditionally |
| **Merge / project gate** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | Project DoD 6 — the non-terminal integration bar, keeping all four `wasm32` steps. Not `cargo xtask ci`; the full gate is `publish-0-2-0-alpha-1`'s |

**Not run by this story:** `cargo xtask proof-artefact` (no `trybuild` case here),
`cargo xtask lint-changelog`'s rule-count assertion (CF-29 counts *rules*, and none is added),
and `cargo run -p course-subscriptions` (AC-003 of the project, another story's).

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation, in this PR |
| --- | --- | --- |
| **The coherence trap.** An implementer writes both flavour impls on one type and spends the session arguing with `error[E0119]` | High / medium — it is the first shape everyone reaches for | Named three times before any code: context pack decision 1, the behaviour table's first row, and the implementation notes. RS-20-4 is an anchor bound to AC-005 |
| **The convenient mutant.** `violate_next` reports `Some(head)` because it is "more informative", and every test still passes | Medium / **high** — this is the failure that ships an instrument proving nothing | AC-002 asserts `conflicting_position.is_none()` **explicitly**, so AC-003's retry test cannot pass for the wrong reason. Discovery already wrote the mutant out in full (`discover.md:85-114`) |
| **The seeding trap.** `GappyMemoryStore` is built by `restore`-ing strided positions into `MemoryEventStore`, which then allocates its next position *below* its own head | Medium / high — the shortcut looks obviously correct | Context pack decision 5 and the behaviour table both cite `memory.rs:388-389`; AC-008's conformance run fails loudly if it is attempted, because VT-11's monotonicity rule catches it |
| **Literal-position assertions creep in** because they are deterministic and readable | Medium / medium — and it would be planted in the crate that teaches the opposite | AC-008 states the relation form and gives the reviewer an `rg` check; the discipline is CLAUDE.md's and is repeated in the story's Merge DoD |
| **Adding `memory` to `default` changes the testkit's default surface** | Low / low pre-publish, higher after | Nothing is published yet, so no consumer is pinned; the change is additive (RS-51-1) and is recorded as a `## [Unreleased] → Added` entry now, so `publish-0-2-0-alpha-1` inherits an accurate changelog rather than reconstructing one |
| **Slice coupling.** `given-when-then-dsl` (HS-S0025) consumes `FaultyStore` to demonstrate a retry loop with no database; a signature change here after that story starts costs both | Medium / medium | The public signatures are fixed by `_design.md:606-626` and restated in this spec's behaviour table. The two stories are implemented in **one context** as M4's single surface, this one first; any signature the DSL needs and this story did not ship is a DG-numbered design amendment, not a quiet edit |
| **`FaultyStoreError<E>` is a public item the signed-off design does not list** | Certain / low | Raised as **DG-1** in the implementation report so the design's item table is amended rather than diverged from. The alternative — a `String` error, or reusing `S::Error` — is forbidden (RS-30-2) and impossible (uninhabited `MemoryStoreError`) respectively |
| **Someone "tidies" `GappedPositionStore`** in the mutant registry now that a published equivalent exists | Low / high — it is CF-5's conformant control | The PR boundary forbids any change under `tests/mutation_coverage/` by name, and context pack decision 6 explains why the duplication is deliberate |
| **The `Send` guard is native-only**, so a `wasm32`-only regression could slip | Low / low | The `wasm32` harness check still compiles the testkit's harnesses; the guard covers the axis (`Send` on the stream) that the edge target cannot express anyway |

## Dependencies

**Blocks on:** *(none)* — `depends_on: []`. This story consumes only the frozen contract crate
and the testkit's existing fixture machinery, both of which are in the tree today. It is why M4
leads with it (`_storymap.md:55`) and why M4 can proceed in parallel with M5.

**Unlocks:**

- **`given-when-then-dsl`** (HS-S0025) — its slice-mate and the other half of project AC-009.
  It `depends_on` this story and consumes `FaultyStore` to demonstrate a caller's retry loop
  with no database (`_storymap.md:56`). Landing this one first is what lets M4 be mounted as a
  single integrated surface.

**Neither blocks nor is blocked by:** `projection-trait-and-runner` (M5) benefits from
`GappyMemoryStore` for AC-006's off-by-one hazard (`_decomposition.md:509-514`), but does not
depend on it — the runner's own fixture is `MemoryProjectionStore`, which is HS-P0010's and is
not in this tree.

## Anchors (progressive disclosure)

Open these at the moment named, not before. Everything load-bearing enough to change a decision
is here; everything already distilled is in the context pack above and does not need re-reading.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `standards/rust/20-two-flavour-ports.md` (RS-20-4, `:202-215`) | States the two-types rule with the compiled example and the `error[E0119]` diagnostic. Reading it costs five minutes; rediscovering it from the compiler costs an afternoon | **Before writing the first line** of either wrapper | AC-005 |
| `standards/rust/21-send-is-not-inherited.md` | Why the `Send` flavour's obligations are not inherited from the bare one — the reasoning behind AC-004's spawn guard and the `Sync` requirement on the arming counters | Immediately after RS-20-4, before `SendFaultyStore` | AC-004 |
| `crates/happenstance-core/src/store.rs:96-260` | The port itself: `read`'s exact non-`async` signature, `append`'s `&self`, the associated `Error` bound, and the doc forbidding `append`'s return as the next `after` anchor | While writing each `impl` — copy the signatures from here rather than inferring them | AC-004, AC-009 |
| `crates/happenstance-core/src/error.rs:135-161` | `ConditionViolated`'s `Option` field, the paragraph explaining why `None` is legitimate for a store with no interactive transaction, and `unspecified()` — the constructor the injected failure uses | While implementing `violate_next`'s injected error | AC-002 |
| `crates/happenstance-core/src/memory.rs:293-400` | The reference store's append/read bodies — how `Query::matches`, `AppendCondition::is_violated_by` and `ReadOptions` compose. Also `:388-389`, the length-based allocation that makes seeding a gappy store this way impossible | While writing `GappyMemoryStore`'s allocator and read path | AC-007, AC-008 |
| `crates/happenstance-core/src/memory.rs:643-680` | `spawns_from_generic` — the exact shape of the guard that rejects an `async fn read` refactor, including why the bound is written at the definition | While writing `faulty_store_send_guard.rs` | AC-004 |
| `crates/happenstance-testkit/src/fixtures.rs:173-290` | `MemoryFixture`/`MemoryHandle`: the handle-newtype-not-blanket-impl reasoning, the delegation shape, and the capability-declension idiom with its stated reason | When writing either conformance fixture | AC-005, AC-008, EC-007 |
| `crates/happenstance-testkit/src/contract.rs:120-173`, `:372-430` | The `Fixture` contract and `Capability` — what one fixture instance means, what `connect()` promises, and the constructor that rejects an empty declension reason | Beside `fixtures.rs`, when declaring `SECOND_HANDLE` / `REOPEN` | AC-005, EC-007 |
| `crates/happenstance-testkit/src/suite.rs:1490-1567` | `read_from_a_gap_position` and its **interior-gap branch** — the rule `GappyMemoryStore` exercises, plus CF-6's note on why literal positions are forbidden | Before running the suite against the gappy store, and when writing its instrument test | AC-008 |
| `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:44-116` | `GappedPositionStore`, the existing `pub(crate)`, `!Send`, fixed-stride variant. Read it to see the allocator shape already proven here — and to confirm you are **adding reach**, not refactoring CF-5's control | Before writing the gappy allocator; **do not edit this file** | AC-008 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs:24-39` | The in-tree precedent for asserting a mutant by its **specific disagreement** rather than `#[should_panic]` — the shape both wrong-caller tests copy | While writing either mutant test | AC-003, AC-007 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | Why an instrument that cannot fail anything is decorative, and why each mutant ships beside a conformant sibling | If you are tempted to ship a store without its wrong caller | AC-003, AC-007 |
| `.kb/decisions/0011-read-laziness-and-isolation.md` | Why `read` is lazy and what a store may and may not do at call time — the decision `fail_next_read`'s first-poll behaviour honours | While implementing `fail_next_read` | AC-004 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | Position assignment and visibility: why `head()` answers *am I caught up?* by comparison and never by subtraction — the correct handler in AC-007's conformant sibling | While writing the resume/checkpoint sibling test | AC-007 |
| `crates/happenstance-core/src/projection.rs:104-106` | `ReadOptions::from` is **inclusive**, so resuming means advancing *past* the checkpoint via `SequencePosition::next` (which returns `Option`) | While writing the conformant handler | AC-007 |
| `standards/rust/30-error-taxonomy.md` | The `#[source]`-chain and no-`String`-errors rules that `FaultyStoreError<E>` must satisfy, and the `#[non_exhaustive]` reasoning | While declaring `FaultyStoreError` | AC-009 |
| `standards/rust/70-rustdoc-obligations.md` | The first-sentence-is-a-complete-claim rule, `# Errors` sections, and the doctest obligations every public item here carries | While writing the rustdoc, before running `cargo doc` | AC-010 |
| `standards/rust/51-features-and-no-std.md` | RS-51: a feature only ever *adds*, and what a `#[cfg]` over a `pub` item owes the powerset | While editing `crates/happenstance-testkit/Cargo.toml` | AC-006, NF-003 |
| `standards/rust/11-const-construction-and-panics.md` | When a documented panic is legitimate and how it must be stated — the atom that decides EC-006's disposition | Only if you hit the position-overflow question | AC-008 |
| `standards/rust/60-what-a-test-must-prove.md` | What separates a test that proves something from one that merely runs — the bar both mutant tests are held to | Before finalising either mutant test's assertion | AC-003, AC-007 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md:606-626`, `:696-700`, `:749-751`, `:832`, `:845-857`, `:983-993` | The **binding** signed-off design: the exact constructor signatures, crate-root placement, visibility/feature table, the opened-on-demand transience disposition, the density numbers, and the named anti-patterns | Before writing signatures, and again before writing rustdoc | AC-001, AC-006, AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/misbehaving-testkit-stores/discover.md:83-141` | *The wrong implementation* — all three mutants written out in full, including the one that passes every check | If any assertion in this story starts to feel like a formality | AC-002, AC-003, AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md:787`, `:503-519`, `:572-584` | The testing brief's AC-009 row (the tier and instrument for these fixtures), the two caller hazards stated in full, and the architecture brief's two-types note | When planning the test files, and when sizing what "integration fixture" means here | AC-003, AC-007, AC-005 |
| `spec/SPECIFICATION.md:1020-1027` (VT-11), `:2748-2820` (ES-9) | The two `[FROZEN]` clauses this story is an instrument *for*. Read to confirm what is permitted; **do not edit** — a change needs a new ADR | Before claiming `GappyMemoryStore` is conformant | AC-008 |
| `crates/happenstance-testkit/src/registry.rs:310-330` | `block_on` and the doctest-harness idiom the testkit already uses, so no runtime dependency is added for an example | While writing each item's doctest | AC-010 |
| `crates/happenstance-testkit/Cargo.toml:24-58` | The current features block (`default = []`, `proptest`), the unconditional `happenstance-core` `memory` feature, and the existing docs.rs metadata the gate badge relies on | While declaring the `memory` feature | AC-006 |

## Clarifications resolved during spec

1. **The stride's default, and whether gaps are uniform** (deferred by `discover.md:51-54`).
   `with_stride(NonZeroU64)` is the **only** constructor at the alpha: no `Default`, no implicit
   stride, and no second irregular-gap constructor. Rationale — the design lists exactly one
   constructor (`_design.md:625`), a default stride is a number nobody chose, and irregular gaps
   add a second axis of variation without a caller asking for one. If a consumer needs
   arbitrary gaps, that is a new item and a design amendment, not a silent extra method.
2. **Whether the new stores implement `Fixture` and join `event_store_conformance!`**
   (deferred by `discover.md:55-57`). **Yes — and the fixtures stay in `tests/`, unpublished.**
   CLAUDE.md's rule admits no exception for a store that ships in a testkit, and running the
   suite is what makes AC-005's and AC-008's claims falsifiable. But a *fixture* is a testing
   detail of this crate, not part of its public vocabulary; only the four items in the design's
   list become `pub`.
3. **`FaultyStoreError<E>` is a fifth public item the design does not list — DG-1.** The port's
   associated `Error` type forces it and `MemoryStoreError` being uninhabited makes every
   alternative impossible. Recorded as a design amendment in the implementation report rather
   than absorbed silently, so `_design.md`'s item table can be corrected by the human who
   signed it.
4. **The `memory` feature is declared *and* added to `default`.** `_design.md:619-626` writes
   the `cfg` and `crates/happenstance-testkit/Cargo.toml:35-41` does not declare the feature, so
   a literal implementation yields an item that silently does not exist. Declaring it satisfies
   the design; putting it in `default` is what makes the item the design signed off the item a
   `cargo add happenstance-testkit` user actually meets (AC-006).
5. **`GappyMemoryStore` implements the `Send` flavour only.** Implementing `SendEventStore`
   yields `EventStore` free through `trait_variant`'s blanket impl, and keeps the instrument
   usable inside `tokio::spawn`. Per RS-20-4, a bare-flavour-only consumer would need a second
   *type*, not a second impl — and none is asked for.
6. **"AC-012" in the PR boundary is the *project*'s AC-012** (*contract defects found by use are
   recorded*, `project.md:206-208`), not one of this story's ten. Defects this story finds in the
   frozen contract are logged and routed to `defect-log-and-macros-verdict`; none is fixed here.
7. **AC ids are exactly AC-001 … AC-010**, as decided by the first pass. None was added or
   dropped. Project AC-009's *DSL* half is deliberately absent — it is `given-when-then-dsl`'s,
   and claiming it here would be the overclaim this repository keeps catching.
8. **EC-006 and EC-008 are the two residuals this spec deliberately leaves to the
   implementer**, each with its deciding atom named and each requiring the choice to be stated
   in the rustdoc and asserted where assertable. A spec that pretends to settle a question its
   author could not test is worse than one that names the residual.
