---
item: "HS-S0185"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — The opening encounter reaches a refused append

## Findings Ledger

The story's outcome as the review gate reads it. **Five of eight criteria are satisfied; three
carry a crate-root clause that the merged tree makes unauthorable and are routed rather than
flipped.** The reader-facing surface this story exists to land — `docs/first-encounter.md` —
is complete, mounted, executed by the gate, and reachable from the crate root.

| AC | Result | What proves it | Mount |
| --- | --- | --- | --- |
| AC-001 | **satisfied** | The step-3 doctest, executed: `cargo test -p xtask --doc -- first_encounter` → 3 passed. The print sits inside the matched `Err(AppendError::ConditionViolated(_))` arm (`docs/first-encounter.md:112-117`), so `refused: ConditionViolated` cannot appear unless the store refused; the output block at `:122-124` is byte-equal to that stdout. `xtask/tests/first_encounter.rs::step_three_prints_the_refusal_from_inside_the_matched_arm` | `xtask/src/narrative.rs:128-136` |
| AC-002 | **partial — routed** | Met: the fence executes rather than type-checking (`crates/happenstance/src/lib.rs:26-64`), no roadmap sits above it, the adapter redirect is last, ADR-0006's reasoning survives verbatim (`:76-81`), and literal `[happenstance_core]` bracket pairs are **zero under both** doc invocations where the baseline measured 2. Not met: `## A boundary refuses` and its refusal fence — see BC-002 | `crates/happenstance/src/lib.rs` |
| AC-003 | **satisfied** | `::every_later_step_opens_on_its_two_line_header_block` (two rendered lines, one-hop link to step one, first element under both later headings), `::step_one_carries_the_answered_need_in_line_ones_place`, `::every_step_fence_is_a_complete_program` — and tier 3, which compiles and runs all three fences as independent units | `xtask/src/narrative.rs:128-136` |
| AC-004 | **satisfied** | Red then green on the real doctest: untagged racing append → `the boundary did not hold: Ok(SequencePosition(2))`; `.with_tags(held)` → refused. `::the_racing_append_and_the_after_are_both_load_bearing` additionally rejects `Query::all()` and `Tags::empty()` | `xtask/src/narrative.rs:128-136` |
| AC-005 | **satisfied** | Zero `#`-prefixed lines on the authored page (`::no_hidden_line_carries_any_part_of_the_boundary`); on the crate root, two hidden lines, neither a boundary construct, with the `#[tokio::main]` wrapper decided **permitted** and recorded as observation O1 | `crates/happenstance/src/lib.rs` |
| AC-006 | **satisfied** | `cargo run -p xtask -- lints` → `3 pages, all consistent`, which is the checker that reports *unregistered* and *dangling registration* in both directions; `docs/README.md:20`; `IGNORE_ALLOWANCES` still `&[]`; no `ignore`/`no_run`/`compile_fail` anywhere on the page | `xtask/src/narrative.rs:128-136` |
| AC-007 | **partial — routed** | Met in full on the page: 68 columns, 24 lines, 435-character paragraphs, one answered-need per surface above the first fence, zero affordances, no prior-model vocabulary, facade-only imports, seven elements per step. Not met: three **inherited** crate-root numbers (35 lines vs 32, 70 columns vs 68, two `##` headings over 22) — see BC-002 | `crates/happenstance/src/lib.rs` |
| AC-008 | **partial — routed** | Met: all three steps close on an inline clause link (ES-8, ES-26, ES-25); `spec-trace` → `no problems found`; no `MUST` on either surface. Not met: the crate-root section that would carry the citation does not exist — see BC-002 | `xtask/src/narrative.rs:128-136` |

**The one deferral, stated once.** `_design.md` composes a `## A boundary refuses` section on
the crate root carrying the refusal fence, and `merge-forward-preflight/_baseline.md` routed the
crate root's density overages here on the assumption that "the rewrite replaces the program".
The merged tree forbids both routes: `crates/happenstance/tests/doc_budget.rs` asserts the crate
root carries **exactly one** fence and caps its module doc at 130 lines (this story's additions
reach exactly 130), so the section can exist only by deleting HS-P0016's signed-off landing
program — which `project.md`'s risk table names as outside this project's seam, which two
vocabulary bullets refer to by name, and which is the crate's one demonstration of the typed
layer ADR-0006 gave the bare name to. EC-008's response is to record and route, never to fold
in and never to edit a file a human signed off. The record is **BC-002** in
`.bklg/docs-that-teach/application-author-path/boundary-refusal-encounter/_conditions.md`,
routed to the `_design.md` sign-off owner. It is one contradiction with three acceptance
consequences, not three separate shortfalls.

