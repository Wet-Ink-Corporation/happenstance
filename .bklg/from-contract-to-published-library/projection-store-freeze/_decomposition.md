# Briefs — Freeze `ProjectionStore` behind a suite that can fail (HS-P0010)

Companion file. Not a redkiln item; no frontmatter. Each brief below follows
`.redkiln/templates/briefs/brief.md` (Intent / Acceptance Criteria / Notes)
inside its own `##` section. Grounding for all of them is `_grounding.md`; the
project charter and its AC-001 – AC-016 are `project.md`.

---

## UX brief

### Intent

**Scope this brief covers.** The surface an adapter author meets while making a
projection store correct, and only that: the port they write against —
`ProjectionStore`, its `Batch` vocabulary, `reset`, and the `Checkpoint` /
`Authority` / `CommitError` / `ResetError` types the target shape adds
(`spec/SPECIFICATION.md:4642-4719`) — plus **what the conformance runner prints
back at them**. The initiative's decomposition warrants `ux` here for exactly
those two things: *"`ProjectionStore`, its `Batch` vocabulary and `reset` are
public API a caller writes against, and DT-3 is literally what does the suite's
output say to a human"*
(`.bklg/from-contract-to-published-library/_decomposition.md:270-272`).

**There is no screen.** `userFacing` is false for this initiative
(`.bklg/from-contract-to-published-library/_decomposition.md:266`) and
`.redkiln/config.yaml` declares no `design.capture`, so the perceptual review is a
declared skip rather than a silent pass (`CLAUDE.md`, *Where the work lives*). The
interface is a **type surface plus one text surface**, and every criterion below is
written in the medium this project actually has:

- **The type surface** — `crates/happenstance-core/src/projection.rs`, the port and
  its module doc, plus `ProjectionProbe` behind the new `conformance` feature
  (Architecture brief AC-A02; `spec/SPECIFICATION.md:4998-5013`). What the shape
  costs a caller is a first-class question here because five impls already exist
  and every one of them is rewritten by this project (`_grounding.md` §2).
- **The text surface** — the lines a conformance run emits. This is not a
  metaphor: the initiative's AC-04 (*"the adapter author is told when they are
  finished"*) and AC-05 (*"told, with a reason, where a guarantee does not apply
  to them"* — a declined capability *"reports the fixture's stated reason and
  still appears in the output; it never vanishes from the binary"*) are owned by
  this project (`../initiative.md:317-322`, BR-13 at `:296`) and are literally
  about emitted text.

**Who this is for.** Almost entirely one persona, and this brief does not blur it —
the distillation's own risk list says conflating the application author with the
adapter author *"would hide a real difference"*, because their trust is earned by
different artefacts
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:356-360`).
As the charter instructs, this cites the initiative's distillation and carries its
qualification with it: **none of the four personas has been directly observed; every
citation is secondary evidence** (`…/personas-and-journeys.md:361-363`), and
promotion into `.kb/product/` is `closeout-and-durable-audience`'s (HS-P0019).

| Persona | What they are trying to do here | What they are afraid of |
| --- | --- | --- |
| **P2 — the adapter author** (`…/personas-and-journeys.md:114-153`) | Get *"an executable definition of 'correct' they can run against their own storage system, rather than a prose specification they have to interpret"* (`:125-127`) | That the port quietly assumed something their storage cannot provide, discovered late — the objection Marten's author acted on when he rejected storage-agnosticism outright (`:144-152`) |
| **P3 — the local-first / edge developer** (`…/personas-and-journeys.md:182-189`) | Keep the constrained single-threaded target genuinely covered, not nominally | Tooling that quietly stops covering it. AC-016 is their rule, and the text surface is weakest exactly there (AC-U11) |

P1 (the application author) and P4 (the evaluator) are **out of frame here by
scope, not by oversight**: the application-facing `Projection` trait and the runner
are `typed-layer-and-alpha-release`'s (HS-P0011) and the crate landing page is
`publication-and-positioning`'s (HS-P0016) (`project.md`, *Out of scope*). Do not
decide either surface through this port's naming.

**The two decisions this brief hands to `_design.md`, unresolved on purpose.**
DT-3 — *how a consumer learns a guarantee does not apply to their implementation*:
the suite's own output, per-adapter documentation, or both with one authoritative
(`../initiative.md:422`) — and DT-8 — *whose adapter-author bar the suite holds*
(`:427`), a question the distillation raised first and left open
(`…/personas-and-journeys.md:378-381`). AC-006 and AC-007 place both in
`_design.md`, and the Testing brief above already records DT-3 as **not
test-provable**, checked by the design-stage review (DoD 7). This brief does not
pre-empt either; it states the bar each answer is judged against — AC-U01 and
AC-U02.

### Acceptance Criteria

Each is observable and names its instrument. They are UX-grain refinements of
`project.md`'s AC-001 – AC-016 and DR-02 / DR-03 / DR-07 / DR-08, and add no scope.

#### The bar the two deferred decisions are judged against

- **AC-U01 — DT-3's answer accounts for every reason-writer already in the tree.**
  Whichever source `_design.md` names authoritative, the resolution enumerates the
  three that exist today and says which defers to which: the **fixture-written**
  reason (`Capability::declined`, `crates/happenstance-testkit/src/contract.rs:378-419`),
  the **testkit-written** reason for a fact no adapter should paraphrase
  (`NO_CEILING_REASON`, `contract.rs:444-456`, whose own doc says why it is *"the
  one place the skip machinery here differs"*), and per-adapter prose. A resolution
  that names one authoritative source without disposing of the other two has minted
  the second divergent declension policy `project.md`'s risk list forbids —
  *"one policy, or a stated reason for two"* — and CF-40's atom is cited as
  authoritative rather than re-litigated (AC-006;
  `.kb/open-questions/cf-40-fixture-limits-ownership.md`). Instrument: the
  design-stage review (DoD 7); nothing here is test-provable, per the Testing brief.
- **AC-U02 — DT-8's answer states its cost to an author this repository did not
  write.** If the bar is held for an outside author, the extension surface is the
  documented pair `projection_store_conformance!` + `ProjectionProbe`, and its cost
  must remain **one feature flag on a dependency the adapter already has, and no
  new edge in the graph** — the coherence argument the specification already spells
  out at `spec/SPECIFICATION.md:5016-5025`, and the reason AC-A02 puts the probe in
  the contract crate. If the narrower arm is taken, the recorded scope says so
  where an outsider meets it — the testkit's own crate doc — and not only in
  `_design.md`, because a bar that is internal-only in fact and unqualified in
  public is the failure DT-8 names (*"a suite's own permissiveness becomes publicly
  scrutinised the moment outsiders make claims with it"*, `../initiative.md:427`).
  Instrument: the design-stage review, plus AC-007's conditional fixture.

#### The type surface — what the shape costs a caller

- **AC-U03 — Every state the port can report is one a caller can name, and the ones
  it cannot produce are unrepresentable.** `Checkpoint` ships as the three-variant
  enum, not `(Option<SequencePosition>, bool)`, for the reason the specification
  already gives — the tuple can spell `(None, true)`, *"authoritative, never run —
  which means nothing"* (`spec/SPECIFICATION.md:4643-4658`) — and the port's rustdoc
  carries that sentence where the reader meets `checkpoint`, once (RS-70-5,
  `standards/rust/70-rustdoc-obligations.md:243`). Instrument: the `_design.md`
  signature block and its doctest (DoD 7).
- **AC-U04 — Two error enums stay two, so a caller matches only arms their call can
  produce.** `CommitError` and `ResetError` are not merged into one
  `ProjectionError<E>`: `commit`'s caller must not have to consider `Refused`,
  which `commit` cannot produce, and `#[non_exhaustive]` already forces a wildcard
  arm without also forcing dead ones (`spec/SPECIFICATION.md:4722-4727`). Each
  carries the adapter's error as a type parameter, never as a `String`
  (`standards/rust/30-error-taxonomy.md:15,71`). Instrument: `cargo check` at every
  call site, and the design review for the split itself.
