---
item: "HS-S0147"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — The rules tree, its router, its rank in the chain, and its announcement

## TDD Evidence

Ten tests were written first, in `xtask/src/lint_pages.rs`'s `mod tests`, against a
tree with no router and a `docs/README.md` that had never heard of it. The red run is
`cargo test -p xtask --bin xtask lint_pages`: **23 passed; 10 failed**. Nine of the ten
failed reading `…\standards/pages/README.md: The system cannot find the file specified.
(os error 2)`; the tenth failed its own assertion, `docs/README.md's 'Looking for / It is
at' table links the router`. Every failure is an assertion about missing behaviour. The
green run, after the router and the two `docs/README.md` edits landed, is **33 passed; 0
failed**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `router_opens_with_the_scope_paragraph_and_the_band_table` | RED: router missing. GREEN: `# Page standards` is line 1, the scope paragraph carries "load one rule, never the tree", and the band table's five first cells read `` `00` ``…`` `40` `` in order. |
| AC-002 | `router_states_its_rank_without_editing_the_chain` | RED: router missing. Then RED a second time, on the *guard half*: the assertion that `standards/rust/README.md` still contains its precedence blockquote verbatim failed because this is a Windows checkout with `core.autocrlf=true` and the file arrives CRLF. Normalised line endings before matching (a fix to the check's reach, not to its strength); GREEN. |
| AC-003 | `router_indexes_every_atom_in_the_tree` | RED: router missing. GREEN: two in-region rows for two atoms, both links resolving, no `show all` and no `<details`. |
| AC-004 | `router_index_rows_are_byte_identical_to_the_generator` | RED: router missing. GREEN: header and separator asserted against the **live** region of `standards/rust/README.md`, and each data row derived from the real atom by a local copy of `generated_region`'s format. This is the spec's `diff` turned into a compiled assertion. |
| AC-005 | `router_states_the_one_source_line_rule_for_load_when` | RED: router missing. GREEN: `## The shape of a rule` present, "one source line" stated with its reason, and every atom in the tree checked against it. |
| AC-006 | `router_states_what_checks_this_tree_and_what_does_not` | RED: router missing. GREEN: the section states "no gate step reads `standards/pages/`" and names `page-need-checker-mounted-in-the-gate`. |
| AC-007 | `the_repository_index_reaches_the_rules_tree_in_one_hop` | RED: `docs/README.md` had no row. Then RED twice more — once because the `[PROVISIONAL — settles at …]` marker had wrapped across a source line, and once because the preserved closing sentence had. Fixed by putting the marker on one line and by flattening whitespace before matching prose; GREEN. |
| AC-008 | `router_regions_are_in_the_binding_order` | RED: router missing. GREEN: the heading sequence is exactly the six the design fixes, in order, and none of nine occlusion / size / widget markers appears. |
| AC-009 | `router_is_inside_its_budgets` | RED: router missing. GREEN: 4,316 bytes ≤ 8,192; no prose line over 96 columns; 7 `## Start here` rows ≤ 12; no fence at all, so neither a `rust`-tagged nor an untagged one. |
| — | `router_is_created_by_the_router_story` | The **inversion** `need-vocabulary-and-declaration-form` scheduled. RED at this story's start (the file did not exist); GREEN once it did. It replaces `router_is_not_created_by_this_story` rather than deleting it quietly, which is what that story's spec and EC-006 require. |

No test was weakened to reach green. The three RED-after-GREEN episodes were all line-ending
or line-wrapping artefacts of matching prose from a Windows checkout; each was fixed by
making the assertion read words rather than byte offsets, and each still fails if the words
go away.

## Commits

| SHA | Subject |
| --- | ------- |
| `9dacc7d` | `feat(page-need-discipline): Router, precedence and announcement` |

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `standards/pages/README.md` | **New**, 4,316 bytes. Seven regions in the binding order: `# Page standards`, the scope paragraph, the five-row band table, `## Precedence` (the constitution-atom-tier blockquote), `## Start here` (7 intent rows), `## Index` (the marker-delimited generated region, two rows), `## The shape of a rule` (the atom grammar and the one-source-line constraint), `## What checks this tree, and what does not`. No fence anywhere; the grammar is shown in inline code spans, as `standards/rust/README.md:99-107` shows the constitution's. |
| `docs/README.md` | **Two edits.** One new "Looking for / It is at" row at `:24` linking `../standards/pages/README.md`. The pin-by-path paragraph at `:37-42` now names the rules tree as a **fourth** pinned tree carrying `[PROVISIONAL — settles at \`page-need-checker-mounted-in-the-gate\`]`; the closing reasoning sentence is preserved verbatim and only its referent moved. |
| `xtask/src/lint_pages.rs` | **Test module only** (plus one stale doc comment corrected on `ROUTER`). Six new helpers — `router`, `load_when`, `expected_index_row`, `generated_region`, `table_rows`, and the `TREE` list — and ten tests. `router_is_not_created_by_this_story` became `router_is_created_by_the_router_story`. |
| `…/router-precedence-and-announcement/_ledger.md` | Nine rows flipped `false → true`, each carrying a `file:line`, a command's captured output, and the passing test id. |

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test -p xtask --bin xtask lint_pages` | **RED** 23 passed / 10 failed → **GREEN** 33 passed / 0 failed |
| `cargo test -p xtask` | green — 211 + 168 + 63 across the three targets |
| `cargo clippy -p xtask --locked --all-targets --all-features -- -D warnings` | green |
| `cargo fmt --all --check` | green (formatter `--write` pass run last, after the edits) |
| `cargo xtask lints` | green — `27 atoms, all consistent`; `2 pages, all consistent` |
| `cargo xtask spec-trace` | green — `traceability: no problems found` |
| `cargo xtask ci --fast` | green — `all required checks passed (--fast: 4 optional step(s) not run)` |
| `cargo xtask affected --base main` | green, still **wide** — EC-007, recorded and not fixed |
| `git diff main -- standards/rust/README.md` | empty |
| `wc -c < standards/pages/README.md` | `4316` (ceiling 8,192) |

## Notes

**Deviation 1 — this PR touches `xtask/src/`, which its own boundary forbade.** The spec's
Clarification 2 declares the static tier empty "by construction", because the story was
written to merge *after* `need-vocabulary-and-declaration-form` and therefore to have no
module of its own to put a test in. Inside this slice that premise does not hold: the slice
is implemented in one context, `xtask/src/lint_pages.rs` exists as of `55b987b`, and the
slice-mate's own spec **schedules this story to invert
`router_is_not_created_by_this_story`** — which is an `xtask/src/` edit by definition. So
the nine mechanical checks the spec listed as commands were written as tests in that module
instead of being run once and pasted. This strictly increases what is checked, contradicts
nothing in `_design.md`, and hands
`page-need-checker-mounted-in-the-gate` the three assertions the spec's `static` row asks it
to inherit already written. `git diff main -- xtask` is therefore **not** empty; the
merge-DoD line that asked for it was written against a `main` where the slice-mate had
already merged.

**Deviation 2 — the announcement names a fourth tree, not a third.** `docs/README.md`
already said *"Three trees are read by the gate rather than only by people"*: HS-P0020's
narrative tree landed between this spec being written and this slice being implemented. The
paragraph therefore gains a fourth, with the marker, and "Moving any of the three" became
"Moving any of them". The reasoning sentence is untouched
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).

**Deviation 3 — `## The shape of a rule` shows the grammar in inline code, not in a fence.**
The spec's Implementation notes expected at least one `text` fence there. A fence containing
`# NN — Title` would have put a line beginning `# ` into the file, which AC-008's heading
capture reads as a seventh heading. The constitution solves the same problem the same way
(`standards/rust/README.md:99-107` is prose with inline spans), so this follows the
precedent rather than inventing around it — and it makes the router's fence set empty, which
satisfies AC-009's fence rule vacuously and honestly.

**EC-001 and EC-002 both held.** The region was authored last, from the two atoms that
exist, so it is neither empty nor dangling. No `## Start here` row and no index row points
at bands `20`, `30` or `40`: those rows land with the slice-mates that create the files, in
this same slice. The band **table** names all five bands, because the band table is the
namespace rather than an index.

**`cargo xtask affected --base main` widened to the whole workspace and was not "fixed".**
`standards/pages/` is unrecognised by `affected_packages` and absent from `INERT`. The
repair is a pair of edits that only makes sense with the checker
(`xtask/src/affected.rs:222-223`, CR-4), and it belongs to
`page-need-checker-mounted-in-the-gate`.

**Correction, 2026-08-18 (slice review fix, not part of this story's checkpoint).** Two
things this report records as green were wrong and have been repaired in a later commit on
the same branch. First, `## What checks this tree, and what does not` said *"no gate step
reads `standards/pages/`"* and `docs/README.md` said *"no gate step reads it yet"*; both
were false as merged, because the same slice's `xtask/src/lint_pages.rs` test module reads
every atom in the tree under `cargo xtask ci`'s mandatory `tests` step. Both now say what
is actually missing — the **dedicated** `lint-pages` step and the directory-walking corpus
reader — and `::router_states_what_checks_this_tree_and_what_does_not` pins the corrected
sentence and rejects the old one. Second, AC-002's and AC-004's assertions read
`standards/rust/README.md` directly; they now compare against `PRECEDENCE_CHAIN` and
`GENERATED_HEADER`, literals inlined in `xtask/src/lint_pages.rs` and commented with the
lines they were copied from, so a pages-tree test cannot go red for a constitution-tree
reason (RS-81-3). `git diff main -- standards/rust/README.md` stays the ledger-side proof
that the chain is unedited.
