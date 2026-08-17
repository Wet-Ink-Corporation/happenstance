---
item: HS-P0012
stage: implementation
created: 2026-08-17T00:53:30.326Z
updated: 2026-08-17T00:53:30.326Z
template_sig: 4c5f37d6
rendered_sig: e7b82b44
---

# Slice ledger — The first adapter that is not an instrument

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| bench-harness-and-adr | approved | benchmark-harness 2665883, adr-0022-append-condition-strategy 791b929 | (this commit) |
| durable-event-store | changes-requested | schema-migration-and-identity 8381c89, append-atomicity-and-store-limits 23bc776, lazy-read-with-snapshot-ceiling 0c6ce2b, wide-query-chunked-not-refused 11596b4, sqlite-fixture-and-whole-suite 41a2064 | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### durable-event-store

- **Issue:** BLOCKING — the slice leaves cargo xtask ci --fast red. The docs step fails: `` public documentation for `event_store` links to private item `SqliteEventStore::migrate` `` at crates/happenstance-sqlite/src/event_store.rs:39, under RUSTDOCFLAGS=-D warnings (xtask/src/main.rs:344-351). Introduced by 8381c89; base main did not carry the link. cargo xtask affected cannot catch it because it deliberately excludes the documentation build (xtask/src/affected.rs:41-42). This breaks sqlite-fixture-and-whole-suite/spec.md:431 NF-006, lazy-read-with-snapshot-ceiling/spec.md:616 and :682, schema-migration-and-identity/spec.md:548, and project.md DoD 5.
  **Fix:** At crates/happenstance-sqlite/src/event_store.rs:39, drop the intra-doc link on the private fn — either plain code formatting (`SqliteEventStore::migrate`) or point the reader at the public [`SqliteEventStore::open`], which is what actually applies it. Then re-run `cargo xtask ci --fast` end to end and record the result, not just `affected`.

- **Issue:** .bklg/from-contract-to-published-library/sqlite-durable-store/schema-migration-and-identity/_ledger.md AC-008 is flipped satisfied:true with evidence ending 'The `docs` step of cargo xtask ci --fast builds the rustdoc'. That step is precisely what now fails, and it fails on the very block AC-008 delivers. The ledger is the DoD-4 evidence trail, so a row whose closing claim is false is worse than a missing one.
  **Fix:** After fixing the doc link, re-run the docs step and replace the evidence sentence with the observed result (e.g. 'RUSTDOCFLAGS=-D warnings cargo doc -p happenstance-sqlite --no-deps clean; docs step of cargo xtask ci --fast green').

- **Issue:** Every append scans the whole event table under the write lock. crates/happenstance-sqlite/src/event_store.rs:722-726 stamps identity with `UPDATE event SET origin_store = ?, origin_position = position WHERE origin_position IS NULL`. The only index covering origin_position is `UNIQUE (origin_store, origin_position)` with origin_store leading, so nothing can seek `origin_position IS NULL` (no ANALYZE, so no skip-scan) — the statement is O(log size) per append and it runs inside BEGIN IMMEDIATE with every other writer queued behind it. This is the same class of defect as the join-back-to-event the schema amendment exists to remove (documented at :81-88), and append-atomicity-and-store-limits/spec.md NF-001's stated rationale is lock hold time. Correctness is fine (BEGIN IMMEDIATE serialises writers, so no other NULLs are in flight); the cost is not.
  **Fix:** Bound the UPDATE by the batch's own positions — `WHERE position >= ?` seeded with positions[0], or `WHERE position IN (…)` over the assigned positions — so the stamp is an index seek over the batch instead of a scan over the log. Add an append-path assertion (tests/append.rs already has a query_plan-style helper in tests/migration.rs:127) that the stamp does not full-scan.

- **Issue:** Ledger line citations have drifted from the code after the fix pass. wide-query-chunked-not-refused/_ledger.md AC-001 cites `query_sql.rs:142-176` for chunks (actually :120-171) and `event_store.rs:1226-1300` for the merge (actually :1293-1385); AC-003 cites `:1302-1319` (actually :1364-1379); AC-005 cites `:1276-1282` (actually :1334-1340). These are the review's own evidence trail.
  **Fix:** Re-derive the file:line ranges in the two touched ledgers against HEAD before closing the slice.

- **Issue:** wide-query-chunked-not-refused/_ledger.md AC-007's evidence honestly discloses that two public items were added to happenstance-sqlite, but its verifying_test field still asserts '`cargo doc -p happenstance-sqlite --no-deps` shows no new public item'. The row contradicts itself.
  **Fix:** Rewrite the verifying_test to match the disclosed deviation: the invariant AC-007 protects is happenstance-core and happenstance-testkit byte-identical (git diff empty), with MAX_QUERY_ARMS_PER_STATEMENT and planned_statement_count named as the two deliberate, documented additions AC-002 requires.

- **Issue:** xtask/src/spec_trace.rs is outside every slice story's declared PR boundary (see scopeDrift). The change is necessary and correct, but it is unattributed.
  **Fix:** Either record it explicitly in append-atomicity-and-store-limits' report as a forced gate repair with its reason, or move it to the story that owns spec/citation reconciliation (spec-and-code-reconciliation) and note the dependency.
