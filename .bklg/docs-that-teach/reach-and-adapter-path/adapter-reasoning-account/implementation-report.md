---
item: "HS-S0155"
stage: implement
created: "2026-08-17T13:16:13.497Z"
updated: "2026-08-17T13:16:13.497Z"
---

# Implementation Report — The sequenced adapter reasoning account

## TDD Evidence

**What "test first" means for this story, stated before the table so it is not mistaken for a
missing test suite.** This project takes **no testing brief** by decision (`project.md`, "Out of
scope"; `spec.md` `## Tests and CI`), and NF-005 caps this story at **zero** new gate steps, **zero**
checkers and **zero** `xtask/src/` constants. A bespoke `#[test]` re-implementing this page's `rg`
assertions would be exactly the checker that NF-005 and architecture notes N-1/N-4 forbid, and it
would be a checker for one page. So the failing assertions were the repository's **own mandatory
gate steps**, run against the page before it was mounted and before it declared a need — pre-existing,
mechanical, and failing for precisely the behaviour each AC names. Everything they cannot see is the
recorded `rg` and measurement pass in `_verification.md`, which is what the spec asks for by name.

Red was real: the two runs below happened with `docs/adapter-reading-order.md` present in the tree
and the page not registered and not declaring a need. Neither failure is a typo, an import error or a
compile error — each is the assertion for a missing behaviour.

| AC | Assertion (and where it lives) | Red → Green |
| -- | ------------------------------ | ----------- |
| AC-001 | `check_registration`, both directions — `xtask/src/lint_narrative.rs:536-563`, run by the mandatory `every narrative page is checked` step (`:351`) | **RED** `xtask/src/narrative.rs — does not include adapter-reading-order.md; its examples are never compiled` **and** `xtask/src/narrative.rs — no mod adapter_reading_order; one module per page is what keeps a doctest failure's line number relative to the page` → **GREEN** `6 pages, all consistent`, after `xtask/src/narrative.rs:153-162` |
| AC-007 | `check_declarations` — `xtask/src/lint_pages.rs:630`, run by the mandatory `every page declares one need` step (`:188`) | **RED** `docs/adapter-reading-order.md — no > **Answers:** line; see standards/pages/00-one-need.md` → **GREEN** `6 pages, 16 rules, all consistent`, after `docs/adapter-reading-order.md:3` |
| AC-006 | `check_citations` — `xtask/src/lint_narrative.rs:1197`; every citation-shaped token on the page is resolved against `spec/SPECIFICATION.md` and an unknown one fails the build | Green on the first run **and non-vacuous**: `VT-11` (`:32`) and `CF-15` (`:42`) are the two ids it resolves. Falsified by hand during authoring — mistyping one id reproduces `cites …, which SPECIFICATION.md does not define` |
| AC-003, AC-004, AC-005, AC-008, AC-009, AC-010 | recorded `rg` one-liners and two measurements, `_verification.md` | Each command and its output is transcribed there; the page was authored against them rather than checked afterwards (`spec.md`, implementation notes: "author the `_verification.md` companion as you go") |
| AC-002 | rendered-line measurement at a 96- and a 105-column content box, `_verification.md` | 15 rendered lines against the design's ≤ 15 of an ≈ 27-line first screen. The first draft's lead-in ran to two rendered lines and put the total at 16 — over budget — and was cut to one line rather than an entry being dropped |

The two red transcripts are also the falsifications the ACs themselves name. AC-001 says it is
"falsified by deleting the registration entry and finding the gate still green"; the red run *is*
that state, and the gate was not green.

## Commits

One checkpoint commit, on `initiative/docs-that-teach`, not pushed.

| SHA | Subject |
| --- | ------- |
| *`git log -1 --format=%h --grep "Story: reach-and-adapter-path/adapter-reasoning-account"`* | `feat(reach-and-adapter-path): The sequenced adapter reasoning account` |

The SHA cell is a lookup rather than a literal, for the reason the slice-mate story recorded first:
**the SHA of a commit containing this file cannot be written inside this file** — writing it changes
the tree, which changes the SHA. The commit is identified by the one thing about it that is stable,
its `Story: reach-and-adapter-path/adapter-reasoning-account` trailer; the literal belongs on the
item, via `redkiln record-links HS-S0155 --sha <sha>` before the advance that files this report,
which is what `require_commit_provenance: true` reads.

Files in that commit: `docs/adapter-reading-order.md` (new), `docs/README.md`,
`xtask/src/narrative.rs`, this story's `spec.md`, `_ledger.md`, `_verification.md` (new), this file
and `report.md`, plus `.redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl`, which the
pre-commit hook stages itself. The hook ran; `--no-verify` was not used.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `docs/adapter-reading-order.md` | **New.** The page: 78 lines, 704 words, five regions in the order `_design.md` binds — H1 + the answered-need declaration (`:1`, `:3`), the six-entry reading order (`:7-19`), the `MemoryEventStore` caveat in position at entry 1 (`:28-35`), six sections each carrying "N of 6" (`:21`–`:69`), and a terminal region stating the page's limits and offering exactly one hop (`:71-77`). No fence, no raw HTML, no fold, one markdown link |
| `xtask/src/narrative.rs` | **The mount.** `mod adapter_reading_order` with its `include_str!` at `:160-161`, preceded by the comment the file's convention asks for (`:153-158`) explaining what registration buys this particular page. Nine added lines; nothing else in `xtask/src/**` is touched — no constant, no step, no checker |
| `docs/README.md` | One row in the tree's `Page / Read it at` table (`:21`), so a human browsing `docs/` reaches the page without reading `xtask/src/` |
| `.bklg/…/adapter-reasoning-account/spec.md` | Body only. The PR boundary's placeholder glob `<HS-P0020-PINNED-TREE-ROOT>/**` bound to `docs/**` plus the one registration file, with the evidence for the binding written beneath it |
| `.bklg/…/adapter-reasoning-account/_ledger.md` | Ten rows flipped `false → true` with cited evidence; `<TREE>` bound in every `mount_point`; the preamble's placeholder note rewritten to record that the binding happened. No criterion re-worded |
| `.bklg/…/adapter-reasoning-account/_verification.md` | **New.** The reviewer-repeatable pass: EC-001's re-verification, ten AC sections, every command and its output |
| `.bklg/…/adapter-reasoning-account/implementation-report.md`, `report.md` | **New.** This file and the findings ledger |

## Gates

| Command | Result |
| ------- | ------ |
| `cargo xtask narrative` | `6 pages, all consistent` |
| `cargo xtask lint-pages` | `6 pages, 16 rules, all consistent` |
| `cargo xtask narrative-doctests` | `all doctests ran in 3.13s` — the page produces no doctest, because it carries no fence; it is walked as a file by the step above |
| `cargo xtask affected --base main` | **`affected gate passed`** — fmt, clippy `-D warnings`, `233 passed; 0 failed` in `xtask`'s lib target, the five file-reading lints and `spec-trace`. This is the story-grain gate `.redkiln/config.yaml` wires (`verify.affected_gate`) |

The affected packages for this story are `xtask` (the registration) and the file-reading lints over
`docs/`; `happenstance` and `happenstance-core` are unchanged by it and were compiled by the same run.

Formatting: `cargo fmt --check` runs inside the affected gate and is green. The one Rust file touched
is `xtask/src/narrative.rs`; the rest of the diff is markdown, which no formatter in this repository
owns.

## Notes

Three deviations, all recorded rather than absorbed.

**1. The need token is `explanation`, not the routing-shaped need the design worded.** `_design.md`
region 1 states the answered need as *"in what order do I read what already exists, to build an
adapter"*, and HS-P0021's page-need rule had not landed when that was signed off. It has now, and it
closes the need set at four tokens (`xtask/src/lint_pages.rs:119-135`). `orientation` — the token that
sentence's shape points at — is unavailable twice over, and neither reason is a preference:
`RP-10-3` allows **one** `orientation` page per directory level and `docs/read-the-worked-example.md:3`
already holds it for `docs/` (enforced by `check_orientation_ceiling`, `xtask/src/lint_pages.rs:685`);
and `RP-10-2` caps an `orientation` page at links plus one sentence per destination and says it
teaches nothing, which this page's own signed-off density budget (≤ 5 sentences / ~120 words of
connective tissue per source, plus a required caveat) exceeds by design. `explanation` is the honest
token for the journey state the spec frames every AC from — Persona 2 at **B3**, "building a model of
what an adapter is shaped like". **The need sentence itself is unchanged**: it is the declaration's
question, verbatim, so AC-007's substance holds and only vocabulary the design could not have known
about moved. Full reasoning in `_verification.md`, section AC-007.

