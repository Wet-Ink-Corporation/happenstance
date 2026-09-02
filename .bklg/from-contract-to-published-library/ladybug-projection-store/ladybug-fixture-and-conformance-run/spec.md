---
item: HS-S0078
stage: spec
created: 2026-08-12T13:47:16.087Z
updated: 2026-08-12T13:47:16.087Z
template_sig: 87bbf1d0
rendered_sig: 69851cc6
---

# Spec — `LadybugFixture` and the projection conformance run, registered as a proof artefact

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — Goals, DoD 7, BR-03/BR-04ʳ |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` — AC-004, DR-5, DR-9 |
| This spec | `.bklg/from-contract-to-published-library/ladybug-projection-store/ladybug-fixture-and-conformance-run/spec.md` |
| Key briefs | `.bklg/…/ladybug-projection-store/_decomposition.md` — architecture **M1** (the conformance run), **M2** (capability declension), **M3** (`port_shape.rs`), **M4** (the proof-artefact registry); testing brief's **AC-004** row and *Fixtures / seams* |
| Story map row | `.bklg/…/ladybug-projection-store/_storymap.md:44` (slice `conformance-run`), rationale at `:71-78` |
| This story's discover | `.bklg/…/ladybug-fixture-and-conformance-run/discover.md` — signal ledger, the three wrong implementations |
| Signed-off design | `.bklg/…/ladybug-projection-store/_design.md` — **no user-facing surface**; approved 2026-08-12. This story renders no surface and adds no public API item. |
| Roadmap pointer | `RUNBOOK.md:4393-4444` — phase 11; its proof artefact is *"the projection suite green on a third shape"* (`RUNBOOK.md:4427-4430`) |

## One-line PR slice

Write `LadybugFixture` (a temp directory per instance), invoke
`projection_store_conformance!` from a new `tests/` target, extend `port_shape.rs`, and
register the target in `xtask/src/proof.rs`'s `ARTEFACTS` so an emptied file cannot pass
the gate.

## Executive summary

By the time this story opens, `happenstance-ladybug` compiles against the real `lbug`
driver with four real bodies behind `SendProjectionStore` and no `#![allow(clippy::todo)]`
(`fill-the-bodies-and-ps-34-disposition`, HS-S0077). It has still never been asked a
single question by the conformance suite — the state `CLAUDE.md` refuses to call an
adapter, and the state in which the freeze re-test has produced no evidence at all.

**The delta this PR lands is the run.** A `LadybugFixture` that hands every instance its
own isolated LadybugDB directory and obeys
`crates/happenstance-testkit/src/contract.rs:1-63`'s fixture contract; a new test target
whose entire body is `projection_store_conformance!` over that fixture — not a
hand-rolled module and not a subset; an extension (never a replacement) of
`crates/happenstance-ladybug/tests/port_shape.rs`'s existing `const _` instantiations; a
deliberately row-shaped negative-control fixture run against the same suite so that a
green run means something; and a row in `xtask/src/proof.rs`'s `ARTEFACTS` (`:133`) so
that an emptied target fails the gate an intact one passes.

What it does **not** land: the read-your-own-writes projection (slice-mate
`read-your-own-writes-projection`, HS-S0079, which owns PS-12), the freeze verdict
(`freeze-verdict-document`), the build-cost measurement (`cold-build-cost-and-ci-shape`)
and packaging (`package-completeness-and-name-claim`). This story produces the *evidence*
those consume: a rule list, a declension list with reasons, and a command that a stranger
can re-run.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted anchor
in the second half of this spec.

**D1 — The macro invocation *is* the test target, whole.** The body is the
`projection_store_conformance!(LadybugFixture::new())` invocation over a fixture, exactly
as `CLAUDE.md`'s *rule that matters* spells the event-store form
(`happenstance_testkit::event_store_conformance!(MyFixture::new());`). **Not a
hand-rolled test module, and not a subset** (`_decomposition.md:127-142`, M1). A suite
invoked in part has not been invoked, and a subset is the failure mode that is invisible
in the output: an absent rule leaves no trace, so the *only* thing that detects it is
D11's count reconciliation.

**D2 — Isolation is a temp directory per fixture instance, and the rule that catches
sharing is inherited, not written here.** One fixture instance is one isolated backing
store; each `connect()` is one handle onto that store; two instances share nothing
(`crates/happenstance-testkit/src/contract.rs:14-24`). Rules are handed
`impl AsyncFn() -> F` precisely so isolation is a *conformance rule* rather than a
testkit meta-test — the event-store spelling is
`two_fixture_instances_observe_none_of_each_others_appends`
(`crates/happenstance-testkit/src/suite.rs:210`). Pointing every instance at one shared
LadybugDB path is named in the testkit's own docs as **an adapter's mistake**, and the
LadybugDB variant is more tempting than the SQLite one: a per-instance directory costs a
real filesystem create *and* a C++ engine open, so sharing one is the obvious
optimisation the moment the suite feels slow. `crates/happenstance-testkit/src/fixtures.rs:243-271`'s
`MemoryFixture` is the reference to read first.

**D3 — A capability this fixture cannot satisfy is *declined with a real reason* and
still appears in the run.** `Capability::declined("…")`, never `#[cfg]`
(`contract.rs:25-63`; `project.md:154-157`, DR-5). Two mechanical traps, both worth
knowing before the first red run: an empty reason on an **associated** const is evaluated
lazily and fails at **codegen**, so `cargo clippy` passes and `cargo build`/`cargo test`
fail (`contract.rs:404-410`); and the `SKIP` line is only readable because the gate
passes `--show-output` (`xtask/src/main.rs:131-151`). This story **consumes**
`.kb/open-questions/cf-40-fixture-limits-ownership.md`; it does not re-decide where
fixture constants live.

**D4 — The `ARTEFACTS` row must name qualified test names, because an empty list makes
the check vacuous.** `xtask/src/proof.rs`'s `check()` filters the row's `tests` against
`cargo test --list` and bails on the absent ones (`proof.rs:195-217`); a row with an
empty `tests` slice asserts nothing and passes an emptied target — reproducing exactly
the hole M4 exists to close (`cargo test` exits 0 on `running 0 tests`,
`xtask/src/main.rs:170-178`). The names are **read out of `--list`, not guessed**: the
macro wraps its rules in a module, and `proof.rs:58-70` requires the fully qualified
spelling.

**D5 — `port_shape.rs` is extended, never replaced, and its bound collapse is a
finding.** The file already instantiates generic code at both batch shapes through a
`const _` block, with `weak_flavour` and `send_flavour` in **separate modules** because
having both trait names in scope makes every method call `error[E0034]` (`CLAUDE.md`
constraint 4; the comment is at `port_shape.rs:11-14`). Its
`for<'a> S::Batch<'a>: Send` bound (`port_shape.rs:61`) collapses to `S::Batch: Send`
once the lifetime is gone — PS-5's predicted ergonomic win, *observed*
(`_decomposition.md:155-164`). That collapse (or its survival) is recorded for the
verdict; it is not a tidy-up. **Do not weaken `port_shape.rs` to make it compile — if a
bound has to change, the change is the finding** (`_decomposition.md:445-446`).

**D6 — A green run is meaningless without the negative control, and the control is
recorded rather than asserted green.** The wrong implementation this story most plausibly
ships is *the "graph" fixture whose batch is a SQL batch in different type names*: one
node label, scalar properties, no relationships, no traversal. It passes every rule,
emits the full count, satisfies AC-004 as written, and re-tests the freeze **against the
shape that froze it** (`discover.md`, *The wrong implementation*). So a `RowShapedFixture`
lives beside the conformance target in `crates/happenstance-ladybug/tests/` — it cannot
live in the testkit, which is not this project's to edit (`_decomposition.md:86-88`) —
and is run against the same suite with its outcome **written down**. Both outcomes are
informative: full pass ⇒ *"the suite as registered is blind to the axis this project was
run to test"*, a first-class verdict finding routed to `projection-store-freeze`; a
rejection ⇒ that rule is named as the one that actually bit on the unlike axis. The
control is written **before** the real run is interpreted, for the same reason the axes
were committed before any body was filled in (`project.md:180-184`, AC-001).

