---
item: HS-S0154
stage: spec
created: 2026-08-17T13:16:12.681Z
updated: 2026-08-17T13:16:12.681Z
template_sig: 87bbf1d0
rendered_sig: a974d8d2
---

# Spec — The pointer policy, landed as in-tree substrate

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` |
| This spec | `.bklg/docs-that-teach/reach-and-adapter-path/pointer-policy-and-inventory/spec.md` |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` |
| Briefs (UX + architecture, one file) | `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` |
| Grounding (verified anchors) | `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` |
| Story map / merge order | `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` |
| Sign-off mock (context only) | `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` |
| Roadmap pointer | none — this initiative carries no separate roadmap artefact; `_storymap.md`'s **Merge order** is the sequencing of record |

Story slug `pointer-policy-and-inventory` · item `HS-S0154` · archetype **foundation** ·
slice **`pointer-policy`** · `depends_on: []` · blocks `HS-S0155`, `HS-S0156`, `HS-S0158`,
`HS-S0159` · traces to project **AC-001, AC-003, AC-011**.

## One-line PR slice

Land `_design.md`'s DT-10 resolution as in-tree substrate: the permitted pointer forms, the
enumerated pointer inventory with a named rot-guard per row, and the widget-rejection record —
so no later story chooses a pointer shape case by case.

## Executive summary

This PR adds one compiled module — `xtask/src/pointers.rs` — and mounts it at
`xtask/src/lib.rs`. The module carries three things a later story reads *instead of* re-reading
a design document: the DT-10 policy as prose in its module doc, the pointer register as a
`const` slice of typed rows, and a pure `validate` function plus its unit tests, which is what
turns "every row names its guard" from a review habit into a build failure.

