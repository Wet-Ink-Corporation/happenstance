---
item: HS-S0029
stage: spec
created: 2026-08-12T13:46:26.084Z
updated: 2026-08-12T13:46:26.084Z
template_sig: 87bbf1d0
rendered_sig: 10bd3722
---

# Spec — The worked example, rewritten on the typed layer and actually executed

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/worked-example-on-typed-layer/spec.md` |
| Key brief — architecture | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md`, *Composition roots* (items 2, 3, 4) and *The contracts this layer consumes unchanged — and five ways to get them wrong* |
| Key brief — ux | same file, *What the compiler says, and where it points* (AC-U07) and *The two rendered surfaces* |
| Key brief — testing | same file, AC-003 row of the test-level matrix and *Notes → AC-003 needs an execution instrument, not a compile check* |
| **Signed-off design (BINDING)** | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` — approved 2026-08-12, *Sign-off*; surface `worked-example-transcript` |
| Story map (this story's row) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md`, M6 `worked-example-and-proof` |
| Roadmap pointer | `RUNBOOK.md:3971-4102` (phase 7's goal and exit criteria); `RUNBOOK.md:524` (the `happenstance-macros` criterion this rewrite is measured for) |

Traces to project **AC-003** (`project.md`, *Acceptance criteria*). Depends on
`command-loop` and `decision-model-composition`, and transitively on the M2/M3 items
those two stand on (`domain-event-and-decision-model`, `codec-and-feature-forwarding`).

## One-line PR slice

Rewrite `examples/course-subscriptions/` onto `happenstance` — the manifest dependency
moves off `happenstance-core`, `parse_capacity` (`src/main.rs:231`) and the
`format!("…").into_bytes()` payload (`:97-100`) are **deleted rather than wrapped**,
the private `commit` (`:207`) is deleted *into* `happenstance::commit`, `main`'s
observable steps (`:37-79`) survive verbatim — and add the integration test that
actually **executes** the binary through `CARGO_BIN_EXE_course-subscriptions`, because
nothing in the gate runs `main` today and AC-003 is an execution claim.

## Executive summary

**What this PR lands.** The consumer that makes the whole project's claim observable:
one application, written the way an application author would write it, importing
`happenstance` and nothing below it — plus the first instrument in this repository
that runs a binary and reads its output.

**Delta against what exists.** The example works today and proves the wrong thing. It
imports `happenstance_core` directly (`examples/course-subscriptions/src/main.rs:24-27`),
so it exercises the crate *adapter* authors pin rather than the crate a user
`cargo add`s (`_decomposition.md`, *Composition roots*, item 3). It names its event set
twice — once as a `Query` (`:114-125`), once as a `match` over strings (`:140-155`) —
which is exactly the DCB hazard phase 7 exists to close, and which today no compiler,
clippy or conformance rule sees (`project.md`, *In scope*, `DecisionModel` bullet). It
hand-rolls `parse_capacity` (`:231`) with an `unwrap_or(0)` because *"decoding is the
typed layer's job, and that layer does not exist yet"* (`:229-230`). And its private
`commit` (`:207`) names its own destination in its doc comment — *"it belongs in the
typed layer"* — then `bail!`s where a retry belongs (`:217-221`). Every one of those
four apologies is now removable, because M2, M3 and the slice-mate landed the things
they were apologising for.

**The half nobody has built yet.** `cargo run -p course-subscriptions` has never been
run by the gate. Verified in this tree: `xtask/src/` contains no `course-subscriptions`
reference at all, and `CARGO_BIN_EXE`/`assert_cmd` appear in no manifest or source file
in the workspace. `cargo test --locked --workspace --all-features`
(`xtask/src/main.rs:143-154`) *compiles* the example and never executes `main`. DoD 1
and AC-003 are both written with execution verbs — *"completes"*, *"runs on"*, *"with
no `todo!()` reached"* — so until something runs the binary, the initiative's flagship
`@smoke` scenario is checked off by a compile.

**What this PR does not do.** It adds no public item. Every name it uses was landed by
M2/M3; if the example cannot be written without a new one, that is a finding for the
slice to route, not a licence to widen this PR (see *PR boundary*).

## Context pack

Twelve decisions. Each is settled — this story implements them, it does not re-open
them.

**1. `main`'s observable steps survive verbatim, because they are the acceptance
criterion.** `examples/course-subscriptions/src/main.rs:37-79` is a **signed-off
surface**: `worked-example-transcript` in `_design.md`, *Surfaces*, whose composition
section says in terms *"`main`'s observable steps survive verbatim — they **are** the
behaviour AC-003 asserts. What changes is beneath them"* (`_design.md`, *Composition →
`worked-example-transcript`*). Seven `== … ==` sections in the order they stand today,
each preceded by a blank line except the first, indented result lines, refusals
carrying the literal `rejected: ` prefix, then `== final log ==` and one row per event
in the existing `{:>3}  {:<22} {:?}` column form. The rewrite happens **beneath**
`main`, not through it. A "tidier" transcript is a failed story.

**2. Delete, do not wrap.** Three constructs are removed outright, and each is removed
because the thing that made it necessary now exists:

- `parse_capacity` (`:231-237`) — a hand-rolled byte scraper whose failure mode is
  `unwrap_or(0)`, i.e. *a course with no capacity silently becomes a course with
  capacity zero*. Capacity now arrives through `DomainEvent::decode`.
- the payload `format!("{{\"capacity\":{capacity}}}").into_bytes()` (`:97-100`; the
  story map cites the same expression as `:96-99`) — replaced by the codec, and the
  `&b"{}"[..]` empty payloads at `:168` and `:192` with it.
- the private `commit` (`:207-224`) — deleted *into* `happenstance::commit`. Its own
  doc comment is the warrant, and the architecture brief calls this deletion *"the
  integration test for the command loop"* (`_decomposition.md`, *Composition roots*,
  item 2).

A wrapper that keeps any of the three alive behind a nicer name fails this story: the
project's claim is that the typed layer absorbed them, not that it coexists with them.

**3. The event set is named once, and that is the point of the whole phase.** Today
`subscribe` declares `[COURSE_DEFINED, STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED]` in a
`QueryItem` (`:114-125`) and then re-declares the same set as `match` arms over
`event_type().as_str()` (`:140-155`). After this PR the query is **derived** —
`Boundary::query` off the models' `const EVENT_TYPES` and their validated `scope`
(`_design.md`, *Signatures*; *Shape decision*, `DecisionModel::query` row) — and the
fold is a `match` over the domain **enum**, which the compiler makes exhaustive. There
is no second place to state the set, which is DR-02 held structurally rather than by
review.

**4. `subscribe` is the composition case, and it must use a tuple boundary.** Its
two-item query (`:114-125`) is two consistency concerns — *the course's capacity and
everyone holding a seat* and *this student's own history* — and the slice-dependency
`decision-model-composition` exists so those are two decision models OR'd into one
query by `impl Boundary for (B1, B2)`, with no macro in the caller's face
(`decision-model-composition/spec.md`, *Executive summary*; `_design.md`, *Shape
decision*, Composition row). Collapsing them into one hand-written model would make
the example pass while deleting the demonstration project AC-004 is scored on. This is
the mount that turns *"composing several into one query is the mechanism that makes a
dynamic consistency boundary dynamic"* (`crates/happenstance/src/lib.rs:44-46`) from
prose into running code.

**5. The command loop is consumed, never re-derived.** Every handler goes through
`happenstance::commit` (JSON) with a caller-visible `Retry`. The example therefore
inherits, and must not locally re-implement, the three properties the loop was written
to make unwritable: the `after` anchor comes from the read and never from `append`'s
return (`crates/happenstance-core/src/store.rs:131-145`); `conflicting_position` is
carried, never branched on (`crates/happenstance-core/src/error.rs:139-147`); the retry
bound is a required argument, not a hidden default. No `AppendCondition`, no
`read_decision_model` call and no `AppendError` match survives in the example — if one
does, the loop did not absorb the handler.

**6. The dependency moves, and the move is the claim.** `examples/course-subscriptions/Cargo.toml`
depends on `happenstance-core` today. *"An example still depending on
`happenstance-core` proves nothing about the crate a user installs"*
(`_decomposition.md`, *Composition roots*, item 3). After this PR the example's only
`happenstance*` dependency is `happenstance` with `std`, `memory` and `json`, plus
`serde` with `derive` for the domain enum (`serde` is already in
`[workspace.dependencies]`, root `Cargo.toml:48`). Contract names still resolve —
`happenstance` re-exports them through `pub use happenstance_core::*;`
(`crates/happenstance/src/lib.rs:75`), and `bytes` with them
(`crates/happenstance-core/src/lib.rs:126`) — so `happenstance_core::` should appear
nowhere in the example's source or manifest afterwards.

