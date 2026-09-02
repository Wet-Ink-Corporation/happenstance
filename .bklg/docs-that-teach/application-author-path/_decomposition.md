---
Companion file — not a Redkiln item. No frontmatter fields are system-owned here;
edit freely. Written for HS-P0022 `application-author-path`. This file holds *all*
of this project's briefs; the `testing` brief is authored into its own section by
its own author. Only `ux` and `testing` are warranted here
(`.bklg/docs-that-teach/_decomposition.md:319`).
---

# Briefs — The Application Author's Path (HS-P0022)

## UX brief

### Intent

Scope the *reader-facing experience* of the two surfaces this project authors — the
opening encounter and the conceptual-bridge material — for Persona 1, the
application author
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:56-144`).
This brief states what the experience must **be**, not how a page is laid out: the
reader's intents and the states they can legitimately be in, the documentation
primitives this repository already owns and the implementer must compose rather than
invent, the accessibility floor, and the interaction-quality invariants that separate
a page that teaches from a page that merely renders. Layout, medium and page
composition are `_design.md`'s (DT-1, DT-4, DT-5, DT-6); the substrate is HS-P0020's.

The reader's medium is documentation, so the "UI" here is the pinned narrative tree,
rustdoc, fenced code, anchors and links. That does not make the interaction-quality
bar softer. Persona 1's stated fear is **silent wrongness** — "a mental model that
looks right, compiles, runs, and is quietly wrong"
(`personas-and-journeys.md:99-106`) — and the affordances below are chosen because
each one is answerable to that fear rather than to page count. The initiative's own
non-goal is explicit: *"Volume. Pages written is not the measure"*
(`.bklg/docs-that-teach/initiative.md`, Non-goals).

**Precondition, not a preference.** No page is authored until AC-014's merge forward
from `initiative/from-contract-to-published-library` lands. This worktree's
`crates/happenstance/src/lib.rs` is 75 lines and does **not** contain the
`Tags::empty()` defect the initiative's evidence names; that file has been replaced,
not merely diverged, on the sibling branch
(`.bklg/docs-that-teach/application-author-path/_grounding.md:23-32`). Every line
citation to `lib.rs` in this brief is to the *shape* of an existing mechanism, not to
prose that survives the merge.

---

### The reader, and what they are trying to do

Stated as intent, not as mechanism. Each intent names the state the reader must
reach; how they get there is `_design.md`'s to compose.

**UI-1 — "Show me the thing this library is for, working."**
The reader wants to watch a consistency boundary refuse an append, in a program they
ran themselves. Today they get API shape and an empty boundary
(`personas-and-journeys.md:107-129`, steps 2-3). *Reached when:* the program's own
output carries the refusal. Traces to AC-004, DR-01, initiative DoD-3.

**UI-2 — "Tell me it is real and not a story about itself."**
The reader wants evidence the boundary is load-bearing rather than decorative. The
only affordance that answers this is one they can operate: break it, watch something
fail, put it back. *Reached when:* the page states the exact edit, the exact
failure, and the exact revert, and the reader can run all three. Traces to AC-005,
DR-03, initiative DoD-4.

**UI-3 — "Take the rule I already have and show me how to say it here."**
The reader arrives holding a cross-entity invariant in ordinary event-sourcing
vocabulary and wants it carried, without a translation step of their own, to a
`Query`, a fold and an `AppendCondition`. *Reached when:* the carry completes with
no step that requires opening `spec/SPECIFICATION.md` or the crate source. Traces to
AC-008, DR-08, initiative AC-02.

**UI-4 — "Tell me what you think I already believe."**
The reader wants to know which prior model the teaching is arguing against, once,
in a place they can point at — not re-derived differently on each page. Traces to
AC-001, AC-009, DR-06, DR-07, initiative AC-08. The dossier records this as
genuinely unresolved and upstream of everything else
(`interaction-patterns.md:547-566`, tension 6; and `_grounding.md:151-158` confirms
zero grounding for it anywhere in `.kb/`).

**UI-5 — "I found the good example. Let me actually read it."**
The reader wants `examples/course-subscriptions/` to be reachable and to speak for
itself. Its module doc is one of the three strongest explanations this project has
written and is currently unpublished and unlinked
(`examples/course-subscriptions/src/main.rs:1-19`;
`examples/course-subscriptions/Cargo.toml:8`). Traces to AC-010.

**Non-intent, explicitly.** The reader who knows neither event sourcing nor DCB is
out of scope, settled on evidence at intake
(`personas-and-journeys.md:42-52`). No affordance below is designed for them, and a
page that starts teaching event sourcing has changed the product.

---

### States, framed from intent

Every state below is one a real reader lands in. The right-hand column is the
contract; a page that cannot satisfy it for a state is defective in that state even
if it renders.

| State | How the reader got here | What must be true |
| --- | --- | --- |
| **Cold start** | Followed the crate root or the front-door table (`docs/README.md:12-23`) into the opening encounter at step one | The one answered-need is visible before the first fence (AC-012); the DT-1 anchor decision is either applied or explicitly cited, never silently assumed |
| **Mid-sequence arrival** | Landed on step four from rustdoc search or a web result, having never seen steps one to three | The page says which step this is, what the earlier steps established, and reaches step one in one hop. This is the *named* failure mode of staged disclosure (`interaction-patterns.md:161-166`), so it is a designed state, not an edge case |
| **Refusal observed** | Ran the program; saw `AppendError::ConditionViolated` | The refusal came from the program's output, not from a sentence beside it; the clause it satisfies is cited (ES-25, `spec/SPECIFICATION.md:3693`, `[FROZEN]`) and not restated |
| **Deliberately broken** | Removed the boundary to test UI-2 | The page told them exactly what to remove and what failure to expect; the failure is legible enough to recognise as *the* failure and not a build error |
| **Restored** | Reverted the removal | The check passes again, and nothing else in their tree needs repairing. Reversibility is a state, not a footnote |
| **Mid-bridge** | Halfway through carrying their own invariant across | Every term introduced so far is either defined on the page or is a term they arrived with; no forward reference to the specification or the source is load-bearing (AC-008) |
| **Handed off** | Followed the link to `examples/course-subscriptions/` | They know what they are about to read and why, before they leave; the module doc's own words arrive intact, not paraphrased (AC-010) |
| **Vocabulary seam** | Read `use happenstance::{…}` on a page, then opened the example, which imports `happenstance_core` | The seam is named in one sentence at the link. See the vocabulary rule under Notes — this is a real, unavoidable friction and the brief's position is *name it*, not hide it |

---

### Primitives to compose — the design system this repository actually has

There is no CSS token layer here and the implementer must not invent one. The
primitive layer is the repository's own documentation machinery, and every item
below is a real, existing (or HS-P0020-owned) mechanism with a path.

**Owned by HS-P0020, consumed here — never re-implemented:**

- **The pinned narrative tree.** Its path is a constant in `xtask/src/`, not a
  convention (`.bklg/docs-that-teach/checked-documentation-surface/project.md`,
  AC-001). Pages authored here land *in* that tree. Creating a parallel tree, a
  second renderer, or a page outside it is out of contract.
- **The compiled fence.** Every Rust fence is compiled against the real crates by a
  `REQUIRED` step with `probe: None` (ibid., AC-002). This project writes fences; it
  does not choose or wire the mechanism.
- **The enumerated allowance list.** The single sanctioned way to mark a block as
  not-checked (ibid., AC-004). If DT-6 ships an uncompiled contrast, it goes on that
  list under AC-003's stated basis. **Inventing a second marker is a defect** even if
  it renders identically.

**Owned by HS-P0021, consumed here:**

- **The answered-need notation.** Whatever form HS-P0021 defines, in that form
  exactly. DR-14 forbids a second notation, and AC-012 is checked by a reviewer
  applying HS-P0021's own check to this project's page set.

**Existing in-repo mechanisms with a proven shape:**

- **Verbatim surfacing via `#[cfg(doctest)] mod X { #![doc = include_str!(…)] }`** —
  `xtask/src/constitution.rs:39-50`. This is the repository's proven way to pull an
  existing markdown file into a checked doc surface *without paraphrasing it*, which
  is precisely AC-010's bar. Its rationale carries a hard constraint the implementer
  must honour: **one module per included file**, because concatenated includes report
  a failure in the nineteenth file at a line number counted from the first, "which
  maps to no file a reader can open" (`xtask/src/constitution.rs:11-18`).
