---
item: HS-S0150
stage: spec
created: 2026-08-17T13:16:09.796Z
updated: 2026-08-17T13:16:09.796Z
template_sig: 87bbf1d0
rendered_sig: 4d24c637
---

# Spec — The page-need checker, mounted as an ordinary gate step

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — DoD-8; AC-07; BR-04 |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — ownership and the DAG |
| Project charter | [`.bklg/docs-that-teach/page-need-discipline/project.md`](../project.md) — AC-001, AC-003, AC-004, AC-006, AC-008; DR-06 |
| This spec | `.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — Architecture brief AC-001/AC-003/AC-006/AC-008 and **Notes 1 (CR-0…CR-4), 2, 3, 4, 5, 6, 7, 9**; UX brief UX-007, UX-008, UX-012, Note 1 (diagnostic primitives); Testing brief AC-001/AC-003-005/AC-006/AC-008 and its Notes |
| **Signed-off design (binding)** | [`../_design.md`](../_design.md) — `## Surfaces` (the pinned-constant table), `### S4 — lint-terminal-output`, `## Composition` S4, `## Density budget`, `## States`, `## Anti-patterns` 10-13, `## Sign-off` conditions 1-4 |
| Story map / merge order | [`../_storymap.md`](../_storymap.md) — slice `page-need-gate-step`, row 5 of 8 |
| Grounding | [`../_grounding.md`](../_grounding.md) — no Accepted decision atom constrains this work |
| Dependency spec (the const) | [`../need-vocabulary-and-declaration-form/spec.md`](../need-vocabulary-and-declaration-form/spec.md) — `xtask/src/lint_pages.rs`, `NEEDS`, `RULE_DIR`, `ROUTER`, the membership accessor |
| Dependency spec (the router) | [`../router-precedence-and-announcement/spec.md`](../router-precedence-and-announcement/spec.md) — the generated-region **forward contract** and the `[PROVISIONAL]` marker this story retires |

## One-line PR slice

Turn `xtask/src/lint_pages.rs` from a const-and-docs module into a working check — a
line-carrying declaration parser, the zero/two/unenumerated rejections, the `orientation`
ceiling, both trees' `.with_context()` **and** vacuity guards, the rules tree's own shape
checks, and the router's generated `## Index` region — and mount it in one change as an
ordinary gate step in `REQUIRED`, dispatch, `print_help`, `lint_steps` and `affected.rs`'s
unconditional list plus `INERT`.

## Executive summary

The four `discipline-on-disk` stories put the discipline on disk. **Nothing reads it.** This PR
is the machine, and it is the only story in the project that touches `xtask/src/main.rs` beyond
one module line.

**Delta against what already exists at this story's start.**

- `xtask/src/lint_pages.rs` exists and carries `Need`, `NEEDS` (four closed tokens), `MAX_NEEDS`
  with its `const _` ceiling assertion, `RULE_DIR = "standards/pages"`, `ROUTER =
  "standards/pages/README.md"`, a pure membership accessor, the limits-first module docs, and one
  module-scoped `#![allow(dead_code, reason = "…page-need-checker-mounted-in-the-gate…")]`
  ([`../need-vocabulary-and-declaration-form/spec.md`](../need-vocabulary-and-declaration-form/spec.md),
  `## Behavior and interfaces`). **This PR deletes that `allow`** — it names this story as the
  one that removes it, and leaving it is the defect.
- `standards/pages/` holds `README.md` (the router, with a hand-populated `<!-- BEGIN GENERATED
  -->` region) and the band `00`/`10`/`20`/`30`/`40` atoms. **Nothing checks any of it.**
- `docs/README.md:25-29` names the new tree with a `[PROVISIONAL — settles at
  page-need-checker-mounted-in-the-gate]` marker
  ([`../router-precedence-and-announcement/spec.md`](../router-precedence-and-announcement/spec.md),
  `## Data and migrations`). **This PR removes the marker in the commit that makes the sentence
  true**, and leaving it is a defect in *this* story.
- `xtask/src/affected.rs` has no `standards/pages/` entry, so a prose-only change to the rules
  tree widens the affected set to every workspace member. Correct, slow, and this PR's to fix —
  as the **pair**, never the `INERT` half alone.

What this PR does **not** do is observe the check failing. That is
`declaration-check-seen-to-fail`, the slice-mate, and the two stories merge in that order
([`../_storymap.md`](../_storymap.md), "Merge order"). This story's obligation is that the
failure, when it is staged, is *legible*: a path, a line, what is wrong, and what to do.

## Context pack

Read this section and you can start. Everything below it is signposted retrieval.

### The decisions this story is downstream of, stated as decisions

**The vocabulary, the paths and the names are already fixed, and re-deciding one is a rename
that is not a rename.** `RULE_DIR = standards/pages`, `ROUTER = standards/pages/README.md`,
the task name `lint-pages`, and the step name **`every page declares one need`**
([`../_design.md`](../_design.md), `## Surfaces`, the pinned-constant table; `## Sign-off`
condition 2). The step name is depended on **by value** in `lint_steps()`
(`xtask/src/main.rs:799-808`) and `steps_named` panics on a mismatch (`:816-826`) — which is the
intended failure, not a hazard to route around.

**There are two pinned trees, and both need both guards.** This checker reads its own rules tree
*and* the pages tree HS-P0020 pins. Each read is `?`-propagated with a `.with_context()` naming
the expected path (the shape at `xtask/src/lint_constitution.rs:212`), **and** each carries an
empty-tree `bail!` in the spelling of `lint_constitution.rs:176` — *"{ATOM_DIR} holds no atoms,
so every check below is vacuous"*. A checker that guards its own tree and trusts the pages tree
unconditionally is the plausible wrong implementation the architecture brief names directly
([`../_decomposition.md`](../_decomposition.md), Architecture brief AC-001; Testing brief AC-001,
"makes four tests, not two"). A green run over an emptied tree is the decorative-step failure
`RUNBOOK.md:920-925` already cost this repository once.

**One path constant for the pages tree, not two.** HS-P0020 pins it as
`xtask::narrative::TREE` (`.bklg/docs-that-teach/checked-documentation-surface/_design.md`,
`## Items` / `## Signatures`). This story makes that constant `pub(crate)` and **references it**;
declaring a second `PAGE_DIR` here is the *"three lists that must agree"* defect
`xtask/src/spec_trace.rs:122-160` records, and it is worse here because the second copy drifts
silently the day the tree moves ([`../_decomposition.md`](../_decomposition.md), Architecture
brief Note 2, obligation 1). This is **not** in tension with RS-81-3
(`standards/rust/81-checks-that-cannot-be-types.md:209`): that rule forbids one scanner ranging
over two trees, not two scanners agreeing on where one tree is.

**The declaration's grammar is settled and this story only *reads* it.** Immediately after the
page's `# Title`, one blank line either side, a single line
`> **Answers:** ` + `` `token` `` + ` — <the reader's question>?`. The token is one member of
`NEEDS`, in backticks; the clause after ` — ` ends in `?`. Nothing may be interposed between the
H1 and the declaration ([`../_design.md`](../_design.md), `### S1`, `## Composition` S1). The
precedent is the repository's own `> **Load when:** …` line
(`standards/rust/00-prime-directives.md:1-8`), parsed by `load_when`
(`xtask/src/lint_constitution.rs:247-257`).

