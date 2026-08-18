# Falsification record — the gate watched failing on a page broken on purpose

A dated instrument. It records three runs of `cargo xtask ci` against one named commit: a
green baseline, a red run after a one-line edit that makes a claim on
`docs/append-conditions.md` false, and a green run after that edit is reverted. Everything
below is machine output pasted verbatim or a number counted off it.

**This record is never edited to agree with a later tree.** When the tree moves and a
transcript here goes stale, the answer is a second dated section from a second run, appended
below this one. That is the discipline `_design.md` `## Mock` already applies to its own
mock — "editing the mock to agree with the corrected design would delete the evidence" — and
it binds this file for the same reason: an instrument is only legible against the frame that
produced it.

---

## Run of 2026-08-17

### Provenance

| What | Value |
| --- | --- |
| Date | 2026-08-17 |
| Commit (`git rev-parse HEAD`) | `6368e2b1c45c6506d0258441035c48829370dc31` |
| Branch | `initiative/docs-that-teach` |
| Worktree | `D:\repos\happenstance\.claude\worktrees\docs-that-teach` |
| `rustc -Vv` | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, host `x86_64-pc-windows-msvc`, LLVM 22.1.6 |
| `cargo -V` | `cargo 1.97.1 (c980f4866 2026-06-30)` |
| Toolchain source | `rust-toolchain.toml` pins `channel = "1.97.1"` — the pin and ADR-0029's MSRV floor currently coincide |
| OS / shell | Windows 11 (`MINGW64_NT-10.0-26200`), git-bash |

`git cat-file -e 6368e2b1c45c6506d0258441035c48829370dc31` resolves. The commit is
`chore(checked-documentation-surface): seal slice specification-pin approved`, the last
commit of milestone 3, so both of this story's dependencies —
`pinned-narrative-tree-and-compiling-step` (the fixture page and the compile step) and
`narrative-checker-mounted-with-pinned-path` (the checker step) — are ancestors of it. The
gate below is the assembled gate, not a partial one.

The exact command sequence, in order, is the whole of what was run:

```console
git rev-parse HEAD
git status --porcelain
cargo xtask ci                                     # run 1 — baseline
git apply <the hunk in § The edit>                 # or edit the line by hand
cargo xtask ci                                     # run 2 — red
RUSTDOCFLAGS=-D warnings cargo test --locked -p xtask --doc   # the counterfactual, page still broken
cargo test --locked -p xtask --doc -- --list                  # the doctest's registered name
git checkout -- docs/append-conditions.md
cargo xtask ci                                     # run 3 — recovered
git status --porcelain
```

### Predictions, pre-registered

Written into this file **before** run 2 was started, so that the interpretation of the red
run is not chosen after the fact. Each prediction names its source and the disposition of
every outcome it admits.

