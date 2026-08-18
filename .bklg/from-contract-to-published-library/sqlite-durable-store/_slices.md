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

- **Issue:** BLOCKING — the terminal story's ledger states a falsification experiment that does not reproduce, on the row carrying the slice's headline criterion. .bklg/from-contract-to-published-library/sqlite-durable-store/sqlite-fixture-and-whole-suite/_ledger.md AC-001 evidence asserts: '**The run means something, checked rather than assumed:** changing `MAX_EVENT_DATA_LEN` by one byte turns `append_reports_exceeded_store_limits` red, so the suite is live against this fixture.' I ran exactly that — crates/happenstance-sqlite/src/event_store.rs:245, 1_048_576 -> 1_048_575 — and got `test result: ok. 89 passed; 0 failed`. It cannot go red, and the same file says why two rows later: AC-005's evidence records that crates/happenstance-sqlite/tests/conformance.rs:177-183 MIRRORS the adapter constant, so the mutation moves the declared and the enforced number together. The artifact asserts a design property and, as evidence, an experiment that property makes impossible — the same self-contradiction class the previous review raised against wide-query AC-007 and against schema-migration AC-008 (which was blocking). In a project whose premise is that six things look like evidence and are not, this sentence is exactly that.
  **Fix:** Replace the sentence with the mutation that actually falsifies, and say why the obvious one cannot. Verified working: change `>` to `>=` at crates/happenstance-sqlite/src/event_store.rs:482 (`if event.data().len() >= Self::MAX_EVENT_DATA_LEN`) and `cargo test -p happenstance-sqlite --test conformance` reports `88 passed; 1 failed` with `dcb_conformance::append_reports_exceeded_store_limits` in the failures list. Record that the constant itself is NOT a valid mutation precisely because the fixture mirrors it (which is AC-005's design, not a defect), so liveness has to be probed at the enforcement site rather than at the declaration.

- **Issue:** BLOCKING (cheap) — four evidence rows in the same ledger cite line ranges in their own mount file that drifted 31 lines, and commit 2172b40's message claims 'Every `file:line` in the five ledgers is re-derived against HEAD', which is false for this one. In .bklg/from-contract-to-published-library/sqlite-durable-store/sqlite-fixture-and-whole-suite/_ledger.md: AC-001 cites conformance.rs:207 for the macro invocation (actual :238; :207 is the reopen doc comment); AC-003 cites :165-173 for `connect` (actual :196-204; :165-173 is the middle of the MID_BATCH_FAULT declension string); AC-004 cites :192-204 for `reopen` (actual :223-235; :192-204 is `connect`'s body); AC-005 cites :146-152 for the three mirrored ceilings (actual :177-183; :146-152 is the middle of a doc comment). The drift was introduced by the slice's own fix commit 9a10dbf, which grew conformance.rs by 31 lines after line 133. This matters because AC-002/AC-003/AC-004 are the three rows the spec explicitly says a green rule cannot discharge — their evidence IS the pointer at the fixture body, and every one of those pointers now lands on unrelated text.
  **Fix:** Re-derive the four ranges against HEAD: AC-001 -> crates/happenstance-sqlite/tests/conformance.rs:238; AC-003 -> :196-204; AC-004 -> :223-235; AC-005 -> :177-183. (AC-006's :133-172 and AC-002's :75-95 are close enough to stand; the SECOND_HANDLE :123 and REOPEN :128 cites are already exact.) The other four ledgers' event_store.rs and connection.rs citations I spot-checked all resolve correctly, so this is the one file the sweep missed.

- **Issue:** ADVISORY, not blocking — `redkiln verify --item HS-S0039 --grain story` reports `{"name":"boundary","ran":false,"pass":true,"detail":"no boundary declared"}` and skips provenance for the same reason, even though .bklg/from-contract-to-published-library/sqlite-durable-store/wide-query-chunked-not-refused/spec.md:292-298 declares a `## PR boundary` fence structurally identical to the four siblings that DO get checked (same heading, single balanced fence, same stage `plan`, same frontmatter shape). So one of the five stories has never had its boundary machine-checked. I verified 11596b4 manually: it touches only crates/happenstance-sqlite/src/event_store.rs, src/query_sql.rs, tests/wide_query.rs and its own .bklg directory — inside the declared fence. No drift, but the checker is silently blind here.
  **Fix:** No change needed to this slice. Raise it as a redkiln finding (same family as the #94 boundary defect already recorded in this initiative's telemetry) so a story's fence cannot be silently unenforced, and note in the slice record that HS-S0039's boundary was verified by hand at review rather than by the gate.

- **Issue:** NOTE, correctly out of scope — crates/happenstance-sqlite/src/lib.rs:5 still says 'Every operation that touches SQL is `todo!()`' and crates/happenstance-sqlite/tests/shapes.rs:3 still says 'Nothing here runs a query — every body is `todo!()` in the crate under test.' Both are now false of a crate that just passed 89 conformance rules against a real file. I confirmed this is assigned, not forgotten: instrument-markers-removed-and-gate-green/_ledger.md AC-003 names event_store.rs:3, projection_store.rs:3, tests/shapes.rs:3 and CLAUDE.md:33 alongside the front-page rewrite. Likewise the four remaining `todo!()`s in projection_store.rs and the scoped `#![allow(clippy::todo)]` at lib.rs:83, which DR-01 says die with the LAST todo!(), not this one.
  **Fix:** Nothing here. Recorded so the next reviewer does not re-raise it, and so the sibling story is not allowed to close without it.

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