**2. `spec.md`'s PR boundary gained a second line, `xtask/src/narrative.rs`, and that is the mount
rather than a widening.** EC-001 asked this story to block if the tree had not landed. It has: the
tree is `docs/`, and registration in it *is* an `include_str!` + `mod` pair in
`xtask/src/narrative.rs`. There is no way to mount a page in this tree without touching that one file,
which is why the boundary's own "In this PR" list already permits "the tree's own registration file".
What the boundary excludes — the pinned-tree constant, the `REQUIRED` step, any checker — is untouched,
and NF-005's three zeros hold.

**3. Design finding F5 is closed for this surface, and `route` is bound.** `_design.md` kept
`route: TBD` for `adapter-reasoning-account` because inventing a path would have been the
invented-primitive failure the design stage exists to prevent. The path now exists and is not invented:
`docs/adapter-reading-order.md`, chosen against `PATH_BUDGET` (`xtask/src/lint_narrative.rs:266`, 32
characters — this path is 28) and against the module-name derivation the registration check compares in
both directions.

**One thing deliberately not done.** The single onward hop takes **no** pointer-register row. The
register in `xtask/src/pointers.rs` counts pointers installed under the policy's rule 2 — a *secondary
pointer at an item*, on the reference surface, admitted by gates (i)–(iv). This hop is an ordinary
in-tree link between two pages of one tree, already guarded by that tree's own registration check;
filing a row for every such link would spend a cap of eight on links the register was not built to
track. NF-005 allows *at most* one row for this story and it takes zero, said here rather than left
to be discovered.
