# Limits evidence — what was measured, and what every delivered check does not verify

The companion to the `# What this does not verify` sections in
`xtask/src/narrative.rs` and `xtask/src/lint_narrative.rs`. The limits are stated **in those
modules**, where a contributor already is; this file holds the measurements behind two of them
and the reconciliation that says the list is complete. A limit whose measurement lives only in
a terminal someone closed is a limit nobody can re-check.

Like `observed-failure-falsification`'s `_falsification.md`, this is a **dated instrument**. A
later tree that makes a transcript here stale gets a second dated section from a second run;
the first is never rewritten to agree with it.

---

## Run of 2026-08-17

### Provenance

| What | Value |
| --- | --- |
| Date | 2026-08-17 |
| Commit (`git rev-parse HEAD`) | `b0bb9bdcbfa840db518a8fec6ea8df0b05b8cdfe` |
| Branch | `initiative/docs-that-teach` |
| Working directory | `D:\repos\happenstance\.claude\worktrees\docs-that-teach` (the repository root) |
| Toolchain | `rustc 1.97.1 (8bab26f4f 2026-07-14)`, `cargo 1.97.1 (c980f4866 2026-06-30)`, host `x86_64-pc-windows-msvc` |
| Toolchain source | `rust-toolchain.toml` — `channel = "1.97.1"`. ADR-0029's MSRV floor and this pin currently coincide; the probe names the pin as the toolchain of measurement and does not move it |
| Shell | git-bash on Windows 11 (`MINGW64_NT-10.0-26200`) |

Every transcript below names the exact command that produced it. A measurement without its
toolchain is a claim about nothing, and a measurement without its command is one nobody can
re-take.

---

## Limit 3 — the `RUSTDOCFLAGS` probe, re-run against the step as wired

### Why it had to be re-run rather than cited

This repository held two claims about `RUSTDOCFLAGS` and they did not agree.

- `xtask/src/constitution.rs:31-36` records a probe of its own: *"`RUSTDOCFLAGS=-D warnings`
  recovers rustc's default-on lints inside a doctest — a probe confirmed `non_snake_case` fails
  the build under it — but not the workspace's `[lints]` table and not clippy."*
- The upstream reports say `cargo test --doc` drops the variable
  (rust-lang/cargo#13697 → rust-lang/rust#67533, still open; carried into this initiative's
  research at `_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md`).

Architecture brief Note 10 item 3 forbids copying either and asks for the measurement. Two
further reasons this step needed its own: it reaches `rustdoc` through an extra
`cargo run -p xtask` hop the earlier probe never had, and the toolchain has moved to 1.97.1,
where rustdoc **merges** doctests into one bundle — a mechanism that did not exist when the
earlier probe was taken.

### The stimulus, and the control that proves it is a stimulus

A probe whose trigger does not fire measures nothing, so the trigger was confirmed first,
outside the doctest machinery entirely:

```console
$ rustc --edition 2024 --crate-type bin -o probe_control.exe probe_control.rs
```

```text
warning: variable `notSnakeCase` should have a snake case name
 --> probe_control.rs:2:9
  |
2 |     let notSnakeCase = 1;
  |         ^^^^^^^^^^^^ help: convert the identifier to snake case: `not_snake_case`
  |
  = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: 1 warning emitted
```

`probe_control.rs` is the fence body wrapped in `fn main() { … }`, which is what rustdoc does
to a doctest. `non_snake_case` is warn-by-default, it is not in the `unused` group rustdoc
allows, and it fires on this exact snippet. That is the stimulus the three observations below
apply.

The probe fence was added to `docs/append-conditions.md` for the duration of the run and
reverted in the same change. Nothing in the merged commit carries it — `git status --porcelain`
was clean of `docs/` afterwards. The transcripts are the artefact; the fences are not.

### Observation (a) — a rustc default-on lint under the step's own `RUSTDOCFLAGS`

The step exactly as `REQUIRED` declares it (`xtask/src/main.rs:496-509`): same program, same
arguments, same `env`.

