---
item: HS-S0102
stage: discover
created: 2026-08-12T13:03:23.079Z
updated: 2026-08-12T13:03:23.079Z
template_sig: 86ce4036
rendered_sig: 513d520b
---

# Discover — IngestStore and MemorySyncPeer get real bodies

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: replace the `todo!()` bodies with real `IngestStore` and `MemorySyncPeer` ones and drop the scoped `#![allow(clippy::todo)]`, so a batch pushed from one in-memory store lands at the receiver's local tail with the foreign `EventId` preserved, group-atomically and idempotently — with the doctest on `MemorySyncPeer` that shows one round trip holding no state | `_storymap.md`, *Slices* table, `ingest-seam-and-memory-oracle` row 2 | The first real crossing of a store boundary in the repository, and the oracle every later slice measures against |
| **Depends on `memory-store-ingest-seam` (HS-S0101)**, which supplies the additive inherent `&self` operation on `MemoryEventStore` that accepts an already-identified `SequencedEvent` | `_storymap.md`, *Slices* `depends_on`; `_decomposition.md`, *Composition root* §5 | Without that door there is no truthful body to write; the `todo!()`s say so in their own text |
| **AC-002** — the central question has a written answer, and the code is the half that would not exist if the answer were wrong | `project.md`, *Acceptance criteria*, AC-002; `_storymap.md`, *Coverage* AC-002 row | This story's share: the bodies keep `EventStore` untouched and make the answer operational |
| The four `todo!()` bodies, each carrying the reason it is one: `store_id`, `ingest` (*"append mints an `EventId` per event, so no foreign identity can be preserved"*), `holds` (*"`contains_event_id` would answer this, once the placeholder `EventId` is unified"*), `watermark` | `crates/happenstance-sync/src/ingest.rs:206-231` | Four bodies, three unblocked by the seam and one by unifying the identity type. The `todo!()` messages are the work list |
| The scoped allow: *"`clippy::todo` is denied workspace-wide… The phase that implements replication removes both the bodies and this line."* | `crates/happenstance-sync/src/lib.rs:135-139` | Dropping the allow is part of the deliverable, not a follow-up. It is the mechanical proof no body was left behind |
| `MemorySyncPeer`'s bodies are **already real** — *"Unlike the two stand-ins in this crate's `tests/`, the bodies here are real. A fixture whose methods are `todo!()` cannot be the thing a conformance suite is pointed at"* | `crates/happenstance-sync/src/memory.rs:1-10` | What this story adds to the peer is the doctest and end-to-end exercise, not a rewrite. Read the file before assuming otherwise |
| `identity::StoreId`, `identity::EventId` and `identity::RecordedAt` are **placeholders phase 4 deletes**, deliberately not re-exported so the collision with the settled types is unwriteable by accident | `crates/happenstance-sync/src/identity.rs:24-41`; `crates/happenstance-sync/src/lib.rs:149-153` | Phase 4 has landed: `SequencedEvent` now carries `id: happenstance_core::EventId` (`crates/happenstance-sync/src/ingest.rs:44-48`). The unification is due |
| **SY-5 `[FROZEN]`** — ingest MUST append at the tail, never below a position already assigned; rule `ingested_events_land_above_the_local_head`, *"anchored on positions the store actually assigned"* | `spec/SPECIFICATION.md:5954-5971` | The clause names the anchoring discipline in its own `Rule:` field |
| **SY-11 `[FROZEN]`** — re-delivery of an event the receiver already holds MUST be a no-op: no second copy, no compensation, no error; rule `redelivery_of_an_accepted_group_is_a_no_op` | `spec/SPECIFICATION.md:6135-6152` | Its `Rejects:` is the inversion, not the duplicate — *"A duplicate is visible; an inversion looks like a decision"* |
| **SY-12 `[FROZEN]`** — dedupe is on the store-assigned `EventId`, reachable without decoding `Event::data` or `Event::metadata` | `spec/SPECIFICATION.md:6153-6183` | Dedupe by identity, never by payload comparison. Also the clause whose exemplar was withdrawn once already |
| **SY-30** — the group decomposition is explicit on the wire and never re-inferred: *"a receiver that flattened the batch and appended event-by-event would publish a state the origin never had, and no amount of ordering fixes that"* | `crates/happenstance-sync/src/peer.rs:185-190`; `spec/SPECIFICATION.md:6700` | Group atomicity is a property of `ingest`, and it is invisible to any assertion on counts |
| **SY-15 / SY-16 `[FROZEN]`** — every port method completable in one round trip; resume state is an owned, transferable value the caller supplies, not a handle the peer holds | `spec/SPECIFICATION.md:6292-6340` | SY-16's `Rejects:` is *"an in-memory cursor, which passes every test written against a process that stays alive"* — the exact hazard an in-memory oracle invites |
| The port cannot express "one round trip with no held state"; it *"has to be checked by a fixture peer that counts its own round trips"* | `crates/happenstance-sync/src/peer.rs:43-50` | The doctest can demonstrate the shape; only a counting fixture can prove it. The fixture is HS-S0103's |
| `Ingested { appended, skipped, last_local }` — `last_local` is *"Local, and unrelated to the position inside the events' own `EventId`s: this is arrival order here, that is authorship order there."* | `crates/happenstance-sync/src/ingest.rs:178-191` | The return type already encodes the distinction this project exists to defend |
| Standing instruction: the phase-2 crate's *findings* are not the same as its `todo!()`s — move each finding into the ADR that consumed it **before** deleting the paragraph that holds it | `_decomposition.md`, *Notes*, "One thing to carry forward that no AC captures" | Implementing the crate deletes most of `ingest.rs:20-82`. That prose is the evidence the sketch was built to produce |
| Doctests on every new public item, and `happenstance-sync` is `publish = false` like the testkit, so its doctests need the out-of-package harness rather than the default `cargo test --doc` | `standards/rust/70-rustdoc-obligations.md`; `standards/rust/62-doctests-and-harnesses.md` RS-62-5; `_decomposition.md`, testing brief *Unit* | A doctest that never runs is documentation, not evidence |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered: no,
and this story is where the answer becomes a body.** SY-1 and SY-6 are `[FROZEN]`
(`spec/SPECIFICATION.md:5841-5867`, `:5973-6029`) and the trait's own
documentation already commits to it: *"disagreeing with an event's content is not
available as an option, because the event is already durable somewhere else and
refusing it only guarantees the two logs never converge"*
(`crates/happenstance-sync/src/ingest.rs:143-148`). The implementation consequence
is exact: `IngestStore::ingest` takes `&[ReplicatedEvent]` and **no condition
parameter**, and its body must never call the store's conditional append path.
An adapter's `Error` is for storage failures only.

