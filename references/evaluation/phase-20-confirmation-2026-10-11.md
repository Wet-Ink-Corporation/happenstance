# Phase 20 confirmation record: HS-P0020, HS-P0021, HS-P0022 against HEAD (2026-10-11)

Written for #150. The gaps it found are filed as #232 – #236 under epic #66.

**Tree walked:** `origin/main` at `c7b60e6`, in a clean detached worktree. Nothing was committed, and the worktree was clean (`git status --porcelain` empty) after every injection.
**Method:** I read every story `spec.md` under the three `.bklg/docs-that-teach/<project>/` folders, which is 26 stories and 214 acceptance criteria. For each criterion I looked for evidence in the tree at HEAD. Where the criterion is a recorded procedure (a ledger walk, a captured run), I cite the story's frozen record and check what of it still holds at HEAD.
**Ran at HEAD, all green:**
- `cargo test -p xtask --bins`: 506 passed.
- `cargo xtask narrative`: `6 pages, all consistent`.
- `cargo xtask lint-pages`: `6 pages, 16 rules, all consistent`.
- `cargo xtask narrative-doctests`: 7 doctests from 3 pages.
- `cargo test -p happenstance --test doc_surface`: 11 passed. `--test boundary_refusal`: 3 passed.
- `cargo test -p xtask --test first_encounter`: 19 passed. `--test falsification_drill`: 9 passed.
- `cargo test -p course-subscriptions --test reach`: 7 passed.

I did not run `cargo xtask ci` in full, because memory on this host is tight. Its narrative steps were run one at a time, as listed above.

**Two status values only.** A criterion is *shipped* if I saw the evidence at HEAD or in the story's record. It is a *GAP* if HEAD contradicts it. "Superseded as designed" means a criterion that asserted a deliberately temporary state, which a later story in the same project replaced as planned. Those count as shipped.

| Project | Criteria | Shipped | GAP |
|---|---|---|---|
| HS-P0020 checked-documentation-surface | 79 | 79 | 0 (2 routed findings left open, see Gaps) |
| HS-P0021 page-need-discipline | 81 | 81 | 0 |
| HS-P0022 application-author-path | 54 | 50 | 4 |

---

### Narrative checks are steps of `cargo xtask ci`

`cargo xtask ci` runs `run_steps(REQUIRED)` (`xtask/src/main.rs:1342-1343`), and `--fast` runs the same `REQUIRED` array (`:1367-1368`). `REQUIRED` opens at `xtask/src/main.rs:147` and closes at `:930`. Three narrative steps sit inside it, all `probe: None`, so none of them can skip:

- `xtask/src/main.rs:738-766`: `narrative_doctests::STEP`, the step `the narrative tree's examples compile`. It runs `cargo run -p xtask -- narrative-doctests` with `RUSTDOCFLAGS=-D warnings`. The step name is defined at `xtask/src/narrative_doctests.rs:71`.
- `xtask/src/main.rs:781-809`: `lint_narrative::STEP`, the step `every narrative page is checked` (defined at `xtask/src/lint_narrative.rs:358`).
- `xtask/src/main.rs:810-835`: `lint_pages::STEP`, the step `every page declares one need` (defined at `xtask/src/lint_pages.rs:188`).

The two file-reading steps are also reached by other routes:
- `cargo xtask lints`, via `lint_steps()` at `xtask/src/main.rs:1318-1319`.
- `cargo xtask affected`, which calls both unconditionally at `xtask/src/affected.rs:181` and `:189`.

The compiling step is also covered, unfiltered, by the workspace `tests` step (`xtask/src/main.rs:185`).

---

### HS-P0020 checked-documentation-surface: all 79 shipped

Unless a criterion says otherwise, test names are in `xtask/src/lint_narrative.rs`. All of them pass in the 506-test run.

