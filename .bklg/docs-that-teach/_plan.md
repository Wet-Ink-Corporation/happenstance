---
item: HS-I0007
stage: plan
created: 2026-08-17T20:00:00.000Z
updated: 2026-08-17T20:00:00.000Z
---

# Plan: docs-that-teach

Rollup authored by `redkiln-review` acting as planning verifier, from the 6 approved project
decompositions/story maps and the 55 produced story specs. Verified against `.bklg/docs-that-teach/`
on disk in this worktree; see `## Verification result` for method and findings.

## Coverage map

Every initiative business requirement and acceptance criterion, traced through its owning
project(s) (per `_decomposition.md`'s traceability matrix, independently re-verified) to the
specific stories that discharge it.

| ID | Requirement (abbreviated) | Project(s) | Story(ies) |
| --- | --- | --- | --- |
| BR-01 | Narrative material compiled against real crates by the gate | HS-P0020 | `pinned-narrative-tree-and-compiling-step`, `narrative-checker-mounted-with-pinned-path`, `fence-discipline-and-allowance-list` |
| BR-02 | Gate obligation observed *failing* on a deliberately broken page | HS-P0020 | `observed-failure-falsification` |
| BR-03 | Opening encounter demonstrates a boundary doing its job | HS-P0022 | `boundary-refusal-encounter`, `boundary-falsification-drill` |
| BR-04 | Every page carries a named, checkable answered-need | HS-P0021 | `need-vocabulary-and-declaration-form`, `page-need-checker-mounted-in-the-gate`, `declaration-check-seen-to-fail` |
| BR-05 | Dated friction log from a non-author, non-insider reader | HS-P0024 | `friction-log-skeleton`, `non-insider-recruitment`, `session-run-against-pinned-tree` |
| BR-06 | Log follows an auditable shape and is routed to someone who can act | HS-P0024 | `friction-log-skeleton`, `route-and-escalate` |
| BR-07 | Teaching states which prior mental model it argues against | HS-P0022 | `tension-resolutions` (DT-1) |
| BR-08 | Result is reachable where a Rust developer already looks | HS-P0023 | `front-door-pointer`, `front-door-walk-record` |
| BR-09 | Specification wins; a page cites clauses and never restates them | HS-P0021 (authoring rule + spot check) + HS-P0020 (mechanical resolution) | `reviewer-and-citation-procedures`; `narrative-citation-resolution`, `spec-trace-clause-id-accessor` |
| BR-10 | Frozen documentation MUSTs pinned by clause id before any rewrite | HS-P0020 | `frozen-documentation-must-pin` |
| BR-11 | Hidden content proven inside the checked surface, or absent | HS-P0020 (demonstration/forbid) + HS-P0021 (DT-8 rule) | `hidden-content-resolution`; `fold-line-rule` |
| BR-12 | Discipline's home decided; gate-read tree pinned by path | HS-P0021 (where it lands) + HS-P0020 (pinning by path) | `router-precedence-and-announcement`; `pinned-narrative-tree-and-compiling-step`, `narrative-checker-mounted-with-pinned-path` |
| BR-13 | Audience model reconciled with the staged persona work | HS-P0025 | `reconciliation-ledger`, `charter-open-question-disposition` |
| BR-14 | Comprehension claim scoped to what the method supports | HS-P0024 | `scope-the-claim` |
| BR-15 | Adapter author meets the explanation where the problem fires | HS-P0023 | `store-error-site-rewrite`, `adapter-reasoning-account` |
| BR-16 | Evaluator's second question has somewhere to go | HS-P0023 | `evaluator-onward-links`, `second-question-walk-records` |
| BR-17 | Personas promoted into the durable product layer at closeout | HS-P0025 | `staged-audience-payload`, `audience-ingest-wave`, `product-layer-mounting` |
| BR-18 | Whether the shift is taught with a picture is decided | HS-P0022 | `tension-resolutions` (DT-5/DT-6) |
| AC-01 | First program teaches what the library is for | HS-P0022 | `tension-resolutions`, `boundary-refusal-encounter`, `boundary-falsification-drill` |
| AC-02 | Application author can carry their own invariant across | HS-P0022 | `invariant-to-appendcondition-bridge` (built on `tension-resolutions`) |
| AC-03 | Reader is not taught something the library no longer does | HS-P0020 | `pinned-narrative-tree-and-compiling-step`, `narrative-checker-mounted-with-pinned-path`, `fence-discipline-and-allowance-list`, `narrative-tree-story-grain-selection`, `observed-failure-falsification` |
| AC-04 | Adapter author meets the explanation where the problem finds them | HS-P0023 | `store-error-site-rewrite`, `adapter-reasoning-account`, `error-site-walk-record` |
| AC-05 | Adapter author can follow the reasoning, not just the recipe | HS-P0023 | `adapter-reasoning-account` |
| AC-06 | Evaluator's second question has somewhere to go | HS-P0023 | `pointer-policy-and-inventory`, `evaluator-onward-links`, `second-question-walk-records` |
| AC-07 | Any reader can tell what a page is for before reading it | HS-P0021 | `need-vocabulary-and-declaration-form`, `page-need-checker-mounted-in-the-gate`, `declaration-check-seen-to-fail` |
| AC-08 | Reader arriving with the wrong prior model is met | HS-P0022 | `tension-resolutions`, `answered-need-and-anchor-review` |
| AC-09 | Team can see where teaching failed a real person | HS-P0024 | `dt9-and-fixed-protocol`, `friction-log-skeleton`, `non-insider-recruitment`, `session-run-against-pinned-tree` |
| AC-10 | Someone acted on what that reader found | HS-P0024 | `disposition-every-stumble`, `route-and-escalate`, `content-fixes-from-dispositions` |
| AC-11 | Reader looking for the teaching finds it from where they already are | HS-P0023 | `pointer-policy-and-inventory`, `front-door-pointer`, `front-door-walk-record` |
| AC-12 | Specification stays the single normative voice | HS-P0021 (page defers) + HS-P0020 (citation resolves) | `reviewer-and-citation-procedures`; `narrative-citation-resolution`, `spec-trace-clause-id-accessor` |
| AC-13 | Next author is not starting from scratch | HS-P0021 | `governed-page-cites-the-discipline`, `playbook-atom-staged-for-ingest` |
| AC-14 | Next initiative inherits this one's audience | HS-P0025 (+ HS-P0024's handoff) | `staged-audience-payload`, `audience-ingest-wave`, `product-layer-mounting`, `reconciliation-ledger`, `charter-open-question-disposition`; `handoff-note-to-closeout` |