**Hub-and-spoke versus peer-to-peer — not this story's.** ADR-0027 owns it; this
story wires two in-memory stores through one peer and is topology-blind.

**Does the placeholder `identity::EventId` get unified with
`happenstance_core::EventId` here? — Answered yes, and it is a prerequisite
rather than a tidy-up.** `holds`'s own `todo!()` says the only obstruction is that
the two are different types (`crates/happenstance-sync/src/ingest.rs:224`), and
`identity.rs:24-41` records that the trio is phase 4's to delete and that the
non-re-export exists so *"every site phase 4 must revisit"* is greppable by one
name. Phase 4 landed. Whether `ReplicatedEvent` and `Watermark` stay in this
crate — they are *"genuinely replication's own"* — is not in question; only
`StoreId`, `EventId` and `RecordedAt` are.

**Is dedupe by `EventId` or by content? — Answered: `EventId`, and never
content.** SY-12 requires the identity be reachable without decoding
`Event::data` or `Event::metadata`, and `EventStore::contains_event_id` already
answers it (`crates/happenstance-core/src/memory.rs:414`). A content hash is
separately forbidden by SY-13 (`spec/SPECIFICATION.md:6211`).

**Is atomicity per group or per call? — Deferred to spec, with the shape fixed.**
The trait says *"Each group lands atomically or not at all"*
(`crates/happenstance-sync/src/ingest.rs:133-138`) while the signature takes a
flat `&[ReplicatedEvent]`. Reconciling those two is real work: either the group
boundary is carried into `ingest` or group atomicity is composed above it from
`EventGroup`s. Whichever lands, SY-30's obligation — the receiver never re-infers
the decomposition — must survive it, and the choice is ADR-0026's to authorise.

**Does `MemorySyncPeer` go behind a `memory` feature? — Deferred to spec, and it
is a recorded discrepancy rather than an oversight.** `RUNBOOK.md:4581-4585` asks
for the feature; `crates/happenstance-sync/src/lib.rs:145` declares `pub mod
memory;` unconditionally; `crates/happenstance-sync/Cargo.toml:44` scopes the
existing `memory` feature to the coherence proof only. Both answers are cheap
while `publish = false` and neither is cheap afterwards
(`_decomposition.md`, *Composition root* §4).

**Does this story add conformance rules? — No.** The suite does not exist until
`sync-testkit-crate-and-rule-registry` (HS-S0103). This story's evidence is
integration tests inside `happenstance-sync` plus the doctest; the rules that
generalise them arrive one slice later, which is the correct order — a rule
written against an implementation that already exists is written by someone who
believes the implementation is right, so the rules must be justified by clauses
rather than by this code.

## Decision

