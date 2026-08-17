---
item: HS-I0007
stage: intake
created: 2026-08-17T00:54:15.619Z
updated: 2026-08-17T00:54:15.619Z
template_sig: ab516678
rendered_sig: f5202efc
---

# Intake Brief — From Accurate to Teachable

## Problem

happenstance's documentation is accurate and unteachable, and nothing in the
repository can tell the difference. The machinery keeping its *documents* honest is
unusually strong — `missing_docs`, `missing_errors_doc` and `missing_panics_doc`
under a gate running `-D warnings`; `broken_intra_doc_links` denied; three rustdoc
builds; all four READMEs compiled as doctests; `spec-trace` over 200 clauses, 95
rules and 358 citations — and **none of it reads a sentence**. All twenty-four gate
steps in `xtask/src/main.rs` were inspected and not one does.
`standards/rust/70-rustdoc-obligations.md` supplies its own counter-example: the
doc comment *"Returns the head position. # Errors: Returns an error on failure."*
is green under the entire workspace. The published `0.2.0-alpha.1` shows the cost —
its opening program calls `Tags::empty()` twice, so the consistency boundary that
is the whole reason DCB exists is zeroed out in the first code anyone runs, while
the page separately explains that composing decision models is *"the mechanism that
makes a dynamic consistency boundary dynamic"*. The three strongest explanations
this project has written — the workspace README's DCB argument, the worked
example's module doc, and `MemoryEventStore`'s walk-through — are respectively
unpublished, unpublished, and reachable only by a reader who already knows to look.
The fuse is fixed: phase 12 is the irreversible act, because a crates.io release
cannot be edited and docs.rs renders a version once.

## Desired Outcome

Documentation held to the same standard as the code: a reader who has installed the
crate can be taught, and the teaching is **checked rather than asserted**. Four
things, from the seed's vision:

- Narrative documentation compiled against the real crates by the gate, so a page
  cannot drift from the API the way prose normally does.
- A reader who is not the author getting through it, with what they got stuck on
  written down.
- A named answer to *which need does this page answer*, so a page can be finished
  rather than merely added to.
- A rendered result met where a Rust developer already looks, not a site they have
  to be told about.

We will know because a page deliberately broken fails the gate, and because the
friction log of a non-author reader exists and is dated.

## Constraints

Binding, from the seed's *What must remain true* — these constrain any answer and
are not preferences:

- Narrative documentation is compiled by the gate or it does not ship. A page of
  ```ignore``` fences is the documentation form of a `todo!()` body.
- The medium is the ecosystem's own — rustdoc for reference, a book for narrative.
- Which need a page answers is a decision with a name; a page answering a second
  need has a defect.
- Where any page and `spec/SPECIFICATION.md` disagree, **the specification wins**. A
  guide cites clauses and never restates them, and the precedence chain in
  `standards/rust/README.md` is **not extended** by this work.
- The nine documentation MUSTs that `[FROZEN]` clauses impose on
  `happenstance-core`'s doc comments were discharged in the phase 5/6
  reconciliation. Work that rewrites those comments leaves them discharged.
- Evidence beats argument: the settling artefact must be one that **would not exist
  if the design were wrong**.
- Whatever ships at first publish is permanent for that version.

**Scope exclusion.** The published surface — README landing copy, status-truth
vocabulary, the maturity census, peer positioning, docs.rs manifest metadata and
rendered-page preflight — is owned by project `publication-and-positioning`
(HS-P0016) on `initiative/from-contract-to-published-library`, against a `_design.md`
signed off with no conditions on 2026-08-12 and a fifty-one-frame mock. That work
asks whether what the surface *claims* is true; this initiative asks whether a
reader can be *taught*. This work does not reopen a resolved design gate.

**Non-goals.** A reader who knows neither event sourcing nor DCB is out of scope —
settled on evidence, and work that quietly reopens it has changed the product.
Benchmarks and performance reporting belong to a separate seed
(`references/seeds/measured-not-claimed.md`) and are not folded in.

**Not a port freeze.** This initiative freezes no port, so the port-freeze gate box
is satisfied vacuously rather than by an argument; it is ticked on that basis and
the reasoning is recorded here rather than left implicit.

## Open Questions

Carried deliberately unsettled from the seed, plus the seams found while scoping:

- **What the chapters are**, which need each page answers, where each lives, how the
  result is hosted, and whether a given example belongs to a page or to a doctest.
- **Where a documentation standard lives, if one is written.** Across twenty-eight
  constitution atoms there is no rule about narrative structure, audience, worked-example
  design or diagrams, and of seventeen ADRs none concerns documentation. Whether the
  gap is filled by new constitution atoms, a separate corpus, `.kb/playbooks/` atoms
  or nothing is undecided — and constrained, because a tree the gate reads must be
  pinned by path in `xtask/src/` the way `spec/` and `standards/rust/` already are.
