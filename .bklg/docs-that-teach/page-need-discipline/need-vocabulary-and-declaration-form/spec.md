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

## Acceptance criteria

Fourteen criteria. Each is written from a reader's or an author's intent — the three drafted
personas (`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`), the *next page
author* and the *non-author reviewer* the UX brief names as S2's readers, and the *implementer of the
consuming story* who is this foundation's first real user. Each crosses the whole of this story's
stack: the `const`, the module docs, the two atoms, and the mount that makes them compile.

Every test path below is real once this story lands: `xtask/src/lint_pages.rs` is the file this story
creates, and its `#[cfg(test)] mod tests` is where every `::tests::…` name lives. Tests that read a
rule atom open **one named path** under `RULE_DIR`; none of them calls `read_dir`, because the
directory-reading `atoms()` analogue is `page-need-checker-mounted-in-the-gate`'s and building a
second one here is the duplication the whole slice exists to prevent.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the checker story and the router story will each need to know which needs exist, and the repository has already paid for a vocabulary spelled in two places (`xtask/src/spec_trace.rs:107-121`, *"three lists that must agree"*), **WHEN** either implementer looks for the enumeration, **THEN** they find exactly one: `const NEEDS: &[Need]` in `xtask/src/lint_pages.rs`, four members in the design's render order — `orientation`, `tutorial`, `how-to`, `explanation` — over `struct Need { token, job }` with both fields carrying doc comments and the struct carrying a doc comment naming everything that must agree with it (the membership test, the router's generated row, band 10's table); and no second enumeration of these tokens exists anywhere in `xtask/src/` | *Static:* `xtask/src/lint_pages.rs::tests::needs_holds_the_four_tokens_in_design_order` asserts the length, the order and each token verbatim. *Repo-state, ledger-recorded:* `rg -n '"how-to"' xtask/src` returns exactly one line, and it is the `NEEDS` definition |
| AC-002 | **GIVEN** the evaluator has twenty minutes and no second attempt, and this workspace already has two authoritative reference surfaces (rustdoc and `spec/SPECIFICATION.md`) that a third would quietly compete with, **WHEN** an author reaches for `reference` as a page's need, **THEN** the membership accessor returns `None` for it, and `standards/pages/10-the-need-set.md` states *why* it was subtracted — not that it is absent, but that a reference bucket in a narrative tree is either permanently empty or becomes a second specification — so the author reads a reason rather than guessing at an oversight | *Static:* `::tests::reference_is_not_a_member`; `::tests::band_ten_states_why_reference_was_subtracted` asserts `standards/pages/10-the-need-set.md` contains the subtraction rationale and the words `spec/SPECIFICATION.md` |
| AC-003 | **GIVEN** DT-2's named failure mode is bucket proliferation — content straining to be two things and the answer being "add a fifth token" instead of splitting the page — **WHEN** any future contributor adds a seventh member to `NEEDS`, **THEN** `cargo check -p xtask` fails on `const _: () = assert!(NEEDS.len() <= MAX_NEEDS, …)` before a single test runs, with a message that says splitting the page is the remedy; the ceiling is a compile-time fact nobody can delete by deleting a test | *Static (compile-time):* the `const _` assertion, exercised by `cargo check -p xtask`. *Procedural, ledger-recorded:* add a seventh member, capture the `cargo check -p xtask` failure verbatim, revert, re-run green, `git status` clean — all three captures in `_ledger.md` |
| AC-004 | **GIVEN** the checker story must judge a page's token and the router story must render it, and two parsers that agree today drift tomorrow, **WHEN** either needs to know whether a token is a member, **THEN** there is exactly one function to call — `fn need(token: &str) -> Option<&'static Need>`, exact, case-sensitive, no trimming, no aliasing, no plural form — and its behaviour on the near misses an author actually types is pinned by tests rather than by hope | *Static:* `::tests::the_accessor_accepts_each_of_the_four_tokens`; `::tests::the_accessor_is_exact_and_case_sensitive` rejects `reference`, `guide`, `Explanation`, `explanation ` (trailing space) and `how_to` |
| AC-005 | **GIVEN** `_design.md` sign-off condition 2 records that renaming this directory later is *not* a rename — it costs three `xtask/src/main.rs` edits, an `INERT` entry and two `affected` tests — **WHEN** the checker and router stories need the tree's address, **THEN** it is a `const` here and not a convention: `RULE_DIR = "standards/pages"` and `ROUTER = "standards/pages/README.md"`, in the shape of `ATOM_DIR`/`ROUTER` (`xtask/src/lint_constitution.rs:55-58`); `RULE_DIR` resolves to a real directory holding this story's two atoms, and `ROUTER` is **documented as deliberately absent** until `router-precedence-and-announcement` creates it, so its absence at this checkpoint reads as a scheduled obligation rather than a broken pin | *Static:* `::tests::rule_dir_holds_this_storys_two_atoms` opens `standards/pages/00-one-need.md` and `standards/pages/10-the-need-set.md` by name and asserts both are non-empty; `::tests::router_is_not_created_by_this_story` asserts `ROUTER` does **not** yet exist and carries the comment naming the story that creates it |
| AC-006 | **GIVEN** the application author's stated fear is *silent wrongness* — a model that looks right and is quietly wrong — and a check whose limits are undocumented is read as a guarantee (`xtask/src/lint_constitution.rs:11-13`), **WHEN** anyone opens `xtask/src/lint_pages.rs`, **THEN** the very first thing in the module docs is `# What this does not verify`, above any description of what it *does*, with item 1 unhedged — *it checks that a need is declared, never that the page answers it* — naming DR-07's reviewer procedure as the instrument for the rest, followed by the other five limits the architecture brief requires (the set is not judged; the fold rule binds only textual markers; clause ids are not resolved; an empty tree passes everything below the vacuity guard; length is not quality) | *Static:* `::tests::module_docs_open_with_what_this_does_not_verify` — `include_str!("lint_pages.rs")` asserts the header precedes every other `//!` heading and that all six limits are present, item 1 first (RS-81-1, `standards/rust/81-checks-that-cannot-be-types.md:11`) |
| AC-007 | **GIVEN** HS-P0020's hosting choice is still open and the declaration form rests on exactly one assumption about the medium, **WHEN** someone later picks a renderer, **THEN** the module docs already tell them what breaks: a named paragraph stating the form assumes only that the medium renders CommonMark blockquotes as visible body text in document order, assumes nothing about front matter or directory-derived navigation, and that a renderer which strips or relocates leading blockquotes, or requires front matter, **invalidates this decision and re-opens DR-05** — in those words, so a hosting change is visibly a re-opening rather than a silent contradiction | *Static:* `::tests::module_docs_carry_the_hosting_assumption` asserts the `include_str!`'d source contains "CommonMark blockquotes", "document order" and "re-opens DR-05". *Procedural:* the paragraph read against `../_design.md` "Hosting assumption…" and sign-off condition 1, cited in `_ledger.md` |
| AC-008 | **GIVEN** an `xtask/src/*.rs` file absent from the bin crate's module list is compiled by nothing, and the gate runs clippy with `-D warnings` over `--all-targets`, **WHEN** this story's checkpoint is taken, **THEN** `mod lint_pages;` sits in `xtask/src/main.rs:64-70` in alphabetical position, `cargo check -p xtask` and `cargo test -p xtask` both compile and run the new module, `cargo xtask ci --fast` is green, and the module carries **exactly one** module-level `#![allow(dead_code, reason = "…")]` whose reason names `page-need-checker-mounted-in-the-gate` as the story that deletes it — not `#![expect]`, not crate-level, not per-item, not a bare `allow` | *Gate-integration:* `cargo check -p xtask`, `cargo test -p xtask`, `cargo xtask ci --fast`, all captured in `_ledger.md`. *Repo-state:* `rg -n 'page-need-checker-mounted-in-the-gate' xtask/src` returns the reason string; `rg -c 'allow\(dead_code' xtask/src/lint_pages.rs` returns `1` |
| AC-009 | **GIVEN** UX-001's test is *read nothing but the region above the first prose paragraph and name the need*, and the adapter author's measured defect is an answer that existed three documents from where they were standing, **WHEN** a page author opens `standards/pages/00-one-need.md`, **THEN** it fixes the declaration as composed, in-place presentation and not as a bare capability: exactly one per page; the literal line `> **Answers:** \`token\` — <question>?`; immediately after the `# Title` with one blank line above and below; **nothing may be interposed** — no badge row, no table of contents, no admonition, no "last updated" line; ≤ 96 characters hard and ≤ 80 target; and when the question will not fit, the rule states that the page is answering more than one need and the overflow is the diagnosis | *Static:* `::tests::band_zero_fixes_the_declaration_grammar` asserts `standards/pages/00-one-need.md` contains the literal template line, the "immediately after the H1" placement clause, the interposition prohibition and the 96-character budget; `::tests::band_zero_states_the_overflow_diagnosis` asserts the yield-order sentence is present |
| AC-010 | **GIVEN** the declaration is never-fold class 1 and the mock found that rustdoc already wraps every page in an open `<details>` nobody has observed, **WHEN** a reviewer applies band 00, **THEN** it states that the declaration may never sit behind a fold, tab, inactive panel or `<details>` — persistent chrome in every state, in every medium — **and** it defers *what counts as a mechanism*, including a renderer-supplied open-by-default wrapper, to band 20 by number in `> **See also:**`, without settling DT-8 Part 3 in passing; and the atoms themselves contain no `<details>`, no tab strip and no accordion, so the tree does not violate on its own pages the rule it is writing | *Static:* `::tests::band_zero_forbids_occlusion_and_defers_the_mechanism_list` asserts the never-fold sentence is present, that `See also` names band `20`, and that no sentence in band 00 enumerates permitted mechanisms; `::tests::the_rules_tree_contains_no_disclosure_markup` asserts neither atom contains `<details`, `<summary` or a tab directive |
| AC-011 | **GIVEN** AC-003 of the project requires the resolution *and its rejected options* to be recorded because the perceptual review is a confirmed skip and this written record is the only record, **WHEN** a future contributor asks why the set is what it is, **THEN** `standards/pages/10-the-need-set.md` carries the four tokens as a table (token · the page's job · success for the reader, the third column prose-only and deliberately absent from the `const`), names the three losers and why each lost — literal Diátaxis (the evidence names this project's exact kind of subject as where the four-box split strains), drop-the-enumeration (a lint cannot check membership in an open set), persona-keyed (a need is a property of the page, a persona of the reader) — and states the amendment rule: changing the set is a **two-part commit**, the `const` and this atom together, never a one-part one | *Static:* `::tests::band_ten_names_every_needs_token_and_no_other` reads the real atom, asserts every `NEEDS` token appears and no other backticked need token does, failing with a message naming **which token moved** (RS-81-5, `standards/rust/81-checks-that-cannot-be-types.md:335`); `::tests::band_ten_names_the_three_rejected_options`; `::tests::band_ten_states_the_two_part_amendment_rule` |
| AC-012 | **GIVEN** DT-3's stated cost is a category the source taxonomy does not have, and the mitigation is that the category cannot grow into a sink, **WHEN** the evaluator's landing page is written, **THEN** band 10 carries both ceilings as rules a reviewer applies without the author: **RP-10-2** — an `orientation` page contains links and at most one sentence per destination saying what that destination answers, teaches nothing, and the moment it explains, instructs or references it is answering a second need and must be split; and **RP-10-3** — at most one `orientation` page per directory level | *Static:* `::tests::band_ten_carries_both_orientation_ceilings` asserts `## RP-10-2.` and `## RP-10-3.` headings exist, that RP-10-2's text contains the "teaches nothing" clause and the split remedy, and that RP-10-3 states the one-per-directory-level limit |
| AC-013 | **GIVEN** the next page author must be able to load one rule rather than the corpus (UX-012), and an unstyled, shapeless atom satisfies every content assertion above perfectly, **WHEN** either new atom is opened, **THEN** it is *composed* in the repository's own enforced atom grammar and not merely correct: `# NN — Title`, `> **Load when:** …`, `> **See also:** …` naming sibling bands **by number, not by link** (so no dangling link exists before those bands land), `---`, then each `## RP-NN-N. <imperative sentence>` followed by **Why.** · **Do** · **Not** · **Rejects.** · **Evidence.** in that fixed order — with ≤ 6 rules and ≤ 16,384 bytes per atom, prose ≤ 96 columns outside tables, and every `**Rejects.**` naming a wrong page that could actually ship in ≥ 120 characters | *Static:* `::tests::both_atoms_carry_the_atom_head_grammar`; `::tests::both_atoms_carry_the_five_sections_in_order` (the `SECTIONS` order at `xtask/src/lint_constitution.rs:67-81`); `::tests::both_atoms_are_inside_the_rule_and_byte_ceilings` (`MAX_RULES_PER_ATOM = 6` at `:88`, `MAX_ATOM_BYTES = 16_384` at `:95`); `::tests::prose_lines_stay_within_ninety_six_columns`; `::tests::every_rejects_section_names_a_wrong_page` |
| AC-014 | **GIVEN** this tree is deliberately **not** registered with the doctest harness (`xtask/src/lib.rs:28`, CR-0), so a `rust` fence here would be a Rust claim nothing in the workspace compiles, **WHEN** either atom shows an example, **THEN** every fence is tagged `text` or `markdown` — a `rust`-tagged fence and an untagged fence are both wrong, the second so that a future decision to register the tree cannot be undermined retroactively — and every `**Evidence.**` section orders its citations repository `path:line` first, then the discovery dossier, then external URLs with a `*(checked …)*` stamp | *Static:* `::tests::no_rust_tagged_and_no_untagged_fence_in_the_rules_tree` scans both atoms' fence openers; `::tests::evidence_sections_cite_the_repository_first` asserts the first citation in each `**Evidence.**` block is a repo-relative `path:line` |

