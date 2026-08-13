---
item: HS-S0091
stage: discover
created: 2026-08-12T13:02:59.822Z
updated: 2026-08-12T13:02:59.822Z
template_sig: 86ce4036
rendered_sig: 7fd39ec1
---

# Discover — The public surface is diffed against what is already on the registry

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:63 | A mandatory `cargo xtask` step diffing the decided crate set's public surface against the `0.2.0-alpha.1` registry baseline, "distinct from `CONTRIBUTING.md`:291-296's branch-point `cargo-semver-checks` job, which proves nothing about the last release"; commit a dated report; derive the version from findings; fail closed with a stated reason when the baseline is unreachable; prove it rejects a seeded breaking change. |
| AC-002 | `project.md`:229-232 | "A surface comparison runs against the `0.2.0-alpha.1` registry baseline, its report is committed in the tree, and the published version number is the one that report's findings imply. Running it against a deliberately breaking change fails it." |
| `dependsOn: crate-set-decision` | manifest, `_storymap.md`:165 | "after 1.1; unordered with 2.1–2.3" — needs the decided crate set (three crates) to know what surface to diff. |
| The existing, insufficient instrument | `CONTRIBUTING.md`:285-301 | PR-grain `cargo-semver-checks` with `--baseline-rev` pointed at the branch point already runs on every PR. Its own text: "It proves nothing about the last released version: a break merged two pull requests ago is part of the baseline and so is invisible. The registry baseline that would catch it is not available yet... Phase 12 keeps both baselines once a real `0.1.0` exists." This is exactly what AC-002's registry-baseline diff is *for* — the gap the PR-grain job admits to leaving open. |
| `project.md`'s own framing | `project.md`:47-54 | "Nothing here can currently see a semver break... there is no surface-diff step, because until `typed-layer-and-alpha-release` ships `0.2.0-alpha.1` there is no baseline to diff against." Confirms this instrument is genuinely new, not an extension of something existing. |
| Substrate dependency, not evidence | `project.md`:355-358 | "The only inbound coupling that is substrate rather than evidence is the alpha baseline... without it AC-002 has nothing to compare against and the project's central instrument does not exist." |
| Fail-closed discipline to reuse | `.kb/decisions/0010-the-suite-must-prove-itself.md`, cited at `_decomposition.md`:705-707 | "a skip is reported, never silent" — the deployment brief requires this exact discipline when the alpha baseline is unreachable, rather than a silent pass. |
| CI-implication, already decided | `_decomposition.md`, Deployment brief "CI implication" (682-701) | This step joins `xtask/src/main.rs`'s Mandatory list, not behind a tool probe (no external tool to probe for — the tool here is `cargo-semver-checks` itself against a registry, always resolvable in principle once the baseline exists). |
| Testing brief's named wrong implementation | `_decomposition.md`, Testing brief AC table, AC-002 row | "a seeded breaking change (a removed public item, a changed signature) that the diff does not flag." |
| Release-path reuse, post-`0.2.0` | `_decomposition.md`, Deployment brief "Release path if a published version changes" (lines 736-741) | "AC-002's registry-baseline surface diff, once it exists as a mandatory gate step, is what closes that gap for every release **after** this one too — each future release re-runs it against the *then-current* published version." Confirms this is a permanent gate addition, not a one-shot check for `0.2.0` alone. |
| Baseline availability at discovery time | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md`:11-12 | `status: in-review`, `stage: design` — the `0.2.0-alpha.1` registry baseline this story diffs against does not exist yet. Expected: this project is rank 3, and `typed-layer-and-alpha-release` (rank 2) is upstream substrate this project's Dependencies section already names as a blocker. |

## Questions

- **What tool implements the diff?** `CONTRIBUTING.md:291` already uses `cargo-semver-checks`; the natural choice is the same tool pointed at `--baseline-version` (the published registry crate) instead of `--baseline-rev` (a git commit). Not fully settled — deferred to spec, which should confirm `cargo-semver-checks` supports a registry-version baseline directly (it does, via `--baseline-version`) rather than requiring a separate download-and-diff mechanism.
- **The baseline doesn't exist yet at discovery time.** As with `projection-port-ship-shape`, this is expected DAG ordering, not a discovery gap — `typed-layer-and-alpha-release` is upstream and this project's Dependencies section already accepts the wait. Deferred: the "fail closed with a stated reason" behaviour is exactly the interim state this story's instrument must handle correctly, so the instrument's design should be written and tested (via the seeded-fixture unit tests) even before the real baseline is reachable.
- **Does the report's committed format need to match `xtask/src/package.rs`'s existing report style, or `spec_trace`'s?** Deferred to spec; no brief specifies a shared format, and the testing brief's fixture discipline (`_decomposition.md`:513-519) only requires synthetic `cargo-semver-checks`-shaped output for unit tests, not a specific report schema.

## Decision

A new, mandatory `cargo xtask` step diffs the decided three-crate public surface against the `0.2.0-alpha.1` registry baseline — using `cargo-semver-checks` pointed at the *published registry version*, not a branch point — commits a dated report, and derives the published version number from what it finds. This is a distinct instrument from `CONTRIBUTING.md:291-296`'s existing PR-grain job, which explicitly proves nothing about the last release; conflating the two would leave exactly the gap `CONTRIBUTING.md` already admits to. When the alpha baseline is unreachable (true until `typed-layer-and-alpha-release` ships it), the step fails closed with a stated reason rather than passing silently. Spec will confirm the tool invocation (`--baseline-version` against the registry) and add the step to `xtask/src/main.rs`'s Mandatory list with the module doc updated in the same change.

## The wrong implementation

A surface-diff step that runs `cargo-semver-checks` with `--baseline-rev` pointed at the branch point or the previous commit on `main` — the same invocation `CONTRIBUTING.md:291-296` already runs on every PR — relabelled as "the registry surface diff" and wired into `xtask/src/main.rs`'s Mandatory list. It compiles, it runs, it even fails on an obviously breaking change *within the current branch's history* — every mechanical property a naive test would check passes. It is still wrong, because it never compares against what is actually published: a breaking change merged two pull requests ago, already part of every subsequent branch's baseline, is invisible to it — precisely the limitation `CONTRIBUTING.md:295-298` states in its own words about the *existing* job. Relabelling that job rather than building a distinct registry-baseline comparison would leave AC-002's actual claim ("diffed... not asserted," "against the `0.2.0-alpha.1` registry baseline") unmet while looking, to a shallow review, like it was satisfied. What catches it: the seeded-fixture unit test this story must carry (`#[cfg(test)] mod tests`, per `xtask/src/package.rs:408-457`'s shape) constructs a scenario where a breaking change was merged to `main` several commits before the current branch point — a `--baseline-rev` diff would not flag it, while a `--baseline-version` diff against the registry crate would, and the test asserts the latter.

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