**D7 — The testkit is not this project's to edit, so "fix the rule in the same change"
takes its only available form.** Nothing under `crates/happenstance-testkit/` is in this
project's seams table (`_decomposition.md:86-88`). A rule that seems wrong is never
`#[cfg]`-ed out, never routed around with a retry, and never made to pass by weakening
the fixture. The defect is raised with `projection-store-freeze` (HS-P0010) **and the
reason is written down in the same change that observes it**, so the observation and its
explanation never travel separately (`discover.md`, *What if a suite rule seems wrong*).

**D8 — The fixture must not narrow the harness.** The suite is emitted through the
existing registry and inherits the tokio, blocking and wasm emitters unchanged
(`crates/happenstance-testkit/src/registry.rs:93-103`; the emitter table is
`crates/happenstance-testkit/src/lib.rs:55-67`), and `__emit_blocking` needs no async
runtime at all. A fixture that only works under a multi-threaded tokio runtime has
quietly narrowed the harness, and that is a **finding**, not a configuration detail
(`discover.md`, question 3). Which emitter this target uses follows ADR-0025's Q3 answer
(`spawn_blocking` behind a runtime gate / blocking on the executor thread /
blocking-only). Note the mechanical consequence: choosing a non-default emitter requires
the macro's **general arm** (`mod_name = …, emit = …, fixture = …` —
`lib.rs:311-330`), which is the macro's own form, not a hand-rolled subset, and it fixes
the module name D4's rows must match.

**D9 — Never assert on a literal position value.** The specification permits gaps and a
conformant adapter may leave them (`CLAUDE.md`, *The rule that matters*). This binds the
negative control and any fixture-level assertion this story adds, not only the inherited
rules.

**D10 — Nothing `[FROZEN]` is touched.** PS-2 (`spec/SPECIFICATION.md:4760-4774`,
`[FROZEN]`) is the clause this run is evidence **for** — it requires *"two adapters at
opposite ends of the batch-shape axis"* to have passed. This story supplies the second
end; it does not amend the clause, and `cargo xtask spec-trace` stays green. PS-12's
answer belongs to slice-mate `read-your-own-writes-projection`, and moving PS-34's marker
is `projection-store-freeze`'s (`RUNBOOK.md:602`).

**D11 — Rule-count equality is a diff against HS-P0010's registration list, not a
guess.** AC-004 requires the count emitted to equal the count `projection-store-freeze`
registered (`project.md:193-196`; `_decomposition.md:474`). If that project shipped a
`for_each_projection_store_rule!` analogue of
`crates/happenstance-testkit/src/registry.rs:93-103`, the reconciliation is mechanical
against the macro's own list; if it did not, it is a by-name diff against the
registration list in HS-P0010's artefacts. Either way the number is **derived from the
suite**, never from this story's expectations.

**D12 — DR-9: the run has to be re-takeable by a stranger.** *"Passed against N adapters"
is a snapshot and the snapshot has to be re-takeable* (`project.md:172-174`). The
`ARTEFACTS` row is half of that; the recorded command, the emitted rule list, the
declension list with reasons and the commit are the other half, and they are what
`freeze-verdict-document` reads. They are recorded in this story's own folder, because
`references/evaluation/` belongs to `preflight-and-unlike-axes` (the axes) and
`freeze-verdict-document` (the verdict) — two files in two commits, because AC-001 is an
ordering claim about commits (`_decomposition.md:210-218`, M9).

**D13 — A new dev-dependency is a workspace-level decision and must clear `cargo
deny`.** The workspace pins *every* third-party version in `[workspace.dependencies]`
(`Cargo.toml:17-19`), and nothing in the tree currently depends on a temp-directory
crate. The pin is added there and named `…workspace = true` in
`crates/happenstance-ladybug/Cargo.toml`'s `[dev-dependencies]`. `cargo deny` resolves on
this machine and therefore runs rather than skipping (`CLAUDE.md`, *Commands*); the
manifest already records two occasions where a dependency's licence or advisory graph
failed the gate (`Cargo.toml:55-68`, `:87-92`). If the chosen crate fails, the answer is
a different crate — **not** a `deny.toml` exemption, which is a workspace policy edit
outside this story.

**D14 — The preconditions are discharged upstream and are not re-litigated here.**
`preflight-and-unlike-axes` (HS-S0074) already established that
`crates/happenstance-core/src/projection.rs` shipped `type Batch;`, that the write seam
exists and that `projection_store_conformance!` resolves from `happenstance-testkit`; a
red preflight halts the project rather than being worked around locally
(`_decomposition.md:33-41`, AC-A01). `fill-the-bodies-and-ps-34-disposition` (HS-S0077)
supplies four real bodies. There is nothing to drive before them — a suite run against
`todo!()` bodies panics rather than reports.

**Persona-journey slice.** The actor is the reviewer holding the freeze: someone who will
be asked to believe a *held* / *did not hold* verdict. What this story hands them is not
"green"; it is a legible run — every registered rule accounted for, every declension
carrying the fixture's own stated reason, a negative control whose outcome says whether
the green was discriminating, and a command they can run themselves
(`_storymap.md:13-24`, activity **C**).

## Integration contract

- **Archetype**: `capability` — a slice through the fixture, the suite, the compiler's
  view of the port, and the gate that notices all three.
- **Slice / milestone**: `conformance-run`. Slice-mate: `read-your-own-writes-projection`
  (HS-S0079), which merges after this story and consumes its fixture and target
  (`_storymap.md:44-45`, merge order `:134-135`).
- **Mount point**: **`xtask/src/proof.rs`** — the `ARTEFACTS` constant at `:133` gains a
  row for this crate's conformance target. This is the composition root that makes the
  run part of the gate rather than a file in a directory: `cargo test` exits 0 on
  `running 0 tests`, so an emptied target passes the step a deleted one fails
  (`xtask/src/main.rs:170-178`; the argument is `proof.rs:9-24`). The test target itself,
  `crates/happenstance-ladybug/tests/<conformance target>.rs`, is what that row mounts —
  the target *is* the mount, and the registry row is what makes it noticeable
  (`_storymap.md:71-74`).
- **Wires into** (real sibling contracts consumed, not invented):
  - `crates/happenstance-testkit/src/contract.rs` — `Fixture`, `Capability`,
    `RuleOutcome`; the isolation contract at `:1-63` and the empty-reason trap at
    `:404-410`.
  - `crates/happenstance-testkit/src/fixtures.rs:243-271` — `MemoryFixture`, the
    reference fixture and the declension-with-a-real-reason exemplar.
  - `crates/happenstance-testkit/src/lib.rs:55-67`, `:311-357` — the emitter table and
    the macro's arms (the projection macro mirrors this shape).
  - `crates/happenstance-testkit/src/registry.rs:93-103` — the rule registry the emitted
    count reconciles against.
  - `crates/happenstance-core/src/projection.rs` — the port (**read, never edited**;
    `_decomposition.md:86-88`).
  - `crates/happenstance-ladybug/src/projection_store.rs` — `LadybugProjectionStore::new`
    and `connect` (`:230-249`), the single-writer note (`:204-213`), the `Send`-flavour
    evidence (`:252-257`).
  - `crates/happenstance-ladybug/tests/port_shape.rs` — extended in place (M3).
  - `Cargo.toml` `[workspace.dependencies]` and
    `crates/happenstance-ladybug/Cargo.toml` `[dev-dependencies]` — D13.
- **Renders surfaces**: **none**. `_design.md` records no user-facing surface for this
  project and its `## Items` / `## Signatures` blocks are `N/A`; this story adds **no new
  `pub` item to `happenstance-ladybug`'s public API**. `LadybugFixture` and the
  negative-control fixture live under `crates/happenstance-ladybug/tests/`, which keeps
  the crate's published surface exactly as signed off — a fixture promoted into `src/` as
  a `pub` item would be a semver promise nobody made.
