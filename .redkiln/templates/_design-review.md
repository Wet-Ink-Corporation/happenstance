---
item: "" # the project item id, e.g. P-0012 — YOU fill this in
stage: design-review
created: "" # ISO 8601, e.g. 2026-01-31T09:00:00Z
updated: ""
---

<!--
SHAPE REFERENCE, not a rendered template. `design-review` is not a stage, so the
CLI never renders this file and never substitutes anything: the design-review
subagent authors it whole, frontmatter included. The fields above are literal
blanks for that agent to fill — mustache placeholders would survive verbatim into
the committed artifact. Same pattern as `_ledger.md`.
-->

# Design review — &lt;project title&gt;

The perceptual review: what the built surfaces **actually look like** in the running app, judged against
the signed-off `_design.md`. This is the only record in the pipeline produced by looking rather than
reading. Every other artifact — acceptance criteria, the acceptance ledger, the reachability audit, the
DoD run, the code review — is satisfied by a completely unstyled render, so none of them can tell a
composed surface from a bare one.

## Verdict

approved | changes-requested | not-applicable

`not-applicable` means the review did **not run** — the project declares no surfaces, or the repo declares
no `design.capture` command. That is absence of evidence, not a pass. Never record it as `approved`.

## Captures

Every image looked at. A finding a reader cannot open is not actionable.

| Surface | State | Viewport | Theme | Capture | Reads as |
| ------- | ----- | -------- | ----- | ------- | -------- |
|         |       |          |       |         |          |

## Findings

Each finding: what was **seen** (not what the code says), its surface/state/viewport/theme, the capture
path, and its severity.

- **presence** — a control with no presentation, structure that does not render, an invisible or
  unreachable element, a blank surface. Blocking regardless of test results.
- **fidelity** — a deviation from the signed design's composition, transience policy, density budget,
  hierarchy, or named anti-patterns.
- **state** — a declared state, viewport, or theme in which the surface breaks.
- **holistic** — the surface satisfies each itemised check and would still leave an author confused.

## Would an author understand this screen?

The plain answer, per surface, and why. This is the question no other gate in the pipeline asks.

## Required changes

If changes-requested, what must change before re-review — stated so it can be checked against a new
capture.
