# Fence inventory — every fenced block this project authored

Companion to `spec.md`, beside `_ledger.md`. It carries **`§ Fences`**, **`§ Drills`** and
**`§ Routing`**, and it is read cold at closeout (NF-002): every claim below resolves from this
file plus the command in its own row, with no re-run of the audit.

Read `§ Fences` for what exists, `§ Drills` for what was made to fail, `§ Routing` for what this
story could not close. `_citations.md` beside it carries the clause half.

**Dated 2026-08-19**, against `initiative/docs-that-teach` at `9dc139a`, on the pinned
`rust-toolchain.toml` **1.97.1**. Every command below was run from the repository root of the
worktree `D:/repos/happenstance/.claude/worktrees/docs-that-teach`.

**`git grep`, not `rg`.** `rg` is this repository's convention in prose and is **not on this
machine's `PATH`** — `merge-forward-preflight/_baseline.md:90-95` established that a
re-derivation command which only runs in one agent's sandbox is a claim rather than a check.
Every re-derivation column below is `git grep -n`, a `cargo` subcommand, or `cargo test -- --list`.

---

## § Substrate confirmation

EC-001 first, because a drill run before the enforcer exists cannot distinguish its own failure
from an inherited one, and because `project.md:345-350` refuses the weaker criterion that would
otherwise fill the hole.

| what EC-001 requires | found | how it was re-derived |
| --- | --- | --- |
| `the narrative tree's examples compile` in `REQUIRED` | **present**, `xtask/src/main.rs:571` (`name: narrative_doctests::STEP`) | `git grep -nF "name: narrative_doctests::STEP," -- xtask/src/main.rs` |
| `every narrative page is checked` in `REQUIRED` | **present**, `xtask/src/main.rs:614` (`name: lint_narrative::STEP`) | `git grep -nF "name: lint_narrative::STEP," -- xtask/src/main.rs` |
| the step banners those constants spell | `"the narrative tree's examples compile"` (`xtask/src/narrative_doctests.rs:71`), `"every narrative page is checked"` (`xtask/src/lint_narrative.rs:351`) | `git grep -n "pub(crate) const STEP" -- xtask/src` |
| both inside `cargo xtask lints`, therefore inside the story grain | **yes**, `xtask/src/main.rs:1038-1047` (`lint_steps`) | `cargo xtask lints` prints both banners — see `§ Drills` D-1 |
| the harness file HS-P0020 pinned | `xtask/src/narrative.rs` — the filename the spec's mount point names; **no deviation to record** under EC-005 | `git grep -nF 'const HARNESS: &str = "xtask/src/narrative.rs";' -- xtask/src/lint_narrative.rs` → `:257` |
| `IGNORE_ALLOWANCES` | **empty**: `const IGNORE_ALLOWANCES: &[(&str, &str, &str)] = &[];` at `xtask/src/lint_narrative.rs:304` | `git grep -nF "const IGNORE_ALLOWANCES" -- xtask/src/lint_narrative.rs` |
| the tree the checker sweeps | `TREE = "docs"`, `xtask/src/lint_narrative.rs:239` | `git grep -nF 'const TREE: &str = "docs";' -- xtask/src/lint_narrative.rs` |

**EC-001 did not fire. EC-005 did not fire.** Both narrative steps are `REQUIRED`, the harness is
at the filename the mount point names, and `IGNORE_ALLOWANCES` is the empty slice — so
`_design.md:206-208`'s "puts nothing on HS-P0020's enumerated allowance list" is true of the
constant itself and not only of this project's rows in it.

**EC-006 did not fire.** All three of this project's tree pages are registered, one
`#[cfg(doctest)] mod` per file, at `xtask/src/narrative.rs:135`, `:141` and `:150`. No
registration line had to be added, so the mount is *observed* rather than *made*:

```
git grep -n "mod first_encounter\|mod carry_your_invariant\|mod read_the_worked_example" -- xtask/src/narrative.rs
xtask/src/narrative.rs:135:mod first_encounter {
xtask/src/narrative.rs:141:mod carry_your_invariant {
xtask/src/narrative.rs:150:mod read_the_worked_example {
```

---

## § Fences

