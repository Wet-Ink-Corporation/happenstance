---
title: Interaction-pattern dossier — From Accurate to Teachable
kind: distillation/interaction-patterns
item: HS-I0007
slug: docs-that-teach
status: draft-for-planning
promoted: false
sources:
  - .bklg/docs-that-teach/_discovery/research/01-rust-narrative-docs-prior-art-and-page-need-taxonomy-how-tok.md
  - .bklg/docs-that-teach/_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md
  - .bklg/docs-that-teach/_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md
  - .bklg/docs-that-teach/_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md
  - .bklg/docs-that-teach/_discovery/research/05-interaction-pattern-prior-art.md
---

# Interaction-pattern dossier

Design evidence for the surfaces this initiative will eventually build:
rustdoc reference pages and a narrative book (medium not yet chosen past
"the ecosystem's own," per the intake brief). This dossier names every
interaction pattern the research surfaced — what it is, where it is proven,
its documented failure modes, and whether this problem's shape fits it —
without choosing among them. Problem-space only: no pattern below is
recommended for adoption; several are named specifically because the
evidence rules them out or narrows their safe use to almost nothing.

**This problem's shape**, for calibrating fit conditions below, as stated
across the source material: three named audiences (application authors,
adapter authors, a twenty-minute evaluator); one core mental-model shift to
teach (aggregate/stream intuition → dynamic consistency boundary); six-plus
adapter skeletons that could each want adapter-scoped content; five
`happenstance-core` modules in the 300–1000 line range; dozens of public
items reachable by rustdoc search; zero diagrams and zero narrative pages
in the workspace today; and a gate (`cargo xtask ci`) that already treats
"printed a warning and exited 0" as the failure mode to design against
everywhere else in the repository.

## Candidate patterns

### The ecosystem's built-in surface: rustdoc search, `doc(alias)`, intra-doc links, item collapse/expand

**What it is.** Interaction rustdoc ships without any added tooling: a
keyboard-driven, type-signature-aware search; `#[doc(alias = "...")]` for
synonym findability; intra-doc links with automatic disambiguation; and a
`[+]`/`[-]` collapse/expand affordance on every item, togglable page-wide
with the `+`/`-` keys.

**Proven in the wild.** Built and documented by the Rust project itself
(rustdoc's own "How to read rustdoc" and "Advanced features" pages); every
crate on docs.rs inherits it for free.

**Documented failure modes.** Under-discovered even by people who value it
once they find it — the Rust project's own 2024H2 goals document names
rustdoc search's low discoverability as a problem worth a dedicated
initiative, i.e. the tool works but readers don't know to reach for it.

**Fit conditions.** Strong fit at this problem's actual fanout: dozens of
items across five 300–1000-line modules is exactly the scale type-driven
search and `doc(alias)` are built for, and it costs nothing per adapter
added. Weak fit for the evaluator persona's named gap — "good until the
second question, and then nowhere to go" is a linking problem (nothing
points from narrative prose into the reference item that answers the next
question), which intra-doc links solve directly but only if something
authors the link; the tool doesn't supply the link on its own.

### mdBook's built-in Playground run button

**What it is.** A "run" button on Rust code fences that sends the block to
the Rust Playground and renders output inline; opt-out per block
(`noplayground`) or globally.

**Proven in the wild.** Core mdBook, not a preprocessor — ships with every
mdBook site including tokio's, serde's (where used), and diesel's.

**Documented failure modes.** None sourced specifically; the caveat is
scope, not defect — the Playground compiles the block standalone and has no
access to the workspace's own crates, so it fits toy snippets, not an
example that exercises `happenstance-core` or an adapter crate.

**Fit conditions.** Fits an isolated concept-illustration block with no
crate dependency. Poor fit for any example this initiative's proof artefact
cares about, because those examples are precisely the ones that must
compile against the real crate — the Playground button demonstrates a
different, unconnected copy of the code, not the one the gate checked.

### Two-surface split: reference at docs.rs, narrative in a separate book, one explicit pointer between them