**No uncovered ids.** All 18 business requirements, all 14 acceptance criteria, all 15
Definition-of-Done scenarios and all 10 design tensions carry a named project owner in
`_decomposition.md`'s own traceability matrix (independently re-checked against the 6 projects'
`project.md` acceptance-criteria sections and against the story index's `tracesTo` fields; no
discrepancy found). Design-tension ownership (`_decomposition.md`, `## Design tension ownership`)
and Definition-of-Done ownership are reproduced there and are not re-derived here.

## Project index

"Briefs present" reports what was verified **on disk** in this worktree (file existence and
non-stub content, not what a digest or brief claimed). All six projects carry `project.md`,
`_decomposition.md`, `_grounding.md`, `_storymap.md` and `_design.md`, each substantive
(no file under 90 lines; none is the unauthored template skeleton). All six are `status:
in-review`, `stage: design` in their own frontmatter, i.e. approved through the design gate and
awaiting this plan rollup.

| Project | Objective | Briefs present (verified on disk) | Stories | Status |
| --- | --- | --- | --- | --- |
| HS-P0020 `checked-documentation-surface` | The substrate every other project consumes: pin the narrative tree by path, wire its build/render into `cargo xtask ci` as an ordinary `REQUIRED` step, compile every fence against the real crates, and prove the check by watching it fail. Carries BR-10's clause-id pin. | project.md (350L), `_decomposition.md` (843L, architecture/ux/testing/deployment briefs), `_grounding.md` (263L), `_storymap.md` (174L), `_design.md` (786L, signed off 2026-08-17, 6 surfaces) | 10 | in-review / design |
| HS-P0021 `page-need-discipline` | Make "which need does this page answer" a labelled, reviewer-checkable property, and write the discipline down where it durably binds (`standards/pages/`). | project.md (381L), `_decomposition.md` (1038L, architecture/ux/testing briefs), `_grounding.md` (313L), `_storymap.md` (156L), `_design.md` (730L, 5 surfaces) | 8 | in-review / design |
| HS-P0022 `application-author-path` | Persona-1 vertical slice: settle which prior mental model the teaching argues against, build the bridge from it, and stage an opening encounter in which a consistency boundary actually refuses an append. | project.md (415L), `_decomposition.md` (650L, ux/testing briefs), `_grounding.md` (211L), `_storymap.md` (163L), `_design.md` (1064L, 4 surfaces, signed off with density-budget corrections) | 8 | in-review / design |
| HS-P0023 `reach-and-adapter-path` | Reachability and the persona-2 slice together (one pointer policy, DT-10): the front door into the narrative material, the evaluator's second question, and the adapter author's error meeting its explanation at the site it fires. | project.md (357L), `_decomposition.md` (792L, architecture/ux briefs), `_grounding.md` (211L), `_storymap.md` (147L), `_design.md` (768L, 5 surfaces) | 8 | in-review / design |
| HS-P0024 `comprehension-evidence` | The non-substitutable second instrument: a genuine non-author, non-insider reader walks the assembled material; what stopped them is recorded in an auditable shape and every stumble is dispositioned. | project.md (372L), `_decomposition.md` (513L, ux/testing briefs), `_grounding.md` (228L), `_storymap.md` (195L), `_design.md` (116L, explicitly no rendered surface / no public API) | 10 | in-review / design |
| HS-P0025 `durable-audience-closeout` | Terminal DoD owner: reconcile the audience with the separately staged persona set, promote it into `.kb/product/` through the ingest path, and re-observe all fifteen initiative DoD scenarios on the assembled tree from a clean checkout. | project.md (300L), `_decomposition.md` (909L, architecture/ux/testing briefs), `_grounding.md` (189L), `_storymap.md` (164L), `_design.md` (98L, explicitly no rendered surface) | 11 | in-review / design |

Total: 55 stories across 6 projects. `HS-P0025` is the only project created `--terminal`; all
others are `--no-terminal` and held to `cargo xtask ci --fast` / `cargo xtask affected --base main`
rather than the whole-initiative gate.

## Story index

