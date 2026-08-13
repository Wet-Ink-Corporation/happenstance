---
item: HS-S0101
stage: discover
created: 2026-08-12T13:03:22.127Z
updated: 2026-08-12T13:03:22.127Z
template_sig: 86ce4036
rendered_sig: 4932cfec
---

# Discover — The one door a foreign identity comes in through

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: give `MemoryEventStore` the one additive **inherent** `&self` operation that accepts an already-identified `SequencedEvent`, so a foreign `EventId` has a door to come in through — `EventStore`'s trait signature byte-identical before and after, and the semver class stated in the ADR that authorises it | `_storymap.md`, *Slices* table, `ingest-seam-and-memory-oracle` row 1 | One method, on a concrete type, in the published contract crate. Deliberately the smallest possible change |
| **Depends on `adr-0026-peer-ingest-and-transport` (HS-S0098)**, which supplies the authorisation and the stated semver class for any change to `happenstance-core` | `_storymap.md`, *Slices* `depends_on`; `_decomposition.md`, AC-A01 | A change to a published crate that no decision record authorises is the thing this edge prevents |
| **AC-002** — the central question has a written answer, and no frozen clause is edited in this project's diff | `project.md`, *Acceptance criteria*, AC-002 | This story's share is the *structural* half: the seam exists and it did not require `EventStore` to change |
| The seam, stated precisely: phase 4 gave `SequencedEvent` an `id` field, so *"a foreign identity has a place to sit and no door to come in through"* — `append` mints `EventId::new(self.store_id, position)` for every event it writes | `crates/happenstance-sync/src/ingest.rs:52-71` | The obstruction is the **write path**, one layer in from the trait. Not the port signature, not the value type |
| `MemoryEventStore::restore` preserves a foreign identity and is no help: it builds a **new** store from an owned snapshot, while `ingest` holds `&self` on an existing one | `crates/happenstance-core/src/memory.rs:163`; `crates/happenstance-sync/src/ingest.rs:56-62` | Restore is the near-miss that looks like the answer. It is not `&self` and it is not additive to a live store |
| Four options priced, with (a) recommended: an additive **inherent** `&self` operation on `MemoryEventStore` — semver-minor, no trait change, no adapter obliged to have an opinion | `_decomposition.md`, *Composition root* §5, options table | (b) growing `EventStore` is **refused**; (c) dropping the coherence proof is a fallback; (d) "real adapters own their types" is true and not sufficient for the oracle |
| The trait seam is genuinely discharged and only the write path is not: *"`EventStore::append` still does not need to change — that is the whole result"* | `crates/happenstance-sync/src/ingest.rs:64-71` | The leak has shrunk twice — port signature → struct fields → one missing store operation. Keep it shrinking |
| **SY-8 `[FROZEN]`** and `RUNBOOK.md:450-455`: identity does not enter `EventStore`; coherence is what makes that hold | `spec/SPECIFICATION.md:6070-6086`; `RUNBOOK.md:450-455` | The refusal of option (b) is normative, not stylistic |
| **SY-5 `[FROZEN]`** — ingest MUST append at the tail; an ingested event MUST NOT be assigned a position below any position the store has already assigned | `spec/SPECIFICATION.md:5954-5971` | The door must open onto the **tail**. `Rejects:` order-preserving insertion by origin position or timestamp |
| **SY-19 `[FROZEN]`** — a replicated event's local position is arrival order, and no expression relates two peers' position numbers because `SequencePosition` carries no origin | `spec/SPECIFICATION.md:6409-6428` | The foreign `EventId` is preserved; the foreign *position* is not reused. Two different values with the same integer inside |
| `EventStore::contains_event_id` already exists on the contract and would answer `IngestStore::holds` exactly; the only obstruction is that the sync crate's placeholder `EventId` is a different type from the contract's | `crates/happenstance-core/src/memory.rs:414`; `crates/happenstance-sync/src/ingest.rs:74-82` | The read half of the seam is already built. Only the write half is missing |
| The `impl IngestStore for MemoryEventStore` coherence proof — local trait, foreign type — is what would be lost under option (c), along with the doctest target | `crates/happenstance-sync/src/ingest.rs:26-31`, `:191-206` | The proof is why the seam is a store operation rather than a new store in the sync crate |
| The residual risk this story is the scheduled landing of: a store-side seam discovered late is a breaking change to a *published* port | `RUNBOOK.md:462-466` | If a **real** peer needs a seam on the **port** rather than on a concrete type, stop: new atom, re-plan |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Answered, and
this story is where the answer becomes structural.** No: SY-1 and SY-6 are
`[FROZEN]` and ADR-0026 reconciles with them
(`spec/SPECIFICATION.md:5841-5867`, `:5973-6029`). The consequence for this
story's signature is concrete and easy to get wrong — **the new operation takes
no `AppendCondition` parameter of any kind.** A door that accepts a condition is a
door someone will evaluate. The strongest available form of the answer is a
signature that cannot express the mistake, and that is what spec must pin.

