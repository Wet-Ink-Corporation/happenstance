---
title: "Integration — Page-Need Discipline"
initiative_slug: docs-that-teach
project_slug: page-need-discipline
terminal: false
dod_green: true
reachability_ok: true
deferred_scenarios:
  - "DoD-1 @smoke — the teaching survives a clean checkout (fresh clone, full gate, narrative builds and renders inside it)"
  - "DoD-2 @smoke — a deliberately broken page fails the gate, by name (re-observed on the assembled tree)"
  - "DoD-3 @smoke — the opening encounter runs and demonstrates a boundary"
  - "DoD-4 @smoke — the boundary claim is checked, not narrated"
  - "DoD-5 — a non-author, non-insider reader completes a stated scenario and it is recorded"
  - "DoD-6 — every stumble in that log has a disposition"
  - "DoD-7 @smoke — the reader reaches the teaching from the front door"
  - "DoD-8 — every page's answered need is stated and singular (re-observed on the assembled tree)"
  - "DoD-9 — the evaluator's second question is walked"
  - "DoD-10 — the adapter author's error meets its explanation"
  - "DoD-11 — the frozen documentation MUSTs are still discharged"
  - "DoD-12 — no page has become a second specification (re-observed on the assembled tree)"
  - "DoD-13 — nothing load-bearing is hidden from the check"
  - "DoD-14 — the discipline is on disk and cited (re-observed on the assembled tree)"
  - "DoD-15 — the audience is durable and reconciled"
---

# Integration — Page-Need Discipline

`HS-P0021`, the **non-terminal** second project of `docs-that-teach`. This audit
proves *this project's* integration bar and nothing wider: every capability it
delivered is mounted and reachable in the running gate, and its own gate is
green. The fifteen whole-initiative Definition-of-Done journeys belong to the
terminal project `HS-P0025 durable-audience-closeout`
([`project.md`](project.md), "Out of scope", `:126-129`) and are listed as
deferred below, not counted here.

Three of those fifteen — DoD-8, DoD-12 and DoD-14 — are the ones this project
supplies the *instrument* for. The instruments were executed here, on the tree as
it stands at this merge, and are recorded under "Project integration bar"; the
whole-initiative re-observation on the **assembled** tree is still HS-P0025's.

Every command in this record was executed in this worktree at `f3b18f7`, on a
tree `git status --porcelain` reports carrying only the session's own telemetry
file. Nothing below is quoted from a story's evidence file except where the row
says so.

## Project integration bar

### The gate commands `.redkiln/config.yaml` wires

| Bar | Command | Ran? | Result |
| --- | --- | --- | --- |
| `integration_scoped` — the non-terminal bar | `cargo xtask ci --fast` | executed | **pass**, exit 0 — `all required checks passed (--fast: 4 optional step(s) not run)`, with `=== every page declares one need ===` printing `2 pages, 16 rules, all consistent` |
| `affected_gate` — the story grain | `cargo xtask affected --base main` | executed | **pass**, exit 0 — `affected gate passed`; `231 passed; 0 failed; 3 ignored`, the three being the constitution's pre-existing `rust,ignore` specimens at `standards/rust/62-doctests-and-harnesses.md:213` and `standards/rust/92-toolchain-limits-and-dead-ends.md:81`, untouched by this project |
| `reachability_static` — the cheap tripwire | `cargo xtask lints` | executed | **pass**, exit 0 — eight banners, closing with `=== every page declares one need ===` / `2 pages, 16 rules, all consistent` |
| `reachability_static`, second half | `cargo xtask spec-trace` | executed | **pass**, exit 0 — 200 clauses, 95 rules, 58 e2e cases, 358 citations checked; `traceability: no problems found`. The two "claimed by no clause" advisories are the pre-existing open questions in `.kb/open-questions/`, not a regression of this project |
| project DoD, `redkiln doctor` | `redkiln doctor` | executed | **pass** — `doctor found no problems` plus **exactly six** `template-drift` advisories: `discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md`, `_design.md`, `_intake-brief.md`. No seventh, none missing; `redkiln adopt --templates` was not run |
| project DoD, `redkiln validate --kb` | `redkiln validate --kb` | executed | **pass** — `validate passed`. `git diff main --name-only -- .kb/` returns exactly one path, and it is under `_intake/` |

