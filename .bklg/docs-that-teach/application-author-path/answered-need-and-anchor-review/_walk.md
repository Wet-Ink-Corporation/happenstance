# The walk — HS-P0021's procedure executed over this project's four surfaces

Companion to `spec.md`, beside `_ledger.md`, whose `## Recorded result` carries the composed
per-page table AC-008 asks for. This file carries the **transcripts** the table is a summary of:
the enumeration and its two cross-checks, the step-by-step walk answers, the pasted mechanical
output, the two-direction calibration, the anchor sweep and the judgement.

Split that way for the reason NF-002 gives one artifact over: a reviewer at closeout resolves the
*result* from `_ledger.md` alone, and comes here only when they want to see the working.

**Dated 2026-08-19**, against `initiative/docs-that-teach` at `17f14b5`, toolchain **1.97.1**.
`git grep`, not `rg` — `rg` is not on this machine's `PATH`
(`merge-forward-preflight/_baseline.md:90-95`), and a command that runs in one sandbox only is a
claim rather than a check. Note that the working copy carries **CRLF** line endings, so a
`git grep` pattern anchored with `$` matches nothing; every command below is written without one.

---

## § 1 — The set, enumerated before it was walked

Enumeration first, walk second. The other order makes *did I miss a page?* unanswerable, because
the reviewer's memory of what they walked becomes the enumeration.

**The authority is `_design.md`'s `## Surfaces` block (`:45-65`)**, four ids resolved to real
paths — not a directory listing, so a designed-but-unauthored page would show as a missing row.

| surface id | path | exists? |
| --- | --- | --- |
| `crate-root-encounter` | `crates/happenstance/src/lib.rs` | yes |
| `opening-encounter` | `docs/first-encounter.md` | yes |
| `conceptual-bridge` | `docs/carry-your-invariant.md` | yes |
| `worked-example-handoff` | `docs/read-the-worked-example.md` | yes |

**EC-001 did not fire.** All four exist, so no row reads *not yet authored* and the walk is over
the whole set rather than over what happened to be on disk.

### Cross-check 1 — `docs/README.md`'s narrative index

```
$ git grep -n '](' -- docs/README.md
docs/README.md:19:| Appending under a condition | [`append-conditions.md`](append-conditions.md) |
docs/README.md:20:| Watching a boundary refuse, in three runnable steps | [`first-encounter.md`](first-encounter.md) |
docs/README.md:21:| Fences the compiler never sees | [`text-fences.md`](text-fences.md) |
```

(The remaining hits, `:14` and `:25-35`, are the prose link to band 10 and the **pointer-out**
table, which routes out of the tree and is not the narrative index.)

### Cross-check 2 — the blocking stories' output

```
$ git diff main --name-only -- docs crates/happenstance/src/lib.rs
crates/happenstance/src/lib.rs
docs/README.md
docs/append-conditions.md
docs/carry-your-invariant.md
docs/first-encounter.md
docs/read-the-worked-example.md
docs/text-fences.md
```

### The two set differences, computed and printed

| difference | members | disposition |
| --- | --- | --- |
| in the **index**, not in the enumeration | `docs/append-conditions.md`, `docs/text-fences.md` | **Not a finding.** Both are HS-P0020's own pages, added by `7020c4c` and `5ecce36` (`git log --diff-filter=A`), and neither is one of `_design.md`'s four surfaces. `text-fences.md` is the retained fixture behind the `text`-fence limit `xtask/src/narrative.rs` states. |
| in the **enumeration**, not in the index | **`docs/carry-your-invariant.md`, `docs/read-the-worked-example.md`** | **A finding — W-1 below.** Two of the four surfaces this project authored are absent from the tree's own narrative index. |
| in the `git diff`, in neither | `docs/README.md` | **Not a finding.** It is the index itself; it changed because HS-P0020 authored it on this branch. |

**No fifth page of this project's set was found by either cross-check**, so nothing was quietly
appended to the table. The one difference that *is* a finding is W-1, and it is routed rather
than absorbed.

**Method, date, walker.** Enumeration from `_design.md:45-65` on 2026-08-19, cross-checked twice
by the two commands above, by the walker named in `§ 2`.

---

## § 2 — The walker, and what they were allowed to read

`standards/pages/40-reviewing-a-page.md:20-26` fixes the performer as **not the author**, and
forbids consulting the author or the page's git history: *"a verdict that needs either is a
verdict about the author rather than about the page."*

**The walker is this story's implementation context.** It authored **none** of the four surfaces:
`docs/first-encounter.md` is HS-S0185's and HS-S0186's (`9493276`, `cc9c4a4`),
`docs/carry-your-invariant.md` is HS-S0187's (`b24d2cd`), `docs/read-the-worked-example.md` is
HS-S0188's (`5805623`), and the crate root's module doc is HS-S0185's. **EC-008 did not fire** —
no row is blocked for want of a non-author walker.