- **The checked-README primitive** — `crates/happenstance/src/lib.rs:10`
  (`#![cfg_attr(doctest, doc = include_str!("../README.md"))]`) with its own reasoning
  at `:1-9`. The crate root is already inside the compiled-prose mechanism this
  project's fences must join.
- **The signpost table** — `docs/README.md:12-23`, a two-column
  `| Looking for | It is at |` routing table already in the tree. If this project's
  material needs to be pointed *at*, the row shape already exists. **The pointer
  policy itself is HS-P0023's** (DT-10); this project supplies content that is
  reachable, and does not decide where the pointer lives.
- **Rustdoc's built-in surface** — intra-doc links (with
  `broken_intra_doc_links` denied workspace-wide), `#[doc(alias)]`, type-aware search,
  the per-item nav bar, and `+`/`-` collapse. The dossier is explicit that this layer
  is under-used and carries none of the hidden-content risk of any preprocessor
  (`interaction-patterns.md:628-634`), and that a missing *link* must not be answered
  with a bespoke *widget* (`interaction-patterns.md:436-441`).
- **The doc-comment style tokens** — `standards/rust/70-rustdoc-obligations.md`, the
  only constitution atom on documentation: RS-70-2 (never an intra-doc link that
  resolves in only some feature configurations), RS-70-3 (teach `doc-valid-idents` a
  proper noun; never `allow(clippy::doc_markdown)`), RS-70-5 (name the alternative
  that lost, once).
