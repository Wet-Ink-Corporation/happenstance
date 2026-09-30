## Phase 13 — `happenstance-sync` and its testkit

> Carried from `RUNBOOK.md:5492-5598` at `f89e184`, verbatim below this note.
> **Edited since the split:** the dependency row now includes phase 17, because
> `0.4.0` changes the published surface this phase builds on — see the
> [roadmap](../roadmap.md). The "Phase 12 is in the dependency row as ordering"
> paragraph is history: phase 12 is done. The ADR-0026 and ADR-0027 work items
> are reworded to say they *record* SY-1 – SY-7 rather than reopen them — the
> section above them already treated those clauses as settled.
> **Edited at phase 16:** the crates.io claim is dated; five items are added —
> the Postgres and Neon `StoreId` restore gap, a KV-capped peer for SY-18, the
> filtered-store instrument SY-27 and SY-28 need before phase 14 would build it,
> the `freeze-by-13` clauses from [the 1.0 dispositions](../ledgers.md), and
> CF-25's bar for the peer-port freeze — with exit criteria for the clauses and
> the restore gap (ADR-0066). The dependency row gains phase 18, because SY-20's
> rule here consumes the convergence declaration phase 18 builds, and the two
> had no order between them.


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
      being that by now there is a crate to justify it with. Both were still
      unclaimed on 2026-09-29 (the registry API answered `404` for each), and both
      are among the nine crates 1.0 promises (ADR-0066), so this is the first
      thing the phase does rather than the last.
- [ ] **The Postgres and Neon `StoreId` has no restore detection**
      (`.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md`,
      VT-6). Both mint once, in a migration —
      `crates/happenstance-postgres/migrations/0001_event_log.sql:111-118` and
      `crates/happenstance-neon/migrations/0001_neon_log.sql:139-141` — and neither
      detects a restore, offers a re-mint, or documents a procedure, which are the
      only two conditions under which VT-6 (by ADR-0014) permits mint-once. A
      `pg_restore` of an older backup, or a Neon branch, which is a clone by
      construction, can therefore re-issue an `(StoreId, position)` pair — the one
      thing VT-6 forbids and the thing a sync watermark trusts. Close it before
      `restored_peer_does_not_reissue_identities` is written, because that rule
      would otherwise be red on the one-shot-HTTP Postgres peer this phase
      builds, and on anything replicating from a native Postgres store. A re-mint
      operation is an additive inherent method, and detection and a documented
      procedure are additive too. Mint-per-open is a behaviour change, and phase
      17's window closes before this phase opens, so phase 17 decides it; if
      phase 17 declined it, the remedy here is one of the additive arms, and
      mint-per-open would be a post-1.0 major.

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
      slot for one. **VT-10 is frozen by ADR-0073 (phase 17)**: the write path is
      the adapter's own row writer. What is left here is turning the SQLite
      spike's `cfg(test)` into a `sync` feature once this crate is published,
      building the real peers' ingest the same way, a sync-owned in-memory oracle
      (the `MemoryEventStore` impl was deleted, not finished), ingest's own error
      type, the watermark's plan assertion, and the policy for an event claiming
      this store's own `StoreId` that it does not hold — pinned, not endorsed, by
      `pinned_vt6_breach_an_unheld_own_id_is_ingested_and_wedges_the_append_that_reaches_it`.
- [ ] **ES-41's held-versus-visible reading.** ADR-0028 froze ES-41 at phase 17
      and left one reading open: where ES-10's frontier separates a committed row
      from a visible one, does `contains_event_id` mean held or visible?
      `happenstance-postgres` and `happenstance-neon` answer held, and record it
      as unsettled. Settle it, and write a rule that stages a committed row above
      the frontier, with a wrong implementation that carries the frontier
      predicate into the probe.
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
- [x] ~~Delete `happenstance-sync`'s placeholder `EventId`, `StoreId` and
      `RecordedAt` from `src/identity.rs`, and use `happenstance-core`'s.~~ Done
      at phase 17 (ADR-0073), pulled forward because the spike's trait had to
      speak the store's own types: the placeholder `RecordedAt` was a `u64`, which
      fired VT-9's restated falsifier by construction.
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
- [ ] **A KV-capped peer for SY-18.** The two real peers above are a SQL-backed
      Durable Object (2 MiB rows) and Postgres over HTTP; neither has a per-value
      cap, so neither can test whether `PeerLimits` prevents a failure or only
      relocates it. SY-18's own falsifier is the Turnstile peer-D shape: a store
      with a 128 KiB cap on each value, against an origin that has already
      committed a 340 KB payload. A fixture peer with that cap is enough; it need
      not be a real KV binding. VT-21's floor is compared across the peer set on
      the same instrument. If it is not built, SY-18 is renewed past 1.0 by a
      record naming the Turnstile experiment — safe, because `PeerLimits` is
      `#[non_exhaustive]` — and not left deferred by default.
