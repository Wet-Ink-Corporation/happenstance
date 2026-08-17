---
item: HS-S0159
stage: spec
created: 2026-08-17T13:16:15.478Z
updated: 2026-08-17T13:16:15.478Z
template_sig: 87bbf1d0
rendered_sig: 52dba81f
---

# Spec — Somewhere for the second question to go

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-16, AC-06, AC-11, AC-12; DoD 9 and 12 |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG, the warranted briefs, the merge-forward rule |
| Project | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` — DR-3, DR-4, DR-5, DR-8; AC-003, AC-005, AC-012 |
| **This spec** | `.bklg/docs-that-teach/reach-and-adapter-path/evaluator-onward-links/spec.md` |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` — surface `evaluator-onward-links`; DT-10's rule; the `#[doc(alias)]` rule; register rows P4/P5; anti-patterns 3, 5, 6, 10, 14, 18 |
| Key briefs | `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` — UX brief (Journey A, invariants 1–9, UX-AC-03/04/08/10/11) and Architecture brief (N-2 CR-3, N-3, N-4, N-9, N-10, N-11; AC-A01, AC-A07) |
| Story map row / merge order | `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` — slice `front-door-reach`, merge order step 3, standing constraints |
| Grounding | `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` — sibling-project dependency state, verified anchors |
| Sibling designs this story consumes (not owned here) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` (D1: the tree is `docs/`; `TREE`/`HARNESS` constants); `.bklg/docs-that-teach/application-author-path/_design.md` (the page set and its headings) |

Roadmap pointer: `RUNBOOK.md` carries the repository plan of record; this story adds no
phase to it and no `spec/SPECIFICATION.md` clause is edited.

## One-line PR slice

Name the two second questions from DR-4's stall points and make each one hop from the page
that answered the first, targeting the answering *passage* — using only intra-doc links,
`#[doc(alias)]`, rustdoc search and the destination's own TOC, and adding no navigation
component.

## Executive summary

This PR lands **two sentences and two register rows**, and nothing else.

The delta is narrow because the content already exists: HS-P0022 authors the pages, HS-P0020
pins the tree that renders and checks them, and `pointer-policy-and-inventory` has already
decided what a pointer may look like. What is missing is the *link* — the seed's verdict is
literal, *"nowhere for that question to go"*
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`, Persona 3,
journey step 3). So this story:

1. **Names two second questions in writing before authoring anything**, drawn from research
   03's three independently observed stall points (DR-4), and records why those two and not
   the third.
2. **Installs one in-passage link per question** on the page that answered the first
   question, each targeting a *heading fragment* on the passage that answers it — not the
   top of a page, not a "See also" block, not a widget.
3. **Files register rows P4 and P5** in the pointer register with the N-3 form and the named
   guard each one gets, so neither pointer is guarded only by memory.

What this PR does **not** land: any answering *content* (HS-P0022's), the front-door pointer
(its slice-mate's), the walks that observe the hops (`second-question-walk-records`'s — and
that separation is deliberate, because AC-005 forbids the author being their own witness),
and any change to a public item, signature, feature or manifest.

## Context pack

The load-bearing decisions this story must honour. Read this section and you can start; the
deeper artefacts are behind the anchors below.

### The persona slice, and why the timing is unforgiving

The reader is **Persona 3, the evaluator** — a Rust developer reading for a bounded budget
before deciding whether to depend on this. Their first question is answered *well* today; the
failure is what happens next. Two things follow and neither is negotiable:

- **The failure state to design against is not a blank page — it is a good page with no
  exit.** The recorded fallback is `spec/SPECIFICATION.md`'s 200 numbered clauses, which is a
  different document than the second question calls for.
- **The whole journey happens inside one reading session, with no second attempt if the first
  one fails silently** (`personas-and-journeys.md`, Cross-persona tensions). There is no
  "they'll find it tomorrow". A hop that lands somewhere plausible-but-wrong is a lost reader,
  not a deferred one.

### Decision 1 — the gap is a missing *link*, not a missing *widget*. This is settled, not open.

DR-5 and the dossier settle it in the same words: the evaluator's gap *"is evidence of a
missing **link**, not a missing **widget**"*
(`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, Breadcrumb /
master-detail navigation, Fit conditions, and again in Anti-patterns). rustdoc renders a
per-item nav bar; the destination surface renders its own structure. The built-in surface is
what carries the reach, and it must be exhausted before anything else is proposed.

Concretely, for this story: **no breadcrumb, no sidebar rail, no per-crate index page, no
hand-rolled TOC, no prev/next footer, no "See also" / "Next steps" / "Further reading"
block, no raw HTML, no inline `style=`, no fold or tab.** The rejection record itself is
`pointer-policy-and-inventory`'s deliverable (project AC-011, sole owner); this story
*inherits* it as a constraint and must not re-litigate it or file a second copy.

### Decision 2 — the link is **inline, sentence-final, inside the answering passage**

`_design.md`'s per-surface table resolves the `evaluator-onward-links` surface to an **inline
link at the end of the answering passage**, and records what lost:

- *A "See also" block at page bottom* — rejected, because it is reached only after the reader
  has already decided to leave, and it is a navigation surface (DR-5).
- *A sidebar or breadcrumb* — rejected, because the medium already renders the equivalent.

`_design.md`'s `## Composition` states the form exactly: **one inline link, sentence-final, at
the end of the passage that answered the first question. No block, no heading, no list. The
link text names the destination *and* the question it answers, and targets a heading fragment
where the answer is not the destination's first screen** (UX-AC-04). This story implements
that; it does not re-decide it.

### Decision 3 — land on the *passage*, which is why the page's closing pointer is not enough

HS-P0020's page composition gives every narrative page a **closing pointer: at most one link
outward, in one slot, at the bottom** — and says in the same breath that *"its policy is
DT-10 and belongs to HS-P0023"*
(`.bklg/docs-that-teach/checked-documentation-surface/_design.md`, `## Composition`, region
6). Two consequences the implementer must hold together:

- The closing pointer sends the reader to a **page**. These two links send a **question** to
  its **passage**. That is UX invariant 3 — *land on the answer, not the top of the page* —
  and project AC-005's own wording: the record names the destination **passage**, not just the
  page. The links are therefore not redundant with the closing pointer and do not consume its
  slot.
- **They are in-passage links, not the closing-pointer slot.** If HS-P0020's landed checker or
  page rule is read to forbid a second outward link on a page, that is a cross-project seam to
  **escalate**, not a story-local re-decision — DT-10 is this project's tension and
  `_design.md`'s sign-off records that *"any project adding a pointer applies the four gates
  recorded here rather than re-deciding them"*.

### Decision 4 — the four gates every pointer this story installs must pass

From `_design.md`'s binding rule, *"one front door, and a pointer only at a stall"*. A
secondary pointer is permitted at a location only where **all four** hold, and the register
row records which:

1. **(i) evidenced stall** — a named, recorded reader failure, not a suspicion. Here: research
   03's independently observed stall points, carried into DR-4.
2. **(ii) the front door provably cannot reach it** — the front-door sentence names the guide;
   it cannot name a passage inside a page the reader has not opened yet.
3. **(iii) subordinate and one line** — it follows the material it hangs off, never precedes
   it, and introduces no heading of its own.