| # | Prediction | Source | If it holds | If it does not |
| --- | --- | --- | --- | --- |
| P1 | Flipping `assert_eq!(store.len(), 0)` to `assert_eq!(store.len(), 1)` makes `cargo xtask ci` exit non-zero. | Project AC-003 (`project.md:208-211`); initiative DoD-2 (`initiative.md:416-420`) | The gate catches a claim that is *false* and not merely *uncompilable*, which is strictly harder than project AC-002's removed-item break. | EC-002. The fences are compiled and not run; no false-but-compiling claim can ever be caught. Recorded as a limits-list item for HS-S0145 **and** routed to `pinned-narrative-tree-and-compiling-step`'s review as a machine defect. The break is **not** swapped for a weaker one, and this story does not close green. |
| P2 | The failure is an **assertion panic**, not a compile error. | Run 1's own transcript shows the doctest reported `... ok` rather than merely compiling, so rustdoc executes it; `assert_eq!(store.len(), 1)` type-checks | The fences are **run**, not merely compiled — a fact nothing in this project had established, and the discriminator between the two worlds this break exists to separate. | If it is a compile error instead, the break did not test what it was written to test and the record says so; if it is neither (P1 fails), EC-002 governs. |
| P3 | The file rustdoc names is the **harness**, `xtask/src/narrative.rs`, not the markdown page — with the **module** resolving the page and the **line** resolving the location inside it. | Architecture brief Note 3 (`_decomposition.md:186-189`) | Note 3's forecast is confirmed and limit 4 of HS-S0145's section is written from a measurement rather than a prediction. | The record wins over the forecast (that story's EC-004). The divergence is written up as a finding and limit 4 is worded from what was observed. |
| P4 | The line number rustdoc reports locates the fence's **opening** line inside `docs/append-conditions.md` — line 9 — and not the broken assertion's own line (13). | Run 1's `--list` name reads `(line 9)`, and the fence opens at `docs/append-conditions.md:9` | The location is actionable at fence granularity, not statement granularity, and AC-003's "identifies the page and the location" is satisfied at that grain. Recorded as measured, with the arithmetic shown. | Whatever it is, it is recorded as measured. |
| P5 | The failing banner is `=== the narrative tree's examples compile ===`, and `=== the constitution's examples compile ===` never prints, because `run_steps` `bail!`s on the first non-zero status (`xtask/src/main.rs:963`) and the narrative step is ordered first. | `xtask/src/main.rs:938-965`; milestone 1's `EC-005` | Ordering preserved attribution, live. | EC-005: the ordering in `REQUIRED` is wrong. Recorded verbatim and routed to `pinned-narrative-tree-and-compiling-step`. Not fixed here, and specifically not "fixed" by filtering the constitution step — its argv also carries the repository README's doctest, which matches neither filter. |
| P6 | The constitution step's own argv, run directly while the page is still broken, **also fails** — it is unfiltered. | `xtask/src/main.rs:519-525`: `cargo test --locked -p xtask --doc` with `RUSTDOCFLAGS=-D warnings`, no filter | Attribution is preserved by **step ordering**, not by the `narrative::` filter, which is what makes P5 non-trivial. | If it passes, the two steps do not overlap and the ordering argument in `main.rs:482-495` is wrong about its own mechanism. Recorded and routed. |
| P7 | Run 2's log is **truncated at the failing step** and says nothing about any step after it. | `run_steps` bails at the first non-zero status | Stated beside the transcript so a red log is never read as whole-gate evidence. | — |
| P8 | Run 3 is green across the whole gate, and neither narrative step prints a `skipped:` line. | Both steps carry `probe: None` (`xtask/src/main.rs:103-105`), which makes the skip line structurally impossible | The `RUNBOOK.md:914-928` failure — a probe-gated step printing `skipped` while two documents vouched for it — is not reproduced. | EC-006. A `skipped:` line in any of the three runs is a halt, and a green run containing one is never recorded as satisfying AC-004. |

Two dispositions are pre-registered as a pair because they are the whole point of the break:
**P1 holding with P2 holding** means the fences are compiled *and executed*; **P1 failing**
means they are compiled and not executed, and a false-but-compiling claim is invisible to
this gate forever. Nothing else discriminates between those two worlds.

**Outcome, in one line each**: P1 held. P2 held. P3 **diverged**. P4 held. P5 **diverged**,
and it is the most important thing in this record. P6 held. P7 held. P8 held.

### How complete each transcript below is

Each of the three runs produced between 4,366 and 5,716 lines, most of it cargo's per-crate
`Compiling` / `Finished` chatter and, in the `tests` step, a 231-line roster of every doctest
in the workspace. Pasting all three whole would put roughly 800 KB of build chatter into a
backlog folder and bury the four regions that carry the evidence, so what is quoted below is:

- the **complete banner sequence** of every run, in order, with each banner's line number in
  the captured log — this is exactly the region where an unexpected `skipped:` line would sit,
  because `run_steps` prints it between banners (`xtask/src/main.rs:952`);
- the **complete, unedited output block** of every step this story renders as a surface, and
  of every failure region, from its banner to the next banner;
- the **complete two-line tail** of the failing run.

Nothing inside a quoted block is elided, shortened or reordered; the abridgement is between
blocks and is stated here rather than performed silently, which is what NF-006 is protecting.
The commands in § *Provenance* reproduce the omitted regions byte-for-byte. Line counts:
run 1 — 5,716 lines; run 2 — 4,366 lines; run 3 — 5,663 lines.

### Baseline — run 1, green

`git status --porcelain` immediately before the run returned exactly one entry:

```text
 M .redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl
```

That file is the redkiln CLI's own append-only session telemetry, written by the tool that
launched this work; it is not a source file, not a page, and not under any tree this story is
allowed to touch. No path under `docs/`, `crates/`, `xtask/`, `spec/` or `standards/` was
modified. That is the sense in which the baseline tree was clean, and it is stated as measured
rather than rounded to "empty".

`cargo xtask ci` then exited **0**. Its complete banner sequence, with the line number each
banner sits at in the captured log:

```text
   4: === formatting ===
   6: === clippy (all targets, all features) ===
   9: === tests ===
4347: === each phase's proof artefacts ===
4402: === wasm32 build of the contract crate ===
4405: === wasm32 check of the conformance harnesses ===
4408: === wasm32 build of the Cloudflare adapter ===
4411: === wasm32 build of the Neon adapter ===
4414: === documentation ===
4419: === specification traceability ===
4426: === no retired rule is still live ===
4429: === no conformance rule reads a clock ===
4432: === no literal position values in the suite ===
4435: === every conformance rule has a changelog entry ===
4438: === the testkit carries its own version ===
4441: === happenstance-core names serde/alloc and base64/alloc ===
4444: === the Rust constitution is internally consistent ===
4447: === the narrative tree's examples compile ===
4464: === the constitution's examples compile ===
4713: === every narrative page is checked ===
4716: === documentation (no default features) ===
4721: === packaged artifacts carry their licences and README ===
4727: === feature powerset ===
4911: === wasm32 feature powerset ===
4979: === licences and advisories ===
5709: === docs.rs configuration (nightly) ===
```

Twenty-six banners. `grep -c "^skipped" run1-baseline.txt` returns **0** — no step in this
gate, optional or otherwise, printed a skip line on this runner. The last line of the run is
`all checks passed`.

The two steps this story observes, complete and unedited:

```text
=== the narrative tree's examples compile ===
  1 page(s)' examples enumerated
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
   Doc-tests xtask

running 1 test
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 63 filtered out; finished in 0.01s

all doctests ran in 2.40s; merged doctests compilation took 1.85s
```

```text
=== every narrative page is checked ===
  1 pages, all consistent
```

Load-bearing in the first block: `1 page(s)' examples enumerated` is the compile step's own
coverage figure, and `running 1 test … 1 passed` is the proof that the enumerated page was
handed to rustdoc and that rustdoc **ran** it rather than only compiling it — `... ok` is a
test result, not a build result. `170 filtered out` is the rest of the workspace's doctests
that the `narrative::` filter excluded. In the second block, `1 pages, all consistent` is the
checker's whole success output: one line, as `_design.md` `## Transience policy` requires.

Wall clock: run 1 was captured as a background job without a timer. Plan a full green
`cargo xtask ci` by run 3's measured **1 m 58 s** on this machine with a warm `target/`.

### The edit

One line, applied to the working tree with the fixture page's fence otherwise untouched. As
a unified diff hunk, so the run is reproducible byte-for-byte:

```diff
diff --git a/docs/append-conditions.md b/docs/append-conditions.md
index afa12b9..a502c44 100644
--- a/docs/append-conditions.md
+++ b/docs/append-conditions.md
@@ -10,7 +10,7 @@ appending (ES-40).
 use happenstance_core::MemoryEventStore;
 
 let store = MemoryEventStore::new();
-assert_eq!(store.len(), 0);
+assert_eq!(store.len(), 1);
 ```
 
 `memory` is a private module (`crates/happenstance-core/src/lib.rs:103`); the type is
```

`git apply --check` accepts this hunk against `6368e2b`. What it produces is code that
type-checks perfectly — `MemoryEventStore::len` returns `usize` and `1usize` is a `usize` —
and states something **false** about the library: a store constructed by
`MemoryEventStore::new()` holds no events. That is the break project AC-003 asks for, and it
is strictly harder than project AC-002's, which removes an item a page calls and therefore
fails at *compile* time.

### Run 2 — red

`cargo xtask ci` exited **1** after **30.2 s**. Its complete banner sequence:

```text
   4: === formatting ===
   6: === clippy (all targets, all features) ===
   9: === tests ===
```

Three banners, and then the run stops. `run_steps` spawns each step as its own process and
`bail!`s on the first non-zero status (`xtask/src/main.rs:938-965`, the `bail!` at `:963`),
so **this log is truncated at the failing step and proves nothing whatever about the
twenty-three steps after it** — including both narrative steps, neither of which printed a
banner in this run. A red `cargo xtask ci` log is evidence about one step. Only run 3 says
anything about the whole gate.

The failure region, complete and unedited, from the end of the doctest roster to the last
line of the run:

```text
failures:

---- xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) stdout ----
Test executable failed (exit code: 101).

