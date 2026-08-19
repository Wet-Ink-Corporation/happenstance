---
item: "HS-S0190"
stage: implement
created: "2026-08-19T00:00:00.000Z"
updated: "2026-08-19T00:00:00.000Z"
---

# Implementation Report — Each page's answered need and every anchor reviewed

## TDD Evidence

**This story writes no Rust, and NF-001 is what makes that a constraint rather than a
shortcut**: *"No `xtask` code, no second corpus, no second checker, no rule atom."* The notation
is HS-P0021's, the walk is HS-P0021's, the checker is HS-P0021's and the corpus constant is
HS-P0020's; adding a test here would be the *three lists that must agree* defect one level out.
The red-then-green obligation is discharged where this story actually has an instrument: **a
two-direction calibration of the procedure itself**, plus every mechanical claim run as a command
whose output is pasted rather than asserted (NF-002).

The calibration is the whole of the story's rigour. Its argument is CLAUDE.md's, applied to a
written procedure: *a rule that no adapter can fail is decorative*. This story's deliverable is
an **absence** — no page carries two needs, no page re-argues the anchor — and an absence is only
demonstrable by showing the instrument detects a presence.

| AC | Falsification / measurement (the "test") | Red → Green |
| --- | --- | --- |
| **AC-004(a)** | HS-P0021's inert specimen `standards/pages/examples/two-needs.md` walked with RP-40-1 | **red** — step 1 answers `no`, two `> **Answers:**` lines both visible in the head → verdict **`fail — two needs`**, matching what the specimen's own `:34-38` says the walk should find. Inertness proved, not assumed: `git grep -c` → 2, and `ls standards/pages/*.md` does not list it |
| **AC-004(b)** | A second declaration injected into `docs/carry-your-invariant.md:4` — a live page of this project's own set, inside `TREE` so a machine can be observed failing | **red**: `cargo run --locked --quiet -p xtask -- lint-pages` → ``docs/carry-your-invariant.md:4 — declares `explanation` and `how-to`; a page answers one need``, and the same failure under `cargo xtask lints` → **green** after revert: hash back to `b636975…`, `git status --porcelain` empty, step back to `5 pages, 16 rules, all consistent` |
| **AC-002** | The form and position checks, run as commands before the human walk | `git grep -c '^> \*\*Answers:\*\*'` → **1** on each of the three tree pages and **1** on the crate root; head regions captured showing nothing interposed; declaration line **above** the first fence on every surface that has one |
| **AC-003** | The mechanical half **run**, not assumed | `cargo run --locked --quiet -p xtask -- lint-pages` → `5 pages, 16 rules, all consistent`. Corpus **shown**: `git grep -n 'const TREE' -- xtask/src/lint_narrative.rs` → `:239`; `git grep -n 'lint_narrative::TREE' -- xtask/src/lint_pages.rs` → `:379`, `:467`, `:519`. **No `PAGE_DIR` exists** |
| **AC-005** | The phrase sweep, all four phrases, output pasted with line numbers | Hits **only** at `docs/carry-your-invariant.md:19`, `:20`, `:22`; the `## Where your streams went` span captured separately as `:17-29`; **zero** on the crate root, the opening encounter, the handoff and the index |
| **AC-006** | The anchor's existence, its inbound links, and the traverse | `git grep -c "Where your streams went"` → **1** at `:17`; one relying page links it and the link resolves; the `documentation` step green under `RUSTDOCFLAGS=-D warnings`. The keyboard traverse **does not complete**, recorded as W-1 |
| **AC-001, AC-008** | The two set differences, computed rather than eyeballed | In the enumeration and **not** in the index: `carry-your-invariant.md`, `read-the-worked-example.md` → **finding W-1**, not a quiet append |

**The one measurement that changed what this story reports.** AC-001's second cross-check was
expected to be a formality. It found that two of the four surfaces are absent from
`docs/README.md`'s narrative index, and the inbound sweep in `_walk.md § 6` then showed they link
only to each other: **an isolated two-page component with no reader-facing route in.** HS-S0188
had already deferred *its own* index row to HS-P0023; what only the last slice could see is that
the deferral, taken twice, left a component rather than a page unreachable.