- **Conformance rule(s)**: every rule `projection_store_conformance!` emits, including
  the projection suite's isolation rule (the analogue of
  `two_fixture_instances_observe_none_of_each_others_appends`,
  `crates/happenstance-testkit/src/suite.rs:210`), which this fixture's temp-directory
  strategy is what passes. **This story writes no conformance rule** — the suite and its
  rules are `projection-store-freeze`'s (`_decomposition.md:86-88`).
- **Clause(s)**: evidence for **PS-2** (`spec/SPECIFICATION.md:4760-4774`, `[FROZEN]` —
  supplies the second end of the batch-shape axis; not amended). Data points for **PS-5**,
  **PS-9** and **PS-11** via D5's bound collapse and the raw-Cypher write vocabulary.
  **PS-12** is the slice-mate's. No clause marker or clause sentence is edited here;
  `cargo xtask spec-trace` stays green.
- **Advances DoD scenario**: initiative **DoD 7** — *"The projection suite discriminates.
  Two structurally unlike batch shapes pass it…"* (`initiative.md`, Definition of Done) —
  by landing the second, unlike shape and, through D6's control, the beginnings of the
  discrimination claim. Also project **DoD 2** (`project.md:230-232`), jointly with
  HS-S0079.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file
changed outside it.

```
crates/happenstance-ladybug/tests/**
crates/happenstance-ladybug/Cargo.toml
Cargo.toml
xtask/src/proof.rs
.bklg/from-contract-to-published-library/ladybug-projection-store/ladybug-fixture-and-conformance-run/**
```

**In this PR**

- `LadybugFixture` — temp-directory-per-instance, `Fixture` impl, capability constants,
  each declension carrying a real fixture-specific reason.
- The conformance test target: `mod` line, `use`, and the
  `projection_store_conformance!` invocation. Nothing else.
- `RowShapedFixture` — the negative control, and the target that runs the suite over it,
  with its outcome recorded.
- `port_shape.rs` extended: `const _` instantiations at the merged batch shape on both
  flavours, with the `Send`-bound collapse recorded.
- `xtask/src/proof.rs` — one `Artefact` row, with a non-empty `tests` list read out of
  `cargo test --list` (mounting this slice; the mount is named in the Integration
  contract and touching it is not scope drift).
- `Cargo.toml` + `crates/happenstance-ladybug/Cargo.toml` — the temp-directory
  dev-dependency, pinned at the workspace and named by the crate.
- This story's folder: the run record (command, emitted rule list, declensions with
  reasons, control outcome, commit) that `freeze-verdict-document` reads.

**Explicitly not in this PR**

- Any edit under `crates/happenstance-testkit/**` or to
  `crates/happenstance-core/src/projection.rs` — the suite, its rules and the port are
  `projection-store-freeze`'s (`_decomposition.md:86-88`).
- The read-your-own-writes projection and PS-12's answer → `read-your-own-writes-projection`.
- The freeze verdict, and anything under `references/evaluation/` →
  `preflight-and-unlike-axes` (axes) and `freeze-verdict-document` (verdict).
- The cold-build measurement, `.github/workflows/ci.yml`, `xtask/src/main.rs` step args
  and `RUNBOOK.md`'s phase 11 body → `cold-build-cost-and-ci-shape`.
- `publish = false`, licences, README, `xtask/src/package.rs`'s `PUBLISHABLE` →
  `package-completeness-and-name-claim`.
- `spec/SPECIFICATION.md` — the six citation repairs merged with HS-S0077, and no clause
  is touched here.
- A `deny.toml` exemption for a dependency that fails the licence or advisory check
  (D13).

**Merge DoD**: `cargo xtask ci` — the whole gate, not `--fast` (`_storymap.md:145-147`) —
is green with the conformance target running under `--show-output`, `cargo xtask
proof-artefact` asserting the new row's named tests out of `--list`, and the run record
committed in this story's folder.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **A fixture instance is one isolated LadybugDB** | `LadybugFixture::new()` creates a fresh temp directory and opens a `Database` over it; the instance owns the directory and removes it on drop. Two instances share no path, no `Database` and no process-global state. `connect()` returns one handle onto *that* store, per-call rather than pooled — the pattern `LadybugProjectionStore::connect` already documents. | `crates/happenstance-testkit/src/contract.rs:14-24`; `crates/happenstance-testkit/src/fixtures.rs:243-271`; `crates/happenstance-ladybug/src/projection_store.rs:238-249` |
| **`Fixture::Store` is the store the rules drive** | The fixture's associated store type is `LadybugProjectionStore` (or the handle type the projection `Fixture` analogue names), constructed by `LadybugProjectionStore::new(database)`. The store takes the `Database` **by value**: a borrowed database makes the store non-`'static`, and a non-`'static` store cannot implement this port on rustc 1.97.1 at all — it ICEs. | `crates/happenstance-ladybug/src/projection_store.rs:230-236`, `:24-37`; `crates/happenstance-ladybug/src/live_handle.rs:38-66` |
| **Capability constants are answered honestly, per capability** | Each `Capability` associated const is either `SUPPORTED` or `declined("<real reason>")`. The reason states *why this store cannot*, not *that it cannot*. LadybugDB's many-readers-one-writer rule is the most likely source of a genuine declension, and `WriteTransactionInUse` under a concurrency-shaped rule is a candidate **stated capability limit** — never a retry loop. | `crates/happenstance-testkit/src/contract.rs:25-63`, `:355-433`; `crates/happenstance-ladybug/src/projection_store.rs:204-213`; `project.md:278-283` |
| **A declined rule still runs and still reports** | No `#[cfg]`, anywhere. A skipped rule emits `SKIP <rule>: <reason>`, readable only because the gate passes `--show-output`. An empty reason on an *associated* const is a **codegen** failure: `cargo clippy` passes, `cargo build`/`cargo test` fail. | `crates/happenstance-testkit/src/contract.rs:404-410`; `xtask/src/main.rs:131-151` |
| **The target's body is the macro and nothing else** | `mod <fixture module>;`, the `use`, the `projection_store_conformance!` invocation. No `#[test]` of the target's own, no rule list, no exclusions. If ADR-0025's Q3 answer requires a non-default emitter, the macro's general arm (`mod_name = …, emit = …, fixture = …`) is used — that is the macro's own form. | `_decomposition.md:127-142`; `crates/happenstance-testkit/src/lib.rs:311-330`; `CLAUDE.md`, *The rule that matters* |
| **Emitted rule count reconciles with the registration** | The count of rules the run emits equals the count `projection-store-freeze` registered — read from the projection analogue of `for_each_event_store_rule!` if it shipped, otherwise diffed by name against HS-P0010's registration list. An absent rule leaves no trace in the output, so this reconciliation is the only detector. | `project.md:193-196`; `crates/happenstance-testkit/src/registry.rs:93-103`; `_decomposition.md:474` |
| **The shared fixture module is used by both targets without a warning** | Each file under `tests/` is its own crate, so `LadybugFixture` lives in a shared module included by both the conformance target and the control target. The gate denies warnings, so an item unused by one includer needs the module organised so nothing is dead — not a blanket `#![allow(dead_code)]` over the fixture. | `xtask/src/main.rs:116-129` (`clippy … -D warnings`) |
| **The negative control runs the same suite and its outcome is recorded** | `RowShapedFixture`: single node label, scalar properties, no relationships, no traversal — a key-value row store with a Cypher accent. Run against the same macro. Outcome **recorded, not asserted green**: a full pass is the finding *"the registered rules do not distinguish a graph batch from a row batch"*, routed to HS-P0010; a rejection names the rule that bit on the unlike axis. Written before the real run is interpreted. | `discover.md`, *The wrong implementation*; `_decomposition.md:86-88` (why it cannot live in the testkit) |
| **`port_shape.rs` gains instantiations, loses nothing it still can hold** | `weak_flavour` and `send_flavour` stay in separate modules (`error[E0034]` otherwise). The `const _` block instantiates the generic runners at the merged batch shape. If `type Batch;` landed, `for<'a> S::Batch<'a>: Send` becomes `S::Batch: Send` and that collapse is recorded as PS-5's predicted relief observed. Lines instantiating a store retired by HS-S0077 go with it. | `crates/happenstance-ladybug/tests/port_shape.rs:11-14`, `:50-61`, `:75-80`; `_decomposition.md:155-164`, `:314-331` |
| **The gate notices the target's contents, not just its filename** | One `Artefact { package: "happenstance-ladybug", target: <target>, tests: [<fully-qualified rule names>] }` row. The names are fully qualified with the emitted module prefix and are **copied out of `cargo test --test <target> -- --list`**, because `check()` asserts them against that listing and a wrong or empty list is respectively a red gate or a vacuous one. | `xtask/src/proof.rs:58-70`, `:133-150`, `:195-217`; `xtask/src/main.rs:170-178` |
| **A rule that looks wrong is raised, never routed around** | Not `#[cfg]`-ed out, not retried until green, not made to pass by weakening the fixture. Raised with `projection-store-freeze` with the reason written down **in the same change that observes it**. | `discover.md`, *What if a suite rule seems wrong*; `CLAUDE.md`, *The rule that matters* |
| **No literal position values are asserted** | The specification permits gaps; any assertion this story adds compares against positions the store actually assigned. | `CLAUDE.md`, *The rule that matters*; `project.md:293-294` |
| **The run is re-takeable** | The exact command, the emitted rule list, every declension with its stated reason, the control's outcome and the commit are recorded in this story's folder, in the form `freeze-verdict-document` needs to name *which implementation, which rules, which clauses, at which commit*. | `project.md:172-174` (DR-9), `:151-153` (DR-4) |
| **The harness is not narrowed** | The fixture works under the emitter the target declares without requiring a runtime the testkit's harness does not provide. A fixture that only works under a multi-threaded tokio runtime is a **finding**, recorded, not a configuration detail. | `crates/happenstance-testkit/src/lib.rs:55-67`; `crates/happenstance-testkit/src/registry.rs:93-103`; `discover.md`, question 3 |

