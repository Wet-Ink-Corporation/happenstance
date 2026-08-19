---
item: "HS-S0186"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Drill observation — 2026-08-18

Initiative DoD-4 is a *human-observed* obligation — "run and observed"
(`.bklg/docs-that-teach/initiative.md`, DoD-4) — and no per-commit check can retire it. This
file is the only record it will ever have, and project DoD item 3 reads it. Every block below
is pasted from a run in this session on the pinned toolchain; nothing is composed, and the
page's quoted block is taken **from here** rather than the other way round.

Two mounts are exercised throughout, and both are inside the one `"tests"` REQUIRED step
(`xtask/src/main.rs`, `cargo test --locked --workspace --all-features -- --show-output`), so no
gate step was added:

- **the page** — `docs/first-encounter.md` step 3, registered as
  `#[cfg(doctest)] mod first_encounter` in `xtask/src/narrative.rs`, run by
  `cargo test -p xtask --doc`;
- **the twin** — `crates/happenstance/tests/boundary_refusal.rs`, run by
  `cargo test -p happenstance --test boundary_refusal`.

## The starting tree

```console
$ git rev-parse --short HEAD
3e0a71e

$ git status --porcelain
?? crates/happenstance/tests/boundary_refusal.rs
?? xtask/tests/falsification_drill.rs
```

Both untracked paths are this story's own new files, authored before the drill was run: the
twin, and the drill's source-reading assertions. No tracked file was modified at the start.

Both mounts green before anything was touched:

```console
$ cargo test -p xtask --doc -- first_encounter
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 55) ... ok
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 16) ... ok
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 96) ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 168 filtered out

$ cargo test -p happenstance --test boundary_refusal
running 2 tests
test without_the_condition_the_same_append_is_accepted ... ok
test the_guarded_append_is_refused ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Direction one — the boundary removed

**The edit, applied at both mounts.** One expression, in one place each. On
`docs/first-encounter.md:112` and on `crates/happenstance/tests/boundary_refusal.rs:67`:

```text
-    match store.append(&[seat], Some(&condition)).await {
+    match store.append(&[seat], None).await {
```

The program still compiles and still runs. That is the whole reason this is the canonical edit
and "empty the query" is not: `Query::from_items([])` returns `Err(InvalidQuery::NoItems)`
(`crates/happenstance-core/src/query.rs:180-190`), which propagates through `?` and goes red for
a *constructor* reason, and deleting the `Query` binding outright is a build error. Neither
lets a reader recognise **the** failure.

**The page mount.** Verbatim, including the exit status and the location the runner names:

```console
$ cargo test -p xtask --doc -- first_encounter
running 3 tests
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 55) ... ok
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 16) ... ok
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 96) ... FAILED

failures:

---- xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 96) stdout ----
Test executable failed (exit status: 101).

stderr:

thread 'main' (56784) panicked at C:\Users\ryanm\AppData\Local\Temp\rustdoctestin4Fam\doctest_bundle_2024.rs:83:15:
the boundary did not hold: Ok(SequencePosition(2))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 96)

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 168 filtered out
error: doctest failed, to rerun pass `-p xtask --doc`
```

**The twin mount.** Same edit, same failure, at a location a reader can open:

```console
$ cargo test -p happenstance --test boundary_refusal
warning: unused variable: `condition`
  --> crates\happenstance\tests\boundary_refusal.rs:63:23
running 2 tests
test without_the_condition_the_same_append_is_accepted ... ok
test the_guarded_append_is_refused ... FAILED

failures:

---- the_guarded_append_is_refused stdout ----

thread 'the_guarded_append_is_refused' (24480) panicked at crates\happenstance\tests\boundary_refusal.rs:67:15:
the boundary did not hold: Ok(SequencePosition(2))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    the_guarded_append_is_refused

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

**Read the two results together, because that is where the finding is.** The refusal assertion
failed; `without_the_condition_the_same_append_is_accepted` **passed**. So the append itself is
fine, the store is fine, and the one thing that changed is the one thing that was removed. EC-001
did not fire: the boundary is load-bearing, not decorative, and the drill is the only instrument
in this repository that could have told us either way.

**Reproducibility (NF-005).** The failing direction was run a second time on the page mount
before anything was reverted:

```console
$ cargo test -p xtask --doc -- first_encounter
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 96) ... FAILED
thread 'main' (65252) panicked at C:\Users\ryanm\AppData\Local\Temp\rustdoctestMcnj1M\doctest_bundle_2024.rs:83:15:
the boundary did not hold: Ok(SequencePosition(2))
```

Two runs, and the two lines that identify the failure are byte-identical: the test-name line
naming `narrative::first_encounter (line 96)`, and `the boundary did not hold:
Ok(SequencePosition(2))`. What differs is the process id (`56784` → `65252`) and rustdoc's
temporary bundle directory (`rustdoctestin4Fam` → `rustdoctestMcnj1M`). **That is which line is
*the* line**, and it is why the page quotes the panic message and the `note:` line beneath it
rather than the stanza above them.

