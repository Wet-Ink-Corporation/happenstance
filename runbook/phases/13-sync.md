## Phase 13 — `happenstance-sync` and its testkit

> Carried from `RUNBOOK.md:5492-5598` at `3916f29`, verbatim below this note.
> **Edited since the split:** the dependency row now includes phase 17, because
> `0.4.0` changes the published surface this phase builds on — see the
> [roadmap](../roadmap.md). The "Phase 12 is in the dependency row as ordering"
> paragraph is history: phase 12 is done. The ADR-0026 and ADR-0027 work items
> are reworded to say they *record* SY-1 – SY-7 rather than reopen them — the
> section above them already treated those clauses as settled.


**Goal.** Replication between happenstance instances, expressed as a **port with
adapters** rather than a protocol with one peer.

**Why here.** The hard questions are internal to the crate, and the easy part is
already paid for by ADR-0003: a peer forwards opaque bytes, never needs the
sender's domain types, cannot fail to parse a payload it does not understand, and
cannot corrupt one by re-encoding it.

**Phase 12 is in the dependency row as ordering, not as a technical
prerequisite.** Nothing in this phase needs a crate to be on crates.io; what it
needs is phases 5, 8, 9 and 10. The 12 is there because the alternative — holding
0.1 for replication — is the sequencing this whole plan exists to reject, and a
dependency row is where that intent is enforced. If 12 slips, this phase is not
blocked by anything real, and a session that reaches here with 12 outstanding
should say so in the log rather than wait.

**Decisions it settles.** ADR-0026 (what a peer is), ADR-0027 (how logs
reconcile). Discharges SY-1 – SY-35 and WF-1's interop deferral.

**The two structural decisions, already settled by the specification.** The port
lives in `happenstance-sync`, not the contract crate — symmetry would argue for
putting it beside `EventStore`, and doing so would put replication back on the
publish path for no gain. And `SyncPeer` describes **one peer**; a `SyncRunner`
fans out. Multi-peer reconciliation, primary/secondary ordering and what to do when
two peers disagree are *policy*, and policy on the port makes every adapter author
inherit the merge problem and makes the conformance suite test a policy rather than
a transport.

**Ingest is unconditional, with compensation** (SY-1 – SY-7). A replicated event is
never refused for a reason that is a function of the receiving store's state.
Where a local condition would have been violated, the receiving side appends the
losing event **and** a domain-supplied compensating event as one atomic unit. The
rationale is convergence, not politeness: rejection is a function of local state,
so different peers reject different events and the union of facts is never reached;
a compensation is an append rather than a refusal, so no peer deletes a fact its
user was told had landed. The domain decides what a compensation means; the port
provides the atomicity and the identity that makes it idempotent.

**Work**

- [ ] **Claim `happenstance-sync` and `happenstance-sync-testkit` on crates.io**, per phase 0's
      rule that a name is reserved when its phase starts, not before — the point
      being that by now there is a crate to justify it with.

- [ ] ADR-0026. Record the ingest boundary SY-1 – SY-7 already fix, and settle
      what makes re-delivery harmless and what the port may assume about a transport it cannot see. The two real peers are a
      Durable Object over a socket and a Postgres over one-shot HTTP with no
      interactive transaction; a `SyncPeer` that cannot be implemented by the second
      is a `SyncPeer` shaped like the first. Phase 2's sketch is the evidence.
- [ ] ADR-0027. Record SY-1 – SY-7's ingest and compensation constraints as
      settled, not open, and decide the merge rule, whether replication is
      whole-log or scoped (SY-27, SY-28 — a spoke holding a filtered subset cannot
      distinguish "not yet received" from "filtered out", so a position-based resume
      watermark against a hub is unsound), idempotent bulk ingest in bounded round
      trips (SY-14), and hub-and-spoke as a first-class topology beside
      peer-to-peer.
- [ ] `IngestStore` in `happenstance-sync` (VT-10) — the seam through which a
      foreign identity arrives, and the reason `EventStore::append` never grew a
      slot for one.
- [ ] `sync_peer_conformance!` in `happenstance-sync-testkit`, emitted through
      phase 1's registry so it inherits the tokio/blocking/wasm flavours. **The
      suite never decodes a payload** (SY-35) — a suite that parses `data` would
      certify a peer that does, and ADR-0003's guarantee is exactly that no peer
      needs to.
- [ ] `MemorySyncPeer` behind a `memory` feature — the oracle, the doctest target,
      and something an application author can test against before any real peer
      exists. The same three-part rationale `memory.rs:16-23` gives for
      `MemoryEventStore`, and the same cold-start problem the projection port had
      without one.
- [ ] Envelope types on phase 5's tested wire format, with the format version
      first.
- [ ] Ingest bound on `EventStore`, not `SendEventStore` — the Cloudflare side is
      single-threaded, and CLAUDE.md rule 4 binds the sync runner too because the
      `!Send` peer sits mid-chain rather than at a leaf.
- [ ] Two real peers — the phase-9 Durable Object and the phase-10 Postgres — plus
      the round trip between a native SQLite store and each.
- [ ] Record the DCB wire interop decision (WF-1) in ADR-0026's envelope section:
      named, deferred, with the experiment being a specific external implementation
      to interoperate with. Not silence.

**Proof artefact.** `sync_peer_conformance!` green against `MemorySyncPeer` and
**two structurally unlike networked peers** — a socket-reachable Durable Object and
a one-shot-HTTP Postgres — plus a round-trip test asserting the payload `Bytes` are
**byte-identical** end to end and that replaying the same batch twice changes
nothing. The byte-identity half is what lifts ADR-0003 from provisional; the
two-peer half is what makes this a port rather than a protocol.

**Exit criteria**

- [ ] ADR-0026 and ADR-0027 written before the code they constrain.
- [ ] `SyncPeer` implemented by three peers, one of which cannot hold a
      transaction open across a round trip.
- [ ] Hub-and-spoke and peer-to-peer are both expressible, and the crate's module
      doc no longer describes only one.
- [ ] `ingest_never_rejects` and `compensation_is_atomic_with_the_losing_event`
      green, with a mutant that fails each.
- [ ] The byte-identical round trip is green, and ADR-0003 loses `provisional`.
- [ ] Every `[DEFERRED]` `SY` clause is either settled or renewed against a named
      experiment; a renewal with no experiment is a build failure under CF-38.

**Cases this makes writable.** E2E-33 – E2E-42, E2E-45, and the idempotency half
of E2E-07 (ES-24).

**Estimate.** 12 days.

**Session log**
