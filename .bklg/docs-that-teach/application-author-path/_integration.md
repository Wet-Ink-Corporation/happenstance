---
title: Integration audit — The Application Author's Path
initiative_slug: docs-that-teach
project_slug: application-author-path
terminal: false
dod_green: true
reachability_ok: true
deferred_scenarios:
  - "DoD-1 — @smoke: the teaching survives a clean checkout (whole assembled tree)"
  - "DoD-2 — @smoke: a deliberately broken page fails the gate, by name (HS-P0020)"
  - "DoD-5 — a non-author, non-insider reader completes a stated scenario (HS-P0024)"
  - "DoD-6 — every stumble in that log has a disposition (HS-P0024)"
  - "DoD-7 — @smoke: the reader reaches the teaching from the front door (HS-P0023)"
  - "DoD-8 — every page answered-need is stated and singular, over the full set (HS-P0025)"
  - "DoD-9 — the evaluator second question is walked (HS-P0023)"
  - "DoD-10 — the adapter author error meets its explanation (HS-P0023)"
  - "DoD-11 — the frozen documentation MUSTs are still discharged (HS-P0020)"
  - "DoD-12 — no page has become a second specification, over the full set (HS-P0025)"
  - "DoD-13 — nothing load-bearing is hidden from the check (HS-P0020, DT-7)"
  - "DoD-14 — the discipline is on disk and cited (HS-P0021)"
  - "DoD-15 — the audience is durable and reconciled (HS-P0025)"
---

# Integration audit — The Application Author's Path