| Story | Project | One-line | depends_on | Traces to |
| --- | --- | --- | --- | --- |
| `pinned-narrative-tree-and-compiling-step` | HS-P0020 | Create the narrative tree at its decided path with a real fixture page, compile every Rust fence as one `#[cfg(doctest)] mod` per page, wire that compile as a `REQUIRED` step with `probe: None`, `--locked`, step-scoped `RUSTDOCFLAGS`. | — | AC-002, AC-009 |
| `narrative-tree-story-grain-selection` | HS-P0020 | Extend `affected.rs`'s selection arm so a narrative-tree-only change selects `xtask`, both directions unit-tested. | `pinned-narrative-tree-and-compiling-step` | AC-009 |
| `narrative-checker-mounted-with-pinned-path` | HS-P0020 | Bin-crate checker module: tree/harness path `const`s, error by name on missing tree, `bail!` on empty, bidirectional page-harness registration, mounted as its own `REQUIRED` step/subcommand/help/`lint_steps`. | `pinned-narrative-tree-and-compiling-step` | AC-001, AC-005 |
| `fence-discipline-and-allowance-list` | HS-P0020 | Walk every fence: reject untagged, enumerate info-string parts exhaustively, reject `ignore`-class fences unless on an enumerated allowance `const`, reverse-sweep the allowance list. | `narrative-checker-mounted-with-pinned-path` | AC-004 |
| `hidden-content-resolution` | HS-P0020 | Close DT-7 and enforce it in the same fence walk — hidden-content markers rejected mechanically (forbid branch chosen; falsification branch not built). | `fence-discipline-and-allowance-list` | AC-006 |
| `spec-trace-clause-id-accessor` | HS-P0020 | Add `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>` beside `all_rules` in `spec_trace.rs`, built from `parse_clauses`/`SECTIONS`. | — | AC-007, AC-008 |
| `narrative-citation-resolution` | HS-P0020 | Parse clause ids a narrative page cites, resolve via `clause_ids`, report dangling ids naming the page. | `narrative-checker-mounted-with-pinned-path`, `spec-trace-clause-id-accessor` | AC-007 |
| `frozen-documentation-must-pin` | HS-P0020 | Re-derive the frozen documentation MUSTs by clause id in one commented `const`, check ids resolve, sites still contain the anchor, derived count matches hand-written count. | `narrative-checker-mounted-with-pinned-path`, `spec-trace-clause-id-accessor` | AC-008 |
| `observed-failure-falsification` | HS-P0020 | Break one claim in the fixture page, run the gate, record the failure verbatim, revert, record green — both halves. | `narrative-checker-mounted-with-pinned-path`, `pinned-narrative-tree-and-compiling-step` | AC-003 |
| `documented-blind-spots-and-their-proofs` | HS-P0020 | "What this does not verify" section first in the new modules, re-run the `RUSTDOCFLAGS` probe, walk a `text`-tagged broken fixture through the gate to prove that limit is real. | `fence-discipline-and-allowance-list`, `frozen-documentation-must-pin`, `narrative-citation-resolution`, `observed-failure-falsification` | AC-010 |
| `need-vocabulary-and-declaration-form` | HS-P0021 | Land the closed need set once as the `NEEDS` const and as the two rule atoms documenting it, with DT-2/DT-3/DR-05's resolutions. | — | AC-003, AC-004 |
| `router-precedence-and-announcement` | HS-P0021 | Stand `standards/pages/` up sibling to `standards/rust/`, router with trigger table + index, rank in the precedence chain without editing it, row in `docs/README.md`. | `need-vocabulary-and-declaration-form` | AC-001, AC-002, AC-011 |
| `fold-line-rule` | HS-P0021 | Resolve DT-8: a rule a reviewer applies without the author, covering the declaration itself as never occludable. | `router-precedence-and-announcement` | AC-005 |
| `reviewer-and-citation-procedures` | HS-P0021 | Cite-never-restate rule, the non-author verdict walk (DR-07), the paraphrase spot check — each run once over the set as it stands. | `router-precedence-and-announcement` | AC-009, AC-010 |
| `page-need-checker-mounted-in-the-gate` | HS-P0021 | Working check: declaration parser, zero/two/unenumerated rejections, orientation ceiling, both trees' guards, router's generated index region; mounted as an ordinary gate step. | `need-vocabulary-and-declaration-form`, `router-precedence-and-announcement`, `fold-line-rule` | AC-001, AC-003, AC-004, AC-006, AC-008 |
| `declaration-check-seen-to-fail` | HS-P0021 | Break a governed page two ways, watch `cargo xtask ci` fail by file/line each time, revert, record a clean recovery. | `page-need-checker-mounted-in-the-gate` | AC-007 |
| `governed-page-cites-the-discipline` | HS-P0021 | At least one page in HS-P0020's tree names the discipline as the reason it is shaped as it is, resolving link both ways. | `router-precedence-and-announcement`, `page-need-checker-mounted-in-the-gate` | AC-011 |
| `playbook-atom-staged-for-ingest` | HS-P0021 | Stage a `kind: playbook`, `authority_tier: guideline` atom under `.kb/_intake/` with method, rejected alternatives, stop-holding conditions. | `fold-line-rule`, `reviewer-and-citation-procedures`, `declaration-check-seen-to-fail` | AC-012 |
| `pointer-policy-and-inventory` | HS-P0023 | Land DT-10's resolution as substrate: permitted pointer forms, enumerated inventory with a named rot-guard per row, widget-rejection record. | — | AC-001, AC-003, AC-011 |
| `adapter-reasoning-account` | HS-P0023 | Author the sequenced adapter reasoning account as a registered page, six existing sources in order, anchored citations, `MemoryEventStore`-is-not-an-adapter caveat. | `pointer-policy-and-inventory` | AC-008, AC-012 |
| `store-error-site-rewrite` | HS-P0023 | Rewrite `store.rs`'s module doc: restore rustc's real `error[E0034]` block, state the narrow unchecked limit, in-place fix first, one guarded pointer to the reasoning account. | `pointer-policy-and-inventory`, `adapter-reasoning-account` | AC-003, AC-006, AC-007, AC-010, AC-012 |
| `error-site-walk-record` | HS-P0023 | A non-author reproduces the E0034 collision and reaches the reasoning account keyboard-only; dated record naming each hop. | `adapter-reasoning-account`, `store-error-site-rewrite` | AC-009 |
| `front-door-pointer` | HS-P0023 | Install DT-10's pointer on both front-door surfaces (crate root, README), above the fold, displacing nothing; register both rows in the inventory. | `pointer-policy-and-inventory` | AC-002, AC-003 |
| `evaluator-onward-links` | HS-P0023 | Name the two second questions from DR-4's stall points and make each one hop from the first-question page, using only intra-doc links/`#[doc(alias)]`/search/TOC. | `pointer-policy-and-inventory` | AC-003, AC-005, AC-012 |
| `front-door-walk-record` | HS-P0023 | Someone who did not install the pointer starts from `cargo add happenstance` and reaches the narrative material keyboard-only; dated record. | `front-door-pointer` | AC-004 |
| `second-question-walk-records` | HS-P0023 | Each named second question walked keyboard-only from the first-question page to its answer; dead ends recorded as failed walks. | `front-door-pointer`, `evaluator-onward-links` | AC-005 |
| `dt9-and-fixed-protocol` | HS-P0024 | Resolve DT-9 (which persona, in terms of intent, with rejected personas and why) and fix scenario/narration mode/severity scale/disqualifying criteria in the same commit. | — | AC-001, AC-002 |
| `friction-log-skeleton` | HS-P0024 | Land the auditable-shape log scaffold: scenario; logger identity/context/date; append-only chronological record with stable stumble ids; severity token; disposition slot with revision block; scope/hand-off sections. | `dt9-and-fixed-protocol` | AC-004 |
| `non-insider-recruitment` | HS-P0024 | Apply the written criteria to a real candidate, record declaration/platform/toolchain/AT, including the ineligible-but-used-anyway state as a stated failure. | `dt9-and-fixed-protocol`, `friction-log-skeleton` | AC-003 |
| `session-run-against-pinned-tree` | HS-P0024 | Run the session once against the post-merge assembled tree; capture concurrently, severity inline, facilitator interventions timestamped, keyboard reach recorded, abandonment a valid end state. | `friction-log-skeleton`, `non-insider-recruitment` | AC-004, AC-005 |
| `disposition-every-stumble` | HS-P0024 | Every severity-marked item gets exactly one disposition (fixed/accepted/routed), readable at the stumble, later changes as a revision beside the original. | `session-run-against-pinned-tree` | AC-006 |
| `route-and-escalate` | HS-P0024 | Submit the log to a named owner and record it; every routed item gets a destination id that resolves; sibling-tension reopenings recorded as escalations carrying the DT id. | `disposition-every-stumble` | AC-007, AC-011 |
| `content-fixes-from-dispositions` | HS-P0024 | Land the small fixed content fixes from the existing primitive layer, proven through `cargo xtask affected` and `cargo xtask ci --fast`. | `disposition-every-stumble` | AC-006 |
| `scope-the-claim` | HS-P0024 | State the narrow claim (real stumbles captured and traceable) wherever this evidence is summarised; assert nothing about exhaustiveness. | `session-run-against-pinned-tree` | AC-008 |
| `second-session-decision` | HS-P0024 | Assess whether findings were dominated by one blocking defect; record the verdict (second session run, or declined for a stated reason). | `disposition-every-stumble` | AC-010 |
| `handoff-note-to-closeout` | HS-P0024 | Write the hand-off section: one directly observed persona, date, tree walked, scope sentence, in a form HS-P0025 can lift. | `route-and-escalate`, `scope-the-claim`, `second-session-decision` | AC-008, AC-009 |
| `merge-forward-baseline` | HS-P0025 | Merge `initiative/from-contract-to-published-library` forward and record the resulting sha as the tree every downstream record cites. | — | AC-012 |
| `post-merge-clause-completeness` | HS-P0025 | Re-run `cargo xtask spec-trace` on the merged tree; write the set-difference statement on any added documentation MUST. | `merge-forward-baseline` | AC-013, AC-012 |
| `reconciliation-ledger` | HS-P0025 | Complete pair-by-pair audience reconciliation record: one row per persona/journey on both sides, merged/superseded/kept-distinct with a named reason, stating which merge-order case held. | `merge-forward-baseline` | AC-001, AC-002, AC-012 |
| `charter-open-question-disposition` | HS-P0025 | Dispose of every charter open question (answered or staged as an `open_question` atom); settle the evaluator question once. | `reconciliation-ledger` | AC-017, AC-003 |
| `design-tension-audit` | HS-P0025 | DT-1 through DT-10 audit table over the merged tree: owning project, resolution or deferral, or an explicit `gap` row. | `merge-forward-baseline` | AC-018 |
| `staged-audience-payload` | HS-P0025 | Author staged persona/journey documents under `.kb/_intake/`: four slots per persona, evaluator decision transcribed, evidence qualifications adjacent, persona HS-P0024 walked named. | `reconciliation-ledger`, `charter-open-question-disposition` | AC-003, AC-006, AC-007 |
| `audience-ingest-wave` | HS-P0025 | Single closeout kb-ingest wave, explicit file list excluding `.kb/_intake/README.md`; persona atoms `kind: concept`, journey atoms `kind: playbook`, both `authority_tier: product`, plus HS-P0021's carried playbook payload. | `staged-audience-payload` | AC-004, AC-005, AC-008, AC-009, AC-011 |
| `product-layer-mounting` | HS-P0025 | Mount every landed atom at all four points: `domain-map.md` appended section, reciprocal edges, `links.kb` via record-links, Knowledge Harvest row; record the two-hop reachability walk. | `audience-ingest-wave` | AC-010, AC-011 |
| `dod-scenario-ledger` | HS-P0025 | Fresh checkout off the merged branch; re-observe all fifteen DoD scenarios, recording observer, checkout path, sha and what was seen. | `product-layer-mounting`, `post-merge-clause-completeness` | AC-014 |
| `scenario-two-fault-injection` | HS-P0025 | Re-observe DoD scenario 2 in both halves: break the pinned page, capture failing-by-name output, revert, re-run green; break never a committed state. | `dod-scenario-ledger` | AC-015, AC-014 |
| `terminal-gate-run` | HS-P0025 | Run `cargo xtask ci`, the terminal e2e grain, last, on the tree carrying atoms, maps, `links.kb`, reconciliation, DT audit and DoD ledger; record green. | `scenario-two-fault-injection`, `product-layer-mounting` | AC-016 |
| `merge-forward-preflight` | HS-P0022 | Merge `initiative/from-contract-to-published-library` forward so every page in this project is authored against the merged `crates/happenstance/src/lib.rs`. | — | AC-014 |
| `tension-resolutions` | HS-P0022 | Author `_design.md` with four resolutions: DT-1, DT-4, and the joint DT-5+DT-6 resolution, as the single citable home every later page points at. | `merge-forward-preflight` | AC-001, AC-002, AC-003 |
| `boundary-refusal-encounter` | HS-P0022 | Author the opening encounter in DT-4's resolved shape: a reader's own program output carries `AppendError::ConditionViolated` (ES-25) against a real `MemoryEventStore`. | `merge-forward-preflight`, `tension-resolutions` | AC-002, AC-004, AC-006 |
| `boundary-falsification-drill` | HS-P0022 | Make the boundary load-bearing: an executed check the tests step already sweeps fails when the query the `AppendCondition` is built from is removed; exact edit, failure and revert on the page. | `boundary-refusal-encounter` | AC-005 |
| `invariant-to-appendcondition-bridge` | HS-P0022 | Bridge material: cross-entity invariant in event-sourcing vocabulary carried to `Query`, `QueryItem`, `Tags`, a fold and `AppendCondition`, no step requiring the spec or source, DT-5/DT-6's shift material as compiled code. | `boundary-refusal-encounter`, `tension-resolutions` | AC-008, AC-013 |
| `surface-course-subscriptions` | HS-P0022 | Bring the worked example within reach: overview.md extracted once and included back verbatim, worked-example-handoff page with orientation prose and the DT-1 anchor citation. | `invariant-to-appendcondition-bridge` | AC-010 |
| `fence-inventory-and-clause-audit` | HS-P0022 | Inventory of every fenced block this project authored, zero opted out or each exception named against the design's exemption and the allowance list; audit every normative claim as a resolving clause citation. | `boundary-refusal-encounter`, `boundary-falsification-drill`, `invariant-to-appendcondition-bridge`, `surface-course-subscriptions` | AC-007, AC-011 |
| `answered-need-and-anchor-review` | HS-P0022 | Non-author reviewer walks the whole page set: exactly one named answered-need per page, DT-1 anchor decision applied identically everywhere. | `boundary-refusal-encounter`, `boundary-falsification-drill`, `invariant-to-appendcondition-bridge`, `surface-course-subscriptions`, `tension-resolutions` | AC-009, AC-012 |