### The scenarios this project owns (`project.md:206-244`)

Each row is an acceptance criterion **this project owns**. The row marked
*falsified live* was broken on purpose in this worktree during this audit,
observed failing, and reverted; `git status --porcelain` afterwards shows only
the session telemetry file.

| AC | Scenario | Ran? | Result |
| --- | --- | --- | --- |
| AC-001 | The discipline has a decided home and is on disk, sibling to `standards/rust/`, with a router that loads one rule rather than the corpus | executed | **pass**. `standards/pages/` holds five rule atoms plus `README.md`; the router's `## Start here` table (`standards/pages/README.md:32-42`) routes by reader intent and its band table (`:10-16`) by number. `_design.md:77` records the rejected homes |
| AC-002 | It sits inside the precedence chain without extending it | executed | **pass**. `standards/pages/README.md:20-25` states the rank at the constitution-atom tier and cites the chain rather than editing it; `git diff --stat main -- standards/rust/README.md` is **empty** — that file is not in the branch diff at all |
| AC-003 | DT-2 resolved and recorded, with the options not chosen named | executed | **pass**. `_design.md:168-212`, "S1 vocabulary — DT-2: the need set", closing `**Resolves.** DT-2, AC-003, DR-03` |
| AC-004 | DT-3 resolved and the need set closed | executed | **pass**. `_design.md:214-238` resolves findability as a first-class need; the set is closed in code at `xtask/src/lint_pages.rs:119-136` with a `const` ceiling assertion at `:146` |
| AC-005 | DT-8 resolved as a stated rule a reviewer applies without the author | executed | **pass**. `_design.md:240-288` closes the never-fold list at five classes; the rule is `standards/pages/20-the-fold-line.md`, indexed at `standards/pages/README.md:54` and applied as step 4 of the walk (`standards/pages/40-reviewing-a-page.md:33`) |
| AC-006 | Every governed page declares exactly one need | executed | **pass**. `cargo xtask lint-pages` gave `2 pages, 16 rules, all consistent`. The declarations are `docs/append-conditions.md:3` (`explanation`) and `docs/text-fences.md:3` (`explanation`). `docs/README.md` carries none **by design** — the walk excludes `README.md` (`xtask/src/lint_pages.rs:496`), the same exclusion the pinned tree's own checker states (`xtask/src/lint_narrative.rs:232-241`), and the divergence is recorded rather than smoothed over (`governed-page-cites-the-discipline/_ledger.md:25-54`) |
| AC-007 | The declaration check has been **seen to fail**, by file and line, then returns green | executed, **falsified live in this audit** | **pass**. A second, unenumerated declaration was inserted at `docs/text-fences.md:5`; `cargo xtask lints` exited 1 with two problems on their own lines — ``docs/text-fences.md:5 — `troubleshooting` is not a need: orientation, tutorial, how-to, explanation; see standards/pages/10-the-need-set.md`` and ``docs/text-fences.md:5 — declares `explanation` and `troubleshooting`; a page answers one need`` — then `xtask failed: every page declares one need failed with exit code: 1`. `git checkout -- docs/text-fences.md` and a re-run gave `2 pages, 16 rules, all consistent`. Both halves observed at the integration grain, independently of the story's own capture |
| AC-008 | The lint rejects a wrong page that could plausibly ship — a named wrong implementation in its own tests | executed | **pass**. `xtask/src/lint_pages.rs::tests::a_page_with_no_declaration_is_one_problem_naming_band_zero` (`:2902`), `::a_second_declaration_is_reported_at_the_second_ones_line` (`:2920`), `::a_malformed_declaration_is_its_own_problem_not_a_missing_one` (`:2954`), `::two_orientation_pages_at_one_level_are_one_problem_naming_both` (`:3030`), `::a_needs_member_missing_from_band_ten_names_which_token_moved` (`:3200`). All ran green inside the affected gate's 231 tests |
| AC-009 | The reviewer procedure is non-author-performable, and a walk of the full set finds no page carrying two needs (DoD-8) | **executed in this audit by a non-author** | **pass**. Transcribed under "The RP-40-1 walk" below. This closes the residual the story ledger left open at `reviewer-and-citation-procedures/_ledger.md:63` — "the second independent walker is owed at this project's review and integration stages" |
| AC-010 | The citation rule exists and the spot check runs over the set (DoD-12) | **executed in this audit by a non-author** | **pass**, with one residual handed forward. Transcribed under "The RP-40-2 sweep" below. The story's own sweep covered the two sentences it added; this one covers all nine files of the corpus, per file with counts, as `standards/pages/40-reviewing-a-page.md:75-79` requires |
| AC-011 | The discipline is cited by what it governs, and is reachable from it (DoD-14) | executed | **pass**. `docs/append-conditions.md:10` names `RP-00-1` as the rule that shaped the page; `docs/README.md:14` names `RP-10-2`; `docs/README.md:28` routes to `standards/pages/README.md`; the back-link `standards/pages/README.md:8` reaches `docs/README.md` in one hop and is link-checked by `check_router` (`xtask/src/lint_pages.rs:812-825`) |
| AC-012 | The playbook atom is staged, not hand-authored | executed | **pass**. `.kb/_intake/lesson-page-need-declaration-discipline.md:1-33` carries `kind: playbook`, `authority_tier: guideline`, `status: proposed`. `git diff main --name-only -- .kb/` returns that path and nothing else, so nothing was written into `.kb/` proper; `redkiln validate --kb` is green; its four `depends_on` / `related` ids all resolve to existing atoms |

