---
id: HS-P0017
uid: 907c23
type: project
slug: replication-identity-and-ingest
title: What a position means across a store boundary
parent: HS-I0006
initiative: from-contract-to-published-library
project: replication-identity-and-ingest
status: in-review
process: project
stage: design
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-12T12:57:18.720Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# What a position means across a store boundary

## One-line objective

Give replication identity a written answer on file — accepted decision atoms or an
explicit reasoned refusal — with a conformance suite and three peers behind it, so
that nobody has to guess what a `SequencePosition` means once it has crossed a
store boundary.

## How this advances the initiative

The charter names two silences and calls silence a non-outcome
([`../initiative.md`](../initiative.md), *Goals*). This project closes the first
of them. It owns **BR-10**, **AC-13** and **DoD 14**, and it is the only project in
the portfolio that can close them
([`../_decomposition.md`](../_decomposition.md), traceability matrix).

The reason the answer cannot be reasoned out on paper is stated in the crate
itself: `SequencePosition` is meaningful only within a single store, so two
instances that each append independently assign the same positions to different
events, and "send everything after position N" is wrong by construction
(`crates/happenstance-sync/src/lib.rs:100-104`). A port designed against one peer
shape encodes that peer's assumptions about identity — the workspace's own
standing rule, *a port with one implementation is shaped like that implementation*
(`crates/happenstance-sync/src/lib.rs:30-35`) — so the answer only becomes evidence
when an unlike second and third peer have run the same suite.

It also lands **after** `0.2.0`. The sync port is deliberately outside the contract
crate so that publishing never waits on replication
(`crates/happenstance-sync/src/lib.rs:37-40`), which is why this project sits at
rank 4 of the DAG at no cost to anything already published
([`../_decomposition.md`](../_decomposition.md), *Sequencing*), and why
`RUNBOOK.md:4526-4532` records phase 12 as an ordering constraint rather than a
technical prerequisite.

## In scope (this project)

- **ADR-0026** — what a sync *peer* is, what the port may assume about a transport
  it cannot see, and what ingest promises (`RUNBOOK.md:4562-4566`, ADR queue row at
  `RUNBOOK.md:305`).
- **ADR-0027** — the merge rule, the compensation contract, replication scope, and
  whether hub-and-spoke and peer-to-peer are one abstraction or two
  (`RUNBOOK.md:4567-4572`, `RUNBOOK.md:306`).
- **The central question, answered out loud:** does ingest re-check the writer's
  asserted append conditions? Reconciled against `spec/SPECIFICATION.md` §5.1 and
  §5.4, which already answer it normatively — SY-1 `[FROZEN]`
  (`spec/SPECIFICATION.md:5841-5867`) and SY-6 `[FROZEN]`
  (`spec/SPECIFICATION.md:5973-6029`), the origin's condition travelling as
  *evidence, not as an instruction*.
- **`IngestStore` made real** in `happenstance-sync` — the seam through which a
  foreign identity arrives, and the reason `EventStore::append` never grew a slot
  for one (`crates/happenstance-sync/src/ingest.rs`, VT-10, SY-8 at
  `spec/SPECIFICATION.md:6070-6086`).
- **`happenstance-sync-testkit`** and `sync_peer_conformance!`, emitted through the
  existing rule registry (`crates/happenstance-testkit/src/registry.rs`) so it
  inherits the tokio, blocking and `wasm32` flavours — and a mutant per rule, under
  CF-1 – CF-5 (`spec/SPECIFICATION.md:7161-7231`).
- **Three peers:** `MemorySyncPeer` as the oracle and doctest target
  (`crates/happenstance-sync/src/memory.rs`), plus two structurally unlike
  networked peers — a socket-reachable Durable Object and a Postgres reached over
  one-shot HTTP with no interactive transaction and no cursor
  (`RUNBOOK.md:4591-4592`).
- **The byte-identical round trip**, which is what lifts ADR-0003's `provisional`
  marker against the ADR's own stated lift condition
  ([`.kb/decisions/0003-opaque-payloads.md`](../../../.kb/decisions/0003-opaque-payloads.md);
  the long-form record's lift condition is cited at
  `spec/SPECIFICATION.md:6168-6171`).
