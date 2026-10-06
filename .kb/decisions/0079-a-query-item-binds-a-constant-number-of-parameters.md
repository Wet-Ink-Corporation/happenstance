---
id: kb-decision-0079
title: A query item binds a constant number of parameters, and happenstance-cloudflare's widths are workerd's
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0079
reversibility: medium
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Phase 17's workerd job measured four statement limits that workerd sets on every Durable Object
  database: 5 compound SELECT terms, 100 bound parameters, 100,000-byte statements and an
  expression depth of 100. happenstance-cloudflare partitioned at SQLite's defaults (400 arms,
  30,000 parameters) and bound one parameter per tag and per type, so on a real object it served
  at most 5 query items, against VT-23's floor of 128, and at most 45 tags in one item. This
  record, on the owner's answers of 2026-10-05, changes the adapter only. Each item renders one arm
  whose tags and types each travel as one JSON array unpacked by json_each(?), so an arm binds 0 to
  3 parameters by its shape and its text does not grow with its width; two or more tags match by
  GROUP BY position HAVING count(*) = n, exact because event_tag's key is (tag, position). One enum
  prices and renders each arm, so the partition cannot drift from the renderer.
  MAX_QUERY_ARMS_PER_STATEMENT is 5 and MAX_QUERY_PARAMETERS_PER_STATEMENT is 90, held under 100
  with the read wrapper's 5 by a compile-time assertion. planned_statement_count keeps its meaning
  and returns new values (128 one-tag items are 26 statements). The test shim opens node:sqlite
  with the same four limits and refuses to open without them, so the gate refuses what workerd
  refuses; that needs Node 24 on the gate's runners. The two adapters' widths need not agree, which
  answers the first thing ADR-0052 did not decide. The per-item wall is now SQLite's length limit
  on one JSON parameter, 8,527 tags of 255 bytes under workerd 1.20260815.1. The JSON is
  hand-escaped; no dependency is added. The whole query as one JSON parameter, and a division
  shape seeded by one tag, were considered and not taken. Supersedes nothing.
depends_on:
  - kb-decision-0052
  - kb-decision-0066
related:
  - kb-decision-0052
  - kb-decision-0066
  - kb-decision-0023
  - kb-decision-0022
  - kb-open-question-workerd-runner-absent-001
source_paths:
  - crates/happenstance-cloudflare/src/query_sql.rs
  - crates/happenstance-cloudflare/src/event_store.rs
  - crates/happenstance-cloudflare/src/host.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - crates/happenstance-cloudflare/tests/wide_query_ceiling.rs
  - harness/workerd/src/probe.rs
  - experiments/durable-object-limits/README.md
  - experiments/durable-object-limits/results/run-workerd-local-2026-10-05.txt
  - .github/workflows/ci.yml
  - references/adr/0079-a-query-item-binds-a-constant-number-of-parameters.md
last_reviewed: 2026-10-05
---

# A query item binds a constant number of parameters, and happenstance-cloudflare's widths are workerd's

The full record, with the measurements, the shape table, the alternatives and their figures, is
[`references/adr/0079-a-query-item-binds-a-constant-number-of-parameters.md`](../../references/adr/0079-a-query-item-binds-a-constant-number-of-parameters.md).

## The question

ADR-0066 §6 put `happenstance-cloudflare`'s 1.0 claim on the real runtime and named two
measurements to take before promising them: the SQL-text wall, and ADR-0052's partition widths.
Phase 17's `workerd` job took them (`experiments/durable-object-limits/README.md`). `workerd` sets
`SQLITE_LIMIT_COMPOUND_SELECT` to 5, `SQLITE_LIMIT_VARIABLE_NUMBER` to 100,
`SQLITE_LIMIT_SQL_LENGTH` to 100,000 and `SQLITE_LIMIT_EXPR_DEPTH` to 100, the same on two releases
and on a deployed object. The adapter partitioned at 400 arms and 30,000 parameters, which are
SQLite's defaults with headroom, and bound one parameter per tag and per type. On a real Durable
Object it therefore served at most 5 query items, and VT-23's rule failed. It served at most 45
tags in one item, because the `AND position IN (…)` chain nested once per tag.

## Decision