stderr:

thread 'main' (23672) panicked at xtask\src\../../docs/append-conditions.md:6:1:
assertion `left == right` failed
  left: 0
 right: 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace



failures:
    xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9)

test result: FAILED. 230 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 17.49s

error: doctest failed, to rerun pass `-p xtask --doc`

xtask failed: tests failed with exit code: 101
error: process didn't exit successfully: `target\debug\xtask.exe ci` (exit code: 1)
```

The last two lines are the tail. `xtask failed: tests failed with exit code: 101` is
`eprintln!("\nxtask failed: {err:#}")` at `xtask/src/main.rs:777` rendering the anyhow error
that `bail!("{} failed with {status}", step.name)` at `:963` produced — one line carrying
both primitives, because the failing step was a `cargo test`, not a nested
`cargo run -p xtask`, so there is exactly one `xtask` process in the chain. The line beneath
it is **cargo's**, not this repository's: it is the `cargo` that launched `target\debug\xtask.exe`
reporting its child's exit status.

**P1 holds.** A page edited so one claim is no longer true of the library makes the assembled
gate exit non-zero. **P2 holds**, and it is the measurement that discriminates: the failure is
an **assertion panic** — `assertion \`left == right\` failed / left: 0 / right: 1`, exit code
101 — not a compile error. The fences in the narrative tree are **compiled and run**. Nothing
in this project had established that before this run; every earlier story's evidence was
consistent with a world in which a false-but-compiling claim passes forever.

### Attribution and its counterfactual

**P5 is falsified, and this is the most valuable output of the run.**

The failing banner is `=== tests ===`. Neither `=== the narrative tree's examples compile ===`
nor `=== the constitution's examples compile ===` printed at all — not because attribution was
preserved, but because the run never reached either of them.

The mechanism, read off the composition root. `REQUIRED` (`xtask/src/main.rs:107`) contains
**three** steps that compile the narrative tree, not two:

| Index | Step name | argv | Compiles `docs/`? |
| --- | --- | --- | --- |
| 2 | `tests` | `cargo test --locked --workspace --all-features -- --show-output` (`:145-155`) | **yes** — `--workspace` includes `xtask`, and `cargo test` runs a lib target's doctests by default. `xtask`'s lib target is `xtask/src/lib.rs`, which is where the harness is declared (`:28`). |
| 16 | `the narrative tree's examples compile` | `cargo run --locked --quiet -p xtask -- narrative-doctests`, `RUSTDOCFLAGS=-D warnings` (`:496-509`) | yes, filtered to `narrative::` |
| 17 | `the constitution's examples compile` | `cargo test --locked -p xtask --doc`, `RUSTDOCFLAGS=-D warnings` (`:519-525`) | yes, unfiltered |

