---
item: HS-P0023
stage: design
created: "2026-08-17"
updated: "2026-08-17"
---

# API surface design — Reach and the Adapter Path

The resolved surface for HS-P0023, signed off by a human before any story spec is written.
Everything below is binding on the implementer.

**This project changes no public API, and it is not therefore exempt from this stage.** The
repurposing note in `.redkiln/templates/_design.md` says the surface a `cargo add happenstance`
user meets is this repository's screen. For this project that screen is *literally* the screen:
four rendered reading surfaces, none of them a Rust item. So this file keeps every section the
template asks for — `## Items` says "no public item" with its reason, and the sections that
follow are answered in the medium this project actually ships into — and it adds the
interaction-design sections the stage exists to decide: `## Surfaces`, `## Pattern decision`,
`## Composition`, `## Transience policy`, `## Density budget`, `## Hierarchy`, `## States`.

**The perceptual review is a standing skip.** `design.capture` is deliberately absent from
`.redkiln/config.yaml` (its lines 75-81 say why). Nothing downstream will look at these surfaces
and disagree. **This written resolution is the only record there will ever be**, which is why the
numbers below are numbers and the anti-patterns below are screenshot-checkable.

---

## Surfaces

Machine-read by the capture harness. `route` is the artefact a reviewer opens; for the two rustdoc
surfaces that is the local `cargo doc` output, whose DOM ids were verified against this worktree's
own `target/doc/happenstance/index.html` (`id="main-content"` and `class="docblock"` both present).
`target/doc/happenstance_core/` was **not** built in this worktree at design time — the selector is
the same generator's and the implementer must confirm it after a `cargo doc -p happenstance-core`.

```yaml
- id: crate-root-front-door
  route: target/doc/happenstance/index.html
  selector: "#main-content > .docblock"
  states:
    - first-screen-1440x900
    - first-screen-1024x768
    - pointer-absent-baseline
    - long-destination-name
- id: readme-front-door
  route: crates/happenstance/README.md
  selector: "rendered markdown, whole document"
  states:
    - first-screen-crates-io
    - first-screen-github
    - diff-against-main
    - narrow-viewport
- id: store-module-error-site
  route: target/doc/happenstance_core/store/index.html
  selector: "#main-content > .docblock"
  states:
    - section-import-one-flavour-not-both
    - e0034-fence-full-width
    - e0034-fence-narrow-viewport
    - heading-outline
- id: adapter-reasoning-account
  route: "TBD — a page in HS-P0020's pinned narrative tree; path bound at implementation"
  states:
    - first-screen-reading-order
    - entered-mid-sequence
    - memory-caveat-in-position
    - terminal-section-onward-hop
- id: evaluator-onward-links
  route: "TBD — HS-P0022's answering pages, inside HS-P0020's pinned tree"
  selector: "the passage that answers question one, final sentence"
  states:
    - link-in-passage
    - destination-fragment-landing
    - dead-end-baseline
```

