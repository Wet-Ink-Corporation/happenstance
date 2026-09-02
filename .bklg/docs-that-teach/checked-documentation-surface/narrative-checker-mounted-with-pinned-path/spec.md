---
item: HS-S0138
stage: spec
created: 2026-08-17T13:16:02.135Z
updated: 2026-08-17T13:16:02.135Z
template_sig: 87bbf1d0
rendered_sig: 2748ee74
---

# Spec — The checker is mounted, the tree is pinned by constant, and no page is an orphan

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project charter | `.bklg/docs-that-teach/checked-documentation-surface/project.md` |
| This spec | `.bklg/docs-that-teach/checked-documentation-surface/narrative-checker-mounted-with-pinned-path/spec.md` |
| Key briefs | `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` — architecture Notes 1 (CR-2/CR-3/CR-4/CR-5), 2, 4, 9, 10; testing brief AC-001/AC-005/AC-009 rows |
| Signed-off design (**binding**) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `## Items`, `## Signatures`, `## Surfaces`, `## Composition`, `## Transience policy`, `## Density budget`, `## Hierarchy`, `## States`, `## Placement and re-export`, `## Anti-patterns` |
| Story map / merge order | `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` — milestone `narrative-checker-discipline`, story 3 of 5 in merge order |
| Grounding | `.bklg/docs-that-teach/checked-documentation-surface/_grounding.md` — "Precedence and non-goals": **no Accepted decision atom governs gate structure, documentation trees or fence compiling** |

## One-line PR slice

Add the bin-crate checker module holding the tree and harness paths as `const`s, erroring by
name on a missing tree and `bail!`ing on an empty one, cross-checking page↔harness
registration in both directions, and mount it as its own `REQUIRED` step, subcommand, help
line and `lint_steps` member.

## Executive summary

Milestone 1 (`pinned-narrative-tree-and-compiling-step`) put the tree on disk and made rustdoc
compile its fences through `xtask`'s **lib** target. That leaves the hole the whole project
was scoped around: `cfg(doctest)` means a page nobody registered, and a registration nobody
deleted, are both invisible to every step except `cargo test` — and even there, invisible in
the direction that matters, because a page the harness never `include_str!`s produces no
failure at all, only silence (`xtask/src/constitution.rs:20-25`).

This PR lands the second half of the machine: a **bin-crate** checker that reads the tree and
the harness *as files*, and a second `REQUIRED` step that runs it. The delta over milestone 1
is four facts the compiler cannot state — the tree's path is a `const` and a missing tree is
an error naming it, an *empty* tree is a hard error rather than a green run, every page is
registered, and every registration names a page that exists — plus the mount that makes the
checker reachable from `cargo xtask ci`, `cargo xtask ci --fast`, `cargo xtask lints` and
`cargo xtask affected --base main`.

It also lands the shape its two slice-mates extend: one accumulating `Vec<String>` of
problems, one `bail!` carrying the count, and a "What this does not verify" section at the top
of the module docs. `fence-discipline-and-allowance-list` and `hidden-content-resolution` add
checks *into* this module; they do not create a second one.

## Context pack

Everything in this section is a decision already taken. Honor it; do not re-decide it.

**`xtask` has two targets that do not share modules, and mounting this in the wrong one ships
a decorative check.** The compiling harness is `xtask/src/narrative.rs`, declared from
`xtask/src/lib.rs` (which today declares exactly `mod constitution;` at `:28`) — that is
milestone 1's, already landed. **This story's checker is a `mod` in the bin crate, declared
alongside `mod lint_constitution;` at `xtask/src/main.rs:65`.** The two never link: the
checker reads the harness with `fs::read_to_string(root.join(HARNESS))` and matches literal
text, exactly as `check_harness` reads `xtask/src/constitution.rs`
(`xtask/src/lint_constitution.rs:423-425`). Getting this backwards is the architecture brief's
named most-likely error (Note 1, CR-1/CR-2), and a `mod` added to `lib.rs` by mistake would
compile clean and check nothing.

**Names are decided, because other code depends on them by value.** The module is
`lint_narrative`, file `xtask/src/lint_narrative.rs`, following `lint_constitution`'s
precedent — it cannot be `xtask/src/narrative.rs`, because `_design.md`'s `## Signatures`
pins that path as the *harness* the checker reads as text. The subcommand is `narrative` and
the step name is the claim sentence **`every narrative page is checked`**: both are
`_design.md` decisions (`## Surfaces`, `gate-narrative-checker-step`; and mock finding 6,
disposed at the design gate — a noun phrase reads as a category, and the banner is the only
thing telling a reader which half failed). The asymmetry between module `lint_narrative` and
subcommand `narrative` is therefore deliberate and signed off; note it in the module docs
rather than "fixing" it.

**Pinning is two mechanisms, and one without the other passes green over an emptied tree.**
The tree's path is a `const TREE: &str = "docs";` in the shape of `ATOM_DIR` / `ROUTER` /
`HARNESS` (`xtask/src/lint_constitution.rs:55-61`). The directory read is `?`-propagated with
`.with_context()` so a missing tree is an error naming the expected path, **and** an empty
tree is a `bail!` before any check runs, mirroring
`bail!("{ATOM_DIR} holds no atoms, so every check below is vacuous")`
(`xtask/src/lint_constitution.rs:176`). A vacuous green is the exact failure this project
exists to refuse (`_design.md`, `## States`, "Empty").

**The registration check is bidirectional, and the reverse half is the one that earns its
keep.** Forward: every page under `TREE` has both an `include_str!("../../{TREE}/{page}")`
and a `mod {module} {` in the harness. Reverse: every `mod` line in the harness names a page
that still exists. `check_harness` already writes both, with the asymmetry documented at
`xtask/src/lint_constitution.rs:445-446` — `cfg(doctest)` hides a module left behind by a
renamed page from every step but `cargo test`. A checker that implements only the forward
direction is the plausible wrong implementation CLAUDE.md's rule asks to be named, and the
testing brief requires a test for each direction.

**Both directions must agree by construction, so the module-name derivation is one pure
function.** `atoms()` derives `NN-slug.md` → `nn_slug` (`xtask/src/lint_constitution.rs:208`).
Page naming was left free to the implementer (architecture brief Note 9) and is **decided
here**: repo-relative path under `TREE`, minus the `.md`, with `/` and `-` mapped to `_` —
`append-conditions.md` → `append_conditions`, `adapters/sqlite.md` → `adapters_sqlite`. One
`fn` computes it and both directions call it; two spellings of the same derivation is how the
two halves of an orphan check start disagreeing.

