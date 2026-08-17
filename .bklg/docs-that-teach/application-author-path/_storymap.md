---
item: HS-P0022
stage: storymap
created: 2026-08-17T03:21:38.635Z
updated: 2026-08-17T03:21:38.635Z
template_sig: 1c63534a
rendered_sig: c762c5f5
---

# Story Map — The Application Author's Path

Eight stories in four slices, covering AC-001…AC-014 of
[`project.md`](project.md). The reader whose path this map traces is Persona 1, the
application author
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:56-144`), and
the backbone below is that reader's own sequence, not the repository's.

Two things fix the shape of this map before any preference does. First, **AC-014 is a
hard preflight**: `crates/happenstance/src/lib.rs` has been *replaced*, not merely
diverged from, on `initiative/from-contract-to-published-library`, so a page authored
against this worktree's copy is a page to be redone at merge
([`_grounding.md`](_grounding.md), §1). Second, **AC-001 sequences DT-1's sign-off
before any page is authored**, and DT-1 is upstream of DT-4
([`../_decomposition.md`](../_decomposition.md), "Why this cut"). Both are therefore
`foundation` stories in one preflight slice, and every capability slice consumes them.

The substrate this project *does not build* — HS-P0020's pinned narrative tree,
compiled-fence step and enumerated allowance list, and HS-P0021's answered-need
notation — is owned by sibling projects **inside this initiative**, so no story here
depends on substrate owned outside it. Where a story needs one of those primitives it
consumes it; it never re-implements it
([`_decomposition.md`](_decomposition.md), UX brief, "Primitives to compose").

## Backbone

The reader's activities, left to right. Each is an outcome the reader reaches, not a
document that exists.

| # | Activity (reader's words) | Outcome | Slice that delivers it |
| --- | --- | --- | --- |
| A1 | "Give me a baseline I am not about to redo." | The tree the pages are written against is the merged one, and the four tensions are settled once, in a place a page can cite | `preflight-and-anchor` |
| A2 | "Show me the thing this library is for, working." | A program the reader ran refuses an append because a boundary held, and the refusal is in the program's own output (UI-1) | `opening-encounter` |
| A3 | "Tell me it is real and not a story about itself." | The reader removes the boundary, watches a repository check fail, reverts, watches it pass (UI-2) | `opening-encounter` |
| A4 | "Take the rule I already have and show me how to say it here." | A cross-entity invariant in ordinary event-sourcing vocabulary reaches a `Query`, a fold and an `AppendCondition` with no off-page step (UI-3, UI-4) | `conceptual-bridge` |
| A5 | "I found the good example. Let me actually read it." | `examples/course-subscriptions/src/main.rs:1-19` reaches the reader in its own words, with the vocabulary seam named at the link (UI-5) | `conceptual-bridge` |
| A6 | "I want to trust the whole set, not one page." | Every fence is exercised or named; every clause is cited, never restated; every page carries exactly one answered-need and one anchor | `page-set-assurance` |

## Slices

Four slices. Cross-slice `depends_on` edges run strictly left to right in the merge
order below and are acyclic.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --- | --- | --- | --- | --- | --- |
| `preflight-and-anchor` | `merge-forward-preflight` | foundation | Merge `initiative/from-contract-to-published-library` forward and record the merge, so every page in this project is authored against the merged `crates/happenstance/src/lib.rs` rather than this worktree's 75-line copy | — | AC-014 |
| `preflight-and-anchor` | `tension-resolutions` | foundation | Author `_design.md` with four resolutions — DT-1 (which prior model the teaching argues against), DT-4 (disclosure shape and its named failure mode), and DT-5+DT-6 as one joint resolution including the exact narrow basis of any fence exemption — as the single citable home every later page points at | `merge-forward-preflight` | AC-001, AC-002, AC-003 |
| `opening-encounter` | `boundary-refusal-encounter` | capability | Author the opening encounter into HS-P0020's pinned tree in DT-4's resolved shape, so a reader following it from step one runs a program whose own output carries `AppendError::ConditionViolated` (ES-25, `spec/SPECIFICATION.md:3693`) against a real `MemoryEventStore`, with every step anchor answerable cold and no hidden `#`-prefixed line carrying any part of the boundary | `merge-forward-preflight`, `tension-resolutions` | AC-002, AC-004, AC-006 |
| `opening-encounter` | `boundary-falsification-drill` | capability | Make the boundary load-bearing to the repository — an executed check the `"tests"` REQUIRED step already sweeps (`xtask/src/main.rs:143-155`) that fails when the query the `AppendCondition` is built from is removed — and give the reader the exact edit, the exact failure and the exact revert on the page, then run and record the drill in both directions | `boundary-refusal-encounter` | AC-005 |
| `conceptual-bridge` | `invariant-to-appendcondition-bridge` | capability | Author the bridge material: a cross-entity invariant stated in ordinary event-sourcing vocabulary carried to `Query`, `QueryItem`, `Tags`, a fold and an `AppendCondition` with no step requiring `spec/SPECIFICATION.md` or the crate source, carrying DT-5/DT-6's resolved shift material — a mapping-table narration of the query/append-condition cycle, and the "wrong model" contrast if one ships | `tension-resolutions`, `boundary-refusal-encounter` | AC-008, AC-013 |
| `conceptual-bridge` | `surface-course-subscriptions` | capability | Bring the worked example within the reader's reach: surface `examples/course-subscriptions/src/main.rs:1-19` verbatim by construction, one module per included file (`xtask/src/constitution.rs:11-18`, `:39-50`), with orientation prose that tells the reader what they are about to read and names the `happenstance` / `happenstance_core` vocabulary seam in one sentence at the link | `invariant-to-appendcondition-bridge` | AC-010 |
| `page-set-assurance` | `fence-inventory-and-clause-audit` | capability | Produce the inventory of every fenced block this project authored showing zero opted out — or each exception named against `_design.md`'s stated exemption and present on HS-P0020's enumerated allowance list — and audit every normative claim as a clause citation that `cargo xtask spec-trace` resolves and that no page restates in its own words | `boundary-refusal-encounter`, `boundary-falsification-drill`, `invariant-to-appendcondition-bridge`, `surface-course-subscriptions` | AC-007, AC-011 |
| `page-set-assurance` | `answered-need-and-anchor-review` | capability | Walk the whole page set with a reviewer applying HS-P0021's own check — exactly one named answered-need per page, above the first fence — and confirm the DT-1 anchor decision is applied identically everywhere with each relying page citing the one recorded location, recording the result as DoD item 7 | `tension-resolutions`, `boundary-refusal-encounter`, `boundary-falsification-drill`, `invariant-to-appendcondition-bridge`, `surface-course-subscriptions` | AC-009, AC-012 |

