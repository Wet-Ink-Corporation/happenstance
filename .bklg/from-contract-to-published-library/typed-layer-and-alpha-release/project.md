---
id: HS-P0011
uid: d57664
type: project
slug: typed-layer-and-alpha-release
title: The typed layer, the worked example, and 0.2.0-alpha.1
parent: HS-I0006
initiative: from-contract-to-published-library
project: typed-layer-and-alpha-release
status: implementing
process: project
stage: implementation
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-16T00:19:48.476Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# The typed layer, the worked example, and 0.2.0-alpha.1

## One-line objective

Put a real consumer above the contract — `DomainEvent`, `DecisionModel`, `Codec`,
the command loop, the application-facing `Projection` trait and runner, a
given/when/then DSL and the worked example rewritten on top of them — and publish
`0.2.0-alpha.1`, so the contract is discovered by use rather than by a type checker
looking at a skeleton.

## How this advances the initiative

The initiative's first goal is *"the contract gets used"*
([`../initiative.md`](../initiative.md), *Goals*). Today the crate people will
actually `cargo add` is five lines — `pub use happenstance_core::*;`
(`crates/happenstance/src/lib.rs:75`) — so nothing has ever played the role of
consumer and no contract defect has been found by anything other than a compiler
agreeing with a `todo!()`. This project is the consumer.

It is also the initiative's second act structurally. `RUNBOOK.md:256-258` states the
7-before-8 constraint in terms — *"the typed layer is the consumer that discovers
contract defects, and discovering them after the flagship adapter is written is the
sequence this plan exists to avoid"* — which is why this project blocks
`sqlite-durable-store`. And it is the only project that can create the semver
baseline: `publication-and-positioning`'s entire BR-07 instrument is a diff against
a prior published version, and there is no prior published version until this one
publishes (`../_decomposition.md`, *Sequencing → Substrate ahead of its consumers*).

Owned from the initiative's spine: **BR-01**, **BR-05**ᵅ (the alpha half),
**AC-01**, **AC-02**, **DoD 1**, **DoD 2**, **DT-2** — plus **BR-15** as the
standard of work every project applies, discharged here as ADR-0020 and ADR-0021.
Contributed but not owned: BR-07 and BR-09 (this project *creates* the baseline and
the first registry artefact; it makes no promise about either) and BR-12.

## In scope (this project)

Scoped from `RUNBOOK.md:3971-4102` (phase 7) and `RUNBOOK.md:4131-4162` (the alpha),
which the decomposition maps onto this project one-to-one.

- **`DomainEvent`** with `const EVENT_TYPES: &'static [EventType]` — const-constructible
  since phase 5 — designed against a hand-written expansion of what a derive would
  emit, so the trait does not move when a derive lands (`RUNBOOK.md:4002-4004`).
- **`DecisionModel`** — `type Event: DomainEvent`, `fn apply(&mut self, Self::Event)`,
  `fn query(&self) -> Query` derived from `EVENT_TYPES` and the model's tag
  constraints, never hand-maintained. This is the mechanism that closes the hazard
  the phase exists for: a DCB handler names its event set twice, and
  `examples/course-subscriptions/src/main.rs:114-173` demonstrates the divergence
  today with no compiler, clippy or conformance signal against it.
- **Composition** for `(P1, P2)`, `(P1, P2, P3)`, … by a small macro, so N is
  compile-time and each model's query fragment is OR'd automatically. Composing
  several models into one query is what makes a dynamic consistency boundary
  *dynamic*.
- **`Codec`** — JSON first, CBOR and postcard behind features; events carry a codec
  tag so one store can hold more than one encoding. `serde` belongs here: ADR-0003's
  prohibition attaches to `happenstance-core`, and after
  [`ADR-0006`](../../../.kb/decisions/0006-bare-name-to-the-typed-layer.md) this crate
  is the *typed* layer whose entire job is encoding.
- **The command loop** — read → decide → append → retry on `ConditionViolated` —
  with the retry policy stated in the docs and distinguished from the retry-safety
  property of a *verbatim* resubmission (`RUNBOOK.md:4015-4020`).
- **The application-facing projection runner** — the `Projection` trait an
  application implements, layered over the checkpoint pump that stays in
  `happenstance-core`, per
  [`ADR-0007`](../../../.kb/decisions/0007-projection-runner-decodes.md). A projection
  nominates its events with `Query`; there is no second filter vocabulary.
- **ADR-0007's falsifier, evaluated** (PS-33) — `RUNBOOK.md:408-413` says it is
  evaluated at this phase's exit *and nowhere else*.
