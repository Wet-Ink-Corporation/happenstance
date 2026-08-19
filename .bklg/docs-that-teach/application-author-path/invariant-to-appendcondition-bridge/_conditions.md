---
item: "HS-S0187"
stage: plan
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Blocking conditions — the bridge page

Named conditions this story must meet or close before it completes. Each carries what is in
conflict, which side is authoritative *now* so authoring is never stalled waiting for a call,
and what closing it requires. A condition is not a note: `spec.md` is the source of truth for
everything else, and where this file and `spec.md` disagree about a **string**, this file wins
until the condition is closed — that is the whole point of it existing.

Authored by HS-S0184 (`tension-resolutions`) while certifying DT-5+DT-6, per project DoD item
9's routing rule (`project.md:300-302`): a thing found and not fixed is recorded against the
story that will meet it, never absorbed.

## BC-001 — §6's heading text: `narrow` or `type-only`

**Status: open. Blocks this story's completion, not its start.**

**The conflict, in the two live strings.**

| where | string it fixes | date |
| --- | --- | --- |
| `_design.md:487-488` (`## Composition` §6) | `## What a type-only guard misses` | signed off 2026-08-17 |
| `spec.md:429` (this story's AC-004, quoting the design) | `## What a type-only guard misses` | 2026-08-17 |
| `tension-resolutions/_resolutions.md` § Anchor table, `conceptual-bridge` §6 row | `What a narrow guard misses` (26 chars, `#what-a-narrow-guard-misses`) | 2026-08-18 |

**Authoritative pending the sign-off owner's call: `What a narrow guard misses`, emitting
`#what-a-narrow-guard-misses`.** Author §6 under that heading. Do not author
`## What a type-only guard misses`, and do not "split the difference" with a third string — a
second live string is exactly the defect this condition exists to end.

**Why that side wins, stated so the call is a confirmation rather than a fresh decision.**
Three recorded statements say the wrong side is a guard **tagged too narrowly**, which
*under*-refuses: `_design.md:206-232` (`## Pattern decision` DT-6 as re-decided), `:1017`
(finding F-6's disposition) and `:1039` (the approver's own row). Two residual strings say
*type-only*, which is the **opposite**, over-refusing failure — the mirror of CF-7
(`spec/SPECIFICATION.md:7670`, `[FROZEN]`) — and `## Composition` was simply never updated when
F-6 landed. This story's own `spec.md` reaches the same conclusion in its executive summary
(item 1) and in AC-004's *body*, which already describes the too-narrow guard and its `is_ok()`
assertion; only AC-004's quoted **title** is stale. The fence that ships under the heading is
certified in `tension-resolutions/_resolutions.md § DT-5+DT-6` — compiled, executed,
`assert!(accepted.is_ok())` passing, 23 rendered lines / 68 columns — and it is a *narrow* guard.
A heading naming the opposite failure would mislabel the one fence on the page whose entire job
is to be labelled correctly.

**This is a semantic rename, not a budget-mandated retitle, and the distinction matters.** The
22-character `##` budget derives from rustdoc's 200px sidebar TOC (`_design.md:575-584`) and
therefore binds `crate-root-encounter` alone; `conceptual-bridge` renders as markdown under
HS-P0020's "the markdown is the render" decision and has no such sidebar at any width
(`_resolutions.md § Anchor table`, sign-off condition 2 at `_design.md:1039`). Both candidate
strings are over 22 characters and neither is affected by the budget. So the old title is wrong
about *what the section teaches*, not too long — do not read this condition as a density fix,
and do not "resolve" it by finding a shorter third title.

**What closing it requires**, in order:

1. The sign-off owner confirms the title, or names a different one. `_design.md` is signed off
   and **is not edited here** (EC-007, `_design.md:1041-1048`): the correction lands as an
   amendment recorded by whoever owns the sign-off, not as an in-place edit by an implementer.
2. The confirmed string and its emitted fragment id are written into the row below, with the
   date and who confirmed.
3. Any page that links §6 uses the confirmed slug. Today nothing links it — §6 is reached by
   reading order from §5 — so the blast radius of the call is this page only, which is why the
   condition blocks completion rather than authoring.
4. `answered-need-and-anchor-review` (HS-S0190) checks the rendered heading against
   `_resolutions.md § Anchor table`; that row and this one must agree when it runs.

| decision | confirmed by | date | final string | emitted id |
| --- | --- | --- | --- | --- |
| *(open)* | — | — | — | — |

**If the call goes the other way** — the sign-off owner keeps `type-only` — then DT-6 itself is
back open, because the fence certified against it asserts `is_ok()` on a *narrow* guard and a
type-only guard is the over-refusing mirror that would make that assertion fail. That is EC-001
on `tension-resolutions`, not a retitle: raise it as a reopen condition on the design rather
than changing the fence to match the heading.