## Data and migrations

**No user data, no schema migration, no persistent store outside the test run.** The only
state this story creates is a LadybugDB directory per fixture instance, and its whole
lifecycle is inside a single `cargo test` process:

- **Creation.** One fresh temp directory per `LadybugFixture` instance, created at
  construction. The path is never module-scoped, never `static`, never derived from a
  fixed name — that is D2's failure mode, and the inherited isolation rule is what catches
  it (`crates/happenstance-testkit/src/contract.rs:14-24`).
- **Teardown.** The directory is removed when the fixture instance drops. Two practical
  notes rather than preferences: a rule that panics still unwinds, so drop still runs; and
  a LadybugDB `Database` holding an open handle must be dropped before the directory is
  removed, which on Windows is the difference between a clean teardown and a
  `PermissionDenied`. A leaked directory is a real defect here because the suite creates
  one per rule per fixture instance.
- **Schema.** The Cypher schema for the read model and the checkpoint node — label names,
  property names, indexes — is **not decided here**: it is ADR-0025's Q1 (transaction
  membership) plus whatever `fill-the-bodies-and-ps-34-disposition` implemented
  (`_decomposition.md:261-274`, `:408-419`). This story consumes it as given. The
  checkpoint write is the last statement before `COMMIT`, inside the same
  `BEGIN TRANSACTION` as the read-model write — that single transaction is PS-1, and it
  is the reason the port has this shape (`crates/happenstance-core/src/projection.rs:13-30`).
- **Dependencies as data.** One new dev-dependency (a temp-directory crate) pinned in
  `[workspace.dependencies]` and named `…workspace = true` by the crate, per D13. Nothing
  in the published dependency graph changes: the crate is a `[dev-dependencies]` entry
  only.
- **No fixtures write outside their own directory**, and nothing here reads or writes any
  path under `references/`, `.kb/` or `spec/`.

## Acceptance criteria