The enumeration is taken from `_design.md`'s `## Surfaces` block (`:45-65`) resolved to real
paths, **not** from a directory listing, so a designed-but-unauthored page would appear below as
a row with no file rather than as an absence nobody counted. All four surfaces exist; the fifth
file, `examples/course-subscriptions/src/overview.md`, is `_design.md:764-766`'s extraction and
is inventoried with them.

**Every fence, its class, its exerciser, and whether that exerciser can fail.**

| id | fence | info string, verbatim | execution class | the mechanism that exercises it | can that mechanism **fail** if the fence opts out? | hidden doctest lines |
| --- | --- | --- | --- | --- | --- | --- |
| **F-01** | `crates/happenstance/src/lib.rs:26` | *(empty — no info string)* | **executed** | `cargo test --locked --workspace --all-features` (`xtask/src/main.rs:158`), as the doctest named `crates\happenstance\src\lib.rs - (line 247)` | **no** — see D-2. The file is outside `TREE`, so no checker reads it; an `ignore` here is skipped by rustdoc and the gate exits 0 | **2**, both permitted (`:31` the `#[tokio::main]` scaffold, `:63` the `Ok(())`) |
| **F-02** | `docs/first-encounter.md:16` | `rust` | **executed** | `cargo xtask narrative-doctests`, as `narrative::first_encounter (line 16)`; walked as a file by `cargo xtask narrative` | **yes** | 0 |
| **F-03** | `docs/first-encounter.md:37` | `text` | **prose** | `cargo xtask narrative`'s fence walk only — `is_doctest` returns false and the body is never compiled (`xtask/src/lint_narrative.rs:731`) | **yes**, for the tag: an untagged or unrecognised info string is a hard error (`:812`, `:833`). Not for the body: a `text` fence's Rust is compiled by nobody, which is `docs/text-fences.md`'s retained subject | 0 |
| **F-04** | `docs/first-encounter.md:55` | `rust` | **executed** | `narrative::first_encounter (line 55)` | **yes** | 0 |
| **F-05** | `docs/first-encounter.md:78` | `text` | **prose** | as F-03 | **yes** (tag only, as F-03) | 0 |
| **F-06** | `docs/first-encounter.md:96` | `rust` | **executed** | `narrative::first_encounter (line 96)` — the refusal, asserted from inside the matched arm | **yes** | 0 |
| **F-07** | `docs/first-encounter.md:123` | `text` | **prose** | as F-03 | **yes** (tag only) | 0 |
| **F-08** | `docs/first-encounter.md:141` | `text` | **prose** | as F-03 | **yes** (tag only) | 0 |
| **F-09** | `docs/first-encounter.md:148` | `text` | **prose** | as F-03 | **yes** (tag only) | 0 |
| **F-10** | `docs/carry-your-invariant.md:60` | `rust` | **executed** | `narrative::carry_your_invariant (line 60)` — the correct guard | **yes** — *observed*, D-1 | 0 |
| **F-11** | `docs/carry-your-invariant.md:102` | `rust` | **executed** | `narrative::carry_your_invariant (line 102)` — the narrow guard that is quietly wrong | **yes** | 0 |
| — | `docs/read-the-worked-example.md` | *(no fence)* | n/a | the page is registered (`xtask/src/narrative.rs:150`) and walked; it produces no doctest | n/a — nothing to opt out | 0 |
| — | `examples/course-subscriptions/src/overview.md` | *(no fence)* | n/a | **none**, and this is R-4 in `§ Routing`: `course-subscriptions` is a **bin-only** package, so `cargo test --doc` finds no library target and the file contributes no doctest to the workspace sweep | n/a today; **no** if a fence were added | 0 |

**Opted-out count: `ignore` 0, `no_run` 0, untagged-treated-as-prose 0, named in
`IGNORE_ALLOWANCES` 0 — total 0**, which is the number `_design.md:206-208` committed this
project to and the sign-off (`:1091-1094`) accepted the consequence of. Nothing below was closed
by writing an exemption; EC-003 did not fire.

**Re-derivation, per column.**