- **AC-U05 — Where the shape is unusual, the doc comment names the alternative that
  lost, at the call site.** Two are load-bearing and both already have their answer
  written down: `begin` is neither `async` nor fallible because *"opening a buffer
  cannot fail, and an adapter that needs a round trip takes it at `commit`"*
  (`spec/SPECIFICATION.md:4692-4694`) — a round trip Neon's one-shot transport
  cannot afford (Architecture brief Note 9) — and `reset` takes the caller's own
  deletes because *"the port has no idea what the read model is"*
  (`spec/SPECIFICATION.md:5165-5170`). Each appears once, in the item's rustdoc,
  not only in an ADR (RS-70-5). This is the audience-specific requirement: the
  reader is fluent in the domain and, per `CLAUDE.md`'s *Who you are working with*,
  new to the idiom.
- **AC-U06 — The `E0195` trap is met with something to copy.** The implementer hits
  *"lifetime parameters or bounds on method `commit` do not match the trait
  declaration"* while writing an impl, and today *nothing says so and there is
  nothing to copy* (`references/adapter-shapes.md:186-194`;
  `_grounding.md` §5). AC-012's documentation therefore lands on the port's own
  rustdoc and `MemoryProjectionStore`'s doctest — the two pages an implementer
  already has open — and the doctest compiles under `cargo test --doc` (Testing
  brief, AC-012 row). A trap documented only in ADR-0017 is documented where the
  person who needs it is not.
- **AC-U07 — A refusal tells the caller *that*, and the design says out loud
  whether it tells them *what*.** `ResetError::Refused` carries no payload
  (`spec/SPECIFICATION.md:4676-4682`), so an operator learns their reset was
  declined and not which policy declined it — while the clause's own reasoning is
  that *"the port supplies the mechanism; the domain decides what to protect"*
  (`:5210-5216`). `_design.md` either gives the variant the store's stated reason or
  records the decision to leave it bare with the reason a caller is expected to
  find instead. Silence here is the failure, not a passing default. Instrument: the
  design review; the behaviour itself is AC-011's rule
  (`refused_reset_changes_nothing`, `:5207-5208`).

#### The text surface — what a run says to a human

- **AC-U08 — One skip vocabulary, one line shape.** A declined projection capability
  is reported through `RuleOutcome`'s existing rendering —
  ``SKIP {rule}: fixture declines `{capability}` — {reason}``
  (`crates/happenstance-testkit/src/contract.rs:500-507`) — reused unchanged, with
  no second format and no projection-local skip type. This is the UX-grain reason
  Architecture brief AC-A06 reuses `Capability` and `RuleOutcome` rather than
  forking them: an author reading one CI log must not have to learn two shapes.
  Instrument: the reused type, plus `cargo check` (a fork would compile, so the
  design review is what rejects it).
