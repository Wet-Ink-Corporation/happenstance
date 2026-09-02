---
title: Product & functional alignment — From Accurate to Teachable
kind: grounding/summary
initiative: HS-I0007
slug: docs-that-teach
date: 2026-08-16
sourcePaths:
  - .bklg/docs-that-teach/_intake-brief.md
  - references/seeds/user-documentation.md
  - .kb/maps/domain-map.md
  - .kb/maps/README.md
  - .kb/product/README.md
  - .kb/design/README.md
  - .kb/concepts/README.md
  - .kb/concepts/torn-reads-and-the-append-condition-boundary.md
  - .kb/maps/decision-map.md
  - .kb/decisions/0006-bare-name-to-the-typed-layer.md
---

# Product & functional alignment — From Accurate to Teachable

Problem-space grounding only. No technology, API, tooling or chapter structure is
proposed below — that is planning's, against the seed's own "deliberately not
decided here" list (`references/seeds/user-documentation.md:214-220`). This maps
the initiative's intent to what the knowledge base already knows *functionally*:
who the product is understood to be for, what it already claims about them, and
where this initiative's audience and this initiative's subject matter sit relative
to that existing map.

## Which functional domains this touches

`.kb/maps/domain-map.md` currently holds exactly two domains, and neither is
documentation, teaching, or audience:

1. **Specification governance & conformance** — whether `spec/SPECIFICATION.md`'s
   clauses, rules and code agree (`.kb/maps/domain-map.md:37-80`).
2. **Contract ports, conformance, and the ADR corpus** — `EventStore` /
   `ProjectionStore` design and the seventeen-ADR record
   (`.kb/maps/domain-map.md:82-142`).