- **Clause citation by stable id** — `spec/SPECIFICATION.md:280` ("Clause IDs are
  stable and are never renumbered"). This is the citation primitive: **cite the id,
  never restate the clause** (AC-011, DR-09; initiative Non-goals). The three ids
  this project's material actually needs are ES-25 (`:3693`, `[FROZEN]`), VT-30
  (`:1813`, `[PROVISIONAL]` — do not imply the shape is frozen) and, if the bridge
  motivates why the boundary must be tag-based rather than type-only, CF-7 (`:7266`,
  `[FROZEN]`).

**Do not hand-roll (each has a named reason, not a taste):**

- Tabs. `mdbook-tabs`' own documentation does not state whether an inactive panel is
  inside `mdbook test` (`interaction-patterns.md:467-482`), the tension is
  HS-P0020's DT-7 and is unresolved, and the full ARIA tablist/tab/tabpanel triad
  plus roving `tabindex` is an obligation this project has no mandate to take on.
- Collapsed admonitions or accordions holding anything load-bearing. NN/g's rule and
  this repository's own realised hazard agree
  (`interaction-patterns.md:238-252`, `:404-410`).
- A bespoke navigation widget, rail, or breadcrumb. The medium already renders the
  equivalent (`interaction-patterns.md:382-390`).
- Custom CSS, JS, or any rendered affordance the base medium does not already ship.
- A second answered-need notation, a second fence-exemption marker, or a second
  place to record the DT-1 anchor decision.
- A diagram vocabulary invented outside DT-5's recorded resolution.

---

### Accessibility floor

The theme belongs to HS-P0020's renderer; **content-level conformance is this
project's**, and the items below are checkable on the page source without a browser.

- **WCAG 2.2 AA at the content layer.** One `h1` per page; no skipped heading levels
  (a reader on a screen-reader heading list is the mid-sequence arrival state's other
  face). Link text is meaningful standing alone — never "here", "this", "see this
  clause". Every abbreviation the reader did not arrive with is expanded on first use
  on each page, DCB included.
- **Colour is never the only channel.** This is not hypothetical here: the closest
  prior art carries *the entire consistency story* in two edge colours — orange for a
  write that must consult a DCB view first, blue for ordinary fan-out
  (`interaction-patterns.md:290-296`). If DT-5 resolves to ship a diagram, the
  read-for-consistency versus plain-append distinction must be carried by a second,
  non-colour channel (an edge label, a line style, an explicit `reads [...]` clause)
  **and** restated in the accompanying prose or mapping table.
- **A text equivalent for every diagram, and it is not an afterthought.** A diagram
  is "prose the gate can render but not typecheck"
  (`interaction-patterns.md:520-526`), so the text equivalent is the artifact that
  keeps it honest as well as the one that makes it accessible. The numbered-list /
  mapping-table narration the dossier records as convergent across three independent
  sources and zero-cost (`interaction-patterns.md:320-340`) discharges both
  obligations at once; if a diagram ships, that narration ships beside it.
- **Reduced motion.** Nothing on this project's pages animates, auto-advances, or
  steps on a timer. A static render is the floor and also the ceiling — this project
  has no reason to test `prefers-reduced-motion` because it introduces nothing that
  would violate it, and introducing something that would is a scope change.
- **Copy fidelity.** What a reader copies out of a fence must be what runs: no shell
  prompt characters inside a Rust fence, no line-number gutter in the copy buffer, no
  elided `…` inside a block claimed to be checked.
- **Keyboard reachability by construction.** Every affordance on these pages is one
  the base medium already renders and already makes keyboard-reachable. See IQ-6.

---

### Interaction-quality invariants

First-class and testable. Each states its own falsification, so a reviewer can run
it rather than judge it.

**IQ-1 — In place, not a context jump.** The bridge completes without the reader
leaving for `spec/SPECIFICATION.md` or the crate source. Clause citations are
provenance, not required reading.
*Test:* strike every off-page link from the page; the argument still completes and
the reader can still write their own `Query`, fold and `AppendCondition`. (AC-008,
DR-08.)

**IQ-2 — Non-occlusion: the disclosure must never hide the thing it discloses.**
Three concrete forms, all checkable on the source:
  1. **No hidden doctest line may carry any part of the boundary.** Rustdoc's `#`
     prefix hides a line from the render while still compiling it — the existing
     crate root uses it at `crates/happenstance/src/lib.rs:61`. A hidden line that
     constructs the `Query`, the `Tags`, the `AppendCondition`, or that performs the
     refusal assertion, produces a page where a boundary appears to hold with no
     boundary visible. That is AC-006's defect wearing a different hat: prose
     claiming a real boundary over code the reader cannot see.
     *Test:* render the page, read only what is visible, and rebuild the boundary
     from it.
  2. **No fold, collapse or panel holds an invariant, a `MUST`, or a constraint the
     page's argument needs.** *Test (the dossier's own):* delete the collapsed
     section — does the page still teach the constraint correctly
     (`interaction-patterns.md:404-410`)?
  3. **If DT-6 ships a "wrong model" contrast, it must not occlude the right one.**
     It is marked as wrong at both its start and its end, is never the last code on
     the page, and never appears above the correct version in reading order — a
     mid-sequence arrival must not be able to mistake it for the recommendation.

