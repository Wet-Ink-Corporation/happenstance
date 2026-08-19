---
item: "HS-S0189"
stage: implement
created: "2026-08-19T00:00:00.000Z"
updated: "2026-08-19T00:00:00.000Z"
---

# Implementation Report — Every fence inventoried and every clause citation audited

## TDD Evidence

**This story writes no Rust and adds no test target, and that is a spec constraint rather than
a shortcut.** `spec.md § Behavior and interfaces` states it outright — *"This story defines,
changes and consumes no Rust interface, and writes no Rust code"* — NF-004 forbids the drills
adding any permanent cost to the gate (*"no new step, no new test target, no new constant"*),
and the PR boundary admits no test directory, so `redkiln verify --grain story` would reject a
file under `xtask/tests/`. The red-then-green obligation is discharged in the medium the story
actually has: **six executed falsifications**, each of which turned a real instrument red on a
real tree before the reverted tree turned it green again.

Every row below was run in this order — **red first, then green** — and every red asserts on the
checker's own `path:line — message` string rather than on a bare non-zero exit, so a checker
failing for an unrelated reason cannot read as a passing drill.

| AC | Falsification (the "test") | Red → Green |
| --- | --- | --- |
| **AC-001** | `no_fence_on_the_page_opts_out_of_the_compiler` and `every_fence_holds_the_density_budget` (`xtask/tests/first_encounter.rs:488`, `:492`) already guard `docs/first-encounter.md`; the inventory's own claim is that **every** fence is exercised, and D-1/D-2 are what make it falsifiable | **red** on D-1's `ignore` (`docs/carry-your-invariant.md:60 — an `ignore` fence needs an `IGNORE_ALLOWANCES` entry…`) → **green** after revert (`5 pages, all consistent`) |
| **AC-002** | **D-1** — `docs/carry-your-invariant.md:60` retagged `rust` → `ignore`, run through `cargo xtask narrative` *and* `cargo xtask lints` | **red** with the file and line named, both commands exit 1 → **green** after revert, `git hash-object` back to `b636975…`, `git status --porcelain` empty |
| **AC-002** | **D-2** — `crates/happenstance/src/lib.rs:26` retagged to `ignore`. **Expected green, and that is the finding.** | **green throughout** — `cargo xtask lints` 0, `cargo test -p happenstance --doc` `7 passed; 1 ignored` 0, `cargo test -p happenstance --tests` 16 binaries ok, `cargo test -p xtask --tests` ok. Recorded as *observed green*, never skipped. Revert restores `8 passed; 0 ignored` |
| **AC-003** | The resolver-per-page table's central claim: `spec-trace` does **not** resolve a page's citation. Falsified by running it on D-3's broken tree | **`spec-trace` exit 0 against a page citing `ES-999`** — the claim survived the attempt to break it, which is what makes the per-page column a measurement rather than a reading of the storymap's shorthand |
| **AC-004** | **D-3** — `docs/first-encounter.md:130` retagged `ES-25` → `ES-999` | **red**: `docs/first-encounter.md:130 — cites `ES-999`, which SPECIFICATION.md does not define` → **green** after revert, hash back to `6307363…`, tree clean |
| **AC-004** | **D-4** — `ES-999` injected on `crates/happenstance/src/lib.rs:86` by replacing a line, so `doc_budget.rs`'s `MODULE_DOC_LINES = 130` could not fire and disguise the result. **Expected green.** | **green throughout** — `lints`, `spec-trace`, `RUSTDOCFLAGS=-D warnings cargo doc`, and 16 test binaries. Recorded as *observed green* |
| **AC-005** | **EC-002 probe**, the one falsification the spec demanded be *established by running the step*: `docs/carry-your-invariant.md:60` set to `rust,no_run` **and its central assertion inverted**, so the page's teaching claim is now false | **green** with `no_run` — `cargo xtask narrative` says `5 pages, all consistent` and the doctest reports `(line 60) - compile … ok`. **Red** with `no_run` removed and the same inverted assertion — `(line 60) … FAILED`. The pair is the measurement: seven characters turn a false page green |
| **AC-006** | The two artifacts' own claim that nothing was absorbed and nothing repaired by exemption | `git diff --stat` on the four planning artifacts **empty**; `git status --porcelain` **empty**; all three drilled files byte-identical to their pre-drill `git hash-object`; `IGNORE_ALLOWANCES` still the empty slice |

