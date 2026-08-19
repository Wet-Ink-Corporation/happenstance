---
item: HS-S0084
stage: discover
created: 2026-08-12T13:02:53.616Z
updated: 2026-08-12T13:02:53.616Z
template_sig: 86ce4036
rendered_sig: "91272406"
---

# Discover — The crate set for 0.2.0 is a recorded decision, not a default

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:56 | "Record, as an ingested decision atom, that `0.2.0` ships exactly `happenstance-core`, `happenstance`, `happenstance-testkit`, naming the four-crate alternative at `RUNBOOK.md`:4450-4451 as rejected and its cost, and assert `xtask/src/package.rs`'s derived set and its `PUBLISHABLE` intention list agree with no reconciliation failure." |
| AC-001 | `project.md`:225-228 | The release exists at `0.2.0` for a set that matches a recorded decision naming the rejected alternative; `reconcile` must agree with no failure. |
| AC-013 | `project.md`:273-276 | Every settled answer lands as a decision atom authored through the ingest path, numbered 0030 or above (0017–0028 allocated to siblings, 0029 on disk). |
| Deployment brief's crate-set decision | `_decomposition.md`, Deployment brief §"The crate-set decision" (lines 574-613) | Already resolves the question: three crates, four-crate alternative from `RUNBOOK.md`:4450-4451 named and rejected on three grounds — `PUBLISHABLE`/`CLAUDE.md` already encode three, a fourth crate needs its own licence/README pair and `package-check` doesn't cover it, and `RUNBOOK.md`'s phase-12 section is independently known stale (its own `#the-46-provisional-clauses` anchor is superseded). |
| `xtask/src/package.rs:86` | `PUBLISHABLE` constant | `["happenstance-core", "happenstance", "happenstance-testkit"]` — the intention list `reconcile` (`:172-218`) already fails on disagreement with `cargo metadata`'s derived set (`:226-241`). This story extends nothing; it authors the atom that makes the existing constant a recorded decision rather than an unexamined default. |
| `CLAUDE.md` Commands section | `CLAUDE.md`:280 | "a `cargo package --list` assertion that each of the **three** publishable crates carries both licence files and a README" — corroborates the three-crate reading independently of `package.rs`. |
| `_grounding.md`, "The registry crate-set tension" | `_grounding.md`:96-125 | The same tension the deployment brief resolves; states the working default is three "unless the deployment brief finds new information — but it must say so, not inherit the runbook's stale four silently." Confirms this is a live, previously-flagged fork, not a discovery-stage invention. |
| AC-013's numbering constraint | `project.md`:207-212 | Atoms take the next free number above the 0017–0028/0029 allocation — 0030 and up. |
| Hand-authoring prohibition | `CLAUDE.md`:103-105 | Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, not by hand; the first hand-authoring attempt was reverted (`0269720`). |

## Questions

- **Is "three crates" actually settled, or still the working default the task brief calls out?** The deployment brief (`_decomposition.md`:576-613) states it as a decision with a named rejected alternative and reasons, not a placeholder — so for this story it is **answered**, not open. The task brief's caution ("which crates publish at 0.2.0... it is not settled") is about the *project* as a whole before this brief existed; within this story's scope the brief has since settled it. Recorded here so a later reader does not re-litigate it as if nothing had decided it.
- **Does deciding early retroactively change scope for the four adapter projects?** `_decomposition.md`:302-309 flags that a crate-set answer larger than three expands adapter-project scope. Answered by staying at three: no adapter project gains a publish-ready package obligation from this decision.
- **Deferred to spec:** the exact wording of the atom's "what lost" section — the deployment brief gives the three rejection reasons in prose; spec turns them into the atom's Alternatives-rejected section without re-deriving them.

## Decision

`0.2.0` ships exactly three crates — `happenstance-core`, `happenstance`, `happenstance-testkit` — matching `xtask/src/package.rs:86`'s `PUBLISHABLE` constant and `CLAUDE.md`'s own Commands section. The four-crate alternative from `RUNBOOK.md:4450-4451` (adding `happenstance-sqlite`) is rejected and named as rejected, because `RUNBOOK.md`'s phase-12 section predates the decomposition gate's decision and is independently known stale (its ledger anchor still cites "46" provisional clauses against a corrected 49). Spec will author the ingested decision atom (0030 or above) recording this, and a verification step that actually runs `xtask/src/package.rs`'s `reconcile` at the decision commit and cites its output — not merely restates `PUBLISHABLE`'s current contents — so the atom's claim of agreement is checked rather than assumed.

## The wrong implementation

An ingested decision atom that states "three crates, matching `PUBLISHABLE`" and cites `xtask/src/package.rs:86` by path — `redkiln validate --kb` passes, the frontmatter is well-formed, the four-crate alternative is named as rejected — but the story is marked done without anyone having actually run `cargo xtask package-check` (or `reconcile` directly) at the decision commit to confirm the derived set and the intention list still agree. `PUBLISHABLE` could already have drifted (a `publish = false` quietly dropped from an adapter crate, or a name added without updating the constant) and the atom would still read as correct prose while the tree it describes disagrees with it. AC-001's own text requires "no reconciliation failure," which is a property of running the check, not of writing a sentence that assumes it. What catches it: citing the actual `cargo xtask package-check` (or `reconcile`) run output in the story's `_ledger.md`, the same discipline `xtask/src/package.rs`'s own module doc names as the reason both the derived set and the hand list are kept — "a list nothing reconciles is the same decorative gate one level up" (`xtask/src/package.rs:30-31`).

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
