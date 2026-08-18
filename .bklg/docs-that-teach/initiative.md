---
id: HS-I0007
uid: 2d0004
type: initiative
slug: docs-that-teach
title: From Accurate to Teachable
parent: null
initiative: docs-that-teach
project: null
status: implementing
process: initiative
stage: implementation
automation: HITL
severity: null
blocked_by: []
blocks: []
tier: standard
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-18T00:09:33.481Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 62affb3b
---
# From Accurate to Teachable

## One-line intent

Take `happenstance`'s documentation from prose that is accurate and unteachable —
and that nothing in this repository can tell apart from prose that teaches — to
pages a reader who is not the author can actually be taught by, where the teaching
is **checked rather than asserted** and the check has been seen to fail.

## Vision and narrative

The machinery keeping this project's *documents* honest is unusually strong.
`missing_docs`, `missing_errors_doc` and `missing_panics_doc` under a gate running
`-D warnings`; `broken_intra_doc_links` denied; three rustdoc builds; all four
READMEs compiled as doctests; a specification tracer over 200 clauses, 95 rules and
358 citations. All twenty-four gate steps were inspected at intake and **none of
them reads a sentence.** The constitution supplies its own counter-example on
purpose: the doc comment *"Returns the head position. # Errors: Returns an error on
failure."* is green under the entire workspace
(`standards/rust/70-rustdoc-obligations.md`, RS-70-5).

That gap is not theoretical, and the published alpha shows what it costs. The first
program anyone runs calls `Tags::empty()` twice, which zeroes out the consistency
boundary that is the whole reason DCB exists, while the same page separately
explains that composing decision models is *"the mechanism that makes a dynamic
consistency boundary dynamic"*. Both statements are individually true. Together they
teach the reader the opposite of the thing. And the three strongest explanations
this project has ever written — the workspace README's DCB argument, the worked
example's module doc, and `MemoryEventStore`'s walk-through — are respectively
unpublished, unpublished, and reachable only by a reader who already knew to look
for them.

Discovery changed the framing in six ways, and each one narrows what this initiative
is actually deciding.

**The two-surface shape is settled prior art, not a blank page.** The crates closest
to this one's shape converged on the same arrangement: reference where the compiler
publishes it, narrative in its own place, and one explicit pointer outward from the
crate root. `tokio`'s crate root literally says guide-level documentation is found
on the website; `diesel` goes further and tells a first-time reader the reference is
the wrong place to start. `axum` deliberately declined, and the cost is documented in
its own tracker by a contributor who tried to reconstruct a narrative from a flat
folder of examples and hit exactly the "which need does this answer, in what order"
problem this initiative exists to answer up front
(`_discovery/research/01-…`).

**Compiled prose is a picking problem, not an inventing problem — and it does not
prove what we want it to prove.** Stock tooling already fails loudly on a broken
page, names the chapter and the line, and is what the Rust book itself gates CI on
today at the same toolchain this workspace pins. So the first proof artefact is
achievable. But *every* tool surveyed is blind to code that still compiles and no
longer demonstrates the claim the surrounding prose makes — which is precisely the
failure already on record here, where a query and a fold stated the same invariant
twice, drifted, and silently oversold inventory. "Compiles" and "teaches correctly"
are different properties that only partly overlap
(`_discovery/research/02-…`). That is the whole reason there are two proof artefacts
and neither may stand in for the other.

**Naming which need a page answers is a stricter bar than any peer holds.** No Rust
convention makes it a labelled, checkable property: the API Guidelines and the
strictest published rubrics govern in-comment reference quality and are silent on
narrative structure entirely. Diátaxis independently names this initiative's exact
diagnosis — it separates functional quality from a deeper axis and states plainly
that documentation *"can be accurate, complete, consistent and also useless"*, which
is the intake brief's "accurate and unteachable" in someone else's words. But its
sharpest critics, and its own site, treat the four-category enumeration as scaffolding
rather than a completeness claim, and the critique lands hardest exactly where this
project sits: dense, unfamiliar conceptual models rather than simple tools. The
discipline can be adopted without importing the taxonomy, and choosing between those
is a real decision this work owns (`_discovery/research/01-…`,
`_discovery/distillation/interaction-patterns.md`).

**This project has no pain to relieve, and every comparable teacher relies on one.**
Every source surveyed teaches the shift by re-narrating the reader's own existing
failure — read-check-write, then pessimistic locking, then optimistic locking, then
the cross-entity constraint neither can solve — in the reader's existing vocabulary,
before introducing a single new term. `happenstance` is DCB-native and never had an
aggregate to kill, so which prior intuition the teaching argues against is an unmade
choice here and was an explicit, early choice everywhere else. It is not even
obvious which prior model the reader arrives with: the incumbent store the largest
group of readers comes from presents stream-per-entity as the unmarked default and
never contrasts it with anything (`_discovery/research/03-…`).

**Comprehension has an established method, and non-authorship is part of it.** A
friction log has a published shape — a scenario stated in the reader's terms, a
chronological record of what was searched, clicked and pasted, reactions captured as
they happen, severity marked inline — and the practice is explicit that insiders
unconsciously route around the rough spots the exercise exists to find. One session
is genuinely defensible evidence, but only for the narrower claim: real stumbles were
captured and are traceable, never that every comprehension failure was found
(`_discovery/research/04-…`).

**The picture nobody has drawn is the one that matters.** The workspace contains zero
diagrams. So, effectively, does the field: the specification's own site illustrates
the aggregate model it is *retiring* with captioned box-and-line diagrams and has no
equivalent for the model it is *introducing*. Filling that gap is closer to original
work than to adopting a convention, and it is optional relative to everything above
(`_discovery/research/03-…`, `_discovery/research/05-…`).

