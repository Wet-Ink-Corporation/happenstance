---
title: "Review — The Application Author's Path"
initiative_slug: docs-that-teach
project_slug: application-author-path
terminal: false
overall: 3
dod_green: true
rubric:
  ac-coverage: 3
  integration-reachability: 3
  test-integrity: 3
  gate-greenness: 3
  brief-fidelity: 3
  intent-fidelity: 2
  presentation-fidelity: 0
---

# Review — The Application Author's Path

`HS-P0022`, the **non-terminal** third project of `docs-that-teach`. This is the
project-grain review gate, run adversarially against the cumulative diff
`7c62e4d1...HEAD` (`6682a5a`). Every gate result below was executed in this
worktree during this review rather than read off an evidence file; the tree was
clean at the start and is clean now, and nothing under `crates/`, `docs/`,
`examples/`, `xtask/` or `spec/` was mutated to produce them.

The fifteen whole-initiative Definition-of-Done journeys belong to later
projects — `HS-P0020`, `HS-P0023`, `HS-P0024` and the terminal
`HS-P0025 durable-audience-closeout`. They are listed under `deferred_scenarios`
in `_integration.md:8-21`, are **deferred, not failed**, and are not counted
here. Initiative DoD **3** and **4** are this project's own and were executed.
`dod_green` above is this project's own integration bar.

Checklist, one line each:

- [x] Postcondition — `_integration.md` exists and is authored (nine executed bar rows, a reachability map, a named open finding); `_review.md` is this file.
- [x] All fourteen project ACs (`project.md:232-277`) traced to reachable, committed work and its tests.
- [x] Affected-package gate re-run by me, scoped: `cargo xtask affected --base main` — exit 0.
- [x] Formatter proven green: `cargo fmt --all --check` — exit 0, clean.
- [x] No test deleted, weakened, skipped or flag-gated; `IGNORE_ALLOWANCES` still empty.
- [x] Every delivered page mounted in the composition root `xtask/src/narrative.rs` and executed.
- [x] Design and brief deviations are recorded amendments (BC-001, BC-002), not silent edits.
- [ ] Perceptual review — **does not apply**; presentation was never observed (see the rubric).

## Verdict

approved

## Rubric

Each dimension scored 0 (absent) to 3 (excellent). `approved` requires every
dimension >= 2, with `gate-greenness` = 3, `integration-reachability` = 3,
`intent-fidelity` >= 2, and the applicable Definition-of-Done bar green.

`presentation-fidelity` is the single exception to "every dimension >= 2", and
the exemption is narrow. Here the design review **does not apply**: `_design.md`
declares four surfaces, but `.redkiln/config.yaml` carries no `design:` block at
all and therefore no `design.capture` — deliberately, and by this repository's
own statement (`CLAUDE.md`, `project.md:281-284`): there is no app to screenshot.
No `_design-review.md` exists and no capture was taken. It scores **0 and is
exempt from the bar, never from the record**: presentation was **NEVER
OBSERVED**. It is not scored 3 on the grounds that nothing was found.