```console
$ RUSTDOCFLAGS="-D warnings" cargo run --locked --quiet -p xtask -- narrative-doctests
```

```text
  1 page(s)' examples enumerated
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
   Doc-tests xtask

running 2 tests
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 18) ... ok
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 63 filtered out; finished in 0.01s

all doctests ran in 2.57s; merged doctests compilation took 2.12s
```

Exit **0**. The load-bearing lines are `(line 18) ... ok` — the probe fence compiled and ran —
and the *absence* of any `warning:` line at all. `-D warnings` did not turn the lint into an
error, and nothing surfaced it as a warning either.

### Observation (b) — the same fence, with the step's `RUSTDOCFLAGS` removed

```console
$ cargo run --locked --quiet -p xtask -- narrative-doctests
```

```text
  1 page(s)' examples enumerated
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.17s
   Doc-tests xtask

running 2 tests
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 18) ... ok
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 63 filtered out; finished in 0.03s

all doctests ran in 4.74s; merged doctests compilation took 4.00s
```

Exit **0**, and byte-identical in every respect that matters. The contrast the design expected —
denied with the variable, warned without it — does not exist: **setting `RUSTDOCFLAGS=-D
warnings` makes no observable difference to this step.**

### Observation (b′) — the extra `cargo run -p xtask` hop is not the cause

The harness's earlier wording blamed the hop. It is not the hop. The same fence, under the
argv the constitution's step uses, with no `xtask` process in the middle:

```console
$ RUSTDOCFLAGS="-D warnings" cargo test --locked -p xtask --doc
```

Exit **0**; `169 passed; 0 failed; 3 ignored` in the merged group and `63 passed` in the
standalone group, with no `warning:` line anywhere in the output. The hop is exonerated and the
harness's "unmeasured, and the hop is why" sentence is replaced by this measurement.

### Observation (c) — a clippy-only lint, under the step as wired

The fence body was swapped for `let value: Option<u8> = Some(1); assert_eq!(value.unwrap(), 1);`
— `clippy::unwrap_used` is `deny` in the workspace `[lints]` table (`Cargo.toml:122`).

```console
$ RUSTDOCFLAGS="-D warnings" cargo run --locked --quiet -p xtask -- narrative-doctests
```

```text
  1 page(s)' examples enumerated
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.25s
   Doc-tests xtask

running 2 tests
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... ok
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 18) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.06s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 63 filtered out; finished in 0.04s

all doctests ran in 7.30s; merged doctests compilation took 5.97s
```

Exit **0**. Unenforced, as Note 10 item 2 predicts and for the reason it gives: `cargo clippy`
does not lint doctests at all.

### The finding, stated rather than reconciled

**On 1.97.1, against the narrative compile step as `REQUIRED` declares it,
`RUSTDOCFLAGS=-D warnings` reaches nothing inside a narrative fence.** Not the workspace
`[lints]` table, not clippy — those were already known — and *not rustc's own default-on lints
either*, which is the half `xtask/src/constitution.rs:31-36` recorded as recovered.

This is EC-001's case: the re-run agrees with neither prior claim as stated. It is written down
as what happened and is **not** reconciled against either, and limit 3 is not softened into
"may not". Three things are deliberately *not* claimed here:

- **Not that the earlier probe was wrong when it was taken.** It is undated and names no
  toolchain, and 1.97.1's rustdoc merges doctests into one bundle — the
  `merged doctests compilation took …` line in every transcript above shows the mechanism is
  active. Whether the merge is the cause is **unmeasured**; naming it as the cause would be
  exactly the copying Note 10 forbids, one level up.
- **Not that `constitution.rs:31-36` should be edited.** That module is
  `standards/rust/`'s and its sentence is about its own step; correcting it is that module's
  owner's change, made against its own re-run. Recorded here and routed, not fixed.
- **Not that this generalises to other toolchains or platforms.** One runner, one pin.

**What changed in the modules because of it.** `xtask/src/narrative.rs` limit 3 now states the
measurement and cites this file. `xtask/src/narrative_doctests.rs`'s "unmeasured here, and the
extra hop is why" sentence is replaced by the measurement, because observation (b′) shows the
hop was not the variable.