**The output grammar is the gate's, not a new one.** One line per problem,
`{path}:{line} — {what is wrong}; {why it matters, or what to do}`; **every** problem printed,
sorted by path then line, then `bail!("{n} problem(s) in …")`; a green run states what it checked;
the repair goes **inside** the problem line and never in a footer
([`../_design.md`](../_design.md), `### S4`, `## Composition` S4, `## Transience policy` S4 rows).
The primitives are already written: report-all-then-bail at
`xtask/src/lint_constitution.rs:169-198`, the success line at `:192`, the vacuity `bail!` at
`:176`, the in-message repair at `:375-379`. A second diagnostic dialect in the same
`cargo xtask ci` output would be its own defect. **Never truncate** — truncation is how the third
problem is missed ([`../_design.md`](../_design.md), `## Density budget`, "Problem lines per run
— unbounded"; `## Anti-patterns` 10).

**Three divergences from `lint_constitution` are deliberate and must be stated in the module's own
docs, not left in a brief** ([`../_decomposition.md`](../_decomposition.md), Architecture brief
Notes 4, 5, 6):

1. **The fence rule inverts.** `check_fences` rejects an *untagged* fence because
   `standards/rust/` **is** registered in the doctest harness (`xtask/src/lib.rs:28`,
   bidirectionally checked by `check_harness`, `lint_constitution.rs:423-458`). This tree is
   deliberately **not** registered (CR-0), so a `rust`-tagged fence here is a Rust claim nothing
   in the workspace compiles. Reject `rust`-tagged **and** untagged; permit `text` and `markdown`.
   There is no `check_harness` equivalent for this tree, and its **absence** must be stated, or
   the next reader sees a checker that looks like `lint_constitution` with a check missing.
2. **The generated region carries more weight here.** For `standards/rust/` it is a convenience
   over a corpus the compiler also reads; here it is the *only* mechanism preventing the router's
   index from disagreeing with its corpus. Say so, because it changes how seriously a reviewer
   should treat a `--write` diff.
3. **No shared abstraction with `lint_constitution`.** `xtask/src/lint_constitution.rs` and
   `xtask/src/constitution.rs` are not refactored on this story's time (RS-81-3). Copying the
   shape is cheap; one error message answering two questions is not.

**The limits are stated first, and the headline limit is unhedged.** *This check proves a need is
**declared**; it never proves the page **answers** it.* Then: it does not judge whether the set is
the right set; the fold rule is enforced only to the extent the markers are textual; it does not
resolve clause ids (that is HS-P0020's `clause_ids`, a sibling of `spec_trace::all_rules` —
**do not build a second parser**); an empty tree passes every check below the vacuity guard; and
length is not quality (`lint_constitution.rs:9-28`; RS-81-1,
`standards/rust/81-checks-that-cannot-be-types.md:11`; [`../_decomposition.md`](../_decomposition.md),
Architecture brief Note 7's six required items).

### The one ambiguity this spec resolves, rather than leaving to discovery

`_design.md`'s `## States` block says the router's generated region disagreeing with `NEEDS` is a
problem "naming which member moved", repaired by `cargo xtask lint-pages --write`. But
`router-precedence-and-announcement` committed that region as **one row per rule atom** — a link,
the atom's first `Load when` line, and its rule ids — and made its shape a *forward contract*:
this checker's first `--write` against the committed router must produce **no diff**
([`../router-precedence-and-announcement/spec.md`](../router-precedence-and-announcement/spec.md),
"The region's shape is a forward contract on the checker story"). And band 10's token table
carries a third column (*success for the reader*) that `NEEDS` deliberately does not hold
([`../_design.md`](../_design.md), DT-2; the `Need` two-field decision in
[`../need-vocabulary-and-declaration-form/spec.md`](../need-vocabulary-and-declaration-form/spec.md)),
so it is not generatable.

**Resolution — two single-source obligations, one of them auto-repairable, both enforced here:**

- **The router `## Index` region** is generated from the rule atoms, equality-checked, and
  `--write` rewrites it. The `--write` repair string goes in the problem line.
- **`NEEDS` ↔ band 10's token table** is a containment-and-exclusion check over
  `standards/pages/10-the-need-set.md`: every `NEEDS` token appears as a backticked token in it,
  and no other backticked need-shaped token does. The failure names **which token moved** (RS-81-5,
  `81-checks-that-cannot-be-types.md:335`) and cites both paths. It is deliberately **not**
  auto-repairable, and the module docs say why in one sentence — the atom holds a prose column
  no `const` can generate. This promotes the unit test the foundation story prototyped into a
  gate problem; it does not re-decide the design.

This is a reading of two already-signed-off artifacts, not a new decision. Do not amend
`_design.md` ([`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)).

### The persona-journey slice this realizes

The reader personas are drafts in
[`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md),
never `.kb/product/` ([`../_decomposition.md`](../_decomposition.md), UX brief "Intent"). This
story serves a fourth reader the other seven do not: **the author who just broke the rule**
(`_storymap.md` backbone row A4). Their whole journey is one terminal run. They learn what is
wrong, *where*, and how to undo it, without opening the checker's source — which is why the
repair is in the line and the location is first (UX-007, UX-008). Downstream, it is the
application author's fear of *silent wrongness* one level up: a page that quietly answers two
needs reads as complete and is not, and until this PR nothing in the repository could see that.

### What this story must not do

Author narrative content; build a fold-checker (HS-P0020's DT-7 demonstration); build a second
clause-id parser; edit `standards/rust/**`, `xtask/src/lint_constitution.rs`,
`xtask/src/constitution.rs`, `xtask/src/lib.rs` (CR-0), `.kb/**`, `.redkiln/templates/**` or
`_design.md`; broaden `affected.rs`'s `standards/rust/` arm to `standards/`; or commit a
deliberately broken page (that is `declaration-check-seen-to-fail`'s reverted, one-off edit —
AC-008's wrong pages are `&str` literals that live forever).

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — the observable user is the author whose `cargo xtask ci` now fails by file and line |
| **Slice / milestone** | `page-need-gate-step` |
| **Slice-mate story slugs** | `declaration-check-seen-to-fail` (merges after this one; it needs the step in `REQUIRED` for `cargo xtask ci` to be the thing observed failing) |
| **Mount point** | **`xtask/src/main.rs`** — the `REQUIRED` gate array at `xtask/src/main.rs:105` gains one `Step` (`probe: None`; `args` copied line for line from the constitution step at `:462-479`, `--locked` included per RS-80-4). The same file carries the three by-name mounts that make it reachable: the dispatch arm at `:689-700` (with the `--write` arm and the unknown-flag branch), the `print_help()` line at `:718`, and the name in `lint_steps()` at `:799-808`. **Five mounts, four of them in this file, in one change** ([`../_decomposition.md`](../_decomposition.md), Architecture brief Note 1, CR-1-CR-3) |
| **Wires into** | `xtask/src/lint_pages.rs` — `NEEDS`, `RULE_DIR`, `ROUTER`, the membership accessor, `MAX_NEEDS` (this story's own module, extended) · `xtask::narrative::TREE` — HS-P0020's pinned pages tree, made `pub(crate)` and referenced, never re-declared (Architecture brief Note 2) · `xtask/src/lint_constitution.rs:169-198,208-246,247-257,334-421,477-508,601-643` — the shape copied, not shared (Note 6) · `xtask/src/spec_trace.rs:workspace_root` — the root resolver both existing checkers use · `xtask/src/affected.rs:118-125` (the unconditional file-reading list) and `:249-266` (`INERT`) |
| **Design-system primitives consumed** | The gate's diagnostic grammar in full ([`../_decomposition.md`](../_decomposition.md), UX brief Note 1): `{path}:{line} — {what}; {why or what to do}`, report-all-then-`bail!` with a count, the success line, the vacuity `bail!`, the in-message `--write` repair. The document grammar it *checks*: `# NN — Title`, `> **Load when:**`, `> **See also:**`, `## RP-NN-N.`, the five fixed sections, and the ceilings `MAX_RULES_PER_ATOM = 6` / `MAX_ATOM_BYTES = 16_384` (`lint_constitution.rs:88,95`) |
| **Renders surfaces** | **`lint-terminal-output`** ([`../_design.md`](../_design.md), `## Surfaces`) — this story creates it and is accountable for its `green`, `single-problem`, `many-problems`, `vacuous-tree`, `missing-tree` and `write-repair-offered` states. `discipline-router` — **changed, not created**: only the generated region between the markers, and only under `Mode::Write`. `page-need-declaration` and `rule-atom` are *read and judged* here, authored elsewhere; `reviewer-procedure` is untouched |
| **Public items** | **None.** `xtask` is `publish = false`; every item added is `pub(crate)` or private ([`../_design.md`](../_design.md), "Template sections that do not apply") |
| **Conformance rule(s)** | **None, and this is not adapter-observable.** No port, no value type, no testkit file is in this diff; `happenstance-testkit`'s suite observes stores and cannot see a markdown tree or a bin-crate module. The instruments that *do* observe this story are `cargo test -p xtask`, `cargo run --locked --quiet -p xtask -- lint-pages`, `cargo xtask lints`, `cargo xtask affected --base main` and `cargo xtask ci` |
| **Clause(s)** | **None discharged, none amended.** Nothing here writes, restates or renumbers a `spec/SPECIFICATION.md` clause; `cargo xtask spec-trace` remains the only writer of its generated sections ([`../_decomposition.md`](../_decomposition.md), Architecture brief Note 8), and must stay green at merge |
| **Advances DoD scenario** | Initiative **DoD-8** — *"Every page's answered need is stated and singular… the check being one a reviewer can actually perform rather than one that depends on the author's memory."* This story lands the half a machine can hold: singularity and membership become properties the gate asserts on every run, so the reviewer walk (band 40) inherits only the judgement no byte count reaches. Secondarily **DoD-14**'s *on disk and reachable* half, by making `docs/README.md:25-29` true rather than provisional |

**Delivered mounted, not as an isolated component.** The proof is behavioural and it is checkable
in one command each: `cargo xtask lints` runs the new step (it is in `lint_steps`, and
`steps_named` panics if the name is absent from `REQUIRED`); `cargo xtask ci` runs it as an
ordinary mandatory step; `cargo xtask affected --base main` runs it *unconditionally*, before any
package selection, because the packages it reads are not the packages the diff touched
(`xtask/src/affected.rs:118-125`). A checker present in `xtask/src/` and absent from any of those
is constructed-but-unmounted.

**The `affected.rs` pair is one change or it is a half-mount.** Adding `standards/pages/` to
`INERT` (`:249-266`) *without* adding the checker to the unconditional list (`:118-125`) makes a
prose-only pull request read **nothing** — strictly worse than today's correct-but-slow widening
([`../_storymap.md`](../_storymap.md), "Why the slices fall here"; Architecture brief Note 1,
CR-4). Note the same holds for the pages tree: `docs/` is already `INERT` (`affected.rs:250`), so
without the unconditional entry a pages-only change would run no page check at all.

## PR boundary

**In this PR**

- `xtask/src/lint_pages.rs` — the check itself: `Mode`, `pub(crate) fn run(mode: Mode)`, the two
  directory readers with `.with_context()` + vacuity `bail!`, the pure line-carrying declaration
  parser, the zero/two/unenumerated checks, the per-directory `orientation` count, the rules
  tree's shape checks (sections, ceilings, fences, links), the generated-region equality and
  `Mode::Write` arm, the `NEEDS` ↔ band-10 agreement check, the expanded limits-first module docs,
  and `#[cfg(test)] mod tests`. **The `#![allow(dead_code, reason = …)]` line is deleted here.**
- `xtask/src/main.rs` — the `Step` in `REQUIRED` (`:105`), the dispatch arm (`:689-700`), the
  `print_help()` line (`:718`), and the name in `lint_steps()` (`:799-808`). Four edits, one file.
- `xtask/src/affected.rs` — the checker on the unconditional list (`:118-125`), the
  `standards/pages/` prefix in `INERT` (`:249-266`), and two tests in its own `mod tests`
  (`:597+`) mirroring `the_relocated_trees_stay_inert` (`:662`) and
  `a_constitution_atom_selects_xtask` (`:688`).
- `xtask/src/narrative.rs` — **one token**: HS-P0020's page-tree `const TREE` becomes
  `pub(crate)`. Nothing else in that file is touched (Architecture brief Note 2, obligation 1).
- `docs/README.md` — the `[PROVISIONAL — settles at …]` marker at `:25-29` is removed, the
  sentence made unconditionally true, and the paragraph's closing discipline preserved in meaning
  ([`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)).
- `standards/pages/README.md` — **writable only under `Mode::Write`, and its committed diff must
  be empty.** A non-empty diff means the generator and the committed region disagree; record it as
  a finding rather than silently accepting the rewrite.
- `.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/**` — this
  spec, its `_ledger.md`, and this story's stage artifacts.

**Explicitly not in this PR**

- **Any deliberately broken page, and any observation of the gate failing.** That is
  `declaration-check-seen-to-fail`, and its edits are reverted; AC-008's wrong pages here are
  `&str` literals in `#[cfg(test)] mod tests`, never a committed file (Architecture brief AC-008;
  Testing brief AC-008, "explicitly **not** a committed broken file").
- **`standards/rust/**` — nothing.** `git diff main -- standards/rust/README.md` stays empty
  (project AC-002).
- **`xtask/src/lint_constitution.rs`, `xtask/src/constitution.rs`, `xtask/src/lib.rs`** — no
  refactor to share code (Note 6), and no registration of this tree with the doctest harness
  (CR-0). Broadening `affected.rs`'s `standards/rust/` arm (`:214-221`) to `standards/` is
  forbidden — it would silently un-compile the constitution.
- **Rule-atom or router *content*.** Bands 00-40 and the router's authored regions belong to the
  four `discipline-on-disk` stories. If a shape check fails against a committed atom, the finding
  is recorded and the fix is scoped to the smallest legal edit, not a rewrite.
- **A page-need index generated into the router** (deferred by `_design.md`), a fold-checker
  (HS-P0020's DT-7), a second `clause_ids` parser (HS-P0020's, Architecture brief AC-010), any
  page in HS-P0020's narrative tree, `.kb/**`, `.redkiln/templates/**`, `_design.md`, any ADR,
  any crate under `crates/`.

**The implementer may touch the composition-root and wiring files named in the Integration
contract** — `xtask/src/main.rs`, `xtask/src/affected.rs`, the one `pub(crate)` token in
`xtask/src/narrative.rs`, and `docs/README.md` — to mount this slice. That is the mount, not
scope drift.

```
xtask/src/lint_pages.rs
xtask/src/main.rs
xtask/src/affected.rs
xtask/src/lint_narrative.rs
standards/pages/README.md
docs/**
standards/rust/**
.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/**
```

> **Amended 2026-08-18, after implementation.** Three corrections, none of them
> a widening of what this story was allowed to build, and all three left visible
> rather than folded into the original list.
>
> 1. `xtask/src/narrative.rs` → **`xtask/src/lint_narrative.rs`**. The declared
>    path names no file in this repository. The module `ee0a500` actually reads
>    `TREE` from — the pin that stops this checker holding a second copy of
>    HS-P0020's page list — is `lint_narrative.rs`, and has been since that
>    project landed. A wrong name in the declaration, not a change of scope.
> 2. `docs/README.md` → **`docs/**`**. Mounting the step re-pointed the pinned
>    citations in `docs/append-conditions.md` and `docs/text-fences.md` as well
>    as the index row. Both are one-line repairs to link targets this story's own
>    pin is what keeps resolving; neither adds or edits any teaching.
> 3. **`standards/rust/**`** added, for the identical reason recorded at
>    `c52b031` for HS-P0020: inserting a `REQUIRED` step into
>    `xtask/src/main.rs` shifts every line number the constitution's **Evidence**
>    lines cite into that file, and `cargo xtask lint-constitution` fails until
>    they are re-pointed. `ee0a500` carries that repair across
>    `standards/rust/{51-features-and-no-std,52-wasm32-and-target-cfg,70-rustdoc-obligations,80-the-gate}.md`.
>    No claim, rule or example in any atom changed — only the line numbers its
>    Evidence lines point at. The 63 constitution doctests are green either way,
>    which is why this repair is mechanical rather than editorial.

**Merge DoD, one line.** `cargo xtask ci` green with the new step named in its output and
`cargo xtask lints` reaching it; `cargo test -p xtask` green including the four directory-guard
tests, the three wrong-page tests and the two `affected` tests; `cargo run --locked -p xtask --
lint-pages --write` producing **no diff**; `git diff main -- standards/rust/README.md` and
`git diff main -- .kb` both empty; `redkiln doctor` still reporting exactly six `template-drift`
advisories; and `_ledger.md` carrying a cited row per AC.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The entry point mirrors the precedent exactly** | `pub(crate) fn run(mode: Mode) -> Result<()>` and `enum Mode { Check, Write }` in `xtask/src/lint_pages.rs`, resolving the workspace root with the shared `workspace_root()` both existing checkers use. One `Vec<String> problems`; every check pushes onto it; nothing returns early on a problem | `xtask/src/lint_constitution.rs:169-198`; [`../_decomposition.md`](../_decomposition.md) Architecture brief Note 1 (CR-1) |
| **Both trees are guarded twice, and the pages tree is not re-pinned** | `fs::read_dir(root.join(RULE_DIR)).with_context(\|\| format!("reading {RULE_DIR}"))?` and the same against `narrative::TREE`; then, for each, `if <empty> { bail!("{DIR} holds no …, so every check below is vacuous") }`. `narrative::TREE` is HS-P0020's const made `pub(crate)`; **no second `PAGE_DIR` is declared** | `xtask/src/lint_constitution.rs:176,208-215`; Architecture brief AC-001 and Note 2; `.bklg/docs-that-teach/checked-documentation-surface/_design.md` `## Signatures` |
| **If HS-P0020's module is absent, halt loudly** | `xtask/src/narrative.rs` and its `TREE` const are the dependency this project's charter names. If they are not in the tree at implementation time, **stop and report** — do not declare a local `PAGE_DIR`, do not stub the pages half, do not `#[cfg]` it away. A silently half-scoped checker is the exact defect the single-constant rule exists to prevent | `project.md` "Dependencies — Depends on HS-P0020"; Architecture brief Note 2 obligation 1; `xtask/src/spec_trace.rs:122-160` |
| **The declaration parser is pure and carries a line number** | One function over one page's text returning each declaration it found with its 1-based line number — the shape `Rule` and `Fence` already carry. It recognises only the settled grammar: a blockquote line whose content starts `**Answers:**`, a backticked token, ` — `, and a question ending `?`. Prose elsewhere on the page that merely *mentions* a need word is not a declaration. The same function serves the counting check, the membership check and every test; there is no second parser | `xtask/src/lint_constitution.rs:143-163,247-257`; Architecture brief AC-006; Testing brief AC-006 |
| **Zero declarations** | One problem per page: the path (no line — there is no offending line), what is missing, and the band-00 atom to read. This is the design's `S1 · Empty` state verbatim in shape | [`../_design.md`](../_design.md) `## States`, S1 Empty; `## Composition` S4's third example line |
| **Two declarations** | One problem per page, at **the line of the offending (second) declaration**, naming both tokens and stating that a page answers one need. A page that strains to be two things is split, never granted a fifth token | [`../_design.md`](../_design.md) `## States` S1 Error, DT-2 "Failure mode and mitigation"; project `project.md` AC-006 |
| **An unenumerated token** | One problem at the declaration's line, naming the offending token **and** the enumerated set, via the membership accessor `NEEDS` already exposes. `reference` is rejected like any other non-member, and the message points at band 10 for why it was subtracted | `xtask/src/lint_pages.rs` (the accessor, from the foundation story); [`../_design.md`](../_design.md) DT-2, `## Anti-patterns` 16 |
| **At most one `orientation` page per directory level** | RP-10-3, and the design states it is checkable by the lint: count `orientation` declarations per directory under the pages tree; two or more in one directory is one problem naming that directory and every offending `path:line`. This is what stops routing pages breeding into the sink DT-3 chose option (a) to avoid | [`../_design.md`](../_design.md) "S1 vocabulary — DT-3" (RP-10-2/RP-10-3); `project.md` DR-04 |
| **Report all, sorted, counted; never truncate** | Problems sort by path then line and print as one block with no blank lines, no per-problem heading, no summary section and no "next steps" paragraph — the output is a diff-shaped worklist. Then `bail!("{n} problem(s) in {RULE_DIR} + {TREE}")`. Thirty problems print thirty lines | [`../_design.md`](../_design.md) `## Composition` S4, `## Density budget` (unbounded), `## Anti-patterns` 10; `xtask/src/lint_constitution.rs:190-198` |
| **A green run says what it checked** | `  {n} pages, {m} rules, all consistent` — a green run that prints nothing is indistinguishable from a step that did not run | [`../_design.md`](../_design.md) `## States` S4 Empty, `## Transience policy` (S4 success line), `## Anti-patterns` 11; `xtask/src/lint_constitution.rs:192`; `RUNBOOK.md:920-925` |
| **The problem line's yield order when it will not fit** | Target ≤ 100 characters. When it cannot fit, the **explanation half** moves into the rule atom and the message cites it by path; `path:line` and the repair pointer are never what is cut, and a wrap happens *after* `path:line — `, never inside it. The design's own mock recorded that this budget is not reachable for every message (finding 1, 112 characters against an inherited 111 at `lint_constitution.rs:369-378`) — so record the measured longest line in the ledger rather than shortening a citation to hit a number | [`../_design.md`](../_design.md) `## Density budget` (yield order S4), `## Mock` finding 1, `## States` "Long label" |
| **The router's `## Index` is generated and equality-checked** | Between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`: header row, separator row, one row per rule atom (`README.md` excluded), each row a link to `NN-slug.md`, the atom's first `Load when` source line, and its comma-separated `RP-NN-N` ids. `Mode::Check` reports inequality with `run \`cargo xtask lint-pages --write\`` **inside** the message; `Mode::Write` rewrites the region. The link half — every `.md` a router links must resolve — is copied as-is | `xtask/src/lint_constitution.rs:334-421` (markers `:392-395`, rows `:400-420`, repair `:375-379`, links `:343-356`); [`../router-precedence-and-announcement/spec.md`](../router-precedence-and-announcement/spec.md) forward contract |
| **The first `--write` produces no diff** | The committed router's region was authored to this generator's output shape — header spelling, separator spelling, column order, link form and `\|`-escaping. A diff on the first `--write` is a **finding**, recorded and reconciled toward the committed router, not a silent rewrite | [`../router-precedence-and-announcement/spec.md`](../router-precedence-and-announcement/spec.md) "The region's shape is a forward contract on the checker story" |
| **`NEEDS` cannot disagree with band 10 in silence** | Containment and exclusion over `standards/pages/10-the-need-set.md`: every `NEEDS` token appears as a backticked token; no other need-shaped backticked token appears. The failure names **which token moved** and cites both `xtask/src/lint_pages.rs` and the atom. Deliberately not `--write`-repairable — the atom carries a prose column no `const` holds — and the module docs say so in one sentence | RS-81-5, `standards/rust/81-checks-that-cannot-be-types.md:335`; Testing brief AC-003/004/005; [`../need-vocabulary-and-declaration-form/spec.md`](../need-vocabulary-and-declaration-form/spec.md) (`Need`'s two fields; the prototype test) |
| **The rules tree's own shape is checked** | A `SECTIONS` analogue — **Why.** · **Do** · **Not** · **Rejects.** · **Evidence.** in that fixed order after each `## RP-NN-N. <imperative sentence>`; `# NN — Title`; `> **Load when:**` (one source line) and `> **See also:**`; ≤ 6 rules per atom and ≤ 16,384 bytes, the byte count in the message; an atom with no `## RP-` rule is a problem. Structural markers only — no length heuristic is adopted without carrying its own "long and vacuous passes here" caveat | `xtask/src/lint_constitution.rs:67-81,82,88,95,477-508`; Architecture brief Note 9 ("Recommended: yes, and keep it to structural markers"), Note 7 item 6; [`../_design.md`](../_design.md) `## Composition` S3 |
| **Fences: `rust`-tagged and untagged are both rejected** | With the reason in the message — nothing compiles this tree, so a `rust` fence is a Rust claim nothing checks; untagged is rejected too so a future decision to register the tree cannot be undermined retroactively. `text` and `markdown` permitted. The **absence** of a `check_harness` equivalent is stated in the module docs | Architecture brief Note 4 (Divergence 1); `xtask/src/lib.rs:28`; `xtask/src/lint_constitution.rs:601-643`; [`../_design.md`](../_design.md) `## Anti-patterns` 13 |
| **Mounted in five places, in one change** | `REQUIRED` (`main.rs:105`): `name: "every page declares one need"`, `program: "cargo"`, `args: ["run","--locked","--quiet","-p","xtask","--","lint-pages"]`, `env: &[]`, **`probe: None`** — a file read with no external tool, so a probe would be a lie. Dispatch (`:689-700`) with the `--write` arm and the unknown-flag branch. `print_help()` (`:718`). `lint_steps()` (`:799-808`) — a load-bearing line: the project's DoD runs `cargo xtask lints` under `verify.reachability_static` | `xtask/src/main.rs:105,462-479,689-700,718,799-808,816-826`; RS-80-1/RS-80-2/RS-80-4, `standards/rust/80-the-gate.md:11,98,245`; Architecture brief Note 1 CR-2/CR-3 |
| **The story-grain selector learns about both trees** | The checker joins the unconditional file-reading list at `affected.rs:118-125` (*"the packages these read are not the packages the diff touched"*), **and** `"standards/pages/"` joins `INERT` (`:249-266`). Two guard tests: the new prefix selects no package, and `standards/rust/` still selects `xtask`. The `standards/rust/` arm at `:214-221` is not broadened | `xtask/src/affected.rs:118-125,214-221,232-266,645-696`; Architecture brief Note 1 CR-4; Testing brief AC-001 gate-integration |
| **The module's limits are stated first, and the divergences with them** | The six required items (declared-not-answered; not whether the set is right; the fold rule only where markers are textual; no clause-id resolution; an empty tree passes everything below the guard; length is not quality), plus the three divergences, plus the hosting-assumption paragraph the foundation story wrote — carried forward, not restated differently | `xtask/src/lint_constitution.rs:9-28`; RS-81-1, `standards/rust/81-checks-that-cannot-be-types.md:11`; Architecture brief Note 7; [`../_design.md`](../_design.md) `## Sign-off` condition 1 |
| **`docs/README.md` stops being provisional** | The `[PROVISIONAL — settles at page-need-checker-mounted-in-the-gate]` marker is removed and the gate-read paragraph reads as unconditionally true — three trees, this checker named among the commands. The closing sentence (moving a tree means editing `xtask/src/` in the same change) is preserved in meaning | `docs/README.md:25-29`; [`../router-precedence-and-announcement/spec.md`](../router-precedence-and-announcement/spec.md) `## Data and migrations`; [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md) |
| **The scaffold allow is deleted** | `#![allow(dead_code, reason = "…page-need-checker-mounted-in-the-gate…")]` names this story as the one that removes it. Every item it covered now has a consumer; the gate's `clippy … -D warnings` (`xtask/src/main.rs:116-127`) proves it | [`../need-vocabulary-and-declaration-form/spec.md`](../need-vocabulary-and-declaration-form/spec.md) "The scaffold is one line and it names the story that deletes it"; `CLAUDE.md` (a scoped allow naming the phase that removes it) |
| **Tests: what runs over strings, and what touches a filesystem** | The declaration parser, the count/membership checks, the `orientation` counter, the generated-region equality and the `NEEDS`↔band-10 check all run over synthetic `&str` — *a synthetic string is the fixture*. Only the four directory-guard tests touch a real filesystem, and only a **fabricated** root under `std::env::temp_dir()` built with `fs::create_dir_all`/`fs::write` — never the workspace's real trees. **No `tempfile` dependency is added**: `xtask/Cargo.toml` carries only `anyhow`. Test modules spell `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]` as `lint_constitution.rs:829` does | Testing brief AC-001 and "Fixtures and seams to mock"; `xtask/src/affected.rs:597+` (synthetic path lists); `xtask/src/lint_constitution.rs:828-829` |

## Data and migrations

**N/A — no schema, no store, no persisted state.** This story adds no crate, no table, no
serialised type and no wire format. Nothing under `crates/` is touched, so `CLAUDE.md`'s binding
constraints (no `#[async_trait]`; no `serde` in `happenstance-core`'s default features;
`EventStore::read`'s non-`async`, top-level-stream shape) are untouched by construction — no port
appears in this diff.

Three things in this story *behave* like migrations without being one, and each is stated here so
it is executed rather than discovered:

1. **The generated region is a format contract, not stored data.** Nothing migrates it. The
   obligation is one-directional and one-shot: this checker's first `cargo xtask lint-pages
   --write` against the router committed by `router-precedence-and-announcement` must produce no
   diff. If it does, the generator is reconciled toward the committed region — the router's shape
   was decided in that PR, and rewriting it here would move the referent rather than the
   reasoning.
2. **`[PROVISIONAL — settles at …]` is a dated claim with a named retirement, not a TODO.** It is
   removed in the same commit that makes `docs/README.md:25-29` true. Leaving it after this step
   merges is a defect in *this* story, inherited explicitly from the router story's spec.
3. **Widening `NEEDS` remains a two-part commit, and after this PR the gate says so.** The `const`
   and band 10's table move together; a one-part commit is now a reported problem naming which
   token moved, and a seventh member still fails at `cargo check` on the foundation story's
   `const _` ceiling assertion. Removing a member is the strictly harder direction: every page
   declaring it becomes unenumerated in the same run, which is the correct failure and is why the
   set was closed at four rather than opened for convenience.

## Acceptance criteria

Twelve criteria, each stated from the intent of a reader who crosses the whole stack — the author who
just broke the rule (`_storymap.md` backbone A4), the application author whose fear is *silent
wrongness*, the adapter author who cannot tell a limit of an instrument from a gap in their own
understanding
([`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md),
Persona 1 and Persona 2), and the evaluator whose measured gap is *good until the second question,
and then nowhere to go*. Every project AC this story traces — AC-001, AC-003, AC-004, AC-006,
AC-008 — is covered below.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **The gate actually runs it, by all three routes.** GIVEN a page author whose `cargo xtask ci` is the last thing between a half-shaped page and `main`, WHEN they run the full gate, the lint family alone, or the story-grain selector, THEN a step named `every page declares one need` runs in all three — as an ordinary `REQUIRED` entry with `probe: None` and `--locked` in its args, printed by `print_help()`, named in `lint_steps()`, and invoked *unconditionally* by `affected::run` before any package selection — so a checker that exists in `xtask/src/` but is unreachable is impossible to ship. *(project AC-001, AC-006)* | **gate-integration.** `cargo run --locked -p xtask -- lints` prints the step name (`steps_named`, `xtask/src/main.rs:816-826`, panics if `REQUIRED` lacks it); `cargo xtask ci` prints it among the mandatory steps; `cargo xtask affected --base main` prints it under `=== the file-reading checks ===` (`xtask/src/affected.rs:118-125`). All three captures in `_ledger.md`. |
| **AC-002** | **An emptied or moved tree fails the run; it never passes it.** GIVEN the application author whose real fear is a check that reports green over nothing, WHEN either pinned tree is absent, THEN the run fails with a `.with_context()` naming the expected path; and WHEN either exists but holds no files, THEN the run `bail!`s in the `lint_constitution.rs:176` spelling — *`{DIR}` holds no …, so every check below is vacuous* — and never prints `0 pages, all consistent`. Both guards, on **both** trees. *(project AC-001)* | **static.** Four `#[cfg(test)]` tests in `xtask/src/lint_pages.rs` over fabricated roots under `std::env::temp_dir()` (`fs::create_dir_all` / `fs::write`, no `tempfile` dependency): missing and empty, rules tree and pages tree. Run by `cargo test -p xtask`. |
| **AC-003** | **Moving the pages tree is one edit, not two.** GIVEN a maintainer who relocates HS-P0020's narrative tree six months from now, WHEN they change its `const`, THEN this checker follows without a second edit, because it reads `xtask::narrative::TREE` (made `pub(crate)` here) and declares no `PAGE_DIR` of its own — the *three lists that must agree* defect `xtask/src/spec_trace.rs:122-160` records, foreclosed rather than documented. *(project AC-001)* | **static + procedural.** `cargo test -p xtask` — a test asserting the checker's pages root resolves from `narrative::TREE`; plus a ledger-recorded `rg -n 'narrative::TREE' xtask/src/lint_pages.rs` (one reference) and a search for any second string literal naming the pages tree in `xtask/src/` (none). |
| **AC-004** | **A page that quietly answers two needs stops being invisible.** GIVEN a page author who wrote something that reads as complete and is not, WHEN the gate runs, THEN zero declarations is one problem naming the page and the band-00 atom to read; two declarations is one problem **at the line of the second one**, naming both tokens; and an unenumerated token — `reference` included — is one problem at the declaration's line naming the offending token *and* the enumerated set; and prose elsewhere on the page that merely mentions a need word is never counted as a declaration. *(project AC-006, AC-008; design anti-pattern 16)* | **static.** `cargo test -p xtask` — parser tests over synthetic `&str` literals: zero, one, two, one-plus-mentioning-prose (must not false-positive), and a malformed `> **Answers:**` line. Each asserts the returned 1-based line number, mirroring `rules_are_split_at_the_next_heading` (`xtask/src/lint_constitution.rs:871-877`). |
| **AC-005** | **The rule can reject something, and it stays able to.** GIVEN a reviewer asking whether this check is decorative, WHEN they run the test suite, THEN three named wrong pages that could plausibly ship — two declarations, none, one unenumerated — each produce a problem naming the correct line number *within the literal*, and none of the three is a committed file, so the rejection is permanent regression coverage rather than a one-off observation. *(project AC-008)* | **static.** Three `#[cfg(test)]` tests in `xtask/src/lint_pages.rs`, synthetic `&str` only (the `xtask/src/affected.rs:640-705` convention). Plus a ledger-recorded `git diff main --stat` showing no committed broken page. |
| **AC-006** | **Routing pages cannot breed.** GIVEN the evaluator whose gap is *nowhere to go after the second question*, WHEN a second `orientation` page appears at one directory level, THEN the run reports one problem naming that directory and every offending `path:line` — RP-10-3, the ceiling that keeps DT-3's new category from becoming the sink option (a) was chosen to avoid. *(project AC-004)* | **static.** `cargo test -p xtask` — a test over a synthetic set of (path, declaration, line) triples: one directory holding two `orientation` pages fails and the message names both paths; two directories holding one each pass. |
| **AC-007** | **The router's index cannot fall behind the corpus it indexes.** GIVEN the next page author who must load one rule rather than the tree, WHEN a rule atom lands without its router row, THEN `cargo xtask ci` fails with the disagreement and the repair instruction `cargo xtask lint-pages --write` **inside** the problem line; `--write` rewrites only the region between the markers; the **first** `--write` against the router committed by `router-precedence-and-announcement` produces **no diff**; and every `.md` the router links resolves. The complete index stays present alongside the filter — never behind a toggle, never a partial list. *(project AC-001, AC-003; design anti-patterns 7, 8)* | **static + gate-integration.** `cargo test -p xtask` — synthetic router-vs-atom-set disagreeing by exactly one atom (fails, names it); `Mode::Write` regenerates to equality; a synthetic router with one dangling link reports a problem. Ledger: `cargo run --locked -p xtask -- lint-pages --write && git diff --exit-code standards/pages/README.md`. |
| **AC-008** | **`NEEDS` and band 10 cannot disagree in silence.** GIVEN a maintainer widening or narrowing the need set, WHEN they change the `const` without `standards/pages/10-the-need-set.md` or the reverse, THEN the run reports a problem naming **which token moved**, citing both paths, and stating in the message that this one is not `--write`-repairable — the atom holds a prose column no `const` can generate. *(project AC-003, AC-004; RS-81-5, `standards/rust/81-checks-that-cannot-be-types.md:335`)* | **static.** Two `#[cfg(test)]` tests over synthetic const/table pairs: a `NEEDS` member missing from the atom (containment), and a need-shaped backticked token in the atom that is not in `NEEDS` (exclusion). Each asserts the offending token appears in the message. |
| **AC-009** | **A rule atom that has drifted is caught at its own line.** GIVEN a reader who must pay for every byte of what they load, WHEN an atom loses one of the five fixed sections, carries no `## RP-` rule at all, exceeds 6 rules or 16,384 bytes, or contains a `rust`-tagged or untagged fence, THEN each is one problem at `path:line`; the byte-ceiling message carries the measured byte count; and the fence message says *why* — nothing in this workspace compiles this tree, so a `rust` fence is a Rust claim no check backs. *(project AC-001; design anti-pattern 13; density budget rows for rules-per-atom, bytes-per-atom, router bytes, `Start here` rows, prose columns)* | **static.** `cargo test -p xtask` — one test per rejection over synthetic atom strings, mirroring `check_shape`'s assertions (`xtask/src/lint_constitution.rs:477-508`); the byte test asserts the number is in the message; the fence tests assert `text` and `markdown` pass while `rust` and untagged fail. |
| **AC-010** | **The terminal tells the author everything, in one run, in reading order.** GIVEN the author who just broke the rule and will read nothing but the terminal, WHEN a run finds three problems in three files, THEN the output is one block **sorted by path then line**, each line `  {path}:{line} — {what is wrong}; {why it matters, or what to do}` with the repair *inside* the line — no per-problem heading, no blank lines between problems, no summary section, no `next steps` paragraph, no box drawing, no spinner or per-file progress, nothing truncated and no `and others` — closed by `bail!("{n} problem(s) in …")`; and a zero-problem run prints `  {n} pages, {m} rules, all consistent` rather than nothing. *(project AC-006, AC-008; `_design.md` `## Composition` S4, `## Transience policy` S4 rows, `## Hierarchy` S4, `## Density budget` S4, anti-patterns 10, 11, 12)* | **static + procedural.** `cargo test -p xtask` — a test asserting the sort is by path then line and is stable regardless of `read_dir` order, and a test asserting the formatter emits one line per problem with `path:line — ` as its prefix. Ledger: the verbatim `green`, `single-problem` and `many-problems` captures from `cargo run --locked --quiet -p xtask -- lint-pages`, plus the **measured longest problem line** recorded against the ≤ 100-character budget and `_design.md` `## Mock` finding 1 (112 characters). |
| **AC-011** | **A prose-only pull request reads the prose, and nothing else.** GIVEN a contributor whose diff touches only `standards/pages/`, WHEN they run `cargo xtask affected --base main`, THEN the page-need checker runs unconditionally *and* `standards/pages/` selects no package — the pair landed in one change — while `standards/rust/` still selects `xtask`, so no lazily broadened `"standards/"` prefix can silently un-compile the constitution. *(project AC-001)* | **static + gate-integration.** Two tests in `xtask/src/affected.rs`'s own `mod tests`, mirroring `the_relocated_trees_stay_inert` (`:662-673`) and `a_constitution_atom_selects_xtask` (`:688-696`). Ledger: `cargo xtask affected --base main` on a rules-tree-only diff, showing the checker ran and no package was selected. |
| **AC-012** | **The instrument states its own blind spot before it states its result.** GIVEN the adapter author who must tell a limit of the tool from a gap in their understanding, WHEN they open `xtask/src/lint_pages.rs`, THEN its first section says — unhedged and first — that the check proves a need is **declared** and never that the page **answers** it, followed by the other five limits and the three divergences from `lint_constitution` including the stated **absence** of a `check_harness` equivalent; the scaffold `#![allow(dead_code, reason = …)]` naming this story is deleted; and `docs/README.md:25-29`'s `[PROVISIONAL — settles at page-need-checker-mounted-in-the-gate]` marker is removed in the same commit that makes the sentence true. *(project AC-001, AC-006; RS-81-1, `standards/rust/81-checks-that-cannot-be-types.md:11`; Architecture brief Note 7)* | **procedural + gate-integration.** Ledger: a field-by-field read of the module docs against Architecture brief Note 7's six items and Notes 4/5/6's three divergences, each cited by `xtask/src/lint_pages.rs:<line>`. `cargo xtask ci`'s clippy step (`-D warnings`, `xtask/src/main.rs:116-127`) green with the `allow` deleted proves every scaffolded item has a consumer. `rg -n 'PROVISIONAL' docs/README.md` returns nothing. |

## Interaction quality

RFC §6.7/D6. This story renders **one** surface — `lint-terminal-output` (S4) — and *changes* one
region of a second (`discipline-router`'s generated `## Index`, and only under `Mode::Write`). It
renders no authored page content: S1 and S3 are read and judged here, authored by the
`discipline-on-disk` slice. So the invariants below are the S4 rows of
[`../_design.md`](../_design.md) plus the two S2 rows this story is accountable for.

**Every invariant that applies is carried by an `AC-###` row in the table above.** This section says
which row carries which, and how it is verified. Nothing here is a free-standing bullet, because a
bullet in this section gets no ledger row and is therefore never gated.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump.** The fix happens where the author already is: `path:line` leads every problem, so nothing sends the reader to a report file, a log artifact or the checker's source (UX brief Note 2, invariant 1's corollary) | **AC-004**, **AC-010** | the parser tests assert the 1-based line; the formatter test asserts `path:line — ` is the line's prefix |
| **Non-occlusion — a filter must not hide what it filters.** At the terminal: report **all** problems, never the first, never truncated (`and others` is anti-pattern 10). On the router: the complete generated `## Index` stays present alongside the `Start here` filter, never behind a toggle and never a partial list (anti-patterns 7, 8) | **AC-010**, **AC-007** | the sort/format tests plus the `many-problems` ledger capture; the generated-region equality test plus the `--write` no-diff run |
| **Preserved focus, scroll and selection** — the terminal analogue. Output is append-only plain text: no spinner, no per-file progress, no cursor control sequences, and a **deterministic** sort so a re-run does not reshuffle the reader's place while they work through the list. `read_dir` order is unspecified and must not reach the output | **AC-010** | the stability test (same problems, shuffled input order, identical output); the `## Transience policy` row *S4 per-file progress — not rendered at all* |
| **Reversibility.** Every state the author enters, they can leave with an instruction they were given at the moment of failure: the `--write` repair sits inside the problem line, and running it leaves no generated file dirty beyond the region between the markers. Nothing provisional outlives the commit that makes it true | **AC-007**, **AC-012** | `--write` then `git diff --exit-code standards/pages/README.md`; `rg -n 'PROVISIONAL' docs/README.md` empty |
| **Keyboard reachability.** There is no pointer surface here at all, and that is a property to preserve rather than assume: output is plain text on stdout/stderr, readable in a pager, in a CI log and in `git diff`, with no colour carrying meaning (UX-002's floor, and the `## Density budget` note that the token is never abbreviated) | **AC-010** | the `green` and `single-problem` ledger captures, taken from a plain non-TTY invocation |

**Composition invariants** — from the signed-off [`../_design.md`](../_design.md), binding on this
story because it renders S4.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all.** The step emits composed output — the step's own name, a problem block, a count — never a bare non-zero exit, never a raw `Err` `Debug` dump, and never silence on success (anti-pattern 11: *a green run that printed nothing*) | **AC-010** | the `green` capture must show the step name and a count; `AC-001`'s `cargo xtask ci` capture shows the step named in the gate's own output |
| **Composition and placement.** `## Composition` S4's order, top to bottom: the step's name (the existing `Step` machinery prints it), then the problem block with no blank lines and no per-problem heading, then the `bail!` count. No summary section, no "next steps" paragraph, no box drawing | **AC-010** | the formatter test; the `many-problems` ledger capture read against `_design.md` `## Composition` S4 |
| **Transience.** Persistent chrome: *every* problem line on every run, the success line, and the repair instruction **inside** the problem line rather than in a footer (anti-pattern 12). Not rendered at all: per-file progress and any spinner — the one control this design deliberately removed | **AC-010**, **AC-007** | the `## Transience policy` S4 rows checked line by line against the three captures |
| **Density budget, with its real numbers.** Problem line: 1 terminal line, **≤ 100 characters** target, wrapping only *after* `path:line — ` if it must; problem lines per run **unbounded, never truncated**. The corpus ceilings the same run enforces: **6** rules per atom, **16,384** bytes per atom, **≤ 8,192** bytes for the router, **≤ 12** `Start here` rows, **≤ 96** prose columns outside tables | **AC-010**, **AC-009** | the measured longest problem line recorded in the ledger (the budget is knowingly breachable — `## Mock` finding 1 measured 112 characters against an inherited 111 at `lint_constitution.rs:369-378`, so the number is *recorded*, not met by shortening a citation); the ceiling tests assert the measured value appears in each message |
| **Hierarchy.** Left to right within one line: `path:line` primary, what-is-wrong secondary, why-it-matters/repair recessive — and *recessive means read third, never read never* (`## Hierarchy`, the closing inversion) | **AC-010** | the formatter test asserts the three-part order; the captures are read against `## Hierarchy` S4 |
| **Named anti-patterns.** 10 (truncation marker), 11 (green run printing nothing), 12 (no `path:line`, or a repair in a footer) → **AC-010**; 13 (`rust`-tagged fence anywhere in the rules tree) → **AC-009**; 7 and 8 (index behind a toggle; index listing only some rules) → **AC-007**; 16 (a `reference` declaration, or a fifth token) → **AC-004** and **AC-008** | as listed | each anti-pattern is the named wrong implementation of the test cited in its AC's row |

**Not applicable, stated rather than skipped.** `_design.md`'s S1, S3 and S5 composition rows govern
*authored* markdown this story does not write, and `## States`' `Narrow viewport` / `Long label`
rows for S1/S2/S3 likewise. The two that *do* reach this story are S4's `Long label` row (a long path
pushing the message past 100 characters → the explanation moves into the rule atom and the message
cites it by path) and S4's `Narrow viewport` row (80 columns → the wrap falls after `path:line —`),
and both are carried by **AC-010**.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The rules tree `standards/pages/` does not exist | `fs::read_dir` is `?`-propagated with `.with_context(\|\| format!("reading {RULE_DIR}"))`, in the shape of `xtask/src/lint_constitution.rs:212`. The run **fails**; it does not skip, warn or continue (`_design.md` `## States`, S4 Error) |
| **EC-002** | The rules tree exists and holds no rule atoms | `bail!` in the `lint_constitution.rs:176` spelling — *`standards/pages` holds no rule atoms, so every check below is vacuous*. Never `0 rules, all consistent` |
| **EC-003** | The pages tree (`narrative::TREE`) does not exist | The same `.with_context()` treatment, naming the pinned path. Guarding one tree and trusting the other is the plausible wrong implementation the Architecture brief's AC-001 names |
| **EC-004** | The pages tree exists and holds no pages | The same vacuity `bail!`. This is the *expected* outcome if the pages tree is still empty when this story merges — the Testing brief says so explicitly (AC-006, "must **fail** on AC-001's vacuity guard") and it is not a reason to soften the guard |
| **EC-005** | `xtask/src/narrative.rs` or its `TREE` const is absent at implementation time (HS-P0020 has not landed) | **Halt loudly and report.** Do not declare a local `PAGE_DIR`, do not stub the pages half, do not `#[cfg]` it away. A silently half-scoped checker is precisely what the single-constant rule exists to prevent (`project.md` "Depends on HS-P0020"; Architecture brief Note 2, obligation 1) |
| **EC-006** | A file inside a pinned tree cannot be read, or is not UTF-8 | `?`-propagate with a `.with_context()` naming **that file's** path, not the directory's. A file the checker cannot read is not a file with no problems |
| **EC-007** | An unknown flag is passed — `cargo xtask lint-pages --wrote` | The dispatch arm's unknown-flag branch, copied from `xtask/src/main.rs:692-696`: print `unknown flag for lint-pages: {flag}`, then `print_help()`, then `ExitCode::FAILURE`. Never fall through to `Mode::Check` |
| **EC-008** | The first `cargo xtask lint-pages --write` produces a **non-empty** diff against the committed router | Record it as a **finding** and reconcile the generator toward the committed region. The router's shape was decided in `router-precedence-and-announcement` and is a forward contract on this story; rewriting it here moves the referent rather than the reasoning ([`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)) |
| **EC-009** | A committed rule atom fails one of this story's new shape checks | Record the finding; scope the repair to the **smallest legal edit** to that atom. Rule-atom *content* belongs to the `discipline-on-disk` stories, and a rewrite here is scope drift wearing a green gate |
| **EC-010** | A page carries a `> **Answers:**` line that does not match the settled grammar — an unbackticked token, a missing ` — `, or a clause that does not end in `?` | One problem **at that line**, reported as a *malformed declaration* naming which part of the grammar failed — never silently counted as "no declaration". Silently degrading a near-miss to zero sends the author to the wrong repair (AC-004's malformed-line test) |
| **EC-011** | Two problems would sort identically (same path, same line) | The order is still deterministic: break the tie on the message text so a re-run never reshuffles the list under a reader working through it (AC-010's stability test) |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | **Cost proportional to a directory read.** No network, no external tool, no compilation — which is exactly why `probe: None` is correct and a probe would be a lie (RS-80-1/RS-80-2, `standards/rust/80-the-gate.md:11,98`). The step must stay in the register `lint_steps()`' own doc comment describes: *"finishes in the time it takes cargo to decide `xtask` is up to date"* (`xtask/src/main.rs:797-798`) | the `cargo xtask lints` capture in the ledger |
| **NF-002** | **No new dependency.** `xtask/Cargo.toml` carries only `anyhow` under `[dependencies]`; the four filesystem tests build fabricated roots with `std::env::temp_dir()` + `fs::create_dir_all` / `fs::write`. **No `tempfile`** (Testing brief, AC-001) | `git diff main -- xtask/Cargo.toml` shows no added dependency |
| **NF-003** | **Deterministic output.** `read_dir` order is unspecified by the standard library; the problem list is sorted by path then line before printing, and generated-region rows are ordered by atom filename, so `--write` is idempotent | AC-010's stability test; `--write` run twice produces one diff, then none |
| **NF-004** | **The gate's own bar.** `cargo fmt --check`, `cargo clippy --all-targets -D warnings`, and the MSRV floor of 1.97.1 (ADR-0029, `CLAUDE.md`). The only `#[allow]` in the new code is the test module's `clippy::unwrap_used` with a `reason`, in the spelling of `xtask/src/lint_constitution.rs:829`; the scaffold `dead_code` allow is **deleted**, not narrowed | `cargo xtask ci` green |
| **NF-005** | **Zero blast radius outside the host.** `xtask` is `publish = false` and is a dependency of nothing; this story touches nothing under `crates/`, so the four wasm32 steps, the `--no-default-features` doc build and the `cargo package --list` licence/README assertion are all unaffected by construction | `cargo xtask ci` green, and `git diff main --stat -- crates/` empty |
| **NF-006** | **Test isolation and parallel safety.** Each filesystem test builds a uniquely-named root under `std::env::temp_dir()`, removes it on completion, and never reads the workspace's real `standards/` or pages tree — so `cargo test -p xtask` stays order-independent and safe under the default parallel harness | `cargo test -p xtask` run twice, and once with `--test-threads=1`, both green |
| **NF-007** | **`spec-trace` stays green and unchanged.** Nothing here writes, restates or renumbers a `spec/SPECIFICATION.md` clause; `cargo xtask spec-trace` remains the only writer of its generated sections (Architecture brief Note 8) | `cargo xtask spec-trace` green; `git diff main -- spec/` empty |

## Implementation notes (non-prescriptive)

These are observations that shorten the path, not instructions. The spec above is the contract.

- **Read the precedent end to end once before writing anything.** `xtask/src/lint_constitution.rs`
  is ~880 lines and contains every shape this story needs: the `Mode` enum and `run` at `:169-198`,
  the directory reader at `:208-246`, `load_when` at `:247-257`, `check_router` with its markers,
  rows and `--write` repair at `:334-421`, `check_shape` at `:477-508`, `check_fences` at `:601-643`,
  and the test module's conventions at `:828-878`. Copying the shape is the decision already taken
  (Architecture brief Note 6); *sharing* it is forbidden.
- **A plausible order of work.** (1) the pure declaration parser and its tests — it needs no
  filesystem and no mount; (2) the two directory readers with both guards, and the four fabricated-root
  tests; (3) the counting, membership and `orientation` checks over the parser's output; (4) the rules
  tree's shape checks; (5) the generated `## Index` region and the `--write` arm, then the no-diff
  run against the committed router; (6) the `NEEDS` ↔ band-10 agreement check; (7) the five mounts
  and the two `affected` tests; (8) the module docs, the `allow` deletion and the `docs/README.md`
  marker. Steps 1-6 can all be green before anything is mounted; step 7 is what makes them real.
- **Keep the problem type ergonomic before it is stringly-typed.** A small internal `struct` carrying
  `(path, Option<line>, message)` sorts cleanly and formats in one place — which is what makes
  AC-010's format and stability tests possible at all. Formatting to `String` at the push site
  instead spreads the composition contract across a dozen call sites and makes the density budget
  unmeasurable.
- **The zero-declaration problem has no line, and that is not a special case to hide.** The design's
  own S4 mock shows a problem line with a path and no `:line`
  ([`../_design.md`](../_design.md) `## Composition` S4, third example line; and `## Mock` finding 4
  records that the precedent's own messages address a whole file in four of six cases). Make the line
  optional in the type rather than inventing a line 0.
- **`load_when` is a two-line parse and it is what makes the generated region possible**
  (`lint_constitution.rs:247-257`; Architecture brief Note 9). Adopting the `> **Load when:**`
  convention for rule atoms is nearly free and the router's rows depend on it.
- **The `--write` no-diff obligation is easiest to satisfy by generating first and comparing to the
  committed file by hand**, before wiring `Mode::Write` to overwrite anything. If they disagree, EC-008
  applies and the generator moves, not the router.
- **`lint_constitution` is deliberately absent from `affected.rs`'s unconditional list while the five
  lints and `spec_trace` are on it** (`xtask/src/affected.rs:118-125`). That is a real inconsistency in
  the tree; this story joins the *majority* precedent (Architecture brief Note 1, CR-4) and does not
  attempt to reconcile the minority one.
- **Where the two `affected` tests go.** `xtask/src/affected.rs`'s own `mod tests` already drives
  `affected_packages` over synthetic path lists (`:640-705`); the two new tests are neighbours of
  `the_relocated_trees_stay_inert` (`:662`) and `a_constitution_atom_selects_xtask` (`:688`), not a
  new module.

## Tests and CI (merge gate)

Tiers as the Testing brief defines them — **static** (a `#[cfg(test)]` unit test),
**gate-integration** (`ci` / `affected` / `lints`, run and observed), **procedural**
(a human runs a written step and the ledger records what happened; `.redkiln/config.yaml`'s
`require_ledger: true` makes that first-class proof). There is deliberately **no compile tier**: this
tree is not registered with the doctest harness (CR-0), so nothing in it is ever compiled.

| tier | command / path | proves |
| --- | --- | --- |
| static | `cargo test -p xtask` → `xtask/src/lint_pages.rs` `#[cfg(test)] mod tests` | AC-002 (four fabricated-root guard tests), AC-003, AC-004 (five parser tests), AC-005 (three named wrong pages), AC-006 (`orientation` ceiling), AC-007 (region equality, `Mode::Write`, dangling link), AC-008 (containment + exclusion), AC-009 (sections, ceilings with the byte count, fences), AC-010 (sort stability, line format) |
| static | `cargo test -p xtask` → `xtask/src/affected.rs` `mod tests` | AC-011 — the new prefix selects no package; `standards/rust/` still selects `xtask` |
| gate-integration | `cargo run --locked --quiet -p xtask -- lint-pages` | the checker alone, while iterating; the source of AC-010's `green` and `single-problem` captures |
| gate-integration | `cargo run --locked -p xtask -- lint-pages --write` then `git diff --exit-code standards/pages/README.md` | AC-007's forward contract — the first `--write` produces no diff (EC-008 otherwise) |
| gate-integration | `cargo xtask lints` | AC-001 — the step is in `lint_steps()` and therefore in `REQUIRED` (`steps_named` panics otherwise); this is `verify.reachability_static` in the project's DoD |
| gate-integration | `cargo xtask affected --base main` | AC-001 and AC-011 — the checker runs unconditionally, and a rules-tree-only diff selects no package; `verify.affected_gate` |
| gate-integration | `cargo xtask ci --fast` | the interim bar this non-terminal project is held to during implementation (`verify.integration_scoped`, `project.md` DoD) |
| gate-integration | `cargo xtask ci` | the merge gate of record — fmt, clippy `-D warnings` (AC-012's proof that the `dead_code` allow is gone), tests, wasm32, docs, `spec-trace` (NF-007), package-check |
| gate-state | `git diff main -- standards/rust/README.md` · `git diff main -- .kb` · `git diff main --stat -- crates/` · `git diff main -- xtask/Cargo.toml` | the PR boundary as a repo-state check: project AC-002 untouched, nothing hand-authored into `.kb/`, no crate touched (NF-005), no dependency added (NF-002) |
| gate-state | `rg -n 'PROVISIONAL' docs/README.md` | AC-012 — the marker is retired in the commit that makes the sentence true |
| procedural | the module docs read field by field against Architecture brief Note 7's six items and Notes 4/5/6's three divergences, each cited by `xtask/src/lint_pages.rs:<line>` in `_ledger.md` | AC-012 — the headline limit is stated first and unhedged; no `#[test]` can assert this |
| procedural | the `green` / `single-problem` / `many-problems` captures pasted verbatim into `_ledger.md`, read against `_design.md` `## Composition` S4, `## Transience policy`, `## Hierarchy` and `## Density budget`; the **measured longest problem line** recorded | AC-010's composition invariants — the design review is a confirmed skip (`design.capture` absent), so this reading is the *only* perceptual instrument this project has |
| repo hygiene | `redkiln doctor` · `redkiln validate --kb` | exactly six `template-drift` advisories, no more and no fewer; nothing landed in `.kb/` (`project.md` DoD) |

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment inside this PR |
| --- | --- | --- |
| **HS-P0020's `narrative::TREE` does not exist yet.** This project depends on it and the dependency is real, not notional | The checker's pages half cannot be written without it, and the tempting workaround — a local `PAGE_DIR` — is the exact defect the single-constant rule forecloses | EC-005: halt and report. Do not stub, do not `#[cfg]`, do not duplicate. This is a project-level dependency question, not an implementation choice |
| **The first `--write` disagrees with the committed router** | Two stories wrote to one region's shape, one of them before the generator existed; a silent rewrite would erase a decision taken in another PR | EC-008: reconcile the generator toward the committed region and record the finding. `git diff --exit-code` on the router is a merge-gate line, not an afterthought |
| **Broadening `affected.rs`'s `standards/rust/` arm to `standards/`** is one keystroke and looks like a simplification | It would silently un-compile the constitution — the failure `an_unrecognised_path_widens_rather_than_narrows` (`:645-648`) exists to catch one level up | AC-011's second test asserts `standards/rust/` still selects `xtask`. The arm at `:214-221` is out of scope in the PR boundary |
| **Half-mounting: `INERT` without the unconditional-list entry** | A prose-only pull request would then read **nothing** — strictly worse than today's correct-but-slow widening. The same trap applies to the pages tree, since `docs/` is already `INERT` (`affected.rs:250`) | The pair is one AC (**AC-011**) with two tests, not two ACs |
| **A shape check firing against a committed rule atom** | The rules tree was authored by three other stories before any checker read it; a failure here is a *finding about their content* landing inside this PR | EC-009: smallest legal edit, recorded. Content rewriting is explicitly out of the PR boundary |
| **Copying `check_fences` unexamined** | Its reason (*"an untagged fence is compiled as Rust"*) is true only because `standards/rust/` **is** registered in the doctest harness. Copied here it is false, and the correct rule is *stricter*, not looser | AC-009's fence tests assert `rust` **and** untagged both fail while `text` and `markdown` pass; AC-012 requires the module docs to state the absence of a `check_harness` equivalent |
| **The 100-character budget is knowingly unreachable for some messages** | `_design.md` `## Mock` finding 1 measured 112 characters against an inherited 111. An implementer who shortens a citation to hit the number damages the thing the line exists for | AC-010 requires the measured longest line to be **recorded**, with the density budget's yield order applied — the location and the repair pointer are never what is cut |
| **The step name is depended on by value in three places** | `lint_steps()` holds a literal, `steps_named` panics on a mismatch, and `_design.md`'s `## Sign-off` condition 2 pins it. A "small improvement" to the wording breaks the build | The name `every page declares one need` is fixed by the signed-off design and restated in the Integration contract. Changing it is a design amendment, not an edit |
| **The slice-mate is blocked until this merges** | `declaration-check-seen-to-fail` needs the step in `REQUIRED` for `cargo xtask ci` to be the thing observed failing, and it is the project's only AC-007 owner | Merge order is fixed in [`../_storymap.md`](../_storymap.md) "Merge order" 2.1 → 2.2; this story does not attempt the observation itself |

## Dependencies

**Blocks on** (must merge first; `depends_on`, and this matches
[`../_storymap.md`](../_storymap.md)'s merge order exactly):

| story slug | what this story needs from it |
| --- | --- |
| `need-vocabulary-and-declaration-form` | `xtask/src/lint_pages.rs` itself — `Need`, `NEEDS`, `MAX_NEEDS` and its `const _` ceiling assertion, `RULE_DIR`, `ROUTER`, the membership accessor, the limits-first module docs, and the one `#![allow(dead_code, reason = …)]` this story deletes. Also `standards/pages/10-the-need-set.md`, the atom AC-008 checks `NEEDS` against |
| `router-precedence-and-announcement` | `standards/pages/README.md` with its hand-populated `<!-- BEGIN GENERATED -->` region — the **forward contract** AC-007's first `--write` must reproduce byte for byte — and `docs/README.md:25-29`'s `[PROVISIONAL — settles at page-need-checker-mounted-in-the-gate]` marker, which this story retires |
| `fold-line-rule` | a non-empty rules tree with bands `00`/`10`/`20`/`30`/`40` present, so AC-002's vacuity guard fails correctly on nothing and AC-009's shape checks have real atoms to run against; and `standards/pages/20-the-fold-line.md`, the atom the design's own S4 example cites |

**Unlocks:**

| story slug | what it takes from this one |
| --- | --- |
| `declaration-check-seen-to-fail` | the step in `REQUIRED`, so `cargo xtask ci` is the thing observed failing by file and line, and the reversibility half is observable (slice-mate; merges immediately after) |
| `governed-page-cites-the-discipline` | a checker that already **pins** the link target, so moving the tree breaks the build rather than the link ([`../_storymap.md`](../_storymap.md), "Merge order" 3.1) |
| `playbook-atom-staged-for-ingest` | transitively, via `declaration-check-seen-to-fail` — the atom records the conditions under which the discipline stops holding, and those are only true once the discipline has stopped moving |

**Cross-project:** HS-P0020 `checked-documentation-surface` must have landed
`xtask/src/narrative.rs` and its `TREE` const. This is a hard dependency (EC-005), not a soft one.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Every path below exists in the tree today
(`test -f`). Open each at the moment named — not before, and not never.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/lint_constitution.rs` | the entire precedent this story copies without sharing: `run` + report-all-then-`bail!` (`:169-198`), the vacuity `bail!` (`:176`), the success line (`:192`), the directory reader and its `.with_context()` (`:208-246`), `load_when` (`:247-257`), `check_router` with markers, rows and the in-message repair (`:334-421`), `check_shape` and the ceilings (`:477-508`), `check_fences` (`:601-643`), and the test module's conventions (`:828-878`) | before writing the first line of the check — read it end to end **once**, then return to the named ranges per check | AC-002, AC-007, AC-009, AC-010 |
| `xtask/src/main.rs` | the mount points, all four in one file: `REQUIRED` (`:105`), the `Step` contract and the normative `probe` doc comment (`:72-103`), the constitution step's args to copy line for line (`:462-479`), the dispatch arm with its `--write` and unknown-flag branches (`:689-700`), `print_help()` (`:718`), `lint_steps()` (`:799-808`) and the `steps_named` panic that enforces the name (`:816-826`) | when mounting — step 7 of the implementation order, after the check is green in isolation | AC-001, EC-007 |
| `xtask/src/affected.rs` | the story-grain selector: the unconditional file-reading list and its reason (`:118-125`), the prose arm that must **not** be broadened (`:214-221`), `is_inert`'s `INERT` with its "deliberately a list" doc (`:232-266`), and the two tests to mirror (`:662-673`, `:688-696`) | in the same change as the mount — the pair is one AC, never two commits | AC-011 |
| `.bklg/docs-that-teach/page-need-discipline/_design.md` | **binding.** `## Surfaces` (the pinned-constant table and the step name), `### S4` (report-all with the repair in the message), `## Composition` S4, `## Transience policy` S4 rows, `## Hierarchy` S4, `## Density budget` with its yield order, `## States` S4, `## Anti-patterns` 7-8 and 10-13, `## Mock` finding 1, `## Sign-off` conditions 1-4 | before writing any output-formatting code, and again when recording AC-010's captures | AC-010, AC-009, AC-007, AC-004 |
| `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` | the three briefs at full depth: Architecture Note 1 (CR-0…CR-4, the five composition roots), Note 2 (the HS-P0020 seam and the single-constant obligation), Note 3 (the data-flow diagram), Notes 4/5/6 (the three divergences to state in the module docs), Note 7 (the six required limits), Note 9 (the shape-check recommendation); UX Note 1 (the diagnostic primitive table) and Note 2 (the five interaction invariants); the whole Testing brief AC list, which is this story's test specification | Notes 1-2 before mounting; Notes 4-7 before writing the module docs; the Testing brief before writing any test | AC-001, AC-002, AC-003, AC-005, AC-009, AC-012 |
| `.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/spec.md` | the exact state of `xtask/src/lint_pages.rs` at this story's start — `Need`'s two fields and *why* it has two, `NEEDS`, `MAX_NEEDS` with its `const _` ceiling, the membership accessor's signature, and the one-line scaffold `allow` that names this story as the one that deletes it | first, before touching the module — it tells you what is already there | AC-004, AC-008, AC-012 |
| `.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/spec.md` | the generated region's **forward contract** — header spelling, separator, column order, link form and `\|`-escaping — and the `[PROVISIONAL]` marker this story retires, with the reason it was written as a dated claim | before implementing the generated `## Index` region, and again before removing the `docs/README.md` marker | AC-007, AC-012 |
| `.bklg/docs-that-teach/page-need-discipline/fold-line-rule/spec.md` | the third dependency's output: which atoms exist in the rules tree and in what shape, and that `PERMITTED_FOLD_MECHANISMS` ships empty — this story enforces the *shape* of those atoms and must not be surprised by their content | before writing AC-009's shape checks, so the checks match the atoms that actually landed | AC-009 |
| `standards/rust/81-checks-that-cannot-be-types.md` | RS-81-1 (`:11`, prove the blind spot in the tests then state it in the docs), RS-81-3 (`:209`, scope a scanner to one tree — and why two scanners agreeing on one path is not a violation), RS-81-5 (`:335`, make the failure say **which one moved**) | before writing the module docs (RS-81-1) and before writing AC-008's message (RS-81-5) | AC-008, AC-012, AC-003 |
| `standards/rust/80-the-gate.md` | RS-80-1 and RS-80-2 (`:11`, `:98`) on what a `probe` means and when `None` is the honest answer; RS-80-4 (`:245`) on `--locked` for any step that resolves dependencies | when writing the `Step` literal — it is five fields and three of them are rule-governed | AC-001, NF-001 |
| `xtask/src/spec_trace.rs` | `:122-160` — the *"three lists that must agree"* record, in-house, which is the argument for AC-003's single constant and AC-008's agreement check; and `workspace_root`, the root resolver both existing checkers use | when deciding where the pages-tree path comes from, and when writing the agreement check's message | AC-003, AC-008 |
| `docs/README.md` | `:12-24` the "Looking for / It is at" table and `:25-29` the gate-read-trees paragraph carrying the `[PROVISIONAL]` marker — the exact prose to make unconditionally true, including the closing sentence about moving a tree meaning an `xtask/src/` edit in the same change | in the last step, in the same commit as the mount | AC-012 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | the governing constraint for **three** edits in this PR: retiring the `[PROVISIONAL]` marker, reconciling the generator toward the committed router, and repairing a rule atom that fails a shape check. Each preserves the reasoning and moves only the referent | before editing `docs/README.md`, and whenever EC-008 or EC-009 fires | AC-012, AC-007 |
| `.bklg/docs-that-teach/page-need-discipline/design/mock.html` | the only rendered artifact this project has — 52 labelled frames including S4's `green`, `single-problem`, `many-problems`, `vacuous-tree`, `missing-tree` and `write-repair-offered`, with density chips measured from each specimen's own bytes, and the five findings in its closing panel | when recording AC-010's captures — compare the real terminal output against the frames rather than against prose | AC-010 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | the measured defects the criteria above are written against: the application author's fear of silent wrongness (Persona 1), the adapter author's `E0034` explanation living three documents from where they stood (`:174-181`), the evaluator's *nowhere to go after the second question* (Persona 3) | when a criterion's framing feels arbitrary — this is where each one's evidence is | AC-004, AC-006, AC-012 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | HS-P0020's `## Items` / `## Signatures` — where `xtask::narrative::TREE` is pinned and in what shape, which is the constant this story makes `pub(crate)` and references rather than re-declaring | before writing the pages-tree reader; if it is not in the tree, EC-005 applies | AC-003, AC-002 |
| `RUNBOOK.md` | `:920-925` and `:931-935` — the decorative-gate incident this repository already paid for: a documentation step that printed warnings and exited 0 because nothing read its output. It is the reason the vacuity guard is a `bail!` and the success line prints a count | once, before deciding that any guard here is "probably unnecessary" | AC-002, AC-010 |

## Clarifications resolved during spec

1. **The AC set is exactly the twelve the front half enumerated** — AC-001 through AC-012, none added
   and none dropped. `_ledger.md` carries the same twelve ids.
2. **Where `_design.md`'s `## States` block and `router-precedence-and-announcement`'s forward
   contract appeared to disagree** about what the generated region contains, the front half resolved
   it as **two** single-source obligations: the router `## Index` (generated, equality-checked,
   `--write`-repairable) and `NEEDS` ↔ band 10 (containment-and-exclusion, deliberately *not*
   auto-repairable). This half carries them as **AC-007** and **AC-008** respectively. That is a
   reading of two signed-off artifacts, not a new decision, and `_design.md` is not amended.
3. **A malformed declaration is its own problem, not a missing one (EC-010).** Neither the design nor
   the briefs stated what happens to a `> **Answers:**` line that is *nearly* right — an unbackticked
   token, a missing ` — `, a clause not ending in `?`. Silently degrading it to "no declaration"
   would send the author to the wrong repair, so it is reported at its own line naming the part of
   the grammar that failed. AC-004's test list carries it.
4. **Tie-breaking in the sort is specified (EC-011).** `_design.md` says "sorted by path then line";
   two problems can share both. The tie breaks on message text, because AC-010's stability property
   is what lets a reader work down the list across re-runs, and an unspecified tie makes that
   property untestable.
5. **The zero-declaration problem carries no line number, and the type says so.** The design's own S4
   example shows a path-only problem line, and `## Mock` finding 4 records that the precedent's
   messages address a whole file in four of six cases. The line is `Option`al in the problem type
   rather than faked as line 0.
6. **The 100-character budget is recorded, not met by shrinking a citation.** `## Mock` finding 1
   measured 112 characters after the yield remedy was applied, against an inherited 111 at
   `lint_constitution.rs:369-378`. AC-010 therefore requires the **measured longest line** in the
   ledger; `## Density budget`'s yield order governs what may shrink, and `path:line` and the repair
   pointer are never it.
7. **AC-012 bundles three things that are one obligation.** Stating the limits first, deleting the
   scaffold `allow`, and retiring the `[PROVISIONAL]` marker are each the same move — no claim
   outlives the commit that makes it true, and no scaffold outlives the story it named. Splitting
   them into three ledger rows would have implied one could land without the others.
8. **Not settled here, and deliberately.** Whether a page-need index is generated into the router
   (deferred by `_design.md` `### S2`, and it would couple two projects' commit cadence); whether
   `lint_constitution`'s absence from `affected.rs`'s unconditional list is a defect (this story
   joins the majority precedent and does not reconcile the minority one); and whether
   `PERMITTED_FOLD_MECHANISMS` ever gains an entry (HS-P0020's DT-7 demonstration owns that, and
   `_design.md` `## Sign-off` condition 3 states the consequence of shipping it empty).
