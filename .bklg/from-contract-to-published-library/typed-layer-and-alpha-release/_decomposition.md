# Briefs — The typed layer, the worked example, and 0.2.0-alpha.1 (HS-P0011)

Companion to [`project.md`](project.md). Holds this project's warranted briefs —
architecture, ux, testing, deployment
(`.bklg/from-contract-to-published-library/_decomposition.md`, *Warranted briefs*,
`typed-layer-and-alpha-release` row). Grounding for all four is in
[`_grounding.md`](_grounding.md); the story map that must cover them is
[`_storymap.md`](_storymap.md).

---

## UX brief

### Intent

**Scope this brief covers.** The developer-facing surface a `cargo add
happenstance` user meets in their first hour: the vocabulary they write against
(`DomainEvent`, `DecisionModel`, composition, `Codec`, the command loop, the
application-facing `Projection` trait and runner), what the compiler and the
runtime say back to them, the given/when/then DSL they test with, and the two
rendered artefacts this project ships — the rustdoc/docs.rs page for
`happenstance` and the terminal transcript of `cargo run -p course-subscriptions`.
The initiative's own decomposition names this **"the primary `ux` brief in the
portfolio"** because *"this is the API a `cargo add` user meets"*
(`.bklg/from-contract-to-published-library/_decomposition.md:274`).

**There is no screen.** `userFacing` is false for this initiative
(`.bklg/from-contract-to-published-library/_decomposition.md:266`) and no
`design.capture` key exists in `.redkiln/config.yaml`, so the perceptual review is
a declared skip, not a silent pass (`CLAUDE.md`, *Where the work lives*). The
"interface" here is a **type surface plus two text surfaces**. Every rule below is
written against that, not translated from a web idiom and left to mean nothing.

