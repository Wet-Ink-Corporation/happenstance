---
item: "HS-S0187"
stage: implement
created: "2026-08-19T00:00:00.000Z"
updated: "2026-08-19T00:00:00.000Z"
---

# Implementation Report — The reader's invariant carried to an AppendCondition

## TDD Evidence

Two red steps, taken in the order the spec's own implementation notes prescribe
(`spec.md:528-536`), and they went red for two different reasons on purpose. The first is the
substrate refusing a page it does not compile. The second is **the page's own assertion
failing against the correct guard** — which is the only cheap proof available that §6's
`is_ok()` is a criterion rather than a decoration.

**RED 1 — the mount, observed as a failure before it was made.** The page was written into
`docs/` and left unregistered. `cargo run -q -p xtask -- lints`:

```
=== every narrative page is checked ===
  xtask/src/narrative.rs — does not include carry-your-invariant.md; its examples are never compiled
  xtask/src/narrative.rs — no `mod carry_your_invariant`; one module per page is what keeps a doctest failure's line number relative to the page

xtask failed: 2 problem(s) in docs
```

That is HS-P0020's *unregistered* state, by name, from the substrate this story mounts into —
not a check this story wrote for itself. A page in the tree that the harness does not name
fails the gate, so "constructed but not mounted" is not a state that can survive here.

**RED 2 — the wrong-side fence, asserted against the *correct* guard.** With the registration
in place and §6 still carrying `held.clone()` — the invariant's tag, not the narrow one —
`cargo test -p xtask --doc -- carry_your_invariant`:

```
running 2 tests
test xtask\src\../../docs/carry-your-invariant.md - narrative::carry_your_invariant (line 58) ... ok
test xtask\src\../../docs/carry-your-invariant.md - narrative::carry_your_invariant (line 99) ... FAILED

thread 'main' panicked at ...doctest_bundle_2024.rs:64:5:
1 free, and it still went in
```

Three things fall out of that one transcript, and only one of them was forecast:

1. **The §6 assertion is load-bearing.** Narrow the tag back and it goes red. It is not a fence
   that passes whatever the code does.
2. **The §5 guard really refuses.** The correct fence passed in the same run, so the pair is a
   pair: one expression apart, `ConditionViolated` becomes `Ok`.
3. **EC-002 did not fire.** The narrative step **executes** doctests — a type-check would have
   reported both fences green. The fallback `xtask/tests/bridge_guard.rs` named at
   `spec.md:498` was therefore not needed and was not written, and the front half's PR boundary
   stands unamended.

**GREEN.** `whose.clone()` at `docs/carry-your-invariant.md:113` is the one expression that
changes, and both fences pass:

```
test xtask\src\../../docs/carry-your-invariant.md - narrative::carry_your_invariant (line 60) ... ok
test xtask\src\../../docs/carry-your-invariant.md - narrative::carry_your_invariant (line 102) ... ok
test result: ok. 2 passed; 0 failed
```

| AC | Test / instrument | Red → Green |
| -- | ----------------- | ----------- |
| AC-001 | The §5 doctest (tier 2 + 3) for the mechanical half; the tier-5 IQ-1 falsification for the carry | **Red** — the page did not exist, so nothing compiled and there was no carry to walk. **Green** — the five off-page clause links (`:40`, `:88`, `:92`, `:137`, `:141`, `:146`) were struck out and the argument still completes end to end; no step needs `spec/SPECIFICATION.md` or a file under `crates/` |
| AC-002 | HS-P0020's narrative REQUIRED step; `cargo test -p xtask --doc` | **Red** — the two *unregistered* problems above. **Green** — `4 pages, all consistent`, one module holding one `include_str!` at `xtask/src/narrative.rs:135-142`, both fences plain `rust`, both executed, `IGNORE_ALLOWANCES` still `&[]` |
| AC-003 | `narrative::carry_your_invariant (line 60)` | **Red** — no fence. **Green** — the whole chain renders unhidden at `:67-79` and the program runs; it is the first code on the page |
| AC-004 | `narrative::carry_your_invariant (line 102)` | **Red, deliberately** — the assertion written first against the correct guard, failing with `1 free, and it still went in`. **Green** — one expression narrowed, `assert!(accepted.is_ok())` passes |
| AC-005 | `git grep` over the pinned tree; the tier-5 anchor walk | **Red** — the anchor did not exist, so every page that will link it had nowhere to point. **Green** — `## Where your streams went` at `:17`, and the four prior-model phrases occur only at `:19-22` across all of `docs/` |
| AC-006 | Tier-5 check against `_design.md:174-204`, `:481-485`, `:592-595` | **Red** — no narration, no table. **Green** — four steps named *tag, query, fold, guard* at `:34,:36,:41,:44`, the three-column table directly beneath at `:46-51`, zero images |
| AC-007 | HS-P0021's own declaration step (`xtask/src/lint_pages.rs:630`) | **Red** — `no > **Answers:** line` would have fired; the page had none until `:3` was written. **Green** — `every page declares one need — 4 pages, 16 rules, all consistent` |
| AC-008 | The `HIDDEN_MARKERS` and info-string checks (mechanical); the measured budget walk (tier 5) | **Red** — nothing to measure. **Green** — the numbers in `## Gates` below, all inside budget, and the checker green on markers and info strings |

