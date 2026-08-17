---
item: HS-S0186
stage: spec
created: 2026-08-17T13:16:33.859Z
updated: 2026-08-17T13:16:33.859Z
template_sig: 87bbf1d0
rendered_sig: 80b076aa
---

# Spec — Removing the boundary makes the repository fail

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — DoD-4 at `:425-428`, BR-03 |
| Decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project | `.bklg/docs-that-teach/application-author-path/project.md` — AC-005, DoD item 3 |
| This spec | `.bklg/docs-that-teach/application-author-path/boundary-falsification-drill/spec.md` |
| Signed-off design (BINDING) | `.bklg/docs-that-teach/application-author-path/_design.md` — `## Composition` (drill placement), `## States`, `## Transience policy`, `## Anti-patterns`, `## Placement and re-export` |
| Key briefs | `.bklg/docs-that-teach/application-author-path/_decomposition.md` — UX brief (UI-2, IQ-5, UX-002) and Testing brief (tiers 3 and 4, the `no_run` note, the AC-005 coupling note) |
| Grounding | `.bklg/docs-that-teach/application-author-path/_grounding.md` — ES-25, CF-7, the merge-forward precondition |
| Upstream substrate (consumed, not built) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `const TREE = "docs"`, `const HARNESS = "xtask/src/narrative.rs"`, `IGNORE_ALLOWANCES`, `HIDDEN_MARKERS` |
| Roadmap pointer | `.bklg/docs-that-teach/application-author-path/_storymap.md` — `## Merge order`, slice 2 (`opening-encounter`), and the routing paragraph at `:158-163` |

## One-line PR slice

Make the boundary load-bearing to the repository — an executed check the `"tests"` REQUIRED step already sweeps (`xtask/src/main.rs:143-155`) that fails when the query the `AppendCondition` is built from is removed — and give the reader the exact edit, the exact failure and the exact revert on the page, then run and record the drill in both directions.

## Executive summary

The slice-mate (`boundary-refusal-encounter`) lands a program whose own output carries
`AppendError::ConditionViolated`. That discharges "the reader sees a refusal". It does **not**
discharge "the refusal is real", and the difference is the whole reason this initiative exists:
a demonstration nothing runs is a claim, and the project's own risk table ranks *"the opening
encounter compiles forever and quietly stops demonstrating its own claim"* first, with AC-005
named as the only instrument against it (`project.md`, Risks table, row 1).

**The delta this PR lands** is three things and no more:

1. The encounter's refusal assertion is *executed* by a check the gate already runs — nothing
   marked `no_run`, nothing marked `ignore`, no entry on HS-P0020's allowance list — so removing
   the boundary turns a green tree red without anybody adding a gate step.
2. A `###` falsification drill at the bottom of step 3 carrying the **exact** edit, the **exact**
   failure text and the **exact** revert, written from a run rather than from imagination.
3. The drill run in both directions and recorded, because DoD-4 is a *human-observed* obligation
   ("run and observed", `initiative.md:406-410`) that no per-commit check can retire.

It adds no gate step, no test harness, no affordance and no public API. What it adds is the
property that the page cannot silently stop meaning what it says.

## Context pack

Everything in this section is a decision already taken, restated here so the implementer needs
nothing else to start. The deeper artifacts are behind the anchors table.

**The persona slice.** Persona 1's stated fear is silent wrongness — *"a mental model that looks
right, compiles, runs, and is quietly wrong"*. UI-2 is the intent this story realizes: *"Tell me
it is real and not a story about itself."* The UX brief is explicit that the only affordance
answering that fear directly is one the reader can operate — break it, watch something fail, put
it back (IQ-5, UX-002). So the drill is reader-facing content *and* a repository property, and
neither half alone closes AC-005.

**What "a check the repository runs" concretely is here.** HS-P0020's signed-off design pins
`const TREE: &str = "docs"` and `const HARNESS: &str = "xtask/src/narrative.rs"` — the lib-crate
harness whose `include_str!` lines register every narrative page as doctests of `xtask`. Rustdoc
doctests *execute* by default, and `xtask/Cargo.toml` already carries `happenstance` and `tokio`
(`macros`, `rt`, `rt-multi-thread`) under `[dev-dependencies]` for exactly this class of use, so
the step-3 program compiles **and runs** as `cargo test -p xtask --doc`. That is swept by the
`"tests"` REQUIRED step, which is `cargo test --locked --workspace --all-features -- --show-output`
(`xtask/src/main.rs:143-155`). **This story wires no new step.** If it finds itself adding one, it
has misread the substrate.

**The twin, and why AC-005 does not wait on substrate.** `_design.md`'s `## Placement and
re-export` records, with the reason, that step 3's program exists **twice** — once in
`crates/happenstance/src/lib.rs` (the docs.rs reader's first encounter) and once as step 3 in the
pinned tree — because `include_str!` cannot reach outside a published package. Both copies are
compiled *and executed*, and both assert the same `ConditionViolated`. This story exploits that
deliberately: the drill's reader-facing instructions target the page, and the story additionally
verifies the crate-root twin fails under the same removal. That twin is executed by
`cargo test -p happenstance --doc` today, so **AC-005 has a proof that does not depend on
HS-P0020's harness having landed**. The storymap is categorical that AC-005 "routes nowhere under
any circumstance" (`_storymap.md:163`); this is what makes that statement affordable.

**The edit must be compile-valid and boundary-specific — and one obvious spelling is not.**
`Query::from_items([])` returns `Err(InvalidQuery::NoItems)`
(`crates/happenstance-core/src/query.rs:180-190`), so "empty the query" propagates a *constructor*
error through `?`. The check goes red, and it goes red for the wrong reason: the reader learns the
query builder rejects empties, not that the boundary was holding anything. Likewise, deleting the
`Query` binding outright is a build error, and the design's `## States` row *Error — the drilled
one* requires the reader recognise **the** failure rather than a build error. **The canonical edit
is therefore to remove the condition from the append** — pass `None` where the program passes
`Some(&AppendCondition::new(seats).after_opt(upto))`. The program still compiles, still runs, the
racing append is now accepted, and the *refusal assertion* — the one line ES-25 is cited for — is
what fails. That is the boundary being removed and noticed, with nothing incidental in the way.