| column | command |
| --- | --- |
| the fence set and info strings | `git grep -n '^```' -- docs/first-encounter.md docs/carry-your-invariant.md docs/read-the-worked-example.md examples/course-subscriptions/src/overview.md` and `git grep -n '^//! ```' -- crates/happenstance/src/lib.rs` |
| the execution class and the exerciser's own name for each fence | `cargo test -p xtask --doc -- --list` (tree pages) and `cargo test -p happenstance --doc -- --list` (crate root) |
| the opt-out count | `cargo xtask narrative` — green means no `ignore` under `TREE` lacks an allowance (`xtask/src/lint_narrative.rs:867`); `git grep -nF "no_run" -- docs crates/happenstance/src/lib.rs examples/course-subscriptions/src/overview.md` → no match |
| `IGNORE_ALLOWANCES` names none of this project's fences | `git grep -nF "const IGNORE_ALLOWANCES" -- xtask/src/lint_narrative.rs` — the slice is empty, so it names nothing at all |
| hidden doctest lines, off the render | `cargo doc -p happenstance --no-deps`, then compare the fence in `crates/happenstance/src/lib.rs:26-64` against `<pre class="rust rust-example-rendered">` in `target/doc/happenstance/index.html` |

### The hidden-line column, read off the DOM and not off the source

`_design.md:554` is the finding this column exists for: a `#`-prefixed line is **absent from the
rendered DOM entirely**, with no hover, focus or toggle that recovers it, so a source read cannot
see what a reader meets.

`crates/happenstance/src/lib.rs:26-64` carries **37 source lines** inside the fence.
`target/doc/happenstance/index.html`'s single `pre.rust.rust-example-rendered` carries **35**.
The two absent lines are exactly `:31` and `:63`:

```
31: # #[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {
63: # Ok::<(), Box<dyn Error>>(()) }
```

Both are inside the permitted set of `_design.md`'s transience row (`:554`): the scaffold and
`Ok(())`. **Nothing forbidden is hidden** — no `Query`, `QueryItem`, `Tags`, `Guard`,
`AppendCondition`, no append call, no read call and no assertion. `assert_eq!(done.attempts, 1)`
is visible at rendered line 35.

The other four surfaces carry **zero** hidden lines, and that is a scan of the fence bodies
rather than an inference from their absence in a diff. The tree pages are markdown, so their
"render" has two readers — GitHub, which shows a `#`-prefixed line literally, and rustdoc, which
hides it — and zero is the only value that reads the same in both.

### Composition preserved under repair

No repair was made, so this is the *unchanged* baseline rather than a post-repair re-check. It is
recorded because AC-006's obligation is preservation, and a baseline nobody wrote down is a
baseline nobody can compare against.

| check | crate root | first-encounter | carry-your-invariant | read-the-worked-example | overview.md |
| --- | --- | --- | --- | --- | --- |
| `#main-content details.top-doc > div.docblock` resolves | **yes** — `<details class="toggle top-doc" open><summary class="hideme">…</summary><div class="docblock">` | n/a (markdown surface, `_design.md:1075` condition 2) | n/a | n/a | n/a |
| fence width ≤ 68 columns | **70** — over, and **owned by HS-B0001 F-2** | 68 | 67 | n/a | n/a |
| fence height ≤ 24 rendered lines (32 on the crate root) | **35** — over, and **owned by HS-B0001 F-1** | 24 | 24 | n/a | n/a |
| `##` heading ≤ 22 characters | **39** and **26** — over, and **owned by HS-B0001 F-3**; the budget binds this surface only (`tension-resolutions/_resolutions.md:338-343`) | budget does not bind (markdown) | budget does not bind | budget does not bind | budget does not bind |
| paragraph ≤ 435 rendered characters | 344 | 234 | 330 | 267 | 359 |
| no fence behind a fold (anti-pattern 3, `_design.md:878`) | 0 `<details>` in authored content | 0 | 0 | 0 | 0 |
| no control outside rustdoc's chrome (anti-pattern 8, `:894`) | 0 | 0 | 0 | 0 | 0 |
| no `use happenstance_core::` in a fence (anti-pattern 13, `:907`) | 0 — the only occurrence in the file is `:242`'s real `pub use happenstance_core::*;`, which is code and not a fence | 0 | 0 | 0 | 0 |
| no diagram, chart or image (anti-pattern 14, `:911`) | 0 | 0 | 0 | 0 | 0 |
| literal `[bracket]` pair in prose (anti-pattern 2, `:875`) | **0** on the render, against the four `_design.md:951-956` measured pre-merge | 0 | 0 | 0 | 0 |
| every fence's output block non-empty (anti-pattern 15, `:912`) | n/a — the clause is scoped to *a step*, and the crate root is not a step page | **3 of 3** `rust` fences carry a `text` output block | **n/a, recorded rather than dropped**: the bridge is not a step page and `_design.md:503-521` composes no output-block slot for it; both fences assert instead of printing. The obligation did not fire, which is the shape `tension-resolutions/_resolutions.md:313-318` records for UX-015 | n/a — no fence | n/a — no fence |