| Dimension | Score | Rationale |
| --------- | ----- | --------- |
| ac-coverage | 3 | All fourteen project ACs are met by real, reachable, committed work with executed evidence; the coverage map below cites each. Independently re-derived here: AC-001/002/003 — `_design.md:81` (DT-1, resolving `:120`), `:122` (DT-4, resolving `:168`), `:170` (DT-5+DT-6 as one joint resolution, resolving `:262`), signed off 2026-08-17 (`:1075`) before any page was authored. AC-004 — `cargo test -p xtask --doc -- first_encounter` printed `3 passed; 0 failed` in my run, and the refusal line is printed from *inside* the `Err(AppendError::ConditionViolated(_))` arm (`docs/first-encounter.md:113-118`), so it cannot appear unless the store refused. AC-005 — see the DoD section: both halves are mechanised (`crates/happenstance/tests/boundary_refusal.rs:82`, `:154`, both green in my run) and the transcript is held byte-equal to the page by a test that runs in the gate. AC-006 — enforced mechanically on this project's own pages (`xtask/tests/first_encounter.rs:355`: no `Query::all()`, no `Tags::empty()`), with one crate-root nuance recorded as F-B below. AC-007 — `xtask/src/lint_narrative.rs:304` is still the empty `IGNORE_ALLOWANCES` slice, verified by me; the census at `fence-inventory-and-clause-audit/_inventory.md:55-92` counts eleven fences, five executed Rust, `ignore` 0 / `no_run` 0 / allowance-listed 0. AC-010 — `examples/course-subscriptions/src/overview.md` is the only copy and is mounted as the bin's module doc by `#![doc = include_str!("overview.md")]` (`examples/course-subscriptions/src/main.rs:5`); its 28 lines are byte-identical to the module doc the diff removed, so it is surfaced, not paraphrased. AC-011 — `spec-trace` green inside the affected gate, and I resolved the least obvious citations by hand (`spec/SPECIFICATION.md:7663` for CF-7 and CF-8, `:3814` for ES-26). AC-012 — `cargo run -p xtask -- lints` printed `every page declares one need — 5 pages, 16 rules, all consistent`. AC-014 — `git merge-base --is-ancestor a5c0f30 7c62e4d1` succeeds, and `crates/happenstance/src/lib.rs` is the merged 242-line copy, not this branch's 76-line one. |
| integration-reachability | 3 | Every delivered capability is mounted in the composition root the gate actually reads, and each mount was traced by me rather than taken from the report. `xtask/src/narrative.rs:128-151` registers all three authored pages as their own `#[cfg(doctest)] mod` (`first_encounter`, `carry_your_invariant`, `read_the_worked_example`), one module per file so a failure names the page; the harness fails in **both** directions — an unregistered page and a registration outliving its page (`xtask/src/lint_narrative.rs:536-564`) — and `cargo run -p xtask -- lints` printed `every narrative page is checked — 5 pages, all consistent`. The five Rust fences are compiled **and executed**: three under `narrative::first_encounter` (lines 16, 55, 96) and two under `narrative::carry_your_invariant` (lines 60, 102). `docs/README.md:20` indexes the opening encounter, and `crates/happenstance/src/lib.rs:66` points at it from the crate root a `cargo add` reader lands on. `overview.md` is mounted by `include_str!` in a real bin crate that is a workspace member, so `cargo test --workspace` sweeps `examples/course-subscriptions/tests/reach.rs`. Nothing is exported-but-unconsumed and nothing is reachable only through a test. `_integration.md` reports `dod_green: true` and `reachability_ok: true` with all nine bar rows executed and **zero** `fixme`, skip or flag-gate — I re-ran four of the nine. The one open item, **W-1** (`docs/carry-your-invariant.md` and `docs/read-the-worked-example.md` link only to each other), is the substance of initiative DoD-7 and DoD-9, both **deferred** to `HS-P0023 reach-and-adapter-path`, whose own AC-004 *is* DoD-7 and whose charter accepts the state in as many words. It was scoped out at **spec** time (`invariant-to-appendcondition-bridge/spec.md:222-223`, `:342-343`), before the pages were written, so it is not counted against this project — but see finding F-A: the asymmetry is real and is the first row the next project must read. |
| test-integrity | 3 | Nothing was gutted, weakened, skipped or deleted. `git diff 7c62e4d1...HEAD --numstat` over the test paths returns four files, **all pure additions, zero deletions**: `crates/happenstance/tests/boundary_refusal.rs` (+165), `examples/course-subscriptions/tests/reach.rs` (+441), `xtask/tests/falsification_drill.rs` (+306), `xtask/tests/first_encounter.rs` (+728). No `#[ignore]`, `todo!()`, `no_run` or `compile_fail` was introduced anywhere under `crates/`, `docs/`, `examples/` or `xtask/tests/`; the only matches are two tests *asserting the absence* of exactly those tokens (`xtask/tests/falsification_drill.rs:163`, `xtask/tests/first_encounter.rs:499`). No double, stub or fixture stands in for a runtime AC: `boundary_refusal.rs` builds a real in-process `MemoryEventStore` per test, and its module doc names the wrong implementation it refuses (`:31-35`, a stubbed store returning a canned `ConditionViolated`). It carries the *falsifying* half as a permanent test — `without_the_condition_the_same_append_is_accepted` (`:154`) — which is what separates a boundary that holds from a store that rejects everything, plus a second test (`:113`) that refuses the easy claim and states in code that `after`'s *presence* is not falsifiable while its *value* is. The 2 `ignored` doctests in the affected run are pre-existing `rust,ignore` specimens under `standards/rust/`, none under `docs/`. The source-reading tests are honestly labelled as composition checks and *supplement* executed behaviour rather than substituting for it. |
| gate-greenness | 3 | Executed by me in this worktree at `6682a5a`, affected scope only — never the unfiltered whole-repo release gate. `cargo xtask affected --base main` (`verify.affected_gate` in `.redkiln/config.yaml`) — **exit 0**, `affected gate passed`, doctest tail `233 passed; 0 failed; 2 ignored`, with `course-subscriptions`, `happenstance` and `xtask` in the affected set; it runs fmt, clippy and the tests for exactly those packages plus the always-on file-reading checks (`xtask/src/affected.rs:28-36`). `cargo fmt --all --check` — **exit 0, clean**: the formatter is proven, not assumed. `cargo run --locked -p xtask -- lints` — green, including `every narrative page is checked — 5 pages, all consistent` and `every page declares one need — 5 pages, 16 rules, all consistent`. `cargo test -p xtask --doc -- first_encounter` — `3 passed; 0 failed`. `cargo test --package happenstance --test boundary_refusal` — `3 passed; 0 failed`. `cargo test --package xtask --test first_encounter` — `19 passed; 0 failed`. No Playwright project exists in this repository, so the collection-only pass does not apply. `cargo xtask ci --fast` (the project bar) is cited from `_integration.md:50`: exit 0, `all required checks passed`. Neither `.redkiln/config.yaml` nor `.redkiln/templates/` nor `xtask/src/main.rs`'s `REQUIRED` array appears in this diff — the gate was consumed, never widened, and `xtask/tests/falsification_drill.rs:169-183` asserts that in code. |
| brief-fidelity | 3 | The UX and testing briefs — the two `_decomposition.md` warrants for this project — are honoured as mechanism rather than as prose. Testing: the tier ladder is real, with tier 2 (compiled fence), tier 3 (executed) and tier 5 (recorded walk) all discharged and **zero** exemptions written; the named substrate fallback (EC-002 and its `bridge_guard.rs`) was not needed because the doctests genuinely execute. UX: the interaction-quality invariants ship as tests (`xtask/tests/first_encounter.rs` — the density budget, one answered-need above the first fence, zero affordances, no hidden line carrying any part of the boundary, facade-only imports binding `EventStore` and never `SendEventStore`, per CLAUDE.md constraint 4). No Accepted KB decision is deviated from: `happenstance-core` is untouched, no `#[async_trait]`, no `serde` moved, `spec/SPECIFICATION.md` unedited, ADR-0006's vocabulary taught throughout and its reasoning preserved verbatim at `crates/happenstance/src/lib.rs:76-81`. `.kb/` carries **no** change in this diff — no atom was hand-authored, which is the `0269720` lesson. The `rewrite-the-referent-never-the-reasoning` test is passed twice: the worked example's module doc moved to `overview.md` byte for byte, and the two crate-root edits are link *spellings* that assert exactly what they asserted before. Where the signed-off design and the merged tree collided, the response was an amendment by the sign-off owner with the cost stated (`_design.md:447-475`, BC-002) and a named blocking condition rather than a silent edit (BC-001, the section 6 heading; `_design.md` explicitly **not** edited, under EC-007). The one PR-boundary widening (`xtask/tests/**`) was taken on its own commit with the reason inline (`4232b34`), admits tests only, and leaves `xtask/src/**` outside. |
| intent-fidelity | 2 | Within each surface the delivered interaction serves the reader's intent, and the design intent behind the signposted anchors is visible in the diff rather than asserted. **In place, not a context jump** — the bridge carries the invariant from ordinary vocabulary to `Query` / `QueryItem` / fold / `AppendCondition` with citations as provenance only; strike every off-page link and the argument still completes (`docs/carry-your-invariant.md:5-88`). **Non-occlusion** — zero `#`-prefixed hidden lines on all three pages, no disclosure widget, no tabs, no badge, verified mechanically; the wrong-model contrast is marked at both ends (`:112`, `:124`), sits strictly after the correct guard, and is never the last code on the page (`:148-155`). **Reversibility** — the falsification drill is a two-minute, one-expression edit with the exact failure quoted and an explicit "putting it back" step (`docs/first-encounter.md:132-162`). **Keyboard reachability** — the enumeration of affordances this project introduced is *empty*, by construction and by test. **DT-4's documented failure mode is actually handled**: steps 2 and 3 each open with a two-line "From step N" header and an `Arrived here cold? Start at step one` link (`:47-50`, `:88-91`), which is the mid-sequence-arrival mitigation that tension owed. **Held at 2 by two set-level intents that are deferred rather than delivered, both stated honestly rather than glossed.** (1) W-1: the *model my invariant in your words* journey — one of the two reader-journeys this project owns (`project.md:49-51`) — has no entrance today; the bridge and the handoff are an island (F-A). (2) BC-002's stated cost: the docs.rs reader meets HS-P0016's `commit` program rather than a refusal and reaches the refusal one hop later, and `_design.md:468-471` says in as many words that "the stronger reading is not delivered and is not deemed delivered". Neither is claimed as met; both are routed. That is a gap left honest, which costs a point rather than the verdict. |
| presentation-fidelity | 0 | **Presentation was NEVER OBSERVED.** The design review **does not apply**: `_design.md` declares four surfaces, but `.redkiln/config.yaml` declares no `design:` block and therefore no `design.capture` — deliberately, because this is a Rust library with no app to screenshot (`CLAUDE.md`; `project.md:281-284` makes the consequence explicit: the written record is the *only* record these choices will ever have). No `_design-review.md` exists on disk and no capture was taken, so there is no perceptual evidence to score. Scored 0 and **exempt from the >= 2 bar — exempt from the bar, never from the record**. The composition, transience, density and hierarchy claims elsewhere in this review are read from source and from mechanised checks, and are **not** a substitute for a perceptual observation that never happened. |

