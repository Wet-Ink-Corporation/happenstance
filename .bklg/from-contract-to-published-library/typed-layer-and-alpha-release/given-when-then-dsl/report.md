---
item: "HS-S0025"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — A given/when/then DSL that cannot hide its own filter

## Findings Ledger

**Outcome: ten of ten ACs satisfied. Nothing deferred, nothing blocked.**

The mount point is `crates/happenstance/src/lib.rs` — composition root 1, and the only
render path this library has. `pub mod testing;` is declared at `:187` behind
`all(feature = "memory", feature = "json")` with its `doc_cfg` attribute, and the module doc
gains one region, *Testing without a database* (`:125-136`), sitting between Features and the
adapter pointer where the design's composition table puts it. The link target is spelled
conditionally (`:154-161`) so it resolves with the features on and off alike.

| AC | Result | What proves it | Where it is mounted |
| --- | --- | --- | --- |
| AC-001 | satisfied | `seeded_then_decided_asserts_emitted_events`, `composed_boundary_folds_both_models` | `src/lib.rs:187` |
| AC-002 | satisfied | `four_regions_render_in_order` (byte offsets, not four `contains` calls) + `panic_location_is_the_callers_line` | `src/testing/render.rs:63-80`, `src/testing/mod.rs:292-317` |
| AC-003 | satisfied | `region_four_names_the_seeded_but_unselected_event`, oracle spelled from the seeded values (RS-60-4) | `src/testing/mod.rs:228-238` |
| AC-004 | satisfied | `empty_seed_keeps_region_four_and_says_so`, `empty_selection_is_not_an_error` | `src/testing/render.rs:106-119` |
| AC-005 | satisfied | `overflow_truncates_selected_first_and_the_diagnosis_last`, `long_event_type_wraps_without_truncating_type_or_position` | `src/testing/render.rs:14-17`, `:127-132`, `:161-194` |
| AC-006 | satisfied | `refusal_is_asserted_through_then_refused` (downcasts back to the caller's own type), `refusal_does_not_route_through_when_err`, `then_on_a_refused_decision_panics_naming_the_refusal` | `src/testing/mod.rs:222-225` |
| AC-007 | satisfied | `seeded_bytes_are_the_bytes_commit_writes`, `nominated_but_undecodable_event_is_an_error`, `non_nominated_type_is_skipped_not_an_error`, `assert_domain_event_rejects_a_variant_outside_event_types` | `src/testing/mod.rs:144-159`, `:378-393` |
| AC-008 | satisfied | `dropping_a_decision_appends_nothing`, `must_use_message_is_verbatim` | `src/testing/mod.rs:266-268` |
| AC-009 | satisfied | `retry_succeeds_after_an_injected_violation` (`attempts == 2`), `retry_refolds_from_a_pristine_model`, `retry_is_bounded` | `tests/retry_without_a_database.rs`, driving `commit_with` against `happenstance_testkit::FaultyStore` |
| AC-010 | satisfied | `testing_is_reachable_by_its_public_path`, `the_crate_root_page_points_at_the_testing_module`, `every_doc_fence_line_fits_the_column_budget`; both `cargo doc` configurations clean | `src/lib.rs:125-136`, `:154-161`, `:187` |

**Error conditions.** EC-001 is `Given::event`'s fallible signature. EC-002 is AC-007's
second test, with the deviation below. EC-003 is `non_nominated_type_is_skipped_not_an_error`
in the fold and `region_four_names_the_seeded_but_unselected_event` in the diagnosis — the
two are not in tension, and both are asserted. EC-004 routes through `when`'s `Err` as
`CommandError::Boundary` with no `unwrap` anywhere in the path. EC-005 is
`an_injected_read_failure_is_not_retried_as_contention`. EC-006 is
`then_on_a_refused_decision_panics_naming_the_refusal` — the wrong implementation, a refusal
compared against `&[]` and passing, is rejected. EC-007 is the `#[must_use]`, whose text is
asserted verbatim. EC-008 is unreachable by construction: `given` exposes no constructor
taking a caller's store, and the *absence* of that constructor is the decision.

**What the reviewer should look at first.** Three deviations, all argued in full in
`implementation-report.md`:

1. **DG-1** — the module is gated on `memory` **and** `json`, not `memory` alone. The design's
   item table names only the first; `Given::event` seeds through `Json`, so a `memory`-only
   gate leaves one powerset configuration referring to a type that does not exist. Both are in
   `default`, so nothing a `cargo add` user meets has changed.
2. **DG-2** — `Given::event` buffers and `when` performs the write, because the signed-off
   `event` signature is synchronous and `append` is not.
3. **DG-3** — `CodecError::UnknownEventType` is **unreachable** through a derived query: the
   query is built from `EVENT_TYPES`, so a nominated type the domain type does not declare
   cannot occur. The test asserts the reachable half of the same contract — a nominated
   payload the codec cannot decode, surfaced as `CommandError::Decode` rather than swallowed.

**NF-006, decided and recorded.** `crates/happenstance` takes
`happenstance-testkit = { path = "../happenstance-testkit" }` — **path-only and versionless**
— so cargo omits it from the published manifest and the two crates' release order stays free.
The alternative, `{ workspace = true }`, carries `version = "0.2.0"` and would have forced the
testkit to publish first. `publish-0-2-0-alpha-1` inherits a decision, not a surprise.

**Two commits that are not this story's.** `cargo xtask ci --fast` caught two defects in files
the slice-mate owns — a redundant intra-doc link target that only `--document-private-items`
sees, and eight `standards/rust/` citations whose anchors moved when the testkit's crate-root
doc gained a region. Both landed as their own commits carrying
`Story: typed-layer-and-alpha-release/misbehaving-testkit-stores`, which is what keeps this
story's PR boundary honest rather than quietly widening it.

**Gate.** `cargo xtask affected --base main` reports **affected gate passed**;
`cargo xtask ci --fast` reports **all required checks passed** — fmt, clippy `-D warnings`,
the whole test suite, all four `wasm32` steps, both documentation builds, `spec-trace`, the
seven file-reading lints and the packaging assertions.

**Unlocks.** Nothing declares this story as a dependency. `defect-log-and-macros-verdict`
reads AC-013's residual off `assert_domain_event` and the mapping ceremony this story makes
visible; `publish-0-2-0-alpha-1` publishes the surface it adds.
