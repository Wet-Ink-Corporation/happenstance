---
item: "HS-S0037"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Implementation Report — Atomic append and the declared ceilings

> **STATUS: ten of ten ACs satisfied.** `SqliteEventStore::append` has a body,
> and the body is mostly *order*: two of its four steps decide what happens
> before any SQL runs, and the fourth decides that everything remaining happens
> inside exactly one `BEGIN IMMEDIATE`.
>
> The mount point is `crates/happenstance-sqlite/tests/append.rs` — seventeen
> tests that drive `append` through the port against a real file and then ask a
> **second raw `rusqlite::Connection`** what is actually in it. That is not a
> weaker instrument than the conformance suite; on one axis it is a stronger one.
> "Nothing of the refused value survives" is a claim about the *file*, and a
> store whose `read` is also wrong satisfies a port-only round trip perfectly.

## TDD Evidence

**RED.** `tests/append.rs` written in full — seventeen tests, the two borrowed
testkit rules, and the three ceiling constants named from the outside — against
an `append` that was still `todo!()`.

```text
running 17 tests
test result: FAILED. 1 passed; 16 failed; 0 ignored

thread 'empty_batch_is_refused_before_any_condition_is_looked_at' panicked at
crates\happenstance-sqlite\src\event_store.rs:597:9:
not yet implemented: SQLite event store: append
```

Sixteen of seventeen, every one naming the same missing behaviour. The
seventeenth is a type-level assertion that the error type is still nameable.

**GREEN.** `check_ceilings`, `append_locked`, `evaluate`, `write_batch`,
`write_tag_rows`, `bump_cardinality`, and the `query_sql` / `row` modules the
read path will share. **17 passed, 0 failed.**

| AC | Test that encodes it | Red → Green |
| --- | --- | --- |
| AC-001 | `::empty_batch_is_refused_before_any_condition_is_looked_at` + `::the_two_borrowed_empty_batch_rules_pass` | `todo!("append")` → `NoEvents` decided above everything else |
| AC-002 | `::each_ceiling_is_exact_at_both_ends` | same panic → accepted at the ceiling, `ExceedsStoreLimit { limit, len }` one unit past it, for all three |
| AC-003 | `::a_refused_append_leaves_the_file_unchanged` | same panic → three tables counted before and after through a connection the store never held |
| AC-004 | `::two_handles_racing_one_condition_yield_one_winner_and_one_rejection` | same panic → one winner, and the loser gets `ConditionViolated` rather than `Store` |
| AC-005 | `::guard_after_is_exclusive_at_the_boundary`, `::a_guard_without_after_sees_the_whole_log`, `::any_violated_guard_refuses_the_whole_batch`, `::a_batch_never_conflicts_with_itself` | same panic → `after` strictly exclusive, `None` unbounded, any guard sufficient, ES-21 held |
| AC-006 | `::append_returns_the_callers_own_last_position`, `::batch_positions_follow_slice_order` | same panic → the caller's own last position, with a second handle committing in between |
| AC-007 | `::every_appended_row_carries_its_identity_and_stamp`, `::tag_cardinality_is_maintained_by_append` | same panic → origin pair, stamp, covering column, exact tag counts |
| AC-008 | `::a_batch_at_the_declared_ceiling_lands_whole`, `::a_failure_mid_batch_leaves_nothing` | same panic → 32,768 tag rows land whole; an injected trigger fault leaves (0, 0, 0) |
| AC-009 | `::contention_waits_rather_than_erroring` | same panic → 32 contended `BEGIN IMMEDIATE` transactions, all committed |
| AC-010 | `tests/shapes.rs` unchanged, `cargo xtask affected --base main` | 10 passed, gate green |

**The mid-batch fault is a real one, not a simulated one.** `::a_failure_mid_batch_leaves_nothing`
installs an `AFTER INSERT ON event` trigger through a *second connection* that
raises once the fourth row of a batch is written, then asserts the append
answered `AppendError::Store` and that all three tables hold zero rows. That is
the shape `Fixture::MID_BATCH_FAULT` describes — nothing a caller holds can reach
between two rows of one transaction, which is precisely why the atomicity being
tested is worth having.

## Commits

`feat(sqlite-durable-store): Atomic append and the declared ceilings` — see the
`Story: sqlite-durable-store/append-atomicity-and-store-limits` trailer. The SHA
is recorded in this story's `report.md` and in the slice digest.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-sqlite/src/event_store.rs` | Three documented `pub const` ceilings; a real `append` with an `# Errors` section naming conditions; `check_ceilings` and `append_locked` beside it; and the free functions `evaluate`, `write_batch`, `write_tag_rows`, `flush_tag_rows`, `bump_cardinality`, `now`, `as_i64`, plus the parameter-budget constants |
| `crates/happenstance-sqlite/src/query_sql.rs` | **New, crate-private.** `Query` → SQL: `Selectivity` (one `tag_cardinality` lookup for the whole query, and only when a multi-tag item exists), `match_sql`, `arms_sql`, `item_sql` with the single-tag fast path and the most-selective-first intersection chain |
| `crates/happenstance-sqlite/src/row.rs` | **New, crate-private.** The encode half of the row codec: the canonical `0x1F`-delimited tag column, bracketed at both ends |
| `crates/happenstance-sqlite/src/lib.rs` | Two module declarations |
| `crates/happenstance-sqlite/tests/append.rs` | **New target.** Seventeen tests, a self-deleting `TempDb`, a minimal `AppendOnlyFixture`, and every file-level claim made through a raw connection |
| `xtask/src/spec_trace.rs` | One `BARE_NAME_MAP` entry — see *Notes* |

