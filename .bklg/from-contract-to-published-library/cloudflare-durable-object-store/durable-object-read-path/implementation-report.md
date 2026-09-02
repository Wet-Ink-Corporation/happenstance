---
item: "HS-S0051"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — A lazy read that is still one sample: ADR-0011's ceiling-and-page

**All seven ACs are satisfied.** `read` is ADR-0011's ceiling-and-page: a position
ceiling captured no later than the first poll, every statement after the first bounded by
it, and no `SqlStorageCursor` held across a suspension point. With `render_read` and
`decode_row` landed, **the crate's last `todo!()` is gone and the scoped
`#![allow(clippy::todo)]` left with it**, which is what the story map's AC-001 row
reserved for whichever body merged last.

The verdict the spec asked for either way: **the ceiling is affordable on this runtime.**
ES-11 and ES-12 both name this adapter as the falsifier they were most at risk from
(`references/adr/0011-read-laziness-and-isolation.md:423-428`), and it does not bite.
Nothing is escalated, nothing is declined, and no promise was softened. What it costs is
recorded rather than waved at: one `max(position)` statement per `read`, and one statement
per page of `PAGE_SIZE = 128` rows — a Durable Object bills by rows read, so the number is
a trade and not a tuning knob.

Two corrections rode with it, both to prose that this diff would otherwise have left
false.

- **The crate documentation cited the wrong clause.** `src/lib.rs` framed the non-snapshot
  cursor as an open choice between honouring "ES-9's laziness requirement" and honouring
  snapshot isolation. ES-9 is `from` *names a position, not an index*; the clauses that
  carry the sample obligation are ES-11 and ES-12, and ES-11 says outright that "laziness
  is therefore permitted and never required". There was no dilemma — only a mechanism to
  implement. The bullet now says so, with the correction stated rather than the old text
  deleted.
- **`send_shape.rs`'s four probe bodies became `unreachable!`.** They are not adapter paths
  and never will be: the type is an instrument and calling it is a category error. `todo!`
  claimed someone would finish them; nobody will. Saying so is what let the scoped allow
  leave with the last *real* `todo!()` instead of outliving it there — which is what
  `publish-ready-crate` verifies rather than causes (its AC-011 and EC-006).

## TDD Evidence

Every test executes on `wasm32-unknown-unknown` under `wasm-bindgen-test-runner`, against
a real Durable Object `SqlStorage` backed by real SQLite (`src/test_object.rs`, landed by
`worker-binding-layer`). **69 tests, 0 failures** — 33 inherited, 36 new.

**Red was run twice, in two different shapes**, because a test written after the body it
covers proves nothing on its own.

**Red 1 — against the missing behaviour.** `render_read` and `decode_row` were put back to
the `todo!()` they stood at when this story opened, with the new test module already in
place. **33 of the 36 new tests failed on that panic**, each naming the fact it was asking
for. The three that passed are the ones that never reach either function and say so:
`reading_an_empty_store_yields_nothing` (the ceiling capture returns `None` first),
`read_limit_zero_yields_nothing` (nothing is executed at all) and
`an_unrepresentable_position_is_an_error_item_on_the_stream` (the ceiling capture is where
an undecodable position surfaces first). Green after restoring both bodies: 69 passed.

**Red 2 — a mutation sweep.** Five named wrong implementations were compiled into the
adapter one at a time and the suite re-run. Each was rejected, and each rejection names the
tests that caught it.

| Mutant | Rejected by |
| --- | --- |
| Paging without a ceiling — `render_read` drops `AND position <= ?`. The `RefetchingPagedStore` shape the spec names as what this story must not become | `read_is_stable_under_an_interleaved_append`, `the_ceiling_is_captured_no_later_than_the_first_poll`, `every_page_statement_carries_the_ceiling_bound`, `all_items_of_one_query_share_one_ceiling` (4 failed / 65 passed) |
| A ceiling re-captured per page rather than once per read — the "one sample per item" shape ES-12 forbids | the same three, plus `one_ceiling_is_captured_per_read_not_one_per_item` (4 failed / 65 passed) |
| `NullHeadPagingStore` — ceiling arithmetic on `head() == None`, which errors on a store whose only fault is being new | `reading_an_empty_store_yields_nothing` (1 failed / 68 passed) |
| The caller's budget never spent — `limit` ignored | `read_limit_applies_after_filtering`, `read_backwards_limit_applies_after_filtering`, `read_backwards_from_with_limit`, `limit_applies_across_items_not_per_item` (4 failed / 65 passed) |
| A read that stamps `now()` and rebuilds the `EventId` from the row's own position instead of the stored origin pair | `decode_row_returns_recorded_at_as_stored`, `decode_row_rebuilds_the_identity_from_the_stored_origin` (2 failed / 67 passed) |

