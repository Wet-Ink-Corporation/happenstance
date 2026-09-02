---
item: HS-P0011
stage: storymap
created: 2026-08-12T03:30:13.293Z
updated: 2026-08-12T03:30:13.293Z
template_sig: 1c63534a
rendered_sig: 4404283a
---

# Story Map — The typed layer, the worked example, and 0.2.0-alpha.1

## Backbone

The activities are the application author's first hour, in the order they happen —
the journey *"Choose a contract before a database"*
([`../initiative.md`](../initiative.md), and
[`_decomposition.md`](_decomposition.md), *UX brief → Intent*). Each column is an
outcome a `cargo add happenstance` user reaches; the slices beneath it are what
make it reachable.

| # | Activity (the user's outcome) | Where it lands | Beat it closes |
| --- | --- | --- | --- |
| **A1** | **Decide the shape before writing it** — the two answers this layer is built on exist as records naming the alternatives that lost | `.kb/_intake/` → `.kb/decisions/` | AC-016's "written first" is a sequencing obligation on this map, not a formality (`_decomposition.md`, *The ADR route, and who invokes it*) |
| **A2** | **Model a consistency boundary** — define typed events, fold a decision model, compose several into one query | `crates/happenstance/src/` | Beat 1 of the journey |
| **A3** | **Run a command against a store** — encode a payload, read → decide → append under a condition, retry on `ConditionViolated` | `crates/happenstance/src/` | Beat 3 |
| **A4** | **Test the decision without a database** — seed, decide, assert; and make the store misbehave on purpose | `crates/happenstance/src/`, `crates/happenstance-testkit/src/` | Beat 2 — today *"pick a database"* and *"does the domain model work"* are one decision (`RUNBOOK.md:3977-3979`) |
| **A5** | **Read the events back into a read model** — an application-facing `Projection` over decoded events, and the clause verdicts the runner's existence settles | `crates/happenstance/src/`, `spec/SPECIFICATION.md`, `experiments/` | Beat 4 |
| **A6** | **See it actually work, and see the compiler protect it** — the canonical DCB example on the typed layer, plus the instrument that proves the protection is real | `examples/course-subscriptions/`, `xtask/src/proof.rs` | Beat 4; the project's proof artefact |
| **A7** | **Get it from the registry, and know what it promises** — `0.2.0-alpha.1`, the churn mitigations, and the record of what using the frozen contract revealed | `crates/happenstance/README.md`, `CHANGELOG.md`, `.kb/_intake/` | P4's one bounded sitting (`_decomposition.md`, *UX brief*, persona table) |

## Slices

The thin vertical slices (stories) under each activity, grouped by the milestone/slice they are delivered
with. Stories sharing a Milestone are implemented together in a single context and mounted as ONE integrated
surface (the implement stage runs one milestone at a time). Cross-milestone `depends_on` edges must be acyclic.
Type each story `capability` (a user-observable slice) or `foundation` (real in-tree substrate a capability
slice in this initiative consumes — never owned outside it, never a double/fixme).