**Coverage of the traced project ACs.** Project **AC-003** (DT-2 resolved and recorded, with the
options not chosen named) is discharged by AC-001, AC-002, AC-003 and AC-011 — the set, the
subtraction, the ceiling that stops it growing, and the written record of the losers. Project
**AC-004** (DT-3 resolved and the need set closed) is discharged by AC-001, AC-003, AC-004 and AC-012
— `orientation`'s membership, the closure enforced by the compiler, the single membership judge, and
the two ceilings that stop the new category becoming a sink. AC-005 through AC-010, AC-013 and AC-014
carry DR-05's form and the presentation obligations `_design.md` binds this story to; none of them is
a project AC on its own, and all of them are conditions the traced two are worthless without.

## Interaction quality

This story renders one of `_design.md`'s declared surfaces — **`rule-atom`**, two instances, in state
`populated` — and specifies `page-need-declaration` normatively without rendering it. The design is
binding here and this story does not re-decide it. Every invariant below is carried by an **AC row in
the table above**, never by a bullet in this section: `redkiln verify` extracts ACs from the table,
so an invariant stated only here would be ungated and untested.

The medium is text, so the two families map onto it exactly and the mapping is stated rather than
assumed (`../_design.md`, "Transience policy": persistent chrome = rendered in the document by
default, in every state).

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the reader learns what a page is for *on that page*; the module's limits are stated in the module | AC-009 (declaration is in the same document a human reads, immediately after the H1), AC-006 (limits first, in the module docs, not in a wiki) | `::tests::band_zero_fixes_the_declaration_grammar`, `::tests::module_docs_open_with_what_this_does_not_verify` |
| **Non-occlusion** — nothing load-bearing is reachable only by opening something | AC-010 (declaration never folded; atoms contain no disclosure markup at all) | `::tests::band_zero_forbids_occlusion_and_defers_the_mechanism_list`, `::tests::the_rules_tree_contains_no_disclosure_markup` |
| **Preserved focus / scroll / selection** — no state to preserve, because no mechanism is introduced that could lose it | AC-010 (the atoms introduce no `<details>`, tab or accordion while `PERMITTED_FOLD_MECHANISMS` is empty) | `::tests::the_rules_tree_contains_no_disclosure_markup`. Stated rather than left blank: a static markdown page has no focus state, and the design forbids adding one here |
| **Reversibility** — everything this story leaves behind can be undone by the story that consumes it, with the instruction attached | AC-008 (the single `allow(dead_code)` names the story that deletes it), AC-011 (the two-part amendment rule), AC-003 (the ceiling failure is reverted and green re-captured) | `rg -n 'page-need-checker-mounted-in-the-gate' xtask/src`; the AC-003 procedural capture ends in `git status` clean |
| **Keyboard reachability** — no affordance is added that a pointer is needed for | AC-013 (`> **See also:**` names bands **by number**, so nothing dangles and nothing requires a click before those bands exist), AC-009 (the declaration is body text a plain-text pager renders) | `::tests::both_atoms_carry_the_atom_head_grammar` asserts the `See also` line names bands numerically and contains no `](` link |