Every project's `_storymap.md` Slices depends_on column was read in full across all 55 rows and
matches the story index's dependsOn field exactly, with no discrepancy. Eleven of 55 stories'
own spec.md Dependencies sections were read in full and matched their depends_on row exactly,
including cross-project soft preconditions explicitly distinguished from depends_on edges (for
example, page-need-checker-mounted-in-the-gate notes that HS-P0020's narrative.rs and TREE const
are a hard precondition per EC-005 but not a depends_on edge, because they precede via project
merge order rather than a story-level edge).

## Merge order

Acyclic: **true**. Reproduced verbatim from the computed result (Kahn's algorithm over the 55
story-level `depends_on` edges, ties broken by story-map seed order); not re-derived here.

### Wave 1

- `checked-documentation-surface/pinned-narrative-tree-and-compiling-step`
- `checked-documentation-surface/spec-trace-clause-id-accessor`
- `page-need-discipline/need-vocabulary-and-declaration-form`
- `reach-and-adapter-path/pointer-policy-and-inventory`
- `comprehension-evidence/dt9-and-fixed-protocol`
- `durable-audience-closeout/merge-forward-baseline`
- `application-author-path/merge-forward-preflight`

### Wave 2

- `checked-documentation-surface/narrative-tree-story-grain-selection`
- `checked-documentation-surface/narrative-checker-mounted-with-pinned-path`
- `page-need-discipline/router-precedence-and-announcement`
- `reach-and-adapter-path/adapter-reasoning-account`
- `reach-and-adapter-path/front-door-pointer`
- `reach-and-adapter-path/evaluator-onward-links`
- `comprehension-evidence/friction-log-skeleton`
- `durable-audience-closeout/post-merge-clause-completeness`
- `durable-audience-closeout/reconciliation-ledger`
- `durable-audience-closeout/design-tension-audit`
- `application-author-path/tension-resolutions`

