---
item: HS-S0125
stage: discover
created: 2026-08-12T13:03:50.125Z
updated: 2026-08-12T13:03:50.125Z
template_sig: 86ce4036
rendered_sig: b7f8e3fe
---

# Discover — The clean-checkout seam and the closeout record it feeds

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Construct the clean-checkout seam the gate must run in... and stand up `_closeout-record.md` as the artefact every later story appends cited evidence to." | `_storymap.md:53` | This story's deliverable is two things, not one: the seam *and* the companion artefact every later story writes into. |
| AC-001 (whole gate green from a clean checkout) | `project.md:192-195` | The run must exit zero on a checkout with no untracked/ignored residue, record the commit SHA, and list every `skipped` step with its absent tool. This story owns the "checkout with no residue" half; the run and its record are `whole-gate-green-on-the-assembled-tree`'s. |
| DR-1 — Clean-checkout assembly | `project.md:129-131` | "The gate runs against a checkout with no untracked or ignored residue and no path dependency standing in for a published version. A warm working tree does not satisfy this." Names the two concrete failure modes the seam must rule out. |
| Fixtures / seams note | `_decomposition.md:116-120` | "The clean-checkout itself is the one seam this project must construct deliberately: a fresh `git worktree add` or clone distinct from any warm working tree... This is a process seam, not a code mock, and it is the thing AC-001 is actually checking for." |
| Risk table, row 1 | `project.md:297` | Warns against substituting `cargo xtask ci --fast`; the wall-clock cost of the real seam plus the whole gate must be budgeted, not shortcut. |
| Merge order, step 1 | `_storymap.md:140-143` | This story is the DAG root: "First because everything downstream cites the run it produces, and because a gate failure here is the finding the whole project exists to surface." Matches `dependsOn: []`. |

## Questions

- **What mechanically counts as "no untracked or ignored residue"?** `_decomposition.md:116-120` names two acceptable constructions — a fresh `git worktree add` or a fresh clone — without picking one. **Deferred to spec**: the spec must pick one concretely and record the exact command plus the `git status --porcelain` (or equivalent) check that proves it clean, because "clean" is the thing AC-001 is actually testing and a vague description of it would make the seam unverifiable.
- **Does a path dependency currently exist anywhere in the workspace that this seam needs to rule out?** Not checked here — this story only needs to construct a checkout where none can be silently substituted; verifying the *current* manifests are free of one is folded into the gate run itself (`cargo package --list` and the workspace `Cargo.toml` resolution) rather than duplicated in this story. **Deferred to spec.**
- The evaluator-persona question (DR-10) and the DoD 13 published-tree delta (DR-4) do not touch this story — it authors no `.kb/product/` atom and states no delta. Noted here only so a reader of this file does not wonder why they are absent; both are answered in `persona-and-journey-intake-staging` and `published-tree-delta-statement` respectively.

## Decision

This story exists because a warm working tree hides exactly the defects the whole project exists to catch: an uncommitted scratch file, a stale `target/` artefact, a `[patch]` or path override standing in for a published version. Those are invisible to every sibling project's own `cargo xtask ci --fast` run, which is why DR-1 makes constructing a genuinely residue-free checkout its own requirement rather than an assumption. The spec that follows will specify: the exact isolation command (worktree or clone) and the exact residue check that proves the tree matches `HEAD` with nothing extra; and the initial shape of `_closeout-record.md` — the single companion artefact `_storymap.md:24-30` names as where the DoD set, the delta, the audit tables and the findings all converge, so AC-003's word "set" is not fourteen ledger entries in fourteen places.

## The wrong implementation

A story that runs `cargo xtask ci` inside the very session/worktree this planning work was authored in, and reports the resulting green run as "the assembled tree, clean checkout." It satisfies "the gate passed" literally, but never constructs the seam DR-1 demands: any leftover scratch file from this planning pass, a `target/` directory carrying stale build artefacts from an earlier debugging session, or a `.cargo/config.toml` / workspace `[patch]` override standing in for a published crate would all be invisible, because nothing here checked for them. It would also silently make `whole-gate-green-on-the-assembled-tree`'s citation worthless, since that story cites *this* story's checkout as its evidence of isolation. What catches it: this story's own `_ledger.md` must cite the actual isolation command run (`git worktree add` or clone, against a named remote ref) and the actual residue check's output (empty), as artefacts — not "ci passed," which proves nothing about where it ran.

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
