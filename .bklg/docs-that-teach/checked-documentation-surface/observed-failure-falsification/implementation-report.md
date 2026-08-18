---
item: "HS-S0144"
stage: implement
created: "2026-08-17T13:16:05.747Z"
updated: "2026-08-17T13:16:05.747Z"
---

# Implementation Report — The gate is watched failing on a page broken on purpose, then recovering

## TDD Evidence

This story is the one story in the project whose proof may not be a `#[test]`. The testing
brief settles it in as many words: AC-003 is "the only AC in this project whose proof is a
recorded procedure rather than a `#[test]` function" and is "not substitutable by a unit test
that calls the checker function directly, because AC-003 as written in `project.md` requires
observing `cargo xtask ci` itself fail"
(`.bklg/docs-that-teach/checked-documentation-surface/_decomposition.md:568-570,581-584`). No
`#[test]` was added, and the `xtask` unit suite is byte-identical to its state on the base
commit — 231 doctests and the same unit set, confirmed by `cargo xtask affected --base main`.

So the Red/Green loop was executed on the **gate itself**, which is exactly the instrument the
AC names, and the pre-registration is what keeps it a falsification rather than a narrative.

**Pre-registration came first.** `_falsification.md` §§ *Provenance* and *Predictions,
pre-registered* were written to disk **before** the break was applied and before run 2 was
started. Eight predictions, P1–P8, each with its source and the disposition of *both* outcomes,
so no interpretation was chosen after the fact. P1/P2 are pre-registered as a pair because
they are the whole discrimination: P1 holding with P2 holding means the fences are compiled
*and executed*; P1 failing means a false-but-compiling claim is invisible to this gate forever
(EC-002).

**Red.** With the pre-registration on disk, one line of `docs/append-conditions.md` was
changed so a claim became false — `assert_eq!(store.len(), 0)` → `assert_eq!(store.len(), 1)`,
code that type-checks perfectly and says something untrue about the library — and
`cargo xtask ci` was run. It exited **1** in 30.2 s, and it failed **for the right reason**:
the failing test is `xtask\src\../../docs/append-conditions.md - narrative::append_conditions
(line 9)` and the failure is `assertion \`left == right\` failed / left: 0 / right: 1`, exit
code 101. Not a compile error, not an import error, not a typo — the assertion the edit
falsified.

**Green.** `git checkout -- docs/append-conditions.md` (deliberately not `git stash`, which
leaves state `git status --porcelain` does not show), then `cargo xtask ci` again: exit **0**
in 1 m 58 s, all 26 banners, `all checks passed`.

**A green baseline came before the red run, and that is not ceremony.** Run 1 was a full
`cargo xtask ci` at the same sha, exit 0. Without it a red run is consistent with a tree that
was already red for an unrelated reason, and the record would attribute a failure to an edit
that did not cause it — the class
`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` is about, one level up.

