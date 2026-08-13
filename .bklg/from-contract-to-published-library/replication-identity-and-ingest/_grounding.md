# Grounding — What a position means across a store boundary (HS-P0017)

Companion file, not an item. Cited paths only; nothing here is authoritative —
`.kb/decisions/`, `spec/SPECIFICATION.md` and `RUNBOOK.md` are.

## 1. What already exists in code (the phase-2 sketch)

`crates/happenstance-sync/` is real today, not a proposal: 165 lines of module
documentation in `crates/happenstance-sync/src/lib.rs` plus five submodules —
`identity.rs` (272 lines), `ingest.rs` (231), `peer.rs` (421), `wire.rs` (359),
`memory.rs` (231). It is a phase-2 *instrument*, explicitly stated as such in its
own doc comment ("Status: a phase-2 sketch, not the protocol") — bodies outside
`memory` are `todo!()`, gated by a scoped `#![allow(clippy::todo)]`
(`crates/happenstance-sync/src/lib.rs:120-124`). Briefs must write against this
sketch, not around it: `MemorySyncPeer` (`memory.rs`), `SyncPeer`/`IngestStore`
signatures (`peer.rs`, `ingest.rs`), `identity::EventId = (StoreId,
SequencePosition)` (`identity.rs:87-101`), and `wire::Envelope<T>` with its
hand-written `Deserialize` (`wire.rs`, ADR-0016 §11) are the surfaces the
architecture brief extends, not reinvents.

Two structural decisions the specification already treats as settled and that
ADR-0026/ADR-0027 must be *consistent with*, not re-litigate: the port lives in
`happenstance-sync`, not the contract crate (RUNBOOK.md:4536-4539), and
`SyncPeer` describes exactly **one** peer — fan-out, merge policy and
hub/spoke ordering belong to a runner above it (RUNBOOK.md:4539-4543,
`crates/happenstance-sync/src/lib.rs:44-56`, spec SY-8/SY-9 at
`spec/SPECIFICATION.md:6070,6090`).

`crates/happenstance-testkit/src/registry.rs` and `src/contract.rs` are the
existing rule-registry pattern AC-003 requires `sync_peer_conformance!` to reuse:
one macro (`for_each_event_store_rule!`) handing the whole rule set to a
caller-supplied emitter, so tokio/blocking/wasm harnesses share one definition
(`crates/happenstance-testkit/src/registry.rs:60-120`); `Capability` and
`Fixture` live in `crates/happenstance-testkit/src/contract.rs`
(`crates/happenstance-testkit/src/lib.rs:168,187,226`) and are what "a declined
capability still reports, rather than vanishing" (AC-003) already means for the
event-store suite. `crates/happenstance-testkit/tests/mutation_coverage.rs` is
the existing mutant-registry pattern for AC-004 — a `const` table per mutant
with a `provenance: &'static str` field naming a real adapter shape
(e.g. `mutation_coverage.rs:330,342,381`), which is CF-2/CF-4's mechanism
(`spec/SPECIFICATION.md:7186-7215`) already implemented once; `sync_peer_conformance!`'s
own mutant registry is the same pattern, not a new one.

## 2. Accepted decision atoms that constrain this project

- **`.kb/decisions/0001-async-port-flavours.md`** (ADR-0001) — no
  `#[async_trait]`, two explicit flavours (`Send`/`!Send`) via `trait_variant`.
  Binds `SyncPeer`/`IngestStore` exactly as it binds `EventStore` (CLAUDE.md
  binding constraint 1). AC-009's wasm32 exercise is this decision's own standing
  guard, extended to the sync path.
- **`.kb/decisions/0003-opaque-payloads.md`** (ADR-0003) — `Event::data` is
  `bytes::Bytes`; `happenstance-core` carries no `serde` in default features; a
  `serde` feature covers envelope types only. **Status: accepted and
  provisional**, lift condition stated in its own body: *"`happenstance-sync`
  round-trips an event between two stores without deserialising its payload"*
  (`.kb/decisions/0003-opaque-payloads.md:91-92`). AC-006's byte-identical
  round trip is this exact lift condition, not a new one the project invents —
  the grounding note should tell the architecture brief to cite the ADR's own
  words, not restate them.
