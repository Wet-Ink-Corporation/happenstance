---
item: HS-S0102
stage: spec
created: 2026-08-12T13:47:41.803Z
updated: 2026-08-12T13:47:41.803Z
template_sig: 87bbf1d0
rendered_sig: e44ae54a
---

# Spec — IngestStore and MemorySyncPeer get real bodies

## Scope lock

| What | Path |
| --- | --- |
| Initiative | `.bklg/from-contract-to-published-library/initiative.md` (BR-10, AC-13, **DoD 14**) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` (DAG rank 4; `publication → replication → retention`) |
| Project | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` (AC-002, DR-5, DR-7, DoD 4) |
| Project briefs | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` — architecture *Composition root* §4 (the oracle, the `memory`-feature discrepancy), §5 (the write-path seam and its four options), §7 (the wire mounts inside `Envelope<T>`); testing *The test mix* (Unit, Integration) |
| Signed-off design | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` — **N/A, and explicitly so**: this project renders no surface, the `## Items` / `## Signatures` blocks read *"N/A — no user-facing surface"*, and the no-surface determination itself is what was approved. Signatures are decided in the architecture brief and here, under the repository-wide rustdoc obligation (`standards/rust/70-rustdoc-obligations.md`) |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/ingest-store-and-memory-peer-round-trip/spec.md` |
| This story's discovery | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/ingest-store-and-memory-peer-round-trip/discover.md` — the signal ledger, the five questions it answered or deferred to this spec, and the three named wrong implementations |
| Story map row | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md`, *Slices* → `ingest-seam-and-memory-oracle`, row 2 |
| Roadmap pointer | `RUNBOOK.md:4573-4595` (phase 13 work list), `RUNBOOK.md:462-466` (the leak warning and its residual risk) |

## One-line PR slice

Replace the `todo!()` bodies with real `IngestStore` and `MemorySyncPeer` ones and drop the scoped
`#![allow(clippy::todo)]`, so a batch pushed from one in-memory store lands at the receiver's local
tail with the foreign `EventId` preserved, group-atomically and idempotently — with the doctest on
`MemorySyncPeer` that shows one round trip holding no state.

## Executive summary

`happenstance-sync` can today *describe* a store boundary and cannot cross one. The coherence half is
proved and compiled — `impl SendIngestStore for MemoryEventStore` exists in
`crates/happenstance-sync/src/ingest.rs:206-231` — and all four of its bodies are `todo!()`, each
carrying the reason it is one. `MemorySyncPeer`'s bodies, by contrast, are already real
(`crates/happenstance-sync/src/memory.rs:1-10`, `:162-231`): it holds groups pushed to it, dedupes on
`EventId` against its own `Watermark`, and hands them back on `pull` under a bounded budget.

**The delta this PR lands is the receiving half, and the wire that joins it to the sending half.** With
the inherent ingest door from `memory-store-ingest-seam` (HS-S0101) in place, the four `todo!()`s
become four bodies; the placeholder `identity::{StoreId, EventId, RecordedAt}` trio unifies with the
contract crate's settled types, which is what unblocks `holds`
(`crates/happenstance-sync/src/ingest.rs:224` names that obstruction itself); and an integration test
walks the whole hop — origin `MemoryEventStore` → `ReplicatedEvent` → `PushBatch` → `MemorySyncPeer`
→ `pull` → `IngestStore::ingest` → receiving `MemoryEventStore` — asserting that the event lands at
the receiver's *local tail* with its *origin's* `EventId` intact, that a replay of the same batch is a
no-op, and that the payload `Bytes` are byte-identical on the far side.

Three things this PR is *not*. It is not the conformance suite: `happenstance-sync-testkit` does not
exist until HS-S0103, and this story's evidence is integration tests inside `happenstance-sync` plus a
doctest. It is not the ADR-0003 `provisional` lift: it produces the byte-identity *evidence* the lift
will cite, and the lift atom is `adr-0003-provisional-lift`'s. And it does not repair a single
specification clause: deleting phase-2 prose that `[FROZEN]` clauses cite by line is real, expected,
and belongs to `frozen-clause-repairs` (HS-S0112) — this story's obligation is to **record every
citation it invalidates**, not to fix them here.

Dropping the crate-level `#![allow(clippy::todo)]` (`crates/happenstance-sync/src/lib.rs:135-139`) is
part of the deliverable rather than a follow-up. It is the mechanical proof that no body was left
behind: with the allow gone, `clippy -D warnings` fails on any surviving `todo!()`.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted anchor.

**1. Ingest does not re-check the writer's asserted conditions, and this story is where that answer
becomes a body.** SY-1 (`spec/SPECIFICATION.md:5841-5867`) and SY-6 (`:5973-6029`) are both `[FROZEN]`:
ingest MUST NOT refuse for any reason that is a function of the receiving store's state, and MUST NOT
evaluate an `AppendCondition` — its own or the origin's — as a precondition. The implementation
consequence is exact and mechanical: `IngestStore::ingest` takes `&[ReplicatedEvent]` and **no
condition parameter**, and its body must never reach the store's *conditional* append path. The
`guard` on `EventGroup` travels as evidence and is not consulted
(`crates/happenstance-sync/src/peer.rs:230-243`). The adapter's `Error` is for storage failures only —
an event this store already holds is a **skip**, never an error
(`crates/happenstance-sync/src/ingest.rs:143-149`). Reconciliation and citation, not deliberation:
re-opening this would be amending a `[FROZEN]` clause, which is out of scope for the whole initiative.

**2. The door into the receiving store is HS-S0101's inherent `&self` operation, and nothing else.**
`EventStore::append` mints `EventId::new(self.store_id, position)` for every event it writes, and
`MemoryEventStore::restore` builds a *new* store from an owned snapshot while `ingest` holds `&self`
(`crates/happenstance-sync/src/ingest.rs:52-71`; `crates/happenstance-core/src/memory.rs:163`). The
architecture brief chose option (a) — one additive inherent method on the concrete type, semver-minor,
no trait change — over growing `EventStore` (refused: `RUNBOOK.md:450-455`, SY-8 at
`spec/SPECIFICATION.md:6070-6086`) and over dropping the coherence proof. **`EventStore`'s trait
signature is byte-identical before and after this project** (architecture AC-A01). If this story finds
it needs a seam on the *port*, that is `RUNBOOK.md:462-466`'s residual risk landing hard: stop, raise a
decision atom and a re-plan — do not widen `append`.

**3. Ingested events land at the local tail, and the foreign position is never a local position.** SY-5
is `[FROZEN]` (`spec/SPECIFICATION.md:5954-5971`): an ingested event MUST NOT be assigned a position
below any position the store has already assigned, and its `Rejects:` is order-preserving insertion by
origin position or timestamp — *"the intuitive merge, and it silently destroys every projection on the
store"*. SY-19 (`:6409-6427`) says the local position is **arrival order** and that no expression
relating two stores' positions can be constructed. The return type already encodes the distinction:
`Ingested.last_local` is documented as *"arrival order here, that is authorship order there"*
(`crates/happenstance-sync/src/ingest.rs:186-190`). Keep it true.

**4. Idempotence is by `EventId` and never by content.** SY-11 `[FROZEN]`
(`spec/SPECIFICATION.md:6135-6152`): re-delivery of an event the receiver already holds MUST be a
no-op — no second copy, no compensation, no error. Its `Rejects:` is an *inversion*, not a duplicate,
because *"a duplicate is visible; an inversion looks like a decision"*. SY-12 `[FROZEN]` (`:6153-6171`)
requires the identity be reachable **without decoding `Event::data` or `Event::metadata`**;
`EventStore::contains_event_id` (`crates/happenstance-core/src/memory.rs:414`) answers it exactly once
the identity types are unified. The wrong implementation this invites is named in discovery:
`ContentDedupingIngest`, which passes every test that counts events and is wrong in both directions at
once — it over-deduplicates two genuinely distinct events with identical content, silently and
unrecoverably, and under-deduplicates the moment any peer re-encodes anything.

**5. The group decomposition is explicit on the wire and the receiver never re-infers it.** SY-30
(`spec/SPECIFICATION.md:6700-6725`, `[PROVISIONAL]`) and
`crates/happenstance-sync/src/peer.rs:183-190`: a receiver that flattened the batch and appended
event-by-event *"would publish a state the origin never had, and no amount of ordering fixes that"*.
Today's signature is flat — `ingest(&self, events: &[ReplicatedEvent])` — while the trait's own prose
says *"each group lands atomically or not at all"* (`crates/happenstance-sync/src/ingest.rs:133-138`).
**Reconciling those two is real work and its authorisation is ADR-0026's, which merges before this
story.** Two shapes are admissible: carry the group boundary into `ingest`, or compose group atomicity
above a flat `ingest` from `EventGroup`s. Read ADR-0026 before writing the body; if the atom is silent
on it, raise it rather than picking — this is a port shape, and `crates/happenstance-sync/src/lib.rs`
is where the choice becomes public. Whichever lands, `FlatteningIngest` (discovery, *The wrong
implementation*) must be rejectable, and it is invisible to any assertion over the log's final state.

