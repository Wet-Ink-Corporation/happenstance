---
item: "HS-S0187"
stage: report
created: "2026-08-19T00:00:00.000Z"
updated: "2026-08-19T00:00:00.000Z"
---

# Report — The reader's invariant carried to an AppendCondition

## Findings Ledger

**Eight of eight criteria are satisfied.** The surface this story exists to land —
`docs/carry-your-invariant.md` — is authored, mounted in HS-P0020's harness, compiled *and
executed* by the gate, and reachable by reading order from the opening encounter's vocabulary.
Nothing is deferred; one named blocking condition (BC-001) is authored to and left open by
design, because closing it is the sign-off owner's call and not an implementer's.

| AC | Result | What proves it | Where it lives |
| --- | --- | --- | --- |
| AC-001 — the carry completes in place | **Met** | Tier 5: the six off-page clause links struck, the argument still completes; tier 2/3: the §5 fence compiles and runs, so what the reader copies is real | `docs/carry-your-invariant.md:1-156`; `implementation-report.md § Gates` |
| AC-002 — mounted, and both fences executed | **Met** | *Unregistered* observed red first (`does not include carry-your-invariant.md`), then `4 pages, all consistent`; `cargo test -p xtask --doc -- carry_your_invariant` → `2 passed`. Zero `IGNORE_ALLOWANCES` entries added | `xtask/src/narrative.rs:135-142` |
| AC-003 — the correct guard, tagged to the invariant | **Met** | `narrative::carry_your_invariant (line 60)` → ok. `Tags::from_pairs` → two `QueryItem`s, one per entity tag set → `Query::from_items` → `read_decision_model` → a visible fold → `AppendCondition::new(rule).after_opt(upto)`, all unhidden, all against a real `MemoryEventStore`, all imported from `happenstance` | `docs/carry-your-invariant.md:60-84` |
| AC-004 — the narrow guard accepts, and CF-8 says it must | **Met** | `narrative::carry_your_invariant (line 102)` → ok, asserting `accepted.is_ok()`. Red first with the correct tag in place (`1 free, and it still went in`). One expression differs. CF-8 cited for the acceptance, CF-7 named as the mirror, ES-27 for tags-not-types | `docs/carry-your-invariant.md:95-146` |
| AC-005 — the prior model named once, at a pinned slug | **Met** | `## Where your streams went` at the confirmed slug; the four prior-model phrases occur three times in the whole pinned tree and all three are inside that section | `docs/carry-your-invariant.md:17-28` |
| AC-006 — narration, fixed vocabulary, three-column table | **Met** | Four steps named *tag, query, fold, guard*, used identically wherever they appear; the table directly beneath, three columns in the fixed order; zero images | `docs/carry-your-invariant.md:30-51` |
| AC-007 — one answered-need, one vocabulary | **Met** | HS-P0021's own step: `every page declares one need — 4 pages, 16 rules, all consistent`. `explanation`, first block after the H1, 57 lines above the first fence. `happenstance_core` appears zero times | `docs/carry-your-invariant.md:3`, `:61-63`, `:103-105` |
| AC-008 — budgets held, nothing behind a control | **Met** | 156 lines, H1 27, fences 23/24 lines at 68 columns, longest paragraph 328, three-column table, zero hidden doctest lines, zero `HIDDEN_MARKERS`, zero affordances introduced | `implementation-report.md § Gates` (the measured table) |

**Mount point.** `xtask/src/narrative.rs:135-142` — one `#[cfg(doctest)] mod
carry_your_invariant` holding exactly one `include_str!("../../docs/carry-your-invariant.md")`,
which is HS-P0020's `HARNESS` and the only thing that turns a markdown file under the pinned
`TREE` into a compiled, executed page. The render path is the page itself. Both halves were
proven load-bearing: the checker was watched failing on the missing registration before it was
added.

**The one thing a reviewer should look at first.** The §6 assertion. It is the whole story:
`assert!(accepted.is_ok())` on a guard tagged to the row the command writes. It was written
against the *correct* guard first and watched fail, so it cannot be the kind of assertion that
would pass whatever the code did. If a future change to `Query`, `Tags` or `AppendCondition`
makes a narrow guard start refusing, this page turns red rather than turning stale — which is
the standing liability DT-6 knowingly bought (`spec.md:514`).

**Deferred, and why it is not a gap.**

- **BC-001 stays open.** §6 ships under `What a narrow guard misses` per the condition's own
  authoritative row. Confirming or replacing that title is the sign-off owner's decision;
  nothing links §6 today, so the call costs one string on one page whenever it is made.
  `_design.md` was not edited (EC-007).
- **Tier 4 and IQ-3 / IQ-5 are not this story's.** The remove → fail → revert drill is
  `boundary-falsification-drill`'s and the step header block is the opening encounter's. Both
  are recorded here as scope lines, per `spec.md:464-465`.
- **The bottom-of-page outward pointer is empty on purpose**, and the slice-mate
  `surface-course-subscriptions` fills it in the same slice.
- **Four project ACs are properties of the page *set*** — AC-007, AC-009, AC-011, AC-012 — and
  belong to `page-set-assurance`'s two stories. This page supplies its rows (two fences, five
  clause citations, one answered-need, one DT-1 anchor) and performs neither walk.

**Gate at the checkpoint.** `cargo xtask affected --base main` → `affected gate passed`,
`233 passed; 0 failed; 2 ignored`, against a `231 passed` baseline measured on the unmodified
tree. The two added tests are this page's two fences.
