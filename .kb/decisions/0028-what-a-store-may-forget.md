---
id: kb-decision-0028
title: What a store may forget is decided outside the port, and a report of it can only arrive additively
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0028
reversibility: medium
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Settles the retention question phase 14 was to answer and ADR-0066 moved to phase 17, because
  one of its two answers adds a required method to EventStore and breaks every published adapter.
  It takes the written refusal, with an additive reservation. Deletion, truncation, compaction,
  redaction and tombstoning stay outside the port through 1.x, so ES-37 is re-dated from "at 0.1".
  ES-38 is the whole answer to what a store that has been deleted from may look like, and ES-40's
  MAY stands: a condition over removed history passes vacuously, and E2E-47's third outcome is
  rejected for 1.x. If phase 14's instrument shows a reader needs a port-level report, it arrives
  as a provided EventStore method whose default answers Unknown, never Complete. A compiling spike
  (experiments/provided-method-spike) showed that such a method is additive under trait_variant
  0.1.3 on Send and !Send implementors, host and wasm32, with cargo-semver-checks silent against
  0.3.2 and a required-method control reported as major. The floor primitive, earliest_position,
  is rejected on ES-39's own grounds: a regulated purge is scattered, not a prefix. ES-41 and
  PS-22 are frozen; ES-39 is rewritten and stays deferred with phase 14 as its settler; SY-32's
  floor is narrowed to resumability; CF-27's report is instrument-local. Tag redaction is refused
  and a crypto-shred of data moves no position, so retention never rewinds a checkpoint over kept
  rows. Nothing published changes in 0.4.0.
depends_on:
  - kb-decision-0013
  - kb-decision-0066
  - kb-decision-0072
related:
  - kb-decision-0063
  - kb-decision-0073
  - kb-decision-0024
  - kb-open-question-es-38-and-gap-read-unowned-001
  - kb-open-question-postgres-neon-store-id-no-restore-001
source_paths:
  - experiments/provided-method-spike/README.md
  - crates/happenstance-core/src/store.rs
  - crates/happenstance-core/src/append.rs
  - crates/happenstance-sync/src/peer.rs
  - spec/SPECIFICATION.md
  - spec/E2E-CASES.md
last_reviewed: 2026-09-29
---

# What a store may forget is decided outside the port, and a report of it can only arrive additively

## The question

What is a store permitted to forget, and how does it say so? ES-39, CF-27 and SY-32 carried it,
and E2E-CASES' eleventh open decision named its two legitimate answers:
- **a port surface** through which a store reports the history it does not hold;
- **a written refusal**: deletion is out of scope for `EventStore`, and here is what a store that
  has been deleted from may look like.

The first, as a required method, breaks `happenstance-core` and all four published adapters.
It is cheap only before 1.0, which is why ADR-0066 moved the decision to phase 17.

## Decision

**The refusal, with an additive reservation.** Nothing published changes in `0.4.0`.

1. **Deletion stays outside the port through 1.x.** `EventStore` grows no delete, truncate,
   compact, redact or tombstone method in any 1.x release. ES-37 said "at 0.1", which went stale
   at `0.2.0`; this record re-dates it and is its authorising record. A purge and its compliance
   marker (E2E-48) are one unit of work only in the adapter's own transaction, and 1.x says so
   rather than pretending otherwise.
2. **ES-38 is the answer to what a deleted-from store may look like.** It keeps every promise
   about the events it still holds: no position reused, survivors unique and monotonic, and
   `Query::all()` meaning what this store holds. **ES-40's MAY stands.** A condition over removed
   history passes vacuously, and `crates/happenstance-core/src/append.rs:29-47` already documents
   it. E2E-47's third outcome, "this log cannot evaluate this condition", is **rejected for 1.x**.
   An `AppendError` variant would type-check, since the enum is `#[non_exhaustive]`, but an append
   admitted today would start to fail, which is a behaviour break. No adapter could produce it
   anyway, because none can see what was removed outside it. If one is ever wanted, it arrives as an
   opt-in, a new method or condition option, never as a change to `append`.
