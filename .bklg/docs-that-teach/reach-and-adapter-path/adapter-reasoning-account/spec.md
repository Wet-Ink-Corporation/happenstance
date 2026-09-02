---
item: HS-S0155
stage: spec
created: 2026-08-17T13:16:13.497Z
updated: 2026-08-17T13:16:13.497Z
template_sig: 87bbf1d0
rendered_sig: 2ec9c6d1
---

# Spec — The sequenced adapter reasoning account

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project item | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` |
| This spec | `.bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/spec.md` |
| Key briefs | `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` — the **UX brief** (interaction-quality invariants 1, 5, 8; UX-AC-07, UX-AC-11; the accessibility floor) and the **architecture brief** (AC-A03, AC-A07; N-2 CR-3/CR-4, N-3, N-8, N-10, N-11) |
| Binding design (signed off 2026-08-17) | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` — `## Surfaces` (`adapter-reasoning-account`), `## Pattern decision`, `## Composition`, `## Transience policy`, `## Density budget`, `## Hierarchy`, `## States`, `## Anti-patterns` |
| Design mock (approved) | `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` |
| Grounding (sibling-project state, verified) | `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` |
| Roadmap pointer | `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` — slice `adapter-error-site`, merge order step 2 |

## One-line PR slice

Author the sequenced adapter reasoning account as a **registered page** in HS-P0020's pinned tree: the six
existing sources in the order an adapter author needs them, each cited in anchored named-subject + path
form, with the "`MemoryEventStore` is the conformance oracle, not an adapter" caveat where it is first
sequenced.

## Executive summary

This PR lands **one page and nothing else**. The reasoning an adapter author needs already exists in this
workspace — it is spread across six surfaces in no reading order, and the project item states that gap
precisely: *"AC-05 is a sequencing job, not an authoring job"*
(`.bklg/docs-that-teach/reach-and-adapter-path/project.md`, "How this advances the initiative", third
bullet). The delta this PR contributes is therefore **order plus connective tissue plus citations** — not
new explanation, not a chapter, and not a relocation of any of the six sources.

Two things make it a *slice* rather than a document. First, the page must be **mounted**: registered in
HS-P0020's pinned narrative tree so that the gate sees it, not dropped in as a loose file (AC-A03).
Second, it is the **destination half** of the `adapter-error-site` slice — its slice-mate
`store-error-site-rewrite` installs the one-hop pointer into it from
`crates/happenstance-core/src/store.rs`, and the two are one slice precisely so neither ships pointing at
nothing (`_storymap.md`, "Why these slices and not others", first bullet).

What is *not* delta, and must not be re-decided here: the page's composition, its reading order's position
on the first screen, the per-heading sequence position, the caveat's placement, the word budget and the
one-onward-hop rule. All of those are settled in `_design.md` and signed off. This story implements that
surface; it does not re-open it.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted anchor (the second pass
authors that table); nothing below needs a file opened to be actionable.

**1. The page's job is order, and volume is the failure mode.** Project AC-008 reads "sequencing, not
volume". The decomposition resolved the initiative's AC-05 to *surfacing and sequencing* on the charter's
"volume is not the measure" non-goal (`.bklg/docs-that-teach/_decomposition.md`, "AC-05's scope"). The
budget is a number, not a sentiment: **≤ 900 words target, 1,200 hard cap**, and **≤ 5 sentences / ~120
words of connective tissue per source**. Past 1,200 words the page has become an authoring job and AC-008
has failed — the design's binding instruction is to **escalate to the initiative, never trim the sequence**
(`_design.md`, `## Density budget`, "whole page" row). Connective tissue is the only thing that ever
yields; a citation and the caveat never do.

**2. Six sources, in this order, decided — including the one the brief asked to be decided explicitly.**

1. `crates/happenstance-core/src/memory.rs` — `MemoryEventStore`'s "The reference implementation" doc and
   its `# Examples` full-DCB-loop doctest (`:16-71`), which is runnable today.
2. `standards/rust/91-adapter-authoring-recipe.md` — RS-91-1..4, the recipe carrying the only compiled
   **non-`memory`** adapter example in the tree (`PgStore`). **Included, deliberately**: the architecture
   brief asked for this to be decided rather than guessed, and `_design.md` binds it as entry 2 — omitting
   a directly on-topic atom from a page whose premise is findability-over-invention would be the odd choice
   (`_decomposition.md`, N-8 "Decide (2) explicitly"; `_design.md`, `## Composition`).
3. `CONTRIBUTING.md`, `## Writing an adapter` (`:69`) — the four-step recipe in contributor voice.
4. `standards/rust/20-two-flavour-ports.md` — why two flavours, with the E0034 collision as a compiled
   negative.
5. `standards/rust/25-what-removes-send-and-sync.md` — what removes the bounds.
6. `CONTRIBUTING.md`, `## Adding a method to a port` (`:97`, the rule at `:104`) — why a provided method is
   never `async fn`. **The heading is `## Adding a method to a port`; there is no "Provided methods"
   heading** — `_design.md`'s closing citation correction records that mistake having already been made
   once, and it is not to be re-made.

