---
item: "HS-S0186"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — Removing the boundary makes the repository fail

## TDD Evidence

Nine source-reading assertions in `xtask/tests/falsification_drill.rs`, written before the
drill existed and before the twin existed, plus the drill itself run in both directions against
two real mounts. The order was the one the spec's implementation notes fix — **capture, then
author**: the failing direction was run and pasted into `_drill-observation.md` first, and the
page's quoted block was taken from that transcript, never the other way round.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `::the_drill_is_a_subsection_at_the_bottom_of_step_three`, `::the_drill_states_the_edit_then_the_failure_then_the_revert` | **Red** — `` `### Try it wrong, then put it back` is not under `## A condition that refuses` on docs/first-encounter.md ``. **Green** — the `###` lands at `docs/first-encounter.md:131`, after the ES-25 citation and as step 3's only subsection, with the three labels present **in order** |
| AC-002 | `::the_twin_asserts_refusal_with_the_condition_and_acceptance_without`, `::no_gate_step_was_added_for_the_drill`, and both mounts executed | **Red** — `crates/happenstance/tests/boundary_refusal.rs is readable` failed: the twin did not exist. **Green** — `cargo test -p xtask --doc -- first_encounter` → 3 passed and `cargo test -p happenstance --test boundary_refusal` → 2 passed, both inside the existing `"tests"` step, `IGNORE_ALLOWANCES` still `&[]` |
| AC-003 | The drill, executed | **Red by construction** — this is the criterion whose red *is* the deliverable. Under the edit, the page mount fails with `the boundary did not hold: Ok(SequencePosition(2))` at `narrative::first_encounter (line 96)` and the twin fails with the same message at `crates\happenstance\tests\boundary_refusal.rs:67:15`. **Green** — both pass with the condition in place |
| AC-004 | The drill's second direction | **Red** — the two failures above. **Green** — one inverse change at each mount, `3 passed` and `2 passed`, `git status --porcelain` empty at the checkpoint |
| AC-005 | `::the_transcript_records_both_directions` | **Red** — the transcript did not exist. **Green** — `_drill-observation.md` carries `## The starting tree`, `## Direction one — the boundary removed`, `## Direction two — the boundary restored`, the closing clean-tree check, and three routed findings |
| AC-006 | `::the_pages_quoted_failure_is_byte_equal_to_the_recorded_one`, `::the_quoted_failure_is_copy_faithful`, `::the_drill_neither_uses_nor_suggests_emptying_the_query` | **Red** — all three failed on the missing drill. **Green** — every non-blank line inside the drill's fences is found byte-for-byte in the transcript; no `…`; no `from_items([])` |
| AC-007 | `::the_drill_is_persistent_and_introduces_nothing`, plus the whole of `xtask/tests/first_encounter.rs` re-run | **Red** — failed on the missing drill. **Green** — no fold, no affordance, no prior-model word, no `MUST`, no second answered-need, no `happenstance_core`; step 3 carries exactly one `###`; all 17 of the slice-mate's page assertions still pass |