**IQ-3 — Mid-sequence arrival is a designed entry point, not an accident.** From any
anchor in the staged sequence the reader can see which step they are on, what the
previous step established, and reach step one in one hop. This is the interaction
shape of AC-002/DR-04 and it discharges the literature's named failure mode for
staged disclosure (`interaction-patterns.md:161-166`).
*Test:* open each step's anchor cold, in a fresh session; can the reader name what
they missed?

**IQ-4 — Place is preserved.** Every step and every named concept has a stable,
human-readable anchor. Anchors are not numbered, so inserting a step does not break
an inbound link. A link from `docs/README.md:12-23` or from a rustdoc intra-doc link
lands on the heading, not the page top. Nothing this project adds resets rustdoc's
`+`/`-` collapse state or the sidebar's scroll position, because this project adds
no such affordance (IQ-6).
*Test:* every anchor this project's own pages cite resolves, and re-ordering a step
in a draft breaks no inbound anchor.

**IQ-5 — Reversibility, in the reader's hands.** AC-005's falsification is not only
a gate obligation; it is the reader's own affordance, and it is the *only* one in
this set that answers Persona 1's stated fear directly
(`personas-and-journeys.md:99-106`). The page states the exact edit that removes the
boundary, the exact failure to expect, and the exact revert, and the reader ends with
a working tree.
*Test:* a reader who has never seen the repository performs remove → observe failure
→ revert → observe pass, from the page alone.

**IQ-6 — Keyboard reachability by construction, zero bespoke affordances.** Every
affordance on this project's pages is one the base medium already renders and already
makes keyboard-reachable. Adding one requires HS-P0020's DT-7 resolved first.
*Test:* enumerate the interactive affordances this project introduced. The correct
answer is none.

**IQ-7 — The refusal is observable, not narrated.** The refusal appears in the
program's own output as `AppendError::ConditionViolated` (ES-25,
`spec/SPECIFICATION.md:3693`), not in adjacent prose.
*Test:* delete every sentence around the fence; a reader who runs the code still sees
the boundary refuse. (AC-004, DR-01.)

**IQ-8 — One need, stated before the first fence.** The page's single answered-need,
in HS-P0021's notation, appears above the first code block in reading order — a
reader who bounces off the fence has already been told what the page is for.
*Test:* HS-P0021's own check, applied by a reviewer, finds no page carrying two
(AC-012).

**IQ-9 — Vocabulary is stable across the reader's whole path.** The reader is taught
one import vocabulary and meets it everywhere this project controls. Per ADR-0006
(`.kb/decisions/0006-bare-name-to-the-typed-layer.md`), that vocabulary is
`use happenstance::{…}` — the shape already modelled at
`crates/happenstance/src/lib.rs:59` — because the application author is exactly the
reader the bare name was given to.
*Test:* no fence this project authors imports from `happenstance_core`; where the
reader is sent somewhere that does, the seam is named in one sentence at the link
(see Notes).

---

### Acceptance criteria

Testable, implementer-facing, and traceable to the project ACs in
`.bklg/docs-that-teach/application-author-path/project.md`.

- **UX-001.** Following the opening encounter from its first step, the reader reaches
  a running program whose **own output** carries the refusal; no prose sentence is
  required to know a refusal happened. *(AC-004, IQ-7, DoD-3.)*
- **UX-002.** The opening encounter states, in the reader's own operating terms, the
  exact edit that removes the boundary, the failure to expect, and the revert — and a
  reader following only the page can perform all three and end with a passing check.
  *(AC-005, IQ-5, DoD-4.)*
- **UX-003.** No hidden doctest line (`#`-prefixed) on any page this project authors
  carries any part of a consistency boundary or the assertion of its refusal; a
  reader reconstructing the boundary from the *rendered* page alone succeeds.
  *(AC-006, DR-02, IQ-2.1.)*
- **UX-004.** No collapsed, folded or otherwise hidden region on any page this
  project authors holds an invariant, a `MUST`, or a constraint the page's argument
  needs; deleting every such region leaves each page's teaching intact.
  *(AC-006, IQ-2.2.)*
- **UX-005.** Every step anchor in the opening encounter, opened cold, tells the
  reader which step they are on, what the previous steps established, and offers a
  one-hop route to step one. *(AC-002, DR-04, IQ-3.)*
- **UX-006.** Striking every off-page link from the bridge material leaves it
  complete: the reader can still carry a cross-entity invariant from ordinary
  event-sourcing vocabulary to a `Query`, a fold and an `AppendCondition` without
  opening `spec/SPECIFICATION.md` or the crate source. *(AC-008, DR-08, IQ-1.)*
- **UX-007.** Every normative claim on these pages is a citation to a resolving
  clause id, and a spot check finds no clause restated in the page's own words. Where
  VT-30 (`spec/SPECIFICATION.md:1813`) is cited, the page does not imply the shape is
  frozen. *(AC-011, DR-09.)*