**What it is.** The reference documentation (rustdoc/docs.rs) and the
narrative documentation (a book) live as two separate surfaces, with the
reference's own crate-root doc containing a one-line pointer outward rather
than attempting to teach.

**Proven in the wild.** The converged norm among the four largest Rust
crates surveyed. tokio's crate root states verbatim "Guide level
documentation is found on the website"; serde's points to serde.rs; diesel's
goes further and tells a first-time reader the reference is the wrong place
to start ("we recommend you start with the getting started guide").

**Documented failure modes.** axum is the counter-case: no book, just
docs.rs plus a flat, unordered `examples/` folder whose own README states no
ordering and no audience. A contributor trying to backfill a book from those
examples hit exactly "I was not sure in which order to try to read them" —
the failure mode is not a broken UI, it's the absence of the split, and the
cost lands on a newcomer with no ordering signal.

**Fit conditions.** Fits a workspace with one crate the reader installs
(`happenstance`) and a genuinely separate concept to teach (DCB) that
reference-comment prose is documented to be a poor place for. Poor fit if
the workspace cannot commit to hosting a second surface at all — the
pattern's entire value is the split existing, not either half alone.

### Diátaxis's four-category taxonomy (tutorial / how-to / reference / explanation) as literal top-level structure

**What it is.** A structural pattern for organizing an entire documentation
tree into four named quadrants, keyed to two axes (action vs. cognition,
study vs. work), claimed by its own authors to be a complete enumeration —
"no other territory to cover."

**Proven in the wild.** Widely adopted; Cloudflare and Gatsby's docs teams
are cited on Diátaxis's own site using it as a literal per-page decision
procedure ("When we weren't sure where a new piece of content should fit
in, we'd consult the framework").

**Documented failure modes.** Its sharpest critique targets exactly this
problem's shape: built for tools with simple conceptual models, and
documented to strain on dense, interrelated concepts — "language concept
maps are just too dense to separate" — with a named fifth need
("conceptual overview") the four boxes don't cover. A second critique notes
the taxonomy is silent on findability/landing-page structure entirely. Even
the framework's own site has an open issue from its users noting the site
itself doesn't follow the structure it prescribes, and a sympathizer
concedes the four categories "feel far too vague" applied literally.
Diátaxis's own "complex hierarchies" page concedes documentation need not
divide into exactly four sections.

**Fit conditions.** Poor fit as literal structure for this problem: the
subject (DCB, a consistency-boundary shift a reader must understand, not a
simple tool to operate) sits closer to the dense-concept end its own critics
name as the strain case, and this initiative's three audiences don't sort
cleanly into the four quadrants. The "one need per page" discipline
underneath it is a separate, better-evidenced claim (see Anti-patterns) that
does not require adopting the four-box enumeration to keep.

### Staged (worked-example) disclosure — "training wheels"

**What it is.** A single running example that starts with the smallest
program that compiles and does something real, then adds one concept per
step in a fixed sequence the reader is expected to follow to the end.
Distinct from hierarchical progressive disclosure (below): staged
disclosure is linear and sequence-dependent, not a menu of optional depth.

**Proven in the wild.** Carroll's controlled "training wheels" studies:
users initially constrained to a core feature set built better mental
models than users given full access from the start, measured *after* the
constraint was lifted — a replicated finding, not a style preference.
dcb.events' own canonical worked example is staged this way (course
creation → capacity change → the multi-entity subscription that is the
actual point), and it is the same course-subscription scenario this
workspace's own example already uses.

**Documented failure modes.** Named directly in the literature: staged
disclosure "is problematic when the steps are interdependent and users must
alternate between them" — a reader who arrives mid-sequence via search (not
by reading start-to-end) can land past the setup with no signal they missed
it.

**Fit conditions.** Strong fit for exactly the artefact the evidence base
names as broken: an opening example with five-to-eight imports and a
hand-written multi-method trait implementation before any concept is named
is the shape the training-wheels research predicts underperforms. Poor fit
for reference pages reached out of sequence — at this problem's fanout
(dozens of items, one hop from search) a reader who searched for one method
signature is not going to walk a staged sequence to find it.