**Who this is for, and to what end.** The durable product layer is structurally
present and functionally empty — `.kb/product/README.md` and `.kb/design/README.md`
are template READMEs and **no `authority_tier: product` atom exists in this tree**
(`.bklg/from-contract-to-published-library/initiative.md:229-233`). This brief
therefore cites the initiative's own distillation, as the charter instructs, and
carries its qualification with it (all four personas rest on secondary evidence;
none has been directly observed —
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`,
*Risks*). Promotion is `closeout-and-durable-audience`'s (HS-P0019), not this
project's.

| Persona | What they are trying to do here | What they are afraid of |
| --- | --- | --- |
| **P1 — the application author** (`…/personas-and-journeys.md`, *Persona 1*) | Model a consistency boundary once, against a contract, and defer *which database* to a decision they can revisit | Being the one who discovers a contract defect in production, after they have already built on it |
| **P4 — the evaluator** (`…/personas-and-journeys.md`, *Persona 4*) | Decide in one bounded sitting whether the claims are real, from the registry page and the docs alone | Adopting on a claim that turns out true for one storage shape only |
| **P2 — the adapter author** (`…/personas-and-journeys.md`, *Persona 2*) | Not be disturbed. They pin `happenstance-core`, which this project must not grow | A port frozen against one storage shape |

The journey this project moves is **"Choose a contract before a database"** — the
application author's first hour, from modelling a boundary to running it, without
the database being decided first
(`.bklg/from-contract-to-published-library/initiative.md:241-244`). Two beats of
that journey are this project's whole UX mandate:

- **Beat 2 today** forces *"pick a database"* and *"does the domain model work"*
  into one decision. `MemoryEventStore` plus the DSL is what separates them
  (`project.md` AC-009; `RUNBOOK.md:3977-3979`).
- **Beat 4 today** leaves the reader on a facade nothing has exercised —
  `crates/happenstance/src/lib.rs:75` is `pub use happenstance_core::*;`, confirmed
  by reading the file (`_grounding.md`, *What already exists*). The worked example
  and the recorded defect log are what close it (AC-003, AC-012).

P4's beat is one-shot and time-boxed: *"they do not get a second pass"*
(`…/personas-and-journeys.md`, *Persona 4*). This project owns only the part of
that surface it authors — the crate's rustdoc, its README `## Stability` section
and the example transcript. **Registry presentation, positioning and the compliance
claim are `publication-and-positioning`'s (HS-P0016) `ux` brief**
(`project.md`, *Out of scope*); do not decide them here.

**The UX decision this brief hands to `_design.md`, unresolved on purpose.** DT-2
— *how much a caller must state before their consistency boundary is checked*:
minimal ceremony with more caught later, or explicit declaration with more caught
at build time (`.bklg/from-contract-to-published-library/initiative.md:421`). It is
error timing versus first-hour cost, and the weight on the second term is that the
repository owner — and by evidence the audience — is **new to idiomatic Rust**
(`CLAUDE.md`, *Who you are working with*; `project.md` risk table, last row).
AC-014 requires it decided in `_design.md` with the trade stated. This brief does
not pre-empt it; it states the bar the answer is judged against (AC-U01 below).

### Acceptance Criteria

Each is observable, and each names the instrument. They are UX-grain refinements
of `project.md`'s AC-001…AC-016 and add no scope.

#### First contact and ceremony

- **AC-U01 — The first-hour cost is a measured artefact, not an opinion.** The
  `## The doctest` section of `_design.md` contains the *complete* smallest program
  that defines an event, folds a decision model and appends under a condition —
  compiling, with no elisions and no `# ` hidden lines carrying required ceremony.
  DT-2 is resolved against **that** text: the doctest is the artefact the trade is
  judged on (`project.md` risk table, *The audience is new to idiomatic Rust*;
  `.redkiln/templates/_design.md`). A resolution that cannot show its own first
  program has not resolved DT-2.
- **AC-U02 — Nothing in the first program requires reading a second page.** Every
  type named in that doctest resolves from `happenstance`'s crate root by intra-doc
  link, and none of them requires the reader to open `happenstance-core`'s docs to
  learn what to pass. Adapter authors are pointed *downward* explicitly, as the
  facade already does (`crates/happenstance/src/lib.rs:69-72`); application authors
  are never pointed downward to complete a first task.
- **AC-U03 — The module doc stops promising and starts linking.** The five
  "Planned, and specified in `spec/SPECIFICATION.md`" bullets at
  `crates/happenstance/src/lib.rs:35-52` are replaced by intra-doc links to the
  real items, in the same order and with the same discriminator prose
  (`crates/happenstance/src/lib.rs:29-33`, *the discriminator is encoding*). A
  reader who lands on the crate root after the alpha must not meet a roadmap.

#### The vocabulary is composed, never re-invented

- **AC-U04 — One filter vocabulary.** A decision model's derived query, a
  projection's event nomination and the DSL's seeding all speak
  `happenstance_core::Query` (`crates/happenstance-core/src/lib.rs:116`). No second
  filter type, no string-matching helper, no `&[&str]` of event-type names appears
  in `happenstance`'s public surface (DR-05; ADR-0007).
- **AC-U05 — The contract's error vocabulary is what a caller matches on.** The
  command loop's retry decision is expressed through
  `AppendError::is_condition_violated()`, not a `happenstance`-local duplicate enum
  (`standards/rust/30-error-taxonomy.md` RS-30-1;
  `crates/happenstance-core/src/lib.rs:106`). Any error type this project *does*
  add carries the adapter's error as a type parameter with `#[source]`, never as a
  `String` (RS-30-2).
- **AC-U06 — A returned aggregate is a named `#[non_exhaustive]` struct, not a
  tuple.** Anything the command loop or the runner hands back that could grow a
  field is a named struct (`standards/rust/40-public-surface-and-evolution.md`
  RS-40-3, RS-13-4). A tuple freezes the arity and makes the next field a breaking
  change on a surface the alpha exists to keep movable.

#### What the compiler says, and where it points

- **AC-U07 — The diagnostic lands on the caller's code.** The AC-002 `trybuild`
  compile-fail case's `stderr` fixture points at the *user's* `match` arm in
  `examples/course-subscriptions/`, not into a macro body. Every path any macro
  expansion emits is `$crate::`-qualified (`standards/rust/41-declarative-macros.md`
  RS-41-1), and the composition macro's arms are ordered most-literal-first so the
  arm that matches is the one the caller meant (RS-41-3). A guarantee whose
  diagnostic names a file the user did not write is a guarantee they cannot act on.
- **AC-U08 — The error message carries the value, not a category.** Any hand-written
  `Display` in this project renders the carried field, and binds a limit constant as
  an explicit named format argument rather than inlining the number
  (`standards/rust/30-error-taxonomy.md` RS-30-4, RS-30-5). *"capacity exceeded"*
  is a category; *"capacity 2 already filled"* is actionable.
- **AC-U09 — A decision cannot be silently dropped.** Any value representing "the
  events this decision would append" is `#[must_use]` with the consequence written
  into the message (`standards/rust/41-declarative-macros.md` RS-41-5). Building a
  decision and discarding it must not compile quietly — for P1, a silently dropped
  append is indistinguishable from a successful one until production.

#### Interaction-quality invariants (first-class, testable)

These are the ones that separate humane from merely functional. Each is stated in
the medium this project actually has.

- **AC-U10 — In place, not a context jump.** Every failure a caller can act on is
  actionable *at the call site* without re-reading the log or opening a second
  document: `ConditionViolated` carries what was violated
  (`crates/happenstance-core/src/lib.rs:106`), and the command loop's retry policy
  is stated in the rustdoc of the loop itself, beside the distinction from the
  retry-safety property of a *verbatim* resubmission (`RUNBOOK.md:4015-4020`). A
  doc comment that says "see the specification" for the policy fails this.
- **AC-U11 — Non-occlusion: a filter must not hide what it filters.** A
  `DecisionModel`'s derived `Query` is **inspectable** — reachable as a value and
  assertable in a test, not private machinery inside `read`. When the given/when/then
  DSL's assertion fails, the failure message names the events that were *seeded but
  not selected* by the model's query, because that set is exactly where a
  fold/query divergence would be invisible. A DSL that reports only "expected X, got
  nothing" hides its own filter and re-opens the hazard ADR-0020 exists to close
  (AC-001; `examples/course-subscriptions/src/main.rs:114-173`).
- **AC-U12 — Reversibility: everything before the append is pure.** Defining events,
  folding a model, composing models and encoding payloads perform no I/O and mutate
  no store, so a caller can build, inspect and discard without consequence. The
  append is the single irreversible act, and nothing performs one as a side effect
  of construction. Feature-wise the same rule holds: **a feature adds**
  (`standards/rust/51-features-and-no-std.md` RS-51-1) — if PS-3 puts the runner
  behind `unstable-projection` (`project.md` risk table; `RUNBOOK.md:3924-3928`),
  turning that feature *off* must leave every unrelated item compiling, and
  `_design.md` states the gating explicitly rather than inheriting it.
- **AC-U13 — Preserved state across retry.** The command loop's retry on
  `ConditionViolated` re-reads and re-decides from fresh state, and **never**
  silently reuses a stale fold or discards the caller's command input. The retry is
  bounded and the bound is visible to the caller — a loop whose only exit is success
  is a hang with better manners. `FaultyStore<S>` is the instrument that proves it
  (AC-009), and the caller's own retry loop must be testable against it *without*
  a database.
- **AC-U14 — Reachability without a search engine.** Every public item this project
  adds is reachable from the crate root's module doc by intra-doc link; no
  intra-doc link resolves in only some feature configurations
  (`standards/rust/70-rustdoc-obligations.md` RS-70-2); and any feature-gated item
  renders its gate on docs.rs via `cfg_attr(docsrs, doc_cfg)` with the
  configuration declared in the manifest (RS-70-4, RS-51-5). A reader must be able
  to tell what a feature turns on without opening `Cargo.toml` — P4 has one sitting.
- **AC-U15 — The alternative that lost is named once, where the reader is.** Every
  unusual construct this project ships carries the rejected alternative in its doc
  comment, once (`standards/rust/70-rustdoc-obligations.md` RS-70-5;
  `CLAUDE.md`, *Who you are working with*: *"when a construct is unusual, say what
  the alternative was and why it lost"*). This is the audience-specific
  accessibility requirement — the reader is fluent in the domain and new to the
  idiom.

#### The two rendered surfaces

- **AC-U16 — The terminal transcript signals by text, never by colour or motion.**
  `cargo run -p course-subscriptions` keeps its text-first structure — section
  markers and an explicit `rejected: {err}` prefix, as today at
  `examples/course-subscriptions/src/main.rs:37-71` — so every outcome is legible
  from the words alone. **Colour is never alone**, which here is satisfied the
  strongest way available: no colour at all. There is no ANSI-colour dependency
  anywhere in the workspace today (`rg` for `termcolor`/`owo-colors`/`anstream`/
  `NO_COLOR` across the tree returns nothing) and this project introduces none; if
  one is ever introduced it must honour `NO_COLOR` and must not be the sole carrier
  of any state. **Reduced motion** is likewise structural: no spinner, no progress
  animation, no in-place line rewriting — including in the polling projection runner,
  whose cost is recorded as a number (AC-010) rather than performed as an animation.
  **Keyboard reachability** for a CLI means no interactive prompt and no TTY
  requirement: the example runs to completion under `cargo run` with stdout piped,
  which is how the gate runs it.
- **AC-U17 — The rendered docs meet the WCAG AA floor for the things we author.**
  Heading levels in `crates/happenstance/README.md` and in the crate's rustdoc
  descend without skipping (the README's current `#`/`##` structure already does);
  every fenced block declares its language so it is announced as code and
  highlighted; any image or badge carries meaningful alternative text and no
  meaning is carried by an image alone; link text is self-describing rather than
  "here" or a bare URL. Contrast and focus order belong to the rustdoc and crates.io
  themes, which this project does not author and must not fight with bespoke inline
  HTML or hand-rolled styling.
- **AC-U18 — The stability posture is legible before the reader commits.** The
  README's `## Stability` section names the phase at which the API stops moving, in
  the reader's own terms, and sits above the first code block a reader will copy;
  `CHANGELOG.md` exists; the yank policy for superseded alphas is stated
  (`project.md` AC-011; `RUNBOOK.md:4149-4155`). For P4 this is the difference
  between a bounded look ending in *adopt* and one ending in *cannot tell*.

#### Consistency with what already exists

- **AC-U19 — Extend the established doc pattern, do not replace it.** The
  doctest-gated README (`#![cfg_attr(doctest, doc = include_str!("../README.md"))]`,
  `crates/happenstance/src/lib.rs:10`, with its D10 rationale comment) stays, so
  every README example a reader copies is compiled. Examples using `?` close with
  `# Ok::<(), E>(())` and `.await` examples get a hidden runtime
  (`standards/rust/62-doctests-and-harnesses.md` RS-62-2), and every `compile_fail`
  fence is paired with a compiling one and its error code is not trusted (RS-62-1).
- **AC-U20 — A declined capability says why, rather than disappearing.** Where the
  DSL or a testkit type cannot offer something, it declines by an associated
  `const` whose constructor rejects an empty reason
  (`standards/rust/40-public-surface-and-evolution.md` RS-40-5) — the
  `event_store_conformance!` capability-declension pattern
  (`crates/happenstance-testkit/src/`, `MemoryFixture` in `fixtures.rs`; `CLAUDE.md`,
  *The rule that matters*). Silence is the failure mode this repository has already
  paid for once.

### Notes

**On translating a web-UX brief into a library.** The template's primitives asked
for — the token/primitive layer the implementer composes rather than hand-rolls —
exist here and are named, they are simply not CSS:

| Web idiom | The real layer in this repo | Path |
| --- | --- | --- |
| Design tokens | The contract crate's exported vocabulary: `Query`, `QueryItem`, `ReadOptions`, `Tag`, `Tags`, `EventType`, `AppendCondition`, `Guard`, `AppendError`, `ConditionViolated`, `ProjectionId` | `crates/happenstance-core/src/lib.rs:105-118` |
| Component primitives | The Rust constitution's 27 atoms — each rule carries a compiled example and a named wrong implementation | `standards/rust/README.md` (router; load one to three, never the corpus) |
| Layout/shell conventions | The facade's module-doc structure and the doctest-gated README | `crates/happenstance/src/lib.rs:1-72` |
| Reference implementation | `MemoryFixture` and `event_store_conformance!` | `crates/happenstance-testkit/src/fixtures.rs` |
| The one "screen" | The rustdoc page and the example transcript | `crates/happenstance/README.md`, `examples/course-subscriptions/src/main.rs` |

Hand-rolling a parallel vocabulary — a second filter type, a local retryable-error
enum, a bespoke tuple return — is the exact analogue of bespoke CSS, and
`standards/rust/40-*` and `30-*` are where it is refused.

**The atoms to load, and only these.** `70-rustdoc-obligations`,
`62-doctests-and-harnesses`, `30-error-taxonomy`,
`40-public-surface-and-evolution`, `41-declarative-macros`,
`51-features-and-no-std`. The router's own instruction is one to three at a time
(`standards/rust/README.md`); this brief spans several surfaces, so pull per task,
not all at once.

**Provisional, and flagged rather than assumed.** Three items are live and must not
be silently resolved by this brief or by a story written from it — all three are
`_grounding.md`, *Tensions*:

1. **The `Projection` trait's concrete shape is provisional on HS-P0010's freeze
   landing first.** `crates/happenstance-core/src/projection.rs`'s own module doc
   still says the port is *not yet frozen*, and `projection-store-freeze` sits at
   `stage: storymap`, the same stage as this project. AC-U04's "one filter
   vocabulary" holds regardless (it is ADR-0007's, not HS-P0010's); the trait's
   batch-facing signature does not.
2. **`trybuild` has no home yet.** It is absent from `Cargo.toml` and `Cargo.lock`,
   and its adoption is HS-P0010's decision per `project.md`'s risk table. AC-U07
   describes the diagnostic quality bar the instrument must meet; it does not
   assume the instrument exists. Request it as an explicit input to HS-P0010 before
   this project's design stage.
3. **PS-18 cannot be counted at design time.** Its clause text
   (`spec/SPECIFICATION.md:5200`) assigns evaluation to *the exit of the
   projection-port phase*, and no SQLite adapter exists in the tree yet. Nothing in
   this brief depends on that count; the architecture brief carries the
   reconciliation (`RUNBOOK.md:4070-4077`).

**Deliberately not decided here.** DT-2's answer (AC-014, `_design.md`); the
runner's feature gating if PS-3 lands (`_design.md`); the crate landing page,
positioning and the compliance claim (HS-P0016's `ux` brief, `project.md`, *Out of
scope*); persona promotion into `.kb/product/` (HS-P0019). DT-1 — *which claim
leads on first contact* (`.bklg/from-contract-to-published-library/initiative.md:420`)
— is HS-P0016's; this project must not settle it by accident through the wording of
the README's opening line, and where the two touch, the `## Stability` section
(AC-U18) is this project's only claim on that page.

**What would make this brief wrong.** If the resolved DT-2 answer produces a first
program whose ceremony exceeds its domain logic, AC-U01 has been satisfied in form
and failed in substance — and that is precisely the condition AC-013 tests for
`happenstance-macros` (*"if the rewritten example carries more mapping boilerplate
than domain logic, the derive is in scope for 0.1"*, `RUNBOOK.md:524`). The two
criteria are the same measurement read at two altitudes; answer them together.

---

## Architecture brief

### Intent

**The single child this brief scopes:** HS-P0011's technical approach — where the
typed layer's code lands, which crate seams it may cross, which contracts it
consumes *unchanged*, and the composition roots each capability must mount into so
the result is an integrated library rather than a set of modules nothing calls.

It is a planning artefact. It names contracts, seams, mount points and the
decisions an implementer inherits; it writes no production code and prescribes no
signature that `_design.md` owns (`.redkiln/templates/_design.md` is where the
public API surface is enumerated and human-approved — DoD 5).

Three things make this project architecturally unlike its siblings, and most of
what follows is a consequence of one of them:

1. **It is the first consumer.** Everything below `happenstance` is `[FROZEN]` and
   already compiles; this project's job is to *use* it and to record what using it
   reveals (AC-012). No architectural liberty here is worth an unrecorded contract
   edit.
2. **It is additive to a crate that is currently a glob re-export.**
   `crates/happenstance/src/lib.rs` is 76 lines and line 75 is
   `pub use happenstance_core::*;`. There is no internal structure to respect and
   no migration to perform — which means every structural mistake made here is one
   a `cargo add` user meets first.
3. **It publishes.** `0.2.0-alpha.1` makes this the first project whose output a
   stranger can resolve, so the seam between "public surface" and "implementation
   detail" stops being notional at this project's exit.

### Acceptance Criteria

Two groups. The first is architecture-grain and structural — obligations
`project.md` does not state outright but which its ACs cannot be met without. The
second maps each project AC to the seam it lands on and the instrument that
observes it. Neither adds scope.

#### Architecture-grain criteria

- **AC-A01 — The glob re-export survives, and nothing shadows a core name.**
  `pub use happenstance_core::*;` (`crates/happenstance/src/lib.rs:75`) stays, and
  every new item is added *beside* it. The crate's own module doc makes this a
  promise to readers — *"Everything the contract crate exports is available here
  under the same paths, so nothing has to be rewritten when the typed layer lands"*
  (`crates/happenstance/src/lib.rs:53-56`). A new `happenstance::Query` or
  `happenstance::EventStore` that is not the contract's is a silent breaking change
  to a published facade and an ambiguity for every existing doctest.
- **AC-A02 — `happenstance-core` is not edited to make the typed layer
  convenient.** This project's diff touches `crates/happenstance/`,
  `crates/happenstance-testkit/`, `examples/course-subscriptions/`, `xtask/`,
  `spec/SPECIFICATION.md`, `CHANGELOG.md` and `.kb/`. A change under
  `crates/happenstance-core/src/**` is admissible only as the *recorded* outcome of
  AC-012's route — a defect written down with its clause ID and routed to a
  decision record — never as a convenience edit discovered mid-implementation. The
  ES-\* clauses are `[FROZEN]`, and a frozen clause is amended by an ADR, not by a
  line edit (`project.md`, *Out of scope*; `_intake-brief.md`, *Clauses*).
- **AC-A03 — The proof artefact is registered in the gate, not merely present in a
  `tests/` directory.** The compile-fail target is a row in
  `xtask/src/proof.rs`'s `ARTEFACTS` (`xtask/src/proof.rs:133`; row shape at
  `:57-70`), naming the tests the clause cites. The gate step *"each phase's proof
  artefacts"* (`xtask/src/main.rs:178`) runs `cargo xtask proof-artefact` rather
  than `cargo test` for exactly this reason, and says so in its own comment:
  `cargo test` exits 0 on `running 0 tests`, *"so an emptied file passes a step
  that a deleted one fails."* An unregistered instrument is the decorative artefact
  this repository has already paid for once (`RUNBOOK.md:3614-3627`).
- **AC-A04 — Every feature this project adds is forwarded, and none changes what
  `default-features = false` means.** `crates/happenstance/Cargo.toml`'s dependency
  on `happenstance-core` deliberately names **no** features because every one is
  forwarded in its own `[features]` block; the manifest comment states the failure
  mode it avoids — the facade and the contract crate disagreeing about the meaning
  of `default-features = false`, *"which is the kind of difference nobody discovers
  until a `no_std` build fails three crates away."* Codec features (and, if PS-3
  fires, an `unstable-projection` forwarding) land in that block under the same
  rule, and a feature only ever **adds**
  (`standards/rust/51-features-and-no-std.md`).
- **AC-A05 — The projection runner streams; only the decision path buffers.** The
  command loop may use `read_decision_model`, which collects into a `Vec`
  (`crates/happenstance-core/src/store.rs:321`) — DCB queries are narrow by
  construction. The runner must **not**: `EventStore::read` returns the stream at
  the top level precisely so a million-event replay is not buffered
  (`crates/happenstance-core/src/store.rs:119` and its doc). A runner that calls
  `collect` defeats the reason the port has the shape ADR-0001/ADR-0008 fought for,
  and makes E2E-25's chunked rebuild unwritable.
- **AC-A06 — The typed layer's `wasm32` claim is stated, either way.** AC-015 asks
  that *"all four `wasm32` steps in `cargo xtask ci` are green"* — and none of them
  compiles this crate. The four are `happenstance-core`
  (`xtask/src/main.rs:203`), the conformance harnesses (`:231`),
  `happenstance-cloudflare` (`:252`) and `happenstance-neon` (`:271`), selected by
  name at `xtask/src/main.rs:784`; the optional `wasm32` feature powerset covers
  `happenstance-core`, `happenstance-neon` and `happenstance-testkit`
  (`xtask/src/main.rs:564-590`). So AC-015 is satisfiable without the typed layer
  ever being compiled for the edge target it exists to support. Either a fifth step
  is added **by name** to `REQUIRED` and to `wasm_steps()`, or `_design.md` records
  that `happenstance` makes no `wasm32` claim at the alpha, and why. Silence here
  is the failure this criterion exists to prevent, not a passing default.

#### Project ACs, by seam and instrument

| AC | Seam it lands on | Architectural obligation, and what observes it |
| --- | --- | --- |
| **AC-001** fold and query cannot disagree | `crates/happenstance/` (new decision module) | `DecisionModel::apply` takes `Self::Event`, a domain enum, so the `match` is exhaustive; `query()` is *derived* from `EVENT_TYPES` + tag constraints. `EventType::from_static` is already `const` (`crates/happenstance-core/src/event.rs:108`), so `const EVENT_TYPES: &'static [EventType]` needs no derive and no macro today. Observed by AC-002's instrument and by the rewritten example compiling. |
| **AC-002** compiler protection, checked | `xtask/src/proof.rs` + a compile-fail target | AC-A03. The negative control is the discriminator — removing the protection must make the case *fail*. Blocked on the `trybuild` input; see *Tensions*. |
| **AC-003** worked example on the typed layer | `examples/course-subscriptions/src/main.rs`, `examples/course-subscriptions/Cargo.toml` | The example's dependency moves from `happenstance-core` to `happenstance` (its `use` block at `examples/course-subscriptions/src/main.rs:24-27` is its whole surface today). `main`'s observable steps (`:37-79`) survive verbatim — they *are* the behaviour AC-003 asserts. `parse_capacity` (`:231`) and the `format!("…").into_bytes()` payload (`:96-99`) are deleted, not wrapped. |
| **AC-004** boundaries compose | new composition macro in `crates/happenstance/` | A declarative macro over tuple arities (`standards/rust/41-declarative-macros.md`), OR-ing each model's query fragment. Mount: the example's `subscribe` case, already a two-item query at `examples/course-subscriptions/src/main.rs:114-125`. |
| **AC-005** typed, evolvable payloads | new codec module + `crates/happenstance/Cargo.toml` `[features]` | JSON default; CBOR/postcard behind features (AC-A04). The codec **tag's location** is contract-adjacent, not a free choice — see *The codec tag has two possible homes*. ADR-0021 records it. |
| **AC-006** projection over decoded events | `crates/happenstance/` runner over `crates/happenstance-core/src/projection.rs` | The runner opens the adapter's batch (`begin`, `:117`), applies decoded events into it, and commits batch + checkpoint in **one** call (`commit`, `:126`). It never calls an `apply()`/`set_checkpoint()` pair — the port deliberately does not offer one (`crates/happenstance-core/src/projection.rs:20-26`). Shape provisional on HS-P0010; see *Tensions*. |
| **AC-007** PS-33 evaluated | `spec/SPECIFICATION.md:5529`, `.kb/decisions/` | Answered by **counting callers of the core checkpoint pump**. Note what the count is over: at HEAD `crates/happenstance-core/src/projection.rs` holds the *port*, `ProjectionId` and nothing that runs — there is no pump *function* in the contract crate to have callers. The verdict is therefore a judgement about where the checkpoint invariant lives, with two admissible outcomes: name the independent caller, or write the superseding ADR that collapses the pump upward (`references/adr/0007-projection-runner-decodes.md`; `RUNBOOK.md:408-413`). |
| **AC-008** the three "does anyone call this?" clauses | `spec/SPECIFICATION.md:5200`, `:5411`, `:5462` | Settled by counting, with the runbook's own escape hatch: *"name the callers, or promote the clause to a documented exclusion"* (`RUNBOOK.md:4070-4077`). PS-18 is not evaluable in this tree; see *Tensions*. |
| **AC-009** test a decision, test misbehaviour | `crates/happenstance-testkit/src/` (`FaultyStore<S>`, `GappyMemoryStore`); DSL in `crates/happenstance/` | The DSL is application-facing and decodes, so it belongs above; the misbehaving stores belong in the testkit, which carries its own version for exactly this class of change (`crates/happenstance-testkit/Cargo.toml:14`, CF-32, asserted by a gate step at `xtask/src/main.rs:425`). `FaultyStore<S>` hits a coherence rule — see *The wrapper store is two types*. |
| **AC-010** polling cost measured | `experiments/` | *"measurements. reproducible, and not in the gate"* (`CLAUDE.md`, repository map). ES-32 (`spec/SPECIFICATION.md:4021`) forbids a tail seam at 0.1, so N views is N independent reads. The number is an artefact with its conditions recorded, never a gate assertion (CF-34: performance is measured by a separate harness and is not the bar). |
| **AC-011** `0.2.0-alpha.1` on the registry | `crates/happenstance/README.md`, root `CHANGELOG.md`, the three publishable manifests | `CHANGELOG.md` already exists with an `## [Unreleased]` section — an edit, not a new file. The README's `## Stability` section is compiled as a doctest through `crates/happenstance/src/lib.rs:10`, so it must not carry an uncompilable fence. `cargo xtask ci`'s packaging step (`xtask/src/main.rs:519`) already asserts each publishable crate ships both licences and a README. Release mechanics belong to the `deployment` brief. |
| **AC-012** contract defects recorded | `.kb/_intake/` → `/redkiln:kb-ingest`; the project's closeout | AC-A02's route. Each entry names the clause ID and its routing; incidental bugs route to the `support` initiative (`.redkiln/config.yaml`). |
| **AC-013** `happenstance-macros` answered | the project's closeout record | The criterion is a measurement over the rewritten example (`RUNBOOK.md:524`), so it cannot be evaluated before AC-003 lands. If the answer is *in*, that is a **new crate** under `crates/` and a new workspace member — a scope change for the runbook to take, not an implementer. |
| **AC-014** DT-2 resolved | `_design.md` | Architecturally DT-2 reduces to one concrete question this brief poses and must not answer — see *The one signature question DT-2 actually is*. |
| **AC-015** the edge flavour survives | generic bounds throughout `crates/happenstance/` | Bind `EventStore`, not `SendEventStore` (`crates/happenstance-core/src/store.rs:80-92` is the pattern, written as a doctest). Import one flavour name per module. Where a runner holds a stream across an await inside a spawned task, state the bound the way `spawns_from_generic` does — `S: SendEventStore + Send + Sync + 'static` (`crates/happenstance-core/src/memory.rs:666`, with per-bound reasoning at `:643-680`). Never `#[async_trait]`, here or in the testkit. Plus AC-A06. |
| **AC-016** ADR-0020/0021 written first | `.kb/_intake/` → `/redkiln:kb-ingest` → `.kb/decisions/` | Atoms are authored by the ingest path, never by hand — hand-writing produces *"the directory layout of the process without the process"*, which is why the first attempt was reverted at `0269720` (`CLAUDE.md`, *Where the work lives*). An accepted atom is immutable (`.kb/decisions/README.md`): a correction after the code is written is a **new** superseding atom. |

### Notes

#### Where the code lands

| Concern | Crate / path | Why there, and not elsewhere |
| --- | --- | --- |
| `DomainEvent`, `DecisionModel`, composition, `Codec`, command loop, `Projection` + runner, given/when/then DSL | `crates/happenstance/src/` | ADR-0006 gives the bare name to the typed layer, and `crates/happenstance/src/lib.rs:29-33` already states the discriminator — **encoding**. *"Anything that knows how a payload is shaped belongs here."* |
| `FaultyStore<S>`, `GappyMemoryStore` | `crates/happenstance-testkit/src/` | They are instruments for other people's tests, and the testkit is the crate with an independent version for exactly that reason (CF-32; `xtask/src/main.rs:425`). Putting them in `happenstance` couples an application's dependency graph to a test double. |
| The polling-cost measurement | `experiments/` | Measurements are reproducible and deliberately outside the gate (`CLAUDE.md`). |
| The clause verdicts | `spec/SPECIFICATION.md` | See *Changing a clause marker is a three-part edit*. |
| ADR-0020, ADR-0021, routed defect entries | `.kb/_intake/`, then `.kb/decisions/` | The ingest path (AC-016 row above). |

Module names inside `crates/happenstance/src/` are the implementer's and
`_design.md`'s to choose. Two structural constraints are not: the crate root keeps
its glob re-export (AC-A01), and the module doc's five "Planned" bullets
(`crates/happenstance/src/lib.rs:35-52`) are the render path for the new surface —
each bullet becomes an intra-doc link to a real item, in place, so a reader landing
on the crate root after the alpha does not meet a roadmap.

#### Composition roots — what each capability must mount into

A capability that compiles but is reachable from nothing is the failure this
section exists to prevent. Every row names a file that already exists.

1. **`crates/happenstance/src/lib.rs`** — the public surface. Every new item is
   `pub use`d at the crate root *and* linked from the module doc. This is the only
   render path the library has.
2. **`examples/course-subscriptions/src/main.rs`** — the running application. Its
   `main` (`:33-79`) is the DCB cycle AC-003 asserts, and its three handlers —
   `define_course` (`:86`), `subscribe` (`:113`), `unsubscribe` (`:177`) — plus the
   shared `commit` (`:207`) are the four call sites the typed layer must absorb.
   `commit`'s own doc comment already names its destination: *"this is the second
   half of every DCB command handler, and it is identical every time — which is
   exactly why it belongs in the typed layer."* Deleting that function *into* the
   library is the integration test for the command loop.
3. **`examples/course-subscriptions/Cargo.toml`** — the dependency must move to
   `happenstance`. An example still depending on `happenstance-core` proves nothing
   about the crate a user installs.
4. **`xtask/src/proof.rs::ARTEFACTS`** (`:133`) — the gate mount for AC-002.
5. **`xtask/src/main.rs::REQUIRED`** (`:105`) — any new gate step, including the
   `wasm32` step AC-A06 may require, goes here and nowhere else. `cargo xtask ci`
   is defined once and *is* what CI runs (`CLAUDE.md`, *Commands*); a step added to
   `.github/workflows/` instead is invisible locally and drifts.
6. **`crates/happenstance/Cargo.toml` `[features]`** — the feature mount (AC-A04).
7. **`crates/happenstance/README.md`** — `## Stability` lands between the existing
   `## Guarantees` (`:41`) and `## Design` (`:51`); the file is compiled as a
   doctest via `crates/happenstance/src/lib.rs:10`.
8. **`CHANGELOG.md`** — `## [Unreleased]` becomes the alpha's section.
9. **`crates/happenstance-testkit/src/lib.rs`** — the testkit root, where
   `FaultyStore`/`GappyMemoryStore` become reachable. `fixtures.rs` is the pattern
   for anything that also wants to be a conformance fixture — `MemoryFixture`
   (`:243`), its `Fixture` impl (`:270`), and the honest declension of a capability
   with a real reason (`:280`), against the trait at
   `crates/happenstance-testkit/src/contract.rs:120-173`.

#### The contracts this layer consumes unchanged — and five ways to get them wrong

The data flow is **`DecisionModel::query()` → `EventStore::read` → decode →
`apply` fold → decide → encode → `AppendCondition` → `EventStore::append` → on
`ConditionViolated`, re-read and re-decide.** Each item below is a property of the
frozen contract that a typed layer written from intuition gets wrong.

1. **`append`'s return value is not a sound `after`.** It returns the position of
   the last appended event, and the doc says in terms that it is *"not a sound
   `after` for a follow-up condition"* unless the caller has already read up to it
   (`crates/happenstance-core/src/store.rs:131-145`). The sound anchor comes from a
   read: `read_decision_model` returns the last position actually observed (`:321`),
   which is what `AppendCondition::after_opt` expects. A command loop that threads
   `append`'s return into the next condition silently excludes exactly the events a
   condition exists to catch.
2. **`ConditionViolated::conflicting_position` is a hint, not a promise.** It is
   `Option<SequencePosition>`, and an adapter with no interactive transaction
   reports `None` legitimately (`crates/happenstance-core/src/error.rs:135-147`). A
   retry loop that branches on `Some` works in-process and stops working against a
   remote store — which is to say it passes every test this project can write and
   fails against `happenstance-neon`.
3. **`ReadOptions::from` is inclusive.** Resuming a projection from its checkpoint
   means advancing past it first; the port's own doc says so
   (`crates/happenstance-core/src/projection.rs:104-106`). `SequencePosition::next`
   returns `Option` (`crates/happenstance-core/src/event.rs:272`), and the `None`
   arm is a real arm. An off-by-one here re-applies one event on every restart:
   invisible for an idempotent projection, corrupting for a counter.
4. **`head()` answers "am I caught up?" by comparison, never by subtraction.** Its
   doc forbids computing `head - checkpoint` as a lag
   (`crates/happenstance-core/src/store.rs:240`) because positions may have gaps —
   which is also why `GappyMemoryStore` exists (AC-009) and why the conformance
   suite may not assert literal positions (`CLAUDE.md`).
5. **A derived `Query` is built through fallible constructors.** `QueryItem::new`
   returns `Result<_, InvalidQuery>` (`crates/happenstance-core/src/query.rs:56`),
   and `Query::Items` is `#[non_exhaustive]`, so `Query::from_items` / `from_item`
   are the only ways in (`:150-199`). A `fn query(&self) -> Query` must therefore
   have decided where the fallibility went — see immediately below.

#### The one signature question DT-2 actually is

DT-2 — *how much must a caller state before their consistency boundary is checked*
— has a concrete technical form: **is `DecisionModel::query()` infallible, and if
so, where did the validation go?**

- **Infallible `query()`** requires the model's `Tags` to have been validated at
  construction, so the model carries validated `Tags` (or `const`
  `Tag::from_static` values, `crates/happenstance-core/src/tag.rs:112`) rather than
  `&str` pairs. More stated up front, errors at build time — DT-2's "explicit
  declaration" pole.
- **Fallible `query() -> Result<Query, _>`** lets a model hold raw strings and
  pushes the error to the first read. Less ceremony, later errors — the "minimal
  ceremony" pole.

Both are legitimate; the choice is `_design.md`'s (AC-014) and is judged against
the doctest `_design.md` requires. This brief insists only that the answer be
**one** of them: two constructors enforcing different rules is the defect
`crates/happenstance-core/src/projection.rs:47-61` already names in prose — *"two
constructors enforcing different rules is the defect that makes an invalid value
reachable through the weaker one."*

#### The codec tag has two possible homes, and they cost different things

AC-005 requires events to carry a codec tag. The frozen contract offers exactly two
places to put it, and ADR-0021 owns the choice:

- **In `Event::metadata`** (`crates/happenstance-core/src/event.rs:379`, read at
  `:400`, `Option<&Bytes>`). Opaque to every store, which is what VT-3 requires:
  *"the contract layer, a store adapter, and a peer MUST NOT parse `data` or
  `metadata`"* (`spec/SPECIFICATION.md:629-637`). Cost: every event must carry it,
  and `metadata` is the same field an application wants for causation/correlation —
  so the typed layer would be defining a private structure inside a public opaque
  field.
- **In `Tags`.** Queryable and validated, but it joins the DCB matching surface: a
  tag participates in query semantics and in every adapter's tag index, so a codec
  tag would widen consistency boundaries that have nothing to do with encoding, and
  it consumes tag budget (`crates/happenstance-core/src/limits.rs`).

The architectural constraint, whichever wins: **no adapter may need to understand
the tag.** A design in which a store must read the codec tag to serve a read puts
domain knowledge into the adapter population — ADR-0003's own reasoning, and the
ground on which ADR-0007 rejected a decoding projection store
(`.kb/decisions/0007-projection-runner-decodes.md`, *Consequences and alternatives
rejected*).

#### The wrapper store is two types, not one

`FaultyStore<S>` wraps a store, and that is the coherence trap the constitution
already names. `trait_variant` emits a blanket `impl<T: SendPort> Port for T`, so a
single type cannot carry both flavours: *"an adapter that must satisfy both
flavours is two types"* (`standards/rust/20-two-flavour-ports.md`, RS-20-4 at
`:203-253`, whose wrong implementation is precisely this and whose diagnostic is
`error[E0119]`). Load that atom and `21-send-is-not-inherited.md` before writing
either wrapper; do not rediscover it from the compiler.

The same atom carries the reason the rest of this project binds `EventStore`: the
blanket impl runs one way only, so the bare flavour is the weaker requirement and
the one that accepts both (`standards/rust/20-two-flavour-ports.md:88-91`).

#### Dependencies, features, and what the MSRV lesson means here

- **`serde` arriving in `happenstance` is the split working, not a violation.**
  ADR-0003 constrains `happenstance-core`
  (`.kb/decisions/0003-opaque-payloads.md`), whose `serde` feature covers envelope
  types only (`crates/happenstance-core/Cargo.toml:37-49`). The typed layer's codec
  must **not** route payload encoding through `happenstance-core/serde`: payloads
  stay `Bytes` at the port and `Codec` operates strictly above it. The standing
  guards are already in the gate — the `--no-default-features` doc build of the
  contract crate (`xtask/src/main.rs:502`) and the manifest lint at `:448`.
- **Two codec dependencies are already in the workspace and one is not.**
  `serde_json` and `postcard` are declared in the root `Cargo.toml`
  `[workspace.dependencies]`, the single source of truth for every third-party
  version; `postcard` currently sits under the `# --- dev / tooling only ---`
  header. If a postcard codec ships, that entry moves above the header — the
  manifest records the precedent and its reasoning for `tokio`: *"No longer
  dev-only, and the move is the point."* A CBOR codec is a **new dependency
  decision** with three consequences to check before it is taken rather than after:
  `cargo deny check licenses` against this workspace's allowlist (the manifest's
  `webpki-roots` comment is the worked example of a rejection nobody predicted),
  `deny.toml`'s `multiple-versions` posture, and the MSRV.