Re-derivation: `cargo doc -p happenstance --no-deps` for the first and last rows; the widths,
heights, headings and paragraph lengths are counts over the fence bodies and prose runs of the
five files, with markdown link targets excluded from the paragraph count because a link renders
as its label and not its URL.

The three crate-root overages were **not** found here. They were found by `HS-S0185` while
measuring its own surfaces, struck from that story's AC-007 by its `spec.md § Amendment — BC-002`,
and given an owner at `.bklg/support/inherited-documentation-defects/crate-root-density-overages/bug.md`
(**HS-B0001**). This audit re-measured all three and confirms them unchanged. Recording them as
this story's findings would give them a second owner, which is the failure `_design.md:1075`'s
sign-off structure exists to prevent.

---

## § Drills

**D-1 and D-2, the opt-out pair. D-3 and D-4 — the clause-id pair — are in
`_citations.md § Drills`**, split by artifact so each headline claim sits beside its own
falsification rather than in a shared appendix (`spec.md § Clarifications resolved during spec`,
item 3). The **EC-002 probe** is below the two drills; it is not one of the four.

Each drill is four fields: **the exact edit**, **the exact command**, **the exact output**, and
**the revert** with the file's `git hash-object` shown identical to its pre-drill value. Each
asserts on the checker's own `path:line — message` shape (`xtask/src/lint_narrative.rs`'s
composed problem line), never on a bare non-zero exit — a checker failing for an unrelated
reason would otherwise read as a passing drill.

D-1 is the one where a mechanism exists. **D-2 is expected to *fail to fail*, and it is the
valuable one**: it is the only evidence that the crate-root fence is protected by review rather
than by machinery. It is recorded as *observed green*, not skipped.

Pre-drill hashes, taken once before any edit and shared with `_citations.md § Drills`:

```
git hash-object docs/carry-your-invariant.md crates/happenstance/src/lib.rs docs/first-encounter.md
b63697589f05c4a1663ef3754a5d96c59b4b6298    docs/carry-your-invariant.md
05c69822bc1512bbea295cdea33400d7a240318b    crates/happenstance/src/lib.rs
63073630c1c167e022193a989bfc6c3b1b5c32ea    docs/first-encounter.md
```

### D-1 — opt one fence out, on a page inside `TREE`. Expected: **red**.

Subject: **F-10**, `docs/carry-your-invariant.md:60`. Chosen over a `first-encounter` fence on
purpose: `xtask/tests/first_encounter.rs:488` already carries
`no_fence_on_the_page_opts_out_of_the_compiler`, so a drill there would have two instruments and
could not tell which one fired. The bridge page has **only** HS-P0020's checker, which is the
mechanism AC-002 is about.

**The edit.**

```diff
--- a/docs/carry-your-invariant.md
+++ b/docs/carry-your-invariant.md
@@ -57,7 +57,7 @@ holding both, a read, a fold over what the read returned, and an append under
 a condition built from that same query. Between the read and the append
 someone else takes the last seat, so the append is refused.

-```rust
+```ignore
 use happenstance::{AppendCondition, Event, EventStore, Query};
```

**The command, and the output.**

```
$ cargo xtask narrative
  docs/carry-your-invariant.md:60 — an `ignore` fence needs an `IGNORE_ALLOWANCES` entry naming it; a comment above the fence does not permit it, because a comment is reviewable only in the diff that introduced it

xtask failed: 1 problem(s) in docs
error: process didn't exit successfully: `target\debug\xtask.exe narrative` (exit code: 1)
```

The same failure under the story-grain command, so the drill is against the gate a checkpoint
actually meets and not only against the subcommand:

