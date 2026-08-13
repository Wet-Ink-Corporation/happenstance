---
item: HS-S0025
stage: spec
created: 2026-08-12T13:46:22.617Z
updated: 2026-08-12T13:46:22.617Z
template_sig: 87bbf1d0
rendered_sig: a87736fe
---

# Spec — A given/when/then DSL that cannot hide its own filter

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) |
| Project | [`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md`](../project.md) |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/given-when-then-dsl/spec.md` |
| Signed-off design (**binding**) | [`../_design.md`](../_design.md) — surface `dsl-failure-message`; the `## Signatures` testing-surface block; `## Visibility and stability`; anti-pattern 12 |
| Project briefs | [`../_decomposition.md`](../_decomposition.md) — architecture (*Composition roots*, *The contracts this layer consumes unchanged*), ux (AC-U11, AC-U12, AC-U13), testing (AC-009 row) |
| Story map | [`../_storymap.md`](../_storymap.md) — M4 `testing-surface`, this story's row |
| Roadmap pointer | `RUNBOOK.md:4031-4037` — phase 7's DSL line item, and `:4034-4041` for the misbehaving stores it drives |
| House style | `standards/rust/60-what-a-test-must-prove.md` (RS-60-4), `standards/rust/62-doctests-and-harnesses.md` (RS-62-2), `standards/rust/20-two-flavour-ports.md` (RS-20-2/20-4), `standards/rust/70-rustdoc-obligations.md` (RS-70-2/70-4/70-5) |

## One-line PR slice

Land the given/when/then DSL over `MemoryEventStore` in `crates/happenstance/`, seeding and
nominating through `happenstance_core::Query` only, whose failure message names the events
**seeded but not selected** by the model's query (AC-U11 — a filter that hides what it filtered
re-opens the hazard ADR-0020 closes), demonstrated by driving a caller's retry loop against
`FaultyStore<S>` with no database.

## Executive summary

This PR adds one public module — `happenstance::testing`, gated on the already-present `memory`
feature — carrying `given`, `Given<B>`, `Decision<E>` and `assert_domain_event`, plus the
four-region panic message that is the whole point of the story, plus the `FaultyStore<S>`
demonstration test that proves a caller's retry loop is testable with no database.

**Delta against what exists.** `crates/happenstance/src/lib.rs` is 76 lines and its only item is
`pub use happenstance_core::*;` (`:75`). By the time this story runs, M2 and M3 have landed
`DomainEvent`, `DecisionModel`, `Boundary`, `Codec`, `commit`/`commit_with`, `Retry`, `Committed`
and `CommandError` beside that glob, and M4's slice-mate has landed `FaultyStore` /
`SendFaultyStore` / `GappyMemoryStore` at `crates/happenstance-testkit/src/`. This story consumes
all of them and adds the first item a *test author* meets. It is also the first thing in the crate
to take a dev-dependency on `happenstance-testkit`, which is a publish-order coupling this spec
names rather than discovers (see *Data and migrations*).

What it does **not** do: it does not decide the surface. `_design.md` is signed off and its
testing-surface signatures, visibility table and four-region composition are binding here. The one
thing this spec resolves is an internal contradiction in that file's `when`/`then_refused` pair,
resolved the only way that leaves the signed-off signatures callable (see *Context pack*, item 4).

## Context pack

The decisions this story must honour. Everything below is stated as a decision, already taken;
the deeper artefacts sit behind the anchors in the second half of this spec.

**1. The failure message's fourth region is the deliverable, not the assertion.** A failed
`then(...)` renders four labelled regions in this order — `expected:`, `actual:`,
`selected by the model's query:`, `seeded but NOT selected:` — and the fourth is why the story
exists. `assert_eq!`-style expected/actual was rejected because it *hides the filter*: when a
model's `EVENT_TYPES` and its fold disagree, the events that were seeded and not selected are the
entire diagnosis, and a message that omits them re-opens exactly the hazard ADR-0020 closes
(`../_design.md`, `dsl-failure-message` pattern row; `../_decomposition.md` AC-U11). The design's
anti-pattern 12 states the failure condition in reviewable terms: *"a DSL assertion failure shows
only expected/actual… a screenshot missing `seeded but NOT selected:` fails."* The in-tree
precedent is `crates/happenstance-testkit/src/contract.rs:500-507`'s `skip_line` — a declined
capability prints its reason rather than vanishing — and this is the same principle applied to a
filter rather than to a capability.

**2. There is exactly one filter vocabulary, and it is `happenstance_core::Query`.** The DSL
nominates events by calling the boundary's derived `query()` and nothing else. It does not accept
an event-type list, a predicate closure or a tag filter of its own. This is DR-05's rule
(*"reuse `Query` rather than inventing a second filter vocabulary"*, `../project.md`) applied to
the test surface: a second vocabulary would let a test select a different set than production
does, and the assertion would then be about the DSL rather than about the model. It is also what
makes region 4 computable at all — *selected* is defined as "what `query()` selected", so
*seeded but not selected* is a set difference the DSL can take, not a guess.

**3. Seeding goes through the same mapping production uses.** `Given::event` takes a
`DomainEvent` value and encodes it with the boundary's codec before appending, so the seeded bytes
are the bytes `commit` would have written. Its signature is fallible for that reason —
`Result<Self, CodecError>` — and the error is the codec's, not a DSL-local one. A DSL that seeded
pre-built `happenstance_core::Event` values would let a test pass against an encoding the
application never produces.

**4. `Decision` carries the refusal; `when`'s `Err` arm carries store and codec failure only.**
`_design.md` gives `when` the return type
`Result<Decision<B::Event>, CommandError<MemoryStoreError, D>>` *and* puts `then_refused(self)` on
`Decision`. Read literally those two cannot both hold — a refusal routed to `CommandError::Refused`
never produces a `Decision` for `then_refused` to be called on. **Resolution, and it is the only
one that leaves both signed-off signatures callable: the decide closure's `Err(d)` is captured
*into* the `Decision`, and `when` returns `Err` only for store and codec failure.** Note the
consequence the type system already states: `MemoryStoreError` is an uninhabited enum
(`crates/happenstance-core/src/memory.rs:291`), so on this store the `Err` arm reduces to codec
failure in practice. This is a reading of the design, not an amendment to it — no signature
changes — and it is recorded here so the implementer does not re-derive it.

**5. The DSL cannot exercise contention, so the retry demonstration is a second, separate test.**
`given(...)` is fixed to `MemoryEventStore` and runs single-threaded, so `ConditionViolated` is
unreachable through it. AC-U13's claim — *"the caller's own retry loop must be testable against
[`FaultyStore<S>`] without a database"* — is therefore discharged by a test in
`crates/happenstance/tests/` that drives `commit`/`commit_with` with a `Retry` bound against
`FaultyStore<MemoryHandle>`, not through `given`. That test asserts the attempt count is visible
on `Committed` and that the loop reaches success after an injected violation.

**6. The retry loop must not branch on `conflicting_position`.** `FaultyStore::violate_next`
reports `conflicting_position: None` on purpose — *"which a remote store legitimately does"*
(`../_design.md`, `## Signatures`), and the port's own doc says a caller *"must be written for
`None`"* and that a loop branching on `Some` *"works against an in-process store and stops working
against a remote one"* (`crates/happenstance-core/src/error.rs:135-147`). The demonstration test is
the instrument that makes that failure mode observable in this crate.

