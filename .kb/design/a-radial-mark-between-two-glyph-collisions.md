---
id: kb-design-radial-mark-collisions-001
title: A radial mark sits between the gear and the brightness glyph, and an odd count is what separates it
kind: concept
status: accepted
authority_tier: design
summary: >-
  The resolved pattern for any radial symbol intended to survive a favicon. A ring of elements
  around a filled centre sits between two of the most crowded shapes in software, and both were
  found by rasterising at 16 and 32px rather than by reasoning about the vector. The gear was
  anticipated and is mitigated by five measurable constraints: seven elements rather than twelve
  or more, gaps 2.19x the element width, a 10.5-unit hub gap so elements never touch the centre,
  corner radius 0.4 of element width, and a solid centre with no bore or aperture; squaring the
  corners was tested and rejected because it buys nothing against the second collision and walks
  back into this one. The display-brightness glyph was not anticipated and is the more dangerous,
  because near-universal eight-fold radial symmetry is that glyph and the symbol stands alone
  exactly where it is weakest — favicon, registry avatar, organisation icon. Resolved by an odd
  count: seven defeats the eight-fold reading, no icon set uses seven, and the sun-like character
  survives; one element at twelve o'clock places the gap at six and keeps the mark symmetric about
  the vertical axis, because an odd count arranged that way reads as chosen while the same count
  rotated so the gap falls at the top reads as a piece that fell off. Rejected on cost: turning
  each element 90 degrees to lie tangentially removes the brightness reading completely and holds
  better at 16px, but stops being a sun, which was the brief — it remains the documented fallback,
  with its own ceiling at ten or more elements where the dashes close into a milled edge and read
  as a loading spinner. Rejected: seven and tangential together, because tangential needs even
  spacing to read as a sequence and at seven it tiles badly. Two anti-patterns proved real:
  irregularity to signal discovery reads as a mistake rather than as intent, and judging a symbol
  at display size only hides both collisions. The defence is reasoned and rendered but not tested
  on a stranger, and until it is the collision is mitigated and unfalsified rather than closed.
depends_on: []
related:
  - kb-design-symbol-annotates-the-wordmark-001
  - kb-reference-brand-geometry-palette-001
  - kb-decision-sd-0002
source_paths:
  - .kb/_intake/brand-symbol-wordmark-lockup-pattern.md
  - .kb/_intake/brand-identity-commitments.md
  - references/brand/logo-explorations-v3-collision.html
  - references/brand/SD-0002-the-mark.md
last_reviewed: 2026-09-02
---

# A radial mark sits between the gear and the brightness glyph, and an odd count is what separates it

**Pattern:** for any radial symbol built as a ring of equal elements around a filled
centre and intended to still read correctly at favicon scale, resolve the element count
and spacing against two named collisions rather than against aesthetic judgement alone,
and verify by rasterising at 16 and 32px — both collisions below were invisible in the
vector drawing and only became obvious once rendered small. Neither was found by reasoning
about the geometry in the abstract.

**Collision one — the gear, anticipated.** A ring of blocky radial elements is close, by
construction, to the universal icon for settings. It is mitigated here by five measurable
constraints acting together rather than any single one: seven elements rather than twelve
or more (a gear reads as a gear well past seven teeth); gaps between elements at 2.19× the
element's own width (gear teeth are close to as wide as the gaps between them, so a wide
gap already reads differently); a 10.5-unit hub gap holding every element clear of the
centre disc (gear teeth are attached to the hub; these elements are not, because they
represent discrete records rather than a fused mechanism); corner radius at 0.4× element
width (a squared corner reads as machined, a rounded one does not); and a solid centre
with no bore, aperture, or inner ring, which a gear's hub always has.

**Rejected:** squaring the corners, tested directly. It buys nothing against the second
collision below and walks the mark straight back toward the first, so it was dropped
rather than kept as a stylistic option.

**Collision two — the display-brightness glyph, unanticipated and more dangerous.**
Near-universal eight-fold radial symmetry is the brightness/settings glyph used across
operating systems, and this collision is worse than the gear because the symbol is asked
to stand alone exactly where it is weakest for disambiguation: the favicon, the registry
avatar, the organisation icon — all places with no accompanying wordmark to anchor the
reading.

**Resolved by an odd count.** Seven elements defeats the eight-fold reading outright, no
mainstream icon set uses seven, and the sun-like character of the mark survives, which an
even count sacrifices. Placement matters as much as count: one element fixed at twelve
o'clock puts the resulting gap at six o'clock and keeps the whole mark symmetric about the
vertical axis. That specific arrangement reads as *chosen* — a deliberate omission at the
bottom — while the same seven elements rotated so the gap falls at the top instead reads
as *a piece that fell off*, because the top of a mark is where the eye lands first and a
gap there reads as damage rather than design.

**Rejected on cost, not on failure:** rotating each element 90° to lie tangentially around
the ring instead of radiating outward from it. This removes the brightness-glyph reading
completely and holds up better than the radiating form at 16px. It was rejected anyway
because it stops reading as a sun at all, which was the brief this mark exists to satisfy
— it remains the documented fallback if seven radiating elements prove insufficient. The
tangential form carries its own ceiling: at ten or more elements the dashes close into a
continuous milled edge and read as a loading spinner, a collision at least as bad as the
one it was chosen to avoid.

**Rejected outright:** combining seven elements with the tangential treatment. Tangential
elements depend on even, generous spacing to read as a sequence rather than a texture; at
seven elements they tile too tightly and the two mitigations interfere with each other
rather than compounding.

**Two anti-patterns proved real** in the process of reaching this pattern, and generalise
past this mark: varying element geometry to signal that a shape was *found* rather than
*designed* reads as a mistake, not as intent — equal elements at an even pitch read as an
ordered sequence, which for a system of record is both the true story and the one a
reader needs to believe. And judging a radial symbol at its display size hides both
collisions above; the favicon raster is the test case that surfaces them, not an
afterthought once the "real" design is settled.

**The boundary on this whole resolution:** the seven-element defence above is reasoned and
rendered, but it has not been tested on a stranger — shown at 16px to someone who has seen
neither the mark nor this record and asked what they think it is. Until that check runs,
treat the gear and brightness-glyph collisions as mitigated and unfalsified rather than as
closed.