### Wave 3

- `checked-documentation-surface/fence-discipline-and-allowance-list`
- `checked-documentation-surface/narrative-citation-resolution`
- `checked-documentation-surface/frozen-documentation-must-pin`
- `checked-documentation-surface/observed-failure-falsification`
- `page-need-discipline/fold-line-rule`
- `page-need-discipline/reviewer-and-citation-procedures`
- `reach-and-adapter-path/store-error-site-rewrite`
- `reach-and-adapter-path/front-door-walk-record`
- `reach-and-adapter-path/second-question-walk-records`
- `comprehension-evidence/non-insider-recruitment`
- `durable-audience-closeout/charter-open-question-disposition`
- `application-author-path/boundary-refusal-encounter`

### Wave 4

- `checked-documentation-surface/hidden-content-resolution`
- `checked-documentation-surface/documented-blind-spots-and-their-proofs`
- `page-need-discipline/page-need-checker-mounted-in-the-gate`
- `reach-and-adapter-path/error-site-walk-record`
- `comprehension-evidence/session-run-against-pinned-tree`
- `durable-audience-closeout/staged-audience-payload`
- `application-author-path/boundary-falsification-drill`
- `application-author-path/invariant-to-appendcondition-bridge`

### Wave 5

- `page-need-discipline/declaration-check-seen-to-fail`
- `page-need-discipline/governed-page-cites-the-discipline`
- `comprehension-evidence/disposition-every-stumble`
- `comprehension-evidence/scope-the-claim`
- `durable-audience-closeout/audience-ingest-wave`
- `application-author-path/surface-course-subscriptions`