`terminal: false`. This project's bar is the **project-scoped** one: every capability it
delivered is mounted into the real render path, and the affected-package gate is green over the
tree as this project leaves it. The whole-initiative Definition-of-Done journeys it does not own
are listed under *Deferred to the terminal project* and are **not** counted against it
(`project.md:289-292`, DoD item 5: "This project is `terminal: false`, so the project-scoped
integration bar applies; HS-P0025 owns the whole-initiative re-observation").

Audited against `initiative/docs-that-teach` at `ec2f658`, worktree
`D:/repos/happenstance/.claude/worktrees/docs-that-teach`, toolchain **1.97.1**
(`rust-toolchain.toml`). Tree clean at the start of the audit and clean at the end.

---

## Project integration bar

The nine boundary-level items of `project.md:279-302`, each **executed** with a pass/fail
result. Nothing below is `fixme`, skipped, or flag-gated off.

| # | Scenario this project owns | Executed | Result |
| --- | --- | --- | --- |
| 1 | `_design.md` signed off carrying resolutions for DT-1, DT-4 and the joined DT-5 + DT-6 | yes — read on disk | **pass** — `_design.md:81` (DT-1, resolves at `:120`), `:122` (DT-4, resolves at `:168`), `:170` (DT-5 + DT-6 as one, resolves at `:262`). Gate signed off 2026-08-17 (`:1041`), amended 2026-08-19 under BC-002 by the sign-off owner (`:447`, `:475`) |
| 2 | Initiative DoD-3 — the opening encounter reaches a running program in which an append is refused because a boundary held | yes — `cargo test -p xtask --doc -- first_encounter`, run by this audit | **pass** — 3 passed, 0 failed. `narrative::first_encounter (line 96)` is `docs/first-encounter.md:96-121`, whose refusal line is printed from inside the `Err(AppendError::ConditionViolated(_))` arm (`docs/first-encounter.md:113-118`) |
| 3 | Initiative DoD-4 — remove the boundary, a repository check fails; revert, it passes | yes — **both directions performed by this audit**, not read off a record | **pass**. Direction one: `docs/first-encounter.md:113` `Some(&condition)` changed to `None`; the command above went **FAILED**, exit 101, `the boundary did not hold: Ok(SequencePosition(3))` at `narrative::first_encounter (line 96)` — byte-identical to the two lines the page quotes at `docs/first-encounter.md:149-150`. Direction two: `git checkout -- docs/first-encounter.md`, `git status --porcelain` empty, re-run gave 3 passed, 0 failed |
| 4 | Every page this project authored builds and renders inside `cargo xtask ci`, through HS-P0020's step | yes — `cargo xtask ci --fast` | **pass** — `=== the narrative tree's examples compile ===  3 page(s) examples enumerated`, with `narrative::first_encounter` (lines 16, 55, 96) and `narrative::carry_your_invariant` (lines 60, 102) all `ok`; `=== every narrative page is checked ===  5 pages, all consistent` |
| 5 | `cargo xtask ci --fast` green on this project's branch | yes | **pass** — exit 0, `all required checks passed (--fast: 4 optional step(s) not run)`. The narrower `cargo xtask affected --base main` was run too: exit 0, `affected gate passed`, with `course-subscriptions`, `happenstance` and `xtask` in the affected set |
| 6 | `cargo xtask spec-trace` passes and every clause citation this project added resolves | yes — inside `ci --fast` | **pass** — 201 clauses, 401 citations checked, `traceability: no problems found`. The narrative checker resolves each page's clause ids against `spec/SPECIFICATION.md` independently (`xtask/src/lint_narrative.rs:1197-1244`) and reported no problem over the five pages |
| 7 | A reviewer walks this project's page set against HS-P0021's answered-need check and records the result | yes | **pass** — `answered-need-and-anchor-review/_walk.md` section 3: four surfaces, four `pass`, no row at `fail` or `indeterminate`. Mechanical half re-observed by this audit inside `ci --fast`: `=== every page declares one need ===  5 pages, 16 rules, all consistent`. The walk's calibration (section 5) shows the instrument returning `fail — two needs` in both directions, so the absence it reports is not vacuous |
| 8 | The merge forward is recorded, and no page was authored against the pre-merge crate root | yes | **pass** — `a5c0f30 Merge initiative/from-contract-to-published-library into docs-that-teach` precedes `7c62e4d redkiln: advance HS-P0022 design -> implementation`. `crates/happenstance/src/lib.rs` is 242 lines here, the merged copy, not this branch's 76-line one |
| 9 | Anything found and not fixed is routed; nothing absorbed silently | yes | **pass** — `HS-B0001` opened at `.bklg/support/inherited-documentation-defects/crate-root-density-overages/bug.md` for the three inherited crate-root density overages; W-1 through W-6 routed in `answered-need-and-anchor-review/_walk.md` section 8; F1/F2/F3 routed in `boundary-falsification-drill/_drill-observation.md` (Findings routed); the fence audit rows in `fence-inventory-and-clause-audit/_inventory.md` (Routing) |

**Ledger state.** Every `AC-###` row across all eight stories' `_ledger.md` files is
`satisfied: true` with cited evidence; the single `satisfied: false` occurrence per file is the
boilerplate paragraph that explains the convention, not a row.

**No opted-out fence anywhere in this project's output.**
`xtask/src/lint_narrative.rs:304` is still the empty slice
`const IGNORE_ALLOWANCES: &[(&str, &str, &str)] = &[];`, and every Rust-class fence on the three
pages is `rust`, executed. The two `ignored` doctests the workspace run reports are pre-existing,
in `happenstance-core`, `happenstance-testkit` and
`standards/rust/62-doctests-and-harnesses.md`; none is in `docs/`.

---

## Deferred to the terminal project

Whole-initiative Definition-of-Done journeys (`.bklg/docs-that-teach/initiative.md:406-468`)
this project does **not** own. Each depends on a later project assembling the whole feature;
running them here would fail by design.

| Initiative DoD | Owner | Why it is not this project's |
| --- | --- | --- |
| 1 — clean checkout, gate green with the narrative built | assembled tree / HS-P0025 | The tree is not assembled until HS-P0023 and HS-P0024 have landed |
| 2 — a deliberately broken page fails by name, then recovers | HS-P0020 | `project.md:124-129` — the mechanism and its falsification are out of scope here |
| 5, 6 — the friction log and its dispositions | HS-P0024 | `project.md:140-143` |
| 7 — the reader reaches the teaching from the front door | HS-P0023 | `project.md:134-139` ("who points at it is HS-P0023's"); `reach-and-adapter-path/project.md:44-47`, `:85`, and its AC-004 |
| 8 — every page answered-need, over the **full** set | HS-P0025 | This project walked its own four surfaces (bar item 7); the set-wide re-observation is closeout's |
| 9, 10 — the evaluator second question; the adapter author error | HS-P0023 | `project.md:134-139` |
| 11 — the frozen documentation MUSTs are still discharged | HS-P0020 | `project.md:148-149` — BR-10's clause-id pin, which this project sits transitively behind |
| 12 — no page has become a second specification, over the **full** set | HS-P0025 | This project's own share is audited in `fence-inventory-and-clause-audit/_citations.md` (Restatement): nine normative sentences, nine resolving clause ids, zero restatements |
| 13 — nothing load-bearing is hidden from the check | HS-P0020 | DT-7 is HS-P0020's (`project.md:124-129`) |
| 14 — the discipline is on disk and cited | HS-P0021 | `project.md:130-133` |
| 15 — the audience is durable and reconciled | HS-P0025 | `project.md:144-147` — no `.kb/` atom is hand-authored here |

Initiative DoD **3** and **4** are this project's, and both were executed above rather than
deferred.

---

## Reachability map

"Mounted" for this project means registered in the narrative harness `xtask/src/narrative.rs`,
which is the composition root the gate reads: a page absent from it is compiled by nothing, and
a registration outliving its page is a hard error in both directions
(`xtask/src/lint_narrative.rs:536-564`).

| Capability | Mount point | Reachable? |
| --- | --- | --- |
| `merge-forward-preflight` — the merged baseline | No artefact of its own; its output is the merged `crates/happenstance/src/lib.rs` (242 lines) every later story was authored against, recorded in `merge-forward-preflight/_baseline.md` | **yes** — the merged file is the one the gate compiles, and its crate-root doctest runs in the workspace sweep |
| `tension-resolutions` — DT-1 / DT-4 / DT-5+DT-6 certified | `tension-resolutions/_resolutions.md`, consumed by `_design.md:81-262` | **yes** — `design.capture` is absent from `.redkiln/config.yaml`, so this written record is the only record these choices will have, and every downstream story cites it |
| `boundary-refusal-encounter` — the opening encounter | `docs/first-encounter.md`, registered at `xtask/src/narrative.rs:131-133`; indexed at `docs/README.md:20`; pointed at from the crate root at `crates/happenstance/src/lib.rs:66`; twinned as an executed test at `crates/happenstance/tests/boundary_refusal.rs` | **yes** — three doctests green under `cargo xtask narrative-doctests`; three `#[tokio::test]` cases green under the workspace `tests` step; nineteen composition assertions green in `xtask/tests/first_encounter.rs` |
| `boundary-falsification-drill` — the remove / observe / revert drill | `docs/first-encounter.md:132-162` (`### Try it wrong, then put it back`); guarded by `xtask/tests/falsification_drill.rs` (9 tests, all green) | **yes** — and **performed by this audit in both directions**, see bar item 3. No gate step was added: the falsifying check is the existing `"tests"` REQUIRED step |
| `invariant-to-appendcondition-bridge` — the conceptual bridge | `docs/carry-your-invariant.md`, registered at `xtask/src/narrative.rs:139-142` | **yes as a mount** — both fences executed (`narrative::carry_your_invariant` lines 60 and 102, green); walked by `cargo xtask narrative` and by `cargo xtask lint-pages`. **Reader-navigation caveat: W-1 below** |
| `surface-course-subscriptions` — the worked-example handoff | `docs/read-the-worked-example.md`, registered at `xtask/src/narrative.rs:148-151`; `examples/course-subscriptions/src/overview.md` mounted as the bin crate module doc by `#![doc = include_str!("overview.md")]` at `examples/course-subscriptions/src/main.rs:5`; reach assertions in `examples/course-subscriptions/tests/reach.rs` (7 tests, all green) | **yes as a mount** — the example is a workspace member (`Cargo.toml:3`), so `cargo test --locked --workspace --all-features` sweeps `reach.rs`; `overview.md` is the single copy, asserted by `overview_is_the_only_copy`. **Reader-navigation caveat: W-1 below** | 
| `fence-inventory-and-clause-audit` — the fence and clause census | `fence-inventory-and-clause-audit/_inventory.md` and `_citations.md`; its substrate claims re-derive against `xtask/src/main.rs` and `xtask/src/lint_narrative.rs` | **yes** — every claim in it re-resolves against the tree as it stands, and the gate re-observed its headline result (allowance list empty, 5 pages consistent) |
| `answered-need-and-anchor-review` — the set-wide walk | `answered-need-and-anchor-review/_walk.md` and `_ledger.md` (Recorded result) | **yes** — the mechanical half is the `every page declares one need` REQUIRED step, re-run green by this audit |

**Nothing is constructed-but-unmounted, exported-but-unconsumed, or reachable only through a
test.** All three authored pages appear in `xtask/src/narrative.rs`; the harness bidirectional
registration check would fail on either direction of drift.

### W-1 — the one open reach gap, and why it is not counted here

`docs/carry-your-invariant.md` and `docs/read-the-worked-example.md` **link only to each
other**. Neither appears in the narrative index of `docs/README.md` (which lists
`append-conditions.md`, `first-encounter.md` and `text-fences.md` only, `:19-21`),
`docs/first-encounter.md` links neither, and the crate-root pointer at
`crates/happenstance/src/lib.rs:66` reaches the opening encounter and stops there. The
keyboard traverse in `answered-need-and-anchor-review/_walk.md` section 6 does not complete.

This audit reproduced that sweep independently and got the same result: the only inbound
references to the bridge anywhere in the tree are `docs/read-the-worked-example.md:12`, two test
constants at `examples/course-subscriptions/tests/reach.rs:30,42`, and the harness line at
`xtask/src/narrative.rs:141`.

It is **deferred, not absorbed**, on three grounds, and the deferral was written before the
pages were:

- `project.md:134-139` places the front-door pointer and "every pointer policy" with HS-P0023,
  and states the seam in one sentence: "A page authored here states its own need and cites its
  clauses; who points **at** it is HS-P0023's."
- `invariant-to-appendcondition-bridge/spec.md:222-223` and `:342-343` scoped the index row out
  at **spec** time — "nothing is added to `docs/README.md`'s signpost table" — so this is a
  planned boundary, not a repair improvised for a failing check.
- `reach-and-adapter-path/project.md:44-47` accepts it: "HS-P0022 authors the application
  author's path — and none of that is reachable until something points at it." Its AC-004 is
  initiative DoD-7, deferred above.

**Two things the terminal project must not lose.** First, the asymmetry: this project's own test
`xtask/tests/first_encounter.rs:479-484` encodes "does not route to the page, so it is compiled
but unreachable" as a defect and applies it to `first-encounter.md` only. Two of the four
surfaces sit in exactly that state today. Second, if HS-P0023 does not land, the *model my
invariant in your words* journey — one of the two reader-journeys this project owns
(`project.md:49-51`) — ships with no entrance. W-1 is recorded in
`answered-need-and-anchor-review/_walk.md` section 8, and it is the row to read first at
closeout.

---

## Missing dependencies

**None.** Nothing this project needed was absent, and nothing it should itself have provided is
missing. Two inherited items are recorded rather than raised as blockers:

- **`Tags::empty()` on the crate root** (`crates/happenstance/src/lib.rs:40`, `:57`) — the
  initiative's own headline evidence (`initiative.md:51-53`), inherited intact from the
  `initiative/from-contract-to-published-library` merge and sitting on HS-P0016 landing copy,
  which `project.md:151-154` places out of scope. Project AC-006 asks that no page construct an
  empty consistency boundary at a point where the surrounding prose claims a real one; the
  lead-in at `crates/happenstance/src/lib.rs:23-24` makes no boundary claim, so the criterion
  holds. Recorded as **O2** at `boundary-refusal-encounter/_conditions.md:126` and routed to the
  `_design.md` sign-off owner with BC-002. **Note for closeout: unlike the density overages, O2
  has no Redkiln item of its own** — it lives as a prose routing only, and the terminal project
  should decide whether that is enough.
- **The three crate-root density overages** — routed to `HS-B0001`
  (`.bklg/support/inherited-documentation-defects/crate-root-density-overages/bug.md`,
  `status: pending`), per the incidental-bug route in `.redkiln/config.yaml`.

---

## Integration verdict

**GREEN for a non-terminal project.**

All nine boundary-level scenarios this project owns were executed and passed, including both
directions of the falsification drill performed live by this audit rather than read off a
transcript. `cargo xtask ci --fast` exits 0 over the tree as this project leaves it, and the
narrower `cargo xtask affected --base main` — the story grain `.redkiln/config.yaml` wires — also
exits 0 with `course-subscriptions`, `happenstance` and `xtask` in the affected set. Every
capability delivered is mounted in the real render path: three pages registered in
`xtask/src/narrative.rs`, five Rust fences compiled **and executed**, zero entries on the ignore
allowance list, three `#[tokio::test]` cases and thirty-five source-reading assertions green
inside the existing `"tests"` REQUIRED step, with no gate step added.

The one open item is **W-1**, a reader-navigation gap on two pages whose *mount* is sound and
whose *pointer* is HS-P0023's by charter and by a spec-time scoping decision. It is carried as
deferred initiative DoD-7, not as a defect of this project, and it is the first row the terminal
project should read.

Nothing here is `test.fixme`, skipped, or gated off behind a flag.
