---
item: HS-S0096
stage: discover
created: 2026-08-12T13:03:05.406Z
updated: 2026-08-12T13:03:05.406Z
template_sig: 86ce4036
rendered_sig: ba9cf32e
---

# Discover — 0.2.0 is published, with the whole gate green on the exact commit

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:68 | "Publish the decided three crates at the version `registry-surface-diff` implies, having run the *whole* `cargo xtask ci` — not `--fast` — on the literal publish commit and committed its dated output, with the four mandatory `wasm32` steps asserted against the published tree and published feature set." |
| AC-001 (this story's share) | `project.md`:225-228, `_storymap.md`:124 | "*publish the decided set*", vs. `crate-set-decision`'s "*decide and reconcile*". |
| AC-014 (this story's share) | `project.md`:277-280, `_storymap.md`:137 | "*asserted by the four `wasm32` steps on the published tree*". |
| AC-016 | `project.md`:284-287 | "`cargo xtask ci` — the whole gate, not `--fast` — passes on the publish commit, including `spec-trace`, `package-check` and the nightly `--cfg docsrs` build that `--fast` drops." |
| `dependsOn` | manifest, `_storymap.md`:172 | `rendered-page-preflight`, `registry-surface-diff`, `clause-maturity-audit`, `deferred-clause-reread`, `crate-set-decision` — every gate instrument this project builds must land before this story runs, by construction. |
| The `--fast` vs. full-gate distinction, stated three times | `project.md`:299-303 (DoD item 5), `CLAUDE.md`:256-257, `.redkiln/config.yaml`:50-60 | `--fast` drops the two feature powersets, `cargo deny`, and the nightly `--cfg docsrs` build — "the two are not interchangeable here, because `--fast` drops precisely the docs-under-all-features and feature-powerset steps AC-007 depends on." |
| Testing brief's named wrong implementation | `_decomposition.md`, Testing brief AC table, AC-016 row | "running only `--fast` and treating it as sufficient — DoD item 5 and `.redkiln/config.yaml:50-60`'s comment on what `--fast` drops... are the check against exactly this mistake." |
| AC-TEST-004 | `_decomposition.md`, Testing brief (536-538) | "The full `cargo xtask ci` run that discharges AC-016 is a committed artefact (its output, dated) distinct from any `--fast` run recorded earlier in the project's integration stage." |
| Merge-gate wiring, this project's actual automatic gates | `_decomposition.md`, Testing brief "Merge-gate commands" (481-505) | Story grain: `cargo xtask affected --base {{base}}`. Non-terminal integration grain: `cargo xtask ci --fast`. AC-016 "sits outside that automatic wiring on purpose" — it is not something `redkiln advance` fires automatically; it must be deliberately, manually invoked at the moment of release. |
| Where the release blocks | `_storymap.md`:186-191 | If `clause-maturity-audit` found a wrong `[FROZEN]` clause, or `registry-surface-diff` reported an out-of-scope break, that is a stop, a new decision atom and a re-plan — not something this story silently absorbs by rounding down a version number. |
| AC-DEP-004, migration/rollback posture | `_decomposition.md`, Deployment brief (749-765) | Migration/backfill N/A (first publish, no compatible predecessor at `0.0.0`); rollback is front-loaded — the gate blocks rather than a post-hoc undo. |
| `wasm32` mandatory-not-optional | `xtask/src/main.rs`:10-14, `CLAUDE.md`:273-276 | Four wasm32 steps are Mandatory, always run, not probe-gated — this story's obligation is running them against the *published* tree and feature set, not inventing a new check. |

## Questions

- **What if `registry-surface-diff` implies a version other than `0.2.0`?** Genuinely open until the diff actually runs against the real baseline. `_storymap.md`:190-191 already answers the procedure: "the version follows the report, and if the report implies something outside `0.2.0`, that is a re-plan rather than a rounded-down version number." Not this story's to pre-decide; recorded as the governing rule rather than deferred as unresolved.
- **Does this story itself run `cargo xtask ci`, or does it consume an already-green run from CI?** `AC-TEST-004` requires the artefact be "distinct from any `--fast` run recorded earlier," implying the full run happens *at* this story, on the literal commit about to publish, not reused from an earlier CI pass on a slightly different commit. Deferred to spec to pin the exact mechanics (local run vs. a dedicated CI job triggered on this commit), but the requirement that it be commit-exact is settled.

## Decision

This story is the irreversible act: publish `happenstance-core`, `happenstance`, `happenstance-testkit` at the version `registry-surface-diff`'s report implies, having first run the *whole* `cargo xtask ci` — never `--fast` — on the literal publish commit, with its dated output committed as a distinct artefact from any earlier `--fast` run. The four mandatory `wasm32` steps are asserted against the published tree and feature set as part of that same full run, discharging AC-014's "asserted, not assumed" half. If any upstream instrument this story depends on found a problem serious enough to block (a wrong `[FROZEN]` clause, an out-of-scope surface break), that block is honoured here rather than worked around with a version-number compromise. Spec pins the exact mechanics of running and recording the full-gate artefact on the exact commit.

## The wrong implementation

A release checklist that runs `cargo xtask ci --fast` at the project's integration stage (which `.redkiln/config.yaml`'s automatic wiring already does on every commit), sees it green, and treats that as satisfying AC-016 — publishing without ever running the full `cargo xtask ci` on the exact commit that ships. This is a realistic failure specifically because `--fast` is the gate that fires *automatically*, on every story and every integration checkpoint, while the full gate is deliberately outside that automatic wiring (`_decomposition.md`:497-499) and must be invoked by hand at the moment of release — it is easy to see "the gate has been green for weeks" and skip the one manual step that actually proves the docs-under-all-features build, the feature powersets, and `cargo deny` all still pass on the literal tree about to be published. This is exactly the testing brief's own named wrong implementation for AC-016. What catches it: AC-TEST-004 requires the full run's dated output to be a committed artefact distinct from any `--fast` run — its literal absence, or a timestamp/commit mismatch between the full-gate artefact and the actual publish commit, is what a reviewer checks for.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