### Progressive disclosure (hierarchical)

**What it is.** Secondary/advanced content hidden behind an explicit
control (a "see also," an expandable section) so that most readers never
leave the primary, simple path; distinct from staged disclosure in that
it's optional depth, not a required sequence.

**Proven in the wild.** NN/g's foundational treatment, contrasted directly
against staged disclosure in the same source.

**Documented failure modes.** Conflating it with staged disclosure is named
as a documented usability cost, not a style choice — the two have different
navigation models and applying the wrong one to a given page (e.g. treating
a reference page as if a reader must walk it start to end) actively
degrades the experience.

**Fit conditions.** Fits a reference page that shows the minimal call and
defers configuration options behind a link. Poor fit as a replacement for
staged disclosure on the opening worked example — the evidence distinguishes
the two by a different failure mode each is documented to cause when used
in the other's place.

### Tabs (adapter-scoped or feature-scoped content)

**What it is.** A single page with mutually exclusive content panels — one
per adapter, one per feature flag — with only one panel visible at a time.
`mdbook-tabs` is real, maintained tooling for this in the mdBook ecosystem.

**Proven in the wild.** `mdbook-tabs` exists as documented, installable
tooling; language/platform-switcher tabs are a common form in
multi-target SDK documentation generally.

**Documented failure modes.** The W3C APG and WebAIM agree the full
tablist/tab/tabpanel ARIA triad plus roving `tabindex` is required for
keyboard and screen-reader access and is the part most implementations skip;
a 2024 accessibility guide catalogs six recurring anti-patterns including
inactive panels hidden with CSS opacity that stay in the accessibility tree
and get announced anyway, and calls tabs "one of the most-copied and
most-broken UI patterns on the web." Separately and specific to this
problem: nothing in `mdbook-tabs`' own documentation states whether an
inactive panel's code sample is included in `mdbook test`, the page's search
index, or Ctrl-F/print — an unverified property, not a confirmed safe one.

**Fit conditions.** The fanout is genuinely in tabs' comfortable range —
six-plus adapters, short labels (crate/adapter names) — which is why this
pattern is tempting rather than obviously wrong. But whether an inactive
panel's content is exercised by anything is, per the research, unconfirmed
and this workspace has already shipped one gate step that looked wired and
wasn't (a documentation step that printed warnings and exited 0 because
nothing read its output). Fit is conditional, not established — see Open
design tension 1.

### Collapsible admonitions / accordions

**What it is.** Callout boxes with a collapsed-by-default state, or
accordion-style expand/collapse sections, used to keep "advanced" or "aside"
content out of the main reading path. `mdbook-admonish` is real, installable
tooling for this.

**Proven in the wild.** Widely used for exercise answers and edge-case
detail in technical books and course materials.

**Documented failure modes.** NN/g names the failure mode directly and by
name, not as a general caution: never hide essential information behind an
accordion fold, because "valuable content that is hidden under an accordion
may be missed altogether." A companion piece adds a cost/benefit test:
accordions help only when most readers need only a few sections, and are
actively harmful when most readers need most of the content on a page.

**Fit conditions.** This is the single sharpest match in the whole dossier
to a hazard already realized in this repository: a reviewer added a sixth
event type, updated a fold, and forgot the query it should have stayed in
sync with — an invariant stated once visibly and once behind a fold, and
nothing caught the drift. A page that states a constraint once in prose and
once inside a collapsed admonition reproduces that exact shape at the
documentation layer. Fit is narrow: safe for a worked-out exercise answer,
unsafe for anything that states an invariant, a `MUST`, or a constraint a
reader needs to hold the whole page's argument together.

### Diff-highlighted before/after code

**What it is.** Two versions of the same requirement — the aggregate-shaped
implementation and the DCB-shaped one — rendered with line-level
added/removed markup, side-by-side or unified.