**3. Staged disclosure, with the sequence stated up front — and its documented failure mode mitigated in
two specific places.** The chosen pattern's recorded failure is a reader who "can land past the setup with
no signal they missed it". The binding mitigation is two-part and both parts are checkable: the numbered
reading order sits **before the first section and inside the first screen**, and **every section heading
carries its own position** ("3 of 6"), so a reader arriving on a fragment from `store.rs` or a search
result learns where they are from the heading they landed on rather than by scrolling up (`_design.md`,
`## Pattern decision`, "Staged disclosure's failure mode, mitigated"; UX invariant 8). Rejected here and
not to be revisited: progressive/hierarchical disclosure (a menu of optional depth destroys the one thing
the page is), tabs or folds (DT-7 is HS-P0020's and this project installs none), and Diátaxis's four boxes.

**4. One page, not two — with a named fallback, not an improvised one.** The architecture brief left this
open (N-11); `_design.md` binds it: one page, because the page's single answered need *is* the order and
splitting it splits the content. **If HS-P0021's rule lands and forbids six sections on one page, the
split is by *phase*** — *why two flavours* | *how to write one* — **never by source**, and region 2 (the
reading order) stays whole on the first page.

**5. The caveat is content, not politeness, and its position is specified.** `MemoryEventStore` is the
conformance suite's oracle and the reference implementation — **not an adapter**. Sequencing it as "the
adapter walk-through" without saying what it is *not* teaches that `RwLock<Vec<_>>` is the shape an adapter
takes, which is the exact monoculture `CLAUDE.md` warns about under "A port is only as well-designed as the
*spread* of what implements it". The caveat goes **at entry 1, where the source is first sequenced** — not
in a closing note (UX-AC-07) — and it is **paired in the same breath with a named adapter at the other end
of that axis**: `standards/rust/91-adapter-authoring-recipe.md`'s `PgStore`, and `references/adapter-shapes.md`.
The source text to defer to is `memory.rs`'s own three-reasons list ("it is the oracle the conformance
suite is validated against…", `:16-31`).

**6. Cite, never restate — and the precedence chain is obeyed, not extended.** ADR-0001
(`.kb/decisions/0001-async-port-flavours.md`) is why two flavours exist; ADR-0008
(`.kb/decisions/0008-one-derivation-for-both-ports.md`) states what one derivation and a provided body owe.
The page **cites both** and must not become a third statement of the rule alongside
`standards/rust/20-two-flavour-ports.md` and `standards/rust/25-what-removes-send-and-sync.md` (N-5, DR-7).
Any normative claim is a **citation into `spec/SPECIFICATION.md` that resolves**, never a restated clause
(DR-8, project AC-012, initiative DoD 12). Decisions are reached **through the supersession graph** in
`.kb/maps/decision-map.md` so the page cites a standing atom rather than a superseded one (`_design.md`,
`## The states the API must express`, "Superseded reasoning").

**7. Citations take the anchored form, because the tree moves under them.** Every citation is **named
subject + path**, with a line range as a convenience rather than the only handle (AC-A07). This is the
problem `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` was written for and
`xtask/src/spec_trace.rs` implements — a windowed search for a short subject string, declining rather than
guessing. A bare `file:line` as the sole handle silently mis-points the moment anything is inserted above
the target.

**8. The page inherits a discipline that does not exist yet, and that is planned for, not discovered.**
HS-P0021 (`.bklg/docs-that-teach/page-need-discipline/project.md`) is still at `stage: storymap` with no
`_design.md` (`_grounding.md`, "Sibling-project dependency state", verified). Author against the
initiative's stated shape — **one named answered need on the page** (HS-P0021 AC-006; initiative DoD 8) —
and expect a conformance pass once the rule lands. **Do not invent the rule here** (N-8, final paragraph).
The need this page declares is fixed by `_design.md`, region 1: *"in what order do I read what already
exists, to build an adapter."*

**9. The mount is the hard external gate, and the correct response to its absence is to block.** The page
is a page in HS-P0020's pinned tree, seen by that tree's registration mechanism — **not** a loose file
under `docs/`, **not** a new `//!` module doc, **not** a section appended to `CONTRIBUTING.md` (AC-A03). If
HS-P0020 has not landed a pinned tree with a registration path, this story **blocks**; it does not ship a
loose file and call it mounted. That is the same structural stance the project takes for AC-010 against the
clause-id pin (`project.md`, risk table, "The `store.rs` rewrite silently un-discharges a frozen clause").
The design records this as tracked-not-closed finding **F5**: this surface keeps `route: TBD` because
inventing a route would be the invented-primitive failure the design stage exists to prevent.

**10. The persona-journey slice this realizes is B2→B3, and it has a hard boundary with B1.** The adapter
author at **B2** wants to tell the difference between *"the port is wrong for me"* and *"I have not
understood the port yet"* — Persona 2's goal verbatim. At **B3** they are building a model of what an
adapter is shaped like. This page serves both. It **does not** serve B1 (getting unstuck): UX invariant 1
puts the in-place fix in `store.rs` ahead of any hop, and *falsifies* itself by deleting this page and
finding `store.rs` no longer tells a reader how to resolve E0034. **A reader who never follows the link
must still be correctly unstuck** — which means nothing this page contains may be load-bearing for B1.

**11. One onward hop at the end, and exactly one.** The terminal region states what the page does *not*
cover and offers **one** onward hop — never a list (UX invariant 5: no dead ends; `_design.md`
`## Composition` region 5). A "See also", "Next steps" or "Further reading" block appended to the bottom is
anti-pattern 5 and is forbidden. The hop's form comes from the href ladder's guarded rungs — an in-tree
markdown link inside the pinned tree, guarded by HS-P0020's registration check (N-3 row 3). **A bare URL
into this repository's own tree is forbidden outright to this project** (`_design.md`, rule 2(iv) and
`## Shape decision`, final row).

**12. Standing constraints this story inherits and does not re-litigate** (`_storymap.md`, "Standing
constraints every story inherits"): no public item, signature, feature or manifest change; no widget, raw
HTML, inline style, folded or tabbed content, no skipped heading levels; self-describing link text — no
"here", no bare URL into this tree.

## Integration contract

- **Archetype**: `capability` — a user-observable slice (the adapter author reaches a page and reads it),
  delivered mounted.
- **Slice / milestone**: `adapter-error-site`. **Slice-mate**: `store-error-site-rewrite` (implemented in
  the same context, mounted as one integrated surface). This story is authored **first** within the slice —
  it is the destination the slice-mate's pointer resolves to (`_storymap.md`, merge order step 2).
- **Mount point**: **HS-P0020's pinned narrative tree.** The page is mounted when it is *registered* —
  reachable by whatever mechanism HS-P0020's AC-005 ("no orphan pages",
  `.bklg/docs-that-teach/checked-documentation-surface/project.md:215-216`) uses — and when its fences, if
  it has any, are compiled by HS-P0020's `REQUIRED` step (its AC-002). The composition roots are the
  architecture brief's **CR-3** (the pinned tree, path constant in `xtask/src/`, precedent
  `xtask/src/lint_constitution.rs:54-61` reading `standards/rust/` by path) and **CR-4** (the `REQUIRED`
  step list in `xtask/src/main.rs`, whose `probe` contract at `:85-102` makes `None` mean *mandatory*).
  **This project does not own `xtask/src/` and must not fork a second checker into it** (N-1, N-4): the
  mounting act here is *registering the page in the tree the sibling pinned*, not adding a step. The
  inbound hop into this page is installed by the slice-mate at
  `crates/happenstance-core/src/store.rs`'s `## Import one flavour, not both` section — this story does not
  write that pointer, but the page must exist at a path that pointer can name.
- **Wires into**:
  - the pointer-policy substrate landed by `pointer-policy-and-inventory` — the permitted pointer forms and
    the href ladder that govern this page's single onward hop;
  - the six cited sources: `crates/happenstance-core/src/memory.rs`,
    `standards/rust/91-adapter-authoring-recipe.md`, `CONTRIBUTING.md` (`:69`, `:97`),
    `standards/rust/20-two-flavour-ports.md`, `standards/rust/25-what-removes-send-and-sync.md`;
  - the decision atoms it cites and does not restate: `.kb/decisions/0001-async-port-flavours.md`,
    `.kb/decisions/0008-one-derivation-for-both-ports.md`, reached via `.kb/maps/decision-map.md`;
  - the normative source it defers to: `spec/SPECIFICATION.md`;
  - the counter-example material for the caveat: `references/adapter-shapes.md`;
  - **design-system primitives only** (`_decomposition.md`, UX brief, "Design-system primitives — compose
    these, hand-roll nothing"): a markdown link on a page inside the pinned tree, the rustdoc/markdown
    heading ladder without skips, and compiled/uncompiled fences. Nothing is hand-rolled.
- **Renders surfaces**: `adapter-reasoning-account` (`_design.md`, `## Surfaces`). Its four declared states
  are this story's to realise: `first-screen-reading-order`, `entered-mid-sequence`,
  `memory-caveat-in-position`, `terminal-section-onward-hop`. `route` stays `TBD` until HS-P0020 pins a
  path — design finding **F5**, dispositioned *tracked, not closed*; final capture of this surface is gated
  on that path landing.
- **Advances DoD scenario**: initiative **DoD 10** — *"The adapter author's error meets its explanation"* —
  by building the explanation the slice-mate's pointer reaches; the *observation* half is
  `error-site-walk-record`'s. Also moves **DoD 8** (every page's answered need is stated and singular) and
  **DoD 12** (no page has become a second specification) toward green for this page.
- **Project ACs traced**: AC-008 (sole owner), AC-012 (shared, one per surface authored — this story owns
  the narrative page).

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed outside it.

```
docs/**
xtask/src/narrative.rs
.bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/**
```

**The first glob was a bound-at-implementation placeholder and is now bound.** HS-P0020 landed on this
branch: the pinned narrative tree is `docs/` (`xtask/src/lint_narrative.rs:239`, `TREE`), its index is
`docs/README.md` (`:250`), and its no-orphan mechanism is the bidirectional registration check
`check_registration` (`:536`) reading the harness at `xtask/src/narrative.rs` (`:257`, `HARNESS`) under the
mandatory `every narrative page is checked` step. `route` is therefore bound to
`docs/adapter-reading-order.md` and design finding **F5** is closed for this surface.

**The second line is the mount and not a widening.** Registration in this tree *is* an
`include_str!` + `mod` pair in `xtask/src/narrative.rs`, so the page cannot be mounted without touching
that one file — which is the "index/nav/manifest entry inside the tree that HS-P0020's no-orphan mechanism
reads" this boundary's "In this PR" list already permits. Nothing else under `xtask/src/**` is touched: no
pinned-tree constant, no `REQUIRED` step, no checker (N-1, N-4; NF-005).