Underneath all six sits one cross-persona finding worth stating on its own: in every
case the answering content **already exists somewhere in this workspace**. The
application author hits the gap inside a running program, the adapter author inside a
compiler error, the evaluator inside a decision with no next page. This is a
findability-and-sequencing problem far more than a content-volume problem, and an
initiative that measures itself in pages written will solve the wrong one.

The fuse is fixed and it is short. A registry release cannot be edited and a rendered
version is rendered once. Whatever ships at first publish is permanent for that
version, so the difference between documentation that teaches and documentation that
merely passes stops being recoverable at exactly that moment.

## Goals

- **Prose that cannot lie about the API without being caught.** Narrative pages are
  compiled against the real crates by the gate, and the gate has been *seen to fail*
  on a page deliberately broken to prove it — not merely seen to pass on good ones.
- **A first fifteen minutes that teaches the one thing this library is for.** The
  reader's opening encounter demonstrates a consistency boundary actually doing its
  job, rather than stating that boundaries matter while showing an empty one.
- **Every page has a named answer to *which need does this page answer*,** so a page
  can be finished rather than merely added to, and a page that has quietly acquired a
  second job is legibly defective rather than merely long.
- **A bridge from where the reader already stands.** The teaching names the specific
  tension in the reader's existing mental model before introducing new vocabulary,
  and *which* prior model it argues against is a stated decision rather than an
  accident of who wrote which page.
- **A witnessed reader.** A dated friction log from someone who is neither the author
  nor an insider records what they got stuck on, and it is routed to someone who can
  act on it rather than filed.
- **A durable, citable discipline for writing a page that teaches**, so the next
  person does not re-derive it and the next crate does not start from zero.
- **The result is met where a Rust developer already looks**, not on a surface they
  have to be told about first.

## Non-goals

- **Reaching zero open questions.** Several below are deliberately unresolved;
  settling one in passing is a defect, not progress.
- **Deciding how the work decomposes** into projects and stories — that is planning's,
  from its own grounding.
- **Volume.** Pages written is not the measure and must not become one; the evidence
  says this is a findability-and-sequencing problem.
- **Rewriting the specification into readable prose.** Where a page and
  `spec/SPECIFICATION.md` disagree, the specification wins; a guide cites clauses and
  never restates them.

## In scope

- The reader's opening encounter with the library — its sequencing, what it
  demonstrates, and whether it demonstrates the boundary rather than describing it.
- Narrative teaching material for the three named audiences, and the decision of
  which need each page answers, recorded per page rather than assumed.
- The conceptual bridge from a reader's prior model to this one, including the
  decision of which prior model is being argued against.
- Whether the aggregate-to-boundary shift is taught with a picture, a before/after
  contrast, a narrated sequence, or some combination — and what the honesty cost of
  each is.
- Bringing the three strongest existing explanations within reach of the reader they
  were written for, given that all three are currently unpublished or undiscoverable.
- The teaching's own checkability: the gate obligation on narrative prose, and the
  demonstration that it fails when the prose is wrong.
- The comprehension evidence: recruiting a genuine non-author non-insider, running
  and recording the session so it is auditable, and acting on what it finds.
- Where a documentation discipline lives durably once written, and what it binds.
- The reconciliation between this initiative's audience model and the one already
  specced to be staged elsewhere, so the audience is adjudicated once.
- Recording, at closeout, the personas and journeys this work was done for, so the
  next initiative inherits an audience rather than re-deriving one.

## Out of scope (explicit non-goals)

Named concretely, so nobody has to guess at the boundary:

- **The published surface's *claims*.** README landing copy, status-truth vocabulary,
  the maturity census, peer positioning, registry manifest metadata and rendered-page
  preflight belong to `publication-and-positioning` (HS-P0016) on
  `initiative/from-contract-to-published-library`, against a `_design.md` signed off
  with no conditions and a fifty-one-frame mock. That work asks whether what the
  surface *claims* is true; this one asks whether a reader can be *taught*. **This
  initiative does not reopen a resolved design gate.** Where the two touch the same
  file, the seam is purpose, not paragraph — and planning owns stating it precisely.
- **The reader who knows neither event sourcing nor DCB.** Settled on evidence at
  intake. Work that quietly reopens it has changed the product, not widened the docs.
- **Benchmarks and performance reporting.** They belong to a separate seed
  (`measured-not-claimed`, staged on another branch and not present in this
  worktree) and are not folded in on the grounds that both are "things we should
  write down".
- **Amending, discharging or restating any specification clause.** This work is
  additive to the specification. It discharges none and amends none.
- **Extending the precedence chain** in `standards/rust/README.md`. A documentation
  discipline sits inside that chain wherever it lands; it does not add a tier to it.
- **Un-discharging the frozen documentation MUSTs.** Any rewrite of
  `happenstance-core`'s doc comments leaves discharged what the phase 5/6
  reconciliation discharged. The referent may be rewritten; the reasoning may not
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- **Independently inventing personas.** HS-S0131 is already specced to stage four of
  them. Two initiatives authoring the same audience is the named failure mode, and
  this initiative reconciles rather than races.
- **Hand-authoring `.kb/` atoms outside the ingest and closeout paths.** The first
  attempt at that was reverted (`0269720`) for producing the directory layout of the
  process without the process.
- **Quizzes or inline recall checks as a substitute for the friction log.** Ruled out
  on evidence at discovery: they measure recall of the author's own prose, not a
  reader's ability to use the library. This is settled, not a design choice to be
  reopened in planning.
- **A marketing site, a hosted service, or documentation addressed to a non-Rust
  audience.** The reader has already run `cargo add`.