**Proven in the wild.** Not native to rustdoc or mdBook. Swift-DocC's own
community has an open, unresolved forum pitch for exactly this feature,
citing the same problem this initiative would face; presentation-layer
tools (code-surfer, refrakt) implement the visual pattern for slide decks
and static sites respectively, confirming the pattern is well-understood
even where Rust's own toolchain hasn't adopted it.

**Documented failure modes.** None specific to the visual pattern itself —
the failure mode is upstream, in what the "wrong" side of the diff actually
is. A rendered diff either compiles both sides against the real crate
(meaning a second, deliberately worse implementation has to be built and
maintained as compiling code forever) or is illustrative prose wearing
code's syntax highlighting — the same shape this repository's own testkit
doctests were already found to have taken (an "elaborate `ignore` fence").

**Fit conditions.** High conceptual fit — this is precisely the
aggregate-to-boundary shift the evidence says the workspace currently has no
diagram or comparison vocabulary for. But the pattern's honesty depends
entirely on whether both sides are gate-checked, which is unresolved — see
Open design tension 5.

### Diagram vocabulary: swimlane / event-model DSL for the write-side query/append cycle

**What it is.** A domain-specific diagram — not a generic flowchart —
showing commands, the events they read for consistency, and the events they
produce, in a swimlane layout that makes the DCB query itself (which events
a decision depends on) a first-class visual element.

**Proven in the wild.** An independent, open-source Mermaid-based DSL
already renders exactly this for DCB, with commands carrying explicit
`reads [...]` clauses against event lanes. A separate, independent teaching
project (`cartshop`, explicitly self-described as "a teaching example")
built a swimlane diagram with two edge colors carrying the entire
consistency story — orange for a write that must consult a DCB view first,
blue for ordinary fan-out with no consistency stake — with an explicit
top-to-bottom reading order stated in the README.

**Documented failure modes.** dcb.events itself, the canonical reference
site for this pattern, diagrams the concept it is retiring (three captioned
box-and-line Aggregate-root diagrams) but has no equivalent diagram for the
concept it is introducing — the query/append-condition cycle is described
only in prose and pseudocode. Even dcb.events' own scenario-visualization
tooling is labeled by the site itself as "unofficial, work-in-progress." No
diagram vocabulary for this specific shift has won even within the DCB
community. The general risk any diagram carries: it is source text a gate
can render but not typecheck against the real `Tags`/`Query` types, so a
diagram can drift from the API the same way prose can, with nothing to
catch it.

**Fit conditions.** Fanout is low here (one core mental-model shift, not
many diagrams needed), which favors investing in getting one diagram right
over adopting a general-purpose diagramming tool. The workspace has zero
diagrams today, so any adoption is green-field, not a migration — and the
field's own reference implementation shows the trap: diagramming only the
retiring model (contrast) while leaving the arriving model (the mechanism
itself) undiagrammed reproduces the exact gap dcb.events has today.

### Numbered-list / mapping-table narration in place of a diagram

**What it is.** A fixed, ordered list of the write-cycle's steps ("1. Tag
events. 2. Query events across streams by those tags. 3. Aggregate tagged
events into a view. 4. Enforce consistency at save time.") or a table
mapping old vocabulary to new, used as prose's own substitute for a
sequence diagram.

**Proven in the wild.** A recurring device across the corpus, not a
one-off: dcb.events uses it for its own write cycle, Marten's design docs
use both the numbered list and a spec-concept-to-implementation mapping
table, and a third independent source (a Chronicle event-store writeup)
uses the same device for its constraint/scope model.

**Documented failure modes.** None named specifically; it is evidently a
working, low-effort alternative precisely where a diagram would otherwise
be needed and isn't available.

**Fit conditions.** Fits the same low-fanout, one-core-mechanism shape as
the diagram pattern above, and requires no new tooling, no gate surface,
and no accessibility burden — the trade is a diagram's spatial legibility
for text's zero build cost. Multiple independent sources reaching for the
same device on the same kind of content (the write-cycle mechanism) is
convergent evidence it is a real, working option, not merely a fallback.

