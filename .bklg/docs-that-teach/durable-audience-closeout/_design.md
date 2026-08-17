---
item: HS-P0025
stage: design
created: 2026-08-17T04:00:07.155Z
updated: 2026-08-17T04:00:07.155Z
---

# API surface design — The Durable, Reconciled Audience

## Items

**This project changes no public API.** It touches no crate under `crates/`, adds no
`pub` item, and compiles nothing new. Its work is entirely in `.kb/` atoms and backlog
markdown: reconciling and promoting personas/journeys, appending to
`.kb/maps/domain-map.md` and `.kb/maps/open-questions-index.md`, writing a
reconciliation record and a DoD re-observation ledger, and merging the sibling branch
forward. None of that is a Rust `pub` item, so there is nothing for `## Signatures`,
`## Shape decision`, `## Placement and re-export`, `## Visibility and stability`,
`## What it costs a caller`, `## What a user meets first`, `## The states the API must
express`, or `## The doctest` to review.

This is confirmed, not assumed: `project.md`'s risk table states directly that "this
project owns no design tension... its surface is the promoted atoms themselves" (see
`project.md`, row *"This project owns no design tension, so its `_design.md` looks
vacuous"*), and `_decomposition.md` states "This project ships no screen. Its
user-facing surface is four artefacts" — (1) promoted `.kb/product/` persona and
journey atoms, (2) navigation appends to `.kb/maps/domain-map.md` and
`.kb/maps/open-questions-index.md`, (3) a written reconciliation record, and (4) a DoD
re-observation ledger plus the DT-1…DT-10 audit table. All four are plain markdown
text, read by future charter authors, closeout reviewers and maintainers — never
rendered as an application screen, a route, or a DOM node. There is no CSS/SCSS file,
no `.tsx`/`.jsx` file, and no design-token or design-system reference anywhere in this
repository outside of markdown prose (checked by glob and by grep over the whole
worktree); this is a text-corpus and backlog-tooling repository with no web frontend
for any project to ship a screen into.

What this project's "surface" actually is, in the sense the promoted atoms must serve:
a persona atom's shape (goal, context, current alternative, fear — per
`.kb/product/README.md:6-13`) and a journey atom's shape are what the *next*
initiative's charter author reads to frame acceptance criteria from. That is the bar
this closeout is designed against, and it is a prose/schema bar (`kind`,
`authority_tier`, `source_paths`), not a layout, hierarchy, density, or chrome
decision — there is no rendered surface for any of those concepts to apply to.

```yaml
# no items — no public API surface, no rendered UI surface
```

## Signatures

N/A — no public surface.

## Shape decision

N/A — no public surface.

## Placement and re-export

N/A — no public surface.

## Visibility and stability

N/A — no public surface.

## What it costs a caller

N/A — no public surface.

## What a user meets first

N/A — no public surface.

## The states the API must express

N/A — no public surface.

## Anti-patterns

N/A — no public surface.

## The doctest

N/A — no public surface.

## Sign-off

No public surface. Nothing here requires human sign-off on a screen, a token, or
an API signature; sign-off on the reconciliation record and the promoted atoms happens
through the project's own review/closeout gates, not through this file.

**Design gate cleared: approved by Ryan Britton (repository owner), 2026-08-17, with no
conditions.** Recorded via `redkiln advance HS-P0025 --verdict approved --stay --apply`.
The verifier found no gaps. `hasSurface: false` was recorded explicitly rather than the
project being silently skipped.

This project owns no design tension, which is deliberate and stated in the decomposition:
it carries owned requirements rather than unresolved interaction choices, which is exactly
why the charter's tension table did not name it.