- **Envelope types on the frozen wire format**, version first
  (`crates/happenstance-sync/src/wire.rs`, `RUNBOOK.md:4586-4587`), and the message
  set the version has never named.
- **WF-1's interoperability half** — settled, or renewed against a *named*
  experiment, in ADR-0026's envelope section. Not silence
  (`RUNBOOK.md:4593-4595`).
- **Resolving, not deleting, the two open-question atoms phase 13 owns:**
  [`.kb/open-questions/sync-message-set-and-format-version.md`](../../../.kb/open-questions/sync-message-set-and-format-version.md)
  and
  [`.kb/open-questions/dcb-reference-publishes-no-wire-format.md`](../../../.kb/open-questions/dcb-reference-publishes-no-wire-format.md)
  (both indexed at `.kb/maps/open-questions-index.md:122-134`).
- **Correcting the specification passages whose words are now wrong** — by decision
  record where the clause is `[FROZEN]`, never by edit.
- **The `!Send` path exercised end to end** through the sync runner: ingest bound on
  `EventStore`, not `SendEventStore`, because the `!Send` peer sits mid-chain rather
  than at a leaf (`RUNBOOK.md:4588-4590`; CLAUDE.md binding constraint 4).

## Out of scope (this project)

Each exclusion names the sibling that owns it, per
[`../_decomposition.md`](../_decomposition.md) *Scope seams*.

- **Publishing `happenstance-sync` or `happenstance-sync-testkit` to the registry** —
  out of this release train by the charter's own *Out of scope* list; what ships and
  when is `publication-and-positioning`'s (HS-P0016). See the open question below on
  whether *claiming a name* is inside that exclusion.
- **What a store may forget, and how a reader finds out** — `retention-and-incomplete-logs`
  (HS-P0018), which owns BR-11, AC-14, DoD 15 and DT-7, and ADR-0028
  (`RUNBOOK.md:307`).
- **ES-39, ES-40 and the suffix store** — `retention-and-incomplete-logs`
  (`RUNBOOK.md:4645-4654`). SY-32 sits on the seam; see AC-011.
- **A DCB wire-interoperability bridge** — deferred, and on the stronger reason that
  the DCB specification and its reference library publish no wire format at all
  ([`.kb/open-questions/dcb-reference-publishes-no-wire-format.md`](../../../.kb/open-questions/dcb-reference-publishes-no-wire-format.md)).
  This project records the deferral's renewal; it does not build the bridge.
- **Amending any `[FROZEN]` clause.** Out of scope for the whole initiative. If this
  work needs one changed, that is a new decision atom and a re-plan, not a line edit.
- **Building the Durable Object and the Postgres/Neon stores themselves** —
  `cloudflare-durable-object-store` (HS-P0013) and `postgres-and-neon-stores`
  (HS-P0014). This project consumes them as peers; it does not write them.
- **The event-store and projection conformance suites** — `projection-store-freeze`
  (HS-P0010) owns the capability-declension policy this project's suite inherits.
- **Incidental defects found in passing** — they route to the `support` initiative
  per `.redkiln/config.yaml:5`.

## Derived requirements

Expanded from **BR-10** (*replication identity across a store boundary must receive
a written decision or an explicit reasoned refusal*) and from **BR-15** (*each answer
is a decision atom stating the alternatives that lost*), which the decomposition
applies inside every project rather than parking in one.

- **DR-1.** ADR-0026 and ADR-0027 are written and merged **before** the code they
  constrain, each answering exactly one question, each naming the alternatives that
  lost (`RUNBOOK.md:4606`, `RUNBOOK.md:264-268`).
- **DR-2.** The answer to *does ingest re-check the writer's asserted conditions* is
  reconciled against the specification before it is written, not after. Where this
  project's own prose and `spec/SPECIFICATION.md` disagree, the specification wins —
  the intake brief's own gate box says so
  ([`_intake-brief.md`](_intake-brief.md), Gate: Intake).
- **DR-3.** A refusal is a legitimate answer and a silence is not. If the outcome is
  a reasoned refusal, the artefact is the decision atom naming the alternatives that
  lost.