**Composition invariants** — taken from `../_design.md` and not re-decided here

| Invariant | Design source | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Presentation exists at all** — an atom is composed in the repository's enforced document grammar, not emitted as bare prose with headings | "S3 — a rule atom, top to bottom"; `standards/rust/README.md:99-107` | AC-013 | `::tests::both_atoms_carry_the_atom_head_grammar`, `::tests::both_atoms_carry_the_five_sections_in_order` |
| **Composition and placement** — the declaration's position *is* its meaning; nothing may sit between the H1 and it | "Composition — S1 — the governed page's head"; Anti-pattern 1 | AC-009 | `::tests::band_zero_fixes_the_declaration_grammar` |
| **Transience** — the declaration is persistent chrome in every state; `Why.`/`Do`/`Not`/`Rejects.`/`Evidence.` likewise; nothing is opened-on-demand | "Transience policy" rows S1, S3 | AC-010, AC-013 | `::tests::the_rules_tree_contains_no_disclosure_markup`, `::tests::both_atoms_carry_the_five_sections_in_order` |
| **Density budget, with its real numbers** — declaration ≤ 96 chars (≤ 80 target); ≤ 6 rules and ≤ 16,384 bytes per atom; prose ≤ 96 columns outside tables; `NEEDS` ≤ 6 | "Density budget" | AC-009 (96/80), AC-013 (6 / 16,384 / 96 columns), AC-003 (≤ 6 tokens) | `::tests::both_atoms_are_inside_the_rule_and_byte_ceilings`, `::tests::prose_lines_stay_within_ninety_six_columns`, the `const _` ceiling assertion |
| **Yield order** — when a budget is exceeded, the *question clause* shortens and the token never does; `Evidence.` yields before `Why.`; `Not` and `Rejects.` never yield | "What yields first when a budget is exceeded", S1 and S3 | AC-009 (the overflow diagnosis), AC-013 (`Rejects.` present and substantive) | `::tests::band_zero_states_the_overflow_diagnosis`, `::tests::every_rejects_section_names_a_wrong_page` |
| **Hierarchy** — the `## RP-NN-N.` imperative is primary, `Do`/`Not` secondary, `Why.`/`Rejects.`/`Evidence.` recessive; carried by heading level and bold run-in markers, never by colour | "Hierarchy", row S3 | AC-013 | `::tests::both_atoms_carry_the_five_sections_in_order` asserts the heading-then-marker structure |
| **Named anti-patterns this story can violate and must not** — 1 (something between H1 and declaration), 2 (a need as colour/icon alone), 4 (two need words in the declaration region), 5–6 (a never-fold class behind a disclosure; any `<details>` at all), 13 (a `rust`-tagged fence), 14 (an atom with no `Not` or `Rejects.`), 16 (a `reference` declaration) | "Anti-patterns" | AC-009 (1, 4), AC-010 (5, 6), AC-011 (2 — the token is a spelled word in a table, never a colour), AC-013 (14), AC-014 (13), AC-002 (16) | the tests named against each of those ACs above |

