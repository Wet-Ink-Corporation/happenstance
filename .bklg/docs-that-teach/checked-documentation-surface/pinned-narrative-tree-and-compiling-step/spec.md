---
item: HS-S0136
stage: spec
created: 2026-08-17T13:16:01.147Z
updated: 2026-08-17T13:16:01.147Z
template_sig: 87bbf1d0
rendered_sig: "49700538"
---

# Spec — The narrative tree exists and every fence in it compiles, mandatorily

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-01, BR-02, DoD scenarios 1 and 2 |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG; HS-P0020 first, no inbound edge |
| Project charter | `.bklg/docs-that-teach/checked-documentation-surface/project.md` — AC-002, AC-009, DR-01/02/03/10/11/12 |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/pinned-narrative-tree-and-compiling-step/spec.md` |
| Key briefs | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` — architecture brief Notes 1 (CR-1/CR-3/CR-4), 3 (mechanism), 4 (data flow), 10 (the limits list); testing brief AC-002/AC-009/AC-010; deployment brief Option A |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — D1 (`docs/` is the tree, the markdown is the render), the fixture at `## The doctest`, `## Composition`, `## Density budget`, `## Anti-patterns` |
| Story map row | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — milestone `compiled-narrative-tree`, first in merge order |
| Roadmap pointer | `RUNBOOK.md:918-925` — the decorative-step precedent this story exists not to repeat |

Authority order for anything this spec does not settle: Accepted decision atoms under `.kb/decisions/`
(none govern gate structure — verified in `_grounding.md`, "Precedence and non-goals") → `CLAUDE.md` →
`standards/rust/80-the-gate.md` and `81-checks-that-cannot-be-types.md` → `_design.md`.

## One-line PR slice

Create the narrative tree at its decided path with a real fixture page, compile every Rust fence in it as
one `#[cfg(doctest)] mod` per page in `xtask/src/narrative.rs` mounted at `xtask/src/lib.rs`, and wire that
compile as a `REQUIRED` `Step` with `probe: None`, `--locked` and step-scoped `RUSTDOCFLAGS`.

## Executive summary

This PR lands the machine's first half: prose lives at a path the gate reads, and the Rust inside it is
compiled against the real workspace crates by a mandatory step of `cargo xtask ci`. Nothing in the
repository reads a sentence today — twenty-four gate steps, none of which opens a page — and after this PR
one of them does.

The delta over what already exists is small and precisely placed. `xtask/src/constitution.rs` already
compiles twenty-seven markdown files as doctests of a `publish = false` crate, one module per file
(`xtask/src/constitution.rs:11-18`); this story does the same thing to a second tree, in a second file, under
a second banner. Three things are genuinely new rather than copied:

1. **The tree.** `docs/` becomes the narrative tree (`_design.md` D1: the markdown *is* the render — no
   mdBook, no site, no build step), carrying one fixture page, composed exactly as `_design.md`'s
   `## The doctest` writes it.
2. **A vacuity guard on the compile step.** The step is filtered to the narrative doctests so its banner
   answers exactly one question, and a filtered `cargo test --doc` exits 0 over `running 0 tests`. So the
   step asserts its doctests out of `--list` before running them (RS-81-4,
   `standards/rust/81-checks-that-cannot-be-types.md:264-272`). This is the only check in the whole project
   that rejects the architecture brief's named most-likely error — `mod narrative;` declared from
   `main.rs` instead of `lib.rs`, which compiles clean while nothing ever compiles a fence.
3. **The index and the pin paragraph.** `docs/README.md` gains the narrative routing table and its
   `:25-29` paragraph — which today names `spec/` and `standards/rust/` as the trees the gate reads by
   path — stops being false.

What this PR does **not** land, and must not be read as: any claim that a page teaches. The step proves
code inside prose still compiles. `documented-blind-spots-and-their-proofs` owns the full limits list and
HS-P0024's friction log owns comprehension; this story owes the "What this does not verify" section its own
additions require, and one unhedged sentence saying the step is silent about teaching (project DoD item 8).

## Context pack

The decisions this story must honor, stated as decisions. Read this whole section before writing code; the
deeper artifacts are linked, and each link says why and when.

**The tree is `docs/`, and the render is the source.** `_design.md` D1 is signed off (2026-08-17, no
conditions): the pinned tree is `docs/` repurposed, and the "render" is the markdown as the repository's own
host renders it, plus rustdoc for the reference layer. **No second render exists** — no `book.toml`, no
mdBook, no stylesheet, no `book/` output directory (`_design.md` `## Anti-patterns` item 10 forbids all of
them appearing anywhere in the repository). mdBook lost on a mechanism, not on taste: it is an external
binary, so it is a `probe:` shape, and `probe: Some(..)` means *skip when absent*
(`xtask/src/main.rs:89-102`) — which DR-03 forbids for this step; wiring it `probe: None` instead makes
every clean checkout without `mdbook` fail the mandatory gate, breaking AC-009. What D1 buys is that the
UX brief's fifth falsifier ("a clean checkout renders something different from what CI checked") becomes
unfalsifiable by construction: there is no render step to skip. What it costs — no sidebar, no cross-page
search, no prev/next — is paid deliberately, and hand-rolling any of them is an anti-pattern here.

**There are two composition roots in `xtask`, they do not share modules, and using the wrong one is the
named most-likely error.** `xtask/src/lib.rs:28` declares `mod constitution;` and is the **doctest** root:
`cargo test -p xtask --doc` compiles the *lib* target's doctests only. The harness for the narrative tree
mounts there as a sibling (`mod narrative;` → `xtask/src/narrative.rs`). A `mod narrative;` added to
`xtask/src/main.rs` instead lands in the *bin* crate, compiles clean, and its fences are compiled by
nothing — the exact silent-pass shape BR-02 exists to prevent (architecture brief Note 1, CR-1). The
`Step` goes in `REQUIRED` at `xtask/src/main.rs:105` and nowhere else (RS-80-1), because `--fast`,
`cargo xtask wasm` and `cargo xtask lints` all select out of that array.

