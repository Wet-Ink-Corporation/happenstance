---
item: "HS-S0188"
stage: implement
created: "2026-08-19T00:00:00.000Z"
updated: "2026-08-19T00:00:00.000Z"
---

# Implementation Report — The worked example surfaced in its own words

## TDD Evidence

**All seven assertions were written and run before any of the artefacts they guard existed**,
which is the only way this story's checks could have been shown to be non-decorative: every one
of them reads a file, and a file-reading check that has never been seen red is
indistinguishable from a comment.

`cargo test -p course-subscriptions --test reach` on the tree as story 1 left it:

```
test result: FAILED. 0 passed; 7 failed

---- overview_is_the_only_copy ----      examples/course-subscriptions/src/overview.md is readable
---- page_is_registered ----             xtask/src/narrative.rs declares no `mod read_the_worked_example {`;
                                         a page the harness does not name is the checker's
                                         `unregistered` state, not a page
---- exactly_one_answered_need ----      docs/read-the-worked-example.md is readable
---- orientation_precedes_departure ---- docs/read-the-worked-example.md is readable
---- seam_named_once_before_the_link --- docs/read-the-worked-example.md is readable
---- the_chain_holds ----                docs/carry-your-invariant.md carries no link to
                                         docs/read-the-worked-example.md; the good example is
                                         unreachable from the page whose reader is looking for it
---- page_holds_its_budgets ----         docs/read-the-worked-example.md is readable
```

Seven named failures, each naming the reader-visible thing that is missing. None is a compile
error, an import error or a typo.

**Green**, after the extraction, the page, the mount and the bridge link:
`cargo test -p course-subscriptions --test reach` → `7 passed; 0 failed`.

**EC-008's rule, discharged rather than assumed.** The specification says a `reach.rs` assertion
must never pass because a link's *text* is present while its target is not, and that every
assertion must have been seen to fail once. The RED run above covers six of the seven tests at
their first assertion. The one that the RED run could not reach — link-target resolution, which
only runs once the page exists — was falsified explicitly: the `overview.md` link was
re-pointed one directory up and

```
thread 'the_chain_holds' panicked at examples\course-subscriptions\tests\reach.rs:374:9:
the link "../examples/course-subscriptions/overview.md" on docs/read-the-worked-example.md
resolves to no file on disk
```

then restored, and it passed again. The assertion resolves the target on disk; it does not
match a string.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `reach.rs::overview_is_the_only_copy`; `cargo doc -p course-subscriptions` under `RUSTDOCFLAGS=-D warnings`; `reach.rs::runs` unchanged | **Red** — `overview.md is readable`. **Green** — `overview.md` is the only copy, `main.rs:5` includes it, no `//!` line remains, and a byte comparison of the stripped `//!` block against `overview.md` printed `IDENTICAL` |
| AC-002 | `reach.rs::page_is_registered`; HS-P0020's narrative checker | **Red** — `declares no mod read_the_worked_example`. **Green** — one module, one `include_str!` (asserted as a *count*), and `every narrative page is checked — 5 pages, all consistent` |
| AC-003 | `reach.rs::exactly_one_answered_need`; HS-P0021's own step | **Red** — the page did not exist. **Green** — one `orientation` declaration, the first block element under the H1, and `every page declares one need — 5 pages, 16 rules, all consistent` |
| AC-004 | `reach.rs::orientation_precedes_departure` | **Red** — the page did not exist. **Green** — two sentences of orientation, the DT-1 anchor cited at a byte offset before the outbound link, and zero occurrences of the four prior-model phrases |
| AC-005 | `reach.rs::seam_named_once_before_the_link` | **Red** — the page did not exist. **Green** — one seam sentence between the anchor and the link, `happenstance::commit` once, `happenstance_core` never, and no sentence of the page appearing verbatim in `overview.md` |
| AC-006 | `reach.rs::the_chain_holds` | **Red** — `docs/carry-your-invariant.md carries no link to docs/read-the-worked-example.md`, then falsified again on a dangling target. **Green** — three hops, every target resolved on disk, `main.rs` last, and the bridge's anchor heading asserted by its own text |
| AC-007 | `reach.rs::page_holds_its_budgets`; HS-P0020's `HIDDEN_MARKERS` rejection | **Red** — the page did not exist. **Green** — 31-character path, 22-character H1, 24 source lines, longest paragraph 267, zero fences, zero images, zero markers, zero affordances |

## Commits

| Commit | What it carries |
| --- | --- |
| `b24d2cd` | The slice-mate's checkpoint, `feat(application-author-path): Invariant to AppendCondition bridge` — the page this story links from and the anchor it cites |
| `5805623` | This story's checkpoint, `feat(application-author-path): Surface course-subscriptions` — the extraction, the page, the mount, the bridge link, `reach.rs`, this report, `report.md`, and the seven flipped ledger rows |

## Changes

| File | Shape of the change |
| --- | --- |
| `examples/course-subscriptions/src/overview.md` | **New.** The module doc's 28 lines, `//! ` prefixes stripped and nothing else touched. Proven byte-identical to the block it replaces |
| `examples/course-subscriptions/src/main.rs` | The `//!` block replaced by `#![doc = include_str!("overview.md")]` at `:5`, above `#![allow(clippy::print_stdout)]`, with four comment lines saying why. No import, handler, assertion or `Cargo.toml` line changed |
| `docs/read-the-worked-example.md` | **New**, 24 source lines. H1, one `orientation` declaration, two sentences of orientation, the DT-1 anchor citation, the seam sentence, the link to `overview.md`, the link to `main.rs` last. Zero fences, zero headings below the H1 |
| `xtask/src/narrative.rs` | **+8 lines.** One `#[cfg(doctest)] mod read_the_worked_example` holding one `include_str!` |
| `docs/carry-your-invariant.md` | **+3 lines** at the bottom — the single closing-pointer slot, filled with one link outward. Wiring, not a re-composition: the slice-mate's seven sections are untouched |
| `examples/course-subscriptions/tests/reach.rs` | **New**, 7 tests, no dependency added, `std` only. Reads five files as text and resolves every link target on disk |
| `.bklg/.../surface-course-subscriptions/_ledger.md`, `implementation-report.md`, `report.md` | Seven rows flipped with cited evidence; the two stage artifacts |

Nothing outside the PR boundary at `spec.md:235-243` was touched. In particular `docs/README.md`
has no new row, `examples/course-subscriptions/Cargo.toml` is unchanged, and neither
`xtask/src/lint_narrative.rs` nor any of HS-P0020's `TREE` / `HARNESS` / `IGNORE_ALLOWANCES` /
`HIDDEN_MARKERS` constants moved.

## Gates

**Story checkpoint — `cargo xtask affected --base main`: PASSED.** `233 passed; 0 failed;
2 ignored` doctests — unchanged from the slice-mate's checkpoint, which is the correct number:
this page authors zero fences, so tier 2 passes over it vacuously and AC-007 is the reason. Plus
`7 passed` in the new `reach` target and `10 passed` in `runs`, which is the check that the
extraction changed nothing the example asserts.

**`cargo fmt --all -- --check`: green.** It was red once — `rustfmt` wanted the `!main.lines()`
chain broken over three lines — and the write pass was run last, after every other edit, per the
gate's own ordering rule.

**Tier 1 — the identical-bytes guarantee, and the render.**

```
$ python -c "compare the stripped //! block against overview.md"
IDENTICAL

$ RUSTDOCFLAGS="-D warnings" cargo doc -p course-subscriptions --no-deps
Documenting course-subscriptions v0.2.0-alpha.1
Generated target\doc\course_subscriptions\index.html

target/doc/course_subscriptions/index.html
  'The canonical DCB worked example'            present
  'What is not in this file, and used to be'    present
  'boundary is drawn per decision'              present
  <details class="toggle top-doc">              present
```

Same position, same content, no new warning. EC-004 did not fire, so the migration was a move.

**Tier 1 — registration.** `cargo run -q -p xtask -- lints`:

```
=== every narrative page is checked ===
  5 pages, all consistent

=== every page declares one need ===
  5 pages, 16 rules, all consistent
```

The second line also carries RP-10-3: `orientation` is capped at one page per directory level,
and this is `docs/`'s only one.

**Tier 3 — the reach target.** `cargo test -p course-subscriptions --test reach` → `7 passed`.

**Tier 5 — the reviewer walk.** Two things no assertion reaches, checked by reading. *Does the
orientation orient?* It says what the destination is (three rules no single entity can hold, and
the decisions taken on them) and why the hop is worth it (the only place the cycle runs over
domain code rather than inside a fence) — and it says neither in the example's own words. *Is
the seam sentence true of the merged file?* `examples/course-subscriptions/src/main.rs` imports
`happenstance` and contains no `Query` and no `AppendCondition`; `commit` is imported at `:37`
and the three handlers hand it a `DecisionModel`. The sentence is true as written, and it names
no crate the file does not import.

**The slice-mate's budgets, re-measured after this story appended to its page.**
`docs/carry-your-invariant.md` is 159 source lines (was 156, budget 250), still one `h1`, still
no non-link prose line over 90 columns, longest paragraph 339. No criterion of HS-S0187's
regressed, and the DT-1 word walk still returns three hits, all on that page.

## Notes

**The vocabulary-seam sentence declines the design's permission rather than taking it.**
`_design.md:497-502` spends the slot on a sentence naming that the example imports
`happenstance_core` directly, and anti-pattern 13 permits that crate's name in exactly this one
place. Post-merge the premise is void — the merged example imports `happenstance` — so naming it
would be a confident falsehood delivered at the moment the reader is leaving. The slot, its
length (one) and its position (immediately before the link) are kept; what it says is the seam
that actually exists: the reader has just written a `Query` and an `AppendCondition` by hand and
the example holds neither, because `happenstance::commit` derives both from a `DecisionModel`.
`reach.rs::seam_named_once_before_the_link` pins that decision by asserting the page contains
`happenstance_core` zero times.

**The prior-model words are held to zero on this page, which is stricter than AC-004 asks.**
AC-004 permits `aggregate` inside the anchor sentence. Taking that permission would have added a
fourth hit to a tree where the slice-mate's AC-005 has just been flipped on the claim that the
four phrases occur on `docs/carry-your-invariant.md` **and nowhere else**. So the anchor sentence
says "the model you arrived with" and cites the anchor, and the assertion is written for zero.
Stricter, and it keeps two ledgers from disagreeing.

**"An include of `overview.md`" is not available on a markdown page, and the resolution is the
one the spec settled.** HS-P0020's D1 is *the markdown is the render* — no build step, no
preprocessor — and markdown has no include. So the page **links** to `overview.md` and quotes
none of it. One copy, one hop, and paraphrase is not a discipline anyone has to keep because
there is no second copy to drift from.

**Routed, not swallowed: a narrative page cannot include a source file's module doc.** That is a
substrate question and `_design.md:736-741` pre-authorises the routing — if HS-P0020's mechanism
ever *can* include it directly, the `overview.md` extraction becomes unnecessary and should be
dropped. Recorded here for HS-P0020 under project DoD item 9, not worked around on the page.

**Stale `:1-19` / `:1-28` citations are a known, costed consequence.** The move invalidates line
citations to the example's module doc across `project.md`, `_grounding.md`, `_storymap.md` and
`_design.md`. `cargo xtask spec-trace` does not read the example, so no gate breaks; the repair
belongs to the closeout's reference reconciliation (`_design.md:736-741`) and this story does not
edit signed-off stage artifacts to chase it.

**`reach.rs` is not registered in `xtask/src/proof.rs`'s `ARTEFACTS`.** That file is outside this
story's PR boundary, and its list is an explicit expectation set rather than an exhaustive index,
so a new target neither breaks it nor is asserted by it. If a later story wants this target's
names asserted out of `--list` the way `runs` and `ui` are, that is a `proof.rs` edit with its
own decision behind it.
