# Grounding — The typed layer, the worked example, and 0.2.0-alpha.1 (HS-P0011)

Companion to `project.md`. Cites what already exists on disk so the four warranted
briefs (architecture, ux, testing, deployment — `_decomposition.md` *Warranted
briefs*, `typed-layer-and-alpha-release` row) and the story map are written against
reality, not against the charter's paraphrase of it.

## What already exists for this project

`project.md` (stage `storymap`) is already fully authored — objective, in/out of
scope, DR-01…DR-12, AC-001…AC-016, a nine-item boundary-level DoD, dependencies,
a seven-row risk table and context anchors are all present and cited.
`_intake-brief.md` is approved (all seven intake gate boxes ticked, `automation:
HITL`). `_storymap.md` exists as the vertical-slice map — this grounding note does
not restate it. **Nothing in `crates/happenstance/src/` exists beyond the facade**:
`crates/happenstance/src/lib.rs` is 76 lines, and line 75 is exactly
`pub use happenstance_core::*;` — confirmed by reading the file, not by trusting the
citation. Every type this project's scope names (`DomainEvent`, `DecisionModel`,
`Codec`, the command loop, `Projection`, the given/when/then DSL, `FaultyStore`,
`GappyMemoryStore`) is absent from the tree today: `rg` for each of those names
across `crates/` returns nothing outside the ADR/RUNBOOK/spec prose describing them.
There is no `happenstance-macros` crate in `crates/` (nine crates exist; that is not
one of them) — AC-013's "answered either way" is a live, unresolved question, not one
this note can pre-empt. There is no `trybuild` dependency anywhere in the workspace
(`Cargo.toml`, `Cargo.lock`) — AC-002's instrument does not exist yet and its
adoption is `projection-store-freeze`'s (HS-P0010) decision to make; see *Tensions*
below.

## Accepted decision atoms that constrain this project

- **ADR-0001** (`.kb/decisions/0001-async-port-flavours.md`) — no `#[async_trait]`
  anywhere; ports are defined once without a `Send` bound and `trait_variant`
  derives the `Send` flavour. Binds this project because the typed layer's command
  loop and projection runner are new async surface built *on* `EventStore`/
  `SendEventStore` — the same discipline applies to any new trait this project adds,
  not only to the two frozen ports (DR-11, AC-015).