- **Whether `RUNBOOK.md` grows a phase.** No phase owns documentation; its only
  documentation criterion is phase 12's *"docs.rs green"*, which checks that HTML
  built.
- **Who supplies the audience model.** `.kb/product/` and `.kb/design/` hold a README
  each and zero atoms. Story HS-S0131 (`persona-and-journey-intake-staging`, under
  HS-P0019) is already specced to stage four personas into `.kb/_intake/` and sits at
  `stage: plan`. Whether this initiative consumes, supersedes or sequences behind it
  is a real dependency — two initiatives independently inventing personas is the
  failure mode.
- **Which documentation MUSTs exist, by clause id.** The seed says nine; the exact
  set is not enumerated anywhere and distillation must pin it before any doc comment
  on `happenstance-core` is rewritten.

## Proof artefact

**Two, and they falsify different things.** The seed names both, and neither can
substitute for the other.

1. **A gate step that fails on a page deliberately broken to prove it.** Prose that
   claims to be compiled must be shown to fail when it is wrong — the same proof
   `spec-trace` was made to give at phase 0, where the exit criterion was the tracer
   *failing* on a deliberately broken clause rather than passing on a good one. A
   documentation gate that has only ever been green is decorative, and this
   repository has already paid for that lesson once: the documentation step printed
   *"generated 3 warnings"* and exited 0 for as long as it ran, because rustdoc does
   not read `RUSTFLAGS`.
2. **A friction log from a reader who is not the author**, dated, recording what
   they got stuck on rather than whether they liked it. This is the artefact that
   would not exist if the documentation were adequate. The precedent is already in
   the tree: `examples/outside-projection-adapter/` is a projection adapter written
   from the rendered documentation alone, in a crate where the orphan rule and the
   non-dev dependency graph behave as they do for a stranger.

`cargo xtask ci` being green is a precondition for looking at these, never one of
them.

## Clauses

**This initiative discharges no clause and amends none.** It is additive to the
specification, and the precedence chain is not extended by it.

What it must **preserve** is the set of documentation MUSTs that `[FROZEN]` clauses
already impose on `happenstance-core`'s doc comments, discharged in the phase 5/6
reconciliation. The seed states there are nine; the set is not enumerated in one
place, and pinning it by clause id is an open question above rather than a claim
made here. Instances confirmed present in `spec/SPECIFICATION.md` while scoping,
offered as the starting thread and not as the answer: `:1196` and `:1285` (`Tag`
equality is byte equality, no normalisation applied, and the documentation must name
it), `:1487`, `:1517` and `:1560` (a store must document its actual limit above each
`MIN_SUPPORTED_*` floor), `:4353` (the port's documentation must state that an
`AppendCondition` is a claim about one store's log), and `:1079`, `:3757`, `:4109`,
`:4146` and `:5602`.

Any rewrite of those comments leaves them discharged. Where this brief and
`SPECIFICATION.md` disagree, the specification wins.

## Research angles

Approved at the intake gate, tailored to this seed rather than drawn from a template:

- **Rust narrative-docs prior art and page-need taxonomy** — how tokio, serde,
  diesel and axum split narrative from reference, what lands on docs.rs versus a
  book, and how each decides which need a page answers. Carries Diátaxis and its
  critics, since *"a page answering a second need has a defect"* is that shape and is
  contested.
- **Compiled-prose tooling** — `mdbook test`, `doc-comment`, `skeptic`,
  `#![doc = include_str!]`, trybuild. Which can fail a CI gate on a deliberately
  broken page.
- **Teaching the aggregate→boundary shift** — how dcb.events, Disintegrate,
  umadb-dcb, Marten and EventStoreDB move readers off aggregates and streams, the
  diagram vocabulary they use, and where readers reliably stall. The workspace
  currently contains zero diagrams.
- **Comprehension as evidence** — documentation usability testing, cognitive
  walkthrough and non-author reader studies; how a friction log is run and recorded
  so it is evidence rather than opinion.
- **`interaction-pattern-prior-art`** — appended automatically because `userFacing`
  is true.

`userFacing: true`. Unusual for a library, and deliberate: the deliverable is a
surface humans read, and this repository already runs perceptual design for
documentation (`publication-and-positioning` carries a `_design.md` and a
fifty-one-frame `design/mock.html`). `design.capture` is absent from
`.redkiln/config.yaml`, so the perceptual review remains a skip — there is no app to
screenshot.

Effort tier `standard`, scored 10 across the six axes with Blast radius, Unknowns,
Surface and Volume each at 2; the per-axis veto alone forces it.

## Source paths

- `references/seeds/user-documentation.md` — the seed, reconciled 2026-08-16 against
  the tree that shipped the alpha (commit `c58ba64`).

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` — where each box's rationale is
written — and will not leave `intake` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