**Consulted:** the rendered and source pages, `spec/SPECIFICATION.md`, `standards/pages/` (the
router, and bands 00, 10, 30 and 40), `_design.md` and `tension-resolutions/_resolutions.md`.
**Not consulted for any verdict:** the authoring stories' specs or ledgers, and no page's git
history. The commit shas above are attribution for this record, gathered from `git log --stat`
for the substrate question *which test files exist*, and were not read for a verdict.

The tier-5 human sign-off that closes this story is the project's review gate; this record is
what it reads.

---

## § 3 — The walk, per page, step by step

RP-40-1's six steps, answered `yes`/`no` per page and **recorded rather than summarised**
(`reviewer-and-citation-procedures/spec.md:491-493`). The four verdicts are `pass`,
`fail — two needs`, `fail — need not answered`, `indeterminate`. There is no soft pass and none
is invented here.

### The form and position checks, mechanically first

```
$ git grep -c '^> \*\*Answers:\*\*' -- docs/first-encounter.md docs/carry-your-invariant.md \
      docs/read-the-worked-example.md
docs/carry-your-invariant.md:1
docs/first-encounter.md:1
docs/read-the-worked-example.md:1

$ git grep -c '^//! > \*\*Answers:\*\*' -- crates/happenstance/src/lib.rs
crates/happenstance/src/lib.rs:1
```

The head region of each page, captured, showing the H1, one blank line, the declaration, one
blank line, and **nothing interposed** — no badge row, no table of contents, no "last updated"
line:

```
docs/first-encounter.md                 docs/carry-your-invariant.md
1  # Your first encounter               1  # Carry your invariant across
2                                       2
3  > **Answers:** `tutorial` — How …    3  > **Answers:** `explanation` — How …
4                                       4
5  Three programs, in order. …          5  ## Your rule, in your words

docs/read-the-worked-example.md         crates/happenstance/src/lib.rs
1  # Read the worked example            19 //! DCB-compliant event sourcing, with batteries.
2                                       20 //!
3  > **Answers:** `orientation` — …     21 //! > **Answers:** `tutorial` — How …
4                                       22 //!
5  The canonical example is three …     23 //! One enum of events, one struct …
```

Declaration line against first-fence line — the declaration is above the first fence in reading
order on every surface that has one:

| surface | declaration at | first fence opens at | above? |
| --- | --- | --- | --- |
| `crates/happenstance/src/lib.rs` | `:21` | `:26` | **yes** |
| `docs/first-encounter.md` | `:3` | `:16` | **yes** |
| `docs/carry-your-invariant.md` | `:3` | `:60` | **yes** |
| `docs/read-the-worked-example.md` | `:3` | **no fence** | **vacuously yes**, recorded in that word rather than left blank |

Token membership against the closed set of `standards/pages/10-the-need-set.md:12-17` —
`orientation` · `tutorial` · `how-to` · `explanation`, compared exactly, no case folding and no
aliasing (RP-10-1):

| surface | token | member? |
| --- | --- | --- |
| `crate-root-encounter` | `tutorial` | yes |
| `opening-encounter` | `tutorial` | yes |
| `conceptual-bridge` | `explanation` | yes |
| `worked-example-handoff` | `orientation` | yes |

### `opening-encounter` — `docs/first-encounter.md`

| step | question | answer |
| --- | --- | --- |
| 1 | exactly one `> **Answers:**` between the H1 and the first paragraph? | **yes** — `:3`, count 1, nothing interposed |
| 2 | token one of band 10's four, spelled exactly? | **yes** — `tutorial` |
| 3 | does every section serve the declared question? | **yes** — the three `##` steps stage one thing (the vocabulary, the boundary that admits, the boundary that refuses) and the `###` drill falsifies the third |
| 4 | is every load-bearing claim visible with nothing opened? | **yes** — no `<details>`, zero hidden doctest lines, all three fences and all three output blocks persistent |
| 5 | does every normative sentence cite a clause id rather than state the rule? | **yes** — three normative sentences, three resolving ids (ES-8, ES-26, ES-25), audited by the slice-mate at `fence-inventory-and-clause-audit/_citations.md § Claims` C-1…C-3 |
| 6 | could a stranger name the need from the head alone? | **yes** — title, declaration, and two sentences ending *"It ends with an append this library refuses"* |

**Verdict: `pass`.**

### `conceptual-bridge` — `docs/carry-your-invariant.md`