- **UX-008.** Each page carries exactly one answered-need, in HS-P0021's notation,
  positioned above the page's first fenced block. *(AC-012, DR-14, IQ-8.)*
- **UX-009.** The DT-1 anchor decision is applied identically on every page and every
  page relying on it cites the one recorded location; no page re-argues it.
  *(AC-001, AC-009, DR-06, DR-07, UI-4.)*
- **UX-010.** `examples/course-subscriptions/src/main.rs:1-19` reaches the reader in
  its own words — surfaced, not paraphrased — and the reader knows before following
  the link what they are about to read and why. Where surfacing uses the
  `#[cfg(doctest)] mod` + `include_str!` shape, it is **one module per included
  file** (`xtask/src/constitution.rs:11-18`). *(AC-010, UI-5.)*
- **UX-011.** Every fence this project authors imports from `happenstance`, not
  `happenstance_core`; where the reader is routed to material that does otherwise,
  the seam is named in one sentence at the point of the link.
  *(ADR-0006, IQ-9.)*
- **UX-012.** No page authored here introduces an interactive affordance the base
  medium does not already render — no tabs, accordions, nav widgets, custom CSS or
  JS. The enumeration of introduced affordances is empty. *(IQ-6,
  `interaction-patterns.md:436-441`, `:467-482`.)*
- **UX-013.** Content-level accessibility holds on every page: one `h1`, no skipped
  heading levels, link text meaningful in isolation, every fence copy-faithful.
  *(Accessibility floor.)*
- **UX-014.** If DT-5 resolves to ship a diagram: the query/append-condition cycle
  itself is drawn (not only the retiring model); no distinction in it is carried by
  colour alone; and a numbered-list or mapping-table narration of the same cycle
  ships beside it as its text equivalent. *(AC-013, DR-11, accessibility floor.)*
- **UX-015.** If DT-6 resolves to ship an uncompiled contrast: it appears on
  HS-P0020's enumerated allowance list, is visibly marked as not-checked on the page,
  is marked wrong at both ends, is never the last code on the page, and never precedes
  the correct version in reading order. *(AC-003, AC-007, DR-12, IQ-2.3.)*

---

### Notes

**The vocabulary seam is real and the position is to name it, not hide it.** ADR-0006
puts the bare name on the typed layer for exactly this reader
(`.kb/decisions/0006-bare-name-to-the-typed-layer.md`), so UX-011 makes
`use happenstance::{…}` the taught vocabulary. But
`examples/course-subscriptions/src/main.rs:24-27` imports `happenstance_core`
directly, which is *correct for that example's own purpose* — it is a store-level
demonstration, not an onboarding surface (`_grounding.md:44-49`). Changing the
example's imports is not this project's call. The module doc AC-010 surfaces
(`:1-19`) contains no imports at all, so surfacing it verbatim is safe; the *source*
the reader may then open is where the seam appears. One sentence of orientation at
the link discharges this. Do not paper over it and do not edit the example.

