---
item: HS-P0023
stage: briefs
created: 2026-08-17
updated: 2026-08-17
---

# Briefs — Reach and the Adapter Path

This companion holds every brief warranted for HS-P0023
(`architecture`, `ux`; no `testing`, no `deployment` —
`.bklg/docs-that-teach/_decomposition.md`, "Warranted briefs per project").
Each brief owns one `##` section and follows `.redkiln/templates/briefs/brief.md`
(Intent / Acceptance Criteria / Notes).

---

## UX brief

### Intent

Two people are stuck in the same way and neither of them can be reached by a page
that waits to be visited. **The evaluator** (Persona 3) has just had their first
question answered well and has formed a second one; the seed's verdict is literal
— *"nowhere for that question to go"*
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`,
Persona 3, journey step 3). **The adapter author** (Persona 2) is looking at a
compiler error in a file that does not explain it, while the explanation sits in
three contributor-facing documents they have no reason to know exist (same file,
Persona 2, journey step 3).

This brief scopes the *experience* of closing both gaps, plus the front door that
makes either reachable at all (AC-002 / initiative AC-11 / DoD-7). It is held to
contract grain: it says what the reading experience must be and what would falsify
it, and it does **not** resolve DT-10 — that is `_design.md`'s, and the perceptual
review is a standing skip (`.redkiln/config.yaml`, line 15: capture commented out),
so the written resolution is the only record there will ever be.

**The medium is not a web app, and this brief does not pretend otherwise.** The
surfaces are rustdoc's rendered crate root, a Markdown README rendered by crates.io
and re-compiled as a doctest, a Rust module doc comment, and whatever narrative
surface HS-P0020 resolves to host. The "design system" is therefore the ecosystem's
own rendering primitives plus this repository's already-established house forms,
enumerated below — and the strongest UX requirement in the whole brief is the
negative one the evidence already settled: **the evaluator's gap is a missing
*link*, not a missing *widget*** (`_discovery/distillation/interaction-patterns.md`,
Anti-patterns, final bullet; and "Breadcrumb / master-detail navigation", Fit
conditions).

---

### User intent, states, and the transitions this project owns

Framed as what the person is trying to do. Mechanism is named only where the
mechanism *is* the experience (a searched string either is in the file or is not).

#### Journey A — the evaluator's second question (AC-002, AC-005; DoD-7, DoD-9)

| State | What they are trying to do | What must be true | What is true today |
| --- | --- | --- | --- |
| **A0 — front door** | Decide inside a bounded reading budget whether to depend on this | On the first screen of either surface a developer sees after `cargo add happenstance`, it is *visible that guide-level material exists and where it is* | `crates/happenstance/README.md` and `crates/happenstance/src/lib.rs:11-71` point outward only to crates.io and `spec/SPECIFICATION.md`. Verified: no narrative pointer on either. |
| **A1 — question formed** | Ask the next, sharper question — a stall point, not a rephrasing of the first | The named second questions (DR-4: dynamism-as-chaos, modelling-vs-routing, the missing replacement noun) are each one hop from the page that answered question 1 | Nowhere to go; the documented fallback is `spec/SPECIFICATION.md`'s 200 clauses, which is a different document than the question calls for |
| **A2 — landed on the answer** | Read the answer and know it *is* the answer | The landing point is the passage that answers, not the top of a long page the reader must re-scan | n/a — no such page exists yet (HS-P0022 authors it) |
| **A3 — oriented after landing** | Get back, or go on, without browser history | The destination states what it assumes and where it sits; no walked path terminates in a dead end | The axum counter-case is the recorded cost of not doing this: *"I was not sure in which order to try to read them"* (`interaction-patterns.md`, Two-surface split, Documented failure modes) |

**The failure state to design against is not a blank page — it is a good page with
no exit.** Persona 3's timing makes this different in kind: *"Persona 3's entire
journey happens inside one reading session, with no second attempt if the first one
fails silently"* (`personas-and-journeys.md`, Cross-persona tensions, third bullet).

#### Journey B — the adapter author at the error site (AC-006, AC-007, AC-008; DoD-10)

| State | What they are trying to do | What must be true | What is true today |
| --- | --- | --- | --- |
| **B0 — the error fires** | Find out what just happened, from where they are standing | The reader's *actual* first interaction is a search — Ctrl-F in the open buffer, or the message pasted into a search box. The exact string rustc emitted must therefore be present verbatim in the file they already have open | `crates/happenstance-core/src/store.rs:36-41` carries a **trimmed** `error[E0034]` block: the error line, the call line, the `^^^^ multiple` caret. Verified: no `= note:` candidate lines, and `TraitVariantBlanketType` appears nowhere in the file |
| **B1 — unblocked** | Get compiling again | The fix is stated *in place*, next to the error, and does not require following any link | Already true and must survive the rewrite: `store.rs:43-45` gives the import rule and the fully-qualified escape hatch |
| **B2 — wants the reasoning** | Tell the difference between *"the port is wrong for me"* and *"I have not understood the port yet"* — this persona's stated goal verbatim (`personas-and-journeys.md`, Persona 2, Goal) | One hop from `store.rs` reaches an account that *sequences* the existing reasoning (DR-7) and cites rather than restates it | The material exists across four surfaces in no reading order; nothing in `store.rs` points at any of it |
| **B3 — reading the account** | Build a model of what an adapter is shaped like | The account says what `MemoryEventStore` **is not** before a reader generalises `RwLock<Vec<_>>` into "the shape an adapter takes" | `crates/happenstance-core/src/memory.rs:16-35` already states it is the conformance oracle; nothing sequences that caveat into an adapter-facing reading order |

**What they are afraid of is the design input.** Not missing information — *silent
wrongness*: "a wall that looks like their own misunderstanding but might be the
port's limit" (`personas-and-journeys.md`, Cross-persona tensions, second bullet).
That is why B1 must not depend on B2: a reader who never follows the link must
still be correctly unstuck, and a reader who does follow it must arrive somewhere
that admits its own limits.

---

### Design-system primitives — compose these, hand-roll nothing

There is no CSS layer to author against, and inventing one would be the deviation.
The primitive set is the ecosystem's plus this repository's own established forms.
**Every pointer, callout and heading this project installs must be one of these.**

| Primitive | Where it already lives | What it is for here |
| --- | --- | --- |
| **Intra-doc link** — `` [`EventStore`] `` | `crates/happenstance-core/src/store.rs:12-14, 23-28` | Every reference-to-reference hop. Resolves to the item, so it *lands on the answer* rather than the top of a page. Governed by `standards/rust/70-rustdoc-obligations.md` RS-70-2 |
| **Conditional / unlinked cross-reference** — name it, do not link it, and say why in one sentence | RS-70-2's **Do** arm; the live instance is `crates/happenstance-core/src/store.rs:77` | The only sanctioned way to reference something that would not resolve in every feature configuration. Reaching for a bare link instead fails the gate's `--no-default-features` doc build |
| **`#[doc(alias = "…")]`** | **Zero occurrences in `crates/` or `standards/`** — verified by direct search | The single unused built-in findability affordance in the workspace. DR-5 requires exhausting the built-in surface before proposing anything else; this is the unexhausted part of it |
| **Rustdoc heading ladder** — `#` then `##`, never skipped | `store.rs:3, 21, 31, 47`; `crates/happenstance/src/lib.rs:13, 27, 53` | Section structure inside a doc comment. Also the accessibility floor (see below) |
| **Compiled fence** — ```` ```rust ```` | `crates/happenstance/src/lib.rs:58-67`; `crates/happenstance/README.md:30-39` | Anything asserting an API fact. Governed by `standards/rust/62-doctests-and-harnesses.md` |
| **Uncompiled fence** — ```` ```text ```` | `store.rs:16-19` (the implication diagram) and `store.rs:36-41` (the E0034 excerpt) | Compiler transcripts and ASCII relations only. Nothing checks these — the page must say so (DR-6) |
| **README-as-doctest** — `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` | `crates/happenstance/src/lib.rs:10` | Makes a malformed pointer *written into the README* a build failure rather than a review miss. AC-002 depends on this mechanism, not on reviewer attention |
| **Blockquote callout** — `> **Status: …**` | `crates/happenstance/README.md:6-11` | The repository's only admonition form. Plain Markdown, renders identically on crates.io, docs.rs and in a plain editor, needs no preprocessor, and cannot hide anything |
| **Routing table** — a two-column *Looking for / It is at* table | `docs/README.md:12-23` | The repository's existing, working answer to "where do I go next". If a routing affordance is needed, this is the composition to reuse; a bespoke one is a DR-5 violation |
| **`#[cfg_attr(docsrs, doc(cfg(…)))]`** | RS-70-4 | Only if a pointer is feature-conditional. `doc_auto_cfg` no longer exists — do not reach for it |