- **AC-U09 — A skip is machine-distinguishable from a pass, not merely visible.**
  `RuleOutcome::report` writes to stdout, which libtest suppresses for a *passing*
  test unless `--show-output` is passed — which is why the gate passes it
  (`xtask/src/main.rs:132,151`; `xtask/src/affected.rs:169-173`) — and the port's
  own doc is blunt that this *"makes the line reachable by a human; it does not make
  anyone read it"* (`contract.rs:515-523`). So the projection family owes a sibling
  of `capability_skips_are_reported`
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:3184`) asserting on
  `RuleOutcome` **values**, which is exactly the instrument AC-005 already names.
  Stated here because AC-05 is an initiative-level promise about output, and a
  promise checked only by stdout is a promise checked by nobody.
- **AC-U10 — The skip names the switch the author actually set.** The `capability`
  field carries the identifier the fixture (or the probe) declares — for PS-12 that
  is `ProjectionProbe::READS_THROUGH_BATCH`, whose `false` arm CF-18 requires be
  emitted as a reported skip rather than omitted (`spec/SPECIFICATION.md:5059-5065`)
  — so a reader is sent to the constant they can change. The precedent is
  `NO_STORE_LIMITS`, which names all three constants precisely because *"a skip
  naming only one of them would send an adapter author to look for the constant they
  did set"* (`contract.rs:435-442`). Instrument: the value assertion of AC-U09.
- **AC-U11 — The reason survives the constrained target.** On
  `wasm32-unknown-unknown` `RuleOutcome::report` is a **no-op** — measured under
  `wasm-bindgen-test-runner`, not assumed (`contract.rs:525-531`) — so the
  projection wasm harness routes `skip_line` to `console_log!` the way `__emit_wasm`
  already does, and AC-016's run is not the one where the stated reason silently
  disappears. Instrument: the new `projection_conformance_wasm.rs` harness under the
  existing mandatory wasm32 conformance-harness step (Testing brief, AC-016 row).
- **AC-U12 — A reasonless declension fails the build, and the fixture author is told
  where that fires.** `Capability::declined("")` is a `const fn` `assert!`
  (`contract.rs:411-419`, RS-40-5 at
  `standards/rust/40-public-surface-and-evolution.md:212`), but on an **associated**
  const it is evaluated lazily and fails at codegen — `cargo build` and `cargo test`
  catch it, `cargo check` and `cargo clippy` do not (`contract.rs:405-410`,
  RS-91-3 at `standards/rust/91-adapter-authoring-recipe.md:156-158`). Every
  projection-fixture capability constant inherits that behaviour, so the fixture
  trait's own rustdoc states it, and any `compile_fail` doctest demonstrating it is
  spelled **bare**, never `compile_fail,E0080`, because rustdoc on 1.97.1 silently
  ignores an error code it cannot match and the stricter-looking spelling is the
  weaker check (`contract.rs:400-403`). Instrument: the doctest, run by
  `cargo test --doc`.

### Notes

**"The caller" is three readers, and they meet different surfaces.** Naming them
separately is what keeps the criteria above from collapsing into one generic user:

| Reader | Where they are | What they see |
| --- | --- | --- |
| The rule author, inside the testkit | `crates/happenstance-testkit/src/` | Generic code over the port and `ProjectionProbe`; AC-U03 – AC-U06 are theirs |
| The adapter author, in their own crate | an impl and a fixture | The port's rustdoc, the `E0195` trap, and their own capability constants; AC-U06, AC-U12 |
| Whoever reads the run | a CI log | Only the text surface; AC-U08 – AC-U11 are the whole of what they get |

**The atoms to load, and only these.** `40-public-surface-and-evolution` (RS-40-5,
the capability-with-a-reason shape), `91-adapter-authoring-recipe` (RS-91-3, which
is this brief's subject matter written as a rule with a named wrong
implementation), and `70-rustdoc-obligations` (RS-70-5). The router's instruction is
one to three at a time (`standards/rust/README.md`); pull `30-error-taxonomy` in
place of one of them for AC-U04's story, not in addition.

**Ground the sibling briefs already hold, deliberately not re-claimed here.**

- *Which* capability constants the projection fixture declares, and whether any is
  a MUST in `SECOND_HANDLE`'s sense, is `_design.md`'s call via Architecture brief
  Note 5. AC-U08 – AC-U10 constrain how a declension **reads**, never which ones
  exist.
- Reusing `Capability` and `RuleOutcome` unchanged is Architecture brief AC-A06's
  decision. AC-U08 supplies the UX-grain reason it is the right one; it is not a
  second decision, and it is not a licence to widen either type.
- Asserting on `RuleOutcome` values rather than stdout is already the Testing
  brief's AC-005 instrument. AC-U09 states *why* the value assertion is the
  load-bearing half; the row above owns the test.
- `ProjectionId::new` stays infallible (Architecture brief AC-A09,
  `crates/happenstance-core/src/projection.rs:44-64`). AC-U03's
  illegal-states-unrepresentable argument is scoped to `Checkpoint` and the two
  error enums and must not be read as a mandate to harden the identifier as a side
  effect.

**Deliberately not decided here.** DT-3's answer and DT-8's answer (`_design.md`,
AC-006 / AC-007); the `unstable-projection` exposure verdict (HS-P0016, AC-015);
the application-facing `Projection` trait, the runner and their vocabulary
(HS-P0011); the crate landing page and how any of this is presented to an evaluator
(HS-P0016).

**What would make this brief wrong.** If the projection fixture ends with **no
declinable capability at all** — every rule runnable by every conformant store —
then AC-U08 – AC-U12 are decorative on this port, which is precisely the failure
`CLAUDE.md`'s corollary names for rules and applies just as well to reporting
machinery. That outcome is not a licence to quietly drop the reporting discipline:
it is a finding, and it belongs in DT-3's resolution, because a suite with nothing
to decline cannot demonstrate the initiative's AC-05 and this project is the one
that owns it (`../initiative.md:320-322`).

---

## Architecture brief

### Intent

Give `ProjectionStore` an executable conformance suite and freeze the port on
what that suite proves, by building the **second instance** of a mechanism the
event-store side already runs — one rule enumeration, one fixture contract, one
mutant registry with exactness meta-tests — rather than a parallel invention.

The scope is deliberately narrow in one direction and wide in another. Narrow:
no runner, no `Projection` trait, no shipped SQL adapter, no verdict on the 0.1
exposure (`project.md` "Out of scope"). Wide: the port's own shape changes by
more than the batch lifetime, because `spec/SPECIFICATION.md:4632-4731` already
states the target trait in full and eleven `[FROZEN]` PS clauses presuppose it.
The implementer inherits that shape as a *specification*, not as a suggestion:
where this brief and the specification disagree, the specification wins
(`_intake-brief.md` Gate: Intake, last box).

Two things this brief exists to stop happening. First, the second batch shape
resolving itself silently into a dependency on `sqlite-durable-store`
(HS-P0012), which is downstream (`_decomposition.md` Sequencing, rank 2). Second,
the freeze being *claimed* on evidence PS-2 does not accept.

### Acceptance Criteria

Architecture-scoped, each tracing to the project AC it constrains. These are
obligations on the *design* the implementation stage lands, not a second copy of
`project.md`'s spine.

- **AC-A01** *(project AC-001, AC-016)* — the projection rule set is written in
  exactly one enumeration, `for_each_projection_store_rule!`, living beside
  `for_each_event_store_rule!` in `crates/happenstance-testkit/src/registry.rs:93-220`;
  the three existing emitters (`registry.rs:222-295`) drive it **without a
  fourth mechanism**, and the way they are made to (see Note 4 — they hard-code
  `$crate::rules::$name` today) is a recorded decision rather than a discovery.
- **AC-A02** *(project AC-009, AC-005)* — the generic write seam is
  `ProjectionProbe` in **`happenstance-core`** behind a new `conformance`
  feature, exactly as `spec/SPECIFICATION.md:4977-5031` specifies it, including
  the coherence argument for why it is not a testkit trait. Any deviation is an
  ADR-0017 clause, not an implementation shortcut.
- **AC-A03** *(project AC-004, DoD 2)* — the second batch shape is built **inside
  `happenstance-testkit`'s own `tests/`** as §4.11's CF-5 conformant variant (the
  buffering adapter PS-4 permits, `spec/SPECIFICATION.md:5686-5691`). No edge is
  added from this project to `sqlite-durable-store`, and none is needed.
- **AC-A04** *(project AC-014, AC-015)* — the design states in writing that
  PS-2's bar (`spec/SPECIFICATION.md:4760-4775`) is **not** met by anything this
  project can build alone, and takes AC-014's second arm — `unstable-projection`
  with a stated reason — rather than deleting the provisional marker. The PS-3
  evidence hands `publication-and-positioning` a finding, not a verdict.
- **AC-A05** *(project AC-013)* — the skeleton-compatibility bar is read as
  pinning each adapter's **own type choices** (`Batch` representation, `Error`
  type, storage strategy), not as forbidding the mechanical restatement of port
  method signatures the new shape forces. `LiveHandleProjectionStore`'s
  disposition is decided in ADR-0017 and named, never silently deleted.
- **AC-A06** *(project AC-005, DR-03, DR-07)* — the projection fixture reuses
  `Capability` and `RuleOutcome` from `crates/happenstance-testkit/src/contract.rs:355-537`
  **unchanged**, and declares no borrowing GAT anywhere (`contract.rs:97-111`).
- **AC-A07** *(project AC-002, AC-003)* — the projection mutant registry carries
  the same `Declared` shape and the same three exactness meta-tests as
  `crates/happenstance-testkit/tests/mutation_coverage.rs`, including the
  no-pass-rate warning ADR-0010 requires, and its scope note replaces the
  current "the projection store port owes its own CF-1 – CF-5 obligation and has
  neither a suite nor a mutant yet" (`mutation_coverage.rs:197-206`).
- **AC-A08** *(project AC-008, risks)* — nothing `[FROZEN]` is line-edited. The
  PS-1 / PS-19 scope gaps and any sibling found by the PS-1 – PS-37 sweep are
  carried as ADR scope with the sweep's result recorded, per
  `.kb/open-questions/ps-1-states-no-progress-obligation.md` and
  `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`.
- **AC-A09** — `ProjectionId::new` stays infallible. Its inconsistency with
  `EventType`/`Tag` is an open question owned elsewhere
  (`.kb/open-questions/projection-id-is-unvalidated.md`;
  `crates/happenstance-core/src/projection.rs:44-64`) and is not repaired as a
  side effect of freezing the port around it.

### Notes

#### 1. The seam map — what is touched, and what it mounts into

A library has no render tree, so "composition root" means the two places a new
item is either reachable or invisible: the crate's `lib.rs` export block and the
feature table that gates it. An item that compiles and is not mounted at both is
an item no adapter can name.

| Mount point | File and lines | What must land there |
|---|---|---|
| Contract crate exports | `crates/happenstance-core/src/lib.rs:98-124` | `Checkpoint`, `Authority`, `CommitError`, `ResetError` re-exported beside `ProjectionId, ProjectionStore, SendProjectionStore` (line 116); `ProjectionProbe` behind `#[cfg(feature = "conformance")]` + `#[cfg_attr(docsrs, doc(cfg(…)))]`, copying the `memory` pattern at lines 101-103 / 120-122; `MemoryProjectionStore` re-exported beside `MemoryEventStore` (line 122) |
| Contract crate features | `crates/happenstance-core/Cargo.toml` `[features]` | a new `conformance` feature (probe only, no new dependency); `memory` already exists and already means `std`; `unstable-projection` if AC-A04's arm is taken |
| The port itself | `crates/happenstance-core/src/projection.rs:1-139` | the whole §4.0 shape; the module header's provisional block (`:1-11`) is what AC-014 disposes of |
| Reference store | new module under `crates/happenstance-core/src/`, gated exactly like `memory` (`lib.rs:101-103`) | `MemoryProjectionStore` — the oracle, the doctest target, the cold-start fix (`RUNBOOK.md:3898-3903`) |
| Suite entry point | `crates/happenstance-testkit/src/lib.rs:265-357` (beside `event_store_conformance!`), `:359-364` (`__private`) | `projection_store_conformance!`; the projection fixture trait **must** be added to `__private`, or the macro expansion cannot name it in the adapter's crate |
| Rule enumeration | `crates/happenstance-testkit/src/registry.rs:93-220` | `for_each_projection_store_rule!`, and whatever change Note 4 settles for the emitters at `:222-295` |
| Orphan meta-test | `registry.rs:346-366` (`declared_rules`), `:410-436` (`no_orphan_rules`) | the scan is `include_str!("suite.rs")` with the literal prefix `    pub async fn `; a projection suite in a *different* file needs its own scan or a generalised one. This is the concrete content of AC-001 |
| Testkit module list | `crates/happenstance-testkit/src/lib.rs:165-189` | the new `mod`/`pub use` lines; the crate doc's "What is checked" table (`:134-151`) and "Where the rule set lives" (`:84-92`) both describe a single family set and must gain the projection family |
| Three harnesses | `crates/happenstance-testkit/tests/memory_conformance.rs`, `memory_conformance_blocking.rs`, `memory_conformance_wasm.rs` | three sibling files invoking `projection_store_conformance!` with the three emitters. The wasm one is `#![cfg(target_arch = "wasm32")]` and is type-checked by `xtask/src/main.rs:231-240` — that step is the whole of AC-016's mechanism |
| Mutant registry | `crates/happenstance-testkit/tests/mutation_coverage.rs` + `tests/mutation_coverage/{harness,mutants,variants}.rs` | `CheckpointOnlyStore`, `TruncatingResetStore`, `ValidatingCommitStore`, and CF-5's conformant buffering variant |
| Skeletons | `crates/happenstance-postgres/src/projection_store.rs:94-104`, `crates/happenstance-ladybug/src/projection_store.rs:258-270`, `crates/happenstance-ladybug/src/live_handle.rs:154-181`, `crates/happenstance-sqlite/src/projection_store.rs:208-225` | AC-013's evidence, and Note 7's problem |
| Specification | `spec/SPECIFICATION.md` §4 and the clause table at `:8630-8666` | maturity markers and rule citations; `cargo xtask spec-trace` is a gate step |
| Knowledge base | `.kb/decisions/`, `.kb/open-questions/`, `.kb/maps/` | ADR-0017/0018/0019 as atoms, and the open questions **resolved rather than deleted** (DR-06) |