- **The three "does anyone call this?" clauses** — PS-18, PS-27, PS-30 — settled by
  counting callers (`RUNBOOK.md:4070-4077`).
- **The testing surface** — a given/when/then DSL over `MemoryEventStore`, plus
  `FaultyStore<S>` and `GappyMemoryStore` in `happenstance-testkit` so a user's
  retry loop and a handler that assumes `position + 1` are testable at all.
- **The polling cost ES-32 makes the runner pay**, recorded as a measurement.
- **ADR-0020** (fold/query agreement) and **ADR-0021** (payload evolution), written
  first, from the runbook's ADR queue (`RUNBOOK.md:299-300`).
- **The worked example rewritten** on typed events and a decision model — no
  `format!("…").into_bytes()` payloads, no hand-rolled `parse_capacity`.
- **`0.2.0-alpha.1` published**, with the three mitigations `RUNBOOK.md:4149-4155`
  names: a `## Stability` section in the README, a `CHANGELOG.md`, and yanking each
  alpha when the next lands.
- **Recording the contract defects this use discovers** as decision-record
  candidates (BR-01).

## Out of scope (this project)

- **Freezing `ProjectionStore`, the apply/write seam, the projection conformance
  suite and the capability-declension policy** → `projection-store-freeze`
  (HS-P0010). This project *consumes* the frozen port.
- **The MSRV promise, the semver diff verdict, the clause-ledger audit, registry
  presentation, positioning and the compliance claim** → `publication-and-positioning`
  (HS-P0016). This project publishes an alpha to *create* the baseline; it makes
  none of the promises.
- **Any storage adapter** → `sqlite-durable-store` (HS-P0012),
  `cloudflare-durable-object-store` (HS-P0013), `postgres-and-neon-stores`
  (HS-P0014). `MemoryEventStore` is all this project needs
  (`RUNBOOK.md:3977-3979`).
- **The third, structurally unlike batch shape and the freeze verdict** →
  `ladybug-projection-store` (HS-P0015).
- **Replication identity and the merge rule** → `replication-identity-and-ingest`
  (HS-P0017). **What a store may forget** → `retention-and-incomplete-logs`
  (HS-P0018).
- **Promoting personas into `.kb/product/`, the BR-15 audit, and DoD 13's whole-gate
  re-observation** → `closeout-and-durable-audience` (HS-P0019).
- **Amending any `[FROZEN]` ES-\* clause.** A defect this use discovers is recorded
  and routed to a decision record; it is not fixed by a line edit
  (`_intake-brief.md`, *Clauses*).

## Derived requirements

Expanded from the initiative requirements this project owns.

| ID | Derived requirement | From |
| --- | --- | --- |
| DR-01 | A domain enum, not a `SequencedEvent`, is what a decision model folds over, so the `match` is exhaustive and divergence between the query and the fold is a compile error rather than a silent DCB failure | BR-01; `RUNBOOK.md:3993-3998` |
| DR-02 | A decision model's `Query` is *derived* from its `EVENT_TYPES` and tag constraints and is never hand-maintained | BR-01; `RUNBOOK.md:4005-4007` |
| DR-03 | Several decision models compose into one query at compile time, because that composition is the DCB mechanism | BR-01, AC-01 |
| DR-04 | The typed layer encodes and decodes payloads and therefore depends on `serde`; `happenstance-core` still must not, in its default features | ADR-0003 as scoped by ADR-0006; `_intake-brief.md`, *Constraints* |
| DR-05 | The application-facing `Projection` trait and runner sit in `happenstance`, over the core checkpoint pump, and reuse `Query` rather than inventing a second filter vocabulary | ADR-0007 |
| DR-06 | Every provisional marker this phase is scheduled to evaluate is evaluated and the verdict written — PS-33, PS-18, PS-27, PS-30 — and PS-32/PS-33/PS-35 leave the clause space with their IDs retained | BR-06 (upstream), BR-15; `RUNBOOK.md:4064-4088` |
| DR-07 | A consumer can test a decision without a database, and can test what happens when the store misbehaves (append failures, position gaps) | BR-01; `RUNBOOK.md:4031-4041` |
| DR-08 | The cost ES-32 imposes on the polling runner is recorded as a number, so a post-0.1 tail-seam decision has something to argue from | BR-01; `RUNBOOK.md:4028-4030` |
| DR-09 | `0.2.0-alpha.1` exists on the registry, carries the three "this is not a commitment" mitigations, and is the artefact a later surface diff can be taken against | BR-05ᵅ, BR-07 (contributes) |
| DR-10 | Every answer this project settles lands as a decision record naming the alternatives that lost — ADR-0020 and ADR-0021 — written before the code they govern | BR-15; `RUNBOOK.md:4059` |
| DR-11 | Nothing here introduces `#[async_trait]`, makes `read` an `async fn`, or binds `SendEventStore` in generic code; the four `wasm32` gate steps stay green | BR-12; `CLAUDE.md` binding constraints 1, 3, 4 |
| DR-12 | Contract defects discovered by using the frozen contract are recorded as defects, with the clause named, rather than absorbed | BR-01; `_intake-brief.md`, *Clauses* |