| Story | AC | Criterion (short) | Status | Evidence |
|---|---|---|---|---|
| spec-trace-clause-id-accessor | 001 | `clause_ids` reads the pinned spec | shipped | `xtask/src/spec_trace.rs:3402` |
| | 002 | families come from `SECTIONS`, not a literal list | shipped | `spec_trace.rs:3425`, `:3452` |
| | 003 | deferred ids resolve (existence, not eligibility) | shipped | `spec_trace.rs:3487` |
| | 004 | unreadable or empty spec blames the checker | shipped | `spec_trace.rs:3511`, `:3526` |
| | 005 | limits documented before `# Errors`; double declaration proven | shipped | `spec_trace.rs:3552`, `:3570` |
| | 006 | temporary `expect(dead_code)` erases itself once consumed | shipped (superseded as designed) | consumed at `lint_narrative.rs:2014`; marker gone, asserted at `:3614` |
| narrative-tree-story-grain-selection | 001 | a `docs/` page selects `xtask` | shipped | `xtask/src/affected.rs:1011` |
| | 002 | other prose selects nothing | shipped | `affected.rs:1019`, `:951`, `:964`, `:939` |
| | 003 | `docs/` no longer in `INERT` | shipped | `affected.rs:1033`; `INERT` at `:360` has no `docs/` |
| | 004 | comments falsified by the change were rewritten | shipped | arm at `affected.rs:296`, comment at `:306` |
| | 005 | observed `affected` run recorded | shipped | `.bklg/.../narrative-tree-story-grain-selection/_observed-affected-run.md` |
| pinned-narrative-tree-and-compiling-step | 001 | fixture page carries no hidden marker and fits the path budget | shipped | `xtask/src/narrative_doctests.rs:413` |
| | 002 | page fences compile against real crates | shipped | `xtask/src/narrative.rs:123-124`; 7 doctests pass at HEAD |
| | 003 | a bin-crate mis-mount is caught | shipped | `narrative_doctests.rs:246` |
| | 004 | narrative banner comes first; required; `--locked`; `RUSTDOCFLAGS` | shipped | `narrative_doctests.rs:289`, `:313`, `:371`, `:381` |
| | 005 | empty or none listing fails; count printed | shipped | `narrative_doctests.rs:237`, `:246`, `:270`; run prints `3 page(s)' examples enumerated` |
| | 006 | subcommand dispatched and in help | shipped | `narrative_doctests.rs:391`, `:399`; `main.rs:1133`, `:1241` |
| | 007 | `docs/README.md` narrative table first | shipped | `narrative_doctests.rs:435`, `:460` |
| | 008 | modules state limits first | shipped | `narrative_doctests.rs:485`; `narrative.rs:3` |
| | 009 | gate green on a fresh clone | shipped | in `REQUIRED` with `probe: None` (`main.rs:738-766`); ledger record; step green at HEAD |
| narrative-checker-mounted-with-pinned-path | 001 | moved tree names the pinned path | shipped | `:2196` |
| | 002 | empty tree is a hard error | shipped | `:2210`, `:2226`, `:2232` |
| | 003 | unregistered page reported | shipped | `:2239`, `:2259`; also seen live (fault injection F5) |
| | 004 | stale registration reported | shipped | `:2341`, `:2363` |
| | 005 | runs on `ci`, `ci --fast`, `lints`, `affected` | shipped | `:2396`-`:2470`; `main.rs:1318`; `affected.rs:181` |
| | 006 | every problem, source order, composed line | shipped | `:2481`, `:2516` |
| | 007 | one summary line on green | shipped | `:2532`; observed `6 pages, all consistent` |
| | 008 | path budget at its four corners | shipped | `:2544` |
| | 009 | limits first; no "verified" claims | shipped | `:2571`, `:2598`, `:2619` |
| fence-discipline-and-allowance-list | 001 | untagged fence rejected | shipped | `:2640` |
| | 002 | unknown info-string part and orphan error code rejected | shipped | `:2667`, `:2804` |
| | 003 | `ignore` needs an allowance entry, not a comment | shipped | `:2836`, `:2857`, `:2867`, `:2907`; also seen live (F7) |
| | 004 | stale allowances reported | shipped | `:2925`, `:2939` |
| | 005 | four-backtick blocks skipped; problems name the page | shipped | `:3005`, `:3092` |
| | 006 | all problems, in order | shipped | `:3127`, `:3140` |
| | 007 | text-fence limit documented | shipped | `:3157` |
| hidden-content-resolution | 001 | each hidden marker reported once per occurrence | shipped | `:3258`, `:3296` |
| | 002 | wrapped fixture fails; unwrapped passes | shipped | `:3308`, `:3332` |
| | 003 | case variants and in-fence markers caught; real index clean | shipped | `:3345`, `:3356`, `:3370` |
| | 004 | `HIDDEN_MARKERS` pinned to the design | shipped | `:3387`, `:3418` |
| | 005 | composed lines; 40 markers give 40 lines; ordering | shipped | `:3432`, `:3446`, `:3458` |
| | 006 | no allowance, env or cfg hook; no green-means-teaches claim | shipped | `:3475`, `:3491`, `:3511`, `:3530` |
| narrative-citation-resolution | 001 | families derived from the resolved set | shipped | `:3578` |
| | 002 | declared clause passes | shipped | `:3631` |
| | 003 | dangling id names page, line and id | shipped | `:3648`; seen live (F6) |
| | 004 | near-misses and undeclared families are problems | shipped | `:3674`, `:3698` |
| | 005 | `RS-`, `ADR-` and `HS-` ids are not citations | shipped | `:3723`, `:3733`, `:3743`, `:3755` |
| | 006 | five dangling ids give five problems; in-fence ids checked | shipped | `:3773`, `:3794`, `:3811` |
| | 007 | unreadable spec or page is a hard error naming the artifact | shipped | `:3828`, `:3847` |
| | 008 | limits before the guarantee | shipped | `:3866` |
| frozen-documentation-must-pin | 001 | every pin entry disposed; no written count | shipped | `:3952`, `:3995` |
| | 002 | unresolvable pinned id names the pin | shipped | `:4045`, `:3598` |
| | 003 | missing anchor and missing site are distinct problems | shipped | `:4067`, `:4105` |
| | 004 | reflow-tolerant; `ES-1` does not match `ES-10` | shipped | `:4122`, `:4144` |
| | 005 | document grew vs. enumeration stale | shipped | `:4163`, `:4182`, `:4227` |
| | 006 | all pin problems reported, never truncated | shipped | `:4256`, `:4280` |
| | 007 | location-first lines | shipped | `:4310` |
| | 008 | limits before the pin; no "correct" claims | shipped | `:4415`, `:4442` (and guards at `:4364`, `:4371`, `:4383`) |
| documented-blind-spots-and-their-proofs | 001 | harness opens with `# What this does not verify` | shipped | `xtask/src/narrative.rs:3`; test `:4848` |
| | 002 | checker same; plain-path cross-reference | shipped | `:4860` |
| | 003 | each limit stated where it holds | shipped | `:4848`, `:5052` |
| | 004 | limit 1 says no mechanical test can close it | shipped | `:4890` |
| | 005 | limit 3 cites the measurement | shipped | `:4915`; `_limits-evidence.md:35` |
| | 006 | retained `text`-fence fixture walked | shipped | `docs/text-fences.md`; `narrative.rs:170-171`; `_limits-evidence.md:217` |
| | 007 | positive pin rots loudly | shipped | `:5078` |
| | 008 | limit 4 quoted from the recorded run | shipped | `_limits-evidence.md:296-309`; `:4941` |
| | 009 | three named wrong implementations rejected | shipped | `:4961`, `:4990`, `:5008`, `:5033` |
| | 010 | completeness reconciliation recorded | shipped | `_limits-evidence.md:326` |
| | 011 | no "documentation is checked" mark | shipped | `:5156` |
| | 012 | fixture is a conforming page | shipped | `:5100`, `:5129` |
| observed-failure-falsification | 001 | green, red, green runs recorded | shipped | `.bklg/.../observed-failure-falsification/_falsification.md:98`, `:184`, `:212`, `:515` |
| | 002 | attributed to the narrative banner | shipped | `_falsification.md:274`; re-observed at HEAD (F2) |
| | 003 | pre-registered predictions vs. what was identified | shipped | `_falsification.md:54`, `:379` |
| | 004 | recovered run, with coverage | shipped | `_falsification.md:515` |
| | 005 | no residue | shipped | `_falsification.md:610` |
| | 006 | provenance shas resolve | shipped | `_falsification.md:19`; `6368e2b` and `b0bb9bd` resolve and are ancestors of `main` (GitHub compare API; the local clone is shallow) |
| | 007 | unhedged "silent about teaching" sentence | shipped | `_falsification.md:751-755` |
| | 008 | composition checked against the design | shipped | `_falsification.md:432` |
| | 009 | density measured | shipped | `_falsification.md:461` |