**6. Resume state is owned and lives outside the peer, and the oracle is where the opposite is most
tempting.** SY-15 and SY-16 are `[FROZEN]` (`spec/SPECIFICATION.md:6292-6340`): every port method
completable in one round trip, resume state an owned transferable value the caller supplies. SY-16's
`Rejects:` is *"an in-memory cursor, which passes every test written against a process that stays
alive"* — and `MemorySyncPeer` is an in-process type, so `cursor: Cell<usize>` is the obvious wrong
answer and the oracle is the worked example a first adapter author copies. `MemoryResume` is already an
owned `Copy` token (`crates/happenstance-sync/src/memory.rs:19-42`); the **doctest this story owes must
show the token held outside the peer**, and the rule that generalises it —
`resume_survives_a_dropped_peer_handle` — is HS-S0103's.

**7. The payload is never decoded, at any hop.** DR-5, ADR-0003's own lift condition
(`.kb/decisions/0003-opaque-payloads.md`), and SY-12's `Rejects:` all rest on it: *"a suite that parses
a payload would certify a peer that does"* (`RUNBOOK.md:4576-4580`). Payload `Bytes` are compared for
equality and never inspected structurally. This story produces the byte-identity measurement; the
`provisional` lift is a **new atom** authored through `.kb/_intake/` and `/redkiln:kb-ingest`
(`adr-0003-provisional-lift`, HS-S0111), never an edit to the accepted atom.

**8. Bind the weaker flavour.** ADR-0001 (`.kb/decisions/0001-async-port-flavours.md`) and CLAUDE.md
constraint 4: generic code binds `IngestStore` / `SyncPeer` / `EventStore`, never the `Send` flavours,
and a module imports only one name of each pair or method calls go ambiguous with `error[E0034]`
(`crates/happenstance-sync/src/peer.rs:26-28`). Implementing `SendIngestStore` for a `Send` type is
correct and gives the bare flavour for free (`crates/happenstance-sync/src/memory.rs:76-80`) — the
constraint is on the *bounds this story writes*, not on which flavour a concrete type implements. SY-17
is `[FROZEN]` (`spec/SPECIFICATION.md:6341`).

**9. No conformance rule is added here, and that ordering is deliberate.** The suite does not exist
until `sync-testkit-crate-and-rule-registry` (HS-S0103). A rule written against an implementation that
already exists is written by someone who believes the implementation is right, so the rules must be
justified by clauses rather than by this code. What this story owes the next one is *named* future
rules — `ingested_events_land_above_the_local_head` (SY-5),
`redelivery_of_an_accepted_group_is_a_no_op` (SY-11), `dedupe_reaches_identity_without_decoding`
(SY-12), `push_envelope_preserves_group_boundaries` (SY-30) — each already named in the clause's own
`Rule:` field, and an integration test here that the rule can later generalise.

**10. Position literals are the headline error in this story specifically.** CF-6 and CLAUDE.md's
gap-permission rule: no assertion on a literal position value. Both stores here are `MemoryEventStore`,
which assigns densely from one, so `assert_eq!(ingested.last_local, SequencePosition::new(4))` would be
green and would encode a `MAY` as a `MUST`. Worse, the *interesting* number in this story came from the
other side of the boundary. **Every position assertion anchors on a head the receiving store reported
before the ingest; the foreign position is only ever compared as part of an `EventId`, never as a
number.**

**11. The findings are not the same as the `todo!()`s.** Implementing this crate deletes most of
`crates/happenstance-sync/src/ingest.rs:20-82` because it stops being true. That prose is the evidence
the phase-2 sketch landed nine phases early to produce (`RUNBOOK.md:462-466`). Move each finding into
the ADR that consumed it **before** deleting the paragraph that holds it, and record every
`spec/SPECIFICATION.md` and test citation this diff invalidates so HS-S0112 can repair rather than
discover them.

