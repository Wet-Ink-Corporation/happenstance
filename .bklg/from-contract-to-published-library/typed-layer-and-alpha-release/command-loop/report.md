---
item: "HS-S0023"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — The command loop: read, decide, append, retry on ConditionViolated

## Findings Ledger

**Outcome: ten of ten ACs satisfied. Nothing deferred, nothing blocked.**

The mount point is `crates/happenstance/src/lib.rs` — the crate root, the only render
path this library has. `commit` (`:154-156`), `commit_with`, `Retry`, `Committed` and
`CommandError` (`:157`) sit beside the surviving `pub use happenstance_core::*;`
(`:160`), and region 4's bullet (`:95-98`) is an intra-doc link to `commit` rather than a
plan.

| AC | Result | What proves it | Where it is mounted |
| --- | --- | --- | --- |
| **AC-001** | satisfied | `command_loop.rs::after_anchor_comes_from_the_read` — a recording wrapper captures every submitted `AppendCondition` and asserts each attempt's `after` equals the anchor its *own* read returned (two attempts, `None` then `Some`). The wrong implementation is compiled beside it in `::threads_append_return_is_rejected` | `command.rs:294` (`after_opt(anchor)`), anchor taken at `:279` |
| **AC-002** | satisfied | `command_loop.rs::refusal_appends_nothing` — positions compared before and after by what the store actually assigned, and zero `append` calls seen | `command.rs:290`, which returns before `encode` and before the append |
| **AC-003** | satisfied | `command.rs::tests::first_try_reports_one_attempt` — drives `commit_with` against a bare `MemoryEventStore` and asserts the `attempts` the loop produced (it no longer builds a `Committed` literal and asserts its own fields back), `command_loop.rs::commit_is_commit_with_json`, `docs_composition.rs::committed_is_non_exhaustive_and_must_use` | `command.rs:77-89`; `lib.rs:157` |
| **AC-004** | satisfied | `command_loop.rs::retry_refolds_from_pristine_state` — the store is seeded with one subscription first, so the closure sees `[1, 2]` and a hoisted clone would record `[1, 3]` — and `::retry_does_not_resubmit_the_previous_batch` (attempt 2 submitted attempt 2's bytes) | `command.rs:274` — a fresh clone at the top of every iteration |
| **AC-005** | satisfied | `command_loop.rs::retries_when_conflicting_position_is_none`, `::branching_on_some_is_rejected` (the wrong predicate compiled beside it), `::exhausted_carries_the_violation` | `command.rs:305` — `is_condition_violated()` and nothing else; `:311` carries the hint on |
| **AC-006** | satisfied | `command_loop.rs::exhaustion_is_bounded_and_named` (2 and 1 appends, counted at the wrapper), `command.rs::tests::retry_has_no_default` (an inherent-vs-trait probe with a positive control), the `compile_fail` fence for `attempts(0)` | `command.rs:41-67`; `lib.rs:157` |
| **AC-007** | satisfied | `command.rs::tests::store_read_failure_chains_to_source`, `::decode_failure_names_its_position`, `::encode_failure_names_its_event_type`, `docs_composition.rs::no_string_payloads_in_command_error` | `command.rs:96-165` |
| **AC-008** | satisfied | `flavours.rs::commit_spawns_from_generic` inside a real `tokio::spawn`, **without** an `S::Error: Send` bound; `::commit_binds_the_weak_flavour` against a hand-written `!Send` store; `::the_weak_flavour_store_is_genuinely_not_send` with a positive control | `command.rs:218` and `:257` bind `EventStore`, the weaker flavour |
| **AC-009** | satisfied | `docs_composition.rs::roadmap_bullet_became_a_link`, `::no_planned_heading_survives`, `::no_item_shadows_a_core_name`, `::retry_policy_is_on_commit_itself`, `::landing_page_names_only_the_persistent_four`; `cargo doc` clean with `broken_intra_doc_links` denied | `lib.rs:95-98` and `:126-127`; the policy at `command.rs:167-210` |
| **AC-010** | satisfied | `docs_composition.rs::density_budget_holds`, `::docsrs_metadata_is_present`; `doc_budget.rs`'s seven; `src/tests.rs::the_crate_root_page_fits_above_the_fold` (35 visible, 2 hidden, both harness); three clean doc builds | `command.rs:211-212` (gate + badge); `Cargo.toml`'s docs.rs block |

**Reviewable claims a reader should check rather than take.**

1. **The `Send` test is not vacuous, and it was proved so by mutation.** Rewriting the
   retry decision as the spec's named wrong implementation — bind the error, await, then
   inspect — makes `tests/flavours.rs` fail to compile with `error[E0277]:
   `<S as SendEventStore>::Error` cannot be sent between threads safely … required by a
   bound in `tokio::spawn``. The mutation was reverted. The spawn function carries no
   `S::Error: Send` bound, so nothing in it can hide the trap.
2. **The `after` anchor is structurally unable to come from `append`.** `append`'s return
   value is bound in exactly one place (`command.rs:302`) and it goes straight into
   `Committed.position`. There is no path from it to an `AppendCondition`.
3. **`conflicting_position` appears nowhere in the crate's code.** Grep it: the only
   occurrences are in `commit`'s doc paragraph naming it as the alternative that lost,
   and in the tests. The retry decision is `is_condition_violated()` alone.
4. **`B: Boundary + Clone` is a bound the design's signature block does not spell**, and
   it is the one place this implementation is wider than the written signature. It is
   required by the design's own *Shape decision* (`DecisionModel: Clone` exists so the
   loop re-folds from the pristine model) and costs a caller nothing, because every
   `DecisionModel` is already `Clone` and tuples of `Clone` are `Clone`. The alternative
   — adding `Clone` to the sealed `Boundary` trait — would have edited M2's frozen
   surface. Worth a reviewer's eye.
