# SD-0002 — The mark is a log that closed into a boundary

**Status:** accepted
**Date:** 2026-08-18
**Decides:** the pictorial mark, the wordmark, and the palette — for every surface
that shows the project a face
**Supersedes:** nothing
**Related:** [`SD-0001`](SD-0001-what-happenstance-means.md) (what the name means),
[`naming.md`](naming.md) (the rules for using it),
[`brand-kit.html`](brand-kit.html) (the applied specification),
[ADR-0006](../adr/0006-bare-name-to-the-typed-layer.md) (the crate naming rule)

---

## The decision

**Seven equal blocks radiate from a solid disc. The blocks are records in an
append-only log — same size, same pitch, no privileged first one. The disc is the
boundary a query drew across them.**

The count is seven and not eight, and the reason is defensive rather than
expressive: **eight-fold radial symmetry is the display-brightness glyph**, and
that collision is recorded below. Seven keeps the sun, and no operating system
icon set uses it. One block sits at twelve o'clock so the gap falls at six, which
preserves symmetry about the vertical axis — an odd count arranged this way reads
as chosen, where the same count rotated reads as a piece that fell off.

The mark says what SD-0001 says, in the only grammar a logo has. A classical
aggregate fixes its consistency boundary when the schema is written: that would be
a ring drawn first and filled afterwards. Here the records are the thing that
exists, and the boundary is the shape they turned out to make. **Deliberate but not
pre-declared** — the same phrase `naming.md` insists on holding precisely.

Equality of the blocks is the load-bearing detail. An earlier exploration varied
the ray lengths to signal "the shape was found, not planned," and it failed for a
reason worth recording: **irregularity does not read as discovery, it reads as an
error**, and order is the one property an event store may never look casual about.
Equal blocks at an even pitch say *ordered sequence*, which is both true and the
thing a reader needs to believe.

It is also a sun, and the sun is warm, which is unclaimed territory: `cqrs-es`,
`disintegrate`, `evento` and `esrs` are uniformly grey, blue or monochrome. The
warmth is a shelf position, not a mood.

The wordmark is lowercase always, matching the crate name, and **the mark is set as
an annotation marker at the top right of the final `e`** — 0.40em across, its top
just above the cap line, its left edge 0.06em *inside* the word's advance so that it
nests in the pocket the round `e` leaves open.

That placement took three attempts and the failures are the useful part. The first
lockup put the mark on the left and closed the word with a small trailing block:
two yellow objects bracketing the word, neither clearly the subject. Moving the
mark to the right and deleting the block fixed that and introduced a subtler
problem — at lockup scale, sitting clear of the word after a gap, **it read as a
logo tacked onto a word rather than as one object.** Shrinking it to under
six-tenths of cap height and tucking it into the `e` is what finally resolved it,
and the reason is a typographic one rather than a compositional one: **a small
raised glyph overlapping the word's advance is read as belonging to the word.** It
is a footnote marker, which is the register this project already writes in — and it
is the one position in which a sun-shaped glyph cannot be mistaken for a
display-brightness control, because brightness controls do not annotate words. Each
of the three arrangements was rendered and looked at; none was resolved by argument.

## What this record does not claim

**The mark does not prove anything, and no reader will decode it unaided.** A
person meeting it cold sees a friendly yellow sun. The argument above is available
to anyone who asks and is written down so that the answer is a citation rather than
a conversation — the same reason ADR-0005's availability justification is quoted
rather than hidden in SD-0001. It is not a claim that the geometry communicates on
its own.

**It also does not settle whether a Rust library needs a visual identity at all.**
That was treated as a real question rather than an assumed yes, and it remains one.
This record answers *what the mark is if there is one*, which is a smaller question
and the only one that was actually blocking.

## Why it lost, for each option not taken

