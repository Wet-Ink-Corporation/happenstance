---
item: "HS-S0186"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — Removing the boundary makes the repository fail

## Findings Ledger

**All seven criteria satisfied.** The boundary the slice-mate's page demonstrates is now
load-bearing to the repository at two executed mounts, the drill that lets a reader prove it for
themselves is on the page in the design's slot, and the run that proves it happened in both
directions is recorded rather than asserted.

| AC | Result | What proves it | Mount |
| --- | --- | --- | --- |
| AC-001 | **satisfied** | `docs/first-encounter.md:132-163` — `### Try it wrong, then put it back`, under `## A condition that refuses`, after the ES-25 citation, step 3's only `###`. Three labelled parts in order: **The edit.**, **What you should see.**, **Putting it back.** `xtask/tests/falsification_drill.rs::the_drill_is_a_subsection_at_the_bottom_of_step_three` and `::the_drill_states_the_edit_then_the_failure_then_the_revert`, the second scanning the labels **in order** so a transposition fails | `xtask/src/narrative.rs:128-136` (page) |
| AC-002 | **satisfied** | Both mounts executed on a clean tree: `cargo test -p xtask --doc -- first_encounter` → 3 passed; `cargo test -p happenstance --test boundary_refusal` → 3 passed. Both inside the existing `"tests"` REQUIRED step. No `ignore`/`no_run`/`compile_fail` at either mount; `IGNORE_ALLOWANCES` still `&[]` | page + `crates/happenstance/tests/boundary_refusal.rs` |
| AC-003 | **satisfied** | The edit applied for real, and applied again after the scenario changed: page → `the boundary did not hold: Ok(SequencePosition(3))` at `narrative::first_encounter (line 96)`; twin → the same message at `crates\happenstance\tests\boundary_refusal.rs:87:15`. Both are the refusal assertion, reached by a program that compiled and ran. `REQUIRED` unchanged | page + twin |
| AC-004 | **satisfied** | One inverse change at each mount and nothing else: 3 passed and 3 passed again, `git status --porcelain` empty at the checkpoint and both mounts byte-identical by `git hash-object` across the 2026-08-19 re-run. EC-005 did not fire | page + twin |
| AC-005 | **satisfied** | `_drill-observation.md`, dated 2026-08-18: starting tree and HEAD sha, both directions verbatim at both mounts, a reproducibility re-run, the closing clean-tree check, the cost note, three routed findings — plus § *Re-run 2026-08-19*, the whole drill performed again end to end after BC-004 changed step 3's program. `::the_transcript_records_both_directions` fails if a half goes missing | both |
| AC-006 | **satisfied** | `::the_pages_quoted_failure_is_byte_equal_to_the_recorded_one` walks every quoted line and requires it in the transcript — the check that catches a drill written from imagination. Plus `::the_quoted_failure_is_copy_faithful` (no elision) and `::the_drill_neither_uses_nor_suggests_emptying_the_query` | page |
| AC-007 | **satisfied** | `::the_drill_is_persistent_and_introduces_nothing` — no fold, no affordance, no prior-model word, no `MUST`, no second answered-need, no `happenstance_core`; one `###` in the step; `cargo run -p xtask -- lints` green (no `HIDDEN_MARKERS`); all 19 page assertions still pass; `spec-trace` resolves ES-25 | page |

**The finding that mattered, and it could have gone the other way.** Under the edit the refusal
assertion failed at both mounts *while* `without_the_condition_the_same_append_is_accepted`
passed. Read together those two results say the append, the store and the tag join are all
sound and the only thing that changed is the only thing removed. EC-001 did not fire: the
boundary is load-bearing, not decorative. That is the initiative's first-ranked risk answered by
measurement, and the drill is the only instrument in this repository that could have answered it
either way.

**One structural deviation, recorded and stronger.** The spec puts the twin in
`crates/happenstance/src/lib.rs` as a second doc fence. The merged crate root cannot carry one —
`boundary-refusal-encounter/_conditions.md` **BC-002**: a signed-off assertion in
`crates/happenstance/tests/doc_budget.rs` allows exactly one fence there, and the module-doc
budget is already at its ceiling. The twin is therefore
`crates/happenstance/tests/boundary_refusal.rs` — the placement this story's own PR boundary
pre-authorises — and it carries the half no doc fence can: the same
append run *without* the condition, asserted accepted. It also reports against a file a reader
can open.

**The trigger was BC-002, and this sentence is here so a later reader does not read it as
EC-002.** This story's EC-002 is the contingency for *the executed path failing to execute* —
HS-P0020's step type-checking without running — and its response routes a substrate finding to
HS-P0020. It did not fire. The executed path executed: `cargo test -p xtask --doc --
first_encounter` → `3 passed`, and `_drill-observation.md` § Direction one shows the page mount
going red under the edit, which a compile-only step could not do. The spec pre-scoped
`crates/happenstance/tests/**` under EC-002 alone, so the file is in-fence for a reason the spec
happened to write down rather than for the reason it actually landed — and reading the row the
other way would misroute a substrate finding to HS-P0020 that nobody made. What the substitution
gives up is that the twin is not a reader-facing surface: the page proves the claim to the
reader, the twin proves it to the repository.

