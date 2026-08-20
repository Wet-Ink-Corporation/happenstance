---
item: "HS-S0154"
stage: implement
created: "2026-08-17T13:16:12.681Z"
updated: "2026-08-17T13:16:12.681Z"
---

# Implementation Report — The pointer policy, landed as in-tree substrate

## TDD Evidence

The tests were written first, against a deliberate stub: `xtask/src/pointers.rs` landed with the
row type, the four-variant enum, an empty `POINTER_REGISTER`, a module doc reading
`//! STUB — the policy is not written yet.`, and a `validate` whose whole body was
`let _ = rows; Ok(())`. That stub is what makes the Red honest in this language. A Rust test for a
type that does not exist fails to *compile*, which proves nothing; a test against a permissive
`validate` fails on the assertion it was written for.

Red run — `cargo test -p xtask --lib pointers`: **13 failed, 5 passed**. The thirteen failed with
either `this register must be rejected: ()` (the stub accepting a wrong register) or
`the policy no longer states [...]` (the pinned subject strings absent from the stub's module
doc). No failure was a compile error, an import error or a typo.

| AC | Test (`xtask/src/pointers.rs`) | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `tests::policy_states_the_dt10_resolution_its_four_gates_and_the_href_ladder` :489 | RED `the policy no longer states ["DT-10", "both, with one authoritative", "one front door, and a pointer only at a stall", "byte-identically", "evidenced stall", "provably cannot reach", "subordinate and one line", "guard from the mechanism table", "the pointer is not installed and the project escalates"]` → GREEN once the module doc at :1-124 was written |
| AC-002 | `tests::permitted_forms_are_closed_and_each_documents_its_guard` :509 | RED `BareUrl does not document what catches it rotting` → GREEN once each variant's doc named its guard (:129-163) |
| AC-002 | `tests::policy_records_the_readme_prose_gap_and_the_alias_rule` :549 | RED `the policy no longer states ["compiles the README's Rust fences", "not its prose", "doc(alias", "search key", "deleting the item deletes the alias", "needs no register row"]` → GREEN at :61-91 |
| AC-003 | `tests::the_register_lands_empty_and_valid` :566 | Green from the Red run and **stated as such**: it asserts the validator is wired to the *live* register, which a permissive stub satisfies. Its non-vacuity comes from the thirteen wrong-register tests around it, not from itself |
| AC-003 | `tests::the_mount_declares_the_module_and_displaces_nothing` :578 | The `pub mod pointers;` half is a **compile precondition** — an unmounted module runs no tests at all (EC-007), so it cannot be shown as an assertion failure. The crate-doc half **was** falsified after the fact: restoring `//! Nothing but a home for the repository README's doctests.` reproduces `the crate doc still opens with a sentence this module made false` at `xtask/src/pointers.rs:596`, and reverting the revert returns 18/18 green |
| AC-004 | `tests::a_row_records_every_fact_a_guard_claim_needs` :605 | Green from the Red run — the row type was part of the stub, because a test cannot construct a row that does not exist. What it adds beyond the type checker is the `Debug` assertion: a rejected row must print itself, not an index |
| AC-005 | `tests::rejects_a_row_whose_guard_is_empty` :627, `tests::rejects_a_row_whose_guard_is_only_whitespace` :641 | RED `this register must be rejected: ()` → GREEN with `check_guard` (:291-303) |
| AC-006 | `tests::rejects_a_bare_url_outright_and_states_the_escalation` :652 | RED, same → GREEN with `check_form` (:305-318). The test also asserts a **negative**: the message must not contain `guard: none` |
| AC-007 | `tests::rejects_link_text_that_does_not_name_its_destination` :674 | RED, same → GREEN with `check_link_text` (:320-343). 28 rejections asserted — seven denied phrases × exact / upper-cased / full-stopped / extended-past-three-words |
| AC-007 | `tests::accepts_a_three_word_noun_phrase` :705 | Green from the Red run by construction; it is the false-positive control for the rule above, and it would have gone red had the rule been written as "reject anything under four words" |
| AC-008 | `tests::rejects_a_row_that_admits_a_deep_answer_and_targets_a_page` :714 | RED, same → GREEN with `check_fragment` (:345-357) |
| AC-008 | `tests::accepts_a_deep_answer_that_targets_a_fragment` :729 | Green from the Red run; the control that stops `check_fragment` from rejecting every deep answer |
| AC-009 | `tests::rejects_duplicate_row_ids` :739 | RED, same → GREEN with `check_unique_id` (:359-370), asserting **both** surfaces are named |
| AC-009 | `tests::reports_every_problem_not_only_the_first` :754 | RED, same → GREEN; two defective rows produce two messages, in register order |
| AC-009 | `tests::every_message_names_its_row_and_states_its_reason` :766 | RED, same → GREEN; five defects, each message opening with its row id and carrying ≥ 120 characters — the `MIN_REJECTS_CHARS` discipline of `xtask/src/lint_constitution.rs` transplanted, and this project's guard against the "unstyled render" failure in a medium that has no pixels |
| AC-009 | `tests::rejects_a_ninth_row_as_the_dt10_alarm` :796 | RED, same → GREEN with `check_cap` (:372-385); asserts the message carries `the pointer policy is wrong`, that eight rows pass, and that `MAX_ROWS_EVER`'s own doc carries `scrollbar` and `policy is wrong` |
| AC-010 | `tests::policy_records_the_declined_widget_and_the_out_of_boundary_prelude` :826 | RED `the policy no longer states ["breadcrumb", "master-detail rail", "per-crate index page", "missing *link*, not a missing *widget*", "prelude", "out of this project's boundary", "owed an ADR", "not rejected"]` → GREEN at :93-108 |

