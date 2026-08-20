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
| 3 | application-author-path | HS-P0022 | 1, 2 | no | done | approved | `application-author-path/_review.md` |
| 4 | reach-and-adapter-path | HS-P0023 | 1, 2, 3 | no | in-progress | — | blocked: redkiln #94/#130 circular gate; 4/8 stories, see run 5 |
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

### Run 4 — HS-P0022 application-author-path

- Entry preflight: tree clean; `entry_baseline` green so the full-suite gate was skipped;
  affected baseline `cargo xtask ci --fast` green.
- **The merge, done in preflight under human supervision rather than inside the workflow.**
  HS-S0183's job is to merge `initiative/from-contract-to-published-library` forward — 294
  commits, a sibling initiative still in flight. It conflicted in five files, and the human
  decision was to resolve it here rather than hand it to an unattended subagent.

  | file | resolution |
  | --- | --- |
  | `xtask/src/spec_trace.rs` | **union**, 230 lines ours + 54 theirs. Both branches added tests to the same `mod tests`; taking either side would have deleted passing tests |
  | `standards/rust/{51,52,70,80}-*.md` | Evidence line numbers only. Both branches had inserted a `REQUIRED` step into `main.rs` and repointed these at their own numbers, so **neither side was correct** against the merged file. Took theirs as the base, recomputed all ten, re-verified — 27 atoms, all consistent |

  One test was red afterwards and is fixed in the merge commit: `doc_budget.rs`'s `read()` now
  normalises CRLF. The test matches source text against literal `\n`; the repo runs
  `core.autocrlf = true` with no `.gitattributes`, so a freshly **checked out** file is CRLF. The
  sibling worktree passes only because its agents **wrote** those files as LF — a fresh clone on
  Windows fails identically, so this is a **latent defect on that branch** the merge exposed
  rather than caused.

  Merge `a5c0f30`, green on `cargo xtask ci --fast` and
  `cargo test --workspace --all-features --no-fail-fast`. HS-S0183's fence was then amended
  (`6d44316`) to name all six conflict-resolution files — and records that they were named
  **after** resolution, not before as its own spec requires, because the point of naming first is
  that nobody decides a file was in scope by having already edited it.
- baseRef: `7c62e4d1`, captured after the `design` -> `implementation` advance. Held stable
  across re-launches.

### Run 4 result — HS-P0022, workflow `wf_d0c0929a-11d` — HALTED (blocked dependency)

- 4/8 stories committed. `preflight-and-anchor` sealed **`approved`** (`7016d65`);
  `opening-encounter` committed (`9493276`, `cc9c4a4`) but **unsealed**; slices 3 and 4 never
  opened. No baseline repairs. 9 agents, 0 errors, 0 retries.
- **`degradedSummary: 2 fatal`**, and the label is wider than the fact. Both entries are
  `strayPaths` — seven helper files agents wrote outside the worktree, all under this session's
  scratchpad (`check_baseline.py`, `measure2.py`, `check_resolutions.py`, and a `dt-probe`
  scratch crate used to certify DT-6). They are verification instruments, not deliverables; no
  repo file outside the worktree was touched and no declared artifact is missing. A real
  isolation-rule violation, benign in effect — recorded rather than filed away.

#### BC-002 — the blocker, and why halting was right

`boundary-refusal-encounter` stopped at its `implement` step rather than delivering three ACs
partially. `_design.md` § Composition composes a `## A boundary refuses` section as a **second**
fence on `crates/happenstance/src/lib.rs`. The merge deleted the premise that was written
against, and the merged page carries its own signed-off assertions:
`crates/happenstance/tests/doc_budget.rs:157` requires **exactly one** fence — *"a second one
would demote the first, which is the page's primary hierarchy signal"* — and
`MODULE_DOC_LINES = 130` caps the module doc, which this story's own additions take to exactly
130. Both verified at the orchestrator against the merged tree before the halt was accepted.

The only route to the section was deleting HS-P0016's signed-off `commit` landing program —
outside this project's seam per `project.md`'s risk table, named by two vocabulary bullets, and
the crate's one demonstration of the typed layer ADR-0006 gave the bare name to.

**Nothing was stubbed, no test weakened, no criterion re-worded, and `_design.md` was not
edited.** The reader-facing surface is complete: `docs/first-encounter.md` is authored, mounted,
executed and reachable, and the crate root took the answered-need line and a pointer to it. This
is the halt-loudly-on-a-missing-dependency path working exactly as intended.

#### Resolved 2026-08-19 — amend the design (`faa8834`)