**Persona-journey slice.** *Event-source at the edge without hand-rolling it* and *Learn when you are
finished* (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`;
the initiative carries these from distillation rather than from `.kb/product/`, which is structurally
present and functionally empty — `initiative.md:227-258`). This is the first place in the repository
where a reader can watch an event leave one store and arrive in another, and the doctest is the artefact
that reader meets first.

## Integration contract

- **Archetype**: `capability` — a slice through identity, the port, the oracle peer and the receiving
  store, observable end to end without a conformance suite.
- **Slice / milestone**: `ingest-seam-and-memory-oracle`. Slice-mates: `memory-store-ingest-seam`
  (HS-S0101, hard predecessor — it supplies the door). The two are implemented in one context and
  merged in that order (`_storymap.md`, *Merge order* §2).
- **Mount point**: **`crates/happenstance-sync/src/lib.rs`** — the crate root is this library's
  composition root. It is where `pub mod ingest;` / `pub mod memory;` are declared (`:143-147`), where
  the public surface is re-exported (`:154-160`), and where the scoped `#![allow(clippy::todo)]` this
  story deletes lives (`:135-139`). A body that exists but is not reachable through that root is not
  mounted; a `todo!()` that survives it is caught by the removed allow.
- **Wires into**:
  - `crates/happenstance-sync/src/ingest.rs:206-231` — the `#[cfg(feature = "memory")] mod
    memory_store_ingest` block, the exact impl site whose four bodies this story replaces.
  - HS-S0101's additive inherent `&self` operation on `happenstance_core::MemoryEventStore` — the only
    write path this story is permitted to call for an already-identified event.
  - `happenstance_core::{EventId, RecordedAt, StoreId}` (`crates/happenstance-core/src/lib.rs:110`) and
    `SequencedEvent { position, id, recorded_at, event }`
    (`crates/happenstance-core/src/event.rs:484-496`) — the settled identity the placeholders unify with.
  - `happenstance_core::EventStore::contains_event_id`
    (`crates/happenstance-core/src/memory.rs:414`) — how `holds` answers without decoding a payload.
  - `crates/happenstance-sync/src/peer.rs` — `PushBatch` (`:183-196`), `EventGroup` (`:230-252`),
    `Ack` (`:253-272`), `Pulled`, `PeerLimits`.
  - `crates/happenstance-sync/src/memory.rs` — `MemorySyncPeer`, `MemoryResume`, `MemoryPeerError`.
  - `crates/happenstance-sync/src/identity.rs` — `ReplicatedEvent` and `Watermark` stay (*"genuinely
    replication's own"*, `:42-43`); `StoreId`, `EventId` and `RecordedAt` go.
  - `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs`,
    `tests/real_peer_shapes.rs` — the two files that spell `identity::` on purpose as the sites the
    unification must revisit (`:58-60` and `:38-40` respectively).
- **Renders surfaces**: **none.** `_design.md` records the no-surface determination for the whole
  project, and its `## Items` / `## Signatures` blocks read *"N/A — no user-facing surface"*. The
  design obligation this story inherits instead is the API-surface one that file redirects to the
  briefs and to `standards/rust/70-rustdoc-obligations.md`: every new public item documented, every
  fallible function carrying `# Errors`, and the doctest compiled by the gate.
- **Conformance rule(s)**: **none added here, deliberately** — `happenstance-sync-testkit` does not
  exist until HS-S0103, and Context pack §9 gives the reason the rules must not be written against this
  implementation. This story's behaviour is nevertheless adapter-observable, and the four rules that
  will observe it are already named by their clauses:
  `ingested_events_land_above_the_local_head` (SY-5), `redelivery_of_an_accepted_group_is_a_no_op`
  (SY-11), `dedupe_reaches_identity_without_decoding` (SY-12) and
  `push_envelope_preserves_group_boundaries` (SY-30). Each integration test this story writes must be
  shaped so that HS-S0103 can generalise it against a fixture rather than rewrite it.
- **Clause(s)**: **discharges in code, amends none.** SY-5, SY-11, SY-12, SY-15, SY-16 and SY-19 are
  `[FROZEN]` and are implemented as written; SY-1 and SY-6 are `[FROZEN]` and constrain the body's
  shape (no condition parameter, no conditional append path); SY-30 is `[PROVISIONAL]` and its group
  obligation must survive whichever `ingest` shape ADR-0026 authorises. **No clause text is edited in
  this diff** — the `(new)` marker drops and the `Rejects:` repairs belong to `frozen-clause-repairs`
  (HS-S0112), and every citation this diff invalidates is recorded for it.
- **Advances DoD scenario**: initiative **DoD 14** — *"Replication has an answer on disk"*
  (`initiative.md:398-402`). The atom is slice 1's; this story is the half that would not exist if the
  answer were wrong, and is what makes DoD 14 an implemented answer rather than a written one. It also
  lands leg one of project **DoD 4**'s proof artefact — the byte-identical payload round-tripped across
  a store boundary — with the remaining two legs (two structurally unlike peers) belonging to
  `durable-object-and-neon-peers`.

## PR boundary

```
crates/happenstance-sync/src/**
crates/happenstance-sync/tests/**
crates/happenstance-sync/Cargo.toml
CHANGELOG.md
.bklg/from-contract-to-published-library/replication-identity-and-ingest/ingest-store-and-memory-peer-round-trip/**
```

**In this PR**

- The four `IngestStore` bodies for `MemoryEventStore`, replacing
  `crates/happenstance-sync/src/ingest.rs:206-231`'s `todo!()`s.
- The identity unification: `identity::{StoreId, EventId, RecordedAt}` deleted in favour of
  `happenstance_core::{StoreId, EventId, RecordedAt}`, with every `identity::`-spelled site revisited
  and the crate-root note explaining the non-re-export (`lib.rs:149-153`) resolved rather than left
  describing a state that no longer exists.
- Whatever change to `IngestStore::ingest`'s signature ADR-0026 authorises for group atomicity, and its
  rustdoc.
- The `MemorySyncPeer` doctest showing one round trip with the resume token held outside the peer, plus
  whatever `MemorySyncPeer` changes the doctest and the round trip actually force (its bodies are
  already real — read `memory.rs` before assuming a rewrite).
- The end-to-end integration test across two `MemoryEventStore`s through one `MemorySyncPeer`:
  identity preserved, local tail, group atomicity, replay a no-op, payload bytes identical.
- Removal of `#![allow(clippy::todo)]` from `crates/happenstance-sync/src/lib.rs:135-139`.
- The `memory`-feature disposition (see *Behavior and interfaces*), applied to
  `crates/happenstance-sync/Cargo.toml` and `lib.rs` if it changes anything.
- A `CHANGELOG.md` entry: precedent exists for this crate (`CHANGELOG.md:1024`), and this is the change
  that makes the crate do something.
- Updates to `tests/ingest_reaches_a_foreign_store.rs`'s and `tests/real_peer_shapes.rs`'s prose where
  the unification falsifies it — notably the *"# The residue"* paragraph
  (`tests/ingest_reaches_a_foreign_store.rs:42-50`), which claims `SequencedEvent` has no field for an
  `EventId` and has been wrong since phase 4.
- The story's own `_ledger.md` and companion notes, including the **record of invalidated citations**
  handed to HS-S0112.

**Explicitly not in this PR**

- Any change to `crates/happenstance-core/**`. The door is HS-S0101's; `EventStore`'s trait signature
  is byte-identical before and after (architecture AC-A01).
- Any edit to `spec/SPECIFICATION.md`. Dropping `(new)` markers and repairing `Rejects:` lines whose
  named symbol this diff moved is `frozen-clause-repairs` (HS-S0112); this story records, it does not
  repair.
- `crates/happenstance-sync-testkit/**` and any conformance rule or mutant — HS-S0103 and HS-S0105.
- The wire message set and any derive on `PushBatch` / `EventGroup` / `ReplicatedEvent` — a derive on a
  public message type *is* a wire format (`crates/happenstance-sync/src/lib.rs:86-94`) and it is
  ADR-0027's and `message-set-on-the-envelope`'s.
- The sync runner, topologies, and the `tokio::spawn` `!Send` proof — `send-free-sync-runner` (HS-S0109).
- Any `.kb/` atom, including the ADR-0003 `provisional` lift — atoms arrive through `.kb/_intake/` and
  `/redkiln:kb-ingest`, and the lift is HS-S0111's.
- `xtask/**` gate constants — `gate-mounts-for-the-sync-suite` (HS-S0104).

**Merge DoD.** `cargo xtask affected --base main` and `cargo xtask lints && cargo xtask spec-trace` are
green, the crate carries no `todo!()` and no crate-level `clippy::todo` allow, the round-trip and
replay tests pass with every position assertion anchored on a store-reported head, and the
`MemorySyncPeer` doctest runs in the gate rather than merely existing.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Identity unifies with the contract crate** | `identity::{StoreId, EventId, RecordedAt}` are deleted; `ReplicatedEvent` and `Watermark` stay and are re-typed against `happenstance_core`'s. The non-re-export existed so *"every site phase 4 must revisit"* is greppable by one name — phase 4 landed, so the grep is now the work list. Four `use` sites in `src/` and two in `tests/` spell `identity::` today. `holds` is blocked by nothing else: its own `todo!()` message says the sole obstruction is that the two `EventId`s are different types. | `crates/happenstance-sync/src/identity.rs:24-43`; `crates/happenstance-sync/src/lib.rs:149-153`; `crates/happenstance-sync/src/ingest.rs:224`; `crates/happenstance-core/src/lib.rs:110`; `crates/happenstance-core/src/event.rs:484-496` |
| **`store_id()` answers with the store's own incarnation** | Returns `MemoryEventStore::store_id()` directly. It is the first check ingest makes and the one that stops a replication loop: telling *"an event I minted, coming back to me"* from *"an event a peer minted"*. | `crates/happenstance-sync/src/ingest.rs:124-129`, `:215-217`; `crates/happenstance-core/src/memory.rs:106-107` |
| **`ingest()` preserves the foreign `EventId` and lands at the local tail** | Each event is written through HS-S0101's inherent `&self` door with its `id` and `recorded_at` carried through unchanged, and receives the *next local position* like any other write. Never `append` (which mints), never `restore` (which builds a new store), never an insertion ordered by origin position or timestamp. | SY-5 `spec/SPECIFICATION.md:5954-5971`; SY-19 `:6409-6427`; `crates/happenstance-sync/src/ingest.rs:52-71`, `:131-141` |
| **`ingest()` takes no condition and evaluates none** | The signature carries no `AppendCondition` parameter and the body never calls the store's conditional append path. `EventGroup::guard` is evidence and is not consulted by the receiver. An event already held is a skip; a content disagreement is not an available outcome. | SY-1 `spec/SPECIFICATION.md:5841-5867`; SY-6 `:5973-6029`; `crates/happenstance-sync/src/peer.rs:230-243`; `crates/happenstance-sync/src/ingest.rs:143-149` |
| **Group atomicity survives, and the shape is ADR-0026's to authorise** | Today `ingest` takes a flat `&[ReplicatedEvent]` while the trait promises *"each group lands atomically or not at all"*. Admissible resolutions: carry the group boundary into `ingest`, or compose atomicity above a flat `ingest` per `EventGroup`. Constraint on both: the receiver never re-infers the decomposition, and a mid-batch failure never publishes a state the origin never had. Read ADR-0026 first; if it is silent, raise rather than choose. | `crates/happenstance-sync/src/ingest.rs:133-138`; `crates/happenstance-sync/src/peer.rs:183-190`; SY-30 `spec/SPECIFICATION.md:6700-6725`; cases E2E-35, E2E-39 (`spec/E2E-CASES.md:913`, `:1018`) |
| **Re-delivery is a no-op, deduped on `EventId`** | Second delivery of an already-held event yields `Ingested { appended: 0, skipped: n, last_local: None }` for a fully-repeated batch: no second copy, no compensation, no error. Dedupe reaches the identity without decoding `Event::data` or `Event::metadata`, via `contains_event_id`. Content-, hash- and metadata-borne dedupe are each forbidden by name. | SY-11 `spec/SPECIFICATION.md:6135-6152`; SY-12 `:6153-6171`; SY-13 `:6211`; `crates/happenstance-core/src/memory.rs:414`; cases E2E-33, E2E-34, E2E-36 |
| **`holds()` and `watermark()` are derived, never counters** | `holds` delegates to `contains_event_id`. `watermark()` is computed from what was actually ingested — the highest position seen per origin `StoreId` — *"so it cannot drift from the log"*, and is monotonic (`Watermark::advance` raises only on an advance). A stored side counter is the wrong implementation. | `crates/happenstance-sync/src/ingest.rs:152-175`; `crates/happenstance-sync/src/identity.rs:208-254` |
| **`Ingested.last_local` is arrival order** | Local, and unrelated to the position inside the events' own `EventId`s. Asserted against a head the receiving store reported *before* the ingest, never against a literal. | `crates/happenstance-sync/src/ingest.rs:178-191`; CF-6, CLAUDE.md *The rule that matters* |
| **One round trip, resume token outside the peer** | `MemorySyncPeer::pull` already stops at the first group that does not fit its budget rather than truncating one, and returns an owned `MemoryResume`. The doctest must carry that token in a local binding across two calls, demonstrating the shape SY-16 requires. The port *cannot express* "one round trip with no held state" — only a counting fixture proves it, and that fixture is HS-S0103's. | SY-15/SY-16 `spec/SPECIFICATION.md:6292-6340`; `crates/happenstance-sync/src/memory.rs:19-42`, `:166-188`; `crates/happenstance-sync/src/peer.rs:43-50` |
| **The payload is opaque at every hop** | `Event::data` and `Event::metadata` are compared as `Bytes` for equality and never inspected structurally, at any hop of the test or the implementation. The receiver's copy is asserted byte-identical to the origin's — the measurement ADR-0003's lift condition names. | DR-5 (`project.md`); `.kb/decisions/0003-opaque-payloads.md`; `crates/happenstance-sync/src/identity.rs:168-170`; `RUNBOOK.md:4576-4580` |
| **Generic code binds the weaker flavour** | `impl SendIngestStore for MemoryEventStore` stays (the type is `Send`, and the bare flavour comes free); every *bound* this story writes names `IngestStore` / `SyncPeer` / `EventStore`. One name of each pair per module. The crate must still build for `wasm32-unknown-unknown` (`cargo xtask wasm`'s existing steps cover the sync crate's dependency surface; the dedicated sync step is HS-S0104's). | ADR-0001 `.kb/decisions/0001-async-port-flavours.md`; CLAUDE.md constraints 1 and 4; SY-17 `spec/SPECIFICATION.md:6341`; `crates/happenstance-sync/src/peer.rs:26-28` |
| **The scoped allow comes out with the bodies** | `#![allow(clippy::todo)]` is deleted from the crate root in this change. With it gone, `clippy --workspace --all-targets -- -D warnings` fails on any surviving `todo!()` anywhere in the crate — including the two stand-ins in `tests/real_peer_shapes.rs`, whose disposition this story must therefore settle (retire the `todo!()`s, keep the finding). | `crates/happenstance-sync/src/lib.rs:135-139`; `crates/happenstance-sync/tests/real_peer_shapes.rs:16-20`; `_decomposition.md`, *Composition root* §4 |
| **`memory`-feature disposition: settled here** | `RUNBOOK.md:4581-4585` asks for `MemorySyncPeer` *behind a `memory` feature*; `lib.rs:145` declares `pub mod memory;` unconditionally; `Cargo.toml:37-44` scopes the existing `memory` feature to the `IngestStore for MemoryEventStore` coherence proof, because that impl needs `happenstance-core/memory`. **Disposition: keep both as they are.** `MemorySyncPeer` depends on nothing feature-gated — it is a peer, not a store — so gating it would cost a feature for no compilation benefit and would hide the oracle from the crate's own doctests; the `memory` feature keeps its existing, narrower meaning. The RUNBOOK line is satisfied in substance (the oracle is available to an application author) and the divergence is recorded here rather than taken silently. Both answers stay cheap while `publish = false`. | `RUNBOOK.md:4581-4585`; `crates/happenstance-sync/Cargo.toml:12`, `:37-44`; `crates/happenstance-sync/src/lib.rs:143-147`; `_decomposition.md`, *Composition root* §4 |
| **Documentation obligations** | Every public item that changes carries rustdoc; every fallible public function carries `# Errors` naming the conditions rather than the error type; the `MemorySyncPeer` doctest is compiled *and run* by the gate. `happenstance-sync` is `publish = false`, so its doctests need the out-of-package harness RS-62-5 names rather than a default `cargo test --doc`, which skips a crate that never publishes. | `standards/rust/70-rustdoc-obligations.md`; `standards/rust/62-doctests-and-harnesses.md` (RS-62-5); `crates/happenstance-sync/Cargo.toml:12` |
| **Findings move before prose is deleted** | `crates/happenstance-sync/src/ingest.rs:20-82` and the header of `tests/ingest_reaches_a_foreign_store.rs` stop being true in this change. Each finding is moved into the ADR that consumed it before its paragraph is deleted, and every `spec/SPECIFICATION.md` clause or test that cites the deleted lines is listed in this story's companion record for HS-S0112. | `_decomposition.md`, *Notes* — *"One thing to carry forward that no AC captures"*; `RUNBOOK.md:462-466`; discover.md *Gate: Discover*, Box 7 |
| **The oracle's assertions share no subroutine with the implementation** | The round-trip test's expectations are written from the clauses, not by reading `MemorySyncPeer`'s or the ingest body's code and restating it. `FlatteningIngest` is invisible to every assertion over the log's final state, so the group-boundary check must observe the log *during* ingest or inject a mid-batch fault. | `standards/rust/60-what-a-test-must-prove.md` (RS-60-4); discover.md, *The wrong implementation* |

## Data and migrations

**No persistent data, no schema, no runtime migration.** Both stores in this story are
`MemoryEventStore`; nothing is written to disk, no wire format is defined or bumped here
(`FORMAT_VERSION`'s disposition is `message-set-on-the-envelope`'s), and no serialised representation
of `ReplicatedEvent`, `EventGroup` or `PushBatch` exists yet — those types still carry **no** derives,
deliberately (`crates/happenstance-sync/src/lib.rs:86-94`).

There are two *source-level* migrations, and both are compile-time-total rather than data migrations:

1. **The identity migration.** `identity::{StoreId, EventId, RecordedAt}` are removed in favour of
   `happenstance_core`'s. Every site is inside `happenstance-sync` (four in `src/`, two in `tests/`);
   nothing outside the crate can be broken by it, because the three types were deliberately never
   re-exported from the crate root — which is exactly the property `identity.rs:34-40` says the
   non-re-export was bought for. The crate is `publish = false`, so there is no downstream consumer and
   no deprecation arm is owed.
2. **The seam migration.** Ingest stops being unimplementable and starts calling HS-S0101's inherent
   door. `MemoryEventStore::restore` remains the only other path that preserves a foreign identity and
   remains unsuitable here: it builds a **new** store from an owned snapshot while `ingest` holds
   `&self` (`crates/happenstance-sync/src/ingest.rs:56-62`). An implementation that reaches for
   `restore` is `RestoreBasedIngest`, named as a wrong implementation in HS-S0101's discovery.

`happenstance-core`'s stored representation is untouched: `SequencedEvent` already carries `position`,
`id`, `recorded_at` and `event` as of phase 4 (`crates/happenstance-core/src/event.rs:484-496`), which
is why this story needs a write path and not a value-type change.

## Acceptance criteria

Every criterion is a persona goal crossing the whole stack — origin store, wire value, peer, ingest,
receiving store — not a capability restated. The personas are the initiative's own
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`): **P2**
the adapter author, who needs an executable definition of *correct* rather than a prose specification
to interpret (`:114-131`); **P3** the local-first / edge developer, who must event-source on a
non-`Send` runtime without hand-rolling the layer (`:182-196`); **P4** the evaluator, who is deciding
whether the replication answer is written down or merely intended. Every position assertion below
anchors on a head the receiving store reported *before* the ingest (CF-6, DR-7); the word "literal"
appears in no verification.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **P2 stops holding two incompatible identities in their head.** GIVEN an adapter author reading `happenstance-sync` to learn what a replicated event *is*, WHEN they follow `ReplicatedEvent` to the type of its `EventId`, THEN there is exactly one `EventId` in the workspace — `happenstance_core::EventId` — with `identity::{StoreId, EventId, RecordedAt}` deleted, `ReplicatedEvent` and `Watermark` retained and re-typed, every `identity::`-spelled site in `src/` and `tests/` revisited, and the crate-root note that explained the deliberate non-re-export (`crates/happenstance-sync/src/lib.rs:149-153`) resolved rather than left describing a state that no longer exists. | `rg -n "identity::(StoreId|EventId|RecordedAt)" crates/happenstance-sync` returns nothing; `cargo test -p happenstance-sync --all-features` green; `crates/happenstance-sync/tests/real_peer_shapes.rs::replicated_event_carries_the_contract_crates_identity` |
| AC-002 | **P4 watches an event leave one store and arrive in another with its authorship intact.** GIVEN two independent `MemoryEventStore`s, each having already appended events of its own, WHEN a batch from the origin is pushed through `MemorySyncPeer`, pulled, and ingested into the receiver, THEN each arriving event is readable from the receiver carrying its **origin's** `EventId` unchanged, and is assigned a *local* position strictly above the head the receiver reported immediately before the ingest — never a position derived from, ordered by, or equal to the origin's. | `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::foreign_identity_survives_and_lands_above_the_local_head` — receiver pre-seeded to a non-trivial head, the head captured into a binding before ingest and every position compared against that binding; SY-5 `spec/SPECIFICATION.md:5954-5971`, SY-19 `:6409-6427` |
| AC-003 | **P2 learns that ingest cannot refuse them for a reason they cannot see.** GIVEN an origin `EventGroup` whose `guard` carries an `AppendCondition` that the *receiving* store's state would falsify, WHEN that group is ingested, THEN it lands in full: `IngestStore::ingest`'s signature carries no condition parameter, the body reaches no conditional append path, `EventGroup::guard` is carried as evidence and never consulted, and the only `Err` the adapter can produce is a storage failure — a content or state disagreement is not an available outcome, and every fallible public function's rustdoc `# Errors` says so in those terms. | `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::a_guard_that_the_receiver_would_falsify_is_still_ingested` (guard populated with a position-relative condition, per the testing brief's *Fixtures and seams* row); a compile-level check that no `AppendCondition` appears in the `ingest` signature; SY-1 `spec/SPECIFICATION.md:5841-5867`, SY-6 `:5973-6029` |
| AC-004 | **P1/P4 never observe a state the origin never had.** GIVEN a `PushBatch` of two or more `EventGroup`s where a decision in group *n+1* was only legal because group *n* had landed whole, WHEN a fault is injected mid-group during ingest, THEN no partial group is visible to any reader of the receiving store — the group either lands entire or not at all — and the receiver never re-infers the decomposition from event order. Whichever shape ADR-0026 authorises (group boundary carried into `ingest`, or atomicity composed above a flat `ingest` per `EventGroup`) is the one implemented, and its rustdoc states which. | `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::a_failed_group_publishes_nothing` — a fault injected inside a group, then the receiver's log read for any member of that group; the oracle written from SY-30 rather than from the ingest body (RS-60-4). The named wrong implementation this rejects is `FlatteningIngest`, invisible to every assertion over the log's final state. SY-30 `spec/SPECIFICATION.md:6700-6725` |
| AC-005 | **P3 reconnects after a dropped edge session and the replay costs nothing.** GIVEN a batch already fully ingested by the receiver, WHEN the identical batch is delivered a second time — the normal case at the edge, not the exceptional one — THEN the outcome is `Ingested { appended: 0, skipped: n, last_local: None }`: no second copy, no compensating event, no `Err`; and the skip decision is reached through the store-assigned `EventId` via `contains_event_id`, with `Event::data` and `Event::metadata` untouched on that path. A partial re-delivery (some held, some new) appends exactly the new ones and skips exactly the held ones. | `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::replaying_the_same_batch_is_a_no_op` and `::a_partial_redelivery_appends_only_the_unheld` — both asserting on *which identities* the receiver holds, never on how many events it holds. Rejects `ContentDedupingIngest` (discovery, *The wrong implementation*). SY-11 `spec/SPECIFICATION.md:6135-6152`, SY-12 `:6153-6171`, SY-13 `:6211` |
| AC-006 | **P4 gets the evidence ADR-0003's lift condition asks for.** GIVEN an event whose `data` and `metadata` are arbitrary bytes that are not valid UTF-8 and not valid JSON, WHEN it crosses the store boundary and is read back from the receiver, THEN its `Bytes` compare equal to the origin's byte for byte, and no assertion on any path — implementation or test — inspects those bytes structurally. | `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::the_payload_bytes_are_identical_on_the_far_side`, with a deliberately non-decodable payload as the negative control that would fail if any path parsed it; DR-5 (`project.md`), `.kb/decisions/0003-opaque-payloads.md`. The lift atom itself is HS-S0111's and is **not** written here |
| AC-007 | **P3 meets the round trip first as a compiled example, not as prose.** GIVEN a reader opening `MemorySyncPeer`'s rustdoc, WHEN they read the first example, THEN they see one complete push/pull round trip in which the `MemoryResume` token is held in a **local binding outside the peer** and handed back on the next call — the shape SY-16 requires — and that example is compiled *and executed* by the gate rather than merely rendered, despite the crate's `publish = false` requiring the out-of-package doctest harness RS-62-5 names. | The doctest on `MemorySyncPeer` in `crates/happenstance-sync/src/memory.rs`, run through the RS-62-5 harness (`standards/rust/62-doctests-and-harnesses.md`); asserted present and executed by `cargo xtask ci --fast`. Rejects `HeldCursorMemoryPeer` — the oracle is the worked example a first adapter author copies. SY-15/SY-16 `spec/SPECIFICATION.md:6292-6340` |
| AC-008 | **P2 can trust the resume point they were handed.** GIVEN a receiver that has ingested batches from two different origin stores, WHEN `watermark()` is asked what it has seen, THEN the answer is *derived from what was actually ingested* — the highest position observed per origin `StoreId` — so it cannot drift from the log, is monotonic under re-delivery and out-of-order arrival (a replay never lowers it), and `holds()` answers by delegating to `contains_event_id` rather than to any stored side counter. | `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs::the_watermark_is_derived_and_monotonic` — two origins, an out-of-order and a replayed batch, the watermark compared against origin-reported positions rather than literals; unit coverage of `Watermark::advance` in `crates/happenstance-sync/src/identity.rs` |
| AC-009 | **P4's "is it implemented, or is it a skeleton?" is answered by the compiler.** GIVEN the crate after this change, WHEN `cargo clippy --workspace --all-targets -- -D warnings` runs, THEN it is green *with* `#![allow(clippy::todo)]` deleted from `crates/happenstance-sync/src/lib.rs:135-139` — which means no `todo!()` survives anywhere in the crate, including the two stand-ins in `tests/real_peer_shapes.rs:16-20`, whose disposition is settled in this change (the `todo!()`s retired, the finding they recorded kept). | `cargo clippy --workspace --all-targets -- -D warnings`; `rg -n "todo!\(" crates/happenstance-sync` returns nothing; `rg -n "allow\(clippy::todo\)" crates/happenstance-sync` returns nothing |
| AC-010 | **P3's runtime survives the feature they came for.** GIVEN the ingest path as this story leaves it, WHEN the crate is built for `wasm32-unknown-unknown` and when any generic helper this story writes is instantiated, THEN every *bound* names `IngestStore` / `SyncPeer` / `EventStore` and never a `Send` flavour, one name of each pair is in scope per module, no `#[async_trait]` is introduced, and the build is green — `impl SendIngestStore for MemoryEventStore` staying is correct and not a counter-example, because the type is `Send` and the bare flavour comes free. | `cargo xtask wasm` (existing steps; the dedicated sync step is HS-S0104's); `rg -n "Send(EventStore|SyncPeer|IngestStore)" crates/happenstance-sync/src` reviewed against the impl sites only; ADR-0001 `.kb/decisions/0001-async-port-flavours.md`, SY-17 `spec/SPECIFICATION.md:6341` |
| AC-011 | **P4 finds the record still true after the sketch stops being one.** GIVEN that implementing this crate falsifies most of `crates/happenstance-sync/src/ingest.rs:20-82` and the header of `tests/ingest_reaches_a_foreign_store.rs`, WHEN a paragraph of that prose is deleted, THEN the finding it held has already been moved into the ADR that consumed it; every `spec/SPECIFICATION.md` clause and test that cites a deleted line is listed in this story's companion record for `frozen-clause-repairs` (HS-S0112); the `memory`-feature divergence from `RUNBOOK.md:4581-4585` is recorded as a decision rather than taken silently; every changed public item carries rustdoc with `# Errors` naming conditions rather than error types; and a `CHANGELOG.md` entry names the change that makes the crate do something. | `_invalidated-citations.md` in this story's own directory, non-empty and each row resolving to a real `file:line` at HEAD; `cargo xtask spec-trace` green; `cargo doc --no-deps -p happenstance-sync` warning-free; `CHANGELOG.md` diff present in the PR |

**Coverage of the traced project AC.** All eleven serve project **AC-002** — *the central question has a
written answer* (`project.md`, *Acceptance criteria*). AC-003 and AC-004 are the answer made
operational; AC-002, AC-005, AC-006 and AC-008 are the behaviour that would be different if the answer
were wrong; AC-001, AC-009, AC-010 and AC-011 are the conditions under which that operational answer is
*checkable* rather than asserted. **No frozen clause is edited in this diff**, which is AC-002's second
half and is verified by `cargo xtask spec-trace` plus the empty `spec/SPECIFICATION.md` diff.

## Interaction quality

**RFC §6.7/D6, and this story renders no surface.** `_design.md` records the no-surface determination
for the whole project and its `## Items` / `## Signatures` / `## Anti-patterns` blocks all read *"N/A —
no user-facing surface"* — and that determination is what a human signed off
(`.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md`, *Sign-off*).
There is no route, DOM node or TUI pane here, so the STATE family has no referent and the COMPOSITION
family is inherited in the medium `_design.md` explicitly redirects to: the **public API surface**, held
to `standards/rust/70-rustdoc-obligations.md`.

This section lists **which AC carries which invariant**. Every one of them is a row in the table above;
none is stated only here, because `redkiln verify` extracts ACs from table cells and bullets in the
acceptance-criteria section and a prose bullet here would never be gated.

| Family | Invariant, in this medium | Carried by | How it is verified |
| --- | --- | --- | --- |
| STATE | *In-place vs context-jump* — N/A. No navigation exists. The nearest analogue is that ingest never *replaces* the receiving store: it appends in place, and the one API that would build a new store from a snapshot (`MemoryEventStore::restore`) is forbidden here. | AC-002 | `::foreign_identity_survives_and_lands_above_the_local_head` — the receiver's pre-existing events are still present and still at their original positions afterwards |
| STATE | *Non-occlusion* — an ingested event never hides or displaces an event the receiver already had, and never lands below one. | AC-002 | same test; SY-5's `Rejects:` is order-preserving insertion by origin position |
| STATE | *Preserved selection / reversibility* — the analogue is idempotence: repeating an operation returns the caller to the same observable state, with no compensating event minted to "undo" a duplicate. SY-11's `Rejects:` is an inversion precisely because *"a duplicate is visible; an inversion looks like a decision"*. | AC-005 | `::replaying_the_same_batch_is_a_no_op`, `::a_partial_redelivery_appends_only_the_unheld` |
| STATE | *Preserved caller state across a boundary* — the resume token survives the handle. The port's whole shape rests on the caller keeping state the peer does not. | AC-007 | the `MemorySyncPeer` doctest, token in a local binding across two calls |
| STATE | *Keyboard reachability* — N/A (no input surface). Its API analogue is that every capability this story lands is reachable through the declared mount point, `crates/happenstance-sync/src/lib.rs`; a body that exists but is not re-exported through that root is not mounted. | AC-009, AC-010 | `cargo clippy … -D warnings` with the allow gone, plus `cargo xtask wasm` compiling the path |
| COMPOSITION | *Presentation exists at all* — the API analogue of "not bare markup": every public item this story changes carries real rustdoc, and every fallible one carries `# Errors` naming the **conditions**, not the error type. An undocumented `pub fn` is this medium's unstyled render. | AC-003, AC-011 | `cargo doc --no-deps -p happenstance-sync` warning-free; `# Errors` present on each fallible public fn |
| COMPOSITION | *What a reader meets first* — the doctest is the composed artefact, not the signature list. This is the first place in the repository a reader can watch an event leave one store and arrive in another, and a doctest that is rendered but never executed is documentation rather than evidence. | AC-007 | the doctest compiled **and run** through the RS-62-5 out-of-package harness inside `cargo xtask ci --fast` |
| COMPOSITION | *Placement and transience* — `ReplicatedEvent` and `Watermark` are persistent chrome (replication's own, they stay); `identity::{StoreId, EventId, RecordedAt}` were revealed-on-demand scaffolding with a stated expiry and are removed on schedule. Nothing new is exported from the crate root that ADR-0026 has not authorised. | AC-001 | `rg` for the placeholder trio returns nothing; the crate-root note resolved |
| COMPOSITION | *Density budget, with its real numbers* — this story adds **zero** new public types to the crate root, **at most one** signature change (`IngestStore::ingest`, only if ADR-0026 authorises the group-boundary shape), **four** trait bodies, **one** doctest, and **removes** three placeholder types and one crate-level `allow`. Any addition beyond that budget is a scope change, not an implementation detail. | AC-001, AC-004, AC-009 | the PR boundary above, read against `git diff --stat` on `crates/happenstance-sync/src/lib.rs` |
| COMPOSITION | *Hierarchy* — the contract crate's identity types outrank this crate's; `happenstance-core`'s trait signature is byte-identical before and after, and no change here reaches across the dependency rule. | AC-001, AC-010 | the empty `crates/happenstance-core/**` diff (architecture AC-A01) |
| COMPOSITION | *Named anti-patterns* — `_design.md` names none (no surface), so the binding set is discovery's three: `ContentDedupingIngest`, `FlatteningIngest`, `HeldCursorMemoryPeer`. Each is rejected by an AC rather than by a comment. | AC-005, AC-004, AC-007 respectively | the three tests named in those rows; each written from the clause, sharing no subroutine with the implementation (RS-60-4) |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | A storage failure occurs partway through a group. | The whole group is absent from the receiver — no partial group is ever readable (AC-004). The adapter returns its `Error`, which exists for storage failures **only**; already-completed *earlier* groups in the same batch remain landed, and `Ingested` reports honestly what did land. |
| EC-002 | An event in the batch is one the receiver already holds. | A **skip**, never an error and never a second copy. It contributes to `skipped`, not to `appended`, and does not lower `watermark()` (AC-005, AC-008). `crates/happenstance-sync/src/ingest.rs:143-149` states this in the trait's own prose. |
| EC-003 | An arriving event's `EventId` names the **receiving** store as its origin — an event this store minted, coming back. | Handled by the same identity check, not by a special case: if held, it is a skip (EC-002). If it is *not* held, that is a store that has forgotten something it minted, which is `retention-and-incomplete-logs`' territory (HS-P0018) and outside this project. Behaviour here must be stated in `ingest`'s rustdoc and recorded in the companion notes; it must not be guessed at silently. `store_id()` is what makes the case distinguishable (`crates/happenstance-sync/src/ingest.rs:124-129`). |
| EC-004 | The batch, or a group within it, is empty. | Not an error: `Ingested { appended: 0, skipped: 0, last_local: None }`. `last_local` is `None` whenever nothing was appended, including a fully-skipped replay. |
| EC-005 | A pulled group does not fit the caller's `PeerLimits` budget. | `MemorySyncPeer::pull` already stops *at the group boundary* rather than truncating a group (`crates/happenstance-sync/src/memory.rs:166-188`) — keep it. A single group that exceeds the budget on its own must not be silently truncated or silently dropped; if the current body does neither correctly, that is a defect to fix in this change with the reason given (CLAUDE.md, *The rule that matters*). |
| EC-006 | ADR-0026 lands without settling the group-atomicity expression (Context pack §5). | **Stop and raise.** Do not pick between carrying the group boundary into `ingest` and composing atomicity above a flat `ingest`: it is a port shape and `crates/happenstance-sync/src/lib.rs` is where the choice becomes public. Raise a decision atom through `.kb/_intake/` and a re-plan. |
| EC-007 | HS-S0101's inherent `&self` door is absent, or turns out to be insufficient for an already-identified `SequencedEvent`. | **Stop and raise** — `RUNBOOK.md:462-466`'s residual risk landing hard. Do not widen `EventStore::append`, do not reach for `MemoryEventStore::restore` (the `RestoreBasedIngest` wrong implementation), and do not add a trait method: SY-8 (`spec/SPECIFICATION.md:6070-6086`) and architecture AC-A01 both forbid it. |
| EC-008 | A `spec/SPECIFICATION.md` clause or an existing test cites a line this diff deletes. | Record it in `_invalidated-citations.md` and continue (AC-011). Repairing it here would be an edit to a `[FROZEN]` clause's neighbourhood without the ADR that authorises it; the repair is `frozen-clause-repairs`' (HS-S0112). |

## Non-functional

| id | Requirement | Evidence |
| --- | --- | --- |
| NF-001 | **`happenstance-core`'s public surface is byte-identical before and after this story.** The door is HS-S0101's; this story consumes it and adds nothing to the contract crate. | An empty `crates/happenstance-core/**` diff; architecture brief AC-A01 |
| NF-002 | **No `Send` bound is introduced on any path, and no `#[async_trait]` anywhere.** CLAUDE.md binding constraint 1 is the standing guard on the `wasm32` target this library's third persona depends on. | `cargo xtask wasm`; `rg -n "async_trait" crates/happenstance-sync` empty |
| NF-003 | **No payload byte is decoded on any path** — implementation or test. `Bytes` are compared for equality only. | AC-006's negative control: a payload that is neither valid UTF-8 nor valid JSON, so any parsing path fails loudly rather than passing by luck |
| NF-004 | **No new entry in `[dependencies]`.** The crate's dependency surface is already `happenstance-core`, `serde`, `thiserror`, `trait-variant` (`crates/happenstance-sync/Cargo.toml:14-25`); implementing four bodies needs none of it widened. `publish = false` is retained, so neither sync crate is dragged into the licence-and-README package assertion. | `git diff crates/happenstance-sync/Cargo.toml`; `cargo package --list` step inside `cargo xtask ci --fast` |
| NF-005 | **Dedupe is a lookup, not a scan.** `holds` answers through `contains_event_id` (`crates/happenstance-core/src/memory.rs:414`) rather than by walking the receiver's log per incoming event. The requirement is on the *shape* — an identity lookup — not on a measured constant; a measurement, if wanted, belongs in `experiments/` and outside the gate. | Code review against `crates/happenstance-sync/src/ingest.rs:152-175`; the `watermark` derivation likewise computed from what was ingested, never from a stored counter (AC-008) |
| NF-006 | **The doctest runs in the gate.** `happenstance-sync` is `publish = false`, so a default `cargo test --doc` skips it entirely; the out-of-package harness RS-62-5 names is what makes AC-007 evidence rather than decoration. | `standards/rust/62-doctests-and-harnesses.md` (RS-62-5); the doctest observed failing when deliberately broken |
| NF-007 | **Story-grain gate cost stays inside `cargo xtask affected --base main`.** This story touches one crate and its tests; the affected set should be `happenstance-sync` plus its reverse dependencies, not the workspace. If it is not, something reached outside the PR boundary. | `cargo xtask affected --base main` output inspected, not just its exit code |

## Implementation notes (non-prescriptive)

Shape suggestions only; the ACs are the contract.

- **Read three files before writing a line.** `crates/happenstance-sync/src/memory.rs` (the peer's
  bodies are already real — this story adds a doctest and an exercise, not a rewrite),
  `crates/happenstance-sync/src/ingest.rs:20-82` (the phase-2 findings, which must be *moved* before
  they are deleted), and HS-S0101's landed door. The four `todo!()` messages at `ingest.rs:206-231`
  are the work list, in order.
- **Do the identity unification first.** `holds`'s own `todo!()` says the sole obstruction is that the
  two `EventId`s are different types (`:224`). Unify, then three of the four bodies become
  near-mechanical and the fourth (`ingest`) is the only one with a real design question in it.
- **Take the allow out early, not last.** Deleting `#![allow(clippy::todo)]` before the bodies exist
  turns clippy into the work list and makes "did I miss one" a compiler question rather than a memory
  question.
- **Write the group-atomicity test before the group-atomicity body.** `FlatteningIngest` passes every
  assertion over the final state of the log, so the test has to be able to observe the log *during*
  the ingest or inject a mid-batch fault. If that test is written after the body, it will be written
  by someone who already knows the body is right — RS-60-4's exact warning.
- **Write the assertions from the clauses, with the clause text open.** SY-5, SY-11, SY-12, SY-15,
  SY-16, SY-19 and SY-30 each carry a `Rejects:` line naming the wrong implementation; those lines are
  better test names than anything derived from reading the implementation.
- **`_invalidated-citations.md` is cheapest as you go.** Every time a line is deleted, `rg` for its
  content in `spec/` and `crates/*/tests/` before the deletion, not after — after, the string is gone
  and the citation is a line number pointing at something else.
- **The `memory`-feature disposition is already made** (*Behavior and interfaces*, last-but-two row):
  keep `pub mod memory;` unconditional and keep the `memory` feature scoped to the coherence proof.
  The work here is recording the divergence from `RUNBOOK.md:4581-4585`, not re-deciding it.

## Tests and CI (merge gate)

Grounded in the project testing brief (`_decomposition.md`, *The test mix, tier by tier* and
*Merge-gate commands*). This story is in the **Unit** and **Integration** tiers; it adds no static
gate step (that is HS-S0104's) and no conformance rule (that is HS-S0103's).

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static | `cargo xtask affected --base main` | The story grain: only what this diff could break. NF-007 — the affected set is one crate and its reverse dependencies, not the workspace. |
| Static | `cargo clippy --workspace --all-targets -- -D warnings` | AC-009 — with `#![allow(clippy::todo)]` deleted, this is the mechanical proof no body was left behind, including the two test stand-ins. |
| Static | `cargo xtask lints && cargo xtask spec-trace` | `reachability_static` (`.redkiln/config.yaml:48`), run unconditionally at story grain. `lint-position-literals` (CF-6) covers DR-7; `spec-trace` proves AC-002's second half — no `[FROZEN]` clause was edited and no `Rule:` citation was orphaned by this diff. |
| Static | `cargo xtask wasm` | AC-010, NF-002 — the ingest path builds for `wasm32-unknown-unknown` and no `Send` bound crept in. |
| Static | `cargo doc --no-deps -p happenstance-sync` | AC-011's documentation half and the COMPOSITION "presentation exists at all" invariant — no undocumented public item, `# Errors` present on each fallible one. |
| Unit | `crates/happenstance-sync/src/identity.rs` (`#[cfg(test)]`) | AC-008 — `Watermark::advance` is monotonic and raises only on an advance; AC-001 — `ReplicatedEvent` re-typed against `happenstance_core`'s identity. |
| Unit (doctest) | The `MemorySyncPeer` doctest in `crates/happenstance-sync/src/memory.rs`, via the RS-62-5 out-of-package harness | AC-007, NF-006 — one round trip, the `MemoryResume` token held in a local binding outside the peer, compiled **and executed** by the gate despite `publish = false`. |
| Integration | `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs` | The story's spine: AC-002 (`::foreign_identity_survives_and_lands_above_the_local_head`), AC-003 (`::a_guard_that_the_receiver_would_falsify_is_still_ingested`), AC-004 (`::a_failed_group_publishes_nothing`), AC-005 (`::replaying_the_same_batch_is_a_no_op`, `::a_partial_redelivery_appends_only_the_unheld`), AC-006 (`::the_payload_bytes_are_identical_on_the_far_side`), AC-008 (`::the_watermark_is_derived_and_monotonic`). Every position assertion anchored on a head the receiver reported before the ingest. |
| Integration | `crates/happenstance-sync/tests/real_peer_shapes.rs` | AC-001 — the `identity::`-spelled sites revisited and the *"# The residue"* paragraph corrected; AC-009 — the two `todo!()` stand-ins retired with their finding kept. |
| Integration | `cargo test -p happenstance-sync --all-features` | The whole crate including the `memory`-feature coherence proof, which is the feature the `IngestStore for MemoryEventStore` impl lives behind (`crates/happenstance-sync/Cargo.toml:37-44`). |
| Integration (project ceiling) | `cargo xtask ci --fast` | `integration_scoped` (`.redkiln/config.yaml:55`). Contains fmt, clippy, the full test run, the four `wasm32` steps, docs, `spec-trace`, the `--no-default-features` doc build and the `cargo package --list` assertion (NF-004). This project is not terminal, so `cargo xtask ci` is **not** this story's bar (DoD 1). |
| Ledger | `_ledger.md` in this story's directory, `require_ledger: true` (`.redkiln/config.yaml:62-67`) | Per DoD 3 — a green gate proves the gate passed, never on its own that the eleven criteria above are the things that work. Each row cites which command or test produced its evidence. |

**Not run here, deliberately.** `sync_peer_conformance!` (HS-S0103), the mutant registry (HS-S0105),
the sync-scoped `TESTKIT_SRC`/`RULE_FILES` gate constants and the two extra `wasm32` steps (HS-S0104),
and `redkiln validate --kb` as a *merge* gate — this story authors no atom, so it must leave
`validate --kb` exactly as green as it found it.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Containment inside this PR |
| --- | --- | --- |
| **ADR-0026 is silent on the group-atomicity shape**, and the implementer picks one to keep moving. | Medium / high — a port shape chosen by an implementer under time pressure becomes public in `lib.rs` and is expensive to move afterwards. | EC-006: stop and raise. The spec states both admissible shapes so that "raise" is cheap and specific, not an open question. |
| **HS-S0101's door does not fit an already-identified `SequencedEvent`**, and `EventStore::append` starts to look like the answer. | Low / very high — it would breach SY-8, architecture AC-A01 and CLAUDE.md's dependency rule in one move. | EC-007: stop and raise. NF-001 makes the breach visible as a non-empty `crates/happenstance-core/**` diff, which is the first thing review looks at. |
| **`FlatteningIngest` ships green.** Every counting assertion passes and the receiver publishes intermediate states the origin never had. | Medium / high — it is the *natural* implementation of a flat `&[ReplicatedEvent]` signature. | AC-004 is written as a mid-batch-fault observation rather than a final-state assertion, and the implementation note says to write that test first. |
| **`ContentDedupingIngest` ships green** because it needs no identity plumbing and passes every count-based replay test. | Medium / high — wrong in both directions and silently lossy. | AC-005 asserts on *which identities* the receiver holds; AC-001 removes the excuse by unifying `EventId` before the bodies are written. |
| **The oracle grows a held cursor**, and the first real adapter author copies it. | Medium / medium — `MemorySyncPeer` is in-process, so it passes every test in a process that stays alive. | AC-007 makes the doctest — the artefact that gets copied — carry the token outside the peer. `resume_survives_a_dropped_peer_handle` generalises it in HS-S0103; this story cannot prove it, and says so rather than pretending. |
| **Phase-2 findings are deleted with the prose that held them.** `ingest.rs:20-82` stops being true and is the obvious thing to remove. | High / medium — the prose is the evidence the sketch was landed nine phases early to produce (`RUNBOOK.md:462-466`), and it is unrecoverable once gone. | AC-011 makes moving the finding a precondition of deleting the paragraph, and the implementation note says to `rg` before the deletion rather than after. |
| **A position literal sneaks into an assertion**, because both stores here assign densely from one and the literal would be green. | Medium / high — it encodes a `MAY` as a `MUST` and is the project's headline error (CF-6, DR-7, discovery Box 6). | Every AC's verification column anchors on a store-reported head; `cargo xtask lint-position-literals` runs at story grain via `cargo xtask lints`. |
| **Scope creep into the wire.** Adding one `derive` to `PushBatch` would make the round trip serialisable and is one line. | Medium / medium — a derive on a public message type *is* a wire format (`crates/happenstance-sync/src/lib.rs:86-94`). | The PR boundary excludes it by name; it is ADR-0027's and `message-set-on-the-envelope`'s. |
| **Coupling to HS-S0101** is a hard sequence, not a soft one — the two are implemented in one context and merged in order. | Certain / contained | `_storymap.md`, *Merge order* §2. Nothing in this story is startable before the door exists; see *Dependencies*. |

## Dependencies

**Blocks on (must merge first):**

- **`memory-store-ingest-seam`** (HS-S0101) — supplies the one additive inherent `&self` operation on
  `MemoryEventStore` that accepts an already-identified `SequencedEvent`. Without it there is no
  truthful `ingest` body to write, and the `todo!()` at `crates/happenstance-sync/src/ingest.rs:52-71`
  says so in its own text. Slice-mate: the two are implemented in one context and merged in this order
  (`_storymap.md`, *Merge order* §2).
- *Transitively*, through HS-S0101: **`adr-0026-peer-ingest-and-transport`** (HS-S0100) — the atom that
  authorises the group-atomicity expression (Context pack §5) and the ingest promise this story turns
  into a body. `RUNBOOK.md:4606` and project DR-1 require it merged before the code it constrains.

**Unlocks (their `depends_on` names this story):**

- **`sync-testkit-crate-and-rule-registry`** (HS-S0103) — the conformance suite is pointed at
  `MemorySyncPeer` as its oracle, and a fixture whose methods are `todo!()` cannot be that oracle. The
  four rules this story names but does not write —`ingested_events_land_above_the_local_head`,
  `redelivery_of_an_accepted_group_is_a_no_op`, `dedupe_reaches_identity_without_decoding`,
  `push_envelope_preserves_group_boundaries` — generalise this story's integration tests against a
  fixture, so those tests must be shaped to be generalised rather than rewritten.
- **`send-free-sync-runner`** (HS-S0109) — the runner fans out over peers and advances the owned resume
  token; it needs a real `ingest` to call and the resume shape AC-007 demonstrates.

**Neither blocked on nor blocking:** `message-set-on-the-envelope`, `byte-identical-round-trip-and-idempotent-replay`
and `adr-0003-provisional-lift` (slice 4) consume this story's evidence but sit behind ADR-0027 and the
wire; `frozen-clause-repairs` (HS-S0112) consumes this story's `_invalidated-citations.md` and is
sequenced last on purpose.

## Anchors (progressive disclosure)

Open these at the moment named, not before. Every path verified present at HEAD.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `crates/happenstance-sync/src/ingest.rs` | The impl site itself: the trait's prose contract (`:124-191`), the four `todo!()` bodies and their stated reasons (`:206-231`), and the phase-2 findings at `:20-82` that must be moved before they are deleted. | First, before writing anything — the `todo!()` messages are the work list in order. | AC-003 |
| `crates/happenstance-sync/src/memory.rs` | `MemorySyncPeer`'s bodies are **already real**, and `MemoryResume` is already an owned `Copy` token (`:19-42`); `pull` already stops at a group boundary rather than truncating (`:166-188`). Assuming a rewrite is the most expensive wrong assumption available here. | Before touching the peer at all, and again when writing the doctest. | AC-007 |
| `crates/happenstance-sync/src/identity.rs` | Records which of the three placeholder types are phase 4's to delete and which (`ReplicatedEvent`, `Watermark`) are *"genuinely replication's own"* and stay — the boundary of the unification, written by the person who created it. | Before the identity unification, which is the first commit. | AC-001 |
| `crates/happenstance-sync/src/lib.rs` | The mount point and this story's composition root: the module declarations (`:143-147`), the re-exports (`:154-160`), the note explaining the deliberate non-re-export of the identity trio (`:149-153`), the scoped `allow` to delete (`:135-139`), and the standing warning that a derive on a public message type *is* a wire format (`:86-94`). | When mounting, and again before adding anything public. | AC-009 |
| `crates/happenstance-sync/src/peer.rs` | `PushBatch` (`:183-196`), `EventGroup` and its `guard` as *evidence, not instruction* (`:230-243`), and the note that the port **cannot express** "one round trip with no held state" — only a counting fixture can (`:43-50`). The last one is why AC-007 demonstrates rather than proves. | Before writing the group-atomicity body and before writing the doctest. | AC-004 |
| `crates/happenstance-sync/tests/ingest_reaches_a_foreign_store.rs` | The existing integration test this story extends, plus the *"# The residue"* paragraph (`:42-50`) that claims `SequencedEvent` has no field for an `EventId` — wrong since phase 4, and a citation this diff invalidates. | When writing the spine tests, and when compiling `_invalidated-citations.md`. | AC-002 |
| `spec/SPECIFICATION.md` | The six `[FROZEN]` clauses this story implements as written — SY-5 (`:5954-5971`), SY-11 (`:6135-6152`), SY-12 (`:6153-6171`), SY-15/SY-16 (`:6292-6340`), SY-19 (`:6409-6427`) — the two that constrain the body's shape, SY-1 (`:5841-5867`) and SY-6 (`:5973-6029`), and SY-30 `[PROVISIONAL]` (`:6700-6725`). Each `Rejects:` line names the wrong implementation and makes a better test name than anything read off the body. | With the clause text open while writing each test's assertions — not afterwards. | AC-005 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0026-peer-ingest-and-transport/spec.md` | The predecessor that authorises the group-atomicity expression and the ingest promise. If the merged atom is silent on the `ingest` shape, EC-006 fires and this is where the scope of the silence is established. | Immediately before choosing the `ingest` signature — the single highest-consequence decision in the story. | AC-004 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/memory-store-ingest-seam/spec.md` | The door's own contract: what HS-S0101's inherent `&self` operation accepts, what it promises about position assignment, and the `RestoreBasedIngest` wrong implementation it names. | Before the first line of the `ingest` body. | AC-002 |
| `crates/happenstance-core/src/memory.rs` | `contains_event_id` (`:414`) is how AC-005's dedupe reaches the identity without decoding a payload; `store_id()` (`:106-107`) is what `store_id()` delegates to; `restore` (`:163`) is the forbidden path and reading why makes the prohibition obvious rather than arbitrary. | When implementing `holds` and `store_id`, and when tempted by `restore`. | AC-005 |
| `standards/rust/60-what-a-test-must-prove.md` | RS-60-4 — spell the oracle in a direction that shares no subroutine with the implementation. This is the rule that makes AC-004's test have to observe the log mid-ingest instead of counting at the end. | Before writing the first assertion, and again if a test starts looking like a restatement of the body. | AC-004 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-5 — a `publish = false` crate's doctests are skipped by the default `cargo test --doc`, so the out-of-package harness is what makes AC-007 evidence rather than decoration. | When wiring the `MemorySyncPeer` doctest into the gate. | AC-007 |
| `standards/rust/70-rustdoc-obligations.md` | The API-surface obligation `_design.md` explicitly redirects to in place of a rendered-surface design: rustdoc on every public item, `# Errors` naming conditions rather than error types. | While writing the signatures, not as a cleanup pass afterwards. | AC-011 |
| `.kb/decisions/0001-async-port-flavours.md` | Why the two flavours exist and why generic code binds the weaker one — the constraint that keeps the `wasm32` target reachable for the edge persona. One name of each pair per module, or method calls go ambiguous. | Before writing any generic bound or helper. | AC-010 |
| `.kb/decisions/0003-opaque-payloads.md` | The lift condition AC-006's measurement is produced *for*. Reading it makes clear why the evidence is a byte comparison and why the atom itself must not be edited — the lift is a new atom, HS-S0111's. | When writing the byte-identity assertion, and before any temptation to touch the atom. | AC-006 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | The architecture brief's *Composition root* §4 (the oracle and the `memory`-feature discrepancy), §5 (the write-path seam and its four options, one of which was chosen), §7 (the wire mounts), and the testing brief's *The test mix* and *Fixtures and seams to mock* — including the row that says the `guard` must be exercised with both `Some` and `None`. | When the story's own text is ambiguous about tier, fixture treatment, or why an option lost. | AC-003 |
| `RUNBOOK.md` | `:4573-4595` (the phase-13 work list, including the `memory`-feature line this story diverges from), `:450-455` (why `EventStore` was refused a slot for a foreign identity), `:462-466` (the leak warning and the residual risk EC-007 escalates on), `:4576-4580` (the payload-opacity obligation). | When settling the feature disposition, and the moment a change starts reaching toward `happenstance-core`. | AC-011 |

## Clarifications resolved during spec

1. **The AC set is exactly the eleven the front half enumerated** — AC-001 through AC-011, unchanged.
   None was added, dropped or renumbered; the ledger matches one row per id.
2. **Where the two deferred discovery questions landed.** Discovery deferred the group-atomicity
   expression and the `memory`-feature disposition to spec. The feature disposition is **settled here**
   (keep `pub mod memory;` unconditional, keep the `memory` feature scoped to the coherence proof, and
   record the divergence from `RUNBOOK.md:4581-4585`) — it appears in *Behavior and interfaces* and is
   carried by AC-011. The group-atomicity expression is **not** settled here and deliberately so: it is
   a port shape and its authorisation is ADR-0026's. AC-004 states the obligation that must survive
   whichever shape lands; EC-006 states what to do if the atom is silent.
3. **How a no-surface project gets an *Interaction quality* section.** `_design.md` is binding and its
   binding content is the **no-surface determination itself**, signed off on 2026-08-12. The section is
   therefore written against the medium that file redirects to — the public API surface — and every
   invariant in it is carried by an AC row in the table above rather than by a prose bullet, so
   `redkiln verify` can extract and gate all of them. The design's *named anti-patterns* block reads
   "N/A", so the binding anti-pattern set is discovery's three, each bound to an AC.
4. **AC-002 is the only project AC traced, and all eleven story ACs serve it.** That is not
   over-tracing: project AC-002 has two halves — the answer exists, and no frozen clause was edited to
   make it exist — and this story is the half of the first that *would not exist if the answer were
   wrong* (`_storymap.md`, *Coverage*, AC-002 row). The other stories sharing AC-002 own different
   responsibilities: the ADR reconciles and cites, the seam supplies the door, the rules make it
   mechanical, and the repair story proves the clauses survived.
5. **`store_id()` on an echoed event needed a stated behaviour, not a body.** Discovery named
   `store_id()` as the check that stops a replication loop but never said what ingest *does* when an
   event this store minted comes back. EC-003 resolves it: the identity check already handles the held
   case as a skip, and the not-held case is a store that has forgotten something it minted — which is
   HS-P0018's territory. The requirement here is that `ingest`'s rustdoc states the behaviour and the
   companion notes record it; guessing silently is the failure mode.
6. **The two `todo!()` stand-ins in `tests/real_peer_shapes.rs` are in scope.** They are not
   implementation bodies, but deleting the crate-level `allow` makes them fail `clippy -D warnings`,
   so their disposition cannot be deferred. AC-009 requires the `todo!()`s retired and the finding they
   recorded kept — the same "move the finding before deleting the prose" discipline as AC-011.
7. **`_invalidated-citations.md` is a named, verifiable artefact rather than a note in the PR
   description.** AC-011 requires it in this story's own directory with each row resolving to a real
   `file:line` at HEAD, because HS-S0112's job is to *repair* citations rather than to rediscover them,
   and a PR description is not something `spec-trace` or a later story can read.
8. **No conformance rule, no KB atom, and no `spec/SPECIFICATION.md` edit in this diff.** Confirmed
   against the storymap's coverage split: rules are HS-S0103/HS-S0105's, atoms arrive only through
   `.kb/_intake/` and `/redkiln:kb-ingest`, and clause repairs are HS-S0112's. `redkiln validate --kb`
   must be exactly as green after this story as before it.