Every story in M2–M6 mounts into the composition roots the architecture brief
enumerates ([`_decomposition.md`](_decomposition.md), *Composition roots*): the new
items are `pub use`d at `crates/happenstance/src/lib.rs` **beside** the surviving
`pub use happenstance_core::*;` (`:75`, AC-A01), and the matching "Planned, and
specified in `spec/SPECIFICATION.md`" bullet at `:35-52` becomes an intra-doc link
to the real item in place. "Build it" and "wire it in" are never separate stories —
a story that does not replace its own roadmap bullet is not done.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| **M1 `decision-records`** | `adr-0020-fold-query-agreement` | foundation | Stage into `.kb/_intake/` the decision that a decision model folds a domain enum and *derives* its `Query` from `EVENT_TYPES` + tag constraints — resolving DT-2's concrete form, *is `query()` infallible and where did the validation go* (`_decomposition.md`, *The one signature question DT-2 actually is*) — and hand off to `/redkiln:kb-ingest` so ADR-0020 exists as an atom before the code it governs. | — | AC-001, AC-014, AC-016 |
| **M1 `decision-records`** | `adr-0021-payload-evolution-and-codec-tag` | foundation | Stage into `.kb/_intake/` the decision on payload evolution — whether `EventType` carries a version suffix, whether an upcaster needs a read-path hook `EventStore` does not have, and which of the two admissible homes the codec tag takes (`Event::metadata`, `crates/happenstance-core/src/event.rs:379`, vs `Tags`) under the constraint that **no adapter may need to understand it** — ingested in the same wave as ADR-0020. | — | AC-005, AC-016 |
| **M2 `typed-vocabulary`** | `domain-event-and-decision-model` | capability | Land `DomainEvent` with `const EVENT_TYPES: &'static [EventType]` (const-constructible already — `crates/happenstance-core/src/event.rs:108`) and `DecisionModel` with `apply(&mut self, Self::Event)` plus the derived `query()` ADR-0020 chose, exported at the crate root with the complete first-program doctest, so a model whose fold and query disagree is unwritable. | `adr-0020-fold-query-agreement` | AC-001, AC-014 |
| **M2 `typed-vocabulary`** | `decision-model-composition` | capability | Land the declarative macro over `(P1, P2)`, `(P1, P2, P3)`, … that OR's each model's query fragment at compile time — `$crate`-qualified paths and most-literal-first arms (`standards/rust/41-declarative-macros.md`) — with a unit test asserting the composed `Query` is the union of the members' own. | `domain-event-and-decision-model` | AC-004 |
| **M3 `codec-and-command-loop`** | `codec-and-feature-forwarding` | capability | Land `Codec` with JSON by default and CBOR/postcard behind forwarded features in `crates/happenstance/Cargo.toml`'s `[features]` block (AC-A04; a feature only ever *adds*), events carrying the tag ADR-0021 sited, round-trip tests per codec — payloads stay `Bytes` at the port and encoding never routes through `happenstance-core/serde`. | `adr-0021-payload-evolution-and-codec-tag` | AC-005 |
| **M3 `codec-and-command-loop`** | `command-loop` | capability | Land read → decide → append → retry-on-`ConditionViolated`, taking its `after` anchor from the read (`read_decision_model`, `crates/happenstance-core/src/store.rs:321`) and never from `append`'s return (`:131-145`), treating `conflicting_position` as the hint it is (`crates/happenstance-core/src/error.rs:135-147`), with a bounded, caller-visible retry, a `#[must_use]` decision value and the policy stated in the loop's own rustdoc beside the *verbatim resubmission* distinction (`RUNBOOK.md:4015-4020`). | `domain-event-and-decision-model`, `codec-and-feature-forwarding` | AC-005 |
| **M4 `testing-surface`** | `misbehaving-testkit-stores` | capability | Land `FaultyStore<S>` and `GappyMemoryStore` in `crates/happenstance-testkit/src/` — two types per flavour, not one (`standards/rust/20-two-flavour-ports.md` RS-20-4), reachable from the testkit root — each shipped with the wrong implementation it rejects: a retry loop that branches on `Some(conflicting_position)`, and a handler that assumes `position + 1`. | — | AC-009 |
| **M4 `testing-surface`** | `given-when-then-dsl` | capability | Land the given/when/then DSL over `MemoryEventStore` in `crates/happenstance/`, seeding and nominating through `happenstance_core::Query` only, whose failure message names the events **seeded but not selected** by the model's query (AC-U11 — a filter that hides what it filtered re-opens the hazard ADR-0020 closes), demonstrated by driving a caller's retry loop against `FaultyStore<S>` with no database. | `command-loop`, `decision-model-composition`, `misbehaving-testkit-stores` | AC-009 |
| **M5 `projection-runner`** | `projection-trait-and-runner` | capability | Land the application-facing `Projection` trait and its runner in `crates/happenstance/`, layered over `crates/happenstance-core/src/projection.rs`'s checkpoint port — nominating events with `Query`, decoding them, opening `begin` and committing read-model write + checkpoint in the single `commit` (`:126`), advancing past the inclusive `from` (`:104-106`), and **streaming, never `collect`ing** (AC-A05) — with its feature gating stated explicitly rather than inherited. | `codec-and-feature-forwarding` | AC-006 |
| **M5 `projection-runner`** | `projection-clause-verdicts` | capability | Write the verdicts the runner's existence makes answerable — PS-33 discharged by naming an independent caller of the checkpoint pump or by the superseding ADR that collapses it upward, PS-27 and PS-30 settled by counting with their integration tests, PS-18 recorded as a documented exclusion (`RUNBOOK.md:4070-4077`; its subject does not exist in the tree) — as the three-part `spec/SPECIFICATION.md` edit: generated §7.1/§7.2, the hand-written §1.3 census, and IDs retained on exit (`:209-212`), with `cargo xtask spec-trace` green. | `projection-trait-and-runner` | AC-007, AC-008 |
| **M5 `projection-runner`** | `polling-cost-measurement` | capability | Land a reproducible harness under `experiments/` that records the N views × N reads cost ES-32 imposes on the polling runner as a **number with its conditions** — outside the gate by construction (`CLAUDE.md`, repository map; CF-34), so a post-0.1 tail-seam decision argues from a measurement rather than an estimate. | `projection-trait-and-runner` | AC-010 |
| **M6 `worked-example-and-proof`** | `worked-example-on-typed-layer` | capability | Rewrite `examples/course-subscriptions/` onto `happenstance` — its manifest dependency moves off `happenstance-core`, `parse_capacity` (`src/main.rs:231`) and the `format!("…").into_bytes()` payload (`:96-99`) are deleted rather than wrapped, `commit` (`:207`) is deleted *into* the command loop, `main`'s observable steps (`:37-79`) survive verbatim — and add the integration test that actually **executes** the binary via `CARGO_BIN_EXE_course-subscriptions`, because nothing in the gate runs `main` today and AC-003 is an execution claim. | `command-loop`, `decision-model-composition` | AC-003 |
| **M6 `worked-example-and-proof`** | `compile-fail-proof-artefact` | capability | Land the compile-fail case that adds a variant to the example's domain enum and its **negative control**, with the diagnostic landing on the user's `match` arm and not inside a macro body (AC-U07), registered as a row in `xtask/src/proof.rs`'s `ARTEFACTS` (`:133`) so the gate runs `cargo xtask proof-artefact` — which a deleted *or* emptied case fails — and verify the negative control fails rather than assuming it. | `worked-example-on-typed-layer` | AC-002 |
| **M7 `alpha-release`** | `edge-flavour-and-wasm-claim` | capability | Settle AC-A06 by name — either a fifth `wasm32` step compiling `happenstance` added to `xtask/src/main.rs::REQUIRED` (`:105`) and `wasm_steps()`, or a recorded statement that the typed layer makes no `wasm32` claim at the alpha and why — and pin the flavour discipline for this crate's own generic code (`EventStore`, never `SendEventStore`; one flavour name per module; no `#[async_trait]`; the `spawns_from_generic` bound pattern, `crates/happenstance-core/src/memory.rs:643-680`) with all four existing `wasm32` steps green. | `command-loop`, `projection-trait-and-runner` | AC-015 |
| **M7 `alpha-release`** | `defect-log-and-macros-verdict` | capability | Land the BR-01 record this project exists to produce: every defect using the `[FROZEN]` contract revealed, each naming its clause ID and its routing to a decision record (never a line edit — AC-A02), staged for `/redkiln:kb-ingest`; plus the `happenstance-macros` verdict measured over the *rewritten* example — boilerplate versus domain logic (`RUNBOOK.md:524`) — recorded in or out either way. | `worked-example-on-typed-layer`, `projection-trait-and-runner` | AC-012, AC-013 |
| **M7 `alpha-release`** | `publish-0-2-0-alpha-1` | capability | Cut `0.2.0-alpha.1` for `happenstance-core` then `happenstance` (`happenstance-testkit` on its own CF-32 number), after one full `cargo xtask ci` — not `--fast`, which drops the feature powerset and `cargo deny` (`xtask/src/main.rs:835`) — with the README `## Stability` section landing between `## Guarantees` and `## Design`, `CHANGELOG.md`'s `## [Unreleased]` becoming the alpha's section *and* its `unstable-projection` claim reconciled with the manifest, the yank policy stated, and post-publish resolution verified with an explicit pre-release requirement. | `compile-fail-proof-artefact`, `projection-clause-verdicts`, `edge-flavour-and-wasm-claim`, `defect-log-and-macros-verdict` | AC-011 |

