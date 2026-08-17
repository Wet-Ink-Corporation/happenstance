# Seed — documenting happenstance

Raw material for `/redkiln:initiative`. This states a problem and a vision. It
deliberately does **not** decompose the work, name projects, or propose a design:
that is `/redkiln:plan`'s to decide, from its own grounding.

*Written 2026-08-11 against `main`. Reconciled 2026-08-16 against the tree including
the typed layer in flight on `initiative/from-contract-to-published-library`, from
which `0.2.0-alpha.1` was published — a rendered page this seed's first draft could
not read, because it did not exist yet. Two claims were discharged by that work and
one was found to be literally false; each correction says so where it stands, rather
than being deleted, because a seed that quietly drops a problem reads as though the
problem was never there. The vision, the audience and the constraints are unchanged.*

## Where this stands

happenstance is a storage-agnostic, DCB-compliant event sourcing library for Rust.
`happenstance-core` defines the contract, adapter crates implement it, and
`happenstance-testkit` decides whether they did.

The machinery that keeps this project's *documents* honest is unusually strong,
and all of it was built before anyone decided to write documentation.
`missing_docs`, `missing_errors_doc` and `missing_panics_doc` are workspace lints
under a gate that runs `-D warnings`, so an undocumented public item is a build
failure. `broken_intra_doc_links` is `deny`, and the documentation is built three
times — all features, `happenstance-core` with none, and a nightly `--cfg docsrs`
build — because a link that resolves in one feature configuration is a hard error
in the one nobody rendered. All four READMEs are compiled as doctests, so the
first code a reader copies cannot rot. `cargo xtask spec-trace` checks 200
clauses, 95 conformance rules and 358 citations, refuses one whose subject has
drifted more than twelve lines from its address, and reports its own coverage so
it cannot pass by checking a quarter of the corpus and saying nothing.

None of it reads a sentence.

## The problem

The gate proves that documentation *compiles* and that an address *resolves*.
Nothing establishes that a person who installed this crate can learn to use it,
which is the property a library is finally judged on — and the one with the
shortest fuse, because a crates.io release cannot be edited and docs.rs renders a
version once.

Six things are true today and should not be:

**The directory reserved for the reader who installed the crate is empty.**
`docs/` holds one file, a 36-line signpost, and its own admission rule already
names three quadrants it does not contain — "guides, tutorials, how-to pages, and
reference material addressed to someone *using* the library". The gap is named in
the evaluation corpus too, as a Medium-severity missing phase covering the DCB
mental model, migrating from aggregates, and testing decision models. No phase of
`RUNBOOK.md` owns it. The runbook's only documentation criterion is that docs.rs
renders green, which is a check that the HTML built.

**Nothing in the gate reads a sentence, and the corpus is written as though it
does.** RS-70-5 — *name the alternative that lost, in the doc comment, once* —
exists because prose is the only instrument that stops a settled decision being
re-proposed, and the only one nothing enforces. "Returns the head position.
# Errors: Returns an error on failure." satisfies every lint in the workspace. So
does a book of ```` ```ignore ```` fences: the testkit's own two doctests were
found to be *"elaborate spellings of ```` ```ignore ````"*, in the crate whose
thesis is that a claim is worth what its test is worth.

**The first program a reader meets demonstrates the API and omits the point.**
Every citation in this paragraph is against
`initiative/from-contract-to-published-library`, which is where the typed layer
lives and where `0.2.0-alpha.1` was published from; none of them resolves on `main`,
whose `crates/happenstance/src/lib.rs` is still the seventy-five-line facade.
`crates/happenstance/src/lib.rs:24-62` opens the crate documentation with a
thirty-line example carrying eight imports and a hand-written five-method
`DomainEvent` impl, before a single concept has been named. It calls
`Tags::empty()` twice — at `:38` and `:55` — so the consistency boundary, the one
mechanism DCB exists to provide, is zeroed out in the first program anyone runs.
The page then explains that composing decision models "is the mechanism that makes
a dynamic consistency boundary *dynamic*" (`:99-101`), which the example has
already declined to show. A reader who copies the opening program and changes the
domain has written classical event sourcing with extra ceremony, and nothing on
the page tells them so.

*(This replaces an earlier reading of the same problem. When this seed was first
written the bare-name crate carried neither `[package.metadata.docs.rs]` nor
`#![cfg_attr(docsrs, feature(doc_cfg))]`. Both landed with the typed layer —
`crates/happenstance/src/lib.rs:178` and `crates/happenstance/Cargo.toml:114-116` —
so the rendering complaint is discharged and the teaching one is what is left.)*

