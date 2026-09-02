---
id: kb-reference-wf-11-memory-ceiling-verdict-001
title: WF-11's falsifier fired at, and the memory ceiling it needs is not constructible on this runtime
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-9 measurement that answers WF-11's peer condition, run 2026-08-19 from
  crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs on wasm32-unknown-unknown inside
  cargo xtask ci, choosing among three admissible shapes through an enum rather than through
  prose. The verdict is (c), the condition is not constructible on this runtime, and it is a
  verdict because it names what is missing rather than reporting an absence: the staircase asked
  the host for 2,047 pages and was granted every one, taking linear memory to 2,169 pages =
  142,147,584 bytes — past Cloudflare's own documented 128 MiB per-isolate limit — refusing
  nothing, and the walk stopped on the probe's own page budget rather than on a refusal. The
  cause is the runner: wasm-bindgen-test-runner over Node against a node:sqlite-backed
  DurableObjectState shim, and a Node isolate has no per-isolate memory cap, which is exactly the
  property the falsifier needs. What would supply it is a real Workers isolate or a runner flag
  capping the linear memory a test module may grow to. The arithmetic changes the question even
  for a real isolate: at 128 MiB the firing payload is 36,604,834 bytes, peak being payload times
  11/3 — the payload plus its roughly 4/3 rendered string plus the roughly 4/3 serialiser buffer
  — which is thirty-five times the 1 MiB MAX_EVENT_DATA_LEN this adapter's fixture declares, so
  no payload this store would accept can fire it and it can only fire on a payload another store
  accepted and this peer is asked to forward. The category finding is untouched and was
  re-confirmed at six sizes: serde's data model has no streaming entry point for a string, so any
  human-readable payload encoding materialises whole. The published cost table reproduced exactly
  on wasm32 from the published seed — a 348,160-byte payload costs 464,218 bytes through the
  human-readable path against 348,163 through the binary one — and the memory claim held at 31
  pages against 17 on identical bytes, with the cheaper path run first so the comparison is
  handicapped against the claim.
depends_on: []
related:
  - kb-decision-0016
  - kb-decision-0003
  - kb-reference-wire-format-measurements-001
  - kb-open-question-human-readable-encoding-limits-001
  - kb-open-question-workerd-runner-absent-001
  - kb-decision-0023
source_paths:
  - .kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md
  - crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs
  - crates/happenstance-cloudflare/tests/support/
  - xtask/src/proof.rs
  - references/adr/0016-the-wire-format.md
last_reviewed: 2026-08-20
---

# WF-11's falsifier fired at, and the memory ceiling it needs is not constructible on this runtime

## What this is a pointer to

`crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs`, run 2026-08-19 on
`wasm32-unknown-unknown` inside `cargo xtask ci`, named in `xtask/src/proof.rs`'s
executed-target registry so it cannot be renamed or emptied without failing the gate.
This atom is the citable verdict; it commits nothing about the wire format, moves no
marker, and answers only the runtime-availability half of
`kb-open-question-human-readable-encoding-limits-001`. That question's own body stays
untouched — this is a new atom, linked to it, per `.kb/open-questions/README.md`'s
resolution shape.

## The verdict, chosen among three admissible shapes

The test structures its outcome as a three-variant enum rather than a pass/fail bit, so
"did not fire" cannot be misread as "cannot fire." The result is **(c): the condition
is not constructible on this runtime.**

The missing element is a memory ceiling the isolate can be made to reach. A staircase
allocator asked the host for 2,047 pages and the host granted every one, taking linear
memory to 2,169 pages — 142,147,584 bytes, past Cloudflare's own documented 128 MiB
per-isolate limit — refusing nothing. The walk stopped on the probe's own page budget,
not on a host refusal. The cause is the harness rather than the mapping: the gate's
runner is `wasm-bindgen-test-runner` over Node against a `node:sqlite`-backed
`DurableObjectState` shim, not `workerd`, and a Node isolate carries no per-isolate
memory cap — exactly the property this falsifier needs and the one thing the runner
does not model. `kb-open-question-workerd-runner-absent-001` records this as a second,
independent consequence of the same absent runner, not a new problem.

## The arithmetic that changes the question

Even granting a real isolate at Cloudflare's documented 128 MiB ceiling, the firing
payload works out to 36,604,834 bytes: peak memory is payload size times 11/3, being
the payload itself plus its roughly 4/3-scaled rendered JSON string plus a roughly
4/3-scaled serialiser output buffer held concurrently. That figure is **thirty-five
times** the 1 MiB `MAX_EVENT_DATA_LEN` this adapter's own fixture declares
(`crates/happenstance-cloudflare/tests/support/`). No payload this store would accept
on write can fire the falsifier; it could only fire on a payload some other, more
permissive store accepted and handed to this adapter to forward — the forwarding
condition WF-11 names, and one this measurement confirms stays unmet here.

## The category finding, re-confirmed rather than weakened

The underlying claim — serde's data model exposes no streaming entry point for a
string, so any human-readable encoding materialises the whole payload in memory before
it can be inspected — was re-observed at six payload sizes and is untouched by the
verdict above. The published cost table reproduced exactly on `wasm32` from the
published seed: at 348,160 raw bytes, the human-readable path costs 464,218 bytes and
the binary path 348,163, matching `kb-reference-wire-format-measurements-001`'s
figures to the byte. The memory-page claim held at the same size — 31 pages against 17
on identical input bytes — with the cheaper (binary) path deliberately run first in the
harness so the comparison could not be flattered by warm-allocator effects.

## What is explicitly not claimed

This atom does not say WF-11 is safe, does not move its `[PROVISIONAL]` marker, does
not re-scope its `MUST`, and says nothing about what the wire format becomes.
Sub-question 2 (streaming scheme, declared capability, or JSON scoped to diagnostics)
and sub-question 3 (bearing on ADR-0003's opaque-payload boundary) are both explicitly
not reached — the first belongs to `replication-identity-and-ingest` (HS-P0017) and
needs a decision record of its own; the second is moot because the probe forwards
bytes it never inspects, which is what makes *forward* the correct verb and the
boundary merely observed here rather than tested.
