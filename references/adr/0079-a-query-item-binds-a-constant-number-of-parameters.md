# ADR-0079 — A query item binds a constant number of parameters, and happenstance-cloudflare's widths are workerd's

- **Status:** accepted.
- **Date:** 2026-10-05
- **Phase:** 17 (the breaking window), lane L6b.
- **Decided by:** the owner, in chat on 2026-10-05, answering D1–D8 of
  `.temper/plans/p17-l6b-workerd-green.md`. This record states the answers; it takes no decision
  beyond them.
- **Answers:** ADR-0066 §6's two measurements, taken on the `workerd` job; and the first item of
  [ADR-0052](../../.kb/decisions/0052-the-query-partition-constants-stay-public.md)'s *what this
  does not decide*.
- **Supersedes:** nothing. [ADR-0023](../../.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md)
  stays accepted; §4 below narrows one of the exclusions it lists for the shim.
- **Evidence:** `experiments/durable-object-limits/results/run-workerd-local-2026-10-01.txt`,
  `run-workerd-deployed-2026-10-02.txt` and `run-workerd-local-2026-10-05.txt`; the spike scripts
  described in §7, which ran on `node:sqlite` 3.53.1 and are not in the tree.

---

## 1. What was measured

`harness/workerd/src/probe.rs` bisects each axis inside a real Durable Object and quotes the refusal
one step above the largest accepted size. At L6a (`1f92d088`):

| Axis | Deployed object | `workerd` 1.20260815.1 | `workerd` 1.20261001.1 | SQLite's default |
| --- | --- | --- | --- | --- |
| compound `SELECT` terms | 5 | 5 | 5 | 500 |
| bound parameters | 100 | 100 | 100 | 32,766 |
| statement length | 100,000 B | 100,000 B | 100,000 B | 1,000,000,000 B |
| expression depth (`1+1+…`) | 100 | 100 | 100 | 1,000 |
| one row's payload | 8,388,637 B | 2,199,995 B | 8,388,637 B | 1,000,000,000 B |
| adapter: query items, one tag each | 5 | 5 | 5 | — |
| adapter: append-condition items | 5 | 5 | 5 | — |
| adapter: tags in one query item | 45 | 45 | 45 | — |