**In this PR**

- The reasoning account page itself: regions 1–5 in the render order `_design.md` `## Composition` binds
  (H1 + answered-need · the numbered reading order · the caveat at entry 1 · six positioned sections · the
  terminal region with one onward hop).
- Its registration in the pinned tree — the index/nav/manifest entry inside the tree that HS-P0020's
  no-orphan mechanism reads. *The implementer may touch the tree's own registration file to mount this
  page; that is the mount, not scope drift.*
- This story's own backlog folder (spec, ledger, reports).

**Explicitly not in this PR**

- `crates/happenstance-core/src/store.rs` — the E0034 restoration, the plain-words ambiguity sentence, the
  narrow unchecked-limit sentence, the pointer *into* this page, and the two `#[doc(alias)]` keys are all
  `store-error-site-rewrite`'s (project AC-006, AC-007, AC-010).
- `xtask/src/**` — the pinned-tree constant, the `REQUIRED` step and any checker are HS-P0020's (N-1, N-4).
- `crates/happenstance/src/lib.rs`, `crates/happenstance/README.md` — the front-door pointer is
  `front-door-pointer`'s, and its adapter-facing variant is **forbidden** on `happenstance`'s crate root by
  ADR-0006's own conclusion stated at `crates/happenstance/src/lib.rs:20-25` (`_design.md`,
  `## Placement and re-export`).
- Any edit to the six sources. **The account orders them; it does not relocate, rewrite or copy them**
  (`_design.md`, `## What a user meets first`).
- The pointer register / inventory itself — `pointer-policy-and-inventory` owns the register and the guard
  rule; this story consumes them.
- The walk that proves the path is reachable — `error-site-walk-record`, deliberately a different pair of
  hands (`_storymap.md`, backbone B6).
- Any new public item, signature, feature or manifest change — including the `prelude` proposal, which is
  out of boundary and owed an ADR (N-9; `_design.md`, `## Items`).