**One substrate finding routed.** EC-004 fired on the page mount: the doctest's panic reports
against a rustdoc temporary bundle under the OS temp directory, not against
`docs/first-encounter.md`, so the doctest's *module name* is the only stable identifier a reader
has. `xtask/src/narrative.rs` already carries this as a measured limit. Routed to **HS-P0020**
as F1, with the full stanza in the transcript; the page quotes the two lines that are identical
across runs and names the reporting doctest in prose, which is EC-004's "quote it exactly" plus
NF-005's "say which line is *the* line". It is not a paraphrase and not a softened drill.

**One assertion of the slice-mate's corrected, not gutted.** `every_step_closes_on_a_clause_citation`
read to the end of the step, which demanded the citation come *after* the drill — the opposite
of `_design.md` `## Composition`. It now reads to the first `###`, and the drill's own test pins
the drill below the citation from the other side, so both halves of the ordering are asserted
where before one was asserted backwards.

**Scope, and how it was answered.** One path landed outside this story's PR-boundary fence at
checkpoint `cc9c4a4`: `xtask/tests/falsification_drill.rs`, the nine source-reading assertions.
Recording it — the disposition it first took, matching the slice-mate's observation O4 — is not
one of the two responses EC-010 sanctions. The fence is now amended to admit `xtask/tests/**`,
with the reason inline and on its own commit (`4232b34`). It was already admitting
`crates/happenstance/tests/**`, so what was missing was `xtask`'s test directory, not a decision
to exclude it; `xtask` is the crate that owns the narrative tree and is `publish = false`, the
widening is tests-only, and the alternative was no mechanical check at all for AC-001, AC-006
and AC-007 — including the byte comparison between the failure the page quotes and the failure
the transcript recorded, which is the check this story's own risk table ranks first.

**The drill was performed a second time, because the scenario under it changed.** The slice's
fix pass corrected step 3 so the guard carries a position the program's own read observed
(`boundary-refusal-encounter/_conditions.md` § **BC-004**), which puts one more event in the
store and moves the position the unconditional append lands at from `2` to `3`. A transcript
that was not re-run would have left the page quoting a failure that no longer happens — the
exact defect `::the_pages_quoted_failure_is_byte_equal_to_the_recorded_one` exists to catch, and
it would have caught it. So both directions were run again at both mounts and recorded in
`_drill-observation.md` § *Re-run 2026-08-19*, with the 2026-08-18 transcripts left standing as
the audit trail. The twin gained the same seeded scenario and a third test,
`::the_after_is_load_bearing_in_its_value_not_in_its_presence`, which passed under the drill's
edit throughout — it builds its own conditions — and is what says the *guard*, not the scenario
around it, is what the edit removes.

## Acceptance

Seven of seven criteria satisfied with cited evidence in `_ledger.md`.

Project **AC-005** — *"removing the boundary from that scenario makes a check the repository
runs fail, observed once as a failure and once as a recovery after reverting"* — is discharged
in full: AC-002 and AC-003 carry "makes a check fail", AC-003 and AC-005 carry "observed as a
failure", AC-004 and AC-005 carry "observed as a recovery". Project DoD item 3 is met. No other
project AC is traced by this story.

Initiative **DoD-4** — *"the boundary claim is checked, not narrated"* — is reached, and reached
as the initiative stated it: run and observed, by a human, in both directions, dated, with the
output pasted rather than described.

The slice `opening-encounter` is complete. The encounter without the drill is the failure this
initiative exists to prevent; both landed in one context and are mounted as one surface.

## Knowledge Harvest

1. **A falsification drill is worth more than the check it names, and the reason is the second
   assertion.** "Remove the boundary and something goes red" is satisfied by a store that
   rejects everything. What makes the claim falsifiable is running the *same* append without the
   condition and asserting it is accepted — the two results read together, not either alone.
   That pairing is cheap, it fits in one file, and it is the shape any future scenario page
   should copy.
2. **A doctest's failure location is not a location.** Under `include_str!` the panic reports
   against a rustdoc temporary bundle whose path exists for one run on one machine; the only
   stable identifier is the doctest's module name, which is what one-module-per-page buys.
   Anyone writing a drill against a doc fence has to design around that, and this is the second
   corpus in this repository to pay for the lesson (`xtask/src/constitution.rs` was the first).
3. **"Empty the query" is not implementable as a falsification.** `Query::from_items([])`
   returns `Err(InvalidQuery::NoItems)`, so it goes red for a constructor reason a reader cannot
   distinguish from a build error. The one spelling that keeps the program compiling and running
   while failing exactly the refusal assertion is passing `None` where the condition went. Worth
   carrying because the project AC's own wording asks for the unimplementable one, and the
   narrowing is recorded rather than discovered twice.
