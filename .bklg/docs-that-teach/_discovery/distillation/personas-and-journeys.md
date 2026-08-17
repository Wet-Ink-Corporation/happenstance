---
title: Personas & journeys — From Accurate to Teachable
initiative: HS-I0007
slug: docs-that-teach
status: draft-for-planning
promoted: false
scope: problem-space-only
sourcePaths:
  - .bklg/docs-that-teach/_intake-brief.md
  - references/seeds/user-documentation.md
  - .bklg/docs-that-teach/_discovery/research/01-rust-narrative-docs-prior-art-and-page-need-taxonomy-how-tok.md
  - .bklg/docs-that-teach/_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md
  - .bklg/docs-that-teach/_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md
  - .bklg/docs-that-teach/_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md
  - .bklg/docs-that-teach/_discovery/research/05-interaction-pattern-prior-art.md
  - .bklg/docs-that-teach/_discovery/grounding/product-functional-alignment.md
  - .bklg/docs-that-teach/_discovery/grounding/backlog-adjacency.md
  - .bklg/docs-that-teach/_discovery/grounding/vocabulary-and-conventions.md
  - .kb/product/README.md
---

# Personas & journeys — From Accurate to Teachable

Three personas, distilled from the seed's own "Who it is for"
(`references/seeds/user-documentation.md:147-166`) and the intake brief's
restatement of the same three (`_intake-brief.md:149-160`), stress-tested
against the five research angles gathered for this initiative. **None is
promoted to `.kb/product/` yet** — `.kb/product/README.md`'s own bar, "a
persona nobody researched is a stock photo with a name," is not fully met
until a real non-author reader has actually walked one of these journeys
(the friction-log proof artefact this initiative is already committed to).
Until then these are draft sketches for planning, built from the strongest
evidence available before that contact happens: the workspace's own measured
findings (the alpha's opening example, the course-subscriptions hazard) plus
external evidence about how comparable readers behave elsewhere.

Problem-space only. Nothing below names a chapter, a tool, a diagram
convention, a file layout, or a gate mechanism; where a pain point has an
obvious content or tooling fix, the fix is left out on purpose and the pain
is stated as a need instead.

**A fourth, explicitly out of scope.** The seed and the intake brief both
name a boundary rather than leave it implicit: *"the reader who knows
neither event sourcing nor DCB is out of scope — settled on evidence, and
work that quietly reopens it has changed the product"*
(`references/seeds/user-documentation.md:164-166`; `_intake-brief.md:80-81`).
Every persona below already has event-sourcing vocabulary (streams,
projections, aggregates, CQRS) before they ever meet `happenstance`. What
none of them can be assumed to have is DCB vocabulary specifically — and
research digest 03 shows that *which* prior model they arrive with (a DDD
aggregate reader, a stream-per-entity/EventStoreDB reader, or something
else) is itself unresolved, not a detail. See Open Questions.

---

## Persona 1 — The application author

**Who they are.** A Rust developer who ran `cargo add happenstance` because
they have a decision to model with an invariant that spans more than one
entity — the class of problem DCB names. They are first among the three,
and the seed is explicit about the consequence: *"the teaching starts from
their problem — the invariant that spans two entities — and not from this
library's architecture"* (`references/seeds/user-documentation.md:149-151`).
They are also, in the seed's own words, *"the reader nobody has watched: no
phase puts the API in front of a user who is not its author before it
freezes"* (`references/seeds/user-documentation.md:152-153`).

**Goal.** Model their own cross-entity consistency boundary correctly on
the first real attempt, using the library's own vocabulary — not a
generic event-sourcing vocabulary they have to translate — and know,
without having to be told twice, why the boundary they wrote actually
holds.

**What happens to them today, measured rather than argued.** The published
`0.2.0-alpha.1`'s opening program — eight imports, a hand-written five-method
`DomainEvent` impl, thirty lines before a single concept is named — calls
`Tags::empty()` twice, at the two points that matter
(`crates/happenstance/src/lib.rs:38,55` per `_intake-brief.md:23-25`). The
same page then states, a few dozen lines later, that composing decision
models *"is the mechanism that makes a dynamic consistency boundary
dynamic"* — a claim the example the reader just ran has already declined to
demonstrate. A reader who copies that program and swaps in their own domain
has, in the seed's words, *"written classical event sourcing with extra
ceremony, and nothing on the page tells them so"*
(`references/seeds/user-documentation.md:77-78`). If they go looking for the
one worked example that does show the boundary
(`examples/course-subscriptions/`), it ships in no package
(`publish = false`), is `include_str!`'d into no rustdoc, and is reachable
only via a signpost inside a README that is itself unpublished
(`references/seeds/user-documentation.md:112-117`). If they reach it anyway,
the evidence base — not an opinion — records that its query and its fold
state the same consistency boundary twice with nothing checking they agree,
and that a prior reviewer added an event type, taught the fold, forgot the
query, and the program silently oversold inventory with no compiler, clippy
or conformance signal (`references/seeds/user-documentation.md:104-110`). The
one example available to teach the boundary is, today, evidence cited as a
hazard rather than as teaching.

**What they are afraid of.** Building on a mental model that looks right,
compiles, runs, and is quietly wrong — discovering the gap only once real
data depends on it, the way the course-subscriptions hazard was itself
discovered: by accident, after the fact, with nothing in the tooling
signalling it. This is the same fear the seed states directly and the
concept atom on torn reads backs independently (per grounding): a defect
found by the reader, not one the project found and fixed first.

**Journey — today.**
1. Arrives with an existing event-sourcing vocabulary (streams, aggregates,
   projections, CQRS) but not necessarily DCB's — research digest 03 shows
   this reader's *specific* prior model (DDD aggregate vs. stream-per-entity
   vs. something else) is not knowable in advance without deciding to anchor
   against one.