## Evidence

### Project AC coverage map

| AC | Met by reachable behaviour? | Evidence |
| --- | --- | --- |
| AC-001 — DT-1 resolved once, signed off before authoring | **yes** | `_design.md:81`, with the chosen option and its reasoning through `:120`; approver row `_design.md:1075` dated 2026-08-17, before the first page commit `9493276` |
| AC-002 — DT-4 resolved, failure mode addressed | **yes** | `_design.md:122-168` (staged, minimal-first, three complete programs); the mitigation ships as the "From step N" header blocks and `Arrived here cold?` links at `docs/first-encounter.md:47-50`, `:88-91`, asserted by `xtask/tests/first_encounter.rs::every_later_step_opens_on_its_two_line_header_block` |
| AC-003 — DT-5 and DT-6 as one joint resolution | **yes** | `_design.md:170-262`, closing on `*Resolves:* DT-5, DT-6, AC-003, AC-013, DR-10, DR-12`; zero uncompiled code ships, so the exemption clause is discharged vacuously and the vacuity is stated, not assumed (`fence-inventory-and-clause-audit/_inventory.md:88-92`) |
| AC-004 — a reader reaches a refusal in the program's own output (DoD-3) | **yes** | `docs/first-encounter.md:96-125`; `cargo test -p xtask --doc -- first_encounter` gave `3 passed; 0 failed` in my run; the printed line is emitted from inside the matched `Err(AppendError::ConditionViolated(_))` arm (`:113-118`) |
| AC-005 — removing the boundary fails a repository check, observed both ways (DoD-4) | **yes** | `crates/happenstance/tests/boundary_refusal.rs:82` (guarded, refused) and `:154` (same append, guard removed, accepted), both green in my run; transcript `boundary-falsification-drill/_drill-observation.md`, held byte-equal to the page by `xtask/tests/falsification_drill.rs:210-235`; both directions performed live by the integration audit (`_integration.md:48`) |
| AC-006 — no empty boundary under prose claiming a real one | **yes**, with nuance F-B | `xtask/tests/first_encounter.rs:355` forbids `Tags::empty()` and `Query::all()` in the authored programs; every fence on the three pages builds a tagged `QueryItem`. The inherited crate-root fence is out of scope by charter (`project.md:151-154`) and is recorded as O2 (`boundary-refusal-encounter/_conditions.md:126`) |
| AC-007 — every fence exercised; inventory shows zero opted out | **yes** | `xtask/src/lint_narrative.rs:304` — `IGNORE_ALLOWANCES` is still the empty slice, verified by me; `fence-inventory-and-clause-audit/_inventory.md:55-92`, eleven fences with the exerciser and its failure mode named per row |
| AC-008 — the bridge carries an invariant to `Query`, a fold and an `AppendCondition` | **yes** | `docs/carry-your-invariant.md:5-88`: the rule in the reader's words (`:7-15`), four named steps and the three-column mapping table (`:30-51`), then the executed guard (`:60-84`) with `Tags`, two `QueryItem`s, one `Query`, `read_decision_model`, a visible fold and `AppendCondition::new(rule).after_opt(upto)`. No step requires `spec/SPECIFICATION.md` or the crate source |
| AC-009 — the DT-1 anchor applied identically, cited from every relying page | **yes** | `answered-need-and-anchor-review/_walk.md:428-465`: all four prior-model phrases occur on one page inside one section span (`docs/carry-your-invariant.md:17-29`), zero hits on the crate root, the opening encounter, the handoff or the index; `xtask/tests/first_encounter.rs::neither_surface_names_the_prior_model` holds it |
| AC-010 — the worked example reachable from this project's material, its explanation surfaced | **yes**, with F-A | `docs/read-the-worked-example.md:19-23` links the explanation then the source, in that order, both targets resolved on disk by `examples/course-subscriptions/tests/reach.rs:331-380`; `overview.md` is the single copy, mounted by `#![doc = include_str!("overview.md")]` (`examples/course-subscriptions/src/main.rs:5`) and asserted unique by `overview_is_the_only_copy` |
| AC-011 — every normative claim a resolving citation, none restated | **yes** | `fence-inventory-and-clause-audit/_citations.md:178-187` — nine normative sentences, nine resolving ids, zero restatements; `cargo xtask spec-trace` green inside the affected gate (201 clauses, 401 citations); I hand-resolved CF-7 and CF-8 (`spec/SPECIFICATION.md:7663`, `:7670`, `:7704`) and ES-26 (`:3814`) |
| AC-012 — exactly one named answered-need per page | **yes** | `cargo run -p xtask -- lints` gave `every page declares one need — 5 pages, 16 rules, all consistent`; the reviewer half is `answered-need-and-anchor-review/_walk.md:145-198`, four surfaces, four `pass`, with the instrument shown returning `fail — two needs` in calibration so the absence is not vacuous |
| AC-013 — if a diagram ships, the query and append-condition cycle is drawn | **yes** (discharged by resolution) | DT-5 resolved to narration; no diagram, chart or image ships anywhere in this diff, and the disposition is recorded rather than left to silence (`invariant-to-appendcondition-bridge/spec.md:113-118`, `_storymap.md:120`) |
| AC-014 — merge forward completed and recorded before the first page | **yes** | `a5c0f30 Merge initiative/from-contract-to-published-library into docs-that-teach`; `git merge-base --is-ancestor a5c0f30 7c62e4d1` succeeds, so the merge precedes the design-to-implementation advance; `crates/happenstance/src/lib.rs` is 242 lines (the merged copy); record at `merge-forward-preflight/_baseline.md` |