- **Rewriting `references/adr/`, `references/evaluation/` or `spec/` into narrative.**
  Those trees address contributors and reviewers; `docs/README.md` already states the
  boundary and it holds.
- **Incidental bugs found in passing.** They route to the `support` initiative per
  `.redkiln/config.yaml`, and are not absorbed as in-scope fixes.
- **Running `redkiln adopt --templates`.** It would silently overwrite six deliberate
  customisations and then fail CI on the absence it created.

## Who this is for

**The application author** who has run `cargo add happenstance` with a cross-entity
invariant to model, and wants to get the boundary right on the first attempt, in the
library's own vocabulary. Their measured pain is not a missing page — it is an
opening example that zeroes the boundary out while the prose beside it explains why
boundaries matter, and a worked example that is unpublished, unlinked, and internally
inconsistent in exactly the way that silently oversold inventory once. Their fear is
modelling it wrong in a way nothing tells them about until production.

**The adapter author** implementing the storage contract against a real system —
today, only this project's own team. They are unusually well served by reference and
recipe: a 314-line constitution atom, a four-step contributing recipe, and a
conformance suite that says pass or fail by name. What they do not have is a walked
account of the reasoning, or a third-party adapter to read. Their measured pain is
sharply specific: the explanation of the one trait-resolution error they will
certainly hit exists in three contributor-facing documents and **not in the file they
are looking at when they hit it**.

**The evaluator**, reading for about twenty minutes to decide whether to adopt. The
README answers their first question well and there is nowhere for the second question
to go. Their fear is adopting — or wrongly rejecting — on a first impression that does
not generalise. The field's three independently-named stall points (dynamism mistaken
for chaos, modelling-versus-routing ambiguity, and no settled replacement noun for
what an Aggregate used to name) are plausible shapes of that second question, and
whether this project's own vocabulary answers them or reproduces them is untested.

Explicitly **not** this initiative's reader: someone who knows neither event sourcing
nor DCB. All three above already have event-sourcing vocabulary; what they may lack is
DCB vocabulary, which is a different and much narrower gap.

## Referenced personas & journeys

The durable product layer is **structurally present and functionally empty**:
`.kb/product/README.md` and `.kb/design/README.md` are layer READMEs and no persona
or journey atom (`authority_tier: product`) exists in this tree. There is nothing
adjudicated to cite, and hand-authoring one here would be exactly the practice that
was reverted at `0269720`.

The three personas above and the journeys they name are therefore carried from this
initiative's own distillation:
[`_discovery/distillation/personas-and-journeys.md`](_discovery/distillation/personas-and-journeys.md),
with the archetypes in
[`_discovery/distillation/opportunities.md`](_discovery/distillation/opportunities.md).

The journeys this work improves, moment by moment:

- *The first fifteen minutes* — the application author's path from an installed crate
  to a program that demonstrates a real consistency boundary, without the boundary
  being empty in the first thing they run.
- *Model my invariant in your words* — the application author's path from a
  cross-entity rule they already understand to its expression in this library's
  vocabulary, with the prior model they arrived with named rather than assumed.
- *Walk the adapter path, not just the recipe* — the adapter author's route from
  intent to a passing suite, meeting the explanation of a compiler error at the
  moment and in the file where it fires.
- *Survive the second question* — the evaluator's bounded look, in which the page that
  answered the first question has somewhere to send them next.

**Flagged for promotion at closeout:** these personas and journeys should be promoted
into `.kb/product/` as `authority_tier: product` atoms as part of this initiative's
closeout, so the next initiative inherits an adjudicated audience. Three
qualifications travel with them and must not be dropped in promotion: none has been
directly observed yet (all evidence is this project's own audit or secondary evidence
about comparable readers); the adapter author's "no third-party adapter exists to
read" rests on internal audit alone; and HS-S0131 is already specced to stage an
overlapping set, so promotion is a **reconciliation**, not a fresh authoring. Which of
these personas the friction log actually walks is an unmade choice, called out below.

## Why now

- **The fuse is fixed and short.** A registry release cannot be edited and a version
  renders once. Documentation that teaches and documentation that merely passes are
  indistinguishable to every instrument this repository owns, right up until the
  moment the difference becomes permanent.
- **The cost is already measured, not predicted.** The published alpha's opening
  program zeroes out the consistency boundary. That is not a risk of shipping
  unteachable documentation; it is a shipped instance of it.
- **The strongest explanations already exist and nobody can reach them.** Three of
  them. This is the cheapest moment this work will ever be, because the expensive
  part — knowing what is true — is done and only sequencing and reach are missing.
- **The audience model is about to be authored twice.** A story to stage personas is
  specced and has not run. Reconciling one audience now is far cheaper than
  adjudicating two later.
- **The instruments to check prose exist and are proven elsewhere.** The tooling that
  fails a build on a broken page is stock, in production on the language's own book,
  at the toolchain this workspace already pins. Nothing is waiting on invention.

## Business requirements matrix