| step | question | answer |
| --- | --- | --- |
| 1 | exactly one declaration between the H1 and the first paragraph? | **yes** — `:3`, count 1 |
| 2 | token spelled exactly? | **yes** — `explanation` |
| 3 | does every section serve *how do I say my own rule in this library's terms?* | **yes** — the rule in the reader's words, the prior model answered, the four names and the mapping table, the guard, the contrast, and the return to the correct version |
| 4 | every load-bearing claim visible with nothing opened? | **yes** — no folds, zero hidden lines, both fences and the three-column table persistent |
| 5 | every normative sentence a clause citation? | **yes** — six sentences, six resolving ids (ES-27, ES-25, VT-30, CF-8, CF-7, ES-27), audited as C-4…C-9 |
| 6 | stranger names the need from the head alone? | **yes** |

**Verdict: `pass`.**

### `worked-example-handoff` — `docs/read-the-worked-example.md`

| step | question | answer |
| --- | --- | --- |
| 1 | exactly one declaration between the H1 and the first paragraph? | **yes** — `:3`, count 1 |
| 2 | token spelled exactly? | **yes** — `orientation` |
| 3 | does every section serve *where can I read a whole DCB program?* | **yes** — four prose slots, all routing: two sentences of orientation, the anchor citation, the vocabulary-seam sentence, and the two links |
| 4 | every load-bearing claim visible? | **yes** — no fence to hide and no fold |
| 5 | every normative sentence a clause citation? | **yes, vacuously and recorded in that word** — the page carries **zero** normative sentences; all three candidate claims answered *no* to RP-30-1's falsification question |
| 6 | stranger names the need from the head alone? | **yes** — and band 10's success condition for `orientation` is *"they leave, correctly, within one screen"*, which the page's 23 lines meet |

**Verdict: `pass`.**

### `crate-root-encounter` — `crates/happenstance/src/lib.rs`

This is the row that needed a decision, and the decision is recorded rather than made silently.

| step | question | answer |
| --- | --- | --- |
| 1 | exactly one `> **Answers:**` line, positioned? | **yes, against a substituted position rule — see below.** Count is 1, at `:21`, immediately below the one-line crate summary and above every `#` section, which is the position `_design.md:426-431` fixed for this surface |
| 2 | token spelled exactly? | **yes** — `tutorial` |
| 3 | does every section serve the declared question? | **yes over the region this project composes; `no` over the whole rendered page — see below** |
| 4 | every load-bearing claim visible with nothing opened? | **yes** — everything this project writes renders inside `<details class="toggle top-doc" open>`, verified on the built `target/doc/happenstance/index.html`; nothing lands inside a `details` that is not `open` (anti-pattern 3) |
| 5 | every normative sentence a clause citation? | **yes, vacuously and recorded in that word** — the crate root carries **zero** normative sentences; all 14 candidate claims answered *no* to RP-30-1, being about this workspace's crate split, features and re-exports |
| 6 | stranger names the need from the head alone? | **yes** over the composed region — summary, declaration, then the program |

**Verdict: `pass`, with two scopings stated, because a `pass` a reviewer cannot audit is the
impression RP-40-1 exists to refuse.**

**Scoping 1 — step 1's anchoring element (EC-005).** A rustdoc crate-root doc comment has **no
markdown `# Title`** for RP-00-2's form to anchor to; rustdoc renders the crate name as the
page's heading. `_design.md:426-431` fixed the position independently of the form: immediately
below the one-line crate summary and above every `##`, which puts it above the first fence by
construction. The **form** is HS-P0021's, verbatim and unmodified — `> **Answers:** \`tutorial\`
— How do I decide, write, and hold an invariant?` — so **no variant notation was invented** and
DR-14's defect did not occur. What is substituted is the *anchor* the position is measured
from. **Routed to HS-P0021 as a form gap (W-2).**

**Scoping 2 — step 3's corpus.** Over the whole rendered page, step 3 answers **no**: `# Features`
is a feature table and `# Testing without a database` answers a different question from
`tutorial`. Over the region `_design.md:413-436` composes — the summary, the declaration, the
ADR-0006 reasoning and the fence — it answers **yes**. The scoping is not a convenience: band 10
excludes rustdoc from the need set **by name**, and gives the reason —
*"This workspace already has two authoritative reference surfaces — rustdoc, and
`spec/SPECIFICATION.md` … A `reference` bucket inside a narrative tree is therefore either
permanently empty or it becomes a second specification"*
(`standards/pages/10-the-need-set.md:25-31`). A rustdoc crate root carrying an item vocabulary
and a feature table is the reference surface doing the job band 10 assigned it, and the
remaining sections are HS-P0016's landing copy, which `project.md`'s risk table places outside
this project (*"purpose, not paragraph. This project does not touch landing copy"*). **Routed to
HS-P0021 as a rule gap (W-3):** RP-40-1 is written for a governed markdown page and states no
behaviour for a rustdoc crate root that carries a declaration by a project's own composition
decision. Without the scoping the verdict is `indeterminate` — the walk could not be completed
from the page alone without substituting a rule the procedure does not supply.

**This is the one judgement in the walk a reviewer might overturn, and it is written so they
can.** If the scoping is rejected, the crate root's verdict is `fail — need not answered` on
copy this project may not touch, and the correct response is a routed finding to HS-P0016's
owner rather than an edit here.