**7. AC-003 is an execution claim, and this PR builds its instrument.** The testing
brief hands the choice of home to `_design.md` (`_decomposition.md`, *Notes → AC-003
needs an execution instrument*), and `_design.md` is silent on it — so this spec
decides: an integration test at `examples/course-subscriptions/tests/runs.rs` that
spawns `env!("CARGO_BIN_EXE_course-subscriptions")` through
`std::process::Command`, asserts the exit status is success, and asserts on stdout. No
new dependency: `CARGO_BIN_EXE_<name>` is a Cargo-provided env var for integration
tests of a package with a binary target, and `assert_cmd` is absent from this workspace
and stays absent. Because `examples/*` are workspace members (root `Cargo.toml:3`),
`cargo test --locked --workspace --all-features` already collects it — the gate needs
no new step.

**8. …and the gate must notice if that instrument is deleted.** `cargo test` exits 0 on
`running 0 tests`, *"so an emptied file passes a step that a deleted one fails"*
(`xtask/src/main.rs:174-177`). The execution test is therefore registered as an
`Artefact` row in `xtask/src/proof.rs::ARTEFACTS` (`:132-149`), naming its package,
its target and its test by name, exactly as the wire negative controls are. That row is
this story's second mount and is the only reason a future tidy-up cannot quietly return
DoD 1 to being checked by a compile. The slice-mate `compile-fail-proof-artefact` edits
the same list for AC-002 — one file, two rows, one context: coordinate, do not conflict.

**9. The transcript's own quality bars are binding, not stylistic.** From `_design.md`,
*Anti-patterns* 9-11 and *Density budget → Terminal transcript*: zero colour, zero
motion, no in-place rewrite; no line past **80 columns**, and a log row never truncates
its position or its event type (the tag list wraps to a continuation line indented 8);
every refusal keeps the literal `rejected: ` prefix and renders the **carried value**
(`course c1 is full (2/2)`), never a category. The refusal messages that exist today at
`:94`, `:159`, `:162`, `:165` and `:189` already satisfy that and are preserved in
substance. The design also records a known overrun to respect rather than provoke: a
long event type overruns the 22-character column and nothing covers it
(`_design.md`, *Mock*, finding 5) — so the rewrite must not lengthen an event type
name.

**10. Write the mapping plainly, because it is being measured.** AC-013 —
*"if the rewritten example carries more mapping boilerplate than domain logic, the
derive is in scope for 0.1"* (`RUNBOOK.md:524`) — is read off **this** rewrite by the
M7 story `defect-log-and-macros-verdict`, and `_design.md` records a falsifiable
prediction of 2.4:1 against the domain, *"checked against the rewritten example, not
against this doctest"* (`_design.md`, *The doctest*, closing note). Do not hide the
`DomainEvent` impl behind a helper, a local macro or a blanket to make the example look
shorter: that answers AC-013 by concealment. Write it the way a first-time user would
have to.

**11. The compile-fail slice-mate needs the fold to live here, in the user's code.**
`compile-fail-proof-artefact` adds a variant to *this* example's domain enum and
requires the diagnostic's `-->` span to land on the user's own `match` arm — *"not into
a macro body, and not anything under `crates/`"* (`_design.md`, *Anti-patterns* 13;
AC-U07). So the domain enum and its `apply` fold must be ordinary, visible code in
`examples/course-subscriptions/src/main.rs`. An implementation that moves the fold
behind an abstraction breaks the slice-mate that follows it in the same milestone.

**12. Defects go in the log; the frozen crate is not edited.** This is the first real
application above a `[FROZEN]` contract, so it is where contract defects surface
(`project.md`, *Risks*, row 5). Anything this rewrite reveals is written down with its
clause ID and routed to a decision record for AC-012 — never fixed by editing
`crates/happenstance-core/**` or a `[FROZEN]` clause (AC-A02; `_storymap.md`, *What is
deliberately not a story here*, third bullet). `_design.md` already carries one such
entry, **D-1** (no infallible `QueryItem` constructor for pre-validated inputs), and
this story is likely to find more.

**The persona slice this realizes.** Activity **A6** of *"Choose a contract before a
database"* — *"see it actually work"* — closing Beat 4 for P1, the application author
(`_storymap.md`, *Backbone*). It is also the artefact P4, the one-shot evaluator, is
pointed at from the README: an example that runs is the difference between a claim and
a demonstration.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice: a running application whose output a human reads |
| **Slice / milestone** | **M6 `worked-example-and-proof`**; slice-mate: `compile-fail-proof-artefact`, which follows this story because its diagnostic points at this example's own `match` arm. Implemented in one context and mounted as one integrated surface |
| **Mount point** | **`examples/course-subscriptions/src/main.rs`** — the running application, composition root item 2 (`_decomposition.md`, *Composition roots*). Its `main` (`:33-79`) is the DCB cycle AC-003 asserts; its three handlers `define_course` (`:86`), `subscribe` (`:113`), `unsubscribe` (`:177`) plus the shared `commit` (`:207`) are the four call sites the typed layer must absorb |
| **Secondary mounts** | `examples/course-subscriptions/Cargo.toml` — the dependency moves to `happenstance` (composition root item 3) · `xtask/src/proof.rs::ARTEFACTS` (`:132-149`) — the gate mount that makes deleting the execution test a loud failure (composition root item 4) |
| **Wires into** | `happenstance::commit` / `Retry` / `Committed` / `CommandError` from slice-predecessor `command-loop` · `happenstance::DomainEvent` / `DecisionModel` / sealed `Boundary` from `domain-event-and-decision-model` · `impl Boundary for (B1, B2)` from slice-predecessor `decision-model-composition` · `happenstance::Json` / `Codec` / `CodecError` and the `json` feature from `codec-and-feature-forwarding` · `happenstance::{MemoryEventStore, Tags, EventType, Query}` through the crate-root glob (`crates/happenstance/src/lib.rs:75`) · `MemoryEventStore::snapshot` for the final-log region · `serde` `derive` from `[workspace.dependencies]` (root `Cargo.toml:48`) |
| **Renders surfaces** | **`worked-example-transcript`** (`_design.md`, *Surfaces*) — rendered, and its shape explicitly **unchanged**: seven `== … ==` sections, `rejected: ` refusals, the aligned final log. States exercised: `accepted`, `rejected`, `final-log`, `piped-non-tty` (how the new test runs it), `narrow-80col`. No other surface is touched; this story creates no surface id |
| **Public items** (`_design.md`, *Items*) | **None.** This story claims no row in the design's `## Items` block — it is the consumer that proves the rows M2 and M3 built. An example that needs a new public item has found a gap in those stories, which is escalated, not filled here |
| **Conformance rule(s)** | **None, and that is correct.** `happenstance-testkit`'s suite observes *adapters*; this is an application above the port, with no adapter behaviour to assert. The port is unchanged, so there is no rule to add or amend. Its falsifiability comes from the execution test and its `ARTEFACTS` registration, not from the suite |
| **Clause(s)** | Discharges none and amends none. The typed layer has no clause namespace in `spec/SPECIFICATION.md` (`_design.md`, header). The example *depends on* `ES-10`'s position-visibility invariant through `commit`'s after-anchor. Any clause-level defect this use reveals is logged for **AC-012** with its ID and routed to a decision record — never a line edit |
| **Advances DoD scenario** | Initiative **DoD 1** — *"@smoke — the worked example runs end to end: `cargo run -p course-subscriptions` completes the canonical DCB cycle … with no `todo!()` reached"* (`initiative.md:360-362`). This story is the whole of that scenario and the first instrument in the repository that can observe it. It also unblocks **DoD 2** (`initiative.md:363-365`), whose compile-fail case is written against this example's domain enum |

## PR boundary

```
examples/course-subscriptions/**
xtask/src/proof.rs
Cargo.lock
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/worked-example-on-typed-layer/**
```

**`Cargo.lock` was added to the fence on 2026-08-16, and it is a repair rather than a
widening.** This spec's own body already *predicts* the movement in two places — *"`Cargo.lock`
may move, because the example gains `serde` and drops a direct"* dependency, and *"`Cargo.lock`
is expected to move, by edges rather than nodes"* — so the story anticipated writing a file its
fence did not admit, which is precisely the contradiction `redkiln verify --grain story`
rejected the checkpoint for. Four sibling specs in this project — `codec-and-feature-forwarding`,
`projection-trait-and-runner`, `compile-fail-proof-artefact` and `publish-0-2-0-alpha-1` —
already carry the entry. The addition admits the lockfile and nothing else:
`[workspace.dependencies]` and every other manifest stay outside.

**In this PR**