| ID | Requirement | Rationale | Priority | Source |
| --- | --- | --- | --- | --- |
| BR-01 | Narrative teaching material must be compiled against the real crates by the gate, and a page whose claim is no longer true of the library must fail it | A page of opted-out fences is the documentation form of a `todo!()` body; it type-checks against anything | Must | `_intake-brief.md` Constraints; `_discovery/research/02-…` |
| BR-02 | That gate obligation must have been observed *failing* on a page deliberately broken to prove it, not only passing on good pages | A gate that has only ever been green is decorative; this repository already paid for that lesson when a docs step printed warnings and exited 0 | Must | `_intake-brief.md` Proof artefact; `RUNBOOK.md:920-924` |
| BR-03 | The reader's opening encounter must demonstrate a consistency boundary doing its job, not an empty one | The published alpha's first program zeroes the boundary out while the page beside it explains why boundaries matter | Must | `_intake-brief.md` Problem; `_discovery/distillation/opportunities.md` Opportunity 1 |
| BR-04 | Every page must carry a named answer to which need it answers, checkable by a reviewer rather than inferred | Without it a page becomes an open-ended sink into which every discovered defect is poured; no Rust convention currently makes this checkable | Must | `_intake-brief.md` Constraints; `_discovery/research/01-…` |
| BR-05 | A dated friction log from a reader who is neither author nor insider must exist, recording what they got stuck on rather than whether they liked it | It is the artefact that would not exist if the documentation were adequate; insider knowledge, not authorship, is what routes a logger around the rough spots | Must | `_intake-brief.md` Proof artefact; `_discovery/research/04-…` |
| BR-06 | The friction log must follow an auditable shape — stated scenario, chronological record, reactions captured, severity marked — and be routed to someone able to act on it | A log that is merely present and dated is a diary; the published method makes routing the non-optional final step | Must | `_discovery/research/04-…` |
| BR-07 | The teaching must state which prior mental model it argues against, or state explicitly that it assumes none | Every comparable project made this choice early and explicitly; this project is DCB-native and has no aggregate pain to relieve | Must | `_discovery/research/03-…`; `_discovery/distillation/personas-and-journeys.md` |
| BR-08 | The rendered result must be reachable where a Rust developer already looks, with an explicit pointer from the crate's own front door | The ecosystem-converged pattern is reference plus narrative with one pointer outward; the counter-example's cost is documented in its own tracker | Must | `_intake-brief.md` Desired Outcome; `_discovery/research/01-…` |
| BR-09 | Where a page and `spec/SPECIFICATION.md` disagree, the specification wins; a page cites clauses and never restates them | Two sources of normative truth is how a guide silently becomes a second, weaker specification | Must | `_intake-brief.md` Constraints; `standards/rust/README.md:23-29` |
| BR-10 | The documentation MUSTs that frozen clauses impose on `happenstance-core`'s doc comments must remain discharged, and the exact set must be pinned by clause id before any of those comments is rewritten | The seed says nine; the set is enumerated nowhere, and rewriting a comment that discharges a frozen clause without knowing it does is how a discharge is lost silently | Must | `_intake-brief.md` Clauses; `spec/SPECIFICATION.md` |
| BR-11 | Any content hidden behind a fold, tab or collapsed panel must be proven — not assumed — to sit inside the same checked surface as visible content, or must not carry load-bearing constraints | This repository has already had an invariant stated in two places drift when only one was visible, and its worst documented failure is a step that looked wired and silently was not | Must | `_discovery/distillation/interaction-patterns.md`; `_discovery/research/05-…` |
| BR-12 | Where a documentation discipline lands must be decided rather than defaulted, and if the gate is to read a tree, that tree is pinned by path the way `spec/` and `standards/rust/` already are | A tree the gate reads by convention rather than by path is a tree that can be moved without anything noticing | Must | `_intake-brief.md` Open Questions; `docs/README.md:25-29` |
| BR-13 | This initiative's audience model must be reconciled with the staged persona work rather than authored independently | Two initiatives independently inventing the same personas is the named failure mode, and the overlap is already close to exact | Must | `_discovery/grounding/backlog-adjacency.md`; `_discovery/grounding/product-functional-alignment.md` |
| BR-14 | The claim made for the comprehension evidence must be scoped to what the method supports — real stumbles captured and traceable — never to exhaustiveness | The small-sample basis that makes one session defensible supports only the narrower claim | Must | `_discovery/research/04-…` |
| BR-15 | The adapter author must meet the explanation of the trait-resolution error at the point and in the file where it fires | It exists today in three contributor-facing documents and not in the file the reader is looking at | Should | `_discovery/distillation/personas-and-journeys.md`; `crates/happenstance-core/src/store.rs` |
| BR-16 | The evaluator's second question must have somewhere to go from the page that answered their first | The gap is a missing link, not a missing widget; existing navigation already performs the widget's job | Should | `_intake-brief.md` Problem; `_discovery/distillation/interaction-patterns.md` |
| BR-17 | The personas and journeys this work serves must be promoted into the durable product layer at closeout, carrying their evidence qualifications | That layer holds zero atoms today; leaving it empty means the next initiative re-derives an audience from scratch | Should | `.kb/product/README.md`; `_discovery/grounding/product-functional-alignment.md` |
| BR-18 | Whether the aggregate-to-boundary shift is taught with a picture must be decided, and if it is, the honesty of the "old model" side must be decided with it | No diagram vocabulary has won even inside the specification's own community, and the illustrated-old/undrawn-new gap is the field's default | Could | `_discovery/research/03-…`; `_discovery/distillation/interaction-patterns.md` |

## Acceptance criteria

Framed from what a person is trying to do, and each observable rather than argued.

- **AC-01 — The application author's first program teaches them what the library is
  for.** Following the opening encounter end to end, they run something in which a
  consistency boundary is actually constraining an append, and can say afterwards what
  it prevented.
- **AC-02 — The application author can carry their own invariant across.** Starting
  from a cross-entity rule stated in their existing vocabulary, they can reach its
  expression in this library's vocabulary without having to read the specification or
  the source to bridge the gap.
- **AC-03 — The application author is not taught something the library no longer
  does.** Every claim a teaching page makes about the API is checked by the repository
  itself, so a page cannot be true when written and false when read.
- **AC-04 — The adapter author meets the explanation where the problem finds them.**
  When the known trait-resolution error fires, the explanation is reachable from the
  file and the message in front of them, not only from contributor documents they did
  not know existed.