3. **The reservation.** If phase 14's instrument shows that a runner or an ingest path needs a
   port-level report, it arrives as a **provided** `EventStore` method whose default answers
   **`Unknown`**. A default of `Complete` is the lie ES-39's *Rejects* line names, because it
   passes every test anyone would write for it, and it is refused here in writing. The spelling
   is ES-4's: `fn … -> impl Future<Output = Result<_, Self::Error>> { async { … } }`, never
   `async fn` with a body. The body captures neither `&self` nor any `Self::` associated value, so
   it needs no `Self: Sync` (ES-3). The name, the report type and its variants are phase 14's.
4. **The floor is rejected.** `earliest_position()` fits a device pruning a prefix and is wrong
   for a regulated purge, which is scattered: a 2019 injury file with a periodical payment order
   survives at a low position while its neighbours are destroyed (ES-39, E2E-46). No 1.x surface
   reports completeness as a floor.
5. **ES-41 is frozen.** See *ES-41* below.
6. **PS-22 is frozen.** **Retention never rewinds a checkpoint over kept rows.** See *PS-22*.
7. **SY-32's floor means resumability, not completeness.** See *SY-32*.
8. **CF-27's report is instrument-local.** See *CF-27*.
9. **ES-39 is rewritten and stays `[DEFERRED]`,** with phase 14 and CF-27's instrument as its
   settler. At 14 it either freezes as the refusal (the port does not report completeness, and a
   caller MUST NOT infer it from any port value, with ES-38's and ES-40's rules as its checkable
   consequences) or gains the provided method. If phase 14 slips past 1.0, the row may become
   `renew-past-1.0`, because by decision 3 firing it is additive, which is ADR-0066's test for
   renewal. Phase 14 does not have to argue that.

## The evidence that the reservation is real

`experiments/provided-method-spike/README.md` asked whether a provided method can be added to a
`#[trait_variant::make(SendEventStore: Send)]` pair after `0.3.2` without breaking anything. It
stated a three-part pass criterion before anything ran, and tested on a scratch worktree of
`52aa951` with `trait-variant` 0.1.3 and rustc 1.97.1.

**`EventStore::history`, defaulting to `Unknown`, passed all three parts:**
- **Implementors.** Every in-tree implementor compiled unchanged, Send and `!Send`, under
  `cargo check --workspace --all-features --all-targets` and the gate's five wasm32 checks,
  `happenstance-cloudflare` on the target where it is `!Send` among them.
- **Generic use.** It was called from `S: EventStore` code and spawned from
  `S: SendEventStore + 'static` code with **no `Sync` bound**.
- **Semver.** `cargo-semver-checks` 0.50.0 reported nothing against the published `0.3.2`, in
  `happenstance-core` or in `happenstance`.

**The control is what makes the silence mean something.** The same invocation on the same
worktree, with one *required* method added instead, reported a major `trait_method_added`.

The spike also found a rule that ES-3 states only half of: **a provided body's future must
capture nothing whose auto-traits the trait does not bound.** A `ProjectionStore` default that
moved `Self::Batch` into its future failed in the derived flavour, and rustc's suggested fix, an
attribute-level `Batch: Send`, is ES-3's trap in another place.

**What the spike does not license:**
- the spike's names;
- a claim that no consumer can break. A new defaulted method can still make a same-named method
  from another trait in scope ambiguous (`E0034`). Cargo's semver reference classes that as a
  possibly-breaking minor change, and this record accepts that cost for whatever name phase 14
  chooses.

## Why not a required method

The tree says a required method would mostly be answered "unknown", so it would buy only the
break. Every published adapter forgets outside the port, where no adapter code runs:
- **SQLite and Cloudflare** allocate with `INTEGER PRIMARY KEY AUTOINCREMENT`
  (`crates/happenstance-sqlite/src/event_store.rs:285`,
  `crates/happenstance-cloudflare/src/event_store.rs:184`).
- **Postgres and Neon** allocate from a sequence read explicitly
  (`crates/happenstance-postgres/src/event_store.rs:938`,
  `crates/happenstance-neon/src/config.rs:94`).
- A raw `DELETE`, a Postgres `TRUNCATE` (which fires no row trigger) and a Durable Object's
  `delete_all()` (`worker-0.8.5/src/durable.rs:449`) are all invisible to the adapter.