### Quizzes / inline comprehension checks

**What it is.** Inline, gate-buildable multiple-choice/short-answer/tracing
questions embedded in a book page. `mdbook-quiz` is real, maintained tooling
built specifically for Rust books.

**Proven in the wild.** `mdbook-quiz`'s own example usage targets an
mdBook, with configurable answer caching and full-screen modes.

**Documented failure modes.** None sourced independently for the UI itself;
the failure mode named is what it measures. The tool checks whether a
reader can answer a question about the material, not whether they can use
the API — a narrower, easier-to-game proxy than this initiative's own
proof-artefact bar. A reader can answer correctly by pattern-matching
wording without having internalized the constraint being asked about — the
identical failure the evidence base already found in the reference-comment
example that satisfies every lint while teaching nothing.

**Fit conditions.** Weak fit as a substitute for the friction-log proof
artefact (an author-authored quiz inherits the author's own blind spots,
the exact bias the friction-log method exists to route around by requiring
a non-author). Not ruled out as a supplementary self-check once real
comprehension evidence exists by other means, but not evidence on its own.

### Breadcrumb / master-detail navigation

**What it is.** An explicit, always-visible trail from the current page
back to its parents (breadcrumb), or a persistent list-plus-detail split for
browsing many similar items (master-detail).

**Proven in the wild.** Decades-old, extensively documented patterns; NN/g
has recommended breadcrumbs "since 1995."

**Documented failure modes.** Breadcrumbs are explicitly not a substitute
for primary navigation and add no value on flat, one-to-two-level
hierarchies; on narrow viewports they wrap and crowd. Master-detail
guidance warns against use "when another navigation pattern is more
suitable... e.g. a side panel without master view" and against reproducing
the master content inside the detail area.

**Fit conditions.** Poor fit as a new widget in this medium: rustdoc
already renders a contextual navigation bar per item and mdBook already
renders a persistent sidebar TOC — both are, functionally, the
breadcrumb/master-detail job already done by the ecosystem's own medium.
The evaluator persona's actual named gap ("good until the second question,
and then nowhere to go") is evidence of a missing *link*, not a missing
*widget* — the fix the medium already supports is intra-doc/inter-page
linking, not a bespoke navigation component layered on top of tooling that
already provides the function.

## Anti-patterns

Moves the evidence shows produce a bad experience for this class of surface,
stated so each can be checked against a screen later:

- **A page answering more than one named need.** Independently evidenced by
  two separate traditions (Diátaxis's quality theory and Mark Baker's
  pre-Diátaxis "Every Page is Page One"), both diagnosing the same failure:
  a page that accumulates every discovered gap becomes "an open-ended sink"
  that stops serving any one reader well. Checkable: does this page have one
  stated purpose, or does it drift into explaining, instructing, and
  referencing all at once because no one drew the line.
- **A load-bearing constraint or invariant stated behind a fold, an
  inactive tab, or a collapsed accordion, with no visible-by-default
  counterpart.** NN/g's own rule, and it maps onto a failure this repository
  has already had at the code layer (an invariant updated in one visible
  place and left stale in a folded one). Checkable: if the collapsed
  section were deleted, would the page still teach the constraint
  correctly? If not, it should not be collapsed.
- **Hidden content whose branch is not inside the same compiled/tested
  surface as the rest of the page.** Tabs, accordions, and any other
  show/hide mechanism inherit this project's worst documented failure
  (a gate step that looked wired and printed warnings while exiting 0)
  unless the hidden branch is provably exercised by the same mechanism that
  checks everything else. Checkable: break the code inside the hidden
  branch on purpose — does the gate fail?
- **`ignore`-fenced or otherwise uncompiled code presented as if it were
  checked.** The evidence base is explicit this reproduces the `todo!()`-body
  shape the seed's own diagnosis names: a block that "type-checks against
  anything." Checkable: does every fenced code block on the page actually
  run under the compiled-prose mechanism, or does at least one silently opt
  out.