**The delta is that the register stops being a table and becomes data.** `_design.md` already
decided DT-10 (option (c), both-with-one-authoritative, bound by the rule "one front door, and
a pointer only at a stall") and already enumerated the five rows the project will end up with.
What does not exist anywhere in this tree is a *place those rows can be filed* and a *thing
that fails when a row is filed badly*. The architecture brief is explicit that the inventory
should be "data of the same shape" as `SUMMARIES` at `xtask/src/lint_constitution.rs:64`
rather than a markdown table in a planning artefact, and the design's `## Shape decision`
rejects the markdown table by name because it "rots invisibly". This story is that shape.

**The register lands empty.** Zero rows. Each installing story
(`store-error-site-rewrite`, `front-door-pointer`, `evaluator-onward-links`) files its own row
in the same PR that installs its pointer — `_storymap.md`'s AC-003 coverage note: "A story that
installs a pointer and files no row has not finished." The validator therefore has to reject
bad rows it has never seen, which is why it is written as a pure function over a slice and
exercised against deliberately-wrong registers in tests, not as an assertion over the one
register that happens to exist.

**Nothing here renders to a reader**, and that is the design's transience call rather than an
omission: "Rendering the register creates precisely the second navigation surface DT-10 warns
about: a page listing every pointer is a navigation surface whose only reader is its
maintainer."

## Context pack

Read this section and you can start. Everything deeper is an anchor below.

### The decision this story makes binding — DT-10, option (c), bound by a rule

`_design.md` resolved DT-10 to **both, with one authoritative**, and the resolution is worth
nothing as a permission slip; the binding half is the rule, which this story encodes:

1. **Exactly one authoritative text.** One sentence, authored once, mirrored byte-identically
   onto both front-door surfaces (`crates/happenstance/src/lib.rs` crate root and
   `crates/happenstance/README.md`). Two renderers, two readers, **one authority — not two
   pointers.**
2. **A secondary pointer is permitted at an item only where all four gates hold**, and the
   register row records which:
   - **(i) evidenced stall** — a named, *recorded* reader failure at that item (a compiler
     diagnostic, a measured second question), not a suspicion. BR-15 is the only one this
     project has.
   - **(ii) the front door provably cannot reach it** — the item is arrived at by deep link,
     search, or a diagnostic.
   - **(iii) subordinate and one line** — it follows the in-place fix, never precedes it, and
     introduces no heading of its own.
   - **(iv) a guard from the mechanism table** — its form is one of rows 1–3 of the
     architecture brief's N-3 table. **Row 4 (a bare URL, guarded by nothing) is forbidden to
     this project outright.**
3. **The href ladder, applied per pointer and recorded in its row:** intra-doc link (guarded by
   `broken_intra_doc_links = "deny"`, `Cargo.toml:134`, across three rustdoc builds) **>** an
   in-tree markdown link inside HS-P0020's pinned tree (guarded by its registration check) **>**
   a named-but-unlinked cross-reference in the form already live at
   `crates/happenstance-core/src/store.rs:77` **>** *nothing*. **If only the fourth rung is
   available the pointer is not installed and the project escalates.**
4. **Aliases are not pointers and need no row.** `#[doc(alias)]` is a *search key*: an attribute
   on an item, so deleting the item deletes the alias — it cannot rot independently.

Do not re-litigate any of the four. The perceptual review is a standing skip
(`design.capture` is deliberately absent from `.redkiln/config.yaml`), so `_design.md`'s
sign-off — Ryan Britton, 2026-08-17 — was the last cheap opportunity to disagree, and its
condition 2 is explicit: "DT-10's pointer policy is authoritative for siblings. Any project
adding a pointer applies the four gates recorded here rather than re-deciding them."

### Where it lands, and the ownership line that decides that

The architecture brief draws one line and it is the load-bearing constraint on this story:
**"Author the list here; ask HS-P0020's design to consume it. This project does not own
`xtask/src/` and must not fork a second checker into it."** (N-4.) Its table row for the
inventory reads "data, ideally a `const` read by HS-P0020's checker | authored *here*, consumed
*there*" (N-1). A `const` has to live in Rust source, and `xtask/` is the only Rust source in
this repository that is not a published crate.

So the decision, taken here so no later story re-takes it:

- **Land the data and the policy — yes.** `xtask/src/pointers.rs`, declared `pub mod pointers;`
  at `xtask/src/lib.rs`.
- **Land a checker over the pointer-bearing surfaces — no.** No new `Step`, no new entry in
  `REQUIRED` (`xtask/src/main.rs`), no traversal of `crates/happenstance/README.md` or any
  rustdoc surface. That step is HS-P0020's, and CR-4 says this project "does not add a step of
  its own if HS-P0020's step can carry the assertion".
- **The self-checks this story *does* add are about the register's own rows**, run as ordinary
  `#[cfg(test)]` unit tests under `cargo test --workspace`. Validating the shape of your own
  data is not forking a checker into someone else's tree.

**Lib, not bin, and this is not a coin flip.** `xtask/src/main.rs:64-70` is the bin crate root
with its own private modules; `xtask/src/lib.rs` is a *separate* crate root carrying only
`mod constitution;`. The register must go in the lib for two reasons: a `pub` item in the lib is
reachable, so `dead_code` never fires on a register nothing reads yet, and a bin target can
`use xtask::pointers::…` from its own package — which is exactly how HS-P0020's future step
consumes it without this story reaching into the bin.

### The mechanism table every row must draw from (N-3), and what catches each form rotting

| # | Form | What catches it rotting |
| --- | --- | --- |
| 1 | Intra-doc link, `` [`Item`] `` or `[text](self::path)` | `rustdoc::broken_intra_doc_links = "deny"` (`Cargo.toml:134`) across three rustdoc builds. **Must resolve in every feature configuration** — RS-70-2 |
| 2 | A link inside a Rust fence in `crates/happenstance/README.md` | the README doctest mount at `crates/happenstance/src/lib.rs:10`. **The fence compiles, so its *prose* is still unchecked; only the code is** |
| 3 | A markdown link on a page inside HS-P0020's pinned tree | HS-P0020's registration check. Does not exist until HS-P0020 lands |
| 4 | A bare URL in prose | **nothing** — forbidden to this project by rule 2(iv) |

**The over-broad claim this story must not inherit.** `project.md`'s AC-002 says the README's
pointer is guarded because the README is compiled as a doctest. `_design.md` measured that and
found it false for the placement it chose: the mount at `crates/happenstance/src/lib.rs:10`
compiles the README's ```` ```rust ```` fences, **not its prose**, so a prose pointer is *not*
guarded by it. The register's P2 row must therefore carry the honest guard ("the mirror
assertion — does not exist yet") rather than the flattering one, and the concrete ask is
handed to HS-P0020 rather than assumed.

### The register's shape, its cap, and what the cap means

Five rows are foreseen (P1 the crate root, P2 the README, P3 `store.rs`, P4/P5 the two
onward links). The cap is **≤ 8 rows at this project's close, ≤ 20 rows ever**, and the design
attaches meaning to it: "**A 21st row means the pointer policy is wrong, not that the register
needs a scrollbar** — it is the DT-10 alarm, wired to a number." This story is what wires it to
a number.

Two invariants come from the accessibility floor and the design's anti-patterns, and both are
mechanical enough to assert on a row:

- **Self-describing link text** — "≥ 3 words and a noun phrase naming the destination — never
  'here', 'this', 'see this page', 'docs', or a bare URL" (anti-pattern 3).
- **Land on the answer, not the top of the page** — where the answer is not on the
  destination's first screen, the pointer targets a *fragment* (UX-AC-04, invariant 3). The row
  declares whether it does, so the two facts cannot drift apart.

### The rejection record (AC-011), which is a policy artefact and not a per-pointer one

AC-011 asks for a widget considered and declined, on record. The consideration exists (N-9):
a breadcrumb, a master-detail rail, a per-crate index page — declined because rustdoc renders a
per-item nav bar and any narrative surface renders a sidebar TOC, so the evaluator's gap "is
evidence of a missing *link*, not a missing *widget*". `_storymap.md` assigns the record to
**this** story and states why: "The rejection is a policy artefact, not a per-pointer one; the
installing stories inherit it as a constraint." It therefore travels with the policy, in the
module doc, where a later implementer reading the register meets it — not in a planning file
they will not open.

The other declined candidate belongs in the same record and must not read as a rejection: a
`prelude` module exporting `EventStore` and not `SendEventStore` would make the E0034 collision
unreachable by the default import path, and `references/evaluation/review-dx-ergonomics.md:421-422`
calls it "the single highest-leverage doc fix in the crate". **It is out of boundary — a public
API addition with a semver surface, owed an ADR — and its absence here is a scoping decision.**

### The persona-journey slice this realizes

This story is upstream of both personas rather than inside either journey, and being honest
about that is part of the spec. Persona 2 (the adapter author) arrives at
`happenstance_core::store` from a diagnostic and never sees the crate root; Persona 3 (the
evaluator) reads inside one session and never leaves it. Those are the two readers the four
gates exist for: gate (ii) is Persona 2's arrival path written as a test a pointer must pass,
and rule 1's single authoritative sentence is Persona 3's session written as a constraint on
copy. The slice this story delivers is the one *the maintainer* performs — B1 in
`_storymap.md`'s backbone: "DT-10 is resolved and the resolution is *data in the tree*, not a
preference in a review comment."

### Standing constraints inherited, stated once

- **No public item, signature, feature or manifest change** in any `happenstance*` crate.
  `xtask` is `publish = false`, so `pub mod pointers;` adds no semver surface anywhere
  (`_design.md`, `## Items`: "No public item is added, changed or removed by this project").
- **No widget, no raw HTML, no inline styles, no folded or tabbed content**, no skipped heading
  levels, and no bare URL into this repository's own tree.
- **`CLAUDE.md`'s binding constraints are untouched** — no `#[async_trait]`, no `serde` in
  `happenstance-core`'s defaults, `read` returns the stream at the top level, generic code binds
  `EventStore`. This story adds no port code at all.
- **Never run `redkiln adopt --templates`.**

## Integration contract

This story is delivered **mounted**. A `pointers.rs` that compiles but is declared nowhere is
the foundation-as-fixme this archetype forbids.

- **Archetype**: `foundation` — real in-tree substrate consumed by four capability slices in
  this same project. Not a double, not a `todo!()`.
- **Slice / milestone**: `pointer-policy`. **Slice-mates: none** — this story is the sole member
  of its milestone, and it blocks `adapter-reasoning-account` (HS-S0155),
  `store-error-site-rewrite` (HS-S0156), `front-door-pointer` (HS-S0158) and
  `evaluator-onward-links` (HS-S0159). It is the only total order in the project's graph
  (`_storymap.md`, Merge order).
- **Mount point**: **`xtask/src/lib.rs`** — the lib crate root, which today declares
  `mod constitution;` and nothing else. This story adds `pub mod pointers;` beside it and
  amends the crate-level `//!` doc, whose current first line ("Nothing but a home for the
  repository README's doctests") stops being true the moment the module is added. That
  declaration is what puts the register inside `cargo clippy --all-targets --all-features -D
  warnings`, `cargo test --workspace` and the gate's `documentation` step.
- **Wires into**:
  - `xtask/src/lint_constitution.rs:64` (`SUMMARIES`) and `:463-473` (`check_summaries`) — the
    working precedent whose *shape* the register copies. Read, not modified, not imported.
  - `xtask/src/main.rs:64-70` and its `REQUIRED` step list — **deliberately not touched**; the
    seam is that HS-P0020's future step does `use xtask::pointers::…` from the bin, which a
    same-package bin may do without any change here.
  - `Cargo.toml:100-137` — the workspace lint policy the module is written against:
    `missing_docs = "warn"`, `missing_debug_implementations = "warn"`, `unreachable_pub =
    "warn"`, `clippy::pedantic`, `unwrap_used = "deny"`, all under CI's `-D warnings`.
  - `crates/happenstance-core/src/store.rs:77` — the named-but-unlinked cross-reference form the
    href ladder's third rung points at. Cited by the policy; the file is not edited by this
    story.
- **Renders surfaces**: **none.** None of `_design.md`'s five surface ids
  (`crate-root-front-door`, `readme-front-door`, `store-module-error-site`,
  `adapter-reasoning-account`, `evaluator-onward-links`) is rendered or changed here. That is
  the design's `## Transience policy` decision for the register — "not rendered to a reader at
  all — build-time data only" — and the reason is that a page listing every pointer *is* the
  second navigation surface DT-10 warns about. The five surfaces are rendered by this story's
  four dependents.
- **Public items**: `_design.md`'s `## Items` block declares `path: ""` — **no public item**.
  This story claims none of it and adds none. The `pub` items it does add are in `xtask`, which
  is `publish = false` and is not a surface the `## Items` block describes.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** No port, no value
  type, no testkit rule is touched; `happenstance-testkit`'s suite cannot observe a doc-pointer
  policy, and inventing a rule for it would be exactly the decorative rule `CLAUDE.md` forbids.
  The behaviour is observed instead by unit tests in `xtask/src/pointers.rs` that feed the
  validator deliberately-wrong registers — the same discipline (a named wrong implementation
  the check rejects) in the medium this story ships into.
- **Clause(s)**: **none discharged, none amended.** No `spec/SPECIFICATION.md` clause is edited
  by this project at all (N-1), so no `[FROZEN]` clause is touched and no ADR is owed.
  `cargo xtask spec-trace` must still pass, as a regression check rather than a claim.
- **Advances DoD scenario**: initiative **DoD 7** ("@smoke — the reader reaches the teaching
  from the front door") is the scenario this moves toward green, by making the pointer that walk
  depends on installable under a guard rather than under memory; **DoD 9** and **DoD 10** ride
  the same substrate through `evaluator-onward-links` and `store-error-site-rewrite`. Nothing
  turns green in this PR — a foundation story's honest claim is that three scenarios become
  *reachable*, and the project-level bar it directly makes satisfiable is **project DoD item 6**:
  "The pointer inventory of AC-003 exists in one place and every entry names its guard."

## PR boundary

**In this PR**

- `xtask/src/pointers.rs` — new. The policy as module doc, the `Pointer` row type and
  `PointerForm` enum, the `POINTER_REGISTER` const (empty), the `validate` function, and the
  `#[cfg(test)]` tests including the deliberately-wrong registers.
- `xtask/src/lib.rs` — the mount: `pub mod pointers;` plus the crate-doc amendment that keeps
  the file's own first sentence true.
- This story's backlog folder — the ledger the second pass authors, and the implementation
  report.

**Explicitly not in this PR**

- Any pointer, on any surface. This story installs zero pointers and files zero rows; the
  register lands empty on purpose.
- Any edit to `crates/happenstance/src/lib.rs`, `crates/happenstance/README.md` or
  `crates/happenstance-core/src/store.rs` — the first two belong to `front-door-pointer`
  (HS-S0158) and the third to `store-error-site-rewrite` (HS-S0156), which sits behind
  HS-P0020's clause-id pin.
- Any new `Step`, any change to `REQUIRED` in `xtask/src/main.rs`, and any checker that reads a
  pointer-bearing surface. That is HS-P0020's step and forking a second one is forbidden by
  N-4.
- Any `#[doc(alias)]` attribute. The two evidenced strings (`E0034`,
  `TraitVariantBlanketType`) are installed by `store-error-site-rewrite`; this story only
  records the *rule* that admits them and the fact that an alias needs no register row.
- Any `.kb/` atom. The pointer policy is project substrate, not settled knowledge; promoting it
  is a closeout concern and hand-writing atoms is forbidden (`CLAUDE.md`: atoms are authored by
  `/redkiln:kb-ingest` from `.kb/_intake/`).
- Any `spec/SPECIFICATION.md` clause, any adapter crate, anything under
  `crates/happenstance-testkit/`.

The implementer **may** touch the composition-root file named in the Integration contract
(`xtask/src/lib.rs`) to mount this slice; that is the mount, not scope drift.

**Merge DoD (one line)**: `cargo xtask ci --fast` is green with `pub mod pointers;` declared,
the validator rejects every one of the four wrong registers in its tests, and the register is
present, empty, capped and unrendered.

```
xtask/src/pointers.rs
xtask/src/lib.rs
.bklg/docs-that-teach/reach-and-adapter-path/pointer-policy-and-inventory/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The policy is one file an implementer opens, not a design document they re-read** | `xtask/src/pointers.rs`'s module `//!` states DT-10's resolution (option (c)), the four gates (i)–(iv) verbatim in substance, the href ladder with its escalation rung, the four N-3 forms with the guard each carries, and the standing prohibition on a bare URL into this repository's own tree. A later story reads this and needs no other artefact to choose a pointer shape. | `_design.md` `## Pattern decision` ("The rule — one front door, and a pointer only at a stall"); `_storymap.md` one-liner for this story |
| **The register is data, and it is a `const` slice** | `pub const POINTER_REGISTER: &[Pointer] = &[];` — a named list of typed rows, shaped after `SUMMARIES` at `xtask/src/lint_constitution.rs:64` so HS-P0020's checker can consume it rather than transcribe it. It lands **empty**; each installing story appends its own row in the same PR that installs its pointer. | `_decomposition.md` N-1 ("authored here, consumed there"), N-4; `_storymap.md` AC-003 coverage row |
| **A row carries everything a guard claim needs, so the claim cannot be vague** | `Pointer { id, surface, destination, form: PointerForm, guard, link_text, answer_on_first_screen: bool, targets_fragment: bool, gates: &'static str }`. `id` is the `P1`…`Pn` label; `guard` is prose naming the mechanism; `gates` records which of (i)–(iv) admitted a secondary pointer, and is empty for the two front-door rows, which are rule 1 not rule 2. Both the row type and the enum derive `Debug` (`missing_debug_implementations = "warn"`). | `_design.md` `## Density budget`, "The five register rows this project installs" table; `Cargo.toml:104` |
| **The four permitted forms are an enum, so a fifth cannot be improvised** | `PointerForm::{IntraDoc, ReadmeRustFence, PinnedTreeMarkdown, BareUrl}` — one variant per N-3 row, each documenting its own guard on the variant. `BareUrl` exists so the validator can *reject* it by name; a form the type cannot express is a form the validator cannot report. | `_decomposition.md` N-3; `_design.md` `## Shape decision`, "bare-URL pointers — forbidden outright to this project" |
| **`validate` is a pure function over a slice, not an assertion over the live register** | `pub fn validate(rows: &[Pointer]) -> Result<(), Vec<String>>`, returning **every** problem rather than the first, with each message naming the row id and the reason — the `check_summaries` message style, where the failure carries its own rationale. Purity is what lets the tests feed it registers that do not exist, which is what stops the checks being vacuous while the real register is empty. | `xtask/src/lint_constitution.rs:463-473`; `CLAUDE.md`, "A rule that no adapter can fail is decorative" |
| **An empty guard is rejected** | A row whose `guard` is empty or whitespace fails validation. This is the single check that makes AC-003 mean anything: "No pointer is installed whose only guard is memory." | `project.md` AC-003; `_design.md` anti-pattern 18; `_decomposition.md` UX brief invariant 9 |
| **A bare URL is rejected outright** | `PointerForm::BareUrl` fails validation unconditionally — N-3 row 4 is permitted in the general table and **forbidden to this project** by rule 2(iv). The message says so, and says the escalation the href ladder prescribes rather than suggesting a workaround. | `_design.md` `## Pattern decision` rule 2(iv) and `## Shape decision`, last row |
| **Non-self-describing link text is rejected** | `link_text` must be ≥ 3 whitespace-separated words and must not be, or begin with, any of `here`, `this`, `see this page`, `docs`, `read more`, or `http://` / `https://`. Comparison is case-insensitive and trims trailing punctuation, because "See this page." is the same defect wearing a full stop. | `_design.md` `## Density budget` ("the visible link text is ≥ 3 words and is a noun phrase"), anti-pattern 3; UX-AC-03 |
| **A row that admits its answer is not on the first screen must target a fragment** | `answer_on_first_screen == false && targets_fragment == false` fails validation. The two booleans exist together so the row cannot claim reachability it does not have — invariant 3's falsification is precisely "an entry whose destination is a page rather than a passage, where the answer is not on the first screen". | `_decomposition.md` UX brief invariant 3, UX-AC-04; `_design.md` `## Composition`, `evaluator-onward-links` |
| **Row ids are unique** | Two rows sharing an id makes every message ambiguous and lets a story overwrite a sibling's row while believing it filed its own. Rejected with both surfaces named. | `_storymap.md` AC-003 coverage: "each installing story owns its own *rows*" |
| **The cap is a number, and exceeding it is an alarm with a stated meaning** | `> 8` rows fails validation now; the `20`-ever ceiling is documented on the constant with the design's sentence attached — a 21st row means the policy is wrong, not that the register needs a scrollbar. The message must not read as an arithmetic complaint. | `_design.md` `## Density budget`, register row |
| **Each rejection has a named wrong register in the tests** | `#[cfg(test)]` builds one deliberately-wrong register per rule — empty guard, `BareUrl`, `link_text: "here"`, first-screen-false-with-no-fragment, duplicate ids, nine rows — asserts `validate` reports it, and asserts the message names the offending row id. A validator whose tests only feed it good data is decorative. | `CLAUDE.md`, "name a plausible wrong implementation it rejects, and write that implementation into the testkit's own `tests/`" |
| **The empty register passes, and that is asserted too** | `validate(POINTER_REGISTER)` is `Ok` at land. Without this the module compiles green while being untested against its own data, and the first story to file a row discovers the validator was never wired to it. | `_design.md` `## Density budget` (5 rows foreseen, 0 at land) |
| **The widget rejection is recorded where the policy lives** | The module doc records the declined bespoke navigation widget (breadcrumb, master-detail rail, per-crate index page) with its reason — rustdoc renders a per-item nav bar and any narrative surface renders a TOC, so the gap "is evidence of a missing *link*, not a missing *widget*" — and records the `prelude` proposal as **out of boundary and owed an ADR, not rejected**. | `_decomposition.md` N-9; `project.md` AC-011; `_storymap.md` AC-011 coverage row |
| **`#[doc(alias)]` is recorded as a search key that needs no row** | The policy states the alias rule (permitted only where the searched string is one rustc, the specification, or a recorded reader question actually emits, and is not the item's own name), that an alias cannot rot independently because deleting the item deletes it, and the named residual risk: the alias *string* can go stale if rustc renames the internal type and nothing catches that. | `_design.md`, "`#[doc(alias)]` — considered, and **used**, narrowly"; `_decomposition.md` N-3 closing paragraph |
| **The over-broad README guard claim is corrected in the substrate** | The policy states that the doctest mount at `crates/happenstance/src/lib.rs:10` compiles the README's Rust fences and **not its prose**, so a prose pointer there is unguarded by it, and that the concrete `check_summaries`-shaped ask is handed to HS-P0020 rather than assumed. A later story must not file P2 with a guard the design already measured as false. | `_design.md` `## Density budget`, "Two named gaps, not assumed away" (1); `_decomposition.md` N-3 row 2 caveat |
| **This story adds no gate step and no second checker** | No `Step` is added to `REQUIRED`; nothing reads `crates/happenstance/README.md` or any rustdoc surface; the only new tests are unit tests over the module's own data. The seam left open is `use xtask::pointers::…` from the bin, which HS-P0020's step takes when it lands. | `_decomposition.md` N-4 ("must not fork a second checker into it"), CR-4 at N-2 |
| **The mount keeps its own file honest** | `xtask/src/lib.rs`'s crate doc currently opens "Nothing but a home for the repository README's doctests." Adding `pub mod pointers;` makes that false, so the sentence is amended in the same change. | `xtask/src/lib.rs:1` |
| **The gate is green with it, on the terms the workspace already sets** | Every `pub` item documented (`missing_docs`), `Debug` on both new types, no `unwrap`/`expect`, `clippy::pedantic` clean, and `cargo xtask spec-trace` still passing. `xtask` is `publish = false`, so nothing here is a semver promise. | `Cargo.toml:100-137`; `xtask/Cargo.toml:7`; `project.md` DoD 1 |

**One behaviour this story deliberately cannot enforce, named rather than faked.** Nothing here
can detect that a story installed a pointer and *filed no row* — the validator sees the register,
not the diff. That obligation lives as policy in the module doc and as an acceptance criterion in
each installing story's own spec, and the honest statement of it belongs in the substrate rather
than a check that pretends to cover it.

## Data and migrations

**N/A — no persisted data, no schema, no migration.** This story adds a compile-time `const`
in a `publish = false` binary-support crate. There is no database in this workspace, no
serialized format is defined or changed (`happenstance-core` keeps payloads as opaque `Bytes`,
ADR-0003), and `POINTER_REGISTER` is read only by Rust source at build time.

The one thing with migration-shaped semantics is the register's **growth path**, and it is
append-only by convention rather than by mechanism:

| Step | Who | What changes |
| --- | --- | --- |
| land | this story | `POINTER_REGISTER = &[]`, cap 8, validator + tests |
| P3 | `store-error-site-rewrite` | appends one row for `crates/happenstance-core/src/store.rs` |
| P1, P2 | `front-door-pointer` | appends the crate-root and README rows |
| P4, P5 | `evaluator-onward-links` | appends the two onward-link rows |
| later | HS-P0020 | adds the `check_summaries`-shaped step that reads the register; **no change to this data's shape is expected**, and if one is needed it is HS-P0020's change, not a migration of this story's |

No row is ever edited in place to point somewhere else without its guard cell being re-checked;
that is what the guard column is for, and it is the whole reason the register is data.

## Acceptance criteria

Ten criteria, framed as the goal a person is pursuing across the whole stack rather than as a
capability the module has. Two of the three personas are downstream of this story and one is
upstream of it, so the frames are honest about who is standing where: **Persona 2** (the adapter
author) arrives at `happenstance_core::store` from a diagnostic, **Persona 3** (the evaluator)
reads inside one session with no second attempt, and **the maintainer** performs backbone B1 —
deciding once how this surface points outward
(`.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md`, Backbone). Where a criterion's
user is the implementer of a *sibling* story, that is not a proxy for a persona: `_storymap.md`
makes those four stories the only route by which Persona 2 and Persona 3 ever see a pointer, so
a policy they cannot act on is a persona failure one PR later.

Every test path below is `xtask/src/pointers.rs`'s own `#[cfg(test)]` module, reached by
`cargo test -p xtask --lib` and by `cargo test --workspace` inside the gate. Test names are
indicative of the obligation, not a naming mandate.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | GIVEN a later story is about to install a pointer for Persona 2, who arrived at `happenstance_core::store` from a compiler diagnostic and never saw the crate root, WHEN its implementer opens `xtask/src/pointers.rs` and reads no other artefact, THEN they learn DT-10's resolution (option (c), both with one authoritative), rule 1's single authoritative front-door text mirrored byte-identically onto two surfaces, all four gates — (i) evidenced stall, (ii) the front door provably cannot reach it, (iii) subordinate and one line, after the in-place fix and introducing no heading, (iv) a guard from the mechanism table — and the href ladder whose last rung is *do not install, escalate*; and they choose a pointer shape without re-reading `_design.md`. | `xtask/src/pointers.rs::tests::policy_states_the_dt10_resolution_its_four_gates_and_the_href_ladder` — `include_str!` over the module's own source crossed with a pinned list of short subject strings, in the shape of `check_summaries` (`xtask/src/lint_constitution.rs:463-473`) |
| **AC-002** | GIVEN that implementer has decided *where* a pointer goes and must now decide its *form*, WHEN they reach for one, THEN `PointerForm` offers exactly the four N-3 rows with each variant documenting the guard it carries; `BareUrl` is documented as forbidden to this project outright rather than merely discouraged; the policy states that the README doctest mount at `crates/happenstance/src/lib.rs:10` compiles the README's Rust fences and **not** its prose, so a prose pointer there is unguarded by it and P2's row may not claim otherwise; and it states that `#[doc(alias)]` is a search key that needs no register row because deleting the item deletes the alias — so a fifth form cannot be improvised and the over-broad guard claim cannot be inherited. | `..::tests::permitted_forms_are_closed_and_each_documents_its_guard`; `..::tests::policy_records_the_readme_prose_gap_and_the_alias_rule` |
| **AC-003** | GIVEN the maintainer performing backbone B1 — deciding once how this surface points outward — WHEN `cargo xtask ci --fast` runs on the merged result, THEN `xtask/src/lib.rs` declares `pub mod pointers;` beside its existing `mod constitution;`, its crate doc no longer opens with a sentence the module has made false, the `#![cfg_attr(doctest, doc = include_str!(...))]` README mount at `xtask/src/lib.rs:21` still stands, `POINTER_REGISTER` is present and empty and `validate(POINTER_REGISTER)` is `Ok`, no `Step` was added to `REQUIRED` in `xtask/src/main.rs`, no file outside the PR boundary changed, and the register is rendered to no reader on any surface. | `..::tests::the_register_lands_empty_and_valid`; `cargo xtask ci --fast`; `cargo xtask affected --base main`; PR-boundary check — `git diff --name-only main` is a subset of the PR boundary block |
| **AC-004** | GIVEN a sibling story that has just installed a pointer and is filing its row in the same PR, WHEN it constructs a `Pointer`, THEN the type requires it to state the row id, the surface, the destination, the form, the guard, the visible link text, whether the answer is on the destination's first screen, whether the pointer targets a fragment, and which of gates (i)–(iv) admitted it — and both `Pointer` and `PointerForm` derive `Debug`, so a rejected row prints itself rather than an index. | `..::tests::a_row_records_every_fact_a_guard_claim_needs` — constructs a complete row, reads every field, and asserts the `Debug` rendering names the row id |
| **AC-005** | GIVEN the project's promise that no pointer is installed whose only guard is memory (`project.md` AC-003), WHEN a row is filed whose `guard` is empty or whitespace, THEN `validate` rejects it and the message names the row id and states why the guard cell exists — so the promise is checkable rather than aspirational, and `_design.md` anti-pattern 18 becomes a build failure. | `..::tests::rejects_a_row_whose_guard_is_empty`; `..::tests::rejects_a_row_whose_guard_is_only_whitespace` |
| **AC-006** | GIVEN an implementer who cannot find a guarded form for a pointer Persona 2 needs and reaches for a URL, WHEN a row carries `PointerForm::BareUrl`, THEN `validate` rejects it unconditionally and the message states the escalation the href ladder prescribes — the pointer is not installed and the project escalates — rather than offering a workaround or a way to record the absence of a guard. | `..::tests::rejects_a_bare_url_outright_and_states_the_escalation` |
| **AC-007** | GIVEN Persona 3 navigating by an extracted link list or a screen reader inside the one session they will spend on this crate, WHEN a row's `link_text` is fewer than three whitespace-separated words, or is (or begins with) 'here', 'this', 'see this page', 'docs', 'read more', `http://` or `https://` — compared case-insensitively and with trailing punctuation trimmed, so 'See this page.' is the same defect wearing a full stop — THEN `validate` rejects it and names the row, and a three-word noun phrase naming the destination passes. | `..::tests::rejects_link_text_that_does_not_name_its_destination` (table-driven over each denied string with its cased and punctuated variants); `..::tests::accepts_a_three_word_noun_phrase` |
| **AC-008** | GIVEN Persona 3 mid-session, taking one hop to the passage that answers their second question, WHEN a row records `answer_on_first_screen: false` together with `targets_fragment: false`, THEN `validate` rejects it — the two booleans exist together so a row cannot claim reachability it does not have, which is exactly invariant 3's stated falsification: an inventory entry whose destination is a page rather than a passage where the answer is not on the first screen. | `..::tests::rejects_a_row_that_admits_a_deep_answer_and_targets_a_page`; `..::tests::accepts_a_deep_answer_that_targets_a_fragment` |
| **AC-009** | GIVEN four sibling stories appending rows to one register across four separate PRs, WHEN `validate` runs over the result, THEN two rows sharing an id are rejected with both surfaces named; **every** problem is reported rather than the first; each message names the offending row id and carries the reason the rule exists, in the message style of `check_summaries` (`xtask/src/lint_constitution.rs:463-473`); and a ninth row fails with the design's own sentence attached — a 21st row means the pointer policy is wrong, not that the register needs a scrollbar — with the 20-rows-ever ceiling documented on the constant. | `..::tests::rejects_duplicate_row_ids`; `..::tests::reports_every_problem_not_only_the_first`; `..::tests::every_message_names_its_row_and_states_its_reason`; `..::tests::rejects_a_ninth_row_as_the_dt10_alarm` |
| **AC-010** | GIVEN a reviewer or a later implementer asking what navigation was considered and declined (`project.md` AC-011), WHEN they read `xtask/src/pointers.rs`'s module doc, THEN they find the bespoke navigation widget — breadcrumb, master-detail rail, per-crate index page — recorded as declined with its reason, that rustdoc renders a per-item nav bar and any narrative surface renders a TOC so the evaluator's gap is evidence of a missing *link* and not a missing *widget*; and they find the `prelude` module recorded as **out of this project's boundary and owed an ADR, not rejected**, so its absence can never be read as a decision against it. | `..::tests::policy_records_the_declined_widget_and_the_out_of_boundary_prelude` |

**Traceability.** Project **AC-001** (DT-10 resolved in writing, made binding) is carried by
AC-001, AC-002 and AC-010. Project **AC-003** (every pointer resolves and something checks it)
is carried by AC-003 through AC-009 — the register plus the six rules that make a row mean
something. Project **AC-011** (no navigation widget, rejection on record) is carried by AC-010.
No project AC traced to this story is unowned, and this story claims none it was not traced.

## Interaction quality

RFC §6.7/D6. This story **renders no surface** — `_design.md`'s `## Transience policy` puts the
register at *"not rendered to a reader at all — build-time data only"*, and its anti-pattern 14
makes a rendered register a defect. So the invariants below are not waived; they are relocated
into the medium this story actually ships into, and each one is an **AC row in the table above**,
because `redkiln verify` extracts criteria from that table and a bullet here would be gated by
nothing.

### State invariants

| Invariant | Carried by | How it is verified here |
| --- | --- | --- |
| **In-place before context-jump** (UX brief invariant 1, applied to the policy medium) | **AC-001, AC-002** | An implementer must be unstuck by the module doc alone. *Falsified by*: deleting `_design.md` and finding `pointers.rs` no longer states the four gates or the ladder — which is exactly what the pinned-substring test asserts. |
| **Non-occlusion — the mount displaces nothing** (invariant 2, in this file rather than a README) | **AC-003** | `pub mod pointers;` is added beside `mod constitution;`; the `cfg(doctest)` README mount at `xtask/src/lib.rs:21` and the constitution module keep working. *Falsified by*: any line of `xtask/src/lib.rs` changing other than the added declaration and the crate-doc sentence the addition made false. |
| **Preserved identity under failure** (the medium's analogue of preserved focus and selection) | **AC-004, AC-009** | A rejected row prints itself (`Debug`) and every message names its row id, so a failing build tells the implementer *which* row and *why*, not that "a validation failed". |
| **Reversibility** | **AC-003, AC-006** | Growth is append-only and a row can be removed without the validator changing shape; and the href ladder's terminal rung is a genuine exit — *do not install, escalate* — rather than a forced choice among bad forms. |
| **Keyboard reachability** | **not applicable, and named rather than dropped** | Nothing here is operable. The accessibility floor's keyboard clause lands on `front-door-walk-record`, `second-question-walk-records` and `error-site-walk-record`, whose records must say the walk was keyboard-only (UX-AC-09). The one keyboard-adjacent obligation this story *can* carry is the screen-reader link-text floor, which is **AC-007**. |

### Composition invariants

Taken from the signed-off `_design.md` and binding. The "unstyled render" failure in this medium
is a bare boolean assertion: a `validate` that returns `false` with no message satisfies every
structural assertion perfectly and teaches nobody anything, which is why the message quality is a
criterion and not a style note.

| Invariant | Source | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — every rejection carries real composed output: the row id, the rule, and the reason it exists, in the `check_summaries` style whose failure text carries its own rationale | `xtask/src/lint_constitution.rs:463-473`; `_design.md` `## Pattern decision`, mitigation 3 | **AC-009** (message shape), **AC-004** (`Debug` on both types) |
| **Composition and placement** — the register is a `const` in the **lib** crate root's module tree, not the bin, and not a new gate `Step`; the seam left open is `use xtask::pointers::…` from a same-package bin | `_decomposition.md` N-4, CR-4 at N-2 | **AC-003** |
| **Transience** — the register is *not rendered*: not persistent chrome, not revealed, not opened on demand. A page listing every pointer is the second navigation surface DT-10 warns about | `_design.md` `## Transience policy`, register row; anti-pattern 14 | **AC-003** |
| **Density budget, with its real numbers** — 0 rows at land, 5 foreseen, **≤ 8 at this project's close, ≤ 20 ever**; exceeding it is the DT-10 alarm and not an arithmetic complaint | `_design.md` `## Density budget`, register row | **AC-003** (lands empty), **AC-009** (the cap and its message) |
| **Hierarchy** — the design's recessive-placement rule for an installed pointer (no heading, subordinate, after the in-place fix) survives into the four dependent stories only if the policy carries it | `_design.md` `## Hierarchy`, `store-module-error-site` row; rule 2(iii) | **AC-001** (gate (iii) stated in the policy) |
| **Named anti-pattern 3** — link text "here", "this", "see this page", "docs", "read more", or a raw `https://` string | `_design.md` `## Anti-patterns`, 3 | **AC-007** |
| **Named anti-pattern 14** — the register rendered as a page a reader can navigate to | `_design.md` `## Anti-patterns`, 14 | **AC-003** |
| **Named anti-pattern 18** — a pointer installed with an empty guard cell | `_design.md` `## Anti-patterns`, 18 | **AC-005** |
| **Rule 2(iv)** — a bare URL, guarded by nothing, forbidden to this project outright | `_design.md` `## Pattern decision`, rule 2(iv) | **AC-006** |
| **Invariant 3 — land on the answer, not the top of the page**, recorded per row so the two facts cannot drift apart | `_decomposition.md` UX invariant 3, UX-AC-04 | **AC-008** |

`_design.md`'s remaining anti-patterns (1, 2, 4–13, 15–17) bind the four dependent stories, not
this one — they are named here only so a later implementer does not re-decide them, and so this
story is not accused of dropping them.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | `validate` is called on a register carrying several defects at once | Returns `Err` with **one message per defect** in a stable order, never the first-and-stop. It returns a `Result`; it never panics, never `unwrap`s and never `expect`s — `unwrap_used = "deny"` (`Cargo.toml:122`) makes that a compile-time obligation, not a review note |
| **EC-002** | A row's guard names a mechanism that **does not exist yet** — P3, P4 and P5 depend on HS-P0020's registration check, which has not landed | **Permitted, and not conflated with an empty guard.** A named future guard is a claim someone can check; an empty cell is not. The policy states the wording (name the mechanism *and* that it does not exist yet); `validate` checks emptiness only, and must not pretend it can tell a real mechanism from a plausible sentence |
| **EC-003** | Someone needs a pointer form outside the four | **Unrepresentable by construction** — `PointerForm` is a closed enum in a `publish = false` crate, so a fifth form is a source edit here, not an improvisation at a call site. The module doc states that adding a variant re-opens `_design.md`'s N-3 table rather than being a local decision |
| **EC-004** | A story installs a pointer on a surface and files **no row** | **Outside this validator's sight, and named rather than faked.** The validator sees the register, not the diff. The obligation lives as policy text here and as an acceptance criterion in each installing story's own spec. Adding a check that pretends to cover it would be the decorative rule `CLAUDE.md` forbids |
| **EC-005** | The register reaches nine rows during this project | The build fails with the alarm message. The repair is the **policy**, not the number: raising the cap re-opens `_design.md`'s density budget, and doing it silently is the failure the cap exists to catch |
| **EC-006** | A legitimate row is rejected by the link-text heuristic (a false positive) | The repair is the link text, or an amendment to `_design.md`'s ≥ 3-word rule — **never** an `#[allow]`, never a special case inside `validate`. This is RS-70-3's discipline (`standards/rust/70-rustdoc-obligations.md`) applied to a lint we wrote ourselves |
| **EC-007** | `pointers.rs` fails to compile and takes `cargo test -p xtask` down with it, including the repository README's doctests | That is the mount working. `xtask/src/lib.rs` is a single crate root; a broken module there is a broken lib target. Do not route around it by moving the register to the bin — the bin cannot be consumed by HS-P0020's future step, which is the whole reason for the lib |

## Non-functional

| id | Requirement | Grounding |
| --- | --- | --- |
| **NF-001** | **No new gate step, no new process, no I/O, no network.** `validate` is a pure function over a `&'static` slice and the new tests are unit tests inside a crate the gate already compiles. The measurable claim: `cargo xtask ci --fast`'s step list is byte-identical before and after | `_decomposition.md` N-4, CR-4 at N-2; `xtask/src/main.rs` `REQUIRED` |
| **NF-002** | **Clean under the workspace lint policy as CI runs it.** Every `pub` item documented (`missing_docs = "warn"`, `Cargo.toml:103`), `Debug` on both new types, `clippy::pedantic` clean, no `unwrap`/`expect` (`Cargo.toml:122`), and CamelCase identifiers backticked in prose because `clippy::doc_markdown` fires on them and RS-70-3 forbids the `allow` repair | `Cargo.toml:100-137`; `standards/rust/70-rustdoc-obligations.md` |
| **NF-003** | **Compiles at the MSRV floor**, 1.97.1 (`Cargo.toml:8`, `rust-toolchain.toml`). Nothing in this module needs a let-chain or any feature newer than the floor, so the CI `msrv` job stays a no-op here rather than becoming this story's problem | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `CLAUDE.md`, binding constraint 5 |
| **NF-004** | **No semver surface anywhere.** `xtask` is `publish = false` (`xtask/Cargo.toml:7`), so `pub mod pointers;` adds nothing a caller can write and the gate's `cargo package --list` assertions over the three publishable crates are untouched | `xtask/Cargo.toml:7`; `_design.md` `## Items` |
| **NF-005** | **Deterministic reporting.** Problems are emitted in a stable order — register order, then rule order within a row — so a failing build produces a reviewable diff and a test can assert on the whole vector rather than on set membership | AC-009; `xtask/src/lint_constitution.rs`'s `problems` accumulation |
| **NF-006** | **Reachable as data by a sibling.** `POINTER_REGISTER`, `Pointer`, `PointerForm` and `validate` are `pub` in the **lib** target, which is both what keeps `dead_code` quiet over a register nothing reads yet and what lets HS-P0020's future step write `use xtask::pointers::…` from the bin with no change here | `_decomposition.md` N-1, N-4; `xtask/src/lib.rs` |

## Implementation notes (non-prescriptive)

What is **binding** is above: the ten criteria, the four gates copied rather than re-derived, the
forms closed at four, the cap at 8, and the register landing empty. Everything here is a
suggestion the implementer may improve on.

- **A plausible shape**, offered so nobody has to invent one: `&'static str` fields throughout,
  so `POINTER_REGISTER` is a `const` in read-only data with no allocation and no lazy
  initialisation; `Result<(), Vec<String>>` from `validate`. A typed error enum is welcome
  *if* it keeps AC-009's message quality — the messages are the deliverable, and a typed error
  whose `Display` says less than the string it replaced is a downgrade wearing a type.
- **Pin short subject strings in the doc tests, not paragraphs.** `xtask/src/spec_trace.rs`
  learned this the expensive way and `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`
  states the rule: a heuristic that cannot tell its own mistakes from the corpus's must decline
  rather than guess. Pinning whole sentences turns ordinary rewording of the policy into a build
  failure, and the first person to hit that will delete the test rather than fix the prose. Pin
  the load-bearing tokens — the gate labels, `BareUrl`, the ladder's escalation clause, the
  widget names, `prelude` — and no more.
- **The doc test reads the module's own source.** `include_str!("pointers.rs")` inside the test
  module is the shortest route; reading through `CARGO_MANIFEST_DIR` is the alternative if the
  self-include reads oddly. Either is fine — what matters is that the policy text is *asserted*
  somewhere, because a module doc nothing checks is exactly the rot this story exists to stop.
- **Write the wrong registers first.** Each rule's test constructs a register that a plausible,
  well-meaning implementer would actually file — a row whose guard reads `""` because the
  mechanism was "obvious", a `link_text` of `"here"`, a page-not-passage destination — and
  asserts the message names the offending row. `CLAUDE.md`'s rule holds in this medium: a rule
  no register can fail is decorative.
- **Amend `xtask/src/lib.rs`'s crate doc in the same change.** Its first line today is
  "Nothing but a home for the repository README's doctests"; a second sentence naming the
  register keeps the file honest at the cost of one line.
- **Do not reach for the bin.** `xtask/src/main.rs` is a separate crate root with private
  modules; a register there is unreachable to a consumer and would push this story into CR-4,
  which `_decomposition.md` N-4 forbids.

## Tests and CI (merge gate)

**This project takes no `testing` brief** — `_decomposition.md`'s warranted-brief set is
`architecture` and `ux`, because most of its criteria are dated observed walks
(`project.md`, Out of scope, final row). This story is the exception in the project: it is the
only one whose deliverable a compiler can hold an opinion about, so its bar is the repository's
ordinary one plus the unit tests it brings.

| Tier | Command / path | Proves |
| --- | --- | --- |
| unit — rules | `cargo test -p xtask --lib` → `xtask/src/pointers.rs` `#[cfg(test)]` | AC-004 through AC-009: each rejection rule rejects a named wrong register, and each message names its row |
| unit — policy text | same module, the `include_str!` substring assertions | AC-001, AC-002, AC-010: the policy actually says the things a later story will read it for, including the rejection record and the README-prose correction |
| unit — live data | `..::tests::the_register_lands_empty_and_valid` | AC-003: the validator is wired to the real `POINTER_REGISTER` and not only to synthetic ones |
| story grain | `cargo xtask affected --base main` | the gate `.redkiln/config.yaml` wires to this grain: fmt, clippy `-D warnings` and tests for `xtask` and its dependents, plus the five file-reading lints and `spec-trace` unconditionally |
| reachability, static | `cargo xtask lints && cargo xtask spec-trace` | the mount is not decoration and no clause citation rotted — `spec-trace` is a regression check here, since this story amends no clause |
| project integration | `cargo xtask ci --fast` | project DoD item 1: the `REQUIRED` set green with `pub mod pointers;` declared — fmt, clippy, tests, four wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build and the packaging assertions |
| docs | the `documentation` step inside `cargo xtask ci --fast` | NF-002: `missing_docs` satisfied on every new `pub` item and rustdoc clean over the new module |
| boundary | `git diff --name-only main` against this spec's PR-boundary block | AC-003's negative half: no new `Step`, no sibling-owned file touched, no `.kb/` atom, no `spec/SPECIFICATION.md` clause |
| ledger | `redkiln verify --grain story` | `.redkiln/config.yaml` `require_ledger: true` — every AC in `_ledger.md` satisfied with cited, non-placeholder evidence |
| provenance | `redkiln record-links --sha <sha>` | `require_commit_provenance: true` — a story that changed files inside its declared boundary records the work commit |

**What no tier here proves, restated so it is not mistaken for coverage.** That a story installed
a pointer and filed no row (EC-004); that a guard someone *named* actually exists (EC-002); and
that the four gates were applied honestly rather than recited. Those are review obligations
carried in each installing story's own spec, and the register is the artefact that makes them
reviewable, not the check that makes them automatic.

## Risks and coupling (PR-scoped)

| Risk / coupling | Note |
| --- | --- |
| **The register lands empty, so every rule is exercised only against synthetic registers** | This is the design, not a gap: the wrong registers *are* the coverage, and `CLAUDE.md`'s discipline applies unchanged — if a rule cannot be given a plausible wrong register, it is decorative and should not be written. The one live assertion (`validate(POINTER_REGISTER)` is `Ok`) is what stops the validator being unwired from its own data |
| **HS-P0020 may never build the checker that consumes the `const`** | Named by `_decomposition.md` N-4 and not assumed away. If the hook never lands, the register is still AC-003's inventory of record and the rows fall back to N-3 forms 1–2, whose guards already exist. `pub` visibility — not a consumer — is what keeps `dead_code` quiet in the meantime |
| **The temptation to fork a second checker into `xtask/src/`** | Forbidden by N-4 in the sharpest words the brief uses. The boundary check in AC-003 is the guard; the seam that makes it unnecessary is `use xtask::pointers::…` from the bin |
| **The link-text rule is a lint we wrote ourselves and can be wrong** | EC-006 fixes the repair path in advance, because the cheap repair (`#[allow]`, or a special case inside `validate`) is the one that quietly disables the rule for everyone |
| **Four stories append rows to one file** | Serialised by `_storymap.md`'s merge order and one row per PR; duplicate-id rejection (AC-009) turns a bad merge into a build failure rather than a silent overwrite of a sibling's row |
| **Merge-forward hazard** | This is the one story in the project **not** exposed to it: `crates/happenstance/src/lib.rs` (75 lines here, 237 on `initiative/from-contract-to-published-library`) and `crates/happenstance/README.md` are untouched here. That is part of why the foundation goes first — the policy can be settled while the sibling branch is still unmerged |
| **Over-pinning the policy text** | A doc test that pins paragraphs makes ordinary rewording a build failure and will be deleted rather than fixed. Pin short subject strings; the `spec_trace` windowed-search lesson is in the tree already |
| **Re-litigating a gate** | `_design.md`'s sign-off (Ryan Britton, 2026-08-17) was the last cheap opportunity to disagree and its condition 2 makes the four gates authoritative for siblings. Re-deciding one inside this story is scope drift, and the wording is copied rather than re-derived for exactly that reason |
| **Nothing external blocks this story** | It does not wait on HS-P0020's clause-id pin, HS-P0020's pinned tree, or HS-P0022's pages. Its only precondition — this project's own design gate — is already signed off, which is what `_decomposition.md` means by "available parallelism" |

## Dependencies

**Blocks on**: nothing. `depends_on: []`, matching `_storymap.md`'s Slices table and its Merge
order step 1 ("Blocks everything; nothing blocks it inside this project"). The story's only
precondition is this project's design gate, recorded as approved in `_design.md`'s `## Sign-off`.

**Unlocks** (by story slug, all four in this project):

| Story slug | Item | What it takes from here |
| --- | --- | --- |
| `adapter-reasoning-account` | HS-S0155 | the policy's citation and no-widget constraints; it files no row itself, being the destination rather than a pointer |
| `store-error-site-rewrite` | HS-S0156 | the four gates for the P3 pointer at the evidenced stall (BR-15), the href ladder's third rung, and the P3 register row |
| `front-door-pointer` | HS-S0158 | rule 1's single authoritative mirrored sentence, and the P1 and P2 rows — including the honest P2 guard the README-prose correction requires |
| `evaluator-onward-links` | HS-S0159 | the fragment rule, the self-describing link-text rule, and the P4 and P5 rows |

Downstream of those, the two walk slices (`error-site-walk-record`, `front-door-walk-record`,
`second-question-walk-records`) inherit the policy transitively and add no dependency on this
story directly.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Every path was confirmed to exist in this
worktree before it was cited.

| Anchor | Why it is load-bearing | When to open it | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` | The binding, signed-off resolution: `## Pattern decision` carries DT-10 option (c) with rule 1, the four gates and the href ladder in their authoritative wording; `## Transience policy` and `## Density budget` carry the not-rendered call and the 8/20 cap with the sentence the alarm message must attach; `## Anti-patterns` 3, 14 and 18 are three of this story's rules | Before writing the module doc's policy section, and again before writing the cap's message — copy the gate wording, do not re-derive it | AC-001, AC-003, AC-005, AC-007, AC-009 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` | N-3 is the four-form mechanism table `PointerForm` encodes, with the per-form caveats (including the README fence-not-prose one); N-4 draws the ownership line — author the list here, ask HS-P0020 to consume it, do not fork a checker; N-9 is the widget rejection and the `prelude` proposal in the words AC-010 must preserve | Before defining `PointerForm` (N-3), before deciding where anything mounts (N-4), and before writing the rejection record (N-9) | AC-002, AC-003, AC-010 |
| `xtask/src/lint_constitution.rs` | The working precedent this story copies rather than invents: `SUMMARIES` at `:64` is the register's shape and `check_summaries` at `:463-473` is the message style whose failure text carries its own rationale — the difference between composed output and a bare assertion | Before writing `POINTER_REGISTER`'s declaration, and again before writing the first rejection message | AC-004, AC-009 |
| `xtask/src/lib.rs` | The mount point itself — 28 lines, `mod constitution;` and a `cfg(doctest)` README include, with a crate doc whose first sentence the new module makes false | At the moment of mounting; read the whole file first, it is short and every line is load-bearing | AC-003 |
| `xtask/src/main.rs` | The bin crate root and the `REQUIRED` step list, with the `probe` contract documented at `:85-102`. It is here to be **read and left alone**: the story's negative half is that no `Step` is added, and knowing what a `Step` looks like is how you recognise that you are about to add one | Only if you find yourself wanting a gate step — then read CR-4 in `_decomposition.md` N-2 and stop | AC-003 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` | The Coverage table's AC-003 row is the split-ownership rule that makes the register land empty ("each installing story owns its own rows; a story that installs a pointer and files no row has not finished"), and the AC-011 row assigns the rejection record to this story with its reason | Before deciding what goes into the register at land, and before deciding where the rejection record lives | AC-003, AC-010 |
| `.bklg/docs-that-teach/reach-and-adapter-path/project.md` | AC-003's exact promise — "No pointer is installed whose only guard is memory" — and AC-011's wording. The rejection messages should read as enforcing these sentences, not as generic validation failures | While writing the empty-guard and bare-URL messages | AC-005, AC-006, AC-010 |
| `Cargo.toml` | The workspace lint policy the module is written against — `missing_docs` and `missing_debug_implementations` at `:101-107`, `clippy::pedantic` and `unwrap_used = "deny"` at `:108-122`, `broken_intra_doc_links = "deny"` at `:134` — all under CI's `-D warnings` | Before writing the first `pub` item, so the doc comments are written once rather than added after the gate complains | AC-003, AC-004 |
| `crates/happenstance-core/src/store.rs` | Line 77 is the live instance of the href ladder's third rung — a named-but-unlinked cross-reference with its one-sentence reason — and `:31-45` is the section a dependent story rewrites. Read for the *form*; this story edits nothing here | When documenting the ladder's third rung, so the policy describes a shape that exists rather than one it imagines | AC-001, AC-002 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-2 is why form 1's guard is worded "must resolve in every feature configuration" rather than just "rustdoc denies broken links"; RS-70-3 is why EC-006 forbids the `allow` repair | While documenting `PointerForm::IntraDoc`, and again if the link-text rule starts producing false positives | AC-002 |
| `references/evaluation/review-dx-ergonomics.md` | Lines 421-422 carry the evidence that makes the `prelude` proposal a live one — "the single highest-leverage doc fix in the crate" — which is why AC-010 records it as out of boundary rather than declined | While writing the rejection record's second half, so the `prelude` entry reads as deferred rather than dismissed | AC-010 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The dossier passage the widget rejection cites: the breadcrumb / master-detail fit conditions, where the gap is "evidence of a missing *link*, not a missing *widget*" | While writing the widget rejection, so its reason is the evidence's and not a paraphrase | AC-010 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 2's arrival path (a diagnostic, never the crate root) and Persona 3's single reading session are what gates (i) and (ii) and the fragment rule exist for. The criteria above are framed from these; open it if a rule starts to look arbitrary | If you are tempted to soften a gate — the persona is the reason it is shaped that way | AC-001, AC-007, AC-008 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` | The verified sibling-project dependency state, and the audit finding that **no Accepted decision atom governs pointer policy** — which is why this story writes substrate rather than citing an ADR, and why it must not author one | Before wondering which ADR to cite for the policy: there is none, deliberately | AC-001 |
| `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` | The sign-off mock, context only. It renders the five surfaces and its closing section carries findings F1–F6 and their dispositions. Nothing in it binds this story — the register is invisible in a mock by construction (F6) | Only if you need to see what the pointers this policy governs will actually look like | AC-001 |

## Clarifications resolved during spec

1. **Exactly the ten AC ids the front half declared** — AC-001 through AC-010. None added, none
   dropped. The ledger enumerates the same ten.
2. **The register lands empty, with zero rows**, rather than pre-filled with the five foreseen
   ones. `_storymap.md`'s AC-003 coverage row gives each installing story ownership of its own
   rows, so a pre-filled register would be five claims this story has not earned — and would make
   every rule vacuously satisfied by data nobody installed.
3. **The cap is enforced at 8 and documented at 20.** Enforcing the 20 as well would be
   unreachable code behind the 8; the ceiling belongs on the constant, with the design's alarm
   sentence, where the person raising the cap will read it.
4. **`validate` reports every problem, not the first.** A story filing a bad row should learn
   everything wrong with it in one run; first-and-stop turns one bad row into three build cycles.
5. **This story renders no surface, and the composition invariants were relocated rather than
   waived.** They land on the validator's *messages*, on the mount's non-displacement, and on the
   register's non-rendering — see `## Interaction quality`. Keyboard reachability is genuinely
   not applicable here and is named as such rather than silently dropped; it belongs to the three
   walk stories.
6. **AC-011's rejection record lives in the module doc**, not in a planning file. `_storymap.md`
   assigns the record to this story, and the reader who needs it is the one already reading the
   register.
7. **The `prelude` proposal is recorded as out of boundary and owed an ADR — not as a
   rejection.** `_decomposition.md` N-9 is explicit that its absence must not be read as a
   decision against it, and `.kb`/ADR authorship is not a side effect of a documentation story.
8. **No `.kb/` atom is authored here.** The policy is project substrate; promoting it to settled
   knowledge is a closeout concern, and atoms are authored by `/redkiln:kb-ingest` from
   `.kb/_intake/` rather than by hand (`CLAUDE.md`).
9. **The "installed a pointer, filed no row" gap is unenforceable here and is named rather than
   faked** — EC-004. It was tempting to add a check that greps the diff; a check that cannot see
   what it claims to check is worse than the honest absence.
10. **No conformance rule is added.** `happenstance-testkit` cannot observe a doc-pointer policy,
    and inventing a rule for it would be precisely the decorative rule `CLAUDE.md` forbids. The
    equivalent discipline — a named wrong implementation the check rejects — is discharged by the
    wrong registers in this module's own tests.