`selector` is deliberately omitted on the two TBD surfaces rather than guessed. HS-P0020 has not
authored `_design.md` and its hosting shape is open (`_grounding.md`, "Sibling-project dependency
state"); inventing a selector for a template that does not exist is the invented-primitive failure
this stage is here to prevent.

---

## Pattern decision

### DT-10 — whether the reference surface points outward once, repeatedly, or contextually

**Owned tension. Resolved: option (c), both, with one authoritative — bound to a rule, not left as
a permission.**

Options (a) and (b) are each falsified by this project's own criteria before any preference is
consulted, which is worth stating because "(c)" is otherwise the answer that looks like indecision:

- **(a) a single pointer from the crate's front door — rejected.** AC-007 requires the reasoning
  account to be reachable *from `store.rs`*, and the reader it serves arrives at
  `happenstance_core::store` from a compiler diagnostic or a search box, never through
  `happenstance`'s crate root. A front-door-only policy is unreachable for the persona the project
  exists to serve. The dossier's own fit note on the built-in surface says the same thing from the
  other side: the evaluator's gap "is a linking problem … which intra-doc links solve directly but
  only if something authors the link"
  (`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, "The ecosystem's
  built-in surface", Fit conditions).
- **(b) pointers at each relevant item — rejected.** AC-002 requires the crate root and the README,
  and neither is an item. More importantly, "each relevant item" has no closing condition, which is
  exactly the recorded price: per-item pointers "become a second navigation surface to keep true"
  (`.bklg/docs-that-teach/initiative.md`, DT-10 row).
- **(c) both, with one authoritative — chosen**, on the three criteria UX-AC-12 names:
  1. *Cost to a reader arriving at a single item out of search.* This is the whole case for
     per-item pointers and it is real here: `pub mod store` renders its own page
     (`crates/happenstance-core/src/lib.rs:99`), so a reader can and does land on the error site
     with the crate root unseen.
  2. *Cost of a second navigation surface going stale.* Paid down by the rule below, which closes
     the set rather than trusting judgement per item, and by keeping the register out of the
     reader's view entirely (see `## Transience policy`).
  3. *Survives the two-surface split either way.* The authoritative sentence names the need and
     not the hosting; only the href is late-bound (UX-AC-02). If HS-P0020 resolves to docs.rs-only,
     the same sentence takes an intra-doc link and gets a stronger guard for free.

**The rule — "one front door, and a pointer only at a stall".** This is the binding half of the
resolution; (c) without it is a permission slip.

1. **Exactly one authoritative text.** One sentence, authored once, mirrored byte-identically onto
   both front-door surfaces. Two renderers, two readers, one authority — *not* two pointers.
2. **A secondary pointer is permitted at an item only where all four hold**, and the register row
   records which:
   - **(i) evidenced stall.** There is a named, recorded reader failure at that item — a compiler
     diagnostic, a measured second question — not a suspicion. BR-15 is the only one this project
     has.
   - **(ii) front door provably cannot reach it.** The item is arrived at by deep link, search or a
     diagnostic.
   - **(iii) subordinate and one line.** It follows the in-place fix, never precedes it, and it
     introduces no heading of its own.
   - **(iv) a guard from the mechanism table.** Its form is one of rows 1–3 of the architecture
     brief's N-3 table. **Row 4 (a bare URL, guarded by nothing) is forbidden to this project
     outright** — AC-003 says no pointer whose only guard is memory, and this rule is what makes
     that checkable rather than aspirational.
3. **The href ladder, applied per pointer at implementation and recorded in its register row:**
   intra-doc link (guarded by `broken_intra_doc_links = "deny"`, `Cargo.toml:134`, across three
   rustdoc builds) **>** an in-tree markdown link inside HS-P0020's pinned tree (guarded by its
   registration check) **>** a named-but-unlinked cross-reference in the form already live at
   `crates/happenstance-core/src/store.rs:77` **>** *nothing*. If only the fourth rung is
   available, the pointer is **not installed** and the project escalates, exactly as it does for
   AC-010 if HS-P0020 lands without a clause-id set.
4. **Aliases are not pointers and need no row.** See the `#[doc(alias)]` decision below.

**Mitigation for the pattern's documented failure mode.** DT-10's recorded cost is the second
navigation surface. Three things pay it down, and all three are checkable:

- the register is **build-time data, never a rendered page** (`## Transience policy`);
- the register is **capped** and the cap is a number (`## Density budget`);
- the register has a **working precedent to copy rather than invent**:
  `xtask/src/lint_constitution.rs:64` declares `SUMMARIES` as a named list of surfaces and
  `check_summaries` at `:463-473` fails the build when a surface stops containing the pinned
  constant, with the failure message carrying the reason. That is the AC-003 inventory's shape.
  **Gap, named not assumed:** no such step exists in `xtask/src/main.rs` today for pointers, and
  this project does not own `xtask/src/` (architecture brief N-1). The concrete ask handed to
  HS-P0020 is in `## Visibility and stability`; the fallback if it never lands is rule 2(iv), which
  needs no new tooling because every permitted form is already guarded.

### Per surface

| Surface | Pattern chosen | Rejected here, and why | Dossier citation | Resolves |
| --- | --- | --- | --- | --- |
| `crate-root-front-door` | **Two-surface split's crate-root pointer**: one sentence of plain prose, above the first `#` heading | *A routing table* — `docs/README.md:12-23` is the repo's working table and it routes to ten destinations; a one-row table is a widget (DR-5). *A blockquote callout* — would out-weight the status disclaimer that follows it. *A new `# Guide` heading* — a one-sentence section in the page outline inverts the hierarchy | "Two-surface split", lines 86-97: tokio's crate root states verbatim "Guide level documentation is found on the website"; serde and diesel converge | DT-10 (a-half) |
| `readme-front-door` | **The same sentence, mirrored**, as its own paragraph between the description and the status callout | *Inside the existing blockquote* — it is HS-P0016's claim-bearing block and DoD-7 makes the seam a diff. *After "Which crate do I want?"* — a reader who has decided skips that section. *A second blockquote* — the file has exactly one admonition and a second halves the first | Same entry; and the axum counter-case at lines 99-104, whose cost is "no ordering signal" for a newcomer | DT-10 (a-half) |
| `store-module-error-site` | **Contextual pointer at an evidenced stall**, recessive, last sentence of the existing `## Import one flavour, not both` section, after the in-place fix | *A new `## Where the reasoning lives` heading* — a fifth ladder entry whose body is one sentence, and equal outline weight to "Why there are two traits". *A pointer above the fix* — fails invariant 1 outright. *An intra-doc link* — the destination is not an item; `store.rs:76-78` already records why this file cannot link out to a feature-gated item | "The ecosystem's built-in surface", Fit conditions (lines 57-64): strong fit at this fanout, but the tool "doesn't supply the link on its own" | DT-10 (b-half, bounded by rule 2) |
| `adapter-reasoning-account` | **Staged disclosure, with the sequence stated up front** — a numbered reading order before the first section, then one section per source | *Progressive/hierarchical disclosure* — a menu of optional depth destroys the one thing the page is (an order). *Tabs or folds* — DT-7 is HS-P0020's and this project installs none. *Diátaxis's four boxes* — six sequenced sources do not sort into four | "Staged (worked-example) disclosure", lines 144-157 (Carroll's replicated training-wheels finding); its failure mode at "Documented failure modes" — a reader "can land past the setup with no signal they missed it" | — (shape follows from DT-10's rule 2(iii), not a tension of its own) |
| `evaluator-onward-links` | **Inline link at the end of the answering passage** | *A "See also" block at page bottom* — reached only after the reader has decided to leave, and it is a navigation surface (DR-5). *A sidebar or breadcrumb* — the medium already renders both | "Breadcrumb / master-detail navigation", Fit conditions (lines 382-390): the gap "is evidence of a missing *link*, not a missing *widget*" | DT-10 (b-half) |

**Staged disclosure's failure mode, mitigated.** The dossier's cited failure is a reader arriving
mid-sequence with no signal that setup preceded them. Mitigation, binding: the reading order is a
numbered list **before** the first section and inside the first screen (see `## Density budget`),
and **each section's heading carries its own position** — "3 of 6" or equivalent — so a reader who
lands on a fragment from `store.rs` or a search result learns their position from the heading they
landed on, not from scrolling up.

### `#[doc(alias = "…")]` — considered, and **used**, narrowly

DR-5 and UX-AC-10 require the built-in surface be exhausted before anything else is proposed, and
`#[doc(alias)]` is the unexhausted part: **zero occurrences in `crates/`, `standards/` or `xtask/`**
(re-verified). This design establishes the convention rather than reusing one, and says so.

**Rule.** An alias is permitted only where the string a reader searches is one that **rustc, the
specification, or a recorded reader question actually emits**, and is not the item's own name or a
substring of it. Two strings qualify today, both from the diagnostic in BR-15:

- `#[doc(alias = "E0034")]` on `EventStore` and on `SendEventStore` — the code the reader pastes.
- `#[doc(alias = "TraitVariantBlanketType")]` on `SendEventStore` — the string in rustc's candidate
  notes, which currently returns nothing anywhere on docs.rs for this crate.

**Rejected**: synonym farming (`eventstore`, `es`, `event-store`), aliases for discoverability of
concepts rather than strings, and aliases as a substitute for the pointer at the stall. An alias
moves a reader who is already searching; it does nothing for the reader who is reading.

**Why no register row.** An alias is an attribute on an item, so deleting the item deletes the
alias — it cannot rot independently (architecture brief N-3). **Residual risk, named:** the alias
*string* can go stale if rustc renames the internal type, and nothing catches that. The cheap
mitigation is a two-line `rg` assertion that every `doc(alias)` string introduced by this project
also appears verbatim in `store.rs`'s `text` fence. **That check does not exist and this project
does not own `xtask/src/`** — it is offered to HS-P0020 alongside the register hook and is not
counted as a guard until it lands.

---

## Composition

Regions in render order, per surface. Where a region already exists, its line numbers are this
branch's and are the non-occlusion baseline (invariant 2).

### `crate-root-front-door` — `crates/happenstance/src/lib.rs`

| # | Region | Occupant | Change |
| --- | --- | --- | --- |
| 1 | pre-heading blurb, line 1 | `//! DCB-compliant event sourcing, with batteries.` (`:11`) | untouched |
| 2 | pre-heading blurb, line 2 | **the front-door pointer, one sentence** | **added — 2 source lines (one blank, one sentence)** |
| 3 | first section | `# Status: a facade over [happenstance_core]` (`:13-25`) | untouched, shifted down 2 |
| 4 | second section | `# What arrives here, and what stays below` (`:27-51`) | untouched, shifted down 2 |
| 5 | third section | `# Using it today` + the compiled fence (`:53-67`) | untouched, shifted down 2 |
| 6 | closing paragraph | the adapter-author routing sentence (`:69-71`) | untouched — and the adapter pointer is **forbidden** here (ADR-0006's own conclusion is stated at `:20-25`; installing it would contradict the page) |

**Region 2 sits above region 3 deliberately.** rustdoc renders pre-heading prose as the item's top
blurb, which is what docs.rs shows in search results and what a reader sees without scrolling. And
the status callout is a *disclaimer*: a reader who bounces off "early, and a facade" must already
have been told the guide exists.

### `readme-front-door` — `crates/happenstance/README.md`

| # | Region | Occupant | Change |
| --- | --- | --- | --- |
| 1 | title + description | `# happenstance` and the DCB sentence (`:1-4`) | untouched |
| 2 | **the pointer** | **the same sentence as region 2 above, byte-identical rendered text** | **added — 2 source lines** |
| 3 | status callout | `> **Status: early, and this crate is currently a facade.**` (`:6-11`) | untouched — HS-P0016's |
| 4 | crate routing | `## Which crate do I want?` (`:13-20`) | untouched, shifted down 2 |
| 5 | concept + compiled fence | `## What DCB buys you` (`:22-39`); the fence opens at `:30` → `:32` | untouched, shifted down 2 |
| 6 | tail | Guarantees / Design / Licence (`:41-60`) | untouched, shifted down 2 |

The pointer sits **above** the status callout for the same reason as on the crate root, and
**outside** it because region 3 is a claim and this project changes no claim (DoD-7).

### `store-module-error-site` — `crates/happenstance-core/src/store.rs`

The heading ladder stays at exactly four entries: `# Why there are two traits` (`:3`),
`## What that means in practice` (`:21`), `## Import one flavour, not both` (`:31`), `## Naming`
(`:47`). Every change lands **inside** the third section, in this order, and the order is binding:

| # | Element | Status | Note |
| --- | --- | --- | --- |
| a | the one-sentence cause | existing (`:33-34`) | untouched |
| b | the `text` fence | **rewritten** (`:36-41`, 6 lines → ~12) | rustc's real output at the pinned 1.97.1 toolchain, including both `= note:` candidate lines and `TraitVariantBlanketType` |
| c | **plain-words naming of the ambiguity** | **added, ~3 lines** | names the method (`read`), names both traits, and states which call is ambiguous — because the caret line `^^^^` is spatial and linearises to nothing for a screen reader or a plain-text search hit |
| d | the in-place fix | existing (`:43-45`) | untouched in substance — import one flavour; the fully-qualified escape hatch. **Nothing may be inserted between (c) and (d)** |
| e | **the narrow unchecked-limit sentence** | **added, ~3 lines** | states that the fence compiles nothing, *and* that the error code itself is asserted by `standards/rust/20-two-flavour-ports.md:179` and `standards/rust/00-prime-directives.md:242` (both `rust,compile_fail,E0034`, both compiled by the constitution step) |
| f | **the pointer to the reasoning account** | **added, ~2 lines** | last sentence of the section, no heading, recessive |
| g | `## Naming` | existing (`:47-52`) | untouched |

**(f) is last and (d) is before it** because invariant 1 is the whole shape of this page: a reader
who never follows the link must still be correctly unstuck.

### `adapter-reasoning-account` — one page in HS-P0020's pinned tree

| # | Region | Occupant |
| --- | --- | --- |
| 1 | H1 + answered-need | one line naming the single need — *"in what order do I read what already exists, to build an adapter"* — under HS-P0021's discipline, obeyed not authored |
| 2 | **the reading order** | a numbered list of the six sources, each ≤ 2 rendered lines: what it is, why it is at that position, and the anchored citation. **Before the first section, inside the first screen.** |
| 3 | the caveat, in position | at entry 1, where `crates/happenstance-core/src/memory.rs:16-71` is first sequenced: it is the conformance suite's oracle and the reference implementation, **not an adapter** (`memory.rs:16-31`) — paired in the same breath with a named adapter at the other end of the axis (`standards/rust/91-adapter-authoring-recipe.md`'s `PgStore`; `references/adapter-shapes.md`) |
| 4 | six sections | one per source, in sequence, each heading carrying its position ("3 of 6"), each body 2–5 sentences of connective tissue plus the citation. No restatement (DR-7, DR-8) |
| 5 | terminal region | what this page does not cover, and **one** onward hop — never a list of them (invariant 5) |

**One page, not two** (the architecture brief N-11 leaves this open; it is bound here). The page's
single answered need *is* the order; splitting it splits the content. **Named fallback:** if
HS-P0021's rule lands and forbids six sections on one page, the split is by *phase* — why two
flavours | how to write one — never by source, and region 2 stays whole on the first page.

**Which six, and the decision the brief asked for explicitly.**
`standards/rust/91-adapter-authoring-recipe.md` **is included** as entry 2: it is a directly
on-topic atom carrying the only compiled non-`memory` adapter example in the tree, and omitting it
from a page whose entire premise is findability-over-invention would be the odd choice.

### `evaluator-onward-links` — HS-P0022's pages

One inline link, sentence-final, at the end of the passage that answered the first question. No
block, no heading, no list. The link text names the destination *and* the question it answers, and
targets a heading fragment where the answer is not the destination's first screen (UX-AC-04).

---

## Transience policy

In this medium the three policies read as: **always-rendered prose** (persistent chrome),
**renderer-owned affordances the reader opens** (opened-on-demand), and
**revealed-on-hover-or-focus** — of which this project installs **zero**, which is a decision and
not an absence.

| Control | Policy | Reason |
| --- | --- | --- |
| the front-door pointer (both surfaces) | **persistent chrome**, above the first heading | It is the only thing that makes the narrative surface exist for a reader who never scrolls. A pointer that must be revealed is the gap restated. |
| the README status callout | **persistent chrome**, unchanged | HS-P0016's block; not ours to fold or fill. |
| the `error[E0034]` fence | **persistent chrome**, never collapsed | Two reasons and either is sufficient: hiding a load-bearing constraint behind a fold is the dossier's second anti-pattern; and the fence *is* the searched string — a folded fence may not be in Ctrl-F. |
| the plain-words ambiguity sentence | **persistent chrome** | It is the non-spatial carrier of the diagnosis. If it can be hidden, the caret line is alone again. |
| the in-place fix | **persistent chrome** | Invariant 1. |
| the unchecked-limit sentence | **persistent chrome** | A caveat that appears only on demand is a caveat nobody reads. |
| the `store.rs` pointer | **persistent chrome**, recessive | One line. Hiding it costs a reveal and gains nothing; recessive placement already gives it the weight it should have. |
| rustdoc's per-item `[+]`/`[-]` collapse | **opened-on-demand, renderer-owned** — neither enabled nor defeated | We do not configure it. The implementer must confirm the module doc is not rendered inside a collapsed item's body, or (b) and (f) are behind a fold we did not choose. |
| rustdoc search and the two `doc(alias)` keys | **opened-on-demand, renderer-owned** | Search is the reader's move. Our contribution is the key, not the overlay. |
| the destination surface's TOC | **renderer-owned, out of scope** | HS-P0020's. This project adds no navigation of its own on top of it (DR-5). |
| all six sources in the reasoning account | **persistent chrome, all six visible** — no fold, no tab, no accordion | DT-7 belongs to HS-P0020 and this project installs none. |
| **the pointer register** | **not rendered to a reader at all** — build-time data only | The sharpest transience call here. Rendering the register creates precisely the second navigation surface DT-10 warns about: a page listing every pointer is a navigation surface whose only reader is its maintainer. It is a `const` list consumed by a checker, in the shape of `SUMMARIES` at `xtask/src/lint_constitution.rs:64`. |

---

## Density budget

Real numbers. The unit this medium gives a reviewer is the **rendered line** and the **source
line**; both are countable from a screenshot and a diff.

**First screen, defined once.** At 1024x768 with browser chrome ≈ 120px and rustdoc's ~24px line
box, the first screen is **≈ 27 rendered lines**. At 1440x900 it is **≈ 32**. Everything below is
budgeted against **27** — design to the smaller and 1440x900 is free headroom. Prose reflows at
roughly 90–110 characters in an ~800px content column, so a source line of ≤ 80 characters renders
as one line; the budgets assume 1:1 and therefore run conservative.

| Surface | Budget | Occupancy after this project | Headroom | What yields first if exceeded |
| --- | --- | --- | --- | --- |
| `crate-root-front-door` | pointer within the **first 5 rendered lines**; costs **≤ 2 source lines** | pointer at rendered line 3; first screen holds summary + pointer + `# Status` + status paragraph + the ADR-0006 paragraph ≈ **16 lines** | ≈ 11 lines | the pointer's own sentence shortens (30-word cap → 20). Nothing pre-existing moves or folds. |
| `readme-front-door` | the compiled fence's opening ``` stays within the **first 35 source lines**; pointer costs **≤ 2 source lines** | fence opens at source line **32** (from 30) | 3 lines | the *pointer* is re-placed, never the fence. If merge-forward with HS-P0016 pushes the fence past 35, placement is re-decided as an invariant-2 judgement — not rebased. |
| `store-module-error-site` | the `## Import one flavour, not both` section stays **≤ 36 source lines** (today 13) | **≈ 34 source lines** (a 2 + b 14 + c 3 + d 3 + e 3 + f 2, measured) | 2 lines | **in this order**: (1) the limit sentence compresses to one clause; (2) the connective prose around the pointer; (3) the fence's non-`= note:` context lines. **The two `= note:` lines and `TraitVariantBlanketType` never yield** — they are AC-006 itself. Cap raised from 30 and the projection corrected from 25 by the mock's findings F2/F3: rustc emits a `help:` hunk per candidate, so the honest fence is 14 source lines, not the budgeted 12. **F3 resolved at the design gate: copy fidelity (invariant 7) wins and the density budget yields.** A reproduced transcript edited to fit a budget no longer matches what the reader has on screen, which destroys the one thing the error-site explanation exists to do. Dropping candidate #1's `help:` hunk is yield rule (3) already exercised; nothing further may be cut. |
| `store-module-error-site`, fence width | **unmeetable, and stated as such** — the target was "no line exceeds 96 characters"; measured, rustc's longest line is **109 characters** and lays out at **1060px** against a **936px** content box | rustc's real candidate note exceeds it **at both viewports**, not only the narrow one | — | **nothing yields: the fence keeps rustc's output verbatim and takes a horizontal scrollbar.** Copy-fidelity (invariant 7) beats reflow, and this trade is deliberate. Corrected by the mock's finding F4: the 96-character budget is recorded here as a *measurement that failed*, not as a target anyone should try to meet, and anti-pattern 16's exception is therefore load-bearing at every viewport rather than an edge case. A future reader must not "fix" this by reflowing the transcript. |
| `adapter-reasoning-account`, first screen | H1 + answered-need (3) + the six-entry reading order (**≤ 12**) = **≤ 15 rendered lines** | ≤ 15 | ≥ 12 | a reading-order entry drops to one line. **The list never scrolls off the first screen and never loses an entry.** |
| `adapter-reasoning-account`, whole page | **≤ 900 words target, 1,200 hard cap**; **≤ 5 sentences / ~120 words** of connective tissue per source | six sections × ≤ 120 words + framing ≈ 850 | ~350 | connective tissue, always. **Never a citation, never the caveat.** Past 1,200 the page has become an authoring job and AC-008's "sequencing, not volume" has failed — escalate to the initiative, do not trim the sequence. |
| the pointer register | **≤ 8 rows at this project's close; ≤ 20 rows ever** | **5 rows** (P1–P5 below) | 3 / 15 | nothing yields. **A 21st row means the pointer policy is wrong, not that the register needs a scrollbar** — it is the DT-10 alarm, wired to a number. |

**Minimum legible size for the primary label.** Type size is the renderer's; what this design
controls is the shortest string a reader must recognise, and it is bounded from both ends:

- the pointer sentence is **one sentence, ≥ 8 and ≤ 30 words**, no parenthetical, no semicolon;
- the visible link text is **≥ 3 words** and is a noun phrase naming the destination — never
  "here", "this", "see this page", "docs", or a bare URL;
- the need named inside the sentence is **≥ 4 words**, so "the guide" alone never stands as the
  offer.

**The five register rows this project installs**, with the guard each one gets:

| # | Surface | Destination | Form (N-3 row) | Guard |
| --- | --- | --- | --- | --- |
| P1 | `crates/happenstance/src/lib.rs` crate root | the narrative guide | href ladder, rung bound at implementation | intra-doc link → `broken_intra_doc_links = "deny"` (`Cargo.toml:134`) if hosting is docs.rs-only; otherwise HS-P0020's registration check |
| P2 | `crates/happenstance/README.md` | the same | same rung as P1 | **the mirror assertion** — a pinned substring of the sentence must appear exactly once in each of the two files; see the gap below |
| P3 | `crates/happenstance-core/src/store.rs` module doc | the reasoning account | in-tree markdown link, or the named-unlinked form live at `store.rs:77` | HS-P0020's registration check that the page exists at that path |
| P4 | HS-P0022's page answering question 1 | the passage answering second question A | in-tree markdown link with fragment | HS-P0020's gate step |
| P5 | same | the passage answering second question B | same | same |

**Two named gaps, not assumed away.** (1) The **mirror assertion** for P2 does not exist: the
README-as-doctest mount at `crates/happenstance/src/lib.rs:10` compiles the README's ```` ```rust ````
fences, **not its prose**, so a prose pointer is *not* guarded by it — AC-002's mechanism claim is
over-broad for the placement this design chooses, and saying so is cheaper than discovering it. The
concrete ask is a `check_summaries`-shaped step: a pinned sentence-fragment constant crossed with
the two front-door paths, failing the build with the reason in the message. Two lines, the same
shape as `xtask/src/lint_constitution.rs:463-473`, and **this project does not own
`xtask/src/`** — it is offered to HS-P0020. (2) Nothing in `cargo xtask ci` verifies keyboard-only
reachability or self-describing link text; AC-004, AC-005 and AC-009 are dated observed walks by
design, and the absence of a linter is stated rather than papered over.

---

## Hierarchy

| Surface | Primary | Secondary | Recessive | What carries the distinction |
| --- | --- | --- | --- | --- |
| `crate-root-front-door` | the crate summary and the pointer — both pre-heading, both inside the docs.rs search blurb | `# Status`, `# Using it today` and its compiled fence | the planned-items bullet list under `# What arrives here` | **Position relative to the first `#` heading, and nothing else.** No bold, no callout, no emoji: bolding the pointer would out-weight a status disclaimer on a crate that is a facade, and weight is a claim. |
| `readme-front-door` | H1, the description, the pointer | the status blockquote | Guarantees / Design / Licence | **Document order plus the single blockquote.** The file has exactly one admonition; the pointer is deliberately not a second one, because two halve each other. |
| `store-module-error-site` | the `error[E0034]` fence and the in-place fix | the plain-words naming of the ambiguity; the unchecked-limit sentence | **the pointer** | **Order within the section, and the pointer carrying no heading.** This inverts the usual instinct on purpose: the moment the pointer stops being recessive, invariant 1 fails and the page starts requiring a hop to unblock. |
| `adapter-reasoning-account` | the numbered reading order | the six section headings, each carrying its position | the connective tissue | **Enumeration and precedence.** The list is the page's only enumerated element and it precedes everything, including the first section. |
| `evaluator-onward-links` | the answering passage itself (HS-P0022's) | — | the link | **Inline, sentence-final, no device.** A link that outweighs the passage it sits in is a "next steps" block wearing a sentence. |

---

## States

| State | `crate-root` / `readme` | `store-module-error-site` | `adapter-reasoning-account` / `onward-links` |
| --- | --- | --- | --- |
| **Empty** — destination does not exist yet | **the pointer is not installed.** No "coming soon", no placeholder href. A pointer into nothing is the dead end invariant 5 forbids and is worse than today's silence. This is the state on this branch right now, and the storymap's slice-3 external gate is what enforces it | the pointer is not installed until the account is a registered page; the fence rewrite and the fix **still ship** — elements (b)–(e) have no dependency on the account | n/a — the page is the destination |
| **Loading** — no analogue in a static medium | the analogue is the **unmerged sibling branch**: `crates/happenstance/src/lib.rs` is 75 lines here and 237 on `initiative/from-contract-to-published-library`. Placement is re-decided at merge as an invariant-2 judgement, never mechanically rebased. **Not verifiable from this worktree** | behind HS-P0020's clause-id pin: copy may be drafted, the first edit may not be made | behind HS-P0020's pinned tree existing |
| **Error** — destination moved or renamed | the row's named guard fires: a rustdoc build fails, or HS-P0020's registration check fails. If the row's guard reads "none", rule 2(iv) was violated and the pointer should never have been installed | same | HS-P0020's gate step |
| **Overflow** — a budget is exceeded | the pointer's sentence shortens; the fence never moves past source line 35 | the yield order in `## Density budget`, and the `= note:` lines never yield | connective tissue yields; past 1,200 words the project escalates rather than trimming the sequence |
| **Long label** — long destination name, or a rustc line > 96 chars | link text is trimmed to the destination's own heading text, **never** to "here" | **the fence keeps rustc's line verbatim and scrolls horizontally** | a reading-order entry drops to one line; its citation is never dropped |
| **Narrow viewport** — 1024x768 and crates.io on a phone | everything reflows; nothing added has a fixed width, a table, or a column | the `text` fence is the only non-reflowing element and gains a horizontal scrollbar — expected, not a defect | the reading-order list is a plain numbered list and wraps |
| **Entered mid-sequence** | n/a | a reader arriving at a fragment from a search result sees the four-entry ladder unchanged | the landed-on heading carries "n of 6", so position is learned from the heading, not from scrolling up |
| **Screen reader / keyboard only** | link text is self-describing; heading ladder unskipped | the caret line is never the only carrier — element (c) is | the reading order is a real list; each hop is a plain link |

---

## Items

**No public item is added, changed or removed by this project.** Its four surfaces are a crate-root
doc comment, a package README, a module doc comment and a narrative page; none is a `pub` item, a
signature, a feature or a manifest entry. The architecture brief states this as a non-goal once so
it is not rediscovered (N-1, N-11), and `_storymap.md`'s standing constraints repeat it for every
story.

```yaml
- path: "" # none — this project changes no public item
  kind: ""
  change: ""
  feature: ""
  clause: ""
```

**The one candidate that would have changed a public item, recorded rather than silently absent.**
A `prelude` module exporting `EventStore` and not `SendEventStore` would make the E0034 collision
unreachable by the default import path, and `references/evaluation/review-dx-ergonomics.md:421-422`
calls it "the single highest-leverage doc fix in the crate". No such module exists today (verified:
neither `crates/happenstance-core/src/lib.rs` nor `crates/happenstance/src/lib.rs` declares one). It
is **out of this project's boundary** — a public API addition with a semver surface, owed an ADR.
Its absence here is a scoping decision, not a rejection.

Two attributes are added and they are not public API in the semver sense — they add no path a
caller can write: `#[doc(alias = "E0034")]` on `EventStore` and `SendEventStore`, and
`#[doc(alias = "TraitVariantBlanketType")]` on `SendEventStore`.

## Signatures

The exact text, as it will be written, so review is against the thing rather than a description of
it. Destinations in `SMALL CAPS` are late-bound per the href ladder; **nothing else is negotiable
without re-opening this file.**

```text
P1 / P2 — the authoritative front-door sentence (identical rendered text on both surfaces):

    Guide-level documentation — what a dynamic consistency boundary is, and how to
    model an application with one — is at [the happenstance guide](DESTINATION).

    22 words. Pinned substring for the mirror assertion: "Guide-level documentation".
    Link text: "the happenstance guide" (3 words, noun phrase, self-describing).
```

```text
P3 — the store.rs pointer, last sentence of `## Import one flavour, not both`:

    For why the two flavours exist at all, and what an adapter looks like once you
    accept them, [the adapter reasoning account](DESTINATION) sequences the
    explanations that already exist, in reading order.

    Named-unlinked fallback, in the form live at store.rs:77 — the adapter reasoning
    account at `TREE-PATH` sequences … . It is named rather than linked because the
    destination is a narrative page, not an item, and an intra-doc link to it would
    not resolve.
```

```text
Element (c) — plain-words naming of the ambiguity, required content, not fixed wording:

    names the method (`read`), names both `EventStore` and `SendEventStore`, and
    states in words that both are in scope. The caret line `^^^^` may not be the only
    statement of it.
```

```text
Element (e) — the narrow limit sentence, required content:

    the fence is `text` and nothing compiles it; the error *code* is asserted by a
    compiled `rust,compile_fail,E0034` example in the constitution
    (standards/rust/20-two-flavour-ports.md:179 and
    standards/rust/00-prime-directives.md:242); the wording of the notes and the
    internal name `TraitVariantBlanketType` are rustc 1.97.1's rendering and are
    asserted by nothing.

    `TraitVariantBlanketType` carries backticks in prose: clippy::doc_markdown fires
    on CamelCase, and RS-70-3 forbids the `allow` repair.
```

## Shape decision

| Item | Chosen shape | Rejected (and why) | Evidence | Resolves |
| --- | --- | --- | --- | --- |
| DT-10 policy | (c) both, with one authoritative — bound by "one front door, and a pointer only at a stall" | (a) front-door-only: unreachable for the reader who lands on `happenstance_core::store` from a diagnostic. (b) per-item: no closing condition, and the front door is not an item | `initiative.md` DT-10 row; `crates/happenstance-core/src/lib.rs:99` (`pub mod store` renders its own page) | DT-10, AC-001 |
| the front-door pointer | one plain sentence, pre-heading, mirrored | routing table (one-row table is a widget); blockquote (out-weights the status disclaimer); new heading (outline inversion) | dossier "Two-surface split" 86-97; `docs/README.md:12-23` is the ten-destination case a table is for | DT-10, AC-002 |
| the `store.rs` pointer | recessive, last sentence, no heading, after the fix | above the fix (fails invariant 1); own `##` (fifth ladder entry, one sentence); intra-doc link (destination is not an item — `store.rs:76-78` records the class of bug) | UX brief invariant 1; RS-70-2 at `standards/rust/70-rustdoc-obligations.md:95`; AC-A02 | DT-10, AC-007 |
| `#[doc(alias)]` | used, two evidenced strings, by a stated rule | synonym farming; aliases as a substitute for the pointer | zero occurrences workspace-wide (verified) — this establishes the convention and says so | UX-AC-10, AC-011, DR-5 |
| the register | build-time data, never a rendered page; capped at 8 rows now / 20 ever | a markdown table in a planning artefact (rots invisibly); a rendered index page (is the second navigation surface) | `xtask/src/lint_constitution.rs:64` and `:463-473` — the working precedent | AC-003, DT-10's stated cost |
| the reasoning account | one page, sequence stated up front, position in each heading | two pages by source (splits the content); folds/tabs (DT-7 is HS-P0020's) | dossier "Staged disclosure" failure mode; N-11 left this open and it is bound here | AC-008 |
| bare-URL pointers | forbidden outright to this project | permitted with an inventory row saying "guard: none" (N-3 row 4 allows it) | AC-003 — "no pointer is installed whose only guard is memory" | AC-003 |

## Placement and re-export

Nothing is re-exported and no module is added to any crate. Placement is a question about *files
and render paths*, and it has one non-obvious constraint worth stating in the coherence slot:

**The adapter pointer may not live on `happenstance`'s front door.** `happenstance` is the
application-facing crate and its crate root already states ADR-0006's conclusion in prose
(`crates/happenstance/src/lib.rs:20-25`), routing adapter authors to `happenstance_core` at
`:69-71`. An adapter pointer there would contradict the page it sits on. Conversely, the guide
pointer may not live on `happenstance-core`: adapter authors pin the contract crate, application
authors reach for the bare name. **The two pointers are on different crates because the ADR put the
audiences on different crates**, and that is the placement decision, not a stylistic one.

Two files are shared and the seams are by *purpose*, not by paragraph:
`crates/happenstance/README.md` — HS-P0016 owns every claim on it, this project owns two added
lines that assert nothing (DoD-7 makes the seam a diff); `crates/happenstance-core/src/store.rs` —
this project owns the third section only, behind HS-P0020's clause-id pin.

## Visibility and stability

| Item | Visibility | `#[non_exhaustive]` / sealed | Feature | Semver promise |
| --- | --- | --- | --- | --- |
| the front-door pointer (P1, P2) | rendered on docs.rs and crates.io | n/a | default | none — doc text carries no semver obligation. Its *destination* does: a moved page is a broken read, which is what the guard is for |
| the `store.rs` rewrite (b)–(f) | rendered at `happenstance_core::store` in all three gate rustdoc builds | n/a | default — and it **must resolve under `--no-default-features`**, which is why (f) is not an intra-doc link (AC-A02) | none |
| `#[doc(alias = "E0034")]` on `EventStore`, `SendEventStore` | rustdoc search index | n/a | default | none — an alias adds no path a caller can write |
| `#[doc(alias = "TraitVariantBlanketType")]` on `SendEventStore` | rustdoc search index | n/a | default | none |
| the pointer register | **not rendered at all** — build-time data | n/a | n/a | none. Deliberately invisible; see `## Transience policy` |

**The concrete ask handed to HS-P0020**, so it is a request on record and not an assumption: a
`check_summaries`-shaped step reading a pinned list of pointer-bearing surfaces and failing when a
surface stops containing its pinned constant. Until it lands, every installed pointer must take a
form from N-3 rows 1–3, which are already guarded; bare URLs are forbidden by rule 2(iv).

## What it costs a caller

The caller here is a reader, and the budget is their attention plus the maintainer's.

- **Front door: 2 source lines, ~22 rendered words, on each of two surfaces.** Zero displacement:
  nothing pre-existing is reworded, folded, or moved by more than 2 lines. The README's compiled
  fence moves from source line 30 to 32, against a budget of 35.
- **Error site: ~12 added source lines** in a section going 13 → ~25, cap 30. The reader who only
  wants to get compiling reads *further* than before — from the fence at (b) past (c) to the fix at
  (d) — because (b) grew by six lines. That is the cost of AC-006 and it is paid deliberately: the
  six lines are the ones the reader searched for.
- **One hop, never two.** No path this project installs requires a second hop to become unstuck.
- **Maintenance: five register rows, forever**, each with a guard that fails the build rather than
  a reviewer's memory. Three of the five depend on HS-P0020's checker existing; if it does not, the
  three collapse to the intra-doc/in-tree forms whose guards already exist.
- **On both port flavours**: nothing here touches a port, a bound, or a future. `EventStore` and
  `SendEventStore` gain doc attributes only, which are erased before type-checking. The cost is
  identical on both flavours and on `wasm32`, and the `--no-default-features` doc build is the one
  gate step this design is actually shaped by (it is why (f) is not an intra-doc link).

## What a user meets first

**The evaluator** meets, in this order: the crate summary line, then the pointer, then the status
callout. The pointer is the second thing on the page for a reason — a facade's disclaimer read
before the offer sends a reader away with the guide unmentioned.

**The adapter author** does not meet the crate root at all. They meet
`happenstance_core::store`, arriving from a diagnostic or a search box, and the first thing they
meet on it is whichever fragment they landed on — most often the `error[E0034]` fence itself.

**What is deliberately not on any front page:** the reasoning account (it is one hop from the error
site and nowhere else), the pointer register (it is not a page), the six sources the account
sequences (they stay where they are — the account orders them, it does not relocate them), and the
`prelude` proposal (out of boundary, owed an ADR).

## The states the API must express

The states this project's *copy* must be able to represent, enumerated here so they are written
into the sentences rather than discovered as a missing clause:

- **Absent destination** — the guide does not exist yet. Expressed by *not installing the pointer*,
  never by a placeholder.
- **Late-bound destination** — the guide exists but its hosting shape is unresolved. Expressed by
  UX-AC-02's split: the sentence names the need, only the href is bound late.
- **Unreachable-by-link destination** — the target is not an item and no link form resolves.
  Expressed by the named-unlinked cross-reference already live at `store.rs:77`.
- **Partially checked** — the error code is asserted, the diagnostic wording is not. Expressed by
  element (e), which must state the *narrow* limit; "nothing checks this" is wrong in the direction
  that discards a real guard (AC-A06).
- **Superseded reasoning** — the account cites a decision that is later superseded. Expressed by
  citing atoms through `.kb/maps/decision-map.md`'s supersession graph, never a bare record.
- **Reader out of sequence** — arrived at section 3 of 6 from a fragment. Expressed by the position
  in each heading.
- **Refused** — a walk that found a dead end. Expressed as a *recorded failed walk*, never a quiet
  re-walk (AC-005, DR-10).

## Anti-patterns

Each is checkable against a screenshot or a diff by someone who cannot read Rust.

1. A screenshot of the crate root's first screen containing no sentence that names guide-level
   material.
2. Two **different** sentences serving as the front-door pointer on the README and the crate root.
   One authoritative text, mirrored.
3. Any pointer whose visible link text is "here", "this", "see this page", "docs", "read more", or
   a raw `https://` string.
4. The front-door pointer rendered inside the README's status blockquote, or as a second blockquote
   anywhere in that file.
5. A "See also", "Next steps" or "Further reading" block appended to the bottom of any page this
   project touches.
6. A table with fewer than three rows used as a navigation device anywhere in this project's diff.
7. A screenshot of `happenstance_core::store` whose `error[E0034]` block does not show two lines
   beginning `= note:`.
8. A screenshot of that same page in which the string `TraitVariantBlanketType` does not appear.
9. Any collapsed or folded element in content this project added — a `<details>`, a tab strip, an
   accordion, a `[+]` the reader must open to reach a constraint.
10. Raw HTML or an inline `style=` attribute in any doc comment or markdown this project adds.
    rustdoc ships light, dark and ayu; a hard-coded colour is unreadable in at least one.
11. Any line in `crates/happenstance/README.md` that has moved down by more than **three** lines
    relative to `main`, or any changed line other than the addition. Three, not two: the mock's
    finding F1 measured the pointer at three source lines — a 22-word sentence with its markdown
    link does not fit one 80-column line — so a two-line rule was violated by this file's own
    placement decision the moment it was written, and a rule the design itself breaks is worse
    than no rule. The compiled fence therefore opens at source line 33, not 32; the budget of 35
    still holds.
12. The `store.rs` pointer appearing **before** the import-rule fix in reading order.
13. A `##` heading in `store.rs` whose entire body is one sentence, or a fifth entry in that file's
    heading ladder.
14. The pointer register rendered as a page a reader can navigate to.
15. Colour, syntax highlighting, or the caret line `^^^^` as the only carrier of which call is
    ambiguous.
16. A screenshot at 1024x768 in which any element this project added is clipped rather than
    reflowed — the `error[E0034]` fence's horizontal scrollbar excepted and expected.
17. A reasoning-account section heading that does not carry its position in the sequence.
18. A pointer installed with an empty guard cell in the register.

Standing, and never re-litigated here: no `#[async_trait]`; no `serde` in `happenstance-core`'s
defaults; `read` returns the stream at the top level; generic code binds `EventStore`, not
`SendEventStore`; no `unwrap`/`expect` in library code. None is touched by this project.

## The doctest

**The honest answer is that the compiled artefact this design leans on already exists and is not
ours to write.** The template asks for a runnable example because the gate compiles it and it
therefore cannot drift. Nothing this project adds is compilable: a prose pointer is prose, and the
`error[E0034]` transcript is a diagnostic, not a program. Writing a Rust fence around either to buy
a green tick would be the `ignore`-fence anti-pattern wearing a compiler.

What *is* compiled, and what element (e) must cite rather than duplicate:

```rust
// standards/rust/20-two-flavour-ports.md:179 and
// standards/rust/00-prime-directives.md:242 each carry, verbatim, a fence opened as
//
//     ```rust,compile_fail,E0034
//
// compiled by the `the constitution's examples compile` step
// (`cargo test --locked -p xtask --doc`, RUSTDOCFLAGS=-D warnings) through
// xtask/src/constitution.rs. It fails the build if importing both flavours ever
// stops producing E0034.
//
// So the error CODE in store.rs's restored fence is asserted — elsewhere, by a
// compiled negative. The candidate-note WORDING and the internal name
// `TraitVariantBlanketType` are rustc 1.97.1's rendering and are asserted by
// nothing. Element (e) states exactly that, and no more: "nothing checks this"
// would be wrong in the direction that throws away a real guard (AC-A06).
```

Two consequences the implementer must not soften. The restored transcript is **reproduced**, by
compiling a deliberate double-import at the pinned toolchain (`rust-toolchain.toml`,
`channel = "1.97.1"`) and pasting from that run's stderr —
`references/evaluation/review-dx-ergonomics.md:404-414` says what the block should *contain*, not
what to copy, because it predates the MSRV raise. And the excerpt stays on the parts that are
stable across toolchains — the error code and the candidate item names — because diagnostic
phrasing moves and pretending otherwise is a claim the page cannot keep.

## Mock

A single self-contained page. It makes **zero requests of any kind** — the repository's own rustdoc
stylesheet is inlined byte-for-byte and the two web fonts the density budget is written in are
inlined as `data:` URIs — so it renders identically from `file://` with the network off.

| Mock | Path | Viewports | Themes | Notes |
| --- | --- | --- | --- | --- |
| Sign-off mock, all five surfaces | `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` | 1440x900, 1024x768 | light | 19 declared states × 2 viewports = **38 labelled frames**, each tagged with surface id / state / viewport / theme, each drawn at its real declared width with a fold marker at the viewport height less ~120px of browser chrome. Frames build as you scroll; press **Build every frame** or open `mock.html#all` to realise all 38 for a capture pass. |

**Substrate, and what is real in it.** `target/doc/static.files/normalize-9960930a.css` and
`target/doc/static.files/rustdoc-17e0aaed.css` (69,637 bytes, rustdoc `1.97.1 (8bab26f4f
2026-07-14)`) are embedded verbatim; the only edit is `@font-face` `src` rewriting.
`SourceSerif4-Regular` and `SourceCodePro-Regular` are inlined — the serif that sets the prose
measure and the monospace that sets the fence width — and every other face keeps its `local()`
source only, because sixteen frames each parse this stylesheet and embedding all eight faces made
the page heavy enough to paint nothing. The rustdoc frames carry rustdoc's own class contract
(`body.rustdoc.mod`, `nav.sidebar`, `#rustdoc-toc`, `ul.block.top-toc`, `main > .width-limiter >
section#main-content.content`, `details.toggle.top-doc > .docblock`, `.example-wrap >
pre.language-text`, `dl.item-table`), mirrored from `target/doc/happenstance/index.html` and
`target/doc/happenstance/store/index.html` rather than approximated. The `error[E0034]` transcript
is a **reproduction**: a deliberate double-import compiled against this worktree's
`happenstance-core` at the pinned 1.97.1 toolchain, pasted from that run's stderr.

**What the mock cannot be honest about, stated on every affected frame.** Three of the five surfaces
have no renderer in this repository. `readme-front-door` is rendered by crates.io and GitHub, whose
stylesheets are not in this tree, so its frames are drawn as a real line-numbered source pane plus a
labelled harness-typography prose pane. `adapter-reasoning-account` and `evaluator-onward-links`
live in HS-P0020's pinned tree, which does not exist — no `book.toml`, no mdBook, no template
anywhere (verified) — so their frames render **composition and density only**. Approving those
frames approves reading order, heading ladder, caveat placement and hop placement; it approves
nothing about type, colour or measure, and it will need re-approving once HS-P0020 lands.

**Six findings the mock produced, all in its closing section.** They are the reason this step is not
decoration, and three of them contradict numbers written above in this file:

- **F1** — the front-door pointer costs **three** source lines, not two: a 22-word sentence with its
  markdown link does not fit one 80-column line. The README's compiled fence therefore opens at
  source line **33**, not 32 (budget 35 still holds); and `## Anti-patterns` item 11 — "moved down
  by more than two lines" — is violated by this file's own placement decision.
- **F2** — the reproduced transcript is 20 lines, because rustc emits a `help:` hunk per candidate.
  The frames drop candidate #1's hunk (keeping both `= note:` lines and `TraitVariantBlanketType`),
  which is yield rule (3) exercised at design time. The fence is still **14** source lines against a
  budgeted 12, and the section lands near **34** against a cap of 30.
- **F3** — F2 is not arithmetic, it is invariant 7 (copy fidelity) and the density budget in direct
  conflict. Sign-off must pick a winner.
- **F4** — measured: the fence's longest line is **109 characters** and lays out at **1060px**
  against a **936px** content box at **1440x900**. It overflows and scrolls at *both* viewports, not
  only the narrow one, so the "≤ 96 characters" fence-width budget is unmeetable and anti-pattern
  16's exception is load-bearing everywhere.
- **F5** — the three-surfaces-without-a-substrate problem above, as a scheduling fact.
- **F6** — everything drawn composes from primitives already live in this tree at a cited line. No
  widget was invented and nothing was hand-rolled. `#[doc(alias)]` is the one newly adopted built-in
  and is invisible in a mock by construction: it changes the search index, not the page.

**All six dispositioned at the design gate on 2026-08-17**, and the three contradictions folded
back into the binding sections above rather than left for sign-off to discover again:

| # | Disposition |
| --- | --- |
| F1 | **Anti-pattern 11 amended** from "two lines" to "three". The design's own placement decision violated the two-line rule the moment it was written, and a rule the design breaks is worse than no rule. The fence opens at source line 33; the budget of 35 holds. |
| F2 | **Density row corrected.** The `store.rs` section cap rises 30 → 36 and the projection 25 → 34 (measured), because rustc emits a `help:` hunk per candidate and the honest fence is 14 source lines, not 12. |
| F3 | **Resolved — copy fidelity wins, the density budget yields.** This is the winner the mock asked sign-off to pick. A reproduced transcript edited to fit a budget no longer matches what the reader has on screen, which destroys the one thing the error-site explanation exists to do. Dropping candidate #1's `help:` hunk is yield rule (3), already exercised; nothing further may be cut. |
| F4 | **Recorded as a failed measurement, not a target.** The ≤ 96-character fence budget is unmeetable: rustc's longest line is 109 characters and overflows at *both* viewports. Anti-pattern 16's exception is load-bearing everywhere, and a future reader must not "fix" this by reflowing the transcript. |
| F5 | **Tracked, not closed.** Two of five surface entries (`adapter-reasoning-account`, `evaluator-onward-links`) keep `route: TBD` because HS-P0020 has not pinned the tree yet. Inventing a route would be the invented-primitive failure this stage exists to prevent. This project's exit criteria gate final capture of those two surfaces on HS-P0020 landing a routable path. |
| F6 | **No action.** Recorded as the positive finding it is. |

One citation correction outside the six: the reasoning account's sixth reading-order entry
labelled a passage in `CONTRIBUTING.md` as the section "Provided methods". No such heading
exists — the real one is `## Adding a method to a port` (`CONTRIBUTING.md:97`), and the rule
cited sits at `:104`. Corrected in `design/mock.html`, which has been republished.

**Reference captures the design review will use** — one per surface per viewport, ten in all:

| Surface | 1440x900 | 1024x768 |
| --- | --- | --- |
| `crate-root-front-door` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/crate-root-front-door@1440x900.png` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/crate-root-front-door@1024x768.png` |
| `readme-front-door` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/readme-front-door@1440x900.png` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/readme-front-door@1024x768.png` |
| `store-module-error-site` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/store-module-error-site@1440x900.png` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/store-module-error-site@1024x768.png` |
| `adapter-reasoning-account` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/adapter-reasoning-account@1440x900.png` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/adapter-reasoning-account@1024x768.png` |
| `evaluator-onward-links` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/evaluator-onward-links@1440x900.png` | `.bklg/docs-that-teach/reach-and-adapter-path/design/reference/evaluator-onward-links@1024x768.png` |

The captures are not committed yet: `design.capture` is absent from `.redkiln/config.yaml` and the
perceptual review is a standing skip, so nothing generates them automatically. Anyone capturing them
by hand should open `mock.html#all`, wait for the counter to read "all 38 frames built", and shoot
each surface's frame group at each viewport.

## Sign-off

**Approved by Ryan Britton (repository owner), 2026-08-17.**

Recorded via `redkiln advance HS-P0023 --verdict approved --stay --apply`, which clears the
review gate without moving the project off `design`.

The mock asked sign-off to pick a winner (F3), and it did: **copy fidelity, invariant 7,
beats the density budget.** A reproduced rustc transcript edited to fit a budget no longer
matches what the reader has on screen, which destroys the one thing the error-site
explanation exists to do. The budget yields instead — the `store.rs` section cap rises to 36
against a measured 34, and the ≤ 96-character fence width is recorded as a *failed
measurement* rather than a target, because rustc's longest line is 109 characters and
overflows at both viewports. Anti-pattern 16's exception is load-bearing everywhere, and a
later reader must not "fix" the transcript by reflowing it.

Two conditions carried, neither blocking:

1. **F5 stays tracked, not closed.** `adapter-reasoning-account` and `evaluator-onward-links`
   keep `route: TBD` because HS-P0020 has not pinned the tree yet. Final capture of those two
   surfaces is gated on HS-P0020 landing a routable path, so the incompleteness is tracked
   rather than forgotten when this project closes.
2. **DT-10's pointer policy is authoritative for siblings.** Any project adding a pointer
   applies the four gates recorded here rather than re-deciding them.

Perceptual review: **standing skip** — `design.capture` is deliberately absent from
`.redkiln/config.yaml` (lines 75-81), there is no app to screenshot, and nothing downstream will
look at these four surfaces and disagree. That is recorded as a skip with its standing reason, per
this project's DoD item 3, and it is why the sign-off on this file is the last opportunity to
disagree cheaply.
