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
