---
item: HS-S0158
stage: spec
created: 2026-08-17T13:16:14.916Z
updated: 2026-08-17T13:16:14.916Z
template_sig: 87bbf1d0
rendered_sig: 16c52342
---

# Spec — The front door points outward, on both surfaces

## Scope lock

| Slot | Path |
| --- | --- |
| Initiative | `.bklg/docs-that-teach/initiative.md` — BR-08, AC-11, DoD scenario 7 |
| Project | `.bklg/docs-that-teach/reach-and-adapter-path/project.md` — AC-002, AC-003, DoD items 1, 6, 7 |
| This spec | `.bklg/docs-that-teach/reach-and-adapter-path/front-door-pointer/spec.md` |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` — approved 2026-08-17; surfaces `crate-root-front-door`, `readme-front-door` |
| Briefs | `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` — UX brief (invariants 2, 9; UX-AC-01, UX-AC-02, UX-AC-03) and architecture brief (N-1, N-2 CR-1, N-3, N-4, N-10; AC-A01) |
| Grounding | `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` — line-level verification of both target files and the sibling-project dependency state |
| Roadmap / merge order | `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md`, "Merge order" — slice 3 `front-door-reach`, and its two hard external gates |
| Story discover | `.bklg/docs-that-teach/reach-and-adapter-path/front-door-pointer/discover.md` |

## One-line PR slice

Install the DT-10 pointer on both front-door surfaces — `crates/happenstance/src/lib.rs`'s crate
root and `crates/happenstance/README.md` — above the fold, displacing nothing, with copy that is
true whichever hosting shape HS-P0020 resolves to, and register both rows in the inventory.

## Executive summary

This PR lands **one sentence, twice, and two register rows.** That is the whole diff, and its
smallness is the point: `crates/happenstance/README.md` and `crates/happenstance/src/lib.rs`'s
crate root are the two surfaces a developer meets after `cargo add happenstance`, and today
neither of them says that guide-level material exists at all — both point outward only to
crates.io and to `spec/SPECIFICATION.md` (verified in `_grounding.md`; `crates/happenstance/src/lib.rs:11-71`,
`crates/happenstance/README.md:1-60`).

The delta over the project charter is that the *policy* is already decided and the *text* is
already written. `_design.md` resolved DT-10 to option (c) — both surfaces, one authoritative
text — and pinned the sentence verbatim in its `## Signatures` block. This story does not choose
a shape, a wording, or a placement. It executes three decisions that are already made (what the
sentence says, where on each surface it sits, what may not move to make room), binds the one
thing deliberately left late (the destination href, chosen off the ladder at implementation),
and files the two inventory rows that make the pointers checkable rather than remembered.

What it deliberately does **not** land: any claim, on either surface. `crates/happenstance/README.md`
is shared with HS-P0016 `publication-and-positioning` on an unmerged branch, and the seam between
the two is *purpose, not paragraph* — they own every claim on that file, this story owns two added
lines that assert nothing. Project DoD item 7 makes that seam checkable by diff rather than by
assertion, which is why "changes nothing else" is an acceptance criterion here and not a courtesy.

## Context pack

The load-bearing decisions this story must honor. Everything deeper is a signposted anchor.

**The policy is already resolved; this story installs the authoritative half of it.** DT-10 is
settled as option (c) — *both* surfaces point outward, with *one* authoritative text — bound by
the rule "one front door, and a pointer only at a stall" (`_design.md`, `## Pattern decision`).
Read the rule carefully, because it is easy to implement as its opposite: **one sentence, authored
once, mirrored byte-identically onto both front-door surfaces. Two renderers, two readers, one
authority — not two pointers.** A second, differently-worded sentence on the README is the
anti-pattern the resolution exists to forbid (`_design.md`, `## Anti-patterns`, item 2), because
two texts are two things to keep true and one of them will go stale.

**The sentence is pinned, not paraphrasable.** `_design.md`'s `## Signatures` block carries it
verbatim: *"Guide-level documentation — what a dynamic consistency boundary is, and how to model
an application with one — is at [the happenstance guide](DESTINATION)."* 22 words; link text
"the happenstance guide"; pinned substring for the mirror assertion, "Guide-level documentation".
Only `DESTINATION` is negotiable, and only by the ladder below. The design states that nothing
else in that block moves "without re-opening this file".

**The pointer belongs on `happenstance`, and the adapter pointer is forbidden here.** ADR-0006
(`.kb/decisions/0006-bare-name-to-the-typed-layer.md`) put the two audiences on two crates:
applications reach for the bare name, adapter authors pin the contract. `crates/happenstance/src/lib.rs:20-25`
already states that conclusion in prose and routes adapter authors to `happenstance_core` at
`:69-71`. **The two pointers are on different crates because the ADR put the audiences on different
crates** — installing an adapter-facing pointer on this crate root would contradict the page it
sits on (`_design.md`, `## Placement and re-export`).

**Placement is above the first `#` heading, and that is a reading decision rather than a
typographic one.** rustdoc renders pre-heading prose as the item's top blurb — what docs.rs shows
in a search result and what a reader sees without scrolling. The section that follows it,
`# Status: a facade over [happenstance_core]` (`crates/happenstance/src/lib.rs:13`), is a
*disclaimer*: a reader who bounces off "early, and a facade" must already have been told the guide
exists. On the README the same ordering holds for the same reason, and the pointer sits **outside**
the status blockquote (`crates/happenstance/README.md:6-11`) because that block is HS-P0016's
claim and this story changes no claim.

**Non-occlusion is the invariant that fails first.** A pointer must not displace what it points
at. Nothing pre-existing may be reworded, folded, or moved down by more than **three** source
lines — three, not two, because the design's own mock measured the pointer at three source lines
and amended anti-pattern 11 rather than shipping a rule the design itself breaks (`_design.md`,
finding F1 and its disposition). The README's compiled fence therefore opens at source line **33**,
against a budget of 35. Nothing may be folded or collapsed to make room; the density budget yields
by *shortening the pointer's own sentence*, never by moving anything else.

**The copy is true before the destination is known; only the href is late-bound.** UX-AC-02 is
what lets this story exist while HS-P0020's hosting shape is still open: the sentence names the
*need* it answers, and the destination is bound at implementation off the **href ladder** —
intra-doc link (guarded by `broken_intra_doc_links = "deny"`, `Cargo.toml:134`, across three
rustdoc builds) **>** an in-tree markdown link inside HS-P0020's pinned tree (guarded by its
registration check) **>** a named-but-unlinked cross-reference in the form live at
`crates/happenstance-core/src/store.rs:77` **>** *nothing*. **A bare URL is forbidden to this
project outright** (`_design.md`, rule 2(iv)), because AC-003 says no pointer whose only guard is
memory.

**If only the fourth rung is available, the pointer is not installed and this story blocks.** The
`Empty` state in `_design.md`'s `## States` is explicit and is the state this branch is in right
now: no "coming soon", no placeholder href. A pointer into nothing is a dead end, which is worse
than today's silence. Escalate exactly as the project escalates for AC-010 if HS-P0020 lands
without a clause-id set — do not ship a pointer to a page that does not exist.

