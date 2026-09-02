---
item: HS-S0101
stage: spec
created: 2026-08-12T13:47:40.377Z
updated: 2026-08-12T13:47:40.377Z
template_sig: 87bbf1d0
rendered_sig: 78eb2325
---

# Spec — The one door a foreign identity comes in through

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/memory-store-ingest-seam/spec.md` |
| Key briefs | `.../replication-identity-and-ingest/_decomposition.md` — *Composition root* §5 (*The write-path seam*, the four priced options), *Acceptance Criteria* AC-A01; *Testing brief* → *Fixtures and seams to mock* |
| Signed-off design | `.../replication-identity-and-ingest/_design.md` — **no surfaces declared**; the API-surface obligation is explicitly delegated to the briefs and to this spec (`_design.md:58-71`, sign-off at `:116-125`) |
| This story's discovery | `.../memory-store-ingest-seam/discover.md` — the signal ledger, the three named wrong implementations, and the five questions it deferred **to this spec** (`discover.md:54-78`) |
| Story map row | `.../replication-identity-and-ingest/_storymap.md:58` (slice `ingest-seam-and-memory-oracle`, row 1) |
| Roadmap pointer | `RUNBOOK.md:450-466` (the leak warning and its residual risk), `RUNBOOK.md:4516-4620` (phase 13) |

## One-line PR slice

Give `MemoryEventStore` the one additive **inherent** `&self` operation that accepts an
already-identified `SequencedEvent`, so a foreign `EventId` has a door to come in through —
`EventStore`'s trait signature byte-identical before and after, and the semver class stated in the
ADR that authorises it.

## Executive summary

This PR lands **one public method and one public result struct** in `happenstance-core`, both
inside the existing `#[cfg(feature = "memory")]` module, and nothing else that a consumer can see.

The delta against the tree as it stands: `crates/happenstance-sync/src/ingest.rs:52-71` records that
the replication seam has shrunk twice — from a port signature, to a struct's fields, to *one missing
store operation* — and that the last step could not be taken from the sync crate, because coherence
lets a crate add a trait to a foreign type and never lets it reach inside one. Phase 4 gave
`SequencedEvent` an `id` field (`crates/happenstance-core/src/event.rs:472-496`), so a foreign
identity has a place to sit; `EventStore::append` mints `EventId::new(self.store_id, position)` for
every event it writes (`crates/happenstance-core/src/memory.rs:386-400`), so it has no door to come
in through; and `MemoryEventStore::restore` builds a **new** store from an owned snapshot
(`crates/happenstance-core/src/memory.rs:138-168`) while ingest holds `&self` on an existing one.
This story opens that door, in the mildest form the architecture brief priced: additive, inherent,
on a concrete type, obliging no adapter to have an opinion and leaving the port alone.

What it is *not*: it is not `IngestStore`'s bodies, not `MemorySyncPeer`, and not a conformance rule.
Those are the slice-mate's and slice 3's. This story is the substrate the slice-mate could not write
truthfully without, and its own proof is that the four `todo!()`s in
`crates/happenstance-sync/src/ingest.rs:212-230` stop being *impossible* and become merely unwritten.

## Context pack

Read this section before opening anything else; everything below it is a signposted anchor.

**1. The answer to the central question is already frozen, and it decides this signature.** Ingest
does **not** re-check the writer's asserted append conditions: SY-1 `[FROZEN]`
(`spec/SPECIFICATION.md:5841-5867`) and SY-6 `[FROZEN]` (`spec/SPECIFICATION.md:5973-6029`) settle
it, and the origin's condition travels as *evidence, not as an instruction*. The consequence for this
story is concrete: **the new operation takes no `AppendCondition` parameter of any kind, and returns
no error.** A door that accepts a condition is a door someone will evaluate; a door that can refuse
is a door someone will make refuse. The strongest available form of a frozen prohibition is a
signature that cannot express the violation, and that is what this spec pins.

**2. The port does not grow — coherence is what makes that hold.** SY-8 `[FROZEN]`
(`spec/SPECIFICATION.md:6070-6086`) and `RUNBOOK.md:450-455` refuse option (b), growing `EventStore`
with an ingest-shaped method. The architecture brief priced all four options and recommends (a), the
additive inherent method, precisely because the blast radius is one method on one concrete type
rather than a port change every adapter author must answer
(`.../replication-identity-and-ingest/_decomposition.md:139-160`). Option (c) — give the sync crate
its own store — is a fallback only, because it discards the "local trait, foreign type" coherence
proof at `crates/happenstance-sync/src/ingest.rs:26-31` and the doctest target with it.

**3. The identity is preserved; the position is not.** These are two different values that both
happen to be integers, and conflating them is the project's own thesis in miniature. VT-5 `[FROZEN]`
(`spec/SPECIFICATION.md:732-745`) requires a store to preserve, unchanged, the `EventId` of an event
it accepts through ingest. SY-5 `[FROZEN]` (`spec/SPECIFICATION.md:5954-5971`) requires the event to
land **at the tail**, above every position this store has already assigned. SY-19 `[FROZEN]`
(`spec/SPECIFICATION.md:6409-6428`) explains why the incoming `position` field must be *discarded*
rather than honoured: `SequencePosition` carries no origin, so there is no expression relating two
peers' numbers and none can be constructed. `SequencedEvent` documents the pair in its own words —
`position` is arrival order here, `id.position()` is authorship order there
(`crates/happenstance-core/src/event.rs:472-482`).

**4. Idempotence must be atomic with the write, not composed above it.** Re-delivery is the *normal*
case, not the exceptional one (`crates/happenstance-sync/src/ingest.rs:136-141`). A caller that asks
`contains_event_id` and then writes takes two locks, and two concurrent ingests of the same batch
both see "not held" and both append. `append` takes the write lock **once** for condition-check and
write, and its comment says why that is what makes the reference store usable as an oracle
(`crates/happenstance-core/src/memory.rs:364-370`). The door inherits that rule: skip-if-held and
append happen under one write lock, and the operation reports what it did.

**5. Authorisation, and what outranks this spec.** ADR-0026 lands in `.kb/decisions/` *before* this
story starts (`depends_on: adr-0026-peer-ingest-and-transport`, HS-S0098), and it is what authorises
a change to the published contract crate and states its **semver class** out loud — that is AC-A01's
requirement, not a nicety (`.../replication-identity-and-ingest/_decomposition.md:529-534`). An
accepted decision atom outranks this spec. Read ADR-0026 first: where it names the shape, visibility
or class, it is binding and any divergence here is a defect in this spec, to be recorded in the
ledger. Where it is silent, this spec's decisions below apply. If ADR-0026 as accepted does **not**
authorise a core-side change at all, stop — that is `RUNBOOK.md:462-466`'s residual risk landing
hard, and it is a new atom and a re-plan, not an improvisation.

**6. The three wrong implementations this story exists to make impossible**, taken from
`discover.md:104-152` and not re-derived here: `ReMintingIngestDoor` (writes
`EventId::new(self.store_id, position)` over the identity it was handed — everything stays green,
dedupe silently dies, the logs never converge); `RestoreBasedIngest` (implements the door via
`snapshot()` + `restore` — preserves identity correctly and silently detaches every handle already
taken onto the store); `OriginPositionInsertingDoor` (inserts at the origin's position so the merged
log "reads in order" — destroys every projection, because `ProjectionStore::checkpoint` is one scalar
and an event below it is never read, never applied, never reported missing). The first is a two-line
slip and is the *default* behaviour of the code path the door is grafted onto.