Milestone 1 reasoned about the ordering of 16 and 17 and pinned it with a unit test —
`xtask::narrative_doctests::tests::the_narrative_step_precedes_the_constitution_step`, which
asserts `step_index(STEP) + 1 == step_index("the constitution's examples compile")`. That
assertion is true and it is not enough: it orders the *pair*, and says nothing about the set
of steps that compile the tree. `tests` sits fourteen places in front of both of them, and it
is the step a contributor's broken page actually meets.

This is milestone 1's `EC-005` realized one step earlier than `EC-005` imagined, and this
story's own EC-005 governs the response: **record the observed banner sequence verbatim and
route it; do not fix it here.** It is routed below as finding **F2**.

**Counterfactual (a) — the constitution step's own argv, page still broken.** P6 holds:

```console
$ RUSTDOCFLAGS="-D warnings" cargo test --locked -p xtask --doc
```

```text
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... FAILED
…
thread 'main' (6880) panicked at C:\Users\ryanm\AppData\Local\Temp\rustdoctestL4ULPt\doctest_bundle_2024.rs:9:1:
assertion `left == right` failed
  left: 0
 right: 1
…
test result: FAILED. 167 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.49s
error: doctest failed, to rerun pass `-p xtask --doc`
```

Exit 101. The step below the narrative step **does** compile and run the narrative pages, so
the overlap milestone 1 reasoned about is real. Between *those two* steps, ordering — not the
`narrative::` filter — is what would keep the failure under the narrative banner. The
correction milestone 1 forbade remains forbidden: filtering the constitution step would drop
the repository README's doctest (`xtask\src\lib.rs - (line 50)` and `(line 127)`, both present
in run 2's roster) out of the gate, because its name carries neither corpus's module path.

**Counterfactual (b) — the narrative step exactly as `REQUIRED` declares it, page still
broken.** This is the only way to see the surface `gate-narrative-compile-step` in state
`fail-broken-fence` on this tree, because `cargo xtask ci` never reaches the step that renders
it:

```console
$ RUSTDOCFLAGS="-D warnings" cargo run --locked --quiet -p xtask -- narrative-doctests
```

```text
  1 page(s)' examples enumerated
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
   Doc-tests xtask

running 1 test
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... FAILED

failures:

---- xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) stdout ----
Test executable failed (exit status: 101).

stderr:

thread 'main' (40632) panicked at C:\Users\ryanm\AppData\Local\Temp\rustdoctestODlOE4\doctest_bundle_2024.rs:9:1:
assertion `left == right` failed
  left: 0
 right: 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace



failures:
    xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 63 filtered out; finished in 0.02s

all doctests ran in 2.50s; merged doctests compilation took 2.01s
error: doctest failed, to rerun pass `-p xtask --doc`

xtask failed: the narrative tree's examples failed with exit code: 101
```

The step renders correctly. Its coverage line still prints first, its failure names the page's
module, and its tail names the step by the claim it makes. The defect F2 records is not that
this step is wrong; it is that under `cargo xtask ci` nobody ever sees it.

### What the failure actually identified

Two different strings identify the failure, they disagree with each other, and only one of
them is stable. Both are recorded as measured.

**The doctest name.** Identical in all four invocations above, and confirmed independently
against the harness's own registry:

```console
$ cargo test --locked -p xtask --doc -- --list
```

```text
xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9): test
```

| Element | Measured | Against architecture brief Note 3 |
| --- | --- | --- |
| File | `xtask\src\../../docs/append-conditions.md` | **Diverges.** Note 3 predicts the file is the harness, `xtask/src/narrative.rs`. Measured: the name is the harness's *directory* (`xtask\src\`), then `../../`, then the **page's own repo-relative path**. The harness's filename never appears. The prediction's consequence — a reader is shown a path prefixed with `xtask/src/` — is right; its subject is wrong. |
| Doctest name / module | `narrative::append_conditions` | **Confirmed.** One `#[cfg(doctest)]` module per page is what keeps the module the page's name. |
| Line | `9` | **Confirmed as forecast, and P4 holds.** `docs/append-conditions.md:9` is the fence's opening ` ```rust ` marker. It is *not* the broken assertion, which is at page line 13. The location is actionable at fence granularity, not statement granularity. |
| Does the line locate the broken assertion inside the page? | **No.** Line 9 opens the fence four lines above the assertion. | Recorded; AC-003 asks the question and this is the answer. |
| Failure kind | **Assertion panic**, exit code 101 | Fences are run, not merely compiled (P2). |

**The panic location, which is a second and different string.** It is not stable, and what
varies is `--show-output`:

| Invocation | Panic location |
| --- | --- |
| `cargo test --locked --workspace --all-features -- --show-output` (the `tests` step, and the only variant `cargo xtask ci` reached) | `xtask\src\../../docs/append-conditions.md:6:1` |
| `RUSTDOCFLAGS=-D warnings cargo test --locked -p xtask --doc` | `C:\Users\ryanm\AppData\Local\Temp\rustdoctestL4ULPt\doctest_bundle_2024.rs:9:1` |
| `RUSTDOCFLAGS=-D warnings cargo run --locked --quiet -p xtask -- narrative-doctests` | `C:\Users\ryanm\AppData\Local\Temp\rustdoctestODlOE4\doctest_bundle_2024.rs:9:1` |
| `RUSTDOCFLAGS=-D warnings cargo test --locked -p xtask --doc -- --show-output` | `xtask\src\../../docs/append-conditions.md:6:1` |