- **The MSRV lesson is procedural, not numeric.** ADR-0029 raised the floor to
  1.97.1 because of *a dependency's build script*, and five of five database crates
  here declare no `rust-version` at all — so `cargo hack --rust-version` cannot
  protect the floor against them (`CLAUDE.md`, binding constraint 5;
  `.kb/decisions/0029-msrv-raised-to-1-97-1.md`). Any new dependency's MSRV impact
  is established by **running the compiler at 1.97.1**, which is the CI `msrv`
  job's second half, not by reading metadata. This project does not own the MSRV
  promise (HS-P0016 does) but it can break it.

#### Changing a clause marker is a three-part edit

AC-007, AC-008 and DR-06 move markers in `spec/SPECIFICATION.md`, and the gate
checks the document against itself (`cargo xtask spec-trace`, a `REQUIRED` step at
`xtask/src/main.rs:315`). The mechanics:

1. **§7.1 and §7.2 are generated** — `cargo xtask spec-trace --write` renders them
   and the gate compares the committed region against a fresh computation
   (`xtask/src/spec_trace.rs:23-37`).
2. **§1.3's census is written by a human and only *checked*** — the total, each
   maturity count, the "of which N are normative" figure and the document's own
   subtraction (`xtask/src/spec_trace.rs:39-50`; today's numbers at
   `spec/SPECIFICATION.md:219-221`). It is deliberately never generated, because
   §7.1/§7.2 and the census come from the same parser and would drift together
   silently. Moving PS-32/PS-33/PS-35 out of the clause space therefore means
   editing those counts by hand and letting the checker disagree with you if you
   are wrong.