Eight criteria. Each is framed from the intent of the actor who will be asked to believe
the freeze verdict — the adapter author on *Learn when you are finished* and the reviewer
on *Decide in one sitting* (`.bklg/from-contract-to-published-library/initiative.md:241-250`)
— and each crosses the whole stack this story touches: fixture → suite → compiler → gate.
All eight together discharge project **AC-004** (`project.md:193-196`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the reviewer holding the projection freeze, who has been told `ProjectionStore` will be re-tested against a batch shape unlike the ones that froze it, **WHEN** they run `cargo xtask ci` on this branch and read the `tests` step top to bottom without opening a second tool, **THEN** a Ladybug conformance target has executed the **whole** `projection_store_conformance!` suite over a `LadybugFixture` — every registered rule accounted for by a pass or a reported skip, none absent — and the number of rules that reported is reconciled, in the run record, against the number `projection-store-freeze` registered, derived from the suite's own enumeration and never from this story's expectations. | `crates/happenstance-ladybug/tests/<conformance target>.rs`, whose entire body is the macro invocation, run by `cargo xtask ci`'s `tests` step (`xtask/src/main.rs:143-151`); the count reconciled against the projection analogue of `crates/happenstance-testkit/src/registry.rs:93-103` (or, absent one, a by-name diff against HS-P0010's registration list) and the diff written into the run record in this story's folder. |
| AC-002 | **GIVEN** an adapter author who will copy this fixture the next time a store adapter needs one, and who knows the tempting shortcut is one LadybugDB directory shared by every instance because a per-instance directory costs a filesystem create *and* a C++ engine open, **WHEN** the suite constructs two `LadybugFixture` instances in one process and writes through each, **THEN** neither observes anything the other wrote — because each instance owns a fresh temp directory and its own `Database`, and each `connect()` is one handle onto *that* store — and the rule that proves it is the suite's **inherited** isolation rule, not a local assertion this story wrote to congratulate itself. | The projection analogue of `two_fixture_instances_observe_none_of_each_others_appends` (`crates/happenstance-testkit/src/suite.rs:210`) passing inside the AC-001 target, against the fixture contract at `crates/happenstance-testkit/src/contract.rs:14-24`; reviewed against `crates/happenstance-testkit/src/fixtures.rs:243-271`. A directory path that is `static`, module-scoped or fixed-name fails review even if the rule is green. |
| AC-003 | **GIVEN** the same reviewer, now asking the only question a green run cannot answer — *what could this store not do, and why* — **WHEN** they read the gate's captured output for the conformance target, **THEN** every rule the fixture cannot satisfy has still **run** and has printed `SKIP <rule>: <reason>` carrying the fixture's own fixture-specific reason (LadybugDB's many-readers-one-writer rule where that is the truth), no rule anywhere is `#[cfg]`-ed out of the binary, and no declension carries an empty reason. | The AC-001 target under `cargo xtask ci`'s `tests` step, which passes `--show-output` precisely so a passing capability-gated rule's line is not written into a void (`xtask/src/main.rs:131-151`); capability constants declared per `crates/happenstance-testkit/src/contract.rs:25-63`, with the empty-reason trap caught at **codegen** by `cargo build`/`cargo test` (`contract.rs:404-410`) rather than by clippy. Every emitted `SKIP` line is pasted verbatim into the run record. |
| AC-004 | **GIVEN** a gate reader who has already been burned once by a step that a *deletion* fails and an *emptying* passes (`xtask/src/proof.rs:9-24`), **WHEN** the conformance target is truncated to its attributes, has its rules renamed, or is replaced by a file that emits nothing, **THEN** `cargo xtask ci` fails **before** the run with a message naming which expected tests are missing — never exiting 0 on `running 0 tests` — because `ARTEFACTS` carries a row for this crate's target whose `tests` list is non-empty and fully qualified with the emitted module prefix, copied out of `cargo test --test <target> -- --list` rather than guessed. | `xtask/src/proof.rs`'s `ARTEFACTS` (`:133-150`) gaining one `Artefact` row, asserted by `cargo xtask proof-artefact` (`check()` at `:195-217`, the qualification requirement at `:58-70`) which the gate runs as its own step (`xtask/src/main.rs:170-178`). Negative control performed by hand — the target emptied, the gate re-run, the failure output recorded — because a row with an empty `tests` slice passes vacuously. |
| AC-005 | **GIVEN** the reviewer reading the verdict's ergonomics section, who was promised that dropping the `Batch` lifetime would relieve callers of higher-ranked bounds (PS-5), **WHEN** they open `crates/happenstance-ladybug/tests/port_shape.rs` after this merges, **THEN** the file has been **extended** — `weak_flavour` and `send_flavour` still in separate modules, generic code still instantiated through the `const _` block at the batch shape that actually merged — and the fate of `for<'a> S::Batch<'a>: Send` is written down as an observation (collapsed to `S::Batch: Send`, or survived), with no bound weakened and no instantiation deleted except the ones that go with the store HS-S0077 retired. | `cargo test -p happenstance-ladybug --test port_shape` (compile-only; the assertions are `const _` instantiations per `standards/rust/61-compile-time-assertions.md`) inside `cargo xtask ci`; diff review that `port_shape.rs:11-14`'s two-module split survives (`error[E0034]` otherwise, CLAUDE.md constraint 4) and that the `:50-61` bound's fate is recorded in the run record, not silently edited. |
| AC-006 | **GIVEN** the reviewer who knows the one way this project can fail while looking successful — a "graph" fixture that is a row store with a Cypher accent, passing every rule and re-testing the freeze against the shape that froze it — **WHEN** they read the run record, **THEN** a `RowShapedFixture` (one node label, scalar properties, no relationships, no traversal) has been run against the **same** suite and its outcome is **recorded, not asserted green**: a full pass is written down as the finding *"the registered rules do not distinguish a graph batch from a row batch"* and routed to `projection-store-freeze`; a rejection names the rule that bit on the unlike axis. The control is committed and its outcome written **before** the real run is interpreted. | `crates/happenstance-ladybug/tests/` carries the control fixture and the target that runs the suite over it (it cannot live in the testkit — `_decomposition.md:86-88`); both targets run by `cargo xtask ci`. The outcome is a written finding in this story's folder, per `standards/rust/60-what-a-test-must-prove.md`'s named-wrong-implementation discipline and `discover.md`, *The wrong implementation*. |
| AC-007 | **GIVEN** a stranger who did not write this adapter and is asked to believe *"passed against N adapters"*, **WHEN** they open this story's folder, **THEN** they find the exact command, the emitted rule list, every declension with its stated reason, the negative control's outcome, the `port_shape.rs` observation and the commit SHA — enough to re-take the snapshot themselves and enough for `freeze-verdict-document` to name *which implementation, which rules, which clauses, at which commit* without re-running anything. | `project.md:172-174` (DR-9) and `:151-153` (DR-4); the record lives in `.bklg/from-contract-to-published-library/ladybug-projection-store/ladybug-fixture-and-conformance-run/` (not `references/evaluation/`, which is `preflight-and-unlike-axes`' and `freeze-verdict-document`'s per `_decomposition.md:210-218`). Verified by a reviewer re-running the recorded command on the recorded commit and getting the recorded rule list. |
| AC-008 | **GIVEN** the maintainer of the suite this story is the first outside consumer of, who has to trust that a green run here was not bought by editing the thing being tested, **WHEN** they read the whole diff, **THEN** nothing under `crates/happenstance-testkit/**` or in `crates/happenstance-core/src/projection.rs` was touched, no rule was `#[cfg]`-ed out, retried until green, or made to pass by weakening the fixture; any rule that looked wrong is raised with `projection-store-freeze` **with its reason in this same change**; no `[FROZEN]` clause marker or sentence in `spec/SPECIFICATION.md` moved and `cargo xtask spec-trace` is green; `happenstance-ladybug` gains **no new `pub` item**; and the one new dev-dependency is pinned in `[workspace.dependencies]`, named `…workspace = true` by the crate, and clears `cargo deny` without a `deny.toml` exemption. | `git diff --stat` showing no path under `crates/happenstance-testkit/`, `crates/happenstance-core/`, `spec/`, `.kb/` or `references/`; `cargo xtask ci`'s `spec-trace`, `cargo deny`, `cargo hack` feature-powerset and docs steps all green (`CLAUDE.md`, *Commands*); `rg -n "#\[cfg" crates/happenstance-ladybug/tests/` returning no rule-level gate; dependency hygiene reviewed against `standards/rust/50-dependency-hygiene.md` and `Cargo.toml:17-19`. |

## Interaction quality

**This story renders no user-facing surface.** `_design.md` is signed off (Ryan Britton,
2026-08-12) with `## Surfaces`, `## Items` and `## Signatures` all `N/A — no user-facing
surface`, and `design.capture` is a declared skip because there is no app to screenshot.
So there is no design system to compose against and no density budget expressed in
pixels.

There is, however, exactly one thing a human perceives from this work, and the whole
project's integrity rests on it: **the gate's captured output and the run record derived
from it.** Treating that as "not a surface" is how a run becomes green text nobody can
read. The two families below are therefore applied to that output, with real numbers where
the artefact supplies them. Every invariant that applies is already an **AC row in the
table above** — nothing is asserted only here.

**STATE invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context-jump** — the evidence appears in the same `cargo xtask ci` scroll the reviewer is already reading; no second tool, second CI job or machine they do not have. | **AC-001**, **AC-003** | The conformance target runs inside the gate's own `tests` step (`xtask/src/main.rs:143-151`), not as a separate manual command. |
| **Non-occlusion** — a passing rule's `SKIP` line is not swallowed by libtest's default stdout capture. | **AC-003** | The `--show-output` flag on the gate's `tests` step, and the comment at `xtask/src/main.rs:131-142` that explains why it is not noise. |
| **Reversibility** — the run is re-takeable: a stranger can reproduce it from the recorded command at the recorded commit, and get the recorded list. | **AC-007** | DR-9 (`project.md:172-174`); a reviewer re-run is the check. |
| **Preserved selection (the analogue that bites here)** — nothing this story adds narrows what the harness can select: the target inherits the tokio / blocking / wasm emitters unchanged, and a fixture that only works under a multi-threaded tokio runtime is a recorded finding, not a config detail. | **AC-008** | `crates/happenstance-testkit/src/lib.rs:55-67`, `crates/happenstance-testkit/src/registry.rs:93-103`; `discover.md`, question 3. |
| **Reachability without a special incantation** — `cargo xtask ci` alone reaches every claim; no rule requires a flag a contributor has to know about. | **AC-001**, **AC-004** | The gate definition in `xtask/src/main.rs`; `cargo xtask proof-artefact` as its own step. |

**COMPOSITION invariants** — the design's `N/A` means these are taken from the *output
artefact* the project's testing brief and M2/M4 already specify, and each carries a real
number or a real shape:

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — a declension is not bare markup: it is `Capability::declined("<reason>")` producing a composed `SKIP <rule>: <reason>` line naming *why this store cannot*, never *that it cannot*. An empty reason is a build failure, not a style nit. | **AC-003** | `crates/happenstance-testkit/src/contract.rs:25-63`, `:404-410`. |
| **Composition / placement** — the run's evidence is composed in one place (this story's folder) in the form `freeze-verdict-document` consumes; it is not scattered across a commit message, a CI log URL and a reviewer's memory. | **AC-007** | `_decomposition.md:210-218` (M9's two-files-two-commits shape, and why this story's record is *not* in `references/evaluation/`). |
| **Transience — what is persistent chrome vs revealed on demand** — the `ARTEFACTS` row is persistent chrome: it runs on every gate invocation forever. The negative control's finding is revealed on demand, in the record. Neither is a one-off console line that exists only in the run that produced it. | **AC-004**, **AC-006** | `xtask/src/proof.rs:133-150`; the recorded finding in this story's folder. |
| **Density budget, with its real numbers** — the row's `tests` list has **≥ 1** entry (a 0-entry list is the vacuous check M4 exists to close), and the reported rule count equals HS-P0010's registered count **exactly** — not "at least", not "about". `ARTEFACTS` grows from **3** rows to **4**. | **AC-001**, **AC-004** | `xtask/src/proof.rs:133-150` (three rows today); `project.md:193-196`. |
| **Hierarchy** — the run record leads with the verdict-relevant facts (rule list, declensions with reasons, control outcome, commit) and keeps the mechanics beneath them, because DR-4's failure mode is a document that says only "held". | **AC-007** | `project.md:151-153`. |
| **Named anti-patterns** — the three the project already named are refused by construction: the row-shaped "graph" fixture (falsified, not avoided by assertion), the shared directory, and the invocation trimmed to what passes. | **AC-006**, **AC-002**, **AC-001** | `discover.md`, *The wrong implementation*, all three sub-headings. |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | A `Capability::declined("")` — an empty reason on an **associated** const. | Not a lint, a **codegen** failure: `cargo clippy` passes and `cargo build`/`cargo test` fail (`crates/happenstance-testkit/src/contract.rs:404-410`). Fix the reason; never delete the constant to make the error go away. |
| **EC-002** | Two fixture instances observe each other's writes. | The inherited isolation rule fails. The fix is a per-instance temp directory, never a rule exclusion and never a `#[cfg]` (`crates/happenstance-testkit/src/contract.rs:14-24`; `crates/happenstance-testkit/src/suite.rs:210`). |
| **EC-003** | The `ARTEFACTS` row is added with an empty `tests` slice, or with names that do not match `--list`. | Empty ⇒ the check passes vacuously and the hole M4 exists to close is reproduced: treat as a defect even though the gate is green. Mismatched ⇒ `check()` bails naming the absent tests (`xtask/src/proof.rs:195-217`); read the names out of `cargo test --test <target> -- --list` rather than editing the assertion. |
| **EC-004** | A concurrency-shaped rule meets LadybugDB's `WriteTransactionInUse` (many readers, exactly one writer). | Declared as a **stated capability limit** with that reason, or raised as a rule defect — **never** a retry loop until green (`crates/happenstance-ladybug/src/projection_store.rs:204-213`; `project.md:278-283`). Whether it is a limit or a defect is the verdict's question, not this story's. |
| **EC-005** | The emitted rule count is lower than the count `projection-store-freeze` registered. | Halt. An absent rule leaves no trace in the output, so a low count is the *only* signal: identify the missing rule by name, and either the invocation was trimmed (fix it here) or the suite did not emit it (raise with HS-P0010, with the reason, in this change). Do not reconcile by lowering the expectation (`project.md:193-196`; `_decomposition.md:474`). |
| **EC-006** | Teardown fails with `PermissionDenied` because a LadybugDB `Database` still holds an open handle on the directory being removed. | Drop the `Database` before the directory. This is a Windows-first failure and a real defect, not flake: the suite creates one directory per rule per fixture instance, so a leaked directory compounds. |
| **EC-007** | The new temp-directory dev-dependency fails `cargo deny`'s licence or advisory check. | Choose a different crate. **Not** a `deny.toml` exemption — that is a workspace policy edit outside this story's boundary (D13; `Cargo.toml:55-68`, `:87-92` record two prior occasions). |
| **EC-008** | A suite rule appears wrong when applied to a graph batch. | Raise it with `projection-store-freeze` (HS-P0010) and write the reason down **in the same change that observes it**. Never `#[cfg]` it out, never route around it with a retry, never weaken the fixture to pass it (`discover.md`, *What if a suite rule seems wrong*; `CLAUDE.md`, *The rule that matters*). |
| **EC-009** | The conformance target will not compile because the merged `Batch` shape differs from what `port_shape.rs` instantiates. | The bound change is the **finding**, recorded for the verdict. Do not weaken `port_shape.rs` to make it compile (`_decomposition.md:445-446`). |