---

## Limit 5 — the `text` fence, walked rather than asserted

### The fixture

`docs/text-fences.md`, retained. Nineteen characters of repo-relative path against a budget of
32, no directory level under `docs/`, H1 `Fences the compiler never sees` at 30 characters
against a budget of 40, registered in `xtask/src/narrative.rs` in both directions
(`include_str!` and `mod text_fences {`), one appended row in `docs/README.md`'s two-column
table with the row already there left where it was, and no `HIDDEN_MARKERS` token anywhere. Its
only fence is tagged ` ```text ` and its body is deliberately false about the library:

```text
let store = MemoryEventStore::new();
assert_eq!(store.len(), 7);
```

`MemoryEventStore::new()` returns an empty store, so `store.len()` is `0`. That is exactly the
break `observed-failure-falsification` applied inside a ` ```rust ` fence, where it failed the
gate with an assertion panic and exit 101.

### The walk

```console
$ cargo xtask narrative
```

```text
  2 pages, all consistent
```

```console
$ RUSTDOCFLAGS="-D warnings" cargo run --locked --quiet -p xtask -- narrative-doctests
```

```text
  1 page(s)' examples enumerated
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.25s
   Doc-tests xtask

running 1 test
test xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.02s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 63 filtered out; finished in 0.03s

all doctests ran in 3.50s; merged doctests compilation took 2.96s
```

Both green, and the whole gate is green with the fixture in the tree (`cargo xtask ci --fast`,
below). **Limit 5 is real, not plausible**: a Rust example whose every claim is false about the
library sits in the pinned tree, is walked by the checker, is handed to no compiler, and no
problem line is emitted about it.

Two numbers in those two transcripts are the proof, and they disagree on purpose. The checker
says **2 pages**; the compile step says **1 page(s)' examples enumerated**. Both are correct:
the checker counts pages in the tree, and the compile step counts pages that *produce a
doctest*, which a `text`-only page does not. That gap is recorded as **finding L2** below.

### The pin that makes it rot loudly

`xtask::lint_narrative::tests::the_text_fixture_is_not_flagged_by_the_real_fence_walk` runs the
**real** `check_page` over the real fixture's real bytes and asserts the walk reports nothing.
It first asserts the fixture still carries a ` ```text ` fence, so it cannot pass vacuously
against a page somebody edited. Its failure message is an instruction rather than a complaint:

> limit 5 is closed: the fence walk now reports a `text` fence. Delete limit 5 from
> `xtask/src/lint_narrative.rs`'s `# What this does not verify` section in this same change,
> and delete this test with it — the limits section is now wrong.

That is RS-81-1's second half made mechanical: the day someone teaches the walk to inspect
`text` fences, the build tells them the documentation is now wrong and which sentence to
delete.

---

## Limit 4 — the forecast, the record, and which one won

Architecture brief Note 3 forecast that a fence failure inside an `include_str!` page is
reported **against the harness file**, `xtask/src/narrative.rs`, with the page resolved by
module name and the line resolved inside the page.
`observed-failure-falsification`'s `_falsification.md` ran the real thing at `6368e2b` and
measured it. Where they disagree, the record wins (EC-004), and limit 4 is worded from the
record.

| Element | Note 3's forecast | Measured | Disposition |
| --- | --- | --- | --- |
| The file the report names | `xtask/src/narrative.rs` | `xtask\src\../../docs/append-conditions.md` — the harness's *directory*, then `../../`, then the **page's own repo-relative path**. The harness's filename never appears. | **Forecast wrong.** Limit 4 now says the path is the page's own, reached through the harness's directory. |
| How the page is resolved | by module name | `narrative::append_conditions` | **Confirmed.** This is the stable identifier and limit 4 says so. |
| How the location is resolved | line inside the page | `(line 9)` — the **opening line of the fence**, not the failing statement at page line 13 | **Confirmed in kind, corrected in grain.** Limit 4 says the line is the fence's opener. |
| The panic's own `file:line` | not forecast | Unstable, and unusable either way: a temporary `…\Temp\rustdoctest<random>\doctest_bundle_2024.rs:9:1`, or — under `--show-output` — the page's path with a line counted inside rustdoc's synthesized doctest source, landing on a sentence of prose | **New.** Limit 4 states it, because a reader who tries to open that location wastes the trip. |