## Acceptance criteria

Project-grain and testable. These are the spine `_storymap.md` must cover.

- **AC-001 — The fold and the query cannot disagree.** `DecisionModel::apply` takes
  `Self::Event` (a `DomainEvent` enum), and `query()` is derived from
  `EVENT_TYPES` plus the model's tag constraints. A model whose fold and query
  disagree is unwritable, and ADR-0020 records why this shape and not the
  alternatives.
- **AC-002 — The compiler protects the domain, and the protection is checked.** A
  `trybuild` compile-fail case adds a variant to the worked example's domain enum
  and the crate fails to compile until the fold handles it; the **negative control**
  — removing the protection makes the case fail — is present and runs in the gate.
  (Initiative AC-02, DoD 2. This is the project's proof artefact.)
- **AC-003 — The worked example runs on the typed layer.**
  `cargo run -p course-subscriptions` completes the canonical DCB cycle with **no
  `todo!()` reached**, using typed events and a decision model — no
  `format!("…").into_bytes()` payload construction and no hand-rolled
  `parse_capacity` survive. (Initiative AC-01, DoD 1.)
- **AC-004 — Consistency boundaries compose.** Tuples of decision models `(P1, P2)`,
  `(P1, P2, P3)`, … produce one OR'd query at compile time, exercised by the worked
  example's multi-model case.
- **AC-005 — Payloads are typed and can evolve.** `Codec` ships with JSON by default
  and CBOR/postcard behind features; an event carries a codec tag; and ADR-0021
  states whether `EventType` carries a version suffix and whether an upcaster needs
  a read-path hook `EventStore` does not have.
- **AC-006 — An application writes a projection against decoded events.** The
  `Projection` trait and its runner exist in `happenstance`, layered over
  `happenstance-core`'s checkpoint pump, nominating events with `Query`
  (`crates/happenstance-core/src/projection.rs`; ADR-0007).
- **AC-007 — ADR-0007's falsifier is evaluated, not inherited.** PS-33 is discharged
  by naming an independent caller of the core checkpoint pump *or* by writing the
  superseding ADR that collapses it upward. An unevaluated falsifier fails this
  criterion.
- **AC-008 — The three "does anyone call this?" clauses are settled by counting.**
  PS-18's protection variant, PS-27's skip record and PS-30's fan-out runner each
  name their callers or are promoted to a documented exclusion.
- **AC-009 — A consumer can test a decision, and can test misbehaviour.** A
  given/when/then DSL seeds a `MemoryEventStore`, invokes the decision and asserts on
  emitted events or the error; `FaultyStore<S>` and `GappyMemoryStore` exist in
  `happenstance-testkit` and a handler assuming `position + 1` fails against the
  latter.
- **AC-010 — The polling cost is a measurement.** The N views × N reads cost ES-32
  imposes is recorded as a number with the conditions it was taken under, not as an
  estimate.
- **AC-011 — `0.2.0-alpha.1` is on the registry.** The published version resolves,
  carries a README `## Stability` section naming the phase at which the API stops
  moving, a `CHANGELOG.md`, and the yank policy for superseded alphas; a later run of
  `cargo-semver-checks` has a baseline to compare against.
- **AC-012 — Contract defects found by use are recorded.** Every defect this
  consumer discovers in the `[FROZEN]` contract is written down with its clause ID
  and routed to a decision record; none is fixed by editing a frozen clause.
- **AC-013 — The `happenstance-macros` question is answered either way.** *If the
  rewritten example carries more mapping boilerplate than domain logic, the derive is
  in scope for 0.1* — evaluated and recorded, in or out.
- **AC-014 — DT-2 is resolved on the record.** How much a caller must state before
  their consistency boundary is checked — minimal ceremony versus explicit
  declaration — is decided in this project's `_design.md`, with the error-timing
  versus first-hour-cost trade stated.
- **AC-015 — The edge flavour survives this layer.** No `#[async_trait]` is
  introduced, `EventStore::read` still returns the stream at the top level, generic
  code in `happenstance` binds `EventStore` rather than `SendEventStore`, and all
  four `wasm32` steps in `cargo xtask ci` are green.
- **AC-016 — ADR-0020 and ADR-0021 are written first.** Both exist as decision atoms
  naming the alternatives that lost, and the matching open-question atoms are
  resolved rather than deleted (`redkiln validate --kb` clean).

## Definition of done (boundary-level)

1. `cargo run -p course-subscriptions` completes the DCB cycle end to end with no
   `todo!()` reached, on typed events. *(AC-003; initiative DoD 1)*
2. The `trybuild` compile-fail case and its negative control are in the gate and
   both behave as specified. *(AC-002; initiative DoD 2)*
3. ADR-0020 and ADR-0021 are accepted atoms under `.kb/decisions/`, written before
   the code they govern, and `redkiln validate --kb` is clean. *(AC-016)*
4. PS-33's verdict is written; PS-18, PS-27 and PS-30 are settled by a counted
   call; PS-32/PS-33/PS-35 have left the clause space with their IDs retained so
   citations resolve. *(AC-007, AC-008)*
5. `_design.md` is authored and human-approved, resolving DT-2 and enumerating every
   public item this project adds to `happenstance`. *(AC-014; the design stage's own
   review gate)*
6. `cargo xtask ci --fast` is green — this project's integration bar per
   `.redkiln/config.yaml:55` — and `cargo xtask spec-trace` passes.
7. `0.2.0-alpha.1` is published and resolvable from the registry with an explicit
   pre-release requirement, with README stability section, changelog and yank policy
   in place. *(AC-011)*
8. The polling-cost measurement and the `happenstance-macros` verdict are recorded
   in the project's closeout, either way. *(AC-010, AC-013)*
9. The defect log from BR-01 exists and is non-speculative: each entry names a
   clause ID and its routing. *(AC-012)*

## Dependencies

**Depends on**

- **HS-P0010 `projection-store-freeze`** — the `Projection` trait and runner are
  layered over a port whose batch shape, write seam and failure policy this project
  does not own. `RUNBOOK.md`'s 6→7 edge; `../_decomposition.md`, *Dependency DAG*.

**Unlocks**

- **HS-P0012 `sqlite-durable-store`** — the non-negotiable 7→8 edge
  (`RUNBOOK.md:256-258`).
- **HS-P0016 `publication-and-positioning`** — which cannot diff a surface against a
  baseline that does not exist until this project publishes.

## Risks and coupling notes

| Risk / coupling | Why it matters | Handling |
| --- | --- | --- |
| **The alpha becomes an argument against changing things.** `RUNBOOK.md:4157-4162` names the failure mode as the author, not the users: a published alpha makes *"we can't change that now"* available | The whole point of publishing here is feedback about decisions still worth challenging | The three mitigations are acceptance criteria, not intentions (AC-011); nothing after this project inherits a stability promise until HS-P0016 makes one |
| **`trybuild`'s adoption is HS-P0010's decision, and this project's proof artefact needs it.** `RUNBOOK.md:3130-3141` records that a `compile_fail` doctest is *not* a stronger check than it looks — rustdoc on 1.97.1 silently ignores an unmatched error-code annotation — and defers pulling `trybuild` in to phase 6 | If HS-P0010 declines the dependency, AC-002 has no instrument and the substitute is measurably weaker | Raise it as an explicit input to HS-P0010's architecture brief before this project's design stage; carry it in `_architecture.md` here if it is still open |
| **PS-3 may put the projection port behind `unstable-projection`.** `RUNBOOK.md:3924-3928` makes that the honest option if the two batch shapes disagree | The application-facing runner would then ship behind an off-by-default feature, changing what a `cargo add` user meets | `_design.md` must state the runner's feature gating explicitly rather than inheriting it; the PS-3 *verdict* stays HS-P0016's |
| **A `compile_fail` instrument inside `tests/` never runs.** Phase 4's artefact was decorative for exactly this reason (`RUNBOOK.md:3614-3627`) — rustdoc collects doctests from the lib target only | AC-002 is this project's entire claim, and an instrument that cannot fail is worse than none | The negative control is the discriminator and is written into AC-002; verify it fails, do not assume it |
| **Encoding lands here for the first time and `happenstance` grows `serde`.** ADR-0003 reads as a prohibition until the crate name is read carefully | Getting this backwards forbids the thing ADR-0006's split exists to allow | Stated in the constraints and in DR-04; the guard that matters is the `--no-default-features` doc build of `happenstance-core` already in `cargo xtask ci` |
| **This project is the first thing to use the frozen `EventStore`, so it is where defects appear.** ES-\* is `[FROZEN]`; a defect is a re-plan, not a patch | Absorbing one quietly would defeat the reason phase 7 sits before phase 8 | AC-012: defects are logged with clause IDs and routed to decision records; incidental bugs route to the `support` initiative per `.redkiln/config.yaml:5` |
| **`happenstance-testkit` moves on its own version.** CF-32 gives it an independent number because adding a conformance rule can turn a passing adapter's CI red | `FaultyStore` and `GappyMemoryStore` land in the testkit, not in `happenstance` | Version the testkit independently at the alpha; do not couple it to `happenstance`'s number |
| **The audience is new to idiomatic Rust** (`CLAUDE.md`, *Who you are working with*), which is DT-2's actual weight | Ceremony that is cheap for a Rust expert is the first-hour cost for this audience | DT-2 is decided in `_design.md` with the trade stated, and `## The doctest` is the artefact it is judged against |

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — vision, BR-01…BR-17, AC-01…AC-15, DoD 1…16, DT-1…DT-8
- [`../_decomposition.md`](../_decomposition.md) — the ten-project DAG, the traceability matrix, and the scope seams
- [`_intake-brief.md`](_intake-brief.md) — the approved problem, constraints, open questions, proof artefact and clause list

Knowledge base:

- [`../../../.kb/decisions/0006-bare-name-to-the-typed-layer.md`](../../../.kb/decisions/0006-bare-name-to-the-typed-layer.md) — why the bare name is the typed layer, and where the `serde` boundary went
- [`../../../.kb/decisions/0007-projection-runner-decodes.md`](../../../.kb/decisions/0007-projection-runner-decodes.md) — the runner splits at the decode boundary; its falsifier is PS-33
- [`../../../.kb/open-questions/projection-store-batch-has-no-apply-seam.md`](../../../.kb/open-questions/projection-store-batch-has-no-apply-seam.md) — consumed, not rediscovered; owned by HS-P0010
- `.kb/maps/open-questions-index.md`, `.kb/maps/decision-map.md`
- `references/adr/0007-projection-runner-decodes.md` — the long record; cite it by `file:line`

Specification, plan and code:

- `RUNBOOK.md:157` — phase 7's row and its proof artefact
- `RUNBOOK.md:244-258` — what must not be parallelised; the 7-before-8 edge
- `RUNBOOK.md:299-300` — ADR-0020 and ADR-0021 in the queue
- `RUNBOOK.md:408-413` — PS-33 evaluated at this phase's exit and nowhere else
- `RUNBOOK.md:498` — ES-32: projections poll at 0.1; this phase records the cost
- `RUNBOOK.md:524` — the `happenstance-macros` in-scope-for-0.1 criterion
- `RUNBOOK.md:3971-4102` — phase 7's goal, hazard, work list and exit criteria
- `RUNBOOK.md:4108-4162` — why the first release is `0.2.0`, and why the alpha lands here
- `spec/SPECIFICATION.md:215-221` — the clause census this project must not disturb
- `crates/happenstance/src/lib.rs:75` — the facade this project replaces
- `crates/happenstance-core/src/projection.rs` — the checkpoint pump the runner layers over
- `crates/happenstance-core/src/memory.rs` — the two tests that pin `read`'s shape; do not delete either
- `crates/happenstance-testkit/` — where `FaultyStore` and `GappyMemoryStore` land
- `examples/course-subscriptions/src/main.rs:114-173` — the twice-named event set, today
- `xtask/src/main.rs` — the gate, defined once
- `.redkiln/config.yaml:55` — `cargo xtask ci --fast`, this project's integration bar
- `.redkiln/templates/_design.md` — the repurposed design stage: this project's is the portfolio's primary API-surface review
- `standards/rust/README.md` — the router; pull one to three atoms, not the corpus
- `CLAUDE.md` — binding constraints 1–5, and the rule that matters

## Companions

- [`_intake-brief.md`](_intake-brief.md) — the approved intake for this project
- [`_decomposition.md`](_decomposition.md) — this project's grounding and its warranted briefs (architecture, ux, testing, deployment)
- [`_storymap.md`](_storymap.md) — the vertical-slice story map covering AC-001…AC-016
- [`../initiative.md`](../initiative.md) — the parent charter
