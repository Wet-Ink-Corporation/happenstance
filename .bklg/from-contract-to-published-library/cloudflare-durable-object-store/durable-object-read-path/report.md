---
item: "HS-S0051"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — A lazy read that is still one sample: ADR-0011's ceiling-and-page

## Findings Ledger

**Outcome: seven of seven ACs satisfied. Nothing blocked, nothing deferred, no conformance
rule added or changed, no `.kb/` write, and no clause text or maturity marker touched.**
ES-11 and ES-12 keep their `[PROVISIONAL]` markers; this story supplies the evidence their
named phase-9 falsifier was supposed to produce, and the marker disposition is ADR-0023's.

**The verdict, stated plainly because the spec required one either way: the ceiling is
affordable here.** ES-11's falsifier is named as *"a cursor-based adapter that cannot hold
a stable snapshot across an `await` and cannot afford the ceiling either — the Durable
Object, phase 9"*. It does not bite. Nothing is escalated, no capability is declined, and
no promise was weakened. The price is one `max(position)` statement per `read` plus one
statement per page of 128 rows, and a Durable Object bills by rows read — so the number is
recorded here rather than discovered as an invoice.

| AC | Result | Proved by | Mount point |
| --- | --- | --- | --- |
| AC-001 | satisfied | `read_path_tests::read_all_replays_every_appended_event`, `::a_replayed_event_carries_everything_it_was_appended_with`, `::read_reaches_the_store_only_through_new`, `::read_pages_at_the_shipped_page_size` | `impl EventStore for CloudflareEventStore`, reached through `CloudflareEventStore::new(sql)` |
| AC-002 | satisfied | `::read_is_usable_from_generic_code_binding_the_bare_port` (compile-time, through `replay_through_the_port<S: EventStore>`); the four `!Send` probes on host *and* `wasm32`; `send_shape`'s two doctests; `happenstance-core`'s two read-shape tests untouched and green | `EventStore::read`, still non-`async`, stream at the top level, no `+ Send` |
| AC-003 | satisfied | `::read_is_stable_under_an_interleaved_append`, `::the_ceiling_is_captured_no_later_than_the_first_poll`, `::every_page_statement_carries_the_ceiling_bound` | `SqlRowStream::poll_next`'s `Deferred → Paging` transition and `render_read`'s ceiling bound |
| AC-004 | satisfied | `::all_items_of_one_query_share_one_ceiling`, `::one_ceiling_is_captured_per_read_not_one_per_item`, `::query_union_is_item_concatenation` | one ceiling AND-ed onto the `UNION` of the query's arms in `query_sql::positions_matching` |
| AC-005 | satisfied | `::a_live_read_stream_does_not_block_an_append_on_one_handle`, `::a_read_issued_during_a_suspended_append_completes`, `::two_live_reads_interleave_without_borrowing_the_object` | `drain_page`, which kills the cursor before the poll returns |
| AC-006 | satisfied | the fourteen option and algebra tests, `::reading_an_empty_store_yields_nothing` among them | `render_read`'s four composed bounds and `query_sql` |
| AC-007 | satisfied | the five decode assertions plus `::the_well_formed_row_decodes` as their control; `::an_unrepresentable_position_is_an_error_item_on_the_stream`; `::metadata_none_and_some_empty_stay_distinguishable`; `::decode_row_returns_recorded_at_as_stored`; `::decode_row_rebuilds_the_identity_from_the_stored_origin` | `decode_row` and the shared `decode_position` the write path landed |

**How the review should read the negative controls.** Three of the seven criteria describe
behaviour that a wrong implementation satisfies structurally, so each carries a rejection
rather than only a confirmation. Five named wrong implementations were compiled in one at a
time and the suite re-run; every one was rejected, and the implementation report names which
tests caught which. Two of those detectors are **committed**, not sweep-only:
`every_page_statement_carries_the_ceiling_bound` and
`one_ceiling_is_captured_per_read_not_one_per_item` read the statements the object was
actually issued, so a future diff that drops the ceiling or re-captures it per item fails on
the next run rather than on the next sweep.

**What this story finished for the project.** With `render_read` and `decode_row` landed the
crate has no `todo!()` left anywhere and the scoped `#![allow(clippy::todo)]` is gone —
project **AC-001** in full and project **DoD 3** as a grep. The story map's AC-001 row
assigned that line to whichever body merged last, and this is it. `publish-ready-crate`
verifies both greps rather than causing them (its AC-011, EC-006), so a survivor there would
have meant this story did not finish; there is none.

**What is explicitly *not* claimed.** This adapter has still not run
`happenstance_testkit::event_store_conformance!` on `workerd`. Twelve read-family rules
observe the facts above and none of them has executed against this store — they run once
`durable-object-host-and-fixture` and `every-rule-under-workerd` land. Until then this is an
implementation, not a conformant adapter, and the crate documentation now says so in those
words.

**Deferred, with the owner named.** ES-11's and ES-12's `[PROVISIONAL]` markers (ADR-0023,
via `adr-0023-and-atom-resolutions`); the 2^53 ceiling as a *declared store limit* and its
measurement (`measured-store-limits`); `check_cursor_still_valid`'s clause-level
consequences, if any, which are ADR-0023's to record. Nothing here was made to look settled
that is not.

**One flake, named so a reviewer does not read it as a finding.** The first
`cargo xtask affected --base main` failed inside `happenstance-sqlite`'s
`concurrent_opens_of_one_path_all_succeed` with `DatabaseBusy`. It is a load-sensitive
concurrency test in a package this diff does not reach; it passed on re-run and on the next
full gate run.
