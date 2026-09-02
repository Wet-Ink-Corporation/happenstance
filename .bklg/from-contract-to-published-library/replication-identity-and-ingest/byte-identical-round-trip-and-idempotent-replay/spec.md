---
item: HS-S0107
stage: spec
created: 2026-08-12T13:47:47.239Z
updated: 2026-08-12T13:47:47.239Z
template_sig: 87bbf1d0
rendered_sig: 2133402e
---

# Spec — A payload survives the boundary unchanged, and replay changes nothing

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — AC-13, DoD 14, and the *Out of scope* list this sits inside |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG edge `publication → replication → retention`, and the warranted-brief table that leaves `ux`/`deployment` blank here |
| Project | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` — AC-006, DR-5, DR-7, DoD 4 |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/byte-identical-round-trip-and-idempotent-replay/spec.md` |
| Key briefs | `.../replication-identity-and-ingest/_decomposition.md` — architecture brief §7 (*The wire mounts inside `Envelope<T>`*), §3 (the mutant-registry shape), *Tension 5* (ADR-0003 cannot lift by edit), *Data flow, one pass end to end*; testing brief *Integration — AC-006's round trip* |
| Signed-off design | `.../replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved 2026-08-12. This story renders nothing; the API-surface obligation is discharged in the briefs, and `design.capture` stays a declared skip |
| Story map row | `.../replication-identity-and-ingest/_storymap.md` — slice `wire-message-set-and-round-trip`, row 2 |
| Discover stage | `.../byte-identical-round-trip-and-idempotent-replay/discover.md` — the signal ledger, the three deferred questions this spec settles, and the three named wrong implementations |
| Roadmap pointer | `RUNBOOK.md:4516-4620` (phase 13 in full); `RUNBOOK.md:4597-4602` is the proof artefact this story is half of |

## One-line PR slice

Assert the payload `Bytes` byte-identical at the receiver across a real store
boundary and replaying the same batch twice an observable no-op, with the
negative control that would fail if any assertion on any path touched
`Event::data` or `Event::metadata`.

## Executive summary

The story before this one (`message-set-on-the-envelope`, HS-S0106) put a message
set on `Envelope<T>` and made a batch travel. **This PR is the measurement that
turns ADR-0003's central claim into evidence**: it lands two conformance rules in
`happenstance-sync-testkit` — one asserting that a payload appended to one store
comes back out of a *second* store byte-identical, one asserting that redelivering
the same batch changes nothing the receiver holds — plus a conformant-variant peer
whose payloads are valid in no codec, which is what makes any payload-decoding
assertion anywhere in the suite fail loudly instead of quietly certifying a peer
that decodes.

The delta against what already exists is narrow and specific. `MemorySyncPeer` is
real (`crates/happenstance-sync/src/memory.rs:1-10`), `IngestStore` has real bodies
and a typed `Ingested { appended, skipped, last_local }` witness
(`crates/happenstance-sync/src/ingest.rs:178-191`) as of slice 2, and the rule
registry and its gate mounts exist as of slice 3. What does **not** exist is any
assertion that the bytes survived — and the repository already holds a measured
example of a codec that is wrong in a way every ordinary round-trip test calls
correct (WF-11's inverted `is_human_readable` branch,
`spec/SPECIFICATION.md:2315-2322`). This PR closes exactly that gap and produces
the citable evidence line that `adr-0003-provisional-lift` (HS-S0108) consumes.
It does **not** lift the marker: an accepted decision atom is immutable, and the
lift is a new atom authored through kb-ingest one story later.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted
anchor; nothing below needs another file to be actionable.

**1. Byte-identical is measured on the way *out* of the receiving store, not on
the way in.** `discover.md` deferred this and it is load-bearing: comparing the
`Bytes` handed to `IngestStore::ingest` proves only that the transport did not
mangle them. The assertion is against what `EventStore::read` returns from the
**receiving** store after the ingest committed, so the store's own persistence
path is inside the loop. For two `MemoryEventStore`s the two spellings are nearly
the same object, which is precisely why the choice is made here rather than
discovered when a SQLite adapter round-trips through a `BLOB` column.

**2. The two assertions land as suite rules under names existing `[FROZEN]`
clauses already claim — never under new names.** `spec-trace`'s check 6 requires
every rule in `RULE_FILES` to be claimed by a clause or retired by one
(`xtask/src/spec_trace.rs:756`, `:94-98`), and this story is forbidden from
editing `spec/SPECIFICATION.md` — the `(new)` marker sweep and the clause
arithmetic belong to `frozen-clause-repairs` and
`clause-arithmetic-and-deferral-renewals`. So:

- the byte-identity rule is **`the_sync_suite_never_decodes`** (SY-35,
  `spec/SPECIFICATION.md:6868-6881`), whose clause text already describes exactly
  this instrument — a fixture domain of payloads "not decodable in any codec"
  against which every rule must still pass. A rule that asserts the bytes survived
  *while being undecodable* is an assertion that cannot be written by decoding;
- the replay rule is **`redelivery_of_an_accepted_group_is_a_no_op`** (SY-11,
  index row at `spec/SPECIFICATION.md:8670-8672`, cases E2E-33/E2E-34/E2E-36).

Both are `†` (do not exist yet) in the clause index today. Inventing a third name
such as `payload_bytes_survive_the_round_trip` fails the gate on a rule no clause
owns; if the implementer concludes a third rule is genuinely needed, that is a
clause change and therefore a **blocker to raise**, not a `SPECIFICATION.md` edit.

**3. The payload fixture is bytes valid in no codec, and this is not decoration.**
SY-12's rule text already prescribes the technique — "payloads and metadata that
are not valid UTF-8 and not valid in any codec; a conformant peer dedupes anyway"
(`spec/SPECIFICATION.md:6153-6161`). High bytes, embedded nulls, invalid UTF-8
sequences, in **both** `Event::data` and `Event::metadata`. This is what makes the
byte-identity assertion unfakeable: a peer or a suite that reached for a decoder
cannot even construct the comparison.

**4. Comparing `Bytes` is allowed; parsing them is forbidden, everywhere, on every
path.** `bytes::Bytes` compares as a byte slice, so `assert_eq!(sent.data,
received.data)` *is* the byte assertion DR-5 wants. What DR-5 forbids is
`str::from_utf8`, `serde_json::from_slice`, a domain decode, or any `Display`
rendering of a payload — including inside a failure message, which is the spelling
that looks most reasonable in review (`project.md`, DR-5;
`RUNBOOK.md:4576-4580`).

**5. The replay no-op is observed over the *set of `EventId`s the receiver holds*,
never over a count.** `discover.md` names the peer that defeats a count:
`BalancedReplayPeer` appends one duplicate and drops one unrelated event from the
same batch, leaving the length identical and `Ingested { appended: 1, skipped: n-1 }`
looking like a healthy overlap. SY-11's own named target is sharper still — an
*inversion*, where redelivery causes the receiver to supersede the event it
previously accepted, leaving count and identities right and the decision wrong
("A duplicate is visible; an inversion looks like a decision",
`spec/SPECIFICATION.md:6144-6152`). The rule therefore asserts three things
together: the identity set is unchanged, the typed witness reads
`appended == 0` with a `skipped` equal to the batch's event count, and **no
compensation event was authored**. The typed witness alone is not sufficient and
is not permitted to stand alone.

**6. The negative control is a *conformant variant* (CF-5), not a mutant.** It is
legal, it declares `fails: &[]`, and it must pass every rule in the suite —
exactly the shape `GappedPositionStore` has for CF-6
(`crates/happenstance-testkit/tests/mutation_coverage.rs:326-338`;
`spec/SPECIFICATION.md:7222-7232`). Its provenance string must name a real peer
shape that makes it plausible and must never be empty (CF-4). Registering it as a
mutant with declared failures is the wrong instrument: a mutant proves a rule can
fail, and what this control proves is that a *correct* peer trips any rule that
overreaches into the payload.

**7. No literal positions, and no comparison of two peers' positions at all.**
CF-6/DR-7: every position read is anchored on a value the store under test
assigned (`spec/SPECIFICATION.md:7233-7249`). Both stores here are dense from one,
so `assert_eq!(head, SequencePosition::new(3))` would pass and would convert a
`MAY` into a `MUST`. Separately, SY-19 is explicit that **no expression relates
two peers' position numbers** (`spec/SPECIFICATION.md:6409-6428`): the origin's
position travels only inside the `EventId` and is never asserted as a number. What
crosses the boundary and is asserted is the payload and the identity; the local
position is arrival order and is only ever compared to itself before and after.

**8. This story leaves `.kb/decisions/0003-opaque-payloads.md` untouched.** It is
`status: accepted` and therefore immutable — `redkiln validate --kb` checks each
accepted atom against `HEAD` (`CLAUDE.md`, *Where the work lives*), and DoD 5 is
that gate. The atom's own Status section names this round trip as its lift
condition; the lift itself is a **new** atom through `/redkiln:kb-ingest` in
HS-S0108 (`_decomposition.md`, *Tension 5*, which cites ADR-0029 amending ADR-0004
as the in-tree precedent). What this story owes HS-S0108 is a precise, citable
evidence line — a rule name, a test path and a commit — and nothing else.

**9. If the round trip cannot be made byte-identical, the deliverable is the
recorded reason, not a weaker assertion.** AC-006's second arm
(`project.md:209-213`). Downgrading to a decoded-value comparison to make the
story close is the single named failure mode of this work (`discover.md`, *The
wrong implementation*: `DecodedValueRoundTrip`), because it satisfies AC-006's
wording while proving nothing.

**10. The persona-journey slice.** *Learn when you are finished* — the adapter
author's loop, from a signature that type-checks to a suite that says pass or fail
and names why
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`).
The concrete moment this story serves: an adapter author ships an event whose
payload is a binary codec their peer's crate has never heard of, replication runs,
a socket drops mid-batch and the batch is redelivered — and they find out from the
suite, not from production, whether their peer re-encoded the payload or authored
a compensation against an event it had already accepted. The initiative's own
framing is AC-13: *nobody has to guess what happens at a store boundary*
(`initiative.md:344-346`).