### The RP-40-1 walk, run at the integration grain

Walker: **this integration audit**, which authored no file in `standards/pages/`,
no page under `docs/`, and no line of `xtask/src/lint_pages.rs`. Consulted: the
rendered pages, `standards/pages/README.md`, and `spec/SPECIFICATION.md`. Not
consulted: any author, and no page's git history was read for the verdict. Date:
**2026-08-18**. Procedure: `standards/pages/40-reviewing-a-page.md:28-42`.

The governed set is not empty — two pages — so this is a verdict and not the
**vacuous** record `standards/pages/40-reviewing-a-page.md:51-54` reserves for an
empty set.

| Page | 1 one declaration | 2 token in the set | 3 sections serve it | 4 nothing folded | 5 normative sentences cite | 6 need nameable from the head | Verdict |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `docs/append-conditions.md` | yes, `:3` and only `:3` | yes, `explanation` | yes — the boundary claim, the compiled fence and the two per-adapter notes (`:24-31`) all serve *why a write re-reads what it decided on*; the adapter notes are position assignment relative to the transaction, which is the mechanism the question asks about | yes — a scan for `<details>` and `<summary>` across `docs/` and `standards/pages/` returns **nothing** | yes — the one normative sentence (`:5-7`) hands authority to `ES-40`, a resolving id, rather than stating a rule in its own voice | yes | **`pass`** |
| `docs/text-fences.md` | yes, `:3` and only `:3` | yes, `explanation` | yes — every paragraph is about which fences the gate never compiles | yes, same scan | yes — its two imperatives (`:26-27`) are repository authoring idiom, which RP-30-1's falsification question (`standards/pages/30-citing-the-specification.md:23-26`) places in the page's own voice: no conformant adapter in another language could violate them | yes | **`pass`** |

