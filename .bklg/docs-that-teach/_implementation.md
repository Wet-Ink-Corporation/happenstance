# Implementation ledger — docs-that-teach

Body-only companion file. Carries no item frontmatter; `redkiln` never writes it.
Maintained by `/redkiln:implement` across runs.

entry_baseline: green @ 2920c66 (`cargo xtask ci`, all checks passed, exit 0, 2026-08-17)

Terminal / DoD-owner project: `durable-audience-closeout` (HS-P0025) — confirmed by
`dodOwner: true` in `redkiln status --json` and by `_plan.md`'s project index
("`HS-P0025` is the only project created `--terminal`").

## Projects, in merge order

| # | project | id | dependsOn | terminal | state | verdict | review |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | checked-documentation-surface | HS-P0020 | — | no | done | approved | `checked-documentation-surface/_review.md` |
| 2 | page-need-discipline | HS-P0021 | 1 | no | done | approved | `page-need-discipline/_review.md` |
| 3 | application-author-path | HS-P0022 | 1, 2 | no | pending | — | — |
| 4 | reach-and-adapter-path | HS-P0023 | 1, 2, 3 | no | pending | — | — |
| 5 | comprehension-evidence | HS-P0024 | 3, 4 | no | pending | — | — |
| 6 | durable-audience-closeout | HS-P0025 | 5 | **yes** | pending | — | — |

## Run log

### Run 1 — HS-P0020 checked-documentation-surface

- Entry preflight: tree clean after `2920c66` (two CLI-written telemetry lines committed);
  branch `initiative/docs-that-teach` in `.claude/worktrees/docs-that-teach`; full-suite
  gate green (see `entry_baseline` above).
- Design gate: already approved during `/redkiln:plan` (`gate_open: false` on all six
  projects); no verdict was outstanding.
- Scope: all 10 stories, 4 slices (`compiled-narrative-tree`, `narrative-checker-discipline`,
  `specification-pin`, `falsification-and-limits`).
- baseRef: recorded at launch below.

### Run 1 result — HS-P0020, workflow `wf_4df07ef2-47d`

- baseRef: `6d56d0dc77b2a079c96233c91332a54f16dc0136`
- 10/10 stories, 4/4 slices sealed `approved`, `degradedSummary: none`, no blockers,
  no baseline repairs. Project review verdict `approved`, overall 3
  (`_review.md`); DoD bar green (`_integration.md`, project-scoped — the 15
  whole-initiative scenarios deferred to HS-P0025).
- Rubric: ac-coverage 3, integration-reachability 3, test-integrity 3,
  gate-greenness 3, brief-fidelity 3, intent-fidelity 2, presentation-fidelity 0.
- `presentation-fidelity` is 0 on every project of this initiative by construction:
  `.redkiln/config.yaml` declares no `design:` block, so no surface is ever captured.
  Adjudicated 2026-08-18: **left as-is** — the written `_design.md` record is the
  record, per `_design.md:775-777` and CLAUDE.md. Not to be re-raised per project.
- Human verdicts given 2026-08-18: all 10 stories **approved**, project **approved**.
  NEITHER IS RECORDED YET — blocked, see below.

#### BLOCKER — the story-grain boundary gate cannot be satisfied in this lane

`redkiln advance <story> --to report` runs `implement`'s command gate,
`redkiln verify --item <id> --grain story`. Its `boundary` check compares the story's
declared `## PR boundary` against `changedFiles(root, base)`, and `base` is
`DEFAULT_BASE = "main"` — hardcoded at `src/store/verify.ts:110`, with no `--base` in
`.redkiln/processes/story.yaml:32`'s gate command and no `--base` on `advance`. After a
ten-story project run the whole project's cumulative diff is on the branch, so every
story is measured against every other story's files. Stories 1-9 cannot pass regardless
of quality.