---

### HS-P0021 page-need-discipline: all 81 shipped

Unless a criterion says otherwise, test names are in `xtask/src/lint_pages.rs`. All of them pass in the 506-test run.

| Story | AC | Criterion (short) | Status | Evidence |
|---|---|---|---|---|
| need-vocabulary-and-declaration-form | 001 | `NEEDS` holds four tokens in design order | shipped | `NEEDS` at `:119`; test `:2902` |
| | 002 | `reference` subtracted, with the reason | shipped | `:2917`, `:2927` |
| | 003 | compile-time ceiling on `NEEDS` | shipped | `const _: () = assert!` at `:146` |
| | 004 | one exact accessor `need()` | shipped | `:174`; tests `:2940`, `:2953` |
| | 005 | `RULE_DIR` and `ROUTER` are constants | shipped (router-absence assertion superseded as designed) | `:157`, `:167`; `:2970`; `:2989` |
| | 006 | module docs open with what is not verified | shipped | `:3317` |
| | 007 | hosting assumption stated | shipped | `:3354` |
| | 008 | `mod lint_pages` mounted; temporary `allow` removed later | shipped (superseded as designed) | `main.rs:89`; no `allow(dead_code)` in `lint_pages.rs` |
| | 009 | band 00 fixes the declaration grammar | shipped | `:3369`, `:3394` |
| | 010 | declaration never folded; mechanisms deferred to band 20 | shipped | `:3404`, `:3422` |
| | 011 | band 10 names tokens, rejected options, amendment rule | shipped | `:3436`, `:3458`, `:3469` |
| | 012 | both orientation ceilings (RP-10-2 and RP-10-3) | shipped | `:3482` |
| | 013 | atom grammar, sections, ceilings, columns | shipped | `:4000`, `:4041`, `:4067`, `:4085`, `:4102` |
| | 014 | fences tagged only `text` or `markdown`; evidence cites repo first | shipped | `:4119`, `:4216` |
| router-precedence-and-announcement | 001 | router head and band table | shipped | `standards/pages/README.md:1-16`; `:2998` |
| | 002 | precedence tier stated, chain not edited | shipped | `README.md:18-30`; `:3028` |
| | 003 | complete generated index | shipped | `README.md:44-57`; `:3059` |
| | 004 | generator output byte-identical | shipped | `:3089`, `:4656` |
| | 005 | shape of a rule; one-line `Load when` | shipped | `README.md:59`; `:3125` |
| | 006 | "what checks this tree" section | shipped (now states the step that reads it, as designed) | `README.md:79-97`; `:3152` |
| | 007 | `docs/README.md` routes to the tree in one hop | shipped (PROVISIONAL marker removed when the checker landed, as planned) | `docs/README.md:32`; `:3189` |
| | 008 | regions in binding order; no disclosure markup | shipped | `:3240`, `:3422` |
| | 009 | router budgets (8,192 bytes, 96 columns, ≤ 12 rows) | shipped | `:3279`; `wc -c` gives 5,429 |
| fold-line-rule | 001 | deletion test, no "use judgement" hedges | shipped | `standards/pages/20-the-fold-line.md:16-24`; `:3549`, `:3564` |
| | 002 | five never-fold classes, closed, no exceptions | shipped | `20-the-fold-line.md:42-57`; `:3580` |
| | 003 | pays band 00's deferral | shipped | `20-the-fold-line.md:5`; `:3603` |
| | 004 | `PERMITTED_FOLD_MECHANISMS` table has zero rows | shipped | `20-the-fold-line.md:93-97`; `:3627` |
| | 005 | four recorded observations; upstream docs do not count | shipped | `20-the-fold-line.md:99-104`; `:3651` |
| | 006 | rustdoc `toggle top-doc` wrapper, both halves | shipped | `20-the-fold-line.md:63-66`; `:3672` |
| | 007 | "what this rule does not do"; no `const` in `xtask` | shipped | `20-the-fold-line.md:145-160`; `:3692`; the constant name appears in `xtask/` only inside the test that asserts its absence (`:3415`) |
| | 008 | grammar, ceilings, fences | shipped | shape tests `:4000`-`:4119` iterate the whole tree; 4 rules, 8,558 bytes |
| | 009 | Start-here row and index row | shipped | `README.md:38`, `:53`; `:3089` |
| reviewer-and-citation-procedures | 001 | band 30 falsification question | shipped | `standards/pages/30-citing-the-specification.md:17-23`; `:3737` |
| | 002 | blind spots stated before RP-30-1 | shipped | `30-citing-the-specification.md:10`; `:3762` |
| | 003 | `### The walk` anchor; performer and what they may consult | shipped | `standards/pages/40-reviewing-a-page.md:21-28`; `:3784` |
| | 004 | yes/no steps only | shipped | `40-reviewing-a-page.md:30-35`; `:3823` |
| | 005 | four-row verdict table | shipped | `40-reviewing-a-page.md:44-49`; `:3853` |
| | 006 | empty set recorded as vacuous | shipped | `40-reviewing-a-page.md:51`; `:3881` |
| | 007 | calibrated on a two-needs specimen | shipped | `standards/pages/examples/two-needs.md` (2 declarations); `:3896` |
| | 008 | RP-40-2 paraphrase spot check run once | shipped | `40-reviewing-a-page.md:68`; `:3922`; ledger |
| | 009 | composed and in budget | shipped | shape tests `:4000`-`:4119` |
| | 010 | reachable from both router regions | shipped | `:3978` |
| governed-page-cites-the-discipline | 001 | `docs/README.md` cites RP-10-2 as visible link text | shipped | `docs/README.md:13-15` |
| | 002 | the citation sits on a governed page | shipped, via the spec's pre-decided EC-001 tie-break | the index is outside the walk (`lint_pages.rs:4390`), so the citation also sits on governed `docs/append-conditions.md:9-10` |
| | 003 | the citing page still obeys the rule it cites | shipped (EC-001) | `docs/append-conditions.md:3` holds one declaration; `lint-pages` green. `docs/README.md` carries no declaration, as the ledger records |
| | 004 | router back-link to `docs/` | shipped | `standards/pages/README.md:8` |
| | 005 | composed citation, not chrome | shipped | `docs/README.md:13-15`; no HTML or admonition |
| | 006 | persistent in every state | shipped | no `<details>` in either file |
| | 007 | relocating `standards/pages` fails the gate | shipped | `:4307`, `:4320` |
| | 008 | back-link limits stated | shipped | `README.md:79-97`; link check `:3940` |
| page-need-checker-mounted-in-the-gate | 001 | runs via `ci`, `lints` and `affected` | shipped | `main.rs:810-835`, `:1319`; `affected.rs:189` |
| | 002 | missing or empty trees fail | shipped | `:4307`, `:4320`, `:4335`, `:4348` |
| | 003 | pages root from the narrative tree's constant | shipped | `lint_pages.rs:1920`; `:4364` |
| | 004 | declaration parser: zero, one, two, mention, malformed | shipped | `:4409`-`:4488` |
| | 005 | three wrong pages rejected at the right line | shipped | `:4523`; seen live (F1, F1b) |
| | 006 | one `orientation` page per directory | shipped | `:4564`, `:4589` |
| | 007 | router index drift names `--write`; dangling link | shipped | `:4636`, `:4656`, `:4686` |
| | 008 | `NEEDS` vs. band 10, both directions | shipped | `:4734`, `:4753`, `:4770` |
| | 009 | atom drift caught at its own line | shipped | `:4788`-`:4871` |
| | 010 | sorted, composed output | shipped | `:4913`, `:4927` |
| | 011 | rules-only PR: checker runs, no package selected | shipped | `affected.rs:189`; `standards/pages/` in `INERT` (`:368`); test `affected.rs:1044` (`the_rules_tree_selects_no_package`) |
| | 012 | blind spot stated first | shipped | `:3317` |
| declaration-check-seen-to-fail | 001-008 | captures 0-4 recorded verbatim | shipped (8 of 8) | `.bklg/.../declaration-check-seen-to-fail/_ledger.md:103`, `:123`, `:160`, `:193`, `:227`; Breaks A and B re-observed at HEAD (F1b) |
| playbook-atom-staged-for-ingest | 001-011 | atom staged under `_intake`, adjudicated, not hand-authored | shipped (11 of 11), then ingested as designed | now `.kb/playbooks/making-the-need-a-page-answers-declared-singular-checkable.md` (`status: accepted`); mapped at `.kb/maps/domain-map.md:792`; `_intake/` holds only `README.md` |

