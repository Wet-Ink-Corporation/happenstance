---
item: HS-S0146
stage: spec
created: 2026-08-17T13:16:07.629Z
updated: 2026-08-17T13:16:07.629Z
template_sig: 87bbf1d0
rendered_sig: cfaa37b6
---

# Spec — The closed need set and the declaration form, landed once

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — AC-07, DoD-8, DoD-14, DT-2, DT-3 |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) |
| Project charter | [`.bklg/docs-that-teach/page-need-discipline/project.md`](../project.md) — AC-003, AC-004; DR-03, DR-04, DR-05 |
| This spec | `.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — Architecture brief AC-003/AC-004 + Notes 1, 2, 4, 7, 9; UX brief UX-001/UX-002/UX-003/UX-012; Testing brief AC-003/AC-004/AC-005 |
| **Signed-off design (binding)** | [`../_design.md`](../_design.md) — S1 pattern decision, DT-2, DT-3, the `standards/pages/` pin table, Density budget, States, Anti-patterns, Sign-off conditions 1–4 |
| Story map / merge order | [`../_storymap.md`](../_storymap.md) — slice `discipline-on-disk`, row 1 of 8, the only `foundation` story |
| Grounding | [`../_grounding.md`](../_grounding.md) — no Accepted decision atom constrains this work; tensions 2 and 4 |

## One-line PR slice

Land the closed need set **once** — as the `NEEDS` const in a new `xtask/src/lint_pages.rs`
mounted in the bin crate's module list, and as the two rule atoms that document it
(`standards/pages/00-one-need.md`, `standards/pages/10-the-need-set.md`) — carrying DT-2's,
DT-3's and DR-05's already-signed-off resolutions, their rejected options, and the HS-P0020
hosting assumption the declaration form rests on.

## Executive summary

This PR creates the rules tree's first two files and the vocabulary they describe, and it
creates the Rust module that will hold every later check.

**Delta against the plan of record.** The one-line slice in [`../_storymap.md`](../_storymap.md)
says "resolve DT-2, DT-3 and DR-05 in `_design.md`". **That half is already done and signed
off** ([`../_design.md`](../_design.md), "Sign-off": approved 2026-08-17, four conditions
carried rather than waived). So this story does **not** decide the taxonomy, the declaration
grammar or findability's status, and **must not edit `_design.md`** — an accepted sign-off is
the referent, and rewriting it is the move
[`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)
forbids. What is left, and what this PR lands, is the *transcription into a place that binds*:

1. `xtask/src/lint_pages.rs` — a new bin-crate module carrying `NEEDS` (four members, closed),
   `MAX_NEEDS`, a compile-time ceiling assertion, the `RULE_DIR` / `ROUTER` path pins the design
   fixed, a pure membership accessor, and the module docs whose "What this does not verify"
   section comes **first**.
2. `standards/pages/00-one-need.md` — band 00: one need per page, and the declaration's exact
   grammar and position.
3. `standards/pages/10-the-need-set.md` — band 10: the four tokens with the rejected options
   named, plus the two `orientation` ceiling rules the design fixed as RP-10-2 and RP-10-3.

Nothing else. No router (`router-precedence-and-announcement`), no checker, no gate step, no
`INERT` entry, no `docs/README.md` row (`page-need-checker-mounted-in-the-gate` and the router
story own those). The reason this is its own story rather than a checkbox on the checker is
stated in [`../_storymap.md`](../_storymap.md), "Why the slices fall here": a need set written
once in prose and once in Rust is the *"three lists that must agree"* defect
`xtask/src/spec_trace.rs:107-116` records, and landing the set before either consumer is what
forecloses it.

## Context pack

Read this section and you can start. Everything below it is signposted retrieval, not
prerequisite reading.

### The decisions this story is downstream of, stated as decisions

**The need set is four closed tokens: `orientation`, `tutorial`, `how-to`, `explanation`.**
DT-2 resolved as *option (c), adopt with a stated local extension* — one subtraction and one
addition, each with a named cause. `reference` is **removed** because rustdoc and
`spec/SPECIFICATION.md` are already this workspace's two authoritative reference surfaces, so a
`reference` bucket in a narrative tree is either permanently empty or becomes a second
specification. `orientation` is **added** (DT-3). Literal Diátaxis lost because the evidence
names *this project's exact kind of subject* — a dense, interrelated conceptual model — as where
the four-box split strains; dropping the enumeration lost because a lint cannot check membership
in an open set, which would make the project a convention; a persona-keyed taxonomy lost because
a need is a property of the page and a persona is a property of the reader, so the adapter author
reading both explanations and how-tos would make almost every page declare two.
(`../_design.md`, "S1 vocabulary — DT-2")