**Explicitly forbidden compositions**, each because it escapes the primitive layer:

- **Raw HTML or inline `style=` inside a doc comment or README.** rustdoc ships
  light, dark and ayu themes; a hard-coded colour is unreadable in at least one of
  them, and the reader cannot override it. This is the concrete form the
  contrast floor takes in this medium.
- **Any tab, accordion, fold or collapsed panel.** DT-7 belongs to HS-P0020
  (`_decomposition.md`, Design tension ownership, DT-7 row) and this project
  installs none. The standing hazard is recorded: this workspace "has already
  shipped one gate step that looked wired and wasn't"
  (`interaction-patterns.md`, Tabs, Documented failure modes).
- **A breadcrumb, sidebar rail, or any bespoke navigation component.** rustdoc
  already renders a per-item nav bar and any narrative surface renders a TOC
  (`interaction-patterns.md`, Breadcrumb / master-detail, Fit conditions).
- **A third-party mdBook preprocessor**, unless the built-in surface above has been
  exhausted first and the rejection recorded (DR-5, AC-011).

---

### Accessibility floor

WCAG AA is the bar; in a text medium it lands in specific, checkable places.

- **Colour never alone.** Nothing this project authors may carry meaning in colour.
  Syntax highlighting is the renderer's, not the author's, and no page may say "the
  highlighted line" or depend on one. (The two-edge-colour swimlane prior art in
  `interaction-patterns.md` is HS-P0022's diagram question, not this project's.)
- **Position and ASCII art never alone.** This is the same rule generalised, and it
  is load-bearing for AC-006. The caret line `^^^^ multiple` in `store.rs:40` is
  spatial: a screen reader linearises it into meaningless characters, and so does a
  plain-text search result. **The restored excerpt must be followed by prose that
  states, in words, which call is ambiguous and why** — the excerpt may not be the
  only carrier of the diagnosis.
- **Reduced motion.** Nothing this project adds animates. The obligation is
  negative and permanent: no affordance may be introduced whose meaning depends on
  a transition. rustdoc's own collapse/expand is instantaneous and is the
  renderer's, not ours.
- **Contrast, inherited not overridden.** Theme contrast is rustdoc's to supply;
  the requirement is not to defeat it (see the raw-HTML prohibition above). The
  same applies to the crates.io and GitHub Markdown renderers of the README.
- **Self-describing link text.** No pointer may read "here", "this", "see this
  page", or a bare URL. The link text must name the destination and the need it
  answers, because a screen-reader user routinely navigates by extracted link list,
  and because a bare URL is unreadable when spoken.
- **Heading ladder without skips.** A doc comment's `#` renders inside an item's
  section; jumping `#` → `###` breaks outline navigation for assistive technology.
  `store.rs:3 → 21 → 31 → 47` is the correct existing pattern and the rewrite must
  preserve it.
- **Keyboard-only reachability, observed not asserted.** Every reach this project
  installs is a plain link or a rustdoc-native affordance (search: `S` or `/`;
  collapse: `+` / `-`). **Each of the three observed walks (AC-004, AC-005,
  AC-009) is performed keyboard-only, and the dated record says so.** That converts
  the floor from a claim into one of the artefacts the project already owes.

---

### Interaction-quality invariants

First-class and testable. Each names its falsification, because a quality bar
nothing can fail is decorative — the same standard `CLAUDE.md` applies to
conformance rules.

1. **In-place before context-jump.** Getting unstuck must never require leaving the
   surface the reader is on. `store.rs`'s module doc must unblock the reader by
   itself (state B1); the pointer to the reasoning account is an *offer*, taken at
   B2. *Falsified by*: deleting the reasoning account and finding that `store.rs`
   no longer tells a reader how to resolve E0034.