### Wave 6

- `page-need-discipline/playbook-atom-staged-for-ingest`
- `comprehension-evidence/route-and-escalate`
- `comprehension-evidence/content-fixes-from-dispositions`
- `comprehension-evidence/second-session-decision`
- `durable-audience-closeout/product-layer-mounting`
- `application-author-path/fence-inventory-and-clause-audit`
- `application-author-path/answered-need-and-anchor-review`

### Wave 7

- `comprehension-evidence/handoff-note-to-closeout`
- `durable-audience-closeout/dod-scenario-ledger`

### Wave 8

- `durable-audience-closeout/scenario-two-fault-injection`

### Wave 9

- `durable-audience-closeout/terminal-gate-run`

## Open risks and unresolved cuts

- **`narrative-scoped-page` was a declared surface nobody built — now struck, resolved.**
  `checked-documentation-surface/_design.md`'s `## Surfaces` manifest previously listed six
  surfaces including `narrative-scoped-page` (the "one page per scope" state DT-7's clause (c)
  describes for content past the 3-scope/25-line threshold), claimed by no story's Integration
  contract "Renders surfaces" line; eight stories across the project explicitly noted it as "not
  rendered/changed" by them. Resolved by striking it from the manifest (`_design.md`, `##
  Surfaces`, now five surfaces) with a note that D2's resolution of DT-7 still specifies the
  pattern in prose for a future content author who exceeds the threshold — the fixture page this
  project ships stays inside it by construction — and that the manifest's job is to list surfaces
  this project actually built a path to, not every pattern DT-7 resolved. The reference-captures
  table was corrected to match. **The story-spec sweep was completed after the run, by the
  orchestrating command**, and the two stories it actually touched are
  `narrative-tree-story-grain-selection` (`spec.md:159`) and
  `documented-blind-spots-and-their-proofs` (`spec.md:197`) — not the pair this section first
  named. The workflow's own re-verification pass caught that its fix pass had corrected one of
  two occurrences in `narrative-tree-story-grain-selection` and had never touched
  `documented-blind-spots-and-their-proofs` at all; both Renders-surfaces lines now read "five".
  A sweep of every `spec.md` in the initiative confirms no stale surface count survives. No story
  merge is reordered by this fix — it is a design-record correction, not a new dependency edge.
- **The cross-branch merge is a standing precondition three projects share.** `application-author-path`,
  `durable-audience-closeout` and (transitively, via HS-P0022) the rest of the DAG all begin with a
  merge-forward story (`merge-forward-preflight`, `merge-forward-baseline`) that pulls
  `initiative/from-contract-to-published-library` (210 commits, ~24k insertions, unmerged as of
  this plan) into this branch. Every story downstream of wave 1 in those two projects is
  authored against text (`crates/happenstance/src/lib.rs`, `spec/SPECIFICATION.md`) that does not
  exist in this worktree yet. The decomposition's own risk table already carries this
  ("This initiative's file touches collide with the publication project's on the same pages") and
  the merge-order/DAG already sequences the two merge stories first in their projects; it is
  recorded here as a live risk rather than a defect, because nothing in the plan can make the
  sibling branch land sooner.
- **DT-9's persona choice is genuinely still open at plan time.** `comprehension-evidence/dt9-and-fixed-protocol`
  is wave 1 and foundational to the entire project (all 9 other stories in HS-P0024 depend on it
  transitively); its own spec is explicit that the persona choice is not pre-empted anywhere
  upstream. A wrong or late choice here re-costs the whole project's sequencing, and Persona 2 (the
  adapter author) is flagged in the story's own context pack as both the hardest to recruit and
  the thinnest-evidence choice if selected.
- **The friction-log session runs once, by design, against a cost/evidence trade already accepted
  at the initiative's charter.** `second-session-decision` (wave 6) is where a second run gets
  decided, not guaranteed; if HS-P0024's single session is dominated by one blocking defect, that
  decision reopens scheduling for a project already mid-DAG. This is the initiative's own accepted
  risk (BR-14, the charter's "One comprehension session is enough" assumption), not a new one
  found in planning.
- **HS-P0025's closure rule for `terminal-gate-run` (commit `S` vs. evidence commit `S+1`) is
  subtle and worth flagging for implementation, not for re-design.** The story's own spec states
  the rule precisely (no gate step reads anything under `.bklg/`, so the evidence commit's diff
  being confined to the project's own backlog folder is sufficient proof); it is recorded here
  because it is the one place in the whole plan where "the tree the gate was run on" and "the tree
  that documents having run it" are provably different commits, and a careless re-implementation
  could try to make them the same commit and get the paradox wrong.
- **Not re-opened, and correctly so:** `references/seeds/measured-not-claimed.md` and
  `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` are
  cited by three stories (`charter-open-question-disposition`, `reconciliation-ledger`) as paths
  that do not resolve in this worktree today; each citation states that fact explicitly and defers
  the real check to the merged tree. This is the initiative charter's own instruction ("must not
  link them as if they were reachable here") correctly carried into the story specs, not a defect.
- **Stage B-2 must be re-run to recompute the merge order.** This pass's fixes were applied
  directly to artifact bodies (`_design.md`'s Surfaces manifest, and two story specs' own "six
  surfaces" text) rather than through `redkiln new`/`redkiln advance`, and the `## Merge order`
  section above is reproduced verbatim from the order this run was launched with — it is a
  function of the story map seed Stage B-2 computed, not re-derived here, so no fix made in this
  pass (nor any future fix that adds or edits a `depends_on` edge) changes it until Stage B-2
  actually runs again. Treat the order above as the last known-good computation, not as
  re-validated against the artifacts as they now stand.