**Sibling contracts this wires to, by name.** The projection suite is not a
standalone binary: it shares `Capability` and `RuleOutcome`
(`contract.rs:355-537`) with the event-store suite, shares the emitters
(`registry.rs:222-295`), shares the `Declared` registry shape
(`mutation_coverage.rs:141-186`) and its harness's `Subject: Fixture + Sized`
convention (`tests/mutation_coverage/harness.rs:60-63`), and shares one
`cargo xtask ci` run. Every one of those is an existing contract with an
existing user; changing one is a change to the event-store suite and must be
justified as such.

#### 2. The accepted decisions that constrain this, and where they bite

- **ADR-0001** (`.kb/decisions/0001-async-port-flavours.md`) — no `#[async_trait]`,
  two flavours from one definition. `projection.rs:87` already carries
  `#[trait_variant::make(SendProjectionStore: Send)]` and keeps it. The
  consequence the port must *document* rather than enforce is PS-36
  (`spec/SPECIFICATION.md:5601-5627`): the `Send` flavour transitively requires
  `Batch: Send`, and `type Batch: Send;` cannot be written because
  `trait_variant` copies associated-type bounds verbatim into the `!Send`
  flavour and would break wasm32.
- **ADR-0008** (`.kb/decisions/0008-one-derivation-for-both-ports.md`) — one
  derivation scheme for both ports, and three rules binding any *provided*
  method: hand-desugar to `-> impl Future`, take `where Self: Sync` at the point
  of use, and compile one body against both flavours (PS-37,
  `spec/SPECIFICATION.md:5628-5651`). This is live here: the probe and any
  defaulted helper on `ProjectionStore` are exactly the surface it governs, and
  ADR-0008's finding that a provided body cannot hold the `Batch` GAT across a
  suspension point is one of the inputs ADR-0017 must weigh
  (`.kb/open-questions/projection-store-batch-has-no-apply-seam.md`, sub-question 1).
- **ADR-0007** (`.kb/decisions/0007-projection-runner-decodes.md`) — where the
  runner splits. Its Context overstates itself and PS-32 requires the
  correction: a *callback-driven* pump compiles against the port as it stands; a
  runner that itself writes into the batch does not
  (`spec/SPECIFICATION.md:5511-5528`). The write seam is grown **because the
  suite needs one**, not because the runner does — that framing is what keeps
  PS-9 honest and keeps the runner in HS-P0011.
- **ADR-0010** (`.kb/decisions/0010-the-suite-must-prove-itself.md`) — a rule may
  not exist without a store that fails it; three meta-tests enforce it; **no
  pass rate is ever quoted**. This is the whole of AC-002/AC-003 and is not
  negotiable at the projection port merely because its suite is new.
- **CLAUDE.md** "The rule that matters" and its two corollaries — never assert
  literal position values, and check the *spread* of implementers before
  freezing. The spread corollary is what Note 3 is about.

#### 3. The DoD-7 / AC-004 second shape — settled here

**Decision: the second structurally unlike batch shape is built inside
`crates/happenstance-testkit/tests/` as the CF-5 conformant variant, not
borrowed from `happenstance-sqlite`.**

Grounds, in order of authority:

1. The specification already assigns it there. §4.11 owes the projection suite
   "CF-5's conformant variant — a store legally different from
   `MemoryProjectionStore` that passes everything — and the obvious one is the
   buffering adapter PS-4 permits, **which doubles as the far end of §6's
   batch-shape axis**" (`spec/SPECIFICATION.md:5686-5691`). The far end is named
   and it is testkit-internal.
2. The precedent is in the tree. `GappedPositionStore` and `PagedStreamStore`
   are conformant variants living in `tests/mutation_coverage/variants.rs` whose
   entire job is to be legally unlike `MemoryEventStore`
   (`mutation_coverage.rs:324-351`); `MemoryFixture` states no CF-40 limit and
   `GappedPositionFixture` is what keeps the limits rule non-vacuous
   (`crates/happenstance-testkit/src/fixtures.rs:234-241`). Building an
   instrument to keep a rule non-vacuous is this repository's established move.
3. The alternative inverts the DAG. `sqlite-durable-store` (HS-P0012) is rank 2
   and lists this project in its `blocked_by` (`project.md` Dependencies); the
   RUNBOOK's own phase-6 proof artefact assumes the rusqlite skeleton "fleshed
   out far enough to commit a real transaction" (`RUNBOOK.md:3935-3940`), which
   is an instrument rather than the shipped adapter — but building it *here*
   means adding `rusqlite` to the testkit's dev-dependencies, which is a
   `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]` entry and a new
   licence/advisory surface for `cargo deny`, to buy a shape the buffering
   variant already provides.

**What this decision does not buy, stated so nobody claims it later.** PS-2 is
`[FROZEN]` and its bar is "two *adapters* at opposite ends of the batch-shape
axis", with rusqlite or `sqlx` on one end and Workers `SqlStorage` or Neon over
one-shot HTTP on the other — and its **Rejects** clause names, verbatim, "the
schedule that freezes this port against `MemoryProjectionStore` and an
in-process rusqlite transaction" as the monoculture to refuse
(`spec/SPECIFICATION.md:4760-4775`). Two testkit instruments do not clear that
bar either. So:

- AC-004 / DoD 2 (**two structurally unlike batch shapes pass the suite**) is
  satisfied in-project, by `MemoryProjectionStore` (apply-on-write) and the
  buffering variant (replay-at-commit).
- PS-2's bar is **not** satisfied in-project and must not be reported as
  satisfied. It is cleared by `cloudflare-durable-object-store` (HS-P0013) /
  `postgres-and-neon-stores` (HS-P0014) / `ladybug-projection-store` (HS-P0015).
- Therefore AC-014 takes its **second arm**: the module goes behind
  `unstable-projection` with a stated reason, which is exactly PS-3's
  `tokio_unstable` idiom (`spec/SPECIFICATION.md:4776-4790`) and exactly what
  `RUNBOOK.md:3924-3928` calls "the honest option if the two batch shapes
  disagree". This project writes the finding; HS-P0016 makes the call (AC-015).

That is the coherent reading of the whole clause set, and it costs the project
nothing it was promised: the freeze verdict was never this project's
(`project.md` "How this advances the initiative").

#### 4. The write seam and the rule-running seam — two different problems

**The write seam is `ProjectionProbe`, in the contract crate.**
`spec/SPECIFICATION.md:4998-5031` gives the trait verbatim — `READS_THROUGH_BATCH`,
`probe_write`, `probe_delete_all`, `probe_read`, `probe_read_through` — behind
`feature = "conformance"`, bare flavour only, and gives the coherence reason it
cannot live in the testkit: an adapter's `tests/` directory is a *different
crate*, where neither a testkit trait nor the adapter's type is local, so the
impl is rejected by the orphan rule and the adapter is forced into a non-dev
dependency on `happenstance-testkit`. This is the part most likely to be
rediscovered expensively; it is already discovered.

Three consequences the implementer should plan for rather than meet:

- `probe_delete_all` exists so `reset` (PS-16) is checkable without the suite
  knowing what a read model is. It is not optional garnish.
- `READS_THROUGH_BATCH` is a `const` gate, and CF-18 requires a `false` adapter
  to emit the rule as a **reported skip**, not to omit it
  (`spec/SPECIFICATION.md:5052-5074`). That is `RuleOutcome::Skipped` reused
  unchanged — the machinery already exists at `contract.rs:458-537`.
- Adding `conformance` to `happenstance-core` widens two gate steps: the
  workspace feature powerset (`xtask/src/main.rs:546-556`) and the **wasm32**
  powerset, which names `happenstance-core` explicitly
  (`xtask/src/main.rs:558-591`). Core has three features today; `conformance`
  plus a possible `unstable-projection` takes eight combinations to thirty-two.
  Each must compile — including `conformance` *without* `memory`.

