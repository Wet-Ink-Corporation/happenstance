---
item: HS-P0017
stage: storymap
created: 2026-08-12T03:30:22.654Z
updated: 2026-08-12T03:30:22.654Z
template_sig: 1c63534a
rendered_sig: 6458c458
---

# Story Map — What a position means across a store boundary

Sixteen stories in seven slices. The spine is [`project.md`](project.md)'s AC-001 – AC-015;
the slicing is [`_decomposition.md`](_decomposition.md)'s architecture brief (*Composition
root*, *Gate mounts*) and testing brief (*The test mix, tier by tier*).

Three shaping facts, stated once so no story re-litigates them:

1. **The decisions gate the code.** AC-001 and `RUNBOOK.md:4606` require ADR-0026 and
   ADR-0027 accepted and merged *before* the code they constrain, and `CLAUDE.md`
   (*Where the work lives*) requires atoms to arrive through `.kb/_intake/` and
   `/redkiln:kb-ingest` — hand-authoring was reverted once already (`0269720`). Slice 1
   is therefore entirely decision-record work, and every later slice depends on it.
2. **The central question is already answered and `[FROZEN]`.** SY-1
   (`spec/SPECIFICATION.md:5841-5867`) and SY-6 (`:5973-6029`) settle whether ingest
   re-checks the writer's asserted conditions. No story deliberates it; ADR-0026
   reconciles and cites it (`_decomposition.md`, *Tension 2*), and the conformance rules
   make it mechanical.
3. **The proof artefact is a suite, not a feature** (DoD 4). So the capability slices
   climb: an event crosses a boundary between two in-memory stores (slice 2), a peer is
   *proved* to conform (slice 3), the wire carries it (slice 4), a fleet runs it
   (slice 5), and two structurally unlike peers run the same suite (slice 6).

## Backbone

The activities a consumer of `happenstance-sync` walks, left to right. Each column names
the slice(s) that make it real.

| # | Activity | Outcome a reader can observe | Slice |
| --- | --- | --- | --- |
| A1 | **Find out what a peer is, and what ingest promises** | Two accepted decision atoms under `.kb/decisions/`, each naming the alternatives that lost, and two open questions resolved rather than deleted | `decisions-of-record` |
| A2 | **Move an event across a store boundary** | `IngestStore` has real bodies; `MemorySyncPeer` carries a batch from one store to another and the foreign `EventId` survives | `ingest-seam-and-memory-oracle` |
| A3 | **Prove a peer conforms** | `sync_peer_conformance!(MyFixture::new())` runs, a declined capability reports its reason, and every rule has a mutant that fails it by name | `sync-conformance-suite` |
| A4 | **Speak the wire, and keep the payload opaque** | A message set instantiates `Envelope<T>`, and a payload's `Bytes` come out the far side byte-identical with replay a no-op | `wire-message-set-and-round-trip` |
| A5 | **Run a fleet** | A runner fans out over peers on the weaker (`!Send`-tolerant) bound, and hub-and-spoke and peer-to-peer are both wirings of the same types | `runner-and-topologies` |
| A6 | **Trust it against a peer unlike the oracle** | One suite green against a socket-reachable Durable Object and a one-shot-HTTP Postgres peer that cannot hold a transaction open | `live-unlike-peers` |
| A7 | **Leave the record true** | Every clause this project touched still rejects something that exists, every deferral names an experiment, and the clause arithmetic is computed rather than asserted | `spec-repairs-and-clause-exit` |

## Slices