**7. The persona-journey slice.** The reader served here is the peer-runner author — the only caller
who should ever hold this door, for the reason `IngestStore` already states: handing it to
application code hands it the ability to forge history (`crates/happenstance-sync/src/ingest.rs:95-96`).
Their journey today ends at a `todo!()` whose message is a statement of this story's absence
(`crates/happenstance-sync/src/ingest.rs:219-221`). After this PR it ends at a compiled call.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate consumed by the capability slice-mate in this
  same project. Not a double, not a flag, not a `todo!()`.
- **Slice / milestone**: `ingest-seam-and-memory-oracle`. Slice-mate:
  `ingest-store-and-memory-peer-round-trip` (HS-S0102), implemented in the same context and merged
  after this one (`.../replication-identity-and-ingest/_storymap.md:122-124`).
- **Mount point**: `crates/happenstance-sync/src/ingest.rs` — the
  `#[cfg(feature = "memory")] mod memory_store_ingest` block at `:193-231`. That block is the
  composition root for this seam: it is the only place in the workspace where a foreign identity is
  handed to a `MemoryEventStore`, and the `todo!()` at `:219-221` is a verbatim statement of this
  story's absence. **The slice is not integrated until that block reaches the new door and
  `cargo test -p happenstance-sync --features memory` compiles it** — the feature matters, because
  `crates/happenstance-sync/Cargo.toml:37-44` scopes `memory` to the coherence proof and a plain
  `cargo test -p happenstance-sync` compiles none of it.
- **Wires into**:
  - `crates/happenstance-core/src/memory.rs` — `MemoryEventStore`, its `RwLock<Vec<SequencedEvent>>`,
    `store_id()` (`:104-108`), `restore` (`:163`), `position_at` (`:276-282`) and the poisoning
    policy at `:190-199`.
  - `crates/happenstance-core/src/event.rs:472-496` — `SequencedEvent`, `#[non_exhaustive]` with four
    public fields, constructed through `SequencedEvent::new`.
  - `crates/happenstance-core/src/identity.rs:1-7` — the module that states that no store-assigned
    value is ever supplied by a caller **through `append`**, and names the ingest port as the seam.
    That sentence is the one this story makes true rather than aspirational; it does not need editing.
  - `crates/happenstance-core/src/store.rs:94` and `:268` — `EventStore` and `contains_event_id`, the
    already-built *read* half of the seam. Neither changes.
  - `crates/happenstance-core/tests/frozen_signatures.rs:236-296` — the standing compile-level guard
    that identity never reached `append`'s signature, and that membership is answerable through the
    port. It must keep compiling **unchanged**.
- **Renders surfaces**: **none.** `_design.md` declares no user-facing surface for this project and
  the determination itself is what was signed off (`_design.md:10-22`, `:116-125`). The public Rust
  items this story adds are the only "surface", and their shape is decided in *Behavior and
  interfaces* below, per `_design.md:58-71`.