4. **(iv) a guard from the N-3 mechanism table**, rows 1–3 only. **Row 4 — a bare URL, guarded
   by nothing — is forbidden to this project outright.** Project AC-003: no pointer is
   installed whose only guard is memory.

And the **href ladder**, applied per pointer and recorded in its row: intra-doc link **>** an
in-tree markdown link inside the pinned tree **>** a named-but-unlinked cross-reference in the
form live at `crates/happenstance-core/src/store.rs:77` **>** *nothing*. **If only the fourth
rung is available, the pointer is not installed and the story escalates.** For this story the
realistic rung is the second: the destinations are markdown pages under `docs/`, not Rust
items, so an intra-doc link would not resolve.

### Decision 5 — the tree is `docs/`, and that is now resolved, not TBD

`_design.md` recorded this surface's route as `TBD` because HS-P0020 had not authored its
design. **It has since.** `.bklg/docs-that-teach/checked-documentation-surface/_design.md`
resolves D1: *"The pinned tree is `docs/`, repurposed… the render is the markdown itself"* —
no mdBook, no static site, no second render. Its `## Signatures` pin the two constants this
story depends on: `const TREE: &str = "docs"` and
`const HARNESS: &str = "xtask/src/narrative.rs"`, the lib-crate harness whose `include_str!`
lines register every page.

So the destination form is an **in-tree markdown link with a heading fragment**, guarded by
HS-P0020's `cargo xtask narrative` step (its unregistered / dangling-registration / hidden-marker
/ unresolvable-clause states are enumerated in that file's `## The states the API must
express`). That is N-3 row 3, and it is a real guard — which is what makes rule 2(iv) satisfiable
here at all.

**What is still late-bound:** the page *filenames*. `docs/first-encounter.md` and
`docs/carry-your-invariant.md` are **derived** from HS-P0022's surface routes
(`{tree}/first-encounter/`, `{tree}/carry-your-invariant/`) crossed with HS-P0020's
`docs/<page>.md` shape — they are not confirmed anywhere. Bind every href to the page path
**actually registered in `xtask/src/narrative.rs`** at implementation time; a guessed path is
the invented-primitive failure the design stage exists to prevent.

### Decision 6 — which page answered the first question, and why not the README

The link's origin is **HS-P0022's `opening-encounter` page** — the page a reader following the
front-door pointer reads first inside the guide, and the page whose material (a running
program in which an append is refused because a boundary held) is what *provokes* both second
questions.

The rejected alternative, recorded so it is not re-opened: **the README is not the origin.**
The README's outward pointer is register row P1/P2 and belongs to the slice-mate
`front-door-pointer`; a second and third pointer there would be per-item pointers at a
location with no evidenced stall, failing design rule 2(i) and 2(iii), and would turn the
README into a routing table — `_design.md` anti-pattern 6 (a table with fewer than three rows
used as a navigation device) and DoD item 7's diff-checkable HS-P0016 seam.

### Decision 7 — this story authors linking prose on a page it does not own

The pages belong to HS-P0022. This story adds **linking prose only** — the story map's
coverage table is explicit that AC-012 lands here for *"any linking prose added to a
sibling-owned page"*. Three obligations follow:

- **No second answered-need.** HS-P0021's discipline is one named answered-need per page; a
  sentence that starts answering a question is content, and content on that page is HS-P0022's.
  The sentence points; it does not explain.
- **No normative claim except a resolving citation.** DR-8: a normative claim is a citation
  into `spec/SPECIFICATION.md` that resolves, and no page restates a clause. The safest link
  sentence makes no normative claim at all.
- **Anchored citation form** (AC-A07): named subject plus path, never a bare line range as the
  only handle — `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` is the rule and
  `xtask/src/spec_trace.rs` the working implementation of it.

### Decision 8 — `#[doc(alias)]` is in the permitted mechanism set, and this story installs none

The story map's one-liner lists `#[doc(alias)]` among the built-in affordances this story may
use. `_design.md`'s alias rule bounds it: **an alias is permitted only where the string a
reader searches is one that rustc, the specification, or a recorded reader question actually
emits, and is not the item's own name.** The two qualifying strings today are `E0034` and
`TraitVariantBlanketType` — both the adapter half's, installed by `store-error-site-rewrite`.

A plain-language second question ("is dynamic a synonym for unstructured?") is not a string any
tool emits, and aliasing concepts rather than strings is explicitly rejected as *synonym
farming*. So this story **installs no alias, and records that as a decision with its reason** —
the design is equally explicit that a decision *not* to use `#[doc(alias)]` "is a decision that
must be written down rather than a default" (UX-AC-10). Aliases need no register row either
way: an alias is an attribute on an item, so deleting the item deletes it (N-3).

### Decision 9 — the honest claim is bounded

