# Grounding — Freeze `ProjectionStore` behind a suite that can fail (HS-P0010)

Companion file. Not a redkiln item; no frontmatter. Written for the briefs
(architecture / ux / testing / deployment) that follow, so they cite reality
rather than restate the intake brief.

## 1. What exists today, precisely

`crates/happenstance-core/src/projection.rs` (140 lines) is the whole port:

- `ProjectionId(Box<str>)` — infallible constructor, deliberately unlike
  `EventType`/`Tag` (`projection.rs:44-64`), because a validating constructor
  on a provisional port would be exactly the decorative-rule pattern
  CLAUDE.md's corollary forbids. This is documented as an **open question**,
  not a decision — `.kb/open-questions/projection-id-is-unvalidated.md`.
- `#[trait_variant::make(SendProjectionStore: Send)] trait ProjectionStore`
  with `checkpoint`, `begin`, `commit`, `rollback` — four methods, no `apply`.
- `type Batch<'a> where Self: 'a` (`projection.rs:97-99`) — a **borrowing**
  GAT. This is the fact the project's central work item (RUNBOOK phase 6,
  first bullet) removes: `type Batch;` with no lifetime parameter.
- The module doc block (`projection.rs:1-30`) states its own provisional
  status and the one invariant the port exists to hold: read-model write and
  checkpoint write must be one transaction. Nothing enforces it — there is no
  `projection_store_conformance!` macro, no rule registry, no mutant. This is
  the gap AC-001 – AC-005 close.

There is **no `apply` seam**. Generic code holding a `P::Batch` can pass it to
`commit` or `rollback` and nothing else — confirmed by
`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`, which traces
the gap to ADR-0007 (`.kb/decisions/0007-projection-runner-decodes.md`) and
names it as blocking exactly the work this project must do: "the port has no
conformance suite... It gets frozen when the first real projection adapter
can be built against it." RUNBOOK.md:3882-3889 assigns closing this seam to
this phase, driven by the suite's own need (`write_probe(&mut Batch)` /
`read_probe(&Store)`) rather than by the runner's need — the runner itself is
`typed-layer-and-alpha-release`'s (HS-P0011), out of scope here per the
intake brief's non-goals.

## 2. Three existing skeletons already implement the pre-freeze shape

All three compile against today's `type Batch<'a>` and will need exactly the
lifetime-removal edit AC-013 demands — nothing else, per RUNBOOK.md:3958-3960
("same underlying `Batch` type, same error type, same bodies"):

- `crates/happenstance-postgres/src/projection_store.rs:100` —
  `type Batch<'a> = sqlx::Transaction<'a, Postgres>` (module doc at line 34
  shows the target shape as `'static`, i.e. **owned**, once the lifetime is
  dropped). This crate's own doc comment (`projection_store.rs:7`) already
  narrates the port's `where Self: 'a` bound and is the crate AC-013 names as
  one of the two skeletons that must compile unchanged apart from the
  lifetime.
- `crates/happenstance-ladybug/src/live_handle.rs:177` and
  `crates/happenstance-ladybug/src/projection_store.rs:265` — **two**
  `SendProjectionStore` impls in one crate, deliberately: `LadybugProjectionStore`
  (owned `GraphWriteSet`) and `LiveHandleProjectionStore` (borrowed
  `GraphWriteHandle<'a>`), so the crate itself already demonstrates both ends
  of the batch-shape axis RUNBOOK.md:3876-3881 says the owned-vs-borrowed
  question is not fully closed by (`live_handle.rs:9-89` narrates the
  compiler transcript directly, `DefId::expect_local` failure and all). This
  is the other skeleton AC-013 names.
- `crates/happenstance-sqlite/src/projection_store.rs:225` —
  `type Batch<'a> = SqliteBatch` (already **owned**, an
  `(SQL, Vec<rusqlite::types::Value>)` buffer per `references/adapter-shapes.md:43`).
  **Not named in AC-013** — `sqlite-durable-store` (HS-P0012) owns shipping
  this adapter as a product; this project's non-goal list excludes it
  explicitly. Still useful evidence: it is a *third* structurally distinct
  batch shape already in the tree, beyond the two AC-004 requires named in
  the proof artefact.