**The contract crate's front page defers its own example, and its largest modules
explain nothing.** `happenstance-core`'s "Getting started" section holds no code
fence; it points at another item's documentation. Five modules — `event.rs` at
988 lines, `memory.rs` at 851, `tag.rs` at 706, `query.rs` at 555, `append.rs` at
389 — carry a single `//!` line each, because `missing_docs` is a rule about
items and one sentence satisfies it. The error a new user is most likely to hit
is documented in a rendering rustc does not produce: `store.rs` shows `E0034`
without its two `= note:` candidate lines, so `TraitVariantBlanketType` — the
string a confused reader would paste into a search — appears on no published
surface. It is in the workspace six times, in `RUNBOOK.md`,
`references/adr/0008-one-derivation-for-both-ports.md` and three evaluation
documents; every one of them is contributor-facing, and none of them is
`crates/happenstance-core/src/store.rs`, which is the file the reader is looking
at when they need it.

**There is one worked example, and the evidence base cites it as a hazard rather
than as teaching.** `examples/course-subscriptions` is 237 lines with no prose
page anywhere. The architectural evaluation records that all three of its
handlers are the same append-condition shape *by accident*, carrying an unclaimed
at-most-once guarantee the project does not know it has; and that its query and
its fold state the consistency boundary twice with nothing checking they agree —
a reviewer added a sixth event type, taught the fold, forgot the query, and the
program silently oversold 7 units of 0 stock with no compiler, clippy or
conformance signal. That hazard is 100% on the user, and command-retry
idempotency guidance is recorded as absent.