### Gate and Definition-of-Done results

Run by me in this worktree at `6682a5a`, affected scope only — `happenstance`,
`xtask`, `course-subscriptions`.

| Command | Result |
| --- | --- |
| `cargo xtask affected --base main` (`verify.affected_gate`) | **exit 0** — `affected gate passed`; doctest tail `233 passed; 0 failed; 2 ignored` (the two are pre-existing `standards/rust/` specimens) |
| `cargo fmt --all --check` | **exit 0**, clean — the formatter is proven green, not assumed |
| `cargo run --locked -p xtask -- lints` | green — `every narrative page is checked — 5 pages, all consistent`; `every page declares one need — 5 pages, 16 rules, all consistent` |
| `cargo test -p xtask --doc -- first_encounter` | `3 passed; 0 failed` — lines 16, 55 and 96 of `docs/first-encounter.md` |
| `cargo test --package happenstance --test boundary_refusal` | `3 passed; 0 failed` |
| `cargo test --package xtask --test first_encounter` | `19 passed; 0 failed` |
| `cargo xtask ci --fast` (project bar) | exit 0, cited from `_integration.md:50` |
| Playwright collection pass | not applicable — no Playwright project in this repository |

**Definition of Done, project grain.** `_integration.md` reports `dod_green:
true` and `reachability_ok: true`, with all nine boundary-level scenarios
(`project.md:279-302`) **executed** and passing, and states plainly that nothing
is `fixme`'d, skipped or flag-gated (`:194`). I re-observed four of the nine
independently: bar 4 and bar 6 through `lints` and `spec-trace` inside the
affected gate, bar 5 through `affected --base main`, and bar 2 through the
doctest run.