---

### HS-P0022 application-author-path: 50 shipped, 4 gaps

| Story | AC | Criterion (short) | Status | Evidence |
|---|---|---|---|---|
| merge-forward-preflight | 001 | merge of `from-contract-to-published-library` recorded | shipped | `_baseline.md:25`; merge `a5c0f30` resolves and is an ancestor of `main` (GitHub compare) |
| | 002 | `ci --fast` on the merge commit | shipped | `_baseline.md:53` |
| | 003 | anchors table | shipped | `_baseline.md:84` |
| | 004 | four dispositions | shipped | `_baseline.md:122` |
| | 005 | composition baseline | shipped | `_baseline.md:146` |
| tension-resolutions | 001-005 | DT-1, DT-4, DT-5+6, anchor table, dispositions | shipped (5 of 5) | `_resolutions.md:38`, `:84`, `:178`, `:325`, `:411`; the anchors (`#where-your-streams-went`, `Try it wrong, then put it back`) are live at HEAD |
| surface-course-subscriptions | 001 | `overview.md` is the only copy | shipped | `examples/course-subscriptions/tests/reach.rs:149` |
| | 002 | page registered in the harness | shipped | `narrative.rs:149-150`; `reach.rs:182`. No doctest is collected because the page has no fences, which AC-007 requires |
| | 003 | one `orientation` declaration | shipped | `docs/read-the-worked-example.md:3`; `reach.rs:207` |
| | 004 | orientation before departure | shipped | `reach.rs:248` |
| | 005 | seam named once, before the link | shipped | `reach.rs:288` |
| | 006 | link chain holds | shipped | `reach.rs:332` |
| | 007 | budgets; no fences or affordances | shipped | `reach.rs:385` |
| boundary-refusal-encounter | 001 | step 3 prints the refusal | shipped | `docs/first-encounter.md:96-125`; `xtask/tests/first_encounter.rs` (19 pass) |
| | 002 | crate root: executing first program | shipped as amended (spec § Amendment BC-002) | `crates/happenstance/src/lib.rs:26-64` (`#[tokio::main]`); pointer to the refusal at `:66` |
| | 003 | step headers for cold arrival | shipped | `first-encounter.md:88-91`; `every_later_step_opens_on_its_two_line_header_block` |
| | 004 | tag join and `after` are both load-bearing | shipped | `crates/happenstance/tests/boundary_refusal.rs` (3 pass) |
| | 005 | no hidden line carries the boundary | shipped | `no_hidden_line_carries_any_part_of_the_boundary` |
| | 006 | registered; no opt-outs | shipped | `narrative.rs:131-132`; `docs/README.md:20` |
| | 007 | density budget on both surfaces | **GAP** | crate root still over budget: fence 35 rendered lines against 32, 70 columns against 68 (`lib.rs:60`), `#` headings of 39 and 26 characters against 22 (HS-B0001, tracked only in frozen `.bklg/support/...`). Step 3 heading `## A condition that refuses` (`first-encounter.md:86`) is 24 characters against 22, and no test checks heading length |
| | 008 | ES-25 carries the normative weight | shipped | `first-encounter.md:130`; spec-trace and narrative citation check green |
| boundary-falsification-drill | 001 | drill subsection at the bottom of step 3 | shipped | `first-encounter.md:132`; `xtask/tests/falsification_drill.rs` (9 pass) |
| | 002 | refusal assertion executed, no opt-outs | shipped | the doctest at line 96 runs: it failed at runtime under F3 |
| | 003 | the page's edit fails on the assertion | shipped | re-run at HEAD (F3): `the boundary did not hold: Ok(SequencePosition(3))` |
| | 004 | the revert restores green with no residue | shipped | F3 revert: 7 doctests pass; porcelain empty |
| | 005 | dated transcript in both directions | shipped | `_drill-observation.md:63`, `:170`, `:260` |
| | 006 | quoted failure is verbatim | shipped | `first-encounter.md:148-151` matches the live panic line. The `note: run with RUST_BACKTRACE=1` line differs here only because this host sets `RUST_BACKTRACE=1` |
| | 007 | persistent, not folded | shipped | `the_drill_is_persistent_and_introduces_nothing` |
| invariant-to-appendcondition-bridge | 001 | rule writable from the page alone | shipped | ledger tier-5 walk |
| | 002 | registered and executed | shipped | `narrative.rs:140-141`; doctests at lines 73, 122 and 164 pass |
| | 003 | `## The guard you would write` is the **first code on the page** | **GAP** (regressed) | `## Where your identifiers come from` and its fence (`docs/carry-your-invariant.md:53-105`), added 2026-09-04 in `400986d` and `cc652c8`, now sits above the guard at `:115-146` |
| | 004 | wrong guard is marked at both ends and is **never the last code on the page** | **GAP** | the WRONG fence at `carry-your-invariant.md:164-189` is the last fence on the page. `## Back to the working version` (`:210`) is prose and a link only. Design anti-pattern 7 says "the last code you see must be the correct guard" |
| | 005 | `#where-your-streams-went`; prior model named there only | shipped | `carry-your-invariant.md:17-19`; the only "which stream" hit is `:19` |
| | 006 | tag, query, fold, guard used consistently | shipped | `carry-your-invariant.md:30` |
| | 007 | declaration first | shipped | `carry-your-invariant.md:3` |
| | 008 | fences ≤ 68 columns, paragraphs ≤ 435 characters | **GAP** (regressed) | the new identifiers fence has lines of 76, 74, 70, 76 and 72 columns (`:83`, `:93`, `:97-99`). The paragraph at `:62-67` renders to about 461 characters. At ship time the page was 156 lines and within budget; it is 220 now |
| fence-inventory-and-clause-audit | 001 | fence inventory | shipped | `_inventory.md:55` |
| | 002 | `ignore` drill fails | shipped | `_inventory.md:181`; re-observed at HEAD (F7) |
| | 003 | claims resolve | shipped | `_citations.md:24`; narrative citation check green |
| | 004 | dangling id drill | shipped | `_citations.md:245`; re-observed at HEAD (F6) |
| | 005 | zero restated `MUST`s | shipped | `_citations.md:131`; no `MUST` at HEAD across `docs/`, `crates/happenstance/src/lib.rs` or `overview.md` |
| | 006 | every defect repaired in place or routed | shipped as a record | `_inventory.md:387`. But R-1, R-2 and R-5 were routed into `.bklg` and are still open at HEAD (see Gaps 4-5) |
| answered-need-and-anchor-review | 001-008 | set enumerated, walked, calibrated, anchor single | shipped (8 of 8) | `_walk.md:19`, `:79`, `:102`, `:253`, `:321`, `:428`. At HEAD: one declaration per page (`lint-pages` green), anchor `carry-your-invariant.md:17`, linked from `read-the-worked-example.md:12` |