Two further findings from that record are carried into the modules rather than lost:

- **Fences are run, not merely compiled** (`_falsification.md` F1). A false-but-compiling
  assertion fails the gate with an assertion panic. So limit 1 is **narrower** than Note 10's
  wording invites: the machine can tell whether a claim an example *asserts* is true. What it
  cannot see is a claim the surrounding prose makes that the fence never asserts. Limit 1 in
  `xtask/src/narrative.rs` is worded to that narrower, true statement.
- **Anti-pattern 7 cannot be satisfied on the compile surface** (`_falsification.md` F4).
  rustdoc's report begins `test ` or `thread 'main' (…) panicked at ` before the path, and at 80
  columns the row breaks inside `(line 9)`. That is rustdoc's output, which this repository does
  not control; it is a measured limit of the mechanism and not a rule the checker can enforce.

---

## The completeness reconciliation

Every check this project delivered, and whether it has a limit on the record. A check with an
unstated limit is the guarantee-by-silence RS-81-1 names, and it fails this story (EC-009).

| Delivered check | Where | Its limit, and where it is stated | Note 10 item |
| --- | --- | --- | --- |
| Pinned tree (`TREE`) | `lint_narrative::check` | Moving the tree without editing the constant is a hard error naming the path — this is the check with **no blind spot of its own**, because the failure mode it guards is exactly the one it detects. Recorded as having none. | — |
| Empty-tree guard | `guard_not_vacuous` | None of its own. A tree holding only its index is still vacuous and is rejected; the guard cannot be satisfied by an empty green. | — |
| Bidirectional registration | `check_registration` | **Stated** in `xtask/src/lint_narrative.rs`: the harness is matched as *text*, so reformatting it — an `include_str!` split across lines, a `mod` line that does not start with `mod ` after trimming — makes a registered page look unregistered. | not one of the six; module-specific |
| Registration's guarantee | `check_registration` | **Stated**: registration proves a page is *compiled*, never that it is *correct*. What a compiled fence does and does not establish is limits 1-4, stated in `xtask/src/narrative.rs` and deliberately not restated here. | 1 |
| Fence discipline (tagging) | `check_fences` | **Stated**: a fence tagged `text` is neither compiled nor flagged, and a block indented four spaces is invisible to the walk while rustdoc still compiles it. | **5** |
| Allowance sweep | `check_allowances` | **Stated**: the reason text is prose this module never interprets; the list narrows the `ignore` hole and does not close it. | not one of the six |
| Hidden markers | `check_hidden_markers` | **Stated**, three ways: an eighth spelling is invisible, the rule is scoped to `TREE` by design, and the scan reads source and cannot know what a renderer does with it. | not one of the six |
| Citation resolution | `check_citations` | **Stated**, four ways: a resolving citation says nothing about the sentence above it; a clause-shaped token inside a fence is not told apart from prose; a near-miss in an undeclared family is silent; a doubly-declared id collapses upstream. | not one of the six |
| Frozen-MUST pin | `check_pin` | **Stated**, three ways: an anchor can survive while the reasoning around it is rewritten; a clause wording its obligation outside `DOCUMENTATION_OBLIGATIONS` is never a candidate; the pin proves a discharge is *present*, never *adequate*. | not one of the six |
| Count agreement / coverage line | `summary` | **Stated** as finding **L2** below and in `xtask/src/narrative_doctests.rs`: the compile step's enumerated count counts pages that produce a doctest, not pages the harness registers. | new, additive |
| Fence compiling | `narrative_doctests::run` | **Stated** in `xtask/src/narrative.rs`: limits 1, 2, 3, 4 and 6. | 1, 2, 3, 4, 6 |
| The compile step's **attribution** | `narrative_doctests::run`'s banner, `run_steps` (`xtask/src/main.rs:963`) | **Stated** in `xtask/src/narrative.rs` as the additive **limit 7**: a broken fence in this tree does not fail under this step's banner. | **7**, new and additive (EC-005) |