2. Reads the crate-root opening example expecting it to teach the one thing
   DCB exists for; it teaches API shape instead and zeroes out the boundary
   twice in the process.
3. Reads the crate's own prose a few lines later claiming the mechanism the
   example just failed to show — nothing on the page flags the gap between
   the two.
4. Goes looking for a fuller worked example; the one that exists is
   unshippable, unlinked, and — per the evidence base — internally
   inconsistent about the very invariant it exists to demonstrate.
5. Either proceeds having internalized the wrong lesson (the seed's exact
   phrase: "classical event sourcing with extra ceremony"), or stalls,
   unable to tell which outcome happened, because nothing checks their
   understanding at any point in the path.
6. No diagram anywhere in the workspace shows the query/append-condition
   write cycle itself (research 03, finding 3) — the one moment DCB's
   difference from everything this reader already knows would be visible at
   a glance.

**Journey — what the initiative should improve (stated as outcomes, not
mechanism).** The gap between step 2 and step 3 — a page teaching syntax
while claiming to teach semantics — is the seed's own diagnosis and the
clearest single target. Step 4's example needs to be *reachable* by this
reader and needs its own stated invariant to actually hold under the same
scrutiny the code around it receives; whether that invariant is stated
narratively, checked mechanically, or both is planning's to decide. Step 1
is a real open question this research surfaces but does not resolve: which
prior mental model (or models) the teaching deliberately argues against is
a scoping decision every comparable project made explicitly, and this
initiative has not made yet. Step 5 is the one this reader's fear names
directly: the comprehension check (paraphrase-style, per research 04 —
"tell me in your own words why this boundary holds") is the shaped-to-fit
instrument for exactly this persona's stated goal.

---

## Persona 2 — The adapter author

**Who they are.** Someone implementing the storage contract against a real
system. Today this is exclusively the project's own team — six skeleton
crates, `todo!()` bodies, `publish = false` — but the seed is explicit that
the persona is general: whoever eventually writes a seventh adapter needs
the same thing the first six needed
(`references/seeds/user-documentation.md:155-158`). The seed's own verdict on
how well this reader is served today is unusually specific: *"better served
than anyone... What they have is reference and how-to. Nobody has been
walked through building one, and there is no third-party adapter to
read"* (`references/seeds/user-documentation.md:155-158`).