- **AC-05 — The adapter author can follow the reasoning, not just the recipe.** They
  can read an account of *why* the contract is shaped as it is, in the order someone
  building against it needs it, distinct from the reference that already exists.
- **AC-06 — The evaluator's second question has somewhere to go.** From the page that
  answers "what is this", they can reach the page that answers whichever question
  they ask next, without being told in advance where to look.
- **AC-07 — Any reader can tell what a page is for before reading it.** Each page
  states the one need it answers, and a page that has acquired a second is visibly
  defective rather than merely long.
- **AC-08 — A reader arriving with the wrong prior model is met, not ignored.** The
  teaching names the prior model it argues against — or states that it assumes none —
  and does so once, consistently, rather than differently on each page.
- **AC-09 — The team can see where teaching failed a real person.** A dated record
  exists of a non-author, non-insider reader's actual path, with the stumbles marked
  by severity and traceable to the pages that caused them.
- **AC-10 — Someone acted on what that reader found.** Each recorded stumble has a
  disposition — fixed, deliberately accepted, or routed — rather than being merely
  archived.
- **AC-11 — A reader looking for the teaching finds it from where they already
  are.** Starting from the crate's own front door, the narrative material is reachable
  without prior knowledge that it exists.
- **AC-12 — The specification stays the single normative voice.** A reader who
  follows a page to a normative claim lands on the clause, not on a paraphrase of it,
  and no page has become a second specification.
- **AC-13 — The next author is not starting from scratch.** Someone writing the next
  page can cite a written discipline for what makes a page teach, rather than
  reconstructing this initiative's reasoning from its output.
- **AC-14 — The next initiative inherits this one's audience.** The personas and
  journeys above exist as durable product-layer atoms, reconciled with the staged set
  rather than duplicating it, and carrying the qualification that their evidence is
  not yet direct observation.

## Definition of Done

Each of these is run and observed to pass on the assembled result, from a clean
checkout, before this initiative is called shipped. None may be left pending. The
repository's own gate being green is a **precondition** for looking at these, never
one of them.

1. **@smoke — the teaching survives a clean checkout.** From a fresh clone with no
   local state, the full gate runs green and the narrative material builds and renders
   as part of it, not as a separate manual step someone remembers to do.
2. **@smoke — a deliberately broken page fails the gate, by name.** A teaching page is
   edited so that one of its claims is no longer true of the library; the gate is run
   and **fails**, identifying the page and the location. The edit is then reverted and
   the gate returns to green. Both halves are observed; the failing half is the one
   that matters.
3. **@smoke — the opening encounter runs and demonstrates a boundary.** A reader
   following the first-fifteen-minutes path start to finish reaches a running program
   in which an append is refused because a consistency boundary held, and the refusal
   is visible in what they see, not only in what the page says.
4. **@smoke — the boundary claim is checked, not narrated.** The scenario in (3) is
   exercised by the repository itself such that removing the boundary from the example
   makes it fail — proving the example demonstrates the claim rather than merely
   accompanying it.
5. **A reader who is not the author and not an insider completes a stated scenario,
   and it is recorded.** The session produces a dated log in the auditable shape:
   stated scenario, the logger's declared non-authorship and non-insider status, a
   chronological record of what they searched, opened and tried, reactions captured as
   they occurred, and severity marked inline.
6. **Every stumble in that log has a disposition.** Reading the log end to end, each
   marked item is traceable to a fix, a recorded deliberate acceptance, or an item
   routed elsewhere — with none left undispositioned.
7. **@smoke — the reader reaches the teaching from the front door.** Starting only
   from what a developer sees after installing the crate, a person with no prior
   knowledge of this repository's layout reaches the narrative material, observed
   rather than asserted.
8. **Every page's answered need is stated and singular.** Walking the full set, each
   page carries its named need, and a review pass over the set finds no page carrying
   two — with the check being one a reviewer can actually perform rather than one that
   depends on the author's memory.
9. **The evaluator's second question is walked.** Starting from the page answering the
   first question, at least two plausible second questions are followed to a page that
   answers them, without dead ends and without leaving the surfaces a Rust developer
   already reads.
10. **The adapter author's error meets its explanation.** Reproducing the known
    trait-resolution failure, the explanation is reached from the file and message in
    front of the reader, observed by someone doing exactly that and nothing else.
11. **The frozen documentation MUSTs are still discharged.** The pinned set of clause
    ids is enumerated in one place, and the specification cross-reference step passes
    over the tree as it stands after every doc comment this work touched.
12. **No page has become a second specification.** Each normative claim a teaching page
    makes is a citation that resolves, and a spot check confirms the page defers to the
    clause rather than restating it.
13. **Nothing load-bearing is hidden from the check.** Any folded, tabbed or collapsed
    content in the shipped material is demonstrated to be inside the same checked
    surface as visible content — by breaking a hidden claim and observing the gate fail
    — or no such content carries a load-bearing claim.
14. **The discipline is on disk and cited.** A written account of what makes a page
    teach exists in its decided home, is reachable from the material it governs, and at
    least one page cites it as the reason it is shaped as it is.
15. **The audience is durable and reconciled.** Persona and journey atoms exist under
    `.kb/product/` with valid frontmatter, pass `redkiln validate --kb`, are reconciled
    with the separately staged set rather than duplicating it, and are linked from this
    initiative's closeout.

## Open design tensions

