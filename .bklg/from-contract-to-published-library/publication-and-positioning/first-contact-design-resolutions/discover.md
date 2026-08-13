---
item: HS-S0087
stage: discover
created: 2026-08-12T13:02:56.144Z
updated: 2026-08-12T13:02:56.144Z
template_sig: 86ce4036
rendered_sig: 3a74a95c
---

# Discover — DT-1, DT-4, DT-5 and DT-6 are decided, each naming what lost

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:59 | "Resolve DT-1, DT-4, DT-5 and DT-6 in this project's `_design.md`, each naming the option that lost, plus the evaluator-vs-application-author persona question... and carry the settled answers into an ingested decision atom." |
| AC-011 | `project.md`:266-269 | Each of DT-1, DT-4, DT-5, DT-6 "has a written resolution in this project's `_design.md`, each naming the option that lost and why. None is left unowned at release." |
| AC-013 | `project.md`:273-276 | The settled answers land as decision atom(s), 0030+, via ingest. |
| **The resolutions already exist and are signed off.** | `_design.md`:191-325 ("DT-1" through "DT-6" sections), and Sign-off (751-768) | DT-1 (lines 191-224), DT-4 (226-261), DT-5 (263-297), DT-6 (299-324) are each fully resolved, each naming the rejected options and why they lost, each ending with a "*Resolves:*" line. Signed off by Ryan Britton, 2026-08-12, at the `/redkiln:plan` design gate. This story's remaining work is narrower than "resolve DT-1/4/5/6" — it is "carry the settled answers into an ingested decision atom," per the storymap's own phrasing. |
| DT-1's resolution | `_design.md`:196-224 | (a) proof-led lead wins; (b) edge-story-first lost (narrows audience, costs nothing to demote to one Guarantees line); (c) two entry points lost twice (crates.io has one README per crate; the persona question resolves to "evaluator is a moment in the application author's journey," which makes (c) incoherent). |
| DT-4's resolution | `_design.md`:228-261 | (c) both, in a specified shape: full statement in the Guarantees block, one link into a clause ID (not a heading), and the absence of `read_from_a_gap_position` ownership stated with its reason, citing `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`. |
| DT-5's resolution | `_design.md`:265-297 | (c) a single dated census sentence defining all four maturity words inline, sited after the status callout; (a) publish-the-ledger lost on density; (b) frozen-only lost outright — "forbidden, not merely worse" (IQ-2). |
| DT-6's resolution | `_design.md`:301-324 | (a) state it, in the existing Prior art slot plus a one-sentence reduced form on the packaged README; (b) silence lost because it would delete existing honest copy. |
| The persona question, resolved as part of DT-1 | `_design.md`:214-222 | "The evaluator is the first fifteen minutes of the application author's journey, not a fifth persona" — resolved here because DT-1 could not be decided without it (`_decomposition.md`:310-314 is the open flag this closes). |
| Numbering constraint | `project.md`:207-212 | 0030 and up. |
| Hand-authoring prohibition | `CLAUDE.md`:103-105 | Atoms come from `/redkiln:kb-ingest`, never hand-written. |
| No `.kb` atom pre-resolves these | `_grounding.md`:154-158 | Grepped, no hits for any DT id — confirms these are this project's own work, not inherited, and confirms the atom this story authors is new rather than an update to something existing. |

## Questions

- **Is there anything left for this story to *decide*?** No — `_design.md` already carries all four resolutions plus the persona question, signed off. This story's discover-stage judgement call is recognizing that the remaining work is transcription-with-fidelity (carrying `_design.md`'s prose into an ingested atom) rather than fresh resolution, and that the risk is losing fidelity in the transcription, not making a new call.
- **One atom for all four DTs, or four atoms?** Not settled by any brief. Deferred to spec: the storymap's own framing ("carry the settled answers into an ingested decision atom," singular) and the fact that DT-1's persona resolution is load-bearing for DT-1 (option (c) is incoherent without it) argue for one atom covering the whole first-contact positioning decision, but spec should confirm against `.kb/decisions/README.md`'s own guidance on atom granularity before committing.
- **Does the atom need to reproduce `_design.md`'s Density budget and Composition sections, or only the DT resolutions themselves?** Deferred to spec. The atom is a decision record (what was decided and what lost); the density/composition detail is implementation guidance for `landing-copy-and-status-truth` and stays in `_design.md`, cited by path rather than duplicated — duplicating it would create the same "two copies drift" failure mode DT-4's own resolution warns against (`_design.md`:246-247).

## Decision

DT-1, DT-4, DT-5 and DT-6 are already resolved, in full, in this project's signed-off `_design.md`, each naming what lost. This story's job is narrower than resolution: transcribe those four resolutions (and the persona-question call they turn on) into a decision atom authored through `.kb/_intake/` and `/redkiln:kb-ingest`, cite `_design.md` by section rather than re-deriving the reasoning, and verify the transcription preserves each rejected option and its stated reason rather than collapsing to "we decided X." Spec will decide atom granularity (one atom or four) and confirm the boundary between what belongs in the atom (the decision) and what stays in `_design.md` (the implementation detail landing-copy-and-status-truth consumes).

## The wrong implementation

An ingested decision atom that states DT-1's winner ("lead with proof-led identity"), DT-4's winner, DT-5's winner and DT-6's winner, passes `redkiln validate --kb`, and is correctly numbered — but drops the "what lost and why" half for one or more of them, e.g. recording DT-5's outcome as "publish a summary" without carrying forward that option (a) — publishing the full 200-row ledger — was rejected specifically on density (200 rows vs. a ~14-line first-screen budget), or that option (b) — frozen-only — is *forbidden*, not merely disfavoured, because it is IQ-2's filter-hides-what-it-filters dishonesty. This is exactly the testing brief's named wrong implementation for AC-011 (`_decomposition.md`, Testing brief AC table, AC-011 row): "a `_design.md` that states a winner without stating what lost, which fails the same bar RS-70-5 sets for doc comments (name the alternative, once)" — applied here to the atom that is supposed to carry `_design.md`'s already-correct reasoning forward without losing it in transcription. What catches it: a content review (not a compiled test) confirming each of the four atom sections names its rejected option(s) and the reason, matching `_design.md`'s own text rather than a shorter paraphrase.

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