---

### Fault injection

All of this ran in the throwaway worktree, and every injection was reverted with `git checkout -- .`. `git status --porcelain` was empty after each one.

**Caveat about the build directory.** Other worktrees on this host share `CARGO_TARGET_DIR=/home/claude/happenstance/target` and rebuild the same `xtask` binary there. `workspace_root()` is `env!("CARGO_MANIFEST_DIR")` (`xtask/src/spec_trace.rs:3353-3358`), so it is fixed at compile time. In that shared directory the binary was observed to have been built from a different worktree (`/home/claude/hs-193/xtask`), which means a run there could read another tree's `docs/`. To get a binary that reads this tree, the transcripts below use an `xtask` built into its own target directory and confirmed with `strings` to embed `/home/claude/hs-150/xtask`.

**F1: remove the `> **Answers:**` line from a page.**
```text
$ sed -i '3d' docs/append-conditions.md && cargo xtask lint-pages
  docs/append-conditions.md — no `> **Answers:**` line; see standards/pages/00-one-need.md

xtask failed: 1 problem(s) in standards/pages + docs
exit=1
$ git checkout -- . && cargo xtask lint-pages
  6 pages, 16 rules, all consistent
```

**F1b: a second declaration plus an unenumerated token, on two pages in one run.**
```text
  docs/append-conditions.md:5 — declares `explanation` and `tutorial`; a page answers one need
  docs/text-fences.md:3 — `reference` is not a need: orientation, tutorial, how-to, explanation; see standards/pages/10-the-need-set.md

xtask failed: 2 problem(s) in standards/pages + docs
exit=1
```