**Rewriting the crate root: referent yes, reasoning no.** Where this project touches
`crates/happenstance/src/lib.rs`'s doc, the test in
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` applies — does the edit
change what the document *asserts*? ADR-0006's argument for why the bare name sits on
the typed layer is reasoning that still stands and is preserved, not rewritten.

**A compiled fence proves less than it looks like it proves.** Doctests do not
receive the workspace `[lints]` table and `cargo clippy` does not lint doctests at
all (`xtask/src/constitution.rs:26-35`). The fence check proves the code compiles;
it does not prove the code is idiomatic. A page whose teaching value rests on an
idiom rather than on a type is not protected by HS-P0020's mechanism, and that gap
is the one this initiative exists to name — compiling and teaching correctly are
"different properties that only partly overlap"
(`.bklg/docs-that-teach/initiative.md`, Vision).

**Degraded diagnostics under `include_str!` constrain, but do not decide, the
surfacing shape.** A failing fence inside an included markdown file reports against
the *including* item's file and line
(`.bklg/docs-that-teach/checked-documentation-surface/project.md`, constraints
table). `xtask/src/constitution.rs:11-18` already paid for this lesson and its
answer — one module per file — is the shape UX-010 requires. Which mechanism
HS-P0020 finally pins is theirs; this constraint travels with whichever one it is.

**DT-1 has no grounding to look up, and this brief does not settle it.** A repo-wide
`rg -n "aggregate" .kb` returns nothing (`_grounding.md:151-158`). The UX consequence
of *any* DT-1 resolution, though, is fixed and stated here as UX-009 and UI-4: the
decision needs one citable home, and no page may open by re-narrating a pain the
reader may not have. `happenstance` is DCB-native and "never had an aggregate to
kill" — a page that opens by relieving imaginary pain is a worse experience than one
that starts from the invariant. That is an interaction constraint on the resolution,
not the resolution.

**DT-5 is cheap to over-invest in.** It is ranked lowest-confidence, carried as a
`Could` (BR-18), and prose or mapping-table narration is recorded as convergent
across three independent sources and zero-cost
(`interaction-patterns.md:320-340`). UX-014 is written so that the *narration* is
required whenever a diagram ships — which means choosing narration alone costs
nothing that shipping a diagram would not also cost.

**Handoffs, so nothing is absorbed silently.** Pointer and reach gaps found while
authoring go to HS-P0023 (which owns DT-10 and every pointer policy). Doubts about
whether the bridge lands go to HS-P0024, not back into this brief. Substrate gaps —
a fence class that cannot express what a page needs, an allowance list that cannot
hold DT-6's exemption — go to HS-P0020. Incidental bugs route to the `support`
initiative per `.redkiln/config.yaml`. This mirrors the project's DoD item 9.

**No deviation from any Accepted decision atom.** ADR-0006 is honoured by UX-011 and
IQ-9. ADR-0007 (`.kb/decisions/0007-projection-runner-decodes.md`) is not engaged:
the opening encounter as scoped — a boundary refusing an append — needs no
`Projection` trait. If any page authored here does grow projection material, ADR-0007
becomes binding and this brief has not scoped it.

---

## Testing brief

### Intent

Scope how every project AC-### in
[`project.md`](project.md) gets *proven* rather than merely written — the test mix,
the gate commands that run it, and the fixtures (or, mostly, the deliberate absence
of fixtures) this project's two surfaces are checked against. This brief is earned by
two things named explicitly in `project.md`'s Companions section: AC-005 (DoD-4 —
removing the boundary must make a repository check *fail*) and DT-6's possible
compiled "wrong model" side, which if it ships is `publish = false` and gate-checked
forever. Everything else this project writes is prose; these two are the only places
where "checked rather than asserted" (the initiative's own framing,
`.bklg/docs-that-teach/initiative.md`, Vision) has to survive contact with `rustc`.

Same precondition as the UX brief: no page is authored, and no test is written
against one, before AC-014's merge forward lands
(`_grounding.md:23-32`, DR-13).

The discipline this brief borrows wholesale from CLAUDE.md's own testing philosophy —
written for adapter conformance rules but stated generally enough to reuse — is:
*"before adding [a check], name a plausible wrong implementation it rejects."* Every
tier below is justified against a named wrong page, not against coverage for its own
sake.

---

### The test mix

This is a documentation project, so "unit/integration/e2e" maps onto something
narrower than usual. Five tiers, ordered by how mechanically they are enforced:

| Tier | What it proves | Mechanism | Enforced by |
| --- | --- | --- | --- |
| **1. Structural (proof by construction)** | Verbatim surfacing actually is verbatim; clause citations resolve; intra-doc links are not silently broken | `#[cfg(doctest)] mod X { #![doc = include_str!(…)] }` — the identical-bytes guarantee, not a diff check (`xtask/src/constitution.rs:39-50`); `cargo xtask spec-trace`; `RUSTDOCFLAGS=-D warnings` on `cargo doc` (denies `broken_intra_doc_links`, RS-70-2) | `cargo xtask ci` — REQUIRED |
| **2. Compiled-fence (HS-P0020's mechanism, consumed not built)** | Every Rust fence in this project's pages type-checks against the real workspace crates | The pinned narrative tree's REQUIRED step, `probe: None` | `cargo xtask ci` — REQUIRED, once HS-P0020 lands the step |
| **3. Executed (doctest or co-located `#[test]`)** | The boundary actually refuses; the refusal is the *program's* output, not adjacent prose | A doctest without `no_run`, or a `#[tokio::test]` in the manner of `examples/course-subscriptions/src/main.rs:195-224`'s `commit` helper | `cargo test --locked --workspace --all-features -- --show-output` — REQUIRED, `xtask/src/main.rs:143-155` |
| **4. Human-observed falsification (recorded once)** | Deleting the boundary breaks something; restoring it un-breaks it | The DoD-4 drill: remove the query the `AppendCondition` is built from, run tier 3's check, watch it fail, revert, watch it pass | Definition of done item 3 — a signed-off observation, not a per-commit gate |
| **5. Design / review sign-off** | DT-1…DT-6 are resolved with reasoning; the answered-need, DT-1-consistency and no-restated-clause obligations hold across the page set | A reviewer walks the page set against `_design.md` and HS-P0021's own check | Definition of done items 1 and 7 |

Tiers 1–3 are what `cargo xtask ci` (or its `--fast` profile) can fail a commit on.
Tiers 4–5 are what a human signs off once, because — as `_design.md`'s own binding
note states — `design.capture` is deliberately absent from `.redkiln/config.yaml`, so
there is no perceptual gate for prose composition; the recorded observation *is* the
only record these checks will ever have, the same reason the UX brief gives for
`_design.md` itself.

**The named wrong implementation each tier rejects**, per the CLAUDE.md discipline
above:

- Tier 1 rejects a page whose "surfaced" module doc has silently drifted from
  `examples/course-subscriptions/src/main.rs:1-19` (AC-010) or whose clause citation
  points at a renumbered or deleted clause (AC-011).
- Tier 2 rejects a fence that does not compile against the real crates at all — the
  base failure mode HS-P0020 exists to close.
- Tier 3 rejects the fence that compiles but is marked `no_run` (or is otherwise
  never executed) — see Notes below. This is the tier that turns AC-004 from "the
  page says a refusal happens" into "the page's own output carries one."
- Tier 4 rejects a boundary that looks real in the source but is not load-bearing —
  a query that happens to never match anything the scenario appends, so removing it
  changes nothing observable. This is the initiative's named central risk
  (`project.md`, Risks table, row 1) and AC-005 is the *only* instrument against it.
- Tier 5 rejects a DT-1 resolution applied on one page and silently re-argued or
  contradicted on another (AC-009) — the exact defect BR-07 exists to prevent.

---

### AC-### to test-tier mapping

Every project AC-### from `project.md`, mapped to at least one tier. "n/a" tiers are
stated, not omitted, so the mapping stays auditable.

| AC | Tier(s) | Proof mechanism |
| --- | --- | --- |
| AC-001 (DT-1 resolved, signed off before authoring) | 5 | `_design.md` sign-off, DoD item 1 |
| AC-002 (DT-4 resolved, failure mode addressed) | 5 | `_design.md` sign-off, DoD item 1 |
| AC-003 (DT-5+DT-6 joint resolution, exemption basis if uncompiled) | 5 | `_design.md` sign-off, DoD item 1 |
| AC-004 (refusal reaches the reader in the program's own output) | 2, 3 | compiled-fence step + executed doctest/test, DoD item 2 |
| AC-005 (removing the boundary fails a check; reverting passes it) | 3, 4 | executed test, run and observed in both directions, DoD item 3 |
| AC-006 (no empty boundary where prose claims a real one) | 3, 5 | tier 3's assertion is the mechanical half; a reviewer spot-check is the prose half — nothing greps prose claims automatically |
| AC-007 (every fence exercised or named against the allowance list) | 1, 2, 5 | compiled-fence step for the mechanical half; a reviewer-maintained inventory of this project's blocks for the "or names each exception" half |
| AC-008 (bridge completes with no off-page reading required) | 2, 5 | fences compile (tier 2); IQ-1's own falsification ("strike every off-page link, the argument still completes") is a reviewer check, tier 5 |
| AC-009 (DT-1 decision applied identically, cited once) | 5 | reviewer spot-check across the page set |
| AC-010 (course-subscriptions surfaced verbatim, reachable) | 1 | `include_str!` structural guarantee (verbatim by construction) + a working link (tier 2's compiled-fence step does not check links; a reviewer confirms reachability, tier 5) |
| AC-011 (normative claims cite, never restate, a resolving clause) | 1, 5 | `cargo xtask spec-trace` resolves the citation; "never restate" is a reviewer spot-check |
| AC-012 (exactly one answered-need per page, HS-P0021's notation) | 5 | HS-P0021's own check, walked by a reviewer, DoD item 7 |
| AC-013 (a shipped diagram draws the arriving model, not only the retiring one) | 5 | reviewer check, conditional on DT-5 |
| AC-014 (merge forward complete and recorded before authoring) | preflight, tier 0 | see Notes — a one-time gate before any story in this project's map starts, not a `cargo xtask ci` step |

Tiers 1–3 are what a story's implementer can run locally and re-run in CI. Tiers 4–5
are what the project's Definition of done items 1, 3 and 7 already require — this
mapping does not invent a new obligation, it names which existing one each AC rides.

---

### Merge-gate commands

The commands below are cited from `xtask/src/main.rs`'s `REQUIRED` array and from
CLAUDE.md's own Commands section — this brief adds none of its own.

- **`cargo xtask ci --fast`** — the bar this project is actually held to. This project
  is `terminal: false` (`project.md` frontmatter), so DoD item 5 names `--fast` as the
  applicable profile; the whole-initiative `cargo xtask ci` re-observation is
  HS-P0025's, at closeout.
- **`cargo test --locked --workspace --all-features -- --show-output`** — REQUIRED
  step `"tests"` (`xtask/src/main.rs:143-155`). This is the command that actually
  executes tier 3 — it is what turns AC-004 and AC-005 from a written claim into a
  pass/fail result, and it is the step to run locally while iterating on the opening
  encounter's fence.
- **`cargo doc --locked --workspace --all-features --no-deps --document-private-items`**
  with `RUSTDOCFLAGS=-D warnings` — REQUIRED step `"documentation"`
  (`xtask/src/main.rs:290-301`). Catches a broken intra-doc link or an unresolved
  `#[doc(alias)]` this project's prose introduces (RS-70-2).
- **`cargo xtask spec-trace`** (equivalently `cargo run -p xtask -- spec-trace`) —
  REQUIRED step `"specification traceability"` (`xtask/src/main.rs:315-327`). Fails
  loudly if this project cites a clause id that does not resolve — the mechanical
  half of AC-011.
- **`cargo xtask affected --base main`** — scopes the gate to what this project's diff
  could break (CLAUDE.md, Commands). The right command for a single story's checkpoint
  commit; `--fast` is the project-level bar.
- **`cargo run -p course-subscriptions`** — explicitly **not** a `REQUIRED` step today
  (`xtask/src/main.rs:105-`, and `project.md`'s In-scope list says so directly: "is
  not a step in `REQUIRED`"). Running the binary by hand proves nothing to the gate.
  If this project's boundary-refusal proof lives in or alongside this example rather
  than in a fresh doctest, it must arrive as a `#[test]`/`#[tokio::test]` target that
  the `"tests"` step already sweeps (`--workspace --all-features` covers every crate,
  including `publish = false` ones) — not as a manual run.

---

### Fixtures and seams — deliberately, none to mock

The instruction here inverts the usual one: **do not introduce a seam.** The only
store this project's checked material may construct is a real, in-process
`happenstance::MemoryEventStore` — the same construction already modelled at
`crates/happenstance/src/lib.rs:60` and `examples/course-subscriptions/src/main.rs:34`
(`MemoryEventStore::new()`), imported per ADR-0006's vocabulary (`use
happenstance::{…}`, UX-011).

Two things this project's tests must *not* reach for, and why:

- **`happenstance_testkit::fixtures::MemoryFixture` and
  `event_store_conformance!`** (CLAUDE.md, "The rule that matters") are the
  conformance suite's own reference fixture, scoped to proving an *adapter* satisfies
  the port. This project is not building or grading an adapter; sweeping the full
  conformance suite over one demonstration scenario would prove the wrong thing and
  cost real CI time doing it.
- **Any mock, stub or hand-rolled fake of `EventStore`.** A mocked store is exactly
  the shape of defect Persona 1's stated fear names — "a mental model that looks
  right, compiles, runs, and is quietly wrong"
  (`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:99-106`).
  IQ-7 and AC-004 require the refusal to be the *program's own output*; a stubbed
  store that returns a canned `AppendError::ConditionViolated` regardless of what was
  actually appended would satisfy the letter of "the page compiles and runs" while
  destroying the one thing the demonstration is for. If a future page needs to
  illustrate a failure mode too expensive to provoke against a real store, that is
  DT-6's uncompiled-contrast lane, not a mock — and it must land on HS-P0020's
  enumerated allowance list, visibly marked, per AC-003.

The DT-6 "wrong model" contrast, if it ships compiled, is not exempt from this either:
it still runs against a real `MemoryEventStore`, it is simply demonstrating the
absence of a condition (`Tags::empty()` or an omitted `after`) rather than illustrating
a mocked failure.

---

### Notes

**`no_run` is the concrete failure mode to check for.** A rustdoc doctest executes by
default under `cargo test`; `no_run` compiles it and stops there. A fence marked
`no_run` passes tier 2 (the compiled-fence step) while silently never reaching tier 3
— the boundary would type-check forever without ever being asked to refuse anything,
which is precisely the "compiles forever while quietly ceasing to demonstrate its own
claim" risk `project.md`'s Risks table names first. The implementer should treat any
`no_run` (or `ignore`) attribute on the boundary-refusal fence as a defect to justify,
not a formatting choice — and per the UX brief's Primitives section, `ignore` is
already the enumerated exemption's job (HS-P0020, AC-004 of that project), not a
second marker this project invents.

**AC-005's proof is coupled to a mechanism this project does not build.** Per
`project.md`'s Risks table (last coupling note): "if that mechanism turns out not to
be able to run the boundary scenario, this project owes a check of its own (an
integration test against `MemoryEventStore` in the manner of
`examples/course-subscriptions/src/main.rs:200-224`) rather than a weaker criterion."
Concretely: before relying on tier 3 to satisfy AC-004/AC-005, confirm HS-P0020's
compiled-fence step actually *executes* doctests (the ordinary rustdoc behaviour)
rather than only type-checking them. If it only type-checks, this project adds its own
`#[tokio::test]`, wired into the `"tests"` REQUIRED step, rather than accepting AC-005
as satisfied by compilation alone.

**AC-007's inventory is a review artifact, not a script.** Nothing in this repository
today enumerates "every fenced block a given project authored" — HS-P0020's mechanism
proves each fence individually compiles (or is on the allowance list); the *inventory*
that AC-007 asks for ("shows zero opted out, or names each exception") is this
project's own bookkeeping, produced once per story and walked by the reviewer
alongside DoD item 7's answered-need check.

**Routing.** A test-infrastructure gap this project finds but does not itself own —
for instance, HS-P0020's compiled-fence step turning out not to execute doctests at
all — routes to HS-P0020 as a substrate gap, per `project.md`'s DoD item 9 and this
brief's UX-companion Notes ("Substrate gaps ... go to HS-P0020"). AC-005 itself does
not route away under any circumstance: the Risks table is explicit that it is "this
project's alone."