**The one anti-vacuity measure worth naming.** The policy tests read this module's own source
through `include_str!`, and the module contains the tests — so a naive assertion would be satisfied
by the test's own pinned literals. `production_source()` (:459) splits the source at the test
attribute and `policy_prose()` (:466) takes only the leading `//!` block, which is the same trick
`xtask/src/lint_narrative.rs` uses to separate shipping source from test source. The pinned strings
are short subjects — `byte-identically`, `evidenced stall`, `prelude` — never paragraphs, because a
test that pins a paragraph turns ordinary rewording into a build failure and gets deleted rather
than fixed (`xtask/src/spec_trace.rs` learned that the expensive way).

## Commits

One checkpoint commit, on `initiative/docs-that-teach`, not pushed.

| SHA | Subject |
| --- | ------- |
| *`git log -1 --format=%h --grep "Story: reach-and-adapter-path/pointer-policy-and-inventory"`* | `feat(reach-and-adapter-path): The pointer policy, landed as in-tree substrate` |

The SHA cell is a lookup rather than a literal, and that is the template's own point stated as a
mechanism: **the SHA of a commit containing this file cannot be written inside this file** —
writing it changes the tree, which changes the SHA. The commit is identified here by the one thing
about it that is stable, its `Story: reach-and-adapter-path/pointer-policy-and-inventory` trailer,
and the literal belongs on the item: `redkiln record-links HS-S0154 --sha <sha>`, run before the
advance that files this report, which is what `require_commit_provenance: true` reads.

Files in that commit: `xtask/src/pointers.rs` (new), `xtask/src/lib.rs`, this story's `_ledger.md`,
`implementation-report.md` and `report.md`, plus
`.redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl`, which the pre-commit hook stages
itself. The hook ran; `--no-verify` was not used.

## Changes

| File | Shape of the change |
| --- | --- |
| `xtask/src/pointers.rs` | **New, 841 lines.** The module doc is the deliverable: DT-10's resolution, rule 1's single mirrored sentence, gates (i)–(iv), the href ladder with its escalation rung, the alias rule, the README-prose correction, the widget rejection with the `prelude` deferral, and a closing section naming what the validator cannot see. Then `PointerForm` (four variants, each documenting its own guard), `Pointer` (nine fields), `MAX_ROWS_AT_PROJECT_CLOSE = 8`, `MAX_ROWS_EVER = 20`, `POINTER_REGISTER = &[]`, and `validate` over five private rule functions. 18 unit tests. |
| `xtask/src/lib.rs` | **The mount, and nothing else.** One changed line — the crate doc's opening sentence, which the new module made false — and seven added lines at the end declaring `pub mod pointers;` with the reason it is in the lib rather than the bin. `mod constitution;` at `:28` and the `cfg(doctest)` README mount at `:21` are byte-identical and unmoved, which is invariant 2 (non-occlusion) discharged as a diff. |
| `.bklg/…/pointer-policy-and-inventory/_ledger.md` | Ten rows flipped `false → true`, each with a `file:line` citation and the passing test id. No criterion re-worded, none added, none removed; 20 changed lines for 10 rows. |
| `.bklg/…/pointer-policy-and-inventory/implementation-report.md`, `report.md` | This file and its sibling. |