**Initiative DoD-4 in particular**, because it is the criterion this project's
own risk table ranks first. I could not re-run the tree-mutating half in this
session — the sandbox refused in-place edits of tracked source — so I settled it
by composing checks I did execute, which is stronger than reading a transcript:
`without_the_condition_the_same_append_is_accepted` proves the same append is
**accepted** once the condition is removed, and the page's own fence ends
`ok => panic!("the boundary did not hold: {ok:?}")` (`docs/first-encounter.md:117`),
so removing the guard necessarily turns `narrative::first_encounter (line 96)`
red. The failure text the page quotes (`:149-150`) is asserted byte-equal to the
recorded transcript by `xtask/tests/falsification_drill.rs:210-235`, which runs
inside the gate — a drill written from imagination cannot survive that. The
integration audit performed both directions live at `ec2f658` (`_integration.md:48`).

The whole-initiative journeys are **deferred, not failed**: DoD-1, 2, 11 and 13
to `HS-P0020`; DoD-5 and 6 to `HS-P0024`; DoD-7, 9 and 10 to `HS-P0023`; DoD-8,
12 and 15 to `HS-P0025` (`_integration.md:8-21`, `:75-87`).

### Escape-hatch, unmounted and scope-drift findings

**No escape hatch found.** Specifically checked and cleared: no double, no-op or
injected stub — the store in every executed check is a real `MemoryEventStore`;
no fixture-pinned assertion — the bridge's second fence asserts `is_ok()` on a
guard that genuinely fails to protect, which is the teaching, and the twin test
asserts the *inverse* outcome of the same scenario; no `fixme`, `#[ignore]`,
`no_run`, `compile_fail` or allowance entry anywhere in this project's output;
no gate step was added, weakened or removed. Three findings are recorded rather
than blocking.