`userFacing` is true and the deliverable is a surface humans read. These are the
interaction-design choices carried from
[`_discovery/distillation/interaction-patterns.md`](_discovery/distillation/interaction-patterns.md)
that this initiative must **settle**, each resolved in its owning project's design
stage and recorded in that project's `_design.md`. Owning projects are named
descriptively; `/redkiln:plan` assigns the real ids. An unowned tension is how a
documented failure mode ships. Note that the perceptual review remains a skip —
`design.capture` is deliberately absent from `.redkiln/config.yaml` and there is no
app to screenshot — which makes the written resolution the *only* record of these
choices.

| ID | Tension | Options | Trade-off | Owning project |
| --- | --- | --- | --- | --- |
| DT-1 | Which prior mental model the teaching anchors against | (a) the aggregate reader; (b) the stream-per-entity reader; (c) explicitly none, teach from the invariant | Every comparable teacher relieves an existing pain and this project has none to relieve; choosing wrong addresses a reader who did not arrive, choosing none forfeits the field's most reliable device | Conceptual bridge |
| DT-2 | Whether the page-need discipline adopts a named external taxonomy or only its one-need-per-page rule | (a) adopt the taxonomy as literal structure; (b) keep the discipline, drop the enumeration; (c) adopt with a stated local extension | The taxonomy brings vocabulary and legibility but its own critics and its own site treat the enumeration as scaffolding, and it strains exactly on dense conceptual models | Page-need discipline |
| DT-3 | Where findability and navigation live in that discipline | (a) a first-class need with its own pages; (b) an implicit byproduct of the chosen structure | Treated implicitly, a landing or index page gets mis-slotted and then flagged as answering a second need; treated explicitly, the discipline grows a category its source taxonomy does not have | Page-need discipline |
| DT-4 | How the opening encounter discloses complexity | (a) staged, minimal-first with each step earning the next; (b) one complete program with commentary | Staged disclosure has controlled-study backing and directly addresses the measured defect, but reads badly for someone arriving out of sequence looking for one answer | First encounter |
| DT-5 | Whether the aggregate-to-boundary shift is taught with a diagram | (a) diagram both the old model and the new; (b) diagram neither; (c) narrate the sequence in prose or a table instead | Diagramming the new model is original work nobody in the field has done; diagramming only the old reproduces the field's own gap; narration is zero-cost and convergent but weaker for a spatial reader | Conceptual bridge |
| DT-6 | Whether a "wrong model" contrast is shown as real code | (a) real, compiled and checked forever; (b) illustrative and explicitly exempt | An unchecked wrong side is the same opted-out shape this project has already found once in its own doctests; a checked wrong side costs maintenance forever and may not even compile against the real crates | Conceptual bridge |
| DT-7 | Whether adapter- or feature-scoped content uses hidden panels | (a) tabs or folds; (b) always-visible, longer pages; (c) separate pages per scope | Hidden panels fit the fanout, but whether an inactive panel is inside the checked surface is undocumented — and this repository's worst documented failure is exactly a step that looked wired and silently was not | Checked prose |
| DT-8 | Where the line falls between a safe aside and a load-bearing constraint that may never be folded | (a) a stated rule in the discipline; (b) reviewer judgement per page | A stated rule is checkable and will be wrong at the edges; judgement is right at the edges and drifts, which is precisely how an invariant stated twice drifted here once already | Page-need discipline |
| DT-9 | Which persona the comprehension session walks first | (a) the application author; (b) the adapter author; (c) the evaluator | The three call for genuinely different comprehension checks, and the session is expensive enough that "all three" is not free; whichever is chosen, the others' evidence stays inferred | Comprehension evidence |
| DT-10 | Whether the reference surface points outward once, repeatedly, or contextually | (a) a single pointer from the crate's front door; (b) pointers at each relevant item; (c) both, with one authoritative | One pointer is the converged ecosystem norm and is easy to miss; per-item pointers meet the reader where they are and become a second navigation surface to keep true | Reach and placement |

## Risks

| Risk | Likelihood / Impact | Mitigation |
| --- | --- | --- |
| A green documentation gate is mistaken for evidence of teachability | High / High | The two proof artefacts falsify different things and neither may substitute for the other; the gate is a precondition for looking at the friction log, never a replacement for it |
| The gate is wired but never fails, and nobody notices | Medium / High | The Definition of Done requires observing it *fail* on a deliberately broken page and then recover, which is the same proof the specification tracer was made to give |
| Pages compile forever while quietly ceasing to demonstrate their own claims | Medium / High | Named as a known blind spot of every tool surveyed; the boundary example is required to fail when its boundary is removed, and the comprehension session is the second, non-substitutable instrument |
| The comprehension evidence is overclaimed as exhaustive | Medium / Medium | The claim is scoped in writing to "real stumbles were captured and are traceable"; the method supports only the narrower claim and the charter says so |
| A genuinely non-insider reader cannot be found, and an insider is used instead | Medium / High | Non-insider status is a stated precondition of the log rather than a nicety; the evidence implicates insider knowledge, not authorship, as what routes a logger around the rough spots |
| Two initiatives ship two different audiences | Medium / High | Reconciliation with the staged persona work is a business requirement and a Definition-of-Done scenario, and promotion happens once, at closeout |
| This initiative's file touches collide with the publication project's on the same pages | Medium / Medium | The seam is purpose, not paragraph, and is stated as an explicit exclusion; planning owns naming the shared files before either side edits them |
| A doc-comment rewrite silently un-discharges a frozen clause | Low / High | The exact clause-id set is pinned before any such comment is touched, and the specification cross-reference step is run over the result |
| Hidden or folded content ends up outside the checked surface | Medium / High | Owned as DT-7 and DT-8 with a required demonstration in the Definition of Done, rather than left to review discipline |
| The initiative is measured in pages written | Medium / Medium | Every acceptance criterion is framed as a reader completing something; the cross-persona finding that this is a findability problem is stated in the narrative |
| The diagram question absorbs effort disproportionate to its evidence | Medium / Medium | Ranked lowest-confidence at distillation, carried as a `Could` requirement and an owned tension, explicitly optional relative to everything above |
| The discipline lands somewhere the gate cannot read, and rots | Medium / Medium | Where it lives is an explicit requirement, with the pinned-by-path constraint stated as a condition rather than discovered later |