## Verification result

**Result: FAIL at time of review; the one recorded gap has since been fixed in this pass** (see
`## Gaps` below — everything else checked clean). Method and findings as originally recorded
below.

**1. Traceability.** All 18 BRs, all 14 ACs, all 15 DoD scenarios and all 10 DTs carry a named
project owner in `_decomposition.md`'s traceability matrix; independently re-checked against each
project's own `project.md` acceptance-criteria section (all six read in full) and against the
55-story index's `tracesTo` fields. **No uncovered ids.**

**2. MECE.** The three BRs `_decomposition.md` splits by responsibility (BR-09, BR-11, BR-12) each
state the seam in both owners' non-goals; no other id is double-owned, and no project traces to
nothing. Confirmed by reading the "Why this cut, and not the charter's" and "MECE confirmation"
sections of `_decomposition.md` and cross-checking against the six projects' own acceptance
criteria.

**3. DAG and merge order.** Acyclic per the computed result. All 55 stories' `_storymap.md` Slices
depends_on column (all six project story maps read in full) matches the computed graph's edges
exactly. 11 of 55 stories' own spec.md Dependencies sections were read in full and matched
their depends_on row exactly, including explicit distinctions between hard depends_on edges
and cross-project soft or also-consumed preconditions, never conflated with a depends_on
edge in any sampled story. This plan's Merge order section above reproduces the nine computed
waves verbatim, in order, with no re-derivation.

