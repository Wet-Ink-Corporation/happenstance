---
item: "HS-S0149"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — The cite-never-restate rule, the non-author verdict walk, and the paraphrase spot check

## Findings Ledger

The story's outcome as the review gate reads it. All ten ACs are satisfied by reachable
behaviour with a `file:line` and a passing test, and both procedures were **actually run**
with their runs recorded verbatim in `_ledger.md` rather than asserted. One residual is
carried openly on AC-007 and is the single thing in this slice a later reader should not
treat as fully closed.

| AC | Result | Proved by | Mount point |
| -- | ------ | --------- | ----------- |
| AC-001 | **Met.** Band 30 hands the author a falsifiable test, not a preference, and fixes the citation as the stable id in visible link text with the stability claim cited. | `standards/pages/30-citing-the-specification.md:17-39,41,71-77`; `::tests::band_thirty_gives_the_clause_or_page_test` | `standards/pages/README.md:54` |
| AC-002 | **Met.** The blind spots come first, asserted by byte offset: resolution is HS-P0020's `clause_ids`, restatement is checked by nothing mechanical, and a green gate is not a claim about paraphrase. | `standards/pages/30-citing-the-specification.md:9-15`; `::tests::band_thirty_states_its_blind_spots_before_its_first_rule` | `standards/pages/README.md:54` |
| AC-003 | **Met.** `### The walk` is a unique anchor inside RP-40-1; the performer is *not the author*, and the author and the page's git history are both forbidden, with the reason attached. | `standards/pages/40-reviewing-a-page.md:14,21-28,68`; `::tests::band_forty_names_who_walks_and_what_they_may_consult` | `standards/pages/README.md:55` |
| AC-004 | **Met.** Six numbered steps, each a single line ending in `?`, each with a stated consequence; no hedging word anywhere in the tree. | `standards/pages/40-reviewing-a-page.md:30-42`; `::tests::the_walk_is_answerable_steps_only`, `::tests::no_rule_in_the_tree_hands_back_the_judgement_it_replaces` | `standards/pages/README.md:55` |
| AC-005 | **Met.** Four verdict rows; `indeterminate` is defined as a defect in the page rather than in the procedure, so no soft pass exists. | `standards/pages/40-reviewing-a-page.md:44-49`; `::tests::the_walk_closes_in_a_four_row_verdict_table` | `standards/pages/README.md:55` |
| AC-006 | **Met, and run.** The instruction says *vacuous, never as a pass*, in those words. The sweep performed for this story is recorded as **vacuous** — `grep -c '^> \*\*Answers:\*\*' docs/*.md` → `0, 0, 0`, no governed page exists at this merge. | `standards/pages/40-reviewing-a-page.md:51-54`; `::tests::the_walk_records_an_empty_corpus_as_vacuous`; the recorded sweep in `_ledger.md` | `standards/pages/README.md:55` |
| AC-007 | **Met in substance, with one residual.** The fixture carries two declarations, is linked from RP-40-1 and is inert by placement. The walk was executed against it step by step from the rendered page alone and reached **`fail — two needs`**; step 1 was executed independently by a machine and agrees. **Residual:** the walker was the implementing agent, who authored the fixture; the criterion asks for a *named person who did not author it*, and no such actor exists inside a single-context slice. The independent human walk is owed at this project's review and integration stages. | `standards/pages/examples/two-needs.md:3-4`; `standards/pages/40-reviewing-a-page.md:26`; `::tests::the_walk_is_calibrated_against_a_two_need_fixture`; the recorded walk in `_ledger.md` | `standards/pages/40-reviewing-a-page.md:26` |
| AC-008 | **Met, and run.** RP-40-2 names the corpus, the per-sentence question and the restatement it hunts. The spot check was run file by file over `standards/pages/**`: six files, sixteen rule imperatives, eight normative-modal occurrences (all mentions or labelled counter-examples), four clause-id occurrences (all citation specimens). **No page has become a second specification.** No second clause-id parser was built; `git diff main -- spec/` is empty. | `standards/pages/40-reviewing-a-page.md:68-96`; `::tests::band_forty_carries_the_paraphrase_spot_check`; the per-file verdicts in `_ledger.md` | `standards/pages/README.md:55` |
| AC-009 | **Met.** Both atoms composed in the tree's enforced grammar and inside every budget: 3 rules / 4,322 bytes and 2 rules / 5,087 bytes; five sections per rule in order; no rule missing its `Not` or `Rejects.`; no prose line over 96 columns; no `rust`-tagged and no untagged fence. | both atoms; the seven whole-tree composition tests | `standards/pages/README.md:54-55` |
| AC-010 | **Met.** Both atoms reachable from **both** router regions; 5 index rows for 5 atoms, byte-identical to the generator's format; every `.md` link in the tree resolves, including the fixture's; nothing behind a disclosure and no meaning carried by size, colour or an icon. | `standards/pages/README.md:54-55`; `::tests::every_markdown_link_in_the_tree_resolves`, `::every_atom_is_reachable_from_both_router_regions`, `::router_index_rows_are_byte_identical_to_the_generator`, `::router_is_inside_its_budgets` | `standards/pages/README.md` |

**Presentation, and what an unstyled render would have failed.** Band 40 is the story's
riskiest surface for a bare render: a file containing the words "walk" and "verdict" passes
a keyword grep. What it fails here is the four-row table with its four literal verdicts, the
six steps each ending in `?`, the `### The walk` anchor positioned inside RP-40-1's body,
and the five fixed sections per rule. Band 30's `**Do**` / `**Not**` pair is a *composed*
contrast — a `text` fence showing a citation beside a `text` fence showing the restatement
it forbids — which is the shape that makes the rule legible without the prose. Hierarchy is
heading level plus bold run-in markers throughout; the tree carries no `<details`, no tab
and no accordion, and the only opened-on-demand control anywhere in the diff is the link to
the fixture.

**Deviation.** This PR touches `xtask/src/`, which its boundary forbade. The slice is
implemented in one context and the module exists; the spec's Clarification 2 named three
assertions as inherited by the checker story and they are written now instead — strictly
more checking, not less. `git diff main -- xtask` is therefore not empty; `git diff main --
spec/`, `-- .kb` and `-- .redkiln/templates` all are.

**Deferred / not this story.** No fold-checker, no second clause-id parser, no gate step,
no `INERT` entry, no `docs/README.md` edit. Re-observing DoD-8 and DoD-12 on the assembled
narrative tree is HS-P0025's; the comprehension evidence is HS-P0024's; the mechanical
half of clause-id resolution is HS-P0020's `clause_ids`.