- **DR-4.** The suite is an instrument, not a decoration: every sync rule carries a
  registered wrong implementation that fails it and passes everything it does not
  (CF-1 – CF-3, `spec/SPECIFICATION.md:7161-7207`).
- **DR-5.** The suite never decodes `Event::data` or `Event::metadata`. A suite that
  parses a payload would certify a peer that does, which is the exact guarantee
  ADR-0003 exists to buy (`spec/SPECIFICATION.md:6153-6171`, `RUNBOOK.md:4576-4580`).
- **DR-6.** The port is proved by *spread*, not by count: three peers, at least one
  of which cannot hold a transaction open across a round trip
  (`RUNBOOK.md:4607-4608`).
- **DR-7.** No conformance rule asserts on a literal position value; every position
  assertion is anchored on a value the store under test assigned (CF-6,
  `spec/SPECIFICATION.md:7233-7249`; CLAUDE.md, *The rule that matters*).
- **DR-8.** A declined capability still runs and reports the fixture's stated reason;
  it never vanishes from the binary (BR-13's standard, exercised here, owned by
  `projection-store-freeze`).
- **DR-9.** Every clause this project touches is checked against the clause ledger,
  not against prose, and this project's stated clause range and the union of its
  ADRs' clause ranges are computed and compared at exit — the standing lesson from
  phase 4 (`RUNBOOK.md:334-336`).
- **DR-10.** Each open-question atom this project consumes is **resolved**, never
  deleted; the record that it was once open is itself worth keeping
  (`.kb/maps/open-questions-index.md:7-13`).

## Acceptance criteria

Project-grain and testable. This is the spine the story map must cover.

- **AC-001 — The decisions exist before the code.** ADR-0026 and ADR-0027 are
  accepted decision atoms under `.kb/decisions/`, authored through the ingest path,
  merged before the code they constrain, and each states the alternatives that lost.
  `redkiln validate --kb` passes over them.
- **AC-002 — The central question has a written answer.** Whether ingest re-checks
  the writer's asserted append conditions is answered in ADR-0026, and the answer is
  reconciled against SY-1 and SY-6 — both `[FROZEN]`
  (`spec/SPECIFICATION.md:5841-5867`, `:5973-6029`). Any disagreement with a frozen
  clause is raised as a new decision atom and a re-plan, and no frozen clause is
  edited in this project's diff.
- **AC-003 — The suite exists and no rule is silently absent.** `sync_peer_conformance!`
  ships in `happenstance-sync-testkit`, is emitted through the existing rule registry
  (`crates/happenstance-testkit/src/registry.rs`) so it inherits the tokio, blocking
  and `wasm32` harnesses, and a declined capability appears in the output with the
  fixture's stated reason rather than vanishing.
- **AC-004 — The suite discriminates.** Every sync rule has a registered mutant that
  fails it and passes every rule it does not declare, and every mutant carries a
  non-empty provenance string naming the real peer shape that makes it plausible
  (CF-1 – CF-4). The suite decodes no payload byte on any path.
- **AC-005 — One suite, three peers, two of them genuinely unlike.** The suite is
  green against `MemorySyncPeer`, a socket-reachable Durable Object peer, and a
  one-shot-HTTP Postgres peer that cannot hold a transaction open across a round
  trip.
- **AC-006 — A payload survives the boundary unchanged, and replay changes nothing.**
  The payload `Bytes` are asserted byte-identical end to end across a store boundary,
  replaying the same batch twice is observably a no-op, and ADR-0003's `provisional`
  marker is lifted against the ADR's own stated lift condition — or the reason it
  cannot be lifted is recorded.
- **AC-007 — Both topologies are first class.** Hub-and-spoke and peer-to-peer are
  each exercised, one adapter type serves both roles simultaneously on different
  edges (SY-9, `spec/SPECIFICATION.md:6090-6106`), and
  `crates/happenstance-sync/src/lib.rs`'s module documentation no longer describes
  only one.
- **AC-008 — The two headline rules are green with negative controls.**
  `ingest_never_rejects` and `compensation_is_atomic_with_the_losing_event` pass, each
  with a mutant that fails it by name (`RUNBOOK.md:4611-4612`).