- **Diagramming the model being retired without ever diagramming the model
  being taught.** This is dcb.events' own documented gap, not a hypothetical
  one — its Aggregate-retiring diagrams exist; its DCB-introducing mechanism
  does not have an equivalent. Checkable: for any page that contrasts an old
  and a new model, is the new model's own mechanism given a diagram (or
  equivalent visual device), not only the old one being contrasted against.
- **A quiz, or any authored comprehension check, treated as evidence a
  reader understood the material.** The tool measures recall of the prose
  that wrote it, authored by the same person whose blind spots the real
  proof artefact exists to catch. Checkable: does a claim of "this was
  tested for comprehension" trace to a non-author reader's dated record, or
  to a self-authored check.
- **A bespoke navigation widget (breadcrumb, master-detail rail) added on
  top of a medium that already renders the equivalent for free.** Checkable:
  before adding a navigation affordance, confirm rustdoc's per-item nav bar
  or mdBook's sidebar TOC doesn't already perform the same function — if it
  does, the actual gap is almost certainly a missing link, not a missing
  widget.
- **Forcing content into a fixed number of top-level structural categories
  when the content doesn't sort cleanly.** Diátaxis's own maintainer calls
  this "horrible," and its own site is cited by its users as failing to
  follow its own structure. Checkable: does forcing a page into a category
  bucket produce an empty or near-empty bucket, or a page straining to be
  two things at once.
- **An opening or canonical example that front-loads full complexity before
  naming a single concept.** The training-wheels evidence is a controlled,
  replicated finding, not a style preference — full access from the start
  measurably produces worse mental models than initial constraint, even
  after the constraint is later lifted. Checkable: does the first example a
  reader meets require understanding every concept simultaneously, or can a
  reader get something real working with one concept at a time.
- **An illustrative "wrong" implementation (e.g., the aggregate side of a
  before/after comparison) presented as code without being compiled code.**
  Same shape as the `ignore`-fence anti-pattern above, specific to
  comparison content: prose wearing syntax highlighting is not what it looks
  like.

## Open design tensions

Named, not resolved. Each is a genuine unresolved choice the evidence
surfaces; each needs an owning project at decomposition and a resolution in
that project's design stage.

1. **Tabs for adapter-scoped content, and whether a hidden panel is ever
   checked.** `mdbook-tabs` is real tooling and a plausible answer to "which
   adapter's variant of this instruction am I reading," at a fanout (six-plus
   adapters) that is genuinely in tabs' comfortable range. But nothing in the
   plugin's documentation states whether an inactive tab's code sample is
   included in `mdbook test`, the page's search index, or Ctrl-F/print, and
   this workspace has already shipped one gate step that looked wired and
   wasn't. *Options*: adopt tabs and prove every panel is exercised by the
   gate as a precondition; reject tabs and accept either duplicated
   per-adapter pages or a single adapter-agnostic walkthrough with
   differences called out in visible prose; restrict tabs to genuinely
   non-normative content (e.g. install-command syntax) where an untested
   panel carries no teaching risk. *What would settle it*: an experiment
   that deliberately breaks the code inside a non-default tab and checks
   whether the gate catches it — the same falsification standard the
   initiative's own proof artefact already demands of every other page.

2. **Where the line sits between "aside" and "load-bearing" for collapsible
   content.** Collapsible admonitions are real, shipped tooling and a
   legitimate way to keep an exercise answer or a rare edge case out of the
   main reading path. NN/g's own rule — never hide essential information
   behind a fold — collides directly with this project's own precedent of a
   constraint that drifted out of sync once only one of its two statements
   stayed visible. *Options*: ban collapsible content for anything stating
   an invariant, constraint, or `MUST`, restricting it to exercises/asides
   only; allow it broadly and rely on review discipline; forbid the pattern
   in this corpus entirely. *What would settle it*: whichever documentation
   standard this initiative produces needs an explicit rule here, because
   "use good judgment" is the same non-answer that let the code-layer
   invariant drift in the first place.