**Failure text is captured, never composed.** Under `include_str!`, a failing fence reports
against the *including* item's file and line, not the markdown file a reader opened — the lesson
`xtask/src/constitution.rs:11-18` already paid for ("a line number counted from the first, which
maps to no file a reader can open"). So the failure quoted on the page may name the harness rather
than the page, and the page must quote **what the reader actually sees**. If that output is too
illegible for a reader to recognise as *the* failure, that is a substrate finding routed to
HS-P0020 (project DoD item 9) — it is never a reason to soften the drill or paraphrase the output.

**Reversibility is a state, not a footnote.** `_design.md`'s `## States` carries *Boundary
restored* as a first-class state and the UX brief's state table requires that after the revert
"the check passes again, and nothing else in their tree needs repairing". The drill therefore ends
with a passing command **and** a clean working tree, and both are observed.

**Composition, binding, not re-decidable here.** The drill is a `###` under step 3, after the
clause citation, at the bottom of the step — "only performable by someone who has just run step 3"
(`_design.md`, `## Composition`). It is **not** a page of its own: a new page would need its own
answered-need in HS-P0021's notation (AC-012, IQ-8), and the design already declined an extra page
for exactly that reason. It is `Persistent` in the transience policy — never inside a fold. Under
`docs/`, `<details` and `<summary` are in HS-P0020's `HIDDEN_MARKERS` with **deliberately no
allowance list**, so a fold there is a gate failure, not a style disagreement.

**Zero opted-out fences.** DT-6 resolved so this project puts **nothing** on
`IGNORE_ALLOWANCES` (`_design.md`, `## Pattern decision`, DT-6). The testing brief names the
concrete failure mode to watch for: a fence marked `no_run` passes the compiled-fence tier while
never reaching the executed tier — "the boundary would type-check forever without ever being asked
to refuse anything". Any `no_run`, `ignore` or `compile_fail` on the refusal fence is a defect this
story exists to prevent, not a formatting choice.

**Vocabulary and prior-model discipline.** Fences import `use happenstance::{…}`, never
`happenstance_core` (ADR-0006, `.kb/decisions/0006-bare-name-to-the-typed-layer.md`; UX-011). The
words "aggregate", "one stream per entity" and "which stream" do not appear on any step of the
opening encounter — the prior model is named on exactly one page in the whole set, the bridge's
*"Where your streams went"* (`_design.md`, `## Anti-patterns` 6). The drill adds no second
answered-need line; step 3's page already carries the one.

**Density.** A step-page fence is ≤ 24 rendered lines and ≤ 68 columns (`_design.md`,
`## Density budget`, re-derived from the mock at 16px/24px code type). The drill's edit is stated
as a diff-shaped instruction plus a quoted output block, not by re-printing the whole program.

**Preconditions inherited from the slice.** No page is authored before the merge forward from
`initiative/from-contract-to-published-library` is complete and recorded (AC-014, DR-13,
`merge-forward-preflight`), and `_design.md` is signed off (AC-001, `tension-resolutions`) — it is,
dated 2026-08-17, with two conditions recorded. The `tokio` dev-dependency on `happenstance` that
makes the crate-root twin executable is `_design.md`'s `## Items` row 3 and is landed by the
slice-mate; this story consumes it and must not add it twice.

## Integration contract

- **Archetype**: `capability` — a reader-observable slice (page content) that is simultaneously a
  repository property (an executed check that can fail).
- **Slice / milestone**: `opening-encounter`. Slice-mate: `boundary-refusal-encounter`
  (`depends_on`). Both are implemented in one context and mounted together — the storymap is
  explicit that the drill "is not a follow-up PR that may slip, because the encounter without it is
  exactly the failure this initiative exists to prevent" (`_storymap.md:141-144`).
- **Mount point**: `xtask/src/narrative.rs` — HS-P0020's `HARNESS`, the lib-crate registration file
  whose `include_str!` line makes the step-3 page a doctest that the `"tests"` REQUIRED step
  executes. The drill section is mounted by living inside a page registered there; the page file is
  the one the slice-mate creates under `docs/` for `_design.md`'s route `{tree}/first-encounter/`
  (`{tree}` = `docs`, HS-P0020 D1). **Second, already-real mount**:
  `crates/happenstance/src/lib.rs`, whose twin program is executed today by
  `cargo test -p happenstance --doc`. A drill that is reachable only through a test, or a page
  fence that is registered nowhere, does not satisfy this contract.
- **Wires into**:
  - `xtask/src/main.rs:143-155` — the `"tests"` REQUIRED step (`probe: None`) that sweeps both
    doctest targets. Consumed, not modified.
  - `crates/happenstance-core/src/append.rs` (`AppendCondition::new`, `Guard`, `after_opt`) and
    `crates/happenstance-core/src/store.rs:321` (`read_decision_model`) — re-exported through
    `happenstance` (`crates/happenstance/src/lib.rs:75`, `pub use happenstance_core::*`; the line
    moves with the AC-014 merge forward, the re-export does not). The
    verified call spelling is fixed by `_design.md`'s `## Signatures` (finding F-5).
  - `crates/happenstance-core/src/query.rs:180-190` — `Query::from_items`'s `InvalidQuery::NoItems`,
    which is why the empty-query spelling is forbidden as the drill's edit.
  - `examples/course-subscriptions/src/main.rs:200-224` — the `commit` helper's
    `AppendCondition::new(query.clone()).after_opt(last_seen)` shape, the in-repo reference the
    contingency `#[tokio::test]` follows if one is needed.
  - HS-P0020's `IGNORE_ALLOWANCES` and `HIDDEN_MARKERS` — consumed as constraints; this story adds
    zero entries to the first and trips none of the second.
- **Renders surfaces**: `opening-encounter` (`_design.md`, `## Surfaces`), state
  **`falsification-drill`** — a state the design already declares, drawn in the mock at
  `.bklg/docs-that-teach/application-author-path/design/mock.html`. No other surface's composition
  changes; `crate-root-encounter` is touched only insofar as the drill is verified against its twin
  fence, whose authorship belongs to the slice-mate.
- **Public items**: none. `_design.md`'s `## Items` adds no public Rust item; this story
  implements no row of it and changes no signature. The `tokio` dev-dependency row is the
  slice-mate's.
- **Conformance rule(s)**: **none, and deliberately.** This story is not adapter-observable: it
  changes no port, no value type and no store behaviour, so a rule in `happenstance-testkit`'s
  `suite.rs` would have nothing to observe and no wrong adapter to reject — a decorative rule by
  CLAUDE.md's own test. The check that observes this story's behaviour is a doctest under the
  `"tests"` step, and the testing brief forbids reaching for `event_store_conformance!` or
  `MemoryFixture` here ("sweeping the full conformance suite over one demonstration scenario would
  prove the wrong thing").
- **Clause(s)**: **ES-25** (`spec/SPECIFICATION.md:3693`, `[FROZEN]`) — cited, never restated; it
  is the clause the refusal assertion makes concrete. **CF-7** (`:7266`, `[FROZEN]`) is available
  as motivation for why a boundary must be tag-based, and is the bridge story's to spend. This
  story **discharges and amends nothing**: the initiative is additive (`project.md`, Out of scope).
- **Advances DoD scenario**: initiative **DoD-4** — *"@smoke — the boundary claim is checked, not
  narrated"* (`initiative.md:425-428`), and project **DoD item 3** ("run and observed in both
  directions"). It also hardens DoD-3, which the slice-mate lands, by making its claim falsifiable.

## PR boundary

```
docs/**
xtask/src/narrative.rs
crates/happenstance/src/lib.rs
crates/happenstance/tests/**
crates/happenstance/Cargo.toml
.bklg/docs-that-teach/application-author-path/boundary-falsification-drill/**
```

**In this PR**

- The `### ` falsification drill under step 3 of the opening-encounter page: the exact edit, the
  exact expected failure quoted from a real run, the exact revert, and the passing command after it.
- Whatever is required for the refusal assertion to be **executed** rather than only compiled at
  both mounts — removing a `no_run`/`ignore` if one appeared, and confirming the page's registration
  line exists in `xtask/src/narrative.rs`.
- Verification that the same removal fails the crate-root twin, so the proof survives a slipped
  harness.
- The recorded two-direction observation as a companion in this story's own folder.
- The contingency `#[tokio::test]` **only if** the executed path turns out not to execute — with
  the substrate finding routed to HS-P0020 in the same change.

**Explicitly not in this PR**

- Authoring the opening encounter's three steps or the crate-root `## Watch a boundary refuse`
  fence, and adding the `tokio` dev-dependency — all `boundary-refusal-encounter`'s
  (`_design.md`, `## Items`; storymap slice 2).
- The pinned tree, the compiling step, the allowance list, the hidden-marker check and the
  deliberately-broken-page falsification of the *mechanism* — HS-P0020. That project proves the
  mechanism can fail; this story proves the *claim* can (`project.md`, Risks table, row 1).
- The bridge's wrong-side contrast (DT-6) — `invariant-to-appendcondition-bridge`.
- The project-wide fence inventory and the clause audit — `fence-inventory-and-clause-audit`.
- Any new gate step, any change to `REQUIRED`, any change to `xtask/src/affected.rs`'s selection
  arm (HS-P0020 owns the `docs/` inertia change), and any amendment to `spec/SPECIFICATION.md`.

The implementer **may** touch the wiring files named in the Integration contract to mount this
slice — the harness registration line and the crate-root twin — and that is not scope drift.

**Merge DoD (one line):** the drill exists on step 3 with output quoted from a real run, the
removal makes `cargo test -p xtask --doc` *and* `cargo test -p happenstance --doc` fail on the
refusal assertion, the stated revert restores both to green with a clean tree, the two-direction
observation is recorded, and `cargo xtask affected --base main` is green at the checkpoint.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The drill is where the design put it | A `###` under step 3, after the clause-citation sentence, at the bottom of the step. Not a page of its own, not above the fence, not inside a fold. | `_design.md`, `## Composition` (`opening-encounter`, "Step 3 additionally carries…"); `## Transience policy` row "The falsification drill" |
| The edit is stated exactly, and it compiles | Remove the condition from the guarded append — pass `None` where the program passes `Some(&AppendCondition::new(seats).after_opt(upto))`. The program still builds and runs; only the refusal assertion fails. | `_design.md`, `## Signatures` (the binding shape); `examples/course-subscriptions/src/main.rs:200-224` for the same call shape |
| The empty-query spelling is forbidden as the drill's edit | `Query::from_items([])` → `Err(InvalidQuery::NoItems)`, propagated by `?`. Red for a constructor reason, not a boundary reason; the reader cannot tell it from a build error. The page must not use or suggest it. | `crates/happenstance-core/src/query.rs:180-190`; `_design.md`, `## States`, *Error — the drilled one* |
| The check is executed, not merely compiled | No `no_run`, no `ignore`, no `compile_fail` on the refusal fence at either mount; zero entries added to `IGNORE_ALLOWANCES`. | Testing brief, Notes ("`no_run` is the concrete failure mode to check for"); `_design.md`, DT-6 ("zero uncompiled fences") |
| The failing check is one the gate already sweeps | `cargo test -p xtask --doc` (page, via `HARNESS`) and `cargo test -p happenstance --doc` (twin), both inside `cargo test --locked --workspace --all-features` — the `"tests"` REQUIRED step. No new step. | `xtask/src/main.rs:143-155`; HS-P0020 `_design.md`, `## Signatures` (`TREE`, `HARNESS`); `xtask/Cargo.toml` `[dev-dependencies]` |
| The quoted failure is captured, not composed | The page reproduces the runner's actual output verbatim, including the file and line the reader will see — which under `include_str!` may name the including item rather than the page. | `xtask/src/constitution.rs:11-18`; `_design.md`, `## States`, *Error — the drilled one* |
| The revert is exact and total | One stated revert; afterwards both commands pass and `git status` is clean. Reversibility is observed, not asserted. | `_design.md`, `## States`, *Boundary restored*; UX brief States table, *Restored* |
| The observation is recorded in both directions | A companion in this story's folder: the commands, the tree state, the verbatim failing output, the revert, the verbatim passing output, and the date. This is the only record DoD-4 will ever have. | `initiative.md:425-428`; `project.md`, DoD item 3; testing brief, tier 4 |
| Contingency, if the executed path does not execute | Land a `#[tokio::test]` in `crates/happenstance/tests/` asserting refusal-with-condition and acceptance-without, swept by the same `"tests"` step — never accept compilation as proof — and route the substrate finding to HS-P0020. | Testing brief, Notes ("AC-005's proof is coupled to a mechanism this project does not build"); `project.md`, Risks, coupling note; `_storymap.md:158-163` |
| The store is real; no seam is introduced | The scenario runs against a real in-process `happenstance::MemoryEventStore`. No mock, stub or hand-rolled fake of `EventStore`, and no `happenstance-testkit` fixture. | Testing brief, "Fixtures and seams — deliberately, none to mock" |
| No affordance, no vocabulary drift, no second need | Zero interactive affordances introduced (IQ-6/UX-012); `use happenstance::{…}` only (ADR-0006/UX-011); no prior-model words on the encounter; the step-3 page keeps exactly one answered-need. | `_design.md`, `## Anti-patterns` 6, 8, 12, 13; `.kb/decisions/0006-bare-name-to-the-typed-layer.md` |
| Nothing normative is restated | Every normative claim in the drill's prose is a citation to a clause id that `cargo xtask spec-trace` resolves — ES-25 for the refusal; no `MUST`/`MUST NOT` sentence written in the page's own words. | `spec/SPECIFICATION.md:3693`; `_design.md`, `## Anti-patterns` 11; UX-007 |

## Data and migrations

**N/A.** This story persists nothing and migrates nothing. It adds no schema, no store, no
serialised format and no on-disk state: its artifacts are a markdown section inside an existing
page, a doctest that already runs against an in-memory store constructed and dropped per test, and
a recorded observation in the backlog tree. `Bytes` payloads in the fences are literals inside the
program and never leave it. The only durable record produced is the two-direction drill
transcript, and its "migration" story is that it is written once and superseded only by a later
re-observation at closeout (HS-P0025).

## Acceptance criteria

Seven criteria, each written from Persona 1's own intent — the application author whose stated
fear is *"a mental model that looks right, compiles, runs, and is quietly wrong"*
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:99-106`) and whose
intent here is UI-2, *"Tell me it is real and not a story about itself."* All seven together
discharge project **AC-005** and initiative **DoD-4**; no other project AC is traced by this
story.

Two mounts appear throughout and both are load-bearing: **the page** (step 3 under `docs/`,
registered in `xtask/src/narrative.rs`, executed by `cargo test -p xtask --doc`) and **the twin**
(the crate-root program in `crates/happenstance/src/lib.rs`, executed by
`cargo test -p happenstance --doc`). Both are swept by the one `"tests"` REQUIRED step at
`xtask/src/main.rs:143-155`.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an application author who has just run step 3 and watched `AppendError::ConditionViolated` appear in their own output, and who now wants evidence the boundary is load-bearing rather than decorative (UI-2, UX-002), **WHEN** they read on to the bottom of step 3, **THEN** they meet a `###` falsification drill — after the clause-citation sentence, at the bottom of that step, on the same page rather than a page of its own — carrying three separately labelled, composed parts in this order: the **exact edit**, the **exact failure to expect**, and the **exact revert**, each stated in operable terms rather than described. | Reviewer walk of the rendered step-3 page against `_design.md` `## Composition` (`opening-encounter`, "Step 3 additionally carries…") and `## Hierarchy`, recorded in `.bklg/docs-that-teach/application-author-path/boundary-falsification-drill/_drill-observation.md` (testing brief, tier 5). The transcript's own commands must be the page's, copied, not re-typed from memory. |
| AC-002 | **GIVEN** the same reader will only believe a drill whose named check the repository genuinely runs, **WHEN** the tree is clean and unmodified, **THEN** the refusal assertion is *executed* at both mounts — no `no_run`, `ignore` or `compile_fail` attribute on either fence, and zero entries added to HS-P0020's `IGNORE_ALLOWANCES` — and each doctest target reports the refusal test as run and passing. | `cargo test -p happenstance --doc` and `cargo test -p xtask --doc`, both with `-- --show-output`, showing the refusal test executing (not merely compiling); plus a grep of the two fences' attribute lines and of HS-P0020's allowance list showing this story added nothing. Both commands are inside `cargo test --locked --workspace --all-features` (`xtask/src/main.rs:143-155`). |
| AC-003 | **GIVEN** the reader performs the page's exact edit — passing `None` where the program passes `Some(&AppendCondition::new(seats).after_opt(upto))` — **WHEN** they run the command the drill names, **THEN** the run **fails on the refusal assertion itself** at both mounts, not on a build error and not on a query-constructor error, and the failing command is one `cargo xtask ci` already sweeps: no step is added to `REQUIRED`, no harness is introduced. | The drill executed for real: apply the edit, run `cargo test -p happenstance --doc` and `cargo test -p xtask --doc`, capture both failures verbatim into `_drill-observation.md`. The failure line must name the `matches!(…, Err(AppendError::ConditionViolated(_)))` assertion. A diff of `xtask/src/main.rs` showing the `REQUIRED` array unchanged. |
| AC-004 | **GIVEN** the reader has watched the check fail and now wants their tree back, and reversibility is a first-class state rather than a footnote (`_design.md` `## States`, *Boundary restored*; UX brief States table, *Restored*), **WHEN** they apply the page's exact revert **and nothing else**, **THEN** both commands pass again and `git status --porcelain` is empty — nothing else in their tree needs repairing, and no second instruction is required to get there. | Second half of the same executed drill: revert, re-run both commands, capture both passes verbatim, and record `git status --porcelain` output as empty, all in `_drill-observation.md`. Followed by `cargo xtask affected --base main` green at the story checkpoint. |
| AC-005 | **GIVEN** initiative DoD-4 is a *human-observed* obligation ("run and observed", `.bklg/docs-that-teach/initiative.md:425-428`) that no per-commit check can retire, **WHEN** the implementer performs the drill, **THEN** a dated transcript in this story's own folder records, in order: the starting tree state, the commands, the edit applied, the verbatim failing output at both mounts, the revert, the verbatim passing output at both mounts, and the closing clean-tree check — because this is the only record DoD-4 and project DoD item 3 will ever have. | Existence and completeness of `.bklg/docs-that-teach/application-author-path/boundary-falsification-drill/_drill-observation.md`, walked by the reviewer against project `project.md` DoD item 3 and the testing brief's tier 4. A transcript missing either direction fails this criterion. |
| AC-006 | **GIVEN** Persona 1's fear is silent wrongness, and a drill only answers it if the reader can recognise **the** failure rather than a build error, **WHEN** the reader compares what they see to what the page quoted, **THEN** the quoted failure is verbatim from the recorded run — including whichever file and line the runner names, which under `include_str!` may be the including item rather than the page — never paraphrased, never re-wrapped, never elided with `…`; **AND** neither the drill nor the surrounding prose uses or suggests emptying the query, because `Query::from_items([])` returns `Err(InvalidQuery::NoItems)` and would go red for a constructor reason the reader cannot tell from a build error. | Byte comparison of the page's quoted failure block against the corresponding block in `_drill-observation.md`. Plus a grep of the page for `from_items([])` and for any `…`/`...` inside the quoted output block, both of which must return nothing. Copy fidelity is UX-013's floor (`_decomposition.md`, Accessibility floor). |
| AC-007 | **GIVEN** the drill is content every reader meets rather than an aside for the curious, and the page must not acquire a second subject while acquiring it, **WHEN** the rendered step-3 page is inspected without running anything, **THEN** the drill is **persistent** — not inside `<details>`/`<summary>` or any fold or collapsed region — introduces **no** interactive affordance, CSS or JS; is the step's one declared additional element and adds no eighth competing one; every fence it references imports `use happenstance::{…}` and never `happenstance_core`; the words "aggregate", "one stream per entity" and "which stream" do not appear; the page still carries exactly one answered-need, above the first fence; and no sentence the drill adds states a `MUST`/`MUST NOT` in the page's own words — the only normative reference is the ES-25 citation. | Reviewer walk against `_design.md` `## Transience policy` (row *The falsification drill*), `## Density budget` (per-step budget), and `## Anti-patterns` 1, 3, 6, 8, 11, 12, 13 — each phrased so it is checkable on a rendered page without reading Rust — recorded in `_drill-observation.md`. Mechanically assisted by a grep of the authored page for `<details`, `<summary`, `happenstance_core`, `aggregate`, `MUST`, and by `cargo xtask spec-trace` resolving the ES-25 citation. |

**Coverage of the traced project AC.** AC-005 of `project.md` has three verbs — *removing the
boundary makes a check the repository runs fail*, *observed once as a failure*, *and once as a
recovery after reverting*. AC-002 and AC-003 above carry the first, AC-003 and AC-005 the second,
AC-004 and AC-005 the third. AC-001, AC-006 and AC-007 are what make the reader able to perform
it at all, which is the half UX-002 and IQ-5 add on top of the gate obligation.

## Interaction quality

RFC §6.7/D6. Every invariant below is blocking and every one is carried by an `AC-###` **row in
the table above** — this section is the map from invariant to id, not a second place criteria are
stated. The medium is a rendered documentation page, which does not soften the bar: the design's
own framing is that on a documentation surface "the states *are* the reader's situations"
(`_design.md`, `## States`).

### STATE invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump.** The drill is performable from step 3 without leaving for `spec/SPECIFICATION.md`, the crate source, or another page. Clause citations are provenance (IQ-1). | AC-001, AC-007 | Strike every off-page link from the drill; the edit, the failure and the revert are still fully stated. Reviewer check, tier 5. |
| **Non-occlusion — the disclosure never hides what it discloses** (IQ-2.1/2.2, UX-003, UX-004). No part of the boundary, and no part of the drill's instructions, sits on a hidden `#`-prefixed doctest line or inside a fold. | AC-007 (fold/`<details>`), AC-002 (the assertion is real and visible, not compiled-and-skipped) | Render the page, read only what is visible, and perform the drill from that alone. Grep for `<details`/`<summary` — both are in HS-P0020's `HIDDEN_MARKERS` with no allowance list. |
| **Reversibility, in the reader's hands** (IQ-5, UX-002). Exactly one stated revert returns the tree to green and to clean. | AC-004 | The executed second direction of the drill plus `git status --porcelain` empty, both recorded. |
| **Preserved place and focus.** The drill introduces no anchor renumbering and no re-ordering of step 3's existing elements; every anchor cited into step 3 still resolves (IQ-4). | AC-001 (placement is *appended* at the bottom of the step, after the citation, changing nothing above it) | Reviewer diff of step 3 before and after: nothing above the `###` moves. |
| **Keyboard reachability by construction** (IQ-6, UX-012). The enumeration of interactive affordances this story introduces is empty. | AC-007 | Enumerate them; the correct answer is none. Also satisfied structurally: no CSS, no JS, nothing the base medium does not already render. |
| **The observable is the program's, not the prose's** (IQ-7). The failure the drill promises is a runner's output, quoted; the pass afterwards likewise. | AC-003, AC-006 | Cover every sentence around the quoted block: the recognisable failure is still on screen. |

### COMPOSITION invariants

Taken from the signed-off `_design.md` (approved 2026-08-17), which is binding and is **not**
re-decided here.

| Invariant (source) | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all.** The drill is three labelled, composed parts — edit, failure, revert — not one undifferentiated paragraph and not bare markup. An unstyled render still has to show three distinguishable things in that order. | AC-001 | Reviewer walk against `_design.md` `## Composition`. |
| **Composition and placement.** `###` under step 3, after the clause-citation sentence, at the bottom of the step; never a page of its own (a new page would owe its own answered-need under HS-P0021's notation — AC-012, IQ-8 — and the design declined it). | AC-001 | Same walk; plus AC-007's single-answered-need check, which is what a fourth page would have broken. |
| **Transience — persistent, never revealed or opened-on-demand.** `_design.md` `## Transience policy` classifies the drill **Persistent**, on the ground that "AC-005's proof is a repository check; the reader-facing instructions are the same content and get the same visibility". | AC-007 | Grep for `<details`/`<summary`; visual check that nothing must be clicked before the edit is legible. |
| **Density budget, with its real numbers.** Step pages are ≤ **24 rendered lines** and ≤ **68 columns** per fence, at 16px/24px code type in a 696px fence interior at 1024×768 (`_design.md` `## Density budget`, re-derived at the design gate from finding F-1). The drill states its edit diff-shaped rather than reprinting the ~20-line program, and is step 3's one declared additional element beyond the seven-element per-step budget. | AC-007 | Measure the authored block; anti-pattern 4 ("a code block with a horizontal scrollbar at 1024×768") is checkable on a screenshot. The one deliberate exception is the *captured* failure output — see Clarifications. |
| **Hierarchy.** The drill is recessive relative to step 3's fence and output block, and the only channels available are position, heading level and form (`_design.md` `## Hierarchy`) — so it gets last position and `###`, and it never precedes or displaces the fence. | AC-001 | Reviewer walk; the fence and its output block are still the first things a reader meets in step 3. |
| **Named anti-patterns.** 1 (a block visibly labelled not-checked), 3 (a collapsed disclosure triangle), 6 (prior-model vocabulary on the encounter), 8 (any control rustdoc does not already ship), 11 (a `MUST` sentence that is not a clause link), 12 (a second answered-need, or one below the first fence), 13 (`use happenstance_core::`). | AC-007, with anti-pattern 1 additionally carried by AC-002 | Each is written to be checkable against a rendered page by someone who cannot read Rust (`_design.md` `## Anti-patterns` preamble). Greps assist; the walk decides. |

**What an unstyled render would still pass, and why these rows exist.** A page that mounted the
drill as a bare paragraph, folded inside a `<details>`, with the failure paraphrased, would
satisfy every mechanical check this story otherwise runs: the doctests still execute, the gate is
still green, `spec-trace` still resolves. AC-001, AC-006 and AC-007 are the criteria that make
that page fail.

## Error conditions

| id | Condition | Required response |
| --- | --- | --- |
| EC-001 | **The canonical edit compiles and the check stays green.** Passing `None` in place of the condition leaves both doctests passing — which means the scenario's racing append never actually conflicts and the boundary was never load-bearing. This is exactly the wrong implementation the testing brief's tier 4 exists to reject: "a query that happens to never match anything the scenario appends". | **Fix the scenario, never the criterion.** The refusal assertion is wrong or the racing append is mis-tagged; correct it with the slice-mate and re-run. Do not weaken AC-003, do not substitute a different edit, and do not mark the ledger row satisfied. AC-005 "routes nowhere under any circumstance" (`_storymap.md:163`). |
| EC-002 | **The executed path turns out not to execute.** HS-P0020's mechanism type-checks the tree's fences without running them, or the page's registration in `xtask/src/narrative.rs` lands under a `cfg` that `cargo test --doc` does not reach. Named in advance by `_design.md` `## Gaps in the substrate` 4 and by the testing brief's Notes. | Land the contingency `#[tokio::test]` in `crates/happenstance/tests/`, asserting refusal-with-condition **and** acceptance-without against a real `MemoryEventStore`, swept by the same `"tests"` step — and route the substrate finding to HS-P0020 in the same change (project DoD item 9). **Never accept compilation as proof.** The crate-root twin is unaffected and still carries AC-003. |
| EC-003 | **The removal produces a build error rather than an assertion failure.** The implementer reaches for `Query::from_items([])` (→ `InvalidQuery::NoItems` through `?`, `crates/happenstance-core/src/query.rs:180-190`) or deletes the `Query` binding outright. | Both spellings are forbidden as the drill's edit. Re-pick the canonical edit — remove the condition from the append. The design's `## States` row *Error — the drilled one* requires the reader recognise **the** failure, not a build error. |
| EC-004 | **The captured failure is illegible as *the* failure.** Under `include_str!` a failing fence reports against the including item's file and line — the lesson `xtask/src/constitution.rs:11-18` already paid for — so the output may name the harness rather than the page the reader opened. | Quote it exactly as it is. If it is too illegible for a reader to recognise, that is a **substrate finding routed to HS-P0020** (project DoD item 9), recorded in `_drill-observation.md`. It is never a licence to soften the drill, paraphrase the output, or add an explanatory sentence that does the recognising for the reader. |
| EC-005 | **The revert does not return the tree to clean.** `git status --porcelain` is non-empty after the stated revert — a `Cargo.lock` touch, a stray artifact, or a second file the edit required. | The drill's revert must be one operation that restores everything the edit touched. If it cannot be, the *edit* is wrong: pick one that touches exactly one expression. AC-004 fails until `git status --porcelain` is empty. |
| EC-006 | **The page-side mount does not exist yet.** HS-P0020's pinned tree or the slice-mate's step-3 page is not in the tree when this story is implemented. | Halt loudly and report, rather than marking page-side rows satisfied against nothing. The crate-root twin in `crates/happenstance/src/lib.rs` is executed by `cargo test -p happenstance --doc` today and independently carries AC-002/AC-003/AC-004 — which is precisely why the storymap can say AC-005 never routes away. The page-side half is the slice's to complete before the slice checkpoint. |

## Non-functional

| id | Requirement | Evidence / check |
| --- | --- | --- |
| NF-001 | **No new gate step.** `REQUIRED` in `xtask/src/main.rs` is unchanged by this story; the falsifying check is the existing `"tests"` step (`:143-155`). A story that adds a step has misread the substrate. | `git diff` on `xtask/src/main.rs` shows no change to the `REQUIRED` array. |
| NF-002 | **No measurable gate cost.** The story adds at most the two in-memory scenarios the slice already introduces; step 3's program stays at roughly 20 rendered lines against a `MemoryEventStore` constructed and dropped per test. No new dependency is compiled for it. | The `"tests"` step's wall-clock before and after, noted in `_drill-observation.md`. |
| NF-003 | **Zero public API surface, zero dependency delta.** No `pub` item is added, changed or removed; `_design.md` `## Items` adds no public Rust item and the `tokio` dev-dependency row is the slice-mate's. `cargo package --list`'s licence/README assertions and the MSRV (1.97.1, ADR-0029) are untouched. | `cargo public-api`-free check by inspection plus `cargo xtask ci --fast` green; `crates/happenstance/Cargo.toml` diff carries no line from this story. |
| NF-004 | **Print, no-JS and reduced-motion by construction.** Everything this story authors is static prose and fenced blocks, so it prints, renders with JavaScript off, and animates nothing. This is a consequence of introducing no affordance (UX-012), not a separate design. | `_design.md` `## States` rows *Print / no-JS* and *Reduced motion*; nothing to test because nothing is introduced. |
| NF-005 | **Reproducibility of the quoted failure.** The output the page asks the reader to match must be reproducible on the pinned 1.97.1 toolchain. Volatile fragments — elapsed times, absolute paths, thread ids — are outside what the reader is asked to recognise, and the drill says which line is *the* line. | Re-run the failing direction a second time and confirm the recognisable line is identical; record both runs' relevant lines in `_drill-observation.md`. |
| NF-006 | **Accessibility floor at the content layer** (UX-013). The `###` does not skip a heading level under step 3's `##`; link text in the drill is meaningful standing alone; the quoted block is copy-faithful — no shell prompt characters inside a Rust fence, no line-number gutter, no elided `…`. | Reviewer check on page source, per the UX brief's Accessibility floor; no automated check exists for this and `_design.md` `## Gaps in the substrate` 5 says so. |

## Implementation notes (non-prescriptive)

- **Do the twin first.** `crates/happenstance/src/lib.rs` is executable today by
  `cargo test -p happenstance --doc`, with no dependency on HS-P0020 having landed anything.
  Running the drill against it first proves the *edit* is the right edit before the page exists,
  and it is what makes EC-006 survivable.
- **Capture before you write.** The order that works is: apply the edit → run → paste the output
  into `_drill-observation.md` → revert → run → paste → *then* author the page's quoted block from
  the transcript. Writing the block first and reconciling afterwards is how a composed failure
  gets shipped, and AC-006 is the criterion that catches it late instead of early.
- **State the edit diff-shaped.** "Change `Some(&AppendCondition::new(seats).after_opt(upto))` to
  `None` on the append at line N" costs three lines; reprinting the program costs twenty and blows
  the step's budget. The reader has the program directly above.
- **The exact call spelling comes from the merged tree**, not from this worktree's copy of
  `crates/happenstance/src/lib.rs` (AC-014, DR-13). `_design.md` `## Signatures` fixes the *shape*
  — `read_decision_model(&store, &seats)` then `AppendCondition::new(seats).after_opt(upto)`, the
  spelling corrected at the design gate by finding F-5 — and `crates/happenstance-core/src/store.rs`
  and `.../append.rs` are where the real signatures live.
- **Watch which command the drill names.** Naming `cargo test --locked --workspace --all-features`
  is honest but slow for a reader; naming `cargo test -p happenstance --doc` is fast and is a
  strict subset of what the gate sweeps. Naming the fast one *and* saying it is part of the gate
  step is the shape that serves both AC-003 and the reader's patience.
- **Resist adding a second edit.** One edit, one failure, one revert. A drill offering two ways to
  break it is a drill the reader has to choose inside, and neither option ends up rehearsed.
- **If something is found and not fixed, route it** — substrate to HS-P0020, pointer/reach to
  HS-P0023, comprehension doubts to HS-P0024, incidental bugs to the `support` initiative
  (`project.md` DoD item 9). Record the route in `_drill-observation.md` so nothing is absorbed
  silently.

## Tests and CI (merge gate)

Grounded in the testing brief's five tiers (`_decomposition.md`, "The test mix") and in the
`REQUIRED` array at `xtask/src/main.rs`. This story adds no command of its own.

| Tier | Command / path | Proves |
| --- | --- | --- |
| 3 — executed (primary) | `cargo test -p happenstance --doc -- --show-output` | The crate-root twin's refusal assertion **runs**, not merely compiles; under the drill's edit it fails on that assertion and after the revert it passes. Carries AC-002, AC-003, AC-004 with no dependency on HS-P0020. |
| 3 — executed (page mount) | `cargo test -p xtask --doc -- --show-output` (page registered in `xtask/src/narrative.rs`, tree `docs/` per HS-P0020's `TREE`) | The same three, at the mount the reader is actually reading. The failure captured here is the one the page quotes (AC-006). |
| 3 — swept by the gate | `cargo test --locked --workspace --all-features -- --show-output` — the `"tests"` REQUIRED step, `probe: None` (`xtask/src/main.rs:143-155`) | Both doctest targets above are inside a step the gate already runs, so no gate step is added (NF-001) and the boundary's removal turns a green tree red on its own. |
| 3 — contingency, only under EC-002 | `crates/happenstance/tests/` — a `#[tokio::test]` in the manner of `examples/course-subscriptions/src/main.rs:200-224`'s `commit` helper, against a real `MemoryEventStore` | AC-005's proof survives a substrate that type-checks without executing. Swept by the same `"tests"` step. Never a mock, stub or `happenstance_testkit` fixture (testing brief, "Fixtures and seams"). |
| 2 — compiled fence | HS-P0020's compiled-fence REQUIRED step over the pinned tree | The step-3 page compiles against the real workspace crates. Consumed, not built here. |
| 1 — structural | `cargo xtask spec-trace` (REQUIRED step `"specification traceability"`, `xtask/src/main.rs:315-327`) | The drill's ES-25 citation resolves — the mechanical half of "cite, never restate" (AC-007, UX-007). |
| 1 — structural | `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings` (REQUIRED step `"documentation"`, `xtask/src/main.rs:290-301`) | No broken intra-doc link is introduced by the drill's prose (RS-70-2, `standards/rust/70-rustdoc-obligations.md`). |
| 4 — human-observed | `.bklg/docs-that-teach/application-author-path/boundary-falsification-drill/_drill-observation.md` | DoD-4 and project DoD item 3: the drill run and observed **in both directions**, dated, with verbatim output. Carries AC-005; it is the only record this obligation will ever have. |
| 5 — review sign-off | Reviewer walk against `_design.md` `## Composition`, `## Transience policy`, `## Density budget`, `## Hierarchy`, `## Anti-patterns` | AC-001, AC-006's no-paraphrase half, AC-007. The composition invariants an unstyled render passes and a reader does not. |
| story checkpoint | `cargo xtask affected --base main` | Only what this diff could break, at the story's checkpoint commit (CLAUDE.md, Commands; `_storymap.md`, "Standing checks per story"). |
| project bar | `cargo xtask ci --fast` | The bar this `terminal: false` project is held to (`project.md` DoD item 5). HS-P0025 owns the whole-initiative `cargo xtask ci` re-observation. |

**The named wrong implementation each tier rejects here.** Tier 3 rejects a fence marked `no_run`
that type-checks the boundary forever without asking it to refuse anything. Tier 4 rejects a
boundary that looks real in the source but is not load-bearing — a query that never matches
anything the scenario appends, so removing it changes nothing observable (`project.md`, Risks,
row 1). Tier 5 rejects a drill folded inside a `<details>`, or one whose failure text was written
rather than captured.

## Risks and coupling (PR-scoped)

| Risk | L / I | Mitigation inside this PR |
| --- | --- | --- |
| **The drill is written from imagination and the quoted failure does not match what a reader sees.** The single most likely defect, because the page can be authored before the run. | Medium / High | AC-006 makes byte-identity with the transcript the criterion, and the implementation notes fix the order (capture, then author). The reviewer diffs the two blocks rather than reading the page for plausibility. |
| **The removal turns out not to break anything** (EC-001) — the boundary was decorative all along. | Low / High | This is the initiative's first-ranked risk, and finding it here is a *success* of the story, not a failure: the drill is the only instrument that can detect it. The response is to fix the scenario with the slice-mate before the slice checkpoint, never to weaken AC-003. |
| **HS-P0020's mechanism type-checks without executing** (EC-002) — AC-005's proof moves. | Medium / Medium | The crate-root twin already executes today, so the story's proof never depends solely on the unlanded substrate. The contingency `#[tokio::test]` is pre-scoped and pre-authorised in the PR boundary; the finding routes to HS-P0020 in the same change. |
| **Scope drift into the slice-mate's work.** The drill sits inside a page the slice-mate authors, so "just fix the fence while I'm here" is one keystroke away. | Medium / Medium | The PR boundary names the three-step encounter, the crate-root `## Watch a boundary refuse` fence and the `tokio` dev-dependency as **not** this PR's. Touching the harness registration line and the twin *is* permitted and is not drift; authoring the steps is. |
| **The captured output blows the 68-column budget** and trips anti-pattern 4. | High / Low | Resolved deliberately below (Clarifications): captured output is quoted as-is and the medium's `overflow-x` handles it, on the same footing as finding F-3's 92-character `Debug` line. Editing the output to fit would make the page lie, which is the larger defect. |
| **Two copies of the program drift.** The page and the twin are the same program in two files, and only *correctness* is protected by both executing. | Medium / Low | Stated and accepted by `_design.md` `## Placement and re-export`. This story additionally verifies the same removal breaks both, which is the strongest identity check available short of a shared file — and the shared-file consolidation is routed to HS-P0020, not attempted here. |
| **`_design.md`'s conditions are unmet at implementation time** — the AC-014 merge forward is not recorded, or the page is authored against this worktree's 75-line `crates/happenstance/src/lib.rs`. | Low / High | Both conditions are on the design's sign-off row (2026-08-17) and `merge-forward-preflight` is upstream in the merge order. Confirm the merge is recorded before writing a line; a page authored against the stale copy is a page to redo. |

## Dependencies

**Blocks on** (must merge first):

- **`boundary-refusal-encounter`** — the slice-mate, and the only `depends_on` edge. It lands the
  three-step encounter, the crate-root `## Watch a boundary refuse` fence and the `tokio`
  dev-dependency; without step 3's program there is nothing to falsify. The two are implemented in
  one context and mounted as one integrated surface: the storymap is explicit that the drill "is
  not a follow-up PR that may slip, because the encounter without it is exactly the failure this
  initiative exists to prevent" (`_storymap.md:141-144`).

Transitively, through the slice-mate: `merge-forward-preflight` (AC-014, DR-13 — the merge forward
recorded) and `tension-resolutions` (AC-001 — `_design.md` signed off, which it is, 2026-08-17,
with two recorded conditions).

**Unlocks:**

- **`fence-inventory-and-clause-audit`** — its inventory of "every fenced block this project
  authored, showing zero opted out" cannot be produced until this story has fixed the attribute
  state of the refusal fences at both mounts (AC-002) and this story's zero additions to
  `IGNORE_ALLOWANCES` are a fact rather than an intention.
- **`answered-need-and-anchor-review`** — its set-wide walk needs step 3 in its final shape,
  including the `###` this story adds, before "exactly one answered-need per page" (AC-012) can be
  checked against the page as it will ship.

**Consumed, not depended on** (owned by sibling projects inside this initiative): HS-P0020's
pinned tree, `TREE`/`HARNESS` constants, `IGNORE_ALLOWANCES` and `HIDDEN_MARKERS`; HS-P0021's
answered-need notation. This story adds nothing to the first and invents no second form of the
last.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Each row says why the artifact is load-bearing
and the moment to open it; every path was confirmed present in this worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/application-author-path/_design.md` | The signed-off, binding design. `## Composition` fixes the drill's slot (a `###` at the bottom of step 3, after the citation, never a page of its own); `## Transience policy` classes it Persistent; `## Density budget` carries the real numbers (24 rendered lines, 68 columns, 16px/24px code type); `## Hierarchy` says which channels can carry "recessive"; `## Anti-patterns` is the reviewer's checklist. None of it is re-decidable here. | Before writing a single line of the drill, and again with the reviewer before the checkpoint. | AC-001, AC-006, AC-007 |
| `.bklg/docs-that-teach/application-author-path/_decomposition.md` | Holds both briefs. The UX brief's UI-2, IQ-5 and UX-002 are the reader-intent this story realizes and its States table fixes *Deliberately broken* and *Restored* as contracted states. The testing brief's tiers 3 and 4, its `no_run` note and its AC-005 coupling note are where the proof obligations come from. | UX brief before authoring the drill's three parts; testing brief before choosing which command the drill names. | AC-002, AC-003, AC-004, AC-005 |
| `xtask/src/main.rs` | The `"tests"` REQUIRED step at `:143-155` (`cargo test --locked --workspace --all-features -- --show-output`, `probe: None`) is the check this story makes load-bearing, and the `REQUIRED` array is what must come back unchanged. `:290-301` and `:315-327` are the documentation and spec-trace steps the drill's prose must not break. | Before claiming any command "the gate already sweeps", and again when diffing for NF-001. | AC-002, AC-003 |
| `crates/happenstance/src/lib.rs` | The crate-root twin: the second, already-real mount, executed by `cargo test -p happenstance --doc` today. It is what makes AC-005 provable without waiting on HS-P0020, and it carries the `include_str!` reasoning (`:7-9`) that explains why the program exists twice. **Read the merged copy, not this worktree's 75-line one** (AC-014). | First, before the page exists — run the drill against it to prove the edit is the right edit. | AC-002, AC-003, AC-004 |
| `crates/happenstance-core/src/query.rs` | `Query::from_items` returns `Err(InvalidQuery::NoItems)` on an empty iterator (`:180-190`). This is the whole reason "empty the query" is forbidden as the drill's edit: it goes red for a constructor reason the reader cannot distinguish from a build error. | When writing the edit instruction, and if anyone proposes emptying the query. | AC-006 |
| `crates/happenstance-core/src/append.rs` | `AppendCondition::new`, `after_opt` and the `#[non_exhaustive]` `Guard` — the real spellings the canonical edit removes and restores. Finding F-5 corrected `_design.md` against this file; do not re-derive the shape from prose. | While writing the exact edit and the exact revert. | AC-003, AC-004 |
| `crates/happenstance-core/src/store.rs` | `read_decision_model` (`:321`) and `EventStore::head` (`:248`) — the pairing the contract crate's own documentation names, and the one the verified fence shape uses. `head` takes no argument, which is what F-5 caught. | Alongside `append.rs`, when confirming the program the drill edits. | AC-003 |
| `xtask/src/constitution.rs` | `:11-18` is the repository's already-paid lesson on `include_str!` diagnostics: a failure reports against the *including* item at a line counted from the first, "which maps to no file a reader can open". This is why the quoted failure may name the harness and why it is quoted rather than composed. `:39-50` is the one-module-per-file shape. | When the captured failure looks wrong, before deciding it is a defect rather than a substrate finding (EC-004). | AC-006 |
| `spec/SPECIFICATION.md` | ES-25 at `:3693` (`[FROZEN]`) is the clause the refusal assertion makes concrete — cited, never restated. CF-7 at `:7266` is the bridge story's to spend, not this one's. Clause ids are stable and never renumbered (`:280`). | When writing the citation the drill's prose carries, and before any sentence that reads like a rule. | AC-007 |
| `examples/course-subscriptions/src/main.rs` | `:200-224`'s `commit` helper is the in-repo reference shape — `AppendCondition::new(query.clone()).after_opt(last_seen)` against a real `MemoryEventStore` — that the contingency `#[tokio::test]` follows if EC-002 fires. | Only if the executed path turns out not to execute. | AC-002 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | HS-P0020's approved substrate design: `const TREE = "docs"`, `const HARNESS = "xtask/src/narrative.rs"`, `IGNORE_ALLOWANCES` (to which this story adds nothing) and `HIDDEN_MARKERS` (which include `<details`/`<summary` with deliberately no allowance list, making a fold a gate failure). | Before mounting the page side, and before assuming anything about where the tree lives. | AC-002, AC-007 |
| `.bklg/docs-that-teach/initiative.md` | DoD-4 at `:425-428` — *"@smoke — the boundary claim is checked, not narrated"* — is the obligation AC-005 records, and it is stated as "run and observed", which is why a transcript exists at all. BR-03 is the requirement behind it. | When writing `_drill-observation.md`, to check the transcript answers the scenario as written. | AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | `:56-144` is Persona 1's measured journey and `:99-106` their stated fear of silent wrongness — the fear every criterion in this story is written against. Without it, AC-006 reads as pedantry about quoting. | Before framing the drill's prose, and when tempted to explain the failure instead of showing it. | AC-001, AC-006 |
| `.bklg/docs-that-teach/application-author-path/_storymap.md` | The slice contract: `opening-encounter` is one mounted surface, the drill is not a follow-up PR (`:141-144`), and AC-005 "routes nowhere under any circumstance" (`:158-163`). The routing paragraph is what to consult when something is found and not fixed. | At slice planning, and again when a substrate gap tempts a reroute. | AC-005 |
| `.bklg/docs-that-teach/application-author-path/design/mock.html` | The `falsification-drill` frame — the only rendered picture of this surface that exists, drawn in rustdoc's own markup under its own stylesheet. It deliberately shows the pre-correction figures and must not be edited; read it for composition, not for numbers. | During the reviewer walk, as the reference for what "at the bottom of step 3" looks like. | AC-001, AC-007 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | The Accepted decision atom behind UX-011 and IQ-9: the taught vocabulary is `use happenstance::{…}` because the application author is exactly the reader the bare name was given to. Anti-pattern 13 is its check. | When any import line is written or reviewed. | AC-007 |
| `standards/rust/70-rustdoc-obligations.md` | The only constitution atom on documentation: RS-70-2 (no intra-doc link that resolves in only some feature configurations), RS-70-3, RS-70-5. The `"documentation"` gate step enforces the first with `-D warnings`. | Before adding any link to the drill's prose. | AC-007 |

## Clarifications resolved during spec

1. **The AC set is exactly the seven the front half enumerated.** AC-001…AC-007 as written above;
   none added, none dropped. The composition invariants RFC §6.7/D6 requires as table rows are
   distributed across AC-001 (presentation, placement, hierarchy), AC-006 (copy fidelity of the
   quoted failure) and AC-007 (transience, density, the named anti-patterns) rather than given one
   row each. Recorded because it is a real trade: it keeps the ledger aligned to the front half's
   decision at the cost of AC-007 being the widest row in the table. The Interaction quality
   section is therefore the map that says which invariant lives in which row, so a reviewer can
   still check them one at a time.

2. **Captured output is exempt from the 68-column budget; authored fences are not.** The density
   budget forbids a fence over 68 columns (anti-pattern 4), and a real `cargo test` failure will
   exceed it — `_design.md`'s own finding F-3 already accepted that the 92-character `Debug` payoff
   line scrolls at every viewport. The resolution is the same one F-3 took: the medium's
   `overflow-x` handles it, and the *arrangement* carries the meaning — the drill names which line
   is **the** line, so the reader recognises the failure without horizontal scrolling being
   load-bearing. Re-wrapping, truncating or eliding the captured output is forbidden outright
   (AC-006, UX-013's copy fidelity), because a page that edits the output it tells the reader to
   expect is exactly the silent wrongness this story exists to prevent.

3. **The canonical edit is "remove the condition from the append", not "empty the query".** The
   project AC's own words are "deleting or emptying the query the append condition is built from",
   and the second half of that phrase is not implementable as stated: `Query::from_items([])`
   returns `Err(InvalidQuery::NoItems)` (`crates/happenstance-core/src/query.rs:180-190`), so the
   check goes red for a constructor reason, and deleting the binding is a build error. Both fail
   `_design.md`'s requirement that the reader recognise **the** failure rather than a build error.
   Passing `None` in place of `Some(&AppendCondition::new(seats).after_opt(upto))` keeps the
   program compiling and running, accepts the racing append, and fails exactly the refusal
   assertion. This is a narrowing of the project AC's phrasing to the one spelling that satisfies
   its intent, not a weakening of it, and it is recorded here rather than discovered in
   implementation.

4. **AC-005's proof does not wait on HS-P0020.** The crate-root twin in
   `crates/happenstance/src/lib.rs` is executed today by `cargo test -p happenstance --doc`, so the
   falsification is provable at one real mount regardless of whether the pinned tree has landed.
   That is what makes `_storymap.md:163`'s categorical "AC-005 routes nowhere under any
   circumstance" affordable, and it is why EC-006 is a halt-and-report rather than a reroute.

5. **No conformance rule is added, deliberately.** CLAUDE.md's bar is that a rule no adapter can
   fail is decorative. This story changes no port, no value type and no store behaviour, so a rule
   in `happenstance-testkit`'s `suite.rs` would have no wrong adapter to reject. The testing brief
   independently forbids reaching for `event_store_conformance!` or `MemoryFixture` here. The check
   that observes this story's behaviour is a doctest under the `"tests"` step.

6. **The drill counts as step 3's one declared additional element**, not as an eighth element
   breaking the per-step budget. `_design.md` `## Composition` states it explicitly ("Step 3
   additionally carries, after item 6, the falsification drill as its own `###`"), so the
   seven-element budget is not violated by its presence — but it is by anything *else* this story
   might be tempted to add to step 3, and nothing else is added.

7. **The observation companion is named `_drill-observation.md`** in this story's own folder. It is
   the tier-4 artifact AC-005 requires and the only record DoD-4 will ever have; it is not a
   report, not a summary, and not a substitute for the ledger. HS-P0025 owns the whole-initiative
   re-observation at closeout, which is the only thing that supersedes it.
