---
title: "Review — Page-Need Discipline"
initiative_slug: docs-that-teach
project_slug: page-need-discipline
terminal: false
overall: 3
dod_green: true
rubric:
  ac-coverage: 3
  integration-reachability: 3
  test-integrity: 3
  gate-greenness: 3
  brief-fidelity: 2
  intent-fidelity: 2
  presentation-fidelity: 0
---

# Review — Page-Need Discipline

`HS-P0021`, the **non-terminal** second project of `docs-that-teach`. This is the
project-grain review gate, run adversarially against the cumulative diff
`b23b238...HEAD`. The gate results below were executed in this worktree during
this review, not read off an evidence file; the tree was clean throughout except
for the session's own telemetry file, and nothing was mutated to produce them.

The whole-initiative Definition of Done (fifteen journeys) belongs to the
terminal project `HS-P0025 durable-audience-closeout`; those journeys are
**deferred**, not failed, and are not counted here. `dod_green` above is this
project's own integration bar.

## Verdict

approved

## Rubric

Each dimension scored 0 (absent) to 3 (excellent). `approved` requires every
dimension >= 2, with `gate-greenness` = 3, `integration-reachability` = 3,
`intent-fidelity` >= 2, and the applicable Definition-of-Done bar green.

`presentation-fidelity` is the single exception to "every dimension >= 2", and
the exemption is narrow. Here the design review **does not apply**:
`.redkiln/config.yaml` carries no `design:` block at all, deliberately and by its
own statement (`.redkiln/config.yaml:75-81`, and `CLAUDE.md`) — there is no app
to screenshot. No `_design-review.md` exists and no capture was taken. It scores
**0 and is exempt from the bar, never from the record**: presentation was **NEVER
OBSERVED**. It is not 3 on the grounds that nothing was found.