The owner answered eight questions on 2026-10-05 (the plan's D1–D8). This record states them.

1. **Each item renders one arm that binds a constant number of parameters (D1).** An item's types
   travel as one JSON array, `event_type IN (SELECT value FROM json_each(?))`. One distinct tag is
   `tag = ?`, the text it always had. Two or more are one JSON array,
   `tag IN (SELECT value FROM json_each(?)) … GROUP BY position HAVING count(*) = ?`. That is
   superset matching because `event_tag`'s primary key is `(tag, position)`: a position carrying
   *k* of the item's *n* tags contributes exactly *k* rows. Every `event_tag` row of a position
   carries its `event_type`, so a type filter keeps all of a position's rows or none. An arm binds
   0, 1, 2 or 3 parameters, and its text is independent of its tag and type counts, so neither the
   statement length nor the expression depth grows with the query. Arms are still joined by
   `UNION`, chunked, and merged by the caller, as before.
2. **One enum prices and renders an arm.** `Arm::of(item)` decides the shape once;
   `Arm::parameters` and `Arm::render` both read only the variant. The partition can no longer
   undercount what the renderer binds, the drift the deleted `item_parameters` documented.
3. **`MAX_QUERY_ARMS_PER_STATEMENT` is 5 and `MAX_QUERY_PARAMETERS_PER_STATEMENT` is 90 (D2).** 5
   is the compound wall exactly, because a term count is exact. 90 plus the read wrapper's 5
   bindings (the ceiling, `from`, `to`, the cursor and `LIMIT`) is 95, under 100; a `const`
   assertion beside the wrapper holds it there. Five arms bind at most 15, so the arm axis binds
   first; the parameter axis stays as the guard against a future arm that binds more.
   `planned_statement_count` keeps its signature and meaning: a query of *n* items is
   `n.div_ceil(5)` statements, so 128 one-tag items are 26 where they were 1. That changed value is
   invisible to a semver tool, so the `0.4.0` trace table carries a hand row and the CHANGELOG an
   entry. A 128-item guard is 26 synchronous statements inside the append turn, with nothing
   awaited, so the turn stays atomic.
4. **The test shim enforces the same four limits, by default (D3).** `DurableObjectHost` opens
   `node:sqlite` with `limits: { compoundSelect: 5, variableNumber: 100, sqlLength: 100000,
   exprDepth: 100 }` and throws unless `db.limits` reports all four. This narrows ADR-0023's "no
   platform storage ceilings" for statement shape, and puts VT-23-at-`workerd` into the required
   gate. `SQLITE_LIMIT_LENGTH` is left at Node's default, because `workerd`'s differs by release and
   by deployment. Node 22's `node:sqlite` has no `limits`, and every runner image's default Node is
   22, so the gate job gains `actions/setup-node` at `"24"`, the one CI edit the owner allowed.
5. **The rewritten tests are approved (D4).** Four host cases in `query_sql.rs`, two cases in
   `tests/wide_query_ceiling.rs` and `xtask/src/proof.rs`'s list. Each still names the wrong
   implementation it was written against; the record lists them.
6. **This is ADR-0079, atom and long form (D5).**
7. **The adapters' widths need not agree (D6).** `happenstance-sqlite` keeps 400 and 30,000,
   SQLite's compiled defaults. This answers the first item under ADR-0052's *what this does not
   decide*: the two adapters are not required to agree, and Cloudflare's front page says so.
8. **The JSON is hand-escaped (D7).** `"` and `\` are escaped, U+0000–U+001F become `\u00XX`, and
   everything else passes raw. `Tag::new` and `EventType::new` already refuse category Cc, so that
   branch is defensive and tested directly. No `serde_json`.
9. **The ledger bases of WF-11, CF-14 and CF-17 are rewritten as drafted (D8).** Dispositions are
   unchanged.

## What still refuses

One item is still the atom of the partition and is never split. What can refuse one item is now
one JSON parameter longer than `SQLITE_LIMIT_LENGTH`: 8,527 tags of the maximum 255 bytes under
`workerd` 1.20260815.1 (measured, `results/run-workerd-local-2026-10-05.txt`), and about 32,500 on
a deployed object (predicted from its 8,388,637-byte row wall). The store accepts 1,024 tags on an
event, and nothing in the contract bounds tags per query item. This replaces the 45-tag
expression-depth wall.

## Considered and not taken

- **The whole query as one JSON parameter**: one statement for any width, constant text. It makes
  both public constants meaningless and would replace them with a byte budget, superseding
  ADR-0052's surface for this crate. Realistic queries are already one statement under the
  decision above. Cheap only before 1.0.
- **A relational-division shape seeded by one tag**: far faster when the seed is selective, about
  three times slower than the `GROUP BY` shape when the seed is a hot tag. Choosing a safe seed
  needs a tag-cardinality table, which is a schema migration. Recorded as a future option.
- **Placeholders capped per item**: keeps the 45-tag wall and the 100-parameter wall per statement.

## Falsifiers

- A real Durable Object refuses `json_each` (its authorizer), or a deployed object refuses a
  statement this adapter emits at 5 arms: the `workerd` job's VT-23 rule and its probe fail.
- A position contributing more `event_tag` rows than its distinct tags (a duplicate
  `(tag, position)`): `an_item_of_the_declared_tag_ceiling_matches_by_superset` fails.
- The planner on a real object scanning `event_tag` instead of seeking its key is a cost regression,
  not a correctness one, and nothing in the tree measures it yet.

## What is not decided

The deployed object's length wall is predicted, not measured, until the `workerd` job's first green
run fills `experiments/durable-object-limits/README.md`'s deployed cells. The vacuity control owed
from L6a, CF-40's `MetadataLen` (17b), ADR-0022 §9, and an observed-eviction test for CF-14 and
CF-17 are out of scope.