**7. Everything before the append is pure, and the `Decision` says so.** AC-U12: defining events,
folding a model, composing models and encoding perform no I/O. `Decision` is `#[must_use]` with the
message `_design.md` authored verbatim — *"a Decision is not appended until it is committed;
dropping it discards the events the decision produced"* — and `#[non_exhaustive]`. The message text
is binding; do not paraphrase it.

**8. The persona-journey slice.** This is Beat 2 of the journey *"Choose a contract before a
database"* — *"does the domain model work"* — and the project exists partly to separate it from
Beat 3, *"pick a database"*, which today are one decision (`RUNBOOK.md:3977-3979`;
`../_storymap.md`, backbone row A4). The transience policy makes the consequence concrete:
`happenstance::testing` is **revealed** — one region on the crate-root landing page pointing at the
module — *"not opened-on-demand because Beat 2 is the beat this project exists to separate from
'pick a database'"* (`../_design.md`, *Transience policy*).

**9. Flavour discipline, unchanged and non-negotiable.** No `#[async_trait]`; generic code binds
`EventStore`, never `SendEventStore`; one flavour name imported per module (`CLAUDE.md` binding
constraints 1, 3, 4; `standards/rust/20-two-flavour-ports.md` RS-20-2/20-3). `MemoryEventStore`
implements the `Send` flavour and is reached through the blanket impl, so the DSL's own bound stays
`EventStore`. The `FaultyStore` / `SendFaultyStore` split exists because `trait_variant`'s blanket
impl makes one type carrying both flavours `error[E0119]` (RS-20-4) — pick the one that matches the
bound at the call site rather than adding a bound to make the other fit.

**10. The oracle must not share a subroutine with the implementation.** RS-60-4
(`standards/rust/60-what-a-test-must-prove.md:276`). The DSL's own tests must compute the expected
"seeded but not selected" set independently of the DSL's set-difference code — otherwise a bug that
drops region 4's contents passes its own test. Spell the oracle from the seeded values the test
already holds.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice a test author meets directly |
| **Slice / milestone** | **M4 `testing-surface`**. Slice-mate: `misbehaving-testkit-stores` (implemented in the same context, mounted as one surface) |
| **Mount point** | **`crates/happenstance/src/lib.rs`** — composition root 1 (`../_decomposition.md`, *Composition roots*): `pub mod testing;` is declared there and the module doc gains the one region that points at it. `testing` is the one deliberate exception to root re-export (`../_design.md`, *Placement and re-export*): it is a **named module**, not root `pub use`s, so the crate's item table is not doubled for a P1 reader |
| **Wires into** | `happenstance::{DomainEvent, DecisionModel, Boundary, Codec, Json, CommandError, CodecError}` (M2/M3, siblings); `happenstance_core::{Query, Event, SequencedEvent, EventStore, MemoryEventStore, AppendCondition}` via the surviving glob at `crates/happenstance/src/lib.rs:75`; `happenstance_core::store::read_decision_model` (`crates/happenstance-core/src/store.rs:321`) as the read half; `happenstance::{commit, commit_with, Retry, Committed}` (M3, `command-loop`) for the retry demonstration; `happenstance_testkit::FaultyStore` (M4 slice-mate) as a **dev-dependency** of `crates/happenstance`; `crates/happenstance-testkit/src/fixtures.rs`'s `MemoryFixture` (`:243-292`) as the reference pattern for what "seed a store" means |
| **Renders surfaces** | **`dsl-failure-message`** (`../_design.md` `## Surfaces`; route `crates/happenstance/src/testing/`, selector: the panic message of a failed `then(...)` on stderr; states `expected-vs-actual`, `seeded-but-not-selected`, `empty-selection`, `long-label`) — this story *creates* it. **`crate-root-rustdoc`** — *changed*, by one added region pointing at `happenstance::testing` per the transience policy; the rest of that surface belongs to its own stories |
| **Public items** | `happenstance::testing::given`, `happenstance::testing::Decision`, `happenstance::testing::assert_domain_event` (the three `## Items` rows carrying `feature: memory`), plus `Given<B>` which the `## Signatures` block names and the `## Items` table omits — it is `given`'s return type and cannot be private |
| **Conformance rule(s)** | **None, and this is deliberate.** Nothing here is adapter-observable: the DSL is an application-facing test vocabulary in `happenstance`, and `happenstance-testkit`'s suite observes *stores*. The rule this story is adjacent to is its slice-mate's — `GappyMemoryStore` is the instrument that makes a `position + 1` assumption fail — and that lives in `misbehaving-testkit-stores`. No port changes here, so no rule is owed |
| **Clause(s)** | **None.** `spec/SPECIFICATION.md`'s namespaces are `VT-`/`ES-`/`PS-`/`SY-`/`WF-`/`CF-`; there is no typed-layer namespace, so this story's only contract is `_design.md` plus ADR-0020 (`../_design.md`, preamble). Nothing here amends a `[FROZEN]` clause, and nothing here may |
| **Advances DoD scenario** | Initiative **DoD 13** (*"the gate is green on the assembled whole"*) directly, through the project's own bar `cargo xtask ci --fast` (`.redkiln/config.yaml:55`; `../project.md` DoD 6). Initiative **DoD 1** indirectly and honestly: this is the instrument that makes the worked example's decision logic assertable without a database, but the example's execution claim is `worked-example-on-typed-layer`'s. **No initiative DoD scenario names the testing surface** — stated plainly rather than stretched |

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed
outside it.

```
crates/happenstance/src/**
crates/happenstance/tests/**
crates/happenstance/Cargo.toml
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/given-when-then-dsl/**
```

**In this PR**

- `crates/happenstance/src/testing/` — `given`, `Given<B>`, `Decision<E>`, `assert_domain_event`,
  and the four-region failure renderer.
- `crates/happenstance/src/lib.rs` — `pub mod testing;` and the module-doc region that points at
  it. This is the mount; it is not scope drift.
- `crates/happenstance/Cargo.toml` — the `happenstance-testkit` dev-dependency, and
  `[package.metadata.docs.rs]` + `rustdoc-args` **if a slice-mate has not already added them**
  (`../_design.md`, *Manifest changes*: neither exists in this file today — verified). Adding them
  is idempotent; adding them twice is a merge conflict, not a defect.
- `crates/happenstance/tests/` — the `FaultyStore<MemoryHandle>` retry demonstration, and the
  wrong-implementation cases that prove `assert_domain_event` and region 4 can fail.

**Explicitly not in this PR**

- `crates/happenstance-testkit/**` — `FaultyStore` / `SendFaultyStore` / `GappyMemoryStore` are the
  slice-mate's (`misbehaving-testkit-stores`). This story *consumes* them.
- `crates/happenstance-core/**` — frozen; a defect found here is logged and routed under AC-012,
  never patched (`../project.md`, *Out of scope*, last bullet; AC-A02).
- `commit` / `commit_with` / `Retry` / `Committed` / `CommandError` — `command-loop`'s items. This
  story may not adjust their signatures to make a test easier; if one does not fit, that is a
  finding for the ledger, not an edit.