## Non-functional

| id | Requirement | Why it is here |
| --- | --- | --- |
| **NF-001** | The merge bar is `cargo xtask ci` — the **whole** gate, not `--fast`. | This project is the one place in the portfolio where wasm32, `cargo-hack`, package-completeness and `spec-trace` meet a new dependency graph (`_storymap.md:145-147`; `_decomposition.md`, *Merge-gate commands*). |
| **NF-002** | The fixture adds no runtime requirement the testkit's harness does not already provide; `__emit_blocking` needs no async runtime at all. | A fixture that only works under a multi-threaded tokio runtime has quietly narrowed the harness — a finding, recorded (`crates/happenstance-testkit/src/lib.rs:55-67`; `discover.md`, question 3). |
| **NF-003** | No new `pub` item on `happenstance-ladybug`. Both fixtures live under `tests/`. | `_design.md` signs off a no-surface project; a fixture promoted into `src/` as `pub` is a semver promise nobody made (`standards/rust/40-public-surface-and-evolution.md`). |
| **NF-004** | The published dependency graph is unchanged: the new pin is a `[dev-dependencies]` entry only, and it is a small, widely-used crate rather than one that pulls a second heavy build. | `cold-build-cost-and-ci-shape` measures the `lbug` C++ cost against a gate that already runs this target (`_storymap.md:136-138`); a second heavy dev-dependency would contaminate that number. |
| **NF-005** | Zero warnings. Each file under `tests/` is its own crate, so the shared fixture module must be organised such that neither includer has dead items — not silenced with a blanket `#![allow(dead_code)]`. | `cargo clippy --workspace --all-targets --all-features -D warnings` (`xtask/src/main.rs:116-129`). |
| **NF-006** | Suite wall-clock stays proportionate: a per-instance directory plus a C++ engine open is paid once per fixture instance, not once per statement. | The temptation to share a directory is a *performance* temptation (D2); making the honest version fast enough is how the temptation is removed rather than resisted. |

## Implementation notes (non-prescriptive)

Not prescriptions — the decisions are ADR-0025's and the implementer's. These are the
places a reasonable first attempt goes wrong.

- **Read `MemoryFixture` first** (`crates/happenstance-testkit/src/fixtures.rs:243-271`),
  then `contract.rs:1-63`. The fixture contract is short and every clause of it is
  load-bearing; the event-store fixture is the shape the projection one mirrors.
- **Order the first red run deliberately.** Get the target compiling with the macro
  invocation and a fixture whose capabilities are all declined with honest reasons, run it
  once, and read the output — the rule names you need for the `ARTEFACTS` row come out of
  `cargo test --test <target> -- --list` on *that* run. Writing the row first means
  guessing the module prefix.
- **The `mod_name` decision comes before the row.** If ADR-0025's Q3 answer requires a
  non-default emitter, the macro's general arm
  (`mod_name = …, emit = …, fixture = …`; `crates/happenstance-testkit/src/lib.rs:311-330`)
  is what you invoke, and it fixes the module prefix every `ARTEFACTS` name must carry.
  Deciding the emitter after writing the row means rewriting the row.
- **Share the fixture module the way `tests/` requires.** Two targets, two crates; a
  `mod` file included by both, organised so neither has an unused item (NF-005).
  `standards/rust/62-doctests-and-harnesses.md` is the atom to pull if the arrangement
  fights the gate.
- **Write the negative control before you interpret the real run.** Not before you *write*
  it — before you *interpret* it. The ordering is the point (D6), and it mirrors AC-001's
  commit-order discipline one level down.
- **Keep the two records apart.** The `SKIP` lines and the rule list are captured output;
  the finding ("this declension is a stated limit because …") is prose. Both go in the
  record; conflating them is how a verdict ends up quoting a log at a reviewer.
- **If the run is green and no named unlike axis was exercised**, that is a finding about
  the suite, recorded and raised — not a licence to add a local rule that makes the run
  look more thorough (`discover.md`, question 4).
- **Standards atoms worth pulling, not the corpus**:
  `standards/rust/91-adapter-authoring-recipe.md` (the fixture-and-suite shape),
  `standards/rust/61-compile-time-assertions.md` (why `port_shape.rs` is `const _` rather
  than `#[test]`), `standards/rust/24-the-blocking-bridge.md` (if Q3's answer touches the
  fixture), `standards/rust/50-dependency-hygiene.md` (the new pin).

## Tests and CI (merge gate)