## Coverage

Every project acceptance criterion is covered by at least one story, and each has a
single owning story (extra rows are the record that *decides* a thing versus the
code that *implements* it, which is AC-016's required split, not duplicated
responsibility).

| Project AC | Covered by | Owner |
| --- | --- | --- |
| AC-001 fold and query cannot disagree | `adr-0020-fold-query-agreement`, `domain-event-and-decision-model` | `domain-event-and-decision-model` |
| AC-002 compiler protection, checked | `compile-fail-proof-artefact` | same |
| AC-003 worked example runs | `worked-example-on-typed-layer` | same |
| AC-004 boundaries compose | `decision-model-composition` | same |
| AC-005 typed, evolvable payloads | `adr-0021-payload-evolution-and-codec-tag`, `codec-and-feature-forwarding`, `command-loop` | `codec-and-feature-forwarding` |
| AC-006 projection over decoded events | `projection-trait-and-runner` | same |
| AC-007 PS-33 evaluated | `projection-clause-verdicts` | same |
| AC-008 PS-18/PS-27/PS-30 counted | `projection-clause-verdicts` | same |
| AC-009 test a decision, test misbehaviour | `misbehaving-testkit-stores`, `given-when-then-dsl` | split by crate: testkit stores vs. the DSL |
| AC-010 polling cost measured | `polling-cost-measurement` | same |
| AC-011 `0.2.0-alpha.1` on the registry | `publish-0-2-0-alpha-1` | same |
| AC-012 contract defects recorded | `defect-log-and-macros-verdict` | same |
| AC-013 `happenstance-macros` answered | `defect-log-and-macros-verdict` | same |
| AC-014 DT-2 resolved on the record | `adr-0020-fold-query-agreement`, `domain-event-and-decision-model` | `adr-0020-fold-query-agreement` |
| AC-015 the edge flavour survives | `edge-flavour-and-wasm-claim` | same |
| AC-016 ADR-0020/0021 written first | `adr-0020-fold-query-agreement`, `adr-0021-payload-evolution-and-codec-tag` | one each |