## Commits

One checkpoint, and this table is filled by the follow-up commit that records it — the sha of a
commit containing this file cannot be inside this file, which is the template's own note and the
pattern `3e0a71e`, `a78ca36`, `b4c0c52` and `17f14b5` already established in this project.

| SHA | Subject |
| --- | ------- |
| `4dedf8a` | `feat(application-author-path): Answered-need and anchor review` |

## Changes

One new file and one ledger, and **nothing else** — this story's diff touches none of the four
surfaces it walked, which is the strongest available statement that no repair was needed.

| File | Shape of the change |
| --- | --- |
| `.bklg/…/answered-need-and-anchor-review/_walk.md` | **New.** `§ 1` the enumeration and its two cross-checks with both set differences computed; `§ 2` the walker and what they were and were not permitted to read; `§ 3` RP-40-1 answered step by step per page, with the crate root's two scopings stated in full; `§ 4` the three-column coverage table and the pasted mechanical output with its corpus shown; `§ 5` the two-direction calibration; `§ 6` the anchor sweep, the inbound-link enumeration and the failed traverse; `§ 7` the cite-or-re-argue judgement with the sentence quoted; `§ 8` the six findings and their destinations |
| `.bklg/…/answered-need-and-anchor-review/_ledger.md` | Eight rows flipped `false → true` with pasted-output evidence, plus a `## Recorded result` section in the body carrying the eight-column per-page table AC-008 asks for and the routed-findings list. No criterion re-worded, none removed |
| `crates/happenstance/src/lib.rs`, `docs/first-encounter.md`, `docs/read-the-worked-example.md`, `docs/README.md`, `xtask/src/narrative.rs` | **No change at all.** In the PR boundary for reading and for a repair that turned out not to be needed |
| `docs/carry-your-invariant.md` | **No net change.** Touched by the AC-004(b) injection and reverted; `git hash-object` `b636975…` before and after |

**The mount, and what makes it observable.** `xtask/src/narrative.rs` is the composition root
because *membership in the set this story makes a claim about is decided there*: a page under
`docs/` the harness does not name is unregistered, and a page outside `TREE` is invisible to the
`every page declares one need` step no matter how correct its declaration is. Both halves are
observed rather than asserted — all three tree pages are named at `:135`, `:141`, `:150`, so no
registration line was needed (and none was added); and the fourth surface is recorded in `§ 4`'s
coverage table **as outside the corpus, with its reviewer named**, which is the second of the two
observations the integration contract asks for.

## Gates

| Gate | Command | Result |
| --- | --- | --- |
| Story grain (`affected_gate`, `.redkiln/config.yaml:40`) | `cargo xtask affected --base main` | **`affected gate passed`** |
| Static reachability (`reachability_static`, `:48`) | `cargo xtask lints && cargo xtask spec-trace` | green — `every page declares one need — 5 pages, 16 rules, all consistent`; `every narrative page is checked — 5 pages, all consistent`; `traceability: no problems found` |
| Project bar (`integration_scoped`, `:55`) | `cargo xtask ci --fast` | **`all required checks passed (--fast: 4 optional step(s) not run)`** |
| Tier 1, structural | the `documentation` REQUIRED step (`xtask/src/main.rs:344`) under `RUSTDOCFLAGS=-D warnings`; `cargo xtask spec-trace` | green inside `ci --fast` — an unresolved intra-doc link would be denied rather than warned |
| Tier 2, the page checker | `cargo run --locked --quiet -p xtask -- lint-pages` | `5 pages, 16 rules, all consistent`, and observed **failing** by `path:line` under the injection |
| Tier 3, executed | **n/a, stated rather than omitted.** This story adds no Rust, no doctest and no `#[test]` | — |
| Tier 4, falsification | the two-direction calibration | recorded in `_walk.md § 5`, both directions, tree clean afterwards |
| Tier 5, review sign-off | RP-40-1 over four surfaces plus AC-007's judgement | recorded in `_walk.md § 3` and `§ 7`, and composed into `_ledger.md § Recorded result` |
| Ledger | `require_ledger` (`:67`) | eight rows, `satisfied: true`, non-placeholder evidence |
| Formatter | `cargo fmt --all --check`, inside `ci --fast`'s first step | green — no Rust file has a net change |
| Residue (NF-004) | `git status --porcelain` | empty; `docs/carry-your-invariant.md` byte-identical to `b63697589f05c4a1663ef3754a5d96c59b4b6298` |
| Boundary (NF-001, NF-006) | `git diff 17f14b5 -- xtask/ standards/ spec/` | **empty** — no `xtask` code, no rule atom, no clause, no `Cargo.toml` |