Grounded in the project testing brief's **AC-004** row and its *Test mix, summarised by
tier* (`_decomposition.md:469-540`).

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Integration (the primary tier)** | `crates/happenstance-ladybug/tests/<conformance target>.rs` — the `projection_store_conformance!(LadybugFixture::new())` invocation — run by `cargo xtask ci`'s `tests` step with `--show-output` (`xtask/src/main.rs:131-151`) | AC-001 (every rule reports), AC-002 (the inherited isolation rule), AC-003 (`SKIP` lines with reasons, visible) |
| **Integration (negative control)** | `crates/happenstance-ladybug/tests/` — the `RowShapedFixture` target, same macro, same suite, outcome recorded not asserted | AC-006; and it is what makes AC-001's green discriminating rather than decorative |
| **Compile-only** | `cargo test -p happenstance-ladybug --test port_shape` — `const _` instantiations in `weak_flavour` and `send_flavour`, kept in separate modules | AC-005 (the port shape holds at the merged batch shape; the `Send`-bound collapse observed) |
| **Gate registry** | `cargo xtask proof-artefact` — its own step in the gate (`xtask/src/main.rs:170-178`), asserting the new `ARTEFACTS` row's names out of `--list` (`xtask/src/proof.rs:195-217`) | AC-004 (an emptied target fails; `running 0 tests` cannot pass) |
| **Gate registry, negative control** | By hand, once: empty the target, re-run `cargo xtask proof-artefact`, record the failure output, restore | AC-004 — a row whose check has never been seen to fail is a row nobody has verified |
| **Static** | `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -D warnings` (`xtask/src/main.rs:116-129`) | NF-005; and it is the tier that catches a blanket `#![allow(dead_code)]` over the shared fixture module |
| **Static (boundary)** | `git diff --stat`; `rg -n "#\[cfg" crates/happenstance-ladybug/tests/`; `cargo xtask spec-trace` | AC-008 — no testkit/core/spec path touched, no rule gated out, no `[FROZEN]` clause moved |
| **Supply chain** | `cargo deny` and the `cargo hack` feature-powerset, both of which resolve on this machine and therefore run rather than skip (`CLAUDE.md`, *Commands*) | AC-008 — the new dev-dependency clears policy without an exemption (EC-007) |
| **Whole gate (merge bar)** | `cargo xtask ci` — **not** `--fast` (`_storymap.md:145-147`) | NF-001; the merge DoD stated in the PR boundary above |
| **Process** | The run record committed in this story's folder: command, rule list, declensions with reasons, control outcome, `port_shape.rs` observation, commit SHA | AC-007 — verified by a reviewer re-running the recorded command at the recorded commit |

`cargo xtask affected --base main` is the story-grain command `.redkiln/config.yaml`'s
`verify:` block wires in, and it will select `happenstance-ladybug` and `xtask`; it is the
inner loop, not the bar.

## Risks and coupling (PR-scoped)

- **The suite accommodates the shape it was written against.** This is the project-level
  risk (`project.md:263-268`) and *this story is where it would be invisible*: a green run
  reads identically whether the rules discriminated or never asked. Mitigation inside this
  PR is AC-006's negative control plus AC-001's count reconciliation; the residual — a
  suite that passes both fixtures — is not a defect to fix here, it is the finding to
  record and route to HS-P0010.
- **The `ARTEFACTS` row is the one place this PR can silently under-deliver.** A row with
  an empty or wrong `tests` list is green today and worthless tomorrow (EC-003). The hand
  negative control in the tests table is the only thing that proves the row bites.
- **Upstream shape drift.** Everything here binds to what `projection-store-freeze` merged
  — the macro's arms, the fixture trait, the capability set, the registered rule list. If
  the shipped shape differs from the sketch these briefs read, **read what shipped**
  (`_decomposition.md:90-119`) and record the divergence; do not reconcile by editing the
  testkit (AC-008).
- **Coupling to `fill-the-bodies-and-ps-34-disposition` is total and one-directional.** A
  suite run against `todo!()` bodies panics rather than reports, so this story cannot start
  early in any degraded form. If HS-S0077 landed a body that is a stub in spirit, this run
  will discover it — and that discovery belongs in the record, not in a local patch to the
  fixture.
- **Slice-mate coupling.** `read-your-own-writes-projection` (HS-S0079) merges next and
  consumes this fixture and this target (`_storymap.md:45`, `:134-135`). Every naming
  decision here — the fixture's module path, the target's name, the emitter and therefore
  the module prefix — is a decision HS-S0079 inherits. Renaming any of them later costs an
  `ARTEFACTS` edit too.
- **`cold-build-cost-and-ci-shape` measures against this target.** It depends on this story
  (`_storymap.md:47`) so that the CI shape is chosen against a gate that already runs the
  conformance run. A second heavy dev-dependency added here would contaminate that number
  (NF-004).
- **Windows teardown.** EC-006 is the most likely source of a "works on my machine"
  divergence in this PR, and the three-OS matrix is where it will surface.
- **What this PR must not absorb.** A rule that seems wrong, a `[FROZEN]` clause that seems
  stale, and a `deny.toml` policy that seems inconvenient are all *findings routed
  elsewhere*, not local edits (AC-008, EC-007, EC-008). The failure mode this story is most
  exposed to is quietly making the tree green.

## Dependencies

**Blocks on**

- **`fill-the-bodies-and-ps-34-disposition`** (HS-S0077) — supplies four real
  `SendProjectionStore` bodies against the real `lbug` driver with
  `#![allow(clippy::todo)]` gone, plus the six `SPECIFICATION.md` citation repairs that
  keep `spec-trace` green. Hard, not soft: a suite run against `todo!()` panics rather than
  reports (`_storymap.md:43-44`, `:60-66`).

Transitively upstream, discharged and **not re-litigated here** (D14):
`preflight-and-unlike-axes` (HS-S0074 — the merged port really shipped `type Batch;`, the
write seam and `projection_store_conformance!`; the dated unlike-axes document),
`adr-0025-three-answers` (HS-S0075 — Q1/Q2/Q3, including the emitter question this target's
macro arm follows), `real-lbug-driver-swap` (HS-S0076).

**Unlocks**

- **`read-your-own-writes-projection`** (HS-S0079) — the slice-mate, which consumes this
  fixture and target to answer PS-12 (`_storymap.md:45`).
- **`cold-build-cost-and-ci-shape`** (HS-S0081) — measures the CI shape against a gate that
  already runs this conformance target (`_storymap.md:47`).
- **`freeze-verdict-document`** (HS-S0082), transitively — this story's run record is the
  raw material for *which implementation, which rules, which clauses, at which commit*
  (`_storymap.md:48`).

## Anchors (progressive disclosure)