**One guard claim in the project charter is over-broad, and the design says so — do not inherit
it.** Project AC-002 reasons that the README is compiled as a doctest under
`#![cfg_attr(doctest, doc = include_str!("../README.md"))]` (`crates/happenstance/src/lib.rs:10`),
so a malformed pointer is a build failure. That mount compiles the README's ```` ```rust ````
**fences, not its prose** — and this design deliberately places the pointer as prose. P2's real
guard is therefore the **mirror assertion** (a pinned substring present exactly once in each of
the two files), which **does not exist yet**: it is a `check_summaries`-shaped step offered to
HS-P0020, and `xtask/src/lint_constitution.rs:64` (`SUMMARIES`) and `:463-473` are the working
precedent it would copy. This story does not own `xtask/src/` and must not fork a second checker
into it (architecture brief N-4). The inventory row states the guard honestly, including "not yet
landed" if that is the truth at merge.

**The register is data, not a page.** The inventory this story files into is build-time data in
the shape of `SUMMARIES`, never a rendered index — a page listing every pointer *is* the second
navigation surface DT-10 warns about (`_design.md`, `## Transience policy`, final row). Its shape
and its home are `pointer-policy-and-inventory`'s (HS-S0154) to land; this story appends rows **P1**
and **P2** to it and invents nothing about it.

**The persona slice: A0, inside one reading session.** The evaluator is deciding within a bounded
reading budget and has no second attempt if the first fails silently (`_decomposition.md`, UX brief
Journey A; `_discovery/distillation/personas-and-journeys.md`, Persona 3). The slice-mate
`evaluator-onward-links` completes the same session at A1–A3, which is why the two are one slice:
a front door that points at material with no onward link reproduces the recorded axum ordering
failure ("I was not sure in which order to try to read them").

**No widget, ever, and no new navigation form.** The evaluator's gap is a missing *link*, not a
missing *widget* (`_discovery/distillation/interaction-patterns.md`, Anti-patterns). No routing
table (`docs/README.md:12-23` is the ten-destination case a table is for; a one-row table is a
widget), no second blockquote, no new `# Guide` heading, no "See also" block, no raw HTML, no
inline styles, no folded or tabbed content.

**Merge forward before implementing, and re-decide placement rather than rebasing it.**
`crates/happenstance/src/lib.rs` is 75 lines on this branch and 237 on
`initiative/from-contract-to-published-library`, where HS-P0016 also edits the README. Placement
is a non-occlusion judgement against the merged tree, **not a mechanical rebase**
(`_decomposition.md`, N-10; `_design.md`, `## States`, `Loading` row). **That branch is not present
in this worktree and its state is NOT VERIFIABLE from here** — re-verify before binding anything
to it.

## Integration contract

This story is delivered **mounted**, on a render path the gate already builds.

- **Archetype**: `capability` — a user-observable slice through every layer of this medium: source
  doc comment → rustdoc render → the surface a developer actually meets.
- **Slice / milestone**: `front-door-reach`. **Slice-mate**: `evaluator-onward-links` (HS-S0159),
  implemented in the same context and mounted as one integrated surface. Either order within the
  slice (`_storymap.md`, Merge order, step 3).
- **Mount point**: **`crates/happenstance/src/lib.rs`** — composition root **CR-1** in the
  architecture brief (N-2). Two render paths hang off this one file: the crate-root `//!` doc
  (lines 11-71) renders on docs.rs, and `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`
  at `:10` mounts `crates/happenstance/README.md` into this crate's own doctest run. The README's
  own reader is crates.io, which never sees the rustdoc — which is why DR-2 requires the pointer on
  both and why one surface is not a substitute for the other.
- **Wires into**:
  - `crates/happenstance/README.md` — the second front-door surface, mounted through `:10` above.
  - The pointer register landed by `pointer-policy-and-inventory` (HS-S0154) — this story appends
    rows P1 and P2, with the guard and the ladder rung per row.
  - The gate's rustdoc builds: `cargo xtask ci`'s docs step, the `--no-default-features` doc build
    of `happenstance-core`, and the nightly `--cfg docsrs` build, all under
    `rustdoc::broken_intra_doc_links = "deny"` (`Cargo.toml:134`).
  - Design-system primitives consumed, all pre-existing (`_decomposition.md`, UX brief,
    "Design-system primitives"): plain rustdoc pre-heading prose, the markdown paragraph, and — if
    the ladder resolves to rung 1 — the intra-doc link. **Nothing is hand-rolled.**
- **Renders surfaces**: `crate-root-front-door` and `readme-front-door` from `_design.md`'s
  `## Surfaces` block, in the states declared there — `first-screen-1440x900`,
  `first-screen-1024x768`, `pointer-absent-baseline`, `long-destination-name`;
  `first-screen-crates-io`, `first-screen-github`, `diff-against-main`, `narrow-viewport`. It
  renders neither of the two `route: TBD` surfaces (`adapter-reasoning-account`,
  `evaluator-onward-links`) and does not close design finding F5.
- **Public items**: **none.** `_design.md`'s `## Items` block declares `path: ""` — this project
  changes no public item, signature, feature or manifest entry. The two `#[doc(alias)]` attributes
  the design records land on `EventStore` / `SendEventStore` in `crates/happenstance-core/src/store.rs`
  and belong to `store-error-site-rewrite`; **this story adds no attribute and no alias.**
- **Conformance rule(s)**: **none, and this is not adapter-observable.** Nothing here touches a
  port, a bound, a future or a value type; doc attributes and prose are erased before type-checking
  and are identical on both port flavours and on `wasm32` (`_design.md`, "What it costs a caller").
  A conformance rule that could observe this change would be a rule about prose, which the suite is
  not for.
- **Clause(s)**: **none discharged or amended.** `spec/SPECIFICATION.md` is not edited and no
  `[FROZEN]` clause is touched; the clause-pin obligation (project AC-010, DoD item 2) belongs to
  `store-error-site-rewrite`, which is the only story editing a `happenstance-core` doc comment.
  `cargo xtask spec-trace` must stay green as a gate step, not as a discharge.
- **Advances DoD scenario**: initiative **DoD scenario 7** — *"@smoke — the reader reaches the
  teaching from the front door. Starting only from what a developer sees after installing the
  crate, a person with no prior knowledge of this repository's layout reaches the narrative
  material, observed rather than asserted."* This story lands the half that makes that walk
  possible; the observation itself is `front-door-walk-record`'s (HS-S0160), which is blocked on
  this story by design so the author is never their own witness.

## PR boundary

**In this PR**

- The pointer sentence added to `crates/happenstance/src/lib.rs`'s crate-root `//!` doc, in region
  2 of `_design.md`'s composition table — after `//! DCB-compliant event sourcing, with batteries.`
  (`:11`), before `# Status` (`:13`).