**Fatal from day one, not report-only.** `probe: None` means mandatory, and the distinction
the `Step` contract draws is normative: "the tool is missing" is a skip, "the tool ran and
found a problem" is a failure (`xtask/src/main.rs:89-102`, RS-80-1/RS-80-2). The in-house
precedent for the alternative is `RUNBOOK.md:918-925` — a probe-gated step that printed
`skipped` on all three runners while two documents vouched for it. `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:50-54`
refuses report-only for the same reason ("a warning inside a green run is invisible within a
week"); its ratchet/exemption-list machinery is **not** owed here, because the tree as
milestone 1 left it already conforms — the one exemption list in this project is
`IGNORE_ALLOWANCES`, and it belongs to the slice-mate.

**Mounting is five sites, not one, and `steps_named` is the tripwire.** A `Step` in `REQUIRED`
(`xtask/src/main.rs:105`), a `mod` in the bin module list (`:65`), a dispatch arm mirroring
`Some("lint-constitution") => …` (`:689`), a line in `print_help()` (`:718`), and membership
in `lint_steps()` (`:799`). `steps_named` panics on a name absent from `REQUIRED`
(`:816-826`), and that panic is the intended failure — it makes a half-mounted step fail the
moment `cargo xtask lints` selects it instead of going quietly missing.

**The checker joins `affected::run`'s unconditional list, and that is a deliberate divergence
from precedent.** `affected::run` runs seven file-reading checks before any package selection
(`xtask/src/affected.rs:118-125`); `lint_constitution` is conspicuously *not* among them. The
recommendation the architecture brief takes (Note 9) is to add this checker, on the module's
own argument at `xtask/src/affected.rs:28-36`: it is a file read that finishes inside the time
cargo takes to decide `xtask` is up to date, and a prose-only story is exactly the case a
package-shaped gate reads nothing for — `.redkiln/config.yaml:40` wires
`cargo xtask affected --base main` as the *story* grain every later story in this initiative
is gated by. The divergence from `lint-constitution`'s absence is a convention, not a
decision, so it needs no ADR — but it must be stated in this module's own docs, or the next
contributor reads two inconsistent precedents and assumes one is a slip.

**The output is a composed surface, and it is signed off.** Accumulate every problem, print
in source order (path then line), count last:
`bail!("{} problem(s) in {TREE}", problems.len())` — the shape of
`xtask/src/lint_constitution.rs:190-199`. Never fail fast: "a check that stops at the first
problem turns one review cycle into six"
(`.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md:218-219`). Never
truncate: `_design.md`'s anti-pattern 8 forbids any "… and N more". A green run prints exactly
one line, `  {n} pages, all consistent` — because "a green check that says nothing is
indistinguishable from a check that did not run … and a green check that says ten lines trains
people to skip it" (`_design.md`, `## Transience policy`). No spinner, no per-page progress
line: the banner printed at `xtask/src/main.rs:864` *is* the loading state.

**The density budget is a gate rule here, and only here.** `_design.md` resolved mock finding
3 by making the location-prefix budget ≤ 48 characters *inclusive of the 16-character
`xtask\src\../../` doctest prefix*, i.e. **≤ 32 characters of repo-relative page path**, and
assigned enforcement to "the **checker**, at the pinning check, before any line is emitted"
(`## States`, "Long label"; `## Anti-patterns` 11). The page-length and H1 budgets stay
review rules — enforcing them would put this project inside HS-P0021's page-need discipline
(`_design.md`, `## Open questions` 2). The line between the two is drawn deliberately: this
one protects the terminal surface, not the editorial one.

**The persona slice.** The reader here is a **contributor running the gate** and a **reviewer
reading its output** (`_storymap.md`, preamble). The reader state this story makes true is
"Arriving from a clean clone — read rendered pages without knowing a build step exists"
(`_decomposition.md`, UX brief, "Reader states"), and the falsifier it closes is *a fence
renders on a page and no compiler ever read it* — which, after milestone 1, survives only as
an **unregistered page**. That is this story's whole reason to exist.

**Two obligations every story in this project carries.** The module's docs open with "What
this does not verify", first rather than last, because "a check whose limits are undocumented
is read as a guarantee" (`xtask/src/lint_constitution.rs:9-13`, RS-81-1); this story owes the
section's existence and the limits its own checks create, while
`documented-blind-spots-and-their-proofs` owns the full six-item contents (architecture brief
Note 10). And **nothing here may claim, anywhere, that the surface proves a page teaches** —
project DoD item 8. No badge, no tick, no "verified" (`_design.md`, anti-pattern 9).

**What this story must not touch.** `xtask/src/lint_constitution.rs` and
`xtask/src/constitution.rs` are not refactored to share code with the new checker: RS-81-3
scopes a scanner to the directory whose behaviour it constrains, and a shared abstraction over
two trees makes one error message answer two questions (architecture brief Note 8). The shape
is cheap to copy — copy it. No dependency is added: `xtask/Cargo.toml:16-21` records the
standing trade, and the checks in this story are `read_dir`, `read_to_string` and string
matching over `std` and `anyhow`.

## Integration contract

- **Archetype**: `capability` — observable through `cargo xtask ci`, `cargo xtask ci --fast`,
  `cargo xtask narrative` and `cargo xtask affected --base main`.
- **Slice / milestone**: `narrative-checker-discipline`. Slice-mates, implemented in the same
  context and mounted as one surface: `fence-discipline-and-allowance-list` (the fence walk,
  into the module this story creates) and `hidden-content-resolution` (DT-7 enforced inside
  that same walk). This story is first of the three in merge order (`_storymap.md`, "Merge
  order" 2.3).
- **Mount point**: **`xtask/src/main.rs`** — the bin crate's composition root, at five sites:
  `mod lint_narrative;` beside `mod lint_constitution;` (`:65`); a `Step` in `REQUIRED`
  (`:105`), placed immediately after milestone 1's `the narrative tree's examples compile`
  step so the two banners read compile-then-check in the order `_design.md`'s `## Composition`
  draws them, and adjacent to the constitution's pair at `:462-492`; a dispatch arm mirroring
  `Some("lint-constitution")` (`:689`); a line in `print_help()` (`:718`); and the step's name
  in `lint_steps()` (`:799`).
- **Wires into**:
  - `xtask/src/narrative.rs` — milestone 1's harness, read **as text** (never linked); the
    `include_str!("../../{TREE}/{page}")` and `mod {module} {` literals are the contract.
  - `xtask/src/main.rs:72-103` — the `Step` struct and the normative `probe` doc comment.
  - `xtask/src/main.rs:816-826` — `steps_named`'s panic, which is what makes a half-mount loud.
  - `crate::spec_trace::workspace_root` (`xtask/src/spec_trace.rs:2311`) — how every
    file-reading check in this crate resolves the repository root.
  - `xtask/src/affected.rs:118-125` — the unconditional file-reading list this checker joins.
  - `docs/` — the pinned tree, and `docs/README.md:25-29`, the prose statement of why the gate
    reads trees by path.
  - No workspace crate, no port, no `Send` bound, no feature: ADR-0001 and ADR-0003 are
    untouched by construction (`_design.md`, `## What it costs a caller`).
- **Renders surfaces**: `gate-narrative-checker-step` (`_design.md`, `## Surfaces`) — this
  story creates it, in its `pass`, `fail-one`, `fail-many`, `fail-empty-tree` and
  `fail-long-path` states. `fail-hidden-marker` is `hidden-content-resolution`'s state of the
  same surface. `narrative-tree-index` is touched only by the one-sentence correction below;
  `narrative-page` and `narrative-scoped-page` are not rendered by this story.
- **Public items** (`_design.md`, `## Items`): `TREE` and `HARNESS`. Their design paths read
  `xtask::narrative::*`; their target-resolved home is `xtask::lint_narrative::*`, because
  `HARNESS`'s own value names `xtask/src/narrative.rs` as a file this module reads as text and
  "the two never link" (`_design.md`, `## Placement and re-export`). `IGNORE_ALLOWANCES` and
  `HIDDEN_MARKERS` are the slice-mates'; `xtask::spec_trace::clause_ids` is milestone 3's.
- **Conformance rule(s)**: none, and this is not adapter-observable. Nothing in this story
  touches `crates/happenstance-testkit/`, a port, a value type or a fixture; the instrument is
  the bin crate's own `#[cfg(test)] mod tests` plus the gate's own output. Stated explicitly
  because a story that changes a port and names no rule is a port change nothing can fail —
  this changes no port.
- **Clause(s)**: none discharged, none amended. This story reads no clause id; citation
  resolution is milestone 3's (`narrative-citation-resolution`, `frozen-documentation-must-pin`).
  Nothing here amends, discharges or restates a `SPECIFICATION.md` clause (architecture brief
  Note 8), and no `[FROZEN]` clause is touched, so no ADR is owed.
- **Advances DoD scenario**: initiative DoD **1** — *"@smoke — the teaching survives a clean
  checkout"*: after this PR the tree is read by a mandatory, probe-less step from a clean
  checkout with no tool to install and no manual step. It also makes DoD **2** reachable: a
  page can only be *broken and caught by name* if it cannot silently escape the check, which
  is what the orphan check delivers — the observation itself belongs to
  `observed-failure-falsification`.

## PR boundary

```
xtask/src/lint_narrative.rs
xtask/src/main.rs
xtask/src/affected.rs
xtask/src/narrative.rs
docs/README.md
standards/rust/**
.bklg/docs-that-teach/checked-documentation-surface/narrative-checker-mounted-with-pinned-path/**
```

> **Amended 2026-08-18, after implementation.** `standards/rust/**` was added
> because it was missing, not to clear a gate. This story mounts a second
> `REQUIRED` step in `xtask/src/main.rs`, which again shifts every line number
> the constitution's **Evidence** lines cite into that file, and
> `cargo xtask lint-constitution` fails until they are re-pointed. `b62de17`
> therefore carries the same 8 lines of citation repair across
> `standards/rust/{51-features-and-no-std,52-wasm32-and-target-cfg,70-rustdoc-obligations,80-the-gate}.md`
> that `pinned-narrative-tree-and-compiling-step` carried for its own mount. The
> omission was an oversight in the spec and went unreported until `redkiln`
> 0.19.0 scoped the boundary check to a story's own commits (redkiln #94). No
> claim, rule or example in any atom was changed — only the line numbers its
> Evidence lines point at.

**In this PR**

- `xtask/src/lint_narrative.rs` — new bin-crate module: `TREE`, `HARNESS`, the page
  enumeration with its `.with_context()` and vacuity guard, the pure module-name derivation,
  the bidirectional registration check, the path-budget check, the accumulate-all reporting,
  the "What this does not verify" section, and `#[cfg(test)] mod tests` in the house shape
  (`xtask/src/lint_constitution.rs:828-878`).
- `xtask/src/main.rs` — the five mount sites named in the Integration contract. This is
  composition-root wiring and is the point of the story, not scope drift.
- `xtask/src/affected.rs` — one call added to the unconditional file-reading list at
  `:118-125`, with the divergence from `lint-constitution`'s absence recorded in this module's
  docs. The selection arm at `:212-224` is **not** touched here — that is
  `narrative-tree-story-grain-selection`'s, landed in milestone 1.
- `xtask/src/narrative.rs` — **module names only**, and only if milestone 1's hand-written
  `mod`s disagree with the derivation decided above. No page is added, removed or re-included
  by this story.
- `docs/README.md` — one sentence at `:25-29`, naming `cargo xtask narrative` as a second
  reader of the tree, so the paragraph stays true. Skip it if milestone 1's landing already
  names this subcommand; the narrative routing table's rows are milestone 1's, not this
  story's.
- This story's own backlog folder: the `_ledger.md` and the implementation report.

**Explicitly not in this PR**

- The fence walk: untagged fences, exhaustive info-string matching, `IGNORE_ALLOWANCES` and
  its reverse sweep (`fence-discipline-and-allowance-list`).
- `HIDDEN_MARKERS` and DT-7's enforcement (`hidden-content-resolution`).
- `clause_ids`, citation resolution and the frozen-MUST pin (milestone `specification-pin`).
- Any observed-failure run or recorded gate transcript (`observed-failure-falsification`), and
  the six-item contents of the limits section (`documented-blind-spots-and-their-proofs`).
- Any teaching page. Fixture material is milestone 1's; the corpus is HS-P0021/22/23's
  (`project.md`, risk table).
- Any refactor of `xtask/src/lint_constitution.rs`, `xtask/src/constitution.rs` or
  `xtask/src/spec_trace.rs`, and any new dependency in `xtask/Cargo.toml`.
- Any edit to `spec/SPECIFICATION.md`, `standards/rust/`, or `.kb/`.

**Merge DoD**: `cargo xtask ci --fast` is green (`.redkiln/config.yaml:55`), `cargo xtask
narrative` passes standalone over the tree as milestone 1 left it printing exactly one summary
line, `cargo xtask lints` includes the new step by name, `cargo xtask affected --base main`
runs the checker unconditionally on a prose-only diff, and `cargo test -p xtask` covers both
directions of the registration check and both pinning failures.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The tree is a `const`, not a convention | `const TREE: &str = "docs";` in `lint_narrative`, in the shape of `ATOM_DIR`/`ROUTER`/`HARNESS`. Moving or renaming the tree without editing this line fails the gate. | `_design.md` `## Signatures`; `xtask/src/lint_constitution.rs:55-61`; `docs/README.md:25-29` |
| A missing tree errors by name | `fs::read_dir(root.join(TREE)).with_context(\|\| format!("reading {TREE}"))?` — `?`-propagated, so the message names the expected path rather than reporting zero pages. | `xtask/src/lint_constitution.rs:208-215`; architecture brief AC-001 |
| An empty tree is a hard error | `bail!("{TREE} holds no pages, so every check below is vacuous")` **before** any check runs. A pinned constant without this guard passes green over a tree someone emptied. | `xtask/src/lint_constitution.rs:176`; `_design.md` `## States` ("Empty") |
| The harness is read as text, never linked | `fs::read_to_string(root.join(HARNESS))` with `HARNESS = "xtask/src/narrative.rs"`; the checker matches the two literals `include_str!("../../{TREE}/{page}")` and `mod {module} {`. This is how a bin-crate check proves a fact about a lib-crate file. | `xtask/src/lint_constitution.rs:423-425`; `_design.md` `## Placement and re-export`; architecture brief Note 2 |
| One pure module-name derivation, shared by both directions | `fn module_name(rel: &str) -> String`: strip `.md`, map `/` and `-` to `_`. `adapters/sqlite.md` → `adapters_sqlite`. Decided here; Note 9 left page naming free, Note 2 requires the derivation be pure and shared. | architecture brief Notes 2 and 9; `xtask/src/lint_constitution.rs:208-246` |
| Forward registration check | For every page under `TREE`: a missing `include_str!` is a problem saying its examples are never compiled; a missing `mod` is a problem saying one module per page is what keeps a doctest failure's line number relative to the page. | `xtask/src/lint_constitution.rs:428-442` |
| Reverse registration check | For every `mod` line in the harness whose derived name matches no page: a problem. `cfg(doctest)` hides a module left behind by a renamed page from every step but `cargo test`, so this half is the one that earns its keep — and it is unreachable from a real gate run once tree and harness agree, which is why it is tested against a harness string. | `xtask/src/lint_constitution.rs:445-458`; testing brief AC-005 |
| Path budget enforced at the pinning check | A page whose repo-relative path exceeds **32 characters**, or that nests a third directory level under `TREE`, is a problem — the location-prefix budget of ≤ 48 characters inclusive of the 16-character `xtask\src\../../` doctest prefix. Enforced because it protects the terminal surface; the page-length and H1 budgets stay review rules. | `_design.md` `## Density budget` (finding 3 disposition), `## States` ("Long label"), `## Anti-patterns` 11, `## Open questions` 2 |
| I/O at the edges, checks pure | Two functions touch the filesystem (page enumeration, harness read); every check is a pure function over `(&[Page], &str, &mut Vec<String>)` and the vacuity guard is pure over `&[Page]`. This is what makes the house test style reachable — `xtask` has no `tempfile` dev-dependency and DR-12 forbids adding one for a lint. | `xtask/Cargo.toml:16-33`; `xtask/src/lint_constitution.rs:828-878` |
| Failure report: all problems, source order, count last | Problems accumulate in one `Vec<String>`, print to stderr with a two-space indent as `{path}:{line} — {message}` (or `{path} — {message}` where no line is meaningful, the harness-registration form), then `bail!("{} problem(s) in {TREE}")`. Never fail fast, never truncate. | `xtask/src/lint_constitution.rs:190-199`, `:428-442`; `_decomposition.md:218-219`; `_design.md` `## Composition`, `## Hierarchy`, anti-pattern 8 |
| Success: exactly one line | `println!("  {} pages, all consistent", pages.len())`. Not zero — a silent green is indistinguishable from a check that did not run (`RUNBOOK.md:918-925`); not ten — that trains people to skip the output. | `xtask/src/lint_constitution.rs:192`; `_design.md` `## Transience policy` |
| Mandatory, probe-less step, named as a claim | `Step { name: "every narrative page is checked", program: "cargo", args: &["run", "--locked", "--quiet", "-p", "xtask", "--", "narrative"], env: &[], probe: None }`, placed after milestone 1's compile step. `--locked` because the invocation resolves dependencies. | `xtask/src/main.rs:72-103`, `:105`, `:462-474`; `_design.md` `## Surfaces`, finding 6 disposition; RS-80-1, RS-80-2, RS-80-4 |
| Reachable by name, four ways | Dispatch arm `Some("narrative") => lint_narrative::run()`, a `print_help()` line, membership in `lint_steps()`, and a call in `affected::run`'s unconditional list. `run_fast` runs all of `REQUIRED` unfiltered, so `--fast` reaches it for free. | `xtask/src/main.rs:689`, `:718`, `:799-808`, `:853-860`; `xtask/src/affected.rs:118-125` |
| A half-mount is loud | `steps_named` panics on a name absent from `REQUIRED`, so a `lint_steps()` entry whose string drifts from the `Step`'s name fails the moment `cargo xtask lints` runs rather than silently selecting nothing. | `xtask/src/main.rs:816-826` |
| Limits first, in the module's own docs | `# What this does not verify` opens the module docs. This story owes the section and the limits its own checks create — at minimum that registration proves a page is *compiled*, not that it is *correct*, and that the reported file for a compile failure is the harness rather than the markdown. The full six-item list is `documented-blind-spots-and-their-proofs`'. | `xtask/src/lint_constitution.rs:9-28`; `xtask/src/constitution.rs:20-36`; RS-81-1; architecture brief Note 10; project DoD 7 |
| The `affected::run` divergence is stated, not left implicit | One paragraph in the module docs: this checker is on the unconditional list and `lint-constitution` is not, on `xtask/src/affected.rs:28-36`'s own argument. A convention, not a decision — no ADR is owed, and none is written. | `xtask/src/affected.rs:28-36`; architecture brief Note 9; `_grounding.md` "Precedence and non-goals" |
| No claim of teaching, anywhere | No badge, tick, shield or "verified" mark; no sentence in code, docs or output implying the check proves comprehension. | project DoD 8; `_design.md` anti-pattern 9 |

## Data and migrations

**N/A.** This story adds no schema, no store, no serialised format and no persisted state; it
reads two files and writes lines to stdout/stderr. Nothing here touches `happenstance-core`,
`serde`, an adapter or a projection store.

Two things are nonetheless *contracts by value* and behave like a migration if changed
carelessly, so they are recorded here rather than discovered later:

- **`TREE`'s value.** It is a repository-wide contract, not a private detail: moving `docs/`
  without editing that line fails the gate (`_design.md` `## Visibility and stability`). The
  paragraph at `docs/README.md:25-29` is its prose mirror and must stay true.
- **The harness text literals.** `include_str!("../../{TREE}/{page}")` and `mod {module} {`
  are matched as text. Reformatting `xtask/src/narrative.rs` so a `mod` line no longer starts
  with `mod ` after trimming, or splitting an `include_str!` across lines, breaks the check
  without breaking the compile — the same coupling `check_harness` already lives with
  (`xtask/src/lint_constitution.rs:428-458`). Record it as a limit rather than defending
  against it with a parser.

## Acceptance criteria

Nine criteria. Each is framed from the intent of one of this project's two readers — the
**contributor running the gate** and the **reviewer reading its output** (`_storymap.md`,
preamble) — standing in for the initiative's three personas, whose journeys all begin at
`docs/` and none of whom ever runs this checker themselves
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`). The reader state
every row below ultimately protects is *"arriving from a clean clone — read rendered pages
without knowing a build step exists"* (`_decomposition.md`, UX brief, "Reader states").
AC-001/AC-002 discharge project AC-001; AC-003/AC-004 discharge project AC-005; AC-005
through AC-009 are the mount and the signed-off surface without which neither is observable.

The tests named as `xtask/src/lint_narrative.rs` do not exist yet — that file is this story's
new module, and its `#[cfg(test)] mod tests` is authored here in the house shape
(`xtask/src/lint_constitution.rs:827-878`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a contributor who has moved or renamed the narrative tree — the way `docs/` itself was once reorganised (`docs/README.md:8-10`) — and has not touched `xtask/src/`, **WHEN** they run `cargo xtask ci`, **THEN** the run fails with a message naming the path the gate expected, so they learn the tree is pinned by a constant rather than discovering weeks later that nothing was being read. A message reporting *zero pages* instead of a missing directory does not satisfy this. | *Static.* A unit test in `xtask/src/lint_narrative.rs` `mod tests` drives the page enumeration at a root with no `TREE` directory and asserts the returned error's `{:#}` chain contains the pinned path — mirroring the shape at `xtask/src/lint_constitution.rs:174-176`. |
| AC-002 | **GIVEN** a contributor whose change has emptied the narrative tree — every page moved, deleted, or relocated under a directory the constant no longer names — **WHEN** the gate runs, **THEN** the step fails before any check executes, saying the tree holds no pages and that every check below would be vacuous, so a green run can never mean "there was nothing to read". A pinned constant with no vacuity guard is the named wrong implementation this row rejects. | *Static.* A unit test asserts the vacuity guard is a hard error over an empty page slice, mirroring `bail!("{ATOM_DIR} holds no atoms, so every check below is vacuous")` (`xtask/src/lint_constitution.rs:176`). A second asserts a one-page slice passes the guard, so the guard is not simply always-failing. |
| AC-003 | **GIVEN** a contributor who has added a markdown page to the tree and forgotten to register it in the harness — the silent-pass shape that survives milestone 1 because `cfg(doctest)` makes an unregistered page produce no failure at all, only silence (`xtask/src/constitution.rs:20-25`) — **WHEN** they run `cargo xtask narrative` or the gate, **THEN** the page is named as a problem, saying its examples are never compiled, so the page cannot ship unchecked. | *Static.* A unit test feeds a page list containing a page and a harness string lacking its `include_str!`, asserts one problem naming the page; a second test omits only the `mod {module} {` line and asserts a distinct problem — the two halves `xtask/src/lint_constitution.rs:428-442` keeps separate. |
| AC-004 | **GIVEN** a contributor who has renamed or deleted a page and left its `mod` behind in the harness, **WHEN** the gate runs, **THEN** the stale registration is reported as a problem naming it — because `cfg(doctest)` hides a leftover module from every step but `cargo test` (`xtask/src/lint_constitution.rs:445-446`), and a reviewer reading the harness cannot tell a live registration from a dead one. A checker implementing only AC-003's direction does not satisfy this row. | *Static.* A unit test feeds a harness string whose `mod` names no existing page and asserts a problem; a companion test asserts the same harness with the page present yields none. This direction is unreachable from a real gate run once tree and harness agree, which is exactly why it is tested against a harness string rather than the tree. |
| AC-005 | **GIVEN** a contributor on a clean clone with no tool installed and nothing to remember, **WHEN** they run any of the four invocation paths this repository actually wires — `cargo xtask ci`, `cargo xtask ci --fast`, `cargo xtask lints`, `cargo xtask affected --base main` — **THEN** the narrative check runs on every one of them, under its own claim-sentence banner `=== every narrative page is checked ===`, and is discoverable by name from `cargo xtask` with no arguments. A step reachable from `ci` but invisible to `--fast` or to the story grain is the failure RS-80-1 names, and `.redkiln/config.yaml:40` makes the story grain the gate every later story in this initiative is actually held to. | *Gate-integration.* `cargo xtask ci --fast` run and the banner observed in its output; `cargo xtask lints` run and the step observed selected by name (a name that drifts from the `Step` makes `steps_named` panic, `xtask/src/main.rs:816-826`); `cargo xtask affected --base main` run on a prose-only diff and the checker observed in the unconditional file-reading block (`xtask/src/affected.rs:118-125`); `cargo xtask narrative` run standalone. Structurally: `probe: None` (`xtask/src/main.rs:89-102`) and a `print_help()` line (`:718`). |
| AC-006 | **GIVEN** a reviewer reading a failing run in a CI log they cannot re-run, with several pages broken at once, **WHEN** the checker fails, **THEN** they get every problem in one run — indented two spaces, in source order (path then line), each line beginning with `{path}:{line}` before an em dash and its message, with the count and the directory last — and never a truncated list. Fail-fast turns one review cycle into six (`_decomposition.md:218-219`); a `… and N more` is `_design.md` anti-pattern 8; a first visual row that does not begin with `path:line` is anti-pattern 7. | *Static.* A unit test asserts that a fixture with three distinct problems (one per check) returns all three, in source order, each matching the composed form `  {path}:{line} — {message}`, and that no message is elided. *Gate-integration:* the standalone `cargo xtask narrative` output is read against `_design.md` `## Composition`'s worked failure report. |
| AC-007 | **GIVEN** a contributor watching a green gate scroll past, **WHEN** the narrative step passes, **THEN** it prints exactly one line — `  {n} pages, all consistent` — and nothing else: no spinner, no dot ticker, no per-page progress line, and never the `skipped:` line. Zero output is indistinguishable from a check that did not run, which is precisely `RUNBOOK.md:918-925`'s failure; ten lines trains people to skip the output (`_design.md` `## Transience policy`). The banner printed at `xtask/src/main.rs:864` is the loading state and no second one is added. | *Static.* A unit test asserts the success path emits exactly one line and that its page count equals the number of pages enumerated. *Gate-integration:* `cargo xtask narrative` over the tree as milestone 1 left it, observed to print one summary line under one banner, with no `skipped:` line anywhere (guaranteed structurally by `probe: None`). |
| AC-008 | **GIVEN** a contributor who has nested a page one directory level too deep, or given it a long filename, **WHEN** the gate runs, **THEN** that page is reported as a problem *before any other line is emitted*, because a page whose repo-relative path exceeds **32 characters** — or that sits at a third directory level under `TREE` — pushes the location off the first visual row of an 80-column log once the 16-character `xtask\src\../../` doctest prefix is counted, starving the surface a reviewer reads (`_design.md` finding 3's disposition; `## Density budget`; anti-pattern 11). The page-length and H1 budgets deliberately stay review rules. | *Static.* Unit tests: a 31-character path passes, a 33-character path is a problem, `docs/a/b/c.md` is a problem, and `docs/adapters/sqlite.md` passes — the four corners of the budget. The budget's arithmetic (≤ 48 inclusive of the 16-character prefix) is restated in the module docs beside the constant. |
| AC-009 | **GIVEN** a contributor or a downstream project reading this new check for the first time, **WHEN** they open the module, **THEN** the *first* thing in its docs is `# What this does not verify`, stating at minimum that registration proves a page is compiled and not that it is correct, and that a compile failure names the harness rather than the markdown; the module also states in one paragraph why it joins `affected::run`'s unconditional list when `lint-constitution` does not; and nowhere in the module, its output, or `docs/README.md` does anything claim the surface proves a page teaches. A check whose limits are undocumented is read as a guarantee (`xtask/src/lint_constitution.rs:9-13`, RS-81-1), and a green step read as evidence of teachability is the initiative's top-ranked risk (`project.md`, risk table). | *Static.* `cargo xtask lint-constitution` and `cargo test --locked -p xtask --doc` still pass, so the docs are compiled rather than merely present. Reviewed by reading: the section is first, not last; the `affected` divergence paragraph exists; and a grep of the diff for `verified`, `badge`, `✓`, `proves that … teaches` returns nothing in a claiming sense (project DoD 8, `_design.md` anti-pattern 9). The six-item contents of the limits section remain `documented-blind-spots-and-their-proofs`'. |

## Interaction quality

This story renders one surface, `gate-narrative-checker-step` (`_design.md` `## Surfaces`),
and its medium is a terminal and a CI log. The invariants below are therefore the terminal
analogues of the state family, plus the composition family taken verbatim from the signed-off
`_design.md`. **Every invariant here is already an `AC-###` row above** — this section only
says which row carries it and how it is verified, because `redkiln verify` extracts ACs from
table cells and bullets in the acceptance-criteria section and a prose bullet here would never
be gated.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump.** The report appears in the same stream the contributor is already reading, under the step's own banner. No side file, no `target/` report, no editor to open, no second command to see the detail. | AC-005, AC-006 | The step's `args` invoke one subcommand that writes to stdout/stderr (`xtask/src/main.rs:880-884` spawns it inheriting the parent's streams); observed in the `cargo xtask ci --fast` run. |
| **Non-occlusion.** Nothing overwrites anything. No carriage-return redraw, no spinner, no cursor repositioning, no ANSI clear — the banner stays visible above the problems and the problems above the count. | AC-006, AC-007 | The success path is asserted to be a single `println!`; the failure path is asserted to emit one line per problem with no control characters. `_design.md` `## States` ("Loading") forbids the ticker explicitly. |
| **Preserved scroll and selection.** The output is append-only, so a contributor's scrollback and any selection they have made survive the run, and the count line is last precisely because it is the line that remains in view when the list is long. | AC-006 | Append-only follows from the two bullets above; the ordering (problems, then count) is asserted by the source-order test and read against `_design.md` `## Composition`. |
| **Reversibility.** Running the checker changes nothing. Unlike `lint_constitution`, which carries a `Mode::Write` arm (`xtask/src/main.rs:689-697`), this module is read-only and takes no `--write` flag — so the surface is safe to invoke at any time and nothing needs undoing. | AC-005, AC-009 | The dispatch arm is `Some("narrative") => lint_narrative::run()` with no flag match; the module opens no file for writing. Stated as a property in the module docs. |
| **Keyboard reachability.** The surface is reached by typing a name — `cargo xtask narrative` — listed in `print_help()`, needing no flag order, no interactive prompt and no TTY. | AC-005 | `cargo xtask` with no argument prints the help containing the subcommand (`xtask/src/main.rs:703-706`, `:718`); the subcommand is run directly. |

**Composition invariants** (from `_design.md`, binding)

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all.** Every problem is a *composed* line — two-space indent, `{path}:{line}`, an em dash, then the message — not a bare `{:?}`, not a raw `anyhow` chain, not an unindented top-level string. The indent is what makes the list read as a body under the banner (`_design.md` `## Hierarchy`). | AC-006 | Unit test matches the composed form character for character against the primitive at `xtask/src/lint_constitution.rs:605-660`. |
| **Composition and placement.** Banner → problems in source order → count carrying the directory. Two steps and two banners, compile then check, so the banner alone says which half failed. | AC-005, AC-006 | Step ordering asserted by position in `REQUIRED` (`xtask/src/main.rs:105`) and observed in the `--fast` run; line ordering by the source-order test. |
| **Transience.** Banner = persistent chrome. Problem lines = opened on demand, by failing — they exist in no other state. Success summary = persistent chrome, one line. Probe skip line = never present. | AC-005, AC-006, AC-007 | `probe: None` makes the skip line unreachable; the success and failure paths are asserted to be mutually exclusive and to emit exactly the shapes above. |
| **Density budget, with its real numbers.** 80-column line; location prefix ≤ **48** characters *inclusive of the 16-character `xtask\src\../../` doctest prefix*, i.e. ≤ **32** characters of repo-relative page path; `docs/` plus ≤ **2** directory levels; filename ≤ **32** characters; problem list **unbounded**; success output **1** line. | AC-006, AC-007, AC-008 | The four-corner path-budget tests; the one-line success test; the no-truncation assertion in the source-order test. |
| **Hierarchy.** Primary is `{path}:{line}`, carried by being first on the line and by the em dash separating it. Secondary is the message. Recessive are the indent, the banner and the count. Nothing is carried by colour, weight or size — none of which this project controls. | AC-006 | The composed-form test pins the ordering within the line; no colour or styling code is added, which is itself the assertion. |
| **Named anti-patterns refused.** 7 — a failure whose first visual row does not begin `path:line`. 8 — any truncated problem list. 9 — any badge, tick, shield or "verified" mark. 11 — a third directory level under `docs/`, or a filename over 32 characters. | AC-006 (7, 8), AC-008 (11), AC-009 (9) | 7 and 8 by the composed-form and no-truncation tests; 11 by the path-budget tests; 9 by review of the diff and of `docs/README.md`, per project DoD 8. |

Two anti-patterns from `_design.md` are **not** this story's to enforce and are named so the
implementer does not reach for them: 1 (a disclosure triangle or tab strip) is
`hidden-content-resolution`'s via `HIDDEN_MARKERS`, and 5 (an untagged or unlisted-`ignore`
fence) is `fence-discipline-and-allowance-list`'s.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | `TREE` does not exist under the workspace root. | `?`-propagated `fs::read_dir` error wrapped with `.with_context()` naming the expected path; non-zero exit; surfaces as `xtask failed: {err:#}` (`xtask/src/main.rs:712`) and then the parent's `bail!("{} failed with {status}")` (`:887`). **Never** "0 pages, all consistent". (AC-001) |
| EC-002 | `TREE` exists and contains no markdown page. | `bail!` naming the tree and saying every check below would be vacuous, **before** any check runs. Not a warning, not a `skipped:` line, not exit 0. (AC-002) |
| EC-003 | `HARNESS` is missing or unreadable. | Hard error naming `HARNESS`'s value. This is not a skip and not a silently-empty harness string: an empty string would report every page as unregistered, which is a confusing failure rather than a wrong one, but it would also report green if the tree were simultaneously empty — EC-002 already forecloses that, and the context message is what makes the real cause legible. (AC-003, AC-004) |
| EC-004 | A page cannot be read, or is not valid UTF-8. | Hard error naming the page, not a skipped page. RS-81-2's posture, one medium over: a scanner that silently skips what it cannot read reports green over exactly the file it failed to inspect (`standards/rust/81-checks-that-cannot-be-types.md:95`). This story's checks do not need page *contents* — the slice-mates' fence walk does — so the read may be deferred to them; if this story reads a page at all, this is the posture. (AC-003) |
| EC-005 | The step's name in `lint_steps()` drifts from the `Step`'s `name` in `REQUIRED`. | `steps_named` panics with `REQUIRED must contain the \`{name}\` step` (`xtask/src/main.rs:816-826`). This is the intended failure and must not be softened to a `find(...).ok()`. (AC-005) |
| EC-006 | One or more problems were found. | Every problem printed, then `bail!("{} problem(s) in {TREE}", problems.len())`. Exit non-zero. The count is the only line guaranteed to be in view after a long list, so it carries the directory. (AC-006) |
| EC-007 | The workspace root cannot be resolved. | Propagated from `crate::spec_trace::workspace_root` (`xtask/src/spec_trace.rs:2311`) unchanged. No new root-finding logic is written; a second way to find the root is a second way to find the wrong one. (AC-001) |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| NF-001 | **Zero new dependencies.** The checks are `read_dir`, `read_to_string` and string matching over `std` plus the `anyhow` already in `xtask/Cargo.toml`. | `xtask/Cargo.toml:16-21` records the standing trade that keeps `rusqlite` and `sqlx` out of the dev graph because every `cargo xtask ci` would build them (DR-12). A markdown parser here would be the same bargain with less to show for it. Held by the diff: no manifest change. |
| NF-002 | **Gate cost stays inside the noise.** One `read_dir`, one file read per page, one harness read, and a line scan — the class `xtask/src/affected.rs:28-36` already argues finishes inside the time cargo takes to decide `xtask` is up to date. No compiler, no network, no process spawn from inside the checker. | This is the whole argument for joining `affected::run`'s unconditional list, so it has to remain true. Held by construction and observed in the `affected --base main` run. |
| NF-003 | **House lint posture unchanged.** No new `#![allow]`, no `unwrap`/`expect` outside `#[cfg(test)]`, and the test module carries the same scoped allow the house style uses (`xtask/src/lint_constitution.rs:829`). | `cargo clippy` with `-D warnings` is a `REQUIRED` step; a scoped allow added to pass it is a silent widening of the constitution. |
| NF-004 | **Platform-neutral paths.** Page paths are reported and compared repo-relative with `/` separators regardless of host, so the same page produces the same problem line, the same module name and the same length measurement on Windows and on CI's Linux. | This repository is developed on Windows (`xtask/src/main.rs:87` says so in the `env` field's own reasoning) and CI is not. A `\`-separated path would silently break the harness literal match and the 32-character budget at the same time. |
| NF-005 | **MSRV, `wasm32`, features and the two binding ADRs untouched.** No `cfg`, no feature, no target, no port, no `Send` bound, no `serde`. | `_design.md` `## What it costs a caller`. ADR-0001 and ADR-0003 are untouched by construction, and stating it is what keeps a reviewer from having to check. |
| NF-006 | **The two constants are documented as contracts, not as details.** `TREE`'s doc comment says that moving the tree without editing the line fails the gate; `HARNESS`'s says it is read as text and never linked. | `_design.md` `## Visibility and stability` makes `TREE`'s *value* a repository-wide contract; `docs/README.md:25-29` is its prose mirror and must stay true (RS-70's obligation applied to a private const is still worth the two lines). |

## Implementation notes (non-prescriptive)

Shape suggestions only. Where one of these disagrees with a section above, the section above
wins.

- **Copy `lint_constitution`'s skeleton, do not abstract over it.** `atoms()` →
  `check_harness()` → accumulate → report (`xtask/src/lint_constitution.rs:169-199`,
  `:208-246`, `:423-458`) is four functions and about a hundred lines. RS-81-3 and
  architecture brief Note 8 both say to copy it; a shared abstraction over two trees makes one
  error message answer two questions.
- **Keep I/O at the edges.** Two functions touch the filesystem — page enumeration and the
  harness read — and every check is a pure function over `(&[Page], &str, &mut Vec<String>)`.
  This is not tidiness: `xtask` has no `tempfile` dev-dependency and NF-001 forbids adding
  one, so purity is what makes the unit tests in AC-002/003/004/006/008 writable at all.
- **A `Page` is probably `{ rel: String, module: String }`** computed once at enumeration, so
  `module_name` is called in exactly one place and both directions of the registration check
  compare against the same string. Deriving it twice is how the two halves start disagreeing.
- **Order the checks so the budget check runs first.** AC-008 says "before any line is
  emitted"; the cheapest reading of that is: enumerate, guard vacuity, check path budgets,
  then read the harness and check registration. It also means a path so long it would wrap is
  reported by a line that does not wrap.
- **Sort the pages once, at enumeration**, so "source order" is a property of the page list
  rather than something each check has to remember. `read_dir` order is not defined.
- **Leave a landing place for the slice-mates.** `fence-discipline-and-allowance-list` and
  `hidden-content-resolution` add checks into this module and push onto the same
  `Vec<String>`. A `fn check_*(pages: &[Page], problems: &mut Vec<String>)` convention and one
  call site list is all that is needed; do not build a registry or a trait for three checks.
- **The `# What this does not verify` section is a doc comment on the module, first**, above
  the "what this does check" prose — the ordering `xtask/src/lint_constitution.rs:9-28` uses
  and RS-81-1 requires. Write only the limits this story's own checks create; the six-item
  list is `documented-blind-spots-and-their-proofs`'.
- **`--locked` on the step's `args`.** The invocation is a `cargo run`, which resolves
  dependencies, and RS-80-4 makes that unconditional
  (`standards/rust/80-the-gate.md:245`).
- **`docs/README.md:25-29` is one sentence, not a rewrite.** The paragraph already names two
  trees the gate reads by path; this adds the third. If milestone 1 already named
  `cargo xtask narrative` there, skip it — a second mention is drift, not thoroughness.

## Tests and CI (merge gate)

Tiers follow the testing brief's four: **static**, **compile**, **gate-integration**,
**end-to-end/fixture** (`_decomposition.md:538-546`). The last tier belongs to
`observed-failure-falsification` and is named here only to mark the boundary.

| tier | command / path | proves |
| --- | --- | --- |
| static | `cargo test -p xtask` → `xtask/src/lint_narrative.rs` `#[cfg(test)] mod tests` | AC-001 (missing tree names the path), AC-002 (empty tree is a hard error, non-empty passes the guard), AC-003 (missing `include_str!` and missing `mod` are two distinct problems), AC-004 (a `mod` naming no page is a problem; the same harness with the page present is not), AC-006 (three problems, source order, composed form, nothing elided), AC-007 (success is exactly one line, count equals pages), AC-008 (31 passes, 33 fails, third level fails, `docs/adapters/sqlite.md` passes) |
| static | `cargo test -p xtask` → `xtask/src/affected.rs` `mod tests` (`:644-702`) | that the tree-selection arm milestone 1 landed still holds, and that adding this checker to `affected::run`'s unconditional list did not widen `INERT` — `a_docs_only_change_selects_nothing` (`:650-654`) and `the_constitution_arm_does_not_widen_to_all_prose` (`:698-702`) stay green |
| static | `cargo xtask narrative` | the whole surface over the real tree: one banner, one summary line, exit 0 — AC-005's standalone path and AC-006/AC-007's composition read against `_design.md` `## Composition` |
| static | `cargo xtask lint-constitution` | this story did not regress the existing checker or its corpus (project DoD 7); the two modules stayed unrefactored |
| compile | `cargo test --locked -p xtask --doc` | AC-009 — the new module's doc examples compile, so the limits section is checked prose rather than a comment; and milestone 1's harness still compiles unchanged |
| gate-integration | `cargo xtask lints` | AC-005 — the step is selected by name out of `REQUIRED`; a drifted string would panic at `xtask/src/main.rs:816-826` rather than select nothing |
| gate-integration | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | AC-005 — the step runs on the mandatory path with no optional tool present; **this is this story's merge bar** (`project.md` DoD 6) |
| gate-integration | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | AC-005 — the checker runs unconditionally on a prose-only diff, which is the story-grain gate every later story in this initiative is held to |
| gate-integration | `cargo xtask ci` | the full gate, run before the **project** is called done, not per story (`project.md` DoD 6) |
| end-to-end/fixture | *deferred* — `observed-failure-falsification` | project AC-003: `cargo xtask ci` observed failing on a deliberately broken page and then green, both outputs recorded. No test in this story substitutes for it, and this story must not claim it does. |

## Risks and coupling (PR-scoped)

| risk | shape | mitigation in this PR |
| --- | --- | --- |
| The `mod` lands in the wrong target | A `mod lint_narrative;` added to `xtask/src/lib.rs` instead of `xtask/src/main.rs` compiles clean and checks nothing — the architecture brief's named most-likely error (Note 1, CR-1/CR-2) | The Integration contract names `xtask/src/main.rs:65` as the site and the checker reads the harness as **text**; if a reviewer sees `lint_narrative` in `lib.rs`, the story is wrong regardless of a green gate |
| Half a mount | The `Step` exists but `lint_steps()`, the dispatch arm or the help line does not — the step is then invisible to one of the four paths | AC-005 exercises all four, and `steps_named`'s panic (EC-005) makes the `lint_steps` half a build-time bug rather than a silent omission |
| The two module-name derivations disagree | Forward and reverse checks each spelling the derivation, so an orphan check passes both ways over the same real mismatch | One pure `module_name` computed at enumeration and stored on `Page`; both directions compare the stored string |
| Milestone 1's hand-written `mod` names do not match the derivation decided here | The new checker reports every page as unregistered on the very PR that adds it | In-PR scope allows **module names only** in `xtask/src/narrative.rs`; no page is added, removed or re-included. If the mismatch is large enough to look like a rewrite, that is a signal the derivation is wrong, not the harness |
| Text coupling to the harness | Reformatting `xtask/src/narrative.rs` — a wrapped `include_str!`, a `mod` line that does not start with `mod ` after trimming — breaks the check without breaking the compile | Recorded as a limit in the module docs (see `## Data and migrations`), the same coupling `check_harness` already lives with. **Not** defended against with a parser: NF-001 |
| The `affected::run` divergence reads as a slip | Two inconsistent precedents in one file, and the next contributor "fixes" one | One paragraph in the module docs stating the divergence and its argument (`xtask/src/affected.rs:28-36`). A convention, not a decision — no ADR, and `_grounding.md` "Precedence and non-goals" is why |
| Scope creep into the slice-mates | The fence walk is one `for` loop away and looks like the same change | `## PR boundary`'s "Explicitly not in this PR" is the line; the slice-mates land in the same context but as their own stories, in merge order |
| A green step read as evidence of teaching | The initiative's top-ranked risk, and this story is the one that ships the green tick's opportunity | AC-009: limits first, no badge, no "verified", and the `documented-blind-spots-and-their-proofs` handoff named rather than implied |
| Cross-branch file collision | `initiative/from-contract-to-published-library` is unmerged and `docs/README.md` is a likely shared file (`project.md`, risk table) | The `docs/README.md` change here is one sentence in an existing paragraph; merge forward before the pull request |

## Dependencies

**Blocks on** — `pinned-narrative-tree-and-compiling-step`. Hard, not soft: this story's
`TREE` must name a tree that exists and its `HARNESS` must name
`xtask/src/narrative.rs` with `include_str!` lines already in it. Run before milestone 1,
every check here reports either a missing tree (AC-001) or an empty one (AC-002) — correct
behaviour that proves nothing about this story. `narrative-tree-story-grain-selection` is
milestone 1's second story and lands before this one in merge order; without it,
`cargo xtask affected --base main` selects no package on a prose-only diff and AC-005's fourth
path cannot be observed. Neither is re-implemented here.

**Unlocks** —

- `fence-discipline-and-allowance-list` — adds the fence walk *into* the module this story
  creates, pushing onto the same `Vec<String>` and reported by the same `bail!`.
- `hidden-content-resolution` — enforces DT-7 inside that same fence walk.
- `narrative-citation-resolution` and `frozen-documentation-must-pin` — both name this story
  in their `depends_on` because both add checks to this module; their other edge is
  `spec-trace-clause-id-accessor`.
- `observed-failure-falsification` — needs the assembled gate, this story's step included, so
  the recorded output is the output a future contributor will actually see.
- `documented-blind-spots-and-their-proofs` — transitively; it fills in the section this story
  creates.

## Anchors (progressive disclosure)

Open these when the bound row says to, not before. Everything needed to *start* is above; this
table is where the depth lives.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/lint_constitution.rs` | The template for this entire module, four sections at once: the pinned-const block (`:55-61`), the vacuity guard (`:175-177`), the accumulate-and-count report (`:169-199`), the module-name derivation (`:208-246`), the bidirectional harness check with its documented asymmetry (`:423-458`), and the house test shape (`:827-878`). Copying it is the instruction; abstracting over it is forbidden. | Before writing the first line of `lint_narrative.rs`, and again at the test module. | AC-001, AC-002, AC-003, AC-004, AC-006, AC-007 |
| `xtask/src/main.rs` | The mount, and the five sites are not adjacent: the bin module list (`:65`), the `Step` contract with `probe`'s normative doc comment (`:72-103`), `REQUIRED` (`:105`), the dispatch arm to mirror (`:689-697`), `print_help()` (`:718`), `lint_steps()` (`:799-808`), `steps_named`'s panic (`:816-826`), `run_fast` (`:853-860`), and the banner and step-failure lines the surface is composed from (`:864`, `:887`). | While mounting, after the module's checks pass their own unit tests. | AC-005, AC-006, AC-007 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | The **binding** signed-off design: `## Signatures` (the exact constants), `## Composition` (the worked failure report), `## Transience policy` (why success is one line), `## Density budget` (the 48/32-character arithmetic and finding 3's disposition), `## Hierarchy`, `## States`, `## Anti-patterns` 7/8/9/11, and `## Placement and re-export` (why the module cannot be `narrative.rs`). It is not re-decided by this story. | Before AC-006, AC-007 and AC-008 — the three rows whose numbers come from here and nowhere else. | AC-006, AC-007, AC-008 |
| `.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md` | Architecture brief Note 1 (the five composition roots and the wrong-target error), Note 2 (the cross-target text seam and the pure-derivation requirement), Note 4 (the two-pass data flow and why AC-005 exists at all), Note 8 (what must not move), Note 9 (the `affected::run` recommendation and the free page-naming this spec pins), Note 10 (the limits list); testing brief AC-001/AC-005/AC-009 rows (`:548-559`, `:598-607`, `:643-657`). | Note 2 before the derivation; Note 9 before touching `affected.rs`; the testing brief rows before writing the tests. | AC-003, AC-004, AC-005, AC-009 |
| `xtask/src/affected.rs` | The module docs' own argument for the unconditional list (`:28-36`), the list itself (`:112-126`), the `INERT` list and selection arm (`:212-260`), and the tests that must stay green (`:644-702`). The divergence this story records is this file's argument, quoted. | Immediately before adding the one call, and again to re-run its tests. | AC-005 |
| `xtask/src/constitution.rs` | The lib-side counterpart, and the source of the sentence this story's limits section starts from: `cfg(doctest)` means an unregistered page produces no failure at all, only silence (`:20-25`), plus the doctest lint gaps (`:27-36`) that are the slice's other documented limits. | When writing `# What this does not verify`. | AC-003, AC-009 |
| `standards/rust/80-the-gate.md` | RS-80-1 (a check is a `Step` in `REQUIRED`, reached by name — the rule the five mount sites exist to satisfy), RS-80-2 (probe means skip-when-absent, never ignore-when-failing), RS-80-4 (`--locked` on any invocation that resolves dependencies). Each carries a compiled example and a named wrong implementation. | Before writing the `Step` literal. | AC-005 |
| `standards/rust/81-checks-that-cannot-be-types.md` | RS-81-1 (prove the blind spot in the check's own tests, *then* state it in the docs — the rule AC-009 is), RS-81-2 (hard-error on what the scanner cannot read — EC-004's posture), RS-81-3 (scope a scanner to the directory whose behaviour it constrains — why no refactor). | Before the test module, and before the limits section. | AC-009 |
| `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` | The KB's own account of why report-only loses ("a warning inside a green run is invisible within a week", `:50-54`) — and, read the other way, why this story owes **no** ratchet or exemption machinery: the tree as milestone 1 left it already conforms. | If the temptation to land this as a warning first appears. | AC-005 |
| `docs/README.md` | The prose mirror of the pin-by-path rule, naming the trees the gate reads (`:25-29`), and the routing table this story does not restructure. It becomes false unless the one sentence lands in the same change. | At the end, when the checker is green. | AC-001 |
| `RUNBOOK.md` | `:918-925` — the in-house precedent this project exists to not repeat: a step that was wired, vouched for by two documents, and printed `skipped` on all three runners. It is the argument for `probe: None` and for a success line that says something. | When deciding whether a green run needs to print anything. | AC-005, AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The three personas and four journeys the acceptance criteria are framed from, carried here because `.kb/product/` holds no `authority_tier: product` atom to cite. It is also the check on this story's honesty: none of these readers ever runs this checker, so no criterion may be written as if the gate serves them directly. | Before re-wording any AC, and before writing anything about what a reader gains. | AC-006, AC-009 |
| `.bklg/docs-that-teach/checked-documentation-surface/_storymap.md` | The slice's merge order (`## Merge order` 2.3-2.5) and the coverage table showing this story **owns** project AC-001 and AC-005 with no co-owner — so a check moved into a slice-mate leaves a project AC unowned. | Before starting, and before moving any check between slice-mates. | AC-001, AC-003 |

## Clarifications resolved during spec

- **The nine AC ids are exactly the front half's, unchanged.** AC-001 and AC-002 split project
  AC-001's two mechanisms (a pinned path whose absence is named, and a vacuity guard) because
  a pinned constant without the guard passes green over an emptied tree — one criterion could
  not fail one of them. AC-003 and AC-004 split project AC-005's two directions for the same
  reason, and because a forward-only checker is the plausible wrong implementation CLAUDE.md's
  rule requires to be named. Nothing was added or dropped.
- **AC-006, AC-007 and AC-008 are composition criteria and are deliberately table rows, not
  prose.** They carry `_design.md`'s composed problem line, its one-line success output and
  its 48/32-character path budget. Without them the story would pass with a checker that
  printed `Err(...)` debug output, or nothing at all on success — every functional assertion
  satisfied, the signed-off surface absent.
- **AC-009 is one row for two obligations** (project DoD 7's limits section and DoD 8's
  no-teaching-claim) because both are properties of the same artifact — the module's own
  docs — and splitting them would produce a row whose only verification is a grep.
- **The page-length and H1 budgets are not gated here, and that is a decision.** `_design.md`
  `## Open questions` 2 keeps them review rules; only the path budget becomes a gate rule,
  because it protects the terminal surface rather than the editorial one. Gating the other two
  would put this project inside HS-P0021's page-need discipline — a second job, which is the
  defect BR-04 exists to make visible.
- **`cargo xtask ci` in full is not this story's bar.** `--fast` is
  (`.redkiln/config.yaml:55`, `project.md` DoD 6). The full gate is a project-close obligation
  and is listed in the tests table as such, so no implementer treats a red `OPTIONAL` step as
  this story's failure.
- **No end-to-end falsification is claimed here.** Project AC-003 requires observing
  `cargo xtask ci` itself fail and recover, and `observed-failure-falsification` owns it. This
  story's unit tests are not evidence for it, and the implementation report must not present
  them as such.
- **No ADR is owed.** `_grounding.md` "Precedence and non-goals" verified that no Accepted
  decision atom governs gate structure, documentation trees or fence compiling; the one
  divergence from in-repo precedent (joining `affected::run`'s unconditional list) is a
  convention discharged by a paragraph in the new module's docs. Writing an ADR as a side
  effect of this story would be the wrong instrument.
- **The module/subcommand name asymmetry is signed off, not a slip.** Module `lint_narrative`
  (following `lint_constitution`), subcommand `narrative`, step name
  `every narrative page is checked`. The module cannot be `narrative.rs` because `HARNESS`
  pins that path as the file this checker reads as text. Note it in the module docs; do not
  "fix" it.
