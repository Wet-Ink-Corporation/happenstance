---
id: kb-open-question-workerd-runner-absent-001
title: The gate executes every Cloudflare rule on a shim, and nothing owns the runner it is not
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Phase 9 runs the whole conformance suite against a real Durable Object SqlStorage mapping on
  wasm32-unknown-unknown, under wasm-bindgen-test-runner over Node against a node:sqlite-backed
  DurableObjectState shim shipped in crates/happenstance-cloudflare/src/host.rs, inside one
  cargo xtask ci. A workerd-class runner inside the gate was considered and is not rejected on
  merit — it is an escalated blocking finding: workerd is an external binary with no
  Windows-native story, versioned by a Node lockfile this repository does not own, where every
  other gate tool is either rustup-pinned or cargo installed from Cargo.lock. What is true today
  is that the harness proves the mapping and proves nothing about the platform: no isolate, no
  eviction, no hibernation, no I/O gate, no event loop re-entering the object mid-await, and none
  of the platform's own storage ceilings. Two consequences are already observed rather than
  feared. ADR-0023 states the exclusion list in its own body rather than leaving it to be
  discovered. And WF-11's memory-ceiling falsifier could not be made to fire at all — a Node
  isolate has no per-isolate memory cap, which is exactly the property the falsifier needs, so
  the verdict is that the condition is not constructible on this runtime. What is not decided is
  whether a workerd-class runner ever enters cargo xtask ci, what it would cost the gate's
  single-command property to admit one, and what stays unproven for as long as it does not.
  Forced by the next clause that needs a platform behaviour rather than a storage behaviour, and
  by phase 12, where first publish turns "conformant on Cloudflare" into a promise.
depends_on: []
related:
  - kb-decision-0010
  - kb-decision-0016
  - kb-decision-0023
  - kb-reference-wf-11-memory-ceiling-verdict-001
  - kb-open-question-human-readable-encoding-limits-001
source_paths:
  - .kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - .kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md
  - references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - crates/happenstance-cloudflare/src/host.rs
  - crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs
  - xtask/src/proof.rs
last_reviewed: 2026-08-20
---

# The gate executes every Cloudflare rule on a shim, and nothing owns the runner it is not

## What is true today

`happenstance-cloudflare`'s conformance suite — the full `event_store_conformance!`
suite, the fixture-contract cases, and the four `!Send` probes — executes on
`wasm32-unknown-unknown` under `wasm-bindgen-test-runner`, against a `node:sqlite`-backed
`DurableObjectState` shim shipped in `crates/happenstance-cloudflare/src/host.rs`. One
row in `xtask/src/proof.rs`'s executed-target registry drives it, inside one
`cargo xtask ci`. This is a finding rather than a choice: what the runbook queued —
*"the `SqlStorage` mapping and the `workerd` harness (`vitest-pool-workers` as its own
CI job)"* — was reconciled against the initiative's AC-004, which requires the run to
be in the same run as the rest of the gate, and a separate CI job cannot satisfy that.

A `workerd`-class runner inside `cargo xtask ci` was the shape the queue row assumed,
and ADR-0023 records that it is **not rejected on merit**. It is an escalated blocking
finding: `workerd` is an external binary with no Windows-native story, versioned by a
Node lockfile this repository does not own, where every other gate tool this project
carries is either `rustup`-pinned or `cargo install`ed from `Cargo.lock`.

What the shim-based harness does not prove is stated in ADR-0023's own body rather than
left to be discovered: no isolate, no eviction, no hibernation, no I/O gate, no event
loop re-entering the object mid-`await`, and none of the platform's own storage
ceilings. Two consequences of that gap are already observed, not merely predicted.
First, ADR-0023 itself states the exclusion list as a scope note in its decision body.
Second, WF-11's memory-ceiling falsifier was fired at directly and could not be made to
land: the staircase asked the host for 2,047 pages and was granted every one, taking
linear memory to 142,147,584 bytes — past Cloudflare's own documented 128 MiB
per-isolate limit — refusing nothing, because a Node isolate has no per-isolate memory
cap. `kb-reference-wf-11-memory-ceiling-verdict-001` records the verdict: the condition
this falsifier needs is not constructible on the runtime that was supposed to supply
it, and that is a second, independent consequence of the same absent runner rather than
a new problem.

## What is not decided

Whether a `workerd`-class runner ever enters `cargo xtask ci`, in any shape — a gated
step, a separate mandatory job, or something else. What it would cost the gate's
single-command property to admit one. And, for as long as it does not, which specific
platform-shaped clauses stay unproven by a green gate — the list ADR-0023 already
names is a floor, not a ceiling, and the next clause that needs a platform behaviour
rather than a storage behaviour will hit it again.

## What forces it

Phase 12, where first publish turns "conformant on Cloudflare" from an internal working
assumption into a claim made to a consumer. Sooner than that: the next fixture-contract
or wire-format clause that depends on isolate-level behaviour — memory ceilings,
eviction, hibernation — rather than on `SqlStorage`'s own shape, since that clause
cannot be discharged by this harness at all.

## Ordered sub-questions

1. Does a `workerd`-class runner belong in the mandatory gate, a probed step with a
   compensating mandatory assertion, or a separate CI job accepted as a claim about CI
   rather than about the gate — reopening the alternative ADR-0023 rejected, now with
   the cost measured rather than feared?
2. If no runner is ever admitted, does the exclusion list ADR-0023 states become a
   living clause-tagged inventory — so a reader can ask "is clause X proven here" —
   rather than a paragraph in one decision's body?
3. Does WF-11's finding that the memory-ceiling condition is unreachable on this
   runtime change what `replication-identity-and-ingest` (HS-P0017) is obligated to
   assume about a forwarding peer's actual ceiling?
