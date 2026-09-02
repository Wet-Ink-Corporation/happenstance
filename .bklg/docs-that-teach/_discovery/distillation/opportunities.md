---
initiative: HS-I0007
slug: docs-that-teach
distillation: opportunities-and-archetypes
stage: discovery
generated: 2026-08-16
promoted: false
scope: problem-space-only
sources:
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
---

# Opportunities and Archetypes — From Accurate to Teachable

Problem-space distillation only. Nothing below names a tool, a crate layout, a
file format, or an implementation choice — those are planning's to make, against
this and the grounding it rests on. Each entry states what a reader would
experience, the evidence that the gap is real and costly, and the gap it closes.
Ranked by value (how much of the seed's core pain it addresses) then confidence
(how directly the evidence supports it).

## How to read the ranking

Value is about the reader outcome, not the effort to get there. Confidence is
about how many independent sources — the seed's own measured incidents, the
research angles, or an outside project's own admitted gap — point the same
direction. An item can be high-value and still be listed below a lower-value
item if its confidence is thin; that ordering is intentional, not an error.

---

## Opportunity 1 — A first fifteen minutes that teaches the one thing DCB is for

**What it is.** The reader's very first encounter with the crate — the code they
copy before they have formed any opinion about the library — demonstrates the
consistency boundary, not just the API surface. Two entities, one invariant that
spans them, a boundary that is genuinely non-empty. The reader leaves the first
program having *seen* the thing the library exists to provide, not having
transcribed a syntactically valid program that happens to compile.

**Evidence.** This is the seed's own sharpest, most concretely measured finding:
the published `0.2.0-alpha.1` opening example calls `Tags::empty()` twice
(`crates/happenstance/src/lib.rs:38,55`), zeroing the boundary in the first
program anyone runs, while the same page later asserts that composing decision
models "is the mechanism that makes a dynamic consistency boundary *dynamic*"
— a claim the example has already declined to demonstrate. Independently, the
interaction-pattern research (angle 05) found direct empirical backing for
exactly this fix in the "training wheels" literature (Carroll's controlled
studies, cited by NN/G): staged, minimal-first disclosure measurably outperforms
a complete-but-dense opening example, and it requires no new machinery — it is
a reordering of existing material. The aggregate-boundary research (angle 03)
also confirms every comparable project teaches this shift by re-narrating the
reader's *existing* pain (locking, cross-entity constraints) before introducing
new vocabulary, which the current eight-import, two-`Tags::empty()` opening
program does not do.

**Gap it closes.** "Accurate and unteachable" stated as concretely as the seed
states it anywhere: a reader who copies the opening program and changes the
domain has written classical event sourcing with extra ceremony, and nothing on
the page tells them so. This is the single highest-confidence, lowest-new-machinery
opportunity in the whole set — it is a content and sequencing fix backed by a
named literature, not a new proof mechanism.

---

## Opportunity 2 — Prose that cannot lie about the API without being caught

**What it is.** A reader's experience of trusting a paragraph of narrative
explanation the same way they already trust the crate's compiled doctests: if
the code inside a narrative page stops matching the real crate, the page itself
fails to build, not just the isolated snippet the author remembered to test.
The reader never encounters an explanation that was true when written and false
when read.

**Evidence.** The seed's own words: "a page of `ignore` fences is the
documentation form of a `todo!()` body: it type-checks against anything," and
the workspace has already paid for the softer version of this once — a
documentation gate step that printed "generated 3 warnings" and exited 0 for
as long as it ran. The prior-art research (angle 02) confirms narrative pages
whose code compiles for real against a live crate is an established, working
pattern elsewhere in the Rust ecosystem — not a novel or risky ask — while also
surfacing the honest limit: every compiled-prose mechanism surveyed proves a
page's code still matches the API, none of them prove the page still teaches
the right lesson. That second claim needs a different instrument (Opportunity 5).

**Gap it closes.** The seed's central diagnosis: "unusually strong machinery
keeping documents honest... and none of it reads a sentence." This opportunity
closes the compiles-and-still-true half of that gap; it does not by itself
close the teaches-correctly half, and should not be sold as though it does.

---

## Opportunity 3 — Every page answers one named question, so a reader always knows why they're on it

**What it is.** A reader arriving at any narrative page can state, without
guessing, what question brought them there and whether this is the page that
answers it. No page silently drifts into answering a second question it was
never designed to carry — the way an "open-ended sink" accumulates whatever a
reviewer discovered was missing, until the page answers everything a little and
nothing well.

**Evidence.** The seed states this as a hard constraint in the reader's own
terms: "which need a page answers is a decision with a name, and a page
answering a second need has a defect." The prior-art research (angle 01) shows
this discipline is older and broader than any one framework — Mark Baker's 2011
"Every Page is Page One" names the identical failure mode and the identical fix
independently of Diátaxis — and that no Rust-ecosystem peer (tokio, serde,
diesel, axum) currently makes a page's answered-need a labeled, checkable
property; this would be a stricter discipline than ecosystem practice, not an
imported convention. The research is equally clear that the strict four-category
completeness claim some frameworks attach to this discipline is contested and
that a landing/findability need doesn't fit cleanly into any of the standard
four — a fifth, distinct concern worth naming rather than mis-slotting
(Opportunity 7).

**Gap it closes.** The seed's evaluator persona experience directly: "what it
cannot do is survive the second question, because there is nowhere for that
question to go." A reader who knows which page they're on, and why, is a
reader who can find the next page instead of concluding the documentation
stopped.

---

## Opportunity 4 — A conceptual bridge from the reader's existing mental model to this one

**What it is.** Before any new vocabulary is introduced, the reader's prior
model — whichever one they actually arrive holding — is named and its specific
failure mode is walked through in terms they already have, the way a compiler
error is more useful when it names what you tried to do rather than only what
went wrong. The reader experiences the new pattern as a resolution to a
tension they now recognize, not as an unmotivated vocabulary drop.

**Evidence.** The aggregate-boundary research (angle 03) found this to be the
converged teaching move across every comparable project (dcb.events,
Disintegrate, Marten, EventStoreDB's own contrast-by-absence) — but also found
that happenstance's situation is the *inverse* of theirs: it was never built on
aggregates, so it has no existing pain to relieve, and no prior project answers
which mental model this crate's actual reader shows up holding. The same
research independently names three specific, previously-documented stall points
(mistaking "dynamic" for "no structure," not knowing whether this is a modeling
technique or a routing technique, no settled replacement noun for what an
Aggregate used to name) that are testable, not merely plausible. The prior-art
research (angle 01) supplies the theoretical vocabulary for why this matters
more here than for a simple tool: Hillel Wayne's "conceptual overview" gap in
Diátaxis is specifically about dense, unfamiliar conceptual models — which is
this crate's situation, not a simple-tool situation.

**Gap it closes.** The seed's application-author persona: "the teaching starts
from their problem — the invariant that spans two entities — and not from this
library's architecture." Also closes a gap none of the surveyed prior art has
closed for itself: which prior intuition (aggregate, stream-per-entity, or
none) this library's docs are actually arguing against is currently an open
question every comparable project answered explicitly and happenstance has not.

---

## Opportunity 5 — Proof that a real, non-author reader got through it — and where they got stuck, on the record

**What it is.** A dated, inspectable record exists showing that someone who
did not write the documentation — and who does not carry this repository's
insider context — used it to actually do something, with their specific points
of confusion written down rather than summarized into a vibe. The organization
gets to see, concretely, where teaching failed rather than trusting that it
didn't.

**Evidence.** The seed names this as one of exactly two proof artefacts and
states the standard explicitly: "an artefact that would not exist if the
documentation were adequate." The comprehension-as-evidence research (angle 04)
grounds this heavily: Google's friction-log method is a published, concrete
template (named scenario, chronological action log, concurrent reactions,
inline severity marks) precisely so the artefact is auditable rather than a
diary; the same research states non-authorship is a stated methodological
requirement, not a courtesy — insiders "unconsciously route around known rough
spots" — and that a single qualitative session already surfaces roughly a
third of real problems, which legitimizes treating one dated log as sufficient
evidence rather than an ongoing program. The precedent already exists in this
workspace: `examples/outside-projection-adapter/`, a real artefact built from
rendered documentation alone by someone facing the orphan rule and dependency
graph exactly as a stranger would.

**Gap it closes.** The seed's own naming of the reader nobody has watched:
"no phase puts the API in front of a user who is not its author before it
freezes." This is the instrument that would have caught the `Tags::empty()`
problem before publish, not after — the fuse the seed says is already fixed.

---

## Opportunity 6 — A path for building an adapter that is walked, not just referenced

**What it is.** An adapter author's experience moves from "here is a recipe
and a suite that tells you when you're done" to "here is what it feels like to
build one, told by someone who did it, with the moments that are surprising
named before they're hit." Reference and how-to survive; a narrative account
of the actual journey — including where it's easy to get the shape wrong — sits
alongside them.

**Evidence.** The seed states plainly that this persona is, today, "better
served than anyone" — a four-step recipe, a 314-line constitution atom, a
conformance suite — and yet: "nobody has been walked through building one, and
there is no third-party adapter to read." That's a gap named by absence rather
than by a measured incident, which is why this opportunity sits mid-list rather
than at the top: it is real, but the evidence is a documented absence, not an
observed failure the way Opportunities 1, 2, and 5 have.

**Gap it closes.** The seed's adapter-author persona getting reference and
how-to but no narrative — the one persona of the three the seed names whose
documentation type-gap is named explicitly rather than inferred.

---

## Opportunity 7 — A page that survives the evaluator's second question

**What it is.** The twenty-minute reader who is deciding whether to depend on
this library gets somewhere to go after the README's first, well-answered
question — a next click that exists, is findable without being told about it,
and answers what they actually ask next rather than trailing off.

**Evidence.** The seed states this persona's exact failure mode: "the README
is written for them and is good; what it cannot do is survive the second
question, because there is nowhere for that question to go." The prior-art
research (angle 01) independently confirms that findability/navigation is a
structural need distinct from any of the standard four content-need
categories — meaning this should be treated as its own concern, not folded
silently into whichever page happens to be nearest.

**Gap it closes.** The evaluator persona's stated failure point precisely, and
guards against a specific mis-step the research flags: a landing or index page
built without recognizing it answers a distinct need can get absorbed into one
of the four standard categories and picked up later as a "page answering a
second need" defect under Opportunity 3's own discipline.

---

## Opportunity 8 — The one place in the DCB ecosystem that draws the new picture, not just the old one

**What it is.** A reader trying to picture the write-side cycle this library
is built around — state a query, read tagged events, compute an append
condition, append under that condition — gets an actual picture of *that*
cycle, not only a picture of the aggregate model being retired.

**Evidence.** The aggregate-boundary research (angle 03) found this gap by
name in the ecosystem's own flagship resource: dcb.events itself diagrams the
aggregate pattern it is retiring but has no equivalent diagram for the pattern
it introduces. The interaction-pattern research (angle 05) confirms no diagram
vocabulary for this shift has "won" even within the DCB community — the closest
prior art is a third-party, independently maintained notation, not anything
shipped by the specification's own site. This is a real, corroborated gap, but
confidence on *this specific crate needing to be the one that fills it* is
lower than Opportunities 1–5: the workspace's own diagram count today is zero,
so this would be new territory rather than a fix to something already
half-built, and no evidence establishes this crate's readers are stalling on
the *absence of a picture* specifically rather than on the conceptual bridge
problem in Opportunity 4 more broadly.

**Gap it closes.** A gap the DCB ecosystem carries collectively, not one
specific to happenstance — closing it here would be filling a hole nobody in
this space has filled yet, which is a real opportunity but a genuinely
optional one relative to the seed's stated constraints.

---

## Opportunity 9 — A written, citable answer to "how do we write a page that teaches," reusable past this one crate

**What it is.** The judgment calls this initiative is about to make repeatedly
— what belongs on a page, how much a reader needs before a term is safe to use,
what a worked example must demonstrate to count as teaching rather than
decorating — get written down once, durably, so the next page (and the next
initiative that writes documentation) inherits the discipline instead of
re-deriving it.

**Evidence.** The seed states the gap in the strongest terms available in this
workspace's own vocabulary: across every existing atom of house style, "there
is no rule about narrative structure, audience, worked-example design or
diagrams," and of the workspace's full decision history, none concerns
documentation. The functional-alignment grounding confirms independently that
the durable-knowledge layer has no home for this subject today — no domain, no
atom — so this isn't a documentation-only gap, it's a knowledge-base gap this
initiative is positioned to notice first.

**Gap it closes.** The seed's meta-observation that the strongest explanations
this project has already written (the DCB argument, the worked example's own
module doc, the `MemoryEventStore` walkthrough) exist, are good, and are simply
unfindable or unrepeatable as a discipline — the difference between one team
getting one page right once, and a standard other pages (and other initiatives)
can be held to.

---

## Opportunity 10 (lower confidence, flagged for planning) — Meeting the reader mid-error, not just mid-page

**What it is.** The moment a reader hits a specific, predictable point of
confusion while actually using the crate — not while reading about it — that
confusion is anticipated and answered close to where it happens, not only in a
document three clicks away that assumes the reader already knows to look.

**Evidence.** The seed's own account of `E0034` is the clearest single
instance: the error a new user is most likely to hit is documented — six times,
across the runbook, an ADR, and evaluation documents — everywhere except in the
one file (`store.rs`) the reader is actually looking at when it happens. This is
a strong, specific piece of evidence, but it's a single instance rather than a
pattern with multiple corroborating cases the way Opportunities 1, 2, and 5 are,
which is why it's ranked here rather than higher — it reads as a real,
fixable, narrow gap rather than a structural opportunity the size of the others.

**Gap it closes.** The gap between documentation existing somewhere in the
repository and documentation existing where a confused reader's eyes actually
are at the moment of confusion.

---

## Archetypes this idea unlocks

Reader-facing patterns, not implementations. Each recurs across more than one
opportunity above and is worth naming as a repeatable shape rather than a
one-off fix.

- **The staged first encounter.** A reader's opening experience is deliberately
  minimal before it is deliberately complete — the opposite of "show everything
  the API can do in the first program." (Opportunity 1)
- **The checked claim.** A page's prose and a page's code are held to the same
  standard the crate's own error handling already is: a claim that isn't
  checked is treated as a claim that isn't made. (Opportunity 2)
- **The named destination.** Every page is reachable because a reader can name
  the question that leads to it, and finding out you're on the wrong page is
  itself informative rather than confusing. (Opportunities 3, 7)
- **The bridge from where the reader already stands.** New vocabulary is
  introduced only after the reader's current model and its specific limit have
  been named in their own terms. (Opportunity 4)
- **The witnessed reader.** At least one real person who isn't the author is
  known, by name and by date, to have gotten through the material — and what
  stopped them is preserved rather than smoothed over. (Opportunity 5)
- **The walked path.** A journey a reader can follow start to finish exists
  alongside the reference material that only makes sense to someone who already
  knows the destination. (Opportunity 6)
- **The durable discipline.** A judgment call made once, well, is written down
  so it doesn't have to be re-argued the next time someone writes a page.
  (Opportunity 9)

---

## Risks

- **Aggregate-to-boundary teaching has no settled diagram vocabulary anywhere
  in the DCB ecosystem** — not even at the specification's own site. Any
  attempt to illustrate the write-side cycle is closer to original work than to
  adopting a convention, and that effort is easy to underestimate.
- **The two named proof artefacts test different things and neither
  substitutes for the other.** A compiled-and-green page proves the code still
  matches the API; it says nothing about whether the surrounding prose still
  teaches the right lesson. Treating a green gate as evidence of teachability
  would quietly recreate the exact failure the seed opens with.
- **A single friction-log session is real evidence, not exhaustive evidence.**
  The literature supports "real stumbles were captured and are traceable" as a
  defensible claim from one dated session; it does not support "every
  comprehension failure was found." Overclaiming from one session risks a
  credibility gap of its own.
- **Recruiting a genuinely non-author reader is harder than it sounds inside
  this repository.** The evidence is specific that *insider knowledge*, not
  merely non-authorship, is what lets a reader route around rough spots — so a
  logger who has read the ADRs or the evaluation corpus may not produce a
  trustworthy log even if they didn't write the page.
- **The DCB vocabulary choice itself may be an unmeasured source of reader
  stall.** Tags, AppendCondition, and SequencePosition diverge from every
  comparable project's own naming; whether that divergence helps or hurts a
  new reader is untested, and no source surveyed settles it either way.
- **Two initiatives could independently invent the audience this documentation
  is written for.** A persona-staging story already exists elsewhere,
  unrun, on a branch not visible from this one. Documentation decisions made
  against a self-invented audience model risk being redone.
- **Filling the ecosystem-wide diagram gap (Opportunity 8) is attractive but
  optional relative to the seed's actual constraints** — worth flagging so
  planning doesn't let the most visually interesting opportunity crowd out the
  higher-confidence, higher-value ones above it.

## Open questions for the planning team

- Which of the ten opportunities above are in scope for this initiative's first
  pass, and which are worth naming now but deliberately deferred — especially
  Opportunity 8 (ecosystem diagram gap), whose value is real but whose
  connection to this crate's specific readers is the least directly evidenced?
- Does the conceptual bridge (Opportunity 4) commit to one prior mental model to
  argue against, multiple, or an explicit statement that none is assumed —
  given that every comparable project made this choice explicitly and
  happenstance currently has not?
- How does this initiative's need for an audience model relate to the
  in-flight, not-yet-run persona-staging story on the sibling branch — consume
  its output once it lands, proceed independently and reconcile later, or
  something else? (Named as a real dependency in both the intake brief and the
  grounding, not a detail.)
- Is a single non-author friction-log session (Opportunity 5) sufficient for
  this initiative's proof standard, or does the desired outcome call for more
  than one session, and against which of the three named personas first?
- Where does Opportunity 9 (a durable, citable documentation discipline) live
  once someone has to decide — not just note as open — where transferable
  practice like this belongs relative to the existing durable-knowledge layers?
- Does "the first fifteen minutes" (Opportunity 1) and "the adapter path"
  (Opportunity 6) belong to the same initiative pass, or does the adapter-author
  journey wait for evidence that the application-author path lands first?