**Four surfaces, four verdicts, four `pass`. No row is at `fail` or `indeterminate`.**

---

## § 4 — Coverage, per page, never averaged

Three columns, one row per surface. The middle column is the reason the table exists: *"4/4
declare one need"* is true and misleading, because one of the four has no machine behind it.

| surface | the instrument that actually observed the declaration | could that instrument have **failed** on this page? | verdict |
| --- | --- | --- | --- |
| `crate-root-encounter` — `crates/happenstance/src/lib.rs` | **a named person** — the walker of `§ 2`. Also `xtask/tests/first_encounter.rs:184` (`the_crate_root_declares_its_one_need_above_its_first_fence`), which HS-S0185 shipped | **no**, for the step that is HS-P0021's: the file is outside `TREE = "docs"` (`xtask/src/lint_narrative.rs:239`) and `every page declares one need` never opens it. A story-specific test exists and is not the discipline's instrument — it can be deleted with the story and nothing in `standards/pages/` would notice | `pass` |
| `opening-encounter` — `docs/first-encounter.md` | the `every page declares one need` REQUIRED step (`cargo run --locked --quiet -p xtask -- lint-pages`), corpus `lint_narrative::TREE` | **yes** | `pass` |
| `conceptual-bridge` — `docs/carry-your-invariant.md` | the same step — and **only** that step; this page has no story-specific test | **yes** — observed, `§ 5` direction (b) | `pass` |
| `worked-example-handoff` — `docs/read-the-worked-example.md` | the same step, plus `examples/course-subscriptions/tests/reach.rs` (`exactly_one_answered_need`) | **yes** | `pass` |

**The `crate-root-encounter` row agrees with the slice-mate**, and the agreement was coordinated
before either was written (implementation note 7). `fence-inventory-and-clause-audit/_inventory.md
§ Fences` reaches **no** for that file's fences and `§ Routing` R-2/R-5 route it; this table
reaches **no** for its declaration and routes it as W-4. Same file, same structural reason — it
is outside `TREE` — and the two tables do not disagree.

### The mechanical half was run, not assumed, and its corpus is shown

```
$ cargo run --locked --quiet -p xtask -- lint-pages
  5 pages, 16 rules, all consistent
```

Five pages: `docs/` holds six markdown files and `docs/README.md` is the index, which the
checker's own constant excludes (`xtask/src/lint_narrative.rs:250`). The crate root is not among
them, which is the coverage table's middle column stated as a count.

The corpus is **shown**, not assumed:

```
$ git grep -n 'const TREE' -- xtask/src/lint_narrative.rs
xtask/src/lint_narrative.rs:239:pub(crate) const TREE: &str = "docs";

$ git grep -n 'lint_narrative::TREE' -- xtask/src/lint_pages.rs
xtask/src/lint_pages.rs:379:        lint_narrative::TREE
xtask/src/lint_pages.rs:467:    let tree = lint_narrative::TREE;
xtask/src/lint_pages.rs:519:            lint_narrative::TREE
```

`xtask/src/lint_pages.rs:443-444` states the rule in its own words: *"The tree is
[`crate::lint_narrative::TREE`] and **no second constant names it**"*. **No `PAGE_DIR` exists**,
so the *three lists that must agree* defect is foreclosed rather than documented.

**EC-003 did not fire, and one deviation is recorded rather than absorbed.** HS-P0021's spec
names the constant `xtask::narrative::TREE`; it lives at `xtask::lint_narrative::TREE`. The
constant is **present** and is the single corpus, which is the substance EC-003 protects, so the
response is to record the path rather than to halt. **W-5.**

### Boundary proof — no `xtask` code, no second corpus, no second checker

NF-001's own command, `git diff main -- xtask/src`, is **not** empty on this branch, and that is
a property of the branch rather than of this story: HS-P0020 and HS-P0021 landed
`lint_narrative.rs` (4,845 lines) and `lint_pages.rs` (3,421 lines) here, which is the substrate
this story consumes. The measurement that answers NF-001 is **this story's own diff**:

```
$ git diff 17f14b5 -- xtask/ standards/ spec/
                                                 # empty
```

This story adds no `xtask` code, no rule atom under `standards/pages/`, no second corpus, no
second checker and no lint — including the content-level accessibility lint `_design.md:945-951`
routes to HS-P0021.

---

## § 5 — The calibration, in both directions

A procedure that has never returned `fail` is decorative — CLAUDE.md's corollary for conformance
rules, applied to a written one. This story's deliverable is an **absence**, and an absence is
demonstrated by showing the instrument detects a presence.

### (a) The inert specimen → `fail — two needs`