- **AC-009 — The constrained runtime keeps its runtime.** The ingest path and the
  sync runner bind `EventStore`, not `SendEventStore`, and the whole path is built and
  exercised for `wasm32` inside the gate rather than asserted in prose.
- **AC-010 — No deferral survives without a named experiment.** Every `[DEFERRED]`
  `SY` clause is settled or renewed against a named experiment; a renewal with no
  experiment fails the gate under CF-38. WF-1's interoperability half is recorded in
  ADR-0026's envelope section — settled or renewed by name, never by silence.
- **AC-011 — SY-32's disposition is written down rather than assumed.**
  `spec/SPECIFICATION.md:7000-7003` states that SY-32 depends on ES-39 and cannot be
  settled ahead of it, and ES-39 is `retention-and-incomplete-logs`' under ADR-0028
  (`RUNBOOK.md:307`, `:4637`). This project records either (a) evidence that SY-32 is
  settleable without ES-39, and settles it, or (b) an explicit handoff to ADR-0028
  with the constraint stated — and the `replication → retention` DAG edge is
  re-checked against whichever answer lands.
- **AC-012 — The corrections go through decision records, not edits.** The
  specification passages whose named wrong implementation is `happenstance-sync`'s own
  superseded proposal are audited by id — SY-12 says so in its own text
  (`spec/SPECIFICATION.md:6173-6183`), and SY-1 and SY-6 name a still-public field on
  `EventGroup` (`crates/happenstance-sync/src/peer.rs`) as a live target. Each clause
  either keeps a wrong implementation that still exists, or is corrected by decision
  record.
- **AC-013 — The questions are resolved, not deleted.** Both phase-13-owned
  open-question atoms reflect a resolved state, the index at
  `.kb/maps/open-questions-index.md` is updated in place, and `redkiln validate --kb`
  passes.
- **AC-014 — The clause arithmetic is done.** This project's stated clause range and
  the union of ADR-0026's and ADR-0027's clause ranges are computed and equal at exit,
  every maturity marker is checked against `spec/SPECIFICATION.md` rather than against
  this charter, and `cargo xtask spec-trace` is green.
- **AC-015 — Nothing was published that the charter excluded.** No sync crate is
  released to the registry by this project, and the disposition on *claiming* the two
  names is recorded in the architecture brief rather than taken silently.

## Definition of done (boundary-level)

1. `cargo xtask ci --fast` is green on this project's tree — the bar
   `.redkiln/config.yaml:50-55` wires to a non-terminal project's integration grain.
   The whole gate is `closeout-and-durable-audience`'s, not this project's.
2. `cargo xtask lints && cargo xtask spec-trace` green
   (`.redkiln/config.yaml:48`), so no clause cites a rule that does not exist and no
   rule is orphaned.
3. Every story carries a `_ledger.md` with cited evidence per AC-###
   (`.redkiln/config.yaml:62-67`) — a green suite proves something works, never that
   the criteria this project was written to satisfy are the things that work.
4. **The proof artefact exists**, and it would not exist if the design were wrong:
   one suite green against three peers, two structurally unlike, with a
   byte-identical payload round-tripped across a store boundary
   (`RUNBOOK.md:4597-4602`). A phase is done when its proof artefact exists, not when
   the gate is green.
5. **DoD 14 is observable:** an accepted decision atom answers whether ingest
   re-checks a writer's asserted conditions — or explicitly refuses, with reasons —
   the corresponding open-question atoms reflect that resolution, and
   `redkiln validate --kb` passes.
6. `HS-P0018` is unblocked: SY-32's disposition (AC-011) is on disk and the handoff,
   if there is one, names ADR-0028.
7. No `[FROZEN]` clause was edited, and `git diff` over `spec/SPECIFICATION.md` shows
   only additions a decision record authorises.

## Dependencies

From the DAG at [`../_decomposition.md`](../_decomposition.md) *Sequencing*. The
frontmatter's `blocked_by` / `blocks` are the CLI's to write; this section is the
human-readable statement of the same edges.

**Depends on**