**4. Dead anchors.** Mechanically checked: every markdown link across all 271 markdown files
under `.bklg/docs-that-teach/` (497 relative links) was resolved against the citing file's own
directory and tested for existence. One unresolved path was found: `_friction-log.md` cited
(relatively, as `../_friction-log.md`) from `comprehension-evidence/content-fixes-from-dispositions/spec.md`
— a legitimate forward reference to an artefact `friction-log-skeleton` (wave 2) creates and
`session-run-against-pinned-tree` (wave 4) fills, both of which precede
`content-fixes-from-dispositions` (wave 6) in the computed merge order; not a defect. Separately,
every backticked absolute repo-path citation across all spec/project/design/grounding/storymap
files was extracted and tested; every apparent miss resolved to the same category (a path this
story or a sibling creates later in the DAG, e.g. `docs/append-conditions.md`,
`standards/pages/*.md`, `xtask/src/lint_narrative.rs`, or the staged intake atom) or to the two
paths the initiative charter itself flags as unresolvable pre-merge
(`references/seeds/measured-not-claimed.md` and the sibling branch's own distillation file), both
cited with that caveat stated inline everywhere they appear. No genuine dead anchor.

**4b. Artifact postcondition.** All six project directories were confirmed on disk to contain
project.md, _decomposition.md, _grounding.md and _storymap.md (plus _design.md and
_intake-brief.md, not required by the check but present), each read or line-counted and
confirmed substantive (90 to 1064 lines; none is the unauthored template skeleton). No absence to
report.

**5. AC ids and Dependencies sections.** All 55 spec.md files carry a Dependencies heading
(mechanically confirmed via full-corpus grep) and at least one AC-row (mechanically confirmed;
zero files failed the pattern match).

**6-7. Distillation faithfulness and signposted, AC-bound anchors.** Nine story specs were read in
full or in substantial part across five of the six projects (checked-documentation-surface:
pinned-narrative-tree-and-compiling-step, hidden-content-resolution; application-author-path:
boundary-refusal-encounter, answered-need-and-anchor-review's Dependencies; page-need-discipline:
page-need-checker-mounted-in-the-gate's Dependencies, documented-blind-spots-and-their-proofs'
Dependencies; reach-and-adapter-path: front-door-pointer's composition binding; comprehension-evidence:
dt9-and-fixed-protocol; durable-audience-closeout: terminal-gate-run, audience-ingest-wave's
Dependencies). Every Context pack sampled states load-bearing decisions inline (design trade-offs,
mount points, the exact wrong implementation each rule rejects, the persona-journey slice) rather
than pointing at a reading list; none flattens intent (for example, terminal-gate-run's context
pack states the closure-rule paradox and its resolution explicitly rather than asserting "record
the gate run"). Every Anchors table sampled carries a why / when-to-open / serves-AC row per
anchor, and the bound AC in each case matched the anchor's actual content on inspection. No hollow
or wrongly-bound anchor found in the sample. This is a sample, not an exhaustive read of all 55
specs (see caveat below).

**8. Design coverage.** Two of six projects (comprehension-evidence, durable-audience-closeout)
declare no rendered surface in _design.md (explicit "N/A, no user-facing surface"); the check is
vacuous for both, correctly. Of the remaining four: page-need-discipline (5 surfaces),
reach-and-adapter-path (5 surfaces) and application-author-path (4 surfaces) have every declared
surface id claimed by at least one story's Renders-surfaces line, all cross-checked against each
project's _design.md surface manifest. **checked-documentation-surface previously declared six
surfaces, one of which — narrative-scoped-page — was claimed by no story anywhere in the
initiative. Fixed in this pass**: `_design.md`'s `## Surfaces` manifest now declares five
surfaces (`narrative-scoped-page` struck, with a note that D2's DT-7 resolution still specifies
the pattern in prose for a future content author past the threshold), the reference-captures
table row and the mock-file description were corrected to match, and the two stories whose own
text asserted "six surfaces" (`narrative-tree-story-grain-selection`,
`spec-trace-clause-id-accessor`) were corrected to "five". Every remaining declared surface in
checked-documentation-surface's manifest is now claimed by at least one story's Renders-surfaces
line. Recorded in Open risks above and in Gaps below as resolved.

**9. Composition ACs as real AC rows.** Every Interaction-quality section sampled (5 stories
across 3 projects) binds every state and composition invariant to an explicit AC row in the
same story's acceptance-criteria table, in a Carried-by or inline dash-AC form, not as a bare
prose bullet. No story sampled left a composition invariant unbound to an AC row.

**10. Scope.** The porcelain status scoped to .bklg/docs-that-teach shows the 55 modified spec.md
files (this run's own artifact of reading the story-spec corpus is not implicated; the
modifications predate this review) and untracked _ledger.md stub files under each story folder,
both entirely inside .bklg/docs-that-teach. No production code and no file outside
.bklg/docs-that-teach in the reported porcelain status. No violation observed within the scoped
check.

**Caveats on method, stated plainly.** This review read 11 of 55 spec.md Dependencies sections in
full, all 6 _storymap.md Slices tables in full (covering all 55 stories' depends_on edges), all 6
project.md acceptance-criteria sections in full, all 6 _decomposition.md design-tension and
Surfaces-relevant sections, and 9 story specs in full or substantial part for distillation and
anchor quality, plus one further Dependencies section per project beyond that for cross-checking —
roughly a sixth of the 55 specs read end-to-end, the rest checked mechanically (AC-id pattern,
Dependencies-heading presence, anchor resolution) or by targeted grep against specific claims
(Renders-surfaces lines, cross-branch citation caveats). The sampled stories were chosen to span
all 6 projects, both foundation and terminal roles, and the more structurally complex slices
(multi-dependency mounts, cross-project preconditions, the closure-rule story). Given the sample's
consistency — every mechanical check passed at full coverage and every manually-read spec matched
the same rigorous pattern (decision-laden Context pack, AC-bound composition invariants,
signposted anchors) — the one design-coverage gap found is assessed as a real, isolated defect
rather than a symptom of broader unsampled failures, but this is inference from a sample, not an
exhaustive read.

## Gaps

- **Issue (fixed in this pass):** checked-documentation-surface/_design.md's Surfaces fenced yaml
  block declared narrative-scoped-page (route docs/topic/scope.md, the "one page per scope" state
  DT-7's option (c) describes past the 3-scope/25-line threshold). No story's Integration contract
  Renders-surfaces line claimed it; eight stories across the project (pinned-narrative-tree-and-compiling-step,
  narrative-checker-mounted-with-pinned-path, narrative-citation-resolution,
  observed-failure-falsification, frozen-documentation-must-pin, hidden-content-resolution,
  narrative-tree-story-grain-selection, spec-trace-clause-id-accessor) named or counted it while
  building or explaining nothing, confirming the gap was known rather than merely missed.
  **Fix applied:** struck narrative-scoped-page from `_design.md`'s `## Surfaces` manifest (now
  five surfaces, not six), with a one-line note that D2's resolution of DT-7 still specifies the
  pattern in prose for a future content author who exceeds the threshold and that this project
  deliberately does not build an instance of it. Corrected the manifest's intro paragraph, the
  reference-captures table (row removed), the mock-file description (now notes it predates the
  strike and retains the extra reference frame), and the two stories whose own text counted "six
  surfaces" (`narrative-tree-story-grain-selection`, `spec-trace-clause-id-accessor`, both
  corrected to "five"). The alternative fix — adding a story that instantiates a second fixture
  page past the threshold — was not taken: it would require a new story via `redkiln new`, which
  is outside this pass's write scope (bodies and `_plan.md` only, no backlog-state mutation).