Linked, not pasted. Open each at the moment named — the Context pack above already carries
the decisions; these carry the depth behind them.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/src/contract.rs` | The fixture contract in full: `:1-63` states *one instance is one isolated store, each `connect()` is one handle*; `:25-63` and `:355-433` are `Capability`; `:404-410` is the empty-reason codegen trap. Everything AC-002 and AC-003 assert is spelled here. | **First**, before writing a line of the fixture. | AC-002 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` at `:243-271` is the reference implementation — the shape to copy, including how a declension states a real reason. | Immediately after `contract.rs`, before choosing the temp-directory strategy. | AC-002 |
| `crates/happenstance-testkit/src/lib.rs` | The emitter table (`:55-67`) and the macro's arms (`:311-357`) — the general arm `mod_name = …, emit = …, fixture = …` is what a non-default emitter requires, and it fixes the module prefix the `ARTEFACTS` row must carry. | Before writing the target's body, and again before writing the `ARTEFACTS` row. | AC-001 |
| `crates/happenstance-testkit/src/registry.rs` | `:93-103` is the rule registry the emitted count reconciles against; the projection analogue is what makes AC-001's count mechanical rather than a guess. | When reconciling the rule count, after the first successful run. | AC-001 |
| `crates/happenstance-testkit/src/suite.rs` | `:210` is `two_fixture_instances_observe_none_of_each_others_appends` — the *inherited* rule that catches a shared directory, and the reason isolation is a conformance rule rather than a testkit meta-test. | When the isolation rule fails, before "fixing" anything. | AC-002 |
| `xtask/src/proof.rs` | `:9-24` argues why naming a target is not enough; `:58-70` requires fully qualified names; `:133-150` is `ARTEFACTS` (three rows today); `:195-217` is the `--list` assertion that bails on absent names. | Before adding the row, and again when the row's check first fails. | AC-004 |
| `xtask/src/main.rs` | `:131-151` is the `tests` step and the comment explaining why `--show-output` is not noise; `:170-178` is the `proof-artefact` step; `:116-129` is clippy `-D warnings`. | When a `SKIP` line does not appear, or a warning fails the gate. | AC-003 |
| `crates/happenstance-ladybug/tests/port_shape.rs` | The file being extended: `:11-14` explains the two-module split (`error[E0034]`), `:50-61` carries the `for<'a> S::Batch<'a>: Send` bound whose collapse is PS-5's predicted relief. | Before touching it — the instruction is *extend, never replace*. | AC-005 |
| `crates/happenstance-ladybug/src/projection_store.rs` | `:230-249` is `new`/`connect` (the store takes the `Database` by value; per-call connections); `:204-213` is the single-writer note behind the most likely genuine declension; `:252-257` is the `Send`-flavour evidence. | While writing `LadybugFixture::new()` and while choosing capability constants. | AC-002 |
| `crates/happenstance-core/src/projection.rs` | The port itself, read never edited — `:13-30` states the one-transaction invariant (PS-1) that the checkpoint write's placement obeys. | When the run's semantics need checking against the port's stated invariant. | AC-001 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/_decomposition.md` | The architecture brief: M1 (`:127-142`, the macro *is* the target), M2 (`:144-153`), M3 (`:155-164`), M4 (`:166-171`), the seams table (`:70-88`, what is not this project's to edit), M9 (`:210-218`), and `:445-446` (do not weaken `port_shape.rs`). | Whenever a boundary question arises — especially "may I edit the testkit?". | AC-008 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/ladybug-fixture-and-conformance-run/discover.md` | The three wrong implementations, in full, with the falsifier for each: the row-shaped "graph" fixture, the shared directory, the trimmed invocation. Question 3 is the harness-narrowing finding; question 5 is the rule-seems-wrong boundary. | Before writing the negative control, and again before interpreting a fully green run. | AC-006 |
| `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md` | AC-004 verbatim (`:193-196`), DR-5 (`:152-157`), DR-9 (`:172-174`), and the risk register's single-writer and literal-position rows (`:278-294`). | When deciding whether a declension is a stated limit or a defect. | AC-003 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | Capability declension with a stated reason — **consumed here, not re-decided**. Where fixture constants live is not this story's question. | If tempted to move or restructure the capability constants. | AC-003 |
| `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` | Generic code can `begin` and `commit` and cannot write in between; its sub-questions are what the merged write seam answers. If the merged seam cannot express a replayable parameterised statement, that is the freeze not holding. | If the fixture cannot drive a write through the shipped seam. | AC-006 |
| `.kb/decisions/0008-one-derivation-for-both-ports.md` | One derivation for both ports, and the GAT-across-a-suspension-point finding — the constraint behind `port_shape.rs`'s bounds and behind why a batch is owned. | When the `port_shape.rs` bound behaves unexpectedly at the merged shape. | AC-005 |
| `.kb/decisions/0001-async-port-flavours.md` | Why `#[async_trait]` is never an option and why the two flavours exist — the reason `weak_flavour` and `send_flavour` stay separate and the reason a fixture must not import both trait names. | Before adding any bound or import to `port_shape.rs`. | AC-005 |
| `standards/rust/91-adapter-authoring-recipe.md` | The house recipe for standing an adapter's fixture and suite up, with a named wrong implementation per rule. | At the start, alongside `contract.rs`. | AC-002 |
| `standards/rust/61-compile-time-assertions.md` | Why `port_shape.rs` proves its claims with `const _` rather than `#[test]`, and what a compile-time assertion must not be weakened into. | Before editing `port_shape.rs`. | AC-005 |
| `standards/rust/60-what-a-test-must-prove.md` | The named-wrong-implementation discipline the negative control is an instance of — a test no implementation can fail is decorative. | Before writing `RowShapedFixture`. | AC-006 |
| `standards/rust/50-dependency-hygiene.md` | What a new pin owes the workspace, and why a `deny.toml` exemption is not the answer to a failing check. | Before adding the temp-directory dev-dependency. | AC-008 |
| `spec/SPECIFICATION.md` | PS-2 at `:4760-4774` (`[FROZEN]`) is the clause this run is evidence *for*; PS-5, PS-9, PS-11 take data points from it. The clause wins wherever a brief disagrees. | When recording which clauses the run touched, for the verdict handoff. | AC-008 |
| `references/adapter-shapes.md` | What the six skeletons told the type checker, including (`:186-191`) that binding an *owned* type to the GAT bought none of PS-5's relief — which is what stops the `port_shape.rs` observation being vacuous. | When writing up the bound-collapse observation. | AC-005 |
| `references/evaluation/README.md` | The evidence lifecycle — *immutable, dated, pinned to a commit, superseded rather than edited* — the discipline the run record follows even though it lives in this story's folder rather than there. | When writing the run record. | AC-007 |

## Clarifications resolved during spec

1. **The eight AC ids are exactly those the front half enumerated** — AC-001 … AC-008,
   none added, none dropped. They partition project **AC-004** into the five things a
   reviewer must be able to check (the whole suite ran; it ran in isolation; declensions
   are legible and reasoned; the gate notices the target's contents; the port shape held)
   plus the three that make the green mean something (the negative control, the re-takeable
   record, the untouched boundary).
2. **The run record lives in this story's folder, not `references/evaluation/`.**
   `_decomposition.md:210-218` (M9) reserves `references/evaluation/` for the two
   commit-ordered documents owned by `preflight-and-unlike-axes` and
   `freeze-verdict-document`. Writing a third file there from this story would put a
   non-evidence artefact into a directory whose README requires supersession rather than
   editing, and would blur AC-001's ordering claim. The PR boundary above enforces it.
3. **"Interaction quality" is not vacuous here even though `_design.md` says `N/A`.** The
   design stage's `N/A` is a determination about *screens*; the gate's captured output is
   still the only thing a human perceives from this work, and the project's own DR-9
   already treats its legibility as a requirement. So the invariants are applied to that
   output — and every one of them was written as an AC row rather than a bullet, because
   `redkiln verify` extracts ACs from table cells and bullets, not from prose.
4. **The conformance target's file name is deliberately left as
   `<conformance target>`.** It cannot be fixed before ADR-0025's Q3 answer selects the
   emitter, which selects the macro arm, which fixes `mod_name` and therefore the qualified
   test names the `ARTEFACTS` row must carry (D4, D8). Fixing a name here would either
   pre-empt that decision or guarantee a rename. What *is* fixed: it lives under
   `crates/happenstance-ladybug/tests/`, its body is the macro invocation and nothing else,
   and its name is a decision `read-your-own-writes-projection` inherits.
5. **The negative control is one fixture, not a second suite.** `RowShapedFixture` is run
   through the same `projection_store_conformance!` macro. Nothing in this story writes a
   conformance rule — that is `projection-store-freeze`'s, and writing one here would be
   the "local rule that makes the run look more thorough" `discover.md` names.
6. **A fully green negative control is not a blocker for this story.** It is a recorded
   finding routed to HS-P0010 and carried into the verdict. Blocking the merge on it would
   put this story in the business of fixing the suite, which `_decomposition.md:86-88`
   forbids; suppressing it would destroy the only evidence that the freeze re-test was
   discriminating.
7. **The temp-directory crate is not named here.** D13 fixes the *process* — pinned in
   `[workspace.dependencies]`, named `…workspace = true`, must clear `cargo deny` without
   an exemption — and leaves the choice to the implementer, because a crate that fails the
   licence or advisory check must be swappable without a spec amendment.
8. **`--show-output` is inherited, not re-argued.** The gate already passes it and
   `xtask/src/main.rs:131-142` states why. This story must not add a second, story-local
   way of surfacing `SKIP` lines; AC-003 is satisfied by the existing step or it is not
   satisfied.
