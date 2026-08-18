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
| durable-event-store | approved | schema-migration-and-identity 8381c89, append-atomicity-and-store-limits 23bc776, lazy-read-with-snapshot-ceiling 0c6ce2b, wide-query-chunked-not-refused 11596b4, sqlite-fixture-and-whole-suite 41a2064 | (this commit) |
| race-model-and-durability | approved | concurrency-family-and-contender-count 995b987, model-family-and-mutant-pass-column 0ad702f, reopen-negative-control-and-durability-verdicts 2d08e0d | (this commit) |
| sqlite-projection-store | approved | projection-store-passes-the-borrowed-suite 1afb47b | (this commit) |
| publishable-and-reconciled | changes-requested | instrument-markers-removed-and-gate-green 62a05dd, crates-io-name-and-packaging-facts ada4962, spec-and-code-reconciliation 54df29a | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### publishable-and-reconciled

- **Issue:** BLOCKING — the deterministic gate is red for HS-S0047. `redkiln verify --item HS-S0047 --grain story` returns `{"pass":false, ... {"name":"boundary","pass":false,"detail":"changed outside declared boundary: standards/rust/01-standard-of-evidence.md"}}`. The edit is correct and forced — the pass moved `silently ignores` from `spec/SPECIFICATION.md:5926` to `:5984`, past `lint_constitution`'s 10-line window (`xtask/src/lint_constitution.rs:111`) — and it is reasoned in `_reconciliation.md:192-211`, but no story's declared boundary admits `standards/`. HS-S0045 and HS-S0046 both return `pass:true`; only this one fails.
  **Fix:** Add `standards/rust/01-standard-of-evidence.md` to the fenced PR-boundary block in `.bklg/from-contract-to-published-library/sqlite-durable-store/spec-and-code-reconciliation/spec.md:199-205`, carrying the one-line reason already written in `_reconciliation.md` (citation-line repair only, the same principle HS-S0045's spec applies to `spec/SPECIFICATION.md`), then re-run `redkiln verify --item HS-S0047 --grain story` and confirm `pass:true`. Do not revert the digit — that would red a REQUIRED gate step.

- **Issue:** HS-S0047's `_ledger.md` AC-005 evidence is stale by the fix commit: it reads `396 citations checked (78 anchored ...)` and `checked 389 to 396 (+7)`, while the tree emits 398 and `_reconciliation.md:32` records 398 (+9). `c510468`'s own message says '396 -> 398' and updated `_reconciliation.md` but not the ledger row. Both numbers clear the baseline, so the criterion still holds, but the story's primary evidence artifact now misquotes the tree it measures — the exact hand-maintained-count-beside-the-list failure this story is about.
  **Fix:** Update the AC-005 evidence string in `.bklg/.../spec-and-code-reconciliation/_ledger.md` to `398 citations checked (78 anchored to their subject, 12 external)` and `checked 389 to 398 (+9)`, matching `_reconciliation.md` and the live run.

- **Issue:** One same-class marker survives the pass and is absent from the verdict table. CF-39's `[PROVISIONAL]` falsifier at `spec/SPECIFICATION.md:8053-8058` still ends 'The instruments are the rusqlite adapter at phase 8 and the Postgres adapter at phase 10, and no adapter has armed a fault yet' — the only remaining sentence in the specification that names phase 8 in the future tense (every other 'phase 8' mention is past tense). Its substantive claim is still true (`SqliteFixture` declines `MID_BATCH_FAULT` by scope, `crates/happenstance-sqlite/tests/support/mod.rs:173-183`), which is why this is a finding rather than a false clause — but `c510468` widened `_reconciliation.md`'s inclusion rule to 'the crate's identity — path or name, prose, Rejects: or maturity marker' precisely to catch markers like this, and the widened rule was not re-run to exhaustion, so CF-39 has no verdict row.
  **Fix:** Add a CF-39 row to `_reconciliation.md`'s 'Clauses naming this crate' table. `unchanged` is a legitimate verdict here if the reason is stated (the injectable trigger fault is available and unabsorbed per `tests/append.rs::a_failure_mid_batch_leaves_nothing`, but arming it belongs to ES-35's owner, so the falsifier is still live); alternatively restate the instrument list inside the same level as ES-41 and CF-40 were. Either way, record it rather than leaving the table's coverage claim over-stated.

## Carried outside the slice loop — a finding a re-launch cannot reach

This section is NOT a surviving-findings block and must not be read as one. It records work owed on
a slice sealed `approved`, which a fresh re-launch **skips** — so nothing in the slice loop will
look at it, and it needs a decision taken outside that loop.

### bench-harness-and-adr (sealed `approved` at 20842af) — HS-S0034 `benchmark-harness`

- **Issue:** `benchmark-harness`'s checkpoint `2665883` changes four files outside its declared PR boundary: `standards/rust/41-declarative-macros.md`, `52-wasm32-and-target-cfg.md`, `62-doctests-and-harnesses.md`, `91-adapter-authoring-recipe.md`. Found only after the fact, because the boundary check was unpassable for every story in this initiative until the 0.19.0 upgrade on 2026-08-17 (upstream #94); slice 1 sealed `approved` on 2026-08-17 without it ever having run. The edit is **9 insertions to 9 deletions and is pure citation re-anchoring** — `crates/happenstance-testkit/src/lib.rs:490→:516`, `:466→:492`, `:505→:531` — compelled because adding `event_store_benchmarks!` to `lib.rs` moved the lines the constitution cites and `cargo xtask lint-constitution` is a gate step, so leaving them stale is a red gate. This is **instance eighteen** of the compelled-boundary class this initiative has recorded since HS-P0010.
  **Fix:** A human decision, not a reviewer's. Either widen `benchmark-harness`'s fence to `standards/rust/**` **scoped to citation re-anchoring only** — rule text, evidence selection, retirement and new atoms explicitly out, per the `aef8990` / `34d5311` precedent — with the argument and the limit stated beside the fence; or attribute the four edits to a story that owns the constitution. Until one of those happens `redkiln advance HS-S0034 --to report` exits 1 on `implement`'s command gate, so **neither an approval nor a rejection can be recorded for HS-S0034**.
  **SETTLED BY HAND, 2026-08-17 (`e020276`), by human decision.** The fence was widened on the first option and on the precedent's exact terms: `standards/rust/**` admits **citation re-anchoring only**, with rule text, evidence selection, retirement and new atoms explicitly excluded, and without reopening `xtask/**`, which `spec.md`'s own exclusion list keeps out — so no new gate step is admitted. The argument, the limit, the nine moved citations and the reason this was settled after the slice had sealed are all recorded beside the fence at `benchmark-harness/spec.md:99-133`. `redkiln verify --item HS-S0034 --grain story` now exits 0 on all four checks. **This item is closed; run 3 inherits nothing from it.**

### wide-query-chunked-not-refused HS-S0039 — read its verdict as unverified

Its `spec.md` declares a valid three-line fence at `:294` that redkiln never parses: an earlier
prose heading, `### What an arm is, and where the chunk boundary comes from` at `:86`, matches
`declaredBoundary`'s heading regex first and the check silently disables itself. Filed upstream as
**redkiln #135**; unfixed on 0.19.0. So this story's boundary was **not checked**, and a green gate
for it means only that no check ran. Verify its diff by hand before treating it as clean.