**Goal.** Understand *why* the contract is shaped the way it is — not just
what to implement, but the reasoning that makes the shape non-arbitrary —
so that when their own storage system disagrees with an assumption baked
into the port, they can tell the difference between "the port is wrong for
me" and "I have not understood the port yet."

**What happens to them today, measured rather than argued.** They have a
314-line constitution atom and a four-step recipe in `CONTRIBUTING.md` —
genuinely strong reference and how-to material, and the seed says so without
qualification. What they do not have is a narrative walk-through of the
reasoning, or a second, independent implementer's account of hitting the
same wall they are about to hit. The seed names one specific, concrete
instance of this gap: the error a new adapter author is most likely to
trigger — `E0034`, from the workspace's two-trait `Send`/non-`Send` design —
is documented in a form rustc itself does not produce (missing its two
`= note:` candidate lines), and the string a confused reader would actually
search, `TraitVariantBlanketType`, appears six times in the workspace, in
`RUNBOOK.md`, an ADR, and evaluation documents — *all* contributor-facing,
*none* of them `crates/happenstance-core/src/store.rs`, the file the reader
is looking at at the moment they need it
(`references/seeds/user-documentation.md:86-99`).

**What they are afraid of.** Hitting a wall — a coherence error, an
assumption their storage system cannot meet — and not being able to tell,
from where they are standing, whether it is a bug in their understanding or
a real limit of the port, because nobody has narrated what hitting that same
wall looked like for the team that wrote it.

**Journey — today.**
1. Reads the specification, the conformance suite, and the recipe to learn
   what "correct" means for their storage system — strong reference
   material, by the seed's own account.
2. Implements the trait; hits the two-trait design's coherence boundary and
   gets `E0034`.
3. Looks at the file they are actually staring at — `store.rs` — for an
   explanation, and finds none; the explanation exists, but only in three
   documents this reader has no reason to already know about.
4. Has no narrative account of *why* the two-trait split exists to check
   their own understanding against — only the recipe (what to do) and the
   spec (what must hold), not the reasoning a first-person walkthrough would
   carry.
5. Eventually passes the conformance suite (or does not), but their path to
   getting there was never observed by anyone outside the project's own
   team — there is, per the seed, no third-party adapter to read and no
   record of a stranger's attempt.

**Journey — what the initiative should improve.** The gap between step 2
and step 3 is a findability problem with a name attached to it already —
the explanation exists, it is simply shelved in contributor-facing
documents instead of the file the reader has open. The gap across steps 3-5
— no narrative walkthrough, no third-party account — is exactly what the
repository's own precedent for this persona's proof artefact already
demonstrates is possible: `examples/outside-projection-adapter/` is *"a
projection adapter written from the rendered documentation alone, in a
crate where the orphan rule and the non-dev dependency graph behave as they
do for a stranger"* (`_intake-brief.md:130-132`), an artefact the seed calls
*"the precedent for a comprehension instrument, and it already exists"*
(`references/seeds/user-documentation.md:212`). This persona's journey has
already been walked once, for a projection adapter; whether an equivalent
walk exists (or should exist) for an event-store adapter is open for
planning.

---

## Persona 3 — The evaluator

**Who they are.** A Rust developer who has not yet decided to adopt
anything, reading for a bounded amount of time before choosing whether to
depend on this. The seed places them precisely: *"reads for twenty minutes
and decides whether to depend on this"*
(`references/seeds/user-documentation.md:160-162`).

**Goal.** Decide, inside that twenty-minute budget, whether the claims that
matter to them (storage-agnostic, DCB-compliant, actually teachable to their
own team later) are real — and, critically, find out *where to go next* if
the README answers their first question but raises a second one.