**Hub-and-spoke versus peer-to-peer — not this story's.** ADR-0027 owns it. The
seam is topology-blind by construction: it is one store accepting one already
identified event.

**Which of the four options lands? — (a), recommended and not yet ratified.**
The architecture brief prices all four (`_decomposition.md`, *Composition root*
§5) and recommends the additive inherent method. The decision is ADR-0026's to
authorise, including the **semver class stated out loud** — additive on a
concrete type in a published crate is semver-minor, and saying so is AC-A01's
requirement rather than a nicety.

**What exactly does the operation take, and what does it return? — Deferred to
spec, under four fixed constraints.** It is `&self`; it accepts an already
identified event carrying its origin's `EventId` and `RecordedAt`; it assigns a
**fresh local position at the tail** (SY-5) and never reuses the origin's; and it
takes no condition. Whether it accepts one event or a slice, and whether the slice
is group-atomic here or group-atomicity is composed above it in
`IngestStore::ingest`, is a real design choice with a real cost either way and
belongs to spec.

**Is the operation `pub`, or `pub` behind a feature? — Deferred to spec, and it
interacts with a discrepancy already on file.** `RUNBOOK.md:4581-4585` asks for
`MemorySyncPeer` behind a `memory` feature while
`crates/happenstance-sync/src/lib.rs:145` declares `pub mod memory;`
unconditionally and `crates/happenstance-sync/Cargo.toml:44` scopes `memory` to
the coherence proof only. Both answers are cheap while `publish = false` and
neither is cheap afterwards — but `happenstance-core` **is** published, so this
one is not symmetric with the sync-side question and must be decided on its own
terms.

**Does this story need a conformance rule? — No, and that is deliberate.** The
rule that proves the seam works is `ingested_events_land_above_the_local_head`
(SY-5), and it belongs to the sync suite, which does not exist until
`sync-testkit-crate-and-rule-registry` (HS-S0103). This story's own evidence is
the existing event-store suite staying green plus a direct unit test that the
foreign `EventId` survives and the local position is fresh.

## Decision

`impl IngestStore for MemoryEventStore` compiles today and its bodies are
`todo!()`, and the reason is not that anyone ran out of time: `EventStore::append`
mints `EventId::new(self.store_id, position)` for every event it writes, and it
does so because `happenstance-core` states that no store-assigned value is ever
supplied by a caller through `append` — the property that keeps the contract's
write path from having to distinguish "I decided this" from "somebody else did and
I am copying it" (`crates/happenstance-sync/src/ingest.rs:52-62`). So a foreign
identity has a place to sit on `SequencedEvent` and no door to come in through,
and the only crate that can open one is the crate that owns the type, because
coherence lets a foreign trait be added to a local type and never lets a third
crate reach inside one. This story opens that door in its mildest available form:
one additive **inherent** `&self` operation on the concrete `MemoryEventStore`,
leaving `EventStore`'s trait signature byte-identical, obliging no adapter to have
an opinion, and costing the published crate a semver-minor bump that ADR-0026
names. The spec stage will cover: the exact signature and its four fixed
constraints — `&self`, already-identified input, fresh local tail position, **no
`AppendCondition` parameter**; the return shape and whether group atomicity lives
here or above; visibility and feature gating; the assertion that
`crates/happenstance-core/src/store.rs`'s `EventStore` is unchanged in the diff;
the semver class as ADR-0026 states it; and the unit evidence that a foreign
`EventId` survives while the local position is assigned fresh.