The last two rows are a controlled pair: identical program, arguments and environment except
for `-- --show-output`, and the panic location changes from a temporary bundle file to the
page's path. That is the whole of the comparison, and it is stated as a measurement rather
than as an explanation of rustdoc's internals.

Two things follow, both of which HS-S0145's limit 4 must carry:

1. **The panic's line number is not a page line.** `docs/append-conditions.md:6` is prose —
   *"writer that saw a consistent view cannot be overtaken between reading and"*. `6` counts
   inside rustdoc's synthesized doctest source, whose first two lines are the wrapper it adds
   around a fence body; the assertion is body line 4, hence 6. So in the one variant where the
   panic names a real file, it names it with a line that points at a different sentence.
2. **Without `--show-output` the panic names a temporary file that no longer exists** when the
   run ends — `…\Temp\rustdoctest<random>\doctest_bundle_2024.rs`. A contributor who runs
   `cargo test -p xtask --doc` while iterating gets a location they cannot open.

The stable, actionable identifier in every variant is the **doctest name**, and the part of it
that resolves the page is the module: `narrative::append_conditions`.

### Composition, checked against the design

`_design.md` `## Composition`, `## Hierarchy`, `## States` and `## Transience policy`, element
by element, against the bytes above. A divergence recorded here is a pass; a divergence
unrecorded would be the failure.

| Predicted element | Design source | Observed | Verdict |
| --- | --- | --- | --- |
| Banner first, on stdout, naming the step | `## Composition`; `main.rs:940` | `=== tests ===`, `=== the narrative tree's examples compile ===`, `=== every narrative page is checked ===` all printed before their step's work | **observed** |
| Two steps, two banners, so the banner alone says which half failed | `## Composition` rule 1 | The banners are distinct and correct — but the failing banner was a *third* step's, `=== tests ===`, which the design's manifest does not contain | **diverged** → F2 |
| A composed failure report, never a bare non-zero exit | `## States`, Error row | Banner → doctest roster → `failures:` block → per-test panic → `test result: FAILED` → tail. Composed. | **observed** |
| The failure body | `## Composition` | **rustdoc's** doctest report, not this repository's. The design says so; it is recorded here as the reason the two rows below diverge. | **observed (not owned)** |
| Two-line stderr tail: `xtask failed: {err:#}` then `{step} failed with {status}` | `## Composition`; mock finding 2; `main.rs:777`, `:963` | One line under `cargo xtask ci` — `xtask failed: tests failed with exit code: 101` — because both primitives compose into a single `eprintln!` when the failing step is not itself a nested `cargo run -p xtask`. The second line present is **cargo's** `error: process didn't exit successfully`. The predicted two-line shape *does* appear when the failing step is a nested `cargo run -p xtask`: counterfactual (b)'s tail is `xtask failed: the narrative tree's examples failed with exit code: 101`, again one xtask line. | **diverged** → F3 |
| Problem lines present only in a failure state | `## Transience policy` | Run 1 and run 3 emit no problem line from either narrative step; run 2's problem region exists only because a step failed | **observed** |
| Success summary is exactly one line | `## Transience policy` | `  1 pages, all consistent` — one line, two-space indent, the shape of `xtask/src/lint_constitution.rs:192` | **observed** |
| The probe skip line is never present | `## Transience policy`; `probe: None` at `main.rs:103-105` | `grep -c "^skipped"` returns 0 in all three runs | **observed** (P8) |
| Hierarchy: `{path}:{line}` primary by being first on the line | `## Hierarchy`, `gate-narrative-checker-step` | Holds for the *checker* surface, which emits no problem line here. On the **compile** surface the first visual row of the failure begins `test ` and the panic line begins `thread 'main' (23672) panicked at `, 34 characters before the location | **diverged** → F4 |
| Anti-pattern 7 — a gate failure whose first visual row does not begin with `path:line` | `## Anti-patterns` 7 | Breached on the compile surface, by rustdoc's own report. Pre-registered in this story's spec as a measured limit of the mechanism rather than a defect to fix | **diverged** → F4 |
| Anti-pattern 8 — a truncated problem list, any "… and N more" | `## Anti-patterns` 8 | No occurrence of `and N more`, `… and` or `... and` anywhere in any of the three logs | **observed** |
| Anti-pattern 9 — the mark that would assert the documentation is checked for correctness or comprehension | `## Anti-patterns` 9 | Nothing in any transcript asserts either | **observed** |