**A wordmark and nothing else.** The honest minimum, and the strongest argument
against everything here: a library's entire visible surface is crates.io, docs.rs,
GitHub and a README, none of which reward a symbol. Rejected because two of those
four surfaces render a square avatar whether or not one is supplied, and an
unsupplied one is a default grey glyph. The choice was never *mark or no mark*; it
was *chosen mark or assigned placeholder*.

**A mechanism mark** — some construction on boundary, condition, query, append.
Rejected for the same reason SD-0001 rejected a mechanism *name*: it describes the
how and leaves the why homeless, and the differentiator here is a why.

**The asterisk.** `*` is the query wildcard, a sunburst, and a footnote marker at
once, and `happenstance*` compresses the project's entire falsification posture
into one character. It came second and it came close. Rejected because since
roughly 2023 a multi-point star in a warm colour has become the universal
AI-generation glyph, and a durable event store cannot afford to be misfiled on
sight. **Retained, deliberately, as a copy device** — the asterisk stays available
as the footnote marker tying a claim to its falsifier, which is the job it was
always best at.

**The chord** — a disc cut where the query fell, the two parts left slightly apart.
The most literal rendering of SD-0001 and the most ownable shape considered.
Rejected on warmth: it carries the argument and nothing else, and the yellow had no
work to do in it.

**A closed circle of undifferentiated dots.** Rejected as a loading spinner.

## Bad.

Five costs, and the first was found by rendering the mark rather than by reasoning
about it.

**It read as a brightness icon, and the fix is reasoned rather than tested.**
Eight rounded elements radiating from a filled disc is the glyph every operating
system uses for display brightness. The gear risk — the one anticipated in advance
— turned out to be fully mitigated; **this one was not anticipated at all, and it
was the larger of the two**, because the mark stands alone precisely where it is
worst: the favicon, the crates.io avatar, the GitHub org icon.

Three levers were cut and rendered. **Seven radial blocks was adopted**: it defeats
the eight-fold reading, no icon set uses seven, and it keeps the sun. Tangential
blocks — each record turned to lie *around* the ring instead of pointing out of it
— solved the collision more completely and was rejected on cost, because it stops
being a sun at all; it is recorded in
[`logo-explorations-v3-collision.html`](logo-explorations-v3-collision.html)
and remains the fallback if seven proves insufficient. Squaring the corners was
tested and rejected: it buys nothing against brightness and walks back into the
gear, exactly as predicted.

**What is still missing is evidence.** Seven is a reasoned defence that has been
looked at by the two people who already know what the mark is meant to be, which is
the weakest possible test. The check that would settle it is showing the mark at
16px to three Rust developers who have seen neither it nor this record, and asking
what they think it is. Until that is done, the collision is *mitigated and
unfalsified*, not closed — and a project whose specification demotes any clause
naming no wrong implementation should not pretend otherwise about its own logo.

**It is one bad decision away from a gear**, and gear marks are the most crowded
shape in developer tooling. The rules in [`brand-kit.html`](brand-kit.html) — seven
blocks not twelve, gaps 2.19× the block width, a 10.5-unit hub gap, rounded corners,
a solid centre with no bore — are what hold it off that reading, and **every one of
them is a constraint that a future well-meaning redraw will be tempted to relax.**
They are written down as values rather than principles so that a violation is
checkable rather than arguable.

**Sun yellow fails contrast as a text colour on light ground** — 1.71:1 on Paper,
1.75:1 on docs.rs white, measured. It is a graphic fill and nothing else, which is
why the palette carries Ember `#A85B00` (4.92:1) purely so that amber-coloured
*text* has a legal value. Any surface that puts the mark on white is relying on
shape alone, and any docs.rs or README header art must sit the mark on ink.

**The first ink was warm on paper and not on screen.** `#191512` was specified as a
warm near-black and measured 17.70:1, and it was wrong: seven points of red over
blue is below the threshold at which anyone perceives hue in text, so it read as
plain black and the warmth the yellow needed to sit against was not there. Replaced
by `#2A211B` — fifteen points of spread, 15.38:1, still comfortably past AA. The
lesson generalises past this palette: **a colour justified by its hex value rather
than by being looked at is not yet a decision**, and both defects in this record
were found by rendering rather than by reasoning.