**11. Fixture shape is inherited, not re-decided.** The sync peer fixture is
single-flavour with **no `trait_variant` and no `Send` flavour**, and its
associated types are **owned, never a GAT** — the second is one of five
ingredients of a rustc ICE this workspace already minimised and which still
reproduces on 1.97.1 (`crates/happenstance-testkit/src/contract.rs:76-84`,
`:97-111`). That contract is HS-S0103's; this story consumes it and must not
widen it.

## Integration contract

- **Archetype**: `capability` — a slice through the wire, the ingest seam, a real
  store boundary and the suite that observes it.
- **Slice / milestone**: `wire-message-set-and-round-trip`. Slice-mates, delivered
  in one context: `message-set-on-the-envelope` (HS-S0106, merges first),
  **this story**, `adr-0003-provisional-lift` (HS-S0108, merges last because it
  cites evidence that does not exist until this story merges).
- **Mount point**: **`crates/happenstance-sync-testkit/src/registry.rs`** — the
  `for_each_sync_peer_rule!` enumeration created by
  `sync-testkit-crate-and-rule-registry`. Both rules are added to that
  enumeration and to the rules module it enumerates, so they are emitted through
  `sync_peer_conformance!` into the tokio, blocking and `wasm32` harnesses like
  every other rule. A rule function that exists but is absent from the
  enumeration is caught by the `no_orphan_sync_rules` meta-test — the shape at
  `crates/happenstance-testkit/src/registry.rs:412-434` — and is exactly the
  "constructed but unmounted" failure this section exists to prevent.