States **not** rendered by this story, named so a reader does not read the table above as
coverage: `fail-removed-item` (milestone 1's, via the temporary `MemoryEventStore::len`
rename), `fail-hidden-marker` (`hidden-content-resolution`'s), `fail-many`, `fail-one`,
`fail-empty-tree` and `fail-long-path` (claimed by nobody in this project), and
`rustdoc-reference-surface` (untouched by this project). The markdown surfaces
`narrative-page`, `narrative-scoped-page` and `narrative-tree-index` were broken and restored
inside this run and are unchanged as shipped.

### Density, measured

Counted off the captured bytes with `awk '{ print length($0) }'`, not by eye. The budget is
`_design.md` `## Density budget`, terminal table: line width 80 columns; location prefix
≤ 48 characters **inclusive** of the 16-character `xtask\src\../../` doctest prefix, i.e.
≤ 32 characters repo-relative.

| Line | Width | Budget | Result |
| --- | --- | --- | --- |
| `test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... FAILED` | 97 | 80 | over — soft-wraps |
| `---- xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) stdout ----` | 98 | 80 | over — soft-wraps |
| `    xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9)` | 85 | 80 | over — soft-wraps |
| `thread 'main' (23672) panicked at xtask\src\../../docs/append-conditions.md:6:1:` | 80 | 80 | exactly at budget |
| `test result: FAILED. 230 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out; finished in 17.49s` | 100 | 80 | over — soft-wraps |
| `xtask failed: tests failed with exit code: 101` | 46 | 80 | within |
| `error: process didn't exit successfully: \`target\debug\xtask.exe ci\` (exit code: 1)` | 83 | 80 | over — soft-wraps (cargo's line) |
| `  1 page(s)' examples enumerated` | 32 | 80 | within |
| `  1 pages, all consistent` | 25 | 80 | within |
| Widest line in run 2's doctest roster (`91-adapter-authoring-recipe.md`, constitution corpus) | 117 | 80 | over — soft-wraps; not this tree's page |

**The location prefix, the number the budget actually protects.**
`xtask\src\../../docs/append-conditions.md` is **41** characters: 16 of `xtask\src\../../`
plus 25 of `docs/append-conditions.md`. Against the budget: 41 ≤ 48 ✔, and repo-relative
25 ≤ 32 ✔. The fixture page's own path fits, with 7 characters of headroom on the compile
surface and 7 on the checker's.

**Where the location sits on the first visual row**, counted by column on the failing line
`test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... FAILED`:

| Span | Columns |
| --- | --- |
| `test ` | 1–5 |
| the 41-character path | 6–46 |
| ` - ` | 47–49 |
| `narrative::append_conditions` | 50–77 |
| `(line 9)` | 79–86 |
| ` ... FAILED` | 87–97 |

So at 80 columns the **path survives on the first visual row with 34 columns to spare, and the
line number does not**: the row breaks inside `(line 9)` at column 80. The page is identified
on the first row; the location inside it is pushed onto the second. That is a real, countable
consequence of rustdoc putting the module path between the file and the line, and it is
recorded as **F4** rather than rounded off.

**Truncation.** `grep -n "and [0-9]* more\|… and\|\.\.\. and"` over all three logs returns
nothing. No problem list was truncated and no transcript region above was elided inside a
quoted block; the abridgement between blocks is stated in § *How complete each transcript
below is* with the line counts and the reproducing commands.

**Green output.** Run 3's narrative checker output is exactly one line
(`  1 pages, all consistent`); the narrative compile step's own output opens with exactly one
coverage line (`  1 page(s)' examples enumerated`) and then hands the terminal to cargo's
doctest report, which is not this repository's to budget.

### Run 3 — recovered

```console
$ git checkout -- docs/append-conditions.md
$ cargo xtask ci
```

Exit **0**, in **1 m 58 s**. `git stash` was deliberately not used: it leaves state
`git status --porcelain` does not show, and AC-005 asserts the revert this record names.

The complete banner sequence — twenty-six banners, the same set and the same order as run 1:

```text
   4: === formatting ===
   6: === clippy (all targets, all features) ===
   9: === tests ===
4347: === each phase's proof artefacts ===
4402: === wasm32 build of the contract crate ===
4405: === wasm32 check of the conformance harnesses ===
4408: === wasm32 build of the Cloudflare adapter ===
4411: === wasm32 build of the Neon adapter ===
4414: === documentation ===
4429: === specification traceability ===
4436: === no retired rule is still live ===
4439: === no conformance rule reads a clock ===
4442: === no literal position values in the suite ===
4445: === every conformance rule has a changelog entry ===
4448: === the testkit carries its own version ===
4451: === happenstance-core names serde/alloc and base64/alloc ===
4454: === the Rust constitution is internally consistent ===
4457: === the narrative tree's examples compile ===
4474: === the constitution's examples compile ===
4723: === every narrative page is checked ===
4726: === documentation (no default features) ===
4731: === packaged artifacts carry their licences and README ===
4737: === feature powerset ===
4874: === wasm32 feature powerset ===
4927: === licences and advisories ===
5657: === docs.rs configuration (nightly) ===
```

`grep -c "^skipped"` returns **0**. Neither narrative step — nor any other step, including the
four whose `probe` is `Some(..)` — printed a skip line on this runner. Both narrative steps
carry `probe: None` (`xtask/src/main.rs:509`, `:551`), which makes the line structurally
impossible for them; `RUNBOOK.md:914-928`'s failure is not reproduced. This is the whole-gate
green, and the only run in this record that claims anything about the twenty-three steps after
the `tests` step.

The two steps this story observes, complete and unedited:

```text
=== the narrative tree's examples compile ===
  1 page(s)' examples enumerated
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.21s
   Doc-tests xtask

running 1 test
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 63 filtered out; finished in 0.02s

all doctests ran in 7.16s; merged doctests compilation took 6.62s
```

```text
=== every narrative page is checked ===
  1 pages, all consistent
```

The last line of the run is `all checks passed`.

**The coverage numbers at this sha, stated so a later reader can falsify them.** At
`6368e2b`, `docs/` holds **two** files — `docs/README.md` (the index, which the harness
deliberately does not register) and `docs/append-conditions.md` (one page). Therefore:

| Figure | Value at `6368e2b` | Primitive |
| --- | --- | --- |
| Narrative compile step, pages enumerated | **1** | `narrative-doctests`' own count |
| Narrative compile step, doctests run | **1 passed; 0 failed**, with 170 filtered out | rustdoc |
| Narrative checker, pages checked | **1** | `  {n} pages, all consistent`, modelled on `xtask/src/lint_constitution.rs:192` |
| Constitution checker, atoms | **27** | `  27 atoms, all consistent` |
| `spec-trace` | 200 clauses, 95 conformance rules, 58 e2e cases, **358 citations checked** | `specification traceability` |

If a later reader finds `docs/` holding six pages and this step still saying `1 pages`, the
number is the thing that told them. That is the whole argument of
`.kb/playbooks/verify-the-referent-and-report-coverage.md`, whose grounding measurement in
this repository is a citation parser that checked 84 of 338 citations while printing
"no problems found": a reader who is told a number can falsify it, and a reader told
"no problems found" cannot.

### Residue

After run 3, `git status --porcelain`:

```text
M  .redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl
?? .bklg/docs-that-teach/checked-documentation-surface/observed-failure-falsification/_falsification.md
```

`docs/append-conditions.md` is absent from that list, which is the whole of AC-005's claim:
the file was edited, observed, and restored, so git sees no change to it. The two entries that
remain are the redkiln CLI's own session telemetry and this record itself. No path under
`docs/`, `crates/`, `xtask/`, `spec/` or `standards/` is modified, and the commit that carries
this story writes only inside its own backlog folder.

The check a reviewer re-runs on the merged tree is
`git diff --stat HEAD~1 -- docs/ crates/ xtask/ spec/ standards/`, which must be empty. A
committed broken fixture would turn the gate red for every later story in this initiative and
would put this project inside the corpus `project.md:294` forbids it to write; that is why
AC-005 asserts the negative directly instead of inferring it from the PR boundary check.

### Findings routed to HS-S0145

Each finding is one stated fact with the line that carries it, under its own heading, because
`documented-blind-spots-and-their-proofs` consumes them as input and a vague finding here
becomes an invented one there.

#### F1 — the narrative tree's fences are run, not merely compiled

`assert_eq!(store.len(), 1)` type-checks and is false. The gate caught it, as an assertion
panic with exit code 101:

```text
thread 'main' (23672) panicked at xtask\src\../../docs/append-conditions.md:6:1:
assertion `left == right` failed
  left: 0
 right: 1
```

**Consequence for the limits list:** limit 1 ("code that still compiles while no longer
demonstrating the surrounding claim") must not be widened into "the gate cannot tell whether a
claim is true". It can, for any claim an example can *assert*. What it cannot see is a claim
the prose makes that the fence does not assert. That is a narrower and more honest limit than
Note 10 item 1's wording invites, and it is the distinction this run bought.

#### F2 — three `REQUIRED` steps compile the narrative tree, and the first of them is `tests`

Routed to **`pinned-narrative-tree-and-compiling-step`** (the owner of step ordering and of
`EC-005`) and to **HS-S0145** as a limits-list item. Not fixed here: EC-009 forbids a story
from repairing the thing it was written to test, and a fix would make every transcript above
describe a tree that never existed on any branch.

The observation: under `cargo xtask ci`, a broken narrative fence fails under
`=== tests ===` at step index 2. Neither `=== the narrative tree's examples compile ===`
(index 16) nor `=== the constitution's examples compile ===` (index 17) prints, because
`run_steps` `bail!`s at the first non-zero status. The mechanism is that `tests` is
`cargo test --locked --workspace --all-features`, and `cargo test` runs a lib target's
doctests; `xtask`'s lib target is the harness's doctest root.

**Consequence for the limits list:** the banner a contributor meets when they break a page is
not the narrative banner. Whatever HS-S0145 writes about attribution must say that, and it may
not repeat milestone 1's claim that ordering keeps a broken narrative fence under the narrative
banner — that claim is true of steps 16 and 17 and false of the gate.

> **Disposition addendum — 2026-08-17, from HS-S0145's review.** Appended rather than edited
> into the paragraphs above: nothing measured here is restated, corrected or softened, and the
> transcripts stand as they were run. What is added is the addressee the routing line lacked.
>
> The first recipient named above, `pinned-narrative-tree-and-compiling-step`, was **sealed at
> `6368e2b`** — the same commit this run measured — so it had no live inbox when this finding
> was written, and the finding existed only as prose in a merged story's evidence file. The
> two halves are now separated and each has an owner:
>
> - **The limits-list half landed.** It is the additive seventh limit in
>   `xtask/src/narrative.rs`'s `# What this does not verify`, cited to this finding, enumerated
>   as `NOTE_TEN`'s seventh entry and held there by
>   `lint_narrative::tests::limit_seven_names_the_step_that_fails_first_and_cites_the_run` and
>   its mutation arm. `documented-blind-spots-and-their-proofs`'s `_limits-evidence.md` carries
>   the reconciliation row and the full disposition. The claim this finding forbids repeating
>   was also still live in `narrative_doctests.rs`'s
>   `the_narrative_step_precedes_the_constitution_step` doc comment, and is corrected there
>   (recorded as **L6** in that record).
> - **The step-ordering half is owned by `FU-1` of `HS-P0020`**, recorded under
>   `## Follow-ups routed out of this project` in
>   `.bklg/docs-that-teach/checked-documentation-surface/project.md`. It carries the two `main.rs`
>   step comments that still state the disproved claim, because they are the step definitions
>   themselves and EC-009 keeps this project's stories out of them.

#### F3 — the failure's tail is one xtask line, not two

`_design.md` `## Composition` (and mock finding 2's correction) predicts a two-line tail:
`xtask failed: {err:#}` from the step's child process, then `{step} failed with {status}` from
the parent. Measured: **one** line carries both primitives —

```text
xtask failed: tests failed with exit code: 101
```

— because `eprintln!("\nxtask failed: {err:#}")` (`main.rs:777`) is rendering the very error
`bail!("{} failed with {status}", step.name)` (`:963`) produced, inside the same process. Two
xtask lines appear only when the failing step is itself a nested `cargo run -p xtask`, and
even then the second line is the outer process's. The line that follows in the log is
**cargo's** (`error: process didn't exit successfully: …`), not this repository's.

**Consequence:** a limits bullet or a design amendment that quotes the tail must quote one
line, and must not attribute cargo's line to `xtask`.

#### F4 — the compile surface cannot satisfy anti-pattern 7, and the line number wraps at 80 columns

Anti-pattern 7 forbids "a gate failure whose first visual row does not begin with `path:line`".
On the compile surface the body is rustdoc's, and it begins `test ` (5 characters) or
`thread 'main' (23672) panicked at ` (34 characters) before the path. Worse for the reader:
the path occupies columns 6–46 and `(line 9)` occupies 79–86, so an 80-column log breaks the
row **inside the line number**.

**Consequence:** limit 4 must say that the page is identified on the first visual row and the
line inside it is not, at 80 columns — and that this is rustdoc's report, which this repository
does not control, rather than a rule the checker is free to enforce.

#### F5 — the panic's file and line are unstable and, when they are a real path, the line is wrong

Four invocations, two answers, and the variable is `-- --show-output`. With it, the panic reads
`xtask\src\../../docs/append-conditions.md:6:1`; without it,
`C:\…\Temp\rustdoctest<random>\doctest_bundle_2024.rs:9:1`. And `6` is not the assertion's page
line (13); it counts inside rustdoc's synthesized doctest source, so it points at a sentence of
prose. The stable identifier in every variant is the doctest **name**, whose module
(`narrative::append_conditions`) resolves the page and whose `(line 9)` resolves the fence.

**Consequence:** limit 4 must be written from the doctest name, not from the panic line, and
must say the panic line is not usable as a location.

#### F6 — a residual carried forward, not resolved here

Milestone 1 flagged that the fixture page's sentence describes a *boundary* property while
`ES-40`'s normative text is about *completeness* (`spec/SPECIFICATION.md:4351`; the boundary
property is argued at `:4293` as the joint consequence of ES-38 and ES-40). This story broke
the fence's **assertion**, which is independent of that sentence, so the falsification is
unaffected either way. Recorded, not edited: `docs/append-conditions.md` is `_design.md`'s
literal artifact and two other stories are pinned to its exact bytes, so any correction is an
amendment by that record's owner.

### What this does not establish

**This observation establishes that the code inside `docs/append-conditions.md` still compiles
and, because rustdoc runs it, that the one claim its fence asserts still holds; it is silent
about whether the page teaches anybody anything, and HS-P0024's friction log is the only
instrument in this initiative that is about comprehension — nothing here substitutes for it.**

Three further limits of this record, stated because a record that does not state its own scope
is read as covering everything:

1. **Run 2's log is truncated at the failing step.** `run_steps` `bail!`s on the first non-zero
   status, so the red run says nothing about the twenty-three steps after `tests`. Only run 3
   is a whole-gate claim.
2. **One fixture page is not the corpus.** `docs/` held exactly two files at this sha, one of
   which is the index. Every number above is a number about a tree with one page in it, and
   nothing here predicts what the same machine does over twenty.
3. **One toolchain on one runner is not a guarantee about others.** Everything above was
   measured on `rustc 1.97.1` / `cargo 1.97.1`, host `x86_64-pc-windows-msvc`, on Windows 11
   under git-bash. The Windows-shaped doctest name (`xtask\src\../../`) is a property of that
   host; a reader on Linux will see `/` and should not read the separator as a defect. Whether
   `--show-output` governs the panic location the same way on another platform or another
   toolchain is unmeasured.

`_design.md` `## Anti-patterns` item 9 forbids "a badge, tick, shield or 'verified' mark
asserting the documentation is checked for correctness or comprehension". None appears in this
record, and the sentence just quoted is the only place those words occur in it.