**Findability is a first-class need with a ceiling, not an implicit byproduct.** DT-3 resolved as
option (a). The stated cost is a category the source taxonomy does not have; the mitigation is
that the category is *unable to grow into a sink*, as two rules this story writes: an
`orientation` page carries links and at most one sentence per destination and **teaches nothing**
(RP-10-2), and there is at most **one** `orientation` page per directory level (RP-10-3). The
rejected option is the one that reproduces the measured defect — the evaluator's *"good until the
second question, and then nowhere to go"*. (`../_design.md`, "S1 vocabulary — DT-3")

**The declaration is a first-line blockquote, and its position is its meaning.** DR-05 resolved:
immediately after the page's `# Title`, one blank line above and below, a single line
`> **Answers:** `` `token` `` — <the reader's question>?`. The token is one member of `NEEDS`, in
backticks; the clause after ` — ` is a question in the reader's voice ending in `?`. **Nothing may
be inserted between the H1 and the declaration** — no badge row, no table of contents, no
admonition, no "last updated" line — because UX-001's test is *read nothing but the region above
the first prose paragraph and name the need*, and any interposed element makes that region
ambiguous. This is not a new convention: it is the repository's own `> **Load when:** …` line
(`standards/rust/00-prime-directives.md:3-6`), parsed by `load_when`
(`xtask/src/lint_constitution.rs:247-257`) and required by `check_shape`. Rejected: YAML front
matter (mdBook has none, and an `include_str!`'d block renders as body text or a horizontal
rule), an HTML comment (machine-readable and invisible — the sidecar failure in an inline
costume), a filename/directory convention (invisible on the rendered page, and it forces the
taxonomy into the tree), a sidecar `pages.toml` (two artefacts that must agree), and a
badge/icon/coloured admonition (fails colour-never-alone and the greyscale test).
(`../_design.md`, "S1 — `page-need-declaration`")

**The hosting assumption is part of the decision and must travel with it.** The form assumes
*only* that the medium renders CommonMark blockquotes as visible body text in document order. It
assumes nothing about front matter, directory-derived navigation, or a renderer's metadata layer.
HS-P0020's hosting choice is still open in this worktree; both of its stated options satisfy the
assumption. A third that does not — a renderer that strips or relocates leading blockquotes, or
one that requires front matter — **invalidates this decision and re-opens DR-05**, and the design
requires the checker's module docs to say so *in those words* (`../_design.md`, "Hosting
assumption…"; sign-off condition 1). This story writes that sentence, because this story is the
one that creates the module.

**`standards/pages/` is pinned, and pinning it is not free.** The design fixed `RULE_DIR =
standards/pages`, `ROUTER = standards/pages/README.md`, the task name `lint-pages` and the step
name `every page declares one need`, and recorded that renaming the directory later *is not a
rename*: it costs three edits in `xtask/src/main.rs`, an `INERT` entry at
`xtask/src/affected.rs:249-260`, and two `affected` tests (sign-off condition 2). This story
creates the directory and takes the two path consts; it does **not** take the `main.rs` dispatch
edits or the `INERT` entry — those belong to `page-need-checker-mounted-in-the-gate`, whose story
map row claims them "in one change".

**The rule-id prefix is `RP-NN-N`.** `RS-` is the constitution's and `PS-` is a live
`spec/SPECIFICATION.md` clause family, so a `PS-` rule id would read as a normative clause it is
not. (`../_design.md`, "Surfaces")

**No ADR constrains this work, and writing one would breach a non-goal.** All seventeen atoms
under `.kb/decisions/` were read by title at grounding; none governs documentation trees, gate
structure or narrative conventions. The binding authority is sub-ADR: `CLAUDE.md`, the five-tier
precedence chain at `standards/rust/README.md:23-29` (**not edited here, and not extended**),
`docs/README.md:25-29`'s pin-by-path rule, and `standards/rust/80-the-gate.md` /
`81-checks-that-cannot-be-types.md`. Do not author an ADR for this story.
(`../_decomposition.md`, Architecture brief, "No Accepted decision atom constrains this work")