## Commits

| Commit | What it carries |
| --- | --- |
| `a5f9b03` | `feat(application-author-path): Invariant to AppendCondition bridge` — the page, its registration, this report, `report.md`, and the eight flipped ledger rows |

## Changes

| File | Shape of the change |
| --- | --- |
| `docs/carry-your-invariant.md` | **New**, 156 source lines. H1, the `explanation` answered-need line, and the seven sections in `_design.md:503-521`'s fixed order. Two `rust` fences — the correct guard (`:60-84`, 23 rendered lines) and the narrow guard (`:102-127`, 24 rendered lines) — differing by one expression. Five clause citations, all resolving. No affordance, no image, no hidden doctest line |
| `xtask/src/narrative.rs` | **+8 lines.** One `#[cfg(doctest)] mod carry_your_invariant` holding exactly one `include_str!`, with a comment saying what the module's second fence asserts and what narrowing the tag back would do. No other line of the harness moved; `TREE`, `HARNESS`, `IGNORE_ALLOWANCES` and `HIDDEN_MARKERS` are untouched |
| `.bklg/.../invariant-to-appendcondition-bridge/_ledger.md` | Eight rows flipped `false → true`, each with a `file:line` and the passing test id. No criterion re-worded |
| `.bklg/.../invariant-to-appendcondition-bridge/implementation-report.md`, `report.md` | **New.** This file and the review-gate findings ledger |

Nothing outside the PR boundary at `spec.md:303-307` was touched. In particular:
`crates/happenstance/src/lib.rs`, `examples/course-subscriptions/`, `docs/README.md`,
`_design.md`, `spec/SPECIFICATION.md` and every `.kb/` atom are unchanged.

## Gates

**Story checkpoint — `cargo xtask affected --base main`: PASSED.** `233 passed; 0 failed;
2 ignored` against a `231 passed` baseline taken on the unmodified tree before a line was
written, so the delta is exactly this story's two doctests. The run includes fmt, clippy
`-D warnings` for the affected packages, the five file-reading lints and `spec-trace`.

**Tier 1 — structural.** `cargo run -q -p xtask -- spec-trace`: no problems. Every clause id
this page cites resolves; HS-P0020's own citation check (their AC-007) is green, which is what
turns an invented id into a build failure rather than a reviewer's note.

**Tier 2 and 3 — compiled and executed.** `every narrative page is checked — 4 pages, all
consistent`, and `cargo test -p xtask --doc -- carry_your_invariant` → `2 passed; 0 failed`.
Tier 3 is the one that matters here and it is proven twice: once by the pass, and once by
RED 2's failure, which is what a `no_run` fence could never have produced.

**Tier 5 — the reviewer walk, measured rather than asserted.**

| What | Budget | Measured | Source |
| --- | --- | --- | --- |
| Page path | ≤ 32 characters | **28** (`docs/carry-your-invariant.md`) | `xtask/src/lint_narrative.rs:266`, enforced |
| Page length | ≤ 250 source lines | **156** | |
| H1 | ≤ 40 characters | **27**, and exactly one `h1` | `:1` |
| `##` headings | 6, no skipped level | 24, 23, 23, 25, 26, 27 characters | `:5,:17,:30,:53,:95,:148` |
| Fence width | ≤ 68 columns | **68** widest line, both fences | `:67` and `:114` are the binding lines |
| Fence height | ≤ 24 rendered lines | **23** (§5) and **24** (§6) | |
| Prose source wrap | ≤ 90 columns | every non-table line ≤ 90 except `:40` and `:92` | both are single-token clause URLs, which the budget exempts by name (`checked-documentation-surface/_design.md:394`) |
| Longest paragraph | ≤ 435 characters | **328** | |
| Mapping table | exactly 3 columns | 3 | `:46-51` |
| Hidden doctest lines | 0 for any boundary expression | **0** anywhere in either fence | |
| `HIDDEN_MARKERS` tokens | 0 | **0** | `xtask/src/lint_narrative.rs:335-343`, mechanical |
| Interactive affordances introduced | none | **none** | no `details`, `summary`, tab, accordion, banner, badge, button, CSS or JS |

