---
item: "HS-S0037"
stage: report
created: "2026-08-17"
updated: "2026-08-17"
---

# Report — Atomic append and the declared ceilings

## Findings Ledger

**Ten of ten ACs satisfied.** The adapter can now be wrong about atomicity in
exactly the two ways that matter, and is not: a probe-then-insert append and a
ceiling enforced by letting SQLite refuse the row both render perfectly under
every single-threaded assertion, and both are rejected here by a test that could
have failed.

**Mount point:** `crates/happenstance-sqlite/tests/append.rs` — a new integration
target, auto-discovered by `cargo test -p happenstance-sqlite` and therefore run
by `cargo xtask affected --base main` and `cargo xtask ci --fast` with no script
edit. It is **not** retired when `tests/conformance.rs` arrives: it asks the one
question the conformance suite cannot, which is *what is in the file after a
refusal*, read through a connection the port never touched.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — the empty batch is refused first | **Met** | `append.rs::empty_batch_is_refused_before_any_condition_is_looked_at` — `append(&[], Some(&c))` with a condition a stored event *would* violate, answering `NoEvents`. Plus `::the_two_borrowed_empty_batch_rules_pass`, which drives the testkit's own `append_rejects_empty_batch` and `empty_batch_is_refused_before_the_condition_is_evaluated` **by name**, so the claim is checked against the sibling's contract rather than a local paraphrase |
| **AC-002** — the ceilings are exact at both ends | **Met** | `append.rs::each_ceiling_is_exact_at_both_ends`. For each of the three: accepted at exactly the constant — the anchor, without which a store that refused *everything* would satisfy the rest — and `ExceedsStoreLimit` with the matching `StoreLimit` and the offending `len` one unit past it. The numbers are `pub const` on the adapter (1 MiB / 128 / 256), so the fixture mirrors them rather than restating them |
| **AC-003** — a refusal costs the log nothing | **Met** | `append.rs::a_refused_append_leaves_the_file_unchanged`. All three refusal kinds against one file, with `event`, `event_tag` and `tag_cardinality` counted before and compared after **through a raw connection the store never held** |
| **AC-004** — the probe and the insert are one decision | **Met** | `append.rs::two_handles_racing_one_condition_yield_one_winner_and_one_rejection`. Two real connections onto one file; one lands; the loser is asserted `ConditionViolated` and **not** `Store` — which is the exact variant the `BEGIN DEFERRED` mutant produces instead |
| **AC-005** — guard semantics | **Met** | Four tests: `after` strictly exclusive at the boundary, `after: None` unbounded, any one violated guard refusing the whole append, and a batch whose own events match its own condition landing (ES-21) |
| **AC-006** — the caller's own last position | **Met** | `append.rs::append_returns_the_callers_own_last_position`, with a **second handle committing in between**, compared against the position actually stored for this batch's last event; and `::batch_positions_follow_slice_order`, asserting strict ascent with no literal position anywhere |
| **AC-007** — identity, stamp, covering column, cardinality | **Met** | `append.rs::every_appended_row_carries_its_identity_and_stamp` and `::tag_cardinality_is_maintained_by_append`, the latter asserting exact counts rather than merely non-zero — a table created and never updated silently restores the plan the schema amendment was made to avoid |
| **AC-008** — statements chunked, transaction not | **Met** | `append.rs::a_batch_at_the_declared_ceiling_lands_whole` — 256 events × 128 tags = **32,768** `event_tag` rows, which no single statement could bind — and `::a_failure_mid_batch_leaves_nothing`, which installs a real trigger through a second connection that raises on the fourth row and asserts (0, 0, 0) afterwards |
| **AC-009** — contention is a wait | **Met** | `append.rs::contention_waits_rather_than_erroring`: four tokio workers, four handles, **32 contended `BEGIN IMMEDIATE` transactions, all committed**, with the test panicking by name on any `Err`. No timeout, watchdog, `sleep` or retry was added around any test |
| **AC-010** — nothing beyond this adapter was decided | **Met** | `git diff --stat HEAD -- crates/happenstance-core crates/happenstance-testkit spec .kb` is **EMPTY**. `cargo test --test shapes` green unchanged; `cargo doc` and `clippy -D warnings` green inside the gate. CF-40's open question is untouched |

## The one thing outside this crate that changed

`xtask/src/spec_trace.rs` gained a single `BARE_NAME_MAP` entry, and it is a
consequence of the mount point rather than a choice. Naming the target
`tests/append.rs` made the basename ambiguous and **twenty-four
`spec/SPECIFICATION.md` citations failed at once** — the identical event
`spec_trace.rs`'s own documentation records for
`crates/happenstance-testkit/src/projection.rs`, where thirteen failed together.
Of the two answers the checker names, qualifying the citations means editing
`spec/SPECIFICATION.md`, which this story's boundary forbids. Every one of the
twenty-four names an item that exists only in `happenstance-core`, and all of
them predate a file that did not exist until today; the anchor check remains the
backstop if any of them is otherwise.

## Deferred, and to whom

- **The `BEGIN DEFERRED` negative control** belongs in
  `crates/happenstance-testkit/tests/mutation_coverage/racers.rs`'s `RACERS`, not
  in `mutants.rs`'s `REGISTRY` as `../_storymap.md` has it: a store wrong only in
  parallel fails no rule of the event-store family, and
  `mutant_registry_is_exhaustive` rejects an empty `fails` list. Owner:
  **`model-family-and-mutant-pass-column`** (HS-S0042). Nothing under
  `crates/happenstance-testkit/` was touched here.
- **CF-40's clause home** stays open, deliberately. This story needed the
  *capability* to declare numeric ceilings and got no say in which ADR owns the
  clause; `.kb/open-questions/cf-40-fixture-limits-ownership.md` is unchanged.

## What is still `todo!()`, on purpose

`read`, `head`, `contains_event_id`, `ReadCursor::fetch_page` and the projection
store's four. `#![allow(clippy::todo)]` is still at
`crates/happenstance-sqlite/src/lib.rs:83`.