The first four are `sqlite3_limit`s `workerd` sets on every database it opens. The adapter's
`MAX_QUERY_ARMS_PER_STATEMENT` was 400 against a wall of 5, and `MAX_QUERY_PARAMETERS_PER_STATEMENT`
was 30,000 against a wall of 100. Its rendering bound one parameter per tag and per type, and chained
one `AND position IN (SELECT … WHERE tag = ?)` per tag after the first, so it served 5 items and 45
tags. `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (VT-23) was the one rule that
failed under `workerd`, on both legs.

**Step 0, before anything was built.** The design rests on `json_each`, and a Durable Object runs an
authorizer that refuses functions it does not allow. A new probe axis bound a JSON array of *n* short
strings to `SELECT count(*) FROM json_each(?)`: `json_each elements in one parameter: accepted
100000; no wall up to 100000` under `workerd` 1.20260815.1. The authorizer allows it.

After L6b (`run-workerd-local-2026-10-05.txt`, `workerd` 1.20260815.1): query items, condition
items and tags in one item all report *no wall up to 1,024*; a new axis, `max-length tags in one
query item`, reports `accepted 8527; refused above it: … string or blob too big: SQLITE_TOOBIG`.
`npx vitest run` executes 98 tests, the 97 rules and the probe, and all pass. The deployed leg runs
only in CI and is recorded there.

## 2. The rendering (D1: option A)

Each item renders one arm, decided once from its shape by `Arm::of`. `{event}` and `{event_tag}` are
the `Tables` names, so a namespaced store renders its own tables. Bindings follow the textual order
of the `?`s.

| Shape (`Arm` variant) | Arm text | Bindings |
| --- | --- | --- |
| `Everything` (unconstructible through the builders) | `SELECT position FROM {event}` | 0 |
| `Types`: no tags | `SELECT position FROM {event} WHERE event_type IN (SELECT value FROM json_each(?))` | 1: types JSON |
| `Tag { types: None }`: one distinct tag | `SELECT position FROM {event_tag} WHERE tag = ?` | 1: the tag |
| `Tag { types: Some }` | `… WHERE tag = ? AND event_type IN (SELECT value FROM json_each(?))` | 2: tag, types JSON |
| `Tags { types: None }`: two or more | `SELECT position FROM {event_tag} WHERE tag IN (SELECT value FROM json_each(?)) GROUP BY position HAVING count(*) = ?` | 2: tags JSON, count |
| `Tags { types: Some }` | `… WHERE tag IN (…json_each(?)) AND event_type IN (…json_each(?)) GROUP BY position HAVING count(*) = ?` | 3: tags JSON, types JSON, count |

**Why `count(*) = n` is superset matching.** `event_tag`'s primary key is `(tag, position)`, so a
position carrying *k* of the item's *n* distinct tags contributes exactly *k* rows, and only
*k = n* survives the `HAVING`. Every `event_tag` row of one position carries that position's
`event_type`, the covering column `query_sql`'s module documentation explains, so a type filter keeps
all of a position's rows or none of them and the count stays exact. `Tags` deduplicates on
construction, so *n* is always reachable.

**Why one tag is its own variant.** It is the commonest DCB item and the cheapest. Its text is
byte-identical to what shipped before, so it pays no `GROUP BY` and no temporary B-tree.

**Why an enum.** The partition must price an arm by what the renderer binds. The code this replaced
had two functions, `item_sql` and `item_parameters`, and the second documented that it was a second
reading of the first and would undercount if the first changed. `Arm::parameters` and `Arm::render`
read only the variant, so there is one reading. `render` takes `self`, so each JSON string moves into
its `SqlValue` rather than being copied.

The read and guard wrappers are unchanged. `evaluate` still loops the chunks of each guard and folds
`max`; the read path still truncates after every chunk (`drain_plan`, `absorb`).

## 3. The constants (D2)

- `MAX_QUERY_ARMS_PER_STATEMENT`: **5**, was 400. The compound wall exactly, because a term count is
  exact and was measured through the adapter on both releases and the deployed object.
- `MAX_QUERY_PARAMETERS_PER_STATEMENT`: **90**, was 30,000. The read wrapper binds at most 5 more
  (`MAX_WRAPPER_BINDINGS`, pinned by `the_wrapper_binds_at_most_max_wrapper_bindings`), and
  `const _: () = assert!(MAX_QUERY_PARAMETERS_PER_STATEMENT + MAX_WRAPPER_BINDINGS <=
  WORKERD_BOUND_PARAMETERS)` holds 95 under 100 at compile time. Five arms bind at most 15, so this
  axis does not bind first today; it stays as the guard against an arm shape that binds more. The
  options were 90, 15 (5 × 3, exactly reachable) or removing it, which is a break.
- `planned_statement_count` keeps its signature and its meaning. Its values change: *n* items are
  `n.div_ceil(5)` statements. 128 one-tag items are 26, where they were 1; 1,000 are 200. Five or
  fewer items, which is every scenario model in `references/scenarios/` (the largest has 4), are
  still one. No semver tool reports a changed value, so the `0.4.0` trace table carries a hand row
  for it, and `CHANGELOG.md` a `### Changed` entry.

ADR-0066 §6 says the widths are measured in the breaking window or not promised. They were measured,
and these are the promised values.

**The append turn.** A 128-item guard is 26 statements inside `append`'s turn rather than 1. They
are synchronous `exec`s with nothing awaited between them, so the turn stays atomic in the sense the
write path needs (`evaluate`'s documentation).

## 4. The shim (D3: option a)

`DurableObjectHost`, the `pub` test surface ADR-0023 built, opens `node:sqlite` with
`limits: { compoundSelect: 5, variableNumber: 100, sqlLength: 100000, exprDepth: 100 }`, and throws
unless `db.limits` reports all four. That is fail-closed: a Node that ignored the option would leave
SQLite's defaults in place and every wall would be a guess again. `length` is left at Node's default,
because `workerd`'s length limit is 2,199,995 B on one release and 8,388,637 B on another.

The options were default-on (this), an opt-in constructor used only by the fixture and these tests,
or no shim change. Default-on closes the statement-shape part of ADR-0023's "no platform storage
ceilings" and puts VT-23-at-`workerd` into the required gate rather than only into the non-required
`workerd` job. It reverses L6a's "do not change the shim" constraint, knowingly.

It has a cost the owner accepted conditionally. Node's `limits` option exists in Node 24's
`node:sqlite` and not in Node 22's (`https://nodejs.org/docs/latest-v22.x/api/sqlite.md` has no
`limits`; the v24 page documents it). The runner images' default Node is 22 (Ubuntu 24.04: 22.23.3;
macOS 15: 22.23.2). So the gate job gains `actions/setup-node` with `node-version: "24"`, pinned to
the SHA the workflow already uses for that action, and nothing else in `.github/` changes.