Anti-patterns 7–12 and 15 belong to surfaces this story does not render — the router, the terminal
output and the reviewer procedure — and are `router-precedence-and-announcement`'s,
`page-need-checker-mounted-in-the-gate`'s and `reviewer-and-citation-procedures`' respectively.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A seventh member is added to `NEEDS` | `cargo check -p xtask` fails on `const _: () = assert!(NEEDS.len() <= MAX_NEEDS, "…")`. The literal message names the remedy the design chose — split the page, do not add a token — because a bare `assertion failed` sends the reader to the source to learn the rule (AC-003) |
| **EC-002** | `NEEDS` and `standards/pages/10-the-need-set.md` disagree by one member | `::tests::band_ten_names_every_needs_token_and_no_other` fails, and the message names **which token moved** rather than reporting inequality (RS-81-5, `standards/rust/81-checks-that-cannot-be-types.md:335`). A test that asserted only "both are non-empty" is the named wrong implementation: it passes silently on exactly the half-landed commit this story exists to foreclose |
| **EC-003** | `#![expect(dead_code)]` is used instead of `#![allow(dead_code, reason = …)]` | The gate fails. `--all-targets` compiles `#[cfg(test)] mod tests`, which uses every item, so the lint does not fire, the expectation is unfulfilled, `unfulfilled_lint_expectations` warns and `-D warnings` (`xtask/src/main.rs`, the clippy step in `REQUIRED`) turns it into a failure. This is a *predicted* failure, written down so the implementer does not discover it and then "fix" it by widening the allow |
| **EC-004** | An atom exceeds 6 rules or 16,384 bytes, or a prose line exceeds 96 columns | `::tests::both_atoms_are_inside_the_rule_and_byte_ceilings` / `::tests::prose_lines_stay_within_ninety_six_columns` fail, naming the atom and the measured value. The remedy is to split the atom or rewrap, never to raise the ceiling: the numbers are `MAX_RULES_PER_ATOM` and `MAX_ATOM_BYTES` on purpose, and two trees teaching two numbers for the same idea is its own defect |
| **EC-005** | A fence in `standards/pages/` is tagged `rust`, or is untagged | `::tests::no_rust_tagged_and_no_untagged_fence_in_the_rules_tree` fails with the reason attached — nothing in the workspace compiles this tree's fences, so a `rust` tag is an unchecked Rust claim (`../_decomposition.md`, Architecture brief Note 4) |
| **EC-006** | `standards/pages/README.md` does not exist at this story's checkpoint | **Not an error.** `ROUTER` is a forward pin; `::tests::router_is_not_created_by_this_story` asserts the absence deliberately, so the first person to run the tests reads a scheduled obligation rather than a broken constant. When `router-precedence-and-announcement` lands, that test is inverted **by that story**, not deleted quietly |
| **EC-007** | `cargo xtask affected --base main` selects every workspace member on this diff | **Not an error, and not to be "fixed" here.** `standards/pages/` is unrecognised by `affected_packages`, so it widens (`xtask/src/affected.rs:222-223`, asserted by `an_unrecognised_path_widens_rather_than_narrows`). Broadening the `standards/rust/` arm to `standards/` would un-compile the constitution; the correct treatment is the `INERT` entry plus the unconditional-list entry plus two guard tests, in one change, and it is `page-need-checker-mounted-in-the-gate`'s (CR-4) |
| **EC-008** | A test in this module calls `read_dir` over `RULE_DIR` | Reject in review. The directory-reading `atoms()` analogue, its `.with_context()` guard and its vacuity `bail!` are the checker story's (`../_decomposition.md`, Testing brief AC-001); two of them is the duplication this slice's ordering exists to prevent. This story's tests open named paths only |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| **NF-001** | **No new dependency.** `xtask/Cargo.toml` gains nothing — `[dependencies]` stays `anyhow` alone | `git diff main -- xtask/Cargo.toml` is empty. The tests read named files with `std::fs`; no `tempfile`, no fixture crate (`../_decomposition.md`, Testing brief, AC-001's note) |
| **NF-002** | **No new gate step, and no change to the gate's step count.** This story adds no `Step` to `REQUIRED`, no dispatch arm, no `print_help` line and no `lint_steps` name | `cargo xtask ci --fast` runs the same steps as on `main`; `steps_named` is untouched. The four mounts are `page-need-checker-mounted-in-the-gate`'s, in one change (CR-2, CR-3) |
| **NF-003** | **The contract crates are untouched.** No `crates/` file appears in the diff, so `CLAUDE.md`'s three binding constraints (`#[async_trait]`, `serde` in `happenstance-core`'s default features, `EventStore::read`'s shape) cannot be affected, and the wasm32 steps are unchanged | `git diff --name-only main -- crates/` is empty |
| **NF-004** | **MSRV unaffected.** The new module uses no feature above the 1.97.1 floor; `const _: () = assert!(…)` in a const item has been stable far below it | the `msrv` CI job, unchanged; `cargo xtask ci` locally |
| **NF-005** | **Determinism.** Every test is hermetic: named-path reads and `include_str!`, no `read_dir`, no temp directories, no network, no clock, no ordering dependence between tests | `cargo test -p xtask` run twice yields identical results; captured once in `_ledger.md` |
| **NF-006** | **Load cost.** A reader who needs one rule loads one file. Both atoms sit inside 16,384 bytes and the tree has no router yet to read first — which is precisely why the router is the next story rather than an optional follow-up | `::tests::both_atoms_are_inside_the_rule_and_byte_ceilings`; the byte counts recorded in `_ledger.md` |
| **NF-007** | **Governance surface untouched.** `redkiln doctor` still reports **exactly six** `template-drift` advisories; `.redkiln/templates/` and `.kb/` are not written | `redkiln doctor`; `git diff main -- .kb` and `git diff main -- .redkiln/templates` both empty |