Sixteen of sixteen. No AC is orphaned and no two stories own the same
responsibility.

**Two criteria have a stage artefact as well as a story, and the story does not
replace it.** AC-014 is decided in `_design.md` and human-approved (DoD 5) *before*
M1 is staged; `adr-0020-fold-query-agreement` is the durable record of that
decision and `domain-event-and-decision-model` is the shape it produces plus the
unit test of the chosen failure mode ([`_decomposition.md`](_decomposition.md),
*Testing brief*, AC-014 row). AC-011's registry resolution is a manual post-publish
check no gate step can reach (DEP-006); the story owns it as an explicit acceptance
step, not as an automated assertion.

## Merge order

Foundation before its consumers, then the slices in dependency order. Each
milestone is handed to one implementer as one integrated surface.

1. **M1 `decision-records`** (foundation) — `adr-0020-fold-query-agreement`,
   `adr-0021-payload-evolution-and-codec-tag`. Both stage into `.kb/_intake/` and
   are ingested in **one** `/redkiln:kb-ingest` wave. This milestone is first
   because AC-016 requires both atoms to exist before the code they govern, and
   because an accepted atom is immutable — a correction after the fact is a second
   atom, not an edit ([`_decomposition.md`](_decomposition.md), *The ADR route, and
   who invokes it*).