3. **Whether Diátaxis's four-category taxonomy is adopted as literal
   structure, or only its "one need per page" discipline is kept.** The
   brief's own language ("a named answer to which need a page answers") is
   Diátaxis-shaped, but this project's three audiences don't sort cleanly
   into tutorial/how-to/reference/explanation, and Diátaxis's sharpest critic
   names this project's exact kind of subject — a dense, interrelated
   conceptual model, not a simple tool — as where the four-box split
   strains. *Options*: adopt the four categories as literal top-level
   structure; adopt only the underlying discipline (name the one need each
   page answers) without committing to exactly four named categories; design
   a taxonomy keyed to the three personas instead of to reader-intent
   quadrants. *What would settle it*: this is explicitly left open by the
   intake brief itself ("what the chapters are... is planning's to choose")
   — this tension is a primary input to that decision, not a call made here.

4. **Diagram vocabulary for the aggregate-to-boundary shift: adopt existing
   prior art, invent a narrower one, or sidestep diagrams via
   before/after code instead.** An open-source Mermaid DSL already renders
   DCB-style `reads [...]` dependencies as swimlanes; even dcb.events' own
   canonical example admits its own scenario-visualization tooling is
   unofficial and unfinished. *Options*: adopt the existing Mermaid
   event-model DSL as-is; design a narrower, project-specific diagram
   convention small enough to keep correct by hand; use diff-highlighted
   before/after code instead of a diagram entirely, trading the "who checks
   a diagram" problem for the "who checks the wrong side of a diff" problem
   named in tension 5. *What would settle it*: whichever choice is made, a
   diagram is prose the gate can render but not typecheck — the underlying
   tension (how much surface area to expose to that specific, permanent
   blind spot) does not go away regardless of which vocabulary is picked.

5. **Before/after code comparison: both sides real and gate-checked, or the
   "wrong" side illustrative and unchecked.** A diff-highlighted or
   side-by-side aggregate-vs-DCB example is the most direct way to show the
   mechanism the seed says the current opening example fails to demonstrate.
   But the "wrong" side is, by construction, a worse design — compiling it
   for real means maintaining a second, deliberately inferior implementation
   forever; not compiling it means it is prose wearing code's syntax
   highlighting, the same shape this repository's own testkit doctests were
   already found to have taken. *Options*: commit to a real, compiled,
   `publish = false` example crate whose only job is being the worse side of
   the comparison, checked by the same gate as everything else; accept an
   unchecked illustrative snippet for this one, explicitly named case, with
   a stated rule for why it's exempt; skip the code-diff pattern and make
   the comparison in prose only, citing `spec/SPECIFICATION.md` clauses
   rather than showing an alternative implementation. *What would settle
   it*: whether "checked or it doesn't ship" has an exception for
   deliberately-wrong reference code, and if so, on what basis the exception
   is drawn narrowly enough not to become a loophole.

6. **Which prior mental model the teaching anchors against — DDD-aggregate
   reader, stream-per-entity/EventStoreDB reader, or neither.** Every
   surveyed source teaches the aggregate-to-boundary shift by first
   re-narrating the reader's *existing* pain (their aggregates, their sagas,
   their duplicate events) before relieving it — but this workspace's own
   reader has no aggregates or per-entity streams to feel pain about, since
   happenstance is DCB-native from the start. EventStoreDB, the incumbent
   most readers will have prior exposure to, presents stream-per-entity as
   an unmarked default with zero contrast to anything else, meaning the most
   common prior a reader carries is "one stream per entity," not "aggregates
   are the old model." *Options*: anchor against the DDD-aggregate reader's
   intuition (the pattern every DCB-adjacent source in the survey uses);
   anchor against the stream-per-entity/EventStoreDB reader's intuition
   instead (the more statistically likely prior, per this survey); design
   content that doesn't anchor against a specific prior at all, teaching the
   mechanism on its own terms. (A reader with no event-sourcing exposure at
   all is out of scope per the intake brief's own non-goals and is not one
   of these options.) *What would settle it*: this is a scoping decision
   every other project in this space made explicitly and early; this
   research names it as unresolved here rather than settling it.