## Implementation notes (non-prescriptive)

**Read `../_design.md` before writing a word of prose.** This story transcribes an approved
decision; it does not make one. The three sections that bind hardest are "S1 vocabulary — DT-2",
"S1 vocabulary — DT-3" and "S1 — `page-need-declaration`", plus the Density budget and the
Anti-patterns list. Where this spec and `_design.md` appear to disagree, `_design.md` wins and this
spec is wrong — say so in the implementation report rather than editing the design.

**Suggested order, because two of these steps fail fast and cheaply.** Write
`xtask/src/lint_pages.rs` first with `NEEDS`, `MAX_NEEDS`, the `const _` assertion and the module
docs; add `mod lint_pages;` to `xtask/src/main.rs` immediately (a module that is not mounted is not
compiled, so every subsequent `cargo check` would be a lie); then write band 10, then band 00, then
the tests that read them. Writing the tests last is deliberate here and only here: the tests assert
properties of prose this story is simultaneously authoring, so writing them first produces
assertions shaped to whatever the author happened to type.

**Reading the module's own source is legitimate and is the only way AC-006 and AC-007 are
checkable.** `include_str!("lint_pages.rs")` inside the module's `#[cfg(test)] mod tests` resolves
relative to the file, needs no dependency and no path const, and turns "the docs say X, first" into
a compiled assertion. It has no in-repo precedent, so say what it is doing in a comment.

**The `Need` struct's third column is prose.** The design's table has three columns; the `const` has
two. A `success: &'static str` field nothing reads is dead code with a doc comment on it, and the
reason both existing fields survive is that each has a named consumer. Resist adding it.

**`#[cfg(test)] mod tests` wants its own allow.** `xtask/src/lint_constitution.rs:829` already spells
the house form: `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]` inside
the test module, not at the top of the file.

**Band 00 must stop short of DT-8 Part 3.** State that the declaration is never folded, tabbed or
collapsed; point at band 20 by number for the mechanism list; do **not** write a sentence about
whether rustdoc's `<details class="toggle top-doc" open>` counts as a mechanism. That sentence is
`fold-line-rule`'s, and answering it here settles an open question in passing — the move
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` names.

**`> **See also:**` names bands by number, not by link.** Bands 20, 30 and 40 do not exist yet.
A markdown link to a file that is not there is a dangling link the router's own check will flag the
moment `router-precedence-and-announcement` lands; naming `20` and `40` in prose is the
constitution's own spelling and costs nothing later.

**Copy the shapes, not the code.** `RULE_DIR`/`ROUTER` copy the *shape* of
`xtask/src/lint_constitution.rs:55-58`; `Need`/`NEEDS` copy the shape and the doc-comment register of
`xtask/src/spec_trace.rs:107-121`. Do not refactor `lint_constitution.rs` or `constitution.rs` to
share anything — RS-81-3 and Architecture brief Note 6 both say a shared abstraction over two trees
makes one error message answer two questions.

**What "done" looks like at the terminal.** `cargo check -p xtask`, `cargo test -p xtask`,
`cargo xtask ci --fast`, `git diff main -- standards/rust/README.md` empty, `git diff main -- .kb`
empty, `redkiln doctor` at exactly six advisories — and `cargo xtask affected --base main` green but
wide, which EC-007 says is correct here.

## Tests and CI (merge gate)

Tiers are the testing brief's four (`../_decomposition.md`, Testing brief, "Acceptance Criteria"):
**static** (a `#[cfg(test)]` unit test, house style at `xtask/src/lint_constitution.rs:828-878`),
**gate-integration** (a real `cargo xtask …` run, observed), **repo-state** (a one-line `git`
assertion whose output is pasted into the ledger), and **procedural (ledger-recorded)** (a human
runs a written step and records what happened). There is deliberately **no compile tier**: this
tree is not registered with the doctest harness (`xtask/src/lib.rs:28`, CR-0), so nothing here is
ever compiled as an example, which is exactly why AC-014 rejects a `rust` fence.