- **`.kb/decisions/0013-position-assignment-and-visibility.md`** (ADR-0013) —
  the visibility invariant is `[FROZEN]` and explicitly **global, not
  per-boundary**, and `SequencePosition` is a within-one-store visibility
  predicate under the accepted Postgres mechanism, not an identity
  (`.kb/decisions/0013-position-assignment-and-visibility.md:28-30`). This is
  the direct textual source for "a position is a statement about one store's
  log" (the project's own title/problem statement) — ADR-0026 should cite it
  rather than re-derive it.
- **`.kb/decisions/0016-the-wire-format.md`** (ADR-0016) — the wire format is
  happenstance's own, no DCB compatibility obligation (the reference publishes
  none); `Envelope<T>` ships with a hand-written `Deserialize` that refuses an
  unknown `format_version` before the message is touched; explicitly **does
  not** settle the replication protocol, message set, or `SyncError`
  extension — those are named ADR-0027's and ADR-0026's respectively
  (`.kb/open-questions/sync-message-set-and-format-version.md:49-52`). ADR-0026
  and ADR-0027 build *on* ADR-0016's envelope, not around it.
- **No ADR-0026 or ADR-0027 exists yet.** `.kb/decisions/` currently holds
  0001–0016 and 0029 only (no 0017–0028, 0030+). Per `CLAUDE.md`'s "Where the
  work lives" section, atoms are authored by `/redkiln:kb-ingest` from
  `.kb/_intake/`, never hand-written — and per the user's own standing
  preference, ADR authorship stays with the runbook's ADR pass, never as a
  side effect of a planning brief. **The architecture brief must record what
  ADR-0026 and ADR-0027 need to say (grounded in the frozen spec clauses
  below) and hand authorship to the implementation phase's kb-ingest step —
  it must not draft ADR bodies as if authoring them.**

## 3. The central question already has a written, frozen answer

AC-002 asks whether ingest re-checks the writer's asserted append conditions.
This is **not open** in `spec/SPECIFICATION.md` — it is answered and `[FROZEN]`:

- **SY-1** (`spec/SPECIFICATION.md:5841-5867`): *"Ingest MUST NOT refuse a
  replicated event for any reason that is a function of the receiving store's
  state... MUST NOT evaluate an `AppendCondition` — its own or the origin's —
  as a precondition on the append."* Rule: `ingest_never_rejects` (new,
  `happenstance-sync-testkit`).
- **SY-6** (`spec/SPECIFICATION.md:5973-6029`): *"A wire-carried
  `AppendCondition` is evidence, not an instruction... a condition carrying a
  position-relative boundary... MUST be refused as ingest input."* Rule:
  `wire_condition_with_after_is_refused`. Names the two fatal defects of
  re-evaluation: a `SequencePosition` is meaningful only inside the store that
  assigned it and serialises as a naked integer, so `after` in the receiver's
  numbering is a vacuous pass over an arbitrary tail; and even
  `after: None` re-evaluation is not idempotent — re-delivery matches the
  receiver's own already-accepted copy and inverts (E2E-33).

So ADR-0026's job (per AC-002) is to **reconcile with SY-1/SY-6**, not decide
the question fresh — the spec already forbids the re-check design and the
`EventGroup::guard` field the sketch carries as evidence
(`crates/happenstance-sync/src/peer.rs:230-243`) is explicitly *evidence for an
adjudicator*, never an instruction to re-evaluate. The compensation contract
(SY-2, `spec/SPECIFICATION.md:5871-5900`, rule
`compensation_is_atomic_with_the_losing_event`) is the FROZEN answer to what
happens instead: the losing event and its domain-supplied compensation append
in the same batch, atomically, so no reader ever observes an unresolved loss.
AC-008 names both rules exactly, matching RUNBOOK.md:4611-4612 verbatim.

