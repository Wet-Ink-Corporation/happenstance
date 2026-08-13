---
item: HS-P0015
stage: intake
created: 2026-08-12T03:23:22.812Z
updated: 2026-08-12T03:23:22.812Z
template_sig: ab516678
rendered_sig: 6f96f60c
---

# Intake Brief — The unlike batch shape, and the freeze verdict

## Problem

A freeze tested only against the shapes that produced it has been tested against
nothing. `projection-store-freeze` will freeze `ProjectionStore` against SQL-shaped
batches; the axis it is most likely to be wrong about is **batch shape and
transactional seam**, and the far end of that axis is a graph store. `lbug`
additionally presents a blocking API to a non-blocking port, which is a second way
the freeze can be wrong that no SQL adapter can reveal. The initiative's own risk
register states the mitigation: the freeze is not called earned until a
structurally unlike batch shape has passed.

## Desired Outcome

The projection suite is green on a non-SQL batch — or fails with a named,
explained failure — and **the written verdict on whether the freeze held exists,
stating what it was checked against and when** (DoD 8). A verdict of *"it did not
hold"* is a result, not a failure of this project; the outcome this project cannot
produce is silence.

## Constraints

- **Depends on** `projection-store-freeze`. Independent of the other adapter
  projects; blocks `publication-and-positioning`.
- **Projection store only.** `happenstance-ladybug` implements no event store, and
  acquiring one is out of scope at any grain.
- **`lbug` blocks; the port does not.** Whatever bridges the two is the crux of
  ADR-0025 and cannot be hidden in an adapter's private helper — if the port cannot
  express it, that is the freeze not holding.
- **An adapter that compiles but has not run the conformance suite is not an
  adapter.**
- **Non-goals**, each naming its owner: freezing the port →
  `projection-store-freeze`; **acting** on a "did not hold" verdict by amending a
  published surface → a new decision atom and a re-plan, per the charter's
  out-of-scope list; any event store → every sibling adapter project.

## Open Questions

- **ADR-0025** — checkpoint placement, how a projection expresses graph mutations,
  and how `lbug`'s blocking API meets a non-blocking port.
- **What counts as "structurally unlike"?** The verdict is only as strong as this
  definition, and it must be written down *before* the suite is run, not chosen
  afterwards to match the result.
- **PS-34's E0195 spelling trap**, re-tested by a third implementer — does the port
  survive a fresh pair of hands, or was it only ever navigable by the people who
  wrote it?
- Whether a "did not hold" verdict arriving *after* `0.2.0` is published creates
  the same surface-change exposure that the retention project was constrained to
  avoid. It should not — this project blocks publication — but the verdict's
  timing relative to the release must be explicit in the design brief.

## Proof artefact

**The written freeze verdict, dated and naming what it was checked against**, on
the back of the projection suite run against a graph-shaped batch. This would not
exist if the design were wrong: a port frozen against SQL batches and never
re-tested produces no artefact at all, and the verdict is the only thing that can
say *"held"* or *"did not hold"* with something behind it. The verdict document is
required whichever way the run goes — that asymmetry is what makes it evidence
rather than a celebration.

## Clauses

- **PS-*** `[PROVISIONAL]` → frozen by `projection-store-freeze`; **re-tested here**.
  This project amends none of them — it reports on whether the freeze held.
- **PS-2** — the single gate under thirteen rows; this is where it meets a shape it
  was not designed against.
- **PS-34** — the E0195 spelling trap, re-tested by a third implementer.
- Settled by **ADR-0025**.
- Nothing `[FROZEN]` is amended. If the verdict is "did not hold", the response is a
  new decision atom and a re-plan.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` — where each box's rationale is
written — and will not leave `intake` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