**One assertion of the slice-mate's was corrected rather than gutted, and the correction is the
design's own composition.** `xtask/tests/first_encounter.rs::every_step_closes_on_a_clause_citation`
read to the end of each step, which made it demand that the citation come *after* the drill —
the opposite of `_design.md` `## Composition` ("Step 3 additionally carries, **after item 6**,
the falsification drill as its own `###`"). It now reads to the first `###`, so it still fails
if a step's own material does not close on its citation, and the drill's placement below the
citation is asserted independently from the other side by
`falsification_drill.rs::the_drill_is_a_subsection_at_the_bottom_of_step_three`. The check is
narrower in scope and no weaker: both halves of the ordering are now pinned, where before one
was pinned backwards.

> **Fix pass, 2026-08-19 — the drill was run again, because the scenario under it changed.**
> The slice-mate's step 3 was reading an empty store, so its `after_opt(upto)` was inert
> (`boundary-refusal-encounter/_conditions.md` § **BC-004**). Corrected, it lands one matching
> seat before the read, which puts one more event in the store and moves the failure the drill
> produces from `Ok(SequencePosition(2))` to `Ok(SequencePosition(3))`. The rows above are the
> implementation run's record and are left as written; what changed since:
> the drill was performed end to end again at both mounts and recorded in `_drill-observation.md`
> § *Re-run 2026-08-19* (page: `narrative::first_encounter (line 96)` FAILED with the new line;
> twin: `boundary_refusal.rs:87:15` with the same line; both green again after the one inverse
> change; `git hash-object` on both mounts equal before the edit and after the revert). The page's
> quoted block was re-taken from that transcript, so
> `::the_pages_quoted_failure_is_byte_equal_to_the_recorded_one` still passes on a line the run
> actually produced. The twin gained the same seed and a third test,
> `::the_after_is_load_bearing_in_its_value_not_in_its_presence` — `3 passed`, not 2 — and the
> drill's own nine assertions and the slice-mate's nineteen are green.

## Commits

| SHA | Subject |
| --- | ------- |
| `cc9c4a4` | feat(application-author-path): Boundary falsification drill |

The checkpoint was amended once, to fold in the clean-tree re-run recorded in
`_drill-observation.md` § Re-run at the story checkpoint. That re-run was performed against the
pre-amend sha `b64448a`, which is the sha the transcript quotes; the two trees differ only in
that record.

## Changes

| File | Shape of the change |
| --- | --- |
| `docs/first-encounter.md` | The `###` slot at the bottom of step 3, filled: `### Try it wrong, then put it back` (`:131-161`) — the heading `tension-resolutions/_resolutions.md` § Anchor table supplies — carrying **The edit.**, **What you should see.** and **Putting it back.**, in that order, after the ES-25 citation and above nothing. Two `text` blocks: the command to run, and the two contiguous verbatim lines of the captured failure. Nothing above the `###` moved |
| `crates/happenstance/tests/boundary_refusal.rs` | **New — the second mount.** Step 3's scenario as two `#[tokio::test]`s against a real `MemoryEventStore`: `the_guarded_append_is_refused`, and `without_the_condition_the_same_append_is_accepted`, which is the drill's own edit as a standing assertion. Both are swept by the existing `"tests"` REQUIRED step. No mock, no stub, no `happenstance-testkit` fixture |
| `xtask/tests/falsification_drill.rs` | **New.** Nine assertions: composition and ordering of the drill's three parts, the twin's two directions, the byte comparison against the transcript, copy fidelity, the forbidden `from_items([])` spelling, transience, and a regression guard that `REQUIRED` and `IGNORE_ALLOWANCES` are untouched |
| `xtask/tests/first_encounter.rs` | One assertion narrowed to the step's own material, for the reason above |
| `.bklg/.../boundary-falsification-drill/_drill-observation.md` | **New.** The tier-4 record: starting tree, both directions at both mounts verbatim, a reproducibility re-run, the cost note, and three routed findings |
| `.bklg/.../boundary-falsification-drill/_ledger.md` | All seven rows flipped to `satisfied: true` with cited evidence |

Not touched: `xtask/src/main.rs` (`REQUIRED` unchanged — NF-001), `crates/happenstance/Cargo.toml`
(NF-003), `spec/SPECIFICATION.md`, `_design.md`, and `IGNORE_ALLOWANCES`, still `&[]`.

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p xtask --test falsification_drill` | **green** — 9 passed |
| `cargo test -p xtask --test first_encounter` | **green** — 17 passed, after the narrowing above |
| `cargo test -p happenstance --test boundary_refusal` | **green** — 2 passed |
| `cargo test -p xtask --doc -- first_encounter` | **green** — 3 passed |
| `cargo fmt --check -p happenstance -p course-subscriptions -p xtask` | **green** |
| `cargo clippy --all-features -p happenstance -p course-subscriptions -p xtask --all-targets -- -D warnings` | **green** |
| `cargo test -p happenstance -p course-subscriptions -p xtask --all-features` | **green** — every target, including the 289-test and 169-test suites |
| `cargo run -p xtask -- lints` | **green** — `3 pages, all consistent`; no `HIDDEN_MARKERS` tripped |
| `cargo run -p xtask -- spec-trace` | **green** — `traceability: no problems found` |
| `cargo run -p xtask -- affected --base main` | **green** — `affected gate passed` |

**One flake, isolated and not this slice's.** Two runs of `cargo xtask affected --base main`
failed inside `happenstance-sqlite` — once on `tests/migration.rs`, once on
`concurrency::append_returns_the_callers_own_last_position` — and both files pass on their own
immediately afterwards. The same command on the stashed (pre-slice) tree also has to be re-run,
and the third run of the slice tree passed clean. `happenstance-sqlite` is not one of this
project's affected packages (`happenstance`, `course-subscriptions`, `xtask`), is untouched by
this slice, and is swept only because the branch carries merge `a5c0f30`, which makes every
workspace package "affected" against `main`. The project-scoped gate above is green on every
run.

## Notes

**The twin moved, and this is the record of where and why.** The spec and `_design.md` both put
step 3's program *twice*: once on the page, once as the crate-root fence in
`crates/happenstance/src/lib.rs`, on the ground that `include_str!` cannot cross a published
package boundary. The merged crate root cannot carry it —
`boundary-refusal-encounter/_conditions.md` **BC-002** records the measurement: one fence only,
by a signed-off assertion in `crates/happenstance/tests/doc_budget.rs`, and a 130-line module
doc budget the slice-mate's additions already sit exactly on. The twin is therefore
`crates/happenstance/tests/boundary_refusal.rs`, which is inside this story's own PR boundary
(`crates/happenstance/tests/**`) and is exactly what its EC-002 pre-scopes.

It is a **stronger** second mount than the doc fence would have been, and the reason is worth
stating because it is the difference between a drill and a demonstration: a doc fence can only
show the guarded append being refused. The twin also runs the *same* append with the condition
removed and asserts it is accepted, which is the assertion that fails if the scenario refuses
for a reason other than the boundary — a guard that matches nothing, an `after` doing all the
work, or a store that rejects everything. It also reports against a file a reader can open,
where the page mount reports against a rustdoc temporary bundle. Both properties are routed and
recorded rather than assumed: **F1** to HS-P0020, **F2** to the `_design.md` sign-off owner.

**EC-001 did not fire, and that was not a foregone conclusion.** The initiative's first-ranked
risk is a boundary that looks real in the source and is not load-bearing, and the drill is the
only instrument that can detect it. Under the edit, the refusal assertion failed at both mounts
while `without_the_condition_the_same_append_is_accepted` passed — so the append, the store and
the tag join are all sound, and the one thing that changed is the one thing removed. Had it
stayed green, the response would have been to fix the scenario with the slice-mate, not to
weaken AC-003.

**EC-004 fired and is routed, not softened.** On the page mount the panic's own `file:line` is
`C:\Users\…\AppData\Local\Temp\rustdoctest…\doctest_bundle_2024.rs:83:15` — a path that exists
on one machine for the length of one run. `xtask/src/narrative.rs` already carries this as a
measured standing limit, which is why the doctest's *module name* is the only stable identifier
and why one module per page is what registration buys. The transcript quotes the stanza in full,
temp path and process id included; the page quotes the two contiguous lines that are identical
across runs and names in prose which doctest reports them. That is NF-005's "the drill says
which line is *the* line" and EC-004's "quote it exactly, route the illegibility", not a
paraphrase.

**One clippy allow, with its reason on the item.** `clippy::cloned_ref_to_slice_refs` wants
`std::slice::from_ref(&seat)` where the twin writes `&[seat.clone()]`. It is allowed, because
the twin's whole job is to be the page's fence a second time and the only defence against the
two drifting is that they read the same — a different spelling in that one file would be a
difference a reviewer has to think about. EC-009 is why both copies exist at all.