3. **An ID that leaves the clause space is retained, not deleted** — the marker is
   `[NON-NORMATIVE]`, *"its ID is retained so that citations resolve"*, and CF-30 is
   the worked example (`spec/SPECIFICATION.md:209-212`; disposition row at `:8775`).
   Deleting an ID breaks every citation, `RUNBOOK.md`'s included.

A `[PROVISIONAL]` or `[DEFERRED]` marker with an empty falsifier is a build failure
rather than a convention (`spec/SPECIFICATION.md:213-217`), so a verdict that
softens a marker without supplying its replacement text fails the gate, not the
review.

#### The ADR route, and who invokes it

ADR-0020 and ADR-0021 are **staged into `.kb/_intake/` and ingested**, never
hand-written into `.kb/decisions/` (`CLAUDE.md`, *Where the work lives*; the
reverted `0269720`). `/redkiln:kb-ingest` is human-invoked — plan it as a handoff at
the point a decision is ready, not as a step inside a story's implementation. The
same applies to the atoms AC-012's defect log generates. `redkiln validate --kb`
(DoD 3) checks frontmatter conformance and accepted-decision immutability against
`HEAD`.

Note the sequencing obligation this creates: AC-016 requires both ADRs written
*before* the code they govern. That is a constraint on the story map, not a
formality — an ADR written afterwards records what was built rather than deciding
it, and the immutability rule then makes any correction a second atom.