## Open questions for the planning team

Carried deliberately unsettled. None is to be settled in passing.

- **What the chapters are** — which need each answers, where each lives, how the
  result is hosted, and whether a given example belongs to a page or to a doctest.
  The two-surface split is strong prior art but it is an input to this decision, not
  the decision.
- **Where a documentation discipline lives, if one is written** — new constitution
  atoms, a separate corpus, playbook atoms, or nothing — given that a tree the gate
  reads must be pinned by path. Across the constitution there is no rule about
  narrative structure, audience, worked-example design or diagrams, and none of the
  seventeen decisions concerns documentation.
- **Whether `RUNBOOK.md` grows a phase.** No phase owns documentation today; its only
  documentation criterion is phase 12's "docs.rs green", which checks that HTML built.
- **Which documentation MUSTs exist, by clause id.** The seed says nine and the set is
  enumerated nowhere. Pinning it is a precondition of touching any doc comment on
  `happenstance-core`, not a task to schedule afterwards.
- **How this initiative and the staged persona work reconcile** — consume after merge,
  supersede, or run parallel and reconcile at closeout. The staged story has not run,
  so there is currently nothing to consume.
- **Which of the ten distilled opportunities are in this initiative's first pass**, and
  which are named now and deliberately deferred — the diagram gap especially.
- **Whether the adapter path waits for evidence that the application-author path
  lands**, or is taken in the same pass.
- **Whether the evaluator is a persona in its own right or an earlier stage of the
  application author's journey.** The sibling initiative's distillation asks the
  identical question about its analogous persona; it is worth resolving once.
- **Whether this project's own vocabulary answers the field's "no replacement noun"
  stall point or reproduces it.** Only the comprehension session can settle this;
  research explicitly cannot.
- **On cross-branch verification:** the intake brief's specific claims about
  `publication-and-positioning` (a `_design.md` signed off on 2026-08-12, a
  fifty-one-frame mock) and about HS-S0131's stage are **asserted and unverifiable
  from this worktree** — that initiative exists only on an unmerged branch, and
  `redkiln status` here sees two initiatives, not three. Two paths the seed cites as
  precedent, `examples/outside-projection-adapter/` and
  `references/seeds/measured-not-claimed.md`, are likewise not present in this tree.
  Planning must re-verify them against the merged tree before binding any sequencing
  to them, and must not link them as if they were reachable here.
- **On discovery coverage:** all five approved research angles, all three grounding
  passes and all three distillation passes were written and are cited below. **No
  discovery artefact is missing.**

## Context anchors

Discovery corpus:

- [`_intake-brief.md`](_intake-brief.md) — the approved intent, constraints and clause thread
- [`_discovery/research/01-rust-narrative-docs-prior-art-and-page-need-taxonomy-how-tok.md`](_discovery/research/01-rust-narrative-docs-prior-art-and-page-need-taxonomy-how-tok.md)
- [`_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md`](_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md)
- [`_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md`](_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md)
- [`_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md`](_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md)
- [`_discovery/research/05-interaction-pattern-prior-art.md`](_discovery/research/05-interaction-pattern-prior-art.md)
- [`_discovery/grounding/product-functional-alignment.md`](_discovery/grounding/product-functional-alignment.md)
- [`_discovery/grounding/backlog-adjacency.md`](_discovery/grounding/backlog-adjacency.md)
- [`_discovery/grounding/vocabulary-and-conventions.md`](_discovery/grounding/vocabulary-and-conventions.md)
- [`_discovery/distillation/opportunities.md`](_discovery/distillation/opportunities.md)
- [`_discovery/distillation/personas-and-journeys.md`](_discovery/distillation/personas-and-journeys.md)
- [`_discovery/distillation/interaction-patterns.md`](_discovery/distillation/interaction-patterns.md)

Knowledge base — the layers this work reads from and eventually writes to:

- `.kb/maps/domain-map.md` — two domains today, both architectural; documentation has
  no functional home yet, and a new subject area is an appended section, never an edit
  (`:144-150`)
- `.kb/maps/README.md`, `.kb/maps/decision-map.md`, `.kb/maps/open-questions-index.md`
- `.kb/product/README.md` — the empty product layer this closeout populates, and its
  law that a persona nobody researched is a stock photo with a name
- `.kb/design/README.md` — zero atoms today; whether narrative prose fits its admission
  rule at all is open
- `.kb/concepts/README.md`
- `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` — an existing correct
  account of the exact mechanism the opening example fails to demonstrate
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the test for any touch
  of a doc comment that already discharges a frozen clause
- `.kb/decisions/0006-bare-name-to-the-typed-layer.md` and
  `.kb/decisions/0007-projection-runner-decodes.md` — which crate a piece of teaching
  is actually describing

Standards, specification and plan:

- `standards/rust/70-rustdoc-obligations.md` — the only constitution atom on
  documentation, and the source of the counter-example that is green under the whole
  workspace
- `standards/rust/README.md:23-29` — the five-tier precedence chain this work sits
  inside and does not extend
- `spec/SPECIFICATION.md` — the normative voice a page cites and never restates;
  the frozen documentation MUSTs to preserve, whose exact set is still to be pinned
- `RUNBOOK.md:920-924` — the decorative-gate-step precedent this initiative's first
  proof artefact exists not to repeat