## 4. AC-012's named audit — clauses citing a superseded wrong implementation

- **SY-12** (`spec/SPECIFICATION.md:6153-6183`) — *"The identity a peer dedupes
  on MUST be the store-assigned `EventId`... without decoding `Event::data` or
  `Event::metadata`."* Its own text records: *"The exemplar was the sync
  crate's own proposal and it was withdrawn: the crate now mints
  `(StoreId, SequencePosition)` (`crates/happenstance-sync/src/identity.rs:87-101`)."*
  Confirmed current: `identity.rs` does mint exactly that pair today. This
  clause's named wrong implementation (identity carried in `Event::metadata`)
  is real and still rejected — no correction needed, only confirmation.
- **SY-1 and SY-6** are the two clauses AC-012 says name "the public
  `EventGroup::guard` field" — confirmed present and public today at
  `crates/happenstance-sync/src/peer.rs:230-243` (module doc calls it
  "evidence, not as an instruction"), exactly as both clauses' `Rejects:`
  sections describe it. Both wrong implementations still exist to be rejected;
  AC-012's audit is confirmatory, not corrective, for these three.
- **The "two mis-worded ingest clauses"** the intake brief also names are a
  distinct, separate flag from AC-012's three — the intake brief does not
  identify them by id, so the architecture brief needs its own pass to find
  clauses that describe ingest as re-checking conditions in a way SY-1/SY-6
  now contradict, if any remain.

## 5. SY-32 → ES-39 → ADR-0028 handoff (AC-011)

`spec/SPECIFICATION.md:6775-6791` (SY-32, `[DEFERRED]`): a peer must report its
retention floor; a runner must detect a peer offline longer than another's
retention window. `spec/SPECIFICATION.md:7000-7003` states plainly: *"ES-39
defers the primitive that lets a store say what it does not hold. SY-32 depends
on ES-39 and cannot be settled ahead of it."* `RUNBOOK.md:4641-4644` confirms
**ES-39/ADR-0028 belongs to the retention project**, not this one: *"ADR-0028.
Either a port surface by which a store reports the history it does not hold, or
an explicit written refusal... Both close E2E-46 and E2E-47."* This project's
own decomposition entry says the same from the other side —
`replication-identity-and-ingest` "does not own... what a store may forget
(`retention-and-incomplete-logs`)" (`.bklg/from-contract-to-published-library/_decomposition.md:48`).
**AC-011's answer is therefore structurally a handoff, not a settlement**: SY-32
stays `[DEFERRED]`, ADR-0026 records the explicit dependency on ADR-0028 and
ES-39 by name, and the `replication → retention` DAG edge
(`.bklg/from-contract-to-published-library/_decomposition.md:151`) is confirmed
correct as-is rather than needing to flip — retention already runs after this
project in the recorded order (RUNBOOK.md phase 12→13→14,
`.bklg/from-contract-to-published-library/_decomposition.md:163,187`).

## 6. AC-013's two phase-13-owned open-question atoms

Both confirmed `status: accepted`, `kind: open_question`, explicitly
"Owned by phase 13" in their own summaries:

- `.kb/open-questions/sync-message-set-and-format-version.md`
  (`kb-open-question-sync-message-set-undesigned-001`) — whether
  `format_version` is per-message or per-connection-negotiated; forced by
  phase 13's message-set design.
- `.kb/open-questions/dcb-reference-publishes-no-wire-format.md`
  (`kb-open-question-dcb-no-published-format-001`) — WF-1's interoperability
  half; already resolved *toward deferral* by ADR-0016's own W7 research
  finding (the DCB reference publishes no format at all), but the atom itself
  says the interoperability *question* — attempt a bridge if one ever
  appears, or decline by design position — stays for phase 13 to answer.
  RUNBOOK.md:4593-4595 requires this be recorded "named, deferred, with the
  experiment being a specific external implementation to interoperate with.
  Not silence" inside ADR-0026's envelope section — this is WF-1's
  interoperability half from AC-010, landing in the same ADR section as this
  atom's resolution.