| AC | Test | Red → Green |
| --- | --- | --- |
| AC-001 | `read_all_replays_every_appended_event`, `a_replayed_event_carries_everything_it_was_appended_with`, `read_reaches_the_store_only_through_new`, `read_pages_at_the_shipped_page_size` | RED on `todo!("phase 9: render the read")` in `render_read`; GREEN through `new(sql)` → `migrate()` → `append()` → `read()` |
| AC-002 | `read_is_usable_from_generic_code_binding_the_bare_port`, plus the four `!Send` probes on both targets | Compile-time: `replay_through_the_port<S: EventStore>` binds the *bare* flavour and calls `read` without `.await`. An `async fn read` refactor or a `+ Send` stops it compiling |
| AC-003 | `read_is_stable_under_an_interleaved_append`, `the_ceiling_is_captured_no_later_than_the_first_poll`, `every_page_statement_carries_the_ceiling_bound` | RED via mutants 1 and 2 |
| AC-004 | `all_items_of_one_query_share_one_ceiling`, `one_ceiling_is_captured_per_read_not_one_per_item`, `query_union_is_item_concatenation` | RED via mutant 2 |
| AC-005 | `a_live_read_stream_does_not_block_an_append_on_one_handle`, `a_read_issued_during_a_suspended_append_completes`, `two_live_reads_interleave_without_borrowing_the_object` | RED on `todo!()`; the third is the one a held cursor fails with `CursorInvalidated` and a held borrow with `AlreadyBorrowed` |
| AC-006 | the fourteen option / algebra tests, `reading_an_empty_store_yields_nothing` among them | RED via mutants 3 and 4 |
| AC-007 | `decode_row_reports_row_shape`, `::_column_type`, `::_stored_event_type`, `::_stored_position_at_the_boundary`, `the_well_formed_row_decodes`, `an_unrepresentable_position_is_an_error_item_on_the_stream`, `metadata_none_and_some_empty_stay_distinguishable`, `decode_row_returns_recorded_at_as_stored`, `decode_row_rebuilds_the_identity_from_the_stored_origin` | RED via mutant 5; `the_well_formed_row_decodes` is the control that stops the five decode-error assertions passing against a decoder that refuses everything |

## Commits

- `durable-object-read-path` — see the story checkpoint commit carrying
  `Story: cloudflare-durable-object-store/durable-object-read-path`.

## Changes

| Path | Shape of the change |
| --- | --- |
| `crates/happenstance-cloudflare/src/event_store.rs` | `read` reshaped into the ceiling-and-page state machine (`StreamState::{Deferred, Paging, Done}` carrying the ceiling, the page cursor, the page and the remaining budget); `max_position` extracted so the ceiling capture and `head` are one statement rather than two spellings; `drain_page`; `render_read`; `decode_row` and `decode_tags`; `READ_COLUMNS` and `PAGE_SIZE`; the `with_page_size` test seam; `check_cursor_still_valid` deleted; and `mod read_path_tests` |
| `crates/happenstance-cloudflare/src/test_object.rs` | The shim records every statement it is issued and exposes them as `issuedStatements`; `statements()` reads them back through the public `SqlStorage::handle` |
| `crates/happenstance-cloudflare/src/lib.rs` | The scoped `#![allow(clippy::todo)]` removed; the status block rewritten; the *capability limits* first bullet corrected from ES-9 to ES-11/ES-12 with the correction stated |
| `crates/happenstance-cloudflare/src/send_shape.rs` | Four probe bodies `todo!()` → `unreachable!`, with the reason on the comment that already explained them |
| `CHANGELOG.md` | One `[Unreleased] / Added` entry. No conformance rule was added or changed, so CF-29 owes nothing |
| `standards/rust/50-dependency-hygiene.md`, `52-wasm32-and-target-cfg.md`, `61-compile-time-assertions.md` | Seven `file:line` citations into `src/lib.rs` re-pointed after the documentation edits moved them. Forced by this diff, not scope drift: `cargo xtask lint-constitution` fails on a citation that has drifted, and leaving them would have been a silent rot |
| `.bklg/.../durable-object-read-path/_ledger.md` | Seven rows flipped with cited evidence |

## Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | green |
| `cargo clippy -p happenstance-cloudflare --all-targets --all-features -- -D warnings` | green, **without** the scoped allow |
| `cargo clippy -p happenstance-cloudflare --all-targets --target wasm32-unknown-unknown -- -D warnings` | green |
| `cargo test -p happenstance-cloudflare --target wasm32-unknown-unknown --lib` (via `wasm-bindgen-test-runner`) | **69 passed, 0 failed** |
| `cargo test -p happenstance-cloudflare` (host) | 4 unit + 2 doctests, all pass |
| `cargo xtask affected --base main` | `affected gate passed` (includes the six file-reading lints and `spec-trace`) |
| `cargo xtask lint-constitution` | `27 atoms, all consistent` |
| `rg -n "todo!\(" crates/happenstance-cloudflare/` | no matches |
| `rg -n "allow\(clippy::todo\)" crates/happenstance-cloudflare/` | no matches |

One flake, named so it is not mistaken for a finding: the first
`cargo xtask affected --base main` failed inside `happenstance-sqlite`'s
`concurrent_opens_of_one_path_all_succeed` with `DatabaseBusy` — a load-sensitive
concurrency test in an unrelated package, green on re-run
(`cargo test -p happenstance-sqlite --all-features --test projection`, 24 passed) and green
on the next full gate run. Nothing in this diff reaches that crate.

