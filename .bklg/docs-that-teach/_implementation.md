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
| 1 | checked-documentation-surface | HS-P0020 | — | no | in-progress | — | — |
| 2 | page-need-discipline | HS-P0021 | 1 | no | pending | — | — |
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
