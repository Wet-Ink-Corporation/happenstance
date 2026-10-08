---
id: kb-decision-0087
title: ES-11 is met on one-shot HTTP by a read-settlement fence
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0087
reversibility: medium
phase: 17
supersedes:
  - kb-decision-0061
superseded_by: null
summary: >-
  Proposed, not decided; it supersedes ADR-0061 only when the owner accepts it. ADR-0061 kept ES-11
  PROVISIONAL with happenstance-neon outside the release set, and its own falsifier 2 has fired:
  query_items_share_one_snapshot went red on Neon (PR #20, CI runs 37504851570 and 37570009097),
  so its "104 of 105" figure is false. Neon is one of 1.0's crates (ADR-0066). On the owner's call
  to spike a fence, lane L8 built one: hold an append until every read the same transport
  dispatched before it has been answered. An answer follows the statement's execution, so the
  read's snapshot precedes the append's commit, an order the store honours, which ADR-0061's
  narrowed condition asks for. The release is driven by the transport's I/O (a spawned task on the
  host, a fetch promise's callback on wasm32), never by polling the stream, which is why a
  handle-side count (F0) deadlocks the rule and this does not. Measured on CI's live-neon job
  against the live endpoint (spike PR #50, head 1177cfc, run 37594816236, attempts 1-3) under a rule
  written before the first run and amended before the first counted run: baseline 172 red of 1,500
  (all C2, Clopper-Pearson 95% 9.90%-13.19%), fence 0 of 1,500 (rule-of-three 0.20% pooled), no
  anchor, no error, no void attempt, both racing rules green in every attempt. Verdict: the fence
  works. Proposed: ES-11's sufficiency condition names the read's answer preceding the append's
  dispatch as a store-honoured order; Neon meets ES-11 and ES-12 through a required
  SqlTransport::reads_settled, a semver-major break landing in 0.4.0 as its own change, not the
  spike; both clauses freeze when the fence is on main; the ordering domain is one transport value;
  ADR-0061's "Postgres gets both halves" is corrected to order by queue position (reasoning). Not
  measured: a wasm32 fetch transport, the second-handle and cross-process case, and contention under
  the fence. Owner questions: D1, D2, D6, D8, D10, D11, D12.
depends_on:
  - kb-decision-0061
  - kb-decision-0011
related:
  - kb-decision-0061
  - kb-decision-0066
  - kb-decision-0071
  - kb-decision-0077
  - kb-decision-0024
  - kb-open-question-one-shot-http-es-11-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - experiments/es-11-fence/README.md
  - experiments/es-11-fence/run.sh
  - experiments/es-11-fence/results/tally.md
  - experiments/es-11-fence/results/raw/37594816236-1.jsonl
  - experiments/es-11-fence/results/raw/37594816236-2.jsonl
  - experiments/es-11-fence/results/raw/37594816236-3.jsonl
  - crates/happenstance-neon/src/transport.rs
  - crates/happenstance-neon/src/event_store.rs
  - spec/SPECIFICATION.md
  - .kb/_intake/decisions/wi-0f1291-the-live-neon-job-is-a-required-check-and.md
  - references/adr/0087-es-11-is-met-on-one-shot-http-by-a-read-settlement-fence.md
last_reviewed: 2026-10-08
---

# ES-11 is met on one-shot HTTP by a read-settlement fence

**Proposed.** The full record, with the mechanism, the rule as written and amended, the alternatives
and the API table, is
[`references/adr/0087-es-11-is-met-on-one-shot-http-by-a-read-settlement-fence.md`](../../references/adr/0087-es-11-is-met-on-one-shot-http-by-a-read-settlement-fence.md).
It supersedes [ADR-0061](0061-es-11s-sufficiency-condition-assumed-a-queue.md) only when the owner
accepts it; until then ADR-0061 stays accepted and unedited. **The spike is not merged.** Landing the
fence is a separate implementation change after acceptance.

## Context

ADR-0061 narrowed ES-11's sufficiency condition to a read *"spawned at the first poll, and ordered
against a later append by something the store itself honours"*
(`references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md:105-106`), recorded that
`happenstance-neon` does not satisfy it, and kept the marker while Neon was outside the release set.
Neon is now one of 1.0's crates (`.kb/decisions/0066-what-1-0-promises.md:376`). And ADR-0061's
falsifier 2 has fired (`references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md:233-235`):
`query_items_share_one_snapshot` went red on Neon in PR #20, CI run `37504851570` and CI run
`37570009097`, so its *"104 of 105"* is false. This session the live Neon job also went red on
exactly these two rules on PRs #40, #44 (`5955982`) and #46 (`4e0f7de`).

## The mechanism

**Hold an `append` until every read the same transport dispatched before it has been answered.** An
answer follows the statement's execution, so the read's snapshot precedes the append's request, its
transaction and its commit: an order the store honours, with no assumption about proxy routing or
HTTP/2 stream order.

The release must come from the agent driving the I/O, not from the caller. The rule runs on one task.
A count on the handle released by the stream (F0) deadlocks: the append waits for the release, the
release waits for the task to poll the stream, and the task is inside the append. So the transport
registers each read-only request **before `round_trip` returns** and settles it from whatever observes
the answer: on the host, the spawned task that ran the request; on `wasm32`, a `fetch` promise's
settlement callback. A transport whose I/O advances only while its future is polled deadlocks too, so
the obligation is written on the trait as a MUST. The bookkeeping (`ReadLedger`) is a mutex and a
waker table, ordered by generation so later reads cannot starve an append, with no executor and no
tokio.

## The measurement

`experiments/es-11-fence/`, CI's `live-neon` job on spike PR #50 at `1177cfc` (merge ref `19b1271`),
run `37594816236`, attempts 1, 2 and 3. The rule was written before the first run and amended on
2026-10-07, before the first counted run, because a review (W1) found it could score a frontier lag
as a falsifier (`experiments/es-11-fence/README.md:184-230`). A pilot under the first rule (run
`37591126575`, attempt 1) is excluded by name; the README says who saw it. The ordering is
self-attested.

From `experiments/es-11-fence/results/tally.md:14-25`:

| Arm | Red | Judged | Detail |
| --- | ---: | ---: | --- |
| baseline | 172 | 1,500 | `es11` 92 of 750, `es12` 80 of 750; every red `late_in_drained` and class C2. Clopper–Pearson 95%: 9.90%–13.19% |
| fence | 0 | 1,500 | rule-of-three 95% upper bound 0.20% pooled, about 0.4% per shape |

No anchor, no error, no void attempt. Both racing rules were green in every attempt, and the whole
live suite passed each time. No C1 red: the append's task never reached the client library before the
read's. No C3 red in either arm. **Verdict under the rule: the fence works.**

## Decision (proposed)

1. ES-11's sufficiency paragraph is restated line-neutrally: ordered by something the store honours —
   **one queue, or the read's answer preceding the append's dispatch**. Still a narrowing.
2. `happenstance-neon` meets ES-11 and ES-12 through `SqlTransport::reads_settled`, a **required**
   method: a **semver-major break of `SqlTransport`**, landing in `0.4.0` as its own change.
3. ES-11 and ES-12 become `[FROZEN]` **when the fence is on `main`**, not on acceptance.
4. The ordering domain is **one transport value**. ES-11's MUST is unchanged; the rules check the
   same-handle case.
5. ADR-0061's *"Postgres gets both halves"* is corrected: Postgres orders by queue position on one
   runtime and one pool (`crates/happenstance-postgres/src/read_stream.rs:94-98`), so it passes by
   margin, not by a store guarantee. Reasoning, not measured.

Had the fence failed with a C3 red, the work would have stopped for the owner's call on an exception,
a dropped claim or a renewed marker. It did not.

## Not measured

A `wasm32` `fetch` transport (none exists; settling from a promise callback is reasoning, and only
compilation for `wasm32-unknown-unknown` without tokio is checked). The second-handle and
cross-process case (two transports are two ordering domains, and no rule observes their order). The
concurrency family's contention under the fence: its rules passed in every attempt and no `Busy`
exhaustion was seen, but contention itself was not measured.

## Falsifiers

1. A C3 red: a fenced red where the append was dispatched no earlier than the read was answered.
2. A conformant `SqlTransport` proves impossible on `wasm32`.
3. The concurrency family regresses with the fence: `Busy` exhaustion, a family red attributable to
   the waits, or a measured loss of contention.
4. Offline test A1 passes against an unfenced adapter.
5. A second-handle or cross-process ordering callers depend on proves checkable through the port.

Not a falsifier: green runs (`references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md:241-242`).

## For the owner

Taken as defaults so the spike could run; none acted on beyond the spike branch.

- **D1:** the shape, F1, a required trait method and so a semver-major break of `SqlTransport`; F5,
  the same mechanism as a documented obligation, is the no-break fallback.
- **D2:** publish `ReadLedger`, `ReadTicket` and `ReadsSettled` (recommended).
- **D6:** freeze when the fence lands on `main` (recommended), not on acceptance.
- **D8:** `wi-0f1291`'s required check comes back when the fence lands on `main`, not when this record
  does, because the race persists on `main` until then
  (`.kb/_intake/decisions/wi-0f1291-the-live-neon-job-is-a-required-check-and.md:25`).
- **D10:** ADR-0061 is marked superseded on acceptance only.
- **D11:** correct *"Postgres gets both halves"* here, labelled reasoning.
- **D12:** land the break in `0.4.0`.