Grouped by milestone. `foundation` stories land real in-tree substrate consumed by a
capability slice in this same project; none of them is a double, a flag or a `todo!()`.

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --- | --- | --- | --- | --- | --- |
| `decisions-of-record` | `adr-0026-peer-ingest-and-transport` | foundation | Re-derive the SY/WF clause ledger from `spec/SPECIFICATION.md` at HEAD, then author ADR-0026 (long form under `references/adr/`, atom via `.kb/_intake/` + `/redkiln:kb-ingest`) answering what a peer is, what the port may assume about an unseen transport, and what ingest promises — reconciling the central question against frozen SY-1/SY-6, recording WF-1's interoperability half in its envelope section, and naming SY-32's handoff to ADR-0028 | — | AC-001, AC-002, AC-010, AC-011, AC-014 |
| `decisions-of-record` | `adr-0027-merge-compensation-and-message-set` | foundation | Author ADR-0027 through the same intake path — the merge rule, the compensation contract, replication scope, whether hub-and-spoke and peer-to-peer are one abstraction or two, and the message set plus the derives on `PushBatch`/`EventGroup`/`ReplicatedEvent` authorised **by name** — taking the complement of ADR-0026's clause range so the split adds up | `adr-0026-peer-ingest-and-transport` | AC-001, AC-010, AC-014 |
| `decisions-of-record` | `open-questions-resolved-and-indexed` | foundation | Resolve both phase-13-owned open-question atoms in place — `sync-message-set-and-format-version` and `dcb-reference-publishes-no-wire-format` — annotate rather than delete, update `.kb/maps/open-questions-index.md`, and leave `redkiln validate --kb && redkiln doctor` green | `adr-0026-peer-ingest-and-transport`, `adr-0027-merge-compensation-and-message-set` | AC-013 |
| `ingest-seam-and-memory-oracle` | `memory-store-ingest-seam` | foundation | Give `MemoryEventStore` the one additive **inherent** `&self` operation that accepts an already-identified `SequencedEvent`, so a foreign `EventId` has a door to come in through — `EventStore`'s trait signature byte-identical before and after, and the semver class stated in the ADR that authorises it | `adr-0026-peer-ingest-and-transport` | AC-002 |
| `ingest-seam-and-memory-oracle` | `ingest-store-and-memory-peer-round-trip` | capability | Replace the `todo!()` bodies with real `IngestStore` and `MemorySyncPeer` ones and drop the scoped `#![allow(clippy::todo)]`, so a batch pushed from one in-memory store lands at the receiver's local tail with the foreign `EventId` preserved, group-atomically and idempotently — with the doctest on `MemorySyncPeer` that shows one round trip holding no state | `memory-store-ingest-seam` | AC-002 |
| `sync-conformance-suite` | `sync-testkit-crate-and-rule-registry` | foundation | Create `crates/happenstance-sync-testkit/` (`publish = false`, its own `version` key), depending on `happenstance-testkit` for `Capability`, `RuleOutcome` and the three emitters, with `for_each_sync_peer_rule!` + `sync_peer_conformance!` in the registry's exact shape (callback as `tt`, fixture hoisted behind `__conformance_fixture`), a single-flavour GAT-free peer fixture carrying its own round-trip counter, and a `no_orphan_sync_rules` meta-test | `ingest-store-and-memory-peer-round-trip` | AC-003, AC-015 |
| `sync-conformance-suite` | `gate-mounts-for-the-sync-suite` | foundation | Teach every gate constant that names only `happenstance-testkit` about the fourth suite — `RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST`, the position-literal lint and the changelog-per-rule lint — and add the two `wasm32` steps (a build of `happenstance-sync` and a check of the sync harness) beside the existing four | `sync-testkit-crate-and-rule-registry` | AC-004, AC-009, AC-014 |
| `sync-conformance-suite` | `headline-rules-and-mutant-registry` | capability | Land the rules the frozen clauses already name — `ingest_never_rejects` (SY-1), `compensation_is_atomic_with_the_losing_event` (SY-2), `wire_condition_with_after_is_refused` (SY-6) — green against `MemorySyncPeer`, each with a compiled wrong peer in the testkit's own `tests/` and a `Declared` mutant entry carrying a never-empty provenance naming the real peer shape that makes it plausible | `sync-testkit-crate-and-rule-registry`, `gate-mounts-for-the-sync-suite`, `adr-0027-merge-compensation-and-message-set` | AC-002, AC-003, AC-004, AC-008 |
| `wire-message-set-and-round-trip` | `message-set-on-the-envelope` | capability | Land the message set as new types instantiating `Envelope<T>` (never an enum around it), with the derives ADR-0027 authorised by name, `FORMAT_VERSION`'s disposition settled, and the refusal of an unknown version proven to happen **before** any message is decoded — plus the WF rules and their mutants | `headline-rules-and-mutant-registry`, `adr-0027-merge-compensation-and-message-set` | AC-006 |
| `wire-message-set-and-round-trip` | `byte-identical-round-trip-and-idempotent-replay` | capability | Assert the payload `Bytes` byte-identical at the receiver across a real store boundary and replaying the same batch twice an observable no-op, with the negative control that would fail if any assertion on any path touched `Event::data` or `Event::metadata` | `message-set-on-the-envelope` | AC-006 |
| `wire-message-set-and-round-trip` | `adr-0003-provisional-lift` | foundation | Lift ADR-0003's `provisional` marker with a **new** atom through kb-ingest that `depends_on` it and cites the round trip as the evidence its own Status section asks for — never an edit to the accepted atom — or, if the round trip cannot be made byte-identical, record why it cannot lift | `byte-identical-round-trip-and-idempotent-replay` | AC-006 |
| `runner-and-topologies` | `send-free-sync-runner` | capability | Land the runner that fans out over peers and advances the owned resume token, bound on `EventStore`/`SyncPeer`/`IngestStore` and never on a `Send` flavour (one name of each pair per module), proved by a real `tokio::spawn` with the `!Send` peer sitting mid-chain rather than at a leaf, and by the `wasm32` gate steps actually running it | `ingest-store-and-memory-peer-round-trip`, `gate-mounts-for-the-sync-suite` | AC-009 |
| `runner-and-topologies` | `hub-and-spoke-and-peer-to-peer-topologies` | capability | Exercise both topologies over the same suite and the same runner, with one adapter type instantiated in both roles on different edges so SY-9's "hub-ness is an edge property" is tested rather than asserted, and bring `crates/happenstance-sync/src/lib.rs`'s module documentation into line with whichever shape landed | `send-free-sync-runner`, `adr-0027-merge-compensation-and-message-set` | AC-007 |
| `live-unlike-peers` | `durable-object-and-neon-peers` | capability | Confirm both environments exist from HS-P0013/HS-P0014 first, then implement `SyncPeer` and `IngestStore` in `happenstance-cloudflare` and `happenstance-neon` (local type, foreign trait — the only place coherence allows it) and run the one suite against all three peers, with an unavailable environment reported as a declined capability carrying the fixture's stated reason and never as a vanished target | `headline-rules-and-mutant-registry`, `message-set-on-the-envelope`, `send-free-sync-runner` | AC-005 |
| `spec-repairs-and-clause-exit` | `frozen-clause-repairs` | capability | Re-run the `re-check|recheck|re-evaluat` sweep and record the finding either way, drop the `(new)` markers from the `Rule:` lines whose rules now exist, and repair — in the playbook's three-part form, MUST verbatim — every clause whose `Rejects:` names a symbol this project changed, each repair authorised by ADR-0026 and justified by the mechanical test that the admitted implementation set is unchanged | `headline-rules-and-mutant-registry`, `adr-0026-peer-ingest-and-transport` | AC-002, AC-012 |
| `spec-repairs-and-clause-exit` | `clause-arithmetic-and-deferral-renewals` | capability | Settle or renew every `[DEFERRED]` SY clause against a *named* experiment (a renewal with no experiment is a build failure under CF-38), then compute the union of ADR-0026's and ADR-0027's clause ranges against this project's stated range, write the arithmetic down, and leave `cargo xtask spec-trace` green | `frozen-clause-repairs`, `gate-mounts-for-the-sync-suite` | AC-010, AC-014 |