| Dimension | Score | Rationale |
| --------- | ----- | --------- |
| ac-coverage | 3 | All twelve project ACs (`project.md:206-244`) are met by real, reachable, committed work with tests. AC-001/002: `standards/pages/` holds a router plus five rule atoms; the rank is stated at `standards/pages/README.md:20-25`, and the branch diff of `standards/rust/README.md` is **empty**, re-run here — the precedence chain is cited, never edited. AC-003/004/005: DT-2, DT-3 and DT-8 are resolved with the losing options named at `_design.md:168-212`, `:214-238`, `:240-288`, and the set is closed in code (`xtask/src/lint_pages.rs:119-136`) behind a compile-time ceiling (`const _: () = assert!` at `:146-150`) rather than a deletable test. AC-006: `cargo xtask ci --fast` printed `2 pages, 16 rules, all consistent` in my run. AC-007: falsified at three grains — three story captures (`declaration-check-seen-to-fail/_ledger.md:139`, `:210-211`), an independent live break at the integration grain (`_integration.md:74`), and five unit tests. AC-008: named wrong pages at `xtask/src/lint_pages.rs:2989`, `:3030`, `:2902`, `:2920`, `:2954`, all green inside the 231-test affected run. AC-009/010: the RP-40-1 walk and the RP-40-2 sweep were executed by a non-author over the full nine-file corpus (`_integration.md:81-140`), closing the residual the story ledger left open (`reviewer-and-citation-procedures/_ledger.md:63`). AC-011: `docs/README.md:14`, `:28`, `docs/append-conditions.md:10`, back-link `standards/pages/README.md:8`, link-checked by `check_router`. AC-012: `.kb/_intake/lesson-page-need-declaration-discipline.md` carries `kind: playbook` and `authority_tier: guideline`, and the branch diff under `.kb/` returns that one path — re-run here. |
| integration-reachability | 3 | Nothing constructed-but-unmounted, and reachability was traced to the running composition root rather than to a test. The checker is a `REQUIRED` step with `probe: None` and `--locked` (`xtask/src/main.rs:555-580`), dispatched as a subcommand (`:781-789`), listed in `print_help` (`:875-880`), and in `lint_steps()` (`:925`) where `steps_named` panics on a name mismatch. `xtask/src/affected.rs:132` calls `lint_pages::run` **unconditionally**, paired with the `"standards/pages/"` entry on `INERT` (`:294`) so a rules-only pull request still reads something — and that pairing is itself tested (`the_rules_tree_selects_no_package`, `the_pages_prefix_does_not_swallow_the_constitution`, `affected.rs:791-820`). The pages tree is addressed through `lint_narrative::TREE` by value (`xtask/src/lint_pages.rs:467`) with no second path constant, foreclosed by a test (`:2843`). Observed running, not merely wired: `=== every page declares one need ===` and `2 pages, 16 rules, all consistent` printed in my own `cargo xtask ci --fast`. The rules themselves are mounted in the router's generated index, which `check_router` holds byte-identical to the corpus (`:867-887`). The one deliverable whose consumer is downstream — the staged KB atom — is staged **by design** (AC-012), not stranded. |
| test-integrity | 3 | No test deleted, gutted, skipped or flag-gated. 81 `lint_pages` tests pass with **0 ignored** (`cargo test -p xtask --bins lint_pages`); the affected gate reports `231 passed; 0 failed; 3 ignored`, the three being pre-existing `rust,ignore` constitution specimens untouched by this project. No `#[ignore]`, `todo!()`, `unimplemented!()` or `#[allow]` appears anywhere in `xtask/src/lint_pages.rs`. No crate under `crates/` or `examples/` was touched at all. The one deliberately-broken fixture, `standards/pages/examples/two-needs.md`, stands in for nothing at runtime: it is a calibration specimen for a **human** procedure, sits outside the corpus reader's top-level scan by construction (`xtask/src/lint_pages.rs:393-406`), and was actually walked to a `fail — two needs` verdict at the integration grain. Two in-slice fix passes strengthened rather than weakened: `a985fdf` replaced an untagged-fence parity count that was invariantly satisfied with a real fence-state walk plus two specimen tests, and `40cd4ae` corrected three surfaces that had shipped a false statement about what reads the tree, pinning the corrected sentences with the tests that had pinned the old ones. `7e927c9` replaced five author-run walks with the adversarial reviewer's. That is an audit trail correcting its own evidence, which is the opposite of gaming. |
| gate-greenness | 3 | Executed by me in this worktree at `94fbae4`, affected-scope only. `cargo xtask ci --fast` — **exit 0**, `all required checks passed (--fast: 4 optional step(s) not run)`, with `=== every page declares one need ===` printing `2 pages, 16 rules, all consistent` and the 63 constitution doctests green, which is what proves the four `standards/rust/**` evidence-line updates in this diff are correct. `cargo fmt --all -- --check` — **exit 0, clean**; the formatter is proven green, not assumed. `cargo xtask affected --base main` — **exit 0**, `affected gate passed`, `231 passed; 0 failed; 3 ignored`. `cargo test -p xtask --bins lint_pages` — `81 passed; 0 failed; 0 ignored`. No Playwright project exists in this repository, so the collection-only pass does not apply. `.redkiln/templates/` and `.redkiln/config.yaml` are absent from the project diff entirely — only the telemetry file changed under `.redkiln/` — so the six-advisory `template-drift` assertion is undisturbed; `redkiln doctor` and `redkiln validate --kb` are cited from `_integration.md:56-57`, the CLI not being on this agent's path. |
| brief-fidelity | 2 | The design's decisions land at the named shapes: `standards/pages/` and `standards/pages/README.md` pinned as `RULE_DIR` and `ROUTER` (`xtask/src/lint_pages.rs:157`, `:167`), the step name `every page declares one need` held by `steps_named`'s panic, the five-band atom grammar enforced by `check_atom_shape` (`:713-784`), the `RP-NN-N` prefix chosen so it cannot be read as a `PS-` clause, the four rejected homes recorded (`_design.md:88-96`), and the atom staged into `.kb/_intake/` rather than hand-authored into `.kb/` — the `0269720` lesson honoured. No Accepted KB decision is deviated from: no crate touched, no `#[async_trait]`, no `serde` in `happenstance-core`, `spec/SPECIFICATION.md` untouched, the five-tier chain not extended. **Held at 2 by two cited defects**, both documentation-accuracy rather than behaviour. (F1) `standards/pages/10-the-need-set.md:72` cites `xtask/src/lint_pages.rs:85` for "the one enumeration" and `:169` cites `:110` for the ceiling — both were exactly right at `55b987b` and are now 34 and 36 lines stale, because later stories in **this same project** grew the module docs. The tree adopted the constitution's `Evidence.` *ordering* without its `path:line (anchor)` form, so `lint_constitution::check_citations` (`xtask/src/lint_constitution.rs:673-719`, `ANCHOR_SLACK` at `:105-110`) has no counterpart here — and the same project had to repair four such citations next door (`standards/rust/51`, `52`, `70`, `80`) precisely because a checker forces it there. (F2) the module's `# What this does not verify` list (`xtask/src/lint_pages.rs:1-30`) and the router's "What checks this tree, and what does not" (`standards/pages/README.md:79-99`) both omit the check's **largest** scope limit: every `README.md` is excluded from the governed set (`:496`). That is the RS-81-1 failure the module's own first line cites. |
| intent-fidelity | 2 | The design intent behind the signposted anchors is honoured mechanically rather than asserted. The BR-12 decision — a tree that binds **now**, pinned by path, inside the chain without extending it — is realised and falsifiable: moving the tree costs three edits in `xtask/src/`, and the reads are `?`-propagated so the error names the expected path (`:393`, `:466`). DR-08's checkable fold rule ships as spirit **and** letter (`standards/pages/20-the-fold-line.md`: the deletion test, five never-fold classes with no reviewer exception, an empty `PERMITTED_FOLD_MECHANISMS` table that makes folding forbidden in practice), and the tree cannot acquire a fold silently (`xtask/src/lint_pages.rs:1901-1913`). The interaction lens, in the medium this project actually renders: **non-occlusion** — RP-00-3 and RP-20-2 class 1 forbid hiding the declaration, and a scan for disclosure and tab markers across `docs/` and `standards/pages/` returns nothing, test-enforced; **in place** — the declaration sits immediately after the H1 with nothing interposed (RP-00-2), and the citation of the discipline sits inside the sentence that depends on it (`docs/append-conditions.md:10`) rather than in a see-also footer; **keyboard reachability** — one hop each way, `docs/README.md:28` out and `standards/pages/README.md:8` back, the back-link link-checked by `check_router`; **reversibility** — write mode rewrites only the generated region and nothing else in the tree (`write_mode_rewrites_only_the_region_and_reaches_equality`, `:3122`); **preserved position** — `docs/README.md`'s existing pointer table survives intact, with the new row appended. **Held at 2 by one live inconsistency at the corner the vocabulary was invented for.** `orientation` was added to the set specifically because, left implicit, a landing page is filed under some other token and is then flagged by RP-00-1 (`standards/pages/10-the-need-set.md:32-37`), yet the checker excludes every `README.md` (`xtask/src/lint_pages.rs:496`), which under this repository's naming convention is every index page — so RP-10-2, RP-10-3 and `check_orientation_ceiling` have no live subject, and `docs/README.md` cites RP-10-2 as the rule that shapes it (`:13-15`) while carrying no declaration RP-10-2's own worked example carries. Nothing in `standards/pages/` defines "governed page" or states the exemption, and RP-10-3's `Not` fragment (`10-the-need-set.md:136`) presupposes a `README.md` that declares `orientation`, so a non-author running RP-40-1 over `docs/README.md` today returns a verdict the gate contradicts. This is **not** scored as an escape hatch: the exclusion predates the project (`xtask/src/lint_narrative.rs:240`, `xtask/src/lint_constitution.rs:212-219`), it was proved by a live probe *before* a word was written (`governed-page-cites-the-discipline/_ledger.md:33-45`), and the team refused the easy repair — adding a declaration to a page no checker walks — on the stated ground that it would be a claim of governance nothing extends (`:74-80`), then recorded the residual and assigned it forward to HS-P0022 (`:120-131`). It is a gap left honest, which is why it costs a point rather than the verdict. |
| presentation-fidelity | 0 | **Presentation was NEVER OBSERVED.** The design review **does not apply**: `_design.md` declares five surfaces, but `.redkiln/config.yaml` declares no `design:` block and therefore no `design.capture`, deliberately and by its own statement (`.redkiln/config.yaml:75-81`, `CLAUDE.md`) — this is a Rust library with no app to screenshot. No `_design-review.md` exists and no capture was taken, so there is no perceptual evidence to score. Scored 0 and **exempt from the >= 2 bar — exempt from the bar, never from the record**. The written design record is therefore the only record of DT-2, DT-3 and DT-8, which `project.md:216` states inside AC-003 itself. |