**The rule-running seam is not free, and this is the trap.** The three emitters
expand to `$crate::rules::$name(__conformance_fixture)` with the module path
**hard-coded** (`registry.rs:229-238`, `:248-257`, `:277-287`). A projection rule
does not live in `crate::rules`, so `projection_store_conformance!` cannot reuse
them as written. Three ways out, none pre-selected here:

1. **Parameterise the emitters** with the rules module path
   (`__emit_tokio!(path = $crate::projection_rules, …)`), keeping one emitter per
   runtime. Cheapest to reason about; touches macros the event-store suite uses,
   so every existing harness must keep compiling — and the arms are
   `#[macro_export]`ed, so an added arm is a public surface change on a crate
   that carries its own version for exactly this reason
   (`crates/happenstance-testkit/Cargo.toml`, the `version` comment).
2. **Three more emitters** (`__emit_projection_tokio`, …). Zero risk to the
   event-store path, and it makes "the wrapper is a parameter" true twice over
   rather than once — but it is the duplication `registry.rs:1-14` argues
   against, and a fourth runtime would then need six emitters.
3. **Re-export projection rules into the existing `rules` module.** Cheapest
   diff, worst outcome: `no_orphan_rules`'s source scan (`registry.rs:352-366`)
   is a textual scan of `suite.rs` and would report every projection rule as
   "registered but not found", so the meta-test AC-001 is built on breaks.

Whichever is chosen, AC-016 is discharged by the *existing* mechanism: the wasm
harness file is type-checked by the "wasm32 check of the conformance harnesses"
step (`xtask/src/main.rs:231-240`), which is mandatory and already runs. That is
why AC-016 says "not as a separately maintained subset" — the subset is not a
risk unless someone invents one.

#### 5. The fixture contract — a second trait, not a widened one

`Fixture::Store: EventStore` (`contract.rs:120-125`) binds the event-store port
in the associated type, so the projection suite needs its own fixture trait. It
should mirror `contract.rs` closely and reuse the parts that are port-agnostic:

- **Reuse unchanged:** `Capability` (`contract.rs:355-433`) and `RuleOutcome`
  (`:458-537`). Both are about *reporting*, not about events, and forking them
  would give an adapter author two skip vocabularies to learn.
- **Mirror, do not share:** the capability *set*. The event-store fixture's
  `SECOND_HANDLE` / `REOPEN` / `MID_BATCH_FAULT` were chosen for that port;
  the projection port needs at least a way to say "this store protects this id
  from reset" for `refused_reset_changes_nothing`
  (`spec/SPECIFICATION.md:5200-5217`), and PS-12's gate is a `ProjectionProbe`
  const rather than a fixture const. Which constants exist, and whether any is a
  MUST in `SECOND_HANDLE`'s sense, is `_design.md`'s call — and DT-3's answer
  must cite `.kb/open-questions/cf-40-fixture-limits-ownership.md` as
  authoritative rather than minting a second declension policy (AC-006).
- **Forbidden:** `type Store<'a> where Self: 'a`, or any borrowing GAT on the
  fixture. It is one of five ingredients of a rustc ICE this repository already
  minimised and which still reproduces on 1.97.1 (`contract.rs:97-111`,
  `experiments/rustc-ice-gat-foreign-trait/`). The fix is the one `MemoryFixture`
  already demonstrates: hand back an owned handle holding a refcount
  (`fixtures.rs:243-292`).
- **Rules take `impl AsyncFn() -> F`, not a made fixture** (`registry.rs:45-49`).
  `commit_rejects_a_foreign_batch` (PS-15) depends on this directly: it needs two
  *isolated* stores, which is two `open()` calls, and it explicitly does not need
  a second handle (`spec/SPECIFICATION.md:5126-5155`).

#### 6. Data flow — what one rule actually does

The shape every projection rule shares, and the reason the probe is load-bearing:

```
rule(open: impl AsyncFn() -> F)
  → open().await                     one isolated projection store
  → fixture.connect().await          one handle (F::Store: ProjectionStore)
  → store.begin()                    PS-6: not async, not fallible → Self::Batch (owned, PS-5)
  → store.probe_write(&mut batch, k, v)        ProjectionProbe, contract crate
  → store.commit(batch, &id, position, Authority::Live).await
                                     PS-1: read-model write + checkpoint, one unit
  → open a *fresh* handle
  → store.probe_read(k).await   AND  store.checkpoint(&id).await
  → assert both present, or both absent — never one
```

`CheckpointOnlyStore` fails at the last line and nowhere else, which is why it is
the mutant that makes the suite non-decorative: without `probe_write` /
`probe_read` the rule cannot observe the read model at all, and the whole suite
reduces to a checkpoint test that a broken store passes
(`RUNBOOK.md:3882-3892`).

Two variations worth naming because they are the other rules' skeletons:
`rollback` / drop rules substitute `store.rollback(batch)` or a bare `drop(batch)`
for the commit and then **open and commit a second batch**, because PS-7's rule is
"rolls back *and the store remains usable*" — a pooled connection whose `Drop`
returns to nothing leaves the store answering `Busy` forever
(`spec/SPECIFICATION.md:4898-4910`). And `rebuild_is_chunk_size_invariant`
replays a fixed probe sequence at chunk sizes 1, 3 and whole-log, with the
probe's write defined as an increment of what the batch can see, so a store
violating PS-12 diverges (`:5086-5098`).

#### 7. The port shape changes by more than the lifetime — and AC-013's real bar

`spec/SPECIFICATION.md:4632-4731` is the target trait. Against today's
`projection.rs:87-139` it changes **five** things, not one:

| Today | Target | Clause |
|---|---|---|
| `type Batch<'a> where Self: 'a` | `type Batch;` | PS-5 |
| `async fn begin(&self) -> Result<Batch, E>` | `fn begin(&self) -> Self::Batch` | PS-6 |
| `checkpoint → Option<SequencePosition>` | `checkpoint → Checkpoint` (`NeverRun` / `Live` / `Rebuilding`) | PS-19, PS-24 |
| `commit(batch, id, position) -> Result<(), E>` | `commit(batch, id, position, Authority) -> Result<(), CommitError<E>>` | PS-15, PS-21, PS-22, PS-24 |
| — | `reset(batch, id) -> Result<(), ResetError<E>>` | PS-16 – PS-18 |

So every skeleton's method signatures must be restated. **AC-013's bar is
therefore about the adapter's own choices, not about the diff's size**: same
underlying `Batch` type, same `Error` type, same storage strategy, same
(`todo!()`) bodies. `PostgresProjectionStore` keeps
`Transaction<'static, Postgres>` (`crates/happenstance-postgres/src/projection_store.rs:99-104`)
and `LadybugProjectionStore` keeps `GraphWriteSet`
(`crates/happenstance-ladybug/src/projection_store.rs:261-270`) — both already
bound *owned* types to the GAT precisely so that dropping it is "a deletion here
rather than a redesign" (that crate's own comment). If an edit to either goes
beyond restating a signature, the port change is what is suspect, not the
skeleton. And the weaker literal bar — "compile unchanged" — is unsatisfiable by
the phase's own central decision and must not be restored
(`RUNBOOK.md:3942-3948`).

**`LiveHandleProjectionStore` is the exception, and it is the interesting one.**
It binds `type Batch<'a> = GraphWriteHandle<'a>`, a genuinely borrowed live
handle on the `Send` flavour, with real bodies rather than `todo!()`
(`crates/happenstance-ladybug/src/live_handle.rs:154-219`). It exists to be the
counter-example to §4.2's argument, and its module doc says so: the `Send`
objections to a borrowed batch "are properties of **rusqlite**, not of live
handles", so "the evidence for dropping the GAT is one-driver-wide"
(`live_handle.rs:16-31`). Under `type Batch;` that impl cannot survive as
written. Two facts must reach ADR-0017 intact:

- The clause should rest on PS-5's *other* argument — the one this instrument
  does not touch — and that argument is stronger than the `Send` one anyway:
  today's port is implementable **only by stores that outlive every batch
  lifetime**, and a non-`'static` store ICEs the compiler in the error-reporting
  path with `DefId::expect_local` (`live_handle.rs:38-66`;
  `references/adapter-shapes.md:311-347`). Plus `error[E0195]`, met from the
  implementer's side (`live_handle.rs:68-83`; `references/adapter-shapes.md:186-191`),
  which is the trap AC-012 documents.