An honest answer would need, on each of four crates, a delete trigger, a holes table and a
migration. Even then `delete_all()` defeats it, because it wipes the bookkeeping along with the
log. The rejected alternative would break those four crates, the testkit's own implementors
(`FaultyStore`, `SendFaultyStore`, `MemoryHandle`, `GappyMemoryStore`) and every stranger's
adapter, and it would churn about 35 in-tree mutants.

**Why the class and not the shape.** ES-39's marker says the primitive is chosen by building
the instrument and watching which reader needs what. That instrument is phase 14's and does not
exist, so choosing a primitive here would be argument. What is decidable now is the *surface
class*: nothing breaking in `0.4.0`, and anything later additive.

## ES-41

The marker's falsifier is an adapter that cannot answer membership without a structure VT-8 does
not already oblige. It had two halves.

**Transport.** Since phase 8 the marker had narrowed this half to *a store with no connection, no
interactive transaction and no cursor for which the membership probe is a whole extra round trip
it cannot fold into anything else*, naming `happenstance-cloudflare` and `happenstance-neon`.
**`happenstance-neon` meets that wording's letter:**
- it probes in one read-only round trip of its own (`crates/happenstance-neon/src/event_store.rs:899`),
  over the `(origin_store, origin_position)` pair VT-8 already indexes, and
  `contains_event_id_reports_membership` passes against a live endpoint (CI run `36638870563`,
  2026-09-29);
- `happenstance-cloudflare` answers in one statement (`crates/happenstance-cloudflare/src/event_store.rs:1066`),
  but that run is on the `node:sqlite` shim, not `workerd`, so the transport evidence rests on Neon.

This record accepts the round trip rather than rewording it away, because no required path pays
it. Ingest deduplicates *inside* the write, against VT-8's uniqueness on the origin pair, and a
probe-then-write would be the race VT-8 forbids (`crates/happenstance-sync/src/ingest.rs:97-101`).
Neither the append path nor the projection runner probes. So the extra round trip falls only on a
caller that asks the question for itself, and no structure beyond VT-8's is needed. ADR-0066
dispositioned this half as answered (`references/adr/0066-what-1-0-promises.md:160`).

**Completeness.** This half is settled by the decision above, not by calling the falsifier
decorative. `contains_event_id` reports whether **this store holds** an event with that
identity. `false` is store-relative in exactly ES-38's sense of `Query::all()`: it does not
distinguish never-held from no-longer-held, and nothing in 1.x asks it to. The tri-state return
is rejected, because changing the return type is a major. A "not retained" answer, if one is
ever wanted, is decision 3's provided method, never a change here.

**One reading the freeze does not fix.** Where ES-10's frontier separates a committed row from a
visible one, whether "holds" means held in the committed log or visible to `read` stays open.
`happenstance-postgres` (`:607`) and `happenstance-neon` answer `true` for a committed row the
frontier has not yet passed, and both record that as unsettled. No rule stages such a row, so
`contains_event_id_reports_membership` passes under either reading. Neither reading can produce a
duplicate through ingest, because ingest does not probe first (`ingest.rs:97-101`). The reading,
and a rule that stages the row, are phase 13's, because phase 13 owns ingest. ES-41 carries this as
non-normative prose, outside its frozen marker.

## PS-22

ADR-0066 declined to freeze PS-22 because "a redaction design could plausibly need to rewind a
checkpoint over rows it keeps". This record removes that reason in two steps:
- **Renumbering is already forbidden.** It is PS-22's named falsifier, and ES-38 `[FROZEN]`
  refuses it. Through the port, every published adapter allocates monotonically and never
  reuses a position. The two ways a log is rewound outside the port are below.
- **Redaction has an answer.** Tag redaction is refused at the port (E2E-49 below), and a
  crypto-shred of `data` moves no position.

So **retention never rewinds a checkpoint over kept rows**, and neither does redaction. Any
rebuild that either forces goes through `reset`, which is atomic with clearing the rows. A
backwards `commit` is never the mechanism.