## Evidence

### Project AC coverage map

| AC | Met by reachable behaviour? | Evidence |
| --- | --- | --- |
| AC-001 — decided home, on disk, with a router | **yes** | `standards/pages/README.md` (band table `:10-16`, `## Start here` `:32-42`, generated index `:49-57`) plus five atoms `00`-`40`; rejected homes and their costs at `_design.md:88-96` |
| AC-002 — inside the precedence chain, not extending it | **yes** | `standards/pages/README.md:20-25` states the rank at the constitution-atom tier and cites the chain; the branch diff of `standards/rust/README.md` is **empty**, re-run in this review; `router_states_its_rank_without_editing_the_chain` (`xtask/src/lint_pages.rs:1507`) |
| AC-003 — DT-2 resolved and recorded | **yes** | `_design.md:168-212`, option (c) with (a), (b) and the persona taxonomy named and refuted; mirrored for a reader at `standards/pages/10-the-need-set.md:39-47` |
| AC-004 — DT-3 resolved, need set closed | **yes**, with residual R2 | `_design.md:214-238`; `NEEDS` at `xtask/src/lint_pages.rs:119-136`; the compile-time ceiling at `:146-150`; `check_need_set` binds the atom's table to the constant in both directions (`:915-962`) |
| AC-005 — DT-8 resolved as a stated rule | **yes** | `_design.md:240-288`; `standards/pages/20-the-fold-line.md` (RP-20-1's deletion test, RP-20-2's five closed classes, RP-20-3's empty mechanism table, RP-20-4's asymmetry); applied as step 4 of the walk (`40-reviewing-a-page.md:33`) |
| AC-006 — every governed page declares exactly one need | **yes**, with residual R2 | `2 pages, 16 rules, all consistent` in my own `cargo xtask ci --fast`; `docs/append-conditions.md:3` and `docs/text-fences.md:3`; the `README.md` exclusion is recorded, probed and argued at `governed-page-cites-the-discipline/_ledger.md:25-80` |
| AC-007 — the declaration check has been seen to fail | **yes** | story captures at `declaration-check-seen-to-fail/_ledger.md:139`, `:210-211` (single- and three-problem runs, exit 1 on the step by name); independently re-broken at the integration grain (`_integration.md:74`), then green after revert |
| AC-008 — a wrong page that could plausibly ship is rejected | **yes** | `the_three_wrong_pages_are_each_rejected_at_the_right_line` (`xtask/src/lint_pages.rs:2989`), `two_orientation_pages_at_one_level_are_one_problem_naming_both` (`:3030`), `a_second_declaration_is_reported_at_the_second_ones_line` (`:2920`), `a_malformed_declaration_is_its_own_problem_not_a_missing_one` (`:2954`), `a_needs_member_missing_from_band_ten_names_which_token_moved` (`:3200`) |
| AC-009 — the reviewer procedure is non-author-performable | **yes** | RP-40-1 walked by a non-author over both governed pages **and** the two-need specimen, returning `pass`, `pass` and `fail — two needs` (`_integration.md:81-109`); no page in the set carries two needs |
| AC-010 — the citation rule exists and the spot check runs | **yes** | `standards/pages/30-citing-the-specification.md` (RP-30-1/2/3); the RP-40-2 sweep over all nine files with per-file verdicts and 29 normative sentences counted (`_integration.md:111-129`) |
| AC-011 — the discipline is cited by what it governs | **yes** | `docs/append-conditions.md:10` names RP-00-1 as the rule that shaped it; `docs/README.md:14` names RP-10-2; `docs/README.md:28` routes to the tree; back-link `standards/pages/README.md:8`, link-checked by `check_router` (`xtask/src/lint_pages.rs:799-858`) |
| AC-012 — the playbook atom is staged, not hand-authored | **yes** | `.kb/_intake/lesson-page-need-declaration-discipline.md:1-33` (`kind: playbook`, `authority_tier: guideline`, `status: proposed`); the branch diff under `.kb/` returns that path and nothing else, re-run here; its four `depends_on` and `related` ids all resolve to existing atoms |