It is also invisible. `examples/course-subscriptions/Cargo.toml:8` is
`publish = false`, so the example ships in no package, is `include_str!`'d into no
rustdoc, and is pointed at from nothing a docs.rs reader can reach — its only
signpost is `cargo run -p course-subscriptions` in the workspace README, which is
itself unpublished (`xtask/Cargo.toml:7`, and the repository README is compiled as
that crate's doctest precisely because it cannot ship). The same is true of the
strongest short DCB argument in the repository, the nineteen-line module doc at
`examples/course-subscriptions/src/main.rs:1-19`. The one piece of flagship
teaching that *is* published — the read-decide-append walk-through on
`MemoryEventStore` — sits on an item page a reader reaches only by already knowing
to go there. So the three best explanations this project has written are, in
rendered terms, either absent or unfindable, and the front page a reader actually
lands on carries none of them.

**Nothing states who any of it is for.** `.kb/product/` holds a scaffold README
and zero persona atoms; `.kb/design/` is empty; the domain map has two domains
and neither is documentation. Every documentation decision so far has been taken
against an audience model that exists only inside two reviews — including the
good ones, like the decision that a reader who knows neither event sourcing nor
DCB is out of scope. `.kb/product/README.md` states the cost itself: *"A persona
nobody researched is a stock photo with a name."*

## The vision

A library whose documentation is held to the same standard as its code: a reader
who has installed the crate can be taught, and the teaching is checked rather than
asserted.

Concretely, that means: narrative documentation that a compiler reads, so a page
cannot drift from the API the way prose normally does; a reader who is not the
author getting through it, with what they got stuck on written down; a clear
answer to *which page answers which need*, so that a page can be finished rather
than merely added to; and a rendered result that a Rust developer meets where they
already look, rather than a site they have to be told about.

## Who it is for

**Application authors** who ran `cargo add happenstance` and have a decision to
model. They are first, and the consequence is that the teaching starts from their
problem — the invariant that spans two entities — and not from this library's
architecture. They are also the reader nobody has watched: no phase puts the API
in front of a user who is not its author before it freezes.

**Adapter authors**, who are better served than anyone — a four-step recipe in
`CONTRIBUTING.md`, a 314-line constitution atom, and a suite that tells them when
they are done. What they have is reference and how-to. Nobody has been walked
through building one, and there is no third-party adapter to read.

**The evaluator**, who reads for twenty minutes and decides whether to depend on
this. The README is written for them and is good; what it cannot do is survive
the second question, because there is nowhere for that question to go.

**Not** the reader who knows neither event sourcing nor DCB. That was settled on
evidence — the README links the specification instead — and work that quietly
reopens it has changed the product.

## What must remain true

These are constraints on any answer, not preferences:

- Narrative documentation is compiled against the real crates by the gate, or it
  does not ship. A page of ```` ```ignore ```` fences is the documentation form of
  a `todo!()` body: it type-checks against anything.
- The medium is the ecosystem's own — rustdoc for reference, a book for narrative.
  A Rust reader looks for a crate's guide in two places, and neither is a bespoke
  site.
- Which need a page answers is a decision with a name, and a page answering a
  second need has a defect. That discipline is the point; a directory layout is
  not.
- Where any page and `spec/SPECIFICATION.md` disagree, the specification wins. A
  guide cites clauses and never restates them, and the precedence chain in
  `standards/rust/README.md` is not extended by this work — the book slots below
  the constitution, not beside the specification.
- Nine documentation MUSTs that `[FROZEN]` clauses impose on `happenstance-core`'s
  own doc comments were discharged in the reconciliation between phases 5 and 6.
  Work that rewrites those comments leaves them discharged.
- Evidence beats argument: the artefact that settles this must be one that **would
  not exist if the design were wrong**. For prose that means a reader who is not
  the author, and a gate step that fails on a page deliberately broken to prove it
  — the same proof the specification's own tracer was made to give.
- Whatever ships at first publish is permanent for that version. A crates.io
  release cannot be edited, and docs.rs renders a version once.

## Supporting material

Read these rather than trusting the summary above; several are long and all of
them are more specific.

| Path | What it carries |
| --- | --- |
| `references/evaluation/review-dx-ergonomics.md` | 712 lines on what it feels like to build a real application on this, cold: the docs read from a standing start, the `E0034` reproduction, the retry loop that cannot be written the obvious way, and a friction log in the order it was encountered. |
| `references/evaluation/review-docs-adr.md` | 474 lines auditing whether this project's documents are true, coherent and load-bearing — including the missing-phases table that files a guide, and the finding that two doctests verified nothing. |
| `references/evaluation/ARCHITECTURAL-EVALUATION.md` | The whole-workspace synthesis. Findings S6 and A1 are what the one worked example actually demonstrates, both measured rather than argued. |
| `references/evaluation/research-rust-api-guidelines.md` | 765 lines auditing this crate against the Rust API Guidelines, every claim compiled. Says which documentation guidelines are already met, which is most of them. |
| `standards/rust/70-rustdoc-obligations.md` | Five rules on what a doc comment owes, each with a compiled example and the wrong implementation it rejects. `62-doctests-and-harnesses.md` is its companion on making an example prove something. |
| `standards/rust/91-adapter-authoring-recipe.md` | 314 lines of adapter how-to, and the best evidence of what this repository's documentation looks like when it is good. |
| `docs/README.md` | 36 lines reserving the tree, routing every other reader elsewhere, and stating the test for what belongs. |
| `spec/SPECIFICATION.md` | 200 numbered clauses, each with a maturity marker, the rule that checks it, and the wrong implementation it forbids. Current truth, and the thing a guide cites rather than repeats. |
| `RUNBOOK.md` | The plan of record. Phase 12 is where documentation stops being editable; the phase 0 log is how the rustdoc gate came to exist, including a documentation step that printed warnings and exited 0 for as long as it had run. |
| `examples/course-subscriptions/` | The canonical DCB example, 237 lines, no prose page, rewritten by phase 7. |
| `examples/outside-projection-adapter/` | **The precedent for a comprehension instrument, and it already exists.** A projection adapter written from the rendered documentation alone, in a crate where the orphan rule and the non-dev dependency graph behave as they do for a stranger. It is an artefact that would not exist if the documentation were adequate — which is the standard of evidence this seed asks for, already met once. It lives on `initiative/from-contract-to-published-library`, not on `main`. |

## Deliberately not decided here

What the chapters are, which need each page answers, where each one lives, how the
result is hosted, and whether a given example belongs to a page or to a doctest.
Also how this decomposes: the constraints above rule out a large class of answers
and are meant to, but the shape of the answer inside them is planning's to choose,
against grounding this seed does not have.

**Where the published surface stops and this work starts.** `publication-and-positioning`
(HS-P0016) already owns the first screen, the status vocabulary, the maturity
census, the peer statement, the docs.rs manifest metadata and the rendered-page
preflight, against a `_design.md` signed off with no conditions on 2026-08-12 and a
fifty-one-frame mock. That work is about whether what the surface *claims* is true;
this work is about whether a reader can be *taught*. The two meet on the same files
— `crates/happenstance/README.md` and the crate's own front page — and the seam is
stated here rather than drawn, because drawing it means reading both bodies of work
against each other. What is settled is only the direction: this initiative does not
reopen a resolved design gate.

**Where a documentation standard lives, if one is written.** The constraint above
says the precedence chain in `standards/rust/README.md` is not extended by this
work. But nothing in this repository currently records how to write a page that
teaches: across all twenty-seven constitution atoms there is no rule about
narrative structure, audience, worked-example design or diagrams, and
`70-rustdoc-obligations.md` — the only atom whose subject is documentation — spends
four of its five rules on lint plumbing and link resolution. Whether the gap is
filled by new constitution atoms, a separate corpus, `.kb/playbooks/` atoms, or
nothing at all is planning's, and it is genuinely constrained: a new tree that the
gate reads must be pinned by path in `xtask/src/` the way `spec/` and
`standards/rust/` already are, or it is a tree nothing checks.

**Whether the plan of record grows a phase.** `RUNBOOK.md` has no documentation
phase and its only documentation criterion — phase 12's *"docs.rs green"* — checks
that HTML built. Whether this work adds a phase, attaches to phase 12, or is
declared to sit outside the runbook entirely is undecided; what is not is that
phase 12 is the irreversible act, and it currently depends on phases that have not
started.

**Who supplies the audience model.** Problem six above says no persona atom exists.
Story HS-S0131 (`persona-and-journey-intake-staging`, under HS-P0019) is already
specced to stage four personas and their journeys into `.kb/_intake/`, and is at
`stage: plan`. Whether this initiative consumes that output, supersedes it, or
sequences behind it is a real dependency and not a detail — every documentation
decision downstream is framed against those personas, and two initiatives
independently inventing them is the failure mode.
