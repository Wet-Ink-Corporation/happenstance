---
item: "HS-S0147"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — The rules tree, its router, its rank in the chain, and its announcement

## Findings Ledger

The story's outcome as the review gate reads it. All nine ACs are satisfied by reachable
behaviour, each with a `file:line`, a captured measurement and a passing test. Nothing is
stubbed or deferred. Three deviations from the spec's own boundary are stated plainly
below rather than buried; one of them is a deliberate widening of what is checked.

| AC | Result | Proved by | Mount point |
| -- | ------ | --------- | ----------- |
| AC-001 | **Met.** The router opens on its H1, the load instruction and the five-row band namespace; two UX-012 walks each ended at exactly one file. | `standards/pages/README.md:1,6,9-15`; `::tests::router_opens_with_the_scope_paragraph_and_the_band_table` | `docs/README.md:24` |
| AC-002 | **Met.** The rank is answered in place, at the constitution-atom tier, adding no tier; `git diff main -- standards/rust/README.md` is empty and the chain's own blockquote is asserted intact from inside the diff. | `standards/pages/README.md:17-29`; `::tests::router_states_its_rank_without_editing_the_chain` | `docs/README.md:24` |
| AC-003 | **Met.** The index is complete, on the same page as the filter, behind no toggle: 2 data rows for 2 atoms, both links resolving. | `standards/pages/README.md:43-53`; `::tests::router_indexes_every_atom_in_the_tree` | `standards/pages/README.md` |
| AC-004 | **Met, mechanically rather than procedurally.** Header and separator asserted against the *live* constitution region; every data row derived from the real atom in `generated_region`'s exact format, so the checker's first `--write` produces no diff by construction. | `standards/pages/README.md:49-52`; `xtask/src/lint_constitution.rs:400-420`; `::tests::router_index_rows_are_byte_identical_to_the_generator` | `standards/pages/README.md` |
| AC-005 | **Met.** `## The shape of a rule` gives the grammar and the one-source-line constraint with its reason, and every atom in the tree is checked against it; no in-region trigger cell ends mid-phrase. | `standards/pages/README.md:55-73`; `::tests::router_states_the_one_source_line_rule_for_load_when` | `standards/pages/README.md` |
| AC-006 | **Met.** The router states its own blind spot first: no gate step reads this tree yet, `page-need-checker-mounted-in-the-gate` adds one, and a check will still never see whether a page *answers* its need. | `standards/pages/README.md:75-86`; `::tests::router_states_what_checks_this_tree_and_what_does_not` | `standards/pages/README.md` |
| AC-007 | **Met, with the count adapted.** One row in the repository index reaches the router in one hop, keyboard only; the pin-by-path paragraph names the tree with `[PROVISIONAL — settles at page-need-checker-mounted-in-the-gate]` and keeps its closing reasoning verbatim. The paragraph already named **three** gate-read trees, so this is the **fourth**. | `docs/README.md:24,37-42`; `::tests::the_repository_index_reaches_the_rules_tree_in_one_hop` | `docs/README.md` |
| AC-008 | **Met.** Six headings in exactly the binding order, the filter above the thing it filters, every region composed in the repository's own textual grammar, and none of nine occlusion / size / widget markers present. | `standards/pages/README.md:1,17,31,43,55,75`; `::tests::router_regions_are_in_the_binding_order` | `docs/README.md:24` |
| AC-009 | **Met, measured not asserted.** 4,316 bytes (ceiling 8,192); no prose line over 96 columns; 7 `## Start here` rows (ceiling 12); no fence at all, so no `rust`-tagged and no untagged one; every `.md` link resolves. | `standards/pages/README.md`; `::tests::router_is_inside_its_budgets` | `standards/pages/README.md` |

**Presentation, and what an unstyled render would have failed.** A file carrying the six
headings and nothing else passes most greps and fails four measurements here: the band
table's five ordered rows, the index row count equalling the tree, the 8,192-byte ceiling,
and the heading order that puts `## Index` **below** `## Start here`. All four are
assertions, not eyeballs. The precedence rank is a blockquote, the filter and the namespace
are tables, the index is a marker-delimited generated region, and hierarchy is carried by
vertical order alone — the index is recessive because it is last and generated, never
because it is smaller. Read through a plain pager with no renderer, every region is legible
and nothing is lost.

**Deviations, stated rather than hidden.**

1. **This PR touches `xtask/src/`, which the spec's PR boundary forbade.** The spec declared
   its static tier empty "by construction" because it expected no module of its own to
   exist. Inside this slice one does: `xtask/src/lint_pages.rs` landed at `55b987b`, and the
   slice-mate's spec explicitly schedules this story to invert
   `router_is_not_created_by_this_story`. The nine mechanical checks were therefore written
   as tests rather than run once and pasted. `git diff main -- xtask` is not empty.
2. **The announcement names a fourth tree, not a third** — HS-P0020's narrative tree already
   occupies the third slot in `docs/README.md`'s paragraph.
3. **`## The shape of a rule` uses inline code, not a fence** — a fence containing
   `# NN — Title` would read as a seventh heading to AC-008's capture; the constitution
   solves the same problem the same way at `standards/rust/README.md:99-107`.

**Deferred / not this story.** Bands `20`, `30` and `40` and their `## Start here` and index
rows belong to the two slice-mates that follow in this same slice; the band table names all
five bands because it is the namespace, not the index. The checker, its gate step, the
`INERT` pair and the removal of the `[PROVISIONAL …]` marker are
`page-need-checker-mounted-in-the-gate`'s, and the marker names that story verbatim so the
obligation is greppable.