### Why the slices are cut here

- **`preflight-and-anchor` is one slice, not two**, because both its stories are
  gates on the same thing: authoring. Splitting them lets a page be written against
  a merged tree with an unsettled anchor, or against a settled anchor on a stale
  tree, and each is a documented way to redo the work.
- **`opening-encounter` holds the page and the check that makes it honest.**
  Separating "author the encounter" from "make the boundary load-bearing" would ship a
  page that compiles forever while quietly ceasing to demonstrate its own claim —
  the risks table's first row, and the named blind spot of every tool the initiative
  surveyed. The check and the page it belongs to are mounted together.
- **`conceptual-bridge` holds the bridge and the handoff**, because the handoff *is*
  the bridge's last step: the UX brief's state table runs "Mid-bridge" straight into
  "Handed off", and the vocabulary-seam sentence lives at the link, not on a page of
  its own.
- **`page-set-assurance` is deliberately last and deliberately set-wide.** AC-007,
  AC-009, AC-011 and AC-012 are properties of a *set* of pages; assigning them to any
  single page story would leave them unprovable until the set existed anyway. Its two
  stories split on mechanism, not on subject: one is what `spec-trace` and the
  allowance list can decide, the other is what only a reviewer can
  (testing brief, tiers 1 and 5).

### Archetype notes

Only two stories are `foundation`, and both are consumed and demonstrated by
capability slices in this same project — neither is a terminal island:

- `merge-forward-preflight` lands real in-tree substrate (the merged crate root) that
  `boundary-refusal-encounter` is authored against and `surface-course-subscriptions`
  imports from. Nothing about it is a double or a fixme.
- `tension-resolutions` lands `_design.md` as the single citable home the anchor
  decision needs (UX-009, DR-07). Every page story consumes it, and
  `answered-need-and-anchor-review` demonstrates that consumption by checking every
  page cites it. `design.capture` is absent from `.redkiln/config.yaml`, so this
  written record is the only record these four choices will ever have.