Red first: `the_host_enforces_workerds_statement_limits` failed at its first assertion (a 6-term
compound accepted) before the shim change. With the shim limited and the old constants, the whole
wasm suite went red exactly where `workerd` had: VT-23's rule, and every case of
`tests/wide_query_ceiling.rs` that crossed the 5-term wall. The gate reproduced `workerd`'s failure
before the fix, which makes it this lane's regression test.

## 5. The tests that changed expectation (D4)

Each still names and rejects the wrong implementation it was written against.

- `query_sql.rs`, `no_statement_of_the_plan_exceeds_the_compound_select_ceiling`: the ceiling is
  `workerd`'s 5, not SQLite's 500.
- `query_sql.rs`, `no_statement_of_the_plan_exceeds_the_bound_parameter_ceiling`: asserts
  `bindings + MAX_WRAPPER_BINDINGS <= 100`, not `bindings <= 32,766`. Same fixture, 400 items of
  1,024 tags.
- `query_sql.rs`, `the_parameter_partition_splits_one_over_the_budget_and_not_before`: re-driven at a
  synthetic budget of 6 with arms of 2 and 3, because per-tag cost is gone and the shipped budget is
  unreachable by five arms.
- `query_sql.rs`, `the_arm_partition_still_binds_on_narrow_items`: 11 items, not 210; its own
  precondition `items < MAX_QUERY_PARAMETERS_PER_STATEMENT` would fail at 210 against 90.
- `wide_query_ceiling.rs`, `the_partition_budget_sits_inside_this_hosts_limits`: replaced by
  `the_host_enforces_workerds_statement_limits` and `a_statement_of_the_widest_chunk_is_accepted_here`.
  Its 30,000-parameter "accepted" leg is refused by design now.
- `wide_query_ceiling.rs`, `a_read_past_the_bound_parameter_ceiling_is_served_from_every_chunk`:
  renamed `a_type_wide_query_binds_one_parameter_per_item_and_is_served`, 11 items of 82 types,
  `planned_statement_count == 3`.
- `xtask/src/proof.rs`, `WIDE_QUERY_CEILING_TESTS`: the names above.
- `wide_query_ceiling.rs`, `an_append_guard_past_the_bound_parameter_ceiling_is_not_refused` and
  `a_parameter_wide_guard_answers_from_every_chunk_not_the_first`: decided on review (finding F1),
  after the first five. Kept at 5 items they planned one statement and could not fail. They now use
  the 11 type-wide items, and the every-chunk case asserts more than one planned statement. A
  first-chunk-only `evaluate` turns the every-chunk case red; an unpartitioned one turns both red.

## 6. Divergence from the sibling (D6) and the escaper (D7)

`happenstance-sqlite` keeps 400 and 30,000. Its walls are SQLite's compiled defaults, and nothing
forces a change there. ADR-0052 left open *whether the two shipping adapters are required to agree*;
they are not. Cloudflare's front page says so, and the sibling's page is not edited.

The JSON arrays are written by a 20-line escaper rather than `serde_json`, which would be a new
runtime dependency. `"` → `\"`, `\` → `\\`, U+0000–U+001F → `\u00XX` (two hex nibbles, no
`fmt::Write`), everything else raw, including non-ASCII and U+2028, which `json_each` returns
byte-exactly. Tags and event types already refuse category Cc, so the control branch is defensive;
`control_characters_encode_as_unicode_escapes` tests it directly, and
`tags_with_json_metacharacters_match_exactly` tests `"`, `\` and an NFC/NFD pair end to end.

## 7. Alternatives, with the spike's figures

