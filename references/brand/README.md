# Brand — the identity, and the records behind it

The visual identity for happenstance: what the mark is, what it means, and the rules
that bind anything carrying the name. Artwork lives in
[`assets/brand/`](../../assets/brand/README.md); this directory holds the reasoning
and the manual.

| Looking for | It is at |
| --- | --- |
| How to apply the identity | [`brand-kit.html`](brand-kit.html) |
| What it looks like in use | [`showcase.html`](showcase.html) |
| What the name means, and how to introduce it | [`SD-0001-what-happenstance-means.md`](SD-0001-what-happenstance-means.md) |
| Why the mark is what it is | [`SD-0002-the-mark.md`](SD-0002-the-mark.md) |
| The evidence behind the mark's final form | [`logo-explorations-v3-collision.html`](logo-explorations-v3-collision.html) |
| The artwork itself | [`assets/brand/`](../../assets/brand/README.md) |

## Why `SD-` and not `ADR-`

The `ADR-` sequence in [`references/adr/`](../adr/) is the architecture record: decisions
that bind the contract, the ports and the wire format. Brand decisions bind copy and
artwork instead, and interleaving them would make the architecture sequence harder to
read for no gain. `SD-` records carry the same structure — the decision, what it does
not claim, why each alternative lost, the costs, and an evidence table — and the KB
atoms in `.kb/decisions/` cite them exactly as they cite ADRs.

## The division of labour

**These records settle; the knowledge base binds.** A record here is the long-form
argument, written once and not maintained against drift. The atoms distilled from it in
`.kb/` are what other work is expected to cite — `.kb/decisions/` for the commitments,
`.kb/design/` for the resolved lockup pattern, `.kb/reference/` for the pointer back
here.

Positioning and market material do not belong in this directory, and personas do not
belong in `.kb/product/` by way of it — that layer wants a researched person with a goal
and a fear, which a brand is not.

## What is settled, and what is not

Settled: the mark and its geometry, the wordmark, the lockups, the palette with measured
contrast, and the rule that copy must name the colloquial "chance" reading of the name
and displace it rather than assert past it.

Not settled: **the trademark search on "happenstance" has not been run.** The
anti-appropriation lever in [`references/seeds/licensing-and-the-commercial-seam.md`](../seeds/licensing-and-the-commercial-seam.md)
is trademark rather than copyright, because Apache-2.0 §6 grants no trademark rights — so
that search gates the commercial layer, and nothing here should be filed or applied to
physical goods until it returns.