Two cases are **out of PS-22's scope**, named so they are not mistaken for counter-examples.
Both rewind the log outside the port, and neither is a legitimate backwards commit:
- **A Durable Object wiped by `delete_all()`** restarts `AUTOINCREMENT` under a new `StoreId`, so
  a checkpoint held elsewhere can sit above the new head. That is a checkpoint and `StoreId`
  mismatch, which belongs to VT-6 and the typed runner.
- **A `pg_restore` of an older backup, or a Neon branch,** rewinds the position sequence under the
  **same** `StoreId`. A checkpoint held in a separate projection store then sits above the
  restored head, and no `StoreId` comparison shows it. That is
  `kb-open-question-postgres-neon-store-id-no-restore-001`, owned by phase 13, not VT-6's mismatch
  as it stands.

The remedy in both is `reset`, so PS-22's conclusion survives them.

## Redaction (E2E-49)

- **A tag cannot be redacted through the port.** `Tag` equality is byte equality, every index is
  keyed on it, and there is no store-side update path.
- **A crypto-shred of `data` is the application's concern.** It changes no position, no tag and
  no identity.

The residual E2E-49 names, a subject reconstructed from surviving tags, is real, and it is the
application's to design around: do not put a singling-out identifier in a tag. The port states
the limit rather than hiding it.

## SY-32

The scalar `PeerLimits::retention_floor` (`crates/happenstance-sync/src/peer.rs:296-301`) is
kept, and it means **the lowest resume point this peer can still satisfy**. A resume token is a
prefix concept, so a floor is the right shape for it. ES-39's objection to floors is about
completeness, not resumability. A scattered purge on a peer is a deleted-from store under ES-38
and ES-40, and SY-32 does not report it in 1.x. The offline-longer-than-the-window detection stays.
Nothing published changes: `happenstance-sync` is `publish = false`, and `PeerLimits` is
`#[non_exhaustive]`, so a later retained-set field is additive.

## CF-27

**The instrument reports what it withholds through its own inherent API, never through an
`EventStore` method.** The instrument is a testkit decorator over any `EventStore`.

**It is widened from "a suffix" to "an arbitrary retained set".** A suffix-only instrument cannot
falsify the floor this record rejects.

Its rule, `suffix_store_is_distinguishable_from_a_young_store`, is re-specified for the refusal
and renamed `instrument_report_is_accurate_and_the_suite_cannot_tell`, because under the refusal
the old name says the opposite of what it asserts. It asserts two things:
- the full suite passes against the instrument, so that indistinguishability at the port is the
  specified outcome, with the recorded pass list as the evidence;
- the instrument's own report is accurate.

**The capability, for sub-question 3.** Phase 14 adds a `Fixture` capability, defaulted to
declined on the `MID_BATCH_FAULT` precedent, through which a fixture removes events outside the
port: a raw `DELETE` on SQLite, Cloudflare, Postgres and Neon. The name is phase 14's. With it,
ES-38's `positions_are_not_reused_after_removal` runs against the real adapters and not only
against the decorator. Its named wrong implementation is a SQLite table declared
`INTEGER PRIMARY KEY` without `AUTOINCREMENT`, which hands out the deleted tail's highest rowid
again. That answers
`kb-open-question-es-38-and-gap-read-unowned-001`'s sub-question 3: the instrument decision is
taken here, and phase 14 builds rather than decides. A fixture instrument gives falsifiability,
not implementability (CF-26), so the completeness axis's residual stays open.

## Left open, deliberately

- **Product scope.** 1.x offers no erasure at the port, and an adopter erases outside it under
  ES-38. Whether scattered regulated erasure becomes a promise is a 2.0 question or a provided-method
  question, and this record answers neither.
- **Whether a wiped Durable Object is the same store** for ES-38 or a new one (VT-6). Phase 14's
  capability must say which in its decline reason.

## Falsifier

This record is wrong if either of the following happens:
- Phase 14's instrument shows that a runner or ingest path cannot be written correctly without a
  report, **and** that no provided method defaulting to `Unknown` would serve, so that the report
  must be required.
- A published adapter must renumber, reuse a position, or rewind a checkpoint over kept rows to
  honour a retention policy.

Either one reopens decision 1, and neither is expected before 2.0.
