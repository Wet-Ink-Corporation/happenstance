---
title: "interaction-pattern-prior-art — research for From Accurate to Teachable"
kind: research
initiative: docs-that-teach
summary: "Every interaction pattern available inside rustdoc+mdBook's own medium has a documented failure mode that lands exactly on this project's known hazards — hidden tabs untested by CI, collapsed accordions burying the constraint a reviewer forgot, and a four-quadrant taxonomy that doesn't fit a project whose audience is three technical personas, not one."
sources:
  - https://rust-lang.github.io/mdBook/format/mdbook.html
  - https://github.com/RustForWeb/mdbook-plugins
  - https://mdbook-plugins.rustforweb.org/tabs.html
  - https://github.com/tommilligan/mdbook-admonish/
  - https://github.com/badboy/mdbook-mermaid
  - https://github.com/cognitive-engineering-lab/mdbook-quiz
  - https://doc.rust-lang.org/rustdoc/how-to-read-rustdoc.html
  - https://doc.rust-lang.org/rustdoc/advanced-features.html
  - https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html
  - https://rust-lang.github.io/rust-project-goals/2024h2/rustdoc-search.html
  - https://www.nngroup.com/articles/progressive-disclosure/
  - https://www.nngroup.com/articles/training-wheels-user-interface/
  - https://www.nngroup.com/articles/accordions-on-desktop/
  - https://www.nngroup.com/articles/accordions-complex-content/
  - https://www.nngroup.com/articles/breadcrumbs/
  - https://www.w3.org/WAI/ARIA/apg/patterns/tabs/
  - https://webaim.org/techniques/tabs/
  - https://accessibility.build/guides/accessible-tabs
  - https://www.w3.org/WAI/ARIA/apg/patterns/breadcrumb/
  - https://developer.microsoft.com/en-us/azure-devops/components/master-detail
  - https://www.oracle.com/webfolder/ux/middleware/alta/patterns/MasterDetail.html
  - https://www.hillelwayne.com/post/problems-with-the-4doc-model/
  - https://newton.cx/~peter/2023/divio-documentation-system/
  - https://diataxis.fr/quality/
  - https://diataxis.fr/how-to-use-diataxis/
  - https://www.diataxis.fr/complex-hierarchies/
  - https://github.com/evildmp/diataxis-documentation-framework/issues/107
  - https://blog.sequinstream.com/we-fixed-our-documentation-with-the-diataxis-framework/
  - https://forums.swift.org/t/pitch-code-block-diffs-in-articles/62477
  - https://dcb.events/
  - https://dcb.events/examples/course-subscriptions/
  - https://dcb.events/topics/aggregates/
  - https://github.com/howarddierking/mermaid-event-model/blob/main/blueprint_dsl_dcb.md
---

# interaction-pattern-prior-art

## Findings

