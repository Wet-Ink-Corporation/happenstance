---
item: "HS-S0045"
stage: implement
created: "2026-08-18"
updated: "2026-08-18"
---

# Implementation Report — The last todo!() and the scoped allow go together, and the gate is green

> **STATUS: six of six ACs satisfied.** The survey the spec asks for first came
> back empty: no `todo!()` macro survived on any SQLite path, so `head` and
> `contains_event_id` were never this story's to write. What remained was the
> half that survives every green run — the scoped `#![allow(clippy::todo)]` and
> a front page that told docs.rs the crate was a skeleton.
>
> The story's one durable addition is a **machine for the half that had none**.
> `crates/happenstance-sqlite/tests/front_page.rs` reads the crate's own sources
> with `include_str!` and fails on a marker that outlives what it marked. Three
> of its seven tests were red on the base tree.

## TDD Evidence

| AC | Test | Red, and for what reason | Green |
| --- | --- | --- | --- |
| **AC-001** | `front_page::the_scoped_allow_is_gone_from_the_crate_root` | Red on base 53a4764: *"src/lib.rs still carries a scoped `#![allow(clippy::todo)]`; the attribute and the last `todo!()` leave together"* — the assertion, not a compile error | green once `lib.rs:79-83` was deleted |
| **AC-001** | `front_page::no_todo_macro_survives_in_the_crate` | **Green on base, and that is the finding, not a gap.** It scans code lines (comments excluded) across all six modules; the six `todo!` hits `grep` finds are every one of them prose. It stands as the regression guard the deleted `#![allow]` used to be | green |
| **AC-002** | `front_page::no_todo_synonym_stands_in_for_work_not_done` | Green on base, with the one permitted `unimplemented!` named and reasoned in the test itself. It is red the moment a **second** one appears — which is the mutant the spec's context pack 3 names, and the one `-D warnings` cannot see because `clippy::unimplemented` is not in the workspace table | green |
| **AC-002** | the four suites: `conformance.rs`, `concurrency.rs`, `projection.rs`, `shapes.rs` | Not red here — they are the *predecessors'* red-then-green, inherited. Cited because AC-002's evidence is execution and never the grep | 90 / 9 / 24 / 10 passed |
| **AC-003** | `front_page::the_front_page_no_longer_describes_a_skeleton` | Red on base: *"src/lib.rs still says \"an instrument, not yet an adapter\""* — and it carries five more phrases, each a sentence the rewrite had to remove rather than soften | green |
| **AC-003** | `front_page::the_front_page_keeps_its_compiled_results` | Green on base and **written to stay green**: it is the non-occlusion guard, and it is the assertion that would have caught the tidy-up that deletes `error[E0195]` along with the status prose | green |
| **AC-003** | `front_page::the_front_page_stays_within_its_density_budget` | Green on base at 76 `//!` lines / 5 headings; the budget is 54–94 and five. It is red for a rewrite that grows the front page into an essay or shrinks it to a sentence | green at 78 / 5 |
| **AC-003** | `front_page::no_module_or_test_target_still_calls_this_crate_a_skeleton` | Red on base: *"tests/shapes.rs still says \"every body is `todo!()` in the crate under test\""* — the marker most easily missed, because it is not in `src/` | green |
| **AC-003** | `migration::module_doc_schema_matches_sqlite_master` | Not this story's test. Run deliberately as the *verification* the spec demands: the `# Intended schema` block compared object by object against `sqlite_master` after a real `migrate`. EC-003 did not fire | green (9 tests) |
| **AC-004** | `cargo xtask spec-trace` | Would go red on a citation this PR's edits moved. `git diff 53a4764 -- spec/` is empty and the summary line is byte-identical to base | `389 citations checked (76 anchored, 12 external)` |
| **AC-005** | `cargo xtask ci --fast` | The whole `REQUIRED` list on the tree as this story leaves it | `all required checks passed (--fast: 4 optional step(s) not run)` |
| **AC-006** | `git diff 53a4764 -- crates/happenstance-sqlite/Cargo.toml xtask/src/package.rs` | Empty, and `package-check` still reconciles exactly three crates | green |

**The honest limit, stated here rather than buried.** None of the above proves
the *truth* of the replacement prose. `cargo doc` under `RUSTDOCFLAGS=-D warnings`
proves that links resolve; nothing in this repository reads documentation for
truth. `front_page.rs` closes the half a string comparison can close — the
markers that must be gone and the compiled results that must survive — and the
other half is a review obligation recorded as one in `_ledger.md`'s AC-003 row.

## Commits

- `4f92c42` — `feat(sqlite-durable-store): The instrument markers come out and the gate is green`