2. **Non-occlusion — a pointer must not displace what it points at.** Adding the
   front-door pointer must not push `crates/happenstance/README.md`'s
   "Which crate do I want?" answer (lines 13-20) or its compiled example (lines
   30-39) below what a reader sees first, and must not replace the crate root's
   `# Using it today` example (`lib.rs:53-67`). Nothing may be folded to make room.
   *Falsified by*: a diff in which any existing README or crate-root content moves
   down by more than the pointer's own height, or in which any line other than the
   addition changes — which is exactly DoD-7's diff-checkable HS-P0016 seam.
3. **Land on the answer, not the top of the page.** Where the answer is not the
   destination's first screen, the pointer targets a fragment, not a bare page.
   Intra-doc links do this natively; a link into narrative prose must carry its
   heading anchor. *Falsified by*: an entry in the AC-003 pointer inventory whose
   destination is a page rather than a passage, where the answer is not on the
   first screen.
4. **Reversibility — every hop is walk-backable.** The destination of each hop
   states what it assumes the reader has already read, so a reader can retrace
   without browser history and can tell whether they arrived out of order. This is
   the axum ordering failure stated as a per-hop requirement. *Falsified by*:
   arriving at any destination in an AC-005 or AC-009 walk and being unable to name
   the page that should have preceded it.
5. **No dead ends.** Every walked path terminates in a page that either answers the
   question or names the next hop. *Falsified by*: any AC-005 path whose terminal
   page does neither — recorded as a failed walk, not quietly re-walked.
6. **Search is a first-class path, not a fallback.** The reader at B0 searches
   before they read. The verbatim rustc string must therefore be present in the
   file: `rg TraitVariantBlanketType crates/happenstance-core/src/store.rs` returns
   a hit. *Falsified by*: that command returning nothing — which is its state today
   and precisely what AC-006 fixes.
7. **Selection and copy survive.** A reader compares the excerpt against their own
   terminal output by copying it. The excerpt must be selectable as contiguous
   plain text — no line numbers, no gutter decoration, no rendered-only markup
   inside the fence. *Falsified by*: pasting the rendered excerpt into a diff
   against real rustc output and finding non-compiler characters.
8. **One reading order, stated once.** The AC-008 account is a *sequence*, and its
   order must be stated at the top rather than implied by scroll position — the
   staged-disclosure failure mode is explicit that a reader arriving mid-sequence
   "can land past the setup with no signal they missed it"
   (`interaction-patterns.md`, Staged disclosure, Documented failure modes).
   *Falsified by*: entering the account at its third section and being unable to
   tell that two sections precede it.
9. **Nothing this project installs is guarded only by memory.** Every pointer's
   rot-detector is named in the AC-003 inventory. *Falsified by*: an inventory row
   with an empty guard column.

---

### Acceptance Criteria

UX-grain and testable. Each maps to the project criterion it serves in
`project.md` and, through it, to the initiative.

- **UX-AC-01 — The front-door pointer is visible before the fold and costs the
  reader nothing else.** Both `crates/happenstance/src/lib.rs`'s crate root and
  `crates/happenstance/README.md` carry it; on each surface it appears in the first
  screen a reader meets, and no pre-existing content is displaced, reworded or
  folded to fit it. *(project AC-002, DoD-7; invariant 2)*
- **UX-AC-02 — The pointer's copy is true regardless of how HS-P0020 resolves
  hosting.** It is authored as one sentence stating that guide-level material
  exists and naming the need it serves; only the destination is late-bound. This is
  what lets AC-001's policy be decided while HS-P0020's hosting shape is still open
  (`_decomposition.md`, Available parallelism). *(project AC-002)*
- **UX-AC-03 — Every installed pointer has self-describing link text and a named
  guard.** No "here", no bare URL; the AC-003 inventory has no empty guard cell.
  *(project AC-003; accessibility floor; invariant 9)*
- **UX-AC-04 — Every pointer whose answer is not on the destination's first screen
  targets a fragment.** Recorded per row in the AC-003 inventory. *(project
  AC-003; invariant 3)*
- **UX-AC-05 — `store.rs` unblocks the reader without a hop.** After the rewrite,
  the module doc still states the import rule and the fully-qualified escape hatch
  in place (today `store.rs:43-45`), and does so *before* offering the pointer to
  the reasoning account. *(project AC-006, AC-007; invariant 1)*
- **UX-AC-06 — The restored E0034 excerpt is searchable, copyable and not
  spatially encoded.** `TraitVariantBlanketType` is present verbatim; the excerpt
  is contiguous plain text inside a ```` ```text ```` fence; the fence's uncompiled
  status is stated on the page; and prose adjacent to it names in words which call
  is ambiguous and between which two items. *(project AC-006, DR-6; accessibility
  floor; invariants 6, 7)*
- **UX-AC-07 — The reasoning account states its reading order at the top and its
  own limits inside.** It names the sequence before the first section, and it says
  what `MemoryEventStore` is *not* — the conformance oracle, not an adapter
  (`crates/happenstance-core/src/memory.rs:16-35`) — at the point it is first
  sequenced, not in a closing caveat. *(project AC-008; invariant 8; `project.md`
  risk table, final row)*
- **UX-AC-08 — Two named second questions each reach an answering passage, and the
  path is recorded.** Each of the two is drawn from DR-4's stall points, each walk
  is keyboard-only, and each record names the destination *passage*, not just the
  page. *(project AC-005, DoD-9; invariants 3, 5)*
- **UX-AC-09 — Both observed walks record their accessibility conditions.** The
  AC-004 and AC-009 records each state that the walk was completed keyboard-only
  and name the walker and their relationship to the work. *(project AC-004,
  AC-009, DR-10)*
- **UX-AC-10 — No widget, and the built-in surface was exhausted first.** The
  design record shows intra-doc links, `#[doc(alias)]`, rustdoc search and the
  destination surface's own TOC were each considered; `#[doc(alias)]` has zero
  occurrences in the workspace today, so a decision *not* to use it is a decision
  that must be written down rather than a default. Any widget considered is
  recorded with its reason for declining. *(project AC-011, DR-5)*
