---
id: kb-decision-0023
title: The SqlStorage mapping and the off-tokio harness, settled by one body of evidence
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0023
reversibility: medium
phase: 9
supersedes: null
superseded_by: null
summary: >-
  How an event store maps onto a Cloudflare Durable Object's SqlStorage, and what harness proves
  it, given that neither tokio nor a thread is available. The title's conjunction is the
  exception kb-playbook-one-decision-per-adr-title-001 states rather than the smell it names:
  both halves are settled by one body of evidence — the conformance suite executing against the
  real worker bindings, off tokio, inside one cargo xtask ci. The mapping's four modelled
  properties are now checked against real bindings rather than a stand-in. exec is a plain fn
  returning Result<SqlCursor>, no future and no connection to acquire, which is the one storage
  in the workspace for which EventStore::read not being async is free rather than awkward —
  ADR-0001's two-trait design paid off rather than tolerated. The cursor is not a snapshot and
  that is preserved rather than papered over: SqlError::CursorInvalidated reports it, and
  ADR-0011's ceiling-first-then-page mechanism is what keeps a cursor off a suspension point.
  Everything is held !Send and !Sync through an Rc, because worker declares unsafe impl Send for
  its storage and cursor and holding either bare would hand this crate Send-ness through an
  escape hatch its own lint policy denies, with four probes including the_probe_is_not_vacuous
  stopping that becoming a claim nobody checks. Shared state is reached through a RefCell that is
  tried, so a re-entrant borrow reports rather than taking the object down. The thrown value is
  retained rather than stringified at the boundary, and ES-6 is the evidence for that: four
  reconstruction tests executed on wasm32 and pinned in xtask/src/proof.rs's registry show a
  caller recovering constraint violation from transport fault without Error carrying Send + Sync,
  so ADR-0009's decision holds and this adapter is the evidence for it rather than the exception
  to it. The harness's shape is a finding rather than a choice: the suite executes on
  wasm32-unknown-unknown under wasm-bindgen-test-runner against a node:sqlite-backed
  DurableObjectState shim, driven by one row added to a three-row executed-target registry,
  inside one cargo xtask ci — replacing the runbook's queued vitest-pool-workers CI job, which
  loses to the requirement that the run be in the same run as the rest of the gate. What the
  harness does not prove is stated rather than left to be discovered: no isolate, no eviction, no
  hibernation, no I/O gate, no event loop re-entering the object mid-await, and none of the
  platform's storage ceilings. ADR-0001 is cited and not lifted — its provisional marker was
  already lifted at phase 1 and ADR-0008 records it, so RUNBOOK.md's instruction to retire it
  again is read and not obeyed. Two subjects are recorded as non-verdicts with named owners
  rather than settled here: a workerd-class runner inside the gate, and the red cargo deny check
  bans that taking the worker dependency produced.
depends_on:
  - kb-decision-0001
  - kb-decision-0011
related:
  - kb-decision-0008
  - kb-decision-0009
  - kb-decision-0010
  - kb-decision-0012
  - kb-decision-0015
  - kb-decision-0016
  - kb-decision-0022
  - kb-open-question-workerd-runner-absent-001
  - kb-open-question-worker-async-trait-ban-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-cf-40-ownership-001
  - kb-reference-wf-11-memory-ceiling-verdict-001
  - kb-playbook-one-decision-per-adr-title-001
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - .kb/_intake/es-6-verdict-against-adr-0009s-prediction.md
  - references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - crates/happenstance-cloudflare/src/sql_storage.rs
  - crates/happenstance-cloudflare/src/host.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - crates/happenstance-cloudflare/tests/durable_object_conformance.rs
  - xtask/src/proof.rs
  - deny.toml
  - RUNBOOK.md
last_reviewed: 2026-08-20
---

# The SqlStorage mapping and the off-tokio harness, settled by one body of evidence

## Decision

This title conjoins a mapping and a harness, which `kb-playbook-one-decision-per-adr-title-001`
treats as a smell. The playbook states its own exception: the conjunction is one
decision with two consequences when both halves are settled by the same evidence. That
is the case here — the single body of evidence is the conformance suite executing
against real `worker` bindings, off `tokio`, inside one `cargo xtask ci` — and this
atom states the exception explicitly so the next reader finds it already cited rather
than re-litigating the split.

