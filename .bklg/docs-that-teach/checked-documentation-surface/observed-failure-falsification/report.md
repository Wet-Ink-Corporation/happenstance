---
item: "HS-S0144"
stage: report
created: "2026-08-17T13:16:05.747Z"
updated: "2026-08-17T13:16:05.747Z"
---

# Report — The gate is watched failing on a page broken on purpose, then recovering

## Findings Ledger

Nine ACs, all satisfied, each citing a named section of `_falsification.md` and the `file:line`
of the primitive it observed. Nothing stubbed, nothing skipped, nothing deferred out of this
story's own set — and no `#[test]` was added, because the testing brief forecloses one here
(`_decomposition.md:568-570,581-584`).

**The single most important thing in this report is not an AC row.** AC-002's predicted outcome
was *falsified* by the run, and the falsification is the deliverable. Read finding **F2** first.

| AC | Result | Evidence | Follow-up |
| -- | ------ | -------- | --------- |
| **AC-001 — a false claim fails the assembled gate, and all three runs are transcribed.** Run 1 exit 0, run 2 exit 1 (30.2 s), run 3 exit 0 (1 m 58 s), all at `6368e2b1c45c6506d0258441035c48829370dc31`. | met | `_falsification.md` §§ *Baseline*, *The edit*, *Run 2 — red*; the hunk is `git apply --check`-able at that sha; failing test `xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9)` | None. The truncation is stated beside the transcript: `run_steps` bails at the first non-zero status (`xtask/src/main.rs:963`), so run 2 printed 3 of 26 banners. |
| **AC-002 — attribution, and the counterfactual.** Prediction **falsified**; counterfactual held. | met (recorded, routed) | `_falsification.md` § *Attribution and its counterfactual*; `xtask/src/main.rs:145-155`, `:496-509`, `:519-525` | **F2**, routed to `pinned-narrative-tree-and-compiling-step` and HS-S0145. Not fixed here (EC-009). |
| **AC-003 — what the failure actually identifies, against a pre-registration.** P1 held, P2 held, P3 diverged, P4 held. | met | `_falsification.md` §§ *Predictions, pre-registered*, *What the failure actually identified*; `cargo test --locked -p xtask --doc -- --list` | **F1** and **F5** to HS-S0145. |
| **AC-004 — the revert restores green, with the coverage numbers.** 26 banners, `all checks passed`, `  1 page(s)' examples enumerated`, `  1 pages, all consistent`, `  27 atoms, all consistent`, 358 citations checked. Zero `skipped:` lines in any of the three runs. | met | `_falsification.md` § *Run 3 — recovered*; `xtask/src/lint_constitution.rs:192`; `probe: None` at `xtask/src/main.rs:509`, `:551` | None. EC-006 not triggered — `RUNBOOK.md:914-928`'s failure is not reproduced. |
| **AC-005 — no residue, no broken page, no source change.** | met | `_falsification.md` § *Residue*; `git status --porcelain` after run 3 | One honest exception, stated rather than rounded: `.redkiln/telemetry/events/…jsonl` remains modified. It is the redkiln CLI's own session telemetry, not a source file and not under any tree this story may touch. |
| **AC-006 — a dated, sha-pinned, re-runnable instrument.** | met | `_falsification.md` § *Provenance* and the file's preamble; `git cat-file -e 6368e2b…` resolves | None. The run is nested under a dated `## Run of 2026-08-17` heading so a later re-run gets a sibling section rather than an edit — `_design.md` `## Mock`'s discipline, applied to this record. |
| **AC-007 — claims exactly what was observed.** One unhedged sentence; three stated limits. | met | `_falsification.md` § *What this does not establish* | None. The prose scan matches the forbidden marks only inside the one sentence quoting `_design.md` anti-pattern 9. |
| **AC-008 — composition fidelity, divergences named.** Twelve rows: nine observed, three diverged. | met | `_falsification.md` § *Composition, checked against the design* | **F2**, **F3**, **F4** to HS-S0145. |
| **AC-009 — density measured with real numbers.** Location prefix 41 characters against a budget of 48 inclusive / 32 repo-relative. | met | `_falsification.md` § *Density, measured* | **F4**: at 80 columns the page survives on the first visual row and the line number does not. |

### Findings, routed