Both are populated entirely from architectural/governance material (compiled
findings, position-visibility experiments, wire-format measurements, ADR
lineage). This initiative's subject — can an installed reader learn to use the
crate — has no functional home in the corpus yet. That is a direct, independently
verified confirmation of the seed's own claim that "the domain map has two
domains and neither is documentation" (`references/seeds/user-documentation.md:127-128`).
Nothing here contradicts that; grounding found the same absence by reading the
map itself rather than trusting the seed's summary. Planning inherits a genuine
gap, not a mapping error: if this initiative lands durable knowledge in `.kb/`,
it will most likely be opening a **new** domain-map section, per
`.kb/maps/domain-map.md:144-150` ("Append a new `##` section... a domain is a
subject area, not a wave").

The **product layer** (`.kb/product/`) and **design layer** (`.kb/design/`) are
the two other functional-concept trees named in `.kb/maps/README.md`'s
neighbourhood, and both are functionally empty: each holds only its own `README.md`
scaffold, zero `persona`, `journey` or resolved-`design` atoms
(`.kb/product/README.md`, `.kb/design/README.md`). This matters directly to the
initiative: its own open question "who supplies the audience model" cannot be
answered by citing an accepted KB atom today — there is not one to cite.

## What .kb/product/README.md already commits this initiative to

`.kb/product/README.md` states the product layer's law in one line this
initiative's whole "not asserted, checked" posture already agrees with: *"A
persona nobody researched is a stock photo with a name"* (`.kb/product/README.md:24`).
The same atom draws two boundaries this initiative should hold to functionally
rather than rediscover:

- A persona is `authority_tier: product`, evidenced, durable, and promoted only
  at an initiative's closeout — never authored mid-initiative as settled fact
  (`.kb/product/README.md:6-21, 28-31`).
- A journey says *what* a persona is trying to accomplish, never *which page or
  control* they touch — "A journey that transcribes today's UI goes stale the
  moment the UI moves" (`.kb/product/README.md:33-36`). The direct functional
  analogue for this initiative: a friction log or a page's stated need should be
  framed the same way — what the reader is trying to accomplish and where they
  stalled, not which heading they clicked past.

## The audience model that already exists, unpromoted

The seed's "who supplies the audience model" question is not speculative — a
concrete, evidenced answer already exists in the tree, just not in `.kb/`. On
branch `initiative/from-contract-to-published-library`,
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`
(frontmatter `status: draft-for-planning`, `promoted: false`) names four
personas, each with a cited goal, current workaround, and fear:

1. **The application author** — `cargo add`s the crate to model a domain-spanning
   consistency invariant; today either couples to a database early or hand-rolls
   a port; fears discovering a contract defect in production, specifically a
   second independent reader (projection/export/replay) silently building a
   wrong answer from an inconsistent view of the log.
2. **The adapter author** — implements the contract against real storage; fears
   a port assumption their storage system cannot honour, discovered late.
3. **The local-first / edge Rust developer** — a `!Send`, `wasm32`-constrained
   variant of personas 1 and 2; explicitly flagged by that distillation as
   "a special case of both 1 and 2, not a fourth concern."
4. **The evaluator (pre-adoption)** — reads for a bounded window before deciding
   whether to depend on this at all; has no external body to check a
   "DCB-compliant" claim against.

**This maps almost one-to-one onto the audiences this initiative's own seed
names.** The seed's "application authors," "adapter authors," and "the
evaluator" (`references/seeds/user-documentation.md:149-166`) are, respectively,
Persona 1, Persona 2, and Persona 4 above — described independently, from a
different initiative's discovery, and landing on the same three roles with
compatible fears (a defect or a gap discovered too late, after commitment). The
seed's explicit non-goal — "not the reader who knows neither event sourcing nor
DCB" — is consistent with all four of those personas, none of whom is that
reader either.

**This is grounding evidence, not permission to reuse the text.** The
distillation itself is unpromoted and carries its own caveats worth carrying
forward: none of the four personas has been directly interviewed or observed,
Persona 4 in particular is "inferred from research framing rather than a named
individual's stated experience," and the file's own closing instruction is that
`.kb/product/README.md`'s bar should be "re-checked at closeout... to confirm or
correct" the sketches (personas-and-journeys.md, Risks section). Treating this
initiative's audience as identical to that draft is doing this initiative's own
audience work by citation, before either project has actually confirmed the fit.

## Tensions and boundary risks

- **Duplicate persona invention is a named, live risk, not a hypothetical.**
  Story `persona-and-journey-intake-staging` (id `HS-S0131`, under project
  `closeout-and-durable-audience`, initiative `from-contract-to-published-library`)
  is already specced — `stage: plan`, `status: ready` — to stage exactly these
  four personas and their journeys into `.kb/_intake/` for promotion
  (confirmed directly: `.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/story.md`).
  If this initiative's planning derives its own audience model from scratch
  before that story lands (or independently of it), the failure mode the
  intake brief itself names is exactly what happens: *"two initiatives
  independently inventing personas"* (`.bklg/docs-that-teach/_intake-brief.md:107-109`).
  This is a sequencing dependency this grounding confirms is real, not a
  concern this initiative can resolve unilaterally — the other initiative's
  branch, not this one, owns that story's advancement.
- **Scope seam with `publication-and-positioning` (HS-P0016) is functionally
  narrower than it first appears, and shares files.** That project's
  `landing-copy-and-status-truth` and `guarantees-and-docs-rs-presentation`
  stories are squarely about Persona 4 (the evaluator) — first-contact trust,
  read in a bounded window, from a page this initiative also has to touch
  (`crates/happenstance/README.md` and the crate's own front page, per the
  intake brief's stated seam, `.bklg/docs-that-teach/_intake-brief.md:72-78`).
  Functionally the two initiatives are answering different questions about the
  *same reader* on the *same page* — "does this project's claim hold" versus
  "can I learn from what's here" — and the intake brief already states the
  right precedence (does-not-reopen), but planning should expect the seam to
  require actual coordination, not just a stated boundary, precisely because
  one evaluator persona's journey spans both.
- **No `.kb/concepts/` atom currently states the product-facing "why" this
  initiative's flagship example is accused of omitting.** The one concept atom
  in the corpus, `torn-reads-and-the-append-condition-boundary.md`, explains
  functionally exactly the mechanism the seed says the crate's opening example
  fails to demonstrate: why an append condition derived from a read's own
  observed maximum cannot catch a torn read, and why that boundary is the
  reason DCB's consistency guarantee exists at all (`.kb/concepts/torn-reads-and-the-append-condition-boundary.md`).
  This is an alignment, not a contradiction: the KB already holds an accurate,
  reviewed account of the exact mechanism the seed says current teaching
  omits (`references/seeds/user-documentation.md:64-78`, `Tags::empty()` twice).
  Any page this initiative eventually writes about the consistency boundary has
  a functionally correct concept atom to be consistent *with* — though per the
  intake brief's own constraint, a guide cites `spec/SPECIFICATION.md`'s
  clauses rather than a KB atom's prose, so this is orientation for planning,
  not a citation source for the page itself.
- **The design layer's admission rule doesn't obviously fit narrative prose,
  and the seed doesn't resolve it.** `.kb/design/README.md` reserves
  `authority_tier: design` atoms for "the pattern chosen for a class of
  surface... signed off" — built for GUI screens, not rustdoc pages or a book.
  The intake brief already flags this as open ("where a documentation standard
  lives, if one is written") rather than assuming `.kb/design/` is the answer;
  grounding confirms there is no atom there today that could be reused or
  extended, so this is genuinely unclaimed territory, not a fit planning can
  discover by reading harder.

## What is confirmed rather than merely repeated from the seed

Every specific claim above was checked against the tree directly rather than
taken on the seed's word: `.kb/maps/domain-map.md` was read and has two
sections, both non-documentation; `.kb/product/README.md` and `.kb/design/README.md`
were read and hold no atoms beyond their own scaffolding; the persona
distillation was read in full on its actual branch and its promotion state
(`promoted: false`) confirmed from its own frontmatter; `HS-S0131`'s `stage` and
`status` were confirmed from its own story card rather than from the brief's
citation of it; and `.kb/decisions/` was checked for any documentation-focused
decision — none of the seventeen ADRs concerns documentation, which matches the
seed's claim at `references/seeds/user-documentation.md:236-238`.