- `examples/course-subscriptions/src/main.rs` rewritten beneath `main`: the domain enum
  and its `DomainEvent` impl, the decision models and their `DecisionModel` impls, the
  three handlers rewritten to derive a boundary and call `happenstance::commit`, and the
  deletion of `parse_capacity`, the `format!` payload construction and the private
  `commit`.
- `main`'s observable steps and the module doc's three-invariant framing (`:1-19`)
  preserved; `#![allow(clippy::print_stdout)]` (`:21`) stays.
- `examples/course-subscriptions/Cargo.toml`: `happenstance-core` → `happenstance`
  (`std`, `memory`, `json`), `serde` with `derive` added, `anyhow` and `tokio` retained.
- `examples/course-subscriptions/tests/runs.rs`: the execution test, spawning
  `CARGO_BIN_EXE_course-subscriptions` and asserting exit status and stdout.
- One `Artefact` row in `xtask/src/proof.rs::ARTEFACTS` naming that package, target and
  test.
- This story's own backlog folder (ledger, report).

**Explicitly not in this PR**

- The `trybuild` compile-fail case, its negative control and their `ARTEFACTS` row —
  the slice-mate's (`compile-fail-proof-artefact`), and still blocked on `trybuild`
  landing as a workspace dependency (`_design.md`, *Sign-off*, inputs this design cannot
  supply).
- Any edit to `crates/happenstance/src/**`, `crates/happenstance-core/src/**` or
  `crates/happenstance-testkit/src/**`. The crate-root mount and the module-doc bullets
  are M2/M3's; if a name the example needs is missing or misshapen, raise it as a slice
  finding and, if it is a contract defect, log it for AC-012.
- `spec/SPECIFICATION.md`, `CHANGELOG.md`, `crates/happenstance/README.md`, any version
  number, the fifth `wasm32` step, and the `happenstance-macros` verdict itself — M7's.
- The given/when/then DSL and `FaultyStore`/`GappyMemoryStore` — M4's. The example is a
  demonstration, not a test-double showcase; it stays uncontended and single-threaded.
- The projection runner. The example remains a write-path demonstration; A5's runner is
  M5's and is feature-gated off by default (`_design.md`, *Visibility and stability*).

The implementer **may** also touch the composition-root/wiring files named in the
Integration contract to mount this slice — the manifest and the `ARTEFACTS` row are the
mount, not scope drift.

**Merge DoD.** `cargo run -p course-subscriptions` completes the seven-section
transcript with no `todo!()` and no panic; the new test executes the binary and is green
inside `cargo test --workspace --all-features`; `cargo xtask proof-artefact` fails if
that test is deleted or emptied; `grep` for `happenstance_core`, `parse_capacity`,
`into_bytes`, `AppendCondition` and `read_decision_model` under
`examples/course-subscriptions/` returns nothing; and `cargo xtask affected --base main`
is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The binary runs to completion | `cargo run -p course-subscriptions` exits 0, having performed define → duplicate-define refusal → two subscribes → duplicate-subscribe refusal → capacity refusal → unsubscribe + re-subscribe → final log. No `todo!()`, no panic, no `unwrap` on a fallible path | `examples/course-subscriptions/src/main.rs:37-79`; `initiative.md:360-362` |
| The transcript is unchanged | Seven `== … ==` sections in today's order and wording, blank line before each but the first, result lines indented 3, refusals prefixed `rejected: `, then `== final log ==` with `{:>3}  {:<22} {:?}` rows | `_design.md`, *Composition → `worked-example-transcript`*; *Anti-patterns* 9-11 |
| Refusals carry values | `course c1 is already defined`, `student s1 is already subscribed to c1`, `course c1 is full (2/2)`, `student s1 is not subscribed to c1` — the value, never a category | `examples/course-subscriptions/src/main.rs:94,162,165,189`; RS-30-4/30-5 via `_design.md`, *States → Error* |
| The domain is one enum | A `#[derive(Debug, Serialize, Deserialize)]` enum covering `CourseDefined`, `StudentSubscribed`, `StudentUnsubscribed`, with `impl DomainEvent` supplying `const EVENT_TYPES`, `event_type()`, `tags()`, `encode`, `decode`. The three bare `const &str` names (`:29-31`) become `EventType::from_static` entries in `EVENT_TYPES` | `_design.md`, *Signatures*, `DomainEvent`; `crates/happenstance-core/src/event.rs:108` (`from_static` is `const`) |
| The fold is exhaustive over the enum | Each decision model's `apply(&mut self, Self::Event)` matches the enum with no `_ => {}` arm. Today's `_ => {}` (`:154`) disappears — it is the arm that made the twice-named set silent | `project.md`, AC-001; `_decomposition.md`, DR-01 |
| The query is derived, never written | Handlers call `Boundary::query` off the models' `EVENT_TYPES` and validated `scope`; no `QueryItem::new` or `Query::from_items` literal survives in the example | `_design.md`, *Shape decision*, `DecisionModel::query` and `Boundary::query` rows; today's `:87-90`, `:114-125`, `:178-181` |
| `subscribe` composes two models | The capacity/seats model and the this-student model are separate `DecisionModel`s passed as a tuple boundary `(B1, B2)`; the OR'd query is the union of their fragments, and each absorbs only what it nominated | `decision-model-composition/spec.md`; `_design.md`, *Shape decision*, Composition row; `examples/course-subscriptions/src/main.rs:114-125` (the shape being replaced) |
| Scopes are validated once, at construction | Each model holds a `Tags` built with `Tags::from_pairs([...])?` in its constructor and returns `&Tags` from `scope()` — the DT-2 resolution, ceremony paid where the caller was already writing `?` | `_design.md`, *Shape decision*, `DecisionModel::scope` row; `crates/happenstance-core/src/tag.rs:304` |
| Payloads are codec-encoded | `Enrolment`-style variants carry their own data (`capacity`, `student`) and are encoded/decoded through `Codec`; no byte literal, no `format!`, no `into_bytes()`, no `parse_capacity` | `_design.md`, *Signatures*, `Codec`; today's `:97-100`, `:168`, `:192`, `:231-237` |
| Capacity comes from a decode | The capacity model's `apply` reads `capacity` off the decoded variant. `unwrap_or(0)` disappears with the function that held it | `examples/course-subscriptions/src/main.rs:231-237` |
| Every handler goes through the loop | `happenstance::commit(&store, boundary, retry, |b| …)` with an explicit `Retry`; the closure returns `Vec<Event>` on acceptance and the handler's own error on refusal. No `AppendCondition`, `read_decision_model` or `AppendError` match remains in the example | `_design.md`, *Signatures*, `commit`; `command-loop/spec.md`, *Behavior and interfaces* |
| The retry bound is visible | The bound appears at the call site (`Retry::attempts(...)` or `Retry::once()`), not defaulted inside the loop. The example is uncontended, so `Committed.attempts` is 1 on every commit — visible bound, unexercised path, and the exercised path is M4's `FaultyStore`'s to prove | `_design.md`, *Shape decision*, Retry-bound row; AC-U13 |
| The example installs the crate a user installs | `examples/course-subscriptions/Cargo.toml` names `happenstance` (`std`, `memory`, `json`) and `serde` (`derive`); no `happenstance-core` entry and no `happenstance_core::` path in the source | `_decomposition.md`, *Composition roots*, item 3; `crates/happenstance/src/lib.rs:75`; root `Cargo.toml:48` |
| Execution instrument | `examples/course-subscriptions/tests/runs.rs` runs `std::process::Command::new(env!("CARGO_BIN_EXE_course-subscriptions"))`, asserts `status.success()`, and asserts the stdout contains each `== … ==` marker in order and all three `rejected: ` lines. Piped stdout, no TTY assumption. No new dependency | `_decomposition.md`, *Notes → AC-003 needs an execution instrument*; `xtask/src/main.rs:143-154` collects it |
| Gate registration | One `Artefact { package: "course-subscriptions", target: "runs", tests: […] }` row in `xtask/src/proof.rs::ARTEFACTS`, so `cargo xtask proof-artefact` fails on a deleted *or* emptied test rather than reporting `running 0 tests` | `xtask/src/proof.rs:132-149`; `xtask/src/main.rs:174-177` |
| Line budget | No transcript line exceeds 80 columns; no event-type name grows past today's 22-character column; a long tag list wraps to a continuation line indented 8 | `_design.md`, *Density budget → Terminal transcript*; *Mock*, finding 5 |
| Boilerplate is written plainly | The `DomainEvent` impl is ordinary visible code — no local macro, no helper trait, no blanket that shrinks it — because AC-013's ratio is measured off this file | `RUNBOOK.md:524`; `_design.md`, *The doctest*, closing note |
| Flavour discipline | Nothing here introduces `#[async_trait]`; the example binds concrete `MemoryEventStore` and any generic helper it writes binds `EventStore`, never `SendEventStore` | `CLAUDE.md`, binding constraints 1 and 4 |
| Frozen crate untouched | No edit to `crates/happenstance-core/**` and no `[FROZEN]` clause amended; findings are logged with clause IDs for AC-012 | `project.md`, *Out of scope*, last bullet; AC-A02 |

