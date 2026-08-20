---
item: HS-P0013
stage: implementation
created: 2026-08-19T14:43:59.389Z
updated: 2026-08-19T14:43:59.389Z
template_sig: 4c5f37d6
rendered_sig: 84ee7610
---

# Slice ledger — The edge store, run rather than asserted

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| wasm-execution-seam | approved | wasm-execution-gate-step 8ea7bb7 | (this commit) |
| real-worker-bindings | approved | worker-binding-layer 310a4c8, durable-object-write-path 3eb91cf, durable-object-read-path 9891320, caller-visible-error-verdict 5955cb3 | (this commit) |
| durable-object-conformance-run | changes-requested | durable-object-host-and-fixture 6fc808e, every-rule-under-workerd 440bbac, measured-store-limits 22529d5 | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### durable-object-conformance-run

- **issue**: crates/happenstance-cloudflare/tests/durable_object_conformance.rs:1 still reads '//! Every event-store conformance rule, executed against a real Durable Object.' The file is untouched since 440bbac ('Every rule, executed under workerd') — it survived both the 2026-08-19 repair pass and the 2026-08-20 ADR-0023-A pass, even though commit 84d5ab9 quotes this exact claim as the slice's central blocking finding. It contradicts kb-decision-0023's exclusion list, src/host.rs:41-42's binding 'It is not a Durable Object runtime, and nothing in this crate may be read as saying it is', and the amendment's own stated principle that a reader who never opens the evidence package must not be told the opposite of what it says. It sits at the reader's landing point the spec designates (every-rule-under-workerd/spec.md:281).
  **fix**: Rewrite the module doc headline of tests/durable_object_conformance.rs to the amended sentence already used elsewhere in the crate — e.g. '//! Every event-store conformance rule, executed on `wasm32-unknown-unknown` under `wasm-bindgen-test-runner` against a real `SqlStorage` mapping — not under `workerd`.' — and add a one-line pointer to `crate::host`'s 'What it is not' section so the exclusion list is one hop away from the executed artefact.
- **issue**: xtask/src/proof.rs:559-563 is stale in a way the executed run disproves: 'this fixture *declines* `MID_BATCH_FAULT` today, so they are its visible `SKIP <rule>: <reason>` lines under `--nocapture`, and they are what `measured-store-limits` will flip.' measured-store-limits is in this same slice and did flip it — MID_BATCH_FAULT is SUPPORTED (tests/support/mod.rs:244), both fault rules Ran, and the Cloudflare row now prints no SKIP line at all. The gate registry's own documentation misdescribes the behaviour of the row it guards, and points a reader at output that does not exist.
  **fix**: Update the CLOUDFLARE_WASM_RULES doc comment at xtask/src/proof.rs:559-568 to past tense: the two atomicity-under-fault rules and `append_reports_exceeded_store_limits` now Ran rather than skipped, MID_BATCH_FAULT rests on a real SQLite trigger, and the reason they stay named is that a regression flipping the constant back to a decline would leave them green as skips.