**Merge DoD (one line)**: the page is registered in the pinned tree and the gate sees it, `cargo xtask ci
--fast` is green on the merged result, and every AC below is discharged with its evidence recorded in the
ledger.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **One registered page, not a loose file** | A single page in HS-P0020's pinned tree, reachable by the tree's own no-orphan mechanism. Not under `docs/`, not a `//!` module doc, not a `CONTRIBUTING.md` section. If the tree or its registration path has not landed, the story blocks rather than shipping unmounted. | `_decomposition.md` AC-A03, N-2 CR-3; `.bklg/docs-that-teach/checked-documentation-surface/project.md:215-216` |
| **One page, with a named fallback** | One page, because the answered need *is* the order. Fallback if HS-P0021's rule forbids six sections on one page: split **by phase** (*why two flavours* \| *how to write one*), never by source; the reading order stays whole on the first page. | `_design.md`, `## Composition`, `adapter-reasoning-account`; `_decomposition.md` N-11 |
| **Region order is binding** | 1 H1 + answered-need · 2 the numbered reading order · 3 the caveat, in position at entry 1 · 4 six sections in sequence · 5 terminal region. No region may be reordered or omitted. | `_design.md`, `## Composition` |
| **The reading order is stated before the first section** | Numbered list of the six sources, each **≤ 2 rendered lines**: what it is, why it is at that position, and its anchored citation. H1 + need (3) + list (≤ 12) = **≤ 15 rendered lines**, inside a first screen defined as ≈ 27 rendered lines at 1024x768. The list never scrolls off the first screen and never loses an entry. | `_design.md`, `## Density budget`, "first screen" row; UX invariant 8 |
| **The six sources, in this order** | (1) `memory.rs` reference-implementation doc + full-DCB-loop doctest `:16-71`; (2) `standards/rust/91-adapter-authoring-recipe.md` (RS-91-1..4, `PgStore`); (3) `CONTRIBUTING.md` `## Writing an adapter` `:69`; (4) `standards/rust/20-two-flavour-ports.md`; (5) `standards/rust/25-what-removes-send-and-sync.md`; (6) `CONTRIBUTING.md` `## Adding a method to a port` `:97` (rule at `:104`). Entry 2 is **included by decision**, recorded so a reviewer need not guess. | `_decomposition.md` N-8; `_design.md`, `## Composition`, "Which six" |
| **Every section heading carries its position** | "3 of 6" or equivalent, on each of the six. A reader landing on a fragment learns their position from the heading, not by scrolling up. | `_design.md`, "Staged disclosure's failure mode, mitigated"; anti-pattern 17 |
| **The caveat, in position and paired** | At entry 1: `MemoryEventStore` is the conformance oracle and reference implementation, **not an adapter** — deferring to `memory.rs`'s own three-reasons list — paired in the same breath with a named adapter at the other end of the axis (`91-adapter-authoring-recipe.md`'s `PgStore`; `references/adapter-shapes.md`). Not a closing note. | `crates/happenstance-core/src/memory.rs:16-31`; UX-AC-07; `project.md` risk table, final row |
| **Anchored citation form** | Every citation is **named subject + path**; a line range is a convenience, never the only handle. An insertion above a target must not silently mis-point the page. | AC-A07; `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`; `xtask/src/spec_trace.rs` |
| **Cites, never restates** | ADR-0001 and ADR-0008 are cited for *why*, reached through `.kb/maps/decision-map.md` so a superseded atom is never cited. The page is not a third statement of the two-flavour rule beside `standards/rust/20-…` and `25-…`. Normative claims are resolving citations into `spec/SPECIFICATION.md`; no clause is restated. The precedence chain at `standards/rust/README.md:25-29` is obeyed and not extended. | DR-7, DR-8; `_decomposition.md` N-5 |
| **One named answered need, declared on the page** | *"In what order do I read what already exists, to build an adapter."* Singular. HS-P0021's rule is **obeyed, not authored**; where it has not landed, author to the initiative's stated shape and expect a conformance pass. | `_design.md`, `## Composition` region 1; `.bklg/docs-that-teach/page-need-discipline/project.md:224-226` (AC-006) |
| **Volume is bounded and the overflow rule is escalation** | ≤ 900 words target, **1,200 hard cap**; ≤ 5 sentences / ~120 words of connective tissue per source. Connective tissue is the only thing that yields — never a citation, never the caveat. Past 1,200 words: escalate to the initiative, do **not** trim the sequence. | `_design.md`, `## Density budget`, "whole page" row |
| **Terminal region: limits plus exactly one onward hop** | States what the page does not cover, then **one** hop — never a list. No "See also" / "Next steps" / "Further reading" block. The hop takes a guarded ladder rung (in-tree markdown link inside the pinned tree, guarded by HS-P0020's registration check); a bare URL into this repository's tree is forbidden outright. | `_design.md`, `## Composition` region 5, rule 2(iv), `## Shape decision` final row; UX invariant 5; anti-pattern 5; N-3 row 3 |
| **Fences: cited, not copied; and never an opt-out** | The `memory.rs` doctest is **cited**, not pasted. If the page carries any Rust fence at all it is compiled by HS-P0020's `REQUIRED` step, and no `ignore`-class fence (or equivalent spelling) is used to escape it. | HS-P0020 AC-002 and AC-004 (`checked-documentation-surface/project.md:204-214`) |
| **Surface hygiene** | No widget, no raw HTML, no inline `style=`, no fold/tab/accordion/`<details>`; heading ladder unskipped; link text is a self-describing noun phrase of ≥ 3 words — never "here", "this", "docs", or a bare URL. All six sources are visible, none behind a fold. | `_storymap.md` standing constraints; UX accessibility floor; `_design.md`, `## Transience policy` and anti-patterns 3, 9, 10 |
| **No public API surface is touched** | No item, signature, feature or manifest change. The two `#[doc(alias)]` keys `_design.md` sanctions are attributes on `EventStore`/`SendEventStore` in `store.rs` and belong to the slice-mate, not to this page. | `_design.md`, `## Items`; `_decomposition.md` N-1 |
| **B1 independence** | Nothing on this page may be load-bearing for getting unstuck. Deleting this page must leave `store.rs` still able to resolve E0034 in place. | UX brief, interaction-quality invariant 1 |

## Data and migrations

**N/A — this story ships prose, and it adds no schema, no runtime data and no migration.** No table, no
serialized type, no persisted state, no manifest entry; the page is text inside a pinned documentation
tree, and the project changes no public item at all (`_design.md`, `## Items`).

Two data-shaped things exist nearby and neither is this story's to write, recorded here so their absence is
deliberate rather than an oversight:

- **The pointer register** is build-time data — a `const` list in the shape of `SUMMARIES` at
  `xtask/src/lint_constitution.rs:64`, consumed by a checker at `:463-473`, and **never rendered as a
  page** (`_design.md`, `## Transience policy`, final row). It is authored by `pointer-policy-and-inventory`
  and consumed here. This story's single onward hop takes a form whose guard already exists (N-3 row 3), so
  it needs no new mechanism; whether it earns a register row is determined by the policy this story
  consumes, not decided here.
- **The pinned-tree path constant** in `xtask/src/` is HS-P0020's (N-1, N-4). This story registers a page
  in that tree; it does not add, move or fork the constant.

## Acceptance criteria

Each criterion is a persona goal crossing the whole slice — the adapter author (Persona 2) at **B2**
(*"tell the difference between 'the port is wrong for me' and 'I have not understood the port yet'"*,
`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:161-165`) and **B3** (building a
model of what an adapter is shaped like) — not a bare capability. Ten criteria; project AC-008 is carried by
AC-001..AC-005 and AC-007..AC-010, project AC-012 by AC-006 and AC-007.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author at B2 who has just been unblocked in `crates/happenstance-core/src/store.rs` and now wants the reasoning, **WHEN** they take the single hop the slice installs, **THEN** they land on a page that is *registered* in HS-P0020's pinned narrative tree — reachable by that tree's own no-orphan mechanism and compiled by its `REQUIRED` step — and **not** a loose file under `docs/`, a new `//!` module doc, or a section appended to `CONTRIBUTING.md`. If the tree or its registration path has not landed, the story **blocks**; an unregistered page is not a mounted page. | HS-P0020's no-orphan step running inside `cargo xtask ci --fast` (`xtask/src/main.rs`; the step's contract at `:85-102`, `probe: None` = mandatory). The registration file and line, plus the gate transcript, recorded in `.bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md`. Falsified by deleting the registration entry and finding the gate still green. |
| AC-002 | **GIVEN** a reader meeting the page for the first time at 1024x768, **WHEN** it renders, **THEN** the five regions appear in the bound order (H1 + answered-need · the numbered reading order · the caveat at entry 1 · six positioned sections · the terminal region), and the **complete six-entry reading order is visible before the first section, inside the first screen** — H1 + need (3) + list (≤ 12) = **≤ 15 rendered lines** against a first screen of **≈ 27** — so the reader learns the sequence from an enumeration rather than from scroll position. The list is the page's only enumerated element, it precedes everything including the first section, and it never loses an entry to fit. | Rendered-line count at 1024x768 recorded in `_verification.md` against the numbers in `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` `## Density budget` ("`adapter-reasoning-account`, first screen" row); region order checked against `_design.md` `## Composition`; visual check against `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` (`#all`, the `adapter-reasoning-account` frame group at 1024x768). |
| AC-003 | **GIVEN** the adapter author at B3 trying to build a model of what an adapter is shaped like, **WHEN** they read the page end to end, **THEN** they meet exactly the six decided sources in the decided order — (1) `crates/happenstance-core/src/memory.rs` (the reference-implementation doc and its full-DCB-loop doctest), (2) `standards/rust/91-adapter-authoring-recipe.md` (RS-91-1..4, the compiled `PgStore`), (3) `CONTRIBUTING.md` `## Writing an adapter` (`:69`), (4) `standards/rust/20-two-flavour-ports.md`, (5) `standards/rust/25-what-removes-send-and-sync.md`, (6) `CONTRIBUTING.md` `## Adding a method to a port` (`:97`, rule at `:104`) — each reached by an **anchored named-subject + path** citation with any line range as a convenience rather than the only handle, and **none of the six copied onto the page** (the `memory.rs` doctest is cited, never pasted). | `rg` assertions recorded in `_verification.md`: each of the six paths appears exactly once as a citation, in document order; each citation carries a quoted subject string that `rg`-matches in the cited file. Sixth entry's heading string asserted as `Adding a method to a port` and **not** "Provided methods" (EC-006). Citation form checked against `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`. |
| AC-004 | **GIVEN** a reader who lands mid-sequence — arriving at section 3 from a `store.rs` fragment or a search result, with no memory of having passed a setup — **WHEN** they read the heading they landed on, **THEN** it carries its own position ("3 of 6" or equivalent) so they learn from that heading alone that two sections precede them, without scrolling up. **No section heading is unpositioned.** | `rg` over the page: six section headings, six position markers, one per heading; recorded in `_verification.md`. Design anti-pattern 17 (`_design.md`, `## Anti-patterns`) is the named wrong implementation. Falsified by a heading that reads as a bare source name. |
| AC-005 | **GIVEN** a reader who will generalise the first implementation they are shown into "the shape an adapter takes", **WHEN** `MemoryEventStore` is first sequenced at entry 1, **THEN** the page states *in that same place* that it is the conformance suite's oracle and the reference implementation and **not an adapter** — deferring to `memory.rs`'s own three-reasons list at `crates/happenstance-core/src/memory.rs:16-31` rather than paraphrasing it — and **names in the same breath at least one adapter at the other end of the storage-shape axis** (`standards/rust/91-adapter-authoring-recipe.md`'s `PgStore`; `references/adapter-shapes.md`). Not a closing caveat, not a footnote. | Position assertion in `_verification.md`: the caveat's first sentence occurs before the first heading of section 2 and inside entry 1's block; the pairing names a second implementation in the same paragraph. Traced to UX-AC-07 (`_decomposition.md`, UX brief) and `project.md`'s risk table, final row. |
| AC-006 | **GIVEN** a maintainer performing DoD 12's spot check ("no page has become a second specification"), **WHEN** they read every normative claim on the page, **THEN** each is a **citation that resolves**: ADR-0001 (`.kb/decisions/0001-async-port-flavours.md`) and ADR-0008 (`.kb/decisions/0008-one-derivation-for-both-ports.md`) are cited for *why* two flavours exist and are reached **through the supersession graph** in `.kb/maps/decision-map.md` so no superseded atom is cited; every `spec/SPECIFICATION.md` claim is a clause id cited, never a clause restated; and the page is **not a third statement** of the two-flavour rule beside `standards/rust/20-two-flavour-ports.md` and `standards/rust/25-what-removes-send-and-sync.md`. The precedence chain at `standards/rust/README.md:25-29` is obeyed and not extended. | Every cited clause id resolved against `spec/SPECIFICATION.md` and recorded in `_verification.md`; where HS-P0020's AC-007 citation step has landed, `cargo xtask ci --fast` fails on a non-existent clause id and that is the mechanical check. Every cited decision atom checked against `.kb/maps/decision-map.md` for `status: accepted` and no supersession. Restatement check: no sentence on the page asserts a rule the two standards atoms assert. |
| AC-007 | **GIVEN** the one-need review pass a reviewer who did not write the page must be able to run (initiative DoD 8), **WHEN** they read the page's first region, **THEN** it declares **exactly one** answered need — *"in what order do I read what already exists, to build an adapter"* — and the page carries no second need. HS-P0021's rule is **obeyed, not authored**: where it has not landed, the page is authored to the initiative's stated shape and expects a conformance pass. | Declaration present in region 1 and singular, recorded in `_verification.md` with its verbatim text; checked against `.bklg/docs-that-teach/page-need-discipline/project.md:224-226` (its AC-006) once that rule lands, and re-checked at HS-P0021's merge. Falsified by a second "this page also explains…" sentence anywhere on the page. |
| AC-008 | **GIVEN** the project's own finding that *"AC-05 is a sequencing job, not an authoring job"*, **WHEN** the finished page is measured, **THEN** it is **≤ 900 words** (hard cap **1,200**) and no single source's connective tissue exceeds **5 sentences / ~120 words**. **WHEN** the count would exceed 1,200, **THEN** the story escalates to the initiative rather than trimming the sequence — connective tissue is the only thing that ever yields, and **neither a citation nor the caveat is ever what yields**. | Word count of the page file recorded in `_verification.md` (`wc -w`), plus a per-section count against the ~120-word ceiling; both compared to `_design.md` `## Density budget`, "`adapter-reasoning-account`, whole page" row. Overflow path is EC-003, not a quiet trim. |
| AC-009 | **GIVEN** a reader who has finished the page and must not be left at a dead end (UX invariant 5), **WHEN** they reach the terminal region, **THEN** it states what the page does **not** cover and offers **exactly one** onward hop — never a list, and never a "See also" / "Next steps" / "Further reading" block — whose form is a **guarded rung of the href ladder** (an in-tree markdown link inside the pinned tree, guarded by HS-P0020's registration check; or the named-but-unlinked cross-reference form live at `crates/happenstance-core/src/store.rs:77`), whose visible link text is a **self-describing noun phrase of ≥ 3 words** — never "here", "this", "docs", "read more" — and which is **not a bare URL into this repository's own tree**. | `rg` assertions in `_verification.md`: exactly one outbound link in the terminal region; no `https://` targeting this repository; no heading matching `See also|Next steps|Further reading`; link text word count ≥ 3. Rung and guard recorded per the pointer policy landed by `pointer-policy-and-inventory`. Design anti-patterns 3 and 5 are the named wrong implementations. |
| AC-010 | **GIVEN** a keyboard-only reader on a screen reader, **and GIVEN** a second reader who never follows the hop out of `store.rs` at all, **WHEN** each reads what is in front of them, **THEN** (a) the page is composed **only** from the enumerated design-system primitives — a markdown link, an unskipped heading ladder, compiled/uncompiled fences — with **no raw HTML, no inline `style=`, no `<details>`, tab, accordion or fold**, all six sources visible rather than revealed, every hop reachable by keyboard, nothing animating, and nothing carrying meaning in colour or position alone; **and** (b) deleting this page entirely leaves `crates/happenstance-core/src/store.rs` still able to resolve `error[E0034]` in place — nothing on this page is load-bearing for getting unstuck (UX invariant 1). | (a) `rg` assertions in `_verification.md` for `<details>`, `<div`, `style=`, `<table`, and a heading-level ladder check with no skips; keyboard-only pass recorded (not a substitute for `error-site-walk-record`, which is a different pair of hands). (b) The B1-independence check: with the page removed from the working tree, `store.rs`'s `## Import one flavour, not both` section still states the import rule and the fully-qualified escape hatch — recorded as a one-paragraph observation in `_verification.md`. |

## Interaction quality

RFC §6.7/D6. Every invariant below is carried by an `AC-###` **row in the table above** — this section
names which id carries which and how it is verified, and adds no ungated bullet. The medium is rendered
markdown inside a pinned documentation tree; "state" and "composition" both land in checkable places.

**STATE invariants**

| Invariant | Carried by | How it is checked |
| --- | --- | --- |
| **In-place before context-jump** — the hop out of `store.rs` is an offer, never the route to being unstuck | **AC-010(b)** | Delete the page; `store.rs` must still resolve E0034 in place (UX brief invariant 1's own falsification) |
| **Non-occlusion** — the orienting element is never pushed off-screen by what it orients | **AC-002** | The six-entry reading order stays complete and above the first section within ≤ 15 rendered lines of a ≈ 27-line first screen; nothing is folded or dropped to make room |
| **Preserved position on mid-sequence entry** — a reader who lands on a fragment does not lose their place | **AC-004** | Every section heading carries "n of 6"; falsified by entering at section 3 and being unable to tell that two precede it |
| **Reversibility** — every hop is walk-backable without browser history | **AC-004**, **AC-009** | Position markers say where the reader is; the terminal region states what the page assumes and does not cover, so a reader can tell whether they arrived out of order |
| **Keyboard reachability** — every reach is a plain link or a renderer-native affordance | **AC-009**, **AC-010(a)** | No bespoke control exists to be unreachable; the keyboard-only pass is recorded, and the *observed* walk stays `error-site-walk-record`'s |
| **Selection and copy survive** — cited material is compared, not retyped | **AC-003** | The `memory.rs` doctest is cited rather than pasted, so there is no second copy for a reader to diff against a drifting original |

**COMPOSITION invariants** — from `_design.md`, signed off 2026-08-17, binding on this surface.

| Invariant | Carried by | How it is checked |
| --- | --- | --- |
| **Presentation exists at all** — every element is real composed presentation from the enumerated primitive set, nothing hand-rolled | **AC-010(a)** | `_decomposition.md`, UX brief, "Design-system primitives — compose these, hand-roll nothing"; `rg` for raw HTML / inline style |
| **Composition and placement** — the five regions render in the bound order | **AC-002** | `_design.md` `## Composition`, `adapter-reasoning-account` table; no region reordered or omitted |
| **Transience** — all six sources are persistent chrome; this project installs **zero** revealed or opened-on-demand content | **AC-010(a)** | `_design.md` `## Transience policy`, "all six sources in the reasoning account" row; anti-pattern 9 |
| **Density budget, with its real numbers** — ≤ 15 rendered lines to the end of the reading order; ≤ 900 words target / **1,200 hard cap**; ≤ 5 sentences / ~120 words of connective tissue per source | **AC-002**, **AC-008** | `_design.md` `## Density budget`, the two `adapter-reasoning-account` rows; overflow escalates (EC-003) rather than trimming the sequence |
| **Hierarchy** — the numbered reading order is primary, the six positioned headings secondary, connective tissue recessive; the distinction is carried by **enumeration and precedence**, not by weight | **AC-002**, **AC-004** | `_design.md` `## Hierarchy`, `adapter-reasoning-account` row: the list is the page's only enumerated element and precedes everything |
| **Named anti-patterns 3, 5, 9, 10, 17** — "here"/bare-URL link text; a "See also" block; any fold; raw HTML or inline style; an unpositioned section heading | **AC-009** (3, 5), **AC-010(a)** (9, 10), **AC-004** (17) | `_design.md` `## Anti-patterns`; each is screenshot- or `rg`-checkable by someone who cannot read Rust |
| **States the surface must express** — `first-screen-reading-order`, `entered-mid-sequence`, `memory-caveat-in-position`, `terminal-section-onward-hop` | **AC-002**, **AC-004**, **AC-005**, **AC-009** respectively | `_design.md` `## Surfaces`, the `adapter-reasoning-account` entry. Final capture is gated on `route` being bound — design finding **F5**, *tracked, not closed* |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | HS-P0020's pinned narrative tree, or its registration path, has not landed when implementation starts | **Block the story.** Do not ship a loose file under `docs/`, do not append to `CONTRIBUTING.md`, do not invent a tree path. This is the same structural stance `project.md`'s risk table takes for AC-010 against the clause-id pin, and the reason `_design.md` keeps `route: TBD` (finding F5). Copy may be drafted; the page may not be mounted anywhere else. |
| **EC-002** | HS-P0021's page-need rule lands and forbids six sections on one page | Take the **named fallback**, not an improvised one: split **by phase** — *why two flavours* \| *how to write one* — **never by source**, and region 2 (the reading order) stays whole on the first page (`_design.md` `## Composition`; `_decomposition.md` N-11). |
| **EC-003** | The finished page exceeds the 1,200-word hard cap | **Escalate to the initiative. Do not trim the sequence.** Yield connective tissue only; never a citation, never the caveat, never an entry in the reading order. Past 1,200 words the page has become an authoring job and project AC-008 has failed — that is a finding to report, not a number to massage (`_design.md` `## Density budget`). |
| **EC-004** | Only the unguarded fourth rung of the href ladder is available for the single onward hop | Take rung 3 — the **named-but-unlinked cross-reference** in the form live at `crates/happenstance-core/src/store.rs:77`, naming the destination and saying in one sentence why it is named rather than linked. A **bare URL into this repository's own tree is forbidden outright** to this project (`_design.md`, rule 2(iv) and `## Shape decision`, final row). If no destination exists at all, record the terminal region's limit statement without a hop and escalate; a placeholder href is the dead end invariant 5 forbids. |
| **EC-005** | A cited source moves or is renamed so its anchored subject string no longer resolves | The anchored form is what makes this *loud* rather than silent: re-anchor on the named subject and re-verify the path. **Never** re-point by line number alone — that is exactly the failure `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` exists for, and `xtask/src/spec_trace.rs` implements the declining-rather-than-guessing behaviour to copy. |
| **EC-006** | The sixth source is cited as a `CONTRIBUTING.md` section called "Provided methods" | **Wrong, and already made once.** No such heading exists; the real one is `## Adding a method to a port` (`CONTRIBUTING.md:97`) and the rule cited sits at `:104`. `_design.md`'s closing citation correction records the mistake so it is not re-made — treat a recurrence as a regression, not a typo. |
| **EC-007** | The page needs a Rust fence that will not compile against the real crates | **Cite, do not paste.** The `memory.rs` walk-through is already a compiled doctest where it lives; a copy on this page is a second thing to keep true. An `ignore`-class fence (or any equivalent spelling) to escape HS-P0020's `REQUIRED` step is forbidden by that project's AC-004 (`.bklg/docs-that-teach/checked-documentation-surface/project.md:212-214`). |
| **EC-008** | A cited decision atom turns out to be superseded | Cite the **standing** atom reached through `.kb/maps/decision-map.md`'s supersession graph, and say nothing about the superseded one. `_design.md`, `## The states the API must express`, "Superseded reasoning", makes this a state the copy must be able to express rather than an accident to discover. |

## Non-functional

| id | requirement | how it is met |
| --- | --- | --- |
| **NF-001** | **Attention budget.** The orienting region costs a reader ≤ 15 rendered lines of a ≈ 27-line first screen at 1024x768; the whole page costs ≤ 900 words (1,200 hard cap). | AC-002 and AC-008. Numbers, not sentiment: `_design.md` `## Density budget` defines the first screen once (1024x768, ~120px chrome, ~24px line box) and every budget is written against the smaller viewport. |
| **NF-002** | **Accessibility floor — WCAG AA in a text medium.** Colour never alone; position and ASCII art never alone; self-describing link text; unskipped heading ladder; nothing animates. | AC-009 and AC-010(a), against `_decomposition.md`, UX brief, "Accessibility floor". The page authors no colour and no spatial encoding, so the obligation here is negative and permanent. |
| **NF-003** | **Narrow viewport.** At 1024x768 nothing this page adds is clipped rather than reflowed: no fixed width, no table used as a navigation device, no column layout. The reading order is a plain numbered list and wraps. | AC-002 and AC-010(a); `_design.md` `## States`, "Narrow viewport" row, and anti-pattern 16 (whose only sanctioned exception — the `error[E0034]` fence's horizontal scrollbar — belongs to the slice-mate, not to this page). |
| **NF-004** | **Theme independence.** The page renders correctly in every theme its renderer ships; no hard-coded colour and no markup that escapes the theme layer. | AC-010(a). This is the concrete form the contrast floor takes here (`_decomposition.md`, UX brief, "Explicitly forbidden compositions", first bullet). |
| **NF-005** | **Maintenance cost is bounded and additive to nothing.** This story adds **zero** gate steps, **zero** checkers, **zero** `xtask/src/` constants and at most **one** pointer register row. | The mount is *registration in a tree the sibling pinned*, not a new mechanism (N-1, N-4). The register is capped at 8 rows at this project's close and 20 ever (`_design.md` `## Density budget`, final row); this page's single hop consumes at most one. |
| **NF-006** | **Citation durability.** An insertion above any cited target must not silently mis-point the page. | AC-003's anchored named-subject + path form, per AC-A07 and `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`. Six of the page's citations point into files under active edit, one of them (`store.rs`) by this story's own slice-mate. |
| **NF-007** | **Honest claim scope.** The page claims to *sequence* existing material and nothing more. It does not claim the sequence has been read by a stranger. | Project `AC-008` is sequencing; whether the sequenced account actually carries an adapter author is HS-P0024's instrument to falsify, dispositioned there and not absorbed here (`project.md`, risk table, "AC-008 may turn out to be insufficient"). |

## Implementation notes (non-prescriptive)

Shape, not prescription. Everything binding is above or in `_design.md`.

- **Bind the mount before writing a word of the page.** Read HS-P0020's landed tree: find the path constant
  in `xtask/src/`, find the registration file its no-orphan mechanism reads, and substitute the concrete
  root into the PR boundary's first glob. If either is absent, stop and raise EC-001 — the drafting is
  cheap to redo, an unmounted page is a false green.
- **Write the caveat first, then the reading order, then the sections.** The caveat (AC-005) is the one
  piece of content that is *not* connective tissue, so drafting it first stops it being squeezed by the
  word budget later. Defer to `memory.rs`'s own three-reasons list (`crates/happenstance-core/src/memory.rs:16-31`)
  — quoting its structure is cheaper and truer than re-arguing it.
- **Draft each section against a word counter, not against a feeling.** ~120 words per source is about one
  short paragraph. If a section wants a second paragraph, the likely cause is that it is explaining rather
  than sequencing — the fix is a sharper "why it is at this position" sentence, not a longer one.
- **Write every citation twice-checkable.** Subject string first (a short phrase that `rg` finds in the
  target), path second, line range last and optional. Run the `rg` for each subject string as you write it;
  a citation that does not resolve at authoring time will not resolve later either.
- **Position markers are cheap to get wrong and cheap to check.** Six headings, six markers, and the numbers
  must agree with the reading order's numbering. `rg` both lists and diff them before review.
- **The onward hop is a policy consumption, not a choice.** `pointer-policy-and-inventory` lands the
  permitted forms and the href ladder; pick the highest available guarded rung, record the row it earns (if
  it earns one), and do not invent a form. EC-004 is the only branch.
- **Do not write the inbound pointer.** The sentence in `store.rs` that reaches this page is
  `store-error-site-rewrite`'s (`_design.md` `## Signatures`, the P3 block). This story's obligation is only
  that the page exists at a path that sentence can name — coordinate the path, author nothing in `store.rs`.
- **Author the `_verification.md` companion as you go**, not at the end. Most of this spec's verification is
  a recorded observation plus an `rg` one-liner; collected at the end they become a retrofit, collected as
  you write they are the evidence the ledger needs.
- **Expect a conformance pass, and leave room for it.** HS-P0021's one-need rule is not written yet
  (AC-007). Declaring the need in one clearly delimited line rather than woven through a paragraph is what
  makes a later mechanical check cheap instead of a rewrite.

## Tests and CI (merge gate)

This project takes **no testing brief** by decision — its walks are observed, not automated
(`project.md`, "Out of scope", final row; `.bklg/docs-that-teach/_decomposition.md`, warranted briefs). The
merge gate is therefore the repository's own gate plus HS-P0020's steps plus a recorded, reviewer-repeatable
verification pass. Every row below is a real command or a real path.

| tier | command / path | proves |
| --- | --- | --- |
| **Gate (required)** | `cargo xtask ci --fast` — defined once in `xtask/src/main.rs` and wired as this project's non-terminal bar (`project.md`, DoD 1) | The merged result is green with the page in the tree: fmt, clippy `-D warnings`, tests, the four wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build. A page that breaks any of them is not merged. |
| **Mount** | HS-P0020's no-orphan registration step, running inside the same `cargo xtask ci` invocation (`.bklg/docs-that-teach/checked-documentation-surface/project.md:215-216`) | **AC-001.** A page added to the pinned tree but not reachable by the compiling mechanism fails the gate rather than shipping unchecked. This is the mount; if the step does not exist yet, EC-001 blocks the story. |
| **Fence compile** | HS-P0020's `REQUIRED` step, `probe: None` (`xtask/src/main.rs:85-102` is the probe contract; its AC-002 and AC-004 at `checked-documentation-surface/project.md:204-214`) | **AC-003, EC-007.** Any Rust fence on the page compiles against the real crates, and no `ignore`-class fence escapes it. The page's preferred state is *no fence at all* — it cites the `memory.rs` doctest instead. |
| **Citation resolution** | `cargo xtask spec-trace` (`xtask/src/spec_trace.rs`), plus HS-P0020's AC-007 clause-id check once landed | **AC-006.** Normative claims are clause ids that resolve. `spec-trace`'s windowed-subject search is also the implementation this page's citation form imitates (`:377-382`, `:2103-2130`). |
| **Text assertions** | `rg` one-liners recorded in `.bklg/docs-that-teach/reach-and-adapter-path/adapter-reasoning-account/_verification.md` | **AC-003, AC-004, AC-005, AC-009, AC-010(a).** Six sources in order; six position markers; the caveat inside entry 1 and paired; exactly one outbound hop and no "See also" heading; no `<details>`, `<div`, `style=`, no bare in-tree URL, no skipped heading level. |
| **Density** | `wc -w` over the page file plus a per-section count, recorded in `_verification.md` against `_design.md` `## Density budget` | **AC-002, AC-008.** ≤ 900 words target / 1,200 hard cap; ≤ ~120 words of connective tissue per source; ≤ 15 rendered lines to the end of the reading order. |
| **Composition (perceptual)** | Manual capture of the `adapter-reasoning-account` frame group from `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html#all` at 1440x900 and 1024x768, filed at `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/adapter-reasoning-account@1024x768.png` | **AC-002, AC-004, AC-005, AC-009.** Region order, first-screen occupancy, caveat placement and hop placement against the signed-off design. **Automated capture is a standing skip** — `design.capture` is absent from `.redkiln/config.yaml` — and final capture is gated on `route` being bound (design finding **F5**, tracked not closed). |
| **B1 independence** | The deletion check recorded in `_verification.md`: remove the page from the working tree and re-read `crates/happenstance-core/src/store.rs`'s `## Import one flavour, not both` section | **AC-010(b).** UX invariant 1's own stated falsification, run rather than asserted. |
| **Story gate** | `redkiln verify --grain story` | The PR boundary holds (no file changed outside the two globs) and every `AC-###` in `_ledger.md` is `satisfied: true` with non-placeholder evidence before `implement → report`. |

## Risks and coupling (PR-scoped)

| Risk / coupling | Note, and what this PR does about it |
| --- | --- |
| **The mount does not exist on this branch** | HS-P0020 is the hard external gate: no pinned tree, no registration path, no `REQUIRED` step yet. The response is EC-001 — **block** — not a loose file. `_design.md` records this as finding **F5** and keeps `route: TBD` deliberately; the PR boundary's first glob is an explicit placeholder for exactly this reason. |
| **HS-P0021's rule is authored after this page** | The page will have a rule retro-fitted to it (AC-007). Mitigated by declaring the single need in a delimited, mechanically-findable line and by authoring to the initiative's stated shape — **not** by inventing the rule here (N-8, final paragraph). A conformance pass at HS-P0021's merge is expected work, not a defect. |
| **Slice coupling with `store-error-site-rewrite`** | The slice-mate's pointer must name this page's path, and this page must exist first (merge order step 2). The coupling is a *path agreement*, and it is the only one: this story writes nothing in `store.rs`, and the slice-mate writes nothing on this page. Both are implemented in one context precisely so neither ships pointing at nothing. |
| **Six citations into files under active edit** | Two point into `CONTRIBUTING.md` and one into `store.rs`, which the slice-mate is rewriting in the same PR pair. The anchored named-subject form (AC-003, NF-006) is the mitigation; a bare line range would be stale before the slice merged. |
| **The word budget and the six-source sequence pull against each other** | Six sources × ~120 words + framing ≈ 850 against a 900 target leaves ~50 words of slack. The resolution is fixed in advance so it is not re-decided under pressure: connective tissue yields, then the story escalates (EC-003). A dropped source or a dropped citation is a spec violation, not a trade. |
| **`MemoryEventStore` as the opening subject** | Sequencing the oracle first is the right reading order and the wrong lesson if unqualified — the monoculture `CLAUDE.md` warns about under "A port is only as well-designed as the *spread* of what implements it". AC-005 is the whole mitigation, and its position (entry 1, paired in the same breath) is what makes it work; moved to a closing note it fails. |
| **The account may prove insufficient** | Sequencing may not be enough to carry an adapter author. That verdict belongs to HS-P0024's friction log and is dispositioned there — this story must not pre-emptively grow into the chapter-length account the decomposition already scoped out (`project.md`, risk table, "AC-008 may turn out to be insufficient"; NF-007). |
| **No automated check for reachability or link quality** | Nothing in `cargo xtask ci` verifies keyboard-only reachability or self-describing link text (`_design.md` `## Density budget`, "Two named gaps", second). Stated rather than papered over: AC-009 and AC-010(a) are `rg`-checkable proxies, and the *observed* walk is `error-site-walk-record`'s, deliberately a different pair of hands. |

## Dependencies

**Blocks on** (must merge first):

- **`pointer-policy-and-inventory`** — lands DT-10's resolution as in-tree substrate: the permitted pointer
  forms, the href ladder's guarded rungs and the register with a named guard per row. This story's single
  onward hop (AC-009) consumes that policy; it does not choose a pointer shape case by case
  (`_storymap.md`, merge order step 1).

**Hard external gate** (not a story, and not satisfiable from inside this project):

- **HS-P0020 `checked-documentation-surface`** — the pinned narrative tree, its registration mechanism and
  its `REQUIRED` compile step. Without them there is nothing to mount into and the story blocks (EC-001).

**Unlocks**:

- **`store-error-site-rewrite`** — its pointer's destination is this page; it is authored second inside the
  same `adapter-error-site` slice (`_storymap.md`, merge order step 2).
- **`error-site-walk-record`** — the observed walk needs both halves of the slice merged before a walker
  who did not write either can reproduce E0034 and reach this page.

## Anchors (progressive disclosure)

Load-bearing depth, deferred not optional. The Context pack above is self-sufficient to start; open these at
the moment named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` | The signed-off, binding design for this surface: `## Composition` (the five regions), `## Density budget` (the two `adapter-reasoning-account` rows), `## Hierarchy`, `## States`, `## Anti-patterns`, and the F5 disposition that keeps `route: TBD`. It is not re-decided by this story. | Before writing the page's outline, and again before every density or placement judgement. | AC-002, AC-004, AC-005, AC-008, AC-009, AC-010 |
| `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` | The approved static mock. Its `adapter-reasoning-account` frames render composition and density only (no substrate exists yet) — approving them approved reading order, heading ladder, caveat placement and hop placement. | When the outline is drafted and you need to check region order and first-screen occupancy against what was signed off; open `#all` and shoot the frame group for the perceptual row. | AC-002, AC-004, AC-005, AC-009 |
| `crates/happenstance-core/src/memory.rs` | The first sequenced source **and** the source text for the caveat: its own three-reasons list (`:16-31`) is what the page defers to rather than paraphrasing, and its `# Examples` full-DCB-loop doctest (`:16-71`) is what is cited rather than pasted. | Before writing entry 1 and the caveat — the first content authored. | AC-003, AC-005 |
| `standards/rust/91-adapter-authoring-recipe.md` | Entry 2, included **by decision** (the architecture brief asked for it to be decided rather than guessed), and the carrier of the only compiled non-`memory` adapter example in the tree — the `PgStore` the caveat pairs against `MemoryEventStore`. | When writing entry 2, and when writing the caveat's pairing clause. | AC-003, AC-005 |
| `CONTRIBUTING.md` | Entries 3 and 6. The exact headings matter: `## Writing an adapter` (`:69`) and `## Adding a method to a port` (`:97`, rule at `:104`). **There is no "Provided methods" heading** — that mistake has already been made once. | When writing entries 3 and 6, and again when checking citations before review. | AC-003, EC-006 |
| `standards/rust/20-two-flavour-ports.md` and `standards/rust/25-what-removes-send-and-sync.md` | Entries 4 and 5, and the two documents the page must **not** become a third copy of. Reading them is how you tell connective tissue from restatement. | When writing entries 4 and 5, and before the restatement check. | AC-003, AC-006 |
| `.kb/decisions/0001-async-port-flavours.md` and `.kb/decisions/0008-one-derivation-for-both-ports.md` | The two Accepted atoms the page cites for *why* two flavours exist and what one derivation with a provided body owes. Cited, never restated. | When writing the "why" sentences in entries 4, 5 and 6. | AC-006 |
| `.kb/maps/decision-map.md` | The supersession graph. Citing through it is what keeps the page from citing a superseded atom — a state `_design.md` requires the copy to be able to express. | Immediately before finalising any decision citation. | AC-006, EC-008 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | The anchored named-subject + path citation form (AC-A07) and why a bare `file:line` mis-points silently the moment anything is inserted above the target. | Before writing the first citation; the form applies to all six. | AC-003, NF-006 |
| `xtask/src/spec_trace.rs` | The working implementation of that anchoring heuristic — a windowed search for a short subject string that declines rather than guesses (`:377-382`, `:2103-2130`). Read it to pick subject strings that a checker could actually find. | When choosing each citation's subject string, and when handling EC-005. | AC-003, EC-005 |
| `.bklg/docs-that-teach/checked-documentation-surface/project.md` | The mount's contract: AC-005 no-orphan pages (`:215-216`), AC-002 mandatory fence compilation and AC-004 no silent opt-out (`:204-214`). This is what "registered" means and what blocks the story if absent. | First, before any implementation — this is the EC-001 check. | AC-001, EC-007 |
| `xtask/src/main.rs` | The `REQUIRED` step list and the `probe` contract at `:85-102` (`None` means mandatory; "the tool ran and found a problem" is never a skip). Composition root CR-4 — read to understand the mount, **not** to add a step: this project does not own `xtask/src/`. | When confirming the page's fences (if any) are compiled by a mandatory step. | AC-001 |
| `xtask/src/lint_constitution.rs` | The precedent for CR-3: `ATOM_DIR` / `ROUTER` / `HARNESS` at `:54-61` pin a tree by path, and `check_summaries` at `:463-473` fails the build when a surface stops containing its pinned constant. The shape HS-P0020's registration check follows and the register's shape (`SUMMARIES`, `:64`). | When binding the PR boundary's tree glob and when recording the hop's guard. | AC-001, AC-009 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` | The UX brief's interaction-quality invariants (1, 5, 8), the accessibility floor, UX-AC-07 and UX-AC-11; the architecture brief's AC-A03, AC-A07, N-3 (the four pointer forms and their guards), N-5, N-8 (the six sources and the required caveat), N-11. | When an invariant's exact falsification is needed, and when the six-source ordering rationale is questioned. | AC-003, AC-005, AC-006, AC-009, AC-010 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 2's goal verbatim (`:161-165`) and journey steps 2–5 — the B2/B3 states every AC above is framed from, and the measured finding that the explanation is shelved in contributor-facing documents. | When an AC's user intent needs grounding, or when tempted to write for a reader other than the adapter author. | AC-003, AC-005, AC-010 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The staged-disclosure entry and its documented failure mode (a reader "can land past the setup with no signal they missed it") — the evidence AC-004's position markers and AC-002's up-front order exist to answer. | When justifying or reviewing the mid-sequence mitigation. | AC-002, AC-004 |
| `.bklg/docs-that-teach/page-need-discipline/project.md` | HS-P0021's AC-006 (`:224-226`) — every governed page declares exactly one need, in a form both the rendered surface and the lint read. The rule this page is authored *toward* and will be conformance-checked against. | When writing region 1's need declaration, and again at HS-P0021's merge. | AC-007 |
| `references/adapter-shapes.md` | The in-tree material naming what the six skeletons told the type checker — the counter-example surface the caveat pairs `MemoryEventStore` against, alongside `PgStore`. | When writing the caveat's pairing clause, if `PgStore` alone reads thin. | AC-005 |
| `spec/SPECIFICATION.md` | The normative voice the page defers to. Any normative claim is a clause id cited here that resolves — never a clause restated on the page. | Whenever a sentence starts to sound normative. | AC-006 |
| `standards/rust/README.md` | The precedence chain at `:25-29`, which this work sits inside and does not extend. | Before asserting anything about how the standards, the specification and the ADRs rank. | AC-006 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` | The verified sibling-project dependency state — HS-P0020 and HS-P0021 both at `stage: storymap` with no `_design.md` at planning time. Re-verify rather than assume; the state may have moved. | At implementation start, together with the EC-001 check. | AC-001, AC-007 |

## Clarifications resolved during spec

- **The AC set is exactly the ten the first pass enumerated** — `AC-001` … `AC-010`. None added, none
  dropped. The ledger matches them one for one.
- **Every composition invariant became a table row, not a prose bullet.** `redkiln verify` extracts ACs by
  matching a leading `| AC-001 |` cell or an `- AC-001:` bullet, so an invariant written as prose in
  `## Interaction quality` would be ungated and untested. Presentation-exists, transience, density,
  hierarchy and the named anti-patterns are therefore carried by AC-002, AC-004, AC-008, AC-009 and
  AC-010; that section maps ids to invariants and asserts nothing on its own.
- **`AC-010` deliberately carries two clauses.** Surface hygiene (a) and B1 independence (b) are one
  criterion because they share a single falsification pass — the page is read as a reader would meet it,
  then deleted to confirm `store.rs` still stands alone. Splitting them would have produced an eleventh id
  the first pass did not enumerate.
- **Verification is recorded observation plus `rg`, and that is stated rather than dressed up as a test
  suite.** This project takes no testing brief by decision; its walks are observed. The `_verification.md`
  companion named throughout is authored by the implementer inside this story's own directory, which the PR
  boundary already permits.
- **The mount's absence is an explicit block, not a degraded mode.** EC-001 resolves the one question the
  front half left implicit: if HS-P0020 has not landed, this story does not ship a loose file with a note.
  That matches `_design.md`'s F5 disposition (tracked, not closed) and `project.md`'s stance on AC-010.
- **The onward hop's fallback is rung 3, not a bare URL.** EC-004 resolves what "one onward hop" means when
  the ladder's top rungs are unavailable: the named-but-unlinked cross-reference form live at
  `store.rs:77`. A bare URL into this repository's own tree stays forbidden outright.
- **Not re-opened here, and named so the silence is visible:** which six sources and their order, the
  one-page decision and its by-phase fallback, the caveat's position, the density numbers, and the
  one-onward-hop rule. All are `_design.md`'s, signed off 2026-08-17.