- [ ] **The filtered-subset store, built here rather than at phase 14.** SY-27's
      falsifier, and SY-28's after it, is a spoke holding a deliberately filtered
      subset of a hub's log, attempting a position-based resume against it. That
      is the same testkit-adjacent instrument as CF-27's suffix store
      (`spec/E2E-CASES.md:1679-1684`), which the ledger gives to phase 14 —
      and 14 runs after this phase. So either this phase builds the instrument, in
      a shape phase 14 then extends into the suffix store, or a record
      re-dispositions SY-27 and SY-28 before this phase exits. Freezing either
      clause without the instrument is the decorative kind of freeze.
- [ ] **The clauses phase 16 gave this phase to freeze.** Each `freeze-by-13` row
      in [the 1.0 dispositions](../ledgers.md), and what freezes it:
      - **VT-6** — `restored_peer_does_not_reissue_identities` in the sync
        testkit, after the restore gap above is closed.
      - **VT-9** — a sync-testkit rule that ingest preserves `RecordedAt`, with a
        mutant; ADR-0066 restates the clock falsifier.
      - **VT-21** — compared across the peer set with SY-18; the tightest shipped
        target, Neon at 131,072 bytes, clears the 64 KiB floor twice over.
      - **VT-24** — SY-14's bulk ingest is the first consumer that batches by the
        128-event floor.
      - **SY-7** — ADR-0027; the falsification test on `MemorySyncPeer` with two
        adjudicator configurations, its divergence recorded as the evidence.
      - **SY-10** — both topologies expressible, as the exit criterion below
        already requires, with `directional_merge_rules_compose`.
      - **SY-14** — the bulk-ingest measurement against the Neon peer.
      - **SY-18** — the KV-capped peer above.
      - **SY-20** — `convergent_projection_is_interleaving_independent`, written
        against the convergence declaration phase 18 builds. Phase 18 runs
        first — it is in this phase's dependency row for that reason — so the
        declaration exists when this rule is written.
      - **SY-22** — the `cost-layers` test; the declaration's placement is fixed
        with SY-21 at phases 17 and 18, both of which precede this phase.
      - **SY-23** — ADR-0027's merge rule.
      - **SY-27, SY-28, SY-29** — together, on the filtered-subset store above;
        a peer-supplied `Query` exists on the port only if replication is scoped.
      - **SY-30** — the two unlike real peers pushing real envelopes.
      - **SY-31** — the runner half. The reserved `sync/` prefix is phase 17's,
        with `projection-id-is-unvalidated`.
      - **CF-40** — whether `payload_len` (data plus metadata) is the budget unit,
        once phase 17 has built ADR-0043's `MetadataLen`.

      WF-1 and WF-11 are renewed past 1.0, not frozen here; this phase records
      WF-1's renewal in ADR-0026 and confirms which encoding the sync transport
      forwards WF-11's payload through.
- [ ] **CF-25's bar for the peer-port freeze**
      (`.kb/open-questions/cf-25-cf-26-portfolio-check-does-not-exist.md`).
      CF-25 is `[FROZEN]` and gates every port freeze on `cargo xtask spec-trace`
      reading §6.5's instrument-portfolio table, and `xtask/src/spec_trace.rs`
      has no portfolio logic. The SY clauses on `SyncPeer` are the next port
      freeze it gates, so each of them either lands with the check built, or
      takes CF-25's second route: naming the axis it accepts risk on and the
      record that accepts it. If the check stays unbuilt, CF-25's `Rule:` line is
      repaired to say the check is a reviewer's walk rather than `spec-trace`,
      under `kb-playbook-repair-frozen-clause-001`. The open question's
      sub-questions 1 and 2 are answered either way; phase 21's clause audit
      holds whichever route was taken.

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
- [ ] Every `freeze-by-13` clause in [`ledgers.md`](../ledgers.md)'s 1.0
      dispositions is `[FROZEN]`, or re-dispositioned by a record that says why.
- [ ] `restored_peer_does_not_reissue_identities` is green against the
      Postgres-backed peer, not only against the memory one, and both
      `happenstance-postgres` and `happenstance-neon` document what they do on a
      restore.
- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Cases this makes writable.** E2E-33 – E2E-42, E2E-45, and the idempotency half
of E2E-07 (ES-24).

**Estimate.** 12 days.

**Session log**