**Why the EC-002 probe is the strongest evidence in the story.** It is the only falsification
here that breaks a page's *meaning* rather than its tagging. With `no_run` on, the bridge page
asserts that a guard **accepts** where the page says it refuses, HS-P0020's checker prints
`all consistent`, and the doctest passes. That is `_decomposition.md:616-625`'s named failure
mode measured rather than quoted, and it is what R-1 routes.

## Commits

One checkpoint, and this table is filled by the follow-up commit that records it — the sha of a
commit containing this file cannot be inside this file, which is the template's own note and the
pattern `3e0a71e`, `a78ca36` and `b4c0c52` already established in this project.

| SHA | Subject |
| --- | ------- |
| `1c65cf5` | `feat(application-author-path): Fence inventory and clause audit` |

## Changes

Two new files, and **nothing else** — the four other entries in the PR boundary show no net
change, which the boundary paragraph predicted and which is now a fact about the diff.

| File | Shape of the change |
| --- | --- |
| `.bklg/…/fence-inventory-and-clause-audit/_inventory.md` | **New.** `§ Substrate confirmation` (EC-001/EC-005/EC-006, all clear), `§ Fences` (11 fence rows + 2 zero-fence rows across five files, with info string, execution class, exerciser, failability and hidden-line count per row; a composition-preservation baseline; and the re-derivation command per column), `§ Drills` (D-1, D-2, the EC-002 probe), `§ Routing` (R-1…R-7) |
| `.bklg/…/fence-inventory-and-clause-audit/_citations.md` | **New.** `§ Claims` (the resolver stated per page *before* any claim is listed; C-1…C-9 with maturity markers verbatim; every link fragment resolved), `§ Restatement` (the pasted `MUST` sweep and HS-P0021's RP-40-2 executed with per-file verdicts and sentence counts), `§ Drills` (D-3, D-4) |
| `.bklg/…/fence-inventory-and-clause-audit/_ledger.md` | Six rows flipped `false → true`, each with real cited evidence. No criterion re-worded, none removed |
| `crates/happenstance/src/lib.rs` | **No net change.** Touched by D-2 and D-4 and reverted; `git hash-object` `05c6982…` before and after |
| `docs/first-encounter.md` | **No net change.** Touched by D-3 and reverted; `63073630…` before and after |
| `docs/carry-your-invariant.md` | **No net change.** Touched by D-1 and the EC-002 probe and reverted; `b636975…` before and after |
| `xtask/src/narrative.rs` | **No change.** All three tree pages were already registered at `:135`, `:141`, `:150`, so EC-006 did not fire and the mount is *observed* rather than made |

**The mount, and what makes it observable.** `xtask/src/narrative.rs` is the composition root
because it is the single file in which *whether a page's fences are exercised at all* is decided.
Two observations discharge it: every page this project authored carries a `#[cfg(doctest)] mod`
line there (`:135`, `:141`, `:150`), and `IGNORE_ALLOWANCES`
(`xtask/src/lint_narrative.rs:304`) names none of this project's fences — it names nothing at
all. Both are re-derivable by `git grep`, and both are what `§ Fences`' zero column is a claim
*about*.

## Gates

| Gate | Command | Result |
| --- | --- | --- |
| Story grain (`affected_gate`, `.redkiln/config.yaml:40`) | `cargo xtask affected --base main` | **`affected gate passed`** |
| Static reachability (`reachability_static`, `:48`) | `cargo xtask lints && cargo xtask spec-trace` | green — `5 pages, all consistent`; `every page declares one need — 5 pages, 16 rules, all consistent`; `traceability: no problems found` |
| Project bar (`integration_scoped`, `:55`) | `cargo xtask ci --fast` | **`all required checks passed (--fast: 4 optional step(s) not run)`** |
| Tier 1, render | `cargo doc -p happenstance --no-deps`; the REQUIRED `documentation` step under `RUSTDOCFLAGS=-D warnings` | green; `details.toggle.top-doc` renders `open`, `div.docblock` is its child, zero literal `[bracket]` pairs in prose |
| Tier 2, compiled fence | `the narrative tree's examples compile` (`xtask/src/main.rs:571`) | green inside `ci --fast` |
| Tier 3, executed | `cargo test --locked --workspace --all-features` (`:158`) | green inside `ci --fast`; the crate-root fence runs as `lib.rs - (line 247)` and the five tree fences as `narrative::<page> (line N)` |
| Tier 4, falsification | D-1 … D-4 and the EC-002 probe | recorded in `_inventory.md § Drills` and `_citations.md § Drills`, both directions each, tree clean after every one |
| Ledger | `require_ledger` (`:67`) | six rows, `satisfied: true`, non-placeholder evidence |
| Formatter | `cargo fmt --all --check`, inside `ci --fast`'s first step | green — no Rust file has a net change |
| Residue | `git status --porcelain` | empty; the three drilled files byte-identical to their pre-drill hashes |

## Notes

**Six deviations and decisions, each recorded rather than absorbed.**

1. **D-1's subject is the bridge page, not the opening encounter.** `xtask/tests/first_encounter.rs:488`
   already carries `no_fence_on_the_page_opts_out_of_the_compiler` for `docs/first-encounter.md`,
   so a drill there would fire two instruments and could not attribute the failure to
   HS-P0020's checker — which is the mechanism AC-002 is about. `docs/carry-your-invariant.md`
   has only the checker. The asymmetry itself is recorded in `§ Restatement`, because a page with
   one instrument and a page with two are not the same page.

2. **D-4 injects a citation rather than retagging one**, because the crate root cites no clause
   at all. The bare `ES-999` token is exactly what `lint_narrative::clause_citations` scans for
   on a tree page, so both halves of the drill present the checker with the same input. The edit
   replaces a line rather than adding one so that `doc_budget.rs`'s `MODULE_DOC_LINES = 130`,
   currently met at exactly 130, could not fire and disguise a citation result as a density one.

3. **The EC-002 probe went further than EC-002 asked.** EC-002 requires establishing whether the
   checker rejects `no_run`. Establishing that it does *not* is only half an answer — the
   interesting question is what the silence buys. So the probe also inverted the fence's central
   assertion, which turns the pair into a measurement: with `no_run` the page is false and green,
   without it the page is false and red. No `no_run` exists on any of this project's surfaces, so
   EC-002's *repair* obligation is discharged vacuously and only the *routing* obligation
   (R-1) remains — the conflation EC-002 warns against did not occur.

4. **Two findings were discovered while measuring and are recorded rather than dropped.** R-3:
   the crate-root fence's doctest is named `(line 247)` in a 242-line file, because the doc
   string is `README.md` concatenated with the module doc — the exact failure
   `xtask/src/constitution.rs:11-18` names, present on the one surface outside the harness. R-4:
   `course-subscriptions` is a **bin-only** package, so `cargo test --doc` finds no library
   target and `overview.md` contributes no doctest to the workspace sweep; `spec.md`'s
   "rides the same `--workspace` sweep" holds for its render and not for its fences. The file
   carries zero fences, so the finding is prospective — which is precisely the case the inventory
   exists to catch before it is expensive.

5. **The crate-root density overages were re-measured and *not* re-owned.** 35 rendered lines
   against 32, 70 columns against 68, two `##` headings at 39 and 26 against 22. All three are
   HS-B0001's, with an origin record and a stated reason for not being this initiative's. Filing
   them again here would give one defect two owners, which is the shape of bookkeeping that makes
   a routing table stop being read.

6. **No repair was made, and no exemption was written.** EC-003 was never reached: all eleven
   fences compile and execute, every cited id resolves, and no sentence restates a clause. The
   only two `## PR boundary` entries that changed are the two new artifacts. Two citations
   (`C-6`, `C-8`) sit at the end of the claim clause rather than of the sentence; both are
   recorded with the reason moving them would be a rewrite rather than a one-line repair
   (EC-008), so the tier-5 reviewer meets the sentences rather than a tick.