Not touched, deliberately: `xtask/src/main.rs` and its `REQUIRED` list (N-4 forbids forking a
second checker into `xtask/`); `crates/happenstance/src/lib.rs`, `crates/happenstance/README.md`
and `crates/happenstance-core/src/store.rs` (owned by HS-S0158 and HS-S0156); any `.kb/` atom; any
`spec/SPECIFICATION.md` clause.

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p xtask --lib pointers` | **18 passed, 0 failed** (13 of them red first) |
| `cargo clippy -p xtask -p happenstance -p happenstance-core --all-targets --all-features -- -D warnings` | clean — `missing_docs`, `missing_debug_implementations`, `clippy::pedantic` and `unwrap_used = "deny"` all satisfied on the new module; no `#[allow]` was added anywhere |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p xtask -p happenstance -p happenstance-core --no-deps --all-features` | clean — including the new `[`pointers`]` intra-doc link in the crate doc, which `broken_intra_doc_links = "deny"` resolves |
| `cargo fmt --check` | clean, before and after the change |
| `cargo xtask affected --base main` | `affected gate passed` — the configured story-grain gate (fmt, clippy `-D warnings`, tests for the affected package set, the five file-reading lints and `spec-trace`) |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` — the project-integration bar AC-003 names, with `pub mod pointers;` declared |
| `redkiln verify --grain story --item HS-S0154` | `affected-gate` **ok**, `ledger` **ok**; `provenance` fails only because `links.commits` is empty before the checkpoint commit exists, and `boundary` fails on the initiative branch's accumulated diff from three earlier projects — see Notes |

The formatter ran last: no `--write` pass was needed, because `cargo fmt --check` was already clean
after the final edit.

## Notes

**`redkiln verify`'s boundary check fails, and it is not this story's diff.** The check diffs
against `main`, and `initiative/docs-that-teach` carries three completed projects ahead of it, so
every file those projects touched is reported as outside this story's boundary. This story's own
working-tree diff is exactly `M xtask/src/lib.rs`, `?? xtask/src/pointers.rs` and its own backlog
folder — a strict subset of the PR-boundary block in `spec.md`. The seven files the provenance
check counts *inside* the boundary are those two plus the five artefacts in this story's folder.

**Line 21 was preserved on purpose.** `spec.md` suggested keeping the crate doc honest by adding a
second sentence, which would have pushed the `#![cfg_attr(doctest, …)]` README mount from
`xtask/src/lib.rs:21` to `:22`. Four sibling specs across two other projects cite `:21` and `:28` by
line, and the ledger's own AC-003 row cites `:21`. Replacing the false first line one-for-one keeps
every one of those citations true and still satisfies the criterion as written — "its crate doc no
longer opens with a sentence the module has made false". The added declaration went to the end of
the file, below `mod narrative;`, where it moves nothing.

**One test helper changed shape between the first and second Green attempt**, and it is worth
recording because it is the kind of thing that gets quietly weakened instead. `variant_doc` first
returned the raw source slice, and `permitted_forms_are_closed_and_each_documents_its_guard` failed
on `BareUrl`: the pinned phrase *forbidden to this project outright* was true in the doc but split
across a line break. The repair was to whitespace-normalise the extracted doc — the same
normalisation `policy_prose` already did — not to shorten the pinned phrase.

**The register lands empty, and every rule is therefore exercised against registers nobody filed.**
That is `_storymap.md`'s AC-003 split-ownership decision, not a gap: each installing story files
its own row in the same change that installs its pointer. `CLAUDE.md`'s discipline holds in this
medium — every rule has a named wrong register in `#[cfg(test)]`, and the one live assertion,
`validate(POINTER_REGISTER)` is `Ok`, is what stops the validator being unwired from its own data.

**Two obligations are named rather than checked**, both by design and both in the module doc's
closing section: that a story installed a pointer and filed no row (EC-004 — `validate` sees the
register, not the diff), and that a guard someone *named* actually exists (EC-002 — `validate`
checks emptiness only, and must not pretend it can tell a real mechanism from a plausible
sentence). A check that cannot see what it claims to check would be worse than the honest absence.