Human decision: **amend the design; do not amend the merged tree.** The crate root does not carry
the refusal; it is authored once, on `docs/first-encounter.md`.

- `_design.md` § Composition region 4 and the `## Hierarchy` primary-element bullet are struck by
  amendment notes that leave the original text standing as the audit trail.
- `spec.md` gains `## Amendment — BC-002` immediately above the acceptance table, naming exactly
  which clause of AC-002, AC-007 and AC-008 falls and what stands in its place.
- `_conditions.md` records the two rejected routes: raising the crate-root budgets (overrides
  another project's signed-off assertion from inside a documentation story — editing the check
  that says no is not answering it) and replacing the landing program (outside the seam, and it
  would meet a reader with an untyped `Bytes` program on the crate whose identity is the typed
  layer).

**The cost is recorded, not absorbed.** The composition decision the spec was built on — *a
reader who lands on docs.rs meets the refusal before they meet the plan* — is not delivered.
That reader meets a `commit` program and reaches the refusal one hop later. The three inherited
AC-007 overages on the crate root are **recorded as owed by HS-P0016**, not waived.

### Run 5 result — HS-P0022, workflow `wf_e181c7e6-6ac` — HALTED at slice 2 review

Re-launched fresh after the BC-002 amendment. Preflight saw `preflight-and-anchor` sealed
`approved` and skipped it; `opening-encounter` re-entered at Review as designed.

- 4/8 stories. `opening-encounter` sealed **`changes-requested`** (`df38576`).
  `degradedSummary: none` — an API 521 killed the seal agent, the retry succeeded, and the
  verdict reached git. Worth noting only because the *first* symptom of a lost seal and a
  retried one look identical from the failures list; git is what settles it.
- The reviewer declined to fire the escape-hatch detector on the BC-002 descope and said why:
  the blocker is external and mechanically verifiable, the route was written into `spec.md` as
  EC-006/EC-008 *before* implementation, the amendment is a separate human-authored commit that
  leaves every original word standing, and the implementer **refused to flip AC-007** — the
  opposite of gaming. That is the right call and is recorded so closeout does not rediscover it.

#### Blocker 1 — provenance, and the boundary red it was faking

