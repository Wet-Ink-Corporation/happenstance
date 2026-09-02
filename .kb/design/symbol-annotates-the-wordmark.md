---
id: kb-design-symbol-annotates-the-wordmark-001
title: A symbol beside a wordmark is set as an annotation marker, not as a second object
kind: concept
status: accepted
authority_tier: design
summary: >-
  The resolved pattern for any lockup pairing a symbol with a wordmark. Set the symbol small and
  raised, overlapping the word's advance, nested into the open counter-space at the top right of
  the final letter; for happenstance that is 0.40em diameter with the top edge 0.06em above the
  cap line and the left edge 0.06em inside the advance (kb-reference-brand-geometry-palette-001
  holds the constants). Two arrangements were rejected by rendering rather than by argument: the
  symbol to the left with a decorative element closing the right, which read as two objects
  bracketing a word with neither clearly the subject; and the symbol to the right at lockup scale
  with a normal gap, which read as a logo tacked onto a word — the size was not the problem, the
  gap was. The mechanism is what makes it portable: a glyph sitting clear of a word after a gap is
  read as a second object however well proportioned it is, while a small raised glyph overlapping
  the advance is parsed as a footnote marker, which is a typographic relationship rather than a
  compositional one. Holds when the wordmark's final letter is round or open — e, o, c, a —
  leaving a pocket at its top right. Stops holding when the final letter is a full-height
  vertical — l, k, t, d — where no pocket exists, the symbol collides or floats, and a
  conventional lockup gap is the better answer.
depends_on: []
related:
  - kb-reference-brand-geometry-palette-001
  - kb-design-radial-mark-collisions-001
  - kb-decision-sd-0002
source_paths:
  - .kb/_intake/brand-symbol-wordmark-lockup-pattern.md
  - references/brand/SD-0002-the-mark.md
  - references/brand/logo-explorations-v3-collision.html
  - assets/brand/
last_reviewed: 2026-09-02
---

# A symbol beside a wordmark is set as an annotation marker, not as a second object

**Pattern:** set the symbol small and raised, overlapping the word's advance, nested into
the open counter-space at the top right of the final letter, rather than placed beside the
word at full lockup scale with ordinary letter spacing around it. For happenstance the mark
sits at 0.40em diameter — 0.57× the wordmark's cap height — with its top edge 0.06em above
the cap line and its left edge 0.06em inside the word's advance; `kb-reference-brand-geometry-palette-001`
carries those constants and is cited rather than restated here. The class of surface this
applies to is any lockup pairing one symbol with one wordmark, not only this identity's own.

**Rejected:** the symbol placed to the left of the word, with a decorative element closing
the composition on the right. Rendered and looked at, it read as two yellow objects
bracketing the word, with neither clearly the subject of the lockup.

**Rejected:** the symbol placed to the right of the word at full lockup scale (0.90em),
separated by an ordinary lockup gap. This read as a logo tacked onto a word — precisely
the failure the eventual pattern exists to avoid. The size of the symbol was not what
caused the failure; the gap was. Shrinking the gap to nothing, while keeping the symbol at
lockup scale, was tried and still failed — the gap alone was not the whole mechanism
either, which is what pushed the resolution toward overlap rather than proximity.

**Mitigates:** the "logo tacked onto a word" failure mode observed directly in the
rejected arrangement above (`references/brand/logo-explorations-v3-collision.html`). The
mechanism behind the fix is what makes the pattern reusable past this one identity: a
glyph that sits clear of a word, on the far side of a gap, is read as a second object no
matter how carefully its size and position are tuned relative to the word. A small,
raised glyph that overlaps the word's own advance is read differently — not as an object
beside the word but as a mark belonging to it, the same way a footnote marker is read as
part of the word it follows rather than as a second word next to it. That is a
typographic relationship, established by overlap and position, rather than a
compositional one established by spacing and balance. Anything that tries to fix this
family of failure by adjusting spacing or size while keeping the symbol clear of the word
is solving the wrong variable.

**Holds when:** the wordmark's final letter is round or open — `e`, `o`, `c`, `a` in Latin
type — because those letterforms leave a real pocket of open counter-space at their top
right for the symbol to nest into without visually colliding with the letter's own stroke.
happenstance's own wordmark ends in `e`, which is why the pattern was viable here at all
rather than merely appealing in principle.

**Stops holding when:** the final letter is a full-height vertical stroke — `l`, `k`, `t`,
`d` — where no such pocket exists. Nested there, the symbol either collides with the
stroke or floats disconnected from it, reproducing the "two objects" failure the pattern
was built to avoid, just moved one letter later. A conventional lockup gap, with the
symbol at full scale and clear of the word, is the better choice for that case — the
annotation pattern is not a universal replacement for a spaced lockup, only the resolution
for the specific letterform shape happenstance's own wordmark presented.