| tier | command / path | proves |
| --- | --- | --- |
| static | `cargo test -p xtask` → `xtask/src/lint_pages.rs::tests::needs_holds_the_four_tokens_in_design_order`, `::reference_is_not_a_member`, `::the_accessor_accepts_each_of_the_four_tokens`, `::the_accessor_is_exact_and_case_sensitive` | AC-001, AC-002, AC-004 — the vocabulary and its single membership judge |
| static (compile-time) | `cargo check -p xtask` — the `const _: () = assert!(NEEDS.len() <= MAX_NEEDS, …)` | AC-003 — the ceiling is a fact the compiler holds, not a test anyone can delete |
| static | `cargo test -p xtask` → `::rule_dir_holds_this_storys_two_atoms`, `::router_is_not_created_by_this_story` | AC-005 — the pins resolve, and `ROUTER`'s absence is a scheduled obligation rather than a break |
| static (source-reading) | `cargo test -p xtask` → `::module_docs_open_with_what_this_does_not_verify`, `::module_docs_carry_the_hosting_assumption` | AC-006, AC-007 — the limits are stated first and the hosting assumption travels with the decision |
| static (atom-reading) | `cargo test -p xtask` → `::band_zero_fixes_the_declaration_grammar`, `::band_zero_states_the_overflow_diagnosis`, `::band_zero_forbids_occlusion_and_defers_the_mechanism_list`, `::the_rules_tree_contains_no_disclosure_markup` | AC-009, AC-010 — the declaration's composition, placement and transience |
| static (atom-reading) | `cargo test -p xtask` → `::band_ten_names_every_needs_token_and_no_other`, `::band_ten_states_why_reference_was_subtracted`, `::band_ten_names_the_three_rejected_options`, `::band_ten_states_the_two_part_amendment_rule`, `::band_ten_carries_both_orientation_ceilings` | AC-002, AC-011, AC-012 — the record DT-2 and DT-3 are worthless without, and the drift check that names which token moved |
| static (atom-reading) | `cargo test -p xtask` → `::both_atoms_carry_the_atom_head_grammar`, `::both_atoms_carry_the_five_sections_in_order`, `::both_atoms_are_inside_the_rule_and_byte_ceilings`, `::prose_lines_stay_within_ninety_six_columns`, `::every_rejects_section_names_a_wrong_page`, `::no_rust_tagged_and_no_untagged_fence_in_the_rules_tree`, `::evidence_sections_cite_the_repository_first` | AC-013, AC-014 — composition, density and the fence divergence; the assertions an unstyled, shapeless atom fails |
| gate-integration | `cargo xtask ci --fast` | AC-008 — the module is mounted, compiles under `-D warnings` with exactly one scoped allow, and the project-scoped bar this non-terminal project is held to is green (`.redkiln/config.yaml`, `verify.integration_scoped`) |
| gate-integration | `cargo xtask affected --base main` | the story grain (`verify.affected_gate`). Expected **green and wide** on this diff — EC-007 |
| gate-integration | `cargo xtask lints` | the file-reading lint family still runs and `steps_named` still agrees with `REQUIRED` — proving this story added no half-mounted step name |
| repo-state | `git diff main -- standards/rust/README.md` | empty. The precedence chain is not edited and no tier is added (project AC-002's mechanical form, borrowed here as a negative guard) |
| repo-state | `git diff main -- .kb` · `git diff main -- .redkiln/templates` · `git diff main -- xtask/Cargo.toml` · `git diff --name-only main -- crates/` | all empty — NF-001, NF-003, NF-007, and the PR boundary's "explicitly not in this PR" list made mechanical |
| repo-state | `rg -c --multiline 'allow\(\s*dead_code' xtask/src/lint_pages.rs` → `1`; `rg -n 'page-need-checker-mounted-in-the-gate' xtask/src` → the reason string | AC-008 — one scoped allow, self-identifying, findable by the story that must delete it. The pattern is multiline on purpose: the attribute wraps, and the single-line spelling this row first carried matches nothing and returns `0` |
| procedural (ledger-recorded) | add a seventh `NEEDS` member → capture `cargo check -p xtask` failing → revert → re-run green → `git status` clean | AC-003 / EC-001 — the ceiling has been *seen* to fail, and the failure is reversible with no residue. Three captures verbatim, per `RUNBOOK.md:920-925`'s lesson that a document vouching for a check is not evidence the check runs |
| procedural (ledger-recorded) | the module-doc hosting paragraph read against `../_design.md` "Hosting assumption…" and sign-off condition 1 | AC-007 — the wording matches the approved design rather than paraphrasing it |
| procedural (ledger-recorded) | `redkiln doctor` | NF-007 — exactly six `template-drift` advisories, no more and no fewer |

**Merge gate of record.** `cargo xtask ci` is the repository's gate (`CLAUDE.md`, "Commands");
`cargo xtask ci --fast` is the interim bar this non-terminal project is held to during implementation
(`.redkiln/config.yaml`, `verify.integration_scoped`; `project.md`, "Definition of done"). Both are
run at this story's checkpoint; `--fast` is the one that blocks.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Treatment inside this PR |
| --- | --- | --- |
| **HS-P0020's hosting choice lands on a shape that strips or relocates leading blockquotes** — the charter's top risk and `_design.md` sign-off condition 1 | Medium / High | Not mitigable here, and deliberately not hidden: AC-007 puts the assumption and the words *re-opens DR-05* in the module docs, so a hosting decision that violates it is a visible re-opening rather than a silent contradiction. Both of HS-P0020's stated options satisfy the assumption today |
| **`cargo xtask affected --base main` widens to every workspace member** | Certain / Low | Accepted and documented (EC-007). The `INERT` entry and the unconditional-list entry are a pair and belong to `page-need-checker-mounted-in-the-gate`; splitting them is the half-mount the story map names. Broadening `standards/rust/`'s arm to `standards/` inside this PR is forbidden by the PR boundary |
| **The `allow(dead_code)` outlives its cause** | Medium / Medium | The reason string names `page-need-checker-mounted-in-the-gate`, so `rg` finds the obligation; that story's own spec is where the deletion is owed. `#![expect]` would self-delete but fails the gate under `--all-targets` (EC-003) — the trade is recorded rather than rediscovered |
| **`ROUTER` points at a file this story does not create** | Certain / Low | AC-005 asserts the absence on purpose. The next story in the slice inverts that test as part of creating the router; it is one line of coupling, in the same slice, implemented in the same context |
| **Band 00 or band 10 quietly settles an open question** — DT-8 Part 3's renderer-wrapper sentence, or the `standards/rust/README.md` precedence text | Medium / High | The PR boundary forbids both files; AC-010's test asserts band 00 enumerates no permitted mechanism; `git diff main -- standards/rust/README.md` empty is a merge-DoD line. Settling an approved-but-open question in passing is the exact failure `.kb/governance/rewrite-the-referent-never-the-reasoning.md` names |
| **The transcription drifts from the approved design** — a token reworded, a ceiling relaxed, a rejected option dropped | Medium / High | AC-002, AC-011 and AC-012 assert the *record* of the losers and the ceilings, not just the winners. `_design.md` is read-only for this story and the merge DoD includes no edit to it |
| **Two atoms are written and then the checker never reads them** | Low / Medium | Out of this story's hands by design, and answered by ordering: this is the only `foundation` story in the project and both consumers are named and scheduled inside the same initiative (`../_storymap.md`, "Merge order") |
| **Slice coupling** — this story's atoms are indexed by a router that does not exist yet, so the slice is not observable until `router-precedence-and-announcement` lands | Certain / Low | Intended. `discipline-on-disk` is implemented in one context and mounted as one surface; this story's own checkpoint is proved by `cargo check`/`cargo test`/`ci --fast`, not by a rendered tree |

## Dependencies

**Blocks on:** *nothing.* `depends_on: []` — this is the project's only `foundation` story and the
first row of `../_storymap.md`'s merge order. It needs no rule atom, no router and no checker to
exist, and it deliberately creates the two consts and the vocabulary before either consumer, so
neither consumer ever holds a second copy of the need set.

**Unlocks:**

| Story slug | What it takes from here |
| --- | --- |
| `router-precedence-and-announcement` | `RULE_DIR`, `ROUTER` and `Need::job` — the router's generated index rows are derived from `NEEDS`, and its band table indexes the two atoms this story creates. It also inverts `::router_is_not_created_by_this_story` |
| `fold-line-rule` | band 00's deferral. Band 20 is the file that answers *what counts as a fold mechanism*, including DT-8 Part 3's sentence on renderer-supplied wrappers, and band 00 points at it by number |
| `reviewer-and-citation-procedures` | the closed set and the one-need rule the verdict walk applies; bands 30 and 40 are written against band 00's grammar |
| `page-need-checker-mounted-in-the-gate` | the membership accessor (it calls `need()` rather than re-deriving the set), `RULE_DIR`/`ROUTER`, and the `#![allow(dead_code, …)]` line it deletes. It also owns the `INERT` entry, the unconditional-list entry and the two `affected` guard tests EC-007 defers |

Outside this project, `governed-page-cites-the-discipline` and `playbook-atom-staged-for-ingest`
depend on this story transitively; neither has a direct edge to it.

## Anchors (progressive disclosure)

Everything above is sufficient to start. Open these only at the moment named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/page-need-discipline/_design.md` | The signed-off design. It is the *referent* this story transcribes: the four tokens and their jobs, DT-2's and DT-3's rejected options, DR-05's exact declaration grammar, the Density budget's real numbers, the Transience policy and the sixteen anti-patterns. It must not be edited | Before writing the first line of either atom, and again before writing the module's hosting-assumption paragraph | AC-001, AC-002, AC-007, AC-009, AC-010, AC-011, AC-012, AC-013 |
| `xtask/src/spec_trace.rs` (lines 100–125) | The in-repo precedent for `NEEDS`: a `struct` + `const` pair carrying the doc comment *"the single place the six clause families are enumerated… three lists that must agree, kept as one so that adding a family cannot half-land."* Copy the register, not the content | While writing `struct Need` and `const NEEDS`, before deciding how many fields it carries | AC-001 |
| `xtask/src/lint_constitution.rs` (lines 9–28, 55–58, 67–81, 88, 95, 828–878) | Four separate obligations in one file: the limits-first module-doc shape (9–28), the `ATOM_DIR`/`ROUTER` const shape (55–58), the five-section `SECTIONS` order the atoms are authored against (67–81), the two ceilings the design reuses by value (88, 95), and the house test-module style including its `unwrap_used` allow (828–878) | Open 9–28 and 55–58 when creating the module; 67–81 and 88/95 when writing the atoms; 828–878 when writing the tests | AC-005, AC-006, AC-013 |
| `standards/rust/00-prime-directives.md` (lines 1–8) | The atom head grammar being copied verbatim in shape: `# NN — Title`, `> **Load when:** …`, `> **See also:** …`, `---`. It is also the pattern citation for the declaration form itself — the repository's only enforced line-level, human-visible, machine-read metadata convention | When writing each atom's first four lines | AC-009, AC-013 |
| `standards/rust/README.md` (lines 99–107) | "The shape of an atom" — the **Why.** · **Do** · **Not** · **Rejects.** · **Evidence.** contract and the `Evidence.` ordering rule, plus the repository's own demonstration that right and wrong are distinguished by the literal words `Do` and `Not` rather than by styling | When writing each `## RP-NN-N` rule body. **Read-only** — this file must not appear in the diff | AC-013, AC-014 |
| `standards/rust/81-checks-that-cannot-be-types.md` (lines 11, 335) | RS-81-1 (*prove the check's blind spot in its own tests, then state it in its own documentation*) is the rule behind the limits-first module docs; RS-81-5 (*make the failure say which one moved*) is the bar the `NEEDS`-versus-band-10 drift test must clear | Before writing the module docs, and before writing `::band_ten_names_every_needs_token_and_no_other`'s failure message | AC-006, AC-011 |
| `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` — Architecture brief Notes 1, 4, 7 and 9 | Note 1 (CR-0/CR-1) is why the mount is `main.rs` and not `lib.rs`; Note 4 is the fence-tag inversion in full, with the reasoning that makes the stricter rule correct; Note 7 enumerates the six required "what this does not verify" items; Note 9 is the list of decisions the design later pinned | Note 1 before mounting; Note 7 before the module docs; Note 4 before writing any fence | AC-006, AC-008, AC-014 |
| `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` — Testing brief | The tier definitions this spec's test table uses, the reason there is no compile tier here, and the rule that a synthetic string *is* the fixture — plus the explicit statement that the directory-guard tests belong to the checker story, which is what EC-008 enforces | Before writing the `#[cfg(test)] mod tests` block | AC-004, AC-013, EC-008 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The three reader personas in their only citable home — drafts, not `.kb/product/` atoms. The evaluator's *"nowhere for that question to go"*, the application author's fear of silent wrongness, and the adapter author's `E0034` explanation three documents away are the measured defects the acceptance criteria are written from | When writing band 10's "success for the reader" column and band 00's rationale, so the prose argues from the measured defect rather than from taste | AC-011, AC-012 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The evidence layer under DT-2, DT-3 and DT-8: the critique of literal Diátaxis for dense conceptual models, the eighth anti-pattern against forcing content into fixed buckets, tension 7 on routing, and the deletion test that DT-8 adopts. `**Evidence.**` sections cite this after repository `path:line` | While writing each atom's `**Evidence.**` section | AC-011, AC-012, AC-014 |
| `.bklg/docs-that-teach/page-need-discipline/design/mock.html` | The composed contact sheet the design was signed off against — 5 surfaces × 26 states. Its rule-atom frames are what "composed, not bare prose" looks like, and its closing panel carries the five findings, including the rustdoc `<details class="toggle top-doc" open>` finding that AC-010 defers rather than answers | Open when the atoms are drafted, to compare their shape against the frames a human approved | AC-010, AC-013 |
| `.bklg/docs-that-teach/page-need-discipline/project.md` | The project's AC-003 and AC-004 verbatim (this story's traced pair), DR-03/DR-04/DR-05, and the Definition of done including `verify.require_ledger` and the six-advisory assertion | When filling `_ledger.md` and when checking that the traced ACs are actually discharged rather than approximated | AC-011, AC-012, NF-007 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The governance atom that forbids the specific failure most available to this story: editing the signed-off `_design.md` so the transcription agrees with itself | The moment the implementer believes `_design.md` is wrong | AC-009, AC-010, AC-011 |
| `RUNBOOK.md` (lines 920–935) | The two recorded incidents that make procedural capture non-negotiable here: a gate step that looked wired and was not, and a document that vouched for a check nobody ran | Before writing the AC-003 procedural capture into `_ledger.md` | AC-003 |

## Clarifications resolved during spec

1. **The AC set is exactly the fourteen the front half decided** — AC-001 through AC-014, one per row
   of the Behavior and interfaces table, with that table's final two rows (*the module's own tests*
   and *not built here*) folded into the verification column and into EC-008 rather than becoming
   criteria of their own. A test-infrastructure row is not a user-intent criterion, and a list of
   things not built is a boundary, not an acceptance. Nothing was added and nothing was dropped.