## Data and migrations

**N/A — and the reason is worth stating, because one thing here looks like a migration
and is not.**

There is no database, no schema and no persisted state. The example constructs a fresh
`MemoryEventStore` on every run (`examples/course-subscriptions/src/main.rs:35`) and
`RUNBOOK.md:3977-3979` is explicit that *"`MemoryEventStore` is all this project
needs"*. Nothing survives the process, so nothing can need backfilling.

The payload **encoding** does change — from bytes assembled by `format!` to bytes
produced by a `Codec` — but because no log outlives a run, no event written by the old
shape is ever read by the new one. That is precisely why the encoding change is
affordable here and would not be later, and it is the reason payload evolution is
ADR-0021's question rather than this story's (`_storymap.md`, M1,
`adr-0021-payload-evolution-and-codec-tag`).

Two related non-obligations, named so silence is not read as an oversight:

- **The codec tag's home is not this story's to observe.** Whether it lands in
  `Event::metadata` (`crates/happenstance-core/src/event.rs:379`) or in `Tags` is
  ADR-0021's, and the binding constraint is that *no adapter may need to understand it*
  (`_design.md`, *Shape decision*, closing note). The example must not read, assert on
  or print the tag — if the transcript's final log would render it, that is a finding,
  not a feature.
- **`Cargo.lock` may move**, because the example gains `serde` and drops a direct
  `happenstance-core` edge. Both crates are already in the graph
  (root `Cargo.toml:48`, `:24`), so the expected delta is edges, not nodes; a new
  transitive node appearing is worth a second look before it is committed.

## Acceptance criteria

Eight criteria. Each is stated from **P1's** goal — *"model their domain's consistency
boundary once, against a contract, and defer which database to a decision they can
revisit later"* — and against P1's fear, *"being the one who discovers a contract defect
in production, after they have already built on it"*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:53-55,68-70`).
This story is activity **A6** of that journey — *"see it actually work"*
(`_storymap.md`, *Backbone*) — and P4, the one-shot evaluator, reads the same artefact.
Together they discharge project **AC-003** (`project.md`, *Acceptance criteria*) and
initiative **DoD 1** (`initiative.md:359-362`).

Every path below is created by this PR. `examples/course-subscriptions/tests/` does not
exist today — verified — so `runs.rs` is a new file and a new test target, which is why
AC-008 exists at all.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **P1 gets a demonstration, not a claim.** GIVEN P1 has been told a consistency boundary can be modelled before a database is chosen, WHEN the **gate** — not a human at a keyboard — executes the compiled `course-subscriptions` binary, THEN the process exits 0 having performed the whole canonical cycle in order: define c1 → refuse the duplicate definition → subscribe s1 and s2 → refuse s1's duplicate → refuse s3 for capacity → unsubscribe s1 and subscribe s3 into the freed seat → print the final log; with **no `todo!()` reached**, no panic, and no `unwrap` on a fallible path. A step that *compiles* the example does not satisfy this | `examples/course-subscriptions/tests/runs.rs::the_binary_completes_the_dcb_cycle` — spawns `std::process::Command::new(env!("CARGO_BIN_EXE_course-subscriptions"))`, asserts `status.success()`, and asserts stdout carries the seven `== … ==` markers **in order** plus all three `rejected: ` lines. On failure it panics with the captured stdout *and* stderr, so the diagnosis is in the message rather than in a re-run. The wrong implementation it rejects is today's: `cargo test --locked --workspace --all-features` (`xtask/src/main.rs:143-154`) builds the binary and never runs it |
| **AC-002** | **P1 and P4 read the transcript they were promised, and the rewrite is invisible in it.** GIVEN the signed-off surface `worked-example-transcript` (`_design.md`, *Surfaces*; approved 2026-08-12), WHEN the binary's stdout is read at 80 columns with stdout piped and no TTY, THEN the **composition** is the one at `examples/course-subscriptions/src/main.rs:37-79` unchanged — seven `== … ==` markers in that order and wording, a blank line before each but the first, result lines indented 3, every refusal carrying the literal `rejected: ` prefix **and its value** (`course c1 is full (2/2)`, never `capacity exceeded`), then `== final log ==` and one `{:>3}  {:<22} {:?}` row per event — AND the **budget** holds: no line past **80 columns**, no marker past **60**, at most **45 lines** total, a tag list wrapping to a continuation line indented **8** while position and event type never truncate, no event-type name grown past the **22**-column field, and **zero** colour and **zero** motion (`_design.md`, *Density budget → Terminal transcript*; *Composition → `worked-example-transcript`*; *Hierarchy*; anti-patterns 9, 10, 11) | Two tests in `examples/course-subscriptions/tests/runs.rs`, both over the captured stdout of the same spawn: `the_transcript_is_the_designed_composition` (marker set and order, the blank-line rule, the 3-space indent, the `rejected: ` prefix on exactly the three refusals, each refusal's carried value asserted literally, and the final-log row shape by regex over position/type/tags columns) and `the_transcript_holds_its_budget` (per-line column counts, marker widths, total line count, and an assertion that no line contains an ANSI escape `\x1b[` or a `\r`). The unstyled render this rejects is a program that prints only the final log: it satisfies every "it ran" assertion and none of these |
| **AC-003** | **P1 is shown the crate they actually installed.** GIVEN P1 ran `cargo add happenstance` because that is the crate the README points at ([ADR-0006](.kb/decisions/0006-bare-name-to-the-typed-layer.md)), WHEN they open the example that is supposed to show them how to use it, THEN its manifest names **`happenstance`** with `std`, `memory` and `json` and `serde` for the domain enum, carries **no `happenstance-core` entry**, and **no `happenstance_core::` path appears anywhere in its source** — every contract name it still uses (`MemoryEventStore`, `Tags`, `EventType`, `Query`) resolving through `pub use happenstance_core::*;` (`crates/happenstance/src/lib.rs:75`) | `examples/course-subscriptions/tests/runs.rs::the_example_depends_only_on_the_typed_layer` — `include_str!("../Cargo.toml")` asserts the `happenstance` line with its three features and the absence of any `happenstance-core` line; `include_str!("../src/main.rs")` asserts `happenstance_core` appears zero times. The build itself is the other half: if a re-exported name did not resolve through the facade, the package would not compile |
| **AC-004** | **P1's capacity rule is enforced by a decode, not by a byte scraper that fails silently.** GIVEN the old example answered *"what capacity?"* with `parse_capacity`'s `unwrap_or(0)` (`:231-237`), WHEN the rewritten example runs, THEN `parse_capacity`, the `format!("{{\"capacity\":{capacity}}}").into_bytes()` payload (`:97-100`), the `&b"{}"[..]` empty payloads (`:168`, `:192`) and the private `commit` (`:207-224`) are **absent from the file** — deleted, not renamed and not wrapped behind a nicer name — and the capacity the refusal prints arrives from `DomainEvent::decode` through the `json` `Codec` | `examples/course-subscriptions/tests/runs.rs::the_deleted_constructs_are_gone` (source assertions over `include_str!("../src/main.rs")`: zero occurrences of `parse_capacity`, `into_bytes`, `\"capacity\\\":` inside a `format!`, `b\"{}\"`, and no `async fn commit`) plus `capacity_refusal_carries_the_decoded_capacity`, which asserts stdout contains `rejected: course c1 is full (2/2)`. That assertion is the behavioural discriminator: an `unwrap_or(0)`-shaped decode makes the **first** subscribe refuse `(0/0)`, so the wrong implementation cannot reach the right transcript |
| **AC-005** | **P1 cannot leave a new event type half-handled, and the compiler tells them where.** GIVEN the hazard this phase exists for — a DCB handler naming its event set twice, once in a query and once in a fold (`:114-125` and `:140-155` today) — WHEN a variant is added to the example's domain enum, THEN every decision model's `apply` fails to compile until its arm is written, because each `apply` matches the **enum** with no `_ => {}` arm and the query is **derived** by `Boundary::query` from `EVENT_TYPES` and the models' validated `scope` — no `QueryItem::new` or `Query::from_items` literal survives; AND the enum and its folds are ordinary, visible code in `examples/course-subscriptions/src/main.rs`, so the slice-mate's diagnostic `-->` span lands on **P1's own match arm** and not in a macro body or under `crates/` (`_design.md`, anti-pattern 13; AC-U07) | `examples/course-subscriptions/tests/runs.rs::the_event_set_is_named_once` — source assertions: no `QueryItem::new`, no `Query::from_items`, no `match` on `event_type().as_str()`, and no `_ => {}` arm inside any `fn apply`; `the_fold_lives_in_the_users_file` asserts the `enum`, its `impl DomainEvent` and each `fn apply` are declared in `src/main.rs` itself and not produced by a `macro_rules!` or a helper trait. The executable proof of the compile error is the slice-mate's `trybuild` fixture and is explicitly **not** in this PR |
| **AC-006** | **P1 sees the mechanism that makes a dynamic boundary dynamic, not a hand-collapsed shortcut.** GIVEN `subscribe` is the case that motivates DCB — the course's capacity and everyone holding a seat is one concern, this student's own history is another — WHEN it builds its boundary, THEN it passes **two distinct `DecisionModel`s as a tuple** `(B1, B2)` whose OR'd query is the union of their fragments, each absorbing only what it nominated, with no macro invocation in the caller's face and no single hand-written model collapsing the two concerns; and **both** refusals that only the pair can produce fire in the same run — `student s1 is already subscribed to c1` and `course c1 is full (2/2)` | `examples/course-subscriptions/tests/runs.rs::subscribe_composes_two_models` — source assertions: two distinct model types with `impl DecisionModel`, both named at `subscribe`'s single `commit` call as a tuple, and no third `QueryItem`-shaped literal; plus the two refusal lines asserted in `the_binary_completes_the_dcb_cycle`. Collapsing the models into one passes every "it ran" assertion and fails the source half — which is the point, because project AC-004 is scored on the demonstration, not on the outcome |
| **AC-007** | **P1 never writes the read-decide-append-retry cycle, and therefore cannot write it wrong.** GIVEN the loop landed by `command-loop`, WHEN each of the three handlers appends, THEN it goes through `happenstance::commit` with an explicit `Retry` **visible at the call site** and a closure returning the events on acceptance or the handler's own error on refusal; and **no `AppendCondition`, no `read_decision_model` call and no `AppendError` match survives anywhere in the example** — so the `after` anchor comes from the read and never from `append`'s return (`crates/happenstance-core/src/store.rs:131-145`), `conflicting_position` is never branched on (`crates/happenstance-core/src/error.rs:139-147`), and the retry bound is a required argument rather than a hidden default. The example is uncontended, so `Committed.attempts` is 1 on every commit: a visible bound on an unexercised path | `examples/course-subscriptions/tests/runs.rs::the_loop_absorbs_every_handler` — source assertions over `include_str!("../src/main.rs")`: zero occurrences of `AppendCondition`, `read_decision_model` and `AppendError`; exactly three `commit(` call sites; and a `Retry::` spelled at each of them. The behavioural half rides on AC-001: a handler that re-derived the cycle and got the anchor wrong would refuse or duplicate somewhere in the seven-section run |
| **AC-008** | **The proof P1's successor inherits cannot be emptied in silence.** GIVEN this repository has already paid once for a proof artefact that a deletion failed and an emptying passed (`xtask/src/proof.rs:9-23`), WHEN the execution test is deleted **or** truncated to zero tests, THEN `cargo xtask proof-artefact` fails by **name** rather than reporting `running 0 tests` — because `xtask/src/proof.rs::ARTEFACTS` carries a row naming package `course-subscriptions`, target `runs`, and the test names AC-001 and AC-002 rest on | `cargo xtask proof-artefact` green with the row present (it asserts the names out of `cargo test --list` before running them, `xtask/src/proof.rs:175-195`), and the step is already inside the gate at `xtask/src/main.rs:178-190`. The negative control is **rehearsed, not assumed**, exactly as AC-002 of the project requires of its own artefact: empty `runs.rs` locally, confirm the step fails naming the missing test, restore, and cite the rehearsal's output in the ledger row |