- `RUNBOOK.md:393` — the provisional-until-publish marker that fixes the fuse

Code and existing surfaces:

- `docs/README.md` — the 36-line signpost already reserving this tree for a
  using-the-library reader, and the statement of why the gate reads trees by path
- `crates/happenstance-core/src/store.rs` — the trait-resolution gap documented
  everywhere except the file the reader is looking at
- `examples/course-subscriptions/` — the one worked example, unpublished and unlinked,
  matching the field's canonical teaching story by convergence rather than by choice
- `references/seeds/user-documentation.md` — the seed, reconciled 2026-08-16

## Companions

Board-invisible drill-down for this card:

- [`_discovery/`](_discovery/) — the full discovery corpus: five research angles, three
  grounding passes, three distillation passes
- [`_decomposition.md`](_decomposition.md) — the project decomposition, authored later
  by `/redkiln:plan`
- [`_storymap.md`](_storymap.md) — the vertical-slice story map, authored later by
  `/redkiln:plan`

## Assumptions

- **The three named audiences are the right ones**, and the excluded fourth stays
  excluded. All three rest on this project's own audit and on secondary evidence about
  comparable readers; none has been directly observed. The comprehension session is the
  first thing that tests this assumption rather than restating it.
- **A non-author, non-insider reader can actually be recruited** within this
  initiative's timeframe. If not, the first proof artefact stands alone and the second
  is unmet — which is a failure of the initiative, not a reason to redefine the bar.
- **One comprehension session is enough** for the narrow claim being made. If the first
  session's findings are dominated by a single blocking defect, a second may be owed.
- **The strongest existing explanations are worth reaching rather than rewriting.**
  The bet is that sequencing and reach dominate content volume; if the friction log
  shows the existing explanations are themselves wrong for the reader, that bet fails
  visibly and early.
- **The specification stays the normative voice throughout.** This work is additive and
  the precedence chain is unchanged; if teaching a clause well requires changing it,
  that is a new decision record and a re-plan, not a line edit.
- **The publication project's design gate stays closed.** This initiative assumes it
  does not need to reopen a resolved design decision to do its own work. If it turns
  out that it does, that is escalated, not absorbed.
- **The sibling branch merges, or does not, on its own schedule.** Nothing here should
  be blocked on it; the audience reconciliation is designed to work in either order.

## Exit criteria for this initiative

This initiative is finished when all of the following hold together, and not before:

- Every Definition-of-Done scenario has been run and observed to pass on the assembled
  result from a clean checkout, with the deliberately-broken-page scenario having been
  observed to **fail** and then recover.
- Every acceptance criterion above is met, and each traces to at least one project and
  one business requirement.
- The dated friction log exists, is auditable in shape, and every stumble in it carries
  a disposition.
- The exact set of frozen documentation MUSTs is enumerated in one place by clause id,
  and every one of them is still discharged.
- Each open design tension above has been resolved in its owning project's `_design.md`,
  or has been explicitly and reasonably deferred with the deferral recorded — none is
  left silently unowned.
- The durable discipline has a decided home, is on disk, and is cited by the material it
  governs.
- The personas and journeys are promoted into `.kb/product/` with valid frontmatter,
  reconciled with the separately staged set, passing `redkiln validate --kb`, and linked
  from closeout.
- Each open question above has been answered, or has been converted into an
  open-question atom that says why it is deliberately still open.
- The repository's own gate is green on the exact tree that carries all of the above.

## Clarifications resolved during intake

Recorded so they are not relitigated:

- **The proof artefact is two artefacts, not one**, and they falsify different things.
  A green gate is a precondition for looking at them and never one of them.
- **The reader who knows neither event sourcing nor DCB is out of scope**, settled on
  evidence. Reopening it changes the product.
- **The published surface's claims belong to `publication-and-positioning`.** This work
  asks whether a reader can be taught, not whether a claim is true, and it does not
  reopen that project's resolved design gate.
- **Benchmarks and performance reporting belong to a different seed** and are not folded
  in on the grounds that both are documentation-shaped.
- **This initiative discharges no clause and amends none.** It is additive, and the
  precedence chain in `standards/rust/README.md` is not extended by it.
- **It freezes no port**, so the port-freeze gate box is satisfied vacuously; that
  reasoning was recorded at intake rather than left implicit.
- **`userFacing` is true**, unusually for a library and deliberately: the deliverable is
  a surface humans read. The perceptual review remains a skip because there is no app to
  screenshot, which puts the whole weight of the design record on the written `_design.md`
  resolutions.
- **Effort tier is `standard`,** scored 10 across six axes with blast radius, unknowns,
  surface and volume each at 2; the per-axis veto alone forces it.

## Related initiatives

- `.bklg/support/initiative.md` (HS-I0005) — where incidental bugs found in passing are
  routed, per `.redkiln/config.yaml`. Not a dependency.
- `.bklg/from-contract-to-published-library/initiative.md` (HS-I0006) — the umbrella
  initiative on the unmerged branch `initiative/from-contract-to-published-library`.
  **Not present in this worktree**; `redkiln status` here does not see it.
- `.bklg/from-contract-to-published-library/publication-and-positioning/project.md`
  (HS-P0016, same branch) — owns whether the published surface's claims are true. The
  scope boundary against it is stated above and is not reopened here.
- `.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/story.md`
  (HS-S0131, same branch, `stage: plan`) — the specced-but-unrun story that stages four
  personas and journeys. The single real answer to "who supplies the audience model",
  and the reconciliation point for BR-13.
- `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`
  (same branch, `promoted: false`) — an independently derived persona set that agrees
  closely with this initiative's. Corroborating, and still two authorings of one
  audience that have not been reconciled.