- **Wires into**:
  - `crates/happenstance-sync/src/ingest.rs` — `IngestStore::ingest` / `holds` /
    `watermark`, and the `Ingested { appended, skipped, last_local }` witness
    (`:178-191`);
  - `crates/happenstance-sync/src/peer.rs` — `PushBatch`, `EventGroup`,
    `Ack { appended, skipped, confirmed }` (`:253-272`);
  - `crates/happenstance-sync/src/identity.rs` — `EventId`, `ReplicatedEvent`
    (`:160-180`: no sender position, and "the payload inside `event` is never
    decoded here");
  - `crates/happenstance-sync/src/memory.rs` — `MemorySyncPeer`, the oracle both
    rules run against first;
  - `crates/happenstance-core/src/memory.rs` — two `MemoryEventStore` instances,
    origin and receiver, plus the additive inherent ingest operation
    `memory-store-ingest-seam` landed;
  - `crates/happenstance-testkit/src/contract.rs` — `Capability` (`:368-433`) and
    `#[must_use] RuleOutcome` (`:471`, `:473-483`), consumed as-is;
  - the message set on `Envelope<T>` from HS-S0106
    (`crates/happenstance-sync/src/wire.rs`), consumed unchanged — this story adds
    no message type and changes no encoding.
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for
  this project and the determination is signed off; there is no surface id to
  claim. The public-item obligation that replaces it is rustdoc on anything this
  story makes `pub` (`standards/rust/70-rustdoc-obligations.md`), and the honest
  answer is that a conformance rule is `pub` only to the harness.
- **Conformance rule(s)**: `the_sync_suite_never_decodes` (SY-35, `[FROZEN]`) and
  `redelivery_of_an_accepted_group_is_a_no_op` (SY-11, `[FROZEN]`). Both are
  adapter-observable and both are `†` in the clause index today
  (`spec/SPECIFICATION.md:8670-8706`).
- **Clause(s)**: discharges SY-11 and SY-35; exercises SY-5, SY-12 and SY-19
  without amending them; supplies the first real-boundary evidence for
  `[PROVISIONAL]` WF-11 without settling it. **No `[FROZEN]` clause is edited**,
  and no clause text is touched at all in this PR.
- **Advances DoD scenario**: initiative **DoD 14** — *replication has an answer on
  disk* (`initiative.md:398-401`) — by producing the evidence the answer rests on;
  the atom itself is HS-S0108's and the `validate --kb` half is
  `open-questions-resolved-and-indexed`'s. Project **DoD 4**, the proof artefact
  (`RUNBOOK.md:4597-4602`), is half-satisfied here: this is the byte-identity
  half, and `durable-object-and-neon-peers` is the two-unlike-peers half.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any
file changed outside it.

```
crates/happenstance-sync-testkit/src/**
crates/happenstance-sync-testkit/tests/**
CHANGELOG.md
.bklg/from-contract-to-published-library/replication-identity-and-ingest/byte-identical-round-trip-and-idempotent-replay/**
```

**In this PR**

- Two rule functions in `happenstance-sync-testkit`'s rules module, added to the
  `for_each_sync_peer_rule!` enumeration in
  `crates/happenstance-sync-testkit/src/registry.rs` (the mount).
- The undecodable-payload fixture domain the two rules build their events from.
- The conformant-variant peer that carries an undecodable payload end to end,
  registered in `crates/happenstance-sync-testkit/tests/` with `fails: &[]` and a
  non-empty provenance (CF-4, CF-5).
- Two registered mutants against `redelivery_of_an_accepted_group_is_a_no_op` —
  the balanced-replay peer and the inversion peer — each with `expect` pins
  naming the exact assertion it should trip (CF-2, CF-3).
- One `CHANGELOG.md` entry per new rule, naming the defect it detects (CF-29,
  `xtask/src/lints.rs:525`).
- This story's own backlog folder: `_ledger.md` and the implementation report.

**Explicitly not in this PR**

- **Any edit to `spec/SPECIFICATION.md`.** Dropping the `(new)` markers is
  `frozen-clause-repairs`'; the clause arithmetic is
  `clause-arithmetic-and-deferral-renewals`'.
- **Any edit to `.kb/decisions/0003-opaque-payloads.md`.** Immutable; the lift is
  HS-S0108's new atom through kb-ingest.
- **Any change to the message set, `Envelope<T>`, `FORMAT_VERSION` or an
  encoding.** HS-S0106 owns the wire; if byte-identity fails because of the
  codec, that is a defect raised against the slice-mate, not repaired here.
- **Any change to `EventStore`'s trait signature.** The write-path seam is
  `memory-store-ingest-seam`'s single additive inherent method, already landed
  (`_decomposition.md`, *Composition root* §5, option (a)).
- **Gate constants** (`RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST`, the
  `wasm32` steps) — `gate-mounts-for-the-sync-suite`'s, already landed in slice 3.
- The live Durable Object and Neon peers (`durable-object-and-neon-peers`).

The implementer **may** also touch the composition-root file named under *Mount
point* to mount these rules; that is not scope drift, and it is inside the
boundary above.

**Merge DoD**: both rules are enumerated in `for_each_sync_peer_rule!` and green
against `MemorySyncPeer`; the conformant variant passes every rule; both replay
mutants fail `redelivery_of_an_accepted_group_is_a_no_op` and nothing else;
`cargo xtask affected --base main`, `cargo xtask lints` and `cargo xtask spec-trace`
are green; and `_ledger.md` cites a real path per AC-###.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **A payload appended to store A is byte-identical when read out of store B** | Origin `MemoryEventStore` appends events whose `data` and `metadata` are valid in no codec. `MemorySyncPeer` carries the `PushBatch`; the receiver ingests; the assertion reads the events back out of the **receiving store** via `EventStore::read` and compares `Bytes` to `Bytes` for both `data` and `metadata`. No decode, no `from_utf8`, no rendering — including in failure messages. | `spec/SPECIFICATION.md:6868-6881` (SY-35); `crates/happenstance-sync/src/identity.rs:160-180`; `project.md` DR-5 |
| **Identity crosses the boundary; position does not** | The receiver stores the foreign `EventId` and assigns its own local position at its own tail. The rule asserts identity equality and **never** relates the two stores' position numbers. | `spec/SPECIFICATION.md:6409-6428` (SY-19); `:5954-5970` (SY-5); `_decomposition.md` *Data flow* |
| **Redelivering an accepted group changes nothing the receiver holds** | Second push of the identical `PushBatch`. Assert (a) the set of `EventId`s the receiver holds is unchanged, (b) `Ingested { appended: 0, skipped: n }` where `n` is the batch's event count, (c) no compensation event was authored. All three, because each alone is defeatable. | `spec/SPECIFICATION.md:8670-8672` (SY-11 index row), `:6144-6152` (the inversion); `crates/happenstance-sync/src/ingest.rs:178-191` |
| **The receiver's head is compared only to itself** | Head position read from the receiving store before and after the replay, compared to each other. No literal `SequencePosition`, anywhere, in either rule. | `spec/SPECIFICATION.md:7233-7249` (CF-6); `crates/happenstance-testkit/tests/mutation_coverage.rs:326-338` |
| **A conformant peer carrying an undecodable payload passes every rule** | The negative control: registered with `kind: ConformantVariant`, `fails: &[]`, non-empty provenance naming a real peer shape (a peer forwarding a codec its crate has never heard of). Any rule anywhere in the suite that decodes a payload fails against it, locally and by name. | `spec/SPECIFICATION.md:7222-7232` (CF-5), `:7209-7221` (CF-4); `crates/happenstance-testkit/tests/mutation_coverage.rs:140-175` |
| **Both replay mutants fail exactly the rule they declare** | `BalancedReplayPeer` (one duplicate appended, one unrelated event dropped — count-preserving) and the inversion peer (redelivery supersedes the previously accepted event). Each declares `redelivery_of_an_accepted_group_is_a_no_op` and nothing else, with an `expect` pin naming the assertion it trips. | `spec/SPECIFICATION.md:7195-7208` (CF-2/CF-3); `discover.md` *The wrong implementation* |
| **The rules are mounted, not merely written** | Present in `for_each_sync_peer_rule!`, therefore emitted into the tokio, blocking and `wasm32` harnesses; the `no_orphan_sync_rules` meta-test fails if a rule function exists outside the enumeration or vice versa. | `crates/happenstance-testkit/src/registry.rs:94-102`, `:412-434`; `crates/happenstance-testkit/src/lib.rs:315-342` |
| **Every rule is claimed by a clause** | Both rule names are already cited by `[FROZEN]` clauses, so `spec-trace` check 6 resolves with no specification edit. A third rule name would be an unclaimed rule and a gate failure — raise it as a blocker instead. | `xtask/src/spec_trace.rs:85-89`, `:756`; `spec/SPECIFICATION.md:8670-8706` |
| **No clock, and no time-dependent assertion** | CF-33's scope covers `happenstance-sync-testkit/src` once `gate-mounts-for-the-sync-suite` taught the lint about it; `recorded_at` travels as a value and is compared as a value, never read from a clock inside a rule. | `xtask/src/lints.rs:33-45`; `spec/SPECIFICATION.md` CF-33 |
| **One changelog entry per rule, naming a defect** | CF-29's lint reads `CHANGELOG.md` and a new suite starts at zero. Each entry names what the rule detects — an inversion on redelivery, a payload that was re-encoded — not that a rule was added. | `xtask/src/lints.rs:525`; `xtask/src/main.rs:394-405` |
| **The evidence line HS-S0108 will cite** | The `_ledger.md` row for the byte-identity AC states rule name, test path and commit in a form `adr-0003-provisional-lift` can quote verbatim into a `.kb/_intake/` document without re-deriving it. | `.kb/decisions/0003-opaque-payloads.md` (Status/summary, lift condition); `references/adr/0003-opaque-payloads.md:14-15` |
| **The escape hatch is a recorded reason, never a weaker assertion** | If byte-identity cannot be achieved, this story's output is a written finding naming the hop that mutated the bytes, routed to HS-S0106 or raised as a blocker. AC-006's second arm, not a downgrade to a decoded-value comparison. | `project.md:209-213`; `discover.md` *The wrong implementation* |

## Data and migrations

**N/A — no persistent schema, no migration, no stored format change.**

Three reasons, each specific rather than generic:

1. **Both stores in this story are `MemoryEventStore`.** Nothing is written to
   disk, so there is no schema to version and no data to migrate. The durable
   adapters are consumed as peers by `durable-object-and-neon-peers`, not built
   here.
2. **The wire format is not changed by this PR.** `FORMAT_VERSION`, the envelope
   and the message set are HS-S0106's and travel unchanged; the derives that
   constitute a wire-format decision were authorised by name in ADR-0027
   (`_decomposition.md`, architecture brief §7). This story adds no message type,
   so there is no format version to bump — and WF-9 is explicit that the version
   moves on a **shape** change (`crates/happenstance-sync/src/wire.rs:46-65`).
3. **The one data-shaped artefact is a test fixture domain**, not persisted
   state: byte strings that are valid in no codec, constructed in the rules and
   discarded with the fixture. It is deliberately not a golden file — a
   checked-in binary blob would make the assertion about a file's contents rather
   than about what crossed the boundary.

## Acceptance criteria

Framed from the persona goal that crosses the whole stack: **the adapter author's
"learn when you are finished" loop**
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`,
*Persona 2 — The adapter author*, journey step 3 — a green run today is "evidence
about one storage shape wearing four hats"). Each criterion is a moment in that
loop, not a capability in isolation.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an adapter author ships an event whose `data` and `metadata` are a binary codec no crate in the receiving process has ever heard of — bytes that are not valid UTF-8, carrying high bytes and embedded nulls — **WHEN** that event is appended to an origin `MemoryEventStore`, carried across the boundary by `MemorySyncPeer` and ingested into a second, separate `MemoryEventStore`, **THEN** reading the event back **out of the receiving store** through `EventStore::read` yields `data` and `metadata` `Bytes` equal to the bytes appended at the origin — and the author learns this from the suite rather than from a corrupted production log. Measured on the way *out* of the receiver, never on the `Bytes` handed to `IngestStore::ingest` (Context pack §1). | Rule `the_sync_suite_never_decodes` in `happenstance-sync-testkit`'s rules module, emitted through `sync_peer_conformance!` from `crates/happenstance-sync-testkit/src/registry.rs`; run in the tokio, blocking and `wasm32` harnesses via `cargo test -p happenstance-sync-testkit --all-features` and `cargo xtask wasm`. Clause SY-35 (`spec/SPECIFICATION.md:6868-6881`). |
| **AC-002** | **GIVEN** the same adapter author's replication run drops a socket mid-batch and the runner redelivers the identical `PushBatch`, **WHEN** the receiver ingests it a second time, **THEN** nothing the receiver holds changes: the **set of `EventId`s** in the receiving store is identical before and after, the typed witness reads `Ingested { appended: 0, skipped: n }` for the batch's `n` events, **and** no compensation event was authored — so the author can tell a healthy overlap from a peer that quietly superseded an event it had already accepted. All three observations together; none may stand alone (Context pack §5). | Rule `redelivery_of_an_accepted_group_is_a_no_op`, same mount, same three harnesses. Clause SY-11 (`spec/SPECIFICATION.md:6144-6152`; index row `:8670-8672`; cases E2E-33/E2E-34/E2E-36). Receiver head read from the receiving store before and after and compared to itself — no literal `SequencePosition`, and no expression relating the two stores' positions (SY-19, `spec/SPECIFICATION.md:6409-6428`). |
| **AC-003** | **GIVEN** an adapter author whose peer is entirely correct but forwards a payload in a codec their crate cannot parse, **WHEN** they run the whole sync suite against it, **THEN** every rule passes — and any assertion anywhere in the suite that reached for a decoder fails immediately, locally and by name, instead of silently certifying a peer that decodes. Every failure message this story authors names the rule, the peer and the assertion and **renders no payload byte** — not through `Display`, not through `{:?}` over the payload, not through `from_utf8` "for a readable message", which is the spelling that looks most reasonable in review (DR-5, `project.md:162-164`). | A `ConformantVariant` entry in `crates/happenstance-sync-testkit/tests/` declaring `fails: &[]` and a non-empty provenance naming a real peer shape (CF-4/CF-5, `spec/SPECIFICATION.md:7209-7232`), in the `Declared`-table shape at `crates/happenstance-testkit/tests/mutation_coverage.rs:140-175`. Diff reviewed for `str::from_utf8`, `serde_json::from_slice` or any domain decode on a suite path: none. |
| **AC-004** | **GIVEN** the author wants a green replay rule to mean something, **WHEN** two deliberately wrong peers run against the suite — `BalancedReplayPeer` (appends one duplicate *and* drops one unrelated event from the same batch, so the count is preserved and `Ingested { appended: 1, skipped: n-1 }` reads like a healthy overlap) and the inversion peer (redelivery supersedes the previously accepted event, leaving count and identities right and the decision wrong) — **THEN** each fails `redelivery_of_an_accepted_group_is_a_no_op` and **passes every rule it does not declare**, with an `expect` pin naming the exact assertion it trips. | Two registered mutants in `crates/happenstance-sync-testkit/tests/`, CF-2/CF-3 shape (`spec/SPECIFICATION.md:7195-7208`). Neither may reuse `MemorySyncPeer`'s own dispatch to fake its bug (`standards/rust/60-what-a-test-must-prove.md`, RS-60-4). Run by `cargo test -p happenstance-sync-testkit --all-features`. |
| **AC-005** | **GIVEN** the author runs the suite on the constrained runtime they actually ship to, **WHEN** the harnesses are built, **THEN** both rules are *mounted, not merely written*: present in `for_each_sync_peer_rule!` and therefore emitted into the tokio, blocking and `wasm32` harnesses, no rule vanishing from any binary, and no rule existing outside the enumeration or the enumeration naming a rule that does not exist. Both names are already claimed by `[FROZEN]` clauses, so `spec-trace`'s check 6 resolves **with no edit to `spec/SPECIFICATION.md`**. | The `no_orphan_sync_rules` meta-test (shape at `crates/happenstance-testkit/src/registry.rs:412-434`); `cargo xtask spec-trace` check 6 (`xtask/src/spec_trace.rs:85-89`, `:756`); `cargo xtask wasm`; `git diff --stat spec/SPECIFICATION.md` empty. |
| **AC-006** | **GIVEN** `adr-0003-provisional-lift` (HS-S0108) must *cite* this measurement rather than re-derive it, **WHEN** this story merges, **THEN** the evidence exists in citable form — `_ledger.md`'s AC-001 row states the rule name, the real test path and the commit, and `CHANGELOG.md` carries one entry per new rule naming the **defect it detects** (a payload that was re-encoded; an inversion on redelivery) rather than that a rule was added. **AND** if byte-identity cannot be achieved, the deliverable is a written finding naming the hop that mutated the bytes, routed to HS-S0106 or raised as a blocker — never a decoded-value comparison that satisfies the wording while proving nothing. | `cargo xtask lints` (CF-29's `changelog_names_every_rule`, `xtask/src/lints.rs:525`); `redkiln verify --grain story` reading `_ledger.md` (`.redkiln/config.yaml:62-67`, `require_ledger: true`); the ledger's AC-001 row quotable verbatim into HS-S0108's `.kb/_intake/` document. |

**Coverage of the traced project AC.** Project **AC-006** (`project.md:209-213`)
has three arms. AC-001 discharges *byte-identical end to end across a store
boundary*; AC-002 discharges *replaying the same batch twice is observably a
no-op*; AC-006 above discharges the evidence-or-recorded-reason arm HS-S0108
consumes. AC-003, AC-004 and AC-005 are the instrument quality that makes the
first two measurements rather than claims — project DR-5, DR-4/CF-1–CF-4 and
project AC-003 respectively.

## Interaction quality

RFC §6.7/D6. This story **renders no surface**: `_design.md` records
`N/A — no user-facing surface` for the whole project and the determination is
signed off (Ryan Britton, 2026-08-12), with `design.capture` a declared skip
(`.redkiln/config.yaml:75-82`). So the *composition* family — placement,
transience, density budget with real numbers, hierarchy, the design's named
screen anti-patterns — has **nothing to bind to here**, and inventing numbers
would be this story re-deciding what a human already approved as absent.

What survives the translation is the family's underlying question: **what does
the person on the other end actually meet?** For a conformance suite that is the
run output and the failure message — which `_design.md` explicitly hands to the
API-surface obligation (*Public API surface note*). Each surviving invariant is
listed below **against the AC-### row that already carries it**, never restated as
a bullet: `redkiln verify` extracts ACs from a leading `| AC-001 |` table cell or
a `- AC-001:` bullet, so an invariant living only in this section gets no ledger
row and is never gated.

| invariant family | translated invariant | carried by | how it is verified |
| --- | --- | --- | --- |
| **State — non-occlusion** | No rule vanishes from a binary. A rule absent from `for_each_sync_peer_rule!` is invisible in a green run — the library analogue of a control that never rendered; and a capability the fixture declines still runs and reports the fixture's stated reason (DR-8; `crates/happenstance-testkit/src/contract.rs:360-375`, where `Capability`'s private field makes an empty reason unconstructible). | **AC-005** | `no_orphan_sync_rules` in both directions; harness output inspected for the declined-reason line. |
| **State — reversibility** | Redelivery *is* the reversibility case: the second delivery must leave the receiver in the state the first left it, observed over held identity rather than an arithmetic that a compensating pair of wrongs satisfies. | **AC-002**, **AC-004** | The `EventId`-set equality assertion, plus the two replay mutants that defeat a count-only assertion. |
| **State — preserved position (the scroll analogue)** | The receiver's head is compared **only to itself**, before and after; the origin's position travels inside the `EventId` and is never asserted as a number. Arrival order is the receiver's own frame and must not be relitigated against a foreign one (SY-19). | **AC-002** | CF-6 / `cargo xtask lint-position-literals` scoped to the sync suite; both rules reviewed for any literal `SequencePosition`. |
| **State — in-place, not a context jump** | Ingest appends at the receiver's own tail and changes nothing already accepted; a redelivery that supersedes an accepted event is the context jump this suite exists to catch, and it "looks like a decision" rather than an error (`spec/SPECIFICATION.md:6144-6152`). | **AC-002**, **AC-004** | The no-compensation observation plus the inversion mutant's `expect` pin. |
| **Composition — presentation exists at all** | The suite's only presentation is the failure message, and it must be composed rather than bare: rule, peer, assertion — and **no payload byte**. A message that dumps the payload to be helpful is exactly the DR-5 violation that survives review. | **AC-003**, **AC-004** | The conformant variant fails any decoding path locally and by name; the mutants' `expect` pins assert the message identifies the assertion. |
| **Composition — the named anti-pattern** | `_design.md` names no screen anti-pattern; the anti-pattern inherited from `discover.md` is `DecodedValueRoundTrip` — an assertion that reads well, satisfies the project AC's wording, and never looked at a byte. | **AC-001**, **AC-003**, **AC-006** | The undecodable fixture domain makes the decoded comparison unwritable; the conformant variant makes it fail if written anyway; AC-006's recorded-reason arm makes "say why" the sanctioned exit instead of the downgrade. |
| **Density budget** | N/A — no rendered surface, therefore no density budget. The nearest real numbers this story is held to are **three harnesses** (tokio, blocking, `wasm32`) and **two rules**, both stated in AC-005. | **AC-005** | `cargo test -p happenstance-sync-testkit --all-features` plus `cargo xtask wasm`. |
| **Keyboard reachability** | N/A — no interactive surface. Its nearest analogue, "reachable without special privilege", is already true: both rules run on the always-on in-process leg with no network, no credentials and no live infrastructure. | **AC-001**, **AC-002** | The `MemorySyncPeer` leg runs on every `cargo xtask affected` pass touching the sync crates. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The round trip is **not** byte-identical — the payload read out of the receiving store differs from the payload appended at the origin. | Stop. The deliverable becomes a written finding naming the hop that mutated the bytes (codec, envelope, ingest path, store), routed to `message-set-on-the-envelope` (HS-S0106) as a defect or raised as a blocker. **Never** downgrade to a decoded-value or `String` comparison to make the story close — that is project AC-006's second arm (`project.md:209-213`) and the named wrong implementation (`discover.md`, *The wrong implementation*). |
| **EC-002** | The implementer concludes a **third** rule name is genuinely needed (for example `payload_bytes_survive_the_round_trip`). | Raise a blocker. A rule no clause claims fails `spec-trace` check 6 (`xtask/src/spec_trace.rs:756`), and adding the claim is a `spec/SPECIFICATION.md` edit this story is forbidden from making — the `(new)` sweep is `frozen-clause-repairs`' and the arithmetic is `clause-arithmetic-and-deferral-renewals`'. |
| **EC-003** | The typed witness and the identity-set observation **disagree** — for example `Ingested { appended: 0, .. }` while the receiver's `EventId` set changed. | The identity set is authoritative; the witness is the defect. Fail the rule with a message naming both observations and raise the discrepancy against `crates/happenstance-sync/src/ingest.rs`. Do not relax the rule to whichever observation is green. |
| **EC-004** | Ingest **rejects** the redelivered group — returns an error, or adjudicates a compensation against an event it already holds. | That is SY-1/SY-11's named fatal defect: re-evaluating the origin's append condition against the receiver's own accepted copy (`spec/SPECIFICATION.md:6008-6014`, E2E-33). Fail the rule and raise against `IngestStore::ingest`. It is not a fixture problem and must not be worked around inside the rule. |
| **EC-005** | The wire codec **refuses** the undecodable fixture bytes — for example a format requiring valid UTF-8 in a payload field. | EC-001's disposition: a byte-identity failure surfaced at the boundary rather than after it, routed to HS-S0106. Softening the fixture to bytes the codec happens to accept destroys the instrument — SY-12's technique is prescribed, not preferred (`spec/SPECIFICATION.md:6153-6161`). |
| **EC-006** | A registered mutant fails **more** than the rule it declares, or the conformant variant fails **anything**. | CF-3/CF-5 violation. Fix the mutant (one that fails two rules is not discriminating) or the over-reaching rule, in the same change, and say which in the implementation report. A conformant variant that fails a rule is the *rule's* defect by construction — that is the whole point of the control. |
| **EC-007** | `.kb/decisions/0003-opaque-payloads.md` appears in this story's diff. | Revert it. The atom is `status: accepted` and therefore immutable; `redkiln validate --kb` checks each accepted atom against `HEAD`. The lift is a **new** atom through `/redkiln:kb-ingest` in HS-S0108 (`_decomposition.md`, *Tension 5*). |
| **EC-008** | A rule compiles on the host but not on `wasm32`, or is quietly `#[cfg]`-ed out of a harness to make the build pass. | A rule excluded from a harness is EC-002 in disguise: the run is green because nothing ran. Fix the rule so it is target-agnostic, or raise a blocker; never narrow the harness set (AC-005, NF-001). |

## Non-functional

| id | requirement | why it bites here |
| --- | --- | --- |
| **NF-001** | Both rules compile and run on `wasm32-unknown-unknown`. No `std::thread`, no blocking sleep, no tokio-only construct in a rule body. | The sync suite inherits three-harness emission; a quietly host-only rule makes the constrained-runtime persona's green run a lie (`cargo xtask wasm`; CLAUDE.md, *Commands*). |
| **NF-002** | No clock. `recorded_at` travels as a value and is compared as a value; nothing in either rule reads the current time. | CF-33 is scoped by directory constant (`xtask/src/lints.rs:33-45`) and `gate-mounts-for-the-sync-suite` taught it about `happenstance-sync-testkit/src`. A time-dependent rule is flaky in exactly the harness that is hardest to debug. |
| **NF-003** | Both rules are deterministic and bounded: O(events in the batch), no retry loop, no sleep, no randomness in the fixture bytes. | The fixture domain is a fixed byte string chosen for its properties (invalid UTF-8, high bytes, embedded nulls), not generated — a failure must reproduce byte-for-byte from the test name alone. |
| **NF-004** | No new dependency is added to `happenstance-sync-testkit` for this story; in particular no codec crate. | A codec in the dependency graph puts the means of violating DR-5 one `use` away. The fixture is a byte literal and the comparison is `Bytes: PartialEq`; anything more is scope this story does not need and a hazard it does not want. |
| **NF-005** | Every item this story makes `pub` carries rustdoc, and the crate's doctests run through the out-of-package harness a `publish = false` crate needs. | Repository-wide obligation (`standards/rust/70-rustdoc-obligations.md`); RS-62-5 (`standards/rust/62-doctests-and-harnesses.md`) — `cargo test --doc` skips a crate that never publishes, so an unrun doctest looks exactly like a passing one. |
| **NF-006** | The rules must not be written by reading `MemorySyncPeer`'s implementation and restating it; the oracle is spelled in a direction sharing no subroutine with the peer under test. | RS-60-4 (`standards/rust/60-what-a-test-must-prove.md`), repeated for this exact tier by the testing brief (`_decomposition.md`, *Integration*, first bullet). A suite written from the oracle is green by construction. |

## Implementation notes (non-prescriptive)

Observations, not instructions; the implementer may reach the same guarantees
another way and should say so in the report.

- **Order of work that keeps the instrument honest.** Write the undecodable
  fixture domain and the conformant-variant peer *first*, before either rule.
  With the control in place a rule that reaches for a decoder cannot be written
  green even briefly — a stronger discipline than writing the rules and then
  checking nobody decoded.
- **Two stores, not one store twice.** Origin and receiver must be genuinely
  separate `MemoryEventStore` instances with separate `StoreId`s. One store
  playing both roles makes the identity assertion trivially true and the
  byte-identity assertion vacuous, because nothing crossed anything.
- **The `EventId` set is the observation, so build it once.** Collecting the
  receiver's identities into a `BTreeSet<EventId>` before and after and comparing
  the two sets expresses "nothing changed" in one assertion no count-preserving
  mutant satisfies. Comparing lengths, or comparing `Vec`s ordered by position,
  are both weaker in ways `BalancedReplayPeer` was written to exploit — and set
  *containment* in one direction is the hole an inversion slips through, so it
  must be equality.
- **`Ingested` is `#[non_exhaustive]`** (`crates/happenstance-sync/src/ingest.rs:178-191`),
  so read it field by field rather than by exhaustive pattern. Note `last_local`
  is `Option<SequencePosition>`: on a pure replay it should be `None`, a fourth
  free observation the rule may assert but must never substitute for the identity
  set.
- **The changelog entries are read by a lint before they are read by a human.**
  CF-29's `changelog_names_every_rule` (`xtask/src/lints.rs:525`) resolves rule
  names out of `RULE_FILES`; write the entry naming the defect, then run
  `cargo xtask lints` rather than assuming the format matched.
- **Failure messages want the identity, not the payload.** `EventId` plus the rule
  name locates any failure here. If a message feels under-informative without the
  bytes, that feeling is the DR-5 pressure this story exists to resist — record it
  in the report rather than resolving it in code.
- **The fixture bytes deserve a comment saying what makes them undecodable.** A
  future reader who cannot see why the literal is what it is will "tidy" it into
  something valid, which silently retires the instrument.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s testing brief (*The test mix, tier by tier* and
*Merge-gate commands*), in the order a story actually runs them.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo xtask affected --base main` | The story grain `.redkiln/config.yaml:40` wires — fmt, clippy `-D warnings` and tests for the packages this diff maps to plus their dependents, re-derived from git including untracked files. The baseline every AC's evidence sits on. |
| Static | `cargo xtask lints` | CF-29 `changelog_names_every_rule` (`xtask/src/lints.rs:525`): one entry per new rule naming a defect — **AC-006**. CF-33's no-clock scope and CF-6's position-literal scope over the sync suite — **NF-002**, **AC-002**. |
| Static | `cargo xtask spec-trace` | Check 6: every rule in `RULE_FILES` claimed by a clause or retired by one (`xtask/src/spec_trace.rs:85-89`, `:756`). Resolves with **no** `SPECIFICATION.md` edit because SY-11 and SY-35 already name both rules — **AC-005**, and EC-002's tripwire. |
| Static | `git diff --stat spec/SPECIFICATION.md .kb/decisions/0003-opaque-payloads.md` is empty | No `[FROZEN]` clause edited and the accepted atom untouched — **AC-005**, **EC-007**, project DoD 7. |
| Unit | `cargo test -p happenstance-sync-testkit --all-features` — `no_orphan_sync_rules` | Both rules enumerated in `for_each_sync_peer_rule!` in both directions, so neither is written-but-unmounted — **AC-005**. Shape at `crates/happenstance-testkit/src/registry.rs:412-434`. |
| Unit | `cargo test -p happenstance-sync-testkit --all-features` — `crates/happenstance-sync-testkit/tests/` mutant registry | The conformant variant passes every rule with `fails: &[]` and non-empty provenance — **AC-003**; both replay mutants fail `redelivery_of_an_accepted_group_is_a_no_op` and nothing else, with `expect` pins — **AC-004**. CF-1–CF-5 shape at `crates/happenstance-testkit/tests/mutation_coverage.rs:140-175`, `:326-338`. |
| Integration | `cargo test -p happenstance-sync -p happenstance-sync-testkit --all-features` — both rules against `MemorySyncPeer`, tokio and blocking harnesses | Two real stores and one real hop: byte-identity read back out of the **receiving** store — **AC-001**; redelivery as an observable no-op over held identity plus the typed witness plus no compensation — **AC-002**. The testing brief's *Integration — AC-006's round trip*. |
| Integration | `cargo xtask wasm` | The same two rules on the constrained runtime, which is why the suite is emitted three times — **AC-005**, **NF-001**, **EC-008**. |
| Integration | `cargo xtask ci --fast` | This project's ceiling (`.redkiln/config.yaml:55`, `integration_scoped`): fmt, clippy, full tests, the wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build and `cargo package --list`. Project DoD 1. |
| Gate | `redkiln validate --kb && redkiln doctor` | The accepted-atom immutability check that catches an edit to `0003-opaque-payloads.md` before it becomes a merge conflict with the discipline itself — **EC-007** (`_decomposition.md`, testing brief, static tier). |
| Gate | `redkiln verify --grain story` over `_ledger.md` | Every AC-### present, `satisfied: true`, with non-placeholder cited evidence (`.redkiln/config.yaml:62-67`) — including the AC-001 row HS-S0108 quotes: **AC-006**, project DoD 3. |

**Not run here.** The live Durable Object and Neon legs (`_decomposition.md`,
testing brief, *Integration*, bullets 2–3) are `durable-object-and-neon-peers`';
this story's legs are the in-process oracle plus the three harnesses. The whole
gate `cargo xtask ci` is `closeout-and-durable-audience`'s per project DoD 1.

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation inside this PR |
| --- | --- | --- |
| **The assertion is written by decoding, because the failure message reads better.** The single named wrong implementation (`DecodedValueRoundTrip`), green against every correct peer *and* against `InvertedHumanReadable` (`spec/SPECIFICATION.md:2315-2322`). | Medium / severe — it satisfies project AC-006's wording, so the story closes and ADR-0003 loses `provisional` on the strength of a test that never looked at a byte. | The undecodable fixture domain makes the decoded comparison unwritable, and the conformant-variant control (AC-003) makes it fail if written anyway. Both land in this PR by construction, neither is deferred. |
| **The replay no-op is asserted as a count** — the cheapest correct-looking spelling. | Medium / high — `BalancedReplayPeer` and the inversion peer both pass it, and an inversion "looks like a decision" (`spec/SPECIFICATION.md:6144-6152`). | AC-002 requires the `EventId` **set**, the typed witness and the no-compensation observation together; AC-004 lands both mutants that defeat the weaker spelling. |
| **Byte-identity fails because of HS-S0106's codec.** The slice-mate introduces the codec one story earlier; this is where a defect in it first becomes visible. | Medium / medium — same slice, so the feedback loop is short. | EC-001: route the finding to HS-S0106 with the mutating hop named. Explicitly **not** repaired here; the wire is outside this PR's boundary. |
| **The mount is missed: rules written, enumeration not updated.** | Low / high — a green run that runs neither rule is indistinguishable from one that runs both. | AC-005 and `no_orphan_sync_rules`, which fails in both directions. The mount file is named under *Integration contract* and sits inside the PR boundary. |
| **A third rule name is invented and `spec/SPECIFICATION.md` edited to claim it.** | Low / high — breaches the story's hardest boundary and collides with two sibling stories' scope. | EC-002: raise a blocker. The empty `git diff --stat spec/SPECIFICATION.md` is a merge-DoD line, and `spec-trace` catches the unclaimed rule regardless. |
| **`.kb/decisions/0003-opaque-payloads.md` is edited "while we are here".** | Low / high — accepted atoms are immutable and the lift belongs to HS-S0108. | EC-007 plus `redkiln validate --kb` in the gate table; `.kb/**` is absent from the PR boundary block. |
| **Coupling: HS-S0108 cannot cite this work precisely** and re-derives the evidence, possibly wrongly. | Medium / medium | AC-006 makes the citable line (rule name, test path, commit) a ledger obligation rather than a courtesy. |
| **Coupling: `durable-object-and-neon-peers` inherits both rules unchanged.** A rule that quietly assumes a store serialising its writers passes here and fails there. | Medium / medium — both stores here are `MemoryEventStore`, the archetypal agreeing shape. | The rules assert only identity, payload bytes and self-relative head movement. No rule relates the two stores' positions (SY-19) or asserts a literal one (CF-6), which is what keeps them portable to a peer with no cursor. |
| **Coupling: the fixture contract is HS-S0103's** (single-flavour, no `trait_variant`, owned associated types rather than GATs — one of five ingredients of a rustc ICE that still reproduces on 1.97.1). | Low / high — widening it to make a rule convenient reopens a known toolchain dead end. | Context pack §11: the contract is consumed, never widened. If a rule cannot be written inside it, that is a blocker against HS-S0103, not a local edit. |

## Dependencies

**Blocks on**

- **`message-set-on-the-envelope`** (HS-S0106) — supplies the message set on
  `Envelope<T>` and the codec the payload travels in. Without it there is no
  boundary to round-trip across, and the codec whose byte-identity is in question
  is introduced there and caught here. Merges first within the slice.

**Unlocks**

- **`adr-0003-provisional-lift`** (HS-S0108) — consumes this story's evidence line
  (rule name, test path, commit) into a **new** atom through `/redkiln:kb-ingest`;
  merges last in the slice, because it cites evidence that does not exist until
  this story merges.
- **`durable-object-and-neon-peers`** — inherits both rules unchanged and runs them
  against the two structurally unlike peers, completing project DoD 4's proof
  artefact of which this story is the byte-identity half (`RUNBOOK.md:4597-4602`).

**Neither blocks nor is blocked by** the specification-repair stories
(`frozen-clause-repairs`, `clause-arithmetic-and-deferral-renewals`): they own the
clause text this story only cites, and the two diffs must not overlap.

## Anchors (progressive disclosure)

Open these at the moment named, not up front. Everything needed to *start* is in
the Context pack; the depth below is deferred, not optional.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` (SY-35, `:6868-6881`) | Names this instrument already — a fixture domain of payloads "not decodable in any codec" — and therefore fixes the name the byte-identity rule must be written under. A different name fails the gate. | Before writing the byte-identity rule, to take the rule name exactly. | AC-001 |
| `spec/SPECIFICATION.md` (SY-11, `:6144-6152`; index row `:8670-8672`) | Carries the replay clause's `Rejects:` — an **inversion**, not a duplicate — which is why the observation is over held identity rather than a count. | Before writing the replay rule, and again before writing the inversion mutant. | AC-002, AC-004 |
| `spec/SPECIFICATION.md` (SY-12, `:6153-6172`) | Prescribes the fixture technique verbatim: payloads and metadata not valid UTF-8 and valid in no codec, against which a conformant peer dedupes anyway. | Before constructing the payload fixture domain. | AC-001, AC-003 |
| `spec/SPECIFICATION.md` (SY-19, `:6409-6428`; CF-6, `:7233-7249`) | The two rules forbidding any expression relating two peers' positions and any literal position value. Both stores here are dense-from-one, so the wrong assertion is green. | Before writing any assertion that mentions a position. | AC-002 |
| `spec/SPECIFICATION.md` (CF-1–CF-5, `:7195-7232`) | The mutant and conformant-variant contract: `fails` declarations, never-empty provenance, and a mutant failing nothing it did not declare. | Before registering the control and the two mutants. | AC-003, AC-004 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` (`:140-175`, `:326-338`) | The compiled `Declared`-table shape to copy, and `GappedPositionStore` — the in-tree precedent for a conformant variant that refutes an over-specified assertion simply by existing. | While writing the registry entries; copy the shape rather than re-deriving it. | AC-003, AC-004 |
| `crates/happenstance-sync/src/ingest.rs` (`:178-191`) | `Ingested { appended, skipped, last_local }` — the typed witness, `#[non_exhaustive]`, with `last_local` documented as local arrival order unrelated to the `EventId`'s position. | While writing the replay rule's witness assertion. | AC-002 |
| `crates/happenstance-sync/src/identity.rs` (`:160-180`) | `ReplicatedEvent` carries no sender position and states outright that "the payload inside `event` is never decoded here" — the wire type is already shaped for this assertion, so no workaround is needed. | Before building the `PushBatch` the rules send. | AC-001, AC-002 |
| `crates/happenstance-sync/src/peer.rs` | `PushBatch`, `EventGroup` (including `guard: Option<AppendCondition>`) and `Ack { appended, skipped, confirmed }` — the exact types the rules construct and read. | While constructing the batch and reading the acknowledgement. | AC-001, AC-002 |
| `crates/happenstance-sync/src/memory.rs` | `MemorySyncPeer`, the in-process oracle both rules run against first — and, per RS-60-4, the thing the assertions must *not* be written by reading. | When wiring the fixture; skim it, do not mine it for the oracle. | AC-001, AC-002 |
| `crates/happenstance-core/src/memory.rs` | `MemoryEventStore` — the origin and receiver instances and the read-back path the byte assertion measures, plus the additive inherent ingest operation `memory-store-ingest-seam` landed. | When standing up the two stores. | AC-001 |
| `crates/happenstance-testkit/src/registry.rs` (`:94-102`, `:412-434`) | The `for_each_*_rule!` enumeration pattern and the `no_orphan_rules` meta-test in both directions — the shape `no_orphan_sync_rules` copies and the mechanism that catches a written-but-unmounted rule. | Immediately after writing each rule, before assuming it runs. | AC-005 |
| `crates/happenstance-testkit/src/contract.rs` (`:360-433`) | `Capability` with its private field: why a declined capability can never carry an empty reason and why a declined rule still runs. Consumed as-is — the sync fixture contract is HS-S0103's and must not be widened. | If a capability question arises while wiring the fixture. | AC-005 |
| `xtask/src/spec_trace.rs` (`:85-89`, `:756`) | Check 6's implementation — every rule in `RULE_FILES` claimed by a clause or retired by one. The precise reason a third rule name is a blocker rather than a small edit. | Before adding any rule name not already in the Context pack. | AC-005 |
| `xtask/src/lints.rs` (`:33-45`, `:525`) | CF-33's directory-scoped no-clock constant and CF-29's `changelog_names_every_rule`, which resolves rule names out of `RULE_FILES`. | When writing the changelog entries, and if a clock ever looks tempting. | AC-006 |
| `.kb/decisions/0003-opaque-payloads.md` | The accepted, immutable atom whose Status section states the lift condition this measurement satisfies. Read it; do not edit it. | Before writing the `_ledger.md` evidence line HS-S0108 quotes. | AC-006 |
| `references/adr/0003-opaque-payloads.md` | The long-form record behind the atom — rejected alternatives and reasoning a ~100-line atom cannot hold. Cite by `file:line` if the evidence line needs the argument and not just the claim. | Only if HS-S0108's citation needs the argument behind the decision. | AC-006 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` (testing brief, *Integration*; *Fixtures and seams to mock*) | States this as an integration-tier assertion by construction, and carries the payload row verbatim: "Never decoded, never asserted on structurally — only compared as opaque `Bytes` for equality." | Before choosing the tier, and again if any seam starts to look mockable. | AC-001, AC-003 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/byte-identical-round-trip-and-idempotent-replay/discover.md` (*The wrong implementation*) | The three named wrong implementations in full — `DecodedValueRoundTrip`, `PayloadTouchingSuite`, `BalancedReplayPeer` and its inversion sibling — with the reasoning each rests on. | Before writing the control and the two mutants. | AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` (*Persona 2*, journey steps 3–4) | Why a green run against agreeing storage shapes is not evidence — the framing every AC above is written from. | If an AC's user-intent framing needs re-grounding mid-implementation. | AC-001, AC-002 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` | The signed-off determination that this project renders **no** user-facing surface, and the note redirecting the surface obligation to rustdoc and the briefs. The reason the composition family has nothing to bind to. | Before writing any presentation-shaped assertion, or if a surface obligation seems to have been dropped. | AC-003 |
| `standards/rust/60-what-a-test-must-prove.md` (RS-60-3, RS-60-4) | One defect per mutant, and an oracle spelled in a direction sharing no subroutine with the implementation — both bite directly on the mutants and on the rules. | Before writing the mutants, and before writing either rule's assertions. | AC-004 |
| `standards/rust/62-doctests-and-harnesses.md` (RS-62-5) | A `publish = false` crate's doctests are skipped by `cargo test --doc`, so an unrun doctest looks exactly like a passing one. | If this story adds a doctest to any public item. | AC-005 |
| `standards/rust/70-rustdoc-obligations.md` | The repository-wide rustdoc obligation that replaces the rendered-surface obligation for this project. | Before making anything `pub`. | AC-005 |
| `RUNBOOK.md` (`:4576-4580`, `:4597-4602`) | DR-5's origin and the proof artefact's exact wording — "one suite green against three peers, two structurally unlike, with a byte-identical payload round-tripped across a store boundary". | When writing the implementation report's claim about what was proven. | AC-006 |
| `.redkiln/config.yaml` (`:40`, `:48`, `:55`, `:62-67`) | The four gate commands this story is actually measured by, and `require_ledger: true` — the reason `_ledger.md` is a merge blocker rather than paperwork. | Before running the merge gate, and when filling the ledger. | AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the six ids the first pass enumerated** — AC-001 …
   AC-006. None added, none dropped. AC-001/AC-002 are project AC-006's two
   assertions; AC-003/AC-004/AC-005 are the instrument quality that makes them
   measurements rather than claims; AC-006 is the evidence-or-reason arm HS-S0108
   consumes.
2. **Byte-identical *where*.** `discover.md` deferred it; settled in Context pack
   §1 and enforced by AC-001 — on the way **out** of the receiving store via
   `EventStore::read` after the ingest committed, never on the `Bytes` handed to
   `IngestStore::ingest`.
3. **How "an observable no-op" is observed.** `discover.md` offered three
   candidates and a bar; settled as **all three together** (identity set, typed
   witness, no compensation) in AC-002, because each alone is defeated by a named
   mutant and the typed witness is explicitly not permitted to stand alone.
4. **Where the negative control lives.** `discover.md` deferred it between this
   story and HS-S0103; settled **here**, as a `ConformantVariant` with `fails: &[]`
   (AC-003). The control only has something to refute once a payload assertion
   exists, which is this story; the fixture *contract* remains HS-S0103's and is
   not widened.
5. **Interaction quality with no surface.** `_design.md` records no user-facing
   surface and a human signed that determination off, so the composition family has
   no numbers to bind. Rather than skip §6.7, the surviving question — what the
   adapter author actually meets — is answered against the run output and the
   failure message, and every invariant named there is carried by an existing AC
   row so `redkiln verify` can extract it. No invariant lives only in prose.
6. **No third rule name.** Both assertions land under names existing `[FROZEN]`
   clauses already claim (SY-35, SY-11). A third name is EC-002: a blocker, never a
   `spec/SPECIFICATION.md` edit.
7. **The escape hatch is not a downgrade.** Project AC-006's second arm is a
   recorded reason naming the mutating hop (EC-001), routed to HS-S0106. A
   decoded-value comparison would satisfy the wording and prove nothing; it is
   forbidden by DR-5 and detected by the control.
8. **`crates/happenstance-sync-testkit/**` does not exist in the tree yet** — it is
   created by this project's earlier slices (`sync-testkit-crate-and-rule-registry`,
   `gate-mounts-for-the-sync-suite`). It is therefore named as the mount point and
   in the PR boundary, but **not** cited as an anchor: every anchor above resolves
   against the tree today, and the shape to copy is the event-store testkit's
   (`crates/happenstance-testkit/src/registry.rs`).