`happenstance-sync` today is a crate that can describe a store boundary and
cannot cross one: `impl SendIngestStore for MemoryEventStore` compiles, proving
the coherence half, and all four of its bodies are `todo!()` with the reason
written into each `todo!` message
(`crates/happenstance-sync/src/ingest.rs:206-231`). With the ingest door from
HS-S0101 in place, three of the four are unblocked outright and the fourth needs
the placeholder `EventId` unified with the contract's, which phase 4 has already
landed. This story makes the crossing real: an event appended to one in-memory
store, wrapped as a `ReplicatedEvent`, pushed through `MemorySyncPeer`, and
ingested into a second store where it lands at that store's **local tail** with
its **origin's `EventId` intact** — group-atomically, and idempotently under
re-delivery, which is the normal case rather than the exceptional one. The
scoped `#![allow(clippy::todo)]` comes out in the same change, which is the
mechanical proof that no body was left behind. The spec stage will cover: the
four `IngestStore` bodies and the identity unification they need; how group
atomicity is expressed given a flat `&[ReplicatedEvent]` signature; the
`Ingested { appended, skipped, last_local }` semantics with `last_local` as
arrival order and never authorship order; the `MemorySyncPeer` doctest showing
one round trip that holds no state between calls; the `memory`-feature
disposition; the removal of the crate-level `allow`; and — before any of the
prose in `ingest.rs:20-82` is deleted — which ADR each of its findings was moved
into.

## The wrong implementation

**The mutant this slice most invites: `ContentDedupingIngest`.** Implement
idempotence by comparing the incoming event's type, tags and payload bytes
against what the store already holds, and skip on a match. It is a natural
shortcut because it needs no identity plumbing at all, and it makes every test
that counts events pass: push a batch twice, the receiver holds one copy of each,
`Ingested { appended: 0, skipped: n }` on the second pass, green. It is wrong in
both directions at once. It **over**-deduplicates — two genuinely distinct events
with identical content (the same sensor reading a minute apart, the same "door
opened" from the same tag set) collapse into one, and the loss is silent and
unrecoverable because there is no record that a second event existed. And it
**under**-deduplicates the moment any peer re-encodes anything, because the
comparison is over bytes rather than over the identity SY-12 requires. SY-13
forbids the closely related hash-over-wire-bytes variant by name
(`spec/SPECIFICATION.md:6211`). The mutant belongs in
`crates/happenstance-sync-testkit/tests/` declared against
`redelivery_of_an_accepted_group_is_a_no_op` and
`dedupe_reaches_identity_without_decoding`, and the rule that catches it must
assert on **which identities** the receiver holds, never on how many events it
holds.

**Its sibling, `FlatteningIngest`, is invisible to the same counting test.** Take
the `PushBatch`, flatten `groups` into one list, append event by event. Every
event arrives, every identity is preserved, replay is a no-op, the payloads are
byte-identical, and the receiver publishes a sequence of intermediate states the
origin never had — which is exactly why the decomposition is *on the wire and
explicit* rather than re-inferred (`crates/happenstance-sync/src/peer.rs:185-190`,
SY-30). No assertion over the final state of the log can see it; only an
assertion made by a reader observing the log *during* the ingest, or a fault
injected mid-batch, can. This is the one rule in the slice whose oracle has to be
written in a direction that shares no subroutine with the implementation
(`standards/rust/60-what-a-test-must-prove.md`, RS-60-4).

**And the oracle's own trap: `HeldCursorMemoryPeer`.** `MemorySyncPeer` is an
in-process type, so giving it a `cursor: Cell<usize>` field and having `pull`
advance it is the obvious implementation and it passes every test in a process
that stays alive — which is every test this slice writes. SY-16's `Rejects:` is
this exact shape and its reason is the deployment rather than the design:
*"tablets that lose signal in a −22 °C cold store; Turnstile's Workers are
cancelled mid-flight; a Durable Object is evicted. The normal termination path at
the edge is the handle going away"* (`spec/SPECIFICATION.md:6323-6340`). The
oracle is where a held cursor is least likely to be noticed and most likely to be
copied by the first real adapter author, because the oracle is the worked example.
The doctest this story owes must therefore show the round trip with the token
carried **outside** the peer, and `resume_survives_a_dropped_peer_handle` is the
rule that generalises it in HS-S0103.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

**Box 6, and this story is where the temptation is strongest.** Both stores here
are `MemoryEventStore`, which assigns densely from one, so
`assert_eq!(ingested.last_local, SequencePosition::new(4))` would be green and
would encode a `MAY` as a `MUST` (CF-6, VT-11). Worse, the *interesting* number
in this story is a position that came from the other side of the boundary, and
asserting on it at all is the project's headline error. Every position assertion
here anchors on a head the receiving store reported before the ingest, and the
foreign position is only ever compared as part of an `EventId`, never as a
number.

**Box 7.** This story edits no `[FROZEN]` clause. It is constrained by SY-5,
SY-11, SY-12, SY-15, SY-16 and SY-30 and implements all six; the design freedom it
does have — group-atomicity expression, the `memory` feature, the identity
unification — is authorised by ADR-0026, which is written first and reaches this
story through `memory-store-ingest-seam`. The one thing that would *look* like a
clause change is deleting the crate prose those clauses cite by line
(`peer.rs:230-243`, `lib.rs:100-104`, `ingest.rs:38-50`); that is a `Rejects:`
repair owned by `frozen-clause-repairs` (HS-S0112), and this story's obligation is
to record every citation it invalidates rather than to fix them here.