One commit, which is the point rather than a convenience: `#![allow]` is silent
in both directions, so a split that leaves the attribute behind in a second
commit is the shape in which the second commit does not happen. Here there was
no last body to pair it with — the survey found none — so the recorded commit
pairs the attribute with the prose and the guard that replaces it.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-sqlite/src/lib.rs` | The `#![allow(clippy::todo)]` at `:83` and its four-line justification at `:79-82` **deleted**. The front page rewritten in one pass: the status banner (`:3`), the "every operation that touches SQL is `todo!()`" paragraph (`:5-14`), the `publish = false` sentence restated as the true and different claim (`:16-19`), and *Open decisions* → *What is settled, and what is still open* (`:45-67`), closed against ADR-0022 and narrowed to the two subjects it left open, each cited by atom name. `# The shape this crate represents`, `# What the type checker has already decided` and `# Not the Cloudflare adapter` carried across unchanged in substance |
| `crates/happenstance-sqlite/tests/front_page.rs` | **New.** Seven tests over `include_str!`'d sources. No database, no query — it is the crate's own prose read as an artefact |
| `crates/happenstance-sqlite/tests/shapes.rs` | `:1-8`'s module doc. It said the target runs nothing because every body is `todo!()`; it now says what a type-level target buys that a green suite does not |
| `CLAUDE.md` | One row of the repository map, `:33`. `🔩 skeleton. event store + projection store.` → `the first adapter. event store + projection store.` The 🔩 definition paragraph and the other five skeleton rows are untouched |

Not touched, deliberately: `crates/happenstance-sqlite/Cargo.toml` (AC-006),
`xtask/src/package.rs` (AC-006), `spec/**` (nothing moved), `RUNBOOK.md` (its
stale checkbox is recorded, not corrected), and the other six scoped
`#![allow(clippy::todo)]`.

## Gates

| Gate | Grain | Result |
| --- | --- | --- |
| `cargo test -p happenstance-sqlite --test front_page` | inner loop | 7 passed (3 red before the change) |
| `cargo test -p happenstance-sqlite` | story | 19 + 9 + 90 + 7 + 9 + 14 + 10 + 18 passed, 0 failed |
| `cargo test -p happenstance-sqlite --all-features --test projection` | story | 24 passed |
| `cargo xtask affected --base main` | story (`affected_gate`) | `affected gate passed` |
| `cargo xtask spec-trace` | story (`reachability_static`) | `no problems found`; summary identical to base |
| `cargo fmt --all -- --check` | formatter | clean |
| `cargo xtask ci --fast` | integration (`integration_scoped`) | `all required checks passed (--fast: 4 optional step(s) not run)` |

**What `--fast` did not run**, by name and not by implication: `feature
powerset`, `wasm32 feature powerset`, `licences and advisories` (`cargo deny`)
and `docs.rs configuration (nightly)`. Outside it entirely: the CI `msrv` job at
a pinned 1.97.1, and `cargo xtask ci` whole. **DR-08 is re-checked here and not
discharged here**: the `bench` feature's target gating is exercised by the
mandatory wasm32 steps, but the wasm32 *feature powerset* is one of the four that
did not run.

## Notes

**Deviation 1 — the residual was empty, so this is a marker-and-prose PR rather
than an implementation PR.** The spec named this story the fallback owner of
`head` and `contains_event_id` and warned that if they arrived the diff would be
large. They did not: `sqlite-fixture-and-whole-suite` closed both. The survey was
run first and is recorded in the AC-001 and AC-002 rows, with the predecessor
credited, rather than the row being closed on a grep that would have looked the
same either way.

**Deviation 2 — the criterion says four wasm32 steps and `REQUIRED` carries
five.** The typed-layer check (`xtask/src/main.rs:319`) was added after this spec
was written. All five ran green, so the criterion is met with room; the count is
corrected in the AC-005 evidence rather than quietly matched.

**Deviation 3 — a test target was added, which the spec called optional.** The
implementation notes floated "earning a machine for the front page" as a doctest
and said to drop it if it cost a dependency or a feature. `front_page.rs` costs
neither: it is `include_str!` and `assert!`. It is deliberately **not** an
`xtask` lint and not a gate step (NF-004): it is a test in the crate under test,
it is not added to `xtask/src/proof.rs`'s `ARTEFACTS`, and it reads no file
outside the package directory — so `cargo package -p happenstance-sqlite` is
unaffected, which the next story depends on.

**Not done, and it is a judgement worth challenging.** `CLAUDE.md:66-73`'s 🔩
definition ends *"None of them has."* — of the skeletons, still true, because
this crate is no longer one. The sentence was left alone under the spec's "one
row changes" instruction. A reviewer who reads it as counting *adapters* rather
than *skeletons* should raise it.