```
$ git grep -c '^> \*\*Answers:\*\*' -- standards/pages/examples/two-needs.md
standards/pages/examples/two-needs.md:2

$ ls standards/pages/*.md
standards/pages/00-one-need.md
standards/pages/10-the-need-set.md
standards/pages/20-the-fold-line.md
standards/pages/30-citing-the-specification.md
standards/pages/40-reviewing-a-page.md
standards/pages/README.md
```

The specimen is **not** listed: it sits under `examples/`, outside the band namespace, so a
corpus reader taking top-level `NN-slug.md` files never sees it and it can stay permanently
broken without turning a gate red (`standards/pages/examples/two-needs.md:13-16`). **Inert,
proved rather than asserted.**

**The walk over it**, top to bottom:

| step | answer |
| --- | --- |
| 1 | **no** — two `> **Answers:**` lines, `:3` and `:4`, both between the H1 and the first paragraph |
| — | the walk ends here: *"A `no` at step 1 or 2 is `fail — two needs` when a second need is visible in the head"* (`standards/pages/40-reviewing-a-page.md:40-41`). A second need is visible in the head |

**Verdict: `fail — two needs`.** It matches what the specimen's own closing paragraph says the
walk should find (`:34-38`). **EC-006 did not fire** — the walk reached `fail`, so it is not
decorative.

### (b) A live page of this project's own set

Injected into `docs/carry-your-invariant.md` — a `docs/` page, per clarification 3, because only
a page inside `TREE` has a mechanical instrument that can be observed failing. The bridge rather
than the handoff, because the handoff carries a second instrument in
`examples/course-subscriptions/tests/reach.rs` and the drill could not then attribute the
failure to HS-P0021's step.

Pre-injection hash: `git hash-object docs/carry-your-invariant.md` →
`b63697589f05c4a1663ef3754a5d96c59b4b6298`.

**1 — the injected diff.**

```diff
--- a/docs/carry-your-invariant.md
+++ b/docs/carry-your-invariant.md
@@ -1,6 +1,7 @@
 # Carry your invariant across

 > **Answers:** `explanation` — How do I say my own rule in this library's terms?
+> **Answers:** `how-to` — How do I build an AppendCondition from my own rule?

 ## Your rule, in your words
```

**2 — the step, failing by `path:line`.**

```
$ cargo run --locked --quiet -p xtask -- lint-pages
  docs/carry-your-invariant.md:4 — declares `explanation` and `how-to`; a page answers one need

xtask failed: 1 problem(s) in standards/pages + docs
```

and under the story-grain command, so the drill is against the gate a checkpoint actually meets:

```
$ cargo xtask lints
=== every page declares one need ===
  docs/carry-your-invariant.md:4 — declares `explanation` and `how-to`; a page answers one need

xtask failed: every page declares one need failed with exit code: 1
error: process didn't exit successfully: `target\debug\xtask.exe lints` (exit code: 1)
```

**The walk over the injected page:** step 1 answers **no** — two declarations, both between the
H1 and the first paragraph, a second need visible in the head. **Verdict `fail — two needs`**,
the same verdict the specimen produced, reached on a live page of this project's set.

**3 — the revert, shown rather than claimed.**

```
$ git hash-object docs/carry-your-invariant.md
b63697589f05c4a1663ef3754a5d96c59b4b6298        # identical to pre-injection
$ git status --porcelain
                                                 # empty
```

**4 — the step, green again.**

```
$ cargo run --locked --quiet -p xtask -- lint-pages
  5 pages, 16 rules, all consistent
```

**Both instruments detected a presence, and both returned to green over a clean tree.** The
report of an absence in `§ 3` and `§ 4` therefore means something.

---

## § 6 — The anchor: named once, linked, resolving

### The prior model is named on exactly one page

```
$ git grep -ni "aggregate" -- crates/happenstance/src/lib.rs docs/first-encounter.md \
      docs/carry-your-invariant.md docs/read-the-worked-example.md docs/README.md
docs/carry-your-invariant.md:22:nothing asks you to name an aggregate, and your aggregates are not the unit

$ git grep -ni "one stream per entity" -- <the same five files>
docs/carry-your-invariant.md:20:you arrived with puts one stream per entity, so a rule spanning two of them

$ git grep -ni "which stream" -- <the same five files>
docs/carry-your-invariant.md:19:Your reflex question is probably *which stream does this go in?* — the model
```

`your aggregates` is the fourth phrase and is the same `:22` hit.

**Every hit is on one page, and every hit is inside one section.**
`## Where your streams went` opens at `docs/carry-your-invariant.md:17`; the next `##`,
`## Tag, query, fold, guard`, opens at `:30`. The section's span is **`:17-29`**, and the four
hits sit at `:19`, `:20` and `:22`. **Zero** hits on the crate root, **zero** on any step of the
opening encounter, **zero** on the handoff and **zero** on the index — which is anti-pattern 6
(`_design.md:888-890`) stated as a count, and it matches the mirror commitment
`invariant-to-appendcondition-bridge/spec.md:430` made.