**EC-004 fired, and this is the routing rather than a softened drill.** The panic's own
`file:line` on the page mount is
`C:\Users\ryanm\AppData\Local\Temp\rustdoctest…\doctest_bundle_2024.rs:83:15` — a path that
exists on one machine for the length of one run, is not `docs/first-encounter.md`, and is not a
location any reader can open. `xtask/src/narrative.rs` already carries this as a measured
standing limit of the mechanism ("the panic's own `file:line` is worse and is not a location at
all: it is a temporary bundle file under the OS temp directory"), which is why the doctest's
*module name* is the only stable identifier and why one module per page is the whole of what
registration buys.

Disposition: **routed to HS-P0020** as a substrate finding (project DoD item 9). It is a
property of the harness, not of this drill, and it is not a licence to paraphrase — the page
quotes two contiguous, verbatim lines and names in prose which doctest reports them, and this
record carries the transcript in full, temp path and process id included. The twin exists partly
for this reason: it reports the same failure against `crates/happenstance/tests/boundary_refusal.rs:67`,
a real file at a real line.

## Direction two — the boundary restored

**The revert.** One operation, the exact inverse of the edit, at each mount:

```text
-    match store.append(&[seat], None).await {
+    match store.append(&[seat], Some(&condition)).await {
```

Nothing else was touched. No `Cargo.lock`, no generated file, no second instruction.

```console
$ cargo test -p xtask --doc -- first_encounter
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 96) ... ok
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 16) ... ok
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 55) ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 168 filtered out

$ cargo test -p happenstance --test boundary_refusal
running 2 tests
test without_the_condition_the_same_append_is_accepted ... ok
test the_guarded_append_is_refused ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**The closing clean-tree check.** EC-005 did not fire:

```console
$ git status --porcelain
?? crates/happenstance/tests/boundary_refusal.rs
?? xtask/tests/falsification_drill.rs
```

`docs/first-encounter.md` is absent from that list, which is the check that matters: it is
byte-identical to its committed state after the edit and the revert, so the drill's one stated
revert returned the tracked tree to exactly where it started. The two paths that remain are this
story's own new files, present before the drill began and unchanged by it.

### Re-run at the story checkpoint, for a literally empty tree

AC-004 asks for `git status --porcelain` **empty**, and the run above was performed while this
story's own two files were still untracked. So the whole drill was performed a second time on
the committed checkpoint `b64448a`, where nothing is outstanding before it starts. Same edit,
same revert, nothing else touched:

```console
$ git status --porcelain
$ git rev-parse --short HEAD
b64448a

$ # edit applied at both mounts: Some(&condition) -> None

$ cargo test -p xtask --doc -- first_encounter
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 96) ... FAILED
---- xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 96) stdout ----
the boundary did not hold: Ok(SequencePosition(2))
test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 168 filtered out

$ cargo test -p happenstance --test boundary_refusal
test without_the_condition_the_same_append_is_accepted ... ok
test the_guarded_append_is_refused ... FAILED
thread 'the_guarded_append_is_refused' (5552) panicked at crates\happenstance\tests\boundary_refusal.rs:78:15:
the boundary did not hold: Ok(SequencePosition(2))
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

$ # revert applied at both mounts: None -> Some(&condition)

$ cargo test -p xtask --doc -- first_encounter
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 55) ... ok
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 16) ... ok
test xtask\src\../../docs/first-encounter.md - narrative::first_encounter (line 96) ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 168 filtered out

$ cargo test -p happenstance --test boundary_refusal
test without_the_condition_the_same_append_is_accepted ... ok
test the_guarded_append_is_refused ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ git status --porcelain
$
```

**`git status --porcelain` prints nothing.** One stated revert, and the tree is byte-identical
to the checkpoint it started from — no `Cargo.lock` touch, no generated file, no second
instruction. The twin's panic now reports `boundary_refusal.rs:78:15` rather than the `:67:15`
of the first run: the `#[allow(clippy::cloned_ref_to_slice_refs, …)]` block and its reason were
added between the two runs, moving the assertion down eleven lines. The message and the test
name — the two things a reader is asked to recognise — are byte-identical across both runs, and
that is NF-005 measured a second time on a different axis than the process id.

## Cost, and what was not added

`NF-002` asks for the gate cost. The two scenarios the slice adds are in-memory and construct
and drop a `MemoryEventStore` per test: the twin's two tests run in **0.00s** wall clock as
libtest reports them, and the page's three doctests are inside a doctest run whose *compilation*
dominates entirely (`merged doctests compilation took 5.31s` of a 5.85s total, for the whole
231-doctest corpus). No new dependency is compiled for either.

`NF-001` — `REQUIRED` in `xtask/src/main.rs` is unchanged by this story; the falsifying check is
the existing `"tests"` step. `NF-003` — no public item added, changed or removed, and
`crates/happenstance/Cargo.toml` carries no line from this story. HS-P0020's
`IGNORE_ALLOWANCES` is still `&[]` and no fence at either mount carries `ignore`, `no_run` or
`compile_fail`.

## Findings routed

| # | finding | routed to |
| --- | --- | --- |
| F1 | The page mount's panic reports against a rustdoc temporary bundle under the OS temp directory rather than against `docs/first-encounter.md`, so the only stable identifier is the doctest's module name. Recorded as EC-004 above with both transcripts. | **HS-P0020** — a property of the harness, already stated as a standing limit in `xtask/src/narrative.rs`; the finding here is that a reader following a drill meets it |
| F2 | The design and `merge-forward-preflight/_baseline.md` both expect step 3's program to exist **twice**, the second copy being the crate-root fence. It is not there: `boundary-refusal-encounter/_conditions.md` **BC-002** records why the merged crate root cannot carry it. The twin therefore lives in `crates/happenstance/tests/boundary_refusal.rs` instead — the placement this story's own PR boundary pre-authorises and its EC-002 pre-scopes — and it is *stronger* than the doc fence would have been: it also asserts acceptance-without, which no single doc fence can. | **the `_design.md` sign-off owner**, with BC-002; nothing here waits on it, because AC-005's proof exists at two real mounts today |
| F3 | Removing the condition leaves `let condition = …` unused, so the twin emits `warning: unused variable: condition` during the drill. Harmless and expected — it is a warning, not an error, so the reader still reaches the assertion failure rather than a build error, which is exactly what the drill needs. Recorded so nobody reads it as the failure. | none — stated, not routed |