No page in the set carries two needs. Step 4's answer is a scan result rather than
an impression, and the tree cannot acquire a fold silently: a `<details`,
`<summary`, `role="tab"` or `{{#tab` marker inside `standards/pages/` is a test
failure (`xtask/src/lint_pages.rs:1904-1909`), and the same markers under `docs/`
are HS-P0020's `lint_narrative` failure.

The walk was also run against the calibration specimen
`standards/pages/examples/two-needs.md`, which carries two declarations at `:3-4`:
step 1 **no**, with a second need visible in the head, so the consequence mapping
at `standards/pages/40-reviewing-a-page.md:37-38` routes straight to
**`fail — two needs`**. The procedure therefore returns both verdicts when run by
someone who did not write it.

### The RP-40-2 sweep, run at the integration grain

Per `standards/pages/40-reviewing-a-page.md:75-79`: file by file, one verdict per
file, with the number of normative sentences examined. Walker and date as above.

| File | Normative sentences examined | Authority | Verdict |
| --- | --- | --- | --- |
| `standards/pages/README.md` | 9 — the precedence block `:20-30`, the shape rules `:61-77` | the precedence chain is quoted with attribution and is beaten by a clause by its own statement; no clause content restated | **pass** |
| `standards/pages/00-one-need.md` | 3 rules | page-authoring idiom; RP-30-1's falsification question answers *no* for each | **pass** |
| `standards/pages/10-the-need-set.md` | 4 rules | idiom; names `spec/SPECIFICATION.md` only as one of the two surfaces that already own `reference` | **pass** |
| `standards/pages/20-the-fold-line.md` | 4 rules | idiom; the one `MUST` on the page (`:71`) is quoted *about* a wrong page, not asserted | **pass** |
| `standards/pages/30-citing-the-specification.md` | 3 rules | idiom. The only clause content in the tree sits inside the `**Not**` fence at `:56-60`, tagged `text`, presented as the forbidden shape — the rule teaching its own violation, not a page becoming a second specification | **pass** |
| `standards/pages/40-reviewing-a-page.md` | 2 rules | idiom | **pass** |
| `docs/README.md` | 1 — `:13-15`, citing `RP-10-2` | a page rule, cited by id, reproducing none of RP-10-2's text | **pass** |
| `docs/append-conditions.md` | 1 — `:5-7`, citing `ES-40` | a visible, resolving clause id; no clause text reproduced, no `MUST`, no restatement | **pass**, with the residual below |
| `docs/text-fences.md` | 2 — `:26-27` | repository idiom, no clause claimed | **pass** |

Corpus: nine files, 29 normative sentences examined. No page has become a second
specification.

**Residual, stated rather than hidden.** `docs/append-conditions.md:5-7` cites
`ES-40`, whose subject is *completeness* — "a conditional append is sound only
over a complete store", `spec/SPECIFICATION.md:4351-4357` — while the sentence's
subject is the boundary re-read. The id **resolves** (HS-P0020's `clause_ids`
check, `xtask/src/lint_narrative.rs:1829`) and the sentence **restates nothing**,
so RP-40-2's discriminator returns `pass` and DoD-12's bar is met as written. But
a citation whose clause is about a different proposition is a defect no rule in
bands 00-40 names and no check reaches. The line predates this project — HS-P0020
authored it at `7020c4c` — so it is recorded here as a handoff to HS-P0025's
DoD-12 re-observation, not repaired inside this project's boundary.

## Deferred to terminal project

Owned by `HS-P0025 durable-audience-closeout`, which re-observes all fifteen on
the **assembled** tree ([`project.md`](project.md), `:126-129`). None is counted
against this project; none was run here.