**What happens to them today, measured rather than argued.** The seed's
verdict is specific and double-edged: *"The README is written for them and
is good; what it cannot do is survive the second question, because there is
nowhere for that question to go"* (`references/seeds/user-documentation.md:161-162`).
This reader is not stalled by a bad first page — they are stalled by a
dead end after a good one. Research digest 01 independently names a
structural reason a landing/orientation page like this is easy to get
wrong without noticing: Diátaxis's four-category split does not cover
findability or navigation by design, so a page whose job is "help this
reader find the next page" can be built and reviewed as if it belonged to
one of the four categories and never checked against its actual job — a
distinct structural concern from the four content-need categories
themselves.

**What they are afraid of.** The same fear Persona 1 carries, but earlier
and lower-stakes to voice: adopting — or, just as costly, wrongly rejecting
— on the strength of a first impression that turns out not to generalize
once they ask the second question themselves.

**Journey — today.**
1. Finds the README; it answers the first question well (per the seed's own
   assessment) — is this storage-agnostic, is this DCB-compliant, does the
   architecture make sense.
2. Forms a second, more specific question — often exactly the kind of
   question research digest 03's three named stall points predict: is
   "dynamic" a synonym for "unstructured," is DCB a modeling technique or a
   routing technique, what replaces the vocabulary they already know
   (Aggregate) in this library's own terms.
3. Has nowhere the README points them for that second question — the seed's
   own phrase is literal: "nowhere for that question to go."
4. Either abandons evaluation with the question unanswered, or falls back to
   reading `spec/SPECIFICATION.md` directly — 200 numbered clauses, written
   for precision rather than for a twenty-minute reader — a different kind
   of document than the one their second question actually calls for.

**Journey — what the initiative should improve.** The gap is at step 3, and
it is the sharpest, most self-contained target of the three personas: this
reader does not need more content, they need the *next* page to exist and
be findable from where they already are. Research digest 04's taxonomy
names the fitting comprehension check directly — plus-minus testing (a
reader marks + or − through the README as they read, then is asked why at
each mark) is built for exactly a document whose "job is to build general
understanding or trust," which is this persona's stated goal in the seed's
own words.

---

## Cross-persona tensions

- **All three personas share one root cause, approached from different
  distances.** Persona 1 hits it inside a running program (the example
  zeroes the boundary it claims to teach); Persona 2 hits it inside a
  compiler error (the explanation exists but not where they are looking);
  Persona 3 hits it inside a decision (the second question has nowhere to
  go). In every case the content that would answer the reader exists
  somewhere in the workspace — the seed's evidence never says the knowledge
  is missing, only that it is unreachable from where the reader actually
  stands. Findability-at-the-point-of-need, not raw content gaps, is the
  common shape underneath all three journeys.
- **Persona 1 and Persona 2 both carry a "silent wrongness" fear, not a
  "missing information" fear.** Both are afraid of a state that looks
  correct and is not — an example that runs but teaches the wrong lesson, a
  wall that looks like their own misunderstanding but might be the port's
  limit. This is a sharper, more specific version of "documentation is
  incomplete," and it argues (per research 04) for a comprehension check
  keyed to correctness of understanding, not just task completion.
- **Persona 3's timing is different in kind, not just degree.** Personas 1
  and 2 can revise their understanding over days or weeks of contact with
  the code; Persona 3's entire journey happens inside one reading session,
  with no second attempt if the first one fails silently. Whatever this
  initiative decides is "good enough" for Persona 3 has to be right on
  first contact, in a way the other two personas' journeys do not demand.
- **Persona 1's opening-example failure and Persona 3's README success sit
  on the same continuum of "how much does a reader need to already know."**
  The seed calls the README good and the opening code example a trap —
  worth noting to planning that "the front door is fine" and "the first
  code sample is not" can both be true on the same page, and a fix aimed at
  one will not automatically reach the other.

## Risks

- **None of these three personas has been directly observed by this
  project.** Every claim above is drawn from the seed's own measured
  findings (the alpha's opening example, the course-subscriptions hazard,
  the E0034 documentation gap) or from secondary evidence about comparable
  readers elsewhere (research digests 01-05). That is consistent with a
  library whose typed layer and documentation work is still in flight, but
  it means these are *inferred* audiences until the friction-log proof
  artefact actually walks one. `.kb/product/README.md`'s own bar should be
  re-checked once that contact happens.