**Anti-pattern 6 in its reviewer-performable form**, run against the rendered pages so the check
survives a phrase the grep did not anticipate: reading the crate root's render and the three
steps of the opening encounter, neither surface asks the reader to think in streams or entities
at all — the opening encounter's vocabulary is `Tags`, `QueryItem`, `Query` and
`AppendCondition` from its first sentence, and the crate root's is `DomainEvent`,
`DecisionModel` and `commit`. **No unanticipated phrasing found. EC-009 did not fire.**

`examples/course-subscriptions/src/overview.md` carries three `aggregate` occurrences at `:3`,
`:11` and `:12`. It is **outside AC-005's stated corpus** and the occurrences are already owned:
`tension-resolutions/_resolutions.md:407` records them as *"the three `aggregate` occurrences the
surfaced module doc brings with it"*, and `docs/read-the-worked-example.md:10-12` names the seam
and sends the reader to the anchor. Recorded rather than re-filed.

### The one recorded location exists exactly once

```
$ git grep -c "Where your streams went" -- docs/carry-your-invariant.md
docs/carry-your-invariant.md:1

$ git grep -n "Where your streams went" -- docs/carry-your-invariant.md
docs/carry-your-invariant.md:17:## Where your streams went
```

One `##` heading, exactly that text. Its slug `#where-your-streams-went` is stable,
human-readable and unnumbered (IQ-4), so inserting a section above it moves no inbound link, and
the id was read off a render rather than predicted
(`tension-resolutions/_resolutions.md § Anchor table`).

**EC-004 fired and its disposition is the one already recorded.** The heading measures **23**
characters against `_design.md`'s 22-character `##` budget. The budget derives from rustdoc's
200px sidebar TOC clipping with `white-space:nowrap; text-overflow:ellipsis`, and
`tension-resolutions/_resolutions.md:338-343` scoped it to `crate-root-encounter` alone, because
a markdown surface has no 200px sidebar at any width and anti-pattern 5 there would be a check no
page could ever fail. **The budget does not bind this surface; no retitle is needed and none was
made.** Renaming to relieve one character would have moved the one string every inbound link
resolves into.

### Every relying page links it, and the link resolves

```
$ git grep -n "where-your-streams-went" -- crates/happenstance/src/lib.rs docs examples standards
docs/read-the-worked-example.md:12:[where your streams went](carry-your-invariant.md#where-your-streams-went).
examples/course-subscriptions/tests/reach.rs:42:const ANCHOR_LINK: &str = "carry-your-invariant.md#where-your-streams-went";
```

| page | relies on the DT-1 decision? | links the one recorded location? | resolution |
| --- | --- | --- | --- |
| `docs/read-the-worked-example.md` | **yes** — the material it hands off to names the prior model in another voice | **yes**, `:12` | target file `docs/carry-your-invariant.md` exists; fragment `#where-your-streams-went` is the slug of `## Where your streams went` at `:17`. Asserted mechanically too, by `examples/course-subscriptions/tests/reach.rs:42`, which is green |
| `crates/happenstance/src/lib.rs` | **no** — zero prior-model phrases; `_design.md:116-118` binds it to name none | n/a | — |
| `docs/first-encounter.md` | **no** — zero prior-model phrases | n/a | — |
| `docs/carry-your-invariant.md` | it **is** the anchor | n/a | — |

**One relying page, one link, and it resolves.** The obligation is conditional in HS-S0185's own
spec — *"**Where the anchor decision is relied on**, the page links to the bridge's
`#where-your-streams-went` heading rather than re-arguing it"* (`boundary-refusal-encounter/spec.md:418`)
— and neither of the two surfaces that name no prior model relies on it.

`tension-resolutions/_resolutions.md:404` predicted that `boundary-refusal-encounter` would
**link** the anchor, *"the first page to link the anchor before the page that renders it exists"*.
The merged tree does not carry that link, and the reason is the conditional above. **Recorded as
W-6** — a consumption-map row the tree overtook — and routed to the closeout's reference
reconciliation rather than corrected in a signed-off record.

### Anti-pattern 2 — no literal bracketed word survives

```
$ git grep -nE '\[`[a-z_:]+`\]' -- crates/happenstance/src/lib.rs
crates/happenstance/src/lib.rs:70://! The discriminator is **encoding**. [`happenstance_core`](happenstance_core)
crates/happenstance/src/lib.rs:135://! only the first one is due now. [`happenstance::testing`][testing-module] is
crates/happenstance/src/lib.rs:145://! Adapter authors should depend on [`happenstance_core`](happenstance_core)
```