1. DoD-1 @smoke — the teaching survives a clean checkout.
2. DoD-2 @smoke — a deliberately broken page fails the gate, by name.
3. DoD-3 @smoke — the opening encounter runs and demonstrates a boundary.
4. DoD-4 @smoke — the boundary claim is checked, not narrated.
5. DoD-5 — a non-author, non-insider reader completes a stated scenario.
6. DoD-6 — every stumble in that log has a disposition.
7. DoD-7 @smoke — the reader reaches the teaching from the front door.
8. DoD-8 — every page's answered need is stated and singular. *Instrument
   delivered and executed here* (the RP-40-1 walk above); the assembled-tree
   sweep is HS-P0025's.
9. DoD-9 — the evaluator's second question is walked.
10. DoD-10 — the adapter author's error meets its explanation.
11. DoD-11 — the frozen documentation MUSTs are still discharged.
12. DoD-12 — no page has become a second specification. *Instrument delivered and
    executed here* (the RP-40-2 sweep above), with the `ES-40` residual handed
    forward.
13. DoD-13 — nothing load-bearing is hidden from the check. This project states
    *what may never be folded* (band 20); HS-P0020 owns the demonstration that a
    hidden branch is inside the checked surface.
14. DoD-14 — the discipline is on disk and cited. *Discharged here on the tree as
    it stands* (AC-011); re-observed at closeout.
15. DoD-15 — the audience is durable and reconciled.

## Reachability map

Every capability this project delivered, traced to the running composition root
rather than to a passing test. For a rules tree the composition root is the
**gate step that reads it**; for a rule it is the **router that indexes it**.

| Capability | Mount point | Reachable? |
| --- | --- | --- |
| Band 00 — one need per page, and the declaration grammar (`standards/pages/00-one-need.md`) | read by `rule_atoms` walking the pinned directory (`xtask/src/lint_pages.rs:393-427`, `RULE_DIR` at `:157`); indexed at `standards/pages/README.md:52`; the grammar is enforced by `declarations` / `declaration` (`:546-600`) and `check_declarations` (`:630`) | **yes** — observed live: the injected fault printed `see standards/pages/00-one-need.md` |
| Band 10 — the closed need set and the `orientation` ceiling (`standards/pages/10-the-need-set.md`) | `check_need_set` binds the atom's token table to the `NEEDS` const in both directions (`xtask/src/lint_pages.rs:915-962`); the ceiling is `check_orientation_ceiling` (`:685-710`); indexed at `standards/pages/README.md:53` | **yes** — observed live: the unenumerated token was rejected against the enumerated four, citing that atom |
| Band 20 — the fold line (`standards/pages/20-the-fold-line.md`) | indexed at `standards/pages/README.md:54`; shape-checked by `check_atom_shape` (`xtask/src/lint_pages.rs:713-784`); applied as step 4 of the non-author walk (`standards/pages/40-reviewing-a-page.md:33`); the tree may not itself fold (`xtask/src/lint_pages.rs:1904-1909`) | **yes**, as a rule. Deliberately not a lint, stated first in the module's own limits (`xtask/src/lint_pages.rs:16-19`) — the reviewer instrument DR-08 and AC-005 ask for |
| Band 30 — cite the clause, never restate it (`standards/pages/30-citing-the-specification.md`) | indexed at `standards/pages/README.md:55`; the mechanical half — does the id resolve — is HS-P0020's `clause_ids` (`xtask/src/lint_narrative.rs:1829`), which this module refuses to re-parse (`xtask/src/lint_pages.rs:20-23`) | **yes** — the seam is stated in both directions and no responsibility is owned twice |
| Band 40 — the non-author walk and the paraphrase sweep (`standards/pages/40-reviewing-a-page.md`, `standards/pages/examples/two-needs.md`) | indexed at `standards/pages/README.md:56`; the fixture is linked from `standards/pages/40-reviewing-a-page.md:26` and is deliberately inert under `examples/`, outside the corpus reader's top-level scan (`xtask/src/lint_pages.rs:393-406`) | **yes** — both procedures were executed end to end in this audit by a non-author |
| The router (`standards/pages/README.md`) | `ROUTER` (`xtask/src/lint_pages.rs:167`), checked by `check_router` (`:799-858`): every markdown link resolves, and the `<!-- BEGIN GENERATED -->` region must equal `generated_index` (`:867-887`) | **yes** — reachable from the material it governs at `docs/README.md:28`, and back at `standards/pages/README.md:8` |
| The page-need checker as a gate step | `xtask/src/main.rs:567` — inside `REQUIRED` (the array opens at `:108`), `probe: None`, `--locked`; dispatch arm at `:781-789`; `print_help` at `:875`; `lint_steps()` at `:925` guarded by `steps_named`'s panic at `:935-945`; `xtask/src/affected.rs:139` unconditional; `xtask/src/affected.rs:300` `INERT` for `standards/pages/` | **yes** — `REQUIRED` is what `cargo xtask ci --fast` runs (`xtask/src/main.rs:973`), and the step printed its banner in this audit's run |
| The declaration check seen to fail | the same mount; no new entry. Reached through `run_ci` | **yes** — reproduced independently in this audit: exit 1, then green after revert |
| The declarations on the governed pages | `docs/append-conditions.md:3`, `docs/text-fences.md:3` — inside `lint_narrative::TREE`, consumed by value at `xtask/src/lint_pages.rs:467` with no second path constant | **yes** — both are named by the checker's page count and by the live failure |
| The citation of the discipline | `docs/README.md:14` and `:28`, `docs/append-conditions.md:10`, `standards/pages/README.md:8` | **yes** — one keyboard hop each way, and the back-link is link-checked by `check_router` |
| The staged playbook atom | `.kb/_intake/lesson-page-need-declaration-discipline.md`, the ingest path `/redkiln:kb-ingest` consumes at closeout | **yes** as a *staged* deliverable, which is exactly AC-012's bar. Its consumer, HS-P0025's ingest, is correctly deferred; nothing was hand-authored into `.kb/` |

