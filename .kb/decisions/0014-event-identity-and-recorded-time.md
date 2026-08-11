---
id: kb-decision-0014
title: The store mints identity, records a time, and the caller supplies neither
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0014
reversibility: medium
phase: 4
supersedes: null
superseded_by: null
summary: >-
  Three types land in happenstance-core, replacing phase-2 placeholders in happenstance-sync:
  StoreId as sixteen opaque bytes (not u128, to avoid an endianness choice and keep Ord agreeing
  with byte-collation storage order), EventId as a store and a position behind private fields
  and accessors (a transparent pair would let a caller assemble an identity from the wrong
  store's identifier with no compiler diagnostic), and RecordedAt as a newtype over signed i64
  milliseconds (unsigned turns timestamp subtraction into wrapping arithmetic; a type alias
  would foreclose impl From<RecordedAt> for SystemTime under the orphan rule). Position, event
  id and recorded time are all assigned by the store at append; the store id is assigned by the
  deployment. An adapter may mint a store id once and re-mint only if it can detect a restore or
  clone, or if the deployment documents when re-minting is invoked; otherwise it must mint a
  fresh incarnation on every open. happenstance-core ships no RecordedAt::now(), because a std
  feature is a compile-time choice resolved per target rather than per capability, and the
  target the clock-availability question is about is exactly the one that might compile std
  in without having a clock; no rule may compare two recorded times or compare one against a
  position. A new required port method, contains_event_id, ships provisional: required rather
  than provided because the only provided form needs Self: Sync, which the workspace's
  deliberately !Sync bare-flavour adapter cannot satisfy; by value rather than by reference,
  because the type is two Copy scalars with nothing to save by borrowing; returning bool rather
  than Option<SequencePosition>, because widening later is additive and narrowing later is
  breaking. SequencedEvent::new is superseded rather than widened, arity two to four with no
  deprecated arm, because nothing is published yet and a compatibility shim would have to
  invent a StoreId and a time. Event::into_parts returns a non-exhaustive struct instead of a
  positional tuple, so a future fifth part is additive rather than breaking. append's signature
  and return type are unchanged: identity does not travel through append, and EventStore gains
  no synchronous store_id() accessor, because that would require every adapter to hold the
  value before the call, which a store with no held connection and one round trip per operation
  cannot do.
depends_on:
  - kb-decision-0008
  - kb-decision-0009
related:
  - kb-decision-0012
  - kb-decision-0013
source_paths:
  - .kb/_intake/0014-event-identity-and-recorded-time.md
  - docs/adr/0014-event-identity-and-recorded-time.md
  - crates/happenstance-core/src/identity.rs
  - crates/happenstance-core/src/event.rs
  - crates/happenstance-sync/src/identity.rs
  - references/adapter-shapes.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# The store mints identity, records a time, and the caller supplies neither

## Decision

Three new types land in `happenstance-core` and delete the phase-2 placeholders that lived in
`happenstance-sync` for the sole purpose of letting an early sketch compile. `StoreId` is
`[u8; 16]`, chosen over `u128` on three grounds that are not about size: a `u128` has an
endianness every adapter must pick, and two adapters that pick differently disagree about
identity while both pass every in-process rule; `Ord` on a byte array is lexicographic over the
exact bytes an adapter stores, agreeing with a database's own `BLOB` collation, where `Ord` on
an integer is numeric and agrees only under one endianness; and an integer invites arithmetic
on a value that is opaque by design. `EventId` is `(StoreId, SequencePosition)` with both
fields private behind `store()`/`position()` accessors — the frozen clause that mandates a
newtype over a tuple is satisfied either way, but a transparent pair would let a caller
construct `EventId { store: their_id, position: my_position }` in one line with no word from
the compiler, and the private form closes exactly that hazard. `RecordedAt` is a signed `i64`
of milliseconds, against an existing placeholder's unsigned choice and against the phase body's
own text: unsigned turns subtracting two timestamps into wrapping arithmetic that produces a
plausible large positive number instead of a negative one, which is the first operation anyone
debugging replication lag reaches for; a type alias rather than a newtype would make
`impl From<i64> for SystemTime` an impl of a foreign trait for a foreign type, which Rust's
orphan rule refuses outright, while the newtype makes the impl ordinary.

Who assigns what: the store assigns `SequencePosition`, `EventId` and `RecordedAt`, all three
at `append`; the deployment assigns `StoreId`, never `happenstance-core` itself, because minting
one needs an entropy source that `no_std` targets do not have. `StoreId` is stable across an
adapter's own reopen when the adapter can detect a restore or is told when to re-mint by its
deployment; otherwise it must mint fresh on every open, which makes an undetectable restore
safe by construction rather than by a procedure nobody can enforce.

`EventStore` gains a new required method, `contains_event_id(&self, id: EventId) -> Result<bool,
Self::Error>`, written here because an existing frozen clause names it and forward-references a
section that did not yet contain it. Required rather than provided, because the only provided
implementation would hold `&self` across an `await` and need `Self: Sync` — a bound the
workspace's `!Send` bare-flavour reference adapter, the entire reason that flavour exists,
cannot satisfy. It ships `[PROVISIONAL]` rather than `[FROZEN]`: the method's normative
obligation is fully in force either way, but no adapter instrument exists yet to prove the
cost claim that every conformant store already holds the index this method needs.

## Provisional

Three markers stay open past this decision, each naming its own falsifier and owning phase:
`StoreId` incarnation cost under an evict-and-revive deployment (the Cloudflare Durable Object,
observed at phase 9, measured at phase 13); wall-clock availability at append time on a target
that cannot supply one (same instrument); and whether the ingest seam that accepts foreign
identity belongs on `happenstance-sync` rather than on `EventStore` itself, falsified only if a
store adapter cannot implement both without duplicating its write path.

## Alternatives rejected

A `std`-gated `RecordedAt::now()` was rejected because a Cargo feature resolves per target, not
per capability — shipping it now would ship a convenience on the exact platform the clock
question is unresolved for. `StoreId::random()` in the contract crate was rejected on the same
entropy-source ground `EventId` was chosen over a UUID for in the first place.
