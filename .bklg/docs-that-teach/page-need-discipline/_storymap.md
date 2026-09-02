---
item: HS-P0021
stage: storymap
created: 2026-08-17T03:21:37.920Z
updated: 2026-08-17T03:21:37.920Z
template_sig: 1c63534a
rendered_sig: ad7c10af
---

# Story Map — Page-Need Discipline

Eight stories in three slices. The cut follows the two deliverables and one seam the
architecture brief names ([`_decomposition.md`](_decomposition.md), Architecture brief,
"Intent"): a **rules tree** that a reader navigates, a **checker** that reads it and the
pages tree HS-P0020 pins, and the declaration form (DR-05) that binds the two. The tree
and the checker are separate slices because they are separate *surfaces* with separate
readers — the next page author reads one, the author who just broke a rule reads the
other — but the const that spans them is landed first, as a foundation story, so neither
slice ever holds a second copy of the need set.

## Backbone

The activities across the top, in the order a person meets them.

| # | Activity | Who is at it | The outcome that makes it true |
| --- | --- | --- | --- |
| A1 | **Decide what a page must declare, and in what form** | the author of the discipline | a closed need set and one declaration grammar, resolved with rejected options named |
| A2 | **Find the one rule that governs what I am about to write** | the next page author, the reviewer | a router that filters without hiding, ranked inside the precedence chain, announced where the repository's other trees are |
| A3 | **Judge a page I did not write** | a non-author reviewer | written procedures that yield a verdict from the page alone |
| A4 | **Be stopped, by name and by line, when I get it wrong** | the author who just broke the rule | a gate step that reports every problem in one run — and that has been watched failing |
| A5 | **Reach the rule from the page, and keep it past this initiative** | every reader; the next initiative | an in-place link from what the discipline governs, and a playbook atom staged for ingest |