Nothing this project delivered is constructed-but-unmounted,
exported-but-unconsumed, or reachable only through a test. The one capability
whose consumer sits in a later project — the staged atom — is staged by design,
not stranded.

## Missing dependencies

None. No scenario this project owns needs a dependency that does not exist.

Two things are *owed elsewhere*, and neither is a gap this project should have
filled:

- **HS-P0025's ingest** of the staged playbook atom. Staging is the deliverable
  (`project.md:242-244`); ingest is closeout's.
- **`check_orientation_ceiling` has no live subject yet.** `docs/` holds zero
  `orientation` pages today, so RP-10-3's ceiling is met with room rather than
  exercised on real content. It is not decorative: the wrong page exists in the
  lint's own tests (`xtask/src/lint_pages.rs:3030`), which is `CLAUDE.md`'s bar,
  and HS-P0023's routing content is what will first stand under it.

## Verdict

**Integration bar green.** This project's own bar — its capabilities reachable,
its affected-package and scoped gates green — is met:

- `cargo xtask ci --fast` exit 0; `cargo xtask affected --base main` exit 0;
  `cargo xtask lints` exit 0; `cargo xtask spec-trace` exit 0.
- `redkiln doctor` clean with exactly six template-drift advisories;
  `redkiln validate --kb` green.
- All twelve acceptance criteria executed and passing — two of them (AC-009,
  AC-010) executed *here* by a non-author, closing the residual the story ledgers
  left for this stage, and one (AC-007) falsified live at the integration grain
  rather than trusted from a story's capture.
- Nothing left `fixme`, skipped, or flag-gated-off.

The fifteen whole-initiative Definition-of-Done journeys are deferred to
`HS-P0025 durable-audience-closeout` and are not counted against this project.
One residual is handed forward with its evidence: the `ES-40` citation at
`docs/append-conditions.md:7` names a clause about a different proposition. It
passes every check and every rule that exists today, and it is worth a human eye
at DoD-12's re-observation.