7. **Findability/landing-page structure: is it a need outside whatever page
   taxonomy gets chosen, and where does it live.** A critique independent of
   the four-category debate above notes Diátaxis's taxonomy classifies
   content but is silent on how a reader gets routed to the right document
   in the first place — an index or landing page is neither a tutorial,
   how-to, reference, nor explanation in the strict sense. This project's
   own evaluator persona names exactly this gap ("good until the second
   question, and then nowhere to go"). *Options*: treat findability/routing
   as a structural concern with its own dedicated page(s), separate from
   whatever taxonomy answers "which need does this page answer"; fold
   routing responsibility into whichever page each taxonomy category treats
   as the entry point; leave it to the two-surface split's own crate-root
   pointer (tension 8) to carry the entire findability burden. *What would
   settle it*: whichever page-need taxonomy tension 3 resolves to, a
   deliberate decision on whether a landing/index page is a first-class
   category of its own or an implicit byproduct of the others.

8. **Hosting shape: docs.rs reference plus a separate narrative book (the
   ecosystem-converged pattern), or some other split.** The two-surface
   pattern is the converged norm among the largest, closest-in-shape Rust
   crates surveyed (tokio, serde, diesel), each with a one-line pointer from
   the crate root outward — but the intake brief explicitly leaves "where
   each chapter lives, how the result is hosted" open, and axum's own gap
   (no book, an unordered flat examples folder, a contributor stalling on
   "what order do I read these in") is a real, cited cost of not doing the
   split at all. *Options*: adopt the two-surface pattern as converged
   ecosystem norm; host narrative content a different way within "the
   medium is the ecosystem's own" constraint; decline a separate narrative
   surface and accept the axum-shaped risk knowingly. *What would settle
   it*: this is named as open by the intake brief itself; this research
   supplies the convergence evidence and the counter-example's documented
   cost as input, not as a resolution.

## Risks and open questions for the planning team

- Every "hidden content" pattern in this dossier (tabs, accordions,
  collapsed admonitions) carries the same underlying risk, independent of
  which specific pattern gets chosen: this workspace has already shipped
  one gate step that looked wired and silently wasn't. Whichever project
  inherits tensions 1 and 2 should be required to demonstrate — not assert —
  that a hidden branch is exercised by the gate, using the same
  deliberately-broken-page falsification standard the initiative's proof
  artefact already commits to elsewhere.
- The diagram-vocabulary and before/after-code tensions (4 and 5) are
  coupled: choosing diff-highlighted code as the answer to one changes what
  the other tension is even asking. Whichever project owns the
  aggregate-to-boundary teaching surface should resolve them together, not
  as two independent design decisions.
- Tension 6 (which prior mental model to anchor against) is upstream of
  content decisions this dossier deliberately does not make (what the
  opening example says, what gets contrasted against what). It should be
  resolved before, not during, authoring of the staged-disclosure opening
  example, since the training-wheels pattern's own fit depends on knowing
  what concept is being staged in relative to what the reader already
  believes.
- No pattern surveyed substitutes for the initiative's own friction-log
  proof artefact. Quizzes are the closest tempting substitute and are
  explicitly ruled out by the evidence — they check recall of the prose
  that authored them, not the ability to act on the API. Planning should
  treat this as settled by the evidence, not reopen it as a design choice.
- The built-in, zero-cost surface (rustdoc search, `doc(alias)`, intra-doc
  links, collapse/expand, mdBook's Playground button) is under-used
  relative to what it already offers and carries none of the hidden-content
  risk above. Whichever project owns navigation/findability (tension 7)
  should treat exhausting this surface as a precondition before evaluating
  any third-party preprocessor, per the evidence that every added pattern
  competes with, rather than starts from, an empty canvas.