The spike ran each shape on `node:sqlite` 3.53.1 under `limits: { compoundSelect: 5,
variableNumber: 100, sqlLength: 100000, exprDepth: 100 }`, over 100,000 events. Five arms of the
widest shape inside the full read wrapper bound 20 parameters, ran to 1,033 bytes and were accepted.
A 1,024-tag item matched correctly. `EXPLAIN QUERY PLAN` showed `SEARCH event_tag USING PRIMARY KEY
(tag=?)` driven by `LIST SUBQUERY … SCAN json_each`, and the `event_type_idx` covering-index search
for `Types`. These are `node:sqlite` timings, not `workerd`'s, and are indicative only; the scripts
were scratch files and are not in the tree.

| Case | Before (shipped chain) | After (this record) |
| --- | --- | --- |
| one tag | 43 µs | 49 µs |
| 2 selective tags | 26 µs | 40 µs |
| selective + hot tag | 26,521 µs | 14,473 µs |
| 3 tags including a hot one | 18,798 µs | 20,829 µs |
| 128 one-tag items | 7,109 µs (one 128-arm statement) | 3,858 µs (26 statements and the merge) |

Both shapes read every `event_tag` row of every tag in the item, so rows read, which is what a
Durable Object bills, are unchanged. Arm text no longer varies with tag or type counts, so
`workerd`'s per-text statement cache sees far fewer distinct statements.

- **B, the whole query as one JSON parameter.** `json_each` over the items, joined to `json_each`
  over each item's tags, `GROUP BY item.key, position`, plus one `Types` arm. One statement for any
  width, constant text, and it ran under the limits in the spike. It lost because it makes both
  public constants meaningless: they would be replaced by a byte budget, superseding ADR-0052's
  surface for this crate, and it gains nothing for realistic queries, which option A already makes
  one statement. It is only cheap before 1.0.
- **C, relational division seeded by one tag** (a correlated `NOT EXISTS` over `json_each`). 34 µs
  against 14,473 µs when the seed is selective, and 44,681 µs when the seed is the hot tag. Without
  a `tag_cardinality` table like the sibling's, which is a schema migration, the seed is arbitrary.
  The `GROUP BY` shape keeps today's cost profile. C is a future option.
- **Placeholders capped per item.** `IN (?,…)` for types and the chain for tags. The 45-tag
  expression-depth wall stays, and so does 100 parameters per statement. Rejected.

## 8. What still refuses

One item is the atom of the partition and is never split: its arm is an intersection over its tags,
and the caller's `UNION` and `max()` cannot recombine the halves of one. What can refuse one item is
one JSON parameter over `SQLITE_LIMIT_LENGTH`: measured at 8,527 tags of 255 bytes under `workerd`
1.20260815.1, whose length limit is 2,199,995 B, and predicted at about 32,500 on the deployed
object, whose limit is 8,388,637 B. The store accepts 1,024 tags on an event. VT-23 permits a store
to refuse wider; this is the first query refusal this adapter has that a 1,024-tag event's own item
cannot reach.

## 9. Consequences

- The workerd job's local leg runs 97 of 97 rules. The deployed leg is expected to, and is recorded
  when it does.
- The required gate now exercises the multi-statement merge through the conformance suite, because
  VT-23's rule plans 26 statements on the shim. The comments that said no gate test built more than
  one statement are corrected.
- A consumer reading `planned_statement_count` sees larger numbers for wide queries.
- Running this crate's wasm32 tests locally needs Node 24.

## 10. Falsifiers

- A real Durable Object refuses `json_each`, or refuses a 5-arm statement this adapter emits: VT-23's
  rule under the `workerd` job fails, and the probe's `json_each` axis reports `not authorized`.
- A duplicate `(tag, position)` row makes `count(*)` overshoot:
  `an_item_of_the_declared_tag_ceiling_matches_by_superset` and
  `tags_and_within_an_item_types_or_within_an_item` fail.
- The planner on a real object scans `event_tag` rather than seeking its key when `sqlite_stat1` is
  present. That is a cost regression, not a correctness one, and nothing in the tree measures it; an
  `EXPLAIN QUERY PLAN` probe in the harness would, if the authorizer allows `EXPLAIN`.

## 11. Out of scope

The vacuity control owed from L6a; CF-40's `MetadataLen` (17b); ADR-0022 §9; an observed-eviction
test for CF-14 and CF-17, whose ledger *bases* this lane rewrites without changing their
dispositions; and any change to `happenstance-sqlite`, `happenstance-core` or the testkit's rules.