**The additive seventh limit, added (EC-005).** The reconciliation above was first written
listing five limits against the compile step and no limit against its *attribution* — which
left the one thing `observed-failure-falsification` called the most valuable output of its run
(`_falsification.md` **F2**) reachable only as prose in a merged story's evidence file. That is
the guarantee-by-silence this story exists to prevent, one level up, so it is added rather than
dispositioned away.

**What it says.** Under `cargo xtask ci` a broken narrative fence fails under `=== tests ===`
at index **2**, because that step is `cargo test --locked --workspace --all-features` and
`cargo test` compiles a lib target's doctests — `xtask`'s lib target being where the harness is
declared. `run_steps` `bail!`s at the first non-zero status, so neither of the tree's own
banners prints at all. Milestone 1's claim that step ordering "keeps a broken narrative fence
under the narrative banner" is true of the two compile steps and **false of the gate**, and
limit 7 is worded to say so.

*One index correction, which changes nothing.* F2's prose puts the two compile steps at
`REQUIRED` indices 16 and 17. The record's own banner list — machine output, and so what
governs — places them eighteenth and nineteenth of twenty-six banners, which is indices **17 and
18**; `xtask/src/main.rs`'s array agrees. `tests` at index **2** is right in both readings and
is the whole of the finding. Recorded here rather than corrected in `_falsification.md`, whose
measurements are never edited to agree with a later reading — which is why limit 7's prose and
`the_narrative_step_precedes_the_constitution_step`'s doc comment name the two steps rather than
number them.

**Where it landed, and what holds it there.**

| Half | Where |
| --- | --- |
| The bullet | `xtask/src/narrative.rs` `# What this does not verify`, immediately after limit 4 — the other attribution limit — so limit 6 stays the closing sentence in both modules and none of the six is reordered |
| Its evidence line | `_falsification.md` finding **F2**, cited in the bullet; `FALSIFICATION_RECORD` in `xtask/src/lint_narrative.rs` asserts the citation is present |
| Its enumeration | a seventh `NOTE_TEN` entry (`n: 7`, owner `HARNESS`), so the presence, instrument and no-restatement checks all cover it |
| Its presence assertion | `::tests::limit_seven_names_the_step_that_fails_first_and_cites_the_run` (the bullet must name `tests`, `--workspace`, `_falsification.md` and `F2`) and `::tests::a_dropped_seventh_limit_is_rejected` (the mutation arm, so it cannot be dropped silently) |
| The array side | `narrative_doctests::tests::the_steps_that_compile_this_tree_are_pinned_in_gate_order` pins the three `REQUIRED` steps that hand these pages to rustdoc, in gate order, so the bullet and the array cannot drift apart again |

**And the defect it reports, routed with an addressee.** The step ordering itself — that the
gate's first compiler of these pages is a step with no narrative banner — is a defect of the
gate's shape, not of this check, and F2's original recipient
(`pinned-narrative-tree-and-compiling-step`) was sealed at `6368e2b` before the finding existed.
It is now owned by **`FU-1` of `HS-P0020`**, recorded in the project charter's
`## Follow-ups routed out of this project`, and cited from F2's disposition line in
`_falsification.md`. Repairing it here is forbidden twice over: EC-009 of
`observed-failure-falsification` (a story may not repair the thing it was written to test) and
the fact that every transcript in that record would then describe a tree that never existed.