**F2: break a compiled `rust` fence** (`docs/first-encounter.md:22`, `MemoryEventStore::new()` changed to `MemoryEventStore::open()`).
```text
$ RUSTDOCFLAGS='-D warnings' cargo xtask narrative-doctests
  3 page(s)' examples enumerated
test xtask/src/../../docs/first-encounter.md - narrative::first_encounter (line 16) ... FAILED
error[E0599]: no associated function or constant named `open` found for struct `MemoryEventStore` in the current scope
   --> xtask/src/../../docs/first-encounter.md:22:35
...
test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 229 filtered out; finished in 0.69s
error: doctest failed, to rerun pass `-p xtask --doc`

xtask failed: the narrative tree's examples failed with exit status: 101
exit=1
```
After the revert: `test result: ok. 7 passed; 0 failed`, exit 0.

**F3: the page's own falsification drill** (`docs/first-encounter.md:113`, `Some(&condition)` changed to `None`). The fence still compiles, so this fails at runtime.
```text
test xtask/src/../../docs/first-encounter.md - narrative::first_encounter (line 96) ... FAILED
the boundary did not hold: Ok(SequencePosition(3))
test result: FAILED. 6 passed; 1 failed; ...
xtask failed: the narrative tree's examples failed with exit status: 101
exit=1
```