#### Tensions carried forward, flagged rather than resolved

1. **PS-18 is not evaluable in this tree.** Its clause text assigns evaluation to
   *"the exit of the projection-port phase"* and asks *"whether the SQLite adapter
   implemented it"* (`spec/SPECIFICATION.md:5200`), while `project.md`'s DR-06 and
   AC-008 group it with PS-27/PS-30 as clauses this project settles by counting.
   The reconciling citation is `RUNBOOK.md:4070-4077`, which supplies the escape
   hatch: *name the callers, or promote the clause to a documented exclusion*. Two
   facts make the first branch unavailable at design time — neither
   `projection-store-freeze` (HS-P0010) nor `sqlite-durable-store` (HS-P0012) has
   shipped an adapter, both being at `stage: storymap`, and **PS-18's subject does
   not exist**: there is no `reset` method and no `ResetError` anywhere in
   `crates/happenstance-core/src/projection.rs`, whose port is `checkpoint`
   (`:110`), `begin` (`:117`), `commit` (`:126`) and `rollback` (`:138`). HS-P0010
   owns introducing it. **Plan for the exclusion branch and be pleasantly
   surprised**; do not write a story that assumes a count is available.
2. **`trybuild` has no home, and AC-002 has no substitute worth the name.** It is
   absent from `Cargo.toml` and `Cargo.lock`, and its adoption is HS-P0010's
   decision — the specification says so directly in PS-36's disposition row:
   pinning a compile-fail diagnostic *"needs a `trybuild`-style stderr snapshot,
   which is a dependency decision phase 6 owns"* (`spec/SPECIFICATION.md:8772`;
   the finding it came from is
   `references/adr/0008-one-derivation-for-both-ports.md:234-240`). The fallback
   stated honestly: a `compile_fail` doctest is **not** an equivalent instrument.
   Rustdoc collects doctests from the lib target only, so one placed in `tests/`
   never runs (`RUNBOOK.md:3614-3627`), and rustdoc on 1.97.1 *silently ignores* an
   unmatched error-code annotation, so the bare form passes on any compile error
   including a typo — which means the negative control, the entire point of AC-002,
   cannot discriminate. **Request `trybuild` as an explicit input from HS-P0010
   before this project's design gate**, and if it is declined, escalate rather than
   substitute.
3. **The `Projection` trait's concrete shape is provisional on HS-P0010.**
   `crates/happenstance-core/src/projection.rs:1-11` still says the port is *"not
   yet frozen"*, and HS-P0010 is at the same stage as this project. What is *not*
   provisional, and can be designed against today: the transactional invariant
   (`:13-30`), the `Batch<'a>` GAT borrowing from the store (`:97-99`), and
   ADR-0007's three shape decisions — `Query` as the only nomination vocabulary,
   `Projection::Store` as an associated type so a cross-store projection is
   unrepresentable, and checkpoints per `(store, ProjectionId)`
   (`.kb/decisions/0007-projection-runner-decodes.md:60-68`). One consequence worth
   stating early: because `Batch<'a>` borrows from the store it is not `'static`
   and cannot be held across a `tokio::spawn` boundary — a fan-out runner (PS-30's
   subject) cannot move a batch into a task.
4. **`CHANGELOG.md` already asserts something the manifest does not.** Its header
   states that *"`ProjectionStore` ships behind an off-by-default
   `unstable-projection` feature"* (`CHANGELOG.md:19-22`), and no such feature
   exists in `crates/happenstance-core/Cargo.toml`'s `[features]` block, which
   carries `default`, `std`, `serde` and `memory` only (`:34-52`). Today that is a
   claim in an unpublished file; at this project's exit it becomes a claim a
   stranger can check. Either the feature lands (PS-3's *verdict* is HS-P0016's,
   but the **gating of the typed runner** is `_design.md`'s per `project.md`'s risk
   table) or the changelog line is corrected before publish. Do not let the alpha
   ship the discrepancy.

#### Deliberately not prescribed

Left to the implementer and to `_design.md`, listed so nobody reads silence as a
decision: module names and file layout inside `crates/happenstance/src/`; whether
`Projection::apply` is synchronous — the callback-driven runner ADR-0007's Context
correction already compiled — or asynchronous and therefore inheriting the
two-flavour question ADR-0008 answers with `trait_variant` (if async, it is
`trait_variant`, never `#[async_trait]`, and the cost is a doubled surface an
application must implement, which is a DT-2 cost as much as a technical one); the
composition macro's arity ceiling; the retry loop's bound and whether it is
caller-supplied; the DSL's assertion vocabulary; and the codec tag's encoding once
ADR-0021 has chosen its home.

Not open, and not this project's to reopen: the four `CLAUDE.md` binding
constraints, ADR-0007's runner split, the transactional invariant on
`ProjectionStore`, and the `[FROZEN]` ES-\* clauses.