2. **M2 `typed-vocabulary`** — `domain-event-and-decision-model`, then
   `decision-model-composition`. Consumes ADR-0020. First surface a reader meets.
3. **M3 `codec-and-command-loop`** — `codec-and-feature-forwarding`, then
   `command-loop`. Consumes ADR-0021 and M2. The write path becomes runnable here.
4. **M4 `testing-surface`** — `misbehaving-testkit-stores` first (it depends on
   nothing in this project), then `given-when-then-dsl`, which demonstrates both.
   `happenstance-testkit` moves on its own version (CF-32); do not couple it to
   `happenstance`'s number.
5. **M5 `projection-runner`** — `projection-trait-and-runner`, then
   `projection-clause-verdicts` and `polling-cost-measurement` (independent of each
   other). Sequenced after M3 because the runner decodes.
6. **M6 `worked-example-and-proof`** — `worked-example-on-typed-layer`, then
   `compile-fail-proof-artefact`, which points its diagnostic at the example's own
   `match` arm and therefore cannot precede it.
7. **M7 `alpha-release`** — `edge-flavour-and-wasm-claim` and
   `defect-log-and-macros-verdict` (independent), then `publish-0-2-0-alpha-1` last.
   The publish is the one irrevocable act in the project.

M4 has no dependency on M5 or M6 and may be merged in parallel with M5 if capacity
allows; nothing else in the order is optional.

## What is deliberately not a story here

Named so silence is not read as an oversight.

- **`MemoryProjectionStore`.** `projection-trait-and-runner`'s transactional test
  needs an in-memory `ProjectionStore` to run against, and shipping that fixture is
  `projection-store-freeze`'s (HS-P0010) own AC-012 — writing a throwaway here would
  be freezing a fixture shape this project does not own ([`project.md`](project.md),
  *Out of scope*, first bullet). It is a **project-level** dependency already
  recorded in [`project.md`](project.md), *Dependencies*, not a story on this map.
- **`trybuild`'s adoption.** AC-002's instrument, and HS-P0010's decision
  (`spec/SPECIFICATION.md:8772`). A `compile_fail` doctest is not a substitute: the
  negative control — the entire point of AC-002 — cannot discriminate
  (`RUNBOOK.md:3614-3627`). Raise it as an explicit input to HS-P0010 **before** this
  project's design gate; escalate rather than substitute if it is declined.
- **Editing `crates/happenstance-core/src/**`.** Admissible only as the recorded
  outcome of AC-012's route, never as a convenience discovered mid-implementation
  (AC-A02). No story is licensed to amend a `[FROZEN]` ES-\* clause.
- **`happenstance-macros` as a crate.** AC-013 answers the question; if the answer is
  *in*, a new workspace member is a scope change for the runbook to take, not a story
  on this map ([`_decomposition.md`](_decomposition.md), AC-013 row).
- **The MSRV promise, the semver verdict, registry presentation and DT-1.** All
  HS-P0016's ([`project.md`](project.md), *Out of scope*). This project creates the
  baseline and makes none of the promises.
- **Persona promotion into `.kb/product/`.** HS-P0019's.

Two steps in this map are **human handoffs, not implementation steps**:
`/redkiln:kb-ingest` (M1, and again for M7's defect log) and `cargo publish` (M7).
Plan them as handoffs at the point the material is ready.
