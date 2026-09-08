---
id: kb-open-question-es-18-byte-identical-conformance-001
title: ES-18 says byte-identical; no adapter restores it, and its own rules ask something weaker
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-18 [FROZEN] second sentence reads "A rejected append MUST leave the
  store byte-identical." happenstance-cloudflare's fix for Q-01 (a
  compensating DELETE that could itself throw, leaving PartialBatch rows in
  the log) closed the clause's first sentence but left the second
  unmeetable: origin_position now marks commit, an unswept row survives a
  failed append, and the position counter deliberately does not roll back
  even on a fully successful compensation, because AUTOINCREMENT's
  high-water mark must not be reused (a reused position collides two
  EventIds, breaking VT-8/VT-11). ES-18's own Rule: block only asks the
  weaker, port-level question its conformance rules check
  (append_is_atomic, condition_rejection_leaves_store_unchanged,
  append_is_atomic_under_a_mid_batch_fault): none compare bytes, and
  Fixture exposes no byte-level snapshot. The premise that this conflict
  applies to every counter-assigning store in the workspace is false and
  was corrected in-session: happenstance-sqlite's AUTOINCREMENT high-water
  mark lives in sqlite_sequence under ordinary transaction control, and a
  rollback restores it (proven by a passing test,
  autoincrement_high_water_is_restored_by_a_rollback). The conflict is
  specific to a store whose atomicity comes from compensation rather than
  a transaction — happenstance-cloudflare, and only it. Not decided:
  whether ES-18's second sentence is amended to the conformance reading,
  left as commentary-augmented tension, or something narrower like
  "byte-identical except for the position counter." ES-18 is [FROZEN]; any
  change is an ADR, not a line edit.
depends_on: []
related:
  - kb-open-question-es-38-and-gap-read-unowned-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/unswept-rows-and-byte-identity.md
last_reviewed: 2026-09-07
---

# ES-18 says byte-identical; no adapter restores it, and its own rules ask something weaker

## What is true today

ES-18 (`spec/SPECIFICATION.md:3440-3446`, `[FROZEN]`) reads: "Either every
event in the batch lands or none does. A rejected append MUST leave the
store byte-identical." The pre-publication review's `Q-01` found
`happenstance-cloudflare` in a reachable, tested, documented state where a
rejected append left rows of the batch behind:
`CloudflareEventStoreError::PartialBatch`, produced when the compensating
`DELETE` that undoes a failed batch itself throws.

The lane fixed the clause's first sentence rather than weakening it —
`origin_position` is now the commit marker, and `head`, the read plan and
the append guard all filter on its presence, so an unswept row carries no
identity and is invisible through the port even when it physically
remains. Two `wasm32` conformance cases exercise this through the port and
were red before the change.

The second sentence stays unmeetable by construction, for three reasons
that all hold simultaneously:

1. **An unswept row can still survive** a failed append when the discard
   itself throws — `PartialBatch` reports it, but the bytes remain on the
   medium.
2. **The position counter never rolls back**, even when compensation fully
   succeeds. The adapter's own documentation says so: `AUTOINCREMENT` must
   keep its high-water mark across a delete, leaving a permitted gap
   rather than reusable positions — because `EventId` is `(store,
   position)`, and a reused position would wear two identities, breaking
   VT-8 and VT-11 (`[FROZEN]`).
3. **ES-18's own `Rule:` block only asks the weaker question.** Its named
   conformance rules describe the store as holding "all of the batch or
   none of it" *through the port*, and nothing in the testkit's `Fixture`
   surface (`connect`, `reopen`, `arm_mid_batch_fault`) takes a byte-level
   snapshot of the medium. ES-22's `snapshot_of` compares what the port
   reports, not the storage bytes.

## The premise correction

An earlier draft of this reasoning claimed the byte-identity/position-
uniqueness conflict holds "for any store that assigns positions from a
monotone counter, which is every store in this workspace." That is false,
and was refuted in-session against `happenstance-sqlite`: its
`AUTOINCREMENT` high-water mark lives in `sqlite_sequence`, an ordinary
table under ordinary transaction control, and a rollback restores it. A
test (`autoincrement_high_water_is_restored_by_a_rollback`) demonstrates
the counter observed advanced inside the transaction and reset by
rollback, with the next committed row landing where it would have without
the aborted batch — no `EventId` can collide, because the rolled-back
positions were never handed to a committed event.
`SqliteEventStore::append_locked` wraps the whole batch and condition
evaluation in one transaction, committed only on success. The conflict is
narrower than first stated: it is specific to a store whose atomicity
comes from compensation rather than from transaction rollback — which in
this workspace is `happenstance-cloudflare` alone.

## The question

Does ES-18's second sentence stay as written, get amended to the weaker
conformance reading ("a rejected append MUST leave the store holding none
of the batch — no event readable, countable, or visible to an append
condition," with commentary naming the position-space and medium-refuse
residues as permitted), get left with contradicting commentary bolted on,
or narrow to something like "byte-identical except for the position
counter" (which is unenforceable by any instrument the testkit currently
has, so it risks restoring the same asserted-but-unchecked shape)?

## Cost of delay

Low today and not compounding: the adapter meets the clause's rules, the
gate is green, and the residue is documented at the site that produces it.
The cost lands on the *next* adapter author who reads a MUST their store
cannot meet and has to decide alone how to read it loosely — which has
already happened, twice, unrecorded until now.

## What this does not settle

Whether the testkit should be able to arm a fault in the compensation path
itself (`Fixture::arm_mid_batch_fault` cannot reach a `DELETE`-side
failure today); whether unswept rows should be swept opportunistically;
and `PartialBatch`'s name, which now reports unswept refuse rather than a
partial batch.