- The byte-identical rendered sentence added to `crates/happenstance/README.md` as its own
  paragraph between the description (`:1-4`) and the status blockquote (`:6-11`).
- The destination href bound off the ladder, and the chosen rung recorded.
- Register rows **P1** and **P2** filed into the inventory landed by HS-S0154, each with a named,
  non-empty guard.
- This story's own backlog folder: the ledger and, later, the implementation report.
- The merge-forward placement re-check against the merged tree, recorded as a finding if placement
  moves.

**Explicitly not in this PR**

- Any change to any claim on either surface — no rewording of the status callout, the
  "Which crate do I want?" routing, the guarantees, or the ADR-0006 paragraph. DoD item 7 is a
  diff, not an assertion.
- Any edit to `crates/happenstance-core/**` — the `store.rs` rewrite, its restored `error[E0034]`
  transcript and the two `#[doc(alias)]` attributes are `store-error-site-rewrite`'s.
- Authoring the destination page, the pinned narrative tree, or its gate step (HS-P0020), and the
  narrative material itself (HS-P0022).
- Authoring the register's shape, the permitted-forms record, or the widget-rejection record
  (HS-S0154, `pointer-policy-and-inventory`).
- Any `xtask/src/**` change, including the mirror-assertion step. It is *asked for* here and
  *owned* by HS-P0020 (architecture brief N-4).
- The onward links from the answering page (`evaluator-onward-links`, same slice, separate story)
  and the observed walk (`front-door-walk-record`).
- A `prelude` module, any public item, feature or manifest change — out of boundary and owed an
  ADR (`_decomposition.md`, N-9).

```
crates/happenstance/src/lib.rs
crates/happenstance/README.md
.bklg/docs-that-teach/reach-and-adapter-path/front-door-pointer/**
```

The fence above is what `redkiln verify --grain story` computes against. **The register file is
deliberately absent from it**, because HS-S0154 binds that path and this spec will not guess it.
When the row is filed, widen this fence in *this spec*, deliberately, naming the real path — never
widen it to turn a red gate green, and never file the rows by writing outside it.

**Merge DoD**: `cargo xtask ci --fast` green on the merged result, both surfaces carrying the same
sentence within the first screen, the diff showing nothing but the two additions, and both register
rows present with a non-empty guard.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **One authoritative sentence, mirrored** | The exact text in `_design.md`'s `## Signatures` P1/P2 block, 22 words, rendered identically on both surfaces. Link text "the happenstance guide" (≥ 3 words, noun phrase); pinned mirror substring "Guide-level documentation"; the named need ≥ 4 words. No parenthetical, no semicolon, 8–30 words. | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md`, `## Signatures`; `## Density budget`, "Minimum legible size" |
| **Crate-root placement** | Region 2 of the composition table: pre-heading blurb, after `:11`, before `# Status` at `:13`. Renders as the docs.rs top blurb. Pointer lands within the **first 5 rendered lines**; first screen holds ≈ 16 of ≈ 27 lines at 1024x768. | `_design.md`, `## Composition` → `crate-root-front-door`; `crates/happenstance/src/lib.rs:11-13` |
| **README placement** | Own paragraph between description and the status blockquote; **outside** it, and not a second blockquote. Compiled fence moves from source line 30 to **33**, budget 35. | `_design.md`, `## Composition` → `readme-front-door`, and finding F1's disposition; `crates/happenstance/README.md:6-11,30` |
| **Non-occlusion** | No pre-existing line reworded, folded or moved down by more than **3** source lines; no line other than the additions changes. If the sentence will not fit the budget, **the sentence shortens** (30-word cap → 20); nothing pre-existing moves. | `_decomposition.md`, UX brief invariant 2 and UX-AC-01; `_design.md`, `## Anti-patterns` item 11 |
| **Late-bound destination, ladder-chosen** | Rung 1 intra-doc link → rung 2 in-tree markdown link inside HS-P0020's pinned tree → rung 3 named-but-unlinked cross-reference in the form live at `store.rs:77` → rung 4 *nothing*. Bare URLs forbidden outright. The chosen rung is recorded in the register row. | `_design.md`, `## Pattern decision`, rule 3 and rule 2(iv); `_decomposition.md`, N-3 (four forms and what catches each rotting) |
| **Destination absent ⇒ pointer absent** | If no rung above "nothing" is available, the pointer is **not installed**: no placeholder, no "coming soon", no href into a page that does not exist. The story blocks and escalates. | `_design.md`, `## States`, `Empty` row; `_storymap.md`, Merge order step 3 (hard external gates) |
| **Register rows P1 and P2** | Two rows appended to HS-S0154's inventory: surface, destination, N-3 form, ladder rung, guard. **No empty guard cell.** P1's guard is `broken_intra_doc_links = "deny"` if hosting resolves docs.rs-only, otherwise HS-P0020's registration check. P2's guard is the mirror assertion, recorded honestly as not-yet-landed if that is true at merge. | `_design.md`, `## Density budget`, "The five register rows"; `_decomposition.md`, N-4; `xtask/src/lint_constitution.rs:64,463-473` (the shape to copy, not to fork) |
| **The README-doctest mount guards code, not prose** | `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` recompiles the README's ```` ```rust ```` fences as this crate's doctest. The prose pointer is outside that guard. The spec states this rather than inheriting AC-002's over-broad claim. | `crates/happenstance/src/lib.rs:10`; `_design.md`, `## Density budget`, "Two named gaps"; `_decomposition.md`, N-2 CR-1 and N-3 row 2 |
| **Gate behaviour** | `cargo xtask ci` stays green with the pointer in place: the docs step, the `--no-default-features` doc build, the nightly `--cfg docsrs` build, and `cargo test` (which parses the README). A rung-1 pointer that does not resolve in **every** feature configuration is a hard failure, not a review note (RS-70-2, AC-A02). | `CLAUDE.md`, Commands; `standards/rust/70-rustdoc-obligations.md`; `Cargo.toml:134` |
| **Accessibility floor** | Self-describing link text — never "here", "this", "see this page", "docs", or a bare URL. No colour or position carries meaning; nothing animates; the heading ladder is unskipped (`lib.rs:13, 27, 53` unchanged); everything reflows at 1024x768 with no fixed width, table or column. | `_decomposition.md`, UX brief "Accessibility floor" and UX-AC-03; `_design.md`, `## States`, narrow-viewport and screen-reader rows |
| **Anti-patterns this story can trip** | 1 (first screen with no guide sentence), 2 (two different sentences), 3 ("here" / bare URL), 4 (pointer inside or beside the status blockquote), 5 ("See also" block), 6 (a table used as navigation), 10 (raw HTML / inline style), 11 (any line moved more than three, or any other line changed), 14 (register rendered as a page), 18 (empty guard cell). | `_design.md`, `## Anti-patterns` |
| **No public surface change** | No `pub` item, signature, feature or manifest entry. No `#[doc(alias)]` — the two evidenced aliases belong to `store.rs` and to `store-error-site-rewrite`. Identical cost on both port flavours and on `wasm32`. | `_design.md`, `## Items`, `## Visibility and stability`; `_decomposition.md`, architecture brief Intent (non-goal) |
| **Merge-forward before implementation** | Re-verify both target files against the merged tree (`initiative/from-contract-to-published-library`: `lib.rs` 237 lines there vs 75 here; README edited by HS-P0016). Placement is re-decided as a non-occlusion judgement. **NOT VERIFIABLE from this worktree.** | `_decomposition.md`, N-10; `_grounding.md`, "Tensions and risks"; `_design.md`, `## States`, `Loading` row |

