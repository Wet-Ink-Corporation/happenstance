---
item: "HS-S0024"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — FaultyStore<S> and GappyMemoryStore in the testkit

## Findings Ledger

**Outcome: ten of ten ACs satisfied. Nothing deferred, nothing blocked.**

The mount point is `crates/happenstance-testkit/src/lib.rs` — the testkit crate root, the
composition root the architecture brief names for exactly these types. `FaultyStore`,
`FaultyStoreError` and `SendFaultyStore` are `pub use`d at `:337`; `GappyMemoryStore` at
`:338-340`, behind `#[cfg(feature = "memory")]` and carrying the `doc_cfg` attribute that
renders its gate badge. The module doc gains one region — *Stores that misbehave on
purpose* (`:262-285`) — so a reader who lands on the crate page meets the instruments
beside the suite rather than only in a search box.

| AC | Result | What proves it | Where it is mounted |
| --- | --- | --- | --- |
| AC-001 | satisfied | `instruments_are_reachable_from_the_crate_root`; every new doctest imports by crate-root path only | `src/lib.rs:337-340` |
| AC-002 | satisfied | `injected_violation_reports_no_conflicting_position` (asserts `is_none()` explicitly) + `injected_violation_does_not_touch_the_inner_store` | `src/faulty.rs:340-343`, `:389-392` |
| AC-003 | satisfied | `retry_gated_on_some_conflicting_position_never_retries` beside `retry_gated_on_is_condition_violated_succeeds_on_the_second_attempt` | `src/faulty.rs:47-53` (the checked decrement) |
| AC-004 | satisfied | `injected_read_failure_is_the_first_polled_item_not_a_call_time_error` + `spawns_from_generic_over_send_faulty_store` | `src/faulty.rs:140-155`, `:318-330`, `:370-382` |
| AC-005 | satisfied | `event_store_conformance!(FaultyFixture::new())` — 89/89 against an **unarmed** wrapper | `tests/faulty_store_conformance.rs` |
| AC-006 | satisfied | the feature is declared and in `default`; the powerset is 6/6; the badge renders in the generated HTML | `Cargo.toml:63-64` |
| AC-007 | satisfied | `handler_assuming_position_plus_one_disagrees_with_the_store` beside `handler_resuming_past_an_inclusive_checkpoint_sees_every_event_once` | `src/gappy.rs:110-128` |
| AC-008 | satisfied | `event_store_conformance!(GappyFixture::new())` — 89/89, ES-9's interior-gap branch included; `assigned_positions_are_strictly_increasing_with_at_least_one_gap` states the relation | `tests/gappy_memory_conformance.rs` |
| AC-009 | satisfied | `injected_and_store_errors_are_distinguishable_and_the_source_chain_survives` | `src/faulty.rs:85-116` |
| AC-010 | satisfied | 20 doctests + 3 compile-fail doctests green; `cargo doc` clean under `-D warnings`; item-table summaries measured on the rendered page at 69 / 68 / 68 / 55 characters | `src/faulty.rs`, `src/gappy.rs` rustdoc |

**Error conditions.** EC-001 is asserted (`arming_zero_arms_nothing`). EC-002 is AC-002's
second test. EC-004 and EC-005 are AC-009's. EC-007 is discharged by both new fixtures
declining `REOPEN` with a real, non-empty reason that the suite prints as a skip. EC-003
(two handles firing concurrently) is **documented as unspecified** on `FaultyStore`, as the
spec requires, and bounded structurally by a `fetch_update` with a checked subtraction — no
single-threaded test is offered as evidence of a schedule. EC-006 and EC-008 are the two
residuals the spec assigned to the implementer; both are decided, stated in the rustdoc,
and recorded as DG-4 in the implementation report.

**What the reviewer should look at first.** The two design deviations that are real:
`FaultyStoreError<E>` is a fifth public item the signed-off `_design.md` item table does
not carry (**DG-1**, and the port's associated type leaves no alternative), and its error
impls are hand-written rather than `thiserror`-derived because NF-001 forbids the
`Cargo.lock` change a new dependency edge would make (**DG-2**). Both are argued in full in
`implementation-report.md`. `_design.md`'s `## Items` block wants amending by the human who
signed it.

**What was deliberately not done.** No conformance rule was added — these are fixtures, and
CF-29's changelog lint counts rules. No file under `crates/happenstance-core/src/**`,
`crates/happenstance-testkit/src/suite.rs` or
`crates/happenstance-testkit/tests/mutation_coverage/` was touched; `GappedPositionStore`
is CF-5's conformant control and stays exactly as it is, because what this story adds is
**reach** — a published, `Send`, caller-strided equivalent — rather than coverage. No
testkit version bump: CF-32's number is `publish-0-2-0-alpha-1`'s to move.

**Gate.** `cargo xtask affected --base main` reports **affected gate passed**. `cargo fmt
--all --check` is clean; `cargo hack --feature-powerset check -p happenstance-testkit` is
6/6; `cargo xtask wasm` is green on all four steps; `git status --short Cargo.lock` is
empty.

**Unblocks.** `given-when-then-dsl` (HS-S0025), the slice-mate that consumes `FaultyStore`
to drive a caller's retry loop with no database, and which carries project AC-009's other
half.