**Two findings this story closed that it did not have to.** BC-003: both `[happenstance_core]`
shortcut references are now explicit intra-doc links, so anti-pattern 2's count is **zero under
both** `cargo doc -p happenstance --no-deps` and the gate's own `--workspace --all-features
--document-private-items` build — the invocation-dependence `_baseline.md` disposition row 9
routed here is closed at the page level, and the gate-level hole (a `-D warnings` build that
renders an unresolved shortcut reference and exits 0) stays routed to `support`. O1: the
`#[tokio::main]` wrapper, which `_design.md:524` places on neither the permitted nor the
forbidden hidden-line list, is decided **permitted** with its reason recorded, rather than
inherited.

**One deviation from the certified fence shape, and it is upward.** Step 3 prints from a `match`
arm that panics on any other outcome, where `_resolutions.md` § DT-4 used
`println!("{refused:?}")` plus `assert!(matches!(…))`. `Display` on `ConditionViolated` never
contains the token and the `Debug` line is 93 characters, so the certified shape could not put
the word in a line that fits; the `match` puts it in a 26-character line that is unreachable
unless the store refused, and reports the actual value when it does not. The fence still lands
at exactly 24 rendered lines and 68 columns — DT-4's zero-headroom measurement — paid for by
folding the `use` block from four lines to three.

**Scope.** One path outside `spec.md`'s PR-boundary fence: `xtask/tests/first_encounter.rs`,
the seventeen source-reading assertions. Recorded as observation O4 rather than answered by
widening the fence, on `_baseline.md` § Carried forward P2's reasoning — a fence widened to
quiet a report stops being a statement about scope.

## Acceptance

Five of eight criteria satisfied with cited evidence in `_ledger.md`; three partial, each
carrying its met half as evidence and its blocked half routed to BC-002. `redkiln verify
--grain story` will therefore report `ledger` red until the sign-off owner answers BC-002, and
that is the intended reading: a partial criterion is not a satisfied one.

Traced project ACs: **AC-004** (`project.md:242-246`, initiative DoD-3 — "following the opening
encounter end to end from its first step, a reader reaches a running program in which an append
is refused because a consistency boundary held, and the refusal appears in the program's own
output") is **discharged in full on the page**, which is the surface that criterion names.
**AC-006** (no empty boundary where prose claims a real one) is discharged by this story's
AC-004 and AC-005. **AC-002** (DT-4's disclosure shape and its documented failure mode) is
*realized* here by AC-003, on the cold arrival it was designed for.

Initiative **DoD-3** is reached. **DoD-4** is made reachable and is closed by the slice-mate.

## Knowledge Harvest

Three things worth carrying past this story, none of them settled here.

1. **A design signed off before a merge can be contradicted by the merge in a way no
   disposition table catches.** `merge-forward-preflight` re-measured every number the design
   named and routed the overages forward — correctly — but it could not see that the *response*
   it assumed (replace the program) was itself forbidden by a test in another project's suite.
   The general shape: a routed condition should name the move it expects, so the receiving story
   can falsify the move rather than only the number. HS-P0025 inherits this.
2. **The instrument that made this story reviewable is a source-reading test, not a doctest.**
   Every fence passing and every clause resolving is satisfied by a page whose steps open on
   prose, hide the boundary behind `#`, and quietly opt a fence out. Seventeen assertions in
   `xtask/tests/first_encounter.rs` are what fail that page, and they cost nothing to run. The
   pattern generalises to the three pages this project has left, and whether it should be
   generalised *into* `xtask/src/lint_narrative.rs` rather than repeated per page is a question
   for `fence-inventory-and-clause-audit`.
3. **An unresolved shortcut intra-doc link warns about nothing.** `` [`x`] `` that fails to
   resolve is left as literal text and produces no diagnostic, so `RUSTDOCFLAGS=-D warnings`
   exits 0 over a broken link a reader sees as `[x]`. An explicit `` [`x`](x) `` resolves — and,
   if it did not, would fail the gate. That is a repository-wide preference with a mechanism
   behind it, and it belongs in `standards/rust/70-rustdoc-obligations.md` rather than in this
   report; the gate half is already routed to the `support` initiative.