## The wrong implementation

**The mutant that matters, and it is not the obvious one.** `ReMintingIngestDoor`
— the inherent operation lands, its signature is exactly right, it is `&self`, it
appends at the tail, and in the body it writes
`EventId::new(self.store_id, position)` over the identity it was handed. It is a
two-line slip and it is the *default* behaviour of the code path it is being
grafted onto. Everything is green: the whole event-store conformance suite passes,
because nothing there knows about foreign identity; `cargo xtask ci` passes; a
round-trip test that counts events passes, because every event does arrive; a
round-trip test that compares payload `Bytes` passes, because the payload is
untouched. What breaks is dedupe. `IngestStore::holds` asks
`contains_event_id(origin_id)`, the store holds a locally minted id instead, the
answer is always `false`, and every re-delivery appends a fresh copy — so the
`skipped` count in `Ingested` is permanently zero and the two logs never converge
while looking healthy from both ends. Re-delivery is the *normal* case, not the
exceptional one (`crates/happenstance-sync/src/ingest.rs:136-141`), so this is a
production-shaped bug that only an assertion on the **identity**, not on the
count, can see. It must live in `crates/happenstance-sync-testkit/tests/` as the
store half of the mutant registry, declared against SY-11's
`redelivery_of_an_accepted_group_is_a_no_op` and SY-12's
`dedupe_reaches_identity_without_decoding`, with a provenance string that is
simply true: this is what happens when the ingest door is implemented by calling
the append path.

**The second mutant is the near-miss the module documentation already names.**
`RestoreBasedIngest` — implement the door by taking `snapshot()`, pushing the
foreign events onto the owned `Vec`, and calling `MemoryEventStore::restore`. It
compiles, it preserves the foreign `EventId` correctly, and it looks like reuse
rather than a rewrite. It is wrong because `restore` builds a **new** store while
the caller holds `&self` on an existing one
(`crates/happenstance-core/src/memory.rs:163`;
`crates/happenstance-sync/src/ingest.rs:56-62`), so any handle already taken onto
the original store — and the fixture contract's whole `SECOND_HANDLE` capability
is about exactly those — silently stops seeing new writes. The existing suite's
`two_handles_observe_each_others_appends` is the rule shaped to catch it, and it
will only do so if the ingest door is exercised through a second handle.

**The third is the intuitive merge, and SY-5 already names it.**
`OriginPositionInsertingDoor` — insert the ingested event at the position its
origin gave it, so the merged log "reads in order". Every event is present, every
identity is preserved, and a test that reads the log back and compares the *set*
of events passes. It destroys every projection on the store, because
`ProjectionStore::checkpoint` is one scalar and an event inserted below an
existing checkpoint is never read, never applied and never reported missing
(`spec/SPECIFICATION.md:5963-5971`). It is also the sharpest instance of this
project's own thesis: it is only tempting if you believe a `SequencePosition`
means something outside the store that assigned it, and SY-19 exists to say it
does not (`spec/SPECIFICATION.md:6409-6428`).

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

**Box 6.** This story adds no conformance rule. Its own unit evidence is held to
the same bar and it is the bar this seam is most likely to trip over: the
assertion is that the ingested event's local position is **above every position
the store had already assigned**, compared against a head the test read back from
the store — never `assert_eq!(position, 4)`. The store under test is
`MemoryEventStore`, which assigns densely from one, so a literal would pass here
and fail against `GappedPositionStore`, the conformant variant that exists
precisely to refute it (`crates/happenstance-testkit/tests/mutation_coverage.rs:326-338`).

**Box 7.** This story edits no `[FROZEN]` clause. It is constrained by three —
SY-5, SY-8 and SY-19 — and conforms to all three by construction: tail insertion,
no trait growth, no reuse of the origin's position. The change to the published
contract crate is authorised by ADR-0026, which is written first and is this
story's `depends_on` edge, and the semver class is stated in that ADR rather than
inferred from the diff.
