# The census

Written by hand from `results/raw/census.txt`, because a table nobody read is a
table nobody checked. Every figure below appears in that file; the `SUBJECT` and
`FAIL` lines are the source rows.

Conditions: `results/raw/conditions.txt`. Repository at
`56ef6c5ddc3476a9a896dfe1824f4c46de33a3ed`, `happenstance-testkit`
0.2.0-alpha.1, `rustc 1.97.1`, census built `--release`.

## The rule set

**89 rules** in the event-store family, enumerated by
`happenstance_testkit::for_each_event_store_rule!` — the one place the set is
written down, and the same one `event_store_conformance!` expands. The review's
findings say "90"; the enumeration at this commit holds 89, and the difference is
noted rather than reconciled because nothing in the argument turns on it.

## The answer

| Subject | Kind | Passed | Skipped | **Failed** |
|---|---|---:|---:|---:|
| `MemoryEventStore` (reference) | control | 83 | 6 | **0** |
| `LogStore` (correct core) | control | 83 | 6 | **0** |
| `ForwardPagingBudgetStore` | **mutant (L1-1)** | 83 | 6 | **0** |
| `NoopReopenFixture` | **mutant (L1-2)** | 86 | 3 | **0** |
| `SwallowedReadFaultStore` | **mutant (L3-01)** | 83 | 6 | **0** |
| `StagedCommitStore` | **mutant (F2-5)** | 83 | 6 | **0** |
| `ForwardPagingBudgetStore + InnerJoin` | injected control | 62 | 6 | 21 |
| `NoopReopenFixture + InnerJoin` | injected control | 64 | 3 | 22 |
| `SwallowedReadFaultStore + InnerJoin` | injected control | 62 | 6 | 21 |
| `StagedCommitStore + InnerJoin` | injected control | 62 | 6 | 21 |
| `SwallowedReadFaultStore` (fault armed by hand) | diagnostic | 61 | 6 | 22 |

> **ANSWER: 4 of 4 wrong implementations fail NOTHING.**

## The controls held, in both directions

**Direction two.** Both conformant controls failed nothing. `MemoryEventStore` is
the testkit's *published* reference fixture, registered unchanged, so the
baseline is not this crate's own opinion of what correct looks like.

**Direction one.** Every injected arm failed 21 or 22 rules, so every one of the
four was genuinely driven: the fixture was constructed, `connect` handed back a
live store, and the rules read through it. `cargo test` would have gone red on any
arm that failed none, and the number would have been discarded.

The 21 rules common to all four injected arms:

```
two_fixture_instances_observe_none_of_each_others_appends
query_all_matches_every_event            query_item_types_are_or
query_items_are_or                       untagged_events_match_query_all
duplicate_items_do_not_duplicate_events  query_item_order_does_not_change_the_result_set
read_from_is_inclusive                   read_backwards_reverses_order
read_limit_truncates                     read_backwards_from_with_limit
read_defaults_to_ascending_order         read_limit_applies_after_filtering
read_backwards_limit_applies_after_filtering
read_from_composes_with_multi_item_query positions_are_strictly_monotonic
head_is_the_highest_visible_position     append_returns_last_written_position
append_is_atomic                         condition_rejection_leaves_store_unchanged
a_live_read_stream_does_not_block_an_append
```

Every one of the 88 failures across the four injected arms was classified as
*the rule rejected the store* — the panic's `location()` is inside
`happenstance-testkit/src/suite.rs`. Not one was a store falling over in this
crate's own code, which is the check that separates "the rule caught it" from
"the instrument broke".

## The one row that differs, and it is the L1-2 evidence

`NoopReopenFixture` **skipped 3 rules where every other subject skipped 6**. The
three it did not skip are the ones a declined `REOPEN` would have skipped:

```
acknowledged_writes_survive_a_reopen
reopened_store_does_not_reissue_an_event_id
recorded_time_survives_a_reopen
```

They **ran**, against a store whose entire durable medium is a `Vec` behind an
`Rc` and whose `reopen` is an empty body, and all three **passed**. That is the
whole of the suite's durability certification, executed and green, over nothing.

Its injected arm confirms the three genuinely read through the store rather than
being no-ops: `NoopReopenFixture + InnerJoin` fails
`acknowledged_writes_survive_a_reopen`, which is the twenty-second rule in its
list and the one the other three injected arms skip.

## The diagnostic row

`SwallowedReadFaultStore (fault armed by hand)` is the same store with the fault
`Some(1)` at `open` rather than never armed. It fails **22** rules, including
`query_all_matches_every_event`, `read_from_is_inclusive`,
`event_ids_are_unique_within_a_store` and
`store_accepts_the_guaranteed_minimum_batch_size` — every one of them a
consequence of a read that came back short and said it had succeeded.

So the suite is **not** blind to the consequence. It has no way to produce it:
`Fixture` carries `MID_BATCH_FAULT` / `arm_mid_batch_fault` for the *write* path
and no read-path analogue, so `arm_read_fault` in this crate is an inherent
method on the fixture that no conformance rule can call. That distinction is the
whole of L3-01, and this row is what makes it a measurement instead of a claim.