## Gates

| Gate | Result |
| --- | --- |
| `cargo test -p happenstance-sqlite --test append` | **17 passed**, 0 failed |
| `cargo test -p happenstance-sqlite --test migration` | **9 passed** — the predecessor still green |
| `cargo test -p happenstance-sqlite --test shapes` | **10 passed**, unchanged in intent |
| `cargo xtask affected --base main` | **PASSED** — fmt, clippy `-D warnings` over all targets and all features, the file-reading lints, `spec-trace`, and the whole test set (227 in the doc pass alone) |
| `cargo fmt --all` | clean; run as the last pass |

`clippy::pedantic` fired four `explicit_iter_loop`s and one `format_push_string`;
all five were fixed at the source, none suppressed.

## Notes

**The `BARE_NAME_MAP` entry, which is the one file outside this crate that
changed and is not scope drift.** Naming the target `tests/append.rs` — the
mount point the spec names — made the basename `append.rs` ambiguous across the
workspace, and **twenty-four `spec/SPECIFICATION.md` citations failed at once**:

```text
spec/SPECIFICATION.md:1416 — citation `append.rs:54-63` is a bare name the
workspace defines 2 times (crates/happenstance-core/src/append.rs,
crates/happenstance-sqlite/tests/append.rs). Qualify it with its path, or add it
to `BARE_NAME_MAP` with the evidence for which one is meant.
```

`spec_trace.rs`'s own documentation records the identical event when
`crates/happenstance-testkit/src/projection.rs` landed and thirteen citations
failed together, and it names the two sanctioned answers. Qualifying
twenty-four citations means editing `spec/SPECIFICATION.md`, which this story's
PR boundary forbids; adding the map entry does not. Every one of the twenty-four
names `AppendCondition`, `Guard`, `guards()`, `after`, `is_violated_by` or the
module doc's precedence paragraph — items that exist only in `happenstance-core`
— and all of them predate a file that did not exist until today. The anchor check
remains the backstop if any of them is otherwise.

**Design decisions the spec left open, taken explicitly:**

- **One `SELECT max(position)` per guard**, not a compound `EXISTS`. ADR-0022 §4
  decided it with a measurement, and the reason is that a guard asks an
  *inequality on the highest matching position* rather than an existence
  question — so one query answers both *whether* and *by which event*, and the
  rejection path needs no second statement.
- **Event rows one at a time, tag rows chunked.** Each event's position is read
  from `last_insert_rowid()` after its own insert, because inferring positions
  arithmetically is forbidden — `AUTOINCREMENT` permits gaps. The parameter
  pressure is entirely in `event_tag` (32,768 rows at the declared ceilings), so
  that is where the chunking is, sized from `PARAMETER_BUDGET / TAG_ROW_PARAMETERS`
  rather than from a literal. The buffer is bounded by the chunk, not the batch.
- **Multi-tag items take an intersection chain seeded by the most selective
  tag**, rather than `GROUP BY … HAVING COUNT(DISTINCT tag)`. ADR-0022 §8 makes
  most-selective-first a requirement and explains why a `GROUP BY` is the wrong
  shape at all: it is an optimisation barrier, so the enclosing `position > ?`
  boundary cannot be pushed through it and the probe materialises every match in
  the log before discarding the ones below the boundary. `tag_cardinality` is
  read once for the whole query, and only when a multi-tag item exists — so the
  common single-tag case costs no extra statement.
- **The ceilings are 1 MiB / 128 tags / 256 events**, all inside the corridor:
  strictly above VT-21's 65,536 bytes, VT-22's 64 tags and VT-24's 128 events,
  and small enough for the rule to allocate ceiling + 1 twice per run. They are
  `pub const` on the adapter so the fixture can *mirror* them rather than restate
  them, which is the seam a declared-here/enforced-there number would open.

**Nothing out of boundary.** `git diff --stat HEAD -- crates/happenstance-core
crates/happenstance-testkit spec .kb` is **empty**: no `StoreLimit` variant, no
item added to the contract crate, no clause or marker touched, and
`.kb/open-questions/cf-40-fixture-limits-ownership.md` unchanged — CF-40's clause
home stays open. `#![allow(clippy::todo)]` is still at `lib.rs:83`, and the only
`todo!()` removed is `append`'s: `read`, `head`, `contains_event_id`,
`fetch_page` and the projection store's four are all still marked.

**Carried forward, and it is the storymap's error rather than this story's.** The
`BEGIN DEFERRED` negative control belongs in
`crates/happenstance-testkit/tests/mutation_coverage/racers.rs`'s `RACERS`, not
in `mutants.rs`'s `REGISTRY`: a store wrong only in parallel fails **no** rule of
the event-store family, and `mutant_registry_is_exhaustive` rejects a row whose
`fails` list is empty. Discharged by `model-family-and-mutant-pass-column`
(HS-S0042). Nothing under `crates/happenstance-testkit/` was touched here.
