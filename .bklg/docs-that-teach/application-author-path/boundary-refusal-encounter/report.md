---
item: "HS-S0185"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — The opening encounter reaches a refused append

## Findings Ledger

The story's outcome as the review gate reads it, **reconciled 2026-08-19 against `spec.md`
§ Amendment — BC-002**, which the sign-off owner authored at `faa8834` *after* the rows below
were first written, and **revised again on the 2026-08-19 fix pass** for AC-004 and AC-007.
**Eight of eight criteria are satisfied.** The reader-facing surface this story exists to land
— `docs/first-encounter.md` — is complete, mounted, executed by the gate, and reachable from
the crate root.

Two reconciliations, in the order they happened. **First**, three rows said *partial — routed*
while the amendment that supersedes them said two of the three stand as met; a story cannot be
presented for review with its ledger and its governing amendment disagreeing about what is
done. AC-002 and AC-008 were flipped against the amendment's *what stands in its place* column.
AC-007 was **not**, because the clause it strikes is *owed*, and at that point the item that
owes it did not exist. **Second**, `8161418` opened **HS-B0001** `crate-root-density-overages`
under the `support` initiative carrying F-1, F-2 and F-3 verbatim, and AC-007 was flipped
against that item's id — not against prose. The earlier refusal to flip stands in the row as
the audit trail, because it was correct at the time and is the reason the destination exists.

**Third, on the fix pass:** a slice review found AC-004's `after` clause claimed met and false —
step 3 read an empty store, so `.after_opt(upto)` was inert and the assertion pinning it was a
source substring. The scenario is corrected, the pinning assertions are behavioural, and the
half of the criterion that no program can satisfy is measured and routed as **BC-004**. See
*The `after` was decoration, and now is not* below.