## Coverage

Every project AC-### is claimed by at least one story, and no two stories own the same
responsibility for it (the second column says which part each owns).

| Project AC | Stories | Split of responsibility |
| --- | --- | --- |
| AC-001 | `adr-0026-peer-ingest-and-transport`, `adr-0027-merge-compensation-and-message-set` | one atom each, both through the ingest path, both merged before the code they constrain |
| AC-002 | `adr-0026-peer-ingest-and-transport`, `memory-store-ingest-seam`, `ingest-store-and-memory-peer-round-trip`, `headline-rules-and-mutant-registry`, `frozen-clause-repairs` | ADR reconciles and cites; the seam and the bodies keep `EventStore` untouched; the rules make SY-1/SY-6 mechanical; the repair story proves no frozen clause was edited |
| AC-003 | `sync-testkit-crate-and-rule-registry`, `headline-rules-and-mutant-registry` | registry shape + orphan meta-test; then rules present in output with declined capabilities reported |
| AC-004 | `gate-mounts-for-the-sync-suite`, `headline-rules-and-mutant-registry` | the no-literal-position lint scope; the mutant registry with provenance |
| AC-005 | `durable-object-and-neon-peers` | one suite, three peers, two genuinely unlike |
| AC-006 | `message-set-on-the-envelope`, `byte-identical-round-trip-and-idempotent-replay`, `adr-0003-provisional-lift` | wire; the byte-identity and replay assertions; the marker lift as a new atom |
| AC-007 | `hub-and-spoke-and-peer-to-peer-topologies` | both topologies, one adapter in both roles, module doc corrected |
| AC-008 | `headline-rules-and-mutant-registry` | the two headline rules with mutants that fail them by name |
| AC-009 | `gate-mounts-for-the-sync-suite`, `send-free-sync-runner` | the `wasm32` gate steps; the weaker bound proved by a real spawn |
| AC-010 | `adr-0026-peer-ingest-and-transport`, `adr-0027-merge-compensation-and-message-set`, `clause-arithmetic-and-deferral-renewals` | WF-1's half in the envelope section; the ADR-side renewals; the CF-38 sweep at exit |
| AC-011 | `adr-0026-peer-ingest-and-transport` | SY-32 recorded as a named handoff to ADR-0028, with the `replication → retention` edge re-checked and confirmed |
| AC-012 | `frozen-clause-repairs` | the audit by id, and the repairs |
| AC-013 | `open-questions-resolved-and-indexed` | both atoms resolved in place, index updated, `validate --kb` green |
| AC-014 | `adr-0026-peer-ingest-and-transport`, `gate-mounts-for-the-sync-suite`, `clause-arithmetic-and-deferral-renewals` | the ledger re-derived and the split fixed before drafting; `RULE_FILES` taught about the sync rules; the arithmetic computed at exit |
| AC-015 | `sync-testkit-crate-and-rule-registry` | the new crate is born `publish = false`, so neither sync crate is released and the recorded name-claim disposition holds |