```
$ cargo xtask lints
=== every narrative page is checked ===
  docs/carry-your-invariant.md:60 — an `ignore` fence needs an `IGNORE_ALLOWANCES` entry naming it; …

xtask failed: 1 problem(s) in docs

xtask failed: every narrative page is checked failed with exit code: 1
error: process didn't exit successfully: `target\debug\xtask.exe lints` (exit code: 1)
```

**The revert.**

```
$ git hash-object docs/carry-your-invariant.md
b63697589f05c4a1663ef3754a5d96c59b4b6298        # identical to pre-drill
$ git status --porcelain
                                                 # empty
$ cargo xtask narrative
  5 pages, all consistent
```

**Verdict: the mechanism fired, named the file and the line, and the tree returned byte-identical.**

### D-2 — opt the crate-root fence out. Expected: **green, and that is the finding**.

Subject: **F-01**, `crates/happenstance/src/lib.rs:26`.

**The edit.**

```diff
--- a/crates/happenstance/src/lib.rs
+++ b/crates/happenstance/src/lib.rs
@@ -23,7 +23,7 @@
 //! One enum of events, one struct that folds them, and one call that reads,
 //! decides, appends and retries:
 //!
-//! ```
+//! ```ignore
 //! use happenstance::{bytes::Bytes, Codec, CodecError, DecisionModel};
```

**The commands, and the outputs. Four instruments, all green.**

```
$ cargo xtask lints
=== every narrative page is checked ===
  5 pages, all consistent
=== every page declares one need ===
  5 pages, 16 rules, all consistent
                                                 # exit 0

$ cargo test -p happenstance --doc
test result: ok. 7 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
                                                 # exit 0 — "1 ignored" is the whole finding

$ cargo test -p happenstance --tests
                                                 # 16 binaries, every one "test result: ok"

$ cargo test -p xtask --tests
test result: ok. 289 passed; 0 failed …
test result: ok. 19 passed; 0 failed …
test result: ok. 9 passed; 0 failed …
                                                 # exit 0
```

The named test that vanished from the run is identified rather than inferred — with the `ignore`
in place, `cargo test -p happenstance --doc` reports:

```
test crates\happenstance\src\lib.rs - (line 247) ... ignored
test crates\happenstance\src\lib.rs - (line 209) ... ok
```

so the crate-root fence is `(line 247)` and the crate README's is `(line 209)`. After the revert
the same command reports `8 passed; 0 failed; 0 ignored`.

**The revert.**

```
$ git hash-object crates/happenstance/src/lib.rs
05c69822bc1512bbea295cdea33400d7a240318b        # identical to pre-drill
$ git status --porcelain
                                                 # empty
$ cargo test -p happenstance --doc
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Verdict: failed to fail, as expected.** The one fence a `cargo add happenstance` reader meets
first can be removed from the compiler's reach by seven characters and the whole gate stays
green. What protects it is `_design.md:872-874`'s anti-pattern 1 — *a code block visibly labelled
as not compiled … This project ships zero* — which is a reviewer reading a diff, and
`_design.md:917-918`'s standing "no `no_run` and no `ignore` on the boundary-refusal fence".
Review, not machinery. **Routed as R-2.**

**A second finding fell out of D-2 and is recorded because it was measured rather than
forecast.** The crate-root fence's doctest is named `(line 247)` in a file of **242 lines**. The
line is counted inside the concatenation of `crates/happenstance/README.md`
(`#![cfg_attr(doctest, doc = include_str!("../README.md"))]`, `crates/happenstance/src/lib.rs:10`)
and the module doc, so it maps to no line a reader can open. That is exactly the failure
`xtask/src/constitution.rs:11-18` names and that `xtask/src/narrative.rs`'s one-module-per-page
rule exists to avoid — present on the crate root, which is outside the harness. The tree pages do
not have it: their doctests are named `narrative::first_encounter (line 16)`,
`(line 55)`, `(line 96)`, `narrative::carry_your_invariant (line 60)` and `(line 102)`, every one
of which is the page's real source line. **Routed as R-3.**

### EC-002 probe — does the checker reject `no_run`?