- **ADR-0003** (`.kb/decisions/0003-opaque-payloads.md`) — `serde` stays out of
  `happenstance-core`'s default features; payloads are opaque `Bytes` there. This
  project is the one place the prohibition *inverts*: `happenstance` (this crate,
  ADR-0006's rename) is the typed layer whose entire job is encoding, so `Codec`
  landing here with a `serde` dependency is the split's intended outcome, not a
  violation (DR-04, project.md *Constraints*). `crates/happenstance/src/lib.rs:29-33`
  already states the discriminator in its module doc: "The discriminator is
  **encoding**... Anything that knows how a payload is *shaped* belongs here."
- **ADR-0006** (`.kb/decisions/0006-bare-name-to-the-typed-layer.md`, status
  `accepted`) — gives `happenstance` (the bare name) to the typed layer and
  `happenstance-core` to the contract; states the serde boundary moves with the
  contract crate, not the name. Its `summary` frontmatter says it is "partly
  superseded by ADR-0007 — the naming decision stands... the projection-runner
  relocation is corrected." That correction is ADR-0007, immediately below, and is
  the one this project must follow, not ADR-0006's original runner placement.
- **ADR-0007** (`.kb/decisions/0007-projection-runner-decodes.md`) — the
  application-facing `Projection` trait and its runner live in `happenstance`,
  layered over the checkpoint pump that stays in `happenstance-core`
  (`crates/happenstance-core/src/projection.rs`, which is real code today — the
  `ProjectionStore`/`SendProjectionStore` port, `ProjectionId`, `Batch<'a>` GAT,
  `checkpoint`/`begin`/`commit`/`rollback` — module-doc-marked **provisional**, no
  conformance suite yet, owned by `projection-store-freeze`). ADR-0007 carries its
  own falsifier at lines 75-79 of the atom file: "falsified if... the core pump
  acquires no caller but the typed one," with the stated fallback "one runner, in
  `happenstance`, with the checkpoint invariant living one crate above the port that
  states it" — this is PS-33, and AC-007/DR-06 require it evaluated here, not
  inherited.
- **ADR-0008** (`.kb/decisions/0008-one-derivation-for-both-ports.md`) — the
  `Send`/`!Send` flavour split is derived once via `trait_variant`, for both
  `EventStore` and `ProjectionStore`. Relevant because any new port-shaped trait
  this project introduces (there is none currently scoped — the `Projection` trait
  an application *implements* is not itself store-shaped) should not reinvent the
  derivation pattern.
- **ADR-0004** (`.kb/decisions/0004-edition-and-msrv.md`) as amended by **ADR-0029**
  (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`) — MSRV 1.97.1, provisional until
  first publish (phase 12) turns it into a promise. This project's `0.2.0-alpha.1`
  is *not* that first publish for MSRV purposes per `project.md`'s own out-of-scope
  line ("The MSRV promise... → `publication-and-positioning`"), so this project may
  use let-chains and anything else 1.97.1 permits without owning the promise.
- **`.kb/decisions/README.md`** — accepted atoms are immutable; a correction is a
  new atom that supersedes, never an edit to the body. Binds ADR-0020 and ADR-0021,
  which this project authors first (AC-016, DoD 3) — if either needs revision after
  the code is written, the fix is a new atom, not a rewrite of the one just
  accepted.

## Existing code / process patterns this project must follow

- **`crates/happenstance-core/src/projection.rs`** — the checkpoint pump this
  project's `Projection` runner layers over. Confirmed by reading the file: it is
  `#[trait_variant::make(SendProjectionStore: Send)]`-derived (ADR-0008's pattern
  applied), `no_std` + `alloc` (`use alloc::boxed::Box; use alloc::string::String;`),
  and its module doc states the transactional invariant this project's runner must
  not violate — read-model write and checkpoint write as **one** transaction, via an
  adapter-owned `Batch<'a>` GAT, never independent `apply()`/`set_checkpoint()`
  calls. `ProjectionId::new` is deliberately infallible (module doc explains why:
  a validating constructor with nothing to check it against would be decorative,
  since the port itself is provisional) — this project's `Projection` trait should
  not add a second, stricter constructor beside it without a reason tied to that
  same discipline.
- **`crates/happenstance-core/src/memory.rs`** — carries the two tests CLAUDE.md
  names by function name: `send_flavour_stream_is_send_in_generic_code` (line 614)
  and `spawns_from_generic` (line 643), which together pin `EventStore::read`
  returning the stream at the top level, not behind `async fn`. Confirmed present
  at those line numbers. Any command-loop or runner code this project writes that
  reads from a generic `S: EventStore` bound must not silently regress this — the
  pattern to imitate is `spawns_from_generic`'s own bound,
  `S: crate::SendEventStore + Send + Sync + 'static` (line 666), when the runner
  needs to hold a stream across an await inside a real task.
- **`crates/happenstance/src/lib.rs`** — the facade this project replaces is
  real and short (76 lines). Its module doc already previews the exact vocabulary
  this project ships (`Codec`, `DomainEvent`, `DecisionModel`, the command loop,
  the typed projection runner) as "Planned, and specified in
  `spec/SPECIFICATION.md`" — so the doc comment structure and the doctest-gated
  README (`#![cfg_attr(doctest, doc = include_str!("../README.md"))]`, line 10, with
  its own D10 rationale comment) are the pattern to extend, not replace.
- **`examples/course-subscriptions/src/main.rs`** — confirmed the twice-named event
  set the project's own text describes: `subscribe()` (lines ~114-173) builds a
  `Query` naming `[COURSE_DEFINED, STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED]` for
  the capacity check, then folds the same three event types by hand in a `match` on
  `sequenced.event_type().as_str()` — two hand-written statements of the same event
  set, exactly the divergence hazard AC-001/ADR-0020 close. `commit()` (further
  down) is the read→decide→append→retry-on-`ConditionViolated` loop already spelled
  out per-handler, with its own doc comment noting "this is the second half of every
  DCB command handler, and it is identical every time — which is exactly why it
  belongs in the typed layer." This is the code this project's command loop
  replaces, and the rewrite's job is to make exactly this duplication a compile
  error.
- **`crates/happenstance-testkit/src/`** (`concurrency.rs`, `contract.rs`,
  `fixtures.rs`, `model.rs`, `registry.rs`, `suite.rs`) — where `FaultyStore<S>` and
  `GappyMemoryStore` land (AC-009). Confirmed neither exists yet by name anywhere
  under `crates/happenstance-testkit/src/`. `fixtures.rs` already holds
  `MemoryFixture`, the reference `Fixture` implementation CLAUDE.md names, which is
  the pattern a new fixture-shaped type should follow, and the crate's own
  `event_store_conformance!` macro (CLAUDE.md, *The rule that matters*) is a
  precedent for how a testkit type advertises capability via associated
  `Capability` constants rather than by silently vanishing when declined.
- **`spec/SPECIFICATION.md`** clause text, read directly rather than paraphrased:
  - **ES-32** (line 4021) — "`EventStore` MUST NOT grow a tail, subscribe or
    notify method at 0.1. Consumers poll," `[PROVISIONAL — falsified if the fan-out
    runner of E2E-32 cannot hold N views within their staleness budget at a
    measured poll interval on a real deployment]`. This is AC-010/DR-08's source:
    the workspace has no benchmark harness, so the number this project records is
    the first one to exist.
  - **PS-18** (line 5200) — reset refusal (`ResetError::Refused`), `[PROVISIONAL —
    falsified if no adapter ever implements protection... Evaluated at the exit of
    the projection-port phase by asking whether the SQLite adapter implemented
    it]`. Its evaluation phase is the *projection-port* phase
    (`projection-store-freeze`, HS-P0010), not this one — AC-008 groups it with
    PS-27/PS-30 as "settled by counting," but the clause text itself assigns PS-18's
    evaluation point upstream of this project. Flagged under *Tensions* below.
  - **PS-27** (line 5411) — skip-and-record atomicity, `[PROVISIONAL... Evaluated
    at the exit of the typed-layer phase against the Kestrel Motor shred case]` —
    this one is explicitly this project's to evaluate.
  - **PS-30** (line 5462) — fan-out runner panic rollback, `[PROVISIONAL —
    falsified if the fan-out runner is not built... Owned by the typed-layer
    phase]` — also explicitly this project's.
  - **PS-33** (line 5529) — ADR-0007's falsifier, `[DEFERRED — the experiment is
    the typed layer itself; the question is answered by counting callers. Owned by
    the typed-layer phase]`, rule "none; it is a phase gate, not an adapter
    obligation."
  - The clause-status table (lines 8614, 8647, 8656, 8659, 8662) confirms ES-32
    `PROVISIONAL` with no rule, PS-18/PS-27/PS-30 `PROVISIONAL` each with a `†`
    rule, PS-33 `DEFERRED` with no rule — consistent with the prose above.

## Tensions / open items to flag, not silently resolve

- **PS-18's own clause text assigns its evaluation to "the exit of the
  projection-port phase," not the typed-layer phase**, while `project.md`'s DR-06
  and AC-008 group PS-18 with PS-27/PS-30 as clauses this project settles "by
  counting." `RUNBOOK.md:4070-4077` is `project.md`'s own citation for the grouping
  and should be read as the reconciling source before the testing/architecture
  brief writes a story for PS-18 outright — if `projection-store-freeze`
  (HS-P0010) has already counted SQLite adapter reset protection by the time this
  project starts, AC-008's PS-18 leg may already be discharged upstream, and this
  project's job would be to *record* that verdict, not re-derive it. Neither
  `sqlite-durable-store` nor `projection-store-freeze` has shipped a SQLite
  adapter yet (both sibling projects sit at `stage: storymap`, same as this one),
  so PS-18 cannot actually be settled by counting real callers until an adapter
  exists — worth stating explicitly in the architecture brief rather than assuming
  the count is available at design time.
- **`trybuild` is not yet a workspace dependency, and its adoption is
  `projection-store-freeze`'s decision, not this project's**, per `project.md`'s
  own risk table ("`trybuild`'s adoption is HS-P0010's decision, and this
  project's proof artefact needs it... If HS-P0010 declines the dependency, AC-002
  has no instrument and the substitute is measurably weaker"). Confirmed by
  searching the workspace: no `trybuild` reference anywhere in `Cargo.toml` or
  `Cargo.lock`, and `projection-store-freeze/project.md` is itself still at
  `stage: storymap` — the dependency has not been resolved one way or the other as
  of this grounding pass. The architecture brief should carry this as an explicit
  input to request from HS-P0010 rather than assume `trybuild` will simply be
  present when this project reaches implementation.
- **This project depends on `HS-P0010 projection-store-freeze`, which is not yet
  past planning** (`stage: storymap`, same stage as this project). The frozen-port
  guarantee AC-006/DR-05 rests on — `ProjectionStore`'s batch shape, write seam and
  capability-declension policy settled — does not exist yet; today
  `crates/happenstance-core/src/projection.rs`'s own module doc still says "this
  port is **not yet frozen**." Nothing in that is a defect in this project's plan
  (the dependency edge is stated correctly in `project.md`'s *Dependencies*
  section, `RUNBOOK.md`'s 6→7 edge), but any brief that describes the
  `Projection` trait's shape in concrete terms should mark it provisional on
  HS-P0010's freeze landing first.
- **No tension found against any Accepted decision atom's substance.** ADR-0001,
  ADR-0003 (as scoped by ADR-0006), ADR-0006, ADR-0007, ADR-0008, ADR-0004/0029 are
  all correctly read and correctly cited in `project.md` already; this note's
  additions are line-level confirmations and the two scheduling caveats above, not
  corrections.

## Anchors for the briefs

- `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md`
  — AC-001…AC-016, DR-01…DR-12, DoD 1-9, risk table (already authored; briefs must
  trace to these, not restate the charter).
- `.bklg/from-contract-to-published-library/_decomposition.md` — *Warranted briefs*
  (this project: architecture + ux + testing + deployment, all four), the `ux`
  rationale naming this project "the primary `ux` brief in the portfolio," and the
  `deployment` rationale ("the two real crates.io release events").
- `.kb/decisions/0001-async-port-flavours.md`, `0003-opaque-payloads.md`,
  `0004-edition-and-msrv.md`, `0006-bare-name-to-the-typed-layer.md`,
  `0007-projection-runner-decodes.md`, `0008-one-derivation-for-both-ports.md`,
  `0029-msrv-raised-to-1-97-1.md` — the seven accepted atoms directly binding on
  this project's conduct.
- `references/adr/0007-projection-runner-decodes.md` — the long record for PS-33's
  falsifier; cite by `file:line` per `.kb/decisions/README.md`.
- `crates/happenstance-core/src/projection.rs` — the checkpoint pump and its
  transactional invariant, read in full above.
- `crates/happenstance-core/src/memory.rs:614,643` — the two pinned tests for
  `EventStore::read`'s shape.
- `crates/happenstance/src/lib.rs` — the current 76-line facade; its module doc's
  "Planned" list is the vocabulary this project ships.
- `examples/course-subscriptions/src/main.rs` — the twice-named event set
  (`subscribe`) and the per-handler `commit()` loop this project's rewrite
  replaces.
- `spec/SPECIFICATION.md:4021` (ES-32), `:5200` (PS-18), `:5411` (PS-27), `:5462`
  (PS-30), `:5529` (PS-33) — clause text read directly, not paraphrased.
- `RUNBOOK.md:299-300`, `:408-413`, `:498`, `:524`, `:3971-4102`, `:4108-4162` —
  already cited in `project.md`'s context anchors; confirmed as the phase-7/alpha
  narrative source.
- `.kb/decisions/README.md` — immutability rule ADR-0020/0021 are written under.