## Interaction quality

RFC §6.7/D6. This story renders exactly one of `_design.md`'s six surfaces —
**`worked-example-transcript`** — and its medium is a terminal with stdout piped. So
*presentation* here means composed text structure a human reads top to bottom, and the
**unstyled render** to beat is concrete and tempting: a program that does the same
appends and prints only `== final log ==`. It exits 0. It reaches no `todo!()`. It
satisfies every execution assertion in AC-001 and it is not the artefact P4 was pointed
at. The composition rows below are what make that fail.

**Every invariant here is carried by an AC row in the table above.** This section says
which row carries it and how it is verified; it introduces no new obligation.

**STATE invariants.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (AC-U10) — every refusal is actionable where it is printed. The message carries the value (`course c1 is full (2/2)`), so the reader never opens a second document or re-runs with a flag to learn what happened | **AC-002**, **AC-004** | `the_transcript_is_the_designed_composition` asserts each refusal's literal carried value; `capacity_refusal_carries_the_decoded_capacity` asserts the number is the decoded one |
| **Non-occlusion — the filter must not hide what it filtered** (AC-U11) | **AC-002** | `== final log ==` prints **every** event the store holds, not the subset any one boundary selected, so a fold/query divergence shows up as a row nobody's transcript accounted for. The row-shape assertion in `the_transcript_is_the_designed_composition` is what keeps that region present |
| **Preserved place across the rewrite** — the analogue of preserved focus/scroll/selection. A reader who knows this transcript finds it unchanged: same seven sections, same order, same wording. The rewrite happens **beneath** `main`, and a "tidier" transcript is a failed story (`_design.md`, *Composition → `worked-example-transcript`*) | **AC-002** | Marker set, order and wording asserted literally against `main.rs:37-79`'s current text |
| **Reversibility** (AC-U12) — everything before the append is pure; the three refusals leave the store untouched, and the final log proves it by holding exactly the accepted events and no probe write | **AC-001**, **AC-007** | The final-log region is asserted in the same run that asserts three refusals fired; the loop's own purity is `command-loop`'s AC-002 and is inherited, not restated |
| **Bounded exit** — no loop whose only exit is success. The retry bound is a required argument spelled at each call site, even though this uncontended run never spends it | **AC-007** | `the_loop_absorbs_every_handler` asserts a `Retry::` at every `commit(` call site; exercising the bound is M4's `FaultyStore`, named in *Risks* |
| **Keyboard reachability**, in this medium: no interactive prompt, no TTY requirement, no environment variable, no flag. The state `piped-non-tty` is how the gate runs it (`_design.md`, *Surfaces*, `states`) | **AC-001** | The test spawns the binary with piped stdout and no terminal attached and asserts it runs to completion; a program that blocked on input would hang the gate rather than pass it |

**COMPOSITION invariants**, from the signed-off `_design.md` (approved 2026-08-12,
*Sign-off*). These are the rows an unstyled render fails.