**F4: an `ignore` fence on the crate root** (`crates/happenstance/src/lib.rs:26`). The gate stays green:
- `cargo xtask lints` exits 0. That includes `every narrative page is checked` and `every page declares one need`.
- `cargo test -p happenstance --tests` reports no failures.

This confirms routed finding R-2 at HEAD (Gap 5).

**F5: an unregistered page** (new `docs/orphan.md` carrying a valid declaration).
```text
  xtask/src/narrative.rs — does not include orphan.md; its examples are never compiled
  xtask/src/narrative.rs — no `mod orphan`; one module per page is what keeps a doctest failure's line number relative to the page

xtask failed: 2 problem(s) in docs
exit=1
```

**F6: a dangling clause id** (`docs/first-encounter.md:130`, `[ES-25]` changed to `[ES-999]`).
```text
  docs/first-encounter.md:130 — cites `ES-999`, which SPECIFICATION.md does not define

xtask failed: 1 problem(s) in docs
exit=1
```

**F7: an `ignore` fence on a tree page** (`docs/append-conditions.md:39`, `rust` changed to `rust,ignore`).
```text
  docs/append-conditions.md:39 — an `ignore` fence needs an `IGNORE_ALLOWANCES` entry naming it; a comment above the fence does not permit it, because a comment is reviewable only in the diff that introduced it

xtask failed: 1 problem(s) in docs
exit=1
```