**The mapping.** The stand-in this adapter carried for two phases modelled four
properties; they are now checked against real bindings. `worker::SqlStorage::exec` is a
plain `fn` returning `Result<SqlCursor>` — no future, no connection to acquire, because
storage is co-located with the object — making this the one adapter in the workspace
for which `EventStore::read` not being `async` is free rather than awkward: ADR-0001's
two-trait design paid off here rather than merely being tolerated. The cursor is not a
snapshot and cannot safely be held across an `await`; that limit is preserved rather
than papered over via `SqlError::CursorInvalidated`, with ADR-0011's ceiling-first-then-
page mechanism keeping a cursor off any suspension point. `worker` declares
`unsafe impl Send` for both its storage and its cursor types, so holding either bare
would hand this crate `Send`-ness through an escape hatch its own lint policy denies;
both are instead held through an `Rc`, `!Send` for every payload, with four probes —
including `the_probe_is_not_vacuous` — stopping that guarantee from becoming an
unchecked claim. Shared state is reached through a `RefCell` that is *tried* rather than
borrowed outright, so a re-entrant borrow on the single-threaded, re-entrant object
reports rather than aborting it.

**ES-6, and why the thrown value is retained rather than stringified.** The mapping
retains the thrown JavaScript value at the boundary (`JsThrow`, not
`StringifiedThrow`), and ES-6's verdict is the evidence for that choice: four
reconstruction tests, executed on `wasm32-unknown-unknown` and named in
`xtask/src/proof.rs`'s registry, show a caller recovering the one fact it must branch
on — constraint violation versus transport fault — from what `append` returns, without
`Error` carrying a `Send + Sync` bound. Two wrong classifiers (evidence-discarding,
over-flattening) were compiled against the real suite and rejected by it, so the green
result is not a suite that happens to pass regardless. ADR-0009's decision holds; this
adapter is the evidence for it rather than an exception carved out around it.

**The harness.** What the runbook queued — a `vitest-pool-workers` CI job under
`workerd` — is incompatible with the initiative's requirement that the run execute
inside the same run as the rest of the gate, and reconciling that incompatibility is
what actually landed: the suite runs on `wasm32-unknown-unknown` under
`wasm-bindgen-test-runner`, against a `node:sqlite`-backed `DurableObjectState` shim in
`crates/happenstance-cloudflare/src/host.rs`, driven by one row added to `xtask/src/proof.rs`'s
executed-target registry, inside one `cargo xtask ci`. A `workerd`-class runner was not
rejected on merit — it is an escalated blocking finding, recorded rather than settled
here (`kb-open-question-workerd-runner-absent-001`), because `workerd` has no
Windows-native story and is versioned by a Node lockfile this repository does not own.
What the shim harness does not prove is stated in this decision rather than left to be
discovered: no isolate, no eviction, no hibernation, no I/O gate, no event loop
re-entering the object mid-`await`, and none of the platform's own storage ceilings.

**ADR-0001, cited and not lifted.** `RUNBOOK.md` instructs that ADR-0001's provisional
marker be formally retired here. That instruction is read and not obeyed: the marker
was already lifted at phase 1 by `LocalMemoryEventStore`, and ADR-0008 records the
lift. `kb-decision-0001` is accepted and immutable. What phase 9 supplies is real-runtime
evidence behind an already-accepted decision — the bare, `!Send` port flavour observed
under execution against a real adapter on the target the two-flavour design was built
for, rather than merely under `cargo check`.

## What this decision does not settle

Two subjects are recorded as non-verdicts with named owners. Whether a `workerd`-class
runner ever enters the mandatory gate is `kb-open-question-workerd-runner-absent-001`'s.
Taking `worker` as a production dependency turned `cargo deny check bans` red, because
`worker` and `worker-macros` depend unconditionally on `async-trait`, which `deny.toml`
bans under ADR-0001; whether to ratify a `wrappers` exception or leave the ban red is
`kb-open-question-worker-async-trait-ban-001`'s. Neither choice widens any
`happenstance` port with a `Send` bound — `worker`'s own use of `#[async_trait]` is
confined to its `DurableObject` trait. No clause is amended and no maturity marker
moves; CF-39, CF-40 and WF-11 are discharged or resolved elsewhere, and no accepted
decision atom's body — ADR-0001, ADR-0008, ADR-0009, ADR-0011, ADR-0012, ADR-0015 or
ADR-0016 — is edited.

## Alternatives rejected

Reading a `max(position)` ceiling after the cursor opens, rejected because it cannot
bound a cursor that is not a snapshot. `Query::index_arms()` as a contract-level
decomposition, rejected for the same reason ADR-0022 rejected it one adapter over: it
does not exist in `happenstance-core` and one adapter's convenience is not grounds to
widen the contract. Keeping the hand-written stand-in instead of taking the real
`worker` dependency, reversed on measurement once the four modelled properties needed
checking against real bindings. `vitest-pool-workers` as its own CI job, a runner-free
harness with no Durable Object at all, and a probe-gated step with no mandatory
compensator all lost to the gate's single-run requirement and its rule that a
constraint checked only when a tool happens to be present is unguarded everywhere else.
