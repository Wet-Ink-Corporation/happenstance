---
id: kb-reference-brand-geometry-palette-001
title: The mark's geometry and the palette's measured contrast, 2026-08-18
kind: reference
status: accepted
authority_tier: note
summary: >-
  The construction numbers behind SD-0002 and the contrast ratios that justify the palette,
  measured 2026-08-18 and true of that date. Geometry: a 100-unit square build grid, blocks at a
  pitch of 360/7 = 51.4286 degrees with the first at 0; display weight disc r15, block 9 x 13,
  corner radius 3.6, block centre at radius 32; compact weight for 32px and below, disc r22, block
  15 x 20, corner radius 5.5, block centre at radius 37; hub gap 10.5 units so blocks never touch
  the disc; gap-to-block-width ratio 2.19 : 1. Wordmark: IBM Plex Sans SemiBold 600 at -2.7%
  tracking over the font's own kerning, delivered as outlined paths. The mark in the lockup: 0.40em
  diameter (0.57x cap height), top edge 0.06em above the cap line, left edge 0.06em inside the
  word's advance — the constants two staged documents both carried and which live here once.
  Palette, WCAG 2.1 relative luminance computed 2026-08-18: Sun #FFB627 at 1.71:1 on paper and
  10.34:1 on ink; Deep #E08700 at 2.68:1 and 6.60:1; Ember #A85B00 at 4.92:1 and 3.13:1; Ink
  #2A211B at 15.38:1 on paper; Paper #FFFCF4 at 15.38:1 on ink. Sun on GitHub dark #0D1117 is
  10.79:1 and on docs.rs white 1.75:1. Ink was #191512 in the first cut and measured warm while
  reading as plain black. This atom records the numbers and draws no conclusion from them;
  kb-decision-sd-0002 is where the conclusions are.
depends_on: []
related:
  - kb-design-symbol-annotates-the-wordmark-001
  - kb-design-radial-mark-collisions-001
  - kb-decision-sd-0002
  - kb-reference-brand-source-locations-001
source_paths:
  - .kb/_intake/brand-identity-commitments.md
  - .kb/_intake/brand-symbol-wordmark-lockup-pattern.md
  - references/brand/SD-0002-the-mark.md
  - references/brand/brand-kit.html
  - assets/brand/
last_reviewed: 2026-09-02
---

# The mark's geometry and the palette's measured contrast, 2026-08-18

## What this is a pointer to

The applied specification lives in `references/brand/brand-kit.html`; the reasoning behind
these numbers lives in `references/brand/SD-0002-the-mark.md`; the rendered artwork is
`assets/brand/`. This atom is the citable snapshot of the numbers those documents carry —
so a decision can rest on a figure without restating how it was produced, and so the same
figure can be cited from more than one place without being copied more than once. It was
true of the working tree on 2026-08-18, the date both the geometry and the contrast were
measured, and is cited rather than restated by `kb-design-symbol-annotates-the-wordmark-001`,
`kb-design-radial-mark-collisions-001` and `kb-decision-sd-0002`.

## Mark geometry

Built on a 100-unit square grid. Seven positions at a pitch of 360° / 7 = 51.4286°, the
first at 0°. Two weights, switched at 32px:

- **Display weight** (above 32px): centre disc `r15`; block `9 × 13`, corner radius `3.6`;
  block centre at radius `32`.
- **Compact weight** (32px and below): centre disc `r22`; block `15 × 20`, corner radius
  `5.5`; block centre at radius `37`.

Hub gap is `10.5` units on both weights — the blocks never touch the disc. The gap between
adjacent blocks is `2.19×` the block's own width.

## Wordmark and lockup geometry

The wordmark is IBM Plex Sans SemiBold (weight 600), tracking −2.7% applied on top of the
font's own kerning, delivered as outlined paths rather than as live type — no font is
required at the point of use. In the horizontal lockup, the mark's diameter is 0.40em,
equal to 0.57× the wordmark's cap height; its top edge sits 0.06em above the cap line, and
its left edge sits 0.06em inside the word's advance, so it overlaps rather than trails it.
These four numbers are what `kb-design-symbol-annotates-the-wordmark-001` cites when it
describes the annotation placement; they are not restated there.

## Palette and measured contrast

WCAG 2.1 relative-luminance contrast, computed 2026-08-18:

| Token | Hex | On paper | On ink |
| --- | --- | --- | --- |
| Sun | `#FFB627` | 1.71:1 | 10.34:1 |
| Deep | `#E08700` | 2.68:1 | 6.60:1 |
| Ember | `#A85B00` | 4.92:1 | 3.13:1 |
| Ink | `#2A211B` | 15.38:1 | — |
| Paper | `#FFFCF4` | — | 15.38:1 |

Two off-palette surfaces were also measured: Sun on GitHub's dark background `#0D1117` is
10.79:1; Sun on docs.rs's white background is 1.75:1.

Ink's first cut was `#191512`, not `#2A211B`. It was chosen for warmth and measured as
such — a roughly seven-point spread between its red and blue channels — but read on
screen as plain black; that spread is below the threshold at which the human eye
perceives hue in a body-text-sized swatch. `#2A211B` roughly doubles the spread. That is
the measurement; the commitment drawn from it — that a colour justified by its hex value
rather than by being looked at is not yet a decision — is `kb-decision-sd-0002`'s, not
this atom's.

## What this atom does not do

It draws no conclusion. It does not say which colourway ships where, why the mark takes
seven elements rather than some other count, or why the symbol sits where it does in the
lockup — those are `kb-decision-sd-0002`, `kb-design-radial-mark-collisions-001` and
`kb-design-symbol-annotates-the-wordmark-001` respectively, each citing the relevant
number from here rather than restating it. If a token in this table moves, this atom is
superseded by a later measurement dated after 2026-08-18; nothing here is expected to be
kept current by hand.