## Data and migrations

**No persistent data, no schema, no store, no migration.** This story adds prose to two files that
are compiled and rendered, not stored; nothing here touches an event store, a projection store, a
serialised envelope or a manifest.

Two data-shaped obligations exist and are named so they are not mistaken for prose:

1. **The register rows are data, not documentation.** P1 and P2 are appended to the inventory
   HS-S0154 lands, in the shape of a `const` list a checker reads —
   `xtask/src/lint_constitution.rs:64` and `:463-473` are the working precedent. Their **format is
   HS-S0154's to define**; this story supplies two rows in it and neither invents nor forks the
   structure. The cap is a real number: 8 rows at this project's close, 20 ever, and a 21st row
   means the pointer policy is wrong rather than that the register needs a scrollbar
   (`_design.md`, `## Density budget`).
2. **The destination href is late-bound, and rebinding it is the only "migration" here.** When
   HS-P0020 resolves the hosting shape, the href may move up the ladder (typically rung 2 → rung 1)
   without the sentence changing a word — that separation is exactly what UX-AC-02 buys. A
   rebinding updates the href on **both** surfaces and the ladder rung in **both** register rows,
   in one change, or the mirror is broken and anti-pattern 2 fires.

## Acceptance criteria

Seven criteria, each framed from the intent of a real reader crossing the whole stack — source doc
comment → rustdoc or markdown render → the screen the person actually meets. The personas are
`_discovery/distillation/personas-and-journeys.md`'s: **Persona 3, the evaluator**, deciding inside
a twenty-minute budget with *"no second attempt if the first one fails silently"* (Cross-persona
tensions, third bullet), and the maintainer who has to keep the pointer true afterwards.