### Gate and Definition-of-Done results

Run by me in this worktree at `94fbae4`, affected scope only. `xtask` is the only
package this diff reaches; the rules tree reaches none, which is exactly what
`xtask/src/affected.rs:294` records.

| Command | Result |
| --- | --- |
| `cargo xtask ci --fast` (`verify.integration_scoped`) | **exit 0** — `all required checks passed (--fast: 4 optional step(s) not run)`; the page-need step printed `2 pages, 16 rules, all consistent` |
| `cargo fmt --all -- --check` | **exit 0**, clean — the formatter is proven green, not assumed |
| `cargo xtask affected --base main` (`verify.affected_gate`) | **exit 0** — `affected gate passed`; `231 passed; 0 failed; 3 ignored`, the three being pre-existing constitution specimens |
| `cargo test -p xtask --bins lint_pages` | `81 passed; 0 failed; 0 ignored` |
| Playwright collection pass | not applicable — no Playwright project in this repository |

Definition of Done, project grain: `_integration.md` reports `dod_green: true`
and `reachability_ok: true`, with `cargo xtask lints` and `cargo xtask spec-trace`
green (`_integration.md:54-55`), `redkiln doctor` clean with **exactly six**
`template-drift` advisories, and `redkiln validate --kb` green (`:56-57`). I
independently confirmed the two claims most easily faked: only
`.redkiln/telemetry/` differs under `.redkiln/` across the whole project diff, so
no template was touched and `redkiln adopt --templates` was not run; and only
`.kb/_intake/lesson-page-need-declaration-discipline.md` differs under `.kb/`.