## Coverage

Every project AC-### is covered by at least one story, and each has a single owning
story. Two ACs are additionally *realized* by a downstream story; the owner column is
where the responsibility sits.

| AC | Owning story | Also realized by | How it is proven (testing brief tier) |
| --- | --- | --- | --- |
| AC-001 | `tension-resolutions` | — | 5 — `_design.md` sign-off, DoD item 1 |
| AC-002 | `tension-resolutions` | `boundary-refusal-encounter` (the staged shape and the mid-sequence entry point, UX-005/IQ-3) | 5 |
| AC-003 | `tension-resolutions` | — | 5 |
| AC-004 | `boundary-refusal-encounter` | — | 2, 3 — compiled fence plus executed test; DoD item 2 |
| AC-005 | `boundary-falsification-drill` | — | 3, 4 — executed test, observed in both directions; DoD item 3 |
| AC-006 | `boundary-refusal-encounter` | — | 3, 5 |
| AC-007 | `fence-inventory-and-clause-audit` | — | 1, 2, 5 |
| AC-008 | `invariant-to-appendcondition-bridge` | — | 2, 5 — IQ-1's strike-every-off-page-link falsification |
| AC-009 | `answered-need-and-anchor-review` | — | 5 |
| AC-010 | `surface-course-subscriptions` | — | 1 — `include_str!`'s identical-bytes guarantee |
| AC-011 | `fence-inventory-and-clause-audit` | — | 1, 5 — `cargo xtask spec-trace` plus a spot check |
| AC-012 | `answered-need-and-anchor-review` | — | 5 — HS-P0021's own check, DoD item 7 |
| AC-013 | `invariant-to-appendcondition-bridge` | — | 5 — conditional on DT-5's resolution |
| AC-014 | `merge-forward-preflight` | — | tier 0 preflight — a one-time gate before any authoring story starts |

No AC is orphaned (14 of 14 covered) and no two stories own the same AC. AC-013 is
conditional on DT-5 resolving to ship a diagram; it is still owned rather than dropped,
because the resolution that discharges it vacuously is itself an AC-003 obligation and
the reviewer must record which way it went.

Three cross-cutting obligations are *not* separate stories, by design: every page story
carries them and `page-set-assurance` proves them across the set — one answered-need in
HS-P0021's notation above the first fence (AC-012), the DT-1 anchor cited rather than
re-argued (AC-009), and `use happenstance::{…}` as the taught vocabulary per ADR-0006
(UX-011, IQ-9). Making each of those its own story would horizontally layer the map.

## Merge order

Slice by slice, foundation before the capability slices that consume it.

1. **`preflight-and-anchor`** — `merge-forward-preflight`, then `tension-resolutions`.
   Nothing else may start: AC-014 gates authoring on the merge, AC-001 gates authoring
   on DT-1's sign-off, and DT-1 is upstream of DT-4.
2. **`opening-encounter`** — `boundary-refusal-encounter`, then
   `boundary-falsification-drill`. Both mount together; the drill is not a follow-up
   PR that may slip, because the encounter without it is exactly the failure this
   initiative exists to prevent.
3. **`conceptual-bridge`** — `invariant-to-appendcondition-bridge`, then
   `surface-course-subscriptions`. Follows the encounter because the bridge reuses the
   vocabulary the encounter taught and the reader path runs encounter → bridge →
   worked example.
4. **`page-set-assurance`** — `fence-inventory-and-clause-audit` and
   `answered-need-and-anchor-review` (independent of each other, one slice). Last,
   because both are properties of the assembled page set.

**Standing checks per story**, from the testing brief's merge-gate list — not new
obligations, the existing ones each story rides: `cargo xtask affected --base main` at
each story's checkpoint, and `cargo xtask ci --fast` as the project-level bar (this
project is `terminal: false`; HS-P0025 owns the whole-initiative re-observation).

**Routing, so nothing is absorbed silently** (project DoD item 9): substrate gaps —
including HS-P0020's compiled-fence step turning out to type-check without *executing*
doctests, which would move AC-005's proof into this project's own `#[tokio::test]` —
go to HS-P0020; pointer and reach gaps to HS-P0023; doubts about whether the bridge
lands to HS-P0024; incidental bugs to the `support` initiative per
`.redkiln/config.yaml`. AC-005 itself routes nowhere under any circumstance.