- The instrument's disposition — deleted, moved to `experiments/`, or kept with
  its transcripts and a note that the port no longer admits it — is a decision
  with a name on it, because deleting the only compiled evidence against the
  decision you are making is how a port gets frozen against its own hypothesis.

Also carry forward: **dropping the lifetime does not close the foreign-batch
hazard.** Tying the batch to the receiver's lifetime was compiled and refuted —
a lifetime names a region, not an instance — and only a generative brand rejects
`b.commit(a.begin())`, which fights `async` and forbids the batch escaping a
closure (`spec/SPECIFICATION.md:5099-5125`; `RUNBOOK.md:3876-3881`). PS-15 stays
`[PROVISIONAL]` and is discharged at run time by `CommitError::ForeignBatch`:
`begin` stamps an identity minted per store instance and `commit` compares. With
an owned batch that stamp is a field and the check is an integer comparison.

#### 8. `[FROZEN]` clauses this work will want to widen — and must not

`PS-1`'s MUST is a *coupling*, not a progress obligation: a `commit` returning
`Ok` that makes neither write durable satisfies the "or not at all" arm, passes
`commit_is_atomic_with_the_read_model`, and fails `commit_advances_the_checkpoint`
— a rule the §4.11 table assigns it that no clause's MUST supports
(`spec/SPECIFICATION.md:4744-4759`). `PS-19` has the identical shape: the clause
is scoped *after a successful reset*, while `fresh_projection_has_no_checkpoint`
asks about an id never seen, and the store that satisfies one and fails the other
is the natural implementation rather than a contrivance (`:5218-5249`).

Both clauses are `[FROZEN]`, both name phase 6 as owner, and both open-question
atoms ask the same thing before any fix is scoped: **check the other 35 PS
clauses for the same shape**, because a systematic pairing defect and two
isolated ones want different repairs. Concretely, for this project:

- The sweep is a deliverable — a recorded finding over PS-1 – PS-37, not a
  side-effect of writing rules.
- The repair is a **new decision atom** under the repair-frozen-clause
  discipline both atoms cite, never a line edit to the clause
  (`CLAUDE.md`, "Changing a `[FROZEN]` clause requires a new ADR, not an edit").
- If the sweep says "systematic", that is ADR scope this project should
  *report* rather than silently absorb: widening ten frozen clauses is a
  re-plan, and `RUNBOOK.md`'s phase-6 budget assumes it is not one.

The adjacent trap: `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`.
A boundary-scoped projection checkpoint reopens ADR-0013's globally frozen
visibility invariant. If the design drifts there, stop and file the decision.

#### 9. Non-prescriptive implementation notes

Things left to the implementer and to `_design.md`, recorded so they are chosen
rather than defaulted:

- **How many rules ship.** §4.11 names seventeen adapter-level rules and moves
  six to an integration crate under CF-36 (`spec/SPECIFICATION.md:5658-5703`).
  The six integration rules belong to `typed-layer-and-alpha-release`; whether
  all seventeen adapter rules land here, or a named subset with the remainder
  carrying an accurate maturity marker, is a scoping decision AC-014 audits.
- **One mutant binary or two.** `mutation_coverage.rs` is 3,527 lines and its
  `Subject: Fixture` harness and `all_rules()` helper are event-store-bound. A
  sibling `projection_mutation_coverage.rs` reusing `Declared`'s shape is the
  low-risk read; a single binary keeps one place to look. Either way the three
  meta-tests must exist for the projection family — every rule has a mutant,
  the registry is exhaustive, and each mutant fails **exactly** its declared
  rules (`mutation_coverage.rs:2733`, `:2753`, `:2888`).
- **Two more hostile stores than the intake brief names.** §4.11 requires
  `TruncatingResetStore` and `ValidatingCommitStore` alongside
  `CheckpointOnlyStore` (`spec/SPECIFICATION.md:5680-5685`). `CheckpointOnlyStore`
  is the *bar*; it is not the whole obligation, because AC-003 requires a mutant
  per rule.
- **DT-8's blast radius.** If the suite's bar is held for an outside adapter
  author (AC-007), the extension surface is the documented pair
  `projection_store_conformance!` + `ProjectionProbe`, and `ProjectionProbe`
  living in the contract crate is what makes that surface cost an outside author
  one feature flag instead of a new dependency edge (`spec/SPECIFICATION.md:5015-5031`).
- **Rustdoc hazard, already paid for once.** An intra-doc link into a module
  absent on some documented configuration is a hard rustdoc error; the testkit
  spells two module names plainly for exactly this reason
  (`crates/happenstance-testkit/src/lib.rs:112-132`). `MemoryProjectionStore`
  behind `memory` and `ProjectionProbe` behind `conformance` are two new
  instances of the same hazard, and the gate runs `cargo doc` for the host only.
- **`begin` becoming infallible is load-bearing for one deployment.** PS-6 is
  provisional and owned by the Neon phase; the point is that `async` + fallible
  *implies a round trip* Neon's one-shot HTTP transport cannot afford and does
  not need (`spec/SPECIFICATION.md:4884-4897`). Do not "improve" it back to
  `async fn begin() -> Result<…>` for symmetry with `EventStore`.
- **Never assert literal positions.** The specification permits gaps, and
  `GappedPositionStore` exists to convict a rule that forgets
  (`mutation_coverage.rs:326-338`; `CLAUDE.md`, "The rule that matters").

#### 10. What would make this brief wrong

Named so the implementation stage can report it rather than absorb it:

1. **A projection rule that observes the read model without the probe.** If one
   is found, PS-11 is over-built and the seam should shrink — that is a finding
   worth an ADR paragraph, not a quiet deletion.
2. **The buffering variant passing every rule trivially.** If the two shapes
   never disagree anywhere, either the rules are shape-blind in a way that hides
   the axis, or the axis is not where §4.2 says it is. PS-3's evidence is
   exactly this question and AC-015 is where the answer goes.
3. **The emitter change breaking an existing harness.** `local_conformance.rs`,
   `memory_conformance*.rs` and `fixture_instruments.rs` all consume the
   emitters; if Note 4's option 1 cannot keep them compiling, take option 2 and
   say why.
4. **The PS-1 – PS-37 sweep returning "systematic".** Then the frozen-clause
   repair is larger than one ADR and this project's scope is wrong — report it
   at the story boundary rather than widening ten clauses under cover of a
   rule-writing story.

---

## Testing brief

### Intent

Prove `ProjectionStore` the same way `EventStore` is proved, by extending the
**same** conformance harness rather than building a second one: one rule
enumeration, one fixture contract, one mutant registry, three runtime
emitters, one `cargo xtask ci` run. This project's whole deliverable is
executable proof — DoD 1 and DoD 2 are a test result, not a design document —
so this brief exists to answer, for every project AC-###, *which tier proves
it, with which command, against which fixture*, and to name explicitly the
one thing that no test tier can prove (AC-006/AC-007/AC-015, decisions
recorded in prose) so nobody looks for a test that was never going to exist.

Traces to [`project.md`](project.md) AC-001 – AC-016 and DoD 1 – 8. It does not
restate the charter; see `project.md` for the acceptance criteria's full text
and the Architecture brief above for the design decisions this brief takes as
given — most load-bearingly **AC-A03** (the second batch shape is a testkit
conformant variant, not `happenstance-sqlite`) and **AC-A02** (`ProjectionProbe`
lives in `happenstance-core` behind a `conformance` feature). This brief does
not re-decide either; it says what proves them once built.

### Acceptance Criteria

One row per project AC-###, mapped to the tier(s) that prove it and the
concrete artefact. Tier vocabulary follows the taxonomy already in use
elsewhere in this initiative
(`.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md`
Testing brief): **Static** is a check that reads source/config without
executing the code under test (`fmt`, `clippy`, `cargo hack`, `cargo deny`,
doc builds, `redkiln validate`); **Unit** is `cargo test` running a function
in-process, including this suite's own meta-tests over its registries, which
check structure rather than behaviour but are ordinary `#[test]` functions;
**Integration** is a conformance-rule actually driving a `ProjectionStore`
implementation through `begin` / write / `commit` / read-back; **E2E** is
`cargo xtask ci` run whole, or a claim provable only by inspecting that run's
combined output.