No scenario this project owns is `fixme`'d, skipped or flag-gated-off. The
fifteen whole-initiative journeys are listed under `deferred_scenarios` and
belong to `HS-P0025 durable-audience-closeout`; three of them (DoD-8, DoD-12,
DoD-14) had their **instrument** delivered and executed here, on the tree as it
stands at this merge.

### Escape-hatch, unmounted and scope-drift findings

**No escape hatch found.** Specifically checked and cleared: no double, no-op or
injected stub; no fixture-pinned assertion — the `NEEDS`-to-band-10 agreement is
checked in *both* directions and deliberately not `--write`-repairable
(`xtask/src/lint_pages.rs:915-962`, and the module's closing note at `:78-81`);
no `fixme`, `#[ignore]` or flag-gated scenario; no unmounted deliverable. The
compile-time ceiling (`const _: () = assert!` at `:146-150`) is stronger than the
test it replaces precisely because whoever adds a seventh token cannot delete it.

**Scope drift: none.** The only files outside this project's stated boundary are
`standards/rust/51-features-and-no-std.md`, `52-wasm32-and-target-cfg.md`,
`70-rustdoc-obligations.md` and `80-the-gate.md`, and every change in them is a
mechanical evidence-line renumber forced by `lint_constitution::check_citations`
when `xtask/src/main.rs` grew — the referent moved, the reasoning did not
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md`), and the 63
constitution doctests plus the `lint-constitution` step prove the new numbers.
`xtask/src/lint_narrative.rs` gained `pub(crate)` on one `const` with the reason
written on it (`:228-241`); no behaviour changed and nothing was deleted.

**Residuals, recorded rather than blocking.** None makes an acceptance criterion
unmet; all three are follow-ups.

- **R1 — two `Evidence.` citations in the new tree have already rotted.**
  `standards/pages/10-the-need-set.md:72` points at `xtask/src/lint_pages.rs:85`
  for "the one enumeration" (`const NEEDS` is at `:119`), and `:169` points at
  `:110` for the ceiling (`const MAX_NEEDS` is at `:146`). Both were correct at
  `55b987b` and drifted inside this project as later stories grew the module
  docs. Nothing catches it: the tree took the constitution's evidence *ordering*
  without its `path:line (anchor)` form, so it has no counterpart to
  `check_citations` (`xtask/src/lint_constitution.rs:673-719`). Sharpened by
  RP-30-3, which correctly teaches that a line number rots
  (`standards/pages/30-citing-the-specification.md:85-88`).
- **R2 — the index exemption is real, precedented, probed, and undocumented in
  the tree.** `xtask/src/lint_pages.rs:496` excludes every `README.md` from the
  governed set. Neither the module's `# What this does not verify` list (`:1-30`)
  nor the router's "What checks this tree, and what does not"
  (`standards/pages/README.md:79-99`) says so, no atom defines *governed page*,
  and RP-10-3's `Not` fragment (`10-the-need-set.md:136`) presupposes a
  `README.md` that declares `orientation`. Consequence: the `orientation` token
  and `check_orientation_ceiling` have no live subject, which the team's own
  forward note states (`governed-page-cites-the-discipline/_ledger.md:120-131`,
  and `_integration.md:205-209`).
- **R3 — RP-10-4's `Rejects.` prose says the set is closed "at six" and speaks of
  a page that "strained against all six"** (`standards/pages/10-the-need-set.md:163-167`)
  while the table above it holds four (`:12-17`). `MAX_NEEDS = 6` makes the
  sentence defensible about the ceiling and wrong about the set as it stands.

Inherited and correctly handed forward, not this project's to repair: the `ES-40`
citation at `docs/append-conditions.md:7` names a clause about completeness while
the sentence is about the boundary re-read. It resolves and it restates nothing,
so RP-40-2 returns `pass`; the residual is stated at `_integration.md:131-140`
and routed to HS-P0025's DoD-12 re-observation. The line was authored by HS-P0020.

### Per-story checkpoint SHAs

Derived from the branch's own history — `git log b23b238..HEAD` filtered on the
`Story: page-need-discipline/` trailer — which agrees with `_slices.md:24-26`:

| Slice | Story | SHA |
| --- | --- | --- |
| discipline-on-disk | need-vocabulary-and-declaration-form | `55b987b` |
| discipline-on-disk | router-precedence-and-announcement | `9dacc7d` |
| discipline-on-disk | fold-line-rule | `dec82c7` |
| discipline-on-disk | reviewer-and-citation-procedures | `a349e04` |
| page-need-gate-step | page-need-checker-mounted-in-the-gate | `ee0a500` |
| page-need-gate-step | declaration-check-seen-to-fail | `d7d27c5` |
| binding-beyond-this-project | governed-page-cites-the-discipline | `263dc7b` |
| binding-beyond-this-project | playbook-atom-staged-for-ingest | `dc58e80` |

Slice seals, all `approved`: `f542855` (discipline-on-disk), `6259749`
(page-need-gate-step), `f3b18f7` (binding-beyond-this-project).
Checkpoint-recording commits: `fdc57d2`, `052864d`. Integration audit: `94fbae4`.

In-slice fix passes, produced by the adversarial slice reviews and **not** scope
drift: `a985fdf` (made the untagged-fence rule able to fail), `40cd4ae`
(corrected a false statement about what checks the pages tree, on three
surfaces), `7e927c9` (cited RP rule ids in the staged atom and transcribed five
non-author walks). A history search for `Baseline-Repair: page-need-discipline`
and `Slice-Repair: page-need-discipline` returns **nothing**: no out-of-band
baseline repair occurred this run.

## Required Changes

None blocking — the verdict is `approved`. R1, R2 and R3 are follow-ups, each
cited to a `file:line`, and the first two are worth carrying into HS-P0022, the
next project to author pages under this discipline:

1. Repair the two rotted citations (R1), and decide whether this tree adopts the
   constitution's `path:line (anchor)` evidence form so a checker can hold them.
   That decision belongs in an amendment to `_design.md`'s S3 evidence rule, not
   in a silent edit.
2. State the index exemption where a reader meets it (R2): one line in
   `xtask/src/lint_pages.rs`'s `# What this does not verify`, one in
   `standards/pages/README.md`'s "What checks this tree, and what does not", and
   a definition of *governed page* in band 00. HS-P0022's first walked page is
   also what will give `orientation` and `check_orientation_ceiling` a live
   subject.
3. Reconcile RP-10-4's "six" with the four-token table (R3).