- **A fourth persona is implicit in the proof artefact but not named
  above.** The friction-log reader described in the intake brief — "a
  reader who is not the author" — is not automatically identical to any one
  of the three personas; they could be recruited as a stand-in for Persona
  1, Persona 2, or Persona 3 depending on which journey the log is meant to
  test. Which persona the friction log actually walks is a choice, not a
  given, and conflating "the friction-log reader" with "the audience" would
  understate that.
- **Persona 2's evidence is the thinnest of the three on the "no third-party
  adapter" claim.** The seed states it as fact (`references/seeds/user-documentation.md:155-158`),
  but nothing in the research digests independently corroborates or
  contests it — it rests entirely on the seed's own internal audit. Worth
  flagging rather than treating as externally validated the way, for
  instance, the E0034 findability gap is (that one is a direct grep of the
  workspace, reproducible by anyone).
- **Duplicate-persona risk is live, not hypothetical.** Grounding confirms
  story HS-S0131 (`persona-and-journey-intake-staging`, under HS-P0019 on
  the unmerged `initiative/from-contract-to-published-library` branch) is
  already specced to stage four personas — including an "application
  author" and an "adapter author" that overlap Personas 1 and 2 above almost
  exactly — into `.kb/_intake/`, and sits at `stage: plan`, not yet run. If
  this initiative's planning treats the sketches above as final without
  reconciling against that story's eventual output, two initiatives will
  have independently invented overlapping personas — the exact failure mode
  `.kb/product/README.md` and the intake brief both name. The two
  distillations were produced independently (this one from `docs-that-teach`
  discovery, the other already on file at
  `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`
  on the sibling branch, not reachable from this worktree's own tooling) and
  agree closely on Persona 1 and Persona 2's shape — which is corroborating,
  not a coincidence to ignore, but it is still two authorings of the same
  audience that have not been reconciled into one.

## Open questions for the planning team

- **Which prior mental model does the teaching argue against?** Research
  digest 03 shows every comparable project (dcb.events, Marten,
  EventStoreDB) made this choice explicitly and early — a DDD-aggregate
  reader, a stream-per-entity reader, or (for the Rust-ecosystem peers)
  deferred the question entirely by starting from wire types. happenstance
  has made no such choice yet, and the three personas above may not all
  share the same answer.
- **Should Persona 3 (the evaluator) be treated as a separate persona from
  Persona 1, or as an earlier stage of the same journey?** The seed
  describes them with different verbs (reads for twenty minutes and
  decides vs. has a decision to model) but the same underlying fear
  (adopting on a claim that does not generalize). The sibling initiative's
  own distillation asks the identical question about its analogous
  "evaluator" persona — worth resolving once, not twice.
- **Does the friction-log proof artefact target one persona's journey, or
  more than one?** Nothing in the intake brief pins this down, and the
  three journeys above call for genuinely different comprehension checks
  (paraphrase for Persona 1's mental model, a findability check for Persona
  2's E0034 gap, plus-minus for Persona 3's trust-building) per research
  digest 04's own taxonomy.
- **How does this initiative's audience model reconcile with HS-S0131's
  eventual output?** The story is specced but unrun, on a branch this
  worktree's own `redkiln status` cannot see. Whether docs-that-teach
  should consume that story's staged personas once merged, supersede them
  with the sketches above, or run in parallel and reconcile at closeout is
  a real sequencing decision, not a detail.
- **What is the missing noun, and does happenstance need to answer it?**
  Research digest 03 names "no settled replacement noun for what an
  Aggregate used to name" as an independently-observed stall point across
  the wider DCB community. happenstance already has its own vocabulary
  (`Tags`, `AppendCondition`, `SequencePosition`, taken directly from the
  spec) — whether that vocabulary already answers this stall point for
  Persona 1, or reproduces it, is exactly what a friction log would reveal
  and this research cannot settle in advance.