## Notes

**Six decisions and deviations, each recorded rather than absorbed.**

1. **NF-001's own command is stale against this branch, and the honest measurement is stated
   instead.** NF-001 asks for `git diff main -- xtask/src` to contain at most a registration line.
   It does not: HS-P0020 and HS-P0021 landed `lint_narrative.rs` (4,845 lines) and
   `lint_pages.rs` (3,421 lines) on this branch, which is the substrate this story *consumes*.
   The measurement that answers what NF-001 protects is this story's own diff,
   `git diff 17f14b5 -- xtask/ standards/ spec/`, which is empty. Recorded rather than reported
   as a pass against a command whose base moved.

2. **The calibration injects into the bridge, not the handoff.** Only a page inside `TREE` has a
   mechanical instrument that can be observed failing (clarification 3), and of the two candidates
   the handoff carries a second instrument in `examples/course-subscriptions/tests/reach.rs`
   (`exactly_one_answered_need`) while the bridge has only HS-P0021's step. A drill with two
   instruments cannot attribute its own failure. The bridge was chosen for the same reason the
   slice-mate chose it for D-1, and the asymmetry is recorded in `_walk.md § 4`'s coverage table.

3. **The crate root's `pass` rests on two scopings, and both are disclosed in full.** RP-00-2
   anchors the declaration to a markdown `# Title` and a rustdoc crate root has none (EC-005), so
   step 1 was answered against the position `_design.md:426-431` fixed. And RP-40-1's step 3 asks
   whether every *section* serves the declared need; over the whole rendered page the answer is
   `no`, because `# Features` and `# Testing without a database` do not serve `tutorial`. The
   scoping to the region `_design.md:413-436` composes is not a convenience — band 10 excludes
   rustdoc from the need set **by name and with a reason**
   (`standards/pages/10-the-need-set.md:25-31`), and the remaining sections are HS-P0016's landing
   copy which `project.md`'s risk table places outside this project. Without the scoping the
   verdict is `indeterminate`, which would block this story; **the scoping is therefore written so
   a reviewer can reject it**, and W-3 routes the rule gap to HS-P0021 either way.

4. **W-1 is the finding this story exists to have found, and it is routed rather than repaired.**
   Two of the four surfaces are unreachable: absent from `docs/README.md`'s index, linked by
   neither the crate root nor the opening encounter, linking only to each other. The repair is two
   index rows, each carrying a one-line orientation description — a pointer-policy decision under
   RP-10-2, two rows rather than one line, and explicitly HS-P0023's DT-10 by this story's own
   clarification 7. HS-S0188 deferred the same row for the same reason
   (`surface-course-subscriptions/report.md:50-53`); what this story adds is that the deferral,
   taken twice, isolated a component rather than a page. **EC-007 held: the repair exceeded one
   line, so it stopped and routed.**

5. **EC-004 fired and its disposition was already decided.** `## Where your streams went` measures
   23 characters against `_design.md`'s 22-character `##` budget. `tension-resolutions/_resolutions.md:338-343`
   scoped that budget to `crate-root-encounter` alone — a markdown surface has no 200px sidebar at
   any width, so anti-pattern 5 there is a check no page could ever fail. **Measured, recorded, not
   renamed.** Renaming to relieve one character would have moved the one string every inbound link
   resolves into, which is the whole of what AC-006 proves.

6. **A CRLF working copy silently defeats `$`-anchored patterns, and the specs' commands assume
   otherwise.** `git grep -n '^## Where your streams went$'` returns **nothing** on this machine
   while the heading exists at `:17`. Every command in `_walk.md` is written without a trailing
   anchor and the reason is stated there, so a later reader re-deriving the record does not
   conclude the heading is missing.