**An adaptive SVG was shipped and had to be withdrawn.** The first outlined files
carried `@media (prefers-color-scheme: dark)` so that one asset could serve light
and dark surfaces. It fails for a reason that is obvious once stated and was not
stated in advance: **a standalone SVG knows the operating system's preference, not
the colour of the surface it was placed on.** A light page opened on a machine set
to dark mode rendered the wordmark cream on cream — invisible. Every asset now
carries one fixed colour, with a separate reversed file, and the caller chooses via
`<picture>`. The page knows its own background; the file cannot.

**The identity is retroactive to a name whose meaning is itself retroactive.**
SD-0001 assigned a meaning after the fact and said so; this record assigns a shape
to that meaning, one layer further out. A hostile reader can walk the chain back to
"the crate name was free." The mitigation is unchanged: be first to say it.

**It is not filed, and it should not be treated as permanent.** The trademark search
on "happenstance" is still open, and
[`references/seeds/licensing-and-the-commercial-seam.md`](../seeds/licensing-and-the-commercial-seam.md)
records it as the gating question for the entire commercial strategy — the
anti-appropriation lever is trademark rather than copyright, because Apache-2.0 §6
grants no trademark rights. **A visual identity built before that search returns is
a sunk cost by choice.** Cheap at this stage; not cheap after it is on stickers.

## What follows from this

- The open question *"visual identity — does it need one?"* is answered narrowly:
  yes, one mark and one wordmark, on the grounds that avatars render regardless. The
  broader question of how much identity a library warrants stays open.
- `naming.md`'s prohibition on chance, luck, dice, serendipity, fortune or
  coincidence imagery **in a logo** is satisfied and should be checked against any
  future extension. A sun is daylight, not fortune. The distinction is thin enough
  that it wants restating rather than assuming.
- The asterisk is reserved as a copy device. If it is ever promoted to a mark, that
  is a new `SD-`, not a drift.
- Assets live in [`assets/brand/`](../../assets/brand/README.md), outlined and
  shipped. No raster fallbacks exist yet.
- Nothing here may be filed, registered or applied to physical goods until the
  trademark search closes.

## Evidence

| Claim | Source |
|---|---|
| The boundary is deliberate but not pre-declared | [`SD-0001`](SD-0001-what-happenstance-means.md); [`naming.md`](naming.md) |
| No chance, luck or fortune imagery in a logo | [`naming.md`](naming.md), *Rules for using it* |
| Lowercase in prose, matching the crate name | [`naming.md`](naming.md) |
| The surveyed market is grey, blue or monochrome | [`references/evaluation/research-rust-ecosystem.md`](../evaluation/research-rust-ecosystem.md) |
| Sun #FFB627 on Paper is 1.71:1; on docs.rs white 1.75:1; on ink 10.34:1; Ember #A85B00 on Paper 4.92:1; Ink #2A211B on Paper 15.38:1 | WCAG 2.1 relative luminance, computed 2026-08-18 |
| Eight radial blocks rasterise as a display-brightness glyph; seven, tangential and sharp-cornered variants cut and compared at 16, 24, 32 and 96px | [`logo-explorations-v3-collision.html`](logo-explorations-v3-collision.html); cairosvg renders inspected 2026-08-18 |
| Wordmark is IBM Plex Sans SemiBold, HarfBuzz-shaped, −2.7% tracking, outlined | [`assets/brand/happenstance-wordmark.svg`](../../assets/brand/happenstance-wordmark.svg) — paths, no live text |
| Trademark, not copyright, is the anti-appropriation lever | [`references/seeds/licensing-and-the-commercial-seam.md`](../seeds/licensing-and-the-commercial-seam.md); Apache-2.0 §6 |
| A library's visible surface is crates.io, docs.rs, GitHub, README | HS-P0016 `ux` brief |
