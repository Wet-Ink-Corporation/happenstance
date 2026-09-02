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
| publishable-and-reconciled | approved | instrument-markers-removed-and-gate-green 62a05dd, crates-io-name-and-packaging-facts ada4962, spec-and-code-reconciliation 54df29a | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

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
