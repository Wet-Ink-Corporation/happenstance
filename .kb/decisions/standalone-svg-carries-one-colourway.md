---
id: kb-decision-standalone-svg-one-colourway-001
title: A standalone SVG carries one fixed colour, and the surface is selected at the point of use
kind: decision
status: accepted
authority_tier: decision
adr_id: SD-0002
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  A standalone SVG must carry one fixed colour, and a prefers-color-scheme media query must not be
  placed inside one. An earlier cut of the lockups embedded @media (prefers-color-scheme: dark) so
  a single file could serve both surfaces, and it failed silently: a standalone SVG knows the
  operating system's preference but not the colour of the surface it was placed on, so a light page
  opened on a machine set to dark mode rendered the wordmark cream on cream, invisible, with no
  error anywhere. The rule that replaces it is ship one file per colourway and select at the point
  of use, where the background is known — a picture element with a source carrying the media query
  and an img carrying the light default. The page knows its own background; the file does not.
  This is recorded as its own atom rather than as a clause of the mark decision because its scope
  is wider than the mark: it generalises past logos to any SVG asset shipped for use on a surface
  it does not control, which in this repository includes README artwork, a docs.rs header and a
  crates.io card. It shares SD-0002's record with kb-decision-sd-0002, which carries the mark
  itself and cites this atom for the reasoning behind its eighth binding rule.
depends_on: []
related: []
source_paths:
  - .kb/_intake/brand-identity-commitments.md
  - references/brand/SD-0002-the-mark.md
  - assets/brand/
last_reviewed: 2026-09-02
---

# A standalone SVG carries one fixed colour, and the surface is selected at the point of use

## Decision

A standalone SVG asset must carry exactly one fixed colour, and a
`prefers-color-scheme` media query must not be embedded inside one. This binds any SVG
shipped for placement on a surface the file itself does not control — not only the
happenstance mark and wordmark, but any brand or documentation asset delivered as a
standalone file.

## What failed, and why

An earlier cut of the lockup SVGs embedded `@media (prefers-color-scheme: dark)` directly
in the file, on the reasoning that one file could then serve both a light and a dark
surface without the consuming page needing to choose. It failed, and it failed silently:
a standalone SVG can read the operating system's colour-scheme preference, but it cannot
read the colour of the surface it has actually been placed on. Those two facts are
independent — a user can run a dark-mode operating system and still be looking at a page
with a light background, and the reverse. The embedded query answered the wrong question.
The concrete failure: a light page, opened on a machine set to dark mode, rendered the
cream-coloured "dark surface" wordmark on its own light-cream page background — invisible,
with no error raised anywhere in the pipeline. Nothing in an SVG's own rendering model
detects or reports this; the file renders exactly as instructed and is simply unreadable.

## What replaces it

Ship one file per colourway — one for use on light surfaces, one for use on dark — and
select between them at the point of use, where the actual background is known:

```html
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/brand/happenstance-lockup-horizontal-reversed.svg">
  <img src="assets/brand/happenstance-lockup-horizontal.svg" alt="happenstance">
</picture>
```

The `<picture>` element's `source` still uses the `prefers-color-scheme` media query, but
it is now evaluated by the *page*, which is positioned to combine the operating-system
signal with its own known background, rather than by the *file*, which has only the first
half of that information. The page knows its own background; the file does not, and no
amount of clever CSS inside the SVG changes that structural fact.

## Why this is its own record rather than a clause of the mark decision

The scope of this rule is wider than the mark. It generalises past logos entirely to any
SVG asset shipped for use on a surface it does not control — in this repository that
already includes README artwork, a docs.rs header image, and a crates.io card, none of
which is a logo question at all. Filing the rule as a clause inside the mark's own
decision record would bury a general delivery rule inside a document whose scope is
narrower than the rule itself, and a reader with a `<picture>`-selection question about,
say, a docs.rs header would have no reason to look inside a record about block geometry
and palette contrast to find it.

This record shares its `SD-0002` identifier with `kb-decision-sd-0002`, which carries the
mark's construction and binding rules and is the companion record — the sharing is
declared in both bodies, and the two are kept as adjacent files precisely so the
relationship is visible in a directory listing even without opening either one.

## Alternatives rejected

Keeping the embedded media query and treating the cream-on-cream failure as a rare edge
case, rejected because the failure mode is silent and produces no error a build or a
review would catch — an asset that is sometimes invisible with no diagnostic is worse
than one that is uniformly wrong, because the uniform failure gets found immediately. A
single "safe" mid-contrast colourway that reads adequately on both light and dark
surfaces was also considered and rejected: it satisfies neither surface's contrast
measurements as well as a matched colourway does, trading a real, verified deficiency on
both surfaces for the appearance of solving the problem once.