---

## Testing brief

### Intent

**Scope this brief covers.** The test mix and merge-gate commands for HS-P0011:
where each of AC-001…AC-016 gets proven, in which tier, against which fixture —
and, where an instrument's home does not exist in the tree yet, that gap is
named rather than assumed away. This brief adds no scope; it maps `project.md`'s
already-authored acceptance criteria onto the repository's already-defined test
grains.

**The gate is defined once, and this project's slice of it is named already.**
`.redkiln/config.yaml`'s `verify:` block wires four grains regardless of who
types the command (`CLAUDE.md`, *Commands*):

| Grain | Command | What it is |
| --- | --- | --- |
| Story | `cargo xtask affected --base {{base}}` | fmt + `clippy -D warnings` + tests for the packages a diff touches, plus their dependents — re-derived from git so staged/unstaged/untracked files are seen (`.redkiln/config.yaml:28-40`) |
| Reachability, static | `cargo xtask lints && cargo xtask spec-trace` | the five file-reading lints, then the specification's cross-references — no compiler runs (`.redkiln/config.yaml:42-48`) |
| Integration, **this project's bar** | `cargo xtask ci --fast` | `REQUIRED` without `OPTIONAL` — every step except the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` build; all four `wasm32` steps stay in (`.redkiln/config.yaml:50-55`; `xtask/src/main.rs:835-857`) |
| Integration, terminal project | `cargo xtask ci` | the whole gate — **not this project's bar**: `project.md`'s frontmatter carries `terminal: false`, and DoD 6 names `cargo xtask ci --fast` explicitly as *"this project's integration bar per `.redkiln/config.yaml:55`"* |

`_design.md`'s own doctest (AC-U01) and every rustdoc/README example
(`crates/happenstance/src/lib.rs:10`) run inside `cargo xtask ci`'s test step,
so the doctest tier is not a separate merge-gate command — it is what `cargo
test --workspace --all-features` already collects.

**One consequence worth stating up front, because it changes what "green"
proves.** `cargo xtask ci --fast` drops the feature powerset (`xtask/src/
main.rs:535-546`, `OPTIONAL`) — the instrument that exercises AC-005's
JSON/CBOR/postcard feature combinations and AC-A04's forwarding claim under
`cargo-hack`. This project's own merge gate is silent on that axis; it is only
checked by the full `cargo xtask ci`, which this project does not own as its
bar. See *Notes → The feature powerset is not this project's gate* below —
it is a deployment-readiness question, not a story-grain one.

### Acceptance Criteria

Each project AC-### mapped to at least one tier and the instrument that proves
it. "Static" = fmt/clippy/lints/spec-trace, no compiled test. "Unit" = `cargo
test` inside a crate. "Integration" = a test that seeds `MemoryEventStore` (or
a fixture) and exercises more than one unit together. "Doctest" = an example
compiled by rustdoc. "E2E/gate" = a step in `cargo xtask ci`/`--fast` itself.
"Measurement"/"Record" = a non-pass/fail instrument this brief does not treat
as a test.