### The decisions this story takes, and their consequences

**The const carries exactly the two columns a machine will read.** `NEEDS` is
`const NEEDS: &[Need]` where `Need { token, job }`, in the design's own order
(`orientation`, `tutorial`, `how-to`, `explanation`) — the shape and the doc comment copied from
`xtask/src/spec_trace.rs:107-121`, which is the in-repo precedent for "this is the single place
X is enumerated… three lists that must agree, kept as one so that adding a family cannot
half-land." The design's third column (*success for the reader*) stays **prose in band 10 and out
of the const**, because a struct field nothing ever reads is dead code with a doc comment on it,
and the whole point of the two fields chosen is that both have a named consumer:
`token` is the membership test, `job` is the router's generated row.

**The ceiling is a compile-time fact, not a unit test.** `MAX_NEEDS: usize = 6` and
`const _: () = assert!(NEEDS.len() <= MAX_NEEDS, "…");`. The design set the ceiling so the answer
to a straining page is never "add a fifth token" (`../_design.md`, DT-2 "Failure mode and
mitigation"). A `#[test]` would prove the same thing and could be deleted by anyone; a `const`
assertion cannot be, and it fails at `cargo check` rather than at `cargo test`.

**The scaffold is one line and it names the story that deletes it.** At this story's checkpoint
nothing in the bin build reads `RULE_DIR`, `ROUTER`, `Need::job` or the membership accessor — the
consumers are the router's generated region and the checker's membership test, both in later
stories. `dead_code` is warn-by-default and the gate runs
`clippy --locked --workspace --all-targets --all-features -- -D warnings`
(`xtask/src/main.rs:116-127`), so this is a hard failure unless it is handled. It is handled by
**one** module-level `#![allow(dead_code, reason = "…consumed by
page-need-checker-mounted-in-the-gate, which deletes this line…")]`, which is exactly the
convention `CLAUDE.md` already runs for skeletons ("a scoped `#![allow(clippy::todo)]` naming the
phase that removes it", worked at `crates/happenstance-sqlite/src/lib.rs:80`). The reason string
carries the consuming story's slug so `rg page-need-checker-mounted-in-the-gate xtask/src` finds
the obligation.

**`#![expect(dead_code)]` is the wrong tool here, and it is worth knowing why.** It looks
strictly better — an `expect` that stops being needed becomes a warning and forces its own
deletion. But the gate builds `--all-targets`, and this module's `#[cfg(test)] mod tests` uses
every item; in the test target the lint therefore does not fire, the expectation is unfulfilled,
`unfulfilled_lint_expectations` warns, and `-D warnings` fails the gate on the very attribute
added to keep it green. Use `allow`.

**Band 00 states the declaration may never be occluded and *defers* the renderer question.** The
mock found that rustdoc wraps every page in `<details class="toggle top-doc" open>`
(`../_design.md`, Mock finding 2), which puts never-fold class 1 inside a disclosure mechanism
nobody has observed. DT-8 Part 3 owes a sentence on whether a renderer-supplied, open-by-default
wrapper counts as a *mechanism*. That sentence is `fold-line-rule`'s (band 20), not this story's.
Band 00 states the rule (the declaration is never folded, tabbed or collapsed) and points at band
20 for the mechanism list; it must not settle the open question in passing.

### The persona-journey slice this realizes

The three reader personas are drafts, not promoted `.kb/product/` atoms — cite
`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`, never `.kb/product/`
(`../_decomposition.md`, UX brief, "Intent"). This story serves the *shape* of their first three
seconds on any page:

- **The evaluator** has twenty minutes and no second attempt; their named gap is not missing
  content, it is *"nowhere for that question to go"*. `orientation`'s existence as a first-class
  token (DT-3) is what stops the page that serves them from being mis-slotted and then flagged by
  this project's own rule as answering a second need.
- **The application author** fears *silent wrongness* — "a mental model that looks right,
  compiles, runs, and is quietly wrong". A page that quietly answers two needs is the same defect
  one level up: it reads as complete and is not. The one-need rule is what makes that visible.
- **The adapter author** carries the sharpest measured defect on record: the `E0034` explanation
  exists, but in three contributor-facing documents, none of them the file the reader is looking
  at. This story does not fix that — it fixes the vocabulary in which "which need does this page
  answer" can be asked at all.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `foundation` — real in-tree substrate (a `const` and two rule atoms; no double, no `todo!()`), consumed inside this initiative by two named stories |
| **Slice / milestone** | `discipline-on-disk` |
| **Slice-mate story slugs** | `router-precedence-and-announcement`, `fold-line-rule`, `reviewer-and-citation-procedures` |
| **Mount point** | **`xtask/src/main.rs`** — the bin crate's module list at `xtask/src/main.rs:64-70` gains `mod lint_pages;` (CR-1, `../_decomposition.md` Architecture brief Note 1). That declaration is the mount: an `xtask/src/*.rs` file not named there is not compiled by anything |
| **Wires into** | `xtask/src/spec_trace.rs:107-121` (the `Need` struct's shape and doc-comment precedent) · `xtask/src/lint_constitution.rs:55-58` (the `ATOM_DIR` / `ROUTER` const shape `RULE_DIR` / `ROUTER` copy) · `xtask/src/lint_constitution.rs:67-81` (the five-section marker list the atoms are authored against) · `standards/rust/00-prime-directives.md:1-8` (the atom head grammar) · `standards/rust/README.md:99-107` ("The shape of an atom") |
| **Design-system primitives consumed** | The textual primitive table at `../_decomposition.md`, UX brief Note 1: the `> **Load when:**` / `> **See also:**` head, `# NN — Title`, `## RP-NN-N. <imperative sentence>`, the five fixed sections **Why.** · **Do** · **Not** · **Rejects.** · **Evidence.**, and the ceilings `MAX_RULES_PER_ATOM = 6` (`xtask/src/lint_constitution.rs:88`) and `MAX_ATOM_BYTES = 16_384` (`:95`) |
| **Renders surfaces** | `rule-atom` — two instances, `standards/pages/00-one-need.md` and `standards/pages/10-the-need-set.md`, in state `populated` and inside every ceiling in the design's Density budget. `page-need-declaration` is **specified normatively here and rendered nowhere**: its `route` is HS-P0020's `PAGE_DIR`, which is unpinned in this worktree (`../_design.md`, "Surfaces"). `discipline-router`, `lint-terminal-output` and `reviewer-procedure` are **not** this story's |
| **Public items** | None. This project changes no public API: its only Rust is in `xtask`, which is `publish = false`, and its items are `pub(crate)` or private by construction (`../_design.md`, "Template sections that do not apply") |
| **Conformance rule(s)** | **None, and this is not adapter-observable.** No port, no value type, no testkit file is touched; `happenstance-testkit`'s suite cannot observe a markdown tree or a bin-crate const. The instruments that *can* observe this story are `cargo check -p xtask` (the compile-time ceiling), `cargo test -p xtask` (this module's own `#[cfg(test)] mod tests`) and `cargo xtask ci --fast` |
| **Clause(s)** | **None.** Nothing here amends, discharges or restates a `spec/SPECIFICATION.md` clause; `cargo xtask spec-trace` remains the only writer of its generated sections (`../_decomposition.md`, Architecture brief Note 8). Band 00 *requires* pages to cite clause ids and never restate them — the rule text for that is `reviewer-and-citation-procedures`' (DR-09) |
| **Advances DoD scenario** | Initiative **DoD-8** — "Every page's answered need is stated and singular… a check a reviewer can actually perform rather than one that depends on the author's memory." This story lands the vocabulary and the form without which "stated and singular" has no referent. Secondarily **DoD-14** — "The discipline is on disk and cited": the *on disk* half begins here (first two atoms in the decided home); *cited* is `governed-page-cites-the-discipline`'s |

**Delivered mounted, not as an isolated component.** The proof of mounting for this story is
that `cargo check -p xtask` compiles the new module and the compile-time ceiling assertion runs,
`cargo test -p xtask` executes its tests, and `cargo xtask ci --fast` is green at the checkpoint
— all of which are false for a file that exists in `xtask/src/` and is absent from
`xtask/src/main.rs:64-70`.

**One expected, non-defective consequence at this checkpoint.** `standards/pages/` is not yet in
`INERT` (`xtask/src/affected.rs:249-260`) and does not match the `standards/rust/` prefix arm
(`:214-221`), so an unrecognised path widens the affected set to every workspace member
(`:222-223`, asserted by `an_unrecognised_path_widens_rather_than_narrows`). `cargo xtask
affected --base main` is therefore **correct and slow** on this story's diff. Do **not** "fix" it
by broadening that arm to `standards/`: the pair (`INERT` entry plus the unconditional-list
entry) is `page-need-checker-mounted-in-the-gate`'s, in one change, with the two guard tests
(`../_decomposition.md`, Architecture brief Note 1, CR-4).

## PR boundary

**In this PR**

- `xtask/src/lint_pages.rs` — new. Module docs (limits first), `Need`, `NEEDS`, `MAX_NEEDS`, the
  `const _` ceiling assertion, `RULE_DIR`, `ROUTER`, the pure membership accessor, one
  module-level `#![allow(dead_code, reason = …)]`, and `#[cfg(test)] mod tests`.
- `xtask/src/main.rs` — the module list at `:64-70` only. One line: `mod lint_pages;`, in
  alphabetical position.
- `standards/pages/00-one-need.md` — new. Band 00: one need per page; the declaration's grammar,
  position and character budget; the never-occluded rule with the mechanism question deferred to
  band 20.
- `standards/pages/10-the-need-set.md` — new. Band 10: the four tokens, the rejected options, and
  RP-10-2 / RP-10-3.
- `.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/**` — this
  story's `_ledger.md` and its implementation report.

**Explicitly not in this PR**

- `standards/pages/README.md`, the band table, the precedence statement, the `## Start here`
  filter, the generated `## Index` region and its markers — `router-precedence-and-announcement`.
- Any `docs/README.md` edit: neither the "Looking for / It is at" row (`:12-24`) nor the
  gate-read-trees paragraph (`:25-29`) — `router-precedence-and-announcement`.
- `standards/pages/20-the-fold-line.md`, `PERMITTED_FOLD_MECHANISMS`, and DT-8 Part 3's sentence
  on renderer-supplied wrappers — `fold-line-rule`.
- Bands 30/40, the cite-never-restate rule text and the two reviewer procedures —
  `reviewer-and-citation-procedures`.
- The page parser, the zero/two/unenumerated checks, the directory guards, the vacuity `bail!`,
  the generated-region equality check, `Mode`, `run`, and every mount in `REQUIRED`, dispatch,
  `print_help`, `lint_steps` and `affected.rs` — `page-need-checker-mounted-in-the-gate`.
- Any edit to `standards/rust/README.md` (`git diff main -- standards/rust/README.md` must stay
  empty), to `xtask/src/lint_constitution.rs`, to `xtask/src/constitution.rs`, to `xtask/src/lib.rs`
  (CR-0: this tree is deliberately **not** registered with the doctest harness), to `.kb/`, to
  `.redkiln/templates/`, or to `_design.md`.
- Any `spec/SPECIFICATION.md` clause, any ADR, any crate under `crates/`.

**The implementer may touch the wiring file named in the Integration contract**
(`xtask/src/main.rs`'s module list) to mount this slice. That is the mount, not scope drift.

**Merge DoD, one line.** `cargo xtask ci --fast` green, `cargo test -p xtask` green,
`cargo xtask affected --base main` green (wide, per the note above), `git diff main --
standards/rust/README.md` and `git diff main -- .kb` both empty, `redkiln doctor` still reporting
exactly six `template-drift` advisories, and `_ledger.md` carrying a cited row per AC.

```
xtask/src/lint_pages.rs
xtask/src/main.rs
standards/pages/00-one-need.md
standards/pages/10-the-need-set.md
.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The vocabulary exists exactly once** | `const NEEDS: &[Need]` in `xtask/src/lint_pages.rs`, four members in render order: `orientation`, `tutorial`, `how-to`, `explanation`. `struct Need { token: &'static str, job: &'static str }`, both fields documented, the struct carrying a doc comment naming what agrees with it (the membership test, the router's generated row, band 10's table) | `../_design.md` "S1 vocabulary — DT-2"; shape and doc-comment precedent `xtask/src/spec_trace.rs:107-121` |
| **`reference` is not a member, and cannot become one by accident** | The accessor rejects `reference`; band 10 states *why* it was subtracted (rustdoc and `spec/SPECIFICATION.md` already own reference, and a bucket here becomes a second specification). A `reference` declaration is anti-pattern 16 | `../_design.md` "S1 vocabulary — DT-2", "Anti-patterns" 16 |
| **The set is closed at six by the compiler** | `const MAX_NEEDS: usize = 6;` and `const _: () = assert!(NEEDS.len() <= MAX_NEEDS, "<literal>");`. A seventh member fails `cargo check -p xtask`, not a test run | `../_design.md` Density budget ("Need tokens in `NEEDS` — ≤ 6, four today"); const-assert has no in-repo precedent and is this story's decision |
| **Membership is one pure function, no second parser** | `fn need(token: &str) -> Option<&'static Need>` — exact, case-sensitive, no trimming, no aliasing. It is the only place a token is judged; the checker story calls it rather than re-deriving the set | `../_decomposition.md` Architecture brief AC-006 ("the same function serves the counting check, the enumeration check and the checker's own tests without a second parser") |
| **Path pins live with the vocabulary** | `const RULE_DIR: &str = "standards/pages";` and `const ROUTER: &str = "standards/pages/README.md";`, in the shape of `ATOM_DIR` / `ROUTER` | `../_design.md` "`standards/pages/` is pinned here" table; `xtask/src/lint_constitution.rs:55-58` |
| **The module's limits are stated first** | Module docs open with `# What this does not verify`, item 1 unhedged: *it checks that a need is declared, never that the page answers it*, naming DR-07's reviewer procedure as the instrument. Then: it does not judge whether the set is the right set; the fold rule is enforced only to the extent the markers are textual; it does not resolve clause ids (HS-P0020's `clause_ids`); a rules tree that is empty passes every check below the vacuity guard; length is not quality | RS-81-1, `standards/rust/81-checks-that-cannot-be-types.md:11`; the shape at `xtask/src/lint_constitution.rs:9-28`; the required contents at `../_decomposition.md` Architecture brief Note 7 |
| **The hosting assumption is in the module docs, in the design's own words** | A named paragraph stating the form assumes only that the medium renders CommonMark blockquotes as visible body text in document order, and that a renderer which strips or relocates leading blockquotes, or requires front matter, **invalidates the decision and re-opens DR-05** | `../_design.md` "Hosting assumption this rests on…" and sign-off condition 1; `project.md` Risks row 1 |
| **Band 00 fixes the declaration grammar** | `standards/pages/00-one-need.md`: exactly one declaration per page; the line is `> **Answers:** \`token\` — <question>?`; it sits immediately after the H1 with one blank line either side; nothing may be interposed; the whole line is ≤ 96 characters (≤ 80 target); the token is never abbreviated, initialised, dropped, or rendered by colour/icon alone; when the question will not fit, **the page is answering more than one need and the overflow is the diagnosis** | `../_design.md` "Composition — S1", Density budget + yield order, UX-001/UX-002 at `../_decomposition.md` UX brief |
| **Band 00 forbids occlusion and defers the mechanism list** | The declaration may never sit behind a fold, tab, inactive panel or `<details>` (never-fold class 1); the *question of what counts as a mechanism* — including a renderer-supplied open-by-default wrapper — is band 20's, cited by band number in `> **See also:**`, not answered here | `../_design.md` DT-8 Parts 1-3, Mock finding 2, Anti-patterns 5-6; UX-003 |
| **Band 10 carries the four tokens and the rejected options** | The token table (token · the page's job · success for the reader — the third column prose-only), and the named losers: literal Diátaxis, drop-the-enumeration, persona-keyed. Plus the amendment rule: a fifth member is a **two-part commit** — the `NEEDS` const *and* the atom that justifies it — never a one-part one | `../_design.md` "S1 vocabulary — DT-2" including "Failure mode and mitigation"; `project.md` AC-003, DR-03 |
| **Band 10 carries the two `orientation` ceilings verbatim in effect** | RP-10-2: an `orientation` page contains links and at most one sentence per destination saying what the destination answers; it teaches nothing, and the moment it explains, instructs or references it is answering a second need and must be split. RP-10-3: at most one `orientation` page per directory level | `../_design.md` "S1 vocabulary — DT-3"; `project.md` DR-04 |
| **Both atoms are shape-clean before the checker exists** | `# NN — Title`; `> **Load when:** …`; `> **See also:** …` naming sibling bands **by number, not by link** (the constitution's own spelling, so no dangling link exists before those bands land); `---`; then `## RP-NN-N. <imperative sentence>` each followed by **Why.** · **Do** · **Not** · **Rejects.** · **Evidence.** in that order. ≤ 6 rules and ≤ 16,384 bytes per atom; prose ≤ 96 columns outside tables; every `**Rejects.**` names a wrong page that could ship and clears 120 characters | `standards/rust/00-prime-directives.md:1-8`; `standards/rust/README.md:99-107`; `xtask/src/lint_constitution.rs:67-81,82,88,95`; `../_design.md` "S3 — a rule atom" and Density budget |
| **No `rust`-tagged and no untagged fence anywhere in `standards/pages/`** | Fences are tagged `text` or `markdown`. A `rust` fence here is a Rust claim nothing compiles (this tree is deliberately absent from `xtask/src/lib.rs:28`); an untagged one is rejected too, so a future decision to register the tree cannot be undermined retroactively | `../_decomposition.md` Architecture brief Note 4 (Divergence 1); `../_design.md` "S3", Anti-patterns 13 |
| **`Evidence.` ordering** | Repository `path:line` first, then the discovery dossier, then external URLs with a `*(checked …)*` stamp | `standards/rust/README.md:99-107`; `../_design.md` "S3" |
| **The scaffold is one line and self-identifying** | Exactly one `#![allow(dead_code, reason = "…")]`, module-scoped, whose reason names `page-need-checker-mounted-in-the-gate` as the story that deletes it. Not `#![expect]` (see the Context pack), not crate-level, not per-item, not `#[allow(dead_code)]` without a `reason =` | `CLAUDE.md` ("🔩 skeleton": a scoped allow naming the phase that removes it), worked at `crates/happenstance-sqlite/src/lib.rs:80`; the gate's clippy invocation at `xtask/src/main.rs:116-127` |
| **The module's own tests, and what they may touch** | Unit tests over the pure accessor (each of the four tokens accepted; `reference`, `guide`, `Explanation` and `explanation ` each rejected) and one test that reads the **real** `standards/pages/10-the-need-set.md` under `RULE_DIR` and asserts every `NEEDS` token appears in it and no other backticked token does — failing with a message naming **which token moved** (RS-81-5). `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]` inside the test module, as `xtask/src/lint_constitution.rs:829` already spells it | `../_decomposition.md` Testing brief AC-003/004/005 (static tier) and Notes ("a synthetic string *is* the fixture"); `standards/rust/81-checks-that-cannot-be-types.md:335` |
| **Not built here, and named so nobody builds it twice** | No directory-reading `atoms()` analogue, no vacuity `bail!`, no `Mode`, no `run`, no generated-region equality check, no page parser, no clause-id resolver. The generated region and the four directory-guard tests are `page-need-checker-mounted-in-the-gate`'s; the clause-id resolver is HS-P0020's `clause_ids` and a second one is forbidden | `../_storymap.md` slice rows; `../_decomposition.md` Architecture brief AC-010 and Note 6 |

## Data and migrations

**N/A — no schema, no store, no persisted state.** This story adds no crate, no table, no
serialised type and no wire format; nothing under `crates/` is touched, and `happenstance-core`'s
binding constraints (`#[async_trait]`, `serde` in default features, `EventStore::read`'s shape)
are untouched because no port appears in this diff.

The one thing in this story that *behaves* like a schema is the closed token set, so its
amendment procedure is stated here rather than discovered later. **Changing `NEEDS` is a
two-part commit and never a one-part one**: the `const` in `xtask/src/lint_pages.rs` and the
table in `standards/pages/10-the-need-set.md` move together, and once
`page-need-checker-mounted-in-the-gate` has landed the router's generated region, a one-part
commit is a gate failure with `cargo xtask lint-pages --write` printed in the message. Adding a
seventh member fails at `cargo check` on the ceiling assertion. Removing a member is the
strictly harder direction and is out of scope for a procedure note: every page declaring it
becomes unenumerated in the same run, which is the correct failure and is the reason the set was
closed at four rather than opened for convenience (`../_design.md`, DT-2 "Failure mode and
mitigation").