`references/adapter-shapes.md` carries the compiler transcripts ADR-0017/18/19
must quote per RUNBOOK.md:3952-3953 ("quoting the compiler errors... rather
than asserting the port survives"): the two independent `SendProjectionStore`
rejections of `type Batch<'a> = rusqlite::Transaction<'a>`
(`adapter-shapes.md:61`), the `error[E0195]` trap from writing the concrete
type instead of `Self::Batch<'_>` (`adapter-shapes.md:186-191` — this is the
PS-34/PS-36 trap AC-012 must document), and the minimised `DefId::expect_local`
ICE transcript around line 315-347.

## 3. The pattern to mirror: `happenstance-testkit`'s event-store suite

This project's own proof mechanism (AC-001 – AC-003) is not being invented —
it is the **second instance** of a pattern the event-store suite already
runs, and the architecture/testing briefs should cite the existing files by
name rather than redescribe the mechanism:

- **One enumeration.** `for_each_event_store_rule!`
  (`crates/happenstance-testkit/src/registry.rs:94-220`) is the single place
  the rule set is written; `no_orphan_rules`
  (`registry.rs:410-436`) is the meta-test AC-001 explicitly asks to be
  mirrored ("mirrors `for_each_event_store_rule!` / `no_orphan_rules`").
  A `for_each_projection_store_rule!` + its own `no_orphan_rules`-shaped
  meta-test is the direct analogue.
- **Fixture contract, not a bare closure.** `Fixture` trait
  (`crates/happenstance-testkit/src/contract.rs:120-353`) with `Capability`
  associated consts (`SECOND_HANDLE` a MUST, `REOPEN` a `SHOULD`,
  `MID_BATCH_FAULT` defaulted-declined) is the shape AC-005 ("a rule whose
  projection capability is declined is still emitted as a test... asserted on
  `RuleOutcome` values") already exists for the event-store port. `RuleOutcome`
  (`contract.rs:458-537`, `#[must_use]`, no `Failed` variant — a failing rule
  panics) is reusable machinery, not a new invention; whether the projection
  suite reuses this exact enum or needs its own capability set (batch-write
  probing needs a capability shape the event-store fixture never needed) is a
  design question for `_design.md`, but the *reporting* discipline —
  distinguishable skip vs pass, stated reason, no silent `#[cfg]`-out — is
  settled precedent, not open.
- **The mutant registry and its exactness meta-tests.**
  `crates/happenstance-testkit/tests/mutation_coverage.rs` (3,527 lines) is
  the model for AC-002/AC-003. Its `REGISTRY: &[Declared]`
  (`mutation_coverage.rs:324` onward) pairs each mutant with the **exact**
  set of rules it fails (`fails`), a `provenance` string naming the real
  adapter mistake it models (never empty — AC-003's requirement echoes this
  field's own doc comment at `mutation_coverage.rs:150-151`), and an optional
  per-rule `expect` pin for the exact assertion substring. Three meta-tests
  are the ones AC-001/AC-002/AC-003 need projection-port siblings of:
  `every_rule_has_a_mutant` (`mutation_coverage.rs:2733-2749`, CF-1 — no rule
  may exist without a failing store), `mutant_registry_is_exhaustive`
  (`mutation_coverage.rs:2753-2860`, CF-2 — registry and enumeration agree,
  no typos, no duplicate names, at least one conformant variant exists), and
  `mutants_fail_exactly_their_declared_rules` (`mutation_coverage.rs:2888` on,
  CF-3 — a mutant fails **exactly** its declared rules, no more; "exactness
  is what makes the registry a map from rules to the bugs they catch"). This
  last one is `CheckpointOnlyStore`'s bar in AC-002: it is not enough that it
  fails *some* rule, it must fail its declared rule and pass every rule it
  did not declare.
- **Never quote a pass rate.** ADR-0010 (`.kb/decisions/0010-the-suite-must-prove-itself.md`)
  is the accepted decision AC-003 cites directly, and its own text is blunt:
  "No pass rate is ever quoted over the mutant set... the denominator is an
  author's choice, so a fraction says how representative the author was
  while reading as though it said how good the suite is"
  (`mutation_coverage.rs:26-31` restates it). The projection registry's own
  doc comment must carry the same warning, matching
  `mutation_coverage.rs:203-206`'s wording.
- **Emitters stay a parameter.** `__emit_tokio` / `__emit_blocking` /
  `__emit_wasm` (`registry.rs:222-295`) are what makes AC-016 (wasm32 in the
  same `cargo xtask ci` run, not a separately maintained subset) free rather
  than novel: the event-store suite already proves the pattern works on all
  three runtimes with one rule enumeration. `crates/happenstance-testkit/tests/memory_conformance_wasm.rs`
  is the existing host for that pattern and the one to point a
  `projection_store_conformance!` wasm harness at.

## 4. Open questions this project is the forcing function for

Four atoms in `.kb/open-questions/` name RUNBOOK phase 6 (this project) as
the thing that forces them, and the intake brief's DT-3/DT-8 are two of the
four re-surfaced at the story-planning grain:

- **`cf-40-fixture-limits-ownership.md`** — DT-3's cited source. ADR-0015
  mints CF-40 (a `Fixture` MUST be able to state `MAX_EVENT_DATA_LEN` /
  `MAX_TAGS_PER_EVENT` / `MAX_EVENTS_PER_BATCH`) and then contradicts itself
  about whether it or ADR-0012 owns the fixture-capability surface as a
  whole. This is an **event-store** open question, not projection-specific —
  but AC-006 requires `_design.md` to resolve DT-3 by citing this atom as
  authoritative rather than re-litigating it, which is squarely in scope
  because the projection fixture will need its own capability-reporting
  policy (declined-capability skip vs a stated numeric limit) and inherits
  whichever precedent this atom settles.
- **`projection-store-batch-has-no-apply-seam.md`** — as above (§1). Names
  ADR-0017 by number as its own resolution ("carries ADR-0017 for what a
  projection batch owns and what vocabulary writes into it").
- **`ps-1-states-no-progress-obligation.md`** and
  **`ps-19-scope-narrower-than-its-rule.md`** — both `[FROZEN]` clauses (PS-1,
  PS-19) whose stated MUST is narrower than the rule the specification's
  §4.11 table assigns it. Both atoms name phase 6 (this project) as owner and
  both flag the same thing explicitly: **check the other 35 PS clauses for
  the same shape before scoping the fix**, because the pairing may be
  systematic rather than two isolated defects. This is a concrete, named risk
  for the architecture brief to carry forward rather than rediscover.
- **`projection-id-is-unvalidated.md`** — related but not phase-6-owned per
  its own text; flagged here only so the design brief does not accidentally
  try to fix `ProjectionId::new`'s infallibility as a side effect of touching
  the port it lives beside.

## 5. RUNBOOK.md phase 6 — the plan of record for this project

`RUNBOOK.md:3848-3969` is the section, and it is more prescriptive than the
intake brief alone conveys. Load-bearing details the briefs should not
silently drop:

- **Decisions it settles**, by number: ADR-0017 (the batch shape),
  ADR-0018 (reset), ADR-0019 (failure policy) — RUNBOOK.md:3858. None of the
  three exists yet under `.kb/decisions/` (confirmed: highest numbered atom
  present is 0016, plus 0029). AC-008 is therefore asking this project to
  *write* three new ADRs, not merely reference existing ones — the intake
  brief's "Settled by ADR-0017/18/19" language names decisions that do not
  exist until this project's implementation stage produces them.
- **`CheckpointOnlyStore` is explicitly PS-2's whole bar**
  (RUNBOOK.md:3890-3892, restated at 3934-3935 as *the* proof artefact
  alongside "two implementations at opposite ends of the batch-shape axis").
  AC-002's wording ("fails at least one named rule, and passes every rule it
  does not declare") is the exactness requirement from §3 above, applied to
  this one mutant.
- **A stated `Drop` contract for a batch** (PS-7, RUNBOOK.md:3893-3897): "a
  reviewer's probe found a dropped batch permanently losing the connection,
  after which the store returned `Busy` forever — so the rule is not 'rolls
  back', it is 'and the store remains usable'." This is AC-010's exact
  provenance; cite it rather than restate it more weakly.
- **`reset`'s substitute is explicitly named and explicitly wrong**
  (RUNBOOK.md:3904-3907, AC-011): `commit(empty, id, FIRST)` "silently skips
  event 1 — the substitute all six scenarios reached for and all six got
  wrong." That "all six" is worth carrying into the testing brief as the
  reason this needs a *named* rejecting rule, not just documentation.
- **`MemoryProjectionStore` is ~50 lines, behind the `memory` feature, and
  exists to fix a cold-start problem** (RUNBOOK.md:3898-3903, AC-012):
  implementing the port today fails with `error[E0195]` unless the
  implementer spells the parameter `Self::Batch<'_>` exactly, "nothing says
  so, and there is nothing to copy" — confirmed independently at
  `references/adapter-shapes.md:186-191`.
- **The exit criteria are stated precisely enough to quote verbatim**
  (RUNBOOK.md:3950-3960) and match AC-006/AC-008/AC-013/AC-014 closely:
  ADR-0017/18/19 written *first*; `CheckpointOnlyStore` fails ≥1 named rule;
  two implementations pass; `projection.rs` drops "provisional" or moves
  behind `unstable-projection` with a stated reason; Ladybug and Postgres
  skeletons compile with **no change other than the lifetime removal** — "same
  underlying `Batch` type, same error type, same bodies."
- **"Without amendment" is explicitly the wrong bar**
  (RUNBOOK.md:3942-3948): dropping the GAT necessarily changes every skeleton
  that spells `type Batch<'a>`, so an exit criterion demanding zero change is
  unsatisfiable by the phase's own central decision. AC-013's "no change
  other than the removal of the batch's lifetime parameter" phrasing already
  reflects this corrected bar — the architecture brief should not weaken it
  back to "unchanged."
- **The DoD-7 batch-shape-count tension the intake brief flags is real and
  is RUNBOOK's own stated risk, not invented for this project**: the
  RUNBOOK's proof artefact for phase 6 is explicitly "the phase-2 rusqlite
  skeleton fleshed out far enough to commit a real transaction, and
  `MemoryProjectionStore`" (RUNBOOK.md:3936-3938) — i.e. it expects the
  *second* unlike shape to come from `happenstance-sqlite`, which per this
  initiative's own decomposition (`_decomposition.md:147`) is a **downstream**
  project (`sqlite-durable-store`, rank 2, depends on this one). AC-004's
  requirement that "the second shape's provenance is a decision recorded in
  the architecture brief rather than an assumption" is asking `_design.md` to
  resolve exactly this inversion: either build the second shape inside the
  testkit itself (the `MemoryFixture`-vs-`MemoryEventStore` precedent already
  in the tree, per `fixtures.rs:219-292`) or accept the dependency-order risk
  RUNBOOK's own phase-6 proof artefact currently assumes away.
- **PS-3's evidence, not its verdict, is owed here** (RUNBOOK.md:3924-3928,
  AC-015): ship frozen, or behind `unstable-projection` per the
  `tokio_unstable` idiom, "if the two batch shapes disagree." This project
  writes the finding; `publication-and-positioning` (HS-P0016) makes the
  call, per both the intake brief's non-goals and the decomposition's
  traceability matrix (`_decomposition.md:60-61`, BR-04's `Oᶠ`/`Oʳ` split
  between this project and `ladybug-projection-store`).

## 6. Tensions to flag explicitly in the briefs

- **RUNBOOK's stated phase-6 proof artefact names a downstream project's
  deliverable as one of its two batch shapes.** Already covered in §5 above;
  do not let this resolve itself silently — AC-004/AC-006 require the
  decision recorded, and `_design.md` is where it must land.
- **PS-1 and PS-19 are `[FROZEN]` clauses with an admitted gap between their
  MUST text and the rule table.** Widening either is "an ADR's and not an
  edit's" per both open-question atoms and per `kb-playbook-repair-frozen-clause-001`
  (cited by both). If the conformance-rule work surfaces the same shape
  elsewhere in PS-1 – PS-37 (both atoms explicitly ask the next owner to
  check), that is new ADR scope, not silent repair — consistent with
  CLAUDE.md's "changing a `[FROZEN]` clause requires a new ADR, not an edit."
- **`ProjectionId`'s infallible constructor is deliberately inconsistent
  with `EventType`/`Tag` and is explicitly *not* this project's decision to
  fix** (`projection.rs:47-61`: "There is no decision behind it... Adding a
  fallible `parse` beside this constructor would be worse than either
  choice"). The port freeze should not silently harden this API surface as a
  side effect of touching everything else in the module.
- **The GAT-across-a-suspension-point finding (ADR-0008) constrains which of
  the apply-seam's three candidate shapes can compile**, per
  `projection-store-batch-has-no-apply-seam.md`'s ordered sub-question 1.
  ADR-0017 needs to weigh this, not just the borrowed-vs-owned batch
  question RUNBOOK.md:3861-3868 documents.
- **`happenstance-ladybug` already carries two `SendProjectionStore` impls**
  demonstrating both a borrowed and an owned batch in one crate
  (`live_handle.rs` vs `projection_store.rs`). RUNBOOK.md:3876-3881 is
  explicit that dropping the lifetime does **not** make the foreign-batch
  hazard (handing one store's batch to another's `commit`) unrepresentable —
  "only a generative brand rejects it, and PS-15 records that as provisional
  rather than pretending the owned shape closed it." AC-009/AC-010 should not
  be read as closing that hazard; PS-15 stays provisional per RUNBOOK's own
  words.

## 7. Anchors (repo-relative, for citation in briefs)

- `crates/happenstance-core/src/projection.rs` — the port as it stands today.
- `crates/happenstance-testkit/src/registry.rs` — `for_each_event_store_rule!`,
  `no_orphan_rules`, the three emitters (`__emit_tokio`/`__emit_blocking`/`__emit_wasm`).
- `crates/happenstance-testkit/src/contract.rs` — `Fixture`, `Capability`, `RuleOutcome`.
- `crates/happenstance-testkit/src/fixtures.rs` — `MemoryFixture`/`MemoryHandle`,
  the reference-fixture pattern to mirror for `MemoryProjectionStore` (AC-012).
- `crates/happenstance-testkit/tests/mutation_coverage.rs` — `REGISTRY`,
  `every_rule_has_a_mutant`, `mutant_registry_is_exhaustive`,
  `mutants_fail_exactly_their_declared_rules` — the pattern AC-001/002/003 mirror.
- `crates/happenstance-postgres/src/projection_store.rs` and
  `crates/happenstance-ladybug/src/live_handle.rs` +
  `crates/happenstance-ladybug/src/projection_store.rs` — the two skeletons
  AC-013 names.
- `crates/happenstance-sqlite/src/projection_store.rs` — a third existing
  batch shape, out of this project's scope but relevant context for AC-004.
- `references/adapter-shapes.md:61,186-191,297,311-347` — the compiler
  transcripts ADR-0017/18/19 must quote.
- `RUNBOOK.md:3848-3969` — phase 6 in full: goal, decisions, work items, exit
  criteria, proof artefact.
- `.kb/decisions/0010-the-suite-must-prove-itself.md` (ADR-0010) — the
  no-pass-rate rule, AC-003.
- `.kb/decisions/0007-projection-runner-decodes.md` (ADR-0007) — the
  apply-seam gap's origin, and the Context correction owed at PS-32.
- `.kb/decisions/0008-one-derivation-for-both-ports.md` (ADR-0008) — the
  GAT-across-suspension-point finding constraining apply-seam shape.
- `.kb/decisions/0001-async-port-flavours.md` (ADR-0001) — `!Send`/`Send`
  two-flavour design the projection port already follows via
  `#[trait_variant::make(SendProjectionStore: Send)]`.
- `.kb/open-questions/cf-40-fixture-limits-ownership.md` — DT-3's cited source.
- `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` — names
  ADR-0017 as its own resolution.
- `.kb/open-questions/ps-1-states-no-progress-obligation.md` and
  `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` — both name
  phase 6 as owner, both flag the same-shape-elsewhere risk.
- `.kb/open-questions/projection-id-is-unvalidated.md` — adjacent, not owned
  here; flagged so it is not silently touched.
- `.bklg/from-contract-to-published-library/_decomposition.md:41,144,147` —
  this project's scope seams and DAG position (root, blocks
  typed-layer/postgres-and-neon/ladybug).