- **UX-AC-11 — No page this project authors escapes the theme or hides anything.**
  No raw HTML, no inline styles, no folded or tabbed content, heading ladder
  unskipped. *(project AC-012; accessibility floor)*
- **UX-AC-12 — `_design.md` judges DT-10's three options against stated UX
  criteria, not preference.** At minimum: cost to a reader who arrives at a
  *single* item out of search (the per-item case's benefit), cost of a second
  navigation surface going stale (the per-item case's recorded price — "become a
  second navigation surface to keep true", `initiative.md`, DT-10 row), and
  whether the chosen shape survives the two-surface split HS-P0020 may or may not
  adopt. *(project AC-001, DR-1)*

---

### Notes

**What this brief does not decide.** DT-10 itself. The brief supplies the criteria
(UX-AC-12) and the invariants any resolution must satisfy; the choice among *once*
/ *per-item* / *both-with-one-authoritative* is `_design.md`'s, and because
`design.capture` is absent from `.redkiln/config.yaml` the perceptual review is a
skip — there will be no screenshot pass to catch a bad choice later. The written
resolution is final in a way it would not be on a project with a rendered surface.

**Personas are drafts, and the brief inherits their qualification.** Personas 2 and
3 are not promoted to `.kb/product/`;
`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` carries
its own bar — *"a persona nobody researched is a stock photo with a name"* — and
records that none of the three has been directly observed. Promotion is HS-P0025's
(`_decomposition.md`, BR-13 row). Every "the reader wants…" above therefore reads
as *inferred from the seed's measured findings plus comparable readers elsewhere*,
not as observed behaviour. The one exception is the E0034 findability gap, which
`personas-and-journeys.md` (Risks, third bullet) singles out as "a direct grep of
the workspace, reproducible by anyone" — and this brief re-verified it: no
`= note:` lines and no `TraitVariantBlanketType` anywhere in
`crates/happenstance-core/src/store.rs`.

**No Accepted decision governs any of this.** Confirmed by the initiative's own
audit and by this project's `_grounding.md`: no `.kb/` atom addresses pointer
policy, navigation or documentation structure. Nothing above deviates from an
Accepted ADR because there is none to deviate from. The ADRs this brief touches —
`.kb/decisions/0001-async-port-flavours.md` (why two flavours exist, hence why
E0034 fires) and `.kb/decisions/0008-one-derivation-for-both-ports.md` (the
provided-method rules) — are cited *as the reasoning the account points at*, never
restated (DR-7, DR-8). `.kb/decisions/0006-bare-name-to-the-typed-layer.md` is the
reason the front-door pointer belongs on `happenstance` rather than
`happenstance-core`, and `crates/happenstance/src/lib.rs:20-25` already states that
conclusion in prose — the new pointer must not contradict it.

**Two dependencies bind this brief's ACs and neither has run its design stage.**
HS-P0020's hosting shape leaves UX-AC-01's destination late-bound (hence
UX-AC-02's copy-independent-of-href requirement), and HS-P0021's page-need rule is
what UX-AC-11 ultimately answers to. Both were verified at `stage: storymap` with
no `_design.md` (`_grounding.md`, "Sibling-project dependency state"). Where this
brief needed a rule HS-P0021 has not written yet, it states the requirement in its
own words and expects HS-P0021's rule to subsume it, not contradict it.

**The `text`-fence limit is accepted, not closed.** Nothing in the gate reads a
```` ```text ```` fence, so the restored E0034 excerpt is checked by no mechanism —
`standards/rust/62-doctests-and-harnesses.md` governs compiled fences and a
diagnostic transcript is not one. UX-AC-06 requires the page to say so and requires
the excerpt to stay on the stable parts (the error code, the candidate item names)
rather than pretending to be a byte-exact transcript of one toolchain.

**Merge forward before implementing UX-AC-01.** `crates/happenstance/README.md` is
shared with HS-P0016 on the unmerged
`initiative/from-contract-to-published-library` branch, and
`crates/happenstance/src/lib.rs` diverges hard between branches (75 lines here,
237 there). The decomposition's merge-forward rule applies verbatim. Authoring the
pointer's placement against this branch's 75-line crate root means re-deciding
placement — an invariant-2 judgement, not a mechanical rebase — at merge time.
**NOT VERIFIABLE from this worktree**: the sibling branch's actual current state.

**Anchors.** `.bklg/docs-that-teach/initiative.md` ·
`.bklg/docs-that-teach/_decomposition.md` ·
`.bklg/docs-that-teach/reach-and-adapter-path/project.md` ·
`.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` ·
`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` ·
`.kb/decisions/0001-async-port-flavours.md` ·
`.kb/decisions/0008-one-derivation-for-both-ports.md` ·
`.kb/decisions/0006-bare-name-to-the-typed-layer.md` ·
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` ·
`standards/rust/70-rustdoc-obligations.md` ·
`standards/rust/62-doctests-and-harnesses.md` ·
`standards/rust/20-two-flavour-ports.md` ·
`standards/rust/91-adapter-authoring-recipe.md` ·
`standards/rust/README.md:25-29` ·
`crates/happenstance-core/src/store.rs` ·
`crates/happenstance-core/src/memory.rs` ·
`crates/happenstance/src/lib.rs` · `crates/happenstance/README.md` ·
`docs/README.md` · `.redkiln/config.yaml`

---

## Architecture brief

### Intent

Give the implementer the seams, the render paths and the checking mechanisms for
HS-P0023's four capabilities — the front-door pointer, the evaluator's onward
links, the `store.rs` error-site rewrite, and the sequenced adapter reasoning
account — so that each one **mounts into a surface the gate already builds**
rather than existing as a well-written file nothing renders and nothing checks.

The architectural content of this project is almost entirely *where things
attach and what catches them rotting*. There is no new type, no new trait and no
new public API here; there are four existing render paths, one existing
gate-step shape to extend, and one hard ordering constraint. Everything below is
about those.

**Non-goal, stated once so it is not rediscovered:** nothing in this project
changes a public item, a signature, a feature or a manifest. If a candidate
solution requires one — the `prelude` module discussed under N-9 is the live
example — it is out of this project's boundary and needs an ADR, not a
documentation story.

### Acceptance Criteria

Architecture-grain, keyed to the project's own AC-001..AC-012 in
`project.md`. These are the conditions an implementer's work must satisfy for the
project ACs to be checkable at all.

- **AC-A01 — Every pointer this project installs is one of the four enumerated
  forms in N-3, and its row in the AC-003 inventory names the mechanism from that
  table.** A pointer in a fifth form is either given a mechanism or not installed.
  Traces to project AC-003, AC-011.
- **AC-A02 — No intra-doc link is added that resolves in only some feature
  configurations.** RS-70-2 (`standards/rust/70-rustdoc-obligations.md:95`) and
  the `documentation (no default features)` step in `xtask/src/main.rs` (the
  atom's own evidence cites `xtask/src/main.rs:502`) make this a hard gate
  failure, not a review note. In particular, `crates/happenstance-core/src/store.rs`
  may **not** link `MemoryEventStore`: `store.rs:77-78` already records why, and
  the reasoning account's pointer must survive `--no-default-features`. Traces to
  AC-007.
- **AC-A03 — The reasoning account is a page inside HS-P0020's pinned tree, and
  the pinned tree's own registration check sees it.** It is not a loose file under
  `docs/`, not a new `//!` module doc, and not a section appended to
  `CONTRIBUTING.md`. Concretely: it must be reachable by whatever mechanism
  HS-P0020's AC-005 ("no orphan pages",
  `.bklg/docs-that-teach/checked-documentation-surface/project.md:215-217`) uses,
  or the gate must fail. Traces to AC-008.
- **AC-A04 — The `store.rs` edit happens in the order N-6 states, and the story
  records the three artefacts that order produces:** the clause-id set it read,
  the toolchain the transcript was reproduced on, and the `cargo xtask spec-trace`
  result afterwards. Traces to AC-006, AC-010; DoD item 2.
- **AC-A05 — The E0034 transcript is a reproduction, not a transcription of a
  transcription.** It is produced by compiling a deliberate double-import at the
  pinned toolchain (`rust-toolchain.toml`, `channel = "1.97.1"`) and pasted from
  that run's stderr. `references/evaluation/review-dx-ergonomics.md:404-414` is the
  reference for *what the block should contain* (both `= note:` candidate lines and
  the `help:` line), not the text to copy — it predates the MSRV raise recorded in
  `.kb/decisions/0029-msrv-raised-to-1-97-1.md`. Traces to AC-006.
- **AC-A06 — The page states the *narrow* limit, not the broad one.** What is
  unchecked about the transcript is its *rendering*; the error **code** is already
  gate-checked (N-7). A page that says "nothing checks this" is wrong in the
  direction that loses a real guard. Traces to AC-006, DoD item 5.
- **AC-A07 — Every citation the reasoning account makes into a moving file is
  written in the anchored form of N-8** — a named subject plus a path, never a bare
  line range as the only handle. Traces to AC-008, AC-012.
- **AC-A08 — The adapter-half stories carry no dependency on HS-P0022, and the
  reach-half stories carry both of theirs.** The story map must express the split
  in N-10; collapsing it costs the project its only available parallelism. Traces
  to `project.md`, "Internal ordering worth carrying into the story map".

### Notes

#### N-1. Modules and seams touched, per the `CLAUDE.md` package layout

| Surface | Crate / tree | What this project does to it | Owner of the surrounding file |
| --- | --- | --- | --- |
| `crates/happenstance/src/lib.rs` crate-root `//!` (lines 11-71) | the typed layer | adds **one** pointer line under the DT-10 policy | shared with HS-P0016 on the unmerged sibling branch |
| `crates/happenstance/README.md` | the typed layer's package README | adds the **same** pointer; changes no claim | HS-P0016 owns the claims |
| `crates/happenstance-core/src/store.rs` module `//!` (lines 1-52; the E0034 block at 36-41) | the contract | restores rustc's real output; adds one pointer to the reasoning account | this project, behind HS-P0020's pin |
| HS-P0020's pinned narrative tree | new tree, path constant in `xtask/src/` | adds the reasoning account page (AC-008) and the onward-link targets DoD-9 lands on | HS-P0020 owns the tree and the gate step |
| the AC-003 pointer inventory | data, ideally a `const` read by HS-P0020's checker | authored here, consumed there | see N-4 |

Nothing under `crates/happenstance-testkit/`, no adapter crate, and no
`spec/SPECIFICATION.md` clause is edited by this project. The dependency rule in
`CLAUDE.md` ("everything depends on `happenstance-core`; `happenstance-core`
depends on nothing in this workspace") is untouched, because a doc comment
pointing outward at a rendered page is not a crate dependency — which is also
why the pointer out of `store.rs` cannot be an intra-doc link (AC-A02).

#### N-2. Composition roots — where each capability mounts

There are four, and each is a real render path with an existing owner. A
capability that does not attach to one of these is not delivered.

**CR-1 — the `happenstance` rustdoc root and its README doctest mount.**
`crates/happenstance/src/lib.rs:10` carries
`#![cfg_attr(doctest, doc = include_str!("../README.md"))]`, with the reasoning
in the comment above it (D10). This is the mount that makes AC-002 mechanical
rather than reviewed: the README's fences are recompiled as *this crate's own
doctest*, so a pointer written inside a Rust fence in the README is compiled by
`cargo test`. A pointer written as ordinary markdown prose in the README is
**not** — see the mechanism table. The crate root's own prose renders on
docs.rs; the README's renders on crates.io; the two surfaces do not share a
reader, which is why DR-2 requires the pointer on both.

**CR-2 — the `happenstance-core` rustdoc build, three times.** The `store.rs`
module doc renders into `happenstance-core`'s documentation, which the gate
builds with all features, with `--no-default-features`
(`xtask/src/main.rs`, the `documentation (no default features)` step) and under
nightly `--cfg docsrs`. `rustdoc::broken_intra_doc_links = "deny"`
(`Cargo.toml:134`) makes any unresolved link in any of those three a hard
failure. This is the strictest composition root in the project and the one that
constrains the *form* of the error-site pointer.

**CR-3 — HS-P0020's pinned narrative tree.** The reasoning account (AC-008) and
the pages DoD-9's second questions land on live here, as pages, registered with
whatever mechanism HS-P0020 builds. The precedent for the shape is already in
the tree twice: `cargo xtask spec-trace` parses `spec/` by path and
`cargo xtask lint-constitution` reads `standards/rust/` by path through the
`ATOM_DIR` / `ROUTER` / `HARNESS` constants at
`xtask/src/lint_constitution.rs:54-61`, with `docs/README.md:25-29` stating the
rule ("Moving either tree means editing `xtask/src/` in the same change — which
is the point of pinning them by path rather than by convention"). Note the
registration hole this project inherits: `cfg(doctest)` means a renamed page is
invisible to `cargo test`, which is exactly why `lint-constitution` reads the
*directory* rather than trusting the compiler (`xtask/src/constitution.rs:20-25`).
A reasoning account added to the tree and wired to nothing must fail the gate,
not pass quietly.

**CR-4 — the `REQUIRED` step list in `xtask/src/main.rs`.** Any check this
project needs mounts here, as an ordinary `Step` with `probe: None` (the
`probe` field's doc comment at `xtask/src/main.rs:85-102` is explicit that
`None` means mandatory and that "the tool ran and found a problem" must be a
failure, never a skip). This project does not add a step of its own if
HS-P0020's step can carry the assertion — see N-4.

#### N-3. The four pointer forms, and what catches each one rotting

This table is the substance of AC-003 and the reason DR-3 exists. Pick a form
per pointer *before* writing it.

| Form | Where it may be used | What catches it rotting | Caveat |
| --- | --- | --- | --- |
| Intra-doc link, `[`Item`]` or `[text](self::path)` | any rustdoc surface, pointing at an item in the same crate or a dependency | `rustdoc::broken_intra_doc_links = "deny"` (`Cargo.toml:134`) across three builds | **must resolve in every feature configuration** — RS-70-2, `standards/rust/70-rustdoc-obligations.md:95`; this is the D13 class of bug |
| A link inside a Rust fence in `crates/happenstance/README.md` | the front door, when the pointer can be expressed in compiled code (e.g. a doc-comment example) | the README doctest mount at `crates/happenstance/src/lib.rs:10` | the fence compiles, so its *prose* is still unchecked; only the code is |
| A markdown link on a page inside the pinned tree | the reasoning account, the onward links | HS-P0020's gate step (its AC-002, AC-005, AC-007) | does not exist until HS-P0020 lands; do not author against an assumed shape |
| A bare URL in prose (rustdoc or README) | last resort, external targets only | **nothing** | permitted only with an inventory row saying so and a reason; a bare URL into this repository's own tree is the shape to avoid, because the tree moves and the URL will not |

`#[doc(alias)]` is a fifth thing and is deliberately not in the table: it is not
a pointer, it is a *search key*. It costs nothing per item, it improves the
built-in reach the dossier says to exhaust first
(`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:40-59`),
and it cannot rot independently — it is an attribute on an item, so deleting the
item deletes the alias. Use it freely; it needs no inventory row.

#### N-4. The AC-003 inventory has a working precedent — use it, do not invent one

`xtask/src/lint_constitution.rs:463-473` already implements exactly the check
AC-003 describes, for a different tree: it reads each file in
`SUMMARIES` (`:64`, today `["CLAUDE.md", "CONTRIBUTING.md"]`) and fails if the
text does not contain `ATOM_DIR`, with the failure message carrying the reason —
"a summary that does not point at the corpus is a second copy of it, and one of
the two will be stale". That is a named list of surfaces, crossed with a pinned
tree constant, asserted by the gate.

**The contract this project has with HS-P0020:** the pointer inventory should be
*data of the same shape* — a `const` list of surfaces that the sibling's checker
reads against its own tree constant — rather than a markdown table in a planning
artefact. Author the list here; ask HS-P0020's design to consume it. This
project does not own `xtask/src/` and must not fork a second checker into it.

**If HS-P0020 lands without that hook**, AC-003 is still satisfiable, but only by
choosing pointer forms whose guard is already in the table at N-3, and by
recording in the inventory that the *set* of pointers is unguarded even though
each member is. Say that plainly rather than describing a guard that does not
exist.

#### N-5. The Accepted decisions that constrain this work, and where they bite

- **`.kb/decisions/0001-async-port-flavours.md`** — the two-flavour scheme is the
  *cause* of the E0034 collision this project documents. It is not under revision
  here. The account and the doc comment **cite** it; they do not restate it, per
  the precedence chain at `standards/rust/README.md:25-29` and DR-8.
- **`.kb/decisions/0008-one-derivation-for-both-ports.md`** — the same derivation
  covers `ProjectionStore` and states the provided-method rules that
  `CONTRIBUTING.md:97-140` explains for a contributor. The reasoning account
  sequences both; it must not become a third statement of the rule alongside
  `standards/rust/20-two-flavour-ports.md` and `standards/rust/25-what-removes-send-and-sync.md`.
- **`.kb/decisions/0006-bare-name-to-the-typed-layer.md`** — decides *which front
  door*. `crates/happenstance/src/lib.rs:20-25` already states this ADR's
  conclusion, and `:69-71` routes adapter authors to `happenstance_core`
  deliberately. The new pointer must be consistent with that routing: the
  application-facing narrative is reached from `happenstance`; the adapter
  reasoning account is reached from `happenstance-core`'s `store.rs`. Installing
  the adapter pointer on the bare crate's front door would contradict a standing
  ADR's own text on the same page.
- **`.kb/decisions/0007-projection-runner-decodes.md`** — not load-bearing, but
  cited in the paragraph at `crates/happenstance/src/lib.rs:47-51`. DoD item 7
  makes "changed nothing but the pointer" checkable by diff; do not reflow that
  paragraph.
- **`.kb/decisions/0029-msrv-raised-to-1-97-1.md`** — every fence this project's
  pages carry compiles at the pinned toolchain (`rust-toolchain.toml`,
  `channel = "1.97.1"`), which is also the toolchain AC-A05's transcript must be
  reproduced on. Let-chains are available; nothing here should need them.

**Tension, flagged explicitly and not resolved by any of the above:** no Accepted
decision atom and no `.kb/` atom governs documentation structure, pointer policy
or navigation. This was confirmed by direct search and by the initiative
charter's own audit (`_grounding.md`, "Accepted decision atoms that constrain
this project"). DT-10 is therefore open design space, **not** a deviation from
anything Accepted — and because `design.capture` is deliberately absent from
`.redkiln/config.yaml`, the perceptual review is a permanent skip and
`_design.md`'s written resolution is the only and final record.

#### N-6. The `store.rs` rewrite protocol — order, and the two tests it must pass

The order is structural, not advisory:

1. **HS-P0020's clause-id pin is committed and readable** (its AC-008). Until it
   is, the first edit to `crates/happenstance-core/src/store.rs` may not be made.
   Design and copy may be drafted earlier; implementation may not start.
2. **Read the pinned set** and identify which frozen documentation MUSTs, if any,
   the existing `store.rs:1-52` text discharges.
3. **Edit**, applying both tests below.
4. **Run `cargo xtask spec-trace`** over the resulting tree and record the result
   in the story (DoD item 2).

Two tests, and the *primary* one is not the one `project.md` names first:

- **Primary — `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`.**
  Its test is mechanical and built for exactly this artefact class: a correction
  to a `[FROZEN]` clause is a **repair** if the set of implementations the clause
  admits is unchanged, and otherwise it is a **gap**, which is an ADR's business.
  It also gives the safe form for an obligation already met — keep the MUST
  verbatim, name the discharge as a discharge, cite the code and the test that
  assert it. Restoring the `= note:` lines is a repair by that test: it changes no
  implementation the clause admits.
- **Secondary — `.kb/governance/rewrite-the-referent-never-the-reasoning.md`.**
  "Ask whether the edit changes what the document asserts, not whether it changes
  the document." Apply it, but know what you are applying: **all three of that
  atom's worked instances are ADR-record renames, not doc-comment edits.**
  `CLAUDE.md`'s non-goal language sanctions the application by name, so this is
  not a novel extension — but it is an analogy to a fourth artefact type, not a
  fourth worked instance inside the atom, and the story should say so rather than
  cite it as literal precedent.

#### N-7. What is actually unchecked about the E0034 block — narrower than the risk table says

`project.md`'s risk table says a ```` ```text ```` fence "is invisible to every
gate step", and for that fence that is true. But the *error code itself* is
already gate-checked, twice, elsewhere in the tree:

- `standards/rust/20-two-flavour-ports.md:179` carries a
  ```` ```rust,compile_fail,E0034 ```` fence — a compiled negative example that
  fails the build if importing both flavours ever stops producing E0034;
- `standards/rust/00-prime-directives.md:242` carries the same;
- both are compiled by the `the constitution's examples compile` step
  (`cargo test --locked -p xtask --doc` with `RUSTDOCFLAGS=-D warnings`) through
  `xtask/src/constitution.rs`.

So the honest statement on the page — and the one AC-A06 requires — is: *the
error code is asserted by a compiled fence in the constitution; the wording of
the `= note:` lines and the internal name `TraitVariantBlanketType` are rustc's
rendering, reproduced at 1.97.1, and are not asserted by anything.* That is a
smaller, truer claim than "nothing checks this", and it hands the reader a real
guard instead of none.

Two mechanical details for the rewrite:

- `TraitVariantBlanketType` inside the ```` ```text ```` fence is inert. If it is
  also mentioned in surrounding **prose**, it needs backticks — `clippy::doc_markdown`
  fires on CamelCase in prose, `clippy.toml`'s `doc-valid-idents` list is for
  proper nouns and this is an identifier, and RS-70-3
  (`standards/rust/70-rustdoc-obligations.md:154`) forbids the `allow` repair.
- Keep the excerpt to the parts that are stable across toolchains — the error
  code, the candidate trait and type names — since diagnostic phrasing moves. That
  is the mitigation, not a pretence of checking.

#### N-8. The reasoning account (AC-008): sequencing contract, citation form, and the required caveat

**Shape.** A page in CR-3 whose job is *order*, not content. Six in-tree sources,
in the sequence an adapter author needs:

1. `crates/happenstance-core/src/memory.rs:16-71` — the runnable full-DCB-loop
   walk-through, already a doctest;
2. `standards/rust/91-adapter-authoring-recipe.md` — RS-91-1..4, the six-step
   recipe with a compiled `PgStore` implementing `SendEventStore`;
3. `CONTRIBUTING.md:69-95` — the four-step recipe in contributor voice;
4. `standards/rust/20-two-flavour-ports.md` — why two flavours, and the E0034
   collision as a compiled negative;
5. `standards/rust/25-what-removes-send-and-sync.md` — what removes the bounds;
6. `CONTRIBUTING.md:97-140` — why a provided method is never `async fn`.

**Decide (2) explicitly.** `standards/rust/91-adapter-authoring-recipe.md` is a
fourth reasoning source beyond DR-7's three, and it is the atom carrying the
compiled non-memory adapter example. Omitting an already-written, directly
on-topic atom from a page whose entire premise is findability over invention
would be the odd choice; the recommendation is to include it, and to record the
decision either way so a reviewer does not have to guess.

**Citation form (AC-A07).** These are `file:line` citations into a tree that
moves under them — the exact problem
`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` was written for
and `xtask/src/spec_trace.rs` implements (a windowed search for a short subject
string, "a heuristic that cannot tell its own mistakes from the corpus's must
decline rather than guess";
see `xtask/src/spec_trace.rs:377-382` and `:2103-2130`). Write every citation as
**named subject + path**, with the line range as a convenience rather than the
only handle, so an insertion above the target does not silently mis-point the
page.

**The required caveat, not optional prose.** `MemoryEventStore` is the
conformance suite's oracle and the reference implementation, not an adapter
(`crates/happenstance-core/src/memory.rs:16-31`). Sequencing it as "the adapter
walk-through" without saying what it is *not* teaches that `RwLock<Vec<_>>` is
the shape an adapter takes — the precise monoculture `CLAUDE.md` warns about
under "A port is only as well-designed as the *spread* of what implements it".
The account must name at least one adapter at the other end of that axis;
`standards/rust/91`'s `PgStore` example and `references/adapter-shapes.md` are
the in-tree material for it.

**And the account is a page, so it inherits HS-P0021's discipline** (AC-012): one
named answered-need, normative claims as resolving citations into
`spec/SPECIFICATION.md`, no clause restated. That rule set does not exist yet —
HS-P0021 is still `stage: storymap` with no `_design.md`
(`_grounding.md`, "Sibling-project dependency state"). Author against the
initiative's stated shape and expect a conformance pass once the rule lands;
do not invent the rule here.

#### N-9. Considered and declined here, with reasons (feeds AC-011's rejection record)

- **A bespoke navigation widget** — breadcrumb, master-detail rail, per-crate
  index page. Declined per DR-5 and the dossier's anti-pattern list
  (`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:382-392`
  and `:436-441`): rustdoc renders a per-item nav bar and any narrative surface
  renders a sidebar TOC, so the evaluator's gap "is evidence of a missing *link*,
  not a missing *widget*". Record the consideration and the reason; that record is
  AC-011's deliverable.
- **A `prelude` module exporting `EventStore` and not `SendEventStore`.**
  `references/evaluation/review-dx-ergonomics.md:421-422` calls this "the single
  highest-leverage doc fix in the crate", and it would make the E0034 collision
  unreachable by the default import path. No such module exists today (verified:
  neither `crates/happenstance-core/src/lib.rs` nor `crates/happenstance/src/lib.rs`
  declares one). **It is out of this project's boundary**: it is a public API
  addition with a semver surface, and this project changes no public item. Record
  it as a live, evidenced proposal owed to the typed layer and an ADR — do not
  quietly implement it, and do not let its absence be read as a rejection.
- **Deleting the trimmed excerpt rather than restoring it.** Declined: the
  excerpt's location is the whole value (BR-15 — the explanation meets the reader
  where the error fires); the defect is the *rendering*, not the presence.

#### N-10. Sequencing the implementer inherits

Two halves, and only one of them is blocked on the sibling content project.

| Half | Stories | Blocked on |
| --- | --- | --- |
| **Adapter** — AC-006, AC-007, AC-008, AC-009, AC-010 | the `store.rs` rewrite, the reasoning account, the error walk | AC-001's policy (internal); HS-P0020's clause-id pin **and** its pinned tree existing. **Not** HS-P0022. |
| **Reach** — AC-002, AC-004, AC-005 | the front-door pointer, the two walks | HS-P0022's pages existing (there is nothing to point at otherwise); HS-P0020's hosting shape for the destination path |

Two things that will bite if they are not planned for:

- **Merge forward before implementing AC-002.**
  `crates/happenstance/src/lib.rs` is 75 lines on this branch and 237 on
  `initiative/from-contract-to-published-library`, and
  `crates/happenstance/README.md` is edited by HS-P0016 there. The decomposition's
  operational rule was written for HS-P0022 and applies verbatim here
  (`.bklg/docs-that-teach/_decomposition.md`, "Decisions taken at the gate").
  **NOT VERIFIABLE from this worktree** — that branch is absent; re-verify against
  the merged tree before binding any sequencing to it.
- **The destination path is not knowable yet.** HS-P0020 has not authored
  `_design.md`, so the hosting shape (docs.rs-only versus a separate rendered
  surface) is open. Author AC-002's pointer *text* and its *form* (N-3) against a
  symbolic destination and bind the concrete path at implementation. The DT-10
  policy decision itself does not wait on this — that is precisely the parallelism
  the decomposition names ("HS-P0023's DT-10 pointer policy can be resolved in
  design while HS-P0022 is still authoring").

#### N-11. Deliberately non-prescriptive

Left to `_design.md` and to the implementer, and named here so the silence is
visible rather than accidental:

- **Which of DT-10's three options wins** (one pointer, per-item pointers, or
  both with one authoritative). This brief constrains only the *mechanism* each
  option must carry (N-3, N-4) and states that a per-item policy without a guard
  mechanism is the wrong policy.
- **The wording of every pointer.** One line is the ecosystem norm
  (`interaction-patterns.md:86-110`, the tokio/serde/diesel convergence); the
  sentence itself is a copy decision, not an architectural one.
- **Which two second questions AC-005 walks.** DR-4 names the candidate set;
  choosing from it is the UX brief's and the author's call, and the honest claim
  stays "two plausible second questions were walked".
- **Whether the reasoning account is one page or two.** Six sources in a sequence
  may or may not fit one page under HS-P0021's one-need rule. Decide after that
  rule lands; either shape satisfies AC-A03.
- **Where the account's `#[doc(alias)]` keys go, if any.** Free, unguarded and
  low-risk; use judgement.

**Evidence.** `.kb/decisions/0001-async-port-flavours.md` ·
`.kb/decisions/0008-one-derivation-for-both-ports.md` ·
`.kb/decisions/0006-bare-name-to-the-typed-layer.md` ·
`.kb/decisions/0007-projection-runner-decodes.md` ·
`.kb/decisions/0029-msrv-raised-to-1-97-1.md` ·
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` ·
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` ·
`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` ·
`standards/rust/70-rustdoc-obligations.md:95,154` ·
`standards/rust/20-two-flavour-ports.md:157,179` ·
`standards/rust/00-prime-directives.md:242` ·
`standards/rust/25-what-removes-send-and-sync.md` ·
`standards/rust/91-adapter-authoring-recipe.md` ·
`standards/rust/README.md:25-29` ·
`crates/happenstance-core/src/store.rs:1-52,36-41,77-78` ·
`crates/happenstance-core/src/memory.rs:16-31,16-71` ·
`crates/happenstance/src/lib.rs:10,20-25,47-51,69-71` ·
`crates/happenstance/README.md` ·
`xtask/src/main.rs:85-102` (the `probe` contract) and the
`documentation (no default features)` step ·
`xtask/src/lint_constitution.rs:54-61,64,463-473` ·
`xtask/src/constitution.rs:20-25` · `xtask/src/spec_trace.rs:377-382,2103-2130` ·
`Cargo.toml:134` · `clippy.toml` · `rust-toolchain.toml` · `docs/README.md:25-29` ·
`CONTRIBUTING.md:69-95,97-140` ·
`references/evaluation/review-dx-ergonomics.md:404-414,421-422` ·
`references/adapter-shapes.md` ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:40-59,86-110,382-392,436-441` ·
`.bklg/docs-that-teach/checked-documentation-surface/project.md:200-235` ·
`.bklg/docs-that-teach/_decomposition.md` · `.redkiln/config.yaml`
