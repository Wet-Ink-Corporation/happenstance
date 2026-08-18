---
item: "HS-S0148"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — DT-8 resolved as a rule a reviewer can apply without the author

## Findings Ledger

The story's outcome as the review gate reads it. All nine ACs are satisfied by reachable
behaviour with a `file:line`, a captured measurement and a passing test. Nothing is
stubbed or deferred; no fold-checker was built, which is a decision the atom states in its
own text rather than an omission. Four deviations from the spec's literal verification
forms are recorded below and in `_ledger.md`.

| AC | Result | Proved by | Mount point |
| -- | ------ | --------- | ----------- |
| AC-001 | **Met.** The deletion test is in the atom verbatim, as a question with a yes/no answer and a stated consequence, and no atom in the tree contains any of the five hedging words. | `standards/pages/20-the-fold-line.md:1-40`; `::tests::band_twenty_hands_the_reviewer_the_deletion_test`, `::tests::no_rule_in_the_tree_hands_back_the_judgement_it_replaces` | `standards/pages/README.md:38,53` |
| AC-002 | **Met.** Exactly five enumerated never-fold classes; "no reviewer may grant an exception"; the list is closed and growing it is a disagreement with sign-off condition 3. | `standards/pages/20-the-fold-line.md:42-60`; `::tests::band_twenty_closes_the_never_fold_list_at_five` | `standards/pages/README.md:53` |
| AC-003 | **Met.** Class 1 *is* the declaration; band 20 names `00` by number and band 00 already named `20`, so the pairing is findable from either side and band 00 was not edited to say so. | `standards/pages/20-the-fold-line.md:5,51`; `standards/pages/00-one-need.md:5`; `::tests::band_twenty_pays_band_zeros_deferral` | `standards/pages/README.md:53` |
| AC-004 | **Met.** The permitted-mechanism table exists with **zero** data rows, folding is "forbidden in practice" in those words, and DT-7 is named as what lifts it. | `standards/pages/20-the-fold-line.md:93-105`; `::tests::band_twenty_ships_an_empty_permitted_mechanism_table` | `standards/pages/README.md:53` |
| AC-005 | **Met.** Four recorded observations, in this repository; upstream documentation is not an observation; an unverified property counts as unmet. | `standards/pages/20-the-fold-line.md:99-105`; `::tests::band_twenty_holds_a_mechanism_to_four_recorded_observations` | `standards/pages/README.md:53` |
| AC-006 | **Met, in both halves.** RP-20-2 binds the author's own markup, so rustdoc's `toggle top-doc` wrapper puts no page in breach — **and** the wrapper is recorded as an unmet property owed by the hosting decision, because nobody here has watched what survives `#toggle-all-docs`. | `standards/pages/20-the-fold-line.md:62-69`; `::tests::band_twenty_answers_the_renderer_supplied_wrapper_in_both_halves` | `standards/pages/README.md:53` |
| AC-007 | **Met.** `## What this rule does not do` names all four limits, unfolded. The negative half holds: the only `PERMITTED_FOLD_MECHANISMS` string in `xtask/` is band 00's guard asserting its absence — no constant of that name is declared. | `standards/pages/20-the-fold-line.md:146-159`; `xtask/src/lint_pages.rs:881`; `::tests::band_twenty_states_what_this_rule_does_not_do` | `standards/pages/README.md:53` |
| AC-008 | **Met.** Four rules, five fixed sections each in order, every `Not` naming a wrong page and every `**Rejects.**` over 120 characters; 8,309 bytes, no prose line over 96 columns, **no fence at all**, no disclosure markup anywhere in the tree. | `standards/pages/20-the-fold-line.md`; the seven whole-tree composition tests, widened from a two-atom list to `TREE` by this story | `standards/pages/README.md:53` |
| AC-009 | **Met.** One `## Start here` row keyed to the author's real moment, one generated-index row derived from the atom and byte-identical to the generator's format; router still 4,476 bytes and 7 filter rows. | `standards/pages/README.md:38,53`; `::tests::every_atom_is_reachable_from_both_router_regions`, `::router_index_rows_are_byte_identical_to_the_generator`, `::router_is_inside_its_budgets` | `standards/pages/README.md` |

**Presentation, and what an unstyled render would have failed.** The atom is composed in
the constitution's own grammar — `# 20 — The fold line`, a one-source-line `Load when:`, a
by-number `See also`, `---`, then four `## RP-20-N.` imperatives each carrying **Why.** ·
**Do** · **Not** · **Rejects.** · **Evidence.** in that fixed order. A shapeless render
fails five measurements here: the five-item enumerated class list, the four-rule and
16,384-byte ceilings, the 96-column prose budget, the 120-character `**Rejects.**` floor,
and the five-marker order per rule. Hierarchy is heading level plus bold run-in markers;
nothing is carried by colour, size or an icon; the atom contains no `<details`, no tab and
no accordion, because an atom about folding that folds is anti-pattern 6 self-applied.

**Deviations, stated rather than hidden.**

1. **This PR touches `xtask/src/`**, which its boundary forbade. The slice is implemented in
   one context and the module exists; the nine criteria are assertions rather than pasted
   command output. Two of them caught real defects while drafting.
2. **The atom carries no fence.** Its wrong pages *are* disclosure markup; writing them
   would violate the rule the atom states. It says so at `:9-14`.
3. **AC-004's literal `awk` form is unreachable** — a paragraph line directly above a table
   stops it being a table. The substance (header, separator, zero data rows) is asserted
   directly.
4. **AC-009's `rg` returns one hit, not two** — `## Start here` names bands by number, which
   is the design's cited pattern and what keeps the rows for bands `30` and `40` from
   dangling before those files land.

**Deferred / not this story.** No fold-checker, no marker scanner, no `const`: the
enforcement seam is HS-P0020's DT-7 and the checker story's, and the atom's closing section
says so. Bands `30` and `40` are the remaining slice-mate's. The `[PROVISIONAL …]` marker in
`docs/README.md` is untouched and still named for `page-need-checker-mounted-in-the-gate`.