Project **AC-002** is carried by AC-001, AC-002, AC-003, AC-006 and AC-007; project **AC-003** is
carried by AC-004 and AC-005. Nothing here restates a project AC.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN an evaluator who has just run `cargo add happenstance` and opens the crate's rustdoc root inside a bounded reading budget, WHEN they read the first screen at 1024x768 without scrolling, THEN one sentence of plain pre-heading prose — composed from rustdoc's own top-blurb primitive, with no bold, no callout, no emoji and no new heading — names guide-level material and where it is, rendered within the first **5** rendered lines and above `# Status: a facade over [happenstance_core]`, as persistent chrome no reader has to expand. | `cargo doc -p happenstance --no-deps`, then read `target/doc/happenstance/index.html` at `#main-content > .docblock` (the selector `_design.md`'s `## Surfaces` pins) against the approved `crate-root-front-door` / `first-screen-1024x768` frame in `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html`. Static half: `rg -n "Guide-level documentation" crates/happenstance/src/lib.rs` returns exactly one hit, between `:11` and `:13`, and `rg -n "^//! #" crates/happenstance/src/lib.rs` shows the first `#` heading still after it. |
| AC-002 | GIVEN a crates.io reader who never sees the rustdoc, WHEN they open `crates/happenstance/README.md` as crates.io or GitHub renders it, THEN the **same authoritative sentence** — byte-identical rendered text, carrying the pinned substring `Guide-level documentation` exactly once in each of the two files — appears as its own paragraph between the description and the status blockquote, **outside** that blockquote and not as a second one. | `rg -c "Guide-level documentation" crates/happenstance/README.md crates/happenstance/src/lib.rs` returns `1` for each. Mirror equality: extract the sentence from each file and diff the two — they must be identical after stripping the `//! ` prefix. `rg -n "^> " crates/happenstance/README.md` shows the blockquote still one contiguous block and the pointer outside it. Reviewed against the `readme-front-door` / `first-screen-crates-io` frame in `design/mock.html`. |
| AC-003 | GIVEN a reader who came to the README for the "Which crate do I want?" answer or the compiled example, WHEN the pointer is installed on both surfaces, THEN nothing pre-existing is reworded, folded, collapsed or reflowed; no line moves down by more than **3** source lines; no line other than the two additions changes; the README's compiled Rust fence opens no later than source line **35** (33 measured); and if the budget is threatened, the pointer's own sentence shortens toward 20 words rather than anything pre-existing moving. | `git diff -U0 <merge-base> -- crates/happenstance/README.md crates/happenstance/src/lib.rs` shows only added lines — zero modified and zero deleted lines. `rg -n '^```rust' crates/happenstance/README.md` reports a line number ≤ 35. Reviewed against the `readme-front-door` / `diff-against-main` frame in `design/mock.html`, which is the state `_design.md` declares for exactly this check. |
| AC-004 | GIVEN the evaluator who has read the sentence and decides to follow it, WHEN they activate the link, THEN it lands on real guide-level material through the **highest available rung** of the href ladder (intra-doc link, else an in-tree markdown link inside HS-P0020's pinned tree, else the named-but-unlinked form live at `crates/happenstance-core/src/store.rs:77`); the visible link text is a self-describing noun phrase of **at least three words** and never "here", "this", "docs" or a bare URL; and if only the fourth rung (*nothing*) is available the pointer is **not installed** — no placeholder, no "coming soon" — and the story blocks and escalates. | Rung 1: `cargo xtask ci --fast` covers it — the `documentation`, `documentation (no default features)` (`xtask/src/main.rs:502`) and `docs.rs configuration (nightly)` steps all run under `rustdoc::broken_intra_doc_links = "deny"` (`Cargo.toml:134`), so an unresolved link is a hard failure. Rung 2: HS-P0020's registration check over its pinned tree. Link text and the no-bare-URL rule: `rg -n "https?://" ` over the two added lines returns nothing, and the link text is read against `_design.md`'s `## Density budget`, "Minimum legible size". Rung 4: the absence of the two additions in the diff, plus a recorded escalation in the implementation report. |
| AC-005 | GIVEN a maintainer who must know, a year from now, that this pointer still resolves, WHEN the change merges, THEN rows **P1** and **P2** are present in the pointer register landed by `pointer-policy-and-inventory` — build-time data, never a page a reader can navigate to — each naming its surface, its destination, its N-3 form, the ladder rung chosen, and a **non-empty guard**, with P2's mirror-assertion guard recorded honestly as *not yet landed* if that is the truth at merge. | Read the register file HS-S0154 landed (path bound by that story, not guessed here) and assert two rows whose surface fields are `crates/happenstance/src/lib.rs` and `crates/happenstance/README.md`, each with a non-empty guard cell. The register's own shape check — the `check_summaries`-shaped step whose precedent is `xtask/src/lint_constitution.rs:463-473` reading `SUMMARIES` at `:64` — runs it if HS-S0154 landed one; otherwise the assertion is a reviewed read of the file, stated as such. Anti-pattern 14: `git diff --name-only` shows no new rendered index page. |
| AC-006 | GIVEN a keyboard-only or screen-reader reader at 1024x768, WHEN they reach either front-door surface, THEN the pointer is a plain link reachable by Tab with no widget, no "See also" block, no table, no raw HTML and no inline `style=`; nothing added carries meaning in colour or position; the existing heading ladder (`crates/happenstance/src/lib.rs:13,27,53`) is unskipped and unchanged; everything added reflows without clipping; and a reader who never follows the link is left exactly as unblocked as before, because the pointer is an *offer* and displaces no in-place answer. | Keyboard walk at 1024x768 over the rendered `cargo doc` output and the rendered README, recorded — this is not gate-checkable and `_design.md`'s `## Density budget`, "Two named gaps" (2), says so rather than papering over it. Static half: `rg -n "<[a-z]+|style=" ` over the two added lines returns nothing; `rg -n "^//! #" crates/happenstance/src/lib.rs` shows `13, 27, 53` unchanged in level and order. Reviewed against `design/mock.html`'s `narrow-viewport` and screen-reader rows in `_design.md`'s `## States`. |
| AC-007 | GIVEN the repository's merge gate, WHEN `cargo xtask ci --fast` runs on the merged result, THEN it is green with the pointer in place — every rustdoc step passes under `broken_intra_doc_links = "deny"`, `cargo test` still compiles the README's fences through the doctest mount at `crates/happenstance/src/lib.rs:10`, `cargo xtask spec-trace` stays green, and the diff shows **no** change to any public item, signature, feature, manifest entry or `#[doc(alias)]` attribute. | `cargo xtask ci --fast` (the bar `.redkiln/config.yaml` wires for a non-terminal project; project DoD item 1) and `cargo xtask affected --base main`. `cargo test -p happenstance --doc` for the README mount. `git diff` shows changes confined to the PR-boundary fence, with no `pub `, no `[features]`, no `Cargo.toml` and no `doc(alias)` line added. |

## Interaction quality

RFC §6.7/D6. Every invariant below is **already an AC row above** — this section only says which row
carries it and how it is checked. The medium is prose rendered by rustdoc, crates.io and GitHub, so
"unstyled render" here means *a diff that is textually correct and reads wrong on the page*: the
sentence present but below the fold, or bolded into a callout that out-weights the status
disclaimer, or mirrored as two different sentences. Those are what the composition rows fail.

### STATE invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not context-jump.** A reader who never follows the pointer is left exactly as unblocked as before; the pointer is an offer, not a step. | **AC-006**, and AC-003's "displaces no in-place answer" | Delete the destination in a scratch tree and confirm both surfaces still answer everything they answered on `main` (UX brief invariant 1's falsification, applied to the front door) |
| **Non-occlusion.** Nothing pre-existing is pushed off the first screen, folded, or reworded to make room. | **AC-003** | `git diff -U0` shows added lines only; the README fence's opening line number stays ≤ 35 |
| **Preserved reading position.** The existing heading ladder and section order are untouched, so a reader who bookmarked or deep-linked a section lands where they did before. | **AC-006** (ladder unchanged), **AC-003** (order unchanged) | `rg -n "^//! #" crates/happenstance/src/lib.rs`; `rg -n "^#" crates/happenstance/README.md` |
| **Reversibility.** The hop is walk-backable: the destination is the guide, the reader's browser back button is the medium's own affordance, and no state is lost. | **AC-004** | The recorded walk in `front-door-walk-record` (HS-S0160), which this story deliberately does not perform for itself |
| **Keyboard reachability.** Every affordance added is a plain link; nothing requires a pointer device, a hover, or a reveal. | **AC-006** | Keyboard-only pass at 1024x768, recorded — no linter exists for this and `_design.md` says so |
| **Non-occlusion of the guard.** A pointer with no guard is a state this story may not reach; a missing guard is a red row, not an empty cell. | **AC-005** | Non-empty guard cell in both register rows (anti-pattern 18) |

### COMPOSITION invariants

Taken from `_design.md`, which is **binding**. This story renders two of its five declared
surfaces.

| Invariant | Carried by | The real number or rule |
| --- | --- | --- |
| **Presentation exists at all** — the pointer is composed from a repository primitive, never bare markup | **AC-001** (rustdoc pre-heading top blurb), **AC-002** (markdown paragraph) | `_decomposition.md`, UX brief "Design-system primitives". Nothing is hand-rolled; `design/mock.html`'s finding F6 records that every element composes from a primitive already live in the tree |
| **Composition and placement** | **AC-001**, **AC-002** | Crate root: region **2** of `_design.md`'s composition table — after `:11`, before `# Status` at `:13`. README: between `:1-4` and the status blockquote at `:6-11`, **outside** it |
| **Transience** — persistent chrome, never revealed, never opened-on-demand | **AC-001** ("no reader has to expand"), **AC-006** (no fold, no tab, no `<details>`) | `_design.md`, `## Transience policy`, first row: *"A pointer that must be revealed is the gap restated."* The implementer must also confirm rustdoc has not rendered the module doc inside a collapsed item body |
| **Density budget** | **AC-001**, **AC-003** | First screen ≈ **27** rendered lines at 1024x768. Pointer within the first **5** rendered lines; costs **≤ 3** source lines (amended from 2 by finding F1). README compiled fence opens at source line **33**, budget **35**. Sentence **8–30** words (22 as pinned), link text **≥ 3** words, named need **≥ 4** words. Register **≤ 8** rows at project close, **≤ 20** ever |
| **Overflow / yield order** | **AC-003** | The **pointer's own sentence shortens** (30-word cap → 20). Nothing pre-existing moves, folds or collapses. On the README the *pointer* is re-placed, never the fence |
| **Hierarchy** | **AC-001**, **AC-002** | Position relative to the first `#` heading, and nothing else. No bold, no callout, no emoji, no second blockquote — weight is a claim, and this story makes none |
| **Anti-patterns this story can trip** (`_design.md`, `## Anti-patterns`) | 1 → AC-001; 2 → AC-002; 3 → AC-004; 4 → AC-002; 5, 6, 9, 10 → AC-006; 11 → AC-003; 14, 18 → AC-005; 16 → AC-006 | Each is checkable from a screenshot or a diff by someone who cannot read Rust — which is the point of stating them as numbers rather than as taste |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | Only ladder rung 4 (*nothing*) is available at implementation — HS-P0020 has not resolved a hosting shape and no in-tree page exists. | **Do not install the pointer.** No placeholder href, no "coming soon", no link to a `TODO`. The story **blocks** and escalates exactly as the project escalates for AC-010 (`_design.md`, `## States`, `Empty` row). A pointer into nothing is worse than today's silence. |
| **EC-002** | A rung-1 intra-doc link resolves in the default build but fails under `--no-default-features` or the nightly `--cfg docsrs` build. | Hard gate failure, not a review note (RS-70-2, `standards/rust/70-rustdoc-obligations.md`; AC-A02). Drop one rung — an in-tree markdown link, or the named-but-unlinked form at `store.rs:77` — and record the demotion in the register row. Never `allow` the lint. |
| **EC-003** | Merge-forward finds the placement invalid: on `initiative/from-contract-to-published-library` `crates/happenstance/src/lib.rs` is 237 lines, and HS-P0016 has edited the README. | **Re-decide placement as a non-occlusion judgement, do not rebase the hunk mechanically** (`_decomposition.md`, N-10). If the merged README would push the compiled fence past source line 35, the *pointer* moves, never the fence. Record the re-decision as a finding in the implementation report. |
| **EC-004** | The register file HS-S0154 lands is not present, or its shape does not admit the fields P1/P2 need. | The story **blocks on its dependency**. Do not invent a register, do not fork a second checker into `xtask/src/` (architecture brief N-4), and do not widen the PR-boundary fence to a guessed path. Widening the fence is a deliberate edit to *this spec*, naming the real path. |
| **EC-005** | The two sentences drift apart — a reword lands on one surface only. | Anti-pattern 2 fires. If the mirror assertion has landed, the build fails with the reason in the message; if it has not, the register row already says the guard is absent and the drift is a review finding. Any rebinding of the destination updates **both** surfaces and **both** rows in one change. |
| **EC-006** | `clippy::doc_markdown` fires on a CamelCase token in the added prose. | Backtick the identifier. The `allow` repair is forbidden by RS-70-3 (`standards/rust/70-rustdoc-obligations.md`), and `clippy.toml`'s `doc-valid-idents` is for proper nouns, not identifiers. |
| **EC-007** | The destination page is renamed or moved after this story merges. | The row's named guard fires — a rustdoc build fails, or HS-P0020's registration check fails. If the row's guard reads "none", rule 2(iv) was violated and the pointer should never have been installed (`_design.md`, `## States`, `Error` row). |
| **EC-008** | rustdoc renders the crate-root module doc inside a collapsed item body, putting the pointer behind a `[+]` nobody chose. | Confirm from the built `target/doc/happenstance/index.html` that `#main-content > .docblock` is not inside a closed `details.toggle`. If it is, the transience policy is violated by the renderer and the finding is escalated — this story installs no fold and must not inherit one (`_design.md`, `## Transience policy`, rustdoc-collapse row). |

## Non-functional

| id | Requirement | Number or bound |
| --- | --- | --- |
| **NF-001** | The addition stays inside the density budget on both surfaces. | ≤ 3 source lines per surface; pointer within the first 5 rendered lines of the crate root; README fence opening line ≤ 35; first screen ≈ 27 rendered lines at 1024x768, ≈ 16 occupied after this change |
| **NF-002** | No semver surface, and identical cost on every target. | Zero public items, signatures, features or manifest entries changed. Doc attributes and prose are erased before type-checking, so the cost is identical on `EventStore` and `SendEventStore` and on `wasm32` (`_design.md`, "What it costs a caller") |
| **NF-003** | The gate gains no new step and no measurable time. | This story adds nothing to `xtask/src/main.rs`. The mirror assertion is *asked for* and owned by HS-P0020 (architecture brief N-4) |
| **NF-004** | Bounded maintenance forever. | Two register rows, each with a guard that fails a build rather than a reviewer's memory. Register cap 8 rows at project close, 20 ever; a 21st row is the DT-10 alarm, not a scrollbar |
| **NF-005** | WCAG AA as it lands in a text medium. | Self-describing link text; colour and position never the only carrier; nothing animates; heading ladder unskipped; reflow at 1024x768 with no fixed width, table or column (`_decomposition.md`, UX brief "Accessibility floor") |
| **NF-006** | Copy fidelity outranks the density budget where they conflict. | The design gate resolved this at F3 for the `store.rs` transcript; the same ordering applies here in the mirror direction — the **sentence** yields (30 → 20 words) before any pre-existing content does |

## Implementation notes (non-prescriptive)

Sequencing, not prescription — the shape is `_design.md`'s and is not re-decided here.

- **Merge forward first, then decide placement.** Bring the branch level with
  `initiative/from-contract-to-published-library` before writing either hunk, and read both files as
  they stand *there*. This worktree's `crates/happenstance/src/lib.rs` is 75 lines; the merged one
  is reported at 237 and is **not verifiable from here**. Placement is an invariant-2 judgement
  against the merged text.
- **Bind the destination before writing the sentence.** The sentence is pinned; only `DESTINATION`
  moves. Walk the ladder top-down, stop at the first rung that is actually available, and write the
  rung into both register rows in the same change. If the walk ends at rung 4, stop — EC-001.
- **Write the crate-root hunk as two source lines of `//!` plus its blank line**, immediately after
  `:11` and before `:13`. Then produce the README hunk by rendering the same text: strip `//! `,
  keep the markdown link identical. Diff the two sentences against each other before committing —
  that is the cheapest possible stand-in for the mirror assertion that does not exist yet.
- **Build and look.** `cargo doc -p happenstance --no-deps` and open
  `target/doc/happenstance/index.html`. The design's approved frames are in `design/mock.html`;
  open it at `#all` and compare the `crate-root-front-door` and `readme-front-door` groups. This is
  the only perceptual check this project will ever get — `design.capture` is deliberately absent
  from `.redkiln/config.yaml`, so nothing downstream will look at these surfaces and disagree.
- **File the register rows last**, once the rung is known, and widen this spec's PR-boundary fence
  in the same commit that names the real register path. Never widen a fence to turn a red gate
  green.
- **Leave the walk to someone else.** `front-door-walk-record` (HS-S0160) is blocked on this story
  precisely so the author is never their own witness (project DR-10; `_storymap.md`, backbone B6).
- **Do not touch `crates/happenstance-core/**`.** The `store.rs` rewrite, the restored `error[E0034]`
  transcript and the two `#[doc(alias)]` attributes are `store-error-site-rewrite`'s, in a different
  slice, behind a different external gate.

## Tests and CI (merge gate)

This project takes **no testing brief** — `.bklg/docs-that-teach/_decomposition.md`'s warranted-brief
table gives it `architecture` and `ux` only, because its three walk criteria are observed rather than
automated (`project.md`, Out of scope, final row). The bar below is therefore the repository's
standing gate plus the diff and render checks the project's own DoD items 1, 6 and 7 name, and it is
honest about what nothing checks.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Preflight** | `git merge` forward from `initiative/from-contract-to-published-library`, then re-read `crates/happenstance/src/lib.rs` and `crates/happenstance/README.md` | Placement is decided against the tree that will actually merge, not this worktree's 75-line copy (EC-003) |
| **Static — presence** | `rg -c "Guide-level documentation" crates/happenstance/src/lib.rs crates/happenstance/README.md` returns `1` for each | AC-001, AC-002 — one authoritative text, present exactly once per surface; the pinned mirror substring is what a future `check_summaries`-shaped step would assert |
| **Static — mirror** | Extract the sentence from each file, strip `//! `, diff | AC-002 — anti-pattern 2 cannot pass silently while the real assertion is HS-P0020's to land |
| **Static — placement** | `rg -n '^```rust' crates/happenstance/README.md` ≤ 35; `rg -n "^//! #" crates/happenstance/src/lib.rs` first hit still after the pointer | AC-001, AC-003 — the density budget's two hard numbers |
| **Static — form** | `rg -n "https?://|<[a-z]+|style=" ` over the two added lines returns nothing | AC-004, AC-006 — no bare URL (forbidden outright by `_design.md` rule 2(iv)), no raw HTML, no inline style |
| **Diff gate** | `git diff -U0 <merge-base> -- crates/happenstance/README.md crates/happenstance/src/lib.rs` — added lines only, zero modified, zero deleted | AC-003, AC-007 — project **DoD item 7** made checkable by diff rather than by assertion; the HS-P0016 seam |
| **Build gate** | `cargo xtask ci --fast` (`xtask/src/main.rs`) — the bar `.redkiln/config.yaml` wires for a non-terminal project | AC-004, AC-007 — the `documentation`, `documentation (no default features)` (`:502`) and `docs.rs configuration (nightly)` steps under `broken_intra_doc_links = "deny"` (`Cargo.toml:134`); a rung-1 link that resolves in only some feature configurations is a hard failure |
| **Build gate — README mount** | `cargo test -p happenstance --doc` | AC-007 — the doctest mount at `crates/happenstance/src/lib.rs:10` still compiles the README's ```` ```rust ```` fences. **It does not check the prose pointer**, and the register row says so |
| **Build gate — story grain** | `cargo xtask affected --base main` | AC-007 — only what this diff could break, which is the grain `.redkiln/config.yaml`'s `verify:` block wires to stories |
| **Spec gate** | `cargo xtask spec-trace` | AC-007 — no clause citation is disturbed; this story discharges none and must break none |
| **Render review** | `cargo doc -p happenstance --no-deps`, then `target/doc/happenstance/index.html` at `#main-content > .docblock`, compared against `design/mock.html`'s `crate-root-front-door` and `readme-front-door` frames | AC-001, AC-002, AC-006, EC-008 — composition, transience and hierarchy, which no textual assertion can reach |
| **Keyboard / narrow-viewport walk** | Manual pass at 1024x768, keyboard only, recorded in the implementation report | AC-006 — **nothing in `cargo xtask ci` verifies keyboard reachability or self-describing link text**, and `_design.md`'s "Two named gaps" (2) states that rather than papering over it |
| **Dependency gate** | Read the register file landed by `pointer-policy-and-inventory`; assert rows P1 and P2 with non-empty guards. If HS-S0154 landed the checker, it runs — precedent `xtask/src/lint_constitution.rs:463-473` over `SUMMARIES` at `:64` | AC-005 — project **DoD item 6**; a story that installs a pointer and files no row has not finished (`_storymap.md`, Coverage, AC-003 row) |
| **Story gate** | `redkiln verify --grain story` over `_ledger.md` | Every AC above carries a row, `satisfied: true`, with cited evidence — and nothing was flipped without it |

## Risks and coupling (PR-scoped)

| Risk | Exposure in this PR | Handling |
| --- | --- | --- |
| **The merged tree moves both targets** | `crates/happenstance/src/lib.rs` 75 lines here vs 237 there; `crates/happenstance/README.md` edited by HS-P0016. **Not verifiable from this worktree.** | Preflight tier above. Placement is re-decided, not rebased (EC-003). If the merged README pushes the fence past 35, the pointer moves |
| **The HS-P0016 seam** | Two projects edit one README. | The seam is *purpose, not paragraph*: they own every claim, this story owns two lines that assert nothing. DoD item 7 makes it a diff (AC-003) |
| **Both external gates are still open** | HS-P0022's pages must exist and HS-P0020's hosting shape must be resolved (`_storymap.md`, Merge order, step 3). | EC-001. The story blocks rather than shipping a placeholder — the design's `Empty` state is explicit and is the state this branch is in today |
| **P2 has no live guard** | The README-as-doctest mount compiles fences, not prose; the mirror assertion does not exist. | Stated in the register row honestly as *not yet landed*, and asked of HS-P0020 rather than forked into `xtask/src/` (N-4). The interim guard is the mirror diff in the Static tier |
| **The register's path is not knowable yet** | The PR-boundary fence deliberately excludes it. | EC-004. Widen the fence in this spec, naming the real path, in the same commit that files the rows |
| **A second navigation surface** | DT-10's recorded cost — per-item pointers "become a second navigation surface to keep true". | This story adds exactly two rows against a cap of 8 / 20, and the register is build-time data, never a page (anti-pattern 14) |
| **Slice-mate coupling** | `evaluator-onward-links` (HS-S0159) is implemented in the same context and mounted as one surface. | Either order within the slice. They share the policy and the persona session but no file: this story touches `crates/happenstance/**`, that one touches HS-P0022's pages |
| **The author as their own witness** | Tempting to "just check the walk works" while implementing. | Forbidden by design: AC-004's walk is `front-door-walk-record`'s (HS-S0160), which is blocked on this story for exactly that reason. Do not pre-empt it, and do not record a walk here |
| **This is the last cheap disagreement** | The perceptual review is a standing skip; nothing downstream looks at these surfaces. | The render-review tier is not optional. `_design.md`'s sign-off says the written resolution is the only record there will ever be |

## Dependencies

**Blocks on**

- `pointer-policy-and-inventory` (HS-S0154) — lands the pointer register this story appends rows P1
  and P2 to, the permitted-forms record and the widget-rejection record. Its shape and its path are
  that story's; this story invents nothing about either. Matches `_storymap.md`'s `depends_on` for
  this row exactly.

**Slice-mate, not a dependency**

- `evaluator-onward-links` (HS-S0159) — same slice `front-door-reach`, implemented in the same
  context and mounted as one integrated surface, in either order (`_storymap.md`, Merge order,
  step 3).

**Unlocks**

- `front-door-walk-record` (HS-S0160) — depends on this story so its walker is not the author of
  what they walk.
- `second-question-walk-records` (HS-S0162) — depends on this story and on `evaluator-onward-links`.

**External gates, outside this project**

- **HS-P0020 `checked-documentation-surface`** — the hosting shape the href ladder resolves
  against, the pinned tree, and the registration check that guards rung 2.
- **HS-P0022 `application-author-path`** — the narrative material the pointer points *at*. Nothing
  to point at means EC-001.

## Anchors (progressive disclosure)

Load-bearing depth, deferred not optional. The `## Context pack` above is the must-read core; each
row below is opened at the moment named, never in bulk.

| Anchor | Why it is load-bearing | When to open it | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` | **Binding.** Carries the pinned sentence verbatim (`## Signatures`), the composition tables, the transience policy, the real density numbers and the eighteen anti-patterns. Nothing in it is re-decided by this story | Before writing either hunk — read `## Signatures`, `## Composition`, `## Density budget` | AC-001, AC-002, AC-003, AC-006 |
| `.bklg/docs-that-teach/reach-and-adapter-path/design/mock.html` | The approved frames. 38 labelled frames including `crate-root-front-door` and `readme-front-door` at both viewports, and the six findings (F1 amended the three-line rule this story is measured against) | At the render-review tier, after `cargo doc`; open at `#all` | AC-001, AC-002, AC-003 |
| `crates/happenstance/src/lib.rs` | The mount point (CR-1). Lines 10-13 are the exact insertion site; `:20-25` is ADR-0006's conclusion the pointer must not contradict; `:69-71` routes adapter authors away | First, before deciding placement — and again on the merged tree | AC-001, AC-007 |
| `crates/happenstance/README.md` | The second front-door surface. Lines 1-11 fix the insertion site; line 30 is the compiled fence whose new position the budget is written against | With the file above, at placement time | AC-002, AC-003 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_decomposition.md` | UX brief invariants 2 and 9, UX-AC-01..03, the accessibility floor and the design-system primitive table; architecture brief N-3 (the four pointer forms and what catches each rotting) and N-4 (why this story must not fork a checker) | When binding the destination off the ladder, and when writing the register rows | AC-004, AC-005, AC-006 |
| `.bklg/docs-that-teach/reach-and-adapter-path/pointer-policy-and-inventory/spec.md` | The dependency's own spec — the register's shape, its home and its guard rule are decided there, not here | Before filing rows P1 and P2 | AC-005 |
| `xtask/src/lint_constitution.rs` | The working precedent the register copies: `SUMMARIES` at `:64`, `check_summaries` at `:463-473` — a named list of surfaces crossed with a pinned constant, failing the build with the reason in the message | When writing the register rows, and if HS-P0020 asks what shape the mirror assertion should take | AC-005 |
| `Cargo.toml` | Line 134, `broken_intra_doc_links = "deny"` — the guard that makes rung 1 of the ladder stronger than rung 2, across three rustdoc builds | When choosing the ladder rung | AC-004, AC-007 |
| `xtask/src/main.rs` | The gate itself: the `documentation` step, `documentation (no default features)` at `:502`, and the nightly `docs.rs configuration` step. The `probe` contract at `:85-102` is why a tool that runs and finds a problem is never a skip | Before claiming the build gate is green | AC-007 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-2 (a link must resolve in every feature configuration) and RS-70-3 (the `allow` repair is forbidden) — the two rules an over-eager intra-doc link breaks | On EC-002 or EC-006 | AC-004, AC-007 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | Accepted. Why the guide pointer belongs on `happenstance` and the adapter pointer may not | If placement is ever questioned, or if someone proposes adding an adapter pointer here | AC-001 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 3's goal and the timing constraint the criteria are framed from — one reading session, no second attempt if the first fails silently | When judging whether a criterion is framed from intent rather than capability | AC-001, AC-002 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The two-surface split (tokio/serde/diesel convergence) the sentence imitates, and the anti-pattern list that rules out a widget: the gap "is evidence of a missing *link*, not a missing *widget*" | If anything more elaborate than a sentence is proposed | AC-006 |
| `docs/README.md` | The repository's real routing table at `:12-23` — ten destinations. It is the case a table is *for*, and therefore why a one-row table here would be a widget | Only if a routing affordance is proposed | AC-006 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_grounding.md` | Line-level verification of both target files and of the sibling-project dependency state (HS-P0020 and HS-P0021 at `stage: storymap` with no `_design.md`) | Before trusting any line number in this spec, and before assuming an external gate has opened | AC-003, AC-004 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_storymap.md` | Merge order step 3 and its two hard external gates; the Coverage table's AC-003 split ownership ("a story that installs a pointer and files no row has not finished") | At the start, and again before declaring done | AC-005 |
| `.bklg/docs-that-teach/initiative.md` | DoD scenario 7 verbatim, and the DT-10 row's recorded cost that the register cap is wired to | When writing the implementation report's DoD advancement claim | AC-001 |
| `crates/happenstance-core/src/store.rs` | Line 77 is the **live instance** of the named-but-unlinked cross-reference form — ladder rung 3. Read the form, do not edit the file | Only if the ladder falls to rung 3 | AC-004 |

## Clarifications resolved during spec

1. **AC ids are exactly the seven the front half decided** — AC-001 through AC-007. None added, none
   dropped. The ledger matches.
2. **Three source lines, not two, and fence line 33, not 32.** `_design.md`'s `## Composition` and
   `## Density budget` tables say "2 source lines" and "fence opens at source line 32"; its own
   finding **F1** and the F1 disposition table amend both to **3** and **33**, and amend
   anti-pattern 11 from "two lines" to "three". The later disposition wins — it is the design gate's
   own correction of the earlier draft, made because a 22-word sentence with its markdown link does
   not fit one 80-column line. This spec uses 3 and 33 throughout, against an unchanged budget of 35.
3. **Project AC-002's mechanism claim is not inherited.** AC-002 reasons that the README doctest
   mount makes a malformed pointer a build failure. That mount compiles the README's ```` ```rust ````
   fences, not its prose, and this design places the pointer as prose. The spec states the real guard
   (the mirror assertion, which does not exist yet) and records it honestly rather than repeating a
   claim that would not hold. `_design.md`'s "Two named gaps" (1) is where this was first named.
4. **AC-012 is deliberately not traced here.** `_storymap.md`'s Coverage table excludes
   `front-door-pointer` from AC-012 because this story authors no page — it adds one sentence twice.
   The two traced project ACs are AC-002 and AC-003 and no more.
5. **No conformance rule, no clause, no public item.** Nothing here is adapter-observable; doc
   attributes and prose are erased before type-checking. `spec/SPECIFICATION.md` is not edited and
   `cargo xtask spec-trace` is a gate step here, not a discharge.
6. **The keyboard and link-text checks have no linter, and that is stated rather than assumed away.**
   `_design.md`'s "Two named gaps" (2) says nothing in `cargo xtask ci` verifies either. AC-006 is
   therefore verified by a recorded manual pass at 1024x768 — and the *observed walk* that would make
   it evidence rather than assertion is `front-door-walk-record`'s, deliberately not this story's.
7. **The register's path stays out of the PR-boundary fence.** HS-S0154 binds it; guessing it here
   would create a second description that can drift. Widening the fence is an explicit, later edit to
   this spec.
8. **The merged-tree state is asserted, not verified.** Every claim about
   `initiative/from-contract-to-published-library` (237-line `lib.rs`, HS-P0016's README edits) comes
   from `_grounding.md` and `_decomposition.md` and is **not verifiable from this worktree**. The
   preflight tier exists to convert it into a verified fact before either hunk is written.
