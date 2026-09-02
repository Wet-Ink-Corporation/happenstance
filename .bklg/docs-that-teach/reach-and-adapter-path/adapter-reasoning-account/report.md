---
item: "HS-S0155"
stage: report
created: "2026-08-17T13:16:13.497Z"
updated: "2026-08-17T13:16:13.497Z"
---

# Report — The sequenced adapter reasoning account

## Findings Ledger

Ten acceptance criteria, all satisfied by a real, reachable page in the repository's pinned narrative
tree, each with a `file:line` citation and a mechanical or recorded check. Nothing deferred, nothing
blocked, no test weakened, no `#[allow]` added, no gate step or checker introduced.

**The mount, first, because it is the one thing that could have blocked the story.** `EC-001` required
this story to block if HS-P0020's pinned tree had not landed. It has. The tree is `docs/`
(`xtask/src/lint_narrative.rs:239`), the no-orphan mechanism is the bidirectional `check_registration`
(`:536-563`) reading `xtask/src/narrative.rs`, and both run under mandatory steps. The page is
registered at `xtask/src/narrative.rs:160-161`. Design finding **F5** is **closed for this surface**;
`route` is `docs/adapter-reading-order.md`.

| AC | Result | Proof | Mount / where it lives |
| -- | ------ | ----- | ---------------------- |
| AC-001 | **satisfied** | RED `does not include adapter-reading-order.md` + `no mod adapter_reading_order` -> GREEN `6 pages, all consistent` | `xtask/src/narrative.rs:160-161`; also indexed at `docs/README.md:21` |
| AC-002 | **satisfied** | Five regions in the bound order; measured first screen = **15** rendered lines against <= 15 of ~27, at both a 96- and a 105-column box; six list markers, all in region 2 | `docs/adapter-reading-order.md:1`, `:3`, `:7-19`, `:28-35`, `:21/:37/:44/:50/:58/:65`, `:71-77` |
| AC-003 | **satisfied** | Six sources in the decided order in both bands; six anchored subject strings, each `rg`-resolving in its cited file; **zero** fenced blocks, so nothing is copied | `:7-19` and `:24`, `:39`, `:46`, `:52`, `:61`, `:67` |
| AC-004 | **satisfied** | Seven `##`; six read `N of 6 — ...`, numbering agreeing with the reading order entry-for-entry; the seventh names no source | `:21`, `:37`, `:44`, `:50`, `:58`, `:65` |
| AC-005 | **satisfied** | Caveat opens at `:28`, before section 2's heading at `:37`; defers to `memory.rs`'s own three-reasons list; pairs `PgStore` and `references/adapter-shapes.md` in the same paragraph | `docs/adapter-reading-order.md:28-35`; `crates/happenstance-core/src/memory.rs:16-25` |
| AC-006 | **satisfied** | `VT-11` and `CF-15` both resolve under `check_citations` (`xtask/src/lint_narrative.rs:1197`); ADR-0001 and ADR-0008 reached through `.kb/maps/decision-map.md:68`, `:75`, both `accepted`, neither superseded; the page says in its own voice that it states neither the rule nor the decision | `:32`, `:42`, `:54`, `:55-56`, `:69` |
| AC-007 | **satisfied**, with a recorded deviation in vocabulary | RED `no > **Answers:** line` -> GREEN `6 pages, 16 rules, all consistent`; one declaration, immediately after the H1, 96 characters | `docs/adapter-reading-order.md:3` |
| AC-008 | **satisfied** | `wc -w` = **704** against <= 900 / 1,200 cap / ~350 floor; no source's connective tissue over ~120 words (43, 69, 54, 59, 67, 52) | whole page |
| AC-009 | **satisfied** | Exactly **one** markdown link on the page, in the terminal region; limits stated first; no "See also"/"Next steps"/"Further reading"; no `https://`; link text six words | `:73-74`, `:77` |
| AC-010 | **satisfied** | (a) no raw HTML, no `style=`, no `<details>`/tab/fold, no fence, unskipped ladder, all six sources visible; (b) `rg 'adapter-reading-order' crates/` -> nothing, and `store.rs:31-45` still resolves `error[E0034]` in place | whole page; `crates/happenstance-core/src/store.rs:31-45` |

**The one deviation a reviewer must look at.** The declaration's token is `explanation`, not the
routing-shaped need `_design.md` worded. HS-P0021's rule landed after sign-off and closed the need set;
`orientation` is barred twice over — `RP-10-3` (one per directory level, already held by
`docs/read-the-worked-example.md:3`, enforced at `xtask/src/lint_pages.rs:685`) and `RP-10-2` (links plus
one sentence per destination, teaches nothing — which this page's own signed-off density budget and its
required caveat exceed by design). The **need sentence is unchanged**; only the vocabulary moved.
`spec.md` `## Clarifications` said HS-P0021's rule was to be *obeyed, not authored*, and this is what
obeying it produced. Argued in full at `_verification.md`, section AC-007.

**Interaction-quality invariants, and where each is discharged.** In-place before context-jump — the
B1-independence check ran and passed (AC-010(b)), and will be re-run with the pointer installed by the
slice-mate. Non-occlusion — 15 of ~27 rendered lines, measured (AC-002). Preserved position on
mid-sequence entry — six positioned headings (AC-004). Reversibility — the terminal region states what
the page assumes and does not cover (AC-009). Keyboard reachability — one plain link and no bespoke
control (AC-010(a)). Selection and copy survive — the `memory.rs` doctest is cited, not pasted, so
there is no second copy to diff (AC-003).

**Composition against the signed-off design.** Region order, transience (zero revealed or
opened-on-demand content), density (both numbers measured, not asserted), hierarchy (the list is the
page's only enumerated element and precedes everything) and the five named anti-patterns — 3, 5, 9, 10,
17 — all check out against `_design.md` and against the `adapter-reasoning-account` frame group in
`design/mock.html`. Perceptual capture stays a standing skip: `design.capture` is absent from
`.redkiln/config.yaml` and there is no renderer in this repository for a `docs/` page.

**Nothing deferred, and one thing deliberately not done.** The single onward hop files **no**
pointer-register row, because the register counts rule-2 pointers at an item on the reference surface,
not ordinary in-tree links between two pages of one tree. NF-005 permits at most one row; this story
takes zero and says why rather than leaving the absence to be inferred.

**Handed forward.** The slice-mate `store-error-site-rewrite` installs the inbound pointer and must name
this page at `docs/adapter-reading-order.md`; its register row **P3** carries the guard this page's
registration provides. `error-site-walk-record` is the observed walk, deliberately a different pair of
hands, and remains the only instrument that can say whether the sequence actually carries an adapter
author (NF-007).