**F8: a `no_run` fence on a tree page** (`docs/append-conditions.md:39`, `rust` changed to `rust,no_run`).
```text
  6 pages, all consistent
exit=0
```
The checker accepts it silently. This confirms routed finding R-1 at HEAD (Gap 4).

The phase exit criterion, "the narrative tree's checks are in `cargo xtask ci`, and have been watched failing on a deliberately broken page", is met by F1, F2, F5, F6 and F7.

---

### Gaps (filed under epic #66)

**1 (#232). `carry-your-invariant.md`: the identifiers section broke the bridge's ordering and density budgets**

Covers HS-P0022 / invariant-to-appendcondition-bridge AC-003 and AC-008.

`## Where your identifiers come from` (`docs/carry-your-invariant.md:53-105`) was added on 2026-09-04 (`400986d`, `cc652c8`), after the story shipped. It has three effects:
- Its fence is now the first code on the page, ahead of `## The guard you would write` (`:115`). AC-003 and the design's §5 require the guard to come first.
- Its fence lines reach 76 columns (`:83`, `:93`, `:97-99`), over the 68-column budget.
- The paragraph at `:62-67` renders to about 461 characters, over the 435 budget.

No test guards this page's composition, unlike `xtask/tests/first_encounter.rs` for step 1. The fix is to move the section or re-wrap it, or to record a decision that amends the budget, and to add a composition test for the page.

**2 (#233). `carry-your-invariant.md`: the wrong-guard fence is the last code on the page**

Covers HS-P0022 / invariant-to-appendcondition-bridge AC-004.

AC-004 says the WRONG fence is "never the last code on the page". The design's anti-pattern 7 says "the last code you see must be the correct guard". At HEAD, the fence at `:164-189` is the page's last fence. `## Back to the working version` (`:210`) is prose plus a link back. The story's ledger accepted "last thing in reading order" instead.

Either add the correct guard's key expression as code in §7, or record a decision that the link satisfies IQ-2.3, and add a test that pins whichever is chosen.

**3 (#234). Documentation density overages on HS-P0022's surfaces (HS-B0001, never migrated to GitHub)**

Covers HS-P0022 / boundary-refusal-encounter AC-007.

The crate-root overages recorded in frozen `.bklg/support/inherited-documentation-defects/crate-root-density-overages/bug.md` still hold at HEAD:
- The landing fence is 35 rendered lines against a budget of 32.
- It is 70 columns against 68 (`crates/happenstance/src/lib.rs:60`).
- Two `#` headings run 39 and 26 characters against 22.

On top of those, step 3's heading `## A condition that refuses` (`docs/first-encounter.md:86`) is 24 characters against 22, and no test measures heading length. ADR-0088 makes GitHub Issues the tracker, so this needs a GitHub issue to have an owner.

**4 (#235). Narrative checker silently accepts `rust,no_run` fences**

Routed finding R-1 from HS-P0022's fence audit (`.bklg/.../fence-inventory-and-clause-audit/_inventory.md:387`), destination HS-P0020.

`xtask/src/lint_narrative.rs:830` treats `no_run` like `rust`. A page fence retagged `rust,no_run` compiles but stops executing, and `cargo xtask narrative` stays green (F8). That is how a false assertion on a page goes unnoticed. The module's "What this does not verify" sections do not state this limit.

Either require an `IGNORE_ALLOWANCES`-style entry for `no_run`, as `ignore` already does, or state the limit in the module docs and prove it in a test, per RS-81-1.

**5 (#236). Crate-root fence and clause citations are outside every narrative check**

Routed findings R-2 and R-5, destination HS-P0020.

`crates/happenstance/src/lib.rs` is not under `TREE`. Retagging its landing fence `ignore` leaves `cargo xtask lints` and `cargo test -p happenstance --tests` green (F4). A clause citation there would not be resolved by `lint_narrative`'s citation check, which reads only `docs/`; R-5 is taken from the record and was not injected at HEAD.

R-3, that the crate-root doctest's reported line number counts through the README concatenation, is in the same record and was **not** re-verified here.

The fix is to decide whether the checker's corpus reaches the crate root, for example a narrow fence-attribute and citation scan of `lib.rs`. If it does not, state the gap where `lint_narrative.rs` lists its limits.

---

**Suggested ticks.**
- **HS-P0020:** confirmed shipped, 79 of 79. Gaps 4 and 5 are routed findings it inherited, not failures of its own stories.
- **HS-P0021:** confirmed shipped, 81 of 81.
- **HS-P0022:** shipped apart from Gaps 1-3, so it should stay reopened until those three are filed and owned.
- **Phase 20 exit line:** "narrative checks in `cargo xtask ci`, watched failing on a deliberately broken page" can be ticked on this record.
