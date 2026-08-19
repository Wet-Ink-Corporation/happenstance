---
item: HS-S0085
stage: discover
created: 2026-08-12T13:02:54.533Z
updated: 2026-08-12T13:02:54.533Z
template_sig: 86ce4036
rendered_sig: 5349385c
---

# Discover — PS-3 gets a verdict on evidence: frozen, or behind unstable-projection

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:57 | "Settle PS-3... on `projection-store-freeze`'s and `ladybug-projection-store`'s reports rather than re-derivation, and if it is gated, land the flag with RS-51-5 `doc(cfg)` treatment so the port is visible *with* its gate on docs.rs rather than absent." |
| AC-012 | `project.md`:270-272 | "PS-3 has a recorded verdict — frozen, or behind `unstable-projection` — citing the evidence `projection-store-freeze` and `ladybug-projection-store` produced." |
| AC-013 | `project.md`:273-276 | The verdict lands as a decision atom, numbered 0030+, authored via ingest. |
| `dependsOn: crate-set-decision` | manifest | The ship shape is a property of a crate already in the decided three-crate set (`happenstance-core` carries `ProjectionStore`), so this story cannot start ahead of that crate set being fixed — `_storymap.md`:158 states the same ordering ("after 1, because the ship shape is a property of a crate in the decided set"). |
| The ledger's placeholder verdict | `RUNBOOK.md`:601 | "Ship behind `unstable-projection` \| PS-3 \| the two batch shapes disagreeing at phase 6 \| 6, decided at 12." Deployment brief (`_decomposition.md`:622-624) states this project **inherits it as the default absent evidence from the two sibling projects overriding it** — it is not this story's own preference. |
| The two candidate outcomes, already written | `spec/SPECIFICATION.md`:4777-4784 | Frozen outright, or shipped behind `unstable-projection` "with a documented exemption from semver," vs. rejecting "publishing a 0.1 whose most defective surface is semver-binding." Ground the write-up here, per `_grounding.md`:83-94, rather than inventing new language for the same fork. |
| Presentation is bound here, the verdict is not | `_design.md`, "Shape decision" table (lines 509) and "Provisional clauses carried into implementation" (513-518) | Whichever way AC-012 resolves, the item renders on docs.rs *with its gate* — `#![cfg_attr(docsrs, feature(doc_cfg))]` + `#[cfg_attr(docsrs, doc(cfg(feature = "unstable-projection")))]`. `_design.md` is explicit it does **not** pre-empt PS-3 itself: "this design binds only how its outcome renders, either way." |
| IQ-2, non-occlusion | `_decomposition.md`, UX brief (lines 241-258) | An unstable item that vanishes from docs reads as "not supported" — a false claim. This is the failure mode `doc(cfg)` treatment forecloses regardless of which arm wins. |
| Sibling evidence sources, current stage | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md`:11-12, `.bklg/from-contract-to-published-library/ladybug-projection-store/project.md`:11-12 | Both siblings are `status: in-review`, `stage: design` at the time of this discovery pass — their reports (the evidence AC-012 asks this story to cite) have not landed yet. This project is rank 3 of 6 in the DAG and both projection siblings are upstream dependencies (`project.md`'s Dependencies section), so their evidence is expected to exist by the time this story reaches spec/implementation, not by discovery. |

## Questions

- **What does PS-3 actually resolve to — frozen or gated?** Genuinely open; this story does not decide it, it *cites* the sibling projects' verdicts. Deferred to spec, and beyond spec to whenever `projection-store-freeze` and `ladybug-projection-store` close and produce a citable report. `RUNBOOK.md:601`'s placeholder ("ship behind `unstable-projection`") is the default inherited absent contrary evidence, not a pre-decision.
- **The evidence does not exist yet at discovery time.** Both sibling projects are still at `stage: design`. This is not a defect in this story's discovery pass — it is the DAG's normal ordering (`project.md`'s Dependencies section lists `ladybug-projection-store` as a blocker this project's release gate already accepts) — but it means spec cannot cite a final report yet either. Deferred: spec will name the citation format and the story stays blocked at implementation until the sibling reports exist, exactly as `project.md`'s Dependencies section already states.
- **If the port ships gated, does `happenstance-core`'s manifest gain the feature, or does `happenstance` re-export it too?** Not resolved by any brief or by `_design.md` beyond "the item is the same item; what the feature adds is a gate." Deferred to spec, which can answer it mechanically once PS-3's arm is known — it does not change the presentation contract this story records.

## Decision

This story does not decide PS-3; it authors the mechanism for recording whichever verdict `projection-store-freeze` and `ladybug-projection-store` produce, and it binds the *presentation* of that verdict regardless of which arm is chosen — an `unstable-projection` feature, off by default, with RS-51-5's `doc(cfg)` treatment so a gated port is visible with its gate on docs.rs rather than silently absent (IQ-2). Spec will author the decision atom that cites the two sibling projects' reports by path once they exist, and will implement the `#[cfg_attr(docsrs, doc(cfg(...)))]` presentation contract that `_design.md`'s Shape decision table already specifies, independent of the verdict's timing.

## The wrong implementation

A decision atom asserting PS-3's verdict — "the projection port ships frozen" or "the projection port ships behind `unstable-projection`" — with well-formed frontmatter that passes `redkiln validate --kb`, but with no citation of either `projection-store-freeze`'s or `ladybug-projection-store`'s actual report: the verdict re-derived or guessed from `RUNBOOK.md:601`'s placeholder line rather than read off the sibling projects' closed evidence. This is exactly the testing brief's own named wrong implementation for AC-012 (`_decomposition.md`, Testing brief AC table, AC-012 row): "a verdict asserted without citing either sibling project's report." It satisfies AC-013's "authored through ingest" requirement and even names *a* rejected alternative, but it is wrong because the two sibling projects are the only source of the evidence AC-012 requires the verdict to be *on* — a verdict written before their reports exist, or written by re-reading the placeholder instead of the reports, is the same class of defect as the clause-audit finding a wrong `[FROZEN]` clause: a claim dressed as a decision but resting on nothing that could have falsified it. What catches it: the atom's citations resolve to specific report paths/sections in the two sibling projects' closed artifacts, not to `RUNBOOK.md:601` alone.

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