- The projection runner, the worked example, the codec feature matrix, the `wasm32` step and
  anything under `spec/`, `experiments/`, `xtask/` or `.kb/`.

**Merge DoD (one line).** `cargo xtask affected --base main` is green, the four-region message is
observed failing on a deliberately divergent model, and `happenstance::testing` is reachable from
the crate root under default features with every intra-doc link resolving under
`--no-default-features` as well.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `given(boundary) -> Given<B>` | Takes any `B: Boundary` — a single `DecisionModel` through the blanket impl, or a tuple `(B1, B2)`… through the composition impls. Creates a fresh `MemoryEventStore` per call; one `given` is one isolated store, mirroring the fixture rule that *"one fixture instance is one isolated backing store"* | `../_design.md` `## Signatures`; `crates/happenstance-testkit/src/fixtures.rs:243-292`; `CLAUDE.md`, *The rule that matters* |
| `Given::event(event) -> Result<Self, CodecError>` | Encodes the `DomainEvent` with the boundary's codec and appends it with `condition: None`. Builder-chained; fallible because encoding is. The seeded bytes are the bytes `commit` would write | `../_design.md` `## Signatures`; `crates/happenstance-core/src/store.rs:213-217` |
| `Given::when(decide) -> Result<Decision<B::Event>, CommandError<MemoryStoreError, D>>` | Reads with `boundary.query()`, decodes and folds through `Boundary::absorb`, then calls `decide(&B)` once. `async`, and it is the only `await` a test author writes | `../_design.md` `## Signatures`; `crates/happenstance-core/src/store.rs:321-329` |
| Nomination is `Query`-only | The read is `read(&boundary.query()?, ReadOptions::new())`. No event-type list, predicate or tag filter is accepted by the DSL. *Selected* is defined as this query's result set | DR-05, `../project.md`; `../_decomposition.md` AC-U11 |
| A nominated event that cannot be decoded is an error | `Boundary::absorb` skips a **non-nominated** type silently (the query did not name it) but returns `CodecError::UnknownEventType` when a **nominated** event cannot be decoded — that is a real disagreement between `EVENT_TYPES` and the fold, and the DSL must not swallow it | `../_design.md`, *The states the API must express*, **Absent** |
| An empty selection is not an error | `read_decision_model` over a query matching nothing returns `(vec![], None)`; the model is simply never `apply`ed. `given(...)` with no seeded events is a well-formed test | `../_design.md`, *The states the API must express*, **Empty**; `crates/happenstance-core/src/store.rs:309-329` |
| `Decision::then(&[E])` | Passes when the emitted events equal `expected`; otherwise panics with the four-region message. `E: DomainEvent + PartialEq + Debug`. Should carry `#[track_caller]` so the panic names the test's own line rather than the DSL's — the same property `unwrap` and indexing rely on | `../_design.md` `## Signatures`; `standards/rust/60-what-a-test-must-prove.md:112` |
| `Decision::then_refused()` | Passes when the decide closure returned `Err(d)`; the refusal is carried typed (the caller's own `D`), never flattened to a string. Per *Context pack* item 4, the refusal lives on `Decision`, not on `when`'s `Err` arm | `../_design.md` `## Signatures`; `standards/rust/30-error-taxonomy.md` RS-30-2 |
| The four regions, in order | `assertion failed: the decision emitted different events` → `expected:` → `actual:` → `selected by the model's query:` → `seeded but NOT selected:` → one line naming the derived query | `../_design.md`, *Composition* → `dsl-failure-message (top to bottom)` |
| Region truncation is asymmetric and never silent | ≤ 8 event rows per region, each row ≤ 80 columns. `selected by the model's query:` truncates **first**; `seeded but NOT selected:` truncates **last**, because the last region carries the diagnosis. Every truncation prints `… and N more` | `../_design.md`, *Density budget* → *Assertion failure* |
| The empty state keeps its region | With nothing seeded, region 4 prints `seeded but NOT selected: (nothing was seeded)`. The region **stays and says so** — it never disappears | `../_design.md`, *States* table, **Empty** column |
| The long-label state | A long event type wraps; the `not selected` marker column stays left-aligned. Position and event type are never truncated to fit | `../_design.md`, *States* table, **Long label** |
| `assert_domain_event::<E>(&[…])` | Asserts every value's `event_type()` is in `EVENT_TYPES`. It exists because that agreement is **not** compiler-enforceable — a variant can return a type absent from `EVENT_TYPES` and no `const` sees the match arms — and it is the residual AC-013 measures. Its doc comment says so (RS-70-5: name the alternative that lost, once, where the reader is) | `../_design.md`, *Shape decision*, `EVENT_TYPES` ↔ `event_type()` row; `standards/rust/70-rustdoc-obligations.md` RS-70-5 |
| Retry demonstration, no database | An integration test wraps `MemoryHandle` in `FaultyStore::new(...).violate_next(1)`, runs `commit_with` under a `Retry` bound, and asserts the call succeeds with `Committed.attempts == 2`. The loop is never allowed to branch on `conflicting_position`, which the fixture reports as `None` | `../_decomposition.md` AC-U13; `crates/happenstance-core/src/error.rs:135-147`; `../_design.md` `## Signatures`, `FaultyStore::violate_next` |
| No literal position assertions | Nothing in this story asserts `[1, 2, 3]`. Where a position is needed it is the one the store returned. Positions may have gaps, and `head - checkpoint` is never computed | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-core/src/store.rs:240` |
| Feature gate and doc badge | Every new item is `#[cfg(feature = "memory")]`; `memory` is already in `default` (`crates/happenstance/Cargo.toml`, `default = ["std", "memory"]`). The docs.rs metadata + `#![cfg_attr(docsrs, feature(doc_cfg))]` must exist by the time this lands or the gated items render with no badge (AC-U14) | `crates/happenstance/Cargo.toml`; `../_design.md`, *Manifest changes*; `standards/rust/51-features-and-no-std.md` RS-51-5 |
| Doctest shape | The module's example is `async`, so it needs a hidden runtime and an `# Ok::<(), E>(())` closer; code inside the fence stays within 72 columns so it does not scroll horizontally at 1024px | `standards/rust/62-doctests-and-harnesses.md` RS-62-2; `../_design.md`, *Density budget*; anti-pattern 3 |
| Flavour bound | Any generic function this story adds binds `EventStore`, not `SendEventStore`; only one flavour name is imported per module. `FaultyStore<S>` and `SendFaultyStore<S>` are two types, not one, and the call site picks | `CLAUDE.md` binding constraints 1, 3, 4; `standards/rust/20-two-flavour-ports.md` RS-20-2/20-3/20-4 |
| Mounted, not merely compiled | `crates/happenstance/src/lib.rs` declares `pub mod testing;` and the module doc gains the pointer region. A `testing` module reachable only from this crate's own tests is not delivered | `../_decomposition.md`, *Composition roots* item 1; `../_design.md`, *Transience policy* |

## Data and migrations

**N/A — no persistent data, no schema, no migration.** Every store this story touches is a
`MemoryEventStore` created inside a single test and dropped at its end; the DSL performs exactly
one kind of write (`append` with `condition: None`, for seeding) and reads through `Query`. There
is no on-disk format, no checkpoint and no serialised state that outlives a process.

Two manifest-shaped changes are recorded here because they are the only things in this story that
resemble a migration, and one of them has a consequence outside the story:

1. **`crates/happenstance` gains a dev-dependency on `happenstance-testkit`.** No cycle: the
   testkit depends on `happenstance-core` only (`crates/happenstance-testkit/Cargo.toml`,
   `[dependencies]`). But `[workspace.dependencies]` declares it as
   `happenstance-testkit = { version = "0.2.0", path = "crates/happenstance-testkit" }`
   (`Cargo.toml:26`), and a dev-dependency carrying a version must resolve from the registry at
   publish time — which would make `happenstance-testkit` publish **before** `happenstance`, a
   coupling CF-32 exists to avoid and which `publish-0-2-0-alpha-1` (M7) would inherit unannounced.
   The implementer must take one of two admissible routes and **state which in the ledger**: a
   path-only, versionless dev-dependency (cargo strips versionless dev-dependencies from the
   published manifest), or accepting the publish order explicitly and flagging it to M7. Do not
   leave it undecided.
2. **`[package.metadata.docs.rs]` and `rustdoc-args = ["--cfg", "docsrs"]`** are absent from
   `crates/happenstance/Cargo.toml` today and present in `happenstance-core`'s (`:54-56`) and
   `happenstance-testkit`'s (`:56-58`). They are a prerequisite for this story's gated items to
   render a badge, so add them if a slice-mate has not — idempotently, and without changing the
   existing `[features]` block, which already carries `memory` in `default`.

## Acceptance criteria

Every row is a persona goal crossing the full stack, not a capability. The persona is **P1, the
application author** (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:42-80`)
at **Beat 2** of *"Choose a contract before a database"* — *"does the domain model work"* — the
beat this project exists to separate from *"pick a database"* (`../_design.md`, *Transience
policy*). **P4, the evaluator** (`:249-290`) appears in AC-010, because what P4 sees is the
rendered page and nothing else.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** P1 has a `DecisionModel` and no database — no connection string, no `MemoryEventStore` of their own, no fixture — **WHEN** they write `given(model).event(a)?.event(b)?.when(\|m\| m.decide(cmd)).await?` and call `.then(&[expected])`, **THEN** the assertion passes, the whole test runs in-process, and the only `await` they wrote is the one on `when` | `crates/happenstance/src/testing/mod.rs` — `seeded_then_decided_asserts_emitted_events`; and a composed-boundary case `composed_boundary_folds_both_models` over a `(B1, B2)` tuple, proving the same call shape works for the composition `decision-model-composition` landed |
| AC-002 | **GIVEN** P1's fold and their command disagree, so the decision emits the wrong events, **WHEN** `.then(&[…])` fails, **THEN** P1 does not read the log to find out why: the panic renders the header `assertion failed: the decision emitted different events` and then four *labelled* regions in order — `expected:`, `actual:`, `selected by the model's query:`, `seeded but NOT selected:` — followed by one line naming the derived query, and `#[track_caller]` puts the panic's location on P1's own assertion line, not inside the DSL | `crates/happenstance/tests/dsl_failure_message.rs` — `four_regions_render_in_order` (`#[should_panic]`, asserting each label's presence **and** its byte offset ordering) and `panic_location_is_the_callers_line` (`std::panic::catch_unwind` + a `#[track_caller]` location assertion, per `standards/rust/60-what-a-test-must-prove.md` RS-60-1: open the subject inside the caught closure) |
| AC-003 | **GIVEN** P1's `EVENT_TYPES` omits an event type their fold needs — the ADR-0020 hazard, and the one failure the DSL could hide — **WHEN** they seed that event and the assertion fails, **THEN** the event appears by name under `seeded but NOT selected:` and *not* under `selected by the model's query:`, so the diagnosis is on the screen rather than in a second debugging session; the last line names the derived query that did the filtering | `crates/happenstance/tests/dsl_failure_message.rs` — `region_four_names_the_seeded_but_unselected_event`. The expected set is spelled **from the seeded values the test already holds**, never by calling the DSL's own set-difference (RS-60-4, `standards/rust/60-what-a-test-must-prove.md:276`) |
| AC-004 | **GIVEN** P1 is writing the *first* test of a new model and has seeded nothing yet, **WHEN** the assertion fails, **THEN** the fourth region is still there and says `seeded but NOT selected: (nothing was seeded)` — the region never disappears, because a missing region reads as "the DSL has nothing to say about the filter" when it means "there was nothing to filter" | `crates/happenstance/tests/dsl_failure_message.rs` — `empty_seed_keeps_region_four_and_says_so`; plus `empty_selection_is_not_an_error`, which asserts `given(model).when(…)` over an unseeded store returns `Ok` and never `apply`s the model |
| AC-005 | **GIVEN** P1 is debugging a model against a log of 200 seeded events on an 80-column terminal, **WHEN** the assertion fails, **THEN** the message stays readable and stays honest: each region prints at most **8** event rows, each row at most **80** columns, `selected by the model's query:` truncates **first** and `seeded but NOT selected:` truncates **last**, every truncation prints `… and N more`, and a long event type wraps rather than being cut — position and event type are never truncated to fit, and the `not selected` marker column stays left-aligned | `crates/happenstance/tests/dsl_failure_message.rs` — `overflow_truncates_selected_first_and_the_diagnosis_last` (seeds 20, asserts the `selected` region carries `… and N more` while region 4 is still complete) and `long_event_type_wraps_without_truncating_type_or_position`; both assert `line.chars().count() <= 80` over every rendered line |
| AC-006 | **GIVEN** P1's domain *refuses* the command — the course is full — **WHEN** they assert with `.then_refused()`, **THEN** it passes, and their own error type survives the round trip as a typed value rather than a string; a refusal never routes through `when`'s `Err` arm, which is store and codec failure only | `crates/happenstance/src/testing/mod.rs` — `refusal_is_asserted_through_then_refused` and `refusal_does_not_route_through_when_err`; plus `then_on_a_refused_decision_panics_naming_the_refusal` in `crates/happenstance/tests/dsl_failure_message.rs`, which is the wrong-implementation case for EC-006 |
| AC-007 | **GIVEN** P1 wants the test to be evidence about *their application*, **WHEN** they seed with `Given::event(e)`, **THEN** the bytes written are the bytes `commit` would have written — the same codec, the same tag, the same fallibility surfaced as `Result<Self, CodecError>` — and if a seeded event is *nominated* by the query but cannot be decoded, the DSL reports `CodecError::UnknownEventType` instead of quietly folding a shorter log. The same story ships `assert_domain_event`, so P1 can prove every variant's `event_type()` is in `EVENT_TYPES` — the one agreement the compiler cannot make for them | `crates/happenstance/src/testing/mod.rs` — `seeded_bytes_are_the_bytes_commit_writes` (reads the raw `Event` back through the store and compares against the codec's own output, not against a literal), `nominated_but_undecodable_event_is_an_error`, `non_nominated_type_is_skipped_not_an_error`; `crates/happenstance/tests/dsl_failure_message.rs` — `assert_domain_event_rejects_a_variant_outside_event_types` (`#[should_panic]`, the named wrong implementation) |
| AC-008 | **GIVEN** P1 builds a `Decision` and then decides not to assert on it — a half-written test, an early `return` — **WHEN** the value is dropped, **THEN** nothing was appended and nothing was mutated: everything up to the append is pure, and the compiler says so through `#[must_use = "a Decision is not appended until it is committed; dropping it discards the events the decision produced"]`, verbatim | `crates/happenstance/src/testing/mod.rs` — `dropping_a_decision_appends_nothing` (re-reads the store through `boundary.query()` and asserts the seeded set is unchanged) and `must_use_message_is_verbatim` (a `const` string compared against the attribute's text, kept beside it); the lint itself is enforced by `cargo clippy -- -D warnings` in `cargo xtask affected` |
| AC-009 | **GIVEN** P1 has written a retry loop and cannot reproduce a write conflict on demand, **WHEN** they wrap `MemoryEventStore`'s handle in `FaultyStore::new(h).violate_next(1)` and run `commit_with` under a visible `Retry` bound, **THEN** the call succeeds, `Committed.attempts` reads `2`, the loop re-read and re-decided from a pristine model rather than reusing a stale fold — and it never branched on `conflicting_position`, which this fixture reports as `None` exactly as a remote store legitimately does | `crates/happenstance/tests/retry_without_a_database.rs` — `retry_succeeds_after_an_injected_violation` (asserts `attempts == 2` and that no database, socket or temp file is touched), `retry_refolds_from_a_pristine_model`, and `retry_is_bounded` (`violate_next(9)` under `Retry` of 3 → `CommandError::Exhausted { attempts, .. }`, not a hang) |
| AC-010 | **GIVEN** P4 has one sitting to decide whether this library is real and will not run its test suite, **WHEN** they open the crate-root page on docs.rs, **THEN** they find `happenstance::testing` **revealed** — one short region, *"Testing without a database"*, pointing one click away, not interleaved with the four names of the first program and not buried in a second crate — every gated item there carries its `doc_cfg` badge, every intra-doc link resolves with default features *and* with `--no-default-features`, no item summary ends in an ellipsis, and no name in `testing` shadows a contract re-export | `crates/happenstance/tests/mounted_at_the_crate_root.rs` — `testing_is_reachable_by_its_public_path` (names `happenstance::testing::{given, Decision, assert_domain_event}` through the crate root, so a module that compiles but is not declared fails); `cargo doc -p happenstance --no-deps` and `cargo doc -p happenstance --no-deps --no-default-features` with `RUSTDOCFLAGS="-D warnings"`; `cargo xtask ci --fast`'s docs step |

**Traceability.** Project **AC-009** (*"a consumer can test a decision, and can test misbehaviour"*,
`../project.md:192-196`) is covered by AC-001 – AC-010 in full: *test a decision* by AC-001/006/007,
*test misbehaviour* by AC-009. The **misbehaviour instruments themselves** — `FaultyStore` and
`GappyMemoryStore`, and the `position + 1` wrong implementation — are the slice-mate's half of the
same project AC (`../_storymap.md`, coverage row: *"split by crate: testkit stores vs. the DSL"*).

## Interaction quality

RFC §6.7/D6. Every invariant below is carried by an **AC row above**, never by a bullet here — a
bullet in this section gets no ledger row and is never gated. This section says *which* row carries
*which* invariant, and how it is instrumented. The medium is rustdoc, a panic message on stderr, and
a compiler diagnostic; the design translates the three transience categories into that medium
explicitly (`../_design.md`, *Transience policy*, preamble).

**State invariants.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (AC-U10) | **AC-002** | The failure is actionable where it happened: four regions carry the values, and `#[track_caller]` puts the panic location on the caller's assertion line. A message that says "compare the log" is the failure |
| **Non-occlusion — a filter must not hide what it filters** (AC-U11) | **AC-003**, with **AC-004** covering the degenerate case | Region 4 is asserted present *and* correct, against an oracle spelled independently of the DSL's set difference |
| **Preserved state across retry** (AC-U13) | **AC-009** | `retry_refolds_from_a_pristine_model` — the loop re-reads and re-decides; a stale fold or a discarded command input fails it |
| **Reversibility** (AC-U12) | **AC-008** | Dropping a `Decision` leaves the store byte-identical; the append is the single irreversible act |
| **Reachability without a search engine** — this medium's keyboard-reachability analogue (AC-U14) | **AC-010** | The public path is named in a test, the badge and both link resolutions are checked by rustdoc under `-D warnings` |

**Composition invariants**, taken from the signed-off `../_design.md` and binding here.

| Invariant | The design's number or rule | Carried by |
| --- | --- | --- |
| **Presentation exists at all** | The message is *composed*, not a `Debug` dump: a header sentence, four labelled regions, a closing derived-query line (`../_design.md`, *Composition* → `dsl-failure-message (top to bottom)`) | **AC-002** |
| **Composition and placement** | Region order is fixed; the derived-query line is last. `happenstance::testing` is a **named module**, the one deliberate exception to root re-export (*Placement and re-export*) | **AC-002** (order), **AC-010** (placement) |
| **Transience** | `happenstance::testing` is **revealed** — one region on the landing page, one click away. `assert_domain_event` is **revealed inside `testing`**, never on the landing page, because advertising it advertises a residual. `FaultyStore` stays **opened on demand**, in a second crate (CF-32) | **AC-010**; the second-crate boundary is enforced by the *PR boundary* block above |
| **Density budget, with its real numbers** | ≤ **8** event rows per region; ≤ **80** columns per row; `selected` truncates first, the diagnosis last; every truncation prints `… and N more` (*Density budget* → *Assertion failure*). For the doc: ≤ **72** columns inside a fence, first sentence ≤ **80** characters, identifier ≤ **24** characters | **AC-005** (message), **AC-010** (doc) |
| **Hierarchy** | *Primary* is the `seeded but NOT selected:` region — last, nearest the reader's eye in a terminal dump, and last to truncate. *Secondary* is expected/actual. *Recessive* is the one-line derived query (*Hierarchy* → `dsl-failure-message`) | **AC-003** (primacy asserted as truncation order and position), **AC-005** |
| **Anti-pattern 12** — *"a DSL assertion failure shows only expected/actual… a screenshot missing `seeded but NOT selected:` fails"* | `../_design.md`, *Anti-patterns*, 12 | **AC-002** + **AC-003**; the `#[should_panic]` cases are the reviewable equivalent of the screenshot |
| **Anti-pattern 6** — a gated item's page shows no gate badge | *Anti-patterns*, 6 | **AC-010** |
| **Anti-pattern 5** — an item-table summary ending in an ellipsis | *Anti-patterns*, 5, **scoped to this story's own items** per the mock's finding 3 (`../_design.md:1207-1213`): every over-budget summary on that page arrives through `pub use happenstance_core::*` and is frozen by AC-A02 | **AC-010** |
| **Anti-pattern 14** — a name in `happenstance` shadowing a contract re-export | *Anti-patterns*, 14 | **AC-010** |

**Not applicable, stated rather than skipped.** There is no focus, no scroll position and no
selection to preserve; the analogues are the retry's pristine re-fold (AC-009) and the panic
location (AC-002). There is no undo; the analogue is that nothing before the append is irreversible
(AC-008). Anti-patterns 1-4 and 7-11 belong to `crate-root-rustdoc`, `crate-readme` and
`worked-example-transcript`, whose stories own them; this story touches `crate-root-rustdoc` by
exactly one added region and must not otherwise disturb it.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A seeded `DomainEvent` cannot be encoded by the boundary's codec | `Given::event` returns `Err(CodecError)`. It does not panic, and it does not append a partial event. The signature is fallible for this reason and the `?` is the caller's |
| **EC-002** | An event **nominated** by `boundary.query()` cannot be decoded | `when` returns `Err`, carrying `CodecError::UnknownEventType`. This is a real disagreement between `EVENT_TYPES` and the fold and must not be swallowed (`../_design.md`, *The states the API must express*, **Absent**) |
| **EC-003** | An event in the store is **not** nominated by the query | Skipped silently by the fold — the query did not name it — **and** it appears in region 4 when an assertion fails. The two behaviours are not in tension: silence in the fold, visibility in the diagnosis. This is the exact interaction AC-U11 exists for |
| **EC-004** | `Boundary::query()` returns `Err(InvalidQuery::UnconstrainedItem)` — the boundary constrains nothing | Surfaces through `when`'s `Err` as the boundary error; never an `unwrap`, never a panic. The design gives this variant a real meaning rather than treating it as unreachable (`../_design.md`, *The residual*, item 1) |
| **EC-005** | The store fails a read during the retry demonstration (`FaultyStore::fail_next_read`) | The store error surfaces; the loop does **not** retry a read failure as though it were a `ConditionViolated`. A retry budget exhausted by injected violations returns `CommandError::Exhausted { attempts, source }`, not a hang |
| **EC-006** | `then(&[…])` is called on a `Decision` whose closure **refused** | Panics naming the refusal. An implementation that compares a refusal against `&[]` and passes is the wrong implementation this condition rejects: a refusal that reads as "emitted nothing" is exactly the silent pass the story is built to prevent (*Context pack*, item 4) |
| **EC-007** | A `Decision` is constructed and dropped without `then`/`then_refused` | `#[must_use]` fires with the verbatim message. Under the workspace's `-D warnings` this is a build failure in the caller's test, which is the intended strength |
| **EC-008** | An assertion is written against a store the DSL did not create | Unreachable by construction: `given` creates its own `MemoryEventStore` and exposes no constructor taking one. Recorded here because the *absence* of that constructor is a decision — a DSL that accepted a caller's store would need a capability model, which is `happenstance-testkit`'s job, not this module's |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | **No database, no socket, no temp file, no clock.** The whole surface runs in-process against `MemoryEventStore`, and `happenstance` gains **no** new non-dev runtime dependency | This is the story's premise, not a nicety: Beat 2 is separable from Beat 3 only if it needs nothing from Beat 3 (`RUNBOOK.md:3977-3979`) |
| **NF-002** | **A feature adds.** Every new item is `#[cfg(feature = "memory")]`; `--no-default-features` still builds and its doc build is warning-free | RS-51-1 (`standards/rust/51-features-and-no-std.md`); AC-U12's feature half; `cargo xtask ci`'s `--no-default-features` doc step |
| **NF-003** | **Flavour discipline.** No `#[async_trait]`. Every generic bound this story adds is `EventStore`, never `SendEventStore`; one flavour name imported per module; `FaultyStore` and `SendFaultyStore` are chosen at the call site rather than reconciled by a bound | `CLAUDE.md` binding constraints 1, 3, 4; RS-20-2/20-3/20-4. This story must not be the reason the wasm32 steps or `edge-flavour-and-wasm-claim` go red |
| **NF-004** | **No `unwrap`/`expect` in library code.** The panic inside `then` is the one deliberate panic, it is documented in a `# Panics` section, and it is `#[track_caller]` | `standards/rust/00-prime-directives.md`; RS-70-2's obligation that a panicking item says so |
| **NF-005** | **No literal position assertions anywhere.** Where a position is needed it is the one the store returned; nothing computes `head - checkpoint` | `CLAUDE.md`, *The rule that matters*; `crates/happenstance-core/src/store.rs:240`. `GappyMemoryStore` (the slice-mate's) is the instrument that would catch a violation |
| **NF-006** | **The dev-dependency must not silently reorder publication.** The route chosen for `happenstance-testkit` (path-only versionless, or accepted publish order flagged to M7) is recorded in the ledger, not discovered by `publish-0-2-0-alpha-1` | *Data and migrations*, item 1; CF-32 |
| **NF-007** | **MSRV 1.97.1 holds.** Nothing here requires a newer compiler, and no dependency is added that could move the floor | ADR-0029; `CLAUDE.md`, binding constraint 5. Proven by CI's `msrv` job, not by the local gate |
| **NF-008** | **Determinism.** Every test is single-threaded and order-independent; the failure message contains no address, no timestamp and no hash, so a `#[should_panic(expected = …)]` match is stable | RS-60-* generally; a message carrying a `{:p}` would make AC-002's assertion flaky and is forbidden |

## Implementation notes (non-prescriptive)

These are the reasoning traps this story walks into, not instructions.

**The set difference is the whole module.** *Selected* is what `boundary.query()` returned;
*seeded* is what `Given::event` appended. Region 4 is `seeded \ selected`. Keeping the seeded
values in the `Given` builder — rather than re-reading them from the store with a second, wider
query — is the cheaper shape and the honest one: a second query is a second filter vocabulary, and
*Context pack* item 2 forbids exactly that. Whatever shape wins, the DSL's own tests must not use it
to compute their expectation (RS-60-4).

**`MemoryStoreError` is uninhabited** (`crates/happenstance-core/src/memory.rs:291`). That is a
feature — it means the `Err` arm on this store reduces to codec failure — but it also means EC-005's
store-failure path is *not* reachable through `given`. It is reachable through the `FaultyStore`
test, which is a different store. Do not add a fallible memory store to make a test symmetric.

**Rendering belongs in its own function, not in `then`.** A `fn render(expected, actual, selected,
seeded) -> String` that `then` panics with is testable directly at the string level, which is what
AC-002/004/005 need; a renderer that only exists inside a panic can be asserted on only through
`catch_unwind`. Keep `#[track_caller]` on `then` itself — it does not propagate through a helper's
own frame, so the attribute must sit where the panic is raised.

**The `Given` builder is consuming (`self` → `Self`), which the design's signatures already fix.**
That makes `?` in a chain natural and makes a half-built `Given` unusable after an encode failure.

**`assert_domain_event` is a residual, and its doc comment must say so once** — the alternative that
lost is "pretend a hand-written impl can enforce it", and it cannot, because no `const` sees the
match arms (RS-70-5; `../_design.md`, *Shape decision*, the `EVENT_TYPES` ↔ `event_type()` row). The
number that comes out of this story feeds AC-013's verdict in `defect-log-and-macros-verdict`.

**The doctest is `async` and therefore needs a hidden runtime and an `# Ok::<(), E>(())` closer**
(RS-62-2). Two hidden lines, both harness — the design permits exactly those two and no more. Keep
the fence at 72 columns; the domain yields first, never the ceremony.

**If `command-loop`'s signatures do not fit.** They are not this story's to change. Record the
mismatch as a ledger finding and escalate; an edit to `commit_with` inside this PR is scope drift
that the PR boundary will catch.

## Tests and CI (merge gate)

Grounded in `../_decomposition.md`'s testing brief, AC-009 row (*"the given/when/then DSL itself
gets unit tests… `FaultyStore<S>` and `GappyMemoryStore` are integration fixtures"*).

| tier | command / path | proves |
| --- | --- | --- |
| Unit | `cargo test -p happenstance --lib` over `crates/happenstance/src/testing/` | AC-001, AC-006, AC-007, AC-008 — the happy path, the refusal path, codec-faithful seeding, and that a dropped `Decision` appends nothing |
| Unit (rendering) | `cargo test -p happenstance --lib` over the renderer's own `#[cfg(test)] mod tests` | The four-region string at the string level: order, labels, the empty sentinel, the truncation asymmetry. Oracles spelled independently of the set-difference code (RS-60-4) |
| Integration (wrong implementation) | `cargo test -p happenstance --test dsl_failure_message` → `crates/happenstance/tests/dsl_failure_message.rs` | AC-002, AC-003, AC-004, AC-005, EC-006 — every case is a `#[should_panic(expected = …)]` or a `catch_unwind` over a **deliberately divergent model**, which is the "name a plausible wrong implementation" bar `CLAUDE.md` sets for any rule that is not decorative |
| Integration (no database) | `cargo test -p happenstance --test retry_without_a_database` → `crates/happenstance/tests/retry_without_a_database.rs` | AC-009 and EC-005 — the caller's retry loop under `FaultyStore<MemoryHandle>`, `attempts == 2`, the pristine re-fold, the bounded exhaustion, and that no branch reads `conflicting_position` |
| Integration (mount) | `cargo test -p happenstance --test mounted_at_the_crate_root` → `crates/happenstance/tests/mounted_at_the_crate_root.rs` | AC-010's mount half — the items are named through `happenstance::testing::…`, so a module that compiles but is never declared in `lib.rs` fails here rather than passing silently |
| Doctest | `cargo test -p happenstance --doc` | The module example compiles and runs; the two hidden lines are harness only; the fence stays inside 72 columns (checked by the density assertion in the mount test, which reads the source) |
| Feature | `cargo build -p happenstance --no-default-features --features std` | NF-002 — `memory` off leaves everything else compiling; RS-51-1 |
| Docs | `cargo doc -p happenstance --no-deps` and `… --no-default-features`, with `RUSTDOCFLAGS="-D warnings"` | AC-010's link half — no intra-doc link resolves in only some configurations (RS-70-2), and the `doc_cfg` badge renders (RS-70-4/51-5) |
| Lint | `cargo clippy --workspace --all-targets -- -D warnings` | EC-007's `#[must_use]` strength; NF-004's no-`unwrap` rule |
| **Story gate** | `cargo xtask affected --base main` | The story grain wired at `.redkiln/config.yaml`'s `affected_gate` — only what this diff could break, which is the bar for merging this PR |
| **Project gate** | `cargo xtask ci --fast` | `.redkiln/config.yaml`'s `integration_scoped` — the non-terminal project bar, including all four `wasm32` steps (NF-003) |
| Ledger | `redkiln verify --grain story` | `require_ledger: true` — every AC-### in this spec carries a satisfied row with cited evidence before `implement → report` |

Two things this story deliberately does **not** run: the `cargo-hack` feature powerset (this
project's own bar drops it; it is a pre-publish gate owned by `publish-0-2-0-alpha-1`), and any
`trybuild` case (`compile-fail-proof-artefact`'s, and blocked on an input this story does not need).

## Risks and coupling (PR-scoped)

| # | risk | mitigation, inside this PR |
| --- | --- | --- |
| 1 | **The slice-mate has not landed `FaultyStore` yet.** M4 sequences `misbehaving-testkit-stores` first *because it depends on nothing in this project* (`../_storymap.md`, order note 4) | Implement in that order inside the single slice context. If `FaultyStore` is genuinely absent, AC-009 halts loudly — it is not satisfied by a hand-rolled local double, which would be a second fixture shape in a second crate |
| 2 | **`command-loop`'s signatures are consumed, not owned.** A `Retry`/`Committed`/`CommandError` shape that does not fit the demonstration is a real possibility | Record as a ledger finding and escalate. The PR boundary block excludes those files, so an accidental edit fails `redkiln verify --grain story` rather than merging |
| 3 | **The dev-dependency reorders publication.** A versioned dev-dependency on `happenstance-testkit` would force it to publish before `happenstance` | NF-006: one of the two admissible routes is chosen and **named in the ledger**, so M7 inherits a decision rather than a surprise |
| 4 | **Region 4 is tested by the code that produces it.** The most likely defect — a set difference that silently returns empty — passes its own test | RS-60-4 is written into AC-003's verification: the oracle is spelled from the seeded values the test already holds |
| 5 | **The design's `when`/`then_refused` contradiction is resolved by reading, not by amendment.** If a reviewer reads it the other way, two implementations are possible | *Context pack* item 4 records the resolution, the reasoning and the fact that **no signature changes**. If the resolution is rejected, that is a design amendment and a human gate, not a code edit |
| 6 | **Touching `crate-root-rustdoc` collides with its owning story.** This PR adds region 6 to a page other stories also edit | The change is *one region*, named in the PR boundary, and its position (between Features and the adapter pointer) is fixed by `../_design.md`'s *Composition* table, so the merge is positional rather than judgemental |
| 7 | **`#[track_caller]` silently stops working** if the panic moves into a helper frame | AC-002's `panic_location_is_the_callers_line` is the standing guard; it fails the moment the attribute and the panic separate |
| 8 | **A doc line or fence drifts past budget** and the page starts scrolling horizontally at 1024px (anti-pattern 3) | The mount test reads the module source and asserts the fence's column budget — a check that cannot be satisfied by an unstyled render |

## Dependencies

**Blocks on** (must be merged first; all three are in-project):

| story slug | why |
| --- | --- |
| `command-loop` | Supplies `commit`, `commit_with`, `Retry`, `Committed` and `CommandError` — the retry demonstration (AC-009) has nothing to drive without them, and `when`'s error type names `CommandError` |
| `decision-model-composition` | Supplies `Boundary` for tuples, which is what makes `given((b1, b2))` a real case rather than an aspiration (AC-001's composed-boundary test) |
| `misbehaving-testkit-stores` | Supplies `FaultyStore<S>` / `SendFaultyStore<S>`. **Same slice (M4), same context**, sequenced first because it depends on nothing in this project |

**Unlocks.** Nothing in `../_storymap.md` declares `given-when-then-dsl` as a dependency, and that
is stated plainly rather than inflated. Two downstream stories consume it without blocking on it:
`defect-log-and-macros-verdict` reads AC-013's residual off `assert_domain_event` and the mapping
ceremony this story makes visible, and `publish-0-2-0-alpha-1` publishes the surface this story adds
to `happenstance`. `worked-example-on-typed-layer` is independent of this story by the map's own
edges — it depends on `command-loop` and `decision-model-composition`, not on the DSL.

## Anchors (progressive disclosure)

Link, never paste. Each row says why the artefact is load-bearing and the moment to open it.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | The signed-off, **binding** design: the `## Signatures` testing-surface block is the exact API to write, and *Composition*, *Density budget*, *States*, *Hierarchy* and *Anti-patterns* are the four-region message's specification | Before the first line of `src/testing/` — signatures first, then again before writing the renderer | AC-001, AC-002, AC-005, AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | Carries AC-U11's full statement (why a filter that hides its filter re-opens ADR-0020's hazard), AC-U12/U13/U14, and the testing brief's AC-009 row naming the tiers | When writing AC-003's oracle, and again when deciding which tier a test belongs in | AC-003, AC-008, AC-009 |
| `crates/happenstance-core/src/store.rs` | `read_decision_model` (`:321-329`) is the read half `when` is built on; `:131-145` is the port's own prohibition on threading `append`'s return into the next condition; `:240` is the no-gap-arithmetic rule | Before implementing `when`, and before any test that reasons about positions | AC-001, AC-009, NF-005 |
| `crates/happenstance-core/src/error.rs` | `:135-147` states in the port's own words that `conflicting_position: None` is legitimate and that a loop branching on `Some` *"works against an in-process store and stops working against a remote one"* | Before writing the retry demonstration's loop — this is the wrong implementation it must not contain | AC-009 |
| `crates/happenstance-core/src/memory.rs` | `MemoryStoreError` is uninhabited (`:291`), which is why `when`'s `Err` arm reduces to codec failure here; `spawns_from_generic` (`:643-680`) is the workspace's pattern for discharging a `Send` bound at the definition | When the `Err` arm looks untestable, and when adding any generic bound | AC-006, NF-003 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` (`:243-292`) is the reference implementation of *"one fixture instance is one isolated backing store"* — the rule `given`'s per-call store mirrors | Before deciding what `given` constructs and how isolated it is | AC-001 |
| `crates/happenstance-testkit/src/contract.rs` | `skip_line` (`:500-507`) is the in-tree precedent for the whole story: a declined capability **prints its reason rather than vanishing**. Region 4 is that principle applied to a filter | When tempted to omit an empty region — read this before deciding the empty state | AC-004 |
| `standards/rust/60-what-a-test-must-prove.md` | RS-60-4 (`:276`) — the oracle shares no subroutine with the implementation; RS-60-1 (`:12`) — open the subject inside the caught closure, which is how the `catch_unwind` assertions must be written | Before writing any test in `tests/dsl_failure_message.rs` | AC-002, AC-003 |
| `standards/rust/20-two-flavour-ports.md` | RS-20-2/20-3/20-4 — bind `EventStore`, one flavour name per module, and why `FaultyStore` is two types rather than one (`error[E0119]`) | The moment a bound or an import of a flavour name is written | NF-003, AC-009 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-2 — an `.await` example needs a runtime, and how the two hidden lines are permitted to be spelled | Before writing the module's example | AC-010 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-2 (links resolve in every configuration), RS-70-4 (`doc_cfg` badge), RS-70-5 (name the alternative that lost, once) | Before writing the module doc and `assert_domain_event`'s comment; again before the docs step | AC-007, AC-010 |
| `standards/rust/51-features-and-no-std.md` | RS-51-1 (*a feature adds*) and RS-51-5 (the manifest half of the badge) | When adding `#[cfg(feature = "memory")]` and the docs.rs metadata | NF-002, AC-010 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P1 (`:42-80`) is who every AC above is written for; P4 (`:249-290`) is time-boxed and one-shot, which is why AC-010 is about a rendered page and not a compiled item | When an AC's framing feels like a capability rather than a goal — re-read the persona and rewrite it | all |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` | Project AC-009 verbatim (`:192-196`), DR-05 (*reuse `Query` rather than inventing a second filter vocabulary*), and the *Out of scope* rule that routes a frozen-contract defect to AC-012 rather than to an edit | Before adding any filtering vocabulary, and the moment a `happenstance-core` defect is found | AC-003, AC-007 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md` | M4's ordering note — the slice-mate lands first — and the coverage row that splits project AC-009 across the two stories | At the start of the slice, to fix the implementation order | AC-009 |
| `RUNBOOK.md` | `:3977-3979` is the roadmap sentence this story rests on (*"`MemoryEventStore` is all this project needs"*, and Beat 2 separated from Beat 3); `:4031-4041` is phase 7's DSL line item and the misbehaving stores it drives; `:524` is AC-013's boilerplate-versus-domain measurement | When the scope of "no database" is questioned, and when writing `assert_domain_event`'s doc | AC-001, AC-007 |
| `crates/happenstance/src/lib.rs` | The mount point, 76 lines today, whose only item is `pub use happenstance_core::*;` (`:75`). `:53-56` is the promise that no new name shadows a contract re-export | Before adding `pub mod testing;` and the landing-page region | AC-010 |
| `.redkiln/config.yaml` | The `verify:` block wires `affected_gate` and `integration_scoped` to the story and integration grains, and sets `require_ledger: true` | Before claiming the story is done | AC-010, ledger |

## Clarifications resolved during spec

1. **The AC set is exactly the front half's ten**, AC-001 through AC-010. None added, none dropped.
   `assert_domain_event` is verified **inside AC-007** rather than in an eleventh row, because it
   makes the same claim as codec-faithful seeding — *the mapping the test exercises is the mapping
   the application ships* — and splitting it would produce two rows with one idea. Its wrong
   implementation (a variant returning a type absent from `EVENT_TYPES`) is still a named,
   `#[should_panic]` test, so the coverage is not softened by the merge.
2. **`when`'s `Err` arm versus `Decision::then_refused` is resolved by reading, not by amendment**
   (*Context pack*, item 4). The decide closure's `Err(d)` is captured *into* the `Decision`; `when`
   returns `Err` only for store and codec failure. Both signed-off signatures stay callable and
   unchanged. Recorded here as well as in the context pack because it is the one place this spec
   resolves an internal contradiction in a binding, human-approved document.
3. **The retry demonstration is a separate integration test, not a DSL feature.** `given` is fixed
   to a single-threaded `MemoryEventStore`, so `ConditionViolated` is unreachable through it. Adding
   contention *to the DSL* to satisfy AC-U13 was rejected: it would make `given` a store-behaviour
   harness, which is `happenstance-testkit`'s job, and it would put a second fixture vocabulary in
   the crate that must not have one.
4. **`EC-006` is this spec's addition, not the design's.** The design does not say what `then` does
   on a refused `Decision`. Left unstated, the natural implementation compares against `&[]` and a
   refusal passes as "emitted nothing" — a silent pass in the one place this story exists to make
   noisy. The behaviour is therefore specified (panic naming the refusal) and given a test. This is
   a gap filled inside the design's shape, not a change to it.
5. **Anti-pattern 5 is applied scoped to this story's own items**, per the mock's own finding
   (`../_design.md:1207-1213`): every over-budget summary on the crate-root page arrives through
   `pub use happenstance_core::*` and is frozen by AC-A02. Holding this story to the unscoped
   reading would make AC-010 unsatisfiable by construction.
6. **No conformance rule and no specification clause is owed**, and both are stated as decisions in
   the *Integration contract* rather than left blank. Nothing here is adapter-observable and
   `spec/SPECIFICATION.md` has no typed-layer namespace.
7. **The `happenstance-testkit` dev-dependency route is left to the implementer but not left
   open** — one of the two admissible routes must be chosen and named in the ledger (NF-006). A spec
   that picked one would be deciding publication order for M7 from inside a test-surface story; a
   spec that said nothing would let M7 inherit it unannounced. Naming the choice as a ledger
   obligation is the middle path.
