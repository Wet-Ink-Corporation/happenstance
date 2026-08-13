---
item: HS-S0030
stage: spec
created: 2026-08-12T13:46:27.121Z
updated: 2026-08-12T13:46:27.121Z
template_sig: 87bbf1d0
rendered_sig: 61d173a8
---

# Spec — The compile-fail case, its negative control, and its gate row

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — AC-02 (`:310-313`), DoD 2 (`:363-365`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` — AC-002 (`:166-168`), risk rows 2 and 4 (`:266`, `:268`) |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/compile-fail-proof-artefact/spec.md` |
| Story map row | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md:61` (M6), merge order `:127-129` |
| Signed-off design (**binding**) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` — surface `compile-fail-diagnostic` (`:78-83`), pattern decision (`:210-225`), anti-pattern 13 (`:999-1001`), sign-off (`:1224-1259`) |
| Key briefs | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` — architecture AC-A03 (`:372-380`) and AC-U07 (`:132-139`); testing brief AC-002 row (`:780`) and *`trybuild` has no home yet* (`:828-841`) |
| Project intake | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_intake-brief.md` — *Proof artefact* (`:75-82`) |
| Roadmap pointer | `RUNBOOK.md:157` (phase 7's row and its proof artefact); `RUNBOOK.md:3614-3627` (the decorative instrument this story exists not to repeat) |
| Depends on | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/worked-example-on-typed-layer/spec.md` (HS-S0029) |

## One-line PR slice

Land the compile-fail case that adds a variant to the worked example's domain enum
and its **negative control**, with the diagnostic landing on the user's own `match`
arm and not inside a macro body (AC-U07), registered as a row in
`xtask/src/proof.rs`'s `ARTEFACTS` (`:133`) so the gate runs `cargo xtask
proof-artefact` — which a deleted *or* an emptied case fails — and verify the
negative control fails rather than assuming it.

## Executive summary

This PR turns the project's central claim — *the compiler protects the domain* —
from a convention into a checked guarantee, and it is the only story in HS-P0011
whose entire deliverable is an instrument.

**Pointer.** The claim itself is built by the slice-mate `worked-example-on-typed-layer`
(HS-S0029): once `DecisionModel::apply(&mut self, Self::Event)` folds a domain enum
(`_design.md:411-419`), a variant added to that enum makes the fold's `match`
non-exhaustive and the build stops. Nothing in this PR re-argues that shape.

**Delta.** Three artefacts and one row:

1. a `trybuild` **compile-fail fixture** — the example's domain enum plus one
   variant, its fold otherwise verbatim — pinned by a checked-in `.stderr` snapshot
   whose first `-->` span points at the fold's own `match` arm;
2. a **negative control** — the identical file with the new variant handled, which
   must *compile*, so a failure caused by anything other than the missing arm is
   visible rather than mistaken for the guarantee;
3. a **recorded mutation** — the protection deliberately removed (a `_ =>` wildcard
   added to the fail fixture) and the gate observed to go red, because AC-002 is a
   claim about discrimination and a discriminator nobody ran is an assertion;
4. a row in `xtask/src/proof.rs::ARTEFACTS` (`:133`) naming both tests, so deletion
   *and* emptying *and* renaming each fail the gate rather than only the first.

**And one blocking input this story does not own.** `trybuild` is absent from
`Cargo.toml` and `Cargo.lock` (verified: `grep -rn trybuild Cargo.toml Cargo.lock
crates/*/Cargo.toml examples/*/Cargo.toml xtask/Cargo.toml` returns nothing), and
its adoption is HS-P0010's decision (`spec/SPECIFICATION.md:8772`). If it has not
landed when this story starts, **halt and escalate** — the substitute is
measurably weaker and the design has already refused it (`_design.md:215-225`).

## Context pack

Read this section and you can start. Everything deeper is a signposted anchor.

**The decision this story exists to serve, in one sentence.** Initiative AC-02
promises the application author is *"protected by the compiler when their domain
grows… and that protection is asserted by a case that fails if it regresses — not
by a convention"* (`initiative.md:310-313`), and DoD 2 restates it as a scenario
that must be *run and observed*: *"it fails to compile with the expected
diagnostic, and removing the protection makes the case fail"* (`initiative.md:363-365`).
The second clause is the whole story. A case that only proves *something* failed to
compile is satisfied by a typo.

**Why a `compile_fail` doctest is not an option, and this is settled rather than
re-argued.** Two independent defects, both measured in this repository:

- rustdoc collects doctests from the **lib target only**, so a `compile_fail` block
  inside `tests/` is compiled as prose and never handed to a compiler. Phase 4's
  proof artefact was decorative for exactly this reason and it was found by phase 5
  (`RUNBOOK.md:3614-3627`). `examples/course-subscriptions` has no lib target at all.
- rustdoc on 1.97.1 **silently ignores an unmatched error-code annotation**, so
  `compile_fail,E0004` asserts nothing more than bare `compile_fail`, and the bare
  form passes on *any* compile error. The specification says this in PS-36's own
  disposition row — pinning a compile-fail diagnostic *"needs a `trybuild`-style
  stderr snapshot, which is a dependency decision phase 6 owns"*
  (`spec/SPECIFICATION.md:8772`) — and `crates/happenstance-core/src/event.rs:95-106`
  says it about the same construct in its own doc comment.

The consequence is precise: **the negative control cannot discriminate under a
doctest**, and the negative control is the entirety of AC-002. `_design.md:222-225`
records the refusal and the escalation route: *"If HS-P0010 declines it, escalate;
do not substitute."*

**The mechanism being pinned is exhaustiveness, and only exhaustiveness.** The
design is explicit that `EVENT_TYPES` ↔ `event_type()` agreement is **not**
compiler-enforced and is tested by `assert_domain_event` instead
(`_design.md:641`). What *is* compiler-enforced is `DecisionModel::apply`'s fold
over `Self::Event`: a domain enum, matched without a wildcard, so a new variant is
`error[E0004]: non-exhaustive patterns` at the caller's `match`. The fixture must
therefore contain **no `_ =>` arm** — that absence *is* the protection, and it is
also the mutation AC-004 removes.

**Where the diagnostic is allowed to land.** AC-U07: *"the `stderr` fixture points
at the user's `match` arm in `examples/course-subscriptions/`, not into a macro
body"* (`_decomposition.md:132-139`), and anti-pattern 13 makes it a checkable
failure: *"The compile-fail fixture's `-->` span points into a file the user did
not write — a macro body, or anything under `crates/`"* (`_design.md:999-1001`). A
guarantee whose diagnostic names a file the user did not write is a guarantee they
cannot act on.

**One design conflict, resolved here toward the rule and flagged for confirmation.**
`_design.md:80`'s surface manifest gives `compile-fail-diagnostic` the route
`crates/happenstance/tests/ui/`, while the same file's anti-pattern 13 forbids a
span *"anything under `crates/`"* and its selector says the span points into
`examples/course-subscriptions/`. The two cannot both be literally true, because a
`trybuild` fixture's span is the fixture file's own path. **This story binds the
rule and places the harness at `examples/course-subscriptions/tests/ui.rs` with its
fixtures under `examples/course-subscriptions/tests/ui/`**, which satisfies the
selector, AC-U07 and anti-pattern 13 simultaneously. Two facts make that the only
workable placement rather than a preference: the example is a **binary crate with
no lib target**, so a fixture elsewhere could not reference its domain at all; and
the variant must be added *in the fixture*, because you cannot extend an imported
enum — so the fixture necessarily restates the enum, and the file that restates it
is the file the user is standing in. Record the deviation in the implementation
report and raise it at the slice review; do **not** edit `_design.md` and do not
soften anti-pattern 13.

**The gate mount, and why naming the target is not enough.** `xtask/src/proof.rs`
exists because *"`cargo test` exits 0 on `running 0 tests`, so an emptied file
passes a step that a deleted one fails"* (`proof.rs:14-18`; the same argument is in
the gate step's own comment at `xtask/src/main.rs:174-178`). So the row does not
name a target — it names the **tests inside it**, asserted out of `cargo test --
--list` before they run (`proof.rs:199-216`). Two consequences bind the
implementation: the names must be **fully qualified**, which means the test target
wraps its `#[test]` functions in an inner `mod` of the target's own name
(`proof.rs:64-69`, the convention every existing row follows); and the list is a
**subset** check, so a later third test needs no gate edit (`proof.rs:34-39`).

**The persona slice this realizes.** P4's beat is one-shot and time-boxed — the
`cargo add` reader does not get a second pass (`_decomposition.md`, UX brief
persona table). This story does not render anything to that reader. It renders to
the *maintainer* six months out who deletes a fixture with a dead-code instinct:
`WIRE_NEGATIVE_CONTROLS` (`proof.rs:104-118`) exists because that has already
happened here, and its comment names the failure mode — *"a reader with a dead-code
warning and a tidy instinct removes the instrument and leaves the positive tests
green."* This story's fixtures are exactly that shape, and the row is the answer.

**Standing constraints that apply unchanged.** No `#[async_trait]`; no `serde` in
`happenstance-core`'s defaults; `read` returns the stream at the top level; generic
code binds `EventStore`, never `SendEventStore` (`CLAUDE.md`, *Binding
constraints*; `_design.md:1008-1010`). This story writes no port code, so it
touches none of them — but its fixtures compile against `happenstance`, and a
fixture that binds `SendEventStore` would ship the wrong example in the one file a
reader is being pointed at.

**What must not happen.** No edit under `crates/happenstance-core/src/**` (AC-A02;
`_decomposition.md:363-371`). No amendment to a `[FROZEN]` ES-\* clause. No
widening of `deny.toml`'s licence allowlist (`:10-19`) to make a transitive
dependency of `trybuild` pass — that is an escalation, not a boundary edit.

## Integration contract

- **Archetype**: `capability` — user-observable, end to end: the author adds a
  variant, the build stops, and the case that says so runs in the gate.
- **Slice / milestone**: **M6 `worked-example-and-proof`**. Slice-mate:
  `worked-example-on-typed-layer` (HS-S0029). Both are implemented in one context
  and mounted as one surface; this story is second in the slice and cannot precede
  its mate, because it points its diagnostic at the mate's own fold
  (`_storymap.md:127-129`).
- **Mount point**: **`xtask/src/proof.rs`** — a new `Artefact` row in the
  `ARTEFACTS` const (`:133-149`), with a `const` name list beside
  `META_TESTS` (`:85-102`), `WIRE_NEGATIVE_CONTROLS` (`:114-118`) and
  `SYNC_WIRE_TESTS` (`:127-130`). This is the composition root the architecture
  brief names for AC-002: *"the gate mount for AC-002"* (`_decomposition.md:471`).
  Nothing else in the tree would notice the instrument's absence, which is the
  whole argument for mounting here (`xtask/src/main.rs:155-178`).
- **Wires into**:
  - `xtask/src/main.rs:178-190` — the REQUIRED step *"each phase's proof artefacts"*,
    which already invokes `cargo xtask proof-artefact`. **No edit needed**: the row
    is consumed by the existing step, and the help text's artefact count is derived
    from the const (`xtask/src/main.rs:753-761`), so it updates itself.
  - `examples/course-subscriptions/src/main.rs` — the domain enum and
    `DecisionModel` fold HS-S0029 lands; the fixture mirrors it and the `.stderr`
    span names this crate's directory.
  - `examples/course-subscriptions/Cargo.toml` — gains `[dev-dependencies] trybuild`
    and already depends on `happenstance` after HS-S0029 (`_decomposition.md:468-470`).
  - `Cargo.toml:80-98` — `trybuild` is declared once in `[workspace.dependencies]`
    under the `# --- dev / tooling only ---` header, the single-version rule this
    workspace states in its own comment (`Cargo.toml:15-18`).
  - `deny.toml:8-19` — the licence allowlist every new dependency must satisfy
    unmodified.
  - `rust-toolchain.toml` — the pinned `1.97.1` channel the `.stderr` snapshot is
    taken under.
- **Renders surfaces**: **`compile-fail-diagnostic`** (`_design.md:78-83`) — the
  only surface this story renders. Its states: `protection-present-fails-to-compile`,
  `negative-control-compiles`, and `blocked-no-instrument` (which is the halt state,
  not a shipped one). It renders **no** other surface: `crate-root-rustdoc`,
  `crate-readme`, `first-program-doctest`, `worked-example-transcript` and
  `dsl-failure-message` all belong to other stories.
- **Public items**: **none.** This story adds no row to `_design.md`'s `## Items`
  block, and that is correct rather than an omission — the deliverable is a test
  target and a gate row, and neither is a name a caller writes. It must therefore
  not `pub`-export anything from `happenstance` or `happenstance-testkit`.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** The
  conformance suite observes *store* behaviour; the subject here is a compile-time
  property of an application's own fold, which no adapter can pass or fail. The
  analogous instrument in this repository is not a rule but a proof artefact — the
  mutant registry's meta-tests (`crates/happenstance-testkit/tests/mutation_coverage.rs`,
  named at `proof.rs:85-102`) — and this story joins them at the same mount. The
  story changes no port, so `CLAUDE.md`'s *"a port change nothing can fail"* test
  does not bite.
- **Clause(s)**: **discharges none; amends none.** It stands beside **PS-36**, whose
  disposition row records that its rule *"would be right to write and cannot be, on
  stable, without a `trybuild`-style stderr snapshot"* (`spec/SPECIFICATION.md:8772`).
  Landing that snapshot is a fact PS-36's row will eventually want, but **editing
  it is out of scope here** — PS-36's disposition is a `[FROZEN]`-adjacent judgement
  owned by the clause-ledger audit (HS-P0016), and `cargo xtask spec-trace` must
  stay green without touching it.
- **Advances DoD scenario**: **initiative DoD 2 — *"@smoke — the compiler protects
  the domain"*** (`initiative.md:363-365`), and it is the *only* story that moves
  it. Project DoD 2 (`project.md:226-227`) is the same scenario at project grain.

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and
fails on any file changed outside it.

```
examples/course-subscriptions/tests/**
examples/course-subscriptions/Cargo.toml
xtask/src/proof.rs
Cargo.toml
Cargo.lock
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/compile-fail-proof-artefact/**
```

**In this PR**

- `examples/course-subscriptions/tests/ui.rs` — the harness target, its `#[test]`s
  wrapped in `mod ui`.
- `examples/course-subscriptions/tests/ui/*.rs` and the checked-in `*.stderr`
  snapshot — the fail fixture, the negative control, and the pinned diagnostic.
- `xtask/src/proof.rs` — one `Artefact` row plus its name `const`, with the comment
  that earns the row the way `WIRE_NEGATIVE_CONTROLS` (`:104-113`) earns its own.
- `Cargo.toml` + `Cargo.lock` — `trybuild` as a workspace dev/tooling dependency.
- `examples/course-subscriptions/Cargo.toml` — `[dev-dependencies] trybuild.workspace = true`.
- This story's own folder: the ledger and implementation report, carrying the
  recorded mutation and the design-deviation note.

**Explicitly not in this PR**

- Any change under `crates/happenstance/src/**` or `crates/happenstance-core/src/**`.
  The typed layer is HS-S0029's and the milestones before it; the contract crate is
  frozen (AC-A02, `_decomposition.md:363-371`).
- `examples/course-subscriptions/src/main.rs` — HS-S0029 rewrites it; this story
  reads it and must not adjust the domain to make a fixture convenient. If the fold
  needs a change for the diagnostic to land on the `match`, that is a finding
  against the slice-mate, raised in the same context, not a silent edit here.
- `xtask/src/main.rs` — no new REQUIRED step. The existing proof-artefact step
  consumes the new row.
- `spec/SPECIFICATION.md` (including PS-36's disposition row), `CHANGELOG.md` (the
  alpha's, owned by `publish-0-2-0-alpha-1`), and `_design.md` (binding; the route
  conflict is reported, not edited).
- `deny.toml` — a licence failure is escalated, never allowlisted here.

**Merge DoD.** `cargo xtask affected --base main` is green, `cargo xtask
proof-artefact` names and runs both tests, and the recorded mutation has been
performed and observed red before being reverted.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **A variant added to the domain enum stops the build** | The fail fixture restates the example's `DomainEvent` enum with one extra variant and keeps its `DecisionModel::apply` fold otherwise verbatim, **with no `_ =>` arm**. The compiler emits `error[E0004]: non-exhaustive patterns` at the `match`. `trybuild`'s `compile_fail` entry asserts the build fails *and* that stderr matches the checked-in snapshot byte for byte. | `_design.md:411-419` (`apply(&mut self, Self::Event)`); `_design.md:1079-1085` (the fold's two arms); `initiative.md:363-365` |
| **The diagnostic lands on the caller's own `match`** | The snapshot's first error block and its `-->` span name a path under `examples/course-subscriptions/tests/ui/`, never a macro body and never `crates/happenstance/src/**`. Reviewed by reading the committed `.stderr`, which is the artefact — not by trusting an error code. | `_decomposition.md:132-139` (AC-U07); `_design.md:212-214` (selector); `_design.md:999-1001` (anti-pattern 13) |
| **The negative control compiles** | A second fixture, identical to the first except that the added variant **is** handled by a real arm, registered as `trybuild`'s `pass`. One edit is the only delta between the two files. If the fail case were failing for an unrelated reason — a typo, a missing import, a moved item — the control fails too and the pair reports it. | `_intake-brief.md:75-82`; `project.md:166-168`; RS-62-1, `standards/rust/62-doctests-and-harnesses.md:12` (pair every compile-fail with a compiling one; do not trust the code) |
| **The control is verified to discriminate, not assumed to** | Before the PR closes, a `_ =>` wildcard is added to the **fail** fixture and `cargo xtask proof-artefact` is run and observed **red** (trybuild reports the case compiled when it was expected to fail); the edit is then reverted. The exact command and the observed output are recorded in this story's ledger. This is the same discipline the mutant registry applies to conformance rules, at the one place in this project where no registry exists. | `RUNBOOK.md:3614-3627` (an instrument that cannot fail is a finding); `crates/happenstance-testkit/tests/mutation_coverage.rs` via `proof.rs:85-102`; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| **The gate names the tests, not the target** | A new `Artefact { package: "course-subscriptions", target: "ui", tests: … }` row in `ARTEFACTS`, its `tests` a `const` naming both fully-qualified test paths — `ui::<fail-case>` and `ui::<pass-case>` — so the `--list` subset assertion fails on a rename as well as on an emptying. The harness wraps its tests in `mod ui` so the qualified names match, which is the convention every existing row relies on. | `xtask/src/proof.rs:133-149` (the table), `:57-70` (row shape and the `mod` convention), `:199-216` (the subset assertion and its message) |
| **Deletion, emptying and renaming each fail** | Deleting `tests/ui.rs` fails with `no test target named 'ui'`; truncating it to nothing fails the name assertion before a test runs; renaming either test fails the same assertion with the listing printed. `cargo test` alone would pass two of those three. | `xtask/src/main.rs:155-190` (the step's own argument); `xtask/src/proof.rs:9-23` |
| **The instrument is adopted on the workspace's terms** | `trybuild` is declared once in `[workspace.dependencies]` under `# --- dev / tooling only ---`, and consumed as `trybuild.workspace = true`; the example crate is `publish = false`, so nothing published gains an edge. `cargo deny check licenses` passes against the unmodified allowlist, and `cargo deny check advisories` is clean. | `Cargo.toml:15-18` (one version, workspace-wide), `:80-98` (the dev/tooling block and the `postcard` precedent for measuring rather than guessing a feature default); `deny.toml:8-19`; `examples/course-subscriptions/Cargo.toml:8` |
| **The snapshot is stable under the toolchains this repo runs** | rustc diagnostics are version-sensitive, so the snapshot is authored under the pinned `1.97.1` channel — which is also the MSRV and the compiler CI's `msrv` job uses, so the two cannot disagree today. The nightly `--cfg docsrs` step is a rustdoc build and runs no tests. Regeneration is `TRYBUILD=overwrite`, and a regenerated snapshot is reviewed as a diff, never accepted blind. | `rust-toolchain.toml`; `CLAUDE.md`, *Binding constraints* 5 and *Commands* (the `msrv` job runs the same compiler as the gate) |
| **The blocked input has a halt, not a fallback** | If `trybuild` has not been adopted when implementation starts, the story **stops** and escalates to HS-P0010 rather than substituting a `compile_fail` doctest. The design records the refusal in terms and the surface carries a `blocked-no-instrument` state for exactly this. | `_design.md:222-225`, `:78-83`, `:1257-1259`; `_decomposition.md:674-687`; `spec/SPECIFICATION.md:8772`; `project.md:266` |
| **Nothing public is added** | No new `pub` item, no re-export, no feature. The story's whole surface is a test target and a gate row. | `_design.md:229-379` (`## Items` — no row for this story) |

## Data and migrations

**N/A — no data, no schema, no store, no migration.** This story writes no
persistent state and touches no event store, projection store or wire format. The
only files it creates that behave like *fixtures* are the checked-in `.stderr`
snapshots, and they are compiler output pinned as an expectation rather than data:
they are regenerated with `TRYBUILD=overwrite` under the pinned toolchain and
reviewed as a diff, because an accepted-blind snapshot is a test that agrees with
whatever happened. `Cargo.lock` gains `trybuild` and its transitive dev-only
dependencies; that lockfile delta is reviewed for licence conformance
(`deny.toml:8-19`) and for new majors of crates already in the graph
(`deny.toml:23`, `multiple-versions = "warn"`), and it is the only tracked-state
change in the PR.

## Acceptance criteria

Each criterion is framed from what a person is trying to do and crosses the whole
stack — from the edit a human makes to the thing the gate observes — rather than
restating a capability. The personas are the initiative's own, as this project's UX
brief names them: **P1, the application author**, who wants to *"model a consistency
boundary once, against a contract"* and whose named fear is *"being the one who
discovers a contract defect in production, after they have already built on it"*;
**P4, the evaluator**, whose beat is *"one bounded sitting"*; and **P2, the adapter
author**, whose stake here is entirely negative — *"not be disturbed"*
(`_decomposition.md:47-49`, citing
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`).
A fourth reader has no persona atom and is named anyway, because the mount point
exists for them: the **maintainer six months out** with a dead-code warning and a tidy
instinct (`xtask/src/proof.rs:104-118`).

Two test names are fixed here rather than left to the implementer, because
`xtask/src/proof.rs`'s row must spell them and a name chosen twice is a name spelled
two ways: **`ui::an_unhandled_variant_fails_to_compile`** and
**`ui::the_negative_control_compiles`**, both inside `mod ui` in
`examples/course-subscriptions/tests/ui.rs` so the fully-qualified names the gate
asserts actually match (`proof.rs:64-69`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **P1's domain grows and the build stops.** GIVEN P1 has modelled a consistency boundary on the typed layer and shipped it, WHEN a variant is later added to the domain enum and `DecisionModel::apply`'s fold is not extended, THEN the build stops at `error[E0004]: non-exhaustive patterns` instead of compiling and silently ignoring the new fact — and this repository *asserts* that rather than believing it: a `trybuild` `compile_fail` fixture restates the example's enum plus one variant, keeps the fold otherwise verbatim with **no `_ =>` arm**, and is pinned by a checked-in `.stderr` matched byte for byte. | `examples/course-subscriptions/tests/ui.rs` → `ui::an_unhandled_variant_fails_to_compile`, over the fail fixture and its snapshot under `examples/course-subscriptions/tests/ui/`; run by `cargo xtask proof-artefact`. |
| AC-002 | **The diagnostic names a file P1 wrote, and it is the first thing they read.** GIVEN the build has stopped, WHEN P1 reads the compiler's output top-down, THEN the **first** error block is the non-exhaustive-match error and its `-->` span names a path under `examples/course-subscriptions/` at the `match` arm they own — never a macro body, never anything under `crates/` — and nothing precedes it: no unrelated error, no warning from an unused import in the fixture. A guarantee whose diagnostic names a file the user did not write is a guarantee they cannot act on (AC-U07; anti-pattern 13; the surface's own selector). | The committed `.stderr`, asserted byte-for-byte by the AC-001 test — so a span that moves fails the same test — and read as an artefact at the slice review against `_design.md:81`, `:212-214`, `:999-1001`. Never verified by trusting an error code (RS-62-1, `standards/rust/62-doctests-and-harnesses.md:12`). |
| AC-003 | **A red build means what it says (the negative control).** GIVEN a reader wants to know the build was stopped by the *missing arm* and not by a typo, a renamed import or an item moved behind a feature, WHEN the pair runs, THEN a second fixture — identical except that the added variant **is** handled by a real arm — compiles and passes, so any unrelated breakage turns both cases red and is legible as breakage rather than banked as the guarantee. One edit is the whole delta between the two files. | `ui::the_negative_control_compiles`, a `trybuild` `pass` entry over the control fixture; run by the same `cargo xtask proof-artefact` invocation. |
| AC-004 | **The discriminator is observed to discriminate, not assumed to.** GIVEN this repository has already shipped one proof artefact that could not fail and found out two phases later, WHEN this story closes, THEN the protection has been deliberately removed — a `_ =>` wildcard added to the **fail** fixture — `cargo xtask proof-artefact` has been run and observed **red**, the edit reverted, and the exact command plus the observed output recorded. AC-002 of the project is a claim about discrimination, and a discriminator nobody ran is an assertion. | The recorded mutation transcript, cited as evidence on this AC's `_ledger.md` row and reproduced in `implementation-report.md`; the discipline is `.kb/decisions/0010-the-suite-must-prove-itself.md`'s, applied at the one place in this project where no mutant registry exists (`RUNBOOK.md:3614-3627`). |
| AC-005 | **The instrument survives a tidy-minded maintainer.** GIVEN a maintainer six months out meets two fixtures that nothing else in the tree references, WHEN they delete the target, truncate it, `#[ignore]` a test or rename one, THEN the gate fails and *names what is missing* — because the new `ARTEFACTS` row names the **tests**, asserted out of `cargo test -- --list` before anything runs, not the target. A later third test needs no gate edit, because the check is a subset one. | A new `Artefact` row plus its name `const` in `xtask/src/proof.rs` (`:133-149`, shape at `:57-70`), enforced by the assertion at `:199-216` and reached by the existing REQUIRED step at `xtask/src/main.rs:178-190`; exercised by `cargo xtask ci --fast`. |
| AC-006 | **P4's sitting and P2's pin both cost nothing.** GIVEN P4 has one bounded sitting and P2 pins `happenstance-core` and must not be disturbed, WHEN `trybuild` enters the tree, THEN it enters **once** in `[workspace.dependencies]` under the `# --- dev / tooling only ---` header and is consumed only as a `[dev-dependencies]` of the `publish = false` example — so no published crate's graph, feature set or MSRV floor moves — `cargo deny check licenses` and `cargo deny check advisories` pass against the **unmodified** `deny.toml`, and no `pub` item, re-export or feature is added anywhere. | `Cargo.toml` (`:80-98`), `examples/course-subscriptions/Cargo.toml`, `Cargo.lock` diffs; `cargo deny check licenses` + `cargo deny check advisories` with `deny.toml:8-19` untouched; the packaging step at `xtask/src/main.rs:519-529` via `cargo xtask ci --fast`; `_design.md`'s `## Items` block (`:229-379`) gains no row. |
| AC-007 | **The pinned diagnostic is a composed artefact, not bare output, and it stays where it belongs.** GIVEN the only thing a reviewer can read to judge this surface is the committed text, WHEN they open it, THEN: the `.stderr` is rustc's own render, **generated** under the pinned `1.97.1` channel and regenerated only via `TRYBUILD=overwrite` reviewed as a diff — never hand-written and never hand-edited to make a case pass; the fixture is a complete program in the shape P1 would actually write, using the example's own domain and `rustfmt`-clean at the 100-column budget, not a synthetic minimal repro; every fixture source line the diagnostic quotes is ≤ 80 columns, so the quoted line and its caret do not wrap at the terminal budget this design sets for text read in a terminal; and the surface's disposition stays **opened on demand** — this PR adds no region, bullet or link to `crate-root-rustdoc` or `crate-readme`. | `cargo fmt --check` and `cargo xtask affected --base main` (rustfmt/clippy on the fixture and on `xtask`); the committed `.stderr` and fixture read at the slice review against `_design.md:78-83` (surface), `:210-214` (pattern), `:810-816` (transience categories) and `:845-856`, `:859-864` (density); the empty `crates/happenstance/**` diff, which the PR boundary already forbids. |

## Interaction quality

RFC §6.7/D6, in the medium this story actually has: there is no screen, so *state* is
what a reader gets back from a command and *composition* is the committed text they
open. Every invariant below is carried by an AC-### row in the table above — this
section only says **which** row carries it and how it is checked. Nothing here is a
free-floating bullet, because a bullet in this section gets no ledger row and is
therefore never gated.

**STATE invariants.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the failure is actionable at the call site without opening a second document | **AC-002** | The `-->` span names the reader's own `match` arm; the analogue of AC-U10 (`_decomposition.md:155-160`) in the compile-time medium. Byte-for-byte in the snapshot. |
| **Non-occlusion** — nothing hides the thing the reader needs | **AC-002**, **AC-005** | AC-002: the E0004 block is *first*, with no preceding warning or unrelated error to scroll past. AC-005: when a named test is absent the gate prints the count, the missing names **and the full listing** rather than a bare exit code (`proof.rs:209-215`). |
| **Preserved position** — the reader's place in the artefact survives a change to it | **AC-007** | `TRYBUILD=overwrite` rewrites the snapshot in place, so git shows what moved; the snapshot is never deleted and re-created, which would render every line as changed and destroy the review. |
| **Reversibility** — no state a reader enters is a state they cannot leave | **AC-004**, **AC-007** | AC-004: the mutation is applied and *reverted*, and the tree ends where it began — the ledger cites the observation, not a lingering edit. AC-007: a regenerated snapshot is a diff a reviewer can reject; an accepted-blind overwrite is the failure mode being forbidden. |
| **Reachability without a special act** — every state is reachable by one copy-pasteable command, no interactive step | **AC-001**, **AC-003**, **AC-005** | All three states reduce to `cargo xtask proof-artefact`. There is no prompt, no TTY assumption and no environment variable required to *run* the cases; `TRYBUILD=overwrite` is required only to *regenerate* one, which is the correct asymmetry. |

**COMPOSITION invariants**, taken from the signed-off `_design.md`, which is binding on
this story because it renders the `compile-fail-diagnostic` surface (`:78-83`).

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the surface is a composed artefact, not raw markup | **AC-007** | The `.stderr` is rustc's real render of a real program, and the fixture is written in the shape P1 writes rather than as a minimal repro. A synthetic two-line repro would satisfy every "it fails to compile" assertion perfectly and teach nothing. |
| **Composition and placement** — the pattern and its selector | **AC-002** | Pattern `pinned-stderr-snapshot`; selector *"the .stderr fixture's first error block and its `-->` span"* (`_design.md:79-81`). The story's resolution of the route conflict is stated in the Context pack and reported, not edited. |
| **Transience** — persistent chrome vs revealed vs opened on demand | **AC-007** | This surface is **opened on demand** under the design's library reading of the three categories (`_design.md:810-816`): reaching it requires an explicit act — running the gate or opening `tests/ui/`. The PR therefore adds nothing to the two persistent-chrome surfaces, and the PR boundary makes that mechanical. |
| **Density budget, with its real numbers** | **AC-007** | Fixture code at the **100-column** rustfmt budget (`_design.md:847`), enforced by `cargo fmt --check`; every fixture line the diagnostic quotes ≤ **80 columns**, so rustc's quoted source line plus its caret row fits the terminal budget the design sets at `:861`. The `.stderr` itself is generated and is not budgeted — it is measured by keeping the *source* it quotes short, which is the only lever that exists. |
| **Hierarchy** — primary, secondary, recessive | **AC-002** | Primary is the first error block and its span, carried by **position** (first) and by being the only error. Recessive is everything trybuild prints around it. A fixture that emits a warning before the error inverts that and fails AC-002. |
| **Named anti-patterns** | **AC-002**, **AC-006** | AC-002 carries anti-pattern **13** verbatim — *"The compile-fail fixture's `-->` span points into a file the user did not write"* (`_design.md:999-1001`) — which is the only numbered anti-pattern addressed to this surface. AC-006 carries the standing list at `_design.md:1008-1010` as it applies to fixture code: a fixture that bound `SendEventStore` or reached for `#[async_trait]` would ship the wrong example in the one file a reader is pointed at. Anti-patterns 1-12, 14 and 15 belong to other surfaces; this story must leave every one of them exactly as it found it. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | `trybuild` has not been adopted as a workspace dependency when implementation starts (HS-P0010's decision, still open — `project.md:266`, `_decomposition.md:828-841`) | **Halt and escalate.** Do not substitute a `compile_fail` doctest: the negative control cannot discriminate under one, and the negative control is the whole of AC-002. The surface carries `blocked-no-instrument` for exactly this state (`_design.md:82`, `:222-225`). The story stops at the slice boundary; it does not ship a weaker instrument and label it AC-002. |
| EC-002 | A transitive dependency of `trybuild` carries a licence outside `deny.toml`'s allowlist, or trips `cargo deny check advisories` | **Escalate; never widen the allowlist.** `deny.toml:8-19` is a boundary this story does not own, and the `postcard` comment at `Cargo.toml:87-93` is the in-tree precedent for measuring a dependency's real graph rather than assuming its default features are harmless. |
| EC-003 | The generated `.stderr`'s first `-->` span points into `crates/`, into a macro body, or at anything other than the fold's `match` | The **fixture placement or the fold is wrong**, and the fix is there. Hand-editing the snapshot to say the right thing is forbidden: it converts the only artefact that can fail into one that agrees with whatever happened (AC-007). |
| EC-004 | The mutation of AC-004 is applied and the gate stays **green** | The instrument is decorative and the story cannot close. This is the phase-4 failure repeated, and the response is the same one that found it: report it as a finding, fix the case so it discriminates, and re-run — do not record the AC as satisfied with a green mutation. |
| EC-005 | The fold that HS-S0029 lands carries a `_ =>` wildcard, or matches on something other than `Self::Event`, so no added variant can make it non-exhaustive | AC-001 is unreachable and this is a **finding against the slice-mate**, raised in the same context and the same slice review — never a silent edit to `examples/course-subscriptions/src/main.rs` from this story, and never a fixture rewritten to prove a weaker claim. |
| EC-006 | The gate's name assertion fires because a test was renamed deliberately | Follow the message's own instruction: update `xtask/src/proof.rs` **and** anything citing the old name in the same change (`proof.rs:210-215`). Within this story that also means the AC row and the ledger's `verifying_test` field, which must not drift from the code. |
| EC-007 | The snapshot fails to match under a channel other than the pin (a nightly experiment, a contributor's default toolchain) | Not a defect in the case. rustc diagnostics are version-sensitive; the snapshot is authored under `rust-toolchain.toml`'s `1.97.1` and that pin is what the gate and the CI `msrv` job both run today. Regenerate only when the pin itself moves, and review the diff. |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| NF-001 | The new row must reuse `proof.rs`'s shared `cargo_args` path rather than a bespoke invocation | `--all-features` is matched to the gate's preceding test step on purpose, so this step reuses fingerprints instead of rebuilding every test target from scratch — *"the difference between a few seconds and none — per entry"* (`proof.rs:151-163`). A hand-rolled `Command` for this artefact would pay the whole rebuild alone. |
| NF-002 | Nothing published grows an edge, a feature or a floor | `trybuild` is a dev-dependency of a `publish = false` crate (`examples/course-subscriptions/Cargo.toml:8`), declared once workspace-wide (`Cargo.toml:15-18`). AC-006 is the checked form of this. |
| NF-003 | The MSRV does not move | The floor is `1.97.1` and equals the pin (ADR-0029, `CLAUDE.md` *Binding constraints* 5). `cargo hack --no-dev-deps --rust-version` cannot see a dev-dependency at all, so the check that would actually notice is CI's full `cargo test --workspace --all-features` at 1.97.1 — which is what running the local gate under the pin already reproduces. If `trybuild` or its graph will not build at 1.97.1, that is EC-002's escalation shape, not a floor raise. |
| NF-004 | No public surface growth | No `pub` item, no re-export, no feature, no `_design.md` `## Items` row. The deliverable is a test target and a gate row, and neither is a name a caller writes. |
| NF-005 | Determinism | The pair must give the same answer on a clean checkout as on a warm one. `trybuild` writes under `target/tests`; no fixture may depend on ambient state, a network fetch, a wall clock or an environment variable other than `TRYBUILD`. |
| NF-006 | Gate legibility | A failure of this artefact must say which of the three things went wrong — target missing, name missing, case behaved wrongly — without a reader opening `proof.rs`. The first two are `proof.rs`'s message already; the third is trybuild's own output, which the step does not swallow. |

## Implementation notes (non-prescriptive)

Direction, not instruction. Every one of these is a place a reasonable implementer
could lose an hour; none of them is a design decision this spec is making for them.

- **The harness shape the gate requires.** Two `#[test]` functions inside `mod ui`,
  each building its own `trybuild::TestCases`, because the gate names tests and a
  single test running both cases could only be named once — and the case it did not
  name could then be deleted from the glob in silence. `trybuild` runs on drop, so
  each function's assertions happen at its end.
- **Two `TestCases` in one target run concurrently**, and `proof.rs`'s `cargo_args`
  is a fixed seven-element array with no room to pass `-- --test-threads=1`
  (`proof.rs:164-174`). If concurrency proves flaky, the serialization has to live
  **inside** the test target — a `static` mutex the two tests take — not in the gate's
  invocation. Prefer discovering this by running it twice than by assuming either way.
- **The fixture necessarily restates the enum**, because you cannot add a variant to
  an imported one, and the example is a binary crate with no lib target to import
  from in any case. That is the mechanical reason the fixture lives beside the
  example, and it is worth a comment in the fixture naming the `main.rs` lines it
  mirrors — the mirror is the thing most likely to drift.
- **Keep the fixture's own failure surface at one.** An unused import, a missing
  `fn main`, a `dead_code` warning — any of them can print before the E0004 block and
  break AC-002's *first* clause. `trybuild` compares the whole stderr, so a stray
  warning is also a snapshot churn source. Compile the fixture on its own first,
  read what rustc says, and only then snapshot it.
- **Generate the snapshot, then read it.** `TRYBUILD=overwrite` writes the `.stderr`;
  the review step that matters is opening the written file and checking the span
  before committing it. That order — generate, then read — is what AC-007 is asking
  for, and it is the opposite of the habit that makes snapshot tests worthless.
- **Write the `ARTEFACTS` row the way the existing rows are written.** A named `const`
  beside `META_TESTS`, `WIRE_NEGATIVE_CONTROLS` and `SYNC_WIRE_TESTS`, carrying a
  doc comment that argues why *these* names earn a row — `WIRE_NEGATIVE_CONTROLS`
  (`proof.rs:104-118`) is the closest precedent in both shape and reasoning, because
  its subject is also an instrument nothing else references.
- **Run `cargo xtask proof-artefact` explicitly.** The story-grain gate
  (`cargo xtask affected`) runs fmt, clippy, the affected packages' tests, five
  file-reading lints and `spec-trace` — it does **not** run the proof-artefact step
  (`xtask/src/affected.rs:119-125`). A green `affected` is therefore silent about the
  entire deliverable of this story.
- **Record the design-route deviation** in `implementation-report.md` and raise it at
  the slice review. `_design.md:80` says `crates/happenstance/tests/ui/`; the Context
  pack binds the rule instead. Do not edit `_design.md` and do not soften
  anti-pattern 13 to make the manifest line literally true.

## Tests and CI (merge gate)

Grounded in the project testing brief's AC-002 row, which classifies this as
*Integration (compile-fail)*, registered in `proof.rs` and run by the gate's
proof-artefact step *"rather than `cargo test` — the same reason CLAUDE.md gives"*
(`_decomposition.md:780`).

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo fmt --check`; `cargo clippy --all-targets -D warnings` (both inside `cargo xtask affected --base main`) | The fixture and the control are house-style clean at the 100-column budget, `xtask` still compiles with the new `const` and row, and the fixture carries no lint that would print before the diagnostic. Serves **AC-007**, and protects **AC-002**'s *first-block* clause. |
| Integration (compile-fail) | `examples/course-subscriptions/tests/ui.rs` → `ui::an_unhandled_variant_fails_to_compile` over `tests/ui/*.rs` + its `.stderr` | The added variant stops the build with the expected diagnostic, at the expected span. Serves **AC-001** and **AC-002**. |
| Integration (negative control) | `examples/course-subscriptions/tests/ui.rs` → `ui::the_negative_control_compiles` | The red build was red for the stated reason and not for an unrelated one. Serves **AC-003**. |
| Gate (the mount) | `cargo xtask proof-artefact` | Asserts both fully-qualified names out of `cargo test -- --list` **before** running them, then runs them — so deletion, truncation, `#[ignore]` and rename each fail, which `cargo test` alone would not catch for three of the four. Serves **AC-005**, and is the only command that runs **AC-001**/**AC-003** the way the gate does. |
| Gate (project bar) | `cargo xtask ci --fast` | The REQUIRED set, which includes *"each phase's proof artefacts"* (`xtask/src/main.rs:178-190`) and the packaging step (`:519-529`). This is the project's declared integration bar (`.redkiln/config.yaml`, `integration_scoped`). Serves **AC-005**, **AC-006**. |
| Dependency | `cargo deny check licenses`; `cargo deny check advisories` | The new graph clears the **unmodified** allowlist and carries no advisory. Both tools resolve on this machine, so these run rather than skip (`CLAUDE.md`, *Commands*). Serves **AC-006**. |
| Recorded (not pass/fail) | The AC-004 mutation transcript: add `_ =>` to the fail fixture → `cargo xtask proof-artefact` → observe red → revert | That the instrument can fail. There is no automated tier for this and inventing one would mean committing a permanently-broken fixture; the artefact is the recorded observation, cited in `_ledger.md`. Serves **AC-004**. |
| Story merge gate | `cargo xtask affected --base main` (`.redkiln/config.yaml`, `affected_gate`) | The repository's story-grain bar. **Note the gap**: it does not invoke the proof-artefact step (`xtask/src/affected.rs:119-125`), so green here says nothing about this story's deliverable. `cargo xtask proof-artefact` must be run and cited separately. |
| Ledger | `redkiln verify --grain story` | Every AC-### in this spec has a row in `_ledger.md`, satisfied with real cited evidence, and the PR's changed files stay inside the declared boundary (`require_ledger: true`, `require_commit_provenance: true`). |

## Risks and coupling (PR-scoped)

| risk / coupling | why it matters here | handling inside this PR |
| --- | --- | --- |
| **`trybuild` is not adopted** (HS-P0010's decision, still open) | AC-002 has no instrument, and the substitute is measurably weaker in exactly the dimension the criterion is about | EC-001: halt and escalate. Confirm the dependency is present as the *first* implementation step, before writing a fixture that cannot run. |
| **The fixture is a copy of the example's domain, and nothing forces it to stay one** | Six months out, `main.rs` grows a variant and the fixture does not; the case still passes and now pins a shape the example no longer has | Keep the fixture minimal and comment it as a mirror with a `file:line` pointer. The *guarantee* pinned — exhaustiveness of a fold over `Self::Event` — stays true under drift, so this is a review-grade fidelity risk rather than a correctness one; name it in the implementation report so the slice review can see it. |
| **Two `TestCases` running in parallel in one target** | A flaky proof artefact is worse than none: it trains the reader to re-run rather than to read | Serialize inside the target if needed (see implementation notes); the gate's invocation is fixed and cannot carry `--test-threads`. |
| **Snapshot brittleness across toolchains** | A diagnostic snapshot is a hostage to rustc's wording | Authored under the `rust-toolchain.toml` pin, which the gate and the CI `msrv` job both run today, so the two cannot disagree (`CLAUDE.md`, *Commands*). A future pin bump regenerates it as a reviewed diff — expected, not a defect (EC-007). |
| **The `_design.md` route conflict** (`:80` vs `:212-214` and anti-pattern 13) | Two binding statements that cannot both be literally true; picking silently would look like drift from a signed-off design | Bound in the Context pack toward the rule and the selector, reported in the implementation report, raised at the slice review. `_design.md` is not edited by this story. |
| **`trybuild`'s dependency graph** (a proc-macro-free but non-trivial dev graph) | A licence outside the allowlist or a `RUSTSEC` advisory turns a test-only dependency into a gate failure | EC-002: escalate rather than widen. `Cargo.toml:87-93`'s `postcard` note is the precedent for measuring the real graph before assuming defaults are harmless. |
| **The story gate is silent on the story's own deliverable** | `cargo xtask affected` green is the habitual "done" signal here, and it never touches `proof-artefact` | Stated in the Tests table and in the merge DoD; the ledger's evidence for AC-005 must cite a `cargo xtask proof-artefact` run, not an `affected` run. |
| **Slice-mate ordering** | This story points its diagnostic at HS-S0029's fold and cannot precede it (`_storymap.md:127-129`) | Implemented second in the same context. If the fold turns out to be un-protectable (EC-005), that is a finding against the mate, resolved in the slice, not absorbed here. |
| **`spec/SPECIFICATION.md` PS-36 looks like it wants an edit** | Landing the snapshot is precisely the fact PS-36's disposition row says is missing (`:8772`), so the temptation to update it is strong and immediate | Out of scope by the PR boundary. The clause ledger is HS-P0016's; `cargo xtask spec-trace` must stay green **without** touching it. |

## Dependencies

**Blocks on** — must be merged before this story starts:

- **`worked-example-on-typed-layer`** (HS-S0029, the slice-mate). It lands the
  `DomainEvent` enum and the `DecisionModel::apply` fold this story's fixture mirrors
  and whose `match` arm the diagnostic must name. Same slice, same context, this story
  second (`_storymap.md:61`, `:127-129`).

**Unlocks** — waiting on this story:

- **`publish-0-2-0-alpha-1`** (HS-S0033, M7). The alpha names this story explicitly
  among its dependencies (`_storymap.md:64`), and initiative DoD 2 must be observed
  green before the release is cut — the story frontmatter's `blocks` field records the
  same edge.

**External input, not a story edge:** `trybuild`'s adoption as a workspace dependency
is **HS-P0010** (`projection-store-freeze`)'s decision, carried as a risk in
`project.md:266` and as an open input in `_decomposition.md:828-841`. It is a
precondition, and its absence is EC-001's halt rather than a dependency this project
can satisfy for itself.

## Anchors (progressive disclosure)

Deferred depth, not optional depth. Each row says why the artefact is load-bearing and
the moment to open it; none of them is pasted above, and none needs reading before the
Context pack.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/proof.rs` | The mount point. Carries the `Artefact` shape, the inner-`mod` convention that makes fully-qualified names match, the subset-check argument, and the failure message a rename produces. The row cannot be written correctly without it. | Before writing the `ARTEFACTS` row — first thing after the fixtures compile. | AC-005 |
| `xtask/src/main.rs` | The REQUIRED step *"each phase's proof artefacts"* (`:178-190`) and its own comment arguing why naming a target is insufficient; also the proof that **no new gate step is needed** and that the help text's count is derived (`:753-761`). | Before deciding whether the gate needs an edit — it does not, and this is where that is confirmed. | AC-005 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | Binding. The surface manifest (`:78-83`), the pattern and its refusal of the doctest route (`:210-225`), anti-pattern 13 (`:999-1001`), the transience categories (`:810-816`) and the density budgets (`:845-864`). Also the route conflict this spec resolves. | Before writing the fixture (span placement) and again before the slice review (composition invariants). | AC-002, AC-007 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | AC-U07 in full (`:132-139`), the testing brief's AC-002 row (`:780`) and *`trybuild` has no home yet* (`:828-841`) — the three places the instrument's requirements and its blocked status are stated at brief grain. | At story start, to confirm the blocked input, and again when writing the fixture's span expectation. | AC-002, AC-006 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-1 is this story's rule in the constitution: pair every compile-fail with a compiling one, and do not trust the error code — with the measured evidence that rustdoc 1.97.1 reports a mismatched annotation as passing. | Before writing the negative control; it is the reason the control exists rather than a nicety. | AC-003 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision atom behind AC-004: an instrument that cannot fail is decorative, and the answer is a mutation that is run rather than argued. | Before performing the recorded mutation, so the ledger evidence is framed the way the decision asks for. | AC-004 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The in-tree precedent for a proof artefact held to its own names, and the target the first `ARTEFACTS` row points at. Read it for the shape of a test target that exists to fail. | When writing the `ARTEFACTS` row and its `const`. | AC-005 |
| `RUNBOOK.md` | `:3614-3627` is the recorded phase-4 failure this story exists not to repeat — a `compile_fail` instrument inside `tests/` that never ran; `:157` is phase 7's row and its proof artefact. | Before AC-004, and any time a shortcut starts to look reasonable. | AC-004 |
| `spec/SPECIFICATION.md` | PS-36's disposition row (`:8772`) states that pinning a compile-fail diagnostic *"needs a `trybuild`-style stderr snapshot, which is a dependency decision phase 6 owns"* — the clause this story stands beside and must not edit. | Before touching anything spec-shaped, and to confirm the boundary if the temptation to update PS-36 arises. | AC-001 |
| `crates/happenstance-core/src/event.rs` | `:95-106` carries the same rustdoc warning in the tree's own doc comment, at the same construct. It is the shortest proof that the doctest substitute is refused on measurement rather than on taste. | If EC-001 fires and a substitute starts to look acceptable. | AC-001 |
| `Cargo.toml` | `:15-18` states the one-version-workspace-wide rule; `:80-98` is the `# --- dev / tooling only ---` block `trybuild` joins, and `postcard`'s comment there is the worked precedent for measuring a dependency's real feature graph. | Before adding the dependency. | AC-006 |
| `deny.toml` | The licence allowlist (`:8-19`) and `multiple-versions = "warn"` (`:23`) that the new lockfile delta must clear **unmodified**. | Before running `cargo deny`, and immediately if it fails. | AC-006 |
| `rust-toolchain.toml` | The `1.97.1` channel the snapshot is authored under, and the reason a snapshot generated anywhere else is not the artefact. | Before generating the `.stderr`. | AC-007 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/worked-example-on-typed-layer/spec.md` | The slice-mate's spec: the domain enum and the fold this story's fixture mirrors, and the shape the diagnostic must land on. | Before writing the fixture — it is the source the mirror is taken from. | AC-001 |
| `standards/rust/50-dependency-hygiene.md` | The bar a new dependency clears in this workspace, and the vocabulary the implementation report should use when justifying `trybuild`. | While adding the dependency, and when writing the report's justification. | AC-006 |

## Clarifications resolved during spec

1. **A seventh acceptance criterion was added.** The first pass enumerated AC-001
   through AC-006. The interaction-quality obligation (RFC §6.7/D6) requires every
   applicable composition invariant to be a gated AC row rather than prose, and three
   of them — *presentation exists at all*, *transience*, and the *density budget* —
   had no home among the six, which are all about behaviour and adoption. **AC-007**
   carries them, plus the snapshot's provenance rule (generated, never hand-edited),
   which is the invariant that keeps the whole instrument honest. The ledger carries
   seven rows to match.
2. **The `_design.md` route conflict is bound toward the rule, not the manifest.**
   `:80` routes the surface to `crates/happenstance/tests/ui/`; `:81`'s selector and
   anti-pattern 13 require the span to land in `examples/course-subscriptions/` and
   forbid *"anything under `crates/`"*. A `trybuild` fixture's span is its own path, so
   both cannot hold. The harness lands at `examples/course-subscriptions/tests/ui.rs`.
   Reported at the slice review; `_design.md` is not edited by this story.
3. **The two test names are fixed by this spec.** `ui::an_unhandled_variant_fails_to_compile`
   and `ui::the_negative_control_compiles`, inside `mod ui`. They are spelled in three
   places by construction — the harness, `xtask/src/proof.rs` and this ledger — and a
   name left to the implementer would be a name chosen three times.
4. **AC-004's mutation is a recorded observation, not an automated tier.** Automating
   it would mean committing a fixture that is permanently wrong, or a harness that
   edits its own inputs. The artefact is the transcript, cited on the ledger row, which
   is the same shape the runbook's phase-4 finding was closed with.
5. **The story gate does not cover the story.** `cargo xtask affected` runs no
   proof-artefact step (`xtask/src/affected.rs:119-125`, verified). This is stated in
   the Tests table, the risk table and the merge DoD so it cannot be discovered at
   review time; the AC-005 evidence must cite a `cargo xtask proof-artefact` run.
6. **PS-36 is left alone deliberately.** Landing the snapshot is the fact its
   disposition row says is missing, and updating it here would be a clause-ledger edit
   made as a side effect. It is HS-P0016's, and `cargo xtask spec-trace` stays green
   without it.
7. **No ADR is written by this story.** The instrument choice is already recorded in
   the design and the briefs; a decision atom, if one is owed for `trybuild`'s
   adoption, belongs to HS-P0010's decision and to the runbook's ADR pass, never to a
   story as a side effect.