- **Conformance rule(s)**: **none added here, deliberately.** The rule that observes this behaviour
  is `ingested_events_land_above_the_local_head` (SY-5, `spec/SPECIFICATION.md:5957-5961`) and it
  belongs to `happenstance-sync-testkit`, which does not exist until
  `sync-testkit-crate-and-rule-registry` (HS-S0103). VT-5's preservation half is
  `ingest_preserves_origin_identity`, likewise (`spec/SPECIFICATION.md:740-744`). This story's own
  evidence is the existing event-store suite staying green plus direct unit tests in
  `crates/happenstance-core/src/memory.rs`'s `mod tests` — held to the same no-literal-position bar
  (CF-6), because `MemoryEventStore` assigns densely from 1 and a literal would pass here while
  failing against `GappedPositionStore`
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:326-338`).
- **Clause(s)**: discharges nothing on its own and **amends nothing**. Constrained by SY-5, SY-8,
  SY-19 and VT-5, all `[FROZEN]`, and conformant to all four by construction. SY-12's current live
  wrong implementation — *"`impl IngestStore for MemoryEventStore` cannot be written truthfully"* —
  is retired by this slice, and the repair is **not this story's**: `frozen-clause-repairs` owns it
  (`.../replication-identity-and-ingest/frozen-clause-repairs/discover.md:28`).
- **Advances DoD scenario**: initiative **DoD 14** — *replication has an answer on disk*
  (`.bklg/from-contract-to-published-library/initiative.md:397-401`), whose structural half is that
  the answer is a thing the code can actually do. Also feeds **DoD 11** (*the release is diffed, not
  asserted*): this is the only item this project adds to a published crate's surface, and
  `closeout-and-durable-audience/published-tree-delta-statement/spec.md:323` already cites this story
  by name as the reason HS-P0017's commits do not move the published *port*.

## PR boundary

```
crates/happenstance-core/src/memory.rs
crates/happenstance-core/src/lib.rs
crates/happenstance-core/tests/**
crates/happenstance-sync/src/ingest.rs
CHANGELOG.md
.bklg/from-contract-to-published-library/replication-identity-and-ingest/memory-store-ingest-seam/**
```

**In this PR**

- The additive inherent `&self` operation on `MemoryEventStore` and its `#[non_exhaustive]` result
  struct, with rustdoc, a doctest, and the alternative that lost named once (RS-70-5,
  `standards/rust/70-rustdoc-obligations.md:243`).
- The crate-root re-export of the new result type from `crates/happenstance-core/src/lib.rs`, so a
  caller can name it through the defining crate's own re-export (RS-40-4,
  `standards/rust/40-public-surface-and-evolution.md:171`).
- Unit tests in `crates/happenstance-core/src/memory.rs`'s existing `mod tests`, and any addition to
  `crates/happenstance-core/tests/frozen_signatures.rs` needed to keep its claims true — **as
  additions; its existing functions are not edited.**
- The `CHANGELOG.md` `[Unreleased] / ### Added` entry naming the item and its semver class.
- The mount: `crates/happenstance-sync/src/ingest.rs`'s module documentation, which currently asserts
  the door cannot exist (`:38-82`, `:200-205`), corrected to cite ADR-0026 and the new operation —
  and the `memory_store_ingest::ingest` body wired to it, if the slice-mate has not already taken it.

**Explicitly not in this PR**

- `EventStore`, `SendEventStore` or any other trait in `crates/happenstance-core/src/store.rs`.
  Byte-identical, and `crates/happenstance-core/Cargo.toml` is outside the boundary on purpose: a
  diff there means a new feature was added, which this spec forbids without a recorded decision.
- The remaining `IngestStore` bodies (`store_id`, `holds`, `watermark`), `MemorySyncPeer`, the
  removal of the crate-level `#![allow(clippy::todo)]` (`crates/happenstance-sync/src/lib.rs:135-139`),
  and the unification of the sync crate's placeholder `identity::EventId` with the contract's —
  all `ingest-store-and-memory-peer-round-trip`'s (HS-S0102).
- Any conformance rule, any `happenstance-sync-testkit` file, any mutant. Slice 3's.
- Any edit to `spec/SPECIFICATION.md`. `frozen-clause-repairs` (HS-S0113) owns SY-12's repair, and
  amending a `[FROZEN]` clause is out of scope for the whole initiative.
- Any hand-authored `.kb/` atom. ADR-0026 arrived through `.kb/_intake/` and `/redkiln:kb-ingest`
  in the predecessor story; this story authors no atom (`CLAUDE.md`, *Where the work lives*).

**Merge DoD one-liner** — `cargo xtask affected --base main` and `cargo xtask lints && cargo xtask
spec-trace` green, `cargo test -p happenstance-core --all-features` and `cargo test -p
happenstance-sync --features memory` green, `cargo xtask ci --fast` green, and
`git diff --exit-code main -- crates/happenstance-core/src/store.rs` reports nothing.

## Behavior and interfaces

Signature decisions are this spec's to make: `_design.md` declares no surfaces and delegates the
public-API question here (`_design.md:58-71`), and `discover.md:54-78` deferred exactly these five
questions to this stage. Names below are binding on shape and normative on spelling; an implementer
who changes a *name* must record the reason in `_ledger.md`, and one who changes a *shape* is
diverging from this spec and must stop.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The door: one additive inherent `&self` operation** | `pub fn ingest_at_tail(&self, events: &[SequencedEvent]) -> IngestedAtTail`, inherent on `MemoryEventStore`, inside the existing `#[cfg(feature = "memory")]` module. No trait, no port, no new flavour. | `_decomposition.md:150` option (a); `crates/happenstance-core/src/memory.rs:84-200` (the `impl` block it joins) |
| **Why not `ingest`** | An inherent method wins method resolution over a trait method on the same type, and `impl SendIngestStore for MemoryEventStore` puts `ingest` on this very type. A shared name makes `store.ingest(..)` resolve to the inherent one silently at every call site that has the trait in scope — a confusing type error at best and the wrong call at worst. The name also names the invariant, which forecloses `OriginPositionInsertingDoor` at the call site. | `crates/happenstance-sync/src/ingest.rs:212-230`; `discover.md:142-152` |
| **Identity and time are preserved verbatim** | The stored `SequencedEvent` carries the incoming `id` and `recorded_at` unchanged. The door mints nothing and stamps nothing — which is also why it needs no clock and is therefore total on `wasm32`, where `SystemTime::now()` aborts (`memory.rs:239-274`). | VT-5 `[FROZEN]`, `spec/SPECIFICATION.md:732-745`; `crates/happenstance-core/src/event.rs:487-491` |
| **The incoming `position` is discarded, not honoured** | Each accepted event is assigned the next local position by the same rule `append` uses (`position_at(stored.len() + n)`), so the local log stays dense and every ingested event lands above the store's current head. The incoming `position` field is read by nothing. | SY-5 `[FROZEN]`, `spec/SPECIFICATION.md:5954-5971`; SY-19 `[FROZEN]`, `:6409-6428`; `crates/happenstance-core/src/memory.rs:276-282` |
| **One call is one atomic unit** | The whole operation — dedupe and write — runs under a single `self.events.write()`, exactly as `append` does, recovering from poisoning the same way. A caller needing *group* atomicity calls once per group; that is how `IngestStore::ingest`'s "each group lands atomically or not at all" composes above this. A per-event door was rejected: a loop of N lock acquisitions cannot be made group-atomic from above. | `crates/happenstance-core/src/memory.rs:364-370`, `:190-199`; `crates/happenstance-sync/src/ingest.rs:131-141` |
| **Re-delivery is a skip, decided under the same lock** | An event whose `EventId` this store already holds is skipped, not appended and not refused. The check is the same predicate `contains_event_id` answers (`event.id == id`), evaluated inside the write lock; a caller that pre-checked through the port and then wrote would race with itself. | ES-41, `spec/SPECIFICATION.md:4381`; `crates/happenstance-core/src/memory.rs:414-419` |
| **The result: a named `#[non_exhaustive]` struct** | `pub struct IngestedAtTail { pub appended: usize, pub skipped: usize, pub last_local: Option<SequencePosition> }`, re-exported from the crate root. Not a tuple (freezes arity), not `Vec<SequencePosition>` (an allocation to answer a question nobody asked), not bare `usize` (loses the tail position the runner needs). Named distinctly from `happenstance_sync::Ingested` so the two never collide in one scope. | RS-13-4, `standards/rust/13-sealing-and-exhaustiveness.md:144`; RS-40-3, `standards/rust/40-public-surface-and-evolution.md:127`; `crates/happenstance-sync/src/ingest.rs:178-191` |
| **The signature cannot express a refusal** | No `AppendCondition` parameter, no `Result`. `MemoryStoreError` is uninhabited (`memory.rs:284-291`), so a `Result` here would be a type-level lie about a store that has no failure mode, and an error path is the first thing a receiver reaches for when it wants to reject. SY-1 says it may not. | SY-1 `[FROZEN]`, `spec/SPECIFICATION.md:5841-5867`; SY-6 `[FROZEN]`, `:5973-6029`; `discover.md:34-41` |
| **An empty slice is a no-op, not an error** | `ingest_at_tail(&[])` returns `appended: 0, skipped: 0, last_local: None`. This deliberately differs from `append`, which returns `AppendError::NoEvents` on an empty batch — and the difference is not laxity: `append` must return *a position* and cannot invent one, while this return type carries `Option<SequencePosition>` and has nothing to invent. A peer with nothing new to send is the ordinary case. | ES-20 precedence note, `crates/happenstance-core/src/memory.rs:343-362` |
| **No origin validation, and no loop-stopper here** | The door does not check that the incoming `EventId`'s store differs from `self.store_id()`. An event this store minted, coming back, is already held and is therefore skipped by the ordinary path; a rejection branch would be a refusal SY-1 forbids. Distinguishing "mine" from "theirs" is `IngestStore::store_id`'s job, one layer up. | `crates/happenstance-sync/src/ingest.rs:124-129`; `crates/happenstance-core/src/memory.rs:104-108` |
| **Visibility: plain `pub`, no new feature** | It inherits `#[cfg(feature = "memory")]` from the module it lives in, and that is the whole gating story. An off-by-default `unstable-ingest` gate was considered and rejected: no `unstable-*` feature exists anywhere in the tree today (`grep` over every `Cargo.toml` finds none — `projection-store-freeze` owns introducing that convention), and a new core feature would make the sync crate's coherence proof feature-conditional in a second dimension. Recorded as considered, not silently skipped. | `crates/happenstance-core/Cargo.toml:33-49`; `crates/happenstance-sync/Cargo.toml:37-44`; `CHANGELOG.md:13-22` |
| **Semver class, stated rather than inferred** | Additive on a concrete type in a published crate: **semver-minor**. The authoritative statement is ADR-0026's; this story mirrors it in the rustdoc and in `CHANGELOG.md`'s `[Unreleased] / ### Added`. If ADR-0026 states a different class or a different visibility, ADR-0026 wins and the divergence goes in `_ledger.md`. | AC-A01, `.../replication-identity-and-ingest/_decomposition.md:529-534`; `CHANGELOG.md:1-11` |
| **The port is byte-identical** | `crates/happenstance-core/src/store.rs` is unchanged, and `frozen_signatures.rs`'s `append_takes_no_identity` — generic over `S: EventStore` — still compiles and passes untouched. That function is the compile-level proof that identity never reached `append`'s signature; it is why the assertion is a *compile* before it is a test. | `crates/happenstance-core/tests/frozen_signatures.rs:236-264`; SY-8 `[FROZEN]`, `spec/SPECIFICATION.md:6070-6086` |
| **Every existing handle sees the write** | The door mutates the store behind `&self`; it never rebuilds one. A second handle (`Arc` clone) taken *before* the ingest observes the ingested events afterwards — the property `RestoreBasedIngest` silently breaks and that the suite's `two_handles_observe_each_others_appends` is shaped to catch. | `crates/happenstance-core/src/memory.rs:138-168` (`restore`, the near-miss); `discover.md:129-141` |
| **The doctest is the mount, in miniature** | The rustdoc example builds two stores, appends to the first, takes a `SequencedEvent` from its `snapshot()`, ingests it into the second, and asserts the `id` is still the *first* store's while the `position` is the second store's fresh tail. It is run by `cargo test --doc` in the gate, so the described example is a checked one. | `standards/rust/70-rustdoc-obligations.md`; `CLAUDE.md`, *Commands* |

## Data and migrations

**N/A — no persistent schema, no stored data, no migration.** `MemoryEventStore` holds a
`RwLock<Vec<SequencedEvent>>` in process memory and nothing it holds outlives the process
(`crates/happenstance-core/src/memory.rs:73-76`, `:202-211`). No adapter is touched, so no durable
table, index or column changes; no on-disk or on-wire format is added or altered, and
`crates/happenstance-sync/src/wire.rs`'s envelope is not in this story's boundary.

The one thing that behaves *like* a migration is the published-surface change, and it is handled as
one rather than as a code detail: the new items are additive, the semver class is **minor** and is
stated in ADR-0026 rather than inferred from the diff, the `CHANGELOG.md` `[Unreleased] / ### Added`
entry is part of this PR, and `published-tree-delta-statement`
(`.bklg/from-contract-to-published-library/closeout-and-durable-audience/published-tree-delta-statement/spec.md:323`)
already expects to enumerate exactly this addition against the `0.2.0` registry baseline. Nothing
here is a breaking change and nothing requires a consumer to act.

## Acceptance criteria

The persona served throughout is the **adapter/peer-runner author** on the journey *Learn when you
are finished* — the loop from a signature that type-checks to a suite that says pass or fail and
names why (`.bklg/from-contract-to-published-library/initiative.md:227-247`;
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`). Two
criteria (AC-008, AC-009) serve the **evaluator** on *Decide in one sitting* — the reader who judges
whether this library's published surface is one they can adopt. Each criterion is written as that
person's goal crossing the whole stack, not as a capability the code happens to have.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a peer-runner author holding a `MemoryEventStore` behind `&self` and a batch of `SequencedEvent`s that some *other* store authored, **WHEN** they look for a way to put those events into this store, **THEN** they find exactly one operation — `MemoryEventStore::ingest_at_tail(&self, &[SequencedEvent]) -> IngestedAtTail`, inherent, additive, inside the existing `#[cfg(feature = "memory")]` module — and they reach it without implementing a trait, enabling a new feature, or constructing a second store. | `cargo test -p happenstance-core --all-features` — a unit test in `crates/happenstance-core/src/memory.rs`'s `mod tests` (`:440`) named `ingest_at_tail_is_inherent_and_takes_shared_self` that calls it through `&MemoryEventStore` with no trait in scope; plus the item's doctest, run by `cargo test -p happenstance-core --doc`. |
| AC-002 | **GIVEN** an event authored by store A carrying A's `EventId` and A's `RecordedAt`, **WHEN** the runner ingests it into store B, **THEN** reading it back from B yields byte-identical `id` and `recorded_at` — so the runner can dedupe across the fleet on identity rather than on hope, and `ReMintingIngestDoor` (which re-mints `EventId::new(self.store_id, position)` and leaves every other assertion green) fails this and only this. | `cargo test -p happenstance-core --all-features` — `ingest_preserves_foreign_identity_and_recorded_at` in `memory.rs`'s `mod tests`, asserting against the `EventId` captured from store A's own `snapshot()` (never a literal), and asserting `id.store() != b.store_id()`. |
| AC-003 | **GIVEN** store B whose head is at some position it assigned itself, **WHEN** the runner ingests a foreign event whose own `position` field is *below* that head, **THEN** the event lands at B's fresh tail — above every position B has already assigned — and the incoming `position` is read by nothing, so the runner's projections never have to reason about an event appearing beneath a checkpoint (`OriginPositionInsertingDoor` fails here). | `cargo test -p happenstance-core --all-features` — `ingest_assigns_a_fresh_local_position_above_the_head` in `memory.rs`'s `mod tests`, comparing against positions the store actually assigned (CF-6 bar, `crates/happenstance-testkit/tests/mutation_coverage.rs:326-338`) and never against literals; the foreign event is constructed with a deliberately low origin `position`. |
| AC-004 | **GIVEN** a peer that re-delivers — the normal case, not the exceptional one — **WHEN** the runner ingests the same batch twice, or two tasks ingest overlapping batches concurrently, **THEN** each `EventId` is stored at most once, the second delivery is reported as `skipped` rather than appended or refused, and the decision is taken under the same single write lock as the write, so a runner cannot lose a race it did not know it was in. | `cargo test -p happenstance-core --all-features` — `ingest_skips_ids_the_store_already_holds` and a concurrency test that drives two `ingest_at_tail` calls with an overlapping batch from two threads over a shared `Arc<MemoryEventStore>` and asserts the total stored count, in `memory.rs`'s `mod tests`. |
| AC-005 | **GIVEN** a runner that must advance its own resume token after a push, **WHEN** the call returns, **THEN** it can name the outcome type through the defining crate's root re-export (`happenstance_core::IngestedAtTail`) and read `appended`, `skipped` and `last_local` from it without matching a tuple or paying for an allocation it did not ask for — and the type is `#[non_exhaustive]`, so a later field is not a breaking change for them. | `cargo test -p happenstance-core --all-features` — a test that imports the type *only* as `happenstance_core::IngestedAtTail` and reads all three fields; `cargo doc` / `cargo xtask ci --fast`'s docs step for the re-export; `crates/happenstance-core/tests/frozen_signatures.rs` addition asserting the struct is `#[non_exhaustive]` (a same-crate literal-construction test cannot prove this, so the assertion lives in the out-of-crate `tests/` target). |
| AC-006 | **GIVEN** SY-1's frozen prohibition on ingest re-checking the writer's asserted append conditions, **WHEN** a future implementer tries to make the receiver reject an event, **THEN** they find the signature cannot express it — no `AppendCondition` parameter of any kind, no `Result` — and an empty batch is a no-op returning `appended: 0, skipped: 0, last_local: None` rather than the `AppendError::NoEvents` that `append` owes, because a peer with nothing new to send is ordinary traffic. | `cargo test -p happenstance-core --all-features` — `ingest_at_tail_of_nothing_is_a_no_op` in `memory.rs`'s `mod tests`; plus a compile-level assertion in `crates/happenstance-core/tests/frozen_signatures.rs` that pins the function's type (`let _: fn(&MemoryEventStore, &[SequencedEvent]) -> IngestedAtTail = MemoryEventStore::ingest_at_tail;`), which stops compiling the moment a parameter or a `Result` is added. |
| AC-007 | **GIVEN** a runner that already handed clones of its `Arc<MemoryEventStore>` to a projection task and a query path, **WHEN** it ingests a batch through its own handle, **THEN** every handle taken *before* the ingest observes the ingested events afterwards — the store is mutated in place behind `&self` and never rebuilt, which is precisely what `RestoreBasedIngest` (`snapshot()` + `restore`) silently breaks while preserving identity correctly. | `cargo test -p happenstance-core --all-features` — `ingest_is_visible_through_a_handle_taken_before_it` in `memory.rs`'s `mod tests`, shaped after `crates/happenstance-testkit/src/suite.rs:265`'s `two_handles_observe_each_others_appends`; plus the existing event-store conformance suite staying green (`cargo test -p happenstance-testkit --all-features`). |
| AC-008 | **GIVEN** an evaluator or an adapter author who has already implemented `EventStore` against the published `0.2.0` contract, **WHEN** this change lands, **THEN** nothing they wrote needs to change: `crates/happenstance-core/src/store.rs` is byte-identical, no port gained an ingest-shaped method (SY-8 `[FROZEN]`), no new Cargo feature appeared, and `frozen_signatures.rs`'s existing `append_takes_no_identity` still compiles and passes **unedited**. | `git diff --exit-code main -- crates/happenstance-core/src/store.rs crates/happenstance-core/Cargo.toml` reports no change; `git diff main -- crates/happenstance-core/tests/frozen_signatures.rs` shows additions only, no edits to `:236-296`; `cargo test -p happenstance-core --test frozen_signatures --all-features` green; `cargo xtask spec-trace` green (no `[FROZEN]` clause edited). |
| AC-009 | **GIVEN** an evaluator deciding in one sitting whether to adopt, **WHEN** they read the new item, **THEN** it arrives fully presented rather than bare: rustdoc that names the alternative that lost, a compiled doctest showing a real two-store round trip with the origin `id` surviving and the local position fresh, a `CHANGELOG.md` `[Unreleased] / ### Added` entry, and the **semver class stated out loud** (minor) and traceable to ADR-0026 rather than inferred from the diff. | `cargo test -p happenstance-core --doc` compiles and runs the doctest; `cargo xtask ci --fast`'s docs step green with `-D rustdoc::broken_intra_doc_links`; manual-but-gated: the `CHANGELOG.md` entry and the rustdoc semver sentence are cited `file:line` in `_ledger.md`, and must agree with the accepted `.kb/decisions/` ADR-0026 atom. |
| AC-010 | **GIVEN** the peer-runner author whose journey today dead-ends at a `todo!()` whose message *is* this story's absence (`crates/happenstance-sync/src/ingest.rs:219-221`), **WHEN** they open the real composition root, **THEN** the `#[cfg(feature = "memory")] mod memory_store_ingest` block reaches the new door from live code and the whole block compiles under the `memory` feature — the seam is mounted in the tree, not merely available in a crate nobody calls. | `cargo test -p happenstance-sync --features memory` compiles the block (a plain `cargo test -p happenstance-sync` compiles none of it — `crates/happenstance-sync/Cargo.toml:37-44`); `cargo xtask affected --base main` green; the module documentation at `crates/happenstance-sync/src/ingest.rs:38-82` and `:200-205`, which currently asserts the door cannot exist, cites ADR-0026 and the new operation instead. |

**Coverage of the traced project AC.** Project **AC-002** (*the central question has a written
answer, reconciled against SY-1/SY-6, and no frozen clause is edited in this project's diff* —
`.../replication-identity-and-ingest/project.md:189-195`) is split per `_storymap.md`'s coverage
table: ADR-0026 writes the answer, and this story owns the **structural** half. AC-006 is the answer
made unexpressible in a signature; AC-008 is the "no frozen clause edited, and the port did not have
to move" half; AC-003 and AC-002 are SY-5/SY-19/VT-5 honoured by construction. No other project AC
is claimed here.

## Interaction quality

`_design.md` declares **no surfaces** for this project and that determination is itself what was
signed off (`_design.md:10-22`, `:116-125`), delegating the surface question to the **public API**
(`_design.md:58-71`). So the composition family below is read against the API surface, which is the
only thing a person meets here — and the analogue is exact, not a stretch: an API with correct
behaviour and no rustdoc, no doctest, no changelog entry and no stated semver class is the library's
version of an unstyled render, and it passes every behavioural assertion in this spec perfectly.

Every invariant below is carried by a row in the table above. Nothing in this section is a
standalone requirement.

**STATE invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — the operation mutates the live store behind `&self`; it never rebuilds one and never hands back a replacement the caller must swap in. | AC-001, AC-007 | `ingest_is_visible_through_a_handle_taken_before_it`; the `fn` type pin in `frozen_signatures.rs` fixes the receiver as `&MemoryEventStore`. |
| **Non-occlusion** — the addition hides nothing that already worked. `ingest_at_tail` is deliberately *not* named `ingest`, because an inherent method wins method resolution over a trait method on the same type and `impl SendIngestStore for MemoryEventStore` puts `ingest` on this very type; and `EventStore`/`store.rs` is byte-identical. | AC-001, AC-008 | `cargo test -p happenstance-sync --features memory` compiles both the inherent call and the trait impl in one scope; `git diff --exit-code main -- crates/happenstance-core/src/store.rs`. |
| **Preserved identity/selection** — the caller's own values survive the operation untouched: `id` and `recorded_at` verbatim, and every position the store had already assigned still means what it meant. | AC-002, AC-003 | `ingest_preserves_foreign_identity_and_recorded_at`; `ingest_assigns_a_fresh_local_position_above_the_head`. |
| **Reversibility / safe repetition** — there is no destructive step to undo, and repeating the call is defined rather than merely tolerated: re-delivery is a skip, and an empty batch is a no-op. | AC-004, AC-006 | `ingest_skips_ids_the_store_already_holds`; `ingest_at_tail_of_nothing_is_a_no_op`. |
| **Reachability** — the library analogue of keyboard reachability is that the capability is reachable from the real composition root without a private item, an unpublished type, or a feature the caller cannot turn on. The result type is nameable through the crate root; the door is reachable from `memory_store_ingest`. | AC-005, AC-010 | The root-re-export test; `cargo test -p happenstance-sync --features memory`. |

**COMPOSITION invariants** (read against `_design.md:58-71`'s API-surface obligation, since no visual
surface exists)

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the item is not bare markup: rustdoc prose, a compiled doctest, the alternative that lost named once, a changelog entry, and a stated semver class. | AC-009 | `cargo test -p happenstance-core --doc`; the docs step of `cargo xtask ci --fast`; `_ledger.md` cites the `CHANGELOG.md` and rustdoc lines. |
| **Density budget, with its real numbers** — **2** new public items (one method, one struct), **0** new traits, **0** new Cargo features, **0** changes to any port, **3** public fields on the result. A third public item is over budget and needs a recorded decision. | AC-001, AC-005, AC-008 | `git diff main -- crates/happenstance-core/src/` reviewed against the budget; `git diff --exit-code main -- crates/happenstance-core/Cargo.toml`; `cargo public-api`-free check by inspection is not sufficient — the `frozen_signatures.rs` pin is what makes it mechanical. |
| **Hierarchy** — the method is the primary item and `IngestedAtTail` is subordinate to it: named after the operation, re-exported at the root beside the store it serves, and documented from the method rather than the other way round. | AC-005 | Rustdoc intra-doc links resolve under `-D rustdoc::broken_intra_doc_links`; the root re-export test. |
| **Transience** — nothing here is revealed on demand or hidden behind a flag day: the capability is persistent chrome (plain `pub`, inheriting only the module's existing `memory` cfg). An off-by-default `unstable-ingest` gate was considered and rejected on the record (*Behavior and interfaces*, visibility row). | AC-001, AC-008 | `git diff --exit-code main -- crates/happenstance-core/Cargo.toml`; `cargo xtask ci --fast`'s feature-powerset step. |
| **Named anti-patterns** — the three wrong implementations from `discover.md:104-152` (`ReMintingIngestDoor`, `RestoreBasedIngest`, `OriginPositionInsertingDoor`) must each be rejected by a *named, distinct* test rather than by the suite in aggregate. | AC-002, AC-003, AC-007 | One test per anti-pattern, as listed in those rows; each must fail if and only if its own anti-pattern is written. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | The batch is empty (`&[]`). | Not an error. Returns `appended: 0, skipped: 0, last_local: None`. Deliberately unlike `append`'s `AppendError::NoEvents` (`crates/happenstance-core/src/memory.rs:343-362`), because `append` must return *a* position and this return type has `Option` and nothing to invent. Carried by AC-006. |
| EC-002 | The batch contains an `EventId` the store already holds — including an id this store itself minted, arriving back around a loop. | Not an error. Skipped, counted in `skipped`, decided inside the same write lock. No origin check, no rejection branch: a rejection is the refusal SY-1 `[FROZEN]` forbids (`spec/SPECIFICATION.md:5841-5867`). Carried by AC-004. |
| EC-003 | The batch contains two events with the *same* `EventId`. | The first is appended, the second is skipped by the same predicate — the dedupe set must reflect writes made earlier in *this* call, not just the store's state on entry. A per-event snapshot of the held set is the wrong implementation here. Carried by AC-004. |
| EC-004 | The `RwLock` is poisoned by a panic in another thread. | Recovered exactly as `append` recovers, by the existing policy at `crates/happenstance-core/src/memory.rs:190-199`. The door introduces no second poisoning policy and no new `Err` variant; `MemoryStoreError` stays uninhabited (`:284-291`). |
| EC-005 | An incoming `SequencedEvent` carries a `position` that is below, equal to, or wildly above this store's head. | All three are ordinary. The field is read by nothing; a fresh local tail position is assigned regardless (SY-19 `[FROZEN]`, `spec/SPECIFICATION.md:6409-6428`). There is no validation branch and therefore no error. Carried by AC-003. |
| EC-006 | **Process-level, not runtime:** ADR-0026 as accepted does not authorise a change to `happenstance-core`, or states a different shape, visibility or semver class than this spec. | Stop. ADR-0026 outranks this spec (*Context pack* §5). A different class or visibility is recorded as a divergence in `_ledger.md` and followed; a refusal of the core-side change altogether is `RUNBOOK.md:462-466`'s residual risk landing hard — raise it as a blocker for a new atom and a re-plan, never improvise option (c) in the moment. |

## Non-functional

| id | requirement | why, and how it is checked |
| --- | --- | --- |
| NF-001 | **One lock acquisition per call.** The whole operation runs under a single `self.events.write()`. | Group atomicity cannot be composed above a per-event door that takes N locks, and two-lock (`contains_event_id` then write) callers race. `crates/happenstance-core/src/memory.rs:364-370`. Checked by the concurrency test in AC-004 and by inspection of the diff. |
| NF-002 | **Dedupe cost is stated, not discovered.** The reference store's membership check is a linear scan (`crates/happenstance-core/src/memory.rs:414-419`), so one call is O(stored × batch). That is acceptable for an oracle and must be said in the rustdoc, not silently shipped. | `MemoryEventStore` is a reference implementation, not a production store; an implementer who reaches for a `HashSet` here is changing the store's shape and needs a decision. Checked by the rustdoc sentence, cited in `_ledger.md`. |
| NF-003 | **Total on `wasm32`.** The door mints no identity and stamps no time, so it never calls `SystemTime::now()`, which aborts on that target (`crates/happenstance-core/src/memory.rs:239-274`). | `cargo xtask wasm` — the `happenstance-core` `wasm32` build inside `cargo xtask ci --fast`. |
| NF-004 | **No new dependency and no new Cargo feature** in `happenstance-core`. `serde` stays out of default features (ADR-0003, binding constraint 2). | `git diff --exit-code main -- crates/happenstance-core/Cargo.toml`; `cargo xtask ci --fast`'s `cargo hack` feature-powerset and `cargo deny` steps. |
| NF-005 | **MSRV unchanged at 1.97.1** (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`). Nothing here needs a newer language feature; if the implementation reaches for one, that is a recorded decision, not a convenience. | CI's `msrv` job (`cargo hack check --no-dev-deps --rust-version` plus a full test run at 1.97.1). |
| NF-006 | **Docs build with `--no-default-features`.** The new items sit behind the existing `memory` cfg and must not break the no-default-features doc build or leave a dangling intra-doc link in it. | The `--no-default-features` doc build step of `cargo xtask ci --fast`. |
| NF-007 | **`unsafe`-free and panic-free on every documented path.** No `unwrap()` on caller-supplied data; the only panic path is the pre-existing poisoning policy. | `cargo clippy --workspace --all-targets -- -D warnings` inside `cargo xtask ci --fast`. |

## Implementation notes (non-prescriptive)

Shape is binding (*Behavior and interfaces*); the following is how the author of this spec expects it
to go, and an implementer who finds a better route inside the same shape should take it and say so in
`_ledger.md`.

- **Write the `frozen_signatures.rs` pin first.** The `fn`-type assertion in AC-006 is the cheapest
  possible guard and it fails to compile the instant someone adds an `AppendCondition` or a `Result`.
  Landing it before the body means the frozen constraint is enforced by the compiler for the whole
  rest of the story rather than remembered.
- **The dedupe set is the subtle part.** EC-003 says the set must include events appended earlier in
  the same call. The simplest correct thing is to hold the write guard, iterate the batch, and check
  membership against the vector as it grows — which is also why NF-002's cost has to be written down
  rather than optimised in silence.
- **Position assignment must reuse `position_at`** (`crates/happenstance-core/src/memory.rs:276-282`),
  not open-code an increment. Two rules that compute the same thing drift, and `append` already owns
  the rule.
- **The doctest is the mount in miniature** and is the best place to discover a shape problem: two
  stores, an append to the first, a `SequencedEvent` lifted from its `snapshot()`, an
  `ingest_at_tail` into the second, then assert `id` is still the first store's and the local
  position is the second store's fresh tail. If that example is awkward to write, the signature is
  wrong — fix the signature, in this spec, before writing the body.
- **The mount is not optional and is not the slice-mate's to remember.** If
  `ingest-store-and-memory-peer-round-trip` has not yet taken the
  `memory_store_ingest::ingest` body, this story wires it far enough to prove the door is reachable
  from live code; the remaining `todo!()`s (`store_id`, `holds`, `watermark`) stay. Correct the
  module documentation at `crates/happenstance-sync/src/ingest.rs:38-82` and `:200-205` in the same
  change — it currently asserts, in prose, that this door cannot exist.
- **Do not touch the crate-level `#![allow(clippy::todo)]`** (`crates/happenstance-sync/src/lib.rs:135-139`).
  Removing it is HS-S0102's, and removing it early turns three legitimate remaining `todo!()`s into
  a red gate that this story cannot green.
- **Read ADR-0026 before the first line of code**, not after the body is written. Everything in this
  spec's *Behavior and interfaces* table is subordinate to it.

## Tests and CI (merge gate)

Tiers per the project testing brief
(`.../replication-identity-and-ingest/_decomposition.md:625-800`). This story has **no integration
tier**: the conformance rules that observe this behaviour (`ingested_events_land_above_the_local_head`,
SY-5; `ingest_preserves_origin_identity`, VT-5) belong to `happenstance-sync-testkit`, which does not
exist until HS-S0103 — that is a deliberate, recorded absence, not a gap.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo xtask affected --base main` | The story-grain gate: nothing this diff could break is broken. The first command run, and the one `.redkiln/config.yaml` wires to the story grain. |
| Static | `cargo xtask lints && cargo xtask spec-trace` | `reachability_static` (`.redkiln/config.yaml:48`), unconditional per story; and that no `[FROZEN]` clause was edited and no `Rejects:` symbol rotted — AC-008's spec-side half. |
| Static | `git diff --exit-code main -- crates/happenstance-core/src/store.rs crates/happenstance-core/Cargo.toml` | AC-008's port half, mechanically: the port is byte-identical and no Cargo feature appeared. NF-004. |
| Static | `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings` (inside `cargo xtask ci --fast`) | NF-007; house style. |
| Unit | `cargo test -p happenstance-core --all-features` → `crates/happenstance-core/src/memory.rs` `mod tests` (`:440`) | AC-001, AC-002, AC-003, AC-004, AC-005, AC-006, AC-007 — one named test per criterion, positions compared against what the store assigned rather than against literals (CF-6). |
| Unit | `cargo test -p happenstance-core --test frozen_signatures --all-features` → `crates/happenstance-core/tests/frozen_signatures.rs` | AC-006's compile-level pin and AC-005's `#[non_exhaustive]` assertion, **added** beside `:236-296` without editing it; and that `append_takes_no_identity` still passes unchanged (AC-008). |
| Unit (doc) | `cargo test -p happenstance-core --doc` | AC-009: the rustdoc example is a *checked* example, and it is the mount in miniature. |
| Unit | `cargo test -p happenstance-testkit --all-features` | AC-007's regression half: the existing event-store conformance suite, including `two_handles_observe_each_others_appends` (`crates/happenstance-testkit/src/suite.rs:265`), stays green — the door must not perturb the reference store the whole suite is calibrated on. |
| Mount | `cargo test -p happenstance-sync --features memory` | AC-010: the composition root compiles against the new door. The feature is load-bearing — a plain `cargo test -p happenstance-sync` compiles none of `memory_store_ingest` (`crates/happenstance-sync/Cargo.toml:37-44`). |
| Integration-scoped | `cargo xtask ci --fast` | This project's ceiling per DoD 1 (`.redkiln/config.yaml:55`): fmt, clippy, the full test run, the four `wasm32` steps (NF-003), docs, `spec-trace`, the `--no-default-features` doc build (NF-006), the `cargo package --list` licence/README assertion, and — where the tools resolve — `cargo hack` and `cargo deny` (NF-004). |
| KB | `redkiln validate --kb && redkiln doctor` | That ADR-0026 is an accepted atom whose body this story did not edit, and that the six expected `template-drift` advisories are still exactly six. |

**Not run here, and why.** No E2E: `.redkiln/config.yaml:57-60` reserves `cargo xtask ci` for the
terminal project and this project is `terminal: false`. No sync conformance run: the suite does not
exist yet (HS-S0103). No mutant: mutants are slice 3's, and a mutant for a rule that does not exist
is a mutant of nothing.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation, in this PR |
| --- | --- | --- |
| **ADR-0026 authorises a different shape, or no core-side change at all.** This story's entire *Behavior and interfaces* table is downstream of an atom that lands in the story before it. | Low / High | EC-006 is the standing instruction. The dependency edge exists precisely to make this discoverable before any code is written; the first implementation act is reading the accepted atom. If it refuses the core change, this is a blocker and a re-plan, and `RUNBOOK.md:462-466` already anticipated it. |
| **`ReMintingIngestDoor` by accident.** The code path the door is grafted onto mints an `EventId` for every event it writes; preserving the incoming one is a two-line difference from the default behaviour, and everything else stays green if you get it wrong. | Medium / High | AC-002 is a dedicated named test asserting against the *origin's* captured id and that `id.store() != self.store_id()`. This is the single most likely defect in the story and the reason that test is not folded into another. |
| **Name collision with `IngestStore::ingest`.** An inherent method wins method resolution over a trait method on the same type; if this door were named `ingest`, every call site in `memory_store_ingest` with the trait in scope would silently resolve to the inherent one. | Medium / Medium | The name `ingest_at_tail` is normative in *Behavior and interfaces*, and AC-010's mount compiles both in one scope, which is where the collision would surface. |
| **Scope creep into the slice-mate.** Writing the door and then "just finishing" `holds` and `watermark` merges two stories and destroys the merge-order proof. | Medium / Low | *PR boundary* is explicit; `#![allow(clippy::todo)]` stays; the ledger's AC set contains nothing about `IngestStore`'s other bodies. |
| **Coupling to the published surface.** This is the only item this project adds to a published crate. If it lands with the wrong semver class or without a changelog entry, `published-tree-delta-statement` (HS-P0019) inherits a diff it cannot explain. | Low / Medium | AC-009 makes the class and the changelog entry acceptance criteria, and that story already cites this one by name (`closeout-and-durable-audience/published-tree-delta-statement/spec.md:323`). |
| **A test that passes here and would fail against a real adapter.** `MemoryEventStore` assigns densely from 1, so a literal-position assertion passes locally and dies against `GappedPositionStore`. | Medium / Medium | CF-6's bar is written into AC-003's verification; `cargo xtask lint-position-literals` runs in the gate. |

## Dependencies

**Blocks on** (this story cannot start until these merge):

- `adr-0026-peer-ingest-and-transport` (HS-S0098) — supplies the authorisation for a change to the
  published contract crate and the **stated semver class**, which is AC-A01's requirement. Without
  it, AC-009 cannot be satisfied and EC-006 has no answer to check against.

**Unlocks** (these cannot start, or cannot be written truthfully, until this merges):

- `ingest-store-and-memory-peer-round-trip` (HS-S0102) — the slice-mate. Its `IngestStore::ingest`
  body is the direct consumer of this door; today its `todo!()` message is a statement of this
  story's absence.
- `headline-rules-and-mutant-registry` (HS-S0105) — `ingest_never_rejects` (SY-1) needs a peer that
  actually ingests before a rule can observe it not rejecting.
- `frozen-clause-repairs` (HS-S0113) — SY-12's live wrong implementation (*"`impl IngestStore for
  MemoryEventStore` cannot be written truthfully"*) is retired by this slice; the repair is that
  story's, and it needs this to have landed to be true.
- `published-tree-delta-statement` (HS-P0019's story) — enumerates this addition against the `0.2.0`
  registry baseline.

## Anchors (progressive disclosure)

The *Context pack* above is the must-read core and is self-sufficient. Open the following at the
moment named, not before — each is deferred depth, not optional depth.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.kb/maps/decision-map.md` | The index that resolves ADR-0026 to its atom path once the predecessor story's kb-ingest has run. ADR-0026's file does not exist yet, so its path must be looked up rather than guessed. | **First, before any code.** Resolve ADR-0026, read it, and check its shape/visibility/semver statements against *Behavior and interfaces*. | AC-009 |
| `.kb/decisions/0014-event-identity-and-recorded-time.md` | The accepted atom that says the store mints identity and the caller supplies neither — and whose *Provisional* section (`:96-103`) names this exact open item: whether the ingest seam that accepts foreign identity belongs on `happenstance-sync` rather than on `EventStore`. It is the reason the door is inherent on a concrete type. | Before writing the signature, to confirm this story is inside ADR-0014's stated exception rather than contradicting it. | AC-001, AC-008 |
| `crates/happenstance-core/src/memory.rs` | The file being changed. `append`'s single-write-lock body (`:364-400`) is the shape to copy; `position_at` (`:276-282`) is the rule to reuse; `restore` (`:138-168`) is the near-miss to *not* use; the poisoning policy is at `:190-199` and `contains_event_id` at `:414-419`. | Immediately before writing the body. | AC-001, AC-002, AC-003, AC-004, AC-007 |
| `crates/happenstance-sync/src/ingest.rs` | The mount point. `:52-71` is the leak's own account of why the door has to be a store operation; `:193-231` is the block to compile against; `:178-191` is `Ingested`, the name the new result type must not collide with; `:38-82` and `:200-205` are the prose asserting this door cannot exist, which this PR corrects. | When wiring the mount, and again when naming the result struct. | AC-005, AC-010 |
| `crates/happenstance-core/tests/frozen_signatures.rs` | The standing compile-level guard (`:236-296`) that identity never reached `append`. New assertions go *beside* it; its existing functions are not edited, and that fact is itself AC-008's evidence. | Before writing any test — the AC-006 pin is the first thing to land. | AC-006, AC-008 |
| `spec/SPECIFICATION.md` | The four frozen clauses that constrain the signature: SY-1 `:5841-5867`, SY-5 `:5954-5971`, SY-6 `:5973-6029`, SY-8 `:6070-6086`, SY-19 `:6409-6428`, VT-5 `:732-745`. The *Context pack* distils them; open the clause text when a judgement call is close. | When any design question feels open — the clause wins over this spec. | AC-002, AC-003, AC-006, AC-008 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/memory-store-ingest-seam/discover.md` | The three named wrong implementations in full (`:104-152`), with the mechanism by which each stays green while being wrong, and the five deferred questions (`:54-78`) this spec answers. | Before writing the anti-pattern tests — the failure mechanisms are what make the tests discriminating. | AC-002, AC-003, AC-007 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | The architecture brief's *Composition root* §5 prices all four seam options (`:139-160`) and AC-A01 states the semver-class obligation (`:529-534`); the testing brief's tier table (`:625-800`) is what the *Tests and CI* section is grounded in. | When tempted by option (b) or (c), and when adding a test tier not listed above. | AC-001, AC-008, AC-009 |
| `standards/rust/40-public-surface-and-evolution.md` | RS-40-3 (`:127`) and RS-40-4 (`:171`) — why the result is a named struct re-exported from the crate root rather than a tuple, and what a caller must be able to name. | While writing the result type and the `lib.rs` re-export. | AC-005 |
| `standards/rust/13-sealing-and-exhaustiveness.md` | RS-13-4 (`:144`) — what `#[non_exhaustive]` buys and costs, and why the caller-side test has to live outside the defining crate to prove it. | While writing `IngestedAtTail` and its `frozen_signatures.rs` assertion. | AC-005 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-5 (`:243`) — the obligation to name the alternative that lost, once, in the rustdoc. AC-009's "presentation exists at all" is this atom made an acceptance criterion. | While writing the rustdoc and the doctest. | AC-009 |
| `standards/rust/60-what-a-test-must-prove.md` | RS-60-4's "spell the oracle in a direction that shares no subroutine with the implementation" — the reason AC-002's test must assert against the id captured from store A's `snapshot()` rather than recomputing what the door should have stored. | While writing the unit tests, before choosing how to build the expected value. | AC-002, AC-003 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `GappedPositionStore` (`:326-338`) is the conformant variant a literal-position assertion trips over — the concrete reason CF-6's bar applies to this story's own unit tests even though it adds no conformance rule. | Before asserting on any position value. | AC-003 |
| `crates/happenstance-testkit/src/suite.rs` | `two_handles_observe_each_others_appends` (`:265`) is the existing rule shaped to catch exactly the defect `RestoreBasedIngest` introduces; AC-007's test is modelled on it. | While writing AC-007's test. | AC-007 |
| `RUNBOOK.md` | `:450-466` is the leak warning and the residual risk this story is the scheduled landing of — including the instruction to stop and re-plan if a real peer needs a seam on the *port*. `:4516-4620` is phase 13's plan of record. | Only if ADR-0026 refuses the core-side change (EC-006), or if the door will not fit on a concrete type. | AC-008 |
| `CHANGELOG.md` | The `[Unreleased] / ### Added` section (`:13-22`) and the format contract at `:1-11`. The entry is part of this PR, not a follow-up. | Last, with the semver class taken from ADR-0026 rather than inferred. | AC-009 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half declared** — AC-001 through AC-010, none added and
   none dropped. Each of the ten *Behavior and interfaces* rows either became an AC or is subsumed by
   one: the "why not `ingest`" row is folded into AC-001's non-occlusion invariant, the "no origin
   validation" row into EC-002, and the "one call is one atomic unit" row into AC-004 and NF-001.
2. **Whether the door takes one event or a slice — resolved: a slice.** `discover.md:70-77` left it
   open. A slice is what makes NF-001's single lock acquisition possible, and group atomicity is then
   composed above it by calling once per group — which is exactly how `IngestStore::ingest`'s "each
   group lands atomically or not at all" is expressed. A per-event door cannot be made group-atomic
   from above without leaking the lock.
3. **Whether the operation is `pub` or feature-gated — resolved: plain `pub`**, inheriting only the
   module's existing `#[cfg(feature = "memory")]`. `discover.md:79-90` flagged that this question is
   *not* symmetric with the sync-side one because `happenstance-core` is published. An
   off-by-default `unstable-ingest` gate was considered and rejected: no `unstable-*` convention
   exists in the tree, and introducing one belongs to `projection-store-freeze`, not here.
4. **The empty-batch divergence from `append` is deliberate and is now an acceptance criterion**
   (AC-006 / EC-001), because it is the kind of asymmetry a reviewer would otherwise flag as an
   oversight. The justification is that `append` must return a position and cannot invent one, while
   this return type carries `Option<SequencePosition>`.
5. **EC-003 (a duplicate id *within* one batch) was not raised by discovery and is added here.** It
   is the natural implementation slip once the dedupe set is snapshotted on entry, and it is cheap to
   state and cheap to test. It sits under AC-004 rather than as an eleventh AC.
6. **No conformance rule and no mutant, restated as a decision rather than an omission.** The rules
   that observe this behaviour are SY-5's and VT-5's and they belong to `happenstance-sync-testkit`
   (HS-S0103). Adding a rule here would mean adding it to the *event-store* suite, where it would be
   a rule about replication that every store adapter must answer — which is option (b) in a different
   costume, and SY-8 `[FROZEN]` refuses it.
7. **`spec/SPECIFICATION.md` is not edited by this story**, including SY-12, whose live wrong
   implementation this slice retires. `frozen-clause-repairs` (HS-S0113) owns that repair, and the
   sequencing is deliberate: a repair can only name the rules and symbols that exist.