| AC | Result | What proves it | Mount |
| --- | --- | --- | --- |
| AC-001 | **satisfied** | The step-3 doctest, executed: `cargo test -p xtask --doc -- first_encounter` → 3 passed. The print sits inside the matched `Err(AppendError::ConditionViolated(_))` arm (`docs/first-encounter.md:113-118`), so `guarded above Some(SequencePosition(1)): ConditionViolated` cannot appear unless the store refused; the output block at `:123-125` is byte-equal to that stdout. `xtask/tests/first_encounter.rs::step_three_prints_the_refusal_from_inside_the_matched_arm` | `xtask/src/narrative.rs:128-136` |
| AC-002 | **satisfied as amended** | `spec.md` § Amendment — BC-002 strikes the `## A boundary refuses` heading, fence and printed refusal; everything standing in their place is met. The fence executes rather than type-checking (`crates/happenstance/src/lib.rs:26-64`), no roadmap sits above it, the adapter redirect is last (`:145-147`), ADR-0006's reasoning survives verbatim (`:77-82`), literal `[happenstance_core]` bracket pairs are **zero under both** doc invocations where the baseline measured 2, the answered-need line stands above everything (`:21`), and the pointer beneath the fence (`:66`) reaches the refusal in one hop | `crates/happenstance/src/lib.rs` |
| AC-003 | **satisfied** | `::every_later_step_opens_on_its_two_line_header_block` (two rendered lines, one-hop link to step one, first element under both later headings), `::step_one_carries_the_answered_need_in_line_ones_place`, `::every_step_fence_is_a_complete_program` — and tier 3, which compiles and runs all three fences as independent units | `xtask/src/narrative.rs:128-136` |
| AC-004 | **satisfied**, one clause routed | Red then green on the real doctest: untagged event → `the boundary did not hold: Ok(SequencePosition(3))`; `.with_tags(held)` → refused. The `after` half is now a *property* rather than a substring: step 3 seeds, reads `Some(SequencePosition(1))`, races above it and guards on it, pinned by `::step_three_guards_on_a_position_its_own_read_observed` (verified red against the shipped program) and by the executed `boundary_refusal.rs::the_after_is_load_bearing_in_its_value_not_in_its_presence`. The criterion's *"remove `after_opt` and it stops refusing"* clause is unsatisfiable — `after: None` checks the whole log — measured and routed as **BC-004** | `xtask/src/narrative.rs:128-136`, `crates/happenstance/tests/boundary_refusal.rs` |
| AC-005 | **satisfied** | Zero `#`-prefixed lines on the authored page (`::no_hidden_line_carries_any_part_of_the_boundary`); on the crate root, two hidden lines, neither a boundary construct, with the `#[tokio::main]` wrapper decided **permitted** and recorded as observation O1 | `crates/happenstance/src/lib.rs` |
| AC-006 | **satisfied** | `cargo run -p xtask -- lints` → `3 pages, all consistent`, which is the checker that reports *unregistered* and *dangling registration* in both directions; `docs/README.md:20`; `IGNORE_ALLOWANCES` still `&[]`; no `ignore`/`no_run`/`compile_fail` anywhere on the page | `xtask/src/narrative.rs:128-136` |
| AC-007 | **satisfied**, against an item id | Met in full on the page: 68 columns, 24 lines, 435-character paragraphs, one answered-need per surface above the first fence, zero affordances, no prior-model vocabulary, facade-only imports, seven elements per step. The three **inherited** crate-root numbers (35 rendered lines vs 32, 70 columns vs 68 on two lines, two `##` headings over 22 at 39 and 26) are struck by the amendment and now **owned**: `8161418` opened **HS-B0001** `crate-root-density-overages` (`.bklg/support/inherited-documentation-defects/crate-root-density-overages/bug.md`) carrying F-1, F-2 and F-3 verbatim with their budgets and re-measurement commands. Recorded as OWED, not waived. The earlier refusal to flip — while the destination was a project name on an unmerged branch — stands in `_ledger.md` as the audit trail and was correct | `crates/happenstance/src/lib.rs` |
| AC-008 | **satisfied as amended** | `spec.md` § Amendment — BC-002 confines the criterion to `docs/first-encounter.md`, there being no crate-root section to carry a last sentence. All three steps close on an inline clause link in last position (ES-8, ES-26, ES-25); `spec-trace` → `no problems found`; no `MUST` on either surface; the crate-root pointer (`:66`) reaches the ES-25 sentence in one hop. Hardened on the fix pass by `::step_two_reaches_the_boundary_its_citation_claims`, which checks the ES-26 citation against what step 2's program *does* rather than against a resolvable anchor | `xtask/src/narrative.rs:128-136` |

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

**Scope, and how it was answered.** One path landed outside `spec.md`'s five-entry PR-boundary
fence at checkpoint `9493276`: `xtask/tests/first_encounter.rs`, now eighteen source-reading
assertions. It was recorded as observation O4 rather than answered, on the reasoning that a
fence widened to quiet a report stops being a statement about scope. That reasoning was right
about *widening in silence* and wrong about what EC-010 permits: EC-010 names exactly two
responses — revert it, or route it as its own story — and recording is neither. The fence is
now amended to admit `xtask/tests/**`, with the reason inline and on its own commit (`4232b34`),
in the form `a41a1a5` used. The widening admits tests only; `xtask/src/**` stays outside, so no
production path, gate step or `REQUIRED` entry is admitted by it, and EC-010's actual concern —
a second page under `docs/` — is untouched.

