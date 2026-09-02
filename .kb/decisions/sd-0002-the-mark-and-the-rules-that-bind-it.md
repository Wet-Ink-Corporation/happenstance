---
id: kb-decision-sd-0002
title: Seven equal blocks and a lowercase wordmark, and the rules that bind anything carrying the name
kind: decision
status: accepted
authority_tier: decision
adr_id: SD-0002
reversibility: low
phase: null
supersedes: null
superseded_by: null
summary: >-
  Seven equal blocks radiate from a solid disc: the blocks are records in an append-only log —
  same size, same pitch, no privileged first one — and the disc is the boundary a query drew
  across them. The mark states in the only grammar a logo has what the name means. Equality of
  the blocks is load-bearing and was established by rendering rather than by argument:
  irregularity does not read as discovery, it reads as an error, and order is the one property
  an event store may never look casual about. The wordmark is set lowercase always, with the
  mark placed as a small annotation marker at the top right of the final e. Eight rules bind
  anything carrying the name, and they are commitments rather than preferences: seven blocks
  always; one block at twelve o'clock and the mark never rotates; scale uniformly, with no
  stretching, condensing, arching or perspective; one flat fill from the palette, with no
  gradients, shadows, glows, outlines or strokes; clear space of one block length on every side
  at every size; the wordmark stays lowercase, so recast a sentence rather than capitalise the
  name; the mark follows the word and is never placed before it; and assets ship one fixed
  colour per file. One palette commitment sits with them: Sun is a graphic colour and is never
  used for text on a light surface, which is why Ember exists — it is the amber value that
  meets AA. One calibration finding is recorded with the palette because it is the reason a
  token has the value it has: Ink was #191512 in the first cut and read as plain black, and a
  colour justified by its hex value rather than by being looked at is not yet a decision.
  Registration and physical application are gated by the open trademark-search question.
depends_on:
  - kb-decision-sd-0001
related:
  - kb-decision-standalone-svg-one-colourway-001
  - kb-design-radial-mark-collisions-001
  - kb-design-symbol-annotates-the-wordmark-001
  - kb-reference-brand-geometry-palette-001
  - kb-open-question-trademark-search-001
source_paths:
  - .kb/_intake/brand-identity-commitments.md
  - .kb/_intake/brand-symbol-wordmark-lockup-pattern.md
  - references/brand/SD-0002-the-mark.md
  - references/brand/brand-kit.html
  - assets/brand/
last_reviewed: 2026-09-02
---

# Seven equal blocks and a lowercase wordmark, and the rules that bind anything carrying the name

## The decision

Seven equal blocks radiate from a solid disc. Same size, same pitch, no privileged first
block — the blocks are records in an append-only log, and the disc is the boundary a query
drew across them. The mark states, in the only grammar a logo has, what
kb-decision-sd-0001 says the name means: the records are the thing that exists, and the
boundary is the shape they turned out to make.

Equality of the blocks is load-bearing, and it was established by rendering rather than by
argument. An earlier exploration varied the block lengths to signal that the shape was
found rather than planned, and it failed on sight: irregularity does not read as discovery,
it reads as an error, and order is the one property an event store may never look casual
about. The full comparison, and the two glyph collisions the seven-block count was chosen
to sit clear of (the gear, and the eight-fold brightness glyph), are
kb-design-radial-mark-collisions-001's — that finding is cited here, not restated.

The wordmark is set lowercase always, matching the crate name, with the mark placed as a
small annotation marker at the top right of the final `e`. Why that composition — a raised
glyph overlapping the word's advance rather than a symbol set beside it with a lockup gap —
and where it stops holding (a final letter with no pocket to nest into) is
kb-design-symbol-annotates-the-wordmark-001's.

## Rules that bind anything carrying the name

These are commitments, not preferences, and they apply to every use of the mark or
wordmark, not only to the canonical asset files:

1. **Seven blocks, always.** The count is part of the identity, not a stylistic choice.
2. **One block at twelve o'clock. The mark never rotates.** This places the gap at six and
   keeps the mark symmetric about the vertical axis.
3. **Scale uniformly.** No stretching, condensing, arching or perspective.
4. **One flat fill from the palette.** No gradients, shadows, glows, outlines or strokes.
5. **Clear space of one block length on every side**, at every size. Nothing sets inside it.
6. **The wordmark stays lowercase.** Recast a sentence rather than capitalise the name.
7. **The mark follows the word.** It annotates the final letter in the lockup and is never
   placed before the word.
8. **Assets ship one fixed colour per file** — the reason is
   kb-decision-standalone-svg-one-colourway-001's: a standalone SVG knows the operating
   system's colour-scheme preference but not the colour of the surface it is placed on, so
   a `prefers-color-scheme` media query inside the file itself can render invisibly.

The exact geometry behind rules 1–5 (pitch, block dimensions, hub gap, the compact weight
for 32px and below) and the wordmark's type and tracking are
kb-reference-brand-geometry-palette-001's.

## The palette commitment, and one calibration finding

One palette commitment sits beside the eight rules: **Sun is a graphic colour and is never
used for text on a light surface.** Ember exists for exactly this reason — it is the amber
value in the palette that meets AA contrast where Sun does not. The full token table and
the measured contrast ratios behind it are kb-reference-brand-geometry-palette-001's.

One calibration finding is recorded here, beside the commitment it justifies, rather than
with the geometry: Ink was `#191512` in the first cut and read as plain black on screen — a
seven-point spread between its red and blue channels was below the threshold at which hue
is perceived in body text, and only became visible once that spread roughly doubled. **A
colour justified by its hex value rather than by being looked at is not yet a decision.**
This is a lesson about colour perception, not glyph geometry, which is why it lands with
the palette commitment rather than in either design atom.

## What is not yet settled

Registration and any application to physical goods are gated by
kb-open-question-trademark-search-001: the trademark search on "happenstance" has not been
run.
