---
id: kb-reference-brand-source-locations-001
title: Where the brand identity lives, as of 2026-08-18
kind: reference
status: accepted
authority_tier: note
summary: >-
  A pointer, not a copy: the brand material is large, already written down, and should be cited
  by path. True of the working tree on 2026-08-18. assets/brand/ holds eight SVG files with a
  README stating their use, the light/dark selection pattern, the palette tokens and the binding
  rules — two horizontal lockups (ink and reversed), a stacked lockup, a wordmark, a mark for
  32px and above, a compact mark for 32px and below, a currentColor monochrome mark and a
  favicon; the wordmarks are outlined paths, so no font is required at the point of use.
  references/brand/ holds the manual and the records: brand-kit.html, the applied specification
  in ten sections and the thing to hand to anyone applying the identity; showcase.html, the
  identity across the surfaces it will actually meet; SD-0001 and SD-0002, the records; and
  logo-explorations-v3-collision.html, the rendered comparison behind the element count. The
  brand kit is the authority for how do I apply this, the SD- records for why is it like that,
  and the showcase is evidence rather than instruction. Brand decisions are numbered SD- and sit
  deliberately outside the ADR- sequence, because that corpus records contract, port and
  wire-format decisions and interleaving artwork into it makes the architecture sequence harder
  to read for no gain; SD- records carry the same structure as ADRs and are cited from .kb/
  atoms exactly as ADRs are. The identity was developed in a separate strategy workspace holding
  earlier exploration rounds not copied here, and where the two disagree the repository copies
  are the authority: the workspace holds process, the repository holds the settled result. Not
  yet true as of that date: no PNG raster fallbacks have been generated, and the trademark
  search has not been run.
depends_on: []
related:
  - kb-decision-sd-0001
  - kb-decision-sd-0002
  - kb-decision-standalone-svg-one-colourway-001
  - kb-reference-brand-geometry-palette-001
  - kb-open-question-trademark-search-001
source_paths:
  - .kb/_intake/brand-where-the-identity-lives.md
  - references/brand/
  - assets/brand/
last_reviewed: 2026-09-02
---

# Where the brand identity lives, as of 2026-08-18

This is a pointer, not a copy: the brand material below is large, already written down, and
should be cited by path rather than reproduced. Everything stated here is **true of the
working tree on 2026-08-18** — it is a snapshot, not a mirror anyone is obliged to keep
current, so a later change to the artwork or the manual does not make this atom wrong, only
dated.

## The artwork

`assets/brand/` holds eight SVG files, with a README stating their use, the light/dark
selection pattern, the palette tokens and the binding rules. The wordmarks are outlined
paths, so no font is required at the point of use.

- `happenstance-lockup-horizontal.svg` — primary, ink wordmark, for light surfaces.
- `happenstance-lockup-horizontal-reversed.svg` — paper wordmark, for ink surfaces.
- `happenstance-lockup-stacked.svg`
- `happenstance-wordmark.svg`
- `happenstance-mark.svg` — for 32px and above.
- `happenstance-mark-compact.svg` — for 32px and below.
- `happenstance-mark-mono.svg` — `currentColor`, for contexts that set their own colour.
- `favicon.svg` — the ink badge.

## The documents

`references/brand/` holds the manual and the records:

- `brand-kit.html` — the applied specification, in ten sections: the idea, mark
  construction and geometry, clear space, sizes and variants, wordmark, lockups, colour
  with measured contrast, typography, correct use, and the asset index. **This is what to
  hand to anyone applying the identity.**
- `showcase.html` — the identity across the surfaces it will actually meet: README in
  GitHub light and dark, crates.io card, docs.rs bar, favicon at 16px, organisation avatar,
  terminal, Open Graph card, presentation slide, stickers, app icons.
- `SD-0001-what-happenstance-means.md` — the name's meaning and the rules it places on
  copy; see kb-decision-sd-0001.
- `SD-0002-the-mark.md` — the mark, the wordmark, the palette, and the reasoning; see
  kb-decision-sd-0002.
- `logo-explorations-v3-collision.html` — the rendered comparison behind the mark's final
  seven-element count.

## Which source answers which question

This is the half a directory listing cannot supply. The brand kit is the authority for any
question of the form "how do I apply this". The `SD-` records are the authority for any
question of the form "why is it like that" — including the two questions a reader is most
likely to ask: why the name, and why not eight blocks. The showcase is evidence that the
identity survives real surfaces, not instruction for producing it.

## Numbering convention, and precedence

Brand decisions are numbered `SD-` and sit deliberately outside the `ADR-` sequence in
`references/adr/`. That corpus records decisions binding the contract, the ports and the
wire format, and interleaving artwork decisions into it would make the architecture
sequence harder to read for no gain. `SD-` records carry the same structure as ADRs — the
decision, what it does not claim, why each alternative lost, the costs, and an evidence
table — and are cited from `.kb/` atoms exactly as ADRs are: kb-decision-sd-0001 and
kb-decision-sd-0002 do so.

The identity was developed in a separate strategy workspace, which retains earlier
exploration rounds not copied into this repository. Where the two disagree, **the
repository copies are the authority** — the workspace holds process, the repository holds
the settled result.

## Not yet true, as of 2026-08-18

No PNG raster fallbacks have been generated. The trademark search on "happenstance" has not
been run; see kb-open-question-trademark-search-001, which gates any filing, registration,
or application of the identity to physical goods.
