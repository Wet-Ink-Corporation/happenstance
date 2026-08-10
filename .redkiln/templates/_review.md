---
item: "{{item}}"
stage: review
created: "{{created}}"
updated: "{{updated}}"
---

# Review — {{title}}

## Verdict

approved | changes-requested

## Rubric

Each dimension scored 0 (absent) → 3 (excellent). `approved` requires every dimension >= 2, with
`gate-greenness` = 3, `integration-reachability` = 3, `intent-fidelity` >= 2, and the Definition-of-Done
E2E green.

`presentation-fidelity` is the single exception to "every dimension >= 2", and the exemption is narrow.
It turns on whether the design review **applies**, never on whether it happened:

- **Ran** — approval-blocking (>= 2), scored from `_design-review.md`.
- **Does not apply** — this project declares no user-facing surface, or the repo declares no
  `design.capture`. Score 0 and **exempt** from the threshold: there is no perceptual evidence to be had,
  and holding approval hostage to a dimension nothing can ever supply would make every such project
  unapprovable.
- **Expected but missing** — the repo declares `design.capture` and this project has surfaces, yet no
  `_design-review.md` evidence exists (it was skipped, or its agent failed). Score 0 and it **still
  blocks**: verdict `changes-requested`. A required instrument going missing is not an exemption. Re-run
  the design review rather than approving around it.

In every non-running case the score still reads 0 — the exemption is from the BAR, never from the RECORD.

| Dimension | Score | Rationale |
| --------- | ----- | --------- |
| ac-coverage |  |  |
| integration-reachability |  |  |
| test-integrity |  |  |
| gate-greenness |  |  |
| brief-fidelity |  |  |
| intent-fidelity |  |  |
| presentation-fidelity |  |  |

`intent-fidelity` scores BEHAVIOR from the diff; `presentation-fidelity` scores FORM from the perceptual
evidence in `_design-review.md` — screenshots of the running app. They are separate dimensions on purpose:
blending them is how a review scores "it imports the right primitives" as brief-fidelity 3 on a component
that renders with no styling at all. If the design review did not run, score `presentation-fidelity` 0 and
say plainly that presentation was never observed — never 3 on the grounds that nothing was found.

## Evidence

What was checked and the result — including the design-intent (anchors honored), interaction-intent
(in-place, non-occlusion, preserved focus/scroll/selection, reversibility, keyboard reachability), and
presentation (composition, transience, density, hierarchy, anti-patterns — cite `_design-review.md` and the
capture images) findings.

## Required Changes

If changes-requested, the specific changes needed before re-review.