| AC | Tier(s) | How it is proven |
| --- | --- | --- |
| **AC-001** — one enumeration, meta-test over orphans | **Unit** + **Static** | A `for_each_projection_store_rule!`-scoped sibling of `no_orphan_rules` (`crates/happenstance-testkit/src/registry.rs:410-436`), same two-direction check: every `pub async fn` the projection rules module declares appears in the enumeration, and every enumerated name resolves (the resolving direction is free — an unregistered path is `error[E0425]` in every harness, same as today). Static because the macro's own expansion is checked by `cargo check` compiling three harness crates; Unit because the orphan scan is a `#[test]` function that runs and can fail. |
| **AC-002** — `CheckpointOnlyStore` fails by name, passes what it does not declare | **Integration** + **Unit** | Integration: `CheckpointOnlyStore` is registered as a `Fixture` and run against the full projection suite through `projection_store_conformance!`, so the failure is observed by actually calling `begin`/write/`commit` and reading back. Unit: a projection sibling of `mutants_fail_exactly_their_declared_rules` (`mutation_coverage.rs:2862-2907`, CF-3) asserts both directions programmatically — declared rules fail, every other rule passes or is accounted for — so the exactness claim is checked once, not eyeballed per run. |
| **AC-003** — every rule has a registered mutant, provenance non-empty, no pass rate quoted | **Unit** + **Static** | Unit: siblings of `every_rule_has_a_mutant` (CF-1, `mutation_coverage.rs:2720-2749`) and `mutant_registry_is_exhaustive` (CF-2, `:2751-2860`) over the new projection `REGISTRY: &[Declared]`, asserting non-empty `provenance` per entry (`Declared`'s own doc comment, `mutation_coverage.rs:150-151`). Static: the registry's own module doc must carry ADR-0010's no-pass-rate warning verbatim in spirit (`mutation_coverage.rs:203-206`) — a review-time check on the doc comment, not a runtime assertion, because "no pass rate is quoted" is a property of what the code does **not** print, and there is nothing to assert against an absence except reading it. |
| **AC-004** — two structurally unlike batch shapes pass the whole suite | **Integration** + **E2E** | Integration: `MemoryProjectionStore` (apply-on-write) and the CF-5 buffering conformant variant (replay-at-commit, AC-A03) each drive `projection_store_conformance!` to completion with zero failures. E2E: both runs happen inside the same `cargo xtask ci` invocation (the `tests` step), so the proof artefact is one gate run naming both fixtures, not two separate `cargo test` invocations a reviewer has to reconcile by hand. |
| **AC-005** — declined capability still emitted, reports a skip with a reason, distinguishable from a pass | **Unit** + **Integration** | Unit: assert directly on the returned `RuleOutcome` value (`contract.rs:458-537`, `#[must_use]`) for a rule run against a fixture that declines the relevant capability — mirrors the event-store fixture's own tests over `Capability`/`RuleOutcome`, reused unchanged (Architecture brief AC-A06). Integration: the same rule, run inside a real harness invocation, prints `RuleOutcome::skip_line`'s text to stdout (tokio/blocking emitters) so a human reading `cargo xtask ci` output sees the reason, not silence. |
| **AC-006** — DT-3 resolved in `_design.md`, citing CF-40's atom as authoritative | **None (design-stage artefact)** | Not test-provable: a resolution recorded in prose is checked by the design-stage review this project owes (`project.md` DoD 7), not by a test tier. Named here only so no story tries to invent a test for it. |
| **AC-007** — DT-8 resolved; if outside-author bar, a fixture written from documentation alone clears AC-002/AC-005 | **Integration** (conditional on the DT-8 arm taken) | If `_design.md` takes the outside-author arm: a fixture built from the documented extension surface (`projection_store_conformance!` + `ProjectionProbe`, Architecture brief Note 9) — not copied from `fixtures.rs` — is run against the mutant registry and the capability-skip rule, proving the documented surface is sufficient on its own. If the narrower arm is taken, this row is discharged by that decision (Static, "the recorded scope matches what the suite actually holds implementers to") rather than by a second fixture. |
| **AC-008** — ADR-0017/18/19 accepted before the port change lands; open questions resolved not deleted | **Static** | `redkiln validate --kb` (KbFrontmatter conformance, immutability of any prior accepted atom) run **before** the port-shape commit, not after — the ordering is the check, so this is a process gate on commit sequence as much as a schema check. `git diff --diff-filter=D` over `.kb/open-questions/` for the four atoms this project touches must be empty (mirrors the closeout project's AC-006 check, same repo, same command shape). |
| **AC-009** — `type Batch` carries no lifetime; generic code writes and reads back without touching adapter internals | **Unit** + **Static** | Static: `type Batch;` with no lifetime parameter is a signature fact `cargo check` enforces at every call site — a lifetime reappearing anywhere is a compile error, not a test failure. Unit: a rule (or a small standalone test in `happenstance-core`'s own `#[cfg(test)]`, gated by `conformance`) calls `probe_write` then `probe_read` through nothing but the trait, against `MemoryProjectionStore`, and asserts the value round-trips — proving the seam is generic rather than merely present. |
| **AC-010** — a batch dropped without `commit`/`rollback` rolls back **and** the store stays usable | **Integration** | A rule opens a batch, writes through the probe, drops it bare (no `commit`, no `rollback`), then opens a **second** batch on the same handle and commits successfully — PS-7's exact scenario (`RUNBOOK.md:3893-3897`). Proven against a real fixture (Integration, not Unit) because "stays usable" is a claim about the store's subsequent behaviour, which a structural assertion cannot see. A hostile mutant (a store whose `Drop` leaks the connection, per Architecture brief Note 9's "two more hostile stores") is what makes this rule non-decorative — passing it is not automatic. |
| **AC-011** — `reset` clears rows and checkpoint in one unit, scoped to one `(store, id)`, is refusable; `commit(empty, id, FIRST)` is rejected as a substitute | **Integration** + **Unit** | Integration: a rule performs `reset`, then asserts both the read model and the checkpoint are absent for that id and unaffected for a sibling id — the one-unit-and-scoped claim needs a real store round trip. Unit: `commit(empty, id, FIRST)` run as the declared failure mode of a hostile mutant (`ValidatingCommitStore` or a sibling, Architecture brief Note 9) must fail the reset-equivalence rule by name — this is what turns RUNBOOK's observation that "all six [scenarios] reached for and all six got wrong" (`RUNBOOK.md:3904-3907`) into an enforced rejection instead of a warning in prose. |
| **AC-012** — `MemoryProjectionStore` behind `memory`, doctest target, `E0195` trap documented | **Unit** + **Static** | Unit: the port's own rustdoc example (`crates/happenstance-core/src/projection.rs`, mirroring the doctest convention `standards/rust/70-rustdoc-obligations.md` sets and `CLAUDE.md`'s "a doctest in place of a mock" for the design-stage review) compiles and runs against `MemoryProjectionStore` as part of `cargo test --doc`. Static: the nightly `--cfg docsrs` rustdoc build (mandatory-if-tool-present step) renders the `E0195` trap's documentation without the intra-doc-link hazard `crates/happenstance-testkit/src/lib.rs:112-132` already paid for once — the `memory`-gated module must be spelled the same defensive way. |
| **AC-013** — Ladybug and Postgres skeletons compile with only the lifetime removed; `cargo xtask ci` green | **Static** | `cargo check`/`cargo build` on `happenstance-postgres` and `happenstance-ladybug` as part of the workspace `tests`/build steps inside `cargo xtask ci` — compile-only, because every affected method body is still `todo!()` (`crates/happenstance-postgres/src/projection_store.rs:94-104`, `crates/happenstance-ladybug/src/projection_store.rs:258-270`, `crates/happenstance-ladybug/src/live_handle.rs:154-181`). A diff review (which lines changed) is the check that the *only* change is the lifetime removal — not a test a compiler can run, since a compiler cannot tell "the minimal restatement" from "an unrelated edit that also compiles." Recorded as a reviewed diff, per Architecture brief AC-A05. |
| **AC-014** — `projection.rs` drops "provisional" or gates behind `unstable-projection` with a reason; every PS-1 – PS-37 clause carries an accurate maturity marker | **Static** | `cargo xtask spec-trace` (mandatory gate step) over `spec/SPECIFICATION.md`'s clause table, which is exactly the check CLAUDE.md names for this class of claim ("Open questions, deliberately unresolved" — maturity markers "cannot rot into decoration" because spec-trace is a gate step). A `[FROZEN]` clause changing maturity without a new ADR is what this step, plus `redkiln validate --kb`, is jointly positioned to catch. |
| **AC-015** — PS-3 evidence (did the two shapes disagree?) recorded as a finding, no verdict made | **E2E (process, derived from AC-004)** | Not a new test: the finding is written from what AC-004's Integration run actually showed — did any rule pass on one fixture's semantics and fail on the other in a way that needed different handling. If both fixtures pass identically, that itself is the finding (Architecture brief Note 10, "what would make this brief wrong" item 2) and is recorded rather than silently treated as success. |
| **AC-016** — every projection rule passes under the `wasm32` emitter in the same `cargo xtask ci` run | **E2E** + **Integration** | The mandatory "wasm32 check of the conformance harnesses" step (`xtask/src/main.rs:231-243`, `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown`) type-checks a new `projection_conformance_wasm.rs` sibling of `memory_conformance_wasm.rs`, using `__emit_wasm` unchanged (Architecture brief Note 4's resolution). This step already exists and already runs in every `cargo xtask ci`; AC-016 costs this project a new harness file, not a new gate step — which is the whole point of citing this pattern instead of inventing a wasm-specific CI job. |

### Notes

**Test mix, summarised by tier.**

- **Static** — `cargo fmt --check`, `cargo clippy --workspace --all-targets
  --all-features -D warnings`, `cargo hack check --workspace
  --feature-powerset --no-dev-deps` (now covering `happenstance-core`'s new
  `conformance` feature and, if AC-A04's arm is taken, `unstable-projection` —
  Architecture brief Note 4's "eight combinations to thirty-two"), `cargo hack
  check -p happenstance-core -p happenstance-neon -p happenstance-testkit
  --target wasm32-unknown-unknown --feature-powerset --no-dev-deps`
  (`xtask/src/main.rs:558-591`), `cargo deny`, the nightly `--cfg docsrs`
  build, `cargo package --list`, `redkiln validate --kb`. All read/lint/build
  the tree without exercising `ProjectionStore` logic; AC-008, AC-013 and
  AC-014 lean on this tier because the claim they prove is structural
  (a signature shape, a compiled skeleton, a cross-reference table).
- **Unit** — `cargo test --workspace --all-features` as invoked inside `cargo
  xtask ci`'s `tests` step, covering: the projection registry's own
  `no_orphan_rules` sibling (AC-001); the mutant registry's three exactness
  meta-tests (AC-002, AC-003); direct `RuleOutcome` assertions over a
  capability-declined rule (AC-005); the probe round-trip test (AC-009);
  `cargo test --doc` for `MemoryProjectionStore`'s doctest (AC-012). These are
  ordinary in-process `#[test]` functions; they check registries and return
  values, not a store's durability behaviour.
- **Integration** — every `projection_store_conformance!` invocation actually
  driving a `ProjectionStore` impl (`MemoryProjectionStore`, the CF-5
  buffering variant, `CheckpointOnlyStore` and its hostile siblings) through
  `begin` → probe-write → `commit`/`rollback`/`reset` → probe-read /
  checkpoint-read, exercised inside the same `tests` step. This is the tier
  that proves AC-002 (the mutant is caught by real execution, not by
  inspection), AC-004 (two shapes actually run), AC-010/AC-011 (behaviour
  after `Drop` and after `reset` is observed, not asserted structurally), and
  half of AC-016 once the wasm harness exists.
- **E2E** — `cargo xtask ci` run whole, from a clean workspace state, is the
  proof artefact DoD 8 requires as a *precondition* rather than a substitute
  for looking at items 1 and 2 (`_intake-brief.md` Proof artefact,
  `project.md` DoD 8). AC-004, AC-015 and AC-016 are each, in part, claims
  about what one such run showed rather than claims a single isolated test
  proves.

**Merge-gate commands.** Two grains, per `CLAUDE.md` "Commands" and this
project's own `terminal: false` frontmatter (`project.md` frontmatter,
`stage: storymap`):

```console
# story / slice grain, during implementation
cargo xtask affected --base main        # the story grain: only what this diff could break
cargo xtask ci --fast                   # REQUIRED only; the bar a non-terminal project meets

# project boundary grain — this project's own DoD overrides the non-terminal
# default, because AC-013/AC-014/AC-016 each name a mandatory-but-not-`--fast`
# step directly (the wasm32 conformance-harness check, spec-trace)
cargo xtask ci                          # project.md DoD 4: "green on the whole workspace,
                                         # including the mandatory wasm32 steps and spec-trace"
cargo xtask spec-trace                  # AC-014, run standalone during iteration
redkiln validate --kb                   # AC-008
redkiln validate                        # backlog frontmatter conformance on this project's own item
```

The distinction matters here specifically because `--fast` is defined
elsewhere as *not* running the mandatory wasm32 steps or `spec-trace`
(`CLAUDE.md` "Commands"), and two of this project's sixteen ACs (AC-014,
AC-016) are proven by exactly those two steps. A story-grain `--fast` pass is
real progress but is not evidence for AC-014 or AC-016; only the full run at
the project boundary is, and `project.md` DoD 4 says so in the same words used
here rather than a paraphrase.

**Fixtures / seams.**

- **`ProjectionProbe`** (`happenstance-core`, behind `conformance` —
  Architecture brief AC-A02) is the one seam every Integration-tier row above
  depends on: without `probe_write`/`probe_read`/`probe_read_through`/
  `probe_delete_all`, no rule can observe a read model at all, and the whole
  suite degenerates to a checkpoint-only test a broken store passes
  (Architecture brief Note 6, "the mutant that makes the suite
  non-decorative"). It is not mocked — it is the real generic write path,
  compiled once per adapter.
- **A new `ProjectionFixture` trait** (testkit `contract.rs`, mirroring
  `Fixture` — Architecture brief Note 5) is what every row above is actually
  parameterised over. It reuses `Capability` and `RuleOutcome` unchanged and
  must not carry a borrowing GAT (`contract.rs:97-111`, the rustc ICE this
  repository already minimised) — `MemoryFixture`'s owned-handle pattern
  (`fixtures.rs:243-292`) is what to copy, not invent.
- **Reused unmodified:** `Capability`, `RuleOutcome`
  (`contract.rs:355-537`), the three emitters (`registry.rs:222-295`), and
  the `Declared` mutant-registry shape (`mutation_coverage.rs:141-186`). None
  of these is forked for the projection port; a fork would give an adapter
  author two skip vocabularies or two mutant-registry shapes to learn
  (Architecture brief Note 1, "Sibling contracts this wires to, by name").
- **Nothing real is mocked.** `MemoryProjectionStore`, the CF-5 buffering
  variant and the hostile mutant stores are all real, if minimal, trait
  implementations — never a test double standing in for one. This mirrors
  CLAUDE.md's "the rule that matters": a rule is proven against a store that
  can actually fail it, not against a stub configured to fail on cue.
  `happenstance-postgres`/`happenstance-ladybug`/`happenstance-sqlite` are
  explicitly **not** fixtures here — their `todo!()` bodies mean they cannot
  run the suite, so AC-013's proof is Static (compiles) and never Integration
  (runs), and no story should attempt to flesh out a skeleton body to make it
  Integration-provable; that adapter's own project owns doing so.
- **Not built here:** a real SQL or Durable Object fixture. Architecture
  brief AC-A03 already forecloses reaching for `happenstance-sqlite`'s
  skeleton as the second batch shape; this brief's Integration tier is
  correspondingly scoped to `MemoryProjectionStore` and one testkit-internal
  variant, never a downstream adapter's own store.

**What no test tier proves, and why that is not a gap.** AC-006, AC-007's
narrower arm, and AC-015 are each, in whole or part, a written decision rather
than a pass/fail assertion — resolving DT-3 or DT-8 is choosing *which*
documented behaviour the suite commits to, and a test can check that the
chosen behaviour holds (rows above already do) but cannot check that the
right behaviour was chosen. These are proven by the design-stage review
`project.md` DoD 7 names, not by a story adding a test that would only assert
its own author's opinion back at itself.