All three carry a target — two inline, one reference-style resolved by the `cfg_attr` link
definitions at `:161-168` — so **none is an unresolved pair**. The source grep finds link
*syntax*; the anti-pattern is about what **renders** as literal brackets, and that was read off
the render: `fence-inventory-and-clause-audit/_inventory.md` records **zero** literal `[bracket]`
pairs in the crate root's rendered `div.docblock`, against the four `_design.md:951-956` measured
pre-merge. The `documentation` REQUIRED step under `RUSTDOCFLAGS=-D warnings`
(`xtask/src/main.rs:344`) is green, which is what denies an unresolved intra-doc link rather than
warning about it.

### The traverse, keyboard-only — and it does not complete

Attempted as AC-006's verification names it: `crate root → bridge → #where-your-streams-went`,
tab to each link and follow it.

```
crate root  crates/happenstance/src/lib.rs:66
              → "the opening encounter" → docs/first-encounter.md          ✓ one link out
docs/first-encounter.md
              → outbound links: three clause links into spec/SPECIFICATION.md,
                two intra-page "#append-and-read-back" links               ✗ no link to the bridge
docs/README.md  (the tree's index)
              → rows for append-conditions.md, first-encounter.md, text-fences.md
                                                                          ✗ no row for the bridge
```

**The traverse does not reach `#where-your-streams-went`.** Every inbound reference to the bridge
in the whole repository is:

```
$ git grep -n "carry-your-invariant" -- docs crates examples standards xtask RUNBOOK.md README.md
docs/read-the-worked-example.md:12:[where your streams went](carry-your-invariant.md#where-your-streams-went).
examples/course-subscriptions/tests/reach.rs:30:const BRIDGE: &str = "docs/carry-your-invariant.md";
examples/course-subscriptions/tests/reach.rs:42:const ANCHOR_LINK: &str = "carry-your-invariant.md#where-your-streams-went";
xtask/src/narrative.rs:141:    #![doc = include_str!("../../docs/carry-your-invariant.md")]
```

and every inbound reference to the handoff is `docs/carry-your-invariant.md:158`, plus the same
two test constants and the harness line. **The bridge and the handoff link only to each other**:
they are an isolated two-page component with no reader-facing route in from the crate root, from
the opening encounter, or from the tree's own index.

That is **W-1**, and it is a finding about *reach* rather than about the anchor. The anchor half
of AC-006 holds independently — the heading exists once, the one relying page links it, and the
link resolves. What does not hold is that a reader can get to the page the anchor is on.

---

## § 7 — The judgement: cites, or re-argues?

The half no command can settle. One yes/no question per relying page — *does this page state the
prior-model decision in its own words, or does it cite the one location?* — recorded as a
judgement with the sentence it turned on **quoted verbatim**, never as a tick. A restated version
is a second recorded home even when it agrees, which is the defect BR-07 exists to prevent
(`project.md:336`).

| page | verdict | the sentence the verdict turned on |
| --- | --- | --- |
| `docs/read-the-worked-example.md` | **cites** | *"Its own explanation names the model you arrived with, in another voice and correctly for its purpose; the single place that model is answered is [where your streams went](carry-your-invariant.md#where-your-streams-went)."* (`:10-12`) |
| `crates/happenstance/src/lib.rs` | **does not rely** | no sentence names a prior model; the four phrases return zero hits |
| `docs/first-encounter.md` | **does not rely** | as above |
| `docs/carry-your-invariant.md` | **is the anchor** | `## Where your streams went`, `:17-29` |

**The judgement on the one relying page, stated so it can be disagreed with.** The sentence does
three things and none of them is re-arguing. It *names* that a prior model is present in the
material downstream (`names the model you arrived with`), it *disclaims* correcting it there
(`in another voice and correctly for its purpose`), and it *points* — `the single place that
model is answered is` — with the link as the sentence's last element. It does not say what the
prior model is, does not say why it does not apply, and does not repeat any part of
`## Where your streams went`. A reader who followed this sentence instead of the bridge would
learn nothing about streams; they would learn only where to go. That is the shape UX-009 asks
for.

**No repair was needed and none was made**, so AC-005's sweep did not have to be re-run against a
changed phrase corpus.

---

## § 8 — Findings, each with a destination

Six, none absorbed. Nothing below was closed by editing a signed-off record, inventing a
notation, adding a lint or declaring a second corpus.