Two second questions are **plausible**, drawn from the least author-biased candidate set
available; they are not *the* evaluator's real second question, because no evaluator has been
observed. `project.md`'s risk table fixes the wording: the honest claim is *"two plausible
second questions were walked"*, never *"the evaluator's real second question is answered"*.
HS-P0024 is the instrument that can falsify the choice, and this story must not stand in for
it (BR-14's discipline).

### The two questions this story names

Chosen from research 03's three independently observed stall points
(`.bklg/docs-that-teach/_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md`,
which records them as *"Three independent stall points, named by three independent people,
none of them 'the spec is unclear'"*) and carried into DR-4:

| Id | The second question, in the reader's words | Why it is provoked by the origin page | Where it is answered |
| --- | --- | --- | --- |
| **Q-A** | *"Is 'dynamic' just a polite word for unstructured — where is the structure?"* | The opening encounter shows a boundary holding without a stream-per-entity model in sight; dynamism-mistaken-for-chaos is the first stall point | The passage that narrates the four-step cycle and its mapping table on HS-P0022's `conceptual-bridge` — heading `## Tag, query, fold, guard` |
| **Q-B** | *"Is DCB a modelling technique or a routing technique?"* | The reflex question the bridge's own DT-1 resolution names — *"which stream does this go in?"* — is the same confusion in first person | HS-P0022's single recorded prior-model anchor, heading `## Where your streams went` (`application-author-path/_design.md`, DT-1: *"the reader-facing home is the `conceptual-bridge` surface's section 'Where your streams went'"*) |

**The third stall point — no settled replacement noun for what an Aggregate named — is
deliberately not walked here**, and the reason is on record rather than left as an omission:
research 03 itself notes that this repository's vocabulary already diverges from the field's
(`Tags`, `AppendCondition`, `SequencePosition` taken from the DCB specification directly), and
that whether those nouns land is *"a direct probe"* for the friction log — HS-P0024's
instrument, not this story's. Answering it here would require authoring content this story is
not permitted to author.

Two questions is the floor project AC-005 sets ("at least two"), and the floor is what this
story delivers.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice through every layer (a reader with a question reaches its answer) |
| **Slice / milestone** | `front-door-reach` |
| **Slice-mates** (implemented in one context, mounted as one surface) | `front-door-pointer` |
| **Mount point** | `docs/first-encounter.md` — HS-P0022's `opening-encounter` page inside HS-P0020's pinned tree `docs/`, registered by `xtask/src/narrative.rs` and swept by the `cargo xtask narrative` REQUIRED step in `xtask/src/main.rs`. Filename derived from `application-author-path/_design.md`'s route `{tree}/first-encounter/` and `checked-documentation-surface/_design.md`'s `docs/<page>.md` shape — **bind to the path actually registered in the harness, never to this derivation** |
| **Wires into** | `docs/carry-your-invariant.md` (HS-P0022 `conceptual-bridge`) at headings `## Tag, query, fold, guard` and `## Where your streams went` · the pinned-tree constants `TREE` / `HARNESS` (`checked-documentation-surface/_design.md`, `## Signatures`) · the pointer register created by `pointer-policy-and-inventory` (rows P4, P5; shape modelled on `xtask/src/lint_constitution.rs:64` and `:463-473`) · the N-3 mechanism table and href ladder in `reach-and-adapter-path/_decomposition.md` |
| **Design-system primitives consumed** (compose these, hand-roll nothing) | In-tree markdown link with a heading fragment · the destination's own heading structure · rustdoc search (`S` / `/`) · `#[doc(alias)]` — considered, declined, recorded. Full table: `reach-and-adapter-path/_decomposition.md`, UX brief, *Design-system primitives* |
| **Renders surfaces** | `evaluator-onward-links` (surface id, `reach-and-adapter-path/_design.md`, `## Surfaces`) — states `link-in-passage`, `destination-fragment-landing`, `dead-end-baseline`. It **changes** two sibling-owned surfaces by adding one sentence each; it renders no new page |
| **Conformance rule(s)** | **None, and this is not adapter-observable.** Nothing here touches a port, a value type, a bound or a future — no `crates/happenstance-testkit/` rule can observe a markdown link. The mechanical guard is HS-P0020's `cargo xtask narrative` step; the human guard is the observed walk in `second-question-walk-records` |
| **Clause(s)** | None discharged, none amended, no `[FROZEN]` clause touched. `spec/SPECIFICATION.md` is *cited* if and only if the linking prose makes a normative claim — and the preferred sentence makes none (DR-8) |
| **Advances DoD scenario** | Initiative **DoD 9** — *"The evaluator's second question is walked"* — moved toward green: this story builds the two hops and names the two questions; `second-question-walk-records` walks them and turns it green. Secondarily **DoD 12** (*"No page has become a second specification"*), which the linking prose must not break |
| **External gates** (hard, from `_storymap.md` merge order step 3) | HS-P0022's pages must **exist and be registered** — there is nothing to point at otherwise. HS-P0020's hosting shape must be resolved before the destination path is bound (**it now is**: tree = `docs/`). The **merge-forward rule applies verbatim** to the slice, and is **not verifiable from this worktree** |

**This story is delivered mounted.** A pair of links written into a file that no harness
registers and no gate step reads is not this capability — it is a draft. Done means the two
sentences are in registered pages under `docs/`, `cargo xtask ci --fast` is green over the
result, and both hops are followable by a person who is not the author.

## PR boundary

**In this PR**

- The named-questions record (Q-A and Q-B, each traced to its stall point, plus the recorded
  reason the third is not walked) in this story's own backlog folder.
- **Two sentences**: one inline, sentence-final link per question, added at the end of the
  passage on the origin page that raises it.
- **Two register rows** (P4, P5) appended to the pointer register that
  `pointer-policy-and-inventory` landed — each carrying its N-3 form, its href-ladder rung, its
  guard, and its fragment target.
- The recorded `#[doc(alias)]` decision for this story: considered, declined, reason.
- This story's ledger and, at the report stage, its report.

**Explicitly not in this PR**

- **Any answering content.** The passages that answer Q-A and Q-B are HS-P0022's
  (`invariant-to-appendcondition-bridge`, `boundary-refusal-encounter`). This story adds no
  explanation, no example, no heading.
- The front-door pointer on `crates/happenstance/src/lib.rs` and `crates/happenstance/README.md`
  — its slice-mate's rows P1/P2.
- The `store.rs` rewrite, the adapter reasoning account, and the two `#[doc(alias)]` strings
  they carry.
- **The walks themselves.** AC-005 requires a different pair of hands
  (`_storymap.md`, Coverage, AC-005 row).
- Anything under `xtask/src/` — HS-P0020 owns the tree constants, the harness and the checker;
  a second checker forked in here is the explicit failure N-4 warns against.
- Any public item, signature, feature or manifest change (`_storymap.md`, standing
  constraints; architecture brief non-goal). The `prelude` proposal stays out of boundary and
  owed an ADR (N-9).
- The pointer *policy*, the register's shape, and the widget-rejection record — all
  `pointer-policy-and-inventory`'s, consumed here, not re-decided.
- Any navigation widget, hand-rolled TOC, prev/next footer, "See also" block, raw HTML,
  inline style, fold or tab.

**Permitted wiring** — the implementer may also touch the composition-root and register files
named in the Integration contract (the pointer register, and the registered page files the
links land in) to mount this slice. That is not scope drift; it is what "mounted" means here.

**Merge DoD** — `cargo xtask ci --fast` green on the merged result, both hops resolving to
their named heading fragments in registered `docs/` pages, both register rows filed with a
non-empty guard, and no file changed outside this boundary.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Two second questions are named before anything is authored** | Q-A (dynamism-as-chaos) and Q-B (modelling vs. routing), each traced to an independently observed stall point; the third (no replacement noun) is recorded as deliberately not walked, with its reason. DR-4 requires naming before answering, and the story map's coverage table gives the *naming* to this story and the *walking* to another | `reach-and-adapter-path/project.md` (DR-4, AC-005); `.bklg/docs-that-teach/_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md`; `_storymap.md`, Coverage, AC-005 row |
| **One hop, per question, from the origin page** | From the page that answered the first question, each named question reaches its answer in exactly **one** link traversal. No intermediate index page, no "start here" detour | `_decomposition.md`, UX brief, Journey A state A1; `_design.md`, `## What it costs a caller` ("One hop, never two") |
| **The hop lands on the passage, not the page** | Each link carries a heading fragment resolving to the named heading on the destination page. Where the answer is not the destination's first screen, a bare page link is a defect, not a shortcut | UX invariant 3 and UX-AC-04, `_decomposition.md`; `_design.md`, `## Composition`, `evaluator-onward-links` |
| **Form: inline, sentence-final, no device** | One link at the end of the answering passage. No block, no heading, no list, no callout. Hierarchy: the passage is primary, the link is recessive — *"a link that outweighs the passage it sits in is a 'next steps' block wearing a sentence"* | `_design.md`, `## Hierarchy` and `## Pattern decision`, per-surface row |
| **Link text is self-describing and names both destination and question** | ≥ 3 words, a noun phrase, naming the destination *and* the question it answers. Never "here", "this", "see this page", "docs", "read more", or a raw `https://` string — a screen-reader user routinely navigates by extracted link list | `_design.md`, `## Density budget` (minimum legible label) and anti-pattern 3; UX brief, Accessibility floor |
| **Mechanism: in-tree markdown link with fragment; ladder rung recorded** | The destinations are markdown pages, not Rust items, so an intra-doc link would not resolve — rung 2 of the href ladder. The rung chosen is recorded in the register row. Bare URLs into this repository's own tree are forbidden outright | `_design.md`, `## Pattern decision`, rule 3 and rule 2(iv); `_decomposition.md`, N-3; `Cargo.toml:134` (`broken_intra_doc_links = "deny"`, the guard that would apply to rung 1) |
| **Guard: HS-P0020's narrative checker** | Each row's guard is the `cargo xtask narrative` step, which reports *unregistered*, *dangling registration* and the rest as named problem states. If that step has not landed when this story implements, the pointer takes a form already guarded or is **not installed** — never installed with an empty guard cell | `checked-documentation-surface/_design.md`, `## The states the API must express` and `## Signatures` (`TREE`, `HARNESS`); `_decomposition.md`, N-4; `_design.md` anti-pattern 18 |
| **Register rows P4 and P5 are this story's own** | Project AC-003 splits ownership: the foundation story owns the register and the guard rule, each installing story owns its rows. *A story that installs a pointer and files no row has not finished.* Rows carry surface, destination, form, rung, fragment and guard | `_storymap.md`, Coverage, AC-003 row; `_design.md`, `## Density budget`, the five register rows table (P4, P5) |
| **The register is not a page** | Rows are build-time data consumed by a checker, in the shape of `SUMMARIES` and `check_summaries`. Rendering the register would create exactly the second navigation surface DT-10 warns about | `_design.md`, `## Transience policy` (final row) and anti-pattern 14; `xtask/src/lint_constitution.rs:64`, `:463-473` |
| **No dead end; the destination is walk-backable** | Each landing passage either answers the question or names the next hop, and states what it assumes the reader has already read so they can tell whether they arrived out of order. A path that fails is recorded as a **failed walk**, never quietly re-walked | UX invariants 4 and 5, `_decomposition.md`; `_design.md`, `## The states the API must express` (*Refused*) |
| **Empty-destination behaviour: do not install** | If the answering passage does not exist or the heading is not present, **the pointer is not installed** — no placeholder href, no "coming soon". *A pointer into nothing is the dead end invariant 5 forbids and is worse than today's silence* | `_design.md`, `## States`, **Empty** row |
| **The linking prose obeys the page-need discipline** | It introduces no second answered-need on a sibling-owned page, makes no normative claim except as a resolving `spec/SPECIFICATION.md` citation, and restates no clause. Citations are written as named subject + path, not a bare line range | project AC-012, DR-8; `_decomposition.md`, N-8 (citation form, AC-A07); `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` |
| **No widget, no escape from the theme** | No breadcrumb, sidebar, index page, hand-rolled TOC or prev/next footer; no raw HTML or inline `style=`; no fold, tab or `<details>`; heading ladder unskipped and unchanged | DR-5; UX-AC-11; `_design.md` anti-patterns 5, 6, 9, 10; `checked-documentation-surface/_design.md` anti-patterns 1 and 2 |
| **`#[doc(alias)]` considered and declined, in writing** | The two evidenced strings belong to the adapter half; a plain-language question is not a string any tool emits, and synonym farming is rejected. A decision *not* to use the one unexhausted built-in must be written down rather than defaulted | `_design.md`, *`#[doc(alias = "…")]` — considered, and **used**, narrowly*; UX-AC-10 |
| **No conformance rule, no clause, no port** | Nothing here touches a port, a bound, a future or a manifest; both flavours are unaffected because no type changes. Stated explicitly because a story that changes a port and names no rule is a port change nothing can fail — this one changes no port | `_design.md`, `## Items` (no public item) and `## What it costs a caller` (final bullet); `_storymap.md`, standing constraints |
| **The claim stays bounded** | The deliverable is *two plausible second questions have somewhere to go*. Not "the evaluator's real second question is answered", and not "a stranger finds it" — that second claim is HS-P0024's alone | `project.md`, risk table (AC-005 row, and the non-authors-not-non-insiders row); initiative BR-14 |

## Data and migrations

**N/A — no schema, no store, no persisted state, no manifest change, and no migration.**

This story writes prose into two markdown files and rows into a build-time list. Stated
precisely, because "N/A" alone would hide the one thing that *is* data:

- **The pointer register is the only data artefact**, and it is not a database: it is a `const`
  list of surfaces read by a checker, modelled on `SUMMARIES` at
  `xtask/src/lint_constitution.rs:64` and its assertion at `:463-473`. Its schema is created by
  `pointer-policy-and-inventory`; this story **appends two rows** (P4, P5) to it and defines no
  columns. If that story landed the register in a shape with fewer fields than the row content
  this spec requires (surface, destination, form, ladder rung, fragment, guard), the shortfall
  is reported to that story rather than patched here.
- **Capacity, not schema, is the constraint worth stating**: the register is capped at **8 rows
  at this project's close and 20 rows ever**, and *"a 21st row means the pointer policy is
  wrong, not that the register needs a scrollbar"* (`_design.md`, `## Density budget`). This
  story's two rows take the project's total to five — P1, P2 (front door), P3 (error site), P4,
  P5 (here) — inside the cap with headroom.
- **No store adapter, no event, no projection, no `SequencePosition`** is read or written; the
  workspace's dependency rule and both port flavours are untouched.

## Acceptance criteria

Framed from the evaluator's intent — Persona 3, reading on a bounded budget, inside a single
session. Every row is a goal crossing the whole stack (a question in a reader's head reaching the
passage that answers it), not a capability toggle. `redkiln verify` extracts these by the leading
`| AC-001 |` cell, so **every** blocking invariant is a row here and none is a prose bullet
elsewhere in this spec.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** no evaluator has been observed and DR-4 requires the second questions be named *before* they are answered, **WHEN** the implementer starts this story, **THEN** a dated record in this story's folder names exactly two second questions — Q-A (dynamism mistaken for chaos) and Q-B (modelling versus routing) — traces each to its independently observed stall point in research 03, and records why the third stall point is deliberately not walked; and that record exists before the first edit to any page. | `.bklg/docs-that-teach/reach-and-adapter-path/evaluator-onward-links/_named-questions.md` present in the PR, each question citing its stall point in named-subject-plus-path form; read against `_storymap.md` Coverage AC-005 row, which gives *naming* here and *walking* to `second-question-walk-records`; the record's checkpoint commit precedes the page-edit commit in the slice history |
| AC-002 | **GIVEN** an evaluator whose first question was answered well on the origin page and who has formed a sharper one, **WHEN** they follow the link this story installs, **THEN** for each of Q-A and Q-B they reach the passage that answers it in exactly **one** hop, landing on the destination's own heading fragment — never the top of a long page, never an index or a 'start here' detour. | `cargo test -p xtask` — a row test in `xtask/src/pointers.rs` asserting P4 and P5 each carry `targets_fragment: true` and that the registered destination file contains the heading the fragment names; `cargo xtask narrative` (HS-P0020's REQUIRED step in `xtask/src/main.rs`) over the registered tree once it has landed |
| AC-003 | **GIVEN** `_design.md` resolves this surface to an inline link at the end of the answering passage, **WHEN** a reader — including one navigating by extracted link list or by Tab alone — meets it in the rendered page, **THEN** it is one sentence-final inline markdown link whose visible text is a noun phrase of at least 3 words naming both the destination *and* the question it answers (never 'here', 'this', 'see this page', 'docs', 'read more', or a raw URL), reachable with the keyboard because it is a plain link and nothing else, and it introduces no heading, list, block, callout, table, fold, raw HTML or inline `style=`. | `cargo test -p xtask` — the validator's `link_text` rules (at least 3 words, deny list) over the appended rows; `git diff -U0` over the two origin pages showing every added line free of `#`, a list marker, `>`, `<`, and `style=`; read against `_design.md` `## Composition`, `## Hierarchy` and anti-patterns 3, 5, 6, 9 and 10 |
| AC-004 | **GIVEN** a density budget written in real numbers because there will be no perceptual review to catch a bloated one, **WHEN** the two sentences and two rows land, **THEN** each added sentence is a single sentence of at least 8 and at most 30 words with no parenthetical and no semicolon, the need it names is at least 4 words, nothing added is folded, tabbed or revealed on hover or focus, the register is still build-time data rendered to no reader, and `POINTER_REGISTER` holds **5** rows against a cap of **8** now and **20** ever. | word count over both added sentences recorded in the implementation report against the diff; `cargo test -p xtask` — `validate(POINTER_REGISTER)` is `Ok` and the cap assertion holds at 5 of 8; `rg -n "<details>|<summary>|style=" ` over the diff returns nothing; no new rendered page appears in the diff (anti-pattern 14) |
| AC-005 | **GIVEN** a reader who never follows either link and only wants their first question answered, **WHEN** they read the origin passage as it now stands, **THEN** it still answers that question completely and nothing pre-existing on either page has been reworded, reordered, folded, or moved by more than the added sentence's own height — the link is an offer taken after the answer, never a step required to reach it. | `git diff -U0 main...HEAD` over the two origin pages shows additions only and no changed pre-existing line; a revert check recorded in the implementation report — removing the two added sentences restores both files byte-for-byte; read against UX invariants 1 and 2 in `_decomposition.md` |
| AC-006 | **GIVEN** project AC-003 forbids installing any pointer whose only guard is memory, **WHEN** each of the two links is installed, **THEN** its row is appended to `POINTER_REGISTER` in the **same** change, carrying its id (P4, P5), the origin surface, the destination *passage*, an N-3 form from rows 1–3 only (never a bare URL), the href-ladder rung actually used, `targets_fragment: true`, self-describing link text, and a **non-empty named guard** — a story that installs a pointer and files no row has not finished. | `cargo test -p xtask` — `validate` returns `Ok` over the appended register, plus a row test naming P4 and P5 and asserting each guard string is non-empty and each form is not the bare-URL variant; the deliberately-wrong-register tests landed by `pointer-policy-and-inventory` still reject their cases (anti-pattern 18) |
| AC-007 | **GIVEN** the answering passage or its heading may not exist when this story implements, and a pointer into nothing is worse than today's silence, **WHEN** the implementer binds each destination, **THEN** the pointer is installed only if that passage exists, answers the named question, and states what it assumes the reader has already read so the hop is walk-backable; otherwise it is **not installed** — no placeholder href, no 'coming soon' — and the shortfall is recorded as a failed placement and handed to HS-P0022 rather than patched here. | the resolved file and heading recorded per row in the implementation report; `cargo test -p xtask` — the fragment test fails the build when a row's heading is absent; any refusal written up in this story's folder and cited from `_ledger.md` evidence; read against `_design.md` `## States` (**Empty**, **Refused**) and UX invariants 4 and 5 |
| AC-008 | **GIVEN** this story writes prose onto a page HS-P0022 owns and DR-8 forbids any page becoming a second specification, **WHEN** the linking sentence lands, **THEN** it opens no second answered-need on that page, makes no normative claim except a citation into `spec/SPECIFICATION.md` that resolves, restates no clause, writes any citation as named subject plus path rather than a bare line range, and this story's `#[doc(alias)]` decision — considered, declined, with its reason — is on record rather than defaulted. | `cargo xtask spec-trace` green (a REQUIRED step inside `cargo xtask ci --fast`); `rg -n "doc\(alias" ` over the diff returns nothing while the declined decision appears in `_named-questions.md`; the sentence read against HS-P0021's one-need rule, DR-8, and `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` |

**Traceability.** Project **AC-003** → AC-006 (and AC-002's fragment guard). Project **AC-005** →
AC-001, AC-002, AC-007. Project **AC-012** → AC-003, AC-008 (and AC-004's no-device rule). Project
AC-011 is *inherited*, not owned: `pointer-policy-and-inventory` files the rejection record and
AC-003 here is the constraint applied.

## Interaction quality

RFC §6.7/D6. The blocking invariants, in two families. Each is carried by a row **in the table
above** — this section says which row, and how that row is actually falsified. Nothing here is an
extra requirement; a bullet in this section would get no ledger row, no gate and no test.

**Why this section is load-bearing for this story in particular.** There is no perceptual review:
`design.capture` is deliberately absent from `.redkiln/config.yaml`, so no screenshot pass will
ever look at this surface and disagree (`_design.md`, opening note). Every check this story does
get is textual, and **a completely unstyled render satisfies every textual assertion perfectly** —
a bare page link with the text `here`, appended under a `## Next steps` heading, would pass a naive
"the link resolves" test and fail the design entirely. AC-003, AC-004 and AC-005 are what make that
fail.

### STATE invariants

| Invariant | Carried by | How it is falsified |
| --- | --- | --- |
| **In-place before context-jump** — getting the first question answered never requires leaving the page | **AC-005** | Delete both added sentences and find the origin passage no longer answers question one — i.e. the sentence started explaining rather than pointing (UX invariant 1) |
| **Non-occlusion** — a pointer must not displace what it points at, or anything else | **AC-005** | A diff in which any pre-existing line on either page is reworded, reordered, or moved by more than the added sentence's own height (UX invariant 2; project DoD item 7's diff-checkable seam applied to a sibling-owned page) |
| **Land on the answer, not the top of the page** — the hop targets the passage | **AC-002** | A register row whose destination is a page rather than a passage where the answer is not on the destination's first screen (UX invariant 3, UX-AC-04) |
| **Reversibility — every hop is walk-backable** | **AC-007** | Arriving at a destination and being unable to name the page that should have preceded it. This story *checks* it and escalates; it may not author the fix, because that prose is HS-P0022's content (UX invariant 4) |
| **No dead ends** | **AC-007** | A landing passage that neither answers the named question nor names the next hop — recorded as a failed placement, never quietly re-walked (UX invariant 5; `_design.md` `## States`, **Refused**) |
| **Keyboard reachability** | **AC-003** | Any affordance this story installs that is not a plain link — a fold, a script, a widget. The *observed* keyboard-only walk is `second-question-walk-records`'s deliverable; what AC-003 fixes is that there is nothing to reach except a link |
| **Selection, scroll and search survive** | **AC-003**, **AC-004** | Added markup that is rendered-only or non-selectable, or content the reader must open before Ctrl-F can see it. Nothing this story adds may be hidden from a plain-text search of the page |

### COMPOSITION invariants

Taken from the signed-off `_design.md`, which is binding on this story: it implements that design
and does not re-decide it.

| Invariant | The design's own number or rule | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — the pointer is real composed prose, not bare markup | `## Composition`, `evaluator-onward-links`: one inline link, sentence-final, at the end of the answering passage; link text names the destination *and* the question | **AC-003** |
| **Placement** — sentence-final, inside the answering passage, never a page-bottom block | `## Pattern decision`, per-surface row: a 'See also' block at page bottom is rejected because it is reached only after the reader has decided to leave | **AC-003** |
| **Transience** — persistent, always-rendered prose; zero revealed or opened-on-demand affordances; the register is not rendered to a reader at all | `## Transience policy`, final row and the standing zero-folds rule | **AC-004** |
| **Density budget, with its real numbers** — sentence 8–30 words, need ≥ 4 words, link text ≥ 3 words, register 5 rows of 8 now and 20 ever | `## Density budget`, "Minimum legible size for the primary label" and the register row | **AC-004** |
| **Hierarchy** — the answering passage is primary; the link is recessive, inline, sentence-final, carrying no device | `## Hierarchy`, `evaluator-onward-links` row: *a link that outweighs the passage it sits in is a 'next steps' block wearing a sentence* | **AC-003** |
| **Anti-pattern 3** — link text 'here', 'this', 'see this page', 'docs', 'read more', or a raw URL | `## Anti-patterns`, 3 | **AC-003** |
| **Anti-pattern 5** — a 'See also' / 'Next steps' / 'Further reading' block at the bottom of any touched page | `## Anti-patterns`, 5 | **AC-003** |
| **Anti-pattern 6** — a table with fewer than three rows used as a navigation device | `## Anti-patterns`, 6 | **AC-003** |
| **Anti-pattern 9** — any collapsed or folded element in added content | `## Anti-patterns`, 9 | **AC-004** |
| **Anti-pattern 10** — raw HTML or an inline `style=` in added markdown | `## Anti-patterns`, 10 | **AC-003** |
| **Anti-pattern 14** — the pointer register rendered as a page a reader can navigate to | `## Anti-patterns`, 14 | **AC-004** |
| **Anti-pattern 18** — a pointer installed with an empty guard cell | `## Anti-patterns`, 18 | **AC-006** |

**What the mock does and does not approve for this surface.** `design/mock.html` renders
`evaluator-onward-links` for **composition and density only** — HS-P0020's tree had no renderer at
design time, so the frames approve hop placement, hierarchy and word budget and approve *nothing*
about type, colour or measure (`_design.md`, "What the mock cannot be honest about"; finding F5,
dispositioned as **tracked, not closed**). The implementer must not read those frames as a pixel
target, and must not treat their silence on typography as permission to add styling.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | The answering passage or the heading a fragment names does not exist on the destination page when this story implements | **Do not install the pointer.** No placeholder href, no 'coming soon'. Record the absence as a failed placement in this story's folder and hand it to HS-P0022. `_design.md` `## States`, **Empty** |
| EC-002 | HS-P0020's registration check (`cargo xtask narrative`) has not landed, so N-3 row 3's guard does not yet exist | Fall back to a **row-scoped** `#[cfg(test)]` assertion in `xtask/src/pointers.rs` that each of P4/P5's destination file contains its fragment's heading, and record *that* as the row's guard. This is a self-check over this story's own two rows, **not** a second checker forked into `xtask/` — N-4 forbids the latter. If neither guard is available, the pointer is not installed (rule 2(iv)) |
| EC-003 | `pointer-policy-and-inventory` landed the register with fewer fields than a row here needs (surface, destination, form, rung, fragment flag, guard, link text) | Report the shortfall to that story and block. **Do not widen the row type here** — the schema is the foundation story's and a second definition of it is how the register stops being one artefact |
| EC-004 | HS-P0020's page rule or landed checker is read to forbid a second outward link on a page, colliding with the closing-pointer slot | **Escalate as a cross-project seam.** DT-10 is this project's tension and `_design.md`'s sign-off binds siblings to apply its four gates rather than re-decide them. Do not re-decide it story-locally, and do not consume the closing-pointer slot to get around it |
| EC-005 | The passage exists but does not actually answer the named question, or terminates without naming a next hop | Treat as a dead end: do not install, record it as a failed placement, and hand the content gap to HS-P0022. UX invariant 5 |
| EC-006 | The merge-forward rule bites — the origin or destination page differs on the merged tree from what this worktree shows | Re-bind every path, heading and fragment against the **merged** tree before installing. Placement is an invariant-2 judgement, never a mechanical rebase (`_storymap.md`, merge order step 3; **not verifiable from this worktree**) |
| EC-007 | Appending P4 and P5 would take `POINTER_REGISTER` past 8 rows | Stop and escalate. Per `_design.md`, exceeding the cap means the pointer policy is wrong, not that the register needs more room. Two rows against a foreseen five leaves headroom, so hitting this means a sibling filed rows this project did not foresee |
| EC-008 | The anchor slug the harness generates for a heading differs from the heading text (punctuation, case, duplicate-heading suffixes) | Bind the fragment to the anchor the harness actually emits, confirmed by running the check — never to a slug derived by hand. A fragment that looks right and resolves to nothing is the silent failure this whole story exists to remove |
| EC-009 | The origin page's own filename is not what this spec derived (`docs/first-encounter.md`) | Bind to the path **actually registered** in the harness. The derivation in the Integration contract is a convenience, not a fact; a guessed path is the invented-primitive failure the design stage exists to prevent |

## Non-functional

| id | Requirement | Why it is stated |
| --- | --- | --- |
| NF-001 | **No runtime cost, and no measurable build cost.** Two `&'static str`-bearing rows appended to a `const` slice and two sentences of markdown. `cargo xtask ci --fast` gains no step and no dependency from this story | A documentation story that slows the gate has spent a budget it never asked for |
| NF-002 | **No new dependency, no feature, no manifest edit, and no MSRV movement.** The register rows are plain data; nothing here needs `serde`, which keeps ADR-0003's constraint on `happenstance-core` irrelevant to this change rather than merely satisfied | The workspace's binding constraints are easiest to break in a change nobody thinks touches them |
| NF-003 | **Both port flavours and `wasm32` are unaffected.** No type, bound, future or public item changes, so `EventStore` / `SendEventStore` and the `wasm32` builds are untouched by construction | Stated because a change that names no port and edits no signature should say so once rather than leave a reviewer checking |
| NF-004 | **Accessibility floor.** Nothing added carries meaning in colour, position or ASCII art; nothing animates; no contrast is overridden (no raw HTML, no inline style); link text is self-describing and heading ladders are untouched | `_decomposition.md`, UX brief, Accessibility floor — the floor lands in specific, checkable places in a text medium |
| NF-005 | **Maintenance budget: two register rows, forever**, each with a guard that fails a build rather than a reviewer's memory. Total after this story: five of a cap of eight | `_design.md` `## What it costs a caller` — the maintainer's attention is part of the budget |
| NF-006 | **Reader budget: one hop and at most 30 added words per passage.** No path this story installs needs a second hop to reach the answer | `_design.md` `## What it costs a caller`, "One hop, never two" |

## Implementation notes (non-prescriptive)

Shape suggestions, not requirements. The ACs above are the contract.

- **Read the harness before writing a href.** The first action is to open the registration source
  in HS-P0020's landed tree and read off the real page paths and the real anchor slugs. Every path
  in this spec — `docs/first-encounter.md`, `docs/carry-your-invariant.md`, both heading names — is
  *derived*, and the derivation is offered so the implementer knows what to look for, not so they
  can skip looking.
- **Write the questions down first, then the sentences, then the rows.** DR-4's ordering is not
  ceremony: naming the question after authoring the link is how a link acquires a question that
  happens to match wherever it already pointed.
- **Draft both sentences against the word budget before touching a file.** Eight to thirty words,
  one sentence, no parenthetical, no semicolon, link text a noun phrase of three words or more
  naming the destination and the question. It is much cheaper to fail that budget in a scratch
  buffer than in a diff.
- **The two rows are appended, not inserted.** The slice-mate `front-door-pointer` appends P1 and
  P2 to the same `const` in the same slice. Keep ids stable, append in id order, and expect to
  resolve a trivial adjacency conflict rather than a semantic one.
- **Name the row tests after what they reject.** `pointer-policy-and-inventory` established the
  discipline — a validator whose tests only feed it good data is decorative. A test that asserts
  P4 exists proves little; a test that asserts a row with an empty guard or a missing fragment is
  rejected proves the guard.
- **Do not fork a checker.** If a mechanical check is needed that HS-P0020's step does not yet
  provide, it belongs in `xtask/src/pointers.rs` as an assertion over *this register's own rows*,
  scoped and named as such. A second tree-walking checker in `xtask/src/` is N-4's named failure.
- **Keep the linking sentence pointing, not explaining.** The moment it starts answering, it has
  opened a second answered-need on a page whose single need is HS-P0022's, and AC-008 fails.
- **The `#[doc(alias)]` decision is a paragraph, not an omission.** Write down that it was
  considered, that the two evidenced strings belong to the adapter half, and that a plain-language
  question is not a string any tool emits.

## Tests and CI (merge gate)

This project takes **no `testing` brief** — `.bklg/docs-that-teach/_decomposition.md` records
`architecture` and `ux` as the warranted pair, and `project.md` states plainly that automated
checking of the observed walks is owned by nobody, deliberately. So this table is grounded in the
project's **Definition of done** (items 1, 5 and 6), the architecture brief's composition roots
(N-2 CR-3, CR-4) and mechanism table (N-3, N-4), and `CLAUDE.md`'s Commands block. Where a tier is
a human read rather than a command, it says so rather than borrowing a green tick.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Unit (register rows) | `cargo test -p xtask` over `xtask/src/pointers.rs` `#[cfg(test)]` | `validate(POINTER_REGISTER)` is `Ok` with P4 and P5 appended: each guard non-empty, each form from N-3 rows 1–3, each `targets_fragment: true`, link text at least 3 words and not deny-listed, ids unique, 5 rows against the cap of 8 — **AC-004, AC-006** |
| Unit (fragment resolution) | `cargo test -p xtask` — a row-scoped test reading each row's destination file for the heading its fragment names | Each hop lands on a passage that exists. This is the fallback guard of EC-002 and stays scoped to this story's two rows — **AC-002, AC-007** |
| Gate step (sibling-owned) | `cargo xtask narrative` — HS-P0020's REQUIRED step in `xtask/src/main.rs`, once landed | The origin and destination pages are registered, no registration dangles, and the destination path a row names is the path the tree actually serves — **AC-002, AC-006** |
| Gate (whole bar) | `cargo xtask ci --fast` | The non-terminal project's merge bar (project DoD item 1): fmt, clippy with `-D warnings` over `xtask` including the new rows, tests, docs, and `spec-trace` — **AC-004, AC-006, AC-008** |
| Gate (specification) | `cargo xtask spec-trace` | No clause citation in the tree is broken by the added prose, and any citation the linking sentence makes resolves — **AC-008** |
| Diff assertion | `git diff -U0 main...HEAD` over the two origin pages | Additions only; no pre-existing line reworded, reordered or moved beyond the added sentence's height; added lines carry no heading marker, list marker, blockquote, raw HTML or `style=` — **AC-003, AC-005** |
| Diff assertion | `rg -n "<details>|<summary>|style=|doc\(alias" ` over the diff | Zero folds, zero raw HTML, zero aliases installed by this story — **AC-004, AC-008** |
| Revert check (recorded) | Remove the two added sentences locally; `git diff` against the pre-story state is empty | The origin passages are unchanged by anything except the offer — the mechanical form of invariant 1 — **AC-005** |
| Human read (recorded, not automated) | The two added sentences read against `_design.md` `## Composition`, `## Hierarchy`, `## Density budget` and anti-patterns 3, 5, 6, 9, 10, 14, 18 | Composition fidelity. **Nothing in `cargo xtask ci` verifies self-describing link text or composition** — `_design.md` says so in "Two named gaps, not assumed away" (2), and the honest gate here is a recorded read, not a fabricated linter — **AC-003, AC-004** |
| Deferred, by design | The keyboard-only observed walks, one dated record per question | **Not this story's gate.** AC-005 requires a different pair of hands; the walks are `second-question-walk-records`'s deliverable (`_storymap.md`, Coverage, AC-005 row). This story must not close DoD 9 on its own evidence |

**Merge gate, stated as one line:** `cargo xtask ci --fast` green on the merged result, both hops
resolving to their named heading fragments in registered `docs/` pages, both register rows filed
with a non-empty guard, and no file changed outside the PR boundary.

## Risks and coupling (PR-scoped)

| Risk / coupling | Exposure in this PR | Mitigation |
| --- | --- | --- |
| **The register's shape is another story's** | Rows P4/P5 must fit the `Pointer` type `pointer-policy-and-inventory` defines. A field this spec needs and that type lacks blocks the row | EC-003: report the shortfall upward, never widen the type here. The blocking edge is already in `depends_on` |
| **The destination pages are HS-P0022's and may not exist, or may not carry the headings named** | Both hops are unbindable without them; the story map records this as a **hard external gate** on slice 3 | EC-001 and AC-007: no pointer into nothing. The failure mode is a recorded refusal, which is cheap; a placeholder href is expensive and silent |
| **HS-P0020's checker may not have landed** | Without it, N-3 row 3's guard does not exist and rule 2(iv) is unsatisfiable by its intended mechanism | EC-002's row-scoped self-check, recorded honestly as the guard. If even that is impossible, the pointer is not installed |
| **Merge-forward** | The origin page, the destination page and their headings are all unverifiable from this worktree, and `crates/happenstance/src/lib.rs` alone differs by 162 lines between branches | EC-006: re-bind against the merged tree; placement is a judgement, not a rebase. Stated in the Integration contract as a hard external gate |
| **Authoring on a sibling-owned page** | Two sentences land on HS-P0022's pages; the seam is *purpose*, not paragraph | AC-005's diff assertion makes the seam checkable, exactly as project DoD item 7 does for the HS-P0016 README seam |
| **Slice-mate contention on one `const`** | `front-door-pointer` appends P1/P2 to `POINTER_REGISTER` in the same slice and the same context | Append in id order; ids are stable and assigned by `_design.md`'s five-row table, so a conflict is textual adjacency, never two stories claiming one id |
| **Overclaiming the result** | It is tempting to report DoD 9 green when the hops exist | The honest claim is fixed by `project.md`'s risk table: *two plausible second questions were walked* — and even that sentence belongs to the walk story. This PR's claim is narrower still: two plausible second questions now have somewhere to go |
| **The third stall point looks like an omission** | A reviewer may read two-of-three as incompleteness | AC-001 requires the exclusion be recorded with its reason. Two is the floor project AC-005 sets, and the third is a direct probe for HS-P0024's friction log |
| **Anchor-slug drift** | A fragment can look right and resolve to nothing, silently — the exact failure class this story exists to remove | EC-008 plus the fragment-resolution test: the fragment is read off the harness, then asserted |

## Dependencies

**Blocks on** (hard, in-project):

- **`pointer-policy-and-inventory`** — supplies `xtask/src/pointers.rs`: the DT-10 policy as module
  doc, the `Pointer` row type and its `PointerForm` enum, the empty `POINTER_REGISTER`, the
  `validate` function and the deliberately-wrong-register tests. Without it this story has nowhere
  to file P4 and P5, and AC-006 is unsatisfiable. It also owns the widget-rejection record this
  story *inherits* rather than re-files (project AC-011).

**Slice-mate, not a dependency:** `front-door-pointer` — implemented in the same context and mounted
as one surface (`front-door-reach`), in either order within the slice (`_storymap.md`, merge order
step 3). It appends P1/P2 to the same register.

**Unlocks:**

- **`second-question-walk-records`** — walks each of the two named questions keyboard-only and
  produces one dated record per question. It is the story that turns initiative **DoD 9** green, and
  AC-005 requires it be a different pair of hands than this one.

**External gates** (outside this project, hard, and **not verifiable from this worktree**):
HS-P0022's pages exist and are registered; HS-P0020's pinned tree and its registration check are
landed and its hosting shape resolved; the merge-forward rule applied to the slice.

## Anchors (progressive disclosure)

Load-bearing depth is deferred, not optional. The Context pack above is sufficient to start; open
these at the moment named. Every path was confirmed present in this worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` | The **binding** signed-off design: `## Composition` fixes the form of the two sentences, `## Hierarchy` fixes the link as recessive, `## Density budget` carries the real numbers and the five register rows, `## States` carries the Empty and Refused behaviour, and `## Anti-patterns` 3, 5, 6, 9, 10, 14, 18 are the screenshot-and-diff checks | Before writing either sentence, and again before filing either row | AC-003, AC-004, AC-006, AC-007 |
| `.bklg/docs-that-teach/reach-and-adapter-path/pointer-policy-and-inventory/spec.md` | The register this story appends to: the `Pointer` field list, the `PointerForm` variants, `validate`'s contract (every problem, not the first), the cap, and the deliberately-wrong-register test discipline | Before writing the two rows or their tests | AC-006 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | Resolves the tree to `docs/` and pins the `TREE` and `HARNESS` constants; its `## The states the API must express` enumerates unregistered, dangling-registration and unresolvable-clause — the guard AC-006 names and the failure modes EC-002/EC-008 fall into | When binding the destination paths and choosing each row's guard | AC-002, AC-006, AC-007 |
| `.bklg/docs-that-teach/application-author-path/_design.md` | Owns the origin and destination pages, their routes and their headings — including DT-1's ruling that the prior-model anchor's reader-facing home is the `conceptual-bridge` section 'Where your streams went', which is Q-B's landing | Before binding either fragment, and when checking that the landing passage states its assumptions | AC-002, AC-007 |
| `.bklg/docs-that-teach/_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md` | The three independently observed stall points Q-A and Q-B are drawn from, and the source of the recorded reason the third is a friction-log probe rather than this story's | While writing the named-questions record, before any page edit | AC-001 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 3's journey and the one-session timing that makes a silent wrong landing a lost reader rather than a deferred one — the reason AC-007 refuses rather than placeholders | When judging whether a landing passage is good enough to point at | AC-001, AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The dossier entry that settles the gap as a missing **link**, not a missing **widget**, with its fit conditions and anti-patterns — the evidence behind every 'no device' clause in AC-003 | If any reviewer or implementer proposes a navigation affordance | AC-003 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` | The UX brief's nine interaction-quality invariants and accessibility floor, and the architecture brief's N-3 form-and-guard table, N-4 register precedent and N-8 citation form | When choosing the href ladder rung, and when writing the guard cell or any citation | AC-002, AC-006, AC-008 |
| `xtask/src/lint_constitution.rs` | The working precedent the register copies: `SUMMARIES` at `:64` and `check_summaries` at `:463-473`, whose failure message carries its own reason. Read it rather than inventing a check shape | Before writing the row tests, if a message needs a shape to copy | AC-006 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | The named-subject-plus-path citation rule (AC-A07), and why a bare line range is not a handle in a tree that moves | If the linking sentence carries any citation at all | AC-008 |
| `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` | The sign-off frames for this surface — and the frames' own honesty note that they approve composition and density **only**, because HS-P0020's tree had no renderer at design time | When reviewing the two sentences for placement and weight; never as a pixel target | AC-003, AC-004 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` | The sibling-project dependency state and the verified-anchor record — what was actually true in this worktree at planning time, so the implementer knows what to re-verify rather than trust | At the start of implementation, before assuming any sibling artefact exists | AC-007 |
| `crates/happenstance-core/src/store.rs` | The live named-but-unlinked cross-reference at `:77`, which is rung 3 of the href ladder in its real form — the shape to copy if a link cannot resolve | Only if rungs 1 and 2 are both unavailable for a destination | AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half enumerated** — AC-001 through AC-008. None
   added, none dropped; the ledger carries the same eight ids.
2. **The register's file is `xtask/src/pointers.rs`, mounted at `xtask/src/lib.rs`.** The front half
   described the register generically as a `const` list modelled on `SUMMARIES`; the blocking
   story's spec pins the actual module, the `Pointer` field list and `validate`'s signature. This
   story **appends two rows and adds no module**, and if a needed field is missing it reports rather
   than widens (EC-003).
3. **The guard when HS-P0020's checker has not landed.** Resolved as a row-scoped `#[cfg(test)]`
   assertion inside `xtask/src/pointers.rs` over this story's own two rows — explicitly not a second
   tree-walking checker, which N-4 forbids — and recorded as the guard in the row. If neither that
   nor HS-P0020's step is available, the pointer is not installed.
4. **Reversibility on a page this story does not own.** UX invariant 4 requires the destination to
   state what it assumes. That prose is HS-P0022's content, so AC-007 makes it a *precondition the
   implementer checks and escalates*, not something this story authors. This is the only place a UX
   invariant is discharged by refusal rather than by construction, and it is deliberate.
5. **`#[doc(alias)]` is declined here, in writing.** The two evidenced strings are the adapter
   half's; a plain-language question is not a string any tool emits and aliasing concepts is synonym
   farming. AC-008 makes the *record* of that decision checkable, per UX-AC-10's rule that a
   decision not to use the one unexhausted built-in must be written down rather than defaulted.
6. **The third stall point is excluded on the record, not omitted.** AC-001 requires the reason —
   it is a direct probe for HS-P0024's friction log and answering it here would require authoring
   content this story may not author.
7. **Composition has no automated gate, and this spec says so rather than inventing one.**
   `_design.md` already measured the gap ("Two named gaps, not assumed away", item 2): nothing in
   `cargo xtask ci` verifies self-describing link text or keyboard reachability. AC-003 and AC-004
   are therefore carried by diff assertions plus a **recorded human read**, and the observed walk
   stays with `second-question-walk-records` where AC-005 put it.
8. **Every page path and heading in this spec is derived, not confirmed.** `docs/first-encounter.md`,
   `docs/carry-your-invariant.md`, `## Tag, query, fold, guard` and `## Where your streams went` come
   from crossing HS-P0022's routes with HS-P0020's `docs/<page>.md` shape. EC-008 and EC-009 make
   binding them to the harness's real registration an obligation rather than a courtesy.