**The DT-1 word walk**, run over the whole pinned tree rather than over this page:

```
$ git grep -n -i -E "aggregate|one stream per entity|which stream" -- docs/
docs/carry-your-invariant.md:19:Your reflex question is probably *which stream does this go in?* — the model
docs/carry-your-invariant.md:20:you arrived with puts one stream per entity, so a rule spanning two of them
docs/carry-your-invariant.md:22:nothing asks you to name an aggregate, and your aggregates are not the unit
```

Three hits, all inside `## Where your streams went` (`:17-28`), none anywhere else in the
tree. That is AC-005's exclusivity claim as a measurement.

**The IQ-1 falsification.** Every off-page link on the page is a clause citation in final
position in its sentence — ES-27 (`:40`), ES-25 (`:88`), VT-30 (`:92`), CF-8 (`:137`), CF-7
(`:141`), ES-27 again (`:146`). Striking all six leaves six sentences that still say what they
say; the only remaining link is `#the-guard-you-would-write` at `:154`, which is on-page. The
"Where you saw it" column names the first encounter's steps in plain text rather than linking
them, so there is nothing to strike there either. The carry completes.

## Notes

**BC-001 is authored to, and is not closed by this story.** §6 ships as
`## What a narrow guard misses`, emitting `#what-a-narrow-guard-misses`. That is the string
`_conditions.md:32-35` names as authoritative pending the sign-off owner's call, and
`_conditions.md:12-14` says that where it and `spec.md` disagree about a *string*, it wins
until the condition is closed. AC-004's own body, this story's executive summary, `_design.md`'s
`## Pattern decision`, finding F-6's disposition and the approver's row all describe a guard
tagged **too narrowly**; only AC-004's quoted *title* and `_design.md:487` still say
*type-only*, which names the opposite, over-refusing failure. **The condition's decision row is
left open**: confirming the title is the sign-off owner's, not an implementer's, and step 3 of
its closing procedure is satisfied in advance — nothing links §6 today, so the blast radius is
this page alone.

**EC-007 was met and routed, not patched.** `_design.md:373-384` and `:487` were read, found to
carry the stale *type-only* wording, and left exactly as they are. A signed-off design is
amended by decision, not in passing (`project.md:300-302`). This paragraph is the record; the
route is BC-001.

**EC-008 was met without spending a compile cycle.** `Tags::from_iter([("course", "c1")])` was
never typed. `Tags` implements `FromIterator<Tag>` (`crates/happenstance-core/src/tag.rs:479`),
so the fallible `Tags::from_pairs([...])?` (`:304`) is what both fences use, which is also what
the worked example already uses.

**AC-013's disposition, recorded rather than left to silence.** DT-5 resolved to **narration**.
No diagram, chart or image ships from this story; the shift is carried by the four-step
narration at `:32-44` and the three-column mapping table at `:46-51`, with the vocabulary
fixed at *tag, query, fold, guard* so it cannot drift across the pages that come after this
one. The reviewer record for project AC-013 is: **discharged by narration, no diagram
shipped** (`_storymap.md:120`, `_design.md:174-204`).

**Two design residuals were consumed rather than re-derived.** The certified DT-6 fence in
`tension-resolutions/_resolutions.md § DT-5+DT-6` is a single-entity program, and AC-003 asks
for one `QueryItem` per entity's tag set plus a visible fold. The shipped §5 fence keeps the
certified program's *structure* — same use block shape, same event, same unconditional append,
same condition, same guarded append — and adds a second query item and the fold. The one-delta
property is preserved by construction: §6 was derived from §5 by editing one expression, and
the two were diffed before committing. The diff is 5 lines, of which exactly **one** decides
program behaviour (`held.clone()` → `whose.clone()`); the other four are the `AppendError`
import the wrong side no longer needs, the two `WRONG` markers, and the assertion stating the
opposite outcome — which is the same accounting `_resolutions.md:259-269` gives.

**The outward-pointer slot at the bottom of the page is deliberately empty.** HS-P0020 fixes
one such slot per page (`checked-documentation-surface/_design.md:303-305`) and the slice-mate
`surface-course-subscriptions` fills it. §7 carries an on-page link back up to §5 and nothing
outward.

**One reading recorded so a reviewer meets the argument rather than the overrun.** AC-004 says
the wrong fence is "never the last code on the page", and the binding design gives §7 no fence
of its own (`_design.md:518-521`: one paragraph and a link). Both are satisfied the way the
design intends: §7 is the last thing in reading order, it names the correct expression inline
(`held`, at `:152`), and it ends on a link up to §5. No `text` fence was invented to hold a
Rust line, because this project ships zero uncompiled fences and an uncompiled fence carrying
code is exactly what that commitment forbids.