| # | Finding | Evidence | Routed to |
| - | ------- | -------- | --------- |
| **F1** | **The narrative tree's fences are run, not merely compiled.** `assert_eq!(store.len(), 1)` type-checks and is false; the gate caught it as an assertion panic, exit 101. Nothing in this project had established that. | the `left == right` assertion failure in run 2, `left: 0`, `right: 1` | HS-S0145 — limit 1 must not be widened into "the gate cannot tell whether a claim is true". It can, for any claim an example *asserts*. |
| **F2** | **Three `REQUIRED` steps compile the narrative tree, and the first of them is `tests` at index 2.** A broken page fails under `=== tests ===`; neither narrative banner (index 16) nor the constitution's (17) prints, because `run_steps` bails first. `cargo test --locked --workspace --all-features` runs every lib target's doctests, and `xtask`'s lib target (`xtask/src/lib.rs:28`) is the harness's doctest root. Milestone 1's `the_narrative_step_precedes_the_constitution_step` orders 16 and 17 and is silent about 2. | run 2's complete three-banner sequence; `xtask/src/main.rs:145-155`, `:496-509`, `:519-525`, `:963` | `pinned-narrative-tree-and-compiling-step` (owner of step ordering and of its own `EC-005`) **and** HS-S0145. Deliberately **not** fixed here — EC-009. |
| **F3** | **The failure's tail is one xtask line, not two.** `xtask failed: tests failed with exit code: 101` carries both primitives, because the `eprintln!` at `:777` renders the error the `bail!` at `:963` produced, inside the same process. The line beneath it is cargo's. `_design.md` `## Composition` and mock finding 2 both over-specify. | run 2's last two lines | HS-S0145, and an amendment for `_design.md`'s owner. |
| **F4** | **The compile surface cannot satisfy anti-pattern 7, and the line number wraps at 80 columns.** rustdoc's report begins `test ` (5 chars) or `thread 'main' (23672) panicked at ` (34 chars) before the path; the path occupies columns 6-46 and `(line 9)` columns 79-86. | § *Density, measured*; widths counted with `awk` | HS-S0145 — limit 4 must say the page is identified on the first visual row and the location inside it is not. This is rustdoc's output, which this repository does not control. |
| **F5** | **The panic's `file:line` is unstable and, when it is a real path, the line is wrong.** With `-- --show-output`: `xtask\src\../../docs/append-conditions.md:6:1`. Without it: a temporary `doctest_bundle_2024.rs:9:1` under the OS temp directory. Established by a controlled pair differing only in that flag. And `:6` counts inside rustdoc's synthesized doctest source, so it points at a line of prose; the assertion is at page line 13. | § *What the failure actually identified*, the four-row invocation table | HS-S0145 — limit 4 must be written from the doctest **name**, not the panic line, and must say the panic line is not usable as a location. |
| **F6** | **A residual carried forward, not resolved.** The fixture's sentence describes a *boundary* property while `ES-40`'s normative text is about *completeness* (`spec/SPECIFICATION.md:4351`; the boundary property argued at `:4293`). The break is to the fence's assertion, which is independent of it. | `_falsification.md` § *Findings routed to HS-S0145*, F6 | `_design.md`'s owner. Recorded, not edited — two other stories are pinned to that page's exact bytes. |

### Mount point

`xtask/src/main.rs`'s `REQUIRED` array (`:107`), exercised **as a whole** through
`cargo xtask ci`. This story is the one that *reads* that composition root rather than adding
to it: no `Step`, no `mod`, no dispatch arm, no help line. The artifact-side mount is
`_falsification.md`, mounted into the project's evidence chain by being cited from every row of
`_ledger.md` and consumed by HS-S0145 for limits 1 and 4 — an evidence artifact nothing cites
is the documentation analogue of a constructed-but-unmounted component.

### Surfaces rendered

`gate-narrative-compile-step` in **`fail-broken-fence`** (the only story in the project that
renders it) and in `pass`; `gate-narrative-checker-step` in `pass`. Not rendered, and named as
unrendered rather than left to be assumed: `fail-removed-item`, `fail-hidden-marker`,
`fail-many`, `fail-one`, `fail-empty-tree`, `fail-long-path`, and `rustdoc-reference-surface`.

### Deferred

Nothing from this story's own set. Everything the run turned up that could be *fixed* is
deliberately not fixed here and is routed above, because EC-009 is the rule that keeps this
story's transcripts describing a tree that actually existed.

### Gate

`cargo xtask ci` green (run 3, whole gate) · `cargo xtask ci --fast` green ·
`cargo xtask affected --base main` reporting `affected gate passed` with `231 passed; 0 failed` ·
`cargo fmt --all --check` clean · `cargo xtask lint-constitution` reporting `27 atoms, all consistent`.

### Review disposition — 2026-08-17

Two things the slice review settled about this story, recorded here so a later reader does not
have to reconstruct them.

**F2 now has an addressee.** Its routing named `pinned-narrative-tree-and-compiling-step`, which
was sealed at `6368e2b` — the commit this run measured — so the finding had no live inbox. The
*limits-list* half landed as the additive seventh limit in `xtask/src/narrative.rs` (see
`documented-blind-spots-and-their-proofs/_limits-evidence.md` and its finding **L6**); the
*step-ordering* half is owned by `FU-1` of `HS-P0020`. `_falsification.md`'s F2 carries the
disposition addendum. Nothing measured in this record was edited.

**NF-006 — "transcripts complete, never abridged" — reviewed and accepted as stated.** The three
logs are 4,366–5,716 lines and are quoted by region, which § *How complete each transcript below
is* declares rather than performs silently. The review's finding: no change required. The
complete banner sequence with log line numbers is quoted — precisely the region NF-006 exists to
protect — `grep -c "^skipped"` is reported as **0** for all three runs, every rendered-surface
block and the whole failure region are unedited, and § *Provenance* gives the commands that
reproduce the omitted regions byte-for-byte. Recorded so the deviation is not later mistaken for
an unnoticed gap.