5. **The vocabulary link is reference-style on purpose.** `[**The command loop**](commit)`
   is a hard rustdoc error in every build without `json`. The bullet stays one `//!` line
   and only the definition is conditional, so the page composes identically in both
   feature states. The rendered HTML is
   `<li><a href="fn.commit.html"><strong>The command loop</strong></a>`.
6. **AC-004's pristine-clone half is a falsifier, and it was not one in the first
   pass.** Seeded from an empty store, attempt 1 folds nothing, so the sound loop and
   the hoisted-clone loop both record `[0, 1]` — the assertion rejected only *"the
   loop never re-reads"*. `Contended::seed` now lands one subscription before the
   call, attempt 1 folds `1`, and the sound loop records `[1, 2]` where a hoisted
   clone double-counts to `[1, 3]`. Proved by mutation: hoisting `let mut model =
   boundary.clone();` out of the loop fails this test and **no other test in the
   file**, and the mutation was reverted. This is the same correction
   `src/tests.rs:318-331` records for `nomination_agrees_with_the_derived_query`.
7. **The unit tier's AC-003 test drives the loop rather than a struct literal.**
   `first_try_reports_one_attempt` used to build a `Committed` and assert its own
   fields back, which proves that field assignment works. It now runs `commit_with`
   against a bare `MemoryEventStore` over a module-local one-variant domain and a
   module-local `Wire` codec, and asserts the `attempts` the loop produced and a
   `position` equal to the one the store assigned. The local codec is tagged `wire`,
   not `json`, so the lib test target gained no feature requirement beyond `memory`.
8. **Nothing under `crates/happenstance-core/src/**` moved, and no new defect was found.**
   The one residual — `Event::new` returning `Result` for an already-validated
   `EventType` — is absorbed into `CommandError::Boundary` rather than `unwrap`ed, and is
   the same shape as inherited **defect candidate D-1**, still routed to AC-012's log.

**Nothing deferred.** No AC is partially met, no test is skipped or `ignore`d, and the
one instrument this story does not have — M4's public `FaultyStore<S>` — is stood in for
by a wrapper private to this crate's test target, with `happenstance-testkit` kept out of
the non-dev graph (NF-005).