Surfaces S1–S4 from the ux brief ([`_decomposition.md`](_decomposition.md), UX brief,
"Intent") map onto these: S1 (the declaration) is settled in A1 and enforced in A4;
S2 (the tree) is A2; S4 (the reviewer procedure) is A3; S3 (the lint's terminal output)
is A4.

## Slices

Stories sharing a milestone are implemented in one context and mounted as one surface.
Cross-milestone `depends_on` edges are acyclic and are exactly the merge order below.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `discipline-on-disk` | `need-vocabulary-and-declaration-form` | foundation | Resolve DT-2, DT-3 and DR-05 in `_design.md` with the rejected options and the HS-P0020 hosting assumption named, and land the closed need set **once** — as the `NEEDS` const in the new `xtask/src/` module and as the rule atom that documents it. | — | AC-003, AC-004 |
| `discipline-on-disk` | `router-precedence-and-announcement` | capability | Stand the tree up sibling to `standards/rust/` with a router carrying both an intent-keyed trigger table and a complete index, state its rank inside the five-tier chain without editing `standards/rust/README.md:23-29`, and take its row in `docs/README.md`'s table and its name in the gate-read-trees paragraph. | `need-vocabulary-and-declaration-form` | AC-001, AC-002, AC-011 |
| `discipline-on-disk` | `fold-line-rule` | capability | Resolve DT-8 and write it as a rule a reviewer applies without the author — *if the collapsed section were deleted, would the page still teach the constraint correctly?* — covering the declaration itself, which may never be occluded. | `router-precedence-and-announcement` | AC-005 |
| `discipline-on-disk` | `reviewer-and-citation-procedures` | capability | Write the cite-never-restate rule and the two procedures a byte count cannot replace — the non-author verdict walk (DR-07) and the paraphrase spot check — and run each once over the set as it stands, recorded in `_ledger.md`. | `router-precedence-and-announcement` | AC-009, AC-010 |
| `page-need-gate-step` | `page-need-checker-mounted-in-the-gate` | capability | A bin-crate module modelled on `xtask/src/lint_constitution.rs` that parses each page's declaration to a line number, rejects zero, two or unenumerated needs, guards both pinned trees with `.with_context()` **and** a vacuity `bail!`, generates the router's index region from `NEEDS`, and is mounted in `REQUIRED`, dispatch, `print_help`, `lint_steps` and `affected.rs`'s `INERT` in one change. | `need-vocabulary-and-declaration-form`, `router-precedence-and-announcement`, `fold-line-rule` | AC-001, AC-003, AC-004, AC-006, AC-008 |
| `page-need-gate-step` | `declaration-check-seen-to-fail` | capability | Break a governed page two ways — two needs, then an unenumerated need — watch `cargo xtask ci` fail by file and line each time, revert, and record green plus a clean `git status`; all three captures verbatim in the ledger. | `page-need-checker-mounted-in-the-gate` | AC-007 |
| `binding-beyond-this-project` | `governed-page-cites-the-discipline` | capability | At least one page in HS-P0020's pinned tree names the discipline as the reason it is shaped as it is, as a link the checker's own pin keeps resolving, and the discipline links back to the material it governs. | `router-precedence-and-announcement`, `page-need-checker-mounted-in-the-gate` | AC-011 |
| `binding-beyond-this-project` | `playbook-atom-staged-for-ingest` | capability | Stage a `kind: playbook`, `authority_tier: guideline` atom under `.kb/_intake/` carrying the method, the rejected alternatives and the conditions under which the discipline stops holding — with a field-by-field frontmatter check in the ledger, because `redkiln validate --kb` skips `_`-prefixed directories by design. | `fold-line-rule`, `reviewer-and-citation-procedures`, `declaration-check-seen-to-fail` | AC-012 |

### Why the slices fall here

- **`discipline-on-disk` is one slice, not four PR-shaped fragments.** The router's
  index is generated from the corpus it indexes, so a rule atom that lands without its
  router row lands half-mounted — the failure `check_summaries` names as *"a second copy
  of it, and one of the two will be stale"* (`xtask/src/lint_constitution.rs:466-470`).
  The `docs/README.md` announcement is in this slice for the same reason: a tree that
  exists and is unreachable is the adapter author's measured defect one level up
  ([`../_discovery/distillation/personas-and-journeys.md`](../_discovery/distillation/personas-and-journeys.md),
  the `E0034` explanation that lived three documents from where the reader was standing).
- **`need-vocabulary-and-declaration-form` is the only foundation story, and it is
  consumed inside this initiative.** Its substrate is real — a `const` and a rule atom,
  no double and no `todo!()` — and both consumers are named above: the router's generated
  region and the checker's membership test. A need set written once in prose and once in
  Rust is the *"three lists that must agree"* defect `xtask/src/spec_trace.rs:122-160`
  records; landing the set before either consumer is what forecloses it.
- **`page-need-gate-step` is a separate slice because it is a separate surface with a
  separate reader** — S3, the terminal, against S2, the tree. It is not a "wire it in"
  story: it carries its own mount points (CR-1 through CR-4, `_decomposition.md`,
  Architecture brief, Note 1) and everything it needs to be reachable in one change.
  Splitting the `affected.rs` pair out of it would be the half-mount: adding the new
  tree's prefix to `INERT` without the unconditional-list entry makes a prose-only pull
  request read nothing.
- **`declaration-check-seen-to-fail` is its own story, not a checkbox on the checker.**
  It is the only AC in this project proven by a recorded procedure rather than a
  `#[test]`, because a unit test that calls the checker function never invokes
  `cargo xtask ci` ([`_decomposition.md`](_decomposition.md), Testing brief, AC-007).
  A gate step that has only ever been green is the failure `RUNBOOK.md:920-925` already
  cost this repository once.
- **`binding-beyond-this-project` is last because both its stories reach outside this
  project's own diff** — one into HS-P0020's pages tree, one into the ingest path
  HS-P0025 drains. Neither can be observed while the discipline is still moving.

### Deliberately not stories here

Each names the sibling that owns it, per the project charter's non-goals:

- Compiling code fences, the citation-**resolution** check, the pages tree's own pin and
  the hidden-branch demonstration — **HS-P0020** `checked-documentation-surface`.
  `page-need-checker-mounted-in-the-gate` *references* that project's `pub(crate)`
  page-tree const and its `clause_ids` function; it builds neither.
- Authoring narrative content. `governed-page-cites-the-discipline` adds a link and a
  reason to a page HS-P0020/HS-P0022 own; it writes no teaching.
- Ingesting the playbook atom, and re-observing DoD-8, DoD-12 and DoD-14 on the assembled
  tree — **HS-P0025** `durable-audience-closeout`.

## Coverage

Every project AC-### is covered by at least one story, and no two stories own the same
responsibility. Where an AC appears twice the two stories own different halves of it, and
the halves are named.

| Project AC | Story / stories | Note |
| --- | --- | --- |
| AC-001 — decided home, on disk, with a router | `router-precedence-and-announcement`; `page-need-checker-mounted-in-the-gate` | the tree and its router; the pin plus the two directory guards that stop it going vacuous |
| AC-002 — inside the precedence chain, not extending it | `router-precedence-and-announcement` | the rank stated in the router's own text; `git diff main -- standards/rust/README.md` empty |
| AC-003 — DT-2 resolved and recorded | `need-vocabulary-and-declaration-form`; `page-need-checker-mounted-in-the-gate` | the resolution and its rejected options; the generated region that keeps it single-sourced |
| AC-004 — DT-3 resolved, need set closed | `need-vocabulary-and-declaration-form`; `page-need-checker-mounted-in-the-gate` | findability's status decided; membership enforced against the `const` |
| AC-005 — DT-8 resolved as a stated rule | `fold-line-rule` | sole owner |
| AC-006 — every governed page declares exactly one need | `page-need-checker-mounted-in-the-gate` | pure parse tested over synthetic strings; the real-tree walk recorded at merge |
| AC-007 — the check has been seen to fail | `declaration-check-seen-to-fail` | sole owner; both failing halves and the recovery |
| AC-008 — a wrong page that could plausibly ship is rejected | `page-need-checker-mounted-in-the-gate` | three named wrong pages as `&str` literals in the module's own tests |
| AC-009 — the reviewer procedure is non-author-performable | `reviewer-and-citation-procedures` | sole owner |
| AC-010 — the citation rule exists and the spot check runs | `reviewer-and-citation-procedures` | sole owner; the mechanical resolution half is HS-P0020's |
| AC-011 — the discipline is cited by what it governs | `router-precedence-and-announcement`; `governed-page-cites-the-discipline` | announced in the repository's index; named in place by a page it governs |
| AC-012 — the playbook atom is staged, not hand-authored | `playbook-atom-staged-for-ingest` | sole owner |

**No AC is orphaned** (twelve of twelve traced) and **no story traces to nothing** (eight
of eight carry at least one AC). The four ACs with two owners are split by half, not
shared: in each case one story makes the thing *exist* and the other makes it *checked*
or *reached*.

The derived requirements ride along: DR-01/DR-02 on `router-precedence-and-announcement`,
DR-03/DR-04/DR-05 on `need-vocabulary-and-declaration-form`, DR-06 on
`page-need-checker-mounted-in-the-gate`, DR-07/DR-09 on `reviewer-and-citation-procedures`,
DR-08 on `fold-line-rule`, DR-10 on `governed-page-cites-the-discipline`, DR-11 on
`playbook-atom-staged-for-ingest`.

## Merge order

Foundation before its consumers; each slice whole before the next opens.

1. **`discipline-on-disk`**
   1. `need-vocabulary-and-declaration-form` — *foundation*. Nothing else in the project
      can be written until the set is closed and the declaration form is agreed with
      HS-P0020's hosting shape (the charter's top risk).
   2. `router-precedence-and-announcement` — the tree becomes navigable and announced.
   3. `fold-line-rule` and `reviewer-and-citation-procedures` — independent of each other;
      either order, both after the router exists to index them.
2. **`page-need-gate-step`**
   1. `page-need-checker-mounted-in-the-gate` — requires a non-empty rules tree, or its
      own vacuity guard fails it correctly on the first run.
   2. `declaration-check-seen-to-fail` — requires the step to be in `REQUIRED` for
      `cargo xtask ci` to be the thing observed failing.
3. **`binding-beyond-this-project`**
   1. `governed-page-cites-the-discipline` — the link target must be a path the checker
      already pins, so a tree move breaks the build rather than the link.
   2. `playbook-atom-staged-for-ingest` — last, because the atom records rejected
      alternatives and the conditions under which the discipline stops holding, and both
      are only true once the discipline has stopped moving.

The graph is acyclic: every `depends_on` edge above points strictly backwards in this
list, so the edge set is a subset of a total order's edges.