**One `#[cfg(doctest)] mod` per page, and that is what makes a failure nameable.** The obvious spelling —
several `#![doc = include_str!(…)]` attributes on one module — concatenates into a single doc string, so a
failure in the nineteenth file reports a line counted from the first, "which maps to no file a reader can
open" (`xtask/src/constitution.rs:11-18`). One module per page makes the failure read
`xtask::narrative::append_conditions (line 12)`, with the line relative to the page. The residual
degradation is real and must be recorded rather than routed around: the *file* rustdoc names is the harness,
not the markdown (architecture brief Note 3, and the charter's risk table).

**The step is `REQUIRED`, `probe: None`, `--locked`, and its `RUSTDOCFLAGS` live in its own `env`.**
`None` is what makes "the tool is missing" impossible rather than a skip (RS-80-1/RS-80-2,
`standards/rust/80-the-gate.md:11,98`); `--locked` is owed by every gate invocation that resolves
dependencies (RS-80-4, `:245`); `RUSTDOCFLAGS` goes in the `Step`'s own `env` because rustdoc does not read
`RUSTFLAGS` and setting it ambiently leaks into every other step (RS-80-3, `:181`, and the `Step::env` doc
comment at `xtask/src/main.rs:80-88`). `run_fast` runs the entire `REQUIRED` array unfiltered
(`xtask/src/main.rs:853-860`), so a `REQUIRED` member is reached by `cargo xtask ci --fast` for free — that
is two of AC-009's three invocation paths; the third (`cargo xtask affected --base main`) is the
slice-mate's.

**Two steps, two banners — and the ordering is what makes that true.** `_design.md` fixes the step name as
the claim sentence `the narrative tree's examples compile` (surface `gate-narrative-compile-step`; the house
convention is that every `REQUIRED` name is a claim, `xtask/src/main.rs:466`, and mock finding 6 was
re-decided at the design gate on exactly this point). The compile and the checker are separate steps so a
reader can tell from the banner alone which half failed. Two mechanical consequences follow, and neither is
optional:

- The step's doctest filter is `narrative::`, so a constitution failure cannot fire under this banner.
- The step is placed **immediately before** `the constitution's examples compile`
  (`xtask/src/main.rs:488-493`), because that existing step is unfiltered and therefore also compiles the
  narrative fences. `run_steps` bails at the first failing step (`xtask/src/main.rs:886-888`), so ordering
  is what keeps a broken narrative fence attributed to the narrative banner. Do **not** add a filter to the
  constitution step to fix this from the other side: its argv also carries the repository README's doctest
  (`xtask/src/lib.rs:21`), whose test name contains neither `narrative::` nor `constitution::`, so filtering
  it would drop the README out of the gate entirely.

**A filtered `cargo test --doc` is a decorative step unless its doctests are asserted first.** `cargo test`
exits 0 on `running 0 tests` (RS-81-4; `xtask/src/proof.rs:9-23` makes the argument at length: "a step that
a *deletion* fails and an *emptying* passes is checking the filename"). Here the emptying cases are worse
than hypothetical: the CR-1 mis-mount, a page nobody registered, and an empty tree all produce zero matching
doctests and a green step. So the step runs through a thin `xtask` subcommand that lists
(`cargo test --locked -p xtask --doc -- --list`), asserts at least one `narrative::` doctest and prints the
count, then runs the filtered tests — `xtask/src/proof.rs:191-240` is the shape to copy, including the
inherited `RUSTDOCFLAGS` (the `Step`'s `env` is inherited by the spawned cargo, which is why
`proof-artefact` is already wired this way). The bidirectional page↔module registration check is **not**
this story's: `narrative-checker-mounted-with-pinned-path` owns it (AC-005), and implementing it here would
put two versions of one check in the tree.

**The fixture page is the design's artifact, verbatim, not a page of this story's invention.**
`_design.md` `## The doctest` writes the fixture out in full and calls it "the literal artifact
`pinned-narrative-tree-and-compiling-step` and `observed-failure-falsification` build against" — the
substitute for a mock, since this project adds no public API. Reproduce it: H1 `Appending under a condition`
(≤ 40 characters, because the H1 is the index table's left column at the 70-column narrow width — the
design gate dropped this from 60 to 40 on mock finding 4); the reserved answered-need slot present and
**empty** (BR-04's named need is HS-P0021's discipline, and reserving the slot positionally *is* the seam
between the two projects); the clause id inline in the sentence that depends on it, never in a footer; each
fence immediately after the sentence it demonstrates, because "compiles but no longer demonstrates" is the
step's headline blind spot and adjacency is the only thing that makes it reviewable by a human; the scope
band last, visible, level-3, alphabetical, two scopes — inside the ≤ 3 scopes × ≤ 25 lines threshold, so it
correctly stays inline rather than splitting. Hidden panels are forbidden outright (`_design.md` D2/DT-7):
no `<details>`, no `<summary>`, no tab markers anywhere under `docs/`. Enforcement of that ban is
`hidden-content-resolution`'s; **compliance** with it is this story's, and a fixture that violated it would
be the negative fixture rather than the positive one.

**The page's path is a gate-relevant number, not a style note.** The location-prefix budget is ≤ 48
characters *inclusive of the 16-character `xtask\src\../../` doctest prefix*, i.e. **≤ 32 characters of
repo-relative path**, with at most two directory levels under `docs/` and a filename ≤ 32 characters
(`_design.md` `## Density budget`, decided at the design gate on mock finding 3; `## Anti-patterns` item
11). `docs/append-conditions.md` is 25 characters and fits. Fences are ≤ 80 columns, prose source wraps at
≤ 90, the page is ≤ 250 source lines, and **a fence never yields** — never elided, truncated, wrapped
mid-token, or replaced by a prose description of itself.

**Zero new dependencies, and that is a constraint with a record behind it.** The mechanism is `include_str!`
under `#[cfg(doctest)]` against the dev-dependencies `xtask/Cargo.toml:22-33` already declares
(`happenstance`, `happenstance-core` with `memory`, `happenstance-testkit`, `bytes`, `futures-*`, `tokio`,
`trait-variant`). DR-12's standing trade is written into `xtask/Cargo.toml:16-21`: `rusqlite` and `sqlx` are
deliberately absent because every `cargo xtask ci` would then build them. A narrative renderer would be the
same bargain with less to show for it.

**The persona-journey slice.** The "user" here is a **contributor running the gate** and a **reviewer
reading its output** — this project ships no runtime surface. The journey this story realizes: a contributor
edits a narrative page, runs `cargo xtask ci` (or `--fast`), and either sees
`=== the narrative tree's examples compile ===` followed by rustdoc's own report identifying the page module
and a line inside it, or sees one line saying how many pages' examples were enumerated and compiled. A
reader's journey is served only to the extent that `docs/README.md`'s first table now routes to a page whose
code the gate compiled. Nothing in either surface carries a badge, tick or "verified" mark
(`_design.md` `## What a user meets first`; DoD item 8).

**What must not move.** No `SPECIFICATION.md` clause is amended, discharged or restated — AC-007/AC-008
*read* that document in a later milestone; this story does not touch it. The precedence chain in
`standards/rust/README.md:23-29` gains no tier. `xtask/src/lint_constitution.rs` and
`xtask/src/constitution.rs` are not refactored to share code with the new files: RS-81-3 scopes a scanner to
the directory whose behaviour it constrains, and a shared abstraction makes one error message answer two
questions. CLAUDE.md's binding constraints are untouched by construction (nothing here touches a port, an
async fn, a `Send` bound or a feature), and a fence that violated one would be caught by this very step
rather than by a reviewer.

## Integration contract

- **Archetype**: `capability` — user-observable end to end, where the user is a contributor running the
  gate and the observation is a named banner plus a real compile.
- **Slice / milestone**: `compiled-narrative-tree`. Slice-mate: `narrative-tree-story-grain-selection`
  (extends `xtask/src/affected.rs`'s selection arm so a prose-only change selects `xtask`). Both land in one
  context, this one first; the slice-mate is the only reason a later story's own story-grain gate is not
  blind to prose-only changes.
- **Mount point**: `xtask/src/main.rs` — the `REQUIRED` array at `:105` (CR-3), with
  `xtask/src/lib.rs:28` as the **co-required** second root (CR-1, the doctest target). Both are load-bearing
  and neither is sufficient: a step in `REQUIRED` with the harness in the bin crate compiles nothing; a
  harness in the lib crate with no step is reached only by the constitution's step, under a banner that
  names the wrong corpus. A third mount is `xtask/src/main.rs:642-707` / `:718+` (CR-4): the subcommand's
  dispatch arm and its `print_help()` line.
- **Wires into**: `xtask/src/lib.rs:21-28` (the existing doctest root and its stated
  cannot-live-in-a-published-crate reasoning) · `xtask/src/main.rs:72-103` (`Step`, and the `probe` doc
  comment that is the normative statement of what `None` means) · `xtask/src/main.rs:853-892`
  (`run_fast`/`run_steps`, the banner and the bail) · `xtask/src/proof.rs:191-325` (the assert-out-of-`--list`
  shape, and its `Command` + inherited-env spawning) · `xtask/Cargo.toml:22-33` (the dev-dependency set every
  fence resolves against) · `crates/happenstance-core/src/lib.rs:122` and
  `crates/happenstance-core/src/memory.rs:87,171` (`MemoryEventStore`, `::new`, `::len` — the real public
  items the fixture fence calls) · `docs/README.md:12-29` (the index's existing two-column table and the
  pin-by-path paragraph).
- **Renders surfaces** (ids from `_design.md` `## Surfaces`): `gate-narrative-compile-step` (created) ·
  `narrative-page` (created — the fixture page, states `default` and `scoped-inline`) ·
  `narrative-tree-index` (changed — the narrative table added above the pointer-out table, plus the
  `:25-29` paragraph). Not rendered here: `narrative-scoped-page` (the fixture's scope band is inside the
  threshold, so it stays inline by design), `gate-narrative-checker-step`
  (`narrative-checker-mounted-with-pinned-path`'s), `rustdoc-reference-surface` (untouched by this project).
- **Public items** (`_design.md` `## Items`): **none**. No `pub` item is added to any publishable crate;
  `xtask` is `publish = false`. The design's `## Items` block lists the checker's four constants and
  `clause_ids`, all of which belong to later stories. The one addition this story makes that the design's
  block does not name is the compile step's vacuity-guard entry point in the bin crate — see
  "Behavior and interfaces", row 5, which states it as an addition to the mechanism layer the design left
  open and names what it would cost to decline it.
- **Conformance rule(s)**: none, and this is not adapter-observable. This story adds no port, no value type
  and no testkit rule; it changes what the *gate* proves about prose, not what a store must do. The
  house-style rules it is held to instead are RS-80-1, RS-80-2, RS-80-3, RS-80-4 and RS-81-4.
- **Clause(s)**: none discharged and none amended. `spec/SPECIFICATION.md` is read by this milestone only
  as a source of a citation the fixture page makes (`ES-40`, `spec/SPECIFICATION.md:4351`); resolution of
  citations is `narrative-citation-resolution`'s and pinning the frozen documentation MUSTs is
  `frozen-documentation-must-pin`'s. Nothing here needs an ADR — verified in `_grounding.md`, and the story
  map records the same finding.
- **Advances DoD scenario**: initiative **DoD-1** (`@smoke` — "from a fresh clone with no local state, the
  full gate runs green and the narrative material builds and renders as part of it, not as a separate manual
  step someone remembers to do"), which this story moves from *nothing to build* to *green with the tree
  inside it*. It also makes **DoD-2** reachable for the first time by authoring the artifact that scenario
  breaks; DoD-2 itself is `observed-failure-falsification`'s and is not claimed here.

**Delivered mounted, not as a component.** The merge bar is `cargo xtask ci --fast` printing
`=== the narrative tree's examples compile ===` and passing, on this tree, from a clean checkout — not a
harness file that exists and a step that was written down.

## PR boundary

```
docs/README.md
docs/append-conditions.md
xtask/src/lib.rs
xtask/src/main.rs
xtask/src/narrative.rs
xtask/src/narrative_doctests.rs
.bklg/docs-that-teach/checked-documentation-surface/pinned-narrative-tree-and-compiling-step/**
```

**In this PR**

- `docs/append-conditions.md` — the fixture page, composed exactly as `_design.md` `## The doctest` writes
  it, at a path inside the ≤ 32-character repo-relative budget.
- `xtask/src/narrative.rs` — the doctest harness: module docs opening with "What this does not verify",
  then one `#[cfg(doctest)] mod` per page carrying `#![doc = include_str!("../../docs/<page>.md")]`.
- `xtask/src/lib.rs` — `mod narrative;` beside `mod constitution;`, with the one-sentence reason.
- `xtask/src/narrative_doctests.rs` — the vacuity guard and the filtered run (RS-81-4).
- `xtask/src/main.rs` — the `REQUIRED` entry immediately before `the constitution's examples compile`, the
  `mod` declaration, the dispatch arm, and the `print_help()` line.
- `docs/README.md` — the narrative routing table above the existing pointer-out table, and the `:25-29`
  paragraph updated to name the narrative tree as a third tree the gate reads by path.
- This story's own backlog folder — the ledger and the implementation record.

**Explicitly not in this PR**

- `xtask/src/affected.rs` and its tests — the slice-mate `narrative-tree-story-grain-selection` owns the
  selection arm, the `INERT` list (`affected.rs:248-266`) and the two directional unit tests. This story
  must not pre-empt it; note that `a_docs_only_change_selects_nothing` (`affected.rs:651`) asserts over
  `RUNBOOK.md`, not a `docs/` path, so adding a page under `docs/` does not break it and does not force the
  arm early.
- The pinned `TREE` / `HARNESS` / `IGNORE_ALLOWANCES` / `HIDDEN_MARKERS` constants, the missing-tree error,
  the empty-tree `bail!`, the bidirectional registration check, the fence walk and the
  `cargo xtask narrative` step — all `narrative-checker-discipline`'s milestone.
- `xtask/src/spec_trace.rs`, `clause_ids`, citation resolution, the frozen-MUST pin —
  `specification-pin`'s milestone.
- Breaking the fixture and recording the failure (`observed-failure-falsification`), the full six-item
  limits list and the re-run `RUSTDOCFLAGS` probe (`documented-blind-spots-and-their-proofs`).
- Any teaching page. The corpus is HS-P0021/22/23's; this page is test material and the charter's risk
  table says so in as many words.
- Any refactor of `xtask/src/constitution.rs` or `xtask/src/lint_constitution.rs`; any new dependency; any
  `book.toml`, `.css`, `book/` or `site/` artifact.
- `spec/SPECIFICATION.md`, `standards/rust/**`, `CHANGELOG.md`, and every crate under `crates/`.

**Merge DoD**: from a clean checkout, `cargo xtask ci --fast` is green with
`=== the narrative tree's examples compile ===` in its output above the constitution's banner, that step
reports the number of pages it enumerated, `cargo xtask narrative-doctests` fails when the harness is
mis-mounted or empty, and `docs/README.md` no longer says something false about which trees the gate reads.

## Behavior and interfaces

Each row opens with the `AC-###` it will be enumerated as. Evidence paths are the file the behaviour is
copied from, constrained by, or observable in.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **(AC-001)** The narrative tree exists at `docs/` and holds a page composed as the signed-off design writes it | `docs/append-conditions.md`: H1 `Appending under a condition` (26 chars, ≤ 40); the reserved answered-need slot present and empty; the `ES-40` citation inline in the sentence that depends on it; the `rust` fence immediately after the sentence it demonstrates; the scope band last, level-3, alphabetical, two scopes (inside ≤ 3 × ≤ 25); no hidden marker of any kind; ≤ 250 source lines; fences ≤ 80 columns; path 25 chars repo-relative, one directory level | `_design.md` `## The doctest`, `## Composition`, `## Density budget`, `## Anti-patterns` |
| **(AC-002)** Every Rust fence in the tree is compiled against the real workspace crates, one `#[cfg(doctest)] mod` per page | `xtask/src/narrative.rs` carries `#[cfg(doctest)] mod append_conditions { #![doc = include_str!("../../docs/append-conditions.md")] }`. `include_str!` resolves at compile time, so **moving or renaming the page without editing this file is a compile error** — a stronger pin than a runtime path check. Module name derived from the filename by one pure rule (`NN-slug.md`/`slug.md` → `slug`), stated in the module docs because both directions of the later registration check must agree with it by construction | `xtask/src/constitution.rs:11-18,41-44`; architecture brief Note 2 (`atoms()` derivation, `lint_constitution.rs:208-246`) |
| **(AC-003)** The harness is declared from the **lib** target, so the fences are reachable by `cargo test -p xtask --doc` | `mod narrative;` added to `xtask/src/lib.rs` beside `mod constitution;` (`:28`). The named wrong implementation is `mod narrative;` in `xtask/src/main.rs`: it compiles clean and nothing ever compiles a fence. Rejected by AC-005's guard, which is the only thing in the project that can see it | `xtask/src/lib.rs:21-28`; architecture brief Note 1 CR-1 |
| **(AC-004)** The fences are compiled by a mandatory gate step named as a claim, positioned so its banner answers one question | New `Step` in `REQUIRED`: `name: "the narrative tree's examples compile"`, `probe: None`, `env: &[("RUSTDOCFLAGS", "-D warnings")]`, `--locked` on the invocation that resolves dependencies, inserted immediately **before** `the constitution's examples compile`. Doctests filtered to `narrative::` so a constitution failure cannot fire under this banner; ordering is what covers the converse, since the constitution's step is unfiltered and `run_steps` bails at the first failure. The constitution step's argv is **not** filtered in compensation — it also carries the repository README's doctest, whose name matches neither filter | `xtask/src/main.rs:72-103,105,466,488-493,862-892`; RS-80-1/2/3/4 (`standards/rust/80-the-gate.md:11,98,181,245`) |
| **(AC-005)** Zero matching doctests is a hard failure, not a green step | `xtask/src/narrative_doctests.rs`: enumerate with `cargo test --locked -p xtask --doc -- --list`, keep the lines whose name contains `narrative::`, `bail!` naming the expected harness path and the CR-1 mis-mount when the set is empty, print `  {n} page(s)' examples enumerated`, then run `cargo test --locked -p xtask --doc -- narrative::` and `bail!` on a non-zero status. The `Step`'s `RUSTDOCFLAGS` is inherited by the spawned cargo, which is why `proof-artefact` is already wired as `cargo run -p xtask -- …`. **Addition to the design's `## Items` block**, stated rather than smuggled: the design specifies the checker's constants and never specifies this step's argv. Declining the guard is the alternative, and it costs a mandatory step that prints green over zero doctests — `RUNBOOK.md:918-925` exactly. What it deliberately does *not* do is compare pages on disk against modules in the harness; that is `narrative-checker-mounted-with-pinned-path`'s AC-005 and belongs in one place | RS-81-4 (`standards/rust/81-checks-that-cannot-be-types.md:264-272`); `xtask/src/proof.rs:9-23,191-240,292-325` |
| **(AC-006)** The capability is mounted completely, so a half-mount is a build-time bug rather than a silent omission | `mod narrative_doctests;` in `xtask/src/main.rs:64-70`; a dispatch arm mirroring `Some("proof-artefact") => proof::run()` (`:681`); a `print_help()` entry in the register of the existing ones, naming the vacuity argument in one line; **not** added to `lint_steps()` (`:799-808`), which is the file-reading family — this step compiles. `steps_named` panics on a name absent from `REQUIRED` (`:816-826`), so any by-name selection of the new step fails loudly if the entry is missing | `xtask/src/main.rs:64-70,642-707,718-767,799-826` |
| **(AC-007)** The index routes to the page, and the pin-by-path paragraph stops being false | `docs/README.md`: the narrative table (exactly two columns — page name as it appears in the H1, and the link) placed **above** the existing pointer-out table, because a reader who opened `docs/` wants the pages and a reader who wants the specification is being redirected; the existing `:25-29` paragraph updated from "Two of those are read by the gate" to name the narrative tree as a third, so the sentence "moving either tree means editing `xtask/src/` in the same change" stays true of `docs/` itself. No nested list under a parent row; no hand-rolled TOC, breadcrumb or prev/next anywhere | `docs/README.md:12-29`; `_design.md` `## Composition` (`narrative-tree-index`), `## Transience policy`, `## Anti-patterns` items 2 and 6 |
| **(AC-008)** The new files state what they do not verify, first rather than last, and say nothing about teaching | `xtask/src/narrative.rs` and `xtask/src/narrative_doctests.rs` each open with a `# What this does not verify` section, in the shape of `lint_constitution.rs:9-13` ("a check whose limits are undocumented is read as a guarantee"). This story's own additions: (1) code that still compiles while no longer demonstrating the surrounding claim; (2) doctests receive neither the workspace `[lints]` set nor clippy, so `unwrap_used = "deny"` is unenforced inside every fence; (3) the reported *file* is the harness, not the markdown — the module resolves the page and the line resolves the location; (4) a fence tagged `text` is neither compiled nor flagged; (5) `RUSTDOCFLAGS` now passes through an extra `cargo run` hop, so what it actually enforces inside a narrative fence is **unmeasured here** and is `documented-blind-spots-and-their-proofs`' probe to re-run — do not copy `constitution.rs`'s finding or the upstream issue's claim; (6) one unhedged sentence: this step is silent about whether the page teaches | `xtask/src/lint_constitution.rs:9-13`; `xtask/src/constitution.rs:20-36`; architecture brief Note 10; RS-81-1 |
| **(AC-009)** Nothing else in the gate regresses, and no dependency is added | `xtask/Cargo.toml` unchanged — the fixture fence calls `happenstance_core::MemoryEventStore` (`::new` at `memory.rs:87`, `::len` at `:171`), reachable through the existing `happenstance-core` dev-dependency with `memory` on. `the constitution's examples compile` and `cargo xtask lint-constitution` still pass; the repository README's doctest still runs; `cargo xtask ci --fast` is green from a clean checkout with the new banner in its output. `is_inert("docs/")` still returns `true` (untouched here by design), so this story's own `affected_gate` selects `xtask` on the strength of its `xtask/src/**` edits — the prose-only case is the slice-mate's | `xtask/Cargo.toml:16-33`; `crates/happenstance-core/src/lib.rs:122`; `xtask/src/affected.rs:248-266`; `.redkiln/config.yaml:40,55` |

**One flagged residual, recorded rather than silently fixed.** The design's fixture sentence reads "An
append condition is checked against the same boundary the query read … (ES-40)", while ES-40's normative
text is about *completeness* — a conditional append being sound only over a store that holds every event the
condition ranges over (`spec/SPECIFICATION.md:4351-4358`); the boundary property the sentence describes is
argued at `:4293` as the joint consequence of ES-38 and ES-40. The id resolves, so nothing in this milestone
or in `narrative-citation-resolution` fails on it. Reproduce the fixture as signed off and record the
observation here: the fixture is the page `observed-failure-falsification` breaks by making a **true** claim
false, so if the claim's citation is judged mismatched the correction is an amendment to `_design.md`
`## The doctest` by its owner, not a silent edit by this story.

## Data and migrations

**N/A — no data store, no schema, no persisted state, no backfill.** The deployment brief states it
directly: this is a `publish = false` tooling and documentation change inside a Cargo workspace, and the
only artifacts it produces are markdown and Rust source in git.

Three migration-shaped facts that are not migrations but would be missed if unstated:

- **`docs/` is repurposed, not replaced.** `docs/README.md`'s existing content — the statement of what the
  directory is for, the ten-row pointer-out table, and the "what belongs here" section — is preserved and
  added to. The only rewrite is the `:25-29` paragraph, which becomes false otherwise.
- **No published surface changes.** `happenstance-core`, `happenstance` and `happenstance-testkit` are the
  only publishable crates and none of their public API is touched; the fixture fence *calls* existing public
  items and adds none. No version bump, no `CHANGELOG.md` entry is owed by this story
  (`CHANGELOG.md`'s gate check is per conformance rule, and this story adds none).
- **Rollback is `git revert` of one commit, with one ordering hazard.** Once HS-P0021+ has merged real
  pages into the tree this story pins, reverting this story alone leaves those pages unchecked — the
  retro-fitted-check failure mode the DAG's ordering exists to prevent. Do not roll this story back in
  isolation after the corpus lands (deployment brief, "Rollback posture").

## Acceptance criteria

Every criterion is framed from the intent of a real person crossing the whole stack. Three of the
initiative's journeys touch this story
(`.bklg/docs-that-teach/initiative.md:289-301`, carried from
`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`): *the first fifteen minutes*
(the application author), *survive the second question* (the evaluator), and — because this project's
surface is the gate itself — the contributor and reviewer the story map names as this project's users
(`_storymap.md`, preamble). Where a criterion serves a downstream reader, it serves them **only** by making
the code inside the page they read something the gate compiled; no criterion below claims a page teaches.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an application author in *the first fifteen minutes*, who has run `cargo add happenstance` and opened `docs/` looking for how a consistency boundary is actually expressed, **WHEN** they open the page the index routes them to, **THEN** they meet one page composed exactly as the signed-off design writes it — a ≤ 40-character H1, the reserved answered-need line present and empty, the `ES-40` citation inside the sentence that depends on it rather than in a footer, each `rust` fence immediately after the sentence it demonstrates, the two-scope band last and visible at level 3 in alphabetical order, ≤ 250 source lines, fence lines ≤ 80 columns, prose source wrapped ≤ 90 — and **nothing they must click, expand or unfold to read**. | `xtask/src/narrative_doctests.rs::tests::the_fixture_page_carries_no_hidden_marker_and_fits_the_path_budget` (reads `docs/append-conditions.md` as text; asserts none of `_design.md`'s `HIDDEN_MARKERS` tokens appears — `<details`, `<summary`, `{{#tabs`, `{{#tab `, `{{#endtabs`, the admonish fence marker, `<!-- tab` — and that the repo-relative path is ≤ 32 characters). The editorial halves — the H1 cap, the 250-line cap, slot/citation/fence adjacency, band order — are **review** rules by design (`_design.md` `## Open questions` item 2 makes them deliberately not gate rules) and are checked against `_design.md` `## The doctest` at this story's own review gate. |
| AC-002 | **GIVEN** an application author who copies the append-condition example out of a narrative page into their own program, **WHEN** a maintainer later removes, renames or re-signatures the public item that example calls, **THEN** the gate fails on that page before the change can merge, so the reader never meets an example that no longer compiles against the crate they installed. | `cargo test --locked -p xtask --doc -- narrative::` — the mechanism itself, not a test about it. The listing names `xtask::narrative::append_conditions (line N)`, one module per page. Proof it reaches real workspace types: the fence calls `happenstance_core::MemoryEventStore::{new,len}` (`crates/happenstance-core/src/lib.rs:122`, `crates/happenstance-core/src/memory.rs:87,171`); temporarily renaming `len` must fail this command, recorded in the implementation report. The full observed-fail/observed-recover procedure is `observed-failure-falsification`'s. |
| AC-003 | **GIVEN** a contributor wiring the harness for the first time, **WHEN** they declare `mod narrative;` from the bin crate instead of the lib crate, **THEN** the mistake is rejected loudly, rather than producing a repository that compiles clean while nothing on any page is ever compiled. | `cargo test --locked -p xtask --doc -- --list` lists at least one `narrative::` doctest (the lib target is what `--doc` compiles), asserted mechanically by AC-005's guard — which is the only check in this project that can see the mis-mount. `xtask/src/narrative_doctests.rs::tests::a_listing_with_no_narrative_doctest_is_a_problem` is the unit-level statement of the same rejection. |
| AC-004 | **GIVEN** a reviewer reading a red CI log who needs to know which half of the documentation machine broke, **WHEN** a Rust fence on a narrative page stops compiling, **THEN** the first banner they meet is `=== the narrative tree's examples compile ===` — never the constitution's — the step is reached on every runner with no tool to install, and its `RUSTDOCFLAGS` are visible in the step rather than smuggled into the environment of every other step. | `xtask/src/narrative_doctests.rs::tests::the_narrative_step_is_required_and_unprobed`, `::the_narrative_step_precedes_the_constitution_step`, `::the_narrative_step_carries_rustdocflags_in_its_own_env`, `::the_narrative_step_passes_locked` — four assertions over `crate::REQUIRED` (`xtask/src/main.rs:105`), plus the observed `cargo xtask ci --fast` output recorded in the implementation report. |
| AC-005 | **GIVEN** a contributor who mis-mounts the harness, deletes the last page, or adds a page nobody registered, **WHEN** the gate runs, **THEN** the step **fails** naming the expected harness path and the CR-1 mis-mount, and prints how many pages it enumerated when it passes — so a green banner can never mean "there was nothing to compile". | `xtask/src/narrative_doctests.rs::tests::an_empty_listing_is_a_problem`, `::a_listing_with_no_narrative_doctest_is_a_problem`, `::a_listing_with_one_narrative_doctest_is_accepted_and_counted` — over fixture `--list` output strings, in the shape of `xtask/src/proof.rs:191-240`. RS-81-4 is the rule (`standards/rust/81-checks-that-cannot-be-types.md:264-272`). |
| AC-006 | **GIVEN** a contributor iterating on a page who does not want to pay for the whole gate, **WHEN** they run `cargo xtask narrative-doctests` or read `cargo xtask` with no arguments, **THEN** the subcommand runs the same check the gate runs and is listed in the help beside the others — and a half-mounted step (named in the gate, unreachable by hand, or vice versa) is a test or build failure rather than a silent omission. | `xtask/src/narrative_doctests.rs::tests::the_gate_can_select_the_narrative_step_by_name` (calls `steps_named` — `xtask/src/main.rs:816-826` panics on a name absent from `REQUIRED`) and `::the_narrative_step_is_not_a_lint_step` (`lint_steps()`, `:799-808`, is the file-reading family; this step compiles). Observed `cargo xtask narrative-doctests` and `cargo xtask` help output recorded in the implementation report. |
| AC-007 | **GIVEN** the evaluator in *survive the second question*, who lands on `docs/README.md` after one answer and needs somewhere to go next, and **GIVEN** the contributor who reads the same file to learn which trees the gate pins by path, **WHEN** either opens it, **THEN** the narrative table is the **first** table on the page and routes to the page by the name in its H1, the paragraph naming the gate-read trees names the narrative tree as one of them, and no breadcrumb, hand-rolled TOC or prev/next footer was added. | `xtask/src/narrative_doctests.rs::tests::the_index_names_the_narrative_tree_as_gate_read` (asserts the `docs/README.md:25-29` paragraph names the narrative tree and `cargo test -p xtask --doc`) and `::the_narrative_table_precedes_the_pointer_out_table` (byte offset of the narrative table's header row is before `\| Looking for \|`). Absence of the forbidden navigation constructs is review, against `_design.md` `## Anti-patterns` items 2 and 6. |
| AC-008 | **GIVEN** any contributor or downstream project reading the new machinery for the first time, **WHEN** they open `xtask/src/narrative.rs` or `xtask/src/narrative_doctests.rs`, **THEN** the **first** thing in the module docs is what the check does not verify — six limits, ending in one unhedged sentence saying this step is silent about whether the page teaches — so the green step is never inherited as evidence of teachability. | `xtask/src/narrative_doctests.rs::tests::the_new_modules_state_their_limits_first` (reads both files as text — the precedent for a check reading `xtask/src/` source is `xtask/src/lint_constitution.rs:424-425` — and asserts the `What this does not verify` heading precedes every other heading in each module's docs). Contents reviewed against architecture brief Note 10; the `RUSTDOCFLAGS` limit is recorded as **unmeasured here** and left to `documented-blind-spots-and-their-proofs`. `cargo xtask lint-constitution` and `cargo test -p xtask --doc` still pass. |
| AC-009 | **GIVEN** a contributor on a fresh clone with nothing installed beyond the pinned toolchain — no `mdbook`, no site generator, no local state — **WHEN** they run `cargo xtask ci --fast` (and, before the project closes, `cargo xtask ci`), **THEN** it is green with the new banner in its output and **no `skipped` line for it**, the narrative material is built and read as an ordinary step rather than a manual one, nothing already in the gate regresses, and no dependency was added to pay for it. | Gate-integration, observed and recorded: `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, `integration_scoped`) and `cargo xtask ci` (`:60`). Regression tier: `cargo test -p xtask` (including `xtask/src/affected.rs`'s `a_docs_only_change_selects_nothing`, still passing because it asserts over `RUNBOOK.md`) and `cargo xtask lint-constitution`. Zero-dependency tier: `git diff --stat -- xtask/Cargo.toml Cargo.lock` is empty for the dependency tables, and no `book.toml`, `.css` or `book/` path appears in the diff. |

Project AC coverage: **AC-002** (every fence compiled against the real crates, mandatorily) is discharged by
AC-002 + AC-003 + AC-004 + AC-005 here — the mechanism, the target it is declared from, the mandatory step
that runs it, and the guard that stops it from being decorative. **AC-009** (clean checkout, no manual step)
is discharged by AC-009 here for the `ci` and `ci --fast` paths and by AC-004's `probe: None` /
`env`-scoping; the third path, `cargo xtask affected --base main` after a prose-only change, is the
slice-mate `narrative-tree-story-grain-selection`'s and is not claimed here.

## Interaction quality

This story renders three surfaces from the signed-off design (`_design.md` `## Surfaces`):
`narrative-page` and `narrative-tree-index` (markdown a reader meets) and `gate-narrative-compile-step`
(terminal output a contributor meets). The design is binding and is not re-decided here. Below is **which
AC row carries each invariant** — every one of them is an `AC-###` row in the table above, because a bullet
in this section would get no ledger row and never be gated.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Non-occlusion.** Nothing load-bearing is behind a control. No `<details>`, `<summary>`, tab marker or collapsed callout anywhere under `docs/`; the fence band and the citations are always visible (`_design.md` D2/DT-7, `## Transience policy`, anti-pattern 1). | **AC-001** | Text assertion over the fixture page for the `HIDDEN_MARKERS` token set. |
| **In-place, not context-jump.** A claim's provenance is readable without leaving the sentence: the clause id sits inline, not in a footer or behind a link (`_design.md` `## Transience policy`, "Clause citations"). On the terminal side, the failure is readable in the same log the contributor already has — no re-run with a different flag to find out which corpus broke. | **AC-001** (inline citation) · **AC-004** (one banner, one question) | Review against `_design.md` `## The doctest`; `the_narrative_step_precedes_the_constitution_step`. |
| **Preserved position / no relayout.** `docs/README.md` is *extended*, not rewritten: the existing pointer-out table and its ten rows survive intact and in order, so an existing deep reference into that file still lands. The answered-need slot is reserved **positionally** so HS-P0021 can fill it without moving anything else (`_design.md` `## Composition`). | **AC-007** (index) · **AC-001** (reserved slot) | `the_narrative_table_precedes_the_pointer_out_table`; the pointer-out rows are unchanged in the diff. |
| **Reversibility.** A contributor who breaks a fence restores green with `git checkout` alone — there is no generated artifact, cache, `book/` directory or committed render to clean up, because the markdown *is* the render (`_design.md` D1). | **AC-009** | `cargo xtask ci --fast` green after revert; no build output path in the diff. |
| **Keyboard / plain-text reachability.** The media offer no focus model, so the equivalent floor is: every load-bearing string on a page is reachable by Ctrl-F and by `rg` because nothing is hidden, and every page is reachable by a link in the index rather than only by a directory listing (`_design.md` D2 mitigation for (c)). | **AC-001** (nothing hidden) · **AC-007** (a row in the index, not a directory listing) | The same two tests as above. |

**Composition invariants** — from the signed-off `_design.md`; the numbers are its numbers.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all.** The step is not a bare exit code: it prints a claim-sentence banner (`=== the narrative tree's examples compile ===`, `xtask/src/main.rs:864`) and, on success, one two-space-indented count line — the `  {n} atoms, all consistent` primitive (`xtask/src/lint_constitution.rs:192`). The page is not a fence dump: it carries the five composed regions `_design.md` `## Composition` fixes. | **AC-004** (banner) · **AC-005** (count line) · **AC-001** (page regions) | Step-shape unit tests; the guard's count assertion; review against `## The doctest`. |
| **Composition / placement.** Narrative table **above** the pointer-out table; scope band **last**; fence immediately **after** its claim; the new step immediately **before** the constitution's. | **AC-007** · **AC-001** · **AC-004** | `the_narrative_table_precedes_the_pointer_out_table`; `the_narrative_step_precedes_the_constitution_step`; review for the page's internal order. |
| **Transience.** Persistent chrome: H1, reserved slot, inline citations, fences, step banner. Opened-on-demand-by-failing: rustdoc's own doctest report. **Never present:** probe skip line (`probe: None` — structural, DR-03), TOC/breadcrumb/prev-next, Run/Play button, spinner or per-page progress line, and any badge, tick or "verified" mark. | **AC-004** (`probe: None`, no progress noise) · **AC-001** and **AC-007** (nothing hand-rolled, no badge) | `the_narrative_step_is_required_and_unprobed`; review against `_design.md` `## Transience policy` and anti-patterns 2, 4, 9. |
| **Density budget, with its real numbers.** H1 ≤ 40 chars; fence lines ≤ 80 columns; prose source ≤ 90; page ≤ 250 source lines; inline scope band ≤ 3 scopes × ≤ 25 rendered lines; index exactly 2 columns; location prefix ≤ 48 chars **inclusive of the 16-character `xtask\src\../../` doctest prefix** → ≤ 32 characters of repo-relative path, ≤ 2 directory levels under `docs/`, filename ≤ 32 chars; terminal 80 columns; success output **1 line**. | **AC-001** (page + path) · **AC-007** (2 columns) · **AC-005** (one-line success) | Path budget and hidden markers are asserted; the page and index budgets are review rules by design; the one-line success is the guard's own `println!`. |
| **Hierarchy.** Carried only by position, heading level and adjacency — the three levers markdown and a terminal both have. Page: H1 + fence band primary, claim band secondary by adjacency, scope band recessive by position. Index: narrative table primary by position. Terminal: banner first, count last. | **AC-001** · **AC-007** · **AC-004** / **AC-005** | Review against `_design.md` `## Hierarchy`; the ordering tests above. |
| **A fence never yields.** When a budget is exceeded, prose is cut first, then a scope band splits, then the page splits. A fence is never elided, truncated with `…`, wrapped mid-token, or replaced by a prose description of itself. | **AC-001** | Review against `_design.md` `## Density budget`; a yielded fence would also break AC-002's compile. |
| **Named anti-patterns.** 1 (disclosure triangle / tab strip / collapsed callout), 3 (horizontally scrolling fence), 5 (untagged or unlisted-`ignore` fence), 6 (wrapping page title), 11 (third directory level or > 32-char filename) → the page. 2 (hand-rolled TOC/breadcrumb/prev-next), 4 (Run button), 9 (badge/tick/"verified" mark) → the page and the index. 10 (`book/`, `site/`, `book.toml`, any `.css`) → the render shape. | **AC-001** · **AC-007** · **AC-009** | The path/marker test; review; the diff assertion in AC-009. Anti-patterns 7 (failure's first row must start `path:line`) and 8 (no truncated problem list) belong to the **checker's** surface and are `narrative-checker-mounted-with-pinned-path`'s — this step's failure body is rustdoc's own report, which this story does not compose. |

**An unstyled render passes every mechanical assertion above.** Which is why AC-001 and AC-007 exist as
their own rows and are read against `_design.md` by a human: `design.capture` is deliberately undeclared
(`_design.md` preamble), so no perceptual harness will ever run here and the review of these two rows is
the only perceptual gate this surface gets.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | The page is moved or renamed without editing the harness. | **Compile error** from `include_str!`, which resolves at compile time — a harder pin than any runtime path check. The message names the missing path. No runtime branch is written for this case. |
| EC-002 | `cargo test --doc -- --list` returns no line containing `narrative::` — the CR-1 mis-mount (`mod narrative;` in the bin crate), an empty tree, or a page nobody registered. | `bail!` from the step, naming the expected harness path (`xtask/src/narrative.rs`), the lib-crate requirement, and the mis-mount as the likely cause. Exit is **failure**, never a `skipped` line and never a green `running 0 tests` (RS-81-4). |
| EC-003 | A Rust fence on a page fails to compile, or its assertion fails at doctest run time. | The step exits non-zero under its own banner; rustdoc's report names the page module and a line **relative to the page**. `run_steps` bails at the first failing step (`xtask/src/main.rs:886-888`), so the constitution's unfiltered step never runs and never re-attributes the failure. The residual is recorded, not routed around: the *file* rustdoc names is the harness. |
| EC-004 | The `--list` invocation itself fails — the lib target does not build, cargo is unavailable, or the output is unparseable. | Hard error with `.with_context()` naming the command, exactly as `xtask/src/proof.rs`'s `list` does. **Never** interpreted as "no narrative doctests" and never as a pass: an unreadable enumeration and an empty enumeration must produce different messages, or EC-002's diagnosis becomes a guess. |
| EC-005 | The new step is placed **after** `the constitution's examples compile`. | A broken narrative fence is then attributed to the constitution's banner, because that step is unfiltered. Rejected by `the_narrative_step_precedes_the_constitution_step` — the named wrong implementation. Do **not** fix it from the other side by filtering the constitution step: its argv also carries the repository README's doctest (`xtask/src/lib.rs:21`), whose name matches neither filter, and filtering would drop the README out of the gate. |
| EC-006 | Half-mount: the step is in `REQUIRED` but there is no dispatch arm, or a dispatch arm exists with no `REQUIRED` entry, or the help line is missing. | `steps_named` panics on a name absent from `REQUIRED` (`xtask/src/main.rs:816-826`), so any by-name selection fails loudly; a missing dispatch arm makes `cargo xtask narrative-doctests` fall through to the unknown-subcommand path and the `REQUIRED` step's own invocation fail. Both directions are covered by AC-006's tests plus one observed run. |
| EC-007 | `RUSTDOCFLAGS` is set ambiently (shell, `.cargo/config.toml`, or another step) rather than in this `Step`'s `env`. | Forbidden. rustdoc does not read `RUSTFLAGS`, and an ambient `RUSTDOCFLAGS` leaks into every other step's rustdoc invocation (RS-80-3, `standards/rust/80-the-gate.md:181`; `Step::env`'s doc comment, `xtask/src/main.rs:80-88`). Asserted by `the_narrative_step_carries_rustdocflags_in_its_own_env`. |
| EC-008 | `docs/README.md:25-29` is left saying "**Two** of those are read by the gate" after a third tree becomes gate-read. | The file is then false about the repository's own pinning rule. `the_index_names_the_narrative_tree_as_gate_read` fails until the paragraph is corrected in the same change. |
| EC-009 | A fence is added with no language tag, with `ignore`, or with `text` to make a failure go away. | Not mechanically rejected **by this story** — the fence walk and the allowance list are `fence-discipline-and-allowance-list`'s. Until it lands, this is a review obligation and a documented limit (AC-008, limit 4). Recording it as a known hole is required; silently relying on it is not. |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| NF-001 | **Zero new dependencies.** No entry is added to `xtask/Cargo.toml`'s `[dependencies]` or `[dev-dependencies]`, and `Cargo.lock` is unchanged. | The mechanism is `include_str!` under `#[cfg(doctest)]` against the set already declared at `xtask/Cargo.toml:22-33` (`happenstance-core` with `memory` on, which is what makes the fixture fence resolve). DR-12's standing trade is the precedent (`xtask/Cargo.toml:16-21`). Checked by AC-009's diff assertion. |
| NF-002 | **Gate cost.** The added cost is one doctest compile of pages in the tree plus one `--list` enumeration that reuses the same build — no second compile of the workspace, no new build script, no C dependency. | Same shape as the existing constitution step. The `--list` run precedes the filtered run in the same process tree with the same `RUSTDOCFLAGS`, so it warms rather than duplicates the build. |
| NF-003 | **Hermetic and mandatory on every runner.** No network access, no external binary, no tool probe, no `--offline` requirement beyond `--locked`, no toolchain component past the pinned `rust-toolchain.toml`. | `probe: None` (RS-80-1/2) and `--locked` on the invocation that resolves dependencies (RS-80-4, `standards/rust/80-the-gate.md:245`). This is the property D1 bought by refusing mdBook. |
| NF-004 | **No ambient environment mutation.** The only environment change is the `Step`'s own `env`, inherited by the one cargo it spawns. | EC-007; `xtask/src/main.rs:80-88`. |
| NF-005 | **MSRV, wasm32 and features unaffected.** No `cfg`, no feature gate, no target-specific code; the four wasm32 gate steps and the `msrv` CI job are untouched. | Nothing here touches a port, an `async fn`, a `Send` bound or a feature, so ADR-0001 and ADR-0003 are untouched by construction (`_design.md` `## What it costs a caller`). |
| NF-006 | **Output budget.** Success is **one** line inside 80 columns. No progress ticker, no per-page line, no spinner. | `_design.md` `## Transience policy` and `## Density budget` (terminal surface); a green check that says nothing is indistinguishable from one that did not run, and one that says ten lines trains people to skip it. |
| NF-007 | **No publishable surface changes.** No `pub` item is added, changed or removed in `happenstance`, `happenstance-core` or `happenstance-testkit`; `cargo package --list`'s licence/README assertions are unaffected; no `CHANGELOG.md` entry is owed. | `xtask` is `publish = false` (`xtask/Cargo.toml:7`). The fixture fence *calls* existing public items and adds none. |
| NF-008 | **Cross-platform path handling.** The check must work where the doctest name is spelled with backslashes (`xtask\src\../../<page> - narrative::<mod> (line N)`, observed on Windows) as well as forward slashes. | Filter and assert on the `narrative::` substring in the doctest name, never on a path prefix or separator. `_design.md` `## Density budget`, mock finding 3, is where that spelling was observed. |

## Implementation notes (non-prescriptive)

The order that keeps every intermediate state honest, and the traps found while grounding this spec. None
of this is binding; the ACs are.

1. **Write the page first, from `_design.md` `## The doctest` verbatim.** It is the design's artifact, and
   `observed-failure-falsification` builds against the same bytes. Do not improve it. The one observation
   worth carrying forward rather than fixing is recorded at the end of "Behavior and interfaces".
2. **Then `xtask/src/narrative.rs` and the `mod narrative;` in `xtask/src/lib.rs`.** Copy
   `xtask/src/constitution.rs:11-18`'s shape — one `#[cfg(doctest)] mod` per page, each carrying an
   **inner** `#![doc = include_str!(…)]`. The concatenation trap is the reason: several outer
   `#[doc = …]` attributes on one module fuse into a single doc string and a failure in the nineteenth
   file reports a line counted from the first. `#[cfg(doctest)]` (not `cfg_attr`) keeps narrative prose
   out of `xtask`'s own rendered docs, where it would be actively confusing.
3. **Confirm the mechanism before wiring anything:** `cargo test --locked -p xtask --doc -- --list` should
   name `narrative::append_conditions`. If it does not, the `mod` is in the wrong crate — which is the
   whole point of doing this step before the gate step exists.
4. **Then `xtask/src/narrative_doctests.rs`.** Two functions, in `xtask/src/proof.rs:191-240`'s shape: one
   that runs `--list` and returns the matching lines (pure parsing split out so it is unit-testable
   against a fixture string), one that runs the filtered tests and `bail!`s on a non-zero status. Note
   `--` before `--list`, and that `cargo test --doc` needs the doctest filter after a second `--`.
5. **Then the four mounts in `xtask/src/main.rs`**, together, because a partial mount is EC-006: `mod`
   declaration, the `REQUIRED` entry immediately before `the constitution's examples compile`
   (`:488-493`), the dispatch arm beside `Some("proof-artefact") => proof::run()` (`:681`), and the
   `print_help()` line (`:718+`). Do **not** add it to `lint_steps()` (`:799-808`) — that family reads
   files; this one compiles.
6. **Where the unit tests live.** `xtask/src/main.rs` carries no `#[cfg(test)]` module today; a
   `#[cfg(test)] mod tests` inside `xtask/src/narrative_doctests.rs` can reach `crate::REQUIRED` because a
   private crate-root item is visible to descendant modules, which avoids being the first to open a test
   module in the bin's root. Either placement satisfies the ACs; the ledger's test paths assume the
   former. Follow `xtask/src/lint_constitution.rs:827+` for naming — a test name that states the claim.
7. **The `docs/README.md` edit is two edits, not one.** The new table above the existing one, and the
   `:25-29` paragraph. The paragraph is the easy one to forget and the one that makes the file false.
8. **The limits section is written as the modules are written, not after.** It goes first in the module
   docs (`xtask/src/lint_constitution.rs:9-13`'s reason). Limit 5 must say the `RUSTDOCFLAGS` behaviour
   through the extra `cargo run` hop is **unmeasured here** — do not copy `constitution.rs`'s finding or
   the upstream issue's claim; measuring it is `documented-blind-spots-and-their-proofs`' job.
9. **Then break it on purpose once, locally, to see the shape of the failure** — rename `len` in the
   fence, run the step, revert. This is not AC-003 (that is `observed-failure-falsification`'s recorded
   procedure); it is the sanity check that the banner and the module name read the way the design says.

## Tests and CI (merge gate)

Tiers are this repository's existing four, as the testing brief defines them
(`_decomposition.md` `## Testing brief`, "Acceptance Criteria" preamble): **static** (a lint-class check with
its own `#[cfg(test)]` tests, house shape at `xtask/src/lint_constitution.rs:828-878`), **compile** (a
doctest actually compiled by rustdoc — the mechanism, not a test about it), **gate-integration**
(`cargo xtask ci` or a named subset, run and observed), **end-to-end/fixture** (a deliberately broken page
walked through the real gate).

| tier | command / path | proves |
| --- | --- | --- |
| compile | `cargo test --locked -p xtask --doc -- narrative::` | AC-002, AC-003 — every Rust fence in the tree compiles against the real workspace crates, from the lib target, one named module per page. The mechanism itself. |
| compile | `cargo test --locked -p xtask --doc` (unfiltered) | AC-009 — the constitution's atoms and the repository README's doctest still compile; the new modules did not shadow or displace either. |
| static | `xtask/src/narrative_doctests.rs::tests::{an_empty_listing_is_a_problem, a_listing_with_no_narrative_doctest_is_a_problem, a_listing_with_one_narrative_doctest_is_accepted_and_counted}` via `cargo test -p xtask` | AC-005, AC-003 — zero matching doctests is a hard failure with a named cause; the CR-1 mis-mount is the wrong implementation each rejects (RS-81-4). |
| static | `xtask/src/narrative_doctests.rs::tests::{the_narrative_step_is_required_and_unprobed, the_narrative_step_precedes_the_constitution_step, the_narrative_step_carries_rustdocflags_in_its_own_env, the_narrative_step_passes_locked}` | AC-004 — mandatory, unprobed, `--locked`, step-scoped `RUSTDOCFLAGS`, and ordered so the banner answers one question (RS-80-1/2/3/4; EC-005). |
| static | `xtask/src/narrative_doctests.rs::tests::{the_gate_can_select_the_narrative_step_by_name, the_narrative_step_is_not_a_lint_step}` | AC-006 — the mount is complete and in the right family; a name absent from `REQUIRED` panics in `steps_named`. |
| static | `xtask/src/narrative_doctests.rs::tests::the_fixture_page_carries_no_hidden_marker_and_fits_the_path_budget` | AC-001 — DT-7 compliance on the page this story authors, and the ≤ 32-character repo-relative path that protects the terminal surface. |
| static | `xtask/src/narrative_doctests.rs::tests::{the_index_names_the_narrative_tree_as_gate_read, the_narrative_table_precedes_the_pointer_out_table}` | AC-007 — the index routes to the page, the narrative table is first, and the pin-by-path paragraph is no longer false (EC-008). |
| static | `xtask/src/narrative_doctests.rs::tests::the_new_modules_state_their_limits_first` | AC-008 — the limits section exists and is first, in both new modules (project DoD item 7). |
| static | `cargo xtask lint-constitution` | AC-009 — the existing checker still passes; nothing was refactored into a shared abstraction that made one error message answer two questions (RS-81-3). |
| static | `cargo test -p xtask` (whole package) | AC-009 — `xtask/src/affected.rs`'s `a_docs_only_change_selects_nothing` still passes, because it asserts over `RUNBOOK.md` rather than a `docs/` path; the `INERT` list is untouched here by design. |
| gate-integration | `cargo xtask affected --base main` | AC-009's story grain (`.redkiln/config.yaml:40`). This story's own diff touches `xtask/src/**`, so `xtask` is selected on that basis; the prose-only path is the slice-mate's and is **not** claimed here. |
| gate-integration | `cargo xtask ci --fast` | AC-004, AC-009 — the bar for this non-terminal project (`.redkiln/config.yaml:55`, project DoD item 6). `=== the narrative tree's examples compile ===` appears in the output, above the constitution's banner, with no `skipped` line. Output recorded in the implementation report. |
| gate-integration | `cargo xtask narrative-doctests` | AC-006 — the subcommand runs the same check by hand. |
| gate-integration | `cargo xtask ci` | AC-009 — the full gate, run from a clean checkout before the project is called done (project DoD item 1 and 6). |
| end-to-end/fixture | rename `MemoryEventStore::len`, run the compile step, revert (recorded, not automated) | AC-002 — the fence resolves against the real crate rather than against a copy. The **recorded** observed-fail/observed-recover procedure over `cargo xtask ci` is `observed-failure-falsification`'s AC-003 and is deliberately not duplicated here. |

Merge order: this story is **first** in the `compiled-narrative-tree` slice and first in the project
(`_storymap.md` `## Merge order`). Nothing else in the project can be observed until this step is green.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation in this PR |
| --- | --- | --- |
| **The harness is declared from the bin crate** (`xtask/src/main.rs`) and nothing ever compiles a fence, while everything is green. | Medium / **Critical** — the exact silent-pass shape BR-02 exists to prevent, and the architecture brief's named most-likely error. | AC-003 plus AC-005's guard, which is the only mechanism in the project that can see it. Step 3 of the implementation notes verifies the mechanism before the gate step exists. |
| **A green step that compiled nothing.** A filtered `cargo test --doc` exits 0 over `running 0 tests`. | Medium / **Critical** — `RUNBOOK.md:918-925` is the in-house instance. | AC-005: assert doctests out of `--list` before running them, print the count. RS-81-4. |
| **Failure attributed to the wrong banner.** The constitution's step is unfiltered, so it also compiles the narrative fences. | High if ordering is not deliberate / Medium. | AC-004's ordering assertion (EC-005). Explicitly **not** mitigated by filtering the constitution step, which would drop the repository README's doctest out of the gate. |
| **The reported file is the harness, not the markdown.** A reader of a failure sees `xtask/src/narrative.rs` and a module name. | Certain / Medium — a residual, not a bug. | Recorded as limit 3 in AC-008 rather than routed around; one module per page is what keeps the *module* the page's name and the *line* relative to the page. The charter's risk table already carries it. |
| **A green gate is read as evidence that the documentation teaches.** | Medium / **Highest in the initiative**, and this project is the one most able to cause it. | AC-008's unhedged sentence, DoD item 8's review check on every piece of prose this PR writes, and the executive summary's own disclaimer. HS-P0024's friction log is not substitutable. |
| **Scope creep into the checker.** The registration check, the fence walk, the pinned constants and the empty-tree `bail!` are all one story away and all tempting while the file is open. | High / Medium — two versions of one check in the tree. | The PR boundary's "explicitly not in this PR" list, and AC-005's own note that the page↔module comparison is `narrative-checker-mounted-with-pinned-path`'s. |
| **`docs/` stays inert to `affected`,** so a later prose-only PR runs a story gate that compiles nothing. | Certain until the slice-mate lands / High. | Landed by `narrative-tree-story-grain-selection` **in the same context, immediately after this story**. This PR must not pre-empt it and must not break `a_docs_only_change_selects_nothing`. |
| **A second render creeps in** — a `book.toml`, a stylesheet, a `book/` directory — reopening the divergence D1 closed. | Low / High. | `_design.md` anti-pattern 10, restated in the PR boundary and asserted by AC-009's diff check. |
| **Reverting this story alone after the corpus lands** leaves real pages unchecked. | Low / High. | Recorded in "Data and migrations"; the DAG's ordering exists to prevent the retro-fitted check. |

Coupling is narrow and all of it is inside `xtask` plus `docs/`: no crate under `crates/` is touched, no
port, no feature, no target. The one cross-story coupling that is load-bearing is the slice-mate above.

## Dependencies

**Blocks on:** nothing. `depends_on: []` — this story is first in merge order in the project and the
project itself has no inbound edge (`_storymap.md` `## Merge order`;
`.bklg/docs-that-teach/_decomposition.md` "Dependency graph"; `project.md` `## Dependencies`).

**Unlocks, by story slug:**

- `narrative-tree-story-grain-selection` — slice-mate, lands immediately after this in the same context.
- `narrative-checker-mounted-with-pinned-path` — needs a tree and a harness to pin and cross-check.
- `observed-failure-falsification` — needs the fixture page and the assembled step to break.
- Transitively, through those two: `fence-discipline-and-allowance-list`, `hidden-content-resolution`,
  `documented-blind-spots-and-their-proofs`.
- `spec-trace-clause-id-accessor` and its two consumers are independent of this story
  (`_storymap.md`, milestone `specification-pin`) and are **not** unlocked by it.

## Anchors (progressive disclosure)

Open these when the row says to, not before. Every path was confirmed to exist in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | The signed-off design (2026-08-17, no conditions). `## The doctest` is the fixture page written out verbatim; `## Composition`, `## Density budget`, `## Transience policy`, `## Hierarchy` and `## Anti-patterns` are the binding form decisions, including the two the design gate re-took (H1 ≤ 40, path ≤ 32 repo-relative). | **Before writing `docs/append-conditions.md`** — step 1 of the implementation notes. Re-open before the story's review gate to read AC-001 and AC-007 against it, because no perceptual harness ever will. | AC-001 |
| `xtask/src/constitution.rs` | The exact mechanism to copy, and the argument for one `#[cfg(doctest)] mod` per file at `:11-18` — the concatenated-doc-string trap that makes a failure unmappable. `:20-36` is also the worked example of documenting a limit no test can back. | **Before writing `xtask/src/narrative.rs`** — step 2. | AC-002, AC-008 |
| `xtask/src/lib.rs` | The doctest root. `:21` is the repository README's own `cfg_attr(doctest, doc = include_str!)`; `:28` is `mod constitution;`, the sibling `mod narrative;` joins. `:8-16` states why these cannot live in a published crate — the same reasoning that rejected D1's second option. | **Before adding the `mod` declaration** — step 2. Re-read if tempted to move the tree under `crates/`. | AC-003 |
| `xtask/src/proof.rs` | `:191-240` is the assert-names-out-of-`--list` shape to copy, including the `.with_context()` on a failed enumeration (EC-004) and why the step is spawned as `cargo run -p xtask` so the `Step`'s `env` is inherited. `:9-23` is the argument at length: "a step that a *deletion* fails and an *emptying* passes is checking the filename." | **Before writing the vacuity guard** — step 4. | AC-005 |
| `standards/rust/81-checks-that-cannot-be-types.md` | RS-81-4 at `:264-272` is the rule the guard discharges (a zero exit status is evidence of nothing); RS-81-1 at `:11` is the rule AC-008 discharges (prove the blind spot, then state it). | **With the guard (step 4) and with the limits section (step 8).** | AC-005, AC-008 |
| `standards/rust/80-the-gate.md` | RS-80-1 `:11` (a check is a `Step` in `REQUIRED`, reachable by name), RS-80-2 `:98` (a probe means skip-when-absent, which is why `None`), RS-80-3 `:181` (`RUSTDOCFLAGS` in that step's own `env`), RS-80-4 `:245` (`--locked`). All four are asserted by AC-004's tests. | **Before writing the `Step`** — step 5. | AC-004 |
| `xtask/src/main.rs` | All four mount points and the primitives: `Step` and the `probe`/`env` doc comments at `:72-103`, `REQUIRED` at `:105`, the claim-sentence naming convention at `:466`, the constitution step at `:488-493` (what the new step goes immediately before), the dispatch arms at `:642-707`, `print_help` at `:718+`, `lint_steps` at `:799-808`, `steps_named`'s panic at `:816-826`, `run_fast` at `:853-860`, the banner and the bail at `:862-892`. | **Step 5, and again when writing AC-004's and AC-006's tests.** | AC-004, AC-006 |
| `docs/README.md` | The file being extended: `:12-23` is the existing two-column pointer-out table, `:25-29` is the paragraph that becomes false (EC-008). Also the verified `Routing table` primitive in `_design.md`'s opening table. | **Step 7**, and before writing AC-007's two tests. | AC-007 |
| `xtask/src/lint_constitution.rs` | `:9-13` is the "what this does not verify" opening and its reason ("a check whose limits are undocumented is read as a guarantee"); `:424-425` is the precedent for a check reading `xtask/src/` source **as text**, which AC-008's and AC-007's tests rely on; `:827-878` is the house test shape and naming. | **Step 8, and when naming the unit tests** (step 6). | AC-008, AC-007 |
| `crates/happenstance-core/src/lib.rs` | `:122` is the `pub use memory::{MemoryEventStore, …}` re-export — the reason the fixture fence says `happenstance_core::MemoryEventStore` and not `…::memory::…` (mock finding 1: the earlier spelling did not compile, because `memory` is private at `:103`). | **While writing the fixture fence** — step 1. | AC-001, AC-002 |
| `crates/happenstance-core/src/memory.rs` | The real public items the fence calls: `::new` at `:87`, `::len` at `:171`. These are what makes AC-002's "compiled against the real crates" true rather than asserted, and `len` is the item to rename in the local break-it-once check. | **Step 1 and step 9.** | AC-002 |
| `xtask/Cargo.toml` | `:22-33` is the dev-dependency set every fence resolves against (`happenstance-core` with `memory` on — without that feature the fixture fence does not compile). `:16-21` is DR-12's standing trade, the precedent behind NF-001. | **Before adding anything to a fence that is not already in this list** — and to confirm nothing needs adding. | AC-009 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` | The architecture brief's Note 1 (CR-1/CR-3/CR-4, the two composition roots and the named most-likely error), Note 3 (the mechanism and the two-banner argument), Note 10 (the six-item limits list AC-008 seeds); the testing brief's AC-002/AC-009 rows (the tier definitions and the three invocation paths); the deployment brief's Option A and "Rollback posture". | **At the start**, and again at step 8 for Note 10's list. | AC-002, AC-005, AC-008, AC-009 |
| `xtask/src/affected.rs` | `:248-266` is the `INERT` list with `docs/` on it, and the doc comment above it explaining why `standards/rust/` is deliberately absent — the same argument the slice-mate will make for `docs/`. `:651` is `a_docs_only_change_selects_nothing`, which asserts over `RUNBOOK.md` and therefore must keep passing here. | **Only to confirm this PR does not touch it.** Open before writing any selection logic — and then stop, because that is the slice-mate's story. | AC-009 |
| `RUNBOOK.md` | `:918-925` is the in-house precedent this whole story exists not to repeat: a gate step wired, vouched for by two documents, printing `skipped` on all three runners. It is the reason `probe: None` and the vacuity guard are both non-negotiable. | **When tempted to accept a step that can skip, or a filtered `cargo test` with no assertion.** | AC-004, AC-005 |
| `spec/SPECIFICATION.md` | `:4351-4358` is ES-40, the clause the fixture page cites inline. Read it to understand the flagged residual at the end of "Behavior and interfaces" — the sentence describes the boundary property argued at `:4293`, while ES-40 itself is about completeness. The id resolves either way; reproduce the fixture as signed off. | **While writing the fixture's claim sentence** — step 1. | AC-001 |
| `.redkiln/config.yaml` | `:40` wires `affected_gate` as this story's story-grain check and `:55`/`:60` wire `cargo xtask ci --fast` and `cargo xtask ci` as the integration bars — the three invocation paths AC-009 is measured on. Also the verified absence of a `design:` block, which is why AC-001/AC-007's review is the only perceptual gate. | **Before claiming the gate-integration rows of the tests table.** | AC-009 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The three personas and the four journeys the acceptance criteria are framed from; the initiative carries them here because `.kb/product/` holds no persona atom yet (`initiative.md:275-288`). Read it before rewording any AC away from a person's intent. | **Only if an AC's framing is questioned** — the distilled intent is already inline in the criteria table. | AC-001, AC-002, AC-007 |

## Clarifications resolved during spec

1. **The nine AC ids the front half decided are exactly the nine enumerated here** — AC-001 through
   AC-009, in the order the "Behavior and interfaces" rows already declared them. None was added, dropped
   or renumbered, and the ledger carries one row per id.
2. **AC-001 gets a machine assertion, narrowly scoped, rather than being a review-only row.** The design
   deliberately leaves the H1 cap and the 250-line cap as *review* rules (`_design.md` `## Open questions`
   item 2: enforcing them would be this project taking HS-P0021's job), while making the **path budget**
   and the **hidden-marker ban** mechanical. So AC-001's test asserts only those two, on the single fixture
   page this story authors, by path literal. The tree-wide sweep with the `TREE`/`HIDDEN_MARKERS` constants
   is `narrative-checker-mounted-with-pinned-path`'s and `hidden-content-resolution`'s; this test is
   expected to become **redundant** to them — redundant, not wrong — and the alternative was a story that
   creates the artifact and verifies none of it.
3. **The step's vacuity guard is an addition to the design's `## Items` block, stated rather than
   smuggled.** `_design.md` specifies the checker's four constants and `clause_ids`, and never specifies
   this step's argv. The guard is added because the alternative — a filtered `cargo test --doc` with no
   assertion — is `RUNBOOK.md:918-925` exactly. It deliberately does **not** compare pages on disk against
   modules in the harness; that comparison is AC-005 of the checker story and belongs in one place.
4. **The tests all live in the bin crate.** `xtask/src/main.rs` has no `#[cfg(test)]` module today, and
   `crate::REQUIRED` is reachable from a descendant module because a private crate-root item is visible to
   descendants. The ledger's `verifying_test` paths assume `xtask/src/narrative_doctests.rs::tests::…`;
   moving them into `main.rs` would satisfy the same ACs and requires updating the ledger's paths, not the
   criteria.
5. **Anti-patterns 7 and 8 are not this story's.** They constrain the *checker's* composed failure report
   (`path:line` first, no truncated list). This step's failure body is rustdoc's own doctest report, which
   this story neither composes nor may reformat. They are bound to
   `narrative-checker-mounted-with-pinned-path` instead, and the Interaction quality table says so.
6. **AC-009 claims two of the three invocation paths, not three.** `cargo xtask ci` and
   `cargo xtask ci --fast` are both reached because `run_fast` runs the whole `REQUIRED` array unfiltered
   (`xtask/src/main.rs:853-860`). The `cargo xtask affected --base main` path *after a prose-only change*
   is the slice-mate's, and this story's own `affected_gate` passes on the strength of its `xtask/src/**`
   edits. Claiming all three here would let the highest-value integration gap in the project be signed off
   by a story that did not close it.
7. **The `RUSTDOCFLAGS` limit is recorded as unmeasured, not inherited.** The step spawns cargo through an
   extra `cargo run -p xtask` hop, and what `-D warnings` actually enforces inside a narrative fence
   through that hop has not been measured. Copying `xtask/src/constitution.rs`'s finding or the upstream
   issue's claim would be asserting a measurement nobody re-ran; the probe is
   `documented-blind-spots-and-their-proofs`'.
8. **The fixture's ES-40 citation is reproduced as signed off, with the mismatch recorded.** The claim
   sentence describes the boundary property argued at `spec/SPECIFICATION.md:4293`, while ES-40 itself
   (`:4351-4358`) is about completeness. The id resolves, so nothing in this milestone or in
   `narrative-citation-resolution` fails. If the citation is judged mismatched, the correction is an
   amendment to `_design.md` `## The doctest` by its owner — not a silent edit here.
9. **No ADR is written or needed.** The grounding pass verified that no Accepted decision atom under
   `.kb/decisions/` governs gate structure, documentation trees or fence compiling
   (`_grounding.md`, "Precedence and non-goals"; the story map records the same finding). The two
   divergences from in-repo *precedent* in this project are conventions, discharged by a sentence in the
   new modules' own docs.
