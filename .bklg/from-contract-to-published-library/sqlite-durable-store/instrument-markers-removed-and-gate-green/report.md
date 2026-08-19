---
item: "HS-S0045"
stage: report
created: "2026-08-18"
updated: "2026-08-18"
---

# Report — The last todo!() and the scoped allow go together, and the gate is green

## Findings Ledger

**Six of six ACs satisfied. Nothing blocked, nothing deferred.** One criterion
carried an arithmetic error that is corrected in the evidence rather than matched
in silence, and one obligation is honestly recorded as unmachined.

| AC | Result | What proves it | Where it landed |
| --- | --- | --- | --- |
| **AC-001** — the last `todo!()` and the scoped allow leave in one commit | **satisfied** | `front_page::the_scoped_allow_is_gone_from_the_crate_root` (red on base, green now) and `::no_todo_macro_survives_in_the_crate`; the clippy step of `cargo xtask ci --fast` green *because* the exception is gone; the other six scoped allows found untouched | `crates/happenstance-sqlite/src/lib.rs:78` — `#![doc(html_no_source)]` now sits directly above the module list |
| **AC-002** — real SQL, not a synonym | **satisfied** | Execution: 90 conformance + 9 concurrency + 24 projection + 10 shapes + 19 append + 14 read + 18 wide-query passing. `front_page::no_todo_synonym_stands_in_for_work_not_done` names the one permitted `unimplemented!` with its reason, so a second is a failure and not an argument | `crates/happenstance-sqlite/tests/front_page.rs` |
| **AC-003** — the front page states what is true | **satisfied**, with a stated limit | Four machine tests (`::the_front_page_no_longer_describes_a_skeleton`, `::the_front_page_keeps_its_compiled_results`, `::the_front_page_stays_within_its_density_budget`, `::no_module_or_test_target_still_calls_this_crate_a_skeleton`) plus `migration::module_doc_schema_matches_sqlite_master` for the schema block. The *truth* of the replacement prose is a review obligation and is recorded as one | `crates/happenstance-sqlite/src/lib.rs:1-19, :45-67`; `tests/shapes.rs:1-8`; `CLAUDE.md:33` |
| **AC-004** — the specification's citations still resolve | **satisfied**, and nothing needed repairing | No citation into `happenstance-sqlite/src/lib.rs` exists anywhere in `spec/`; `git diff 53a4764 -- spec/` empty; `spec-trace`'s summary line byte-identical to base | — |
| **AC-005** — `--fast` green, and what it skipped named | **satisfied** | `all required checks passed (--fast: 4 optional step(s) not run)`, with the four named and DR-08 recorded as re-checked, not discharged | — |
| **AC-006** — publication stays someone else's decision | **satisfied** | `git diff 53a4764 -- crates/happenstance-sqlite/Cargo.toml xtask/src/package.rs` **empty**; `package-check` still reconciles exactly three crates | — |

## The half that no gate could see, and what was done about it

Every predecessor in this project made the store *work*. This story is the only
one whose increment is **legibility**, and legibility is the quantity this
repository's gate is structurally blind to. `cargo doc` under `-D warnings`
proves intra-doc links resolve. None of `xtask`'s five file-reading lints reads
this crate at all. So the crate could ship conformant, with a rendered front page
announcing *"Status: an instrument, not yet an adapter"* and *"Every operation
that touches SQL is `todo!()`"*, and every check anyone had would stay green.

The response was not to add a lint — NF-004 forbids it, and a `todo!()`-grep step
is defeated by the one-character `unimplemented!()` mutant anyway. It was to add
a **test in the crate under test**: `crates/happenstance-sqlite/tests/front_page.rs`
reads its own sources with `include_str!` and asserts on the markers that must be
gone, the compiled results that must survive their removal, and the density
budget. Three of its seven tests were red on the base tree; the other four are
regression guards written to be red for the *next* rewrite, not this one.

That closes the string-comparison half. It does not close the truth half, and
this section says so rather than letting seven green tests imply otherwise.

## What was deliberately not done

- **`publish = false`, `PUBLISHABLE` and the crate `description`.** Untouched, to
  the byte. The description still ends *"Not yet implemented."* — visibly stale,
  deliberately left, because it belongs to `crates-io-name-and-packaging-facts`
  together with the README and the licence files.
- **`RUNBOOK.md:4230`.** Its phase-8 exit checkbox reads *"`publish = false`
  removed"* and is contradicted by project AC-015, architecture brief AC-A05 and
  the initiative decomposition. It is **recorded as stale** in `_ledger.md`'s
  AC-006 row and not edited: correcting the plan of record is the runbook owner's
  act at closeout, and silently diverging from it is what AC-006 exists to
  prevent.
- **The `# Intended schema` block.** Verified object by object against migration 1
  by a test, not re-authored. EC-003 did not fire.
- **The other six scoped `#![allow(clippy::todo)]`.** Each names its own phase.
- **`references/adapter-shapes.md`** and `docs/README.md:21`'s "six skeletons"
  link text. Both record what was true when written; correcting them would
  falsify a record rather than update a status page.

## Deferred

Nothing in this story. Three items are **routed** rather than deferred, each to a
named owner:

- The crate `description`, README and both licence files →
  `crates-io-name-and-packaging-facts` (AC-015).
- Any clause whose *prose* about this crate is now wrong rather than merely
  displaced → `spec-and-code-reconciliation` (AC-016). This story raised none
  from its own edits, because its own edits moved no cited line.
- DR-08's discharge — a target-gating mistake only the wasm32 feature powerset
  would find → `benchmark-harness`. Re-checked here, not discharged here.

One item for a reviewer to challenge rather than accept: `CLAUDE.md:66-73`'s
skeleton definition still closes *"None of them has."* It remains true of the
skeletons, which is why it was left under the spec's one-row instruction — but it
is the sentence most likely to read as false to someone who has just been told
this crate passed the suite.