- **HS-P0016 `publication-and-positioning`** — ships `0.2.0`. This is *ordering*, not
  a technical prerequisite: `RUNBOOK.md:4526-4532` says nothing in phase 13 needs a
  crate to be on crates.io, and that a session reaching here with 12 outstanding
  should say so in the log rather than wait. The edge exists so that holding the
  release for replication — the sequence the whole plan rejects — cannot happen by
  drift.
- Transitively, the four adapter projects: this project's two unlike peers are
  `cloudflare-durable-object-store` (HS-P0013) and `postgres-and-neon-stores`
  (HS-P0014) wearing peer clothes, and both are `publication-and-positioning`'s
  blockers already. That is a **real** technical dependency, not just ordering, and
  it is the one to watch if HS-P0016 slips for a reason unrelated to them.

**Unlocks**

- **HS-P0018 `retention-and-incomplete-logs`** — needs SY-32, and needs the ingest
  path to exist before a suffix store can be shown to break one
  (`RUNBOOK.md:4658-4661`). See AC-011 for the caveat on which direction SY-32
  actually points.

## Risks and coupling notes

| Risk | Note |
| --- | --- |
| **The intake brief's clause ledger disagrees with the specification.** | The brief lists SY-1 – SY-7 and SY-19 – SY-31 as `[PROVISIONAL]`; SY-1 – SY-6, SY-8, SY-9 and SY-11 – SY-13 read `[FROZEN]` in `spec/SPECIFICATION.md` today (`:5848`, `:5886`, `:5909`, `:5937`, `:5957`, `:5981`, `:6074`, `:6095`, `:6138`, `:6157`, `:6213`). The brief also calls the wire clauses `VT-*`; they are `WF-*`. The intake gate's own last box settles the arbitration — the specification wins — and AC-014 is the mechanism. Re-derive the ledger before ADR-0026 is drafted, not after. |
| **SY-32 may be a DAG inversion.** | `RUNBOOK.md:4535` says phase 13 discharges SY-1 – SY-35; `RUNBOOK.md:307` and `:4637` assign SY-32 to phase 14 / ADR-0028; `spec/SPECIFICATION.md:7000-7003` says SY-32 depends on ES-39 and cannot be settled ahead of it. Three documents, two answers. AC-011 forces the disposition; it must not be settled by whichever document is read last. |
| **A store-side seam discovered late is a breaking change to a *published* port.** | `RUNBOOK.md:462-466` states this residual risk rather than assuming it away, and names its two scheduled mitigations: the `SyncPeer` sketch landed at phase 2, and ADR-0026 must be written against **two** unlike peers, not one. Both are live here. The seam stays out of `EventStore` — coherence is what makes that hold (`RUNBOOK.md:450-455`). |
| **"Claim the names on crates.io" versus "publish no sync crate".** | `RUNBOOK.md:4558-4560` carries a work item to claim both names; the charter's *Out of scope* forbids publishing either to the registry in this release train, and the decomposition repeats the exclusion. A name reservation is a placeholder publish. Disposition owed to the architecture brief (AC-015); do not resolve it in a commit message. |
| **A rule no peer can fail.** | The commonest failure mode in this repository's own history. DR-4 and CF-1 – CF-4 are the guard; before adding a rule, name the plausible wrong peer it rejects and write that peer into the testkit's own `tests/`. |
| **Two of the three peers are networked and cost real infrastructure.** | The Neon axis cannot be faked without destroying the thing it exists to test ([`../_decomposition.md`](../_decomposition.md), *Warranted briefs*). This project carries no `deployment` brief, so the infrastructure it consumes must already exist from HS-P0013 and HS-P0014 — confirm that at the architecture brief, not at implementation. |
| **The `!Send` flavour dropped under schedule pressure.** | It is a binding constraint (CLAUDE.md constraint 1 and 4), and here the `!Send` peer sits *mid-chain* rather than at a leaf, which is precisely the arrangement that tempts a `SendEventStore` bound. AC-009 is the tripwire. |
| **Settling an open question in passing.** | Two atoms are owned by this phase and both must be consumed rather than rediscovered. DR-10 and AC-013 hold the line; hand-authoring a `.kb/` atom outside the ingest path was reverted once already (`0269720`). |

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — BR-10, AC-13, DoD 14, and the *Out of
  scope* list this project sits inside