## Notes

**Why the state machine stayed hand-written, and why that is load-bearing rather than
stylistic.** A generator (`async_stream::stream!`) is the natural spelling of a
chunked-cursor read, and ADR-0011 deliberately declined an `Unpin` bound so one would stay
legal. It was not used here because a coroutine's auto traits are *inferred*, and this
crate's entire value is that its `Send`-ness is decided by its **fields**. The probe at the
crate root asserts `SqlRowStream: !Send`, and that assertion means something only while
that remains true of what the type holds. States were added; the machine was not converted.

**`check_cursor_still_valid` is gone, and its absence is the design.** It detected a torn
read *after* the fact. Ceiling-and-page prevents one: each page is `exec`-ed and drained
into memory before the poll returns, so nothing of a cursor survives to the caller's next
suspension point. `SqlError::CursorInvalidated` stays in `sql_storage` as the report for a
caller driving `exec` directly, which is now the only way to reach it.

**The ceiling bounds `position <= H` in *both* directions, and that is not a slip.** The
events a ceiling excludes are the ones appended after the sample, and those are above `H`
whichever way the read walks. A `>= H` bound under `backwards` would exclude the log
instead of the future — it reads like the symmetric thing to write and is exactly wrong.

**`position IN (…)` rather than a join is what makes "no event yielded twice" structural.**
The query's arms are `UNION`-ed in `query_sql` and `IN` is a membership test, so an event
matching three items is one row rather than three that some later `DISTINCT` has to
collapse.

**The statement log in `test_object.rs` is the only new test affordance, and it exists for
one assertion.** "One ceiling capture per read, not one per query item" is invisible in the
rows that come back — on a store nobody is writing to, both shapes return identical
results. Counting the statements the object was actually issued is the only way to see it,
and the log is read through the public `SqlStorage::handle`, not through a private field.

**`recorded_at` and the `EventId` are the two facts a read must not invent**, and each has
its own test because they fail independently: a read that stamps `now()` loses the time the
store accepted the event, and one that rebuilds the identity from this object's incarnation
plus the row's own position is correct for every locally appended event and wrong for every
ingested one — right up until replication exists, at which point it is silently wrong.

**Out of scope and untouched, as the PR boundary says:** `append`, `head`,
`contains_event_id`, `migrate` and the schema; `crates/happenstance-cloudflare/Cargo.toml`
(no dependency was added); the `Fixture`, the host and the gate step; the numeric ceilings;
every `.kb/` write and every `spec/SPECIFICATION.md` edit — ES-11 and ES-12 keep their
`[PROVISIONAL]` markers, and lifting one is ADR-0023's through `/redkiln:kb-ingest`;
`crates/happenstance-core/` in its entirety, including the two `memory.rs` read-shape tests.

## Slice-review repair (`real-worker-bindings`)

**AC-003, AC-005 and AC-006 each name a committed negative control; what shipped was a
transient mutation sweep.** The sweeps were real evidence on the afternoon they ran and
nothing re-runs them — the difference `caller-visible-error-verdict`'s AC-004 already got
right in this same crate, where the wrong shape is compiled into the test tree. All three
controls are now committed, and each is rejected by the *same* predicate the real read
passes rather than by a predicate that merely resembles it:

* **`WrongPagingStream` + `WrongCeiling::RecapturedPerPage`** — the ceiling-less paging
  read. It and the shipped read are both driven through
  `drain_across_an_interleaved_append`, so one assertion body faces both.
* **`CursorHoldingStream`** — the predecessor design `SqlRowStream`'s own documentation
  describes and rejects: one cursor opened at the first poll and advanced across poll
  boundaries. Driven through `replay_across_an_append_on_one_handle`. This one required
  **strengthening** `a_live_read_stream_does_not_block_an_append_on_one_handle`, which
  counted items: a held cursor lets the append through — it holds an `Rc`, not a `Ref` —
  and then breaks on its next advance with `SqlError::CursorInvalidated`, which an item
  count cannot see. It now asserts outcomes and completeness, so the control can fail it.
* **`WrongCeiling::ArithmeticOnHead`** — the `NullHeadPagingStore` shape ES-9 registers,
  held to the same five option sets as `reading_an_empty_store_yields_nothing` through a
  shared `empty_store_option_sets`, so a control cannot drift onto easier inputs.

Each wrong shape reuses the shipped `render_read`, `drain_page` and `decode_row` verbatim,
so what a control rejects is one decision rather than two implementations.

**And the tests are now executed by the gate.** The whole read path was compiled by the
`wasm32 build of the Cloudflare adapter` step and run by nothing in the gate: `WASM_TARGETS`
held three testkit rows and `unregistered_wasm_harnesses` scanned only the testkit's
`tests/` directory, so no check could notice. `xtask` gained a sibling registry,
`WASM_UNIT_TARGETS`, and a completeness scan over every crate's `src` tree; the
`wasm32 run of the conformance rules` step executes all 81 cases.