| Invariant | Design source | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Presentation exists at all** — every observable step is composed text: a section marker announcing what is about to happen, an indented result, an explicit `rejected: ` prefix. A `{:?}` dump of the log is not this surface | *Composition → `worked-example-transcript`*; *Hierarchy* | **AC-002** | `the_transcript_is_the_designed_composition` fails on a transcript missing any marker, indent or prefix |
| **Composition and placement** — seven `== … ==` sections in the order at `main.rs:37-68`, blank line before each but the first, results indented 3, `== final log ==` **last** | *Composition*, `worked-example-transcript` | **AC-002** | Order asserted as a sequence, not a set; the blank-line rule and the indent asserted per line |
| **Transience** — persistent chrome: the `== … ==` markers and the `rejected: ` prefix, present on every run. Revealed: the final log, which only exists once the cycle has run. Opened on demand: **nothing** — this surface has no flag, no verbosity level and no second mode, and adding one would be a new surface | *Transience policy* (read into this medium) | **AC-002**, **AC-001** | The budget test asserts the total line count, which a hidden verbose mode would blow; the composition test asserts the chrome is present with no flag passed |
| **Density budget, with its real numbers** — ≤ 80 columns per line; ≤ 60 columns per marker; ≤ 45 lines total; tag lists wrap to a continuation line indented 8; position and event type never truncate; the event-type field stays 22 | *Density budget → Terminal transcript* | **AC-002** | `the_transcript_holds_its_budget` counts columns and lines over the real captured stdout |
| **Hierarchy** — markers primary (the only unindented lines, delimiter glyphs, preceded by a blank line); results secondary (3-space indent, `rejected: ` prefix); log rows recessive (column alignment reading as a table) | *Hierarchy*, `worked-example-transcript` paragraph | **AC-002** | The indent assertions are the hierarchy assertions: an unindented result line or an indented marker fails |
| **Named anti-patterns 9, 10, 11** — no colour, no spinner, no percentage, no in-place rewrite; no line past 80 columns and no truncated position or event type; no refusal missing the `rejected: ` prefix or reading as a category rather than a value | *Anti-patterns* | **AC-002** | `the_transcript_holds_its_budget` (9's escape-sequence and `\r` assertions, 10's column counts) and `the_transcript_is_the_designed_composition` (11's prefix and carried-value assertions) |
| **Named anti-pattern 13** — the compile-fail diagnostic must point at the user's own `match` arm, which is only possible if the fold is ordinary code in the example's own file | *Anti-patterns* 13; AC-U07 | **AC-005** | `the_fold_lives_in_the_users_file`. The fixture that reads the span is the slice-mate's and is out of this PR's boundary |

**Anti-patterns 1–8, 12, 14 and 15 do not apply to this story**, and are named so the
silence is not read as an oversight: 1–6 belong to the crate-root rustdoc page and 7–8 to
the README (both M7's), 12 to the DSL failure message (M4's), 14 to `happenstance`'s item
table (M2/M3's), and 15 to the projection runner (M5's). Mock finding 3 — anti-pattern 5
being unfixable inside this project — likewise belongs to the rustdoc surface and not to
this one.

## Error conditions

The first three are **not failures**: they are the refusals the transcript exists to
show, and the run is only correct if all three occur. The rest are the ways this PR can
actually go wrong.

| id | condition | behaviour | evidence |
| --- | --- | --- | --- |
| **EC-001** | A course is defined twice | The handler's decision refuses; `main` prints `   rejected: course c1 is already defined` and continues. The refusal is a `CommandError::Refused` carrying the handler's own error type, not a store error | `examples/course-subscriptions/src/main.rs:41-45,94`; **AC-002** |
| **EC-002** | A student subscribes twice to the same course | `   rejected: student s1 is already subscribed to c1`. Only the composed pair can decide this — the per-student model is the one that sees it | `:52-56,162`; **AC-006** |
| **EC-003** | A subscription would exceed capacity | `   rejected: course c1 is full (2/2)`, with the capacity read off the decoded `CourseDefined` variant | `:58-62,165`; **AC-004** |
| **EC-004** | A refusal that was expected does **not** happen | `main`'s own negative controls `bail!` (`:43`, `:54`, `:60`) and the process exits non-zero, which fails AC-001's `status.success()`. These three arms survive the rewrite verbatim — they are the reason the transcript is a proof rather than a printout | `:43,:54,:60`; **AC-001** |
| **EC-005** | An event in the read cannot be decoded into the domain enum | `CommandError::Decode { position, source }` propagates out of the handler and out of `main`; the position names *which* event. **Not reachable in a single run** — the store is fresh and every event in it was written by this binary — and it must not be swallowed by a `_ => continue`, because that is the fold/`EVENT_TYPES` disagreement ADR-0020 exists to make loud | `_design.md`, *The states the API must express*, **Absent**; **AC-005** |
| **EC-006** | Every attempt is contended | `CommandError::Exhausted { attempts, source }`. **Unreachable here**: one process, `current_thread` flavour, no second writer. The bound is visible and the path is unexercised, and exercising it is M4's `FaultyStore`, not this story's — see *Risks* | `_design.md`, *Shape decision*, Retry-bound row; **AC-007** |
| **EC-007** | `unsubscribe` is called for a student who is not subscribed | `student s1 is not subscribed to c1` (`:189`). Also **not exercised** by `main`, today or after: the only `unsubscribe` call is for a subscribed student. The message is preserved in substance and this story does **not** add a step to exercise it, because *"`main`'s observable steps survive verbatim"* outranks coverage here (`_design.md`, *Composition*) | `:64-67,189`; *Clarifications*, item 6 |
| **EC-008** | The execution test fails | It panics with the captured stdout **and** stderr and the assertion that failed. A test that asserts `status.success()` and discards the output makes every failure a re-run, which is the non-occlusion rule applied to the instrument itself | **AC-001** |
| **EC-009** | The binary panics or reaches a `todo!()` | Non-zero exit, and AC-001 fails. This is the condition DoD 1 is written against, and until this PR nothing in the gate could observe it | `initiative.md:359-362`; **AC-001** |

## Non-functional

| id | requirement | why, and how it is met |
| --- | --- | --- |
| **NF-001** | No `unwrap`, `expect`, `panic!` or panicking index on any path the run reaches | `standards/rust/00-prime-directives.md`; `[lints] workspace = true` in `examples/course-subscriptions/Cargo.toml:15-16` and `cargo xtask affected --base main` runs clippy with `-D warnings`. `#![allow(clippy::print_stdout)]` (`:21`) stays and remains the file's **only** allow — an example whose job is printing earns exactly that one |
| **NF-002** | The mapping boilerplate is written **plainly** — no local `macro_rules!`, no helper trait, no blanket impl that shrinks the `DomainEvent` impl | AC-013's verdict is a ratio measured off *this file* (`RUNBOOK.md:524`), and `_design.md` records a falsifiable prediction of 2.4:1 against the domain to be *"checked against the rewritten example, not against this doctest"* (*The doctest*, closing note). Hiding the impl answers AC-013 by concealment. This is a non-functional constraint rather than an acceptance criterion because the **verdict** is M7's record (`defect-log-and-macros-verdict`), not this PR's pass/fail — see *Clarifications*, item 4 |
| **NF-003** | The execution instrument adds **no** dependency | `CARGO_BIN_EXE_<name>` is a Cargo-provided env var for an integration test of a package with a binary target, and `std::process::Command` is in `core`'s company already. `assert_cmd`, `trybuild` and every similar crate are absent from this workspace and stay absent (`_decomposition.md`, *Notes → AC-003 needs an execution instrument*) |
| **NF-004** | The example's dependency set stays four entries | `happenstance` (`std`, `memory`, `json`), `serde` (its workspace entry already carries `derive`, root `Cargo.toml:48`), `anyhow` (`:81`) and `tokio` (`:77`). A refusal type needing `thiserror` (`:70`) is the one admissible fifth — see *Implementation notes* |
| **NF-005** | The MSRV floor of 1.97.1 does not move | [ADR-0029](.kb/decisions/0029-msrv-raised-to-1-97-1.md). Every crate this example gains is already in the workspace graph, so no new build script can raise the floor underneath us — which is precisely how the floor moved last time (`CLAUDE.md`, binding constraint 5) |
| **NF-006** | Flavour discipline survives the rewrite | No `#[async_trait]`; the example binds the concrete `MemoryEventStore`, and any generic helper it writes binds **`EventStore`**, never `SendEventStore`, importing one flavour name per module (`CLAUDE.md`, binding constraints 1 and 4; `standards/rust/20-two-flavour-ports.md`) |
| **NF-007** | The run is deterministic and fast enough to live in `cargo test` | One process, `#[tokio::main(flavor = "current_thread")]` (`:33`), an in-memory store, no filesystem and no network. Deterministic output is what makes string assertions legitimate rather than flaky; if any assertion needs a retry or a sleep, the transcript has become non-deterministic and that is a finding |
| **NF-008** | The frozen crate is not edited | No change under `crates/happenstance-core/**` and no `[FROZEN]` clause amended. Anything this use reveals is written down with its clause ID and routed to a decision record for **AC-012** (`project.md`, *Out of scope*, last bullet; AC-A02). `_design.md` already carries **D-1**; expect more |

## Implementation notes (non-prescriptive)

Shape suggestions only. `_design.md`'s signatures and the transcript's composition are
binding; the arrangement below is not.

- **Write the execution test first, against the example as it stands today.** It is the
  only artefact in this PR that can be red before anything is rewritten, and a green
  `runs.rs` over the *old* binary proves the instrument works before the rewrite starts
  moving the thing it observes. Then the transcript assertions become a regression
  harness for the rewrite rather than an after-the-fact claim about it.
- **The refusal type is the one thing the old code did not have.** `commit`'s `D` is
  bound `core::error::Error + 'static`, and `anyhow::Error` does **not** implement
  `Error`, so `bail!` cannot cross the closure boundary. A small `enum Refusal` with
  `thiserror` (`Cargo.toml:70`) — or a hand-written `Display` + `Error` pair — carrying
  `AlreadyDefined { course }`, `AlreadySubscribed { student, course }`,
  `Full { course, taken, capacity }` and `NotSubscribed { student, course }` is the
  cheapest shape that keeps every refusal's **value** in the type rather than in a
  formatted string (RS-30-4/30-5), and its `Display` strings are what the transcript
  prints — so write them to match `:94`, `:162`, `:165` and `:189` exactly.
- **Three models, not two.** `define_course` needs one (has this course been defined),
  `unsubscribe` needs one (this student's history), and `subscribe` composes the seats
  model with the per-student model as a tuple. The per-student model is shared between
  `subscribe` and `unsubscribe`, which is the demonstration that a decision model is a
  reusable value rather than a per-handler blob.
- **Each model's constructor is where the `?` goes.** `Tags::from_pairs([...])?`
  (`crates/happenstance-core/src/tag.rs:304`) is fallible, the model holds the result,
  and `scope()` returns `&Tags` — the DT-2 resolution, ceremony paid once where the
  caller was already writing `?` (`_design.md`, *Shape decision*, `DecisionModel::scope`).
- **`EVENT_TYPES` is `const`-constructed.** `EventType::from_static` is a `const` fn
  (`crates/happenstance-core/src/event.rs:108`), so the three bare `const &str`s at
  `:29-31` become entries in the array and then disappear as separate items. Do not index
  `EVENT_TYPES` by a computed position in `event_type()` — match the variant and clone
  the entry, which is the shape `_design.md`'s doctest uses (`*The doctest*`).
- **Keep `main` untouched while you work.** The temptation to adjust a section marker
  while rewriting beneath it is exactly what AC-002 is written to catch; if a marker
  genuinely must change, that is a design change and it is escalated, not applied.
- **The `ARTEFACTS` row is one entry with two names.** Register the two tests AC-001 and
  AC-002 rest on (`the_binary_completes_the_dcb_cycle`,
  `the_transcript_is_the_designed_composition`); `proof.rs`'s check is deliberately a
  **subset** check (`xtask/src/proof.rs:33-39`), so a fourth test later needs no gate
  edit. Note that `cargo_args` passes `--all-features` (`:161-173`) — harmless here, the
  package has none — and that the slice-mate edits the same `const` for AC-002 of the
  project: one file, two rows, one context.
- **Order of work that keeps the tree green:** `runs.rs` against today's binary → the
  domain enum and its `DomainEvent` impl (compiles against M2's names alone) → the three
  models → the manifest swap → the handlers onto `commit` → delete the three constructs
  → the `ARTEFACTS` row → re-run the transcript assertions. The deletions come *after*
  the replacements compile, so the tree is never in a state where the example neither
  builds nor demonstrates.
- **When a name you need is missing, stop.** Every public item this example uses was
  landed by M2/M3. Needing a new one is a finding about those stories — raise it in the
  slice; do not add a public item here (*PR boundary*).

## Tests and CI (merge gate)

Grounded in `_decomposition.md`, *Testing brief* — the **AC-003** row (*"E2E (real
execution)"*) and the four grains `.redkiln/config.yaml`'s `verify:` block wires. This
project's integration bar is `cargo xtask ci --fast`, **not** the full gate:
`project.md`'s frontmatter carries `terminal: false`.

| tier | command / path | proves |
| --- | --- | --- |
| E2E (real execution) | `cargo test -p course-subscriptions --test runs` → `examples/course-subscriptions/tests/runs.rs` | **AC-001** and **AC-002** — the binary is spawned, exits 0, and its stdout is the designed composition inside its budget. This is the tier that did not exist before this PR, and it is the only one that can observe DoD 1 |
| Source lint (same target) | the same command; the `include_str!`-based tests in `runs.rs` | **AC-003**, **AC-004**, **AC-005**, **AC-006**, **AC-007** — the manifest and import surface, the three deletions, the once-named event set and the visible fold, the tuple composition, and the absorbed loop. File-reading assertions are first-class in this repository (`cargo xtask lints`), and keeping them in the test target rather than adding a sixth `xtask` lint is deliberate (*PR boundary*) |
| Gate registration | `cargo xtask proof-artefact` → `xtask/src/proof.rs::ARTEFACTS` | **AC-008** — the named tests are asserted out of `--list` before they run, so an emptied `runs.rs` fails a step a deleted one also fails. Already wired into the gate at `xtask/src/main.rs:178-190`; no new step is added |
| Negative control (manual, rehearsed) | empty `runs.rs`, run `cargo xtask proof-artefact`, restore | **AC-008**'s discriminator. *"Verify the negative control fails rather than assuming it"* (`project.md`, *Risks*, row 4). The output is cited in the ledger row |
| Story grain (merge gate) | `cargo xtask affected --base main` | fmt, `clippy -D warnings` and tests for every package this diff touches plus dependents — `course-subscriptions`, `xtask`, and whatever `happenstance` change the slice carries. Wired by `.redkiln/config.yaml:28-40`, so it runs whether or not anyone types it. **NF-001** rides on the clippy half |
| Workspace test | `cargo test --locked --workspace --all-features` (`xtask/src/main.rs:143-154`) | Collects `runs.rs` with no new wiring, because `examples/*` are workspace members (root `Cargo.toml:3`). Before this PR this same command compiled the example and proved nothing about it running |
| Integration grain (merge gate) | `cargo xtask ci --fast` | This project's bar: `REQUIRED` without `OPTIONAL`, all four `wasm32` steps kept (`.redkiln/config.yaml:50-55`) |
| Manual, once | `cargo run -p course-subscriptions` | The literal sentence DoD 1 is written in. The automated tier above is what keeps it true; this is what a human reads at review |
| **Not this story's gate** | `cargo xtask ci` (full) — feature powerset, `cargo deny`, nightly `--cfg docsrs` | Named so its absence is not mistaken for coverage. The powerset is what proves the codec features compose, and it is M7's pre-publish gate (`_decomposition.md`, *The feature powerset is not this project's gate*) |

**Merge DoD, as a command sequence.** `cargo test -p course-subscriptions --all-targets`,
then `cargo xtask proof-artefact`, then the emptying rehearsal and restore, then
`cargo run -p course-subscriptions` read by a human, then
`cargo xtask affected --base main`. Every AC row above has a passing real-path test and
its ledger row cites it.

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation inside this PR |
| --- | --- | --- |
| **Both slice-halves edit `xtask/src/proof.rs::ARTEFACTS`** | This story adds the `course-subscriptions` row; `compile-fail-proof-artefact` adds the `trybuild` row for project AC-002. Same `const`, same context, same milestone | Add one row, leave the list's ordering and its subset-check semantics alone, and let the slice-mate append. A conflict here is a merge inside one context, not a coordination problem across PRs (*Context pack*, item 8) |
| **The slice-mate is blocked on `trybuild`, and this story is not** | `trybuild`'s adoption is HS-P0010's decision and it is absent from the workspace (`_design.md`, *Sign-off*, inputs this design cannot supply) | This story ships whole regardless: its instrument needs no dependency (NF-003). Do **not** substitute a `compile_fail` doctest for the slice-mate — rustdoc collects doctests from the lib target only and silently ignores an unmatched error code, so the negative control cannot discriminate (`RUNBOOK.md:3614-3627`). Escalate |
| **`commit`, `Boundary` and the tuple impl arrive from stories implemented in the same context** | Every call site here rests on `command-loop` and `decision-model-composition`, and on `domain-event-and-decision-model` and `codec-and-feature-forwarding` beneath them | Bind exactly the signatures in `_design.md`, *Signatures*. A name that arrived with a different shape is a **blocking** discrepancy against a signed-off design: escalate rather than adapt silently, because adapting here hides the defect in the consumer that exists to find it |
| **A "tidier" transcript is the most likely accidental failure** | The rewrite touches every line beneath `main`, and a rewritten handler naturally wants a rewritten message | AC-002 asserts the markers, the order, the indents and the carried values literally. Write the `Refusal` `Display` strings to match `:94`, `:162`, `:165`, `:189` before writing the handlers |
| **The retry path ships visible and unexercised** | `Retry` appears at three call sites and the uncontended run never spends an attempt, so nothing here would notice a loop that ignored its bound | Accepted, and named rather than papered over: `command-loop`'s own tests cover the bound, and `FaultyStore` (M4, `misbehaving-testkit-stores`) is the public instrument. This example must **not** grow a contention harness — it is a demonstration, not a test-double showcase (*PR boundary*) |
| **AC-013's ratio is measured off this file, and a helpful abstraction destroys the measurement** | The `DomainEvent` impl is verbose by design, and shrinking it is the natural instinct of anyone reading it twice | NF-002, plus the sequencing: M7's `defect-log-and-macros-verdict` reads this file. If the ceremony reads as too much, the answer is AC-013's derive, not a local macro (`_design.md`, *Sign-off*, item 1) |
| **The first real application above a `[FROZEN]` contract is where defects appear** | A one-line edit to `happenstance-core` will look cheaper than a decision record every time | NF-008 and AC-A02. `_design.md`'s **D-1** is the worked precedent: name the clause, write the entry, route it to AC-012's log. Incidental bugs go to the `support` initiative (`.redkiln/config.yaml:5`), not into this PR |
| **The example is `publish = false` and therefore easy to under-gate** | Nothing about a non-published package forces it into the gate; that is exactly how it stayed unexecuted this long | AC-008. The `ARTEFACTS` row is the mount that makes the omission loud, and it is the second mount of this story rather than a nicety |

## Dependencies

**Blocks on** — both are slice-predecessors in **M6**'s own milestone chain, implemented
in the same context and merged as one integrated surface:

- **`command-loop`** — `happenstance::commit`, `Retry`, `Committed` and `CommandError`.
  Every handler in this example is a call to that function; without it there is nothing
  to delete the private `commit` (`:207-224`) *into*, and AC-007 has no subject.
- **`decision-model-composition`** — `impl Boundary for (B1, B2)`. `subscribe`'s two
  consistency concerns are the case that motivates DCB, and AC-006 is unwritable without
  the tuple impl. Collapsing them into one hand-written model to unblock is the failure
  this dependency exists to prevent.

Transitively, through those two: **`domain-event-and-decision-model`** (the `DomainEvent`
and `DecisionModel` traits and the sealed `Boundary`) and **`codec-and-feature-forwarding`**
(`Json`, `Codec`, `CodecError` and the `json` feature). Both are M2/M3 and both must be
present before the first line of the rewrite compiles.

**Unlocks**

- **`compile-fail-proof-artefact`** (same milestone, next in the chain) — its fixture adds
  a variant to *this* example's domain enum and requires the `-->` span to land on this
  example's own `match` arm. AC-005's *the fold lives in the user's file* is the
  precondition it consumes.
- **`defect-log-and-macros-verdict`** (M7) — AC-013's ratio is measured off this rewrite
  (`RUNBOOK.md:524`), and this story's contract defects feed AC-012's log.
- Initiative **DoD 2**, whose compile-fail case is written against this example, and
  initiative **DoD 1**, which this story is the whole of.

## Anchors (progressive disclosure)

Link, do not paste. Each row says why the artefact is load-bearing and the moment to open
it. Every path was confirmed present in this tree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `examples/course-subscriptions/src/main.rs` | The mount point and the thing being rewritten. `:37-79` is the transcript that must survive verbatim; `:86-224` is what disappears beneath it; `:231-237` is the byte scraper AC-004 deletes | First, before any other file — read `main` end to end and do not edit it | AC-002 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | The **binding** signed-off design. *Composition → `worked-example-transcript`*, *Density budget → Terminal transcript*, *Hierarchy* and *Anti-patterns* 9-11 and 13 are the whole of AC-002 and AC-005's second half; *Signatures* is what every call site must match | Before writing `the_transcript_holds_its_budget`, and again before the first handler | AC-002 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/command-loop/spec.md` | The predecessor's contract: `commit`'s signature, the after-anchor rule, the `conflicting_position` discipline and what `CommandError` carries. This example consumes all four and must restate none | Before rewriting the first handler, to bind exactly what M6's predecessor shipped | AC-007 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/decision-model-composition/spec.md` | The tuple `Boundary` impl and its OR'd-query semantics — what "each model absorbs only what it nominated" actually means at the call site | Before writing `subscribe`, which is the only composed handler | AC-006 |
| `xtask/src/proof.rs` | The gate mount. `:1-48` is the argument for why naming a target is not enough; `:132-149` is the list to append to; `:161-195` is what the row's names must satisfy | When adding the `ARTEFACTS` row, and again when rehearsing the emptying | AC-008 |
| `xtask/src/main.rs` | `:143-154` is the test step that already collects `runs.rs`; `:174-190` is the proof-artefact step and its comment explaining `running 0 tests`; `:105` is `REQUIRED`, which this story does **not** edit | When deciding whether a new gate step is needed — it is not — and when citing evidence | AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | *Composition roots* items 2-4 name the three mounts; *Notes → AC-003 needs an execution instrument, not a compile check* is the brief that hands this spec the instrument question; the testing brief's AC-003 row is the tier | Before writing `runs.rs`, to see the gap stated in the brief's own words | AC-001 |
| `crates/happenstance/src/lib.rs` | `:75`'s `pub use happenstance_core::*;` is why the contract names still resolve after the dependency moves; `:44-46` is the prose about composition this story turns into running code | When the manifest swap is made and a contract name fails to resolve | AC-003 |
| `crates/happenstance-core/src/event.rs` | `:108`'s `const` `EventType::from_static` is what makes `EVENT_TYPES` const-constructible; `:38-58` is the `Cow` payload that forbids returning `&'static EventType` | When writing the `DomainEvent` impl's first two items | AC-005 |
| `crates/happenstance-core/src/tag.rs` | `:304`'s `Tags::from_pairs` is fallible and is the only way in, which is why each model validates its scope in its constructor | When writing the models' constructors | AC-006 |
| `crates/happenstance-core/src/store.rs` | `:131-145` is the port's own prohibition on threading `append`'s return into the next condition — the defect the loop exists to make unwritable, and the one this example must not reintroduce by hand | Only if a handler is tempted to touch an `AppendCondition` — which AC-007 forbids | AC-007 |
| `crates/happenstance-core/src/error.rs` | `:139-147` records that `conflicting_position: None` is legitimate. The example must never branch on it, directly or through a helper | Same moment as the row above | AC-007 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | The accepted decision that makes `happenstance` the crate an application installs and `happenstance-core` the one adapter authors pin. It is why moving the manifest is a claim and not a tidy-up | Before editing `examples/course-subscriptions/Cargo.toml` | AC-003 |
| `standards/rust/30-error-taxonomy.md` | RS-30-4/30-5 — an error carries the value, not a category. The `Refusal` type's variants and their `Display` strings are governed by it, and the transcript's refusals are its rendered form | When writing the refusal enum, before the handlers | AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P1's goal and fear at `:53-55,68-70` and P4's one-shot sitting at `:249`+ — the intent every AC above is framed from, and the reason a demonstration outranks a passing test | When an AC's framing is in doubt, or when tempted to trade the transcript for coverage | AC-001 |
| `RUNBOOK.md` | `:3971-4102` is phase 7's goal and exit criteria; `:524` is AC-013's criterion, measured off this rewrite; `:3977-3979` is *"`MemoryEventStore` is all this project needs"* | Before deciding anything looks like scope — the runbook usually already assigned it elsewhere | AC-001 |

## Clarifications resolved during spec

1. **Where the execution instrument lives, decided here.** The testing brief handed the
   choice to `_design.md` (*"whether that test lands in `tests/` … or as an explicit new
   `xtask` step is `_design.md`'s call"*), and `_design.md` is silent on it. This spec
   decides: `examples/course-subscriptions/tests/runs.rs`, collected by the existing
   `cargo test --locked --workspace --all-features` step because `examples/*` are
   workspace members (root `Cargo.toml:3`) — **no new gate step** — plus one `ARTEFACTS`
   row so deletion and emptying both fail loudly. A new `xtask` step was rejected: it
   would add a second place the example is named and would still need `proof.rs`'s
   name-assertion to be non-decorative.
2. **The eight AC ids the front half fixed are unchanged; none added, none dropped.**
   Composition and the density budget share **AC-002** rather than splitting into a ninth
   row, because both are read off the same captured stdout by the same pair of assertions
   and a reviewer checking one is already holding the other. The *Interaction quality*
   tables keep the invariants individually named and individually verified, which is
   where the granularity belongs.
3. **No new dependency, in either direction.** `assert_cmd` is not adopted (NF-003), and
   `trybuild` is not substituted for (it is the slice-mate's blocked input). The one
   admissible manifest growth beyond `happenstance` and `serde` is `thiserror` for the
   refusal type, and only because `CommandError`'s `D` is bound `core::error::Error`,
   which `anyhow::Error` does not implement — a real constraint discovered while writing
   this spec, not a preference.
4. **"Write the boilerplate plainly" is NF-002, not an acceptance criterion.** It is
   binding, but its instrument is a *ratio recorded at closeout* by M7's
   `defect-log-and-macros-verdict`, not a pass/fail assertion in this PR. Making it an AC
   would put a row in the ledger that this story cannot honestly flip.
5. **This story adds no public item and no conformance rule, and both silences are
   deliberate.** The testkit's suite observes adapters; this is an application above the
   port. Falsifiability comes from the execution test and its gate registration
   (AC-001, AC-008), which is why AC-008 exists as a criterion rather than as a chore.
6. **`unsubscribe`'s "not subscribed" refusal (`:189`) stays unexercised.** Adding an
   eighth section to demonstrate it would improve coverage and violate the signed-off
   composition, which says `main`'s observable steps survive verbatim. Recorded as
   **EC-007** so the gap is visible; if it is ever worth closing, that is a design change
   to `worked-example-transcript`, not an implementer's call.
7. **`Cargo.lock` is expected to move**, by edges rather than nodes: `serde` and
   `happenstance` are already in the graph. A new transitive **node** appearing means
   something was pulled in that nobody decided on — look before committing.
8. **A missing name escalates rather than widens.** If the example cannot be written
   without a public item M2/M3 did not land, that is a finding about those stories routed
   through the slice; and if the gap is in `happenstance-core`, it is a defect logged with
   its clause ID for AC-012 (NF-008), never a line edit to a `[FROZEN]` crate.