**No AC is orphaned, and nothing is covered twice by the same responsibility.** Two ACs
are deliberately spread across a decision story and a code story (AC-002, AC-006, AC-014):
that is the *decisions gate the code* ordering, not duplication — the atom states the
answer and the later story is the thing that would not exist if the answer were wrong.

Three notes on what this map does **not** slice:

- **No story depends on substrate owned outside this project.** The one core-side change
  (`memory-store-ingest-seam`) is a foundation story inside this project, sequenced as a
  hard predecessor rather than deferred behind a flag. The Durable Object and Neon
  *stores* are HS-P0013's and HS-P0014's and are consumed as built; if either environment
  is absent at `live-unlike-peers`, that is a blocker to raise, never a stand-in to write.
- **No "build it / wire it in" split.** `gate-mounts-for-the-sync-suite` sits in the same
  slice as the crate it mounts, because a suite that never reaches `spec-trace`,
  `lint-position-literals` or the `wasm32` steps is exactly the failure the architecture
  brief calls "silently escapes three checks".
- **No E2E story.** `.redkiln/config.yaml` reserves `cargo xtask ci` for the terminal
  project; `cargo xtask ci --fast` is this project's ceiling (DoD 1).

## Merge order

Slice by slice; within a slice, top to bottom. Foundations precede every capability that
consumes them.

1. **`decisions-of-record`** — `adr-0026-peer-ingest-and-transport` →
   `adr-0027-merge-compensation-and-message-set` → `open-questions-resolved-and-indexed`.
   Nothing in a `.rs` file that a clause constrains may merge before this slice does.
2. **`ingest-seam-and-memory-oracle`** — `memory-store-ingest-seam` →
   `ingest-store-and-memory-peer-round-trip`. The first real crossing of a store boundary,
   and the oracle every later slice measures against.
3. **`sync-conformance-suite`** — `sync-testkit-crate-and-rule-registry` →
   `gate-mounts-for-the-sync-suite` → `headline-rules-and-mutant-registry`. The suite is
   not landed until the gate runs it and a mutant fails it.
4. **`wire-message-set-and-round-trip`** — `message-set-on-the-envelope` →
   `byte-identical-round-trip-and-idempotent-replay` → `adr-0003-provisional-lift`. The
   lift is last because it cites evidence that does not exist until the story before it
   merges.
5. **`runner-and-topologies`** — `send-free-sync-runner` →
   `hub-and-spoke-and-peer-to-peer-topologies`.
6. **`live-unlike-peers`** — `durable-object-and-neon-peers`. This is DoD 4's proof
   artefact; it needs the suite, the wire and the runner all in the tree.
7. **`spec-repairs-and-clause-exit`** — `frozen-clause-repairs` →
   `clause-arithmetic-and-deferral-renewals`. Last on purpose: a repair can only name the
   rules and symbols that exist, and the arithmetic is an exit computation.

Slices 4 and 5 have no edge between them and may be interleaved; slice 6 depends on both.