Not one of the four drills. EC-002 requires the answer to be *established by running the step*
rather than assumed, because `no_run` appears nowhere in HS-P0020's planning corpus, and it
requires **both** obligations discharged: route the substrate gap **and** repair any `no_run` on
this project's surfaces. There is none to repair (`§ Fences`: `no_run` count 0), so only the
routing is owed — but the routing had to be earned by a measurement.

**The edit, in two steps, on `docs/carry-your-invariant.md`.**

```diff
-```rust
+```rust,no_run
…
-    assert!(matches!(refused,
+    assert!(!matches!(refused,
         Err(AppendError::ConditionViolated(_))), "{free} free");
```

The second half is what makes this a falsification rather than an observation: the page's
central claim — that the guard **refuses** — is inverted, so the fence is now false about the
library.

**With `no_run`, and the assertion inverted:**

```
$ cargo xtask narrative
  5 pages, all consistent
                                                 # exit 0

$ cargo test -p xtask --doc -- carry_your_invariant
test …/docs/carry-your-invariant.md - narrative::carry_your_invariant (line 60) - compile ... ok
test …/docs/carry-your-invariant.md - narrative::carry_your_invariant (line 102) ... ok
test result: ok. 2 passed; 0 failed; 0 ignored
```

**With `no_run` removed and the same inverted assertion left in place:**

```
$ cargo test -p xtask --doc -- carry_your_invariant
test …/docs/carry-your-invariant.md - narrative::carry_your_invariant (line 60) ... FAILED
test result: FAILED. 1 passed; 1 failed; 0 ignored
```

**Answer: `no_run` passes HS-P0020's checker silently.** `xtask/src/lint_narrative.rs:823` lists
it in the accepting arm — `"rust" | "no_run" | "should_panic" => {}` — so it is neither an
unrecognised info string nor an `ignore` needing an allowance. And the pair of runs above shows
what that buys: seven characters turn a fence whose assertion is *false* from red to green, and
the page's teaching claim stops being checked while the banner keeps saying `all consistent`.
That is `_decomposition.md:616-625`'s "type-check forever without ever being asked to refuse
anything", measured. **Routed as R-1.**

**The revert.**

```
$ git hash-object docs/carry-your-invariant.md
b63697589f05c4a1663ef3754a5d96c59b4b6298        # identical to pre-drill
$ git status --porcelain
                                                 # empty
$ cargo test -p xtask --doc -- carry_your_invariant
test result: ok. 2 passed; 0 failed; 0 ignored
```

---

## § Routing

Every finding this story did not close, with a named destination. **Nothing is absorbed.** No
finding below was closed by writing an exemption, adding an `IGNORE_ALLOWANCES` entry, or
relaxing a fence — EC-003's prohibition held because it was never reached: no fence on any of
these five files is unwritable as compiled, executed code, and all eleven compile.