**Step 2 said more than its program showed, and now does not.** The sentence beneath step 2's
fence cited ES-26 — *`after` is exclusive, an event at exactly `after` never rejects* — over a
program that read an **empty** store, so `after_opt(None)` meant "no matching event at all" and
the exclusivity boundary was never reached. That is this initiative's headline defect shape in
miniature, on the page written to close it, and no check in the repository could see it:
`::every_step_closes_on_a_clause_citation` requires only a link and `spec-trace` only that the
id resolves. The program is corrected rather than the sentence — step 2 now lands one matching
event, reads it back so `upto` is `Some(SequencePosition(1))`, and guards on exactly that
position, which is also a closer on-ramp to step 3. Its output block, byte-equal to the run,
shows both numbers: `1 seen, up to Some(SequencePosition(1)), at SequencePosition(2)`. The new
assertion `::step_two_reaches_the_boundary_its_citation_claims` fails against the old program
(verified red: *step two never lands an event of its own*), so the citation is checked against
what the fence does rather than against a resolvable anchor. Line numbering is unchanged — the
three fences still open at 16, 55 and 96 — so the drill's quoted `narrative::first_encounter
(line 96)` and `_drill-observation.md` remain byte-accurate.

**The `after` was decoration, and now is not.** The same defect shape survived one step further
on, and the slice review caught it: step 3 read an **empty** store, so `upto` was `None` and
`.after_opt(upto)` was inert — `AppendCondition::new` already carries `after: None`. Delete the
call and the step-3 doctest stayed green, while `_ledger.md` asserted *"both halves of the guard
are therefore load-bearing"* and the test named for the property checked only that the source
contained the substring `after_opt(upto)`. By this repository's own bar, the wrong
implementation it should have rejected is the one that shipped.

Three things changed, and none of them is a re-worded evidence field. **The scenario**: step 3
lands one matching seat before the decision reads, so the read observes
`Some(SequencePosition(1))`, the racing writer lands strictly above it, the guard is built from
that position, and the reader sees it — `guarded above Some(SequencePosition(1)):
ConditionViolated`. **The assertion**: `::step_three_guards_on_a_position_its_own_read_observed`
checks the seed precedes the read, the race sits between the read and the guarded append, and
the output block shows the position; it was verified red against the shipped program.
**The finding**: deleting `after_opt` *strengthens* the guard to whole-log, so the criterion's
second falsification cannot be had by anyone — measured, recorded as **BC-004**, routed to the
sign-off owner with the wording that is true, and pinned by an executed test,
`boundary_refusal.rs::the_after_is_load_bearing_in_its_value_not_in_its_presence`, which runs
both directions: whole-log still refuses; a guard built from a read taken after the race is
accepted, and that acceptance is the lost update.

Two consequences the slice absorbed rather than deferred. The twin was updated to the same
scenario, so the two copies still read as the same program. And the drill's quoted failure moved
from `Ok(SequencePosition(2))` to `Ok(SequencePosition(3))` — one more event in the store — so
the drill was **performed again for real** at both mounts and `_drill-observation.md` gained
§ *Re-run 2026-08-19*, with the earlier transcripts left standing as the audit trail. Step 3's
fence still opens at line 96, because the added line is inside it.

## Acceptance

**Eight of eight criteria satisfied** with cited evidence in `_ledger.md`.

**AC-007 was carried open and now is not, and both halves belong in the record.** Its page half
was met and cited from the start; its three inherited crate-root numbers are struck by
`spec.md` § Amendment — BC-002 and were *owed* to HS-P0016, a project on an unmerged sibling
branch that this branch cannot route to. The row was therefore held at `satisfied: false` — a
criterion whose strike has no owner is not a satisfied one, and flipping it against a prose
sentence naming another branch's project would have reproduced the defect the ledger's own
rules exist to prevent. `8161418` opened the destination: **HS-B0001**
`crate-root-density-overages`, under `HS-P0026 inherited-documentation-defects` in the `support`
initiative, carrying F-1, F-2 and F-3 verbatim with their budgets, their re-measurement commands
and their reader impact, and citing `_conditions.md` § BC-002 and `spec.md` § Amendment — BC-002
as its origin. The row is flipped against that item's id. The overages are **owed, not waived**:
they remain real overages against budgets this project set and met everywhere it was permitted
to, and the item that owns them is where they are answered.

**AC-004 carries one routed clause and is nonetheless satisfied**, which is a different
disposition and is argued rather than assumed: `_conditions.md` § BC-004 measures why *"remove
`after_opt(upto)` and the scenario stops refusing"* cannot be true of any program, states the
wording that is true of the code, routes it to the acceptance-table sign-off owner, and — the
part that distinguishes it from AC-007 — records that no reader is worse off, because what the
criterion actually obliges is met, checked in both directions, and now impossible to satisfy by
accident.

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