| AC | Its instrument | Red → Green |
| -- | -------------- | ----------- |
| AC-001 | `cargo xtask ci` × 3 at `6368e2b`, with the edit as a `git apply --check`-able hunk | Run 1 exit **0** → run 2 exit **1** → run 3 exit **0**. The only variable between runs 1 and 2 is the one-line hunk. |
| AC-002 | Run 2's banner sequence + `RUSTDOCFLAGS=-D warnings cargo test --locked -p xtask --doc` run directly, page still broken | **The prediction was falsified**, which is the most valuable output of the run. The failing banner is `=== tests ===` (step 2), not the narrative step's (step 16). The counterfactual half held: the constitution step's unfiltered argv exits 101 too. Routed as **F2**; not fixed here (EC-009). |
| AC-003 | `cargo test --locked -p xtask --doc -- --list` + run 2's failure region, against the P1–P5 pre-registration | P1 ✔, P2 ✔ (assertion panic — the fences are **run**), P3 ✘ (the name carries the harness's *directory* plus the page's own path, never the harness's filename), P4 ✔ (line 9 = the fence's opening marker, not the assertion at page line 13). A second, unstable identifier — the panic's own `file:line` — was found and pinned to `--show-output` by a controlled pair. |
| AC-004 | Run 3's coverage lines and `grep -c "^skipped"` | `  1 page(s)' examples enumerated`, `running 1 test … 1 passed`, `  1 pages, all consistent`, `  27 atoms, all consistent`; zero `skipped:` lines in any of the three runs. |
| AC-005 | `git status --porcelain` after run 3 | `docs/append-conditions.md` absent from the list; no path under `docs/`, `crates/`, `xtask/`, `spec/`, `standards/` modified. |
| AC-006 | `git cat-file -e` + `git apply --check` against § *Provenance* | Both resolve at `6368e2b`. |
| AC-007 | Document review against `project.md` DoD item 8 and `_design.md` anti-pattern 9 | One unhedged sentence; three stated limits; the forbidden marks occur only inside the single sentence that quotes the anti-pattern. |
| AC-008 | Element-by-element check of the real bytes against `_design.md` `## Composition` / `## Hierarchy` | Twelve rows, nine `observed`, three `diverged` (**F2**, **F3**, **F4**), each routed. |
| AC-009 | `awk '{ print length($0) }'` over the captured transcripts | Ten measured widths; location prefix **41** characters (16 + 25) against a budget of ≤ 48 / ≤ 32; column arithmetic showing the line number wraps off the first visual row at 80 columns (**F4**). |

## Commits

| SHA | Subject |
| --- | ------- |
| `<checkpoint>` | feat(checked-documentation-surface): Observed failure falsification |

The row records the checkpoint **as first written**. A commit cannot contain its own SHA, so
the reachable commit is reported in the slice digest and recorded on the item by
`redkiln record-links --sha` at the advance seam.

Single checkpoint commit on `initiative/docs-that-teach`, carrying only paths inside this
story's own backlog folder. Not pushed.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `.bklg/…/observed-failure-falsification/_falsification.md` | **New, 400+ lines.** The dated, sha-pinned record: provenance, the eight pre-registered predictions with both dispositions each, an explicit statement of how complete each transcript is, the three runs (complete banner sequences plus every surface block unedited), the edit as a unified diff hunk, the attribution finding and its two counterfactuals, the measured identification of the failure, a twelve-row composition table, a measured density table, the residue check, six routed findings, and the closing limits section. |
| `.bklg/…/observed-failure-falsification/_ledger.md` | Nine rows flipped `false` → `true`, each with a real citation to a named section of `_falsification.md` and to the `file:line` of the primitive it observed. No criterion re-worded, removed or added. AC-002's evidence states plainly that its predicted outcome was falsified. |
| `.bklg/…/observed-failure-falsification/implementation-report.md` | **New.** This file. |
| `.bklg/…/observed-failure-falsification/report.md` | **New.** The findings ledger the review gate reads. |
| `docs/append-conditions.md` | **Edited and reverted inside the run. Net zero**, and absent from `git status --porcelain` and from the commit. |

No source file was changed: no `xtask/src/**`, no `crates/**`, no `Cargo.toml`, no
`Cargo.lock`, no `spec/`, no `standards/`. The mount point this story exercises —
`xtask/src/main.rs`'s `REQUIRED` array, run as a whole through `cargo xtask ci` — is the file
this story *reads* rather than adds to; its entire claim is about what that array does when a
page under `docs/` is wrong.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo xtask ci` (run 1, baseline) | exit **0**, 26 banners, `all checks passed`, 0 `skipped:` lines |
| `cargo xtask ci` (run 2, page broken) | exit **1** in 30.2 s, failing at `=== tests ===` |
| `RUSTDOCFLAGS=-D warnings cargo test --locked -p xtask --doc` (page broken) | exit **101** — the counterfactual |
| `RUSTDOCFLAGS=-D warnings cargo run --locked --quiet -p xtask -- narrative-doctests` (page broken) | exit **1** — the narrative step as wired, rendering `gate-narrative-compile-step / fail-broken-fence` |
| `cargo test --locked -p xtask --doc -- --list` | the registered doctest name |
| `cargo xtask ci` (run 3, reverted) | exit **0** in 1 m 58 s, `all checks passed` |
| `cargo xtask affected --base main` | `affected gate passed`; `231 passed; 0 failed; 3 ignored` |
| `cargo fmt --all --check` | clean |
| `cargo xtask lint-constitution` | `27 atoms, all consistent` |
| `cargo xtask ci --fast` | exit **0** — `all required checks passed (--fast: 4 optional step(s) not run)` |

## Notes

**Three deviations from the plan, all of them measurements rather than choices.**

1. **AC-002's predicted banner is falsified, and the story's own EC-005 is the governing
   disposition.** The spec expected the failure to arrive under
   `=== the narrative tree's examples compile ===`. It arrives under `=== tests ===`, fourteen
   steps earlier, because `cargo test --locked --workspace --all-features` runs every lib
   target's doctests and `xtask`'s lib target is the harness's doctest root. Milestone 1's
   `the_narrative_step_precedes_the_constitution_step` orders steps 16 and 17 and is silent
   about step 2. EC-005 says record verbatim and route; EC-009 forbids fixing it here. Both
   were obeyed. This is finding **F2** and it is routed to
   `pinned-narrative-tree-and-compiling-step` and to HS-S0145.

2. **Two extra observations were run that the spec's command sequence does not list**, both
   while the page was still broken and both because the primary one was starved by F2. The
   narrative step was invoked **exactly as `REQUIRED` declares it** (same program, args and
   `env`) so that `gate-narrative-compile-step / fail-broken-fence` — the state the whole
   design was drawn to make legible, and the state no other story renders — was observed rather
   than merely inferred. And a controlled pair of `cargo test --locked -p xtask --doc` with and
   without `-- --show-output` was run, because the panic's `file:line` differed between the two
   invocations already captured and limit 4's wording depends on which is true. Neither is a
   hand-typed approximation of a step, and both are recorded with their exact argv.

3. **The transcripts are quoted block-complete rather than file-complete, and the record says
   so.** Three full runs are 15,745 lines, most of it cargo's per-crate build chatter and a
   231-line doctest roster. `_falsification.md` § *How complete each transcript below is*
   states the policy before the first transcript: every quoted block is unedited and complete
   from banner to banner, every run's **complete** banner sequence is given (which is precisely
   where a `skipped:` line would appear), the line counts and reproducing commands are given,
   and the abridgement is between blocks. NF-006's actual worry — silent elision of the region
   where a `skipped:` line would hide — is answered directly and additionally by a stated
   `grep -c "^skipped"` of 0 on all three runs.

**Nothing was repaired.** F2, F3, F4 and F5 are all defects or over-specifications in
artifacts this story was written to observe. EC-009 is unambiguous — a story that repairs the
thing it was written to test cannot report on it, and its transcripts would describe a tree
that never existed on any branch — so every one of them is recorded and routed, and the
working tree of `xtask/src/`, `crates/`, `spec/` and `standards/` is untouched.

**The `git status --porcelain` was not literally empty, and the record says that rather than
rounding.** One entry remains throughout: `.redkiln/telemetry/events/…jsonl`, the redkiln
CLI's own append-only session telemetry, written by the tool that launched the work. It is not
a source file, not a page, and not under any tree this story may touch. AC-005's substance —
no broken page committed, no source change — holds exactly, and stating the exception is
cheaper than a claim a reviewer can disprove in one command.