| id | finding | evidence | destination | why it is not closed here |
| --- | --- | --- | --- | --- |
| **R-1** | HS-P0020's narrative checker **accepts `no_run` silently**. A `rust,no_run` fence whose assertion is false is green under `cargo xtask narrative` *and* under `cargo test --doc`. | `§ Drills`, EC-002 probe; `xtask/src/lint_narrative.rs:823` | **HS-P0020** `checked-documentation-surface` | It is a change to the checker's closed token set. `spec.md § PR boundary` excludes any change to HS-P0020's checker or its constants beyond a registration line, and `_storymap.md:158-163` routes substrate gaps there. No `no_run` exists on this project's surfaces, so EC-002's *repair* obligation is discharged vacuously and only the *routing* obligation remains. |
| **R-2** | `crates/happenstance/src/lib.rs` is outside `TREE`, so **no mechanism in the repository can fail** when its fence opts out. Seven characters remove the crate's one landing program from the compiler and the gate stays green. | `§ Drills`, D-2 | **HS-P0020** `checked-documentation-surface` — as an input to what its checker's corpus should reach | The file cannot be moved into `TREE`: `include_str!` resolves against the file tree at compile time and a path escaping the package would not resolve once published (`crates/happenstance/src/lib.rs:7-9`), which is why the program deliberately exists twice (`_design.md:743-762`). Widening the checker's corpus is HS-P0020's constant to change. The compensation in place is `_design.md:872-874` anti-pattern 1, which is review. |
| **R-3** | The crate-root fence's doctest is named **`(line 247)` in a 242-line file** — a line counted inside the `README.md` + module-doc concatenation, mapping to nothing a reader can open. The tree pages do not have this, because one module per page keeps the number page-relative. | `§ Drills`, D-2; `crates/happenstance/src/lib.rs:10`; contrast at `xtask/src/constitution.rs:11-18` | **`support` / `inherited-documentation-defects` (HS-P0026)**, beside **HS-B0001** | It is a property of HS-P0016's README include, not of anything this project authored, and the same reasoning that gave HS-B0001 its own item applies: `initiative/from-contract-to-published-library` is merged forward but not to `main`, so HS-P0016 is not a destination reachable from this branch (`crate-root-density-overages/bug.md`, § Notes). |
| **R-4** | `examples/course-subscriptions/src/overview.md` is included by a **bin-only** package, so `cargo test --doc` finds no library target and the file contributes **no doctest** to the workspace sweep. A `rust` fence added there would be compiled by nothing and reported by nothing. Vacuous today — the file carries zero fences — and recorded so the absence is a decision rather than a hole. | `cargo test --workspace --all-features --doc -- --list` returns no entry matching `course` or `overview`; `cargo test -p course-subscriptions --doc` → `error: no library targets found in package` | **HS-P0020** `checked-documentation-surface` | `spec.md`'s `## Behavior and interfaces` states this file "rides the same `--workspace` sweep"; the measurement says that holds for its **render** (`cargo doc -p course-subscriptions`, and `examples/course-subscriptions/tests/reach.rs`'s byte-identity assertions) and not for **doctests**. Correcting where the third mechanism reaches is a substrate decision, and this story may not change a checker's corpus. |
| **R-5** | A clause citation on `crates/happenstance/src/lib.rs` **resolves against nothing**. `spec_trace` reads only `spec/SPECIFICATION.md`; `lint_narrative` sweeps `docs/`; a relative markdown link is not an intra-doc link, so `RUSTDOCFLAGS=-D warnings` does not see it. | `§ Drills`, D-4 | **HS-P0020** `checked-documentation-surface` | Same corpus boundary as R-2, and the same reason it is not designed around. The crate root cites no clause today, so nothing is currently broken — the gap is that nothing would say so if it were. |
| **R-6** | The three crate-root density overages — fence **35** rendered lines against 32, **70** columns against 68, two `##` headings at **39** and **26** against 22 — are unchanged. | `§ Fences`, "Composition preserved under repair" | **HS-B0001** `crate-root-density-overages`, already open | Already owned, with an origin record and a stated reason for not being this initiative's (`bug.md § Notes`). Re-measured and confirmed here; recording them as this story's findings would give one defect two owners. |
| **R-7** | `docs/append-conditions.md:13` carries `use happenstance_core::MemoryEventStore;` inside a `rust` fence. | `git grep -n "use happenstance_core::" -- docs` | **HS-P0020** `checked-documentation-surface` — the page's author | Out of this story's subject: anti-pattern 13 (`_design.md:907-910`) is scoped to *"any page this project authors"*, and `docs/append-conditions.md` is HS-P0020's own page and not one of `_design.md`'s four surfaces. Recorded because the sweep that proved this project's surfaces clean also crossed it, and a hit seen and not written down is the shape of finding that comes back. |

**Nothing routed above blocks this story.** EC-001 did not fire; both headline claims have an
enforcer where one exists, and the two places where none exists are named, measured and given a
destination rather than covered by another page's mechanism.

---

## Re-deriving this record

Six commands, in the order the implementation notes give, and the whole of `§ Fences` and
`§ Substrate confirmation` falls out of them. `§ Drills` is tier 4 — a human-observed
falsification recorded once (`_decomposition.md:485`) — and is re-derived by re-running the four
edits above, not by a command.

```
cargo xtask lints                                       # both narrative steps, green
cargo xtask spec-trace                                  # the specification's own traceability
cargo test -p xtask --doc -- --list                     # every tree fence, by page and line
cargo test -p happenstance --doc -- --list              # the crate root's fence
cargo doc -p happenstance --no-deps                     # the render the hidden-line column reads
cargo xtask ci --fast                                   # the project bar (project.md:17, terminal: false)
```