`.kb/maps/open-questions-index.md` (`kb-map-open-questions-index-001`) is the
index AC-013 requires be "updated in place (annotated, not removed)" once both
resolve — confirmed to exist and to be the map file the `defer_open_question`
disposition maintains per its own summary.

## 7. Non-goals and boundary risk (from the intake brief, confirmed against code)

- **`IngestStore` stays out of `EventStore`.** Confirmed no method on
  `happenstance-core`'s `EventStore` trait takes a foreign identity — the
  seam is `IngestStore` in `crates/happenstance-sync/src/ingest.rs` only,
  matching RUNBOOK.md:440-466's stated discharge ("Identity does not enter
  `EventStore`... coherence is what makes this work: the orphan rule says the
  crate defining a trait is the only crate that can grow it").
  RUNBOOK.md:462-466 names the **residual risk** explicitly and its two
  mitigations (the phase-2 sketch existing nine phases early; ADR-0026 must be
  written against two structurally unlike peers) — the architecture brief
  should carry this risk forward rather than treat it as closed.
- **Neither sync crate publishes here.** `publication-and-positioning` owns
  the release train (`.bklg/from-contract-to-published-library/_decomposition.md:47`);
  AC-015 requires this project record its disposition on *claiming* the two
  crate names (`happenstance-sync`, `happenstance-sync-testkit`) — RUNBOOK.md's
  phase-13 work list opens with exactly that claim as its first bullet
  (RUNBOOK.md:4552-4555), which the architecture brief should cite as the
  existing plan of record rather than invent a new claiming mechanism.

## 8. Anchors used above (verified to exist)

- `.kb/decisions/0001-async-port-flavours.md`
- `.kb/decisions/0003-opaque-payloads.md`
- `.kb/decisions/0013-position-assignment-and-visibility.md`
- `.kb/decisions/0016-the-wire-format.md`
- `.kb/open-questions/sync-message-set-and-format-version.md`
- `.kb/open-questions/dcb-reference-publishes-no-wire-format.md`
- `.kb/maps/open-questions-index.md`
- `crates/happenstance-sync/src/lib.rs`, `identity.rs`, `ingest.rs`, `peer.rs`,
  `wire.rs`, `memory.rs`
- `crates/happenstance-testkit/src/registry.rs`, `src/contract.rs`
- `crates/happenstance-testkit/tests/mutation_coverage.rs`
- `spec/SPECIFICATION.md` (SY-1 – SY-35, WF-1, CF-1–CF-5, ES-39 references
  listed above by line)
- `RUNBOOK.md` (lines 435-467, 4516-4650)
- `.bklg/from-contract-to-published-library/_decomposition.md`
- `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_intake-brief.md`

## 9. Tensions to flag in the architecture brief

1. **AC-001 requires accepted ADR atoms before code, but nothing in this
   planning grain may author them.** The brief must state the handoff to the
   runbook's ADR-authoring step explicitly (kb-ingest), per the user's
   standing preference that ADR authorship never happens as a side effect —
   flag this as a sequencing dependency of the implementation phase, not
   something the planning briefs resolve.
2. **AC-002's "central question"** reads as if it is still open; grounding
   shows SY-1/SY-6 already answer it as `[FROZEN]`. The brief should frame
   ADR-0026's job as reconciliation and citation, not fresh deliberation —
   otherwise there is a real risk of the brief re-opening a frozen clause,
   which CLAUDE.md's binding-constraints section forbids without a new ADR.
3. **AC-011's SY-32 handoff depends on `retention-and-incomplete-logs`
   existing with ADR-0028 as its planned artifact** — confirmed consistent
   with the decomposition and RUNBOOK, but the architecture brief should note
   this project's ADR-0026 can only *record* the dependency, not discharge it;
   discharge is retention's.