Evidence it is the base and not the work: `redkiln verify --item HS-S0145 --grain story
--base b0bb9bd` (that story's own parent) passes all four checks, including `boundary`.
The same command for HS-S0136 fails `boundary` naming eight files belonging to later
stories. `affected-gate`, `ledger` and `provenance` pass throughout.

Not worked around. The fixes available are all governance calls: a redkiln change
(scope `boundary` to the story's own `links.commits`, which `provenance` already reads),
a pack edit that cannot express a per-story base anyway, or deleting boundary blocks to
make the check a no-op. None is mine to make.

#### FINDING — two stories did escape their declared boundary

Independent of the base problem, and found by mirroring `verify.ts`'s own
`declaredBoundary`/`boundaryRegExp`/`REDKILN_MANAGED` logic against each story's OWN
commits rather than `base...HEAD`:

| story | commit | outside its boundary |
| --- | --- | --- |
| HS-S0136 pinned-narrative-tree-and-compiling-step | `7020c4c` | `standards/rust/{51,52,70,80}-*.md` |
| HS-S0138 narrative-checker-mounted-with-pinned-path | `b62de17` | `standards/rust/{51,52,70,80}-*.md` |

The other eight are clean. Both edits are line-number citation repairs (8 lines each,
`xtask/src/main.rs:784` -> `:820` and so on) forced by inserting a REQUIRED step into
`main.rs`, without which `cargo xtask lint-constitution` fails. Mechanically necessary
collateral, not scope creep — but a real escape the two specs' boundaries did not
anticipate, and the escape the check exists to surface.

#### Disposition, 2026-08-18

Filed as **Wet-Ink-Corporation/redkiln#136** — "Story-grain boundary check is
unsatisfiable in the implement lane (hardcoded main base vs. stacked story commits)",
carrying the reproduction, the two contrasting `verify` runs, and the
`links.commits`-scoped fix sketch.

Human decision: **file it, and leave the stories on `plan`.** No boundary block is
widened and no verdict is forced. The ten approved stories keep their recorded
`links.commits` provenance and their green slice seals; their stage transitions wait on
the fix. HS-P0020 is NOT advanced past `implementation` — parking a project at
`in-review` over ten stranded stories would only move the closeout audit failure
earlier.

The two-story boundary escape (HS-S0136, HS-S0138) is left recorded and unrepaired here
so that whoever resolves #136 has a live case the fixed check must still catch.

#### Resolved, 2026-08-18 — redkiln 0.19.0

**#136 was closed as a duplicate of redkiln#94; the fix had already shipped in
0.19.0.** `boundaryCheck` is now called as `boundaryCheck(located.dir, ownScope ??
changed)`, where `ownScope = ownChangedFiles(root, commitLinks(...))` derives the
window from the story's own `links.commits` plus the working tree
(`src/store/verify.ts:135-160`, `:917-935`). The scoping is deliberately confined to
this one check: `affected-gate` and `provenance` keep the broad set, `ledger` keeps
`base`. The rationale at `:110-134` names the same failure this run hit — "the gate was
strictest on the most disciplined specs: declaring no fence passed, declaring a precise
one blocked you."

Leaving the two-story escape unrepaired was worth it: under the fixed check HS-S0136 and
HS-S0138 failed `boundary` naming **exactly** the four `standards/rust/*.md` files and
nothing else, independently reproducing the finding this ledger recorded, while the
other eight passed all four checks. The fix was verified against a real case rather than
taken on trust.

**Boundary amendment (`c52b031`).** `standards/rust/**` added to both stories' `## PR
boundary` fences, each with an inline note giving the reason: mounting a `REQUIRED` step
into `xtask/src/main.rs` shifts every line number the constitution's Evidence lines cite
into that file, and `cargo xtask lint-constitution` fails until they are re-pointed. The
declarations were incomplete; no claim, rule or example in any atom changed. Recorded as
an amendment rather than a quiet edit, because widening a fence to clear a red gate is
the one move this check exists to make visible.

**Verdicts executed** (the ones given before the blocker, unchanged by it):

- All 10 stories: `plan` -> `report`, `--verdict approved --stay`. All now
  `report`/`in-review`, holding for closeout.
- HS-P0020: `implementation` -> `integration` -> `review`, `--verdict approved --stay`.
  Now `review`/`in-review`. The integration gate ran `integration_scoped`
  (`cargo xtask ci --fast`) green — once on retry after a first-run flake, which redkiln
  reported rather than hid.

Project 1 state: **done**.

### Run 2 — HS-P0021 page-need-discipline

- Entry preflight: tree clean after `36b8c19` (two CLI-written telemetry lines committed);
  branch and worktree confirmed; `entry_baseline` already green so the full-suite gate was
  skipped; affected baseline `cargo xtask ci --fast` green.
- baseRef: `b23b238d` — captured after the `design` -> `implementation` advance, so the
  project's cumulative review diff carries implementation work only. **Held stable across
  re-launches of this project.**
- Scope: all 8 stories, 3 slices (`discipline-on-disk`, `page-need-gate-step`,
  `binding-beyond-this-project`). `terminal: false` — HS-P0025 owns the whole-initiative DoD.

### Run 2 result — HS-P0021, workflow `wf_750dc77d-4e9` — HALTED

- 4/8 stories committed (`55b987b`, `9dacc7d`, `dec82c7`, `a349e04`). Slice
  `discipline-on-disk` sealed **`changes-requested`** (`9b173b2`); slices 2 and 3 never
  opened. `degradedSummary: none`; no baseline repairs. Integration, design review and
  project review did not run — the halt is upstream of them.
- One API server error hit the slice implementer and was retried green on attempt 2
  (`_obs.phases`: Stories, retried 1, failed 0). Nothing was dropped; the run is complete
  in the sense that everything it claims to have produced exists.
- The in-slice fix pass had already run (`gate` -> `re-review`) and the finding survived it,
  which is why the seal is a rejection rather than a repair.

#### The blocking finding — a guard that cannot fail

`xtask/src/lint_pages.rs:1689-1700`, the untagged-fence half of
`no_rust_tagged_and_no_untagged_fence_in_the_rules_tree`. It counts lines whose trimmed
content is exactly a bare fence marker and asserts `openers % 2 == 0`. Those lines are the
*closers* of correctly tagged fences, so an untagged fence contributes one opener and one
closer and parity never moves. Proved by mutation: a bare fence appended to
`standards/pages/40-reviewing-a-page.md` left all 51 `lint_pages` tests green, while a
`rust`-tagged fence in the same place went red at `:1681` with a file:line message.

Verified independently at the orchestrator before accepting the halt, by reading the source
rather than relaying the claim. It is CLAUDE.md's *"a rule that no adapter can fail is
decorative"*, and three ACs rest on it — `need-vocabulary` AC-014, `reviewer` AC-009,
`fold-line` AC-008. The `_ledger.md` AC-014 row flips on exactly the unsound parity argument
("an even number of closers matching four tagged openers"), so the evidence has to be
rewritten, not just the test.

A second, **inverse** error sits in the same module: `router_is_inside_its_budgets`
(`:853-864`) asserts every line starting with a fence marker carries info `text` or
`markdown`, but a legitimate *closing* fence has an empty info string — so the first fence
ever added to `standards/pages/README.md` fails it spuriously. Vacuous today only because the
router has no fence. Both are the same mistake about what a bare fence line means, made in
opposite directions, and both take the same fix: track open/close state and bind the
assertion to the opener. The working `fenced` toggle already exists at `:1498-1508`.

#### Non-blocking findings carried forward

- `need-vocabulary-and-declaration-form/_ledger.md` AC-005 `verifying_test` names
  `router_is_not_created_by_this_story`, which no longer exists — the slice-mate inverted it to
  `router_is_created_by_the_router_story` (`xtask/src/lint_pages.rs:544`). The body explains the
  inversion; the field does not.
- `fold-line-rule/_ledger.md` AC-007 `verifying_test` still claims a grep with no matches and an
  empty `git diff main -- xtask`; both are now false (`xtask/src/lint_pages.rs:966`, and the
  slice does touch `xtask/src`). Recorded correctly in the body and in the report's Deviation 1.
- `router-precedence-and-announcement/spec.md` AC-006's THEN row requires the router to say
  "no gate step reads `standards/pages/` yet", which the delivered router correctly contradicts.
  The correction is recorded at `:170-182`; the row itself needs marking as superseded.
- `reviewer-and-citation-procedures` AC-007/AC-004 owe a named human non-author. The review
  independently re-executed the walk and reached the same verdict, which is a second
  reproducible run but not a human name.
- Risk note, no change owed: the fix pass inlined `PRECEDENCE_CHAIN` and `GENERATED_HEADER` as
  literals (`:235-245`) rather than reading `standards/rust/README.md` live. That is what the
  testing brief prescribes for AC-002 and it satisfies RS-81-3, but an edit to the constitution's
  precedence block would no longer redden anything here. `git diff main -- standards/rust/README.md`
  stays a standing ledger command at integration.

#### Disposition

Human decision 2026-08-18: **re-launch fresh.** No story advanced, no verdict recorded, and
HS-P0021 stays on `implementation` — parking it at `in-review` over a slice its own reviewer
rejected is the self-certification this loop exists to prevent. Preflight reads both resume
axes from git, so the relaunch re-enters `discipline-on-disk` at Review with the findings
above as hypotheses, runs the fix -> re-review -> re-seal tail, and then opens slices 2 and 3.
`baseRef` is unchanged at `b23b238d`.

### Run 3 — HS-P0021, workflow `wf_39c79725-e82` — COMPLETE

Re-launched fresh, not resumed. Preflight read both axes from git, saw `discipline-on-disk`
committed but sealed `changes-requested`, and re-entered it **at Review** — no implementer was
dispatched for the four stories already committed. The fix → re-review → re-seal tail ran, then
slices 2 and 3 opened in order. `baseRef` unchanged at `b23b238d`.

- 8/8 stories; all three slices sealed **`approved`**; `degradedSummary: none`; no blockers; no
  baseline repairs. 21 agents, 0 errors.
- Project review **`approved`**, overall **3**, 0 uncovered ACs, 0 missing artifacts
  (`page-need-discipline/_review.md`). Integration bar green — `dodGreen: true`,
  `reachabilityOk: true`, 0 fixme'd, 0 unmounted, the 15 whole-initiative DoD scenarios deferred
  to HS-P0025 by design (`_integration.md`).
- Rubric: ac-coverage 3, integration-reachability 3, test-integrity 3, gate-greenness 3,
  brief-fidelity 2, intent-fidelity 2, presentation-fidelity 0 (exempt, per the standing
  adjudication under run 1 — not re-raised).
- One caveat on the evidence: the safety classifier timed out reviewing `gate:page-need-gate-step`.
  That slice's substance was verified independently at the orchestrator — the fence fix read in
  source, `cargo xtask ci --fast` green, `doctor` and `validate --kb` clean.

#### The run-1 blocker, verified fixed

The parity count is gone. `xtask/src/lint_pages.rs` now walks fence open/close state
(`fence_tag_problems`) and `the_fence_check_rejects_the_three_wrong_fences` holds three wrong
fences as `&str` specimens — bare, `rust`-tagged, and unterminated — each asserted to report on
the **opener's** own `file:line`. The third case was named by neither the reviewer nor this
orchestrator: an unterminated opener re-phases every fence after it. Checked in source before the
halt was accepted as resolved, not taken from the report. The inverse bug in
`router_is_inside_its_budgets` went with it.

#### Two defects the gates found that the adversarial reviewers did not

Both surfaced only when `redkiln advance` ran the deterministic story gate, after the project
review had already scored the work. Neither changes a score; both are places where a reviewer's
*verified* was not the same as *checked*.

**1. Five of eight stories escaped their declared PR boundary.** Amended at `a41a1a5`, with the
reason inline and the original declaration left standing above it as the audit trail.

| story | outside its fence | what it is |
| --- | --- | --- |
| HS-S0147 / HS-S0148 / HS-S0149 | `xtask/src/lint_pages.rs`, 412 / 253 / 277 lines | every hunk inside `mod tests` — tests pinning each story's own rule atom |
| HS-S0150 | `standards/rust/{51,52,70,80}-*.md` | forced citation renumber, identical to `c52b031`'s case |
| HS-S0150 | `xtask/src/lint_narrative.rs` | the fence declared `xtask/src/narrative.rs`, which names no file in this repository |
| HS-S0150 | `docs/append-conditions.md`, `docs/text-fences.md` | pin repairs; the fence said `docs/README.md` |
| HS-S0151 | the slice-mate's `implementation-report.md` | corrected eight lines the checker story wrote about its own behaviour |

Two Merge DoD lines asserted `git diff main -- xtask` is empty. They were false and are corrected
rather than left to read as met.

**The cause of the first three is structural and will recur.** Stories are implemented **one whole
slice per context, integration-first**, and a slice's stories share one Rust module — so a
per-story fence that partitions `lint_pages.rs` between slice-mates cannot be satisfied by a story
that writes any test at all. The alternative was leaving each rule atom unpinned until the checker
story, which is the half-mount `_storymap.md` explicitly rules out. **Carried to HS-P0022–HS-P0025:
stop fencing per story inside a module a whole slice shares.** A fence naming a nonexistent path is
worth noting separately — it is invisible to the check and can only ever under-constrain.

**2. Two `_ledger.md` files were unparseable, from their own checkpoints.** Fixed at `cf7b1ff`.
`ledgerBlock` scans from the `# … ledger` heading for the first fence and abandons the search the
moment it meets another heading (redkiln 0.19.0 `dist/index.js:14761-14779`).
`governed-page-cites-the-discipline` and `playbook-atom-staged-for-ingest` both put narrative
sections in between, so each read as having **no acceptance block at all** rather than a malformed
one — the same shape of failure as #136: strictest on the most thorough artifact.

They were shaped that way at `263dc7b` and `dc58e80`, their own checkpoints; the review fix pass
added to the sections but did not create the problem. So the adversarial slice reviewer and the
project reviewer both cited these ledgers as evidence while, to the tooling, they were absent —
nothing in the workflow runs `redkiln verify --grain story`, and the story gate was the first thing
to look. The fenced block was moved and nothing inside it edited; a pointer sits at the old
position saying so.

#### Verdicts executed, 2026-08-18

- All 8 stories: provenance recorded first (`record-links`, checkpoint SHA each), then `plan` →
  `report`, `--verdict approved --stay`. All now `report`/`in-review`, holding for closeout.
- HS-P0021: `implementation` → `integration` → `review`, then `--verdict approved --stay`
  (`aadf0589`). Now `review`/`in-review`. The integration gate ran `cargo xtask ci --fast` green.

Project 2 state: **done**.