`redkiln verify --item HS-S0185 --grain story` failed `provenance` ("14 file(s) changed inside
this story's declared boundary and links.commits is empty") **and** `boundary`, the latter naming
~200 files from the `a5c0f30` merge. The second was an artifact of the first: 0.19.0 scopes
`boundary` to `ownScope = ownChangedFiles(root, commitLinks(...))` and falls back to the
branch-wide diff when `links.commits` is empty.

Fixed by recording the two checkpoint SHAs (`record-links HS-S0185 9493276`,
`HS-S0186 cc9c4a4`). The slice fix pass `5af116b` was **deliberately not recorded against
either story**: it spans both stories' directories, so attaching it to HS-S0185 would drag its
slice-mate's ledger and report inside HS-S0185's fence and turn `boundary` red for real. Same
disposition as HS-P0021's fix commits.

#### Blocker 2 — AC-007 had a strike with no owner

Correctly red. `spec.md` § Amendment — BC-002 struck three inherited crate-root numbers and
`_conditions.md` recorded them as owed by **HS-P0016**, which lives on the unmerged sibling
branch and is therefore not a destination reachable from here. `_conditions.md:229-232` had
already refused the shortcut in advance: *"Flipping it against a destination that carries no rows
would reproduce, one directory over, exactly the defect this section was written to close."*

Resolved 2026-08-19 at the human's direction by opening a real owner:

- **HS-B0001** `crate-root-density-overages` (severity medium), carrying F-1/F-2/F-3 verbatim
  with budgets, re-measurement commands and reader impact — fence 35 rendered lines against 32,
  70 columns against 68 on two lines, two of four `##` headings at 39 and 26 against 22.
- **HS-P0026** `inherited-documentation-defects` created in the same act, and only because
  `redkiln new` refuses to place a story or a bug directly under an initiative and `support`
  (HS-I0005) is an untouched template carrying no projects. Flagged to the human before it was
  created, because inventing a project inside an un-decomposed initiative is more structure than
  one blocked row justifies on its own.

AC-007 now flips against an item id that exists on this branch and carries the findings.
`redkiln verify --item HS-S0185 --grain story` passes all four checks. The overages are recorded
**owed, not waived**.

#### FINDING — `declaredBoundary` fails open, and hardest on the most on-topic spec

`redkiln verify --item HS-S0186 --grain story` reports **pass** with two of four checks
**skipped**: `boundary — no boundary declared` and `provenance — no boundary declared`. The
fence is plainly there, seven entries at
`boundary-falsification-drill/spec.md:198-207`.

Cause, read in the parser rather than inferred (`dist/index.js:14696-14718`):

```js
const headingIdx = lines.findIndex((l) => /^#{1,6}\s+.*boundary/i.test(l));
// ...then, scanning forward for the fence:
if (/^#{1,6}\s/.test(lines[j] ?? "")) return void 0;   // bail on any heading first
```

It takes the **first** heading anywhere in the file whose text contains "boundary". This story's
**title** is `# Spec — Removing the boundary makes the repository fail` (`:10`); `## Scope lock`
follows at `:12`, before any fence; the function returns `undefined`, and both checks silently
become no-ops. A story *about* boundaries gets **no boundary check**, and the item reports `pass`
rather than a warning.

This is the **third** member of one family this session, and the family is worth naming:

| # | check | how it fails open |
| --- | --- | --- |
| redkiln#136 / #94 | `boundary` | measured every story against `main`, so a precise fence blocked and no fence passed — fixed in 0.19.0 |
| `ledgerBlock` | `ledger` | a heading between the `# … ledger` heading and the fence makes the whole acceptance block *invisible*, so two HS-P0021 ledgers read as having no rows at all |
| `declaredBoundary` | `boundary`, `provenance` | the word "boundary" in a title captures the search; the declared fence is never found |

All three are strictest, or blindest, on the artifact that took the subject most seriously. Not
filed as an issue and **not worked around**: renaming HS-S0186 to dodge the regex would hide the
bug rather than report it, and the story's title is accurate. Recorded here at the human's
direction. The reviewer had already hand-checked every slice commit against both amended fences
and found no real escape, so nothing is known to be hiding behind the skipped check.

Two PR boundaries were also amended inside the run (`4232b34`) to admit `xtask/tests/**`, each
with its reason inline — the same disposition as `a41a1a5` one project back.

### Run 6 result — HS-P0022, workflow `wf_7502f9c7-31c` — COMPLETE

Third launch of this project, fresh each time. Preflight skipped the two sealed slices and
re-entered `opening-encounter` at Review with the orchestrator's fixes in hand.

- 8/8 stories, all four slices sealed **`approved`**. `degradedSummary: none`; no baseline
  repairs; 18 agents, 0 errors. Project review **`approved`**, overall **3**, 0 uncovered ACs,
  0 missing artifacts.
- Rubric: ac-coverage 3, integration-reachability 3, test-integrity 3, gate-greenness 3,
  **brief-fidelity 3** (recovered from HS-P0021's 2), intent-fidelity 2,
  presentation-fidelity 0 (exempt).
- Integration: `dodGreen: true`, `reachabilityOk: true`, 0 fixme'd, 0 unmounted. **The DoD was
  executed, not read** — the audit ran the falsification drill live in *both* directions rather
  than accepting the story's recorded capture, and `spec-trace` resolved 201 clauses and 401
  citations. 9 project DoD items passed; 13 initiative journeys deferred to named owners.
- Scope clean: the production diff is exactly the four declared surfaces, three harness
  registrations, one `docs/README.md` index row, the byte-identical `overview.md` extraction and
  four new test files. `.kb/`, `spec/SPECIFICATION.md`, `.redkiln/config.yaml`,
  `.redkiln/templates/` and `main.rs`'s `REQUIRED` array untouched.

#### Why intent-fidelity is 2 — two deferrals, both stated rather than scored around

- **F-A — the two new pages have no front door.** `docs/carry-your-invariant.md` and
  `docs/read-the-worked-example.md` link only to each other; nothing reaches them from
  `docs/README.md`'s index, from `docs/first-encounter.md`, or from the crate-root pointer. The
  asymmetry the reviewer named is the uncomfortable part: **this same project added a
  narrative-index row** for the opening encounter (`docs/README.md:20`, per
  `boundary-refusal-encounter/spec.md:412`), and its own test at
  `xtask/tests/first_encounter.rs:479-484` calls an unindexed page *"compiled but unreachable"* —
  then two later stories treated the identical act as pointer policy and declined it. Deferred
  initiative DoD-7/DoD-9, owned by **HS-P0023**, scoped out at spec time, so not drift.
- **BC-002's cost, still standing.** The docs.rs reader meets HS-P0016's `commit` program, not a
  refusal. `_design.md:471` records that the stronger reading *"is not delivered and is not
  deemed delivered."*

#### F-B — routed onto HS-B0001 as F-4 (`0110f78`)

The pointer this project added at `crates/happenstance/src/lib.rs:66` (*"To watch that boundary
refuse a write instead"*) sits one line beneath an inherited fence calling `Tags::empty()` twice
(`:40`, `:57`) — the call `initiative.md:51-53` describes as **zeroing out the consistency
boundary**. The landing page therefore demonstrates the boundary being zeroed and, immediately
below, promises a boundary that holds. Both true alone; read top to bottom they teach against
each other.

Not a `docs-that-teach` defect — AC-006 binds pages this initiative *authored* and the fence is
HS-P0016's — but recorded with the finding that matters for next time: **`boundary-refusal-
encounter`'s O2 check cleared AC-006 by reading only the lead-in above the fence and never the
sentence added below it.** The criterion was not unmet; the check was narrower than the page.
Filed onto the existing bug rather than a second item, since it is the same file and the same
root cause.

#### Verdicts executed, 2026-08-19

- All 8 stories: provenance recorded first, then `plan` -> `report`, `--verdict approved --stay`.
  All now `report`/`in-review`, holding for closeout.
- HS-P0022: `implementation` -> `integration` -> `review`, then `--verdict approved --stay`
  (`66335bbd`). Now `review`/`in-review`.

Project 3 state: **done**.

### Carried into HS-P0023 `reach-and-adapter-path`

1. **F-A** — give `docs/carry-your-invariant.md` and `docs/read-the-worked-example.md` an
   entrance. This is initiative DoD-7/DoD-9 and HS-P0023 owns it.
2. **The residuals HS-P0021 assigned to HS-P0022 are still open** and move forward again: the
   `README.md` exclusion from the governed set (`xtask/src/lint_pages.rs:496`), which leaves the
   `orientation` token with no live subject, and the two rotted Evidence citations in
   `standards/pages/10-the-need-set.md:72` and `:169`.
3. **Per-story fences inside a shared module.** Three projects running, three sets of boundary
   amendments. Stories are implemented one whole slice per context, so a fence that partitions one
   Rust module between slice-mates cannot be satisfied by a story that writes any test. Worth
   fixing at plan grain rather than amending per project a fourth time.

---

# HANDOFF — session ended 2026-08-19, mid-HS-P0023

**Read this section first.** The session was ended deliberately, not by a failure. Everything
below is what a fresh `/redkiln:implement docs-that-teach` needs in order to pick up without
re-deriving anything.

## Resume in one line

Preflight, then **launch the workflow fresh** for HS-P0023 (a new `Workflow` call — never
`resumeFromRunId`, never a cache-buster). Git carries both resume axes and preflight reads them.

```
scriptPath: <redkiln workflow-root>/forge-implement.js
args: { slug: 'docs-that-teach', initiativeId: 'HS-I0007', projectId: 'HS-P0023',
        terminal: false, branch: 'initiative/docs-that-teach',
        baseRef: 'ffd0eeb7',                       <- HOLD THIS. Do not re-capture.
        backlogDir: '.bklg/docs-that-teach',
        worktree: '.claude/worktrees/docs-that-teach',
        project: { id: 'HS-P0023', slug: 'reach-and-adapter-path',
                   title: 'Reach and Adapter Path',
                   briefs: ['architecture','ux','testing','design','grounding'] },
        stories: [ ...the 8 below, in this order, grouped by slice... } ]
```

`baseRef` is `ffd0eeb7` — captured after HS-P0023's `design -> implementation` advance. **Keep it
stable across re-launches of this project**; re-capturing it at the current tip would hide the
story commits already made from the cumulative review diff.

## Exact state of HS-P0023

| slice | story | id | state |
| --- | --- | --- | --- |
| `pointer-policy` | `pointer-policy-and-inventory` | HS-S0154 | **committed `ef2eb6b`**, slice sealed **approved** `4457d62` |
| `adapter-error-site` | `adapter-reasoning-account` | HS-S0155 | **committed `af9a241`**, slice NOT sealed |
| `adapter-error-site` | `store-error-site-rewrite` | HS-S0156 | **PARTIAL, in `10e99b2` as WIP** — see below |
| `adapter-error-walk` | `error-site-walk-record` | HS-S0157 | not started |
| `front-door-reach` | `front-door-pointer` | HS-S0158 | not started — **see the known collision below** |
| `front-door-reach` | `evaluator-onward-links` | HS-S0159 | not started |
| `reach-walks` | `front-door-walk-record` | HS-S0160 | not started |
| `reach-walks` | `second-question-walk-records` | HS-S0161 | not started |

`depends_on` and `traces_to` for all eight are in
`.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` § Slices — read it rather than
guessing.

Preflight will skip `pointer-policy` (committed **and** sealed approved) and re-enter
`adapter-error-site`, which has one story committed and one partial.

### `10e99b2` — the WIP commit, and why it is shaped that way

The run was stopped mid-`store-error-site-rewrite`. Its uncommitted work was committed rather
than discarded, **deliberately without a `Story:` trailer**, so the resume window does not count
HS-S0156 as done and the story re-runs. The re-entering implementer will find the work already in
the tree: it is a **starting point no review has seen**, not finished work.

- `crates/happenstance-core/src/store.rs` (+51) — the module-doc rewrite the story exists for.
- `xtask/src/pointers.rs` (+113) — pointer-checker work continuing from HS-S0154.
- `store-error-site-rewrite/spec.md` (+23) — written by the implementer during the run; read it
  before trusting the fence.
- `spec/SPECIFICATION.md`, `spec/E2E-CASES.md`, nine `standards/rust/*.md` — **mechanical citation
  renumbering only**, forced by `store.rs` growing ~47 lines. Diff read line by line before
  committing: no clause, rule, claim or example changed, only the line numbers Evidence and
  `Rejects:` point at. Third occurrence of this pattern in the initiative.

**`cargo xtask ci --fast` was green at `10e99b2`** — run, not assumed — so preflight starts from a
green baseline and the partial work is self-consistent.

### Known collision waiting in HS-S0158 `front-door-pointer`

Measured at preflight, before the run started, and passed to the implementer as a forewarning
rather than a decision. The story must install a pointer on `crates/happenstance/src/lib.rs`
"above the fold, displacing nothing", but that page has **zero headroom**: the module doc is at
**exactly 130/130** against `MODULE_DOC_LINES = 130` (`crates/happenstance/tests/doc_budget.rs`),
because HS-P0022's own pointer at `:66` took it there. "Add a pointer" and "displacing nothing"
are contradictory on a full budget.

Two routes are **already closed**, both rejected when BC-002 was settled on 2026-08-19
(`application-author-path/boundary-refusal-encounter/_conditions.md` § "BC-002 — RESOLVED"):
raising `MODULE_DOC_LINES`, and touching HS-P0016's `commit` landing program.
`crates/happenstance/README.md` — the story's other surface — carries no such budget, and
**HS-B0001 already owns four findings on that same file**. If the crate-root half is genuinely
unbuildable, the right move is a recorded condition routed to the `_design.md` sign-off owner,
exactly as BC-002 was.

## The rest of the initiative

Projects 1–3 are **done**: HS-P0020, HS-P0021 and HS-P0022 are each approved and parked at
`review`/`in-review`, with all their stories approved at `report`/`in-review`, holding for
`/redkiln:closeout`. **Nothing may be advanced to `closeout`/`done`** — that is the human's call
at closeout. Projects 5 and 6 (HS-P0024, HS-P0025) are pending; **HS-P0025 is terminal** and owns
the whole-initiative DoD, so it is the only one launched with `terminal: true`.

### Carried forward, and now deferred more than once

1. **F-A** — `docs/carry-your-invariant.md` and `docs/read-the-worked-example.md` have no
   entrance; nothing links to them but each other. Initiative DoD-7/DoD-9, **owned by HS-P0023**,
   so it is due in the run being resumed.
2. **HS-P0021's two residuals, deferred twice now** — the `README.md` exclusion at
   `xtask/src/lint_pages.rs:496` that leaves the `orientation` token with no live subject, and the
   rotted Evidence citations at `standards/pages/10-the-need-set.md:72` and `:169`. Assigned to
   HS-P0022, which did not touch them.
3. **Per-story fences inside a shared module.** Three projects, three rounds of boundary
   amendments. Stories are implemented one whole slice per context, so a fence partitioning one
   Rust module between slice-mates cannot be satisfied by a story that writes any test. Worth
   fixing at plan grain rather than amending a fourth time.

### Open items with owners

- **HS-B0001** `crate-root-density-overages` (under **HS-P0026**, both created this session) —
  four findings on `crates/happenstance/src/lib.rs`: F-1/F-2/F-3 the density overages, F-4 the
  `Tags::empty()` fence that zeroes the boundary the new pointer promises.
- **Three redkiln parser findings**, recorded above and none worked around: redkiln#136/#94
  (fixed in 0.19.0), `ledgerBlock` (a heading before the fence hides the whole acceptance block),
  and `declaredBoundary` (the word "boundary" in a title captures the search, so HS-S0186 reports
  `pass` with two of four checks skipped). All three fail open, and hardest on the artifact that
  took the subject most seriously.

## Standing rules for whoever resumes

- **`redkiln` is the only writer of item frontmatter.** Every transition is `redkiln advance`
  with `--commit`, never `--apply`.
- **Two calls per story verdict**: `advance <id> --to report --commit`, then
  `--verdict approved --stay --commit` (or `--verdict changes-requested --commit`, no `--stay`).
  Approval requires a human answer; never pass a hardcoded verdict.
- **`record-links` before the advance**, using each story's own checkpoint SHA. Do not attach a
  cross-story fix-pass commit to one story — it drags the slice-mate's files inside that story's
  fence and turns `boundary` red for real.
- **`record-run` first, before anything else, including before any halt.**
- Never `git stash` in this worktree; never `redkiln adopt --templates`; never `--no-verify`.

---

### Run 4 — HS-P0023 `reach-and-adapter-path`, workflow `wf_119c8acb-7d4` — HALTED

- Entry preflight: tree clean after `5dc4d4d` (one CLI-written telemetry `session_end` line
  committed); branch and worktree confirmed; `entry_baseline` already green so the full-suite
  gate was skipped; affected baseline `cargo xtask ci --fast` green, exit 0.
- baseRef: **`ffd0eeb7`**, held stable from the previous session rather than re-captured, exactly
  as the handoff instructed.
- Scope: all 8 stories. Slices 3 and 4 were swapped from `_storymap.md`'s numbering so the
  independent `adapter-error-walk` banks before the known `front-door-reach` budget collision;
  the storymap explicitly permits it ("slices 4 and 3 are concurrent"). `terminal: false`.
- `degradedSummary: none`. No baseline repairs. 6 agents, 0 retried, 0 failed.

#### Result — 3/8 stories, halted at `adapter-error-site` verify

`pointer-policy` was skipped as committed-and-sealed-approved. `adapter-error-site` re-entered,
implemented `store-error-site-rewrite` (`f2c7dbe`), ran review, gate, an in-slice fix pass
(`76e9424`, re-anchoring every `store.rs` citation to its subject), re-review and gate rerun,
then sealed **`changes-requested`** (`64301e9`). Slices 3, 4 and 5 never opened. Integration,
design review and project review did not run — the halt is upstream of them.

#### The blocking finding was a provenance gap, not a code defect

The one surviving finding was a deterministic story-gate failure on HS-S0155:

```
boundary:   changed outside declared boundary: CHANGELOG.md, CLAUDE.md, Cargo.lock,
            Cargo.toml, RUNBOOK.md, [195 files total]
provenance: 16 file(s) changed inside this story's declared boundary and
            links.commits is empty.
```

All three of HS-P0023's committed stories carried `links.commits: []` — the previous session
was interrupted before it reached `record-links`. With that field empty, redkiln 0.19.0's
`boundaryCheck(located.dir, ownScope ?? changed)` has no `ownScope` to use and falls back to the
broad `base...HEAD` set; `base` is `DEFAULT_BASE = "main"`, so HS-S0155 was measured against the
entire initiative branch — including `CHANGELOG.md`, `Cargo.toml` and the whole sibling-merge
HS-P0022 brought forward, none of which any story in this project touched.

This is **not** a recurrence of redkiln#94/#136. That fix works; it is *conditioned* on
`links.commits` being populated, and the interrupted session never populated it. The tell was in
the boundary output itself: `declared but matched no changed file:
.bklg/.../adapter-reasoning-account/**` — the story's own fence matched nothing because the
window being measured was the wrong one.

**Remedy applied, 2026-08-20** — the one the gate's own message prescribes:

| story | sha(s) recorded |
| --- | --- |
| HS-S0154 `pointer-policy-and-inventory` | `ef2eb6b` |
| HS-S0155 `adapter-reasoning-account` | `af9a241` |
| HS-S0156 `store-error-site-rewrite` | `10e99b2`, `f2c7dbe`, `76e9424` |

`10e99b2` (the prior session's trailer-less WIP) and `76e9424` (the in-slice fix pass) were both
checked file-by-file before attribution: both touch only HS-S0156's subject — `store.rs`, that
story's backlog folder, and the mechanical `spec/` + `standards/rust/` citation renumbering the
`store.rs` insertion forces. Neither touches `adapter-reasoning-account/`, so neither drags a
slice-mate's files inside the other story's fence. Committed as `0bf8316`.

Re-verified after recording — **all three now pass**:

```
HS-S0154 (story): pass   [ok] affected-gate  [ok] boundary  [ok] ledger  [ok] provenance
HS-S0155 (story): pass   [ok] affected-gate  [ok] boundary  [ok] ledger  [ok] provenance
HS-S0156 (story): pass   [ok] affected-gate  [ok] boundary* [ok] ledger  [skip] provenance*
```

#### FINDING — the fourth fail-open parser case, and it hit the most careful spec

`*` above is a real gap, not a pass. HS-S0156's gate reports:

> `boundary` — a boundary heading is present but declares nothing parseable; declare the paths
> in a fenced block, or in a `| Path | Change |` table whose first column carries backticked paths

and consequently `provenance` — `[skip] no boundary declared`. So HS-S0156 has **two of four
checks inert**, and its `## PR boundary` — five precise "In this PR" bullets and eight
"Explicitly not in this PR" bullets, among the most carefully drawn in the initiative — binds
nothing mechanically.

The cause is the parser's shape requirement. HS-S0154's bullets parse because each opens with a
backticked path (`` - `xtask/src/pointers.rs` — new. ``). HS-S0156's open with prose (`- The
rewrite of `## Import one flavour, not both` in ...`), so the first backticked run is a heading,
not a path, and `declaredBoundary` finds nothing.

This is the **fourth** finding in this family on this initiative, after redkiln#136/#94, the
`ledgerBlock` heading case and the `declaredBoundary` title-capture case (HS-S0186). All four
fail **open**, and all four bite hardest on the artifact that took the subject most seriously —
the same asymmetry redkiln's own fix rationale names: *"the gate was strictest on the most
disciplined specs: declaring no fence passed, declaring a precise one blocked you."* Here the
inversion is complete: writing the fence in prose makes it vanish.

Not worked around. Rewriting HS-S0156's boundary into a parseable form would be a **boundary
amendment**, which this ledger's own precedent (`c52b031`) requires be recorded as an amendment
with its reason rather than made as a quiet edit — and it is a governance call, not the
orchestrator's.

#### Disposition

Human decision 2026-08-20: **relaunch fresh** (`wf_8077d317-429`, same baseRef `ffd0eeb7`), and
**file the parser finding upstream** the way redkiln#136 was.

No story advanced, no verdict recorded, and HS-P0023 stays on `implementation`. The seal on
`adapter-error-site` was issued by that slice's own reviewer, and a re-review is what clears it —
not the orchestrator's account of why the gate went red. Preflight reads both resume axes from
git, so the relaunch re-enters `adapter-error-site` at Review with the prior finding handed over
as a hypothesis to verify, then continues to slices 3, 4 and 5.

HS-S0156's unparseable boundary is **left as it is** for this run: recorded, not amended. Reshaping
a fence so a gate sees something different is the one move the check exists to make visible, and it
is owed an amendment record with its reason rather than a quiet edit made in passing.

---

### Run 5 — HS-P0023 `reach-and-adapter-path`, workflow `wf_8077d317-429` — HALTED

- Relaunched fresh per run 4's disposition. Same baseRef `ffd0eeb7`, same eight stories.
- `degradedSummary: none`. 8 agents, 0 retried, 0 failed. No baseline repairs.
- **4/8 stories.** `adapter-error-site` re-entered at Review, re-reviewed and sealed
  **`approved`** (`d14affd`) — the run-4 provenance repair cleared it, confirming that diagnosis.
  `adapter-error-walk` then opened, implemented `error-site-walk-record` (`2ac7163`), reviewed
  it, and hit **the identical gate failure**, sealing `changes-requested` (`cd85952`).
- Integration, design review and project review did not run.

#### ROOT CAUSE — two correct redkiln fixes that are incompatible in this lane

Run 4 was read as a one-off gap left by an interrupted session. Run 5 proves it is structural.

- **#94** made `boundary` derive its window from `links.commits` via `ownChangedFiles`, because
  on a long-lived initiative branch `main...HEAD` is the whole initiative and a precise fence
  *"CANNOT PASS, ever"* — `src/store/verify.ts:110-118`, in those words.
- **#130**, new in **0.20.0**, moved all four checks of `verify --grain story` into the slice's
  in-workflow Verify leg. Before 0.20.0 that leg ran `affected_gate` only.

`workflows/forge-implement.js` contains **zero** occurrences of `record-links`; the command
assigns it to the orchestrator, after the workflow returns. So `links.commits` is necessarily
empty when #130's gate runs, `ownChangedFiles` returns `undefined`, and `boundaryCheck(dir,
ownScope ?? changed)` falls back to precisely the window #94 exists to avoid. `provenance` fails
on the same empty field. The slice cannot seal, so the loop cannot proceed unattended.

The whole argument is visible in this one project:

| story | under 0.20.0's four-check leg | why |
| --- | --- | --- |
| HS-S0154 | sealed `approved` | ran under **0.19.0**, whose leg had one check |
| HS-S0155 | **halted** (run 4) | parseable boundary |
| HS-S0156 | passed | boundary **unparseable** — the subheading bug switches provenance off |
| HS-S0157 | **halted** (run 5) | parseable boundary |

Not one story cleared that gate on merit. The two that passed did so because the check did not
exist yet, or because a parser bug hid their fence.

`require_commit_provenance: false` is **not** a workaround: `ownScope` is computed regardless of
that flag, so `boundary` would still fall back. Verified by reading `verify.ts:135-152`, not
assumed.

**Repair applied:** `redkiln record-links HS-S0157 --sha 2ac7163` (`c3e4673`). HS-S0154, HS-S0155,
HS-S0156 and HS-S0157 now all pass `verify --grain story` on all four checks.

#### Disposition

Human decision 2026-08-20: **stop HS-P0023 here and reassess**, and **file both findings
upstream as separate issues**.

**Filed 2026-08-20**, after searching the tracker for duplicates:

- **Wet-Ink-Corporation/redkiln#146** — "forge-implement's story gate reads `links.commits`, which
  nothing in the lane writes until after the workflow returns: #130 restores the pre-#94 boundary
  failure at every slice". No open duplicate. #130 was closed 2026-08-19 in the 0.20.0
  gate-honesty sweep (#138) and its body never mentions `provenance`, `record-links` or
  `links.commits`, so the interaction with #94 was not considered. Carries the four-story table,
  both gate transcripts, the one-command proof, and three ranked fixes.
- **Wet-Ink-Corporation/redkiln#147** — "Residual of #135: `sectionPatterns` ends the boundary
  section at any heading, so a correct fence under `### In this PR` is unreachable". **#135 is
  closed and its fix shipped in 0.20.0** — the anchored heading match and the end of the silent
  pass both work — so this was refiled as a *residual* rather than a new bug. The sharpening that
  earns it its own issue: #135 characterised this bucket as specs "written as prose, with no
  fenced block", and the synthetic case proves a **textbook-correct fence** under a subheading is
  equally invisible. Suggests re-running #135's 135-spec replication with a depth-aware
  terminator, since some of its eight cause-B specs may have had a fence all along.

The two are cross-linked, because they interact: an unparseable boundary switches `provenance`
off, which is the only reason HS-S0156 cleared the gate its slice-mates could not. Landing #147
alone would turn that story red on #146's gate, so the fixes want sequencing together.

Nothing advanced. HS-P0023 stays on `implementation`; all eight stories stay on `plan`; no
verdict recorded anywhere. Two slices remain unopened (`front-door-reach`, `reach-walks`), and
`front-door-reach` still carries the unresolved 130/130 crate-root budget collision described in
the run-3 handoff.

## HS-P0023 state at stop

| slice | story | id | state |
| --- | --- | --- | --- |
| `pointer-policy` | `pointer-policy-and-inventory` | HS-S0154 | committed `ef2eb6b`, sealed **approved** |
| `adapter-error-site` | `adapter-reasoning-account` | HS-S0155 | committed `af9a241`, sealed **approved** |
| `adapter-error-site` | `store-error-site-rewrite` | HS-S0156 | committed `f2c7dbe` (+`10e99b2`,`76e9424`), sealed **approved** |
| `adapter-error-walk` | `error-site-walk-record` | HS-S0157 | committed `2ac7163`, sealed **changes-requested** |
| `front-door-reach` | `front-door-pointer` | HS-S0158 | not started — 130/130 collision waiting |
| `front-door-reach` | `evaluator-onward-links` | HS-S0159 | not started |
| `reach-walks` | `front-door-walk-record` | HS-S0160 | not started |
| `reach-walks` | `second-question-walk-records` | HS-S0161 | not started |

`baseRef` remains **`ffd0eeb7`** — hold it across any future relaunch of this project.

HS-S0157's seal is `changes-requested` on the gate alone; its four checks are now green, so a
relaunch re-enters `adapter-error-walk` at Review and should clear it without an implementer.

Still owed by this project and deferred again: **F-A** (`docs/carry-your-invariant.md` and
`docs/read-the-worked-example.md` have no entrance — initiative DoD-7/DoD-9), and HS-P0021's two
residuals, now deferred three times.