2. **`ROUTER` is pinned here and its file is created by the next story, and that is asserted rather
   than tolerated.** The design's pin table lists both consts as this tree's addresses, but the
   router file itself belongs to `router-precedence-and-announcement`. Rather than leave a const
   pointing at nothing, AC-005 asserts the absence explicitly and names the story that inverts the
   test. This is new, and it is the smaller of two evils: the alternative — deferring `ROUTER` to the
   router story — would mean the two path consts are introduced in two commits, which is the split
   the design's sign-off condition 2 warns costs more than it looks like it does.

3. **The module's own source is read by its own tests.** AC-006 and AC-007 are obligations about
   *documentation*, and documentation is exactly what nothing in this repository currently checks.
   `include_str!("lint_pages.rs")` makes them compiled assertions with no dependency and no path
   const. There is no in-repo precedent, so it is called out here and must be explained in a comment
   at the call site.

4. **The `const _` ceiling assertion is given a procedural "seen to fail" capture.** The project's
   AC-007 (*the check has been seen to fail*) belongs to `declaration-check-seen-to-fail` and is
   about the gate step, not about this story. But a compile-time assertion nobody has watched fire is
   the same decorative shape one level down, and the capture costs one edit and one revert. It is
   recorded as AC-003's procedural evidence and it does not claim any part of the project's AC-007.