- **F-A — the two-page island (W-1), and the asymmetry inside it.**
  `docs/carry-your-invariant.md` and `docs/read-the-worked-example.md` link only
  to each other. Neither is in `docs/README.md`'s narrative index (`:17-21`),
  `docs/first-encounter.md` links neither, and the crate-root pointer
  (`crates/happenstance/src/lib.rs:66`) stops at the opening encounter. I
  reproduced the inbound sweep independently: outside `.bklg/`, the only
  references to the bridge are `docs/read-the-worked-example.md:12`,
  `xtask/src/narrative.rs:141` and two test constants in
  `examples/course-subscriptions/tests/reach.rs`. This is **not counted against
  the project**: it is the substance of initiative DoD-7 and DoD-9, both deferred
  to `HS-P0023`, whose AC-004 *is* DoD-7 and whose charter accepts the state; it
  was scoped out at spec time (`invariant-to-appendcondition-bridge/spec.md:222-223`,
  `:342-343`) before the pages existed, and this project's own nine-item DoD
  carries no reach item. What the next project must not lose is the **asymmetry**,
  which the audit names and this review sharpens: the project *did* add one row
  to that same narrative index — `docs/README.md:20` for the opening encounter,
  required by `boundary-refusal-encounter/spec.md:412` as part of the *mount*
  ("a page in `docs/` the harness does not name is the documentation equivalent
  of built-but-unmounted") — while two later stories treated the identical act as
  pointer policy and declined it. The project's own test encodes that state as a
  defect for the page it does index: `xtask/tests/first_encounter.rs:479-484`,
  "does not route to the page, so it is compiled but unreachable". Two of four
  surfaces sit in exactly that state. Routed at
  `answered-need-and-anchor-review/_walk.md:610` and `_integration.md:116-150`.
- **F-B — an AC-006 nuance at the crate root that O2's analysis does not cover.**
  This project added `crates/happenstance/src/lib.rs:66`, "To watch **that
  boundary** *refuse* a write instead", one line beneath a fence that calls
  `Tags::empty()` twice (`:40`, `:57`) — the call the initiative's own headline
  evidence says "zeroes out the consistency boundary" (`initiative.md:51-53`).
  O2 (`boundary-refusal-encounter/_conditions.md:126`) clears AC-006 by reading
  only the lead-in *above* the fence (`:23-24`) and does not consider the
  sentence the project itself added *below* it. AC-006 binds pages **authored by
  this project**, and that fence is HS-P0016's inherited landing copy, out of
  scope by charter (`project.md:151-154`) and protected by `doc_budget.rs`'s
  one-fence rule — so the criterion is not unmet and this is not a blocker. It is
  recorded because `_integration.md:167-168` already notes that O2, unlike the
  density overages, has **no Redkiln item of its own**; this sentence makes the
  residual slightly sharper than the O2 row states.
- **F-C — one additive edit outside this project's tree.**
  `.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/spec.md:448`
  gained a single anchor row pointing the terminal project's merge story at this
  project's `_baseline.md` and its two process observations (P1, P2). It is body
  text beneath the frontmatter, additive, and it routes rather than rescopes — a
  handoff, not drift. Recorded so it is a decision rather than an accident.

**Scope drift: none.** The production diff is exactly the four surfaces, their
harness registrations, one index row, one module-doc extraction and four new
test files: `crates/happenstance/src/lib.rs` (+21/-5 — the answered-need line,
the pointer, two link spellings), `docs/README.md` (+1),
`docs/first-encounter.md`, `docs/carry-your-invariant.md` and
`docs/read-the-worked-example.md` (new), `examples/course-subscriptions/src/main.rs`
with `src/overview.md` (the byte-identical move), and `xtask/src/narrative.rs`
(+26, three registrations). `.kb/` is untouched, `spec/SPECIFICATION.md` is
untouched, `.redkiln/config.yaml` and `.redkiln/templates/` are untouched, and
`xtask/src/main.rs`'s `REQUIRED` array is untouched. The one out-of-project item
created — `HS-B0001` under `.bklg/support/inherited-documentation-defects/` — is
the incidental-bug route `.redkiln/config.yaml` prescribes, used correctly.

**Worth recording as a discharge in passing:** `HS-P0021`'s routed residual R2 —
the `orientation` token and `check_orientation_ceiling` had no live subject — now
has one: `docs/read-the-worked-example.md:3` declares `orientation` and sits
inside the checker's corpus (`5 pages, 16 rules`).

### Per-story checkpoint SHAs

Derived from the branch's own history —
`git log 7c62e4d1..HEAD --grep "Story: application-author-path/"` — which agrees
with `_slices.md:23-27`.

| Slice | Story | SHA |
| --- | --- | --- |
| preflight-and-anchor | merge-forward-preflight | `63fa959` |
| preflight-and-anchor | tension-resolutions | `661ebfa` |
| opening-encounter | boundary-refusal-encounter | `9493276` |
| opening-encounter | boundary-falsification-drill | `cc9c4a4` |
| conceptual-bridge | invariant-to-appendcondition-bridge | `b24d2cd` |
| conceptual-bridge | surface-course-subscriptions | `5805623` |
| page-set-assurance | fence-inventory-and-clause-audit | `a189aa2` |
| page-set-assurance | answered-need-and-anchor-review | `b7addcf` |

Slice seals, all `approved`: `7016d65` (preflight-and-anchor), `beb2b18`
(opening-encounter), `9dc139a` (conceptual-bridge), `ec2f658`
(page-set-assurance). Checkpoint-recording commits: `3e0a71e`, `e2599e2`,
`a78ca36`, `b4c0c52`, `17f14b5`, `2a73480`. Integration audit: `6682a5a`.

In-slice fix passes, produced by the adversarial slice reviews and **not** scope
drift — the `opening-encounter` slice was sealed `changes-requested` at `df38576`
before being sealed `approved` at `beb2b18`: `5af116b` (reconciled the ledger and
made step 2 earn its citation), `e165c79` (made step 3's `after` earn its place
and routed what it could not), `4232b34` (the `xtask/tests/**` PR-boundary
amendment, on its own commit with the reason inline), `faa8834` and `38c14a7`
(the BC-002 halt and its resolution by the `_design.md` sign-off owner),
`8161418` (opened `HS-B0001` for the inherited density overages) and `3f33318`
(re-certified the DT-6 probe). A history search for
`Baseline-Repair: application-author-path` and
`Slice-Repair: application-author-path` returns **nothing**: no out-of-band
baseline repair occurred this run.

## Required Changes

None blocking — the verdict is `approved`. F-A, F-B and F-C are follow-ups, each
cited to a `file:line`, and the first two should be carried forward rather than
absorbed:

1. **F-A is the first row `HS-P0023 reach-and-adapter-path` reads.** Its DT-10
   pointer policy has to answer two rows in `docs/README.md`'s narrative index
   and at least one forward link out of `docs/first-encounter.md`; until it does,
   the *model my invariant in your words* journey ships with no entrance, and
   this project's own standard (`xtask/tests/first_encounter.rs:479-484`) calls
   that state a defect. If `HS-P0023` slips, it becomes `HS-P0025`'s.
2. **F-B needs an owner.** O2 is prose routing with no Redkiln item, and the
   sentence added at `crates/happenstance/src/lib.rs:66` now names the
   `Tags::empty()` fence as "that boundary". Either the crate-root landing
   program earns a real tag set (HS-P0016's call) or the pointer sentence is
   reworded so it does not lean on the fence above it. Closeout should decide
   which, rather than inherit the ambiguity.
3. **F-C** is already a handoff; `HS-P0025`'s merge story should confirm P1 and
   P2 landed rather than assume the anchor row is enough.