| id | finding | evidence | destination | why it is not closed here |
| --- | --- | --- | --- | --- |
| **W-1** | **`docs/carry-your-invariant.md` and `docs/read-the-worked-example.md` are an isolated two-page component.** Neither appears in `docs/README.md`'s narrative index, `docs/first-encounter.md` links neither, and the crate root links only the opening encounter. They link to each other and nothing links to them. The keyboard traverse in `§ 6` does not complete | `§ 1` cross-check 1; `§ 6` traverse and the two `git grep` inbound sweeps | **HS-P0023** `reach-and-adapter-path` — DT-10 and every pointer policy | HS-S0188 already deferred the handoff's index row to HS-P0023 with the same reasoning (`surface-course-subscriptions/report.md:50-53`); this story extends that finding from one page to the **component**, which is the set-level observation only the last slice can make. Adding rows would mean authoring the index's one-line orientation copy for two pages, which is a pointer-policy decision under RP-10-2 and is two rows rather than one line — EC-007 caps this story at one |
| **W-2** | **HS-P0021's declaration form has no defined anchor on a rustdoc crate root.** RP-00-2 anchors the line to a markdown `# Title`; a crate-root doc comment has none. The form was consumed verbatim and the *position* taken from `_design.md:426-431` | `§ 3`, `crate-root-encounter` scoping 1 | **HS-P0021** `page-need-discipline` | Writing a variant notation is DR-14's named defect, and inventing one to unblock a criterion is the worst version of it. The gap is in the rule, and the rule is HS-P0021's |
| **W-3** | **RP-40-1 states no behaviour for a page outside the governed tree.** Step 3 asks whether every *section* serves the declared need; a rustdoc crate root carries sections the medium requires and band 10 explicitly excludes rustdoc from the need set | `§ 3`, `crate-root-encounter` scoping 2; `standards/pages/10-the-need-set.md:26-31` | **HS-P0021** `page-need-discipline` | Same reason as W-2. The scoping applied here is disclosed in full so a reviewer can reject it; if rejected, the crate root's verdict turns on copy `project.md`'s risk table places with HS-P0016 |
| **W-4** | **Nothing in the repository reads the crate root's answered-need line.** It is outside `TREE`, so `every page declares one need` never opens it. `xtask/tests/first_encounter.rs:184` asserts it, and that is a story's test rather than the discipline's instrument | `§ 4` coverage table, middle column | **HS-P0021** `page-need-discipline`, as an input to what its checker's corpus should reach | Widening the corpus is HS-P0021's constant to change, and the file cannot move into `TREE`: `include_str!` resolves at compile time and a path escaping the package would not resolve once published (`crates/happenstance/src/lib.rs:7-9`). Agrees with the slice-mate's R-2/R-5 about the same file |
| **W-5** | **The corpus constant is `xtask::lint_narrative::TREE`, not `xtask::narrative::TREE`** as HS-P0021's spec names it | `§ 4`, the pasted `git grep` | **HS-P0021** `page-need-discipline` — a citation correction | EC-003 halts when the constant is *absent*. It is present, it is the single corpus, and `lint_pages` declares no `PAGE_DIR` — the substance EC-003 protects holds, so the response is to record the path rather than to halt |
| **W-6** | **`tension-resolutions/_resolutions.md:404`'s consumption map says `boundary-refusal-encounter` links `Where your streams went`; the merged tree does not carry that link** | `§ 6`, the inbound sweep; `boundary-refusal-encounter/spec.md:418` | **HS-P0025** `durable-audience-closeout` — the reference reconciliation | Not a defect: HS-S0185's own obligation is conditional on *relying* on the decision, and the crate root and the opening encounter name no prior model at all, so no link is owed. The stale prediction sits in a signed-off record this story may not edit |

**Vacuous, blocked and not-yet-authored rows: none.** Every surface existed at the time of the
walk (EC-001 clear), a non-author walker was available for all four (EC-008 clear), HS-P0021's
atoms and checker are present (EC-002 clear), the corpus constant is present (EC-003 clear),
EC-004's measurement was taken and its disposition was already recorded, EC-005 fired and is
routed as W-2, EC-006 did not fire (the walk reached `fail` twice), EC-007 held — no repair was
needed and none exceeded one line because none was made — and EC-009 did not fire.

---

## Re-deriving this record

```
git grep -c '^> \*\*Answers:\*\*' -- docs/first-encounter.md docs/carry-your-invariant.md \
    docs/read-the-worked-example.md                          # § 3, one each
cargo run --locked --quiet -p xtask -- lint-pages            # § 4, the mechanical half
git grep -ni "aggregate" -- crates/happenstance/src/lib.rs docs/first-encounter.md \
    docs/carry-your-invariant.md docs/read-the-worked-example.md docs/README.md
                                                             # § 6, and the three sibling phrases
git grep -n "Where your streams went" -- docs/carry-your-invariant.md      # § 6, exactly one
git grep -n "where-your-streams-went" -- crates/happenstance/src/lib.rs docs examples standards
                                                             # § 6, every inbound link
git grep -n "carry-your-invariant" -- docs crates examples standards xtask # § 6, the traverse
cargo xtask affected --base main                             # the story grain
```

`§ 3`'s verdicts and `§ 7`'s judgement are tier 5 — a person, recorded once
(`_decomposition.md:486`) — and are re-derived by re-executing
`standards/pages/40-reviewing-a-page.md` over the four surfaces, not by a command. `§ 5`'s
calibration is tier 4 and is re-derived by re-running the injection.