5. **The design's third table column stays out of the `const`.** *Success for the reader* is prose in
   band 10. Both `Need` fields have named consumers; a third with none is dead code with a doc
   comment on it. Recorded here because a reader comparing the design's table to the `const` will
   notice the difference and should find the reason rather than assume an omission.

6. **`> **See also:**` names sibling bands by number, not by link.** Bands 20, 30 and 40 do not exist
   at this checkpoint. The constitution's own atoms spell `See also` this way, so this is precedent
   rather than a workaround, and it means no dangling link is created that the router's link check
   would flag the moment it lands.

7. **No ADR is written for this story, and the absence is deliberate.** All seventeen atoms under
   `.kb/decisions/` were read by title at grounding and none governs documentation trees, gate
   structure or narrative conventions; the Architecture brief states that authoring one here would
   itself breach the initiative's non-goal against extending the precedence chain. The binding
   authority is sub-ADR and is cited inline throughout.

8. **The `_design.md` half of the story map's one-line slice is already done.** The story map says
   "resolve DT-2, DT-3 and DR-05 in `_design.md`"; sign-off happened on 2026-08-17 with four
   conditions carried. This spec's one-line slice therefore describes the remaining half — the
   transcription into a place that binds — and the Executive summary states the delta explicitly so
   the discrepancy reads as a resolved sequencing fact rather than as scope that went missing.