- The ecosystem's own medium is not interaction-neutral. rustdoc ships a real
  interaction system already — a keyboard-driven search with type-signature
  queries and item-kind filters, `doc(alias)` for synonym findability,
  intra-doc links with automatic disambiguation, and a collapse/expand
  affordance on every item (`[+]`/`[-]`, and `+`/`-` keys to do it for the
  whole page) — built and documented by the Rust project itself
  (https://doc.rust-lang.org/rustdoc/how-to-read-rustdoc.html,
  https://doc.rust-lang.org/rustdoc/advanced-features.html). Any pattern this
  initiative adds on top competes with, rather than starts from, an empty
  canvas.
- mdBook, the other half of the ecosystem's own medium, ships one runnable
  interaction out of the box — a "play" button that sends a Rust code block to
  the Rust Playground and renders its output inline — and it is opt-out per
  block (`noplayground`) or globally
  (https://rust-lang.github.io/mdBook/format/mdbook.html). Every other
  interaction pattern (tabs, admonitions, mermaid diagrams, quizzes) is a
  third-party preprocessor, not core mdBook, and each one adds a build-time
  dependency and a CSS/JS asset the book must vendor.
- Tabs are the most-documented broken pattern on the web, not a settled one.
  WebAIM and the W3C APG agree the tab/tabpanel/tablist ARIA triad plus roving
  `tabindex` is required and is exactly the part most implementations skip
  (https://webaim.org/techniques/tabs/,
  https://www.w3.org/WAI/ARIA/apg/patterns/tabs/); a 2024-vintage
  implementation guide calls tabs "one of the most-copied and most-broken UI
  patterns on the web" and catalogs the same six failure modes recurring
  across implementations (https://accessibility.build/guides/accessible-tabs).
  `mdbook-tabs` exists and is real prior art for showing adapter-specific
  variants of the same page (https://mdbook-plugins.rustforweb.org/tabs.html),
  but nothing in its documentation states whether content in an inactive tab
  is present in the page's search index, is included in `mdbook test`, or
  survives Ctrl-F / print — the three properties this project's gate already
  depends on for every other page.
- Accordions carry a *specific, named* failure mode that maps onto a hazard
  this project has already paid for once. NN/g's own research: "avoid hiding
  any crucial information within the collapsed panels — essential information
  should be presented outside of accordions to ensure it is readily available
  and not easily overlooked"
  (https://www.nngroup.com/articles/accordions-on-desktop/), and their earlier
  piece frames the same finding as a cost/benefit test — accordions are for
  when *most readers need only a few sections*, and are actively harmful when
  most readers need most of the content
  (https://www.nngroup.com/articles/accordions-complex-content/). The seed's
  own evidence is that a reviewer added a sixth event type, updated the fold,
  and forgot the query — an omission of exactly the kind an accordion is
  documented to produce when the missed section was collapsed by default.
- Progressive disclosure is not one pattern but two, and this project's
  problem shape needs to pick between them explicitly. NN/g's foundational
  article distinguishes *progressive disclosure* (hierarchical: hide secondary
  features behind a control, most users never leave the primary screen) from
  *staged disclosure* (linear: walk through a fixed sequence of steps, and
  users who start the sequence are expected to finish it)
  (https://www.nngroup.com/articles/progressive-disclosure/). A worked example
  that grows a `DomainEvent` impl feature by feature (the exact repair the
  seed wants for the 30-line, 8-import opening program) is staged disclosure;
  a reference page that shows the minimal call and defers configuration
  options behind a "See also" is progressive disclosure. Conflating them is a
  documented usability cost, not a style choice.
- "Training wheels" is a named, empirically supported pattern for exactly the
  seed's complaint about the opening example. Carroll's controlled studies
  found that users *initially limited* to a core feature set outperformed
  users given full access from the start, even after the limits were lifted —
  because the constrained interface forced a well-structured mental model
  before complexity arrived (https://www.nngroup.com/articles/training-wheels-user-interface/).
  This is direct, citable support for deferring `Tags`, multi-entity
  invariants, and the five-method `DomainEvent` impl out of the first program
  a reader runs — not a design opinion, a replicated finding.
- Diátaxis — the four-way tutorial/how-to/reference/explanation split the
  intake brief's language ("a named answer to which need does this page
  answer") is shaped like — has a documented class of misfit for exactly this
  project's kind of subject. Hillel Wayne's critique, grounded in Diátaxis's
  own admission that it was designed for *tools* with simple conceptual
  models: "if you look at the Python docs, the tutorial section needs to also
  explain and reference... 4doc is oriented around random-access, languages
  are taught in a sequence-of-lessons. The two don't mix well."
  (https://www.hillelwayne.com/post/problems-with-the-4doc-model/). happenstance
  is closer to that dense-concept-model end (a consistency-boundary shift a
  reader must *understand*, not just a function to *call*) than to Diátaxis's
  home turf. Diátaxis's own author agrees the four categories are not four
  mandatory directories and that forcing empty structure is "horrible"
  (https://diataxis.fr/how-to-use-diataxis/), and even the framework's own
  site has been criticized by its users for not following its own structure
  (https://github.com/evildmp/diataxis-documentation-framework/issues/107) —
  so citing Diátaxis for the *discipline* (one need per page) without
  importing its specific four-box taxonomy is a defensible, cited position,
  not an evasion.
- A first-party, in-language precedent for teaching the aggregate→DCB shift
  already exists and is not a UI pattern this project would be inventing: the
  DCB reference site itself teaches the shift with worked before/after code
  (a classical `CourseAggregate` next to a DCB-adjusted repository) and
  separately visualizes Given/When/Then scenarios with what it calls "an
  unofficial, work-in-progress" library
  (https://dcb.events/examples/event-sourced-aggregate/,
  https://dcb.events/examples/course-subscriptions/) — i.e. even the
  canonical DCB teaching source has not settled on a diagram pattern for this
  and says so. A second, independent instrument — Mermaid-based "event
  modeling" swimlane diagrams that render DCB-style commands with explicit
  `reads [...]` clauses — exists as an open-source DSL
  (https://github.com/howarddierking/mermaid-event-model/blob/main/blueprint_dsl_dcb.md),
  evidence that a swimlane/timeline diagram vocabulary for this exact
  aggregate-vs-boundary distinction is field-tested, not novel.
- Diff-highlighted before/after code has recent, first-party tooling backing
  in doc-generation ecosystems adjacent to Rust's: Swift-DocC's own forum
  pitch for code-block diffs states the problem this project has independently
  — "when writing instructional text, authors often need to show before/after
  states of small code blocks... two code blocks illustrating the state of
  code before and after the changes" is one of only three unsatisfying options
  without native diff support
  (https://forums.swift.org/t/pitch-code-block-diffs-in-articles/62477). No
  such feature exists in mdBook or rustdoc today; a diff view would be a
  bespoke preprocessor, which is the exact class of thing the brief's "medium
  is the ecosystem's own" constraint is wary of.
- Breadcrumb and master-detail patterns are answers to a navigation problem
  this project's medium already owns. NN/g's breadcrumb guidelines exist for
  sites where "users arrive on a deep page directly from an external link" and
  need wayfinding back up a hierarchy (https://www.nngroup.com/articles/breadcrumbs/);
  master-detail is for browsing a one-to-many list with drill-in
  (https://developer.microsoft.com/en-us/azure-devops/components/master-detail,
  https://www.oracle.com/webfolder/ux/middleware/alta/patterns/MasterDetail.html).
  rustdoc's own navigation bar already performs the breadcrumb function
  contextually per item, and mdBook's sidebar TOC already performs it for the
  book. A bespoke breadcrumb widget on top of either would duplicate
  navigation the medium already provides — the finding that supports it is
  named here because the evaluator persona's "second question" problem
  (README is good, has nowhere to go from there) is a real breadcrumb-shaped
  gap, but the fix rustdoc/mdBook already support is *linking*, not a new
  widget.

## Evidence & citations

| Claim | Source |
| --- | --- |
| rustdoc ships type-driven search, `doc(alias)`, intra-doc links, and item collapse/expand as built-in interaction | https://doc.rust-lang.org/rustdoc/how-to-read-rustdoc.html, https://doc.rust-lang.org/rustdoc/advanced-features.html, https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html |
| Rustdoc search is under-discovered even by people who value it once they find it | https://rust-lang.github.io/rust-project-goals/2024h2/rustdoc-search.html |
| mdBook's only built-in interaction is the Rust Playground run button, opt-out per block or globally | https://rust-lang.github.io/mdBook/format/mdbook.html |
| Tabs, admonitions, mermaid, quizzes are third-party mdBook preprocessors, not core | https://github.com/RustForWeb/mdbook-plugins, https://github.com/tommilligan/mdbook-admonish/, https://github.com/badboy/mdbook-mermaid, https://github.com/cognitive-engineering-lab/mdbook-quiz |
| `mdbook-tabs` exists as a real tabbed-content preprocessor for mdBook | https://mdbook-plugins.rustforweb.org/tabs.html |
| Accessible tabs require the full tablist/tab/tabpanel ARIA triad + roving tabindex; most implementations skip parts of it | https://www.w3.org/WAI/ARIA/apg/patterns/tabs/, https://webaim.org/techniques/tabs/, https://accessibility.build/guides/accessible-tabs |
| Accordions must never hide essential information behind the fold; this is a stated design rule, not a style preference | https://www.nngroup.com/articles/accordions-on-desktop/ |
| Accordions are actively harmful when most readers need most of the content, appropriate only when most readers need only a few sections | https://www.nngroup.com/articles/accordions-complex-content/ |
| Progressive disclosure (hierarchical) and staged disclosure (linear/sequential) are distinct patterns with different navigation models and different usability profiles | https://www.nngroup.com/articles/progressive-disclosure/ |
| "Training wheels" interfaces (initially limited feature set) produce better mental models than full-access-from-the-start, even measured after full access is granted | https://www.nngroup.com/articles/training-wheels-user-interface/ |
| Diátaxis was designed for tools with simple conceptual models and is documented to strain on languages/frameworks with dense, interrelated concepts | https://www.hillelwayne.com/post/problems-with-the-4doc-model/ |
| Diátaxis's own author: forcing content into four empty directories is "horrible"; the categories are a guide, not a mandatory structure | https://diataxis.fr/how-to-use-diataxis/ |
| Diátaxis cannot address "functional quality" (accuracy, compiled-ness) — only "deep quality" (fit, flow) — and is conditional on functional quality already existing | https://diataxis.fr/quality/ |
| Diátaxis in complex/polyhierarchical domains: "documentation should be as complex as it needs to be" — the four categories don't force a flat four-way top-level split | https://www.diataxis.fr/complex-hierarchies/ |
| Even the Diátaxis framework's own site has been criticized by users for not following the framework it describes | https://github.com/evildmp/diataxis-documentation-framework/issues/107 |
| A real team's account of applying Diátaxis: explanation is the section engineers over-write and should be cut most aggressively; how-to guides double as a product-design forcing function | https://blog.sequinstream.com/we-fixed-our-documentation-with-the-diataxis-framework/ |
| dcb.events itself teaches the aggregate→DCB shift with paired before/after worked code and admits its own scenario-visualization tooling is unofficial and work-in-progress | https://dcb.events/examples/event-sourced-aggregate/, https://dcb.events/examples/course-subscriptions/ |
| An independent Mermaid-based DSL renders DCB-style commands with explicit `reads [...]` event dependencies as swimlane diagrams | https://github.com/howarddierking/mermaid-event-model/blob/main/blueprint_dsl_dcb.md |
| Swift-DocC's own community has an open, unresolved feature request for diff-highlighted before/after code blocks, citing the same problem this project has | https://forums.swift.org/t/pitch-code-block-diffs-in-articles/62477 |
| Breadcrumbs are a wayfinding answer to a deep-link-entry problem; master-detail is for one-to-many list browsing with drill-in | https://www.nngroup.com/articles/breadcrumbs/, https://developer.microsoft.com/en-us/azure-devops/components/master-detail |

## Candidate patterns

### Staged (worked-example) disclosure — the training-wheels program

**What it is.** A single running example that starts with the smallest
program that compiles and does something real, then adds one concept per
step, in a fixed sequence the reader is expected to follow to the end. Distinct
from *progressive* disclosure (hierarchical, most readers never leave level
one) — see NN/g's own table contrasting the two
(https://www.nngroup.com/articles/progressive-disclosure/).

**Proven in the wild.** Carroll's training-wheels studies are the direct
empirical backing: users constrained to a core feature set initially built
*better* mental models, measured after the constraint was lifted, than users
given full access from the start (https://www.nngroup.com/articles/training-wheels-user-interface/).
dcb.events' own worked example is staged in exactly this way — course
creation, then capacity change, then the multi-entity subscription that is
the actual point (https://dcb.events/examples/course-subscriptions/).

**Documented failure modes.** NN/g names it directly: staged disclosure "is
problematic when the steps are interdependent and users must alternate
between them" — a linear sequence punishes a reader who needs to jump back
and forth, which is exactly what a reference-lookup reader (the evaluator
persona) does and a first-time reader (the application-author persona) does
not (https://www.nngroup.com/articles/progressive-disclosure/). A staged
walkthrough that a reader reaches by search (not by reading start-to-end) can
land them mid-sequence with no way to tell they missed the setup.

**Fit for this problem.** Strong fit for the opening crate-level example the
seed names as broken — five imports and a hand-written `DomainEvent` before a
concept is named is the anti-pattern the training-wheels research predicts
will underperform. Poor fit for `happenstance-core`'s reference pages, which
are reached out of sequence by a reader who already knows what they're
looking for (fanout: dozens of items across five 300–1000-line modules;
depth: one hop from search, not a sequence).

### Tabs (feature/adapter-scoped content)

**What it is.** A single page showing mutually-exclusive content panels (one
per adapter, one per feature flag) with only one visible at a time,
implemented via `mdbook-tabs` or hand-rolled JS/CSS
(https://mdbook-plugins.rustforweb.org/tabs.html).

**Proven in the wild.** Widely used in multi-language/multi-platform SDK docs
(language-switcher tabs are the most common form); `mdbook-tabs` is a
maintained, documented mdBook preprocessor with a real install path.

**Documented failure modes.** The W3C APG and WebAIM agree the tablist/tab/
tabpanel ARIA roles plus roving `tabindex` are required for keyboard and
screen-reader access, and that this is the part most implementations get
wrong (https://www.w3.org/WAI/ARIA/apg/patterns/tabs/,
https://webaim.org/techniques/tabs/). A 2024 accessibility guide catalogs six
recurring anti-patterns, from "no ARIA roles at all" to "inactive panels
hidden with CSS opacity" (content stays in the accessibility tree and gets
announced anyway) (https://accessibility.build/guides/accessible-tabs). None
of `mdbook-tabs`' own documentation states whether inactive-tab content is
included in `mdbook test`, the in-page search index, or a printed/exported
page — properties this project's gate already depends on everywhere else.

**Fit for this problem.** Genuinely tempting: this workspace has six adapter
skeletons plus a reference `MemoryEventStore`, and a reader configuring their
own store wants "the SQLite version of this paragraph." But fanout is real
(6+ adapters) and label length is short (crate names), which is tabs'
comfortable range — the risk is not the UI shape, it's whether content behind
an inactive tab is checked by anything. Given this project's own precedent —
a documentation step that printed warnings and exited 0 because nothing read
its output (`RUNBOOK.md`'s phase-0 log, cited in the seed) — an interaction
pattern whose untested branch is invisible by default is a bad match unless
CI is proven to walk every tab.

### Collapsible admonitions / accordions

**What it is.** Callout boxes (`mdbook-admonish`) with a collapsible variant,
or accordion-style expand/collapse sections, used to defer "advanced" or
"aside" content out of the main reading path
(https://github.com/tommilligan/mdbook-admonish/).

**Proven in the wild.** Widely used for exercise answers, edge-case detail,
and "if you're curious" asides in technical books; the Quarto course-template
convention explicitly reserves collapsed callouts for exercise answers and
hints, never for load-bearing content (evidence cited under Findings from the
`cambiotraining` Quarto template).

**Documented failure modes.** Named directly and specifically by NN/g: never
hide essential information behind an accordion fold, because discoverability
and access cost both degrade — "valuable content that is hidden under an
accordion may be missed altogether"
(https://www.nngroup.com/articles/accordions-on-desktop/). The complex-content
piece adds the harder test: accordions help only when most readers need only
a few sections; they actively hurt when most readers need most of the content
on a page (https://www.nngroup.com/articles/accordions-complex-content/).

**Fit for this problem.** This is the pattern with the sharpest match to a
hazard this project has already suffered. The seed's own evidence: a
reviewer added a sixth event type, updated the fold, and forgot the query —
an unsynced pair of "the consistency boundary, stated once here and once
there" that nothing checked. A page that states an invariant once in visible
prose and once inside a collapsed admonition recreates that exact shape at
the documentation layer: the reader who skips the fold has an incomplete
mental model and the page cannot tell them so. Safe use is narrow — hiding a
worked-out answer to an exercise, not hiding a constraint.

### Diff-highlighted before/after code

**What it is.** Two versions of a code block — the "wrong" (aggregate-shaped)
and "right" (DCB-shaped) implementation of the same requirement — rendered
with line-level added/removed markup, either side-by-side or unified.

**Proven in the wild.** Not native to rustdoc or mdBook. Swift-DocC's own
community identified the same need and has an open, unresolved forum pitch
for it, citing the exact three unsatisfying workarounds this project would
otherwise be stuck with — describe the delta in prose, show two full blocks
with no visual linking, or hand-roll `git diff` markers
(https://forums.swift.org/t/pitch-code-block-diffs-in-articles/62477).
Presentation-layer tools (code-surfer, refrakt) implement the visual pattern
for slide decks and static sites respectively, confirming the pattern itself
is well understood even though Rust's own toolchain hasn't adopted it.

**Documented failure modes.** None specific to the pattern's UI (it is
legible and well-precedented); the failure mode is upstream of the display —
Swift-DocC's own thread spends most of its length on the *encoding* problem
(how do you express "line 2 deleted, two lines added" in a way that's
unambiguous with Objective-C's own leading `+`/`-` method syntax). For this
project specifically: the "aggregate-shaped" side of the diff is, by
definition, a program this project's compiler will not typecheck against
`Tags::empty()` semantics without being obviously wrong — so a rendered diff
either compiles both sides against a real crate (meaning the "wrong" side
must itself be an actual, compiling, worse design — extra surface to
maintain) or is illustrative prose-as-code, which is the same "elaborate
`ignore` fence" shape the testkit's own doctests were already found to be.

**Fit for this problem.** High conceptual fit (this is precisely the
"aggregate → boundary" shift the seed says has no diagram vocabulary in the
workspace) but no native tooling in the medium, and the pattern's honesty
depends entirely on whether both sides of the diff are gate-checked or not —
which is the open design tension below.

### Diagram vocabulary (swimlane / event-model DSL)

**What it is.** A domain-specific diagram type — not a generic flowchart —
showing commands, the events they read for consistency, and the events they
produce, in a swimlane layout that makes the *DCB query* (which events this
decision depends on) a first-class visual element rather than implicit prose.

**Proven in the wild.** An independent, open-source Mermaid-based DSL already
implements exactly this for DCB specifically, rendering commands with
explicit `reads [...]` clauses against event lanes
(https://github.com/howarddierking/mermaid-event-model/blob/main/blueprint_dsl_dcb.md).
dcb.events itself, the canonical reference, visualizes its own worked example
with Given/When/Then scenarios via what it calls unofficial, work-in-progress
tooling (https://dcb.events/examples/course-subscriptions/) — evidence that
the *need* for a diagram vocabulary here is shared by the DCB community at
large, not particular to this workspace, and that no single vocabulary has
won yet.

**Documented failure modes.** None sourced specifically for this DSL (it is
early and small); the general risk with any diagram-as-documentation is the
same "drift" problem this whole initiative exists to solve for prose — a
mermaid diagram embedded via `mdbook-mermaid`
(https://github.com/badboy/mdbook-mermaid) is source text the gate can render
but cannot check for correctness against the actual `Tags`/`Query` types,
because nothing type-checks a diagram.

**Fit for this problem.** The workspace "currently contains zero diagrams"
per the seed — any adoption is a green-field decision, not a migration.
Fanout is low (one core mental-model shift to diagram, not many), which
favors investing in getting one diagram right over adopting a general-purpose
diagramming tool.

### Quizzes / comprehension checks

**What it is.** Inline, gate-buildable multiple-choice/short-answer/tracing
questions embedded in a book page, via `mdbook-quiz`
(https://github.com/cognitive-engineering-lab/mdbook-quiz).

**Proven in the wild.** Built specifically for Rust books (the tool's own
example usage targets an mdBook), with configurable answer caching and
full-screen modes.

**Documented failure modes.** None sourced independently, but the tool's own
scope statement matters here: it checks whether a reader can *answer a
question about the material*, not whether they can *use the API*. That is a
narrower and easier-to-game proxy than the friction-log proof artefact this
initiative's brief already commits to — a reader can answer a quiz correctly
by pattern-matching wording without having internalized the consistency
boundary the quiz is about, which is the same failure the seed found in
"Returns the head position. # Errors: Returns an error on failure." (a
sentence that satisfies every lint without teaching anything).

**Fit for this problem.** Weak fit as a *substitute* for the friction-log
proof artifact the brief already names (a quiz is authored by the same person
who wrote the prose it tests, so it inherits the author's blind spots); it is
not ruled out as a *supplementary* self-check once the real proof exists, but
it cannot be evidence of teaching on its own.

### Breadcrumb / master-detail navigation

**What it is.** An explicit, always-visible trail from the current page back
to its parents (breadcrumb) or a persistent list-plus-detail split for
browsing many similar items (master-detail) — standard patterns for
orientation in deep or wide hierarchies
(https://www.nngroup.com/articles/breadcrumbs/,
https://developer.microsoft.com/en-us/azure-devops/components/master-detail).

**Proven in the wild.** Both are decades-old, extensively documented web and
application patterns; NN/g has recommended breadcrumbs "since 1995."

**Documented failure modes.** Breadcrumbs are explicitly *not* a substitute
for primary navigation and add no value on flat, 1–2-level hierarchies
(https://www.nngroup.com/articles/breadcrumbs/); on mobile they wrap and
crowd. Master-detail's own guidance calls out that it should not be used
"when another navigation pattern is more suitable... e.g. a side panel
without master view" and warns against reproducing the master content inside
the detail area (https://developer.microsoft.com/en-us/azure-devops/components/master-detail).

**Fit for this problem.** Low fit as a *new* widget: rustdoc already renders
a contextual navigation bar per item and mdBook already renders a persistent
sidebar TOC — both are, functionally, breadcrumb/master-detail
implementations the ecosystem's own medium already ships. Building a second
one on top would violate the brief's "medium is the ecosystem's own, not a
bespoke site" constraint for a need the medium already meets. The real gap
the evaluator persona has — "good until the second question, and then
nowhere to go" — is a *linking* gap (nothing points from the README to the
deeper page), not a missing navigation widget.

## Open design tensions

1. **Tabs for adapter-specific content, and whether hidden panels are ever
   checked.** `mdbook-tabs` is real, documented tooling for showing
   sqlite/postgres/cloudflare/neon-specific variants of the same
   instruction — a plausible answer to "which adapter's docs am I reading."
   But nothing in the plugin's own documentation states whether an inactive
   tab's code sample is included in `mdbook test`, and this project has
   already been burned once by exactly this shape of blind spot (a docs step
   that printed warnings and exited 0 because nothing read its output).
   *Options*: (a) adopt tabs and prove in the gate that every panel is
   exercised — an experiment, not an assumption; (b) reject tabs and accept
   either duplicated pages per adapter or a single adapter-agnostic
   walkthrough with adapter differences called out in prose; (c) use tabs
   only for genuinely non-normative content (e.g. installation command
   syntax) where an untested panel carries no teaching risk. *What would
   settle it*: an experiment that deliberately breaks the code inside a
   non-default tab and checks whether `cargo xtask ci` (or `mdbook test`)
   catches it — the same falsification standard the brief's proof artefact
   already demands of every other page.

2. **Collapsed/accordion content, and where the line is between "aside" and
   "load-bearing."** Collapsible admonitions are real, shipped tooling
   (`mdbook-admonish`) and are a legitimate way to keep an exercise answer or
   a rare edge case out of the main reading path. But NN/g's own guidance —
   never hide essential information behind a fold — collides directly with
   this project's own precedent of an invariant that was stated in two
   places and drifted out of sync when only one was visible. *Options*: (a)
   ban collapsible content for anything that states an invariant, constraint,
   or `MUST`, restricting it to exercises/asides only; (b) allow it broadly
   and rely on review discipline; (c) forbid the pattern entirely in this
   corpus. *What would settle it*: whichever documentation standard this
   initiative produces (an open question already carried from the intake
   brief) needs a rule here specifically, because "use good judgment" is the
   same non-answer that let the query/fold invariant drift in code.

3. **Whether Diátaxis's four-way taxonomy is adopted as structure, or only
   its "one need per page" discipline is kept.** The brief's language — "a
   named answer to which need a page answers, so a page can be finished
   rather than merely added to" — is Diátaxis-shaped, but this project's
   audience (application authors, adapter authors, the evaluator) doesn't
   cleanly sort into tutorial/how-to/reference/explanation, and Diátaxis's
   own critics single out exactly this project's kind of subject — a dense,
   interrelated conceptual model, not a simple tool — as where the four-box
   split strains (https://www.hillelwayne.com/post/problems-with-the-4doc-model/).
   *Options*: (a) adopt the four Diátaxis categories as the literal top-level
   structure; (b) adopt only the underlying discipline (name the one need
   each page answers) without committing to exactly four named categories;
   (c) design a project-specific taxonomy keyed to the three personas
   instead of to reader-intent quadrants. *What would settle it*: this is
   explicitly named as undecided by the intake brief itself ("what the
   chapters are... is planning's to choose") — this tension is a primary
   input to that decision, not a call this research makes.

4. **Diagram vocabulary for the aggregate→boundary shift: adopt existing
   prior art, or invent one for this workspace.** An open-source Mermaid DSL
   already renders DCB-style `reads [...]` dependencies as swimlanes, and
   even dcb.events' own canonical example admits its scenario-visualization
   tooling is unofficial and unfinished. *Options*: (a) adopt the existing
   Mermaid event-model DSL as-is; (b) design a narrower, project-specific
   diagram convention (e.g. a fixed two-panel "reads / writes" box per
   worked example) that trades generality for being small enough to keep
   correct by hand; (c) use diff-highlighted before/after code instead of a
   diagram entirely, sidestepping the "who checks a diagram" problem by
   making the artifact code. *What would settle it*: whichever choice is
   made, the tension underneath does not go away — a diagram is prose the
   gate can render but not typecheck, so the choice is really about how much
   surface area to expose to that specific, permanent blind spot.

5. **Before/after code comparison for teaching the shift: both sides real
   and gate-checked, or the "wrong" side illustrative and unchecked.** A
   diff-highlighted or side-by-side aggregate-vs-DCB example is the most
   direct way to show "the mechanism that makes a dynamic consistency
   boundary dynamic" that the seed says the current opening example fails
   to show. But the "wrong" (aggregate) side is, by construction, a worse
   design — compiling it for real means maintaining a second, deliberately
   inferior implementation forever; not compiling it means it is prose
   wearing code's syntax highlighting, the same shape the testkit's own
   doctests were found to be before this initiative. *Options*: (a) commit
   to a real, compiled, `publish = false` "classical" example crate whose
   only job is being the worse side of the comparison, checked by the same
   gate as everything else; (b) accept an unchecked illustrative snippet for
   this one, explicitly-named case, with a rule for why it's exempt; (c) skip
   the code-diff pattern and make the comparison in prose only, citing
   `spec/SPECIFICATION.md` clauses rather than showing an alternative
   implementation. *What would settle it*: a decision on whether "checked or
   it doesn't ship" (the brief's own constraint) has an exception for
   deliberately-wrong reference code, and if so, on what basis it's drawn
   narrowly enough not to become a loophole.

## Implications for the idea

- The strongest, best-evidenced interaction move available to this
  initiative costs nothing to build: **reorder the existing worked example
  into staged disclosure** (training-wheels pattern) rather than adding any
  new UI. This is a content change, not a tooling change, and it has direct
  controlled-study backing
  (https://www.nngroup.com/articles/training-wheels-user-interface/) for the
  exact defect the seed already measured (eight imports and a hand-rolled
  `DomainEvent` impl before a concept is named).
- Every pattern that adds *hidden* content — tabs, accordions, collapsed
  admonitions — inherits this project's own worst documented failure
  (something drifted or lied and nothing noticed, because nothing read it).
  A go/no-go rule for any hidden-content pattern this initiative adopts
  should be: is the hidden branch inside the same `cargo xtask ci` /
  `mdbook test` surface as everything else, or is it invisible to the gate
  the way the old rustdoc-warnings step was. That single question, applied
  per pattern, resolves tensions 1 and 2 above without a new principle.
- The medium's own built-in interaction (rustdoc search, `doc(alias)`,
  intra-doc links, the collapse/expand affordance, mdBook's playground
  button) is under-used relative to what it already offers, and every one of
  it is free — no new dependency, no accessibility burden, no gate surface
  to add. Before adopting any third-party preprocessor (tabs, admonitions,
  mermaid, quizzes), the research suggests exhausting the built-in surface
  first: `doc(alias)` costs one attribute and directly serves the
  `TraitVariantBlanketType`-style findability gap the seed names; intra-doc
  linking from the guide back into rustdoc reference items is the
  "second question" fix the evaluator persona needs, and needs no new
  widget.
- The Diátaxis language already loose in this project's own brief ("which
  need does this page answer") should be treated as borrowing Diátaxis's
  *discipline*, not its *taxonomy* — the evidence that dense, interrelated
  technical subjects strain the four-box split is specific enough to name in
  planning, not a stylistic quibble.
- No credible interaction pattern in this research substitutes for the
  brief's own proof artifact (a dated, non-author friction log). Quizzes are
  the closest tempting substitute and the research is explicit that they are
  not one — they check recall of prose, not the ability to act on it. This
  reinforces rather than complicates the existing proof-artifact plan.
