---
item: "{{item}}"
stage: retrospective
created: "{{created}}"
updated: "{{updated}}"
---

# Retrospective — {{title}}

<!--
These seven sections are the ones the closeout workflow's retrospective agent is
asked to write, in this order. They were previously four unrelated ones — the
prompt enumerated its own list inline and pointed at a plugin asset whose
headings are different again, so the rendered template and the thing written
into it had no section in common. Change one, change the other; the reverse
`template-section-authored` check in `check-prompts.mjs` fails the build
otherwise.

House shape: a one-line recap, then dense evidence tables (one row per
project/story, columns for what was CLAIMED and what was VERIFIED), then prose
only where a table cannot carry the reasoning.
-->

## Recap

One paragraph: what this initiative set out to do.

## Delivered vs descoped

One row per project. State explicitly any planned scope **not** delivered, with
the reason — partial delivery is a fact to record, not one to paper over.

| Project | Stage | Delivered | Descoped (and why) |
| ------- | ----- | --------- | ------------------ |
|         |       |           |                    |

## Coverage

Every initiative AC/BR mapped to the project that closed it. Name any that no
project covers.

| Initiative AC/BR | Project | Evidence |
| ---------------- | ------- | -------- |
|                  |         |          |

## Definition of Done

Grade the initiative against its **own** Definition of Done — the DoD scenarios
in the initiative item — not against a stack of per-project review scores. For
each project, whether the whole-feature DoD ran green with every capability
mounted and reachable. Call out any DoD scenario skipped or left `test.fixme`,
and any capability that shipped unmounted. A green project review does not
substitute for a green DoD: if the assembled feature was never exercised end to
end, say so plainly rather than inverting the gap into a virtue.

## Knowledge harvest

The `.kb` atoms authored during this closeout, and what each one carries
forward. Recorded on the item itself with `redkiln record-links <id> --atom`.

| Atom | Kind | Distilled from |
| ---- | ---- | -------------- |
|      |      |                |

## Lessons learned

Concrete bullets — what worked, what was hard, what blocked and how it resolved.
Mined from the run reports and each project's `report.md`, not recalled.

## Known issues and downstream handoffs

What is promoted OUT of this initiative: open defects, deferred work, and who or
what picks each one up.