| AC | Tier(s) | Instrument, and where it runs |
| --- | --- | --- |
| **AC-001** fold/query agreement | Unit + Doctest | A unit test constructing a `DecisionModel` impl and asserting `apply`'s `match` is exhaustive over `Self::Event`; the rewritten worked example (AC-003) compiling is itself evidence the shape is usable, not decorative. Lives beside the new decision-model code in `crates/happenstance/src/`. |
| **AC-002** compiler protection, checked | Integration (compile-fail) | The `trybuild` case plus its negative control, registered as a row in `xtask/src/proof.rs`'s `ARTEFACTS` (AC-A03) and run by `cargo xtask ci`'s *"each phase's proof artefacts"* step (`xtask/src/main.rs:178-190`), which calls `cargo xtask proof-artefact` rather than `cargo test` — the same reason CLAUDE.md gives: `cargo test` exits 0 on `running 0 tests`, so an emptied case would pass a step a deleted one fails. **Blocked on `trybuild` landing as a workspace dependency** — see *Notes → `trybuild` has no home yet* below; this is the same tension the architecture brief carries and it is not resolved by this brief either. |
| **AC-003** worked example runs | E2E (real execution) | **No instrument exists today that actually runs the binary.** `cargo test --workspace --all-features` compiles `examples/course-subscriptions` but never executes `main`. DoD 1 requires *"`cargo run -p course-subscriptions` completes... with no `todo!()` reached"* — that is an execution claim, not a compilation one. See *Notes → AC-003 needs an execution instrument, not a compile check*. |
| **AC-004** boundaries compose | Unit + Doctest | A unit test over the composition macro asserting the OR'd `Query` for `(P1, P2)` and `(P1, P2, P3)` tuples matches the union of each model's own `query()`; the worked example's `subscribe` case, already a two-item query (`examples/course-subscriptions/src/main.rs:114-125`), is the doctest-adjacent mount named in the architecture brief. |
| **AC-005** typed, evolvable payloads | Unit + feature powerset | Per-codec round-trip unit tests (encode → decode → equal) for JSON (default) and, behind their features, CBOR/postcard; the **feature powerset** (`cargo-hack`, `OPTIONAL`, not `--fast`) is what actually proves the features compose independently and that `default-features = false` still builds. This project's own merge gate (`--fast`) does not run the powerset — flagged in *Intent* above, carried to the deployment brief as a pre-publish gate. |
| **AC-006** projection over decoded events | Integration | A runner test that opens a batch, applies decoded events, and asserts the commit is atomic (batch + checkpoint in one call, never a separate `apply()`/`set_checkpoint()` pair — `crates/happenstance-core/src/projection.rs:126`). **This test's fixture does not exist in the tree yet**: `MemoryProjectionStore` behind the `memory` feature is `projection-store-freeze`'s (HS-P0010) AC-012, not shipped (`.bklg/from-contract-to-published-library/projection-store-freeze/project.md:94,218`). See *Notes → AC-006's fixture is a cross-project dependency, not a gap in this brief*. |
| **AC-007** PS-33 evaluated | Record, not a test | A written verdict — either naming an independent caller of the core checkpoint pump or a superseding ADR — checked structurally by `cargo xtask spec-trace` (marker/citation consistency) and by human review at the design/closeout gate. No compiled assertion proves "we looked and decided"; static tier only. |
| **AC-008** PS-18/PS-27/PS-30 settled by counting | Static (spec-trace) + Integration, split by clause | PS-27 and PS-30 are **testable** clauses with named rules and E2E cases: `skip_and_record_is_atomic` (PS-27, `spec/SPECIFICATION.md:5411-5429`, cases E2E-26/E2E-27) and `panicking_apply_rolls_back` (PS-30, `:5462-5478`, case E2E-28). E2E-28's own case header marks its span *"⚠ e2e crate"* (`spec/E2E-CASES.md:725-...`) — **no workspace e2e crate exists in this tree today** (`find` for `*e2e*` under `crates/`/`examples/` returns nothing but `spec/E2E-CASES.md` itself). PS-18 is **not evaluable here at all**: its own clause text assigns evaluation to *"the exit of the projection-port phase"* (`spec/SPECIFICATION.md:5200`), and its subject (`ResetError`/reset protection) does not exist in `crates/happenstance-core/src/projection.rs` — see the architecture brief's *Tensions* item 1. Testing brief's position: write PS-27/PS-30's integration tests wherever `_design.md` places them (candidates: a `tests/` directory under `crates/happenstance/`, or a new workspace e2e crate if `_design.md` decides one is warranted — that decision is `_design.md`'s, not this brief's), and record PS-18 as excluded per the runbook's own escape hatch rather than writing a story that assumes a count is available. |
| **AC-009** test a decision, test misbehaviour | Unit + Integration | The given/when/then DSL itself gets unit tests (seed → decide → assert on emitted events or on the error, against `MemoryEventStore` — `crates/happenstance-testkit/src/fixtures.rs`'s `MemoryFixture` is the reference pattern for what "seed a store" means). `FaultyStore<S>` and `GappyMemoryStore` are integration fixtures in `happenstance-testkit`: a test asserting a caller's retry loop is exercised by `FaultyStore`'s injected failures (AC-U13), and a test asserting a handler that assumes `position + 1` fails against `GappyMemoryStore`'s gaps — the same principle CLAUDE.md states for the conformance suite itself: *"never assert on literal position values... compare against positions the store actually assigned."* |
| **AC-010** polling cost measured | Measurement, not a test | Lands in `experiments/` — *"measurements. reproducible, and not in the gate"* (`CLAUDE.md`, repository map). No pass/fail assertion; the number and its conditions are the artefact (CF-34: *"performance is measured by a separate harness and is not the bar"*, cited already in the architecture brief). |
| **AC-011** `0.2.0-alpha.1` on the registry | Static (packaging) + manual (registry) | `cargo xtask ci`'s *"packaged artifacts carry their licences and README"* step (`xtask/src/main.rs:519-529`, D11) is a static, compiled check that the tarball is complete — it does not touch the registry. The registry resolution itself (`cargo add happenstance@0.2.0-alpha.1 --locked` against a real index) is a manual post-publish verification; see the **deployment brief**, which owns release mechanics. |
| **AC-012** contract defects recorded | Record, not a test | A defect log entry per finding, each naming a clause ID and its routing (`.kb/_intake/` → `/redkiln:kb-ingest`, human-invoked, never a step inside a story). Verified at closeout, not by a compiled test. |
| **AC-013** `happenstance-macros` answered | Record, not a test | A measurement over the rewritten example — *"if the rewritten example carries more mapping boilerplate than domain logic, the derive is in scope"* (`RUNBOOK.md:524`) — recorded at closeout. Cannot be evaluated before AC-003's rewrite exists, so it is sequenced after, not tested independently. |
| **AC-014** DT-2 resolved | Design-gate review, then Unit/Doctest | The decision itself is `_design.md`'s (human-approved, DoD 5); once made, its consequence is testable: whichever `query()` shape wins (infallible with validated `Tags`, or fallible returning `Result<Query, _>`) gets a unit test asserting the chosen failure mode — a rejected `Tag`/`&str` pair either fails to construct the model or fails at `query()`, and only one of those compiles/passes depending on the resolved answer. |
| **AC-015** the edge flavour survives | Static (wasm32 build) + Unit | `cargo xtask ci --fast` keeps all four `wasm32` steps (`.redkiln/config.yaml:53-55`), so `happenstance-core`'s wasm32 build stays green under this project's own bar — but per the architecture brief's AC-A06, **none of the four compiles `happenstance` itself**. A unit-level pattern test (a doctest or a `#[test]` in a `wasm32`-agnostic module) asserting `EventStore` (not `SendEventStore`) is bound in any new generic function is the closest this project's own gate gets to proving AC-015 for its *own* code, following `spawns_from_generic`'s bound (`crates/happenstance-core/src/memory.rs:643-680`) as the pattern. Whether a fifth `wasm32` step compiling `happenstance` itself is added is `_design.md`'s call per AC-A06 — if it is, that step is this AC's real instrument and belongs in `xtask/src/main.rs::REQUIRED` (`:105`), not a workaround elsewhere. |
| **AC-016** ADR-0020/0021 written first | Static (`redkiln validate --kb`) | Frontmatter conformance and accepted-decision immutability against `HEAD` — not a compiled test, and not authored by this brief (atoms are `/redkiln:kb-ingest`'s to write, never hand-authored — `CLAUDE.md`, *Where the work lives*). |

### Notes

#### Fixtures and seams to mock

| Fixture | Where it lives | What it stands in for |
| --- | --- | --- |
| `MemoryEventStore` | `crates/happenstance-core/src/memory.rs` (existing, real) | The database, for every decision/DSL test. `RUNBOOK.md:3977-3979`: *"`MemoryEventStore` is all this project needs."* |
| `MemoryFixture` | `crates/happenstance-testkit/src/fixtures.rs` (existing, real) | The reference `Fixture` pattern CLAUDE.md names — not itself a mock this project drives directly, but the shape `FaultyStore<S>` and `GappyMemoryStore` follow when they also want to be conformance fixtures (`event_store_conformance!`'s capability-declension pattern). |
| `FaultyStore<S>` | `crates/happenstance-testkit/src/` (new, this project) | A store that misbehaves on demand — injects `ConditionViolated`, `None` `conflicting_position` — so a caller's retry loop is testable without a real database or a race. Two coherence-trap types per the architecture brief's *The wrapper store is two types* note; load `standards/rust/20-two-flavour-ports.md` before writing it. |
| `GappyMemoryStore` | `crates/happenstance-testkit/src/` (new, this project) | A store whose positions have gaps, so a handler that assumes `position + 1` fails loudly instead of silently — the same reason the conformance suite itself may not assert literal positions (CLAUDE.md, *The rule that matters*). |
| `MemoryProjectionStore` | **not yet shipped** — HS-P0010's AC-012 | AC-006's runner test has no in-memory adapter to run against until `projection-store-freeze` lands it behind the `memory` feature. Not this project's to build (it would be freezing conformance/fixture shape, explicitly out of scope per `project.md`, *Out of scope*, first bullet). |
| `Cargo.lock`-pinned `serde_json`/`postcard` | root `Cargo.toml` `[workspace.dependencies]` (existing) | The two codec dependencies already present; a CBOR codec test needs a third that is **not** yet declared (architecture brief, *Dependencies, features, and what the MSRV lesson means here*). |

#### AC-003 needs an execution instrument, not a compile check

DoD 1 and AC-003 both use an execution verb — *"completes,"* *"runs on,"* *"no
`todo!()` reached"* — and nothing in `cargo xtask ci` today invokes the
`course-subscriptions` binary (confirmed: `xtask/src/main.rs` has no
`course-subscriptions` reference at all). `cargo test --workspace
--all-features` compiles the crate as a dependency of the workspace build but
never runs its `main`. The gap is real and cheap to close without a new
dependency: `std::process::Command::new(env!("CARGO_BIN_EXE_course-subscriptions"))`
is a standard-library pattern for an integration test in
`examples/course-subscriptions/tests/` that runs the binary and asserts on its
exit status and stdout — no `assert_cmd` or similar is in the workspace today
(`grep` for `assert_cmd`/`CARGO_BIN_EXE` across `Cargo.toml` returns nothing),
and none needs to be added for this. Whether that test lands in `tests/`
(collected by `cargo test`, and therefore already inside `cargo xtask ci`'s
existing test step with no new gate wiring) or as an explicit new `xtask`
step is `_design.md`'s call; this brief states only that *something* must
actually execute the binary before DoD 1 is honestly checked off.

#### `trybuild` has no home yet

Repeated from the architecture brief rather than silently assumed resolved:
`trybuild` is absent from `Cargo.toml`/`Cargo.lock`, its adoption is
`projection-store-freeze`'s (HS-P0010) decision per `project.md`'s risk table,
and a `compile_fail` doctest is **not** an equivalent instrument — rustdoc
collects doctests from the lib target only, so one placed in `tests/` never
runs, and rustdoc on 1.97.1 silently ignores an unmatched error-code
annotation, so the negative control (AC-002's entire point) cannot
discriminate (`RUNBOOK.md:3614-3627`; `spec/SPECIFICATION.md:8772`). This
testing brief's merge-gate row for AC-002 above is written against the
instrument the criterion actually needs, not a fallback — request `trybuild`
as an explicit input from HS-P0010 before this project's implementation
begins on that story, and escalate rather than substitute if it is declined.

#### AC-006's fixture is a cross-project dependency, not a gap in this brief

`Projection::apply` and its runner are this project's to write (DR-05,
ADR-0007), but proving the runner's transactional invariant by test needs an
in-memory `ProjectionStore` to run it against, and shipping that fixture is
named explicitly as `projection-store-freeze`'s own AC-012 (*"`MemoryProjectionStore`
behind the `memory` feature — the oracle,"* `.bklg/from-contract-to-published-library/
projection-store-freeze/project.md:94,218`). Writing a throwaway in-memory
`ProjectionStore` inside this project to unblock testing would itself be
freezing a fixture shape for the port — the exact thing `project.md`'s
*Out of scope* section assigns to HS-P0010. The dependency edge
(`project.md`, *Dependencies → Depends on*) already names this; this brief's
addition is naming AC-006's *test* instrument specifically as one more thing
blocked on that edge, alongside the `Projection` trait's shape itself
(architecture brief, *Tensions* item 3).

#### The feature powerset is not this project's gate

`cargo xtask ci --fast` — this project's own DoD-6 bar — does not run
`cargo-hack`'s feature powerset (`.redkiln/config.yaml:50-55`; `xtask/src/
main.rs:835-857`, `OPTIONAL` dropped). AC-005 ships codec features (CBOR,
postcard) behind Cargo features, and AC-A04 requires every one of them
forwarded correctly from `happenstance` to `happenstance-core` — both claims
the powerset is the instrument for. This project can and should still run
`cargo hack check --feature-powerset -p happenstance -p happenstance-core`
locally while implementing the codec module (the tool already resolves on
this machine per CLAUDE.md's *Commands* section), but the **merge gate** for
this project's stories does not enforce it. The full `cargo xtask ci` — which
does run it — belongs before the publish step, which is the deployment
brief's concern, not a story-grain one; see the deployment brief's *CI
implication* section.

---

## Deployment brief

### Intent

**Scope this brief covers.** The release/rollout strategy for HS-P0011's one
real infrastructure event: publishing `0.2.0-alpha.1` to crates.io. The
initiative's own decomposition names why this project (and only one sibling,
`publication-and-positioning`) warrants a `deployment` brief at all —
*"the two real crates.io release events (`typed-layer`, `publication`)"*
(`.bklg/from-contract-to-published-library/_decomposition.md:280-284`). This
is the **first** of those two events and the one that creates the semver
baseline the second one diffs against
(`project.md`, *How this advances the initiative*).

**What this is not.** Not `happenstance-sqlite`/`-cloudflare`/`-postgres`/
`-neon`/`-ladybug`/`-sync` — all six carry `publish = false`
(`crates/happenstance-{cloudflare,ladybug,neon,postgres,sqlite,sync}/
Cargo.toml:12`) and none is touched by this release. Not the MSRV promise, the
`cargo-semver-checks` verdict, or registry presentation — all
`publication-and-positioning`'s (`project.md`, *Out of scope*). Not phase 12's
full, stable `0.2.0` publish — `RUNBOOK.md:163` lists that phase's own exit
criteria (docs.rs green, `cargo-semver-checks` against a registry baseline)
separately from this project's, and this project's release is explicitly the
earlier, feedback-gathering pre-release phase 12 diffs against, not phase 12
itself. This project publishes a **pre-release** of a version already reserved
(`RUNBOOK.md:798-803`: exactly the three names this release touches —
`happenstance`, `happenstance-core`, `happenstance-testkit` — were claimed at
phase 0, verified free on 2026-08-06, so name reservation is not this brief's
concern either).

### Acceptance Criteria

- **DEP-001 — What publishes, and what does not.** The three crates carrying no
  `publish = false` — `happenstance`, `happenstance-core`,
  `happenstance-testkit` — are the publishable set (confirmed by reading all
  nine `crates/*/Cargo.toml`; the other six are skeletons with `publish =
  false` and CLAUDE.md's own 🔩 marker: *"an instrument first and a target
  second... not an adapter until it has run the conformance suite. None of
  them has"*). `happenstance` and `happenstance-core` publish at
  `0.2.0-alpha.1`; `happenstance-testkit` publishes on its **own** number,
  independent of the workspace version, per CF-32 (`crates/happenstance-testkit/
  Cargo.toml:4-13`: it is already at `version = "0.2.0"` in-tree, moved there
  by the phase-4 freeze's thirty-four new conformance rules — whether it also
  needs an `-alpha.1` suffix at this release, or ships its `0.2.0` stable
  because CF-32 already treats a testkit minor as breaking on its own
  schedule, is an explicit decision this brief flags rather than assumes: see
  *Notes → Does the testkit publish `-alpha.1` too, or stays stable*.
- **DEP-002 — The pre-release is unreachable by accident.** Cargo will not
  resolve a pre-release version without an explicit pre-release requirement in
  the consumer's manifest (`RUNBOOK.md:4149-4155`, the third of the three
  mitigations). No feature flag or config gate is needed to keep
  `0.2.0-alpha.1` off a default `cargo add happenstance` — the version string
  itself is the gate, and this is Cargo's own mechanism, not something this
  project builds.
- **DEP-003 — The three churn mitigations are acceptance criteria, not
  intentions.** AC-011 states them as observable artefacts, not as good
  intentions: a README `## Stability` section naming the phase the API stops
  moving (mounted between the existing `## Guarantees` and `## Design`
  sections per the architecture brief, *Composition roots* item 7); a
  `CHANGELOG.md` `## [Unreleased]` section becoming the alpha's dated section
  (the file already exists — an edit, not a new file); and a **yank policy for
  superseded alphas**, stated in the README, executed as a real `cargo yank`
  when `0.2.0-alpha.2` (or later) lands — so the resolvable pre-release set is
  always exactly one version (`RUNBOOK.md:4149-4155`). The risk table names
  the failure mode these guard against precisely: *"a published alpha makes
  'we can't change that now' available"* (`project.md`, risk table row 1) —
  the mitigations are the check that the *authors* do not treat their own
  unpublished API as something to protect.
- **DEP-004 — Publish order matches the dependency graph.** `happenstance`
  depends on `happenstance-core` (`crates/happenstance/Cargo.toml:20`) and
  `happenstance-testkit` also depends on `happenstance-core`
  (`crates/happenstance-testkit/Cargo.toml:29`) — `cargo publish` must run for
  `happenstance-core` before `happenstance` (crates.io rejects a manifest
  whose path/workspace dependency cannot resolve against the registry yet).
  `happenstance-testkit` has no ordering dependency on `happenstance` and may
  publish in either relative order once `happenstance-core` is live.
- **DEP-005 — The packaging gate runs before publish, not instead of it.**
  `cargo xtask ci`'s *"packaged artifacts carry their licences and README"*
  step (`xtask/src/main.rs:519-529`, D11) already asserts, via `cargo package
  --list` and reading the resulting tarball's contents (not merely the
  manifest's promise to include them — CLAUDE.md, *Commands*: *"a `cargo
  package --list` assertion that each of the three publishable crates carries
  both licence files and a README"*), that all three publishable crates ship
  correctly. That check is static and pre-existing; it is not this brief's to
  re-describe, only to require it green (via `cargo xtask ci`, the full gate —
  see *Notes → why this release needs the full gate, not `--fast`*) before
  `cargo publish` runs for any of the three crates.
- **DEP-006 — Post-publish verification is a resolution check, not a redeploy.**
  There is no server to roll back and no traffic to shift — a crate registry
  publish is **irrevocable**: `cargo yank` hides a version from *new*
  dependency resolution but does not delete it, and a version once published
  cannot be edited (`RUNBOOK.md:4483-4484`: *"a crates.io release cannot be
  edited"*, said of the later `0.2.0` but true of every crates.io publish
  including this one). Verification is therefore: resolve
  `happenstance@=0.2.0-alpha.1` (and `happenstance-core`,
  `happenstance-testkit`) from a clean environment with an explicit
  pre-release requirement, confirm docs.rs renders the crate (best-effort;
  docs.rs build failures do not block adoption but are worth checking within
  the hour), and confirm the registry page shows the README's `## Stability`
  section. None of this is automatable inside `cargo xtask ci`, which cannot
  reach the live registry.
- **DEP-007 — Rollback posture is "yank the mistake, publish the fix at a new
  version," never edit-in-place.** If a defect is found in
  `0.2.0-alpha.1` post-publish, the response is `cargo yank
  --version 0.2.0-alpha.1` (removing it from new resolution, not from
  existing lockfiles) followed by a `0.2.0-alpha.2` carrying the fix and a
  `CHANGELOG.md` entry explaining what broke between alphas — exactly the
  artefact DEP-003 already requires to exist for this reason
  (`RUNBOOK.md:4148-4149`: *"a `CHANGELOG.md` that lists what broke between
  alphas, which is the artefact that converts 'it churned' into 'it churned
  for these stated reasons.'"*). There is no database migration or backfill
  in this release — `MemoryEventStore` persists nothing across a process,
  and no durable adapter is touched (DEP-001) — so "rollback" here means
  registry-level yank-and-republish only, never a data migration reversal.

### Notes

#### Migration / backfill: N/A, explicitly

This release touches no persisted data. `MemoryEventStore` is in-process and
non-durable by construction; every durable adapter (`happenstance-sqlite`,
`-cloudflare`, `-postgres`, `-neon`, `-ladybug`) carries `publish = false` and
is untouched. There is nothing to migrate and nothing to backfill for this
release, and no future release of *this* project needs to reconcile a
migration against it.

#### Feature flags / config gating: mostly N/A, one real item

There is no runtime feature flag gating this release's *rollout* — a crates.io
publish either exists at a version or it does not, and DEP-002 already covers
the one thing that behaves like a gate (the pre-release requirement Cargo
itself enforces). The one live gating decision belongs to `_design.md`, not to
this brief: **whether the projection runner ships behind an off-by-default
`unstable-projection` Cargo feature**, per PS-3's still-open verdict
(`project.md`, risk table row 3: *"`_design.md` must state the runner's
feature gating explicitly rather than inheriting it; the PS-3 *verdict* stays
HS-P0016's"*). This matters to this brief only insofar as `CHANGELOG.md`
already asserts the feature exists — *"`ProjectionStore` ships behind an
off-by-default `unstable-projection` feature"* (`CHANGELOG.md:19-22`) — while
`crates/happenstance-core/Cargo.toml`'s `[features]` block carries `default`,
`std`, `serde` and `memory` only, no such feature
(`.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/
_decomposition.md`, architecture brief, *Tensions* item 4, already flagging
this). **This release must not publish that discrepancy.** Either the feature
lands in the manifest before `0.2.0-alpha.1` is cut, or the changelog line is
corrected first — this brief adds the release-blocking framing to a tension
the architecture brief already named.

#### Why this release needs the full gate, not `--fast`

The testing brief's merge-gate table runs this project's stories against
`cargo xtask ci --fast`, which is correct for story-grain work — but a
`cargo publish` is not a story, it is the one point where this project's
output becomes something a stranger resolves. `run_fast`'s own doc comment
says as much directly, unprompted by this brief: *"`REQUIRED` without
`OPTIONAL` — the project-scoped bar, **not the release bar**"*
(`xtask/src/main.rs:835`). `--fast` drops the feature powerset, `cargo deny`
and the nightly `--cfg docsrs` build (`.redkiln/config.yaml:50-55`). Of those
three, two matter directly to publish-readiness and neither is optional at
this gate:

- **The feature powerset** is the only instrument that proves AC-005's
  CBOR/postcard-behind-features claim and AC-A04's feature-forwarding claim
  hold in combination, not just individually — exactly what a `cargo add
  happenstance --no-default-features --features cbor` consumer depends on
  working the day this ships.
- **`cargo deny check`** is the licence/advisory/duplicate-version gate a
  published crate cannot un-publish its way out of after the fact — a licence
  violation caught post-publish is a legal problem, not a follow-up commit.

The nightly `--cfg docsrs` build is the weaker case for release-blocking (it
predicts docs.rs rendering rather than gating what ships), but running it once
before publish is cheap and DEP-006 already asks for a docs.rs check anyway.
**Recommendation: run `cargo xtask ci` (the full gate) once, immediately
before cutting the release, in addition to the `--fast` runs every story
already passed** — not as a replacement for the project's DoD-6 bar, which
stays `--fast`, but as an explicit pre-publish step this brief adds because
nothing else in the gate wiring forces it at the release boundary specifically.

#### Does the testkit publish `-alpha.1` too, or stays stable

Flagged, not resolved. `happenstance-testkit` already carries its own
in-tree `version = "0.2.0"` (moved there by the phase-4 freeze, per its own
manifest comment) and CF-32's whole argument is that its version moves on a
*different* schedule from the other two crates precisely because a testkit
minor is itself a breaking event for adapter authors. Whether this project's
release cuts `happenstance-testkit` as a **stable** `0.2.0` (its number is
independent, so nothing forces it to carry `-alpha` even while `happenstance`
and `happenstance-core` do) or as `0.2.0-alpha.1` in lockstep is a real
decision with a real consequence: a stable testkit publish makes a semver
promise about the conformance suite's own rule set while the ports it
conforms is still moving underneath it (`happenstance-core` itself is
pre-1.0 and this release is explicitly a pre-release). This brief does not
resolve it — it is `_design.md`'s to decide alongside DT-2, stated here so it
is not defaulted by whichever version string `cargo publish -p
happenstance-testkit` happens to run first.

#### CI implication

No new CI *infrastructure* is required — unlike `cloudflare-durable-object-store`
(a `workerd` runner) or `postgres-and-neon-stores` (a live Postgres and a live
Neon HTTP endpoint), this release needs nothing that does not already exist:
`cargo xtask ci` already runs in CI today (`CLAUDE.md`, *Commands*: *"it is
defined once in `xtask/src/main.rs` and is exactly what CI runs"*), and
`cargo publish` itself is a manual or human-gated CI step this brief does not
prescribe the mechanics of (no `.github/workflows/` file references `publish`
or `crates.io` today — confirmed by `grep`; `ci.yml` is the only workflow in
the tree). What CI *does* newly need, as a direct consequence of AC-002/DEP-005:
if `trybuild` lands as a dev-dependency (testing brief, *Notes*), it needs no
special CI provisioning — it is a pure-Rust dev-dependency exercised by `cargo
test`, unlike `cargo-hack`/`cargo-deny`/nightly rustdoc, which are already
handled by `OPTIONAL`'s own probe-and-skip pattern (CLAUDE.md, *Commands*:
*"`cargo-hack` and `cargo-deny` both resolve on this machine... A step skips
only when its probe fails to find the tool"*).