**The candidate seventh limit, disposed of.** A *different* candidate, and it is not what
limit 7 became. The research names `compile_fail` on stable
asserting only *that* compilation failed and not *why* — the error-code form being nightly-only
and "unlikely to be stabilized"
(`_discovery/research/02-compiled-prose-tooling-mdbook-test-doc-comment-skeptic-doc-i.md`).
**Disposition: not applicable to this tree, and it is not one of the bullets.** The delivered
fence walk does permit `compile_fail` (`is_rustdoc_tag`, `xtask/src/lint_narrative.rs:719-728`)
and it goes further than the research's concern: a `compile_fail` fence carrying an error code
the prose never names is a problem, and a bare error code without `compile_fail` is a problem
(`an_error_code_without_compile_fail_is_a_problem`,
`a_compile_fail_claiming_a_code_the_prose_never_names_is_a_problem`). So the narrative tree's
`compile_fail` fences are held to naming their code in prose, which is the mitigation the
research's limit asks for — and no page in `docs/` uses `compile_fail` today. Recorded rather
than added, so a future page that does can re-open it against this paragraph.

### Three findings this reconciliation turned up

**L1 — `narrative_doctests.rs` carried two sentences the runs measured to be false.** Its limit
4 read *"The file a failure names is the harness, not the page"* and its limit 3 read *"What
`RUSTDOCFLAGS=-D warnings` enforces inside a narrative fence is unmeasured here … the extra
`cargo run -p xtask` hop"*. The first is refuted by `_falsification.md`; the second by
observations (a), (b) and (b′) above, which also exonerate the hop. Both were corrected in this
change to point at, or state, the measurement. Leaving a known-false limit in a module this
project added would be the exact defect this story exists to prevent, one level up.

**L2 — the compile step's coverage number counts doctests, not registrations.** With two pages
registered, the checker prints `2 pages, all consistent` and the compile step prints
`1 page(s)' examples enumerated`. A page whose fences are all `text` is registered, walked, and
absent from the compile step's number. That is not a defect — the number is a fact about what
rustdoc was given — but a reader who reads it as "pages in the tree" will under-count, so it is
now stated in `xtask/src/narrative_doctests.rs` beside the count it describes.

**L6 — the same module carried a *third* false sentence, and L1's pass missed it.** The doc
comment on `the_narrative_step_precedes_the_constitution_step` read *"`run_steps` bails at the
first failing step, so ordering is the whole of what keeps a broken narrative fence under the
narrative banner"* — the claim `_falsification.md` **F2** states in as many words is "true of
steps 16 and 17 and false of the gate" (F2's words, and its index correction is above), and
which F2 says may not be repeated. It survived L1 because L1 swept the module's
`# What this does not verify` bullets and not its test doc comments. Corrected to L1's standard: the doc comment now says the assertion orders those two
steps only, names `tests` as where a broken narrative fence actually fails, and cites F2 as
the measurement. The assertion itself, and every step name, argument and `env` entry, are
untouched — the boundary forbids changing them, and EC-009 forbids repairing the ordering
here. The new sibling assertion is what keeps the corrected sentence and the array in step.

---

## What this record does not establish

**Nothing in this file, and nothing in either module's limits section, is evidence that any page
in `docs/` teaches anybody anything.** Limit 6 says so unhedged in both modules; HS-P0024's
friction log is the instrument that is about comprehension, and no measurement here substitutes
for it.

Three limits of the measurements themselves:

1. **One toolchain, one runner.** Everything above is `rustc`/`cargo` 1.97.1 on
   `x86_64-pc-windows-msvc`. The `xtask\src\../../` spelling is Windows'; a reader elsewhere
   will see `/`, and the separator is not a defect.
2. **The `RUSTDOCFLAGS` finding is an observation, not a mechanism.** Merged doctests are active
   and the earlier probe predates them, and that is as far as the evidence goes. Naming the
   merge as the cause would be the copying Note 10 forbids.
3. **One `text` fixture is not the corpus.** It proves the hole exists. How often a real
   teaching page falls into it is a question for HS-P0021's page-need discipline and for the
   friction log.

**Anti-pattern 10, checked mechanically.** `git ls-files` matching `.css`, `book.toml`, `book/`,
`site/` or `_site/` returns **0** rows at `b0bb9bd`. No second rendered surface exists that
could carry a claim nothing checks.