- [`../_decomposition.md`](../_decomposition.md) — the traceability matrix, the scope
  seams, and the DAG edge `publication → replication → retention`
- [`_intake-brief.md`](_intake-brief.md) — the approved problem, constraints, proof
  artefact and clause ledger

Knowledge base:

- [`.kb/decisions/0003-opaque-payloads.md`](../../../.kb/decisions/0003-opaque-payloads.md)
  — opaque payloads; its lift condition is this project's round trip
- [`.kb/decisions/0001-async-port-flavours.md`](../../../.kb/decisions/0001-async-port-flavours.md)
  — the two-flavour design the sync runner must not break
- [`.kb/decisions/0009-error-send-sync.md`](../../../.kb/decisions/0009-error-send-sync.md)
  — the `Error` bound, taken for the whole workspace including `SyncPeer`
  (`spec/SPECIFICATION.md:7006-7019`)
- [`.kb/open-questions/sync-message-set-and-format-version.md`](../../../.kb/open-questions/sync-message-set-and-format-version.md)
  — `FORMAT_VERSION = 1` names a message set that does not exist yet; three ordered
  sub-questions this project answers
- [`.kb/open-questions/dcb-reference-publishes-no-wire-format.md`](../../../.kb/open-questions/dcb-reference-publishes-no-wire-format.md)
  — WF-1's interoperability half and why the deferral is stronger, not weaker
- `.kb/maps/open-questions-index.md:122-134` — both atoms, and the rule that a
  resolved question is annotated rather than removed

Specification and plan:

- `spec/SPECIFICATION.md` §5 *The `SyncPeer` port* — §5.1 the bound decision, §5.3
  identity and idempotent ingest, §5.4 the transport floor, §5.8 replication scope,
  §5.10 retention across a peer set, §5.13 what the section does not decide
- `spec/SPECIFICATION.md:5841-5867` (SY-1), `:5973-6029` (SY-6), `:6070-6086` (SY-8),
  `:6153-6207` (SY-12), `:7000-7003` (SY-32's dependency on ES-39)
- `spec/SPECIFICATION.md:7161-7249` — CF-1 – CF-6, the suite's own proof obligation
- `RUNBOOK.md:4516-4620` — phase 13 in full: goal, the two structural decisions
  already settled, the work list, the proof artefact and the six exit criteria
- `RUNBOOK.md:440-466` — the leak warning, its three discharges, and the residual risk
- `RUNBOOK.md:305-307` — ADR-0026, ADR-0027 and ADR-0028 in the queue

Code:

- `crates/happenstance-sync/src/lib.rs` — the port, the topologies, and *the hard
  part, stated honestly* (`:98-133`)
- `crates/happenstance-sync/src/peer.rs` — `SyncPeer`, `EventGroup::guard`, the
  public field SY-1 and SY-6 name as the live wrong implementation
- `crates/happenstance-sync/src/ingest.rs` — `IngestStore`, and why
  `impl IngestStore for MemoryEventStore` cannot be written truthfully today
- `crates/happenstance-sync/src/identity.rs` — the `(StoreId, SequencePosition)` pair
- `crates/happenstance-sync/src/wire.rs` — the frozen envelope, version first
- `crates/happenstance-sync/src/memory.rs` — `MemorySyncPeer`, the oracle
- `crates/happenstance-testkit/src/registry.rs` — the one place rules are enumerated
- `.redkiln/config.yaml:40-67` — the gates this project is measured by

## Companions

- [`_intake-brief.md`](_intake-brief.md) — the approved intake for this project
- [`_decomposition.md`](_decomposition.md) — the warranted briefs (architecture,
  testing), authored by `plan-briefs`
- [`_storymap.md`](_storymap.md) — the vertical-slice story map, authored by
  `plan-briefs`
- [`../initiative.md`](../initiative.md) — the initiative charter
- [`../_plan.md`](../_plan.md) — the initiative-wide plan rollup
