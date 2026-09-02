---
item: HS-S0109
stage: spec
created: 2026-08-12T13:47:50.038Z
updated: 2026-08-12T13:47:50.038Z
template_sig: 87bbf1d0
rendered_sig: a7f94da8
---

# Spec — The runner keeps the constrained runtime its runtime

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/send-free-sync-runner/spec.md` |
| Key briefs | `…/replication-identity-and-ingest/_decomposition.md` — architecture *Composition root* §1–§7 (`:53-212`), *The Accepted atoms that constrain this* (`:214-256`), *Non-prescriptive implementation notes* (`:488-521`), AC-A06/AC-A07 (`:550-557`); testing brief *The test mix, tier by tier* → **Integration**, AC-009's `!Send` tripwire (`:731-748`) and *Merge-gate commands* (`:775-800`) |
| Signed-off design | `…/replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved 2026-08-12. No surface ids exist for this story to render; the public API surface obligation is discharged through the briefs and `standards/rust/70-rustdoc-obligations.md` |
| Story map row | `…/replication-identity-and-ingest/_storymap.md:66` (slice `runner-and-topologies`, row 1) and the merge order at `:132-133` |
| This story's discovery | `…/send-free-sync-runner/discover.md` — the signal ledger, the four deferrals, and the four named wrong implementations |
| Roadmap pointer | `RUNBOOK.md:4516-4620` (phase 13 in full); `RUNBOOK.md:4588-4590` (the `!Send` path end to end); `RUNBOOK.md:462-466` (the residual risk that lands here if a port change is needed) |

## One-line PR slice

Land the runner that fans out over peers and advances the owned resume token, bound on
`EventStore`/`SyncPeer`/`IngestStore` and never on a `Send` flavour (one name of each pair per
module), proved by a real `tokio::spawn` with the `!Send` peer sitting mid-chain rather than at a
leaf, and by the `wasm32` gate steps actually running it.

## Executive summary

`crates/happenstance-sync/src/lib.rs:42-50` has promised a runner since phase 2 — fan-out, ordering
between peers and reconciliation of disagreement live *above* the port, in the same division of
labour that puts the projection runner above `ProjectionStore` — and `crates/happenstance-sync/src/`
contains no such thing. SY-8 is `[FROZEN]` on that division (`spec/SPECIFICATION.md:6070-6086`), so
the port is not finished until the runner exists to hold what the port refused.

**The delta this PR lands** is that runner, plus the two instruments that make its bounds
falsifiable *here* rather than at HS-S0111. The predecessors have already put the substrate in
place: `ingest-store-and-memory-peer-round-trip` (HS-S0102) replaced the `todo!()` bodies so there
is a real `IngestStore` and a real `MemorySyncPeer` to fan out over, and
`gate-mounts-for-the-sync-suite` (HS-S0104) added the two `wasm32` steps — a build of
`happenstance-sync` and a check of the *sync* conformance harness — beside the four at
`xtask/src/main.rs:203-283`. This story is what makes those two steps mean something: a crate build
proves nothing about a generic function's `Send` bounds, because they are satisfied at
instantiation, so the runner has to be *instantiated* with a `!Send` peer inside the harness the
`wasm32` step checks.

Nothing else in this slice moves: `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110) owns
SY-10's directional merge rules and the module-doc correction, and this PR's obligation toward it is
negative — do not foreclose it.

## Context pack

Read this section before opening any file. Everything below is a decision already taken; the
anchors behind it are for retrieval, not for re-deliberation.

**1. The runner binds the weaker flavour, and this is the one place in the workspace most likely to
break that for a good reason.** SY-17 is `[FROZEN]` (`spec/SPECIFICATION.md:6341-6360`): the sync
port *and its runner* MUST be written against `EventStore`, not `SendEventStore` — and by CLAUDE.md
constraint 4 the same holds for `SyncPeer` over `SendSyncPeer` and `IngestStore` over
`SendIngestStore`. The natural implementation spawns a task per peer, `tokio::spawn` demands `Send`,
and the shortest path to a green compile is to bind the `Send` flavour of all three ports. SY-17's
`Rejects:` names exactly that runner, and the reason is the shape of the topology, not tidiness: in
Kestrel Cold Chain the `!Send` peer — SQLite inside a Cloudflare Durable Object — sits in the
**middle** of the chain, a spoke to the Neon estate store and a hub to 138 tablets, so a `Send`-bound
runner *excludes the hub from its own topology*, and "the exclusion is discovered when the adapter is
written, not when the runner is". Bind the bare flavour; import only one name of each pair per module
or method calls go ambiguous with `error[E0034]` (`crates/happenstance-sync/src/peer.rs:26-28`).

**2. The `Send` obligation belongs on the token, and never on the port.** `SyncPeer::Resume` is
`Clone + 'static` and deliberately not `Send`; the port documents this as a *gap rather than a
decision*: `trait_variant` marks the derived futures `Send` and leaves associated types alone, so a
caller that wants to `tokio::spawn` a per-peer task and carry the token out of it must write
`P::Resume: Send` **at its own bound**, and gets `E0277` when it does not
(`crates/happenstance-sync/src/peer.rs:95-118`). The decision this story takes: the runner's own
drive path carries **no** `Send` bound of any kind — not on the ports, not on the token — and any
spawning convenience is a separate, additively-bounded entry point whose extra bound is written on
the *token* (and on whatever the caller hands it), never by swapping a port bound for its `Send`
flavour. A `P::Resume: Send` written unconditionally on the runner is the same exclusion as the
broad bound, one type deeper: it would refuse a peer whose token is `Rc`-held.

**3. "Add a second peer" stays a runner configuration.** The port describes exactly one peer
relationship and says so at the trait (`crates/happenstance-sync/src/peer.rs:65-73`); fan-out is the
runner's, and adding a peer must not be a type change to the port or a breaking change for a caller.
Whether the runner is one type, a trait or a function is *open* by the architecture brief's own
non-prescriptive note (`…/_decomposition.md:493-498`) — settle it here, and record which alternative
lost, but the invariant above is not open.

**4. Hub-ness is an edge property, and this story must not foreclose SY-10.** SY-9 is `[FROZEN]`
(`spec/SPECIFICATION.md:6090-6106`): one adapter type must be usable simultaneously as a hub to one
set of peers and a symmetric peer to another, so the runner takes **no** role parameter, no
`is_hub`, and no two peer collections typed by role. SY-10 is `[PROVISIONAL]`
(`spec/SPECIFICATION.md:6110-6128`) — a different merge rule in each direction of one edge — and it
is `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110)'s to exercise. Build the shape ADR-0027
landed and no more.

**5. The resume token is owned, transferable, and survives the handle.** SY-16 is `[FROZEN]`
(`spec/SPECIFICATION.md:6323-6340`): resume state is a value the caller supplies on each call, never
a handle the peer holds; its `Rejects:` is an in-memory cursor, "which passes every test written
against a process that stays alive and fails the deployment the crate exists for". The runner is
therefore what hands the token *back out* — a token that only ever lives inside a long-lived per-peer
task is the same defect wearing a runner's clothes, and at the edge the normal termination path is
the handle going away.

**6. The runner may not put rejection back above the port.** SY-1 forbids refusing an ingested event
as a function of local state and SY-4 forbids quarantine outright
(`spec/SPECIFICATION.md:5932-5941`). A runner that parks a batch, retries it, or back-pressures a
peer whose events "conflict" never calls a conditional append, so `ingest_never_rejects` stays green
while the rejection has simply moved to where no clause is looking. Transport failure is retryable;
disagreement with content is not a state this runner may hold.

**7. The proof is an instantiation, not a build — and it takes two shapes, not one.** This workspace
has already shipped the wrong proof once: an assertion on a *concrete* stream that passed by
auto-trait leakage whatever the trait said. The replacement writes the bound at the definition
(`crates/happenstance-core/src/memory.rs:614`) and is still not sufficient on its own — 
`spawns_from_generic` (`crates/happenstance-core/src/memory.rs:643`) is what rejects the refactor,
because it holds the value across an await inside a real `tokio::spawn`, and RS-62-3 states the
general form: *a green `#[tokio::test]` says nothing about `Send`*
(`standards/rust/62-doctests-and-harnesses.md:128`). The sync version needs the same pair with the
`!Send` node **mid-chain**: a spawned task for a `Send`-typed edge that carries the token out across
an await, and a real spawn of a `!Send` chain (which only `spawn_local` on a `LocalSet`, or a
current-thread runtime, can accept) whose middle node holds both its peer and its store behind an
`Rc`. A `fn assert_send<T: Send>()` on a concrete runner future is not admissible evidence here.

**8. The `wasm32` gate steps are the deferred-failure catcher, and they are only as good as what
they compile.** `gate-mounts-for-the-sync-suite` added a `wasm32` build of `happenstance-sync` and a
`wasm32` check of the sync conformance harness beside `xtask/src/main.rs:203-283`'s four. A library
crate that never instantiates the runner with a `!Send` type compiles happily for any target — so
the harness the second step checks must contain a real instantiation, or AC-009's sentence *"built
and exercised for `wasm32` inside the gate rather than asserted in prose"* is false while the gate is
green.

**9. The persona-journey slice.** The consumer this story serves is the backbone's A5 — *run a
fleet* (`…/_storymap.md:44`) — an application author who has one store, one peer, and then a second
peer, and who must be able to add the second without a type change and without discovering, three
crates later, that the runner never admitted their runtime. The observable outcome is a fleet that
runs on a runtime with no threads at all.

**10. What is deliberately *not* proved here.** No conformance rule is added: SY-8 says every rule in
`happenstance-sync-testkit` takes exactly one peer handle and a rule needing two would be testing the
runner (`spec/SPECIFICATION.md:6074-6077`), so runner behaviour is integration-tier in
`happenstance-sync`'s own `tests/`. SY-17 is the exception and it is discharged by a **compile** in
the testkit rather than by a rule.

## Integration contract

**Archetype** — `capability`. A user-observable slice through the port, the runner and the gate.

**Slice / milestone** — `runner-and-topologies`. Slice-mates:
`hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110), implemented in the same context and mounted
as one surface; this story merges first (`…/_storymap.md:132-133`).

**Mount point** — `crates/happenstance-sync/src/lib.rs`. This is the crate's composition root: it is
where `pub mod …;` and the crate-root `pub use` list live (`:142-159`), and it is the file whose
module documentation has claimed a runner exists above the port since phase 2 (`:42-50`). The runner
is not landed until it is declared and re-exported here — a module reachable only from a test is the
"constructed but unmounted" failure this contract exists to prevent. Two secondary mounts are part of
the same wiring and are not scope drift: `xtask/src/main.rs`'s `wasm32` step list (the gate mount
that decides whether AC-009's sentence is true, `…/_decomposition.md:176-183`) and
`crates/happenstance-sync-testkit/` (created by HS-S0103), which is where SY-17's `!Send` fixture
peer and the harness instantiation live.

**Wires into** — real sibling contracts, all of them already in-tree at this point in the merge
order:

- `crates/happenstance-sync/src/peer.rs` — `SyncPeer` (bare flavour), `Pulled<R>`, `PushBatch`,
  `EventGroup`, `Ack`, `PeerLimits`, `SyncError`, and `SyncPeer::Resume`'s ownership contract.
- `crates/happenstance-sync/src/ingest.rs` — `IngestStore` (bare flavour): `ingest`, `holds`,
  `watermark`, `store_id`, with real bodies from HS-S0102.
- `crates/happenstance-sync/src/identity.rs` — `EventId`, `ReplicatedEvent`, `Watermark`; the only
  identity that crosses the boundary, and what the tests key their assertions on.
- `crates/happenstance-sync/src/memory.rs` — `MemorySyncPeer`, `MemoryResume`, `MemoryPeerError`:
  the oracle, and a peer that implements `SendSyncPeer` (`:162-164`) and therefore *cannot on its own*
  prove anything about the `!Send` path.
- `happenstance-core`'s `EventStore` (bare flavour) and `MemoryEventStore`, plus the additive
  inherent ingest door HS-S0101 added.
- `crates/happenstance-sync-testkit/` — `Capability`/`RuleOutcome` reuse and the fixture trait from
  HS-S0103; this story adds the `!Send` fixture peer SY-17's `Rule:` field names.
- `tokio` is already a dev-dependency of `happenstance-sync` with `macros`, `rt` and
  `rt-multi-thread` (`crates/happenstance-sync/Cargo.toml`), so the spawn tests need no manifest
  change beyond what a `LocalSet` requires.

**Renders surfaces** — **none.** `…/_design.md` records *"N/A — no user-facing surface"* and the
sign-off approves that determination (`:76-125`). There is no surface id to claim. The API-surface
obligation this project owes instead is rustdoc and a compiled example on every new public item
(`standards/rust/70-rustdoc-obligations.md`), carried as an AC rather than as a footnote.

**Conformance rule(s)** — **none added, deliberately.** SY-8 keeps runner behaviour out of a suite
whose every rule takes one peer handle (`spec/SPECIFICATION.md:6070-6086`). SY-17's obligation is
discharged by `happenstance-sync-testkit` compiling its own suite against a `!Send` fixture peer
holding a `!Send` store behind an `Rc` — a compile, in the testkit, native and `wasm32`.

**Clause(s)** — discharges SY-8 (the runner the clause presupposes now exists), SY-16 (the token the
runner hands back out) and SY-17 (the bound, and its instrument). Constrained by SY-1, SY-4, SY-9 and
SY-19; foreclosing none of SY-10. **No `[FROZEN]` clause is edited.** If the `Resume` gap turns out
to need a *port* change rather than a runner-side bound, that is `RUNBOOK.md:462-466`'s residual risk
landing on a frozen clause: stop, record the finding, raise a new decision atom and a re-plan — never
a `Send` flavour quietly bound one level up.

**Advances DoD scenario** — initiative **DoD 4**, *the constrained-runtime target is exercised in the
gate rather than asserted in prose* (`.bklg/from-contract-to-published-library/initiative.md:369-372`),
applied to the replication path: the runner is the last piece of that path that could exclude the
`wasm32` runtime, and it is a hard precondition for the project's own DoD 4 proof artefact — one
suite green against three peers, one of which is the `!Send` Durable Object peer HS-S0111 writes
(`project.md`, *Definition of done* 4).

## PR boundary

**In this PR**

- The runner in `crates/happenstance-sync/src/` — its shape settled here, declared and re-exported
  from `crates/happenstance-sync/src/lib.rs`, bound on `EventStore`/`SyncPeer`/`IngestStore` with no
  `Send` flavour and no `Send` bound on the drive path; fan-out over N peers as configuration;
  per-peer resume tokens owned by the runner's caller and handed back out.
- The optional, additively-bounded spawning entry point (if one lands), with its `Send` obligation
  written on the token rather than on any port bound, and the demonstration that removing it leaves
  the runner usable.
- The two native instruments in `crates/happenstance-sync/tests/`: the `Send`-typed edge driven
  inside a real `tokio::spawn` with the token carried out across an await, and the A—B—C chain of
  `spec/SPECIFICATION.md:6544-6552` with the **middle** node's peer and store `Rc`-held and driven
  inside a real local spawn.
- The `!Send` fixture peer (peer + store behind an `Rc`) in `crates/happenstance-sync-testkit/`, and
  the harness instantiation that makes the `wasm32` check of the sync harness compile a real runner
  rather than an empty crate.
- Rustdoc on every new public item, `# Errors` sections naming conditions, and a doctest showing one
  exchange over two peers.
- Documentation corrections in `crates/happenstance-sync/src/lib.rs` and `peer.rs` limited to
  sentences this PR makes false — and, per the architecture brief's standing warning
  (`…/_decomposition.md:590-599`), a finding is moved into the ADR that consumed it before the
  paragraph holding it is deleted.
- This story's own backlog folder (`_ledger.md`, the implementation report).

**Explicitly not in this PR**

- Any conformance rule, mutant registry entry or `CHANGELOG.md` rule entry — SY-8, and
  `headline-rules-and-mutant-registry` (HS-S0105) owns the rules that do land.
- Both topologies exercised, directional merge rules, and `lib.rs`'s topology module-doc rewrite —
  `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110), AC-007.
- The message set, `FORMAT_VERSION`'s disposition, and any derive on `PushBatch`/`EventGroup`/
  `ReplicatedEvent` — slice 4; the runner must not depend on it (the two slices have no edge and may
  interleave, `…/_storymap.md:140`).
- Real Durable Object and Neon peers — `durable-object-and-neon-peers` (HS-S0111).
- Any change to `EventStore`'s or `SyncPeer`'s trait signature, and any edit to a `[FROZEN]` clause.
- Gate-constant edits already owned by HS-S0104 (`RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST`);
  this PR consumes them and only adds a harness for the existing `wasm32` steps to compile.

```
crates/happenstance-sync/src/**
crates/happenstance-sync/tests/**
crates/happenstance-sync-testkit/**
.bklg/from-contract-to-published-library/replication-identity-and-ingest/send-free-sync-runner/**
```

**Merge DoD (one line).** `cargo xtask lints && cargo xtask spec-trace`, the sync crates' tests, and
`cargo xtask ci --fast` (which contains the `wasm32` steps) are green, and the `Send`-bound mutant of
this runner fails to compile in `happenstance-sync-testkit` rather than in a crate HS-S0111 does not
own.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Fan-out is the runner's, not the port's** | The runner drives N peer relationships from one configuration value. Adding a second peer changes a value, never a type on the port and never a caller's generic bounds. Whether the runner is a type, a trait or a function is settled in this PR and the losing alternative recorded. | `crates/happenstance-sync/src/peer.rs:65-73`; `crates/happenstance-sync/src/lib.rs:42-50`; `spec/SPECIFICATION.md:6070-6086` (SY-8); `…/_decomposition.md:493-498` |
| **Every generic bound is the weaker flavour** | `EventStore`, `SyncPeer`, `IngestStore`. No `SendEventStore`, `SendSyncPeer` or `SendIngestStore` appears in any bound on the runner or the ingest path, and no module imports both names of a pair — the import discipline is a compiler error (`error[E0034]`), not a preference. | `spec/SPECIFICATION.md:6341-6360` (SY-17); CLAUDE.md constraint 4; `crates/happenstance-sync/src/peer.rs:26-28`; `…/_decomposition.md:554-557` (AC-A07) |
| **No `Send` bound on the drive path, including the token** | The runner's drive path carries no `Send` bound at all. A spawning helper, if it exists, is a separate item whose additional bound is `P::Resume: Send` (plus whatever the caller hands it) — never a port bound swapped for its `Send` flavour. An unconditional `Resume: Send` on the runner is the same exclusion one type deeper. | `crates/happenstance-sync/src/peer.rs:95-118`; `.kb/decisions/0001-async-port-flavours.md`; `.kb/decisions/0009-error-send-sync.md` (nothing gains `+ Send + Sync` on the way past) |
| **The `!Send` node sits mid-chain and is really spawned** | A—B—C, A and C never communicate; B's peer *and* B's store are held behind an `Rc` and are therefore `!Send`; the runner drives B in both directions inside a real local spawn, holding the resume token across an await. Not `assert_send`, not a `#[tokio::test]` that never spawns. | `spec/SPECIFICATION.md:6544-6552` (SY-24's topology, borrowed as the arrangement); `crates/happenstance-core/src/memory.rs:643` (`spawns_from_generic`); `standards/rust/62-doctests-and-harnesses.md:128` (RS-62-3) |
| **The `Send`-capable caller loses nothing** | The same runner instantiated with `Send` types is driven inside a real `tokio::spawn` and its advanced token carried out of the task. This is what shows the weaker bound costs the thread-capable deployment nothing — and it is where a caller-side `Resume: Send` is written and discharged. | `crates/happenstance-core/src/memory.rs:614,643`; `crates/happenstance-sync/Cargo.toml` (tokio `rt-multi-thread` already present) |
| **Resume tokens are owned and handed back out** | One token per peer relationship, opaque, `Clone`, supplied on each call and returned by the runner to its caller — including when the batch is empty, which means "caught up for now" and not "start over". Dropping and reconstructing a peer handle between two exchanges resumes with no gap and no duplicate. | `spec/SPECIFICATION.md:6323-6340` (SY-16); `crates/happenstance-sync/src/peer.rs:95-118`; `crates/happenstance-sync/src/memory.rs:26-42` (`MemoryResume`) |
| **Progress is asserted on identity, never on a position** | Tests assert token advancement and receiver contents keyed by `EventId`; no literal position value, and no expression relating two peers' positions — none can be constructed. | `spec/SPECIFICATION.md:6415-6428` (SY-19); `crates/happenstance-sync/src/lib.rs:100-104`; CLAUDE.md, *The rule that matters* |
| **No rejection above the port** | Transport failure may be retried; content disagreement may not become quarantine, parking or per-peer back-pressure. The runner holds no holding area and no conflict state. | `spec/SPECIFICATION.md:5932-5941` (SY-4); `spec/SPECIFICATION.md:5841-5867` (SY-1); `crates/happenstance-sync/src/ingest.rs:140-160` (ingest's own refusal contract) |
| **Hub-ness stays an edge property** | No role parameter, no `is_hub`, no two peer collections typed by role; one adapter type usable in both roles on different edges. SY-10's directional merge rules are left to HS-S0110 and are not foreclosed. | `spec/SPECIFICATION.md:6090-6106` (SY-9); `spec/SPECIFICATION.md:6110-6128` (SY-10); `crates/happenstance-sync/src/peer.rs:76-83` |
| **`wasm32` compiles a runner, not an empty crate** | The sync conformance harness instantiates the runner with the `!Send` fixture peer, so the `wasm32` harness check added by HS-S0104 fails on a `Send`-bound runner. A crate build alone cannot: a generic function's `Send` bounds are satisfied at instantiation. | `xtask/src/main.rs:203-283` (the four existing steps and their stated reasoning); `…/_decomposition.md:176-183`; `discover.md`, *The wrong implementation*, `SendBoundRunner` |
| **The surface is documented as designed** | Every new public item carries rustdoc; every fallible public function carries an `# Errors` section naming conditions rather than the error type; a doctest shows one exchange over two peers and is run by the gate. `happenstance-sync-testkit` is `publish = false`, so its doctests need RS-62-5's out-of-package harness rather than `cargo test --doc`. | `standards/rust/70-rustdoc-obligations.md`; `standards/rust/62-doctests-and-harnesses.md`; `…/_decomposition.md:713-723` |
| **The findings survive the prose they lived in** | Sentences in `lib.rs`/`peer.rs` that this PR makes false are corrected, and any *finding* they carry is moved into the ADR that consumed it before the paragraph is deleted. | `…/_decomposition.md:590-599`; `crates/happenstance-sync/src/lib.rs:106-133` |

## Data and migrations

**N/A.** This story introduces no storage schema, no on-disk format and no wire-format change.

Three near-misses, stated so they are not mistaken for the absence of the section's subject:

- **The resume token is not data this story defines.** `SyncPeer::Resume` is an opaque associated
  type owned by the adapter (`crates/happenstance-sync/src/peer.rs:95-118`); the runner carries it,
  clones it and hands it back, and never inspects or serialises it. Giving it a representation here
  would be a wire decision, and the wire is slice 4's.
- **No message-set or `FORMAT_VERSION` movement.** `crates/happenstance-sync/src/wire.rs` is
  untouched; `PushBatch`, `EventGroup` and `ReplicatedEvent` gain no derives, because a `#[derive]`
  on a public message type *is* a wire format and belongs to ADR-0027 by name
  (`crates/happenstance-sync/src/lib.rs:86-94`).
- **No store-side migration.** The ingest door into `MemoryEventStore` is HS-S0101's additive
  inherent method, already merged; this story calls `IngestStore` and adds nothing to
  `happenstance-core` (`…/_decomposition.md:528-532`, AC-A01).

## Acceptance criteria

Each criterion is written from the intent of a persona this initiative names, crossing the whole
slice rather than one function. The consumers here are the **application author** running a fleet
(journey *Choose a contract before a database*, backbone A5, `…/_storymap.md:44`), the
**constrained-runtime developer** (journey *Event-source at the edge without hand-rolling it*), and
the **adapter author** who writes the Durable Object peer two stories later (journey *Learn when you
are finished*) — all three carried from
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` and
named at `.bklg/from-contract-to-published-library/initiative.md:227-250`.

Test paths below are the paths the implementer creates; only the anchors are required to exist
today.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **Adding a peer is a configuration change, not a migration.** GIVEN an application author running one store against one peer, WHEN they add a second peer, THEN the change is a value handed to the runner — no type on `SyncPeer`/`IngestStore`/`EventStore` changes, no generic bound in the author's own code changes, and the first peer's exchange behaviour is identical before and after. Fan-out, ordering between peers and reconciliation live in the runner (SY-8), never on the port. | `crates/happenstance-sync/tests/runner_fans_out.rs::second_peer_is_a_configuration_change` — drive one peer, capture the receiver contents keyed by `EventId`; add a second peer to the same runner value; assert the first peer's delivered set is unchanged and the second's arrives, with the test's own bounds untouched between the two arrangements. |
| AC-002 | **The runner admits the constrained runtime it was built for.** GIVEN a constrained-runtime developer whose store, peer and error types are all `!Send`, WHEN they instantiate the runner and drive an exchange, THEN it compiles and runs with no `Send` obligation anywhere on the drive path: no `SendEventStore`, `SendSyncPeer` or `SendIngestStore` in any bound on the runner or the ingest path, no `Resume: Send`, no `+ Send + Sync` added to any error on the way past, and no module importing both names of a flavour pair. | `crates/happenstance-sync/tests/send_free_bounds.rs::the_drive_path_names_no_send_flavour_and_no_send_bound` — a source-level assertion over the runner module and the ingest path (AC-A07's two greppable conditions, `…/_decomposition.md:554-557`), backed at compile time by AC-003's instrument, which cannot build if any of those bounds exists. |
| AC-003 | **The `!Send` node is in the middle of the fleet, and it really runs there.** GIVEN the Kestrel arrangement — A—B—C where A and C never communicate and B is a Durable-Object-shaped node whose *peer and store are both held behind an `Rc`* — WHEN the runner drives B in both directions inside a real local spawn (`LocalSet` or a current-thread runtime) holding the resume token across an await, THEN both edges exchange and B's receiver holds A's and C's events keyed by `EventId`. A `fn assert_send<T: Send>()` on a concrete future, or a `#[tokio::test]` that never spawns, does not satisfy this criterion. | `crates/happenstance-sync/tests/mid_chain_not_send.rs::drives_an_rc_held_middle_node_inside_a_real_local_spawn` — the arrangement of `spec/SPECIFICATION.md:6544-6552`, the instrument shape of `crates/happenstance-core/src/memory.rs:643` (`spawns_from_generic`), and RS-25-5's rule that the `!Send` instrument is built out of `Rc` and never out of an inherited `unsafe impl Send`. |
| AC-004 | **The thread-capable deployment loses nothing for it.** GIVEN an application author on a multi-thread tokio runtime with `Send` types throughout, WHEN they use the spawning entry point to run a peer edge in a real `tokio::spawn` and carry the advanced resume token back out of the task, THEN it compiles and the token is usable in the next exchange — and the only extra bound written to make that possible is on the **token** (`P::Resume: Send`, plus whatever the caller hands the task), never a port bound swapped for its `Send` flavour. Deleting the spawning entry point leaves the runner fully usable. | `crates/happenstance-sync/tests/spawned_edge_carries_the_token_out.rs::spawns_from_generic_and_carries_the_resume_token_out` — bound written at the definition (RS-21-2's `where` form, `standards/rust/21-send-is-not-inherited.md:74`), value held across an await inside a real `tokio::spawn` (RS-62-3, `standards/rust/62-doctests-and-harnesses.md:128`). |
| AC-005 | **The `wasm32` gate compiles a runner, not an empty crate.** GIVEN the adapter author who will write the Cloudflare peer at HS-S0111, WHEN they run `cargo xtask wasm` (and therefore `cargo xtask ci --fast`) on this tree, THEN the sync conformance harness that step checks contains a real *instantiation* of the runner with a `!Send` fixture peer holding a `!Send` store behind an `Rc` — so a `Send`-bound runner fails to compile **here**, in a crate this project owns, rather than being discovered in a crate HS-S0111 does not own. A crate build alone does not satisfy this criterion: a generic function's `Send` bounds are satisfied at instantiation. | `crates/happenstance-sync-testkit/tests/runner_instantiation.rs` for the native leg plus the harness instantiation reached by `xtask`'s `wasm32` check of the sync harness (`xtask/src/main.rs:203-283`, extended by HS-S0104); the fixture peer lives at `crates/happenstance-sync-testkit/src/fixtures/`. Evidence is a green `cargo xtask wasm` **and** the named instantiation site. |
| AC-006 | **A fleet resumes after the process that ran it went away.** GIVEN an edge deployment whose normal termination path is the handle disappearing — a Worker cancelled mid-flight, a Durable Object evicted, a tablet losing signal — WHEN the runner completes an exchange, hands the per-peer resume token back out to its caller, the peer handle is dropped and reconstructed, and the token is supplied to a fresh exchange, THEN replication continues with **no gap and no duplicate**. The token is owned by the caller, opaque, `Clone`, and never inspected or serialised by the runner. | `crates/happenstance-sync/tests/resume_survives_a_dropped_handle.rs::runner_hands_the_token_out_and_resumes_with_no_gap_and_no_duplicate` — SY-16's shape (`spec/SPECIFICATION.md:6323-6340`); assertions keyed on `EventId` and on the receiver's held set, never on a position value. |
| AC-007 | **"Caught up" does not mean "start over".** GIVEN a fleet that has replicated everything currently available, WHEN a peer's `pull` returns an empty batch, THEN the runner still returns the peer's token to its caller, and the next exchange resumes from it rather than from `None` — an empty exchange delivers nothing and re-delivers nothing. | `crates/happenstance-sync/tests/resume_survives_a_dropped_handle.rs::an_empty_pull_returns_the_token_to_use_next` — drive to quiescence, exchange again, assert the receiver's held set is unchanged and the returned token is the one accepted by the following exchange (`crates/happenstance-sync/src/peer.rs:120-135`). |
| AC-008 | **Two stores that have both committed converge; nothing is held back.** GIVEN a receiver whose local state disagrees with an incoming group, WHEN the runner drives that exchange, THEN the events are ingested — the runner holds no quarantine, no parking area, no per-peer conflict state and no content-triggered back-pressure. A **transport** failure may be retried and is surfaced to the caller with the peer it belongs to; disagreement with content is not a state this runner may hold. | `crates/happenstance-sync/tests/no_rejection_above_the_port.rs::transport_failure_retries_but_content_is_never_parked` — a peer that fails transport once then succeeds, and a receiver whose local state disagrees; assert the disagreeing group lands and that no runner-side collection retains it (SY-1, SY-4, `spec/SPECIFICATION.md:5932-5941`). |
| AC-009 | **One node is a hub and a spoke at the same time, and the runner never asks which.** GIVEN one adapter type deployed as a spoke to an estate store and a hub to many tablets, WHEN it is wired into the runner, THEN it is configured per **edge** — the runner exposes no role parameter, no `is_hub`, and no two peer collections typed by role — and nothing in the runner's shape prevents HS-S0110 from later attaching a different merge rule to each direction of one edge (SY-10). | `crates/happenstance-sync/tests/roles_are_edges.rs::one_type_drives_both_edges_without_a_role_parameter` — instantiate one peer type on two edges of the same runner value; assert both exchange, and assert by inspection in review that the runner's public items carry no role-shaped parameter (SY-9, `spec/SPECIFICATION.md:6090-6106`). |
| AC-010 | **A reader of the docs can run the thing without reading the source.** GIVEN an application author meeting the runner for the first time on docs.rs, WHEN they read its rustdoc, THEN every new public item is documented, every fallible public function carries an `# Errors` section naming the **conditions** rather than the error type, and a compiled doctest shows one exchange over two peers end to end — and that doctest is executed by the gate rather than merely present. Any sentence in `lib.rs`/`peer.rs` this PR makes false is corrected, and any *finding* it carries is moved into the ADR that consumed it before the paragraph is deleted. | `cargo test -p happenstance-sync --doc` for the doctest; `cargo xtask ci --fast`'s docs step with `-D warnings` for the rustdoc obligations (`standards/rust/70-rustdoc-obligations.md`); for any doctest that ends up in `happenstance-sync-testkit` (`publish = false`), RS-62-5's out-of-package harness rather than `cargo test --doc` (`standards/rust/62-doctests-and-harnesses.md`). |

**Coverage of the traced project AC.** Project **AC-009** — *"the ingest path and the sync runner
bind `EventStore`, not `SendEventStore`, and the whole path is built and exercised for `wasm32`
inside the gate rather than asserted in prose"* (`project.md:222-224`) — is covered in both halves:
the bounds by AC-002 (and its architecture-grain twin AC-A07), the *exercised in the gate* half by
AC-003 (native, mid-chain, really spawned) and AC-005 (the `wasm32` harness compiling a real
instantiation). Neither half alone discharges it, which is why they are separate rows.

## Interaction quality

**This story renders no surface, and that is a signed-off determination rather than an omission.**
`…/_decomposition.md`'s sibling `_design.md` records *"N/A — no user-facing surface"* across Items,
Signatures, Placement, States, Anti-patterns and The doctest, and the sign-off approves **the
no-surface determination itself** (`…/_design.md:76-125`). There is no route, DOM node, TUI pane or
screenshot here, so the **COMPOSITION** family — presentation exists at all, placement, transience,
density budget, hierarchy, the design's named anti-patterns — has no signed-off design to bind to
and no numbers to budget against. Nothing is silently dropped: the obligation the design stage
transfers in its place is the **public API surface**, and it is carried as AC rows, not as prose.

The two families, translated to this medium, and where each lives as a table row:

**STATE family (library analogues).**

- *In-place rather than a context jump* — AC-001. Adding a peer changes a value the author already
  holds; it does not relocate them into a different type, a different trait bound or a rewrite of
  their call sites. This is the row that would fail if fan-out were expressed by widening the port.
- *Non-occlusion* — AC-002 and AC-008. The runner does not sit on top of the port and hide it: it
  adds no bound the port did not have (AC-002) and no holding area the port refused to give it
  (AC-008). A runner that quarantines occludes `ingest`'s own contract from the caller's view.
- *Preserved state across a context change* — AC-006 and AC-007. The resume token is the state; it
  survives the handle being dropped and reconstructed with no gap and no duplicate (AC-006), and an
  empty exchange preserves it rather than resetting it (AC-007). `HeldCursorRunner` — the mutant
  named in `discover.md` — is exactly the loss of preserved state, invisible in a process that
  stays alive.
- *Reversibility* — AC-004. The spawning entry point is additive and removable: deleting it leaves
  the runner usable, so a caller who takes the convenience is never trapped by its extra bound.
- *Reachability* — AC-005 and AC-010. Reachable from the runtime that needs it (the `wasm32`
  harness instantiates it) and reachable from the documentation (a compiled doctest showing one
  exchange over two peers, run by the gate).

**COMPOSITION family (the API-surface obligation the design transferred).**

- *Presentation exists at all* — AC-010. The library analogue of "a control with real composed
  presentation, not bare markup" is a public item with real rustdoc, an `# Errors` section naming
  conditions, and a compiled example. A runner that type-checks with no docs is the unstyled render:
  it satisfies every structural assertion and is unusable.
- *Placement* — AC-001, via the Integration contract's mount point: declared and re-exported from
  `crates/happenstance-sync/src/lib.rs`'s composition root, not reachable only from a test.
- *Named anti-patterns* — carried from `discover.md`'s four wrong implementations and each bound to
  a row: `SendBoundRunner` → AC-002/AC-005, `AssertSendRunner` → AC-003/AC-004, `HeldCursorRunner` →
  AC-006, and the rejection-above-the-port mutant → AC-008.

No invariant in this section exists only here; every one is a row above, so `redkiln verify` gets a
ledger line for each.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | One peer's `pull` or `push` returns the adapter's `Error` mid-fan-out. | The other peers keep progressing; the failure is surfaced to the caller identified with the peer it belongs to; that peer's resume token is not advanced past what was actually ingested. The error crosses the runner **unchanged in its bounds** — nothing gains `+ Send + Sync` on the way past (`.kb/decisions/0009-error-send-sync.md`, RS-21-3 at `standards/rust/21-send-is-not-inherited.md:169`), because that is precisely what would exclude the Durable Object peer whose error wraps an `Rc<str>` (`crates/happenstance-sync/src/peer.rs:82-90`, `crates/happenstance-sync/tests/real_peer_shapes.rs`). |
| EC-002 | A batch would exceed what the far side can accept. | The runner consults `SyncPeer::limits()` — which is deliberately **not** `async` — *before* pushing, and splits or declines accordingly. Discovering a limit at ingest is discovering it after the write has already committed somewhere else, which is the one moment nothing useful can be done about it (`crates/happenstance-sync/src/peer.rs:152-160`). |
| EC-003 | `IngestStore::ingest` returns an error for a group. | Group atomicity is the store's; the runner surfaces the error and does **not** park, retry-forever or quarantine the batch, and does not advance the peer's token past the failed group. Re-driving the same exchange later is permitted and must be an observable no-op for anything already held (`crates/happenstance-sync/src/ingest.rs:140-175`). |
| EC-004 | A caller supplies a resume token a peer no longer recognises. | The runner passes it through opaquely and returns the adapter's own error. It does not inspect, repair, or fall back to `None` — silently restarting from the beginning is the duplicate-delivery failure AC-006 exists to forbid, wearing a recovery's clothes. |
| EC-005 | A caller reaches for the spawning entry point with a `!Send` resume token. | `error[E0277]` at the caller's own bound, with a message that points at `P::Resume`, not at the store or the peer. This is the documented gap at `crates/happenstance-sync/src/peer.rs:107-118` behaving as documented; a compile-fail case records the diagnostic so a future refactor that "fixes" it by widening a port bound is caught. |
| EC-006 | The `Resume: Send` gap turns out to need a **port** change rather than a runner-side bound. | Stop. This is `RUNBOOK.md:462-466`'s residual risk landing on a `[FROZEN]` clause: record the finding, raise a new decision atom and a re-plan. Never bind a `Send` flavour one level up to make it compile. |

## Non-functional

| id | requirement | check |
| --- | --- | --- |
| NF-001 | Builds on the MSRV and on `wasm32-unknown-unknown`; no new third-party dependency enters `happenstance-sync`'s non-dev graph. `tokio` stays a dev-dependency, and the local-spawn tests use `rt` features already declared (`crates/happenstance-sync/Cargo.toml`). | `cargo xtask ci --fast`; CI's `msrv` job; `standards/rust/50-dependency-hygiene.md`. |
| NF-002 | No assertion anywhere in this PR names a literal position value, and no expression relates two peers' positions — SY-19 says none can be constructed (`spec/SPECIFICATION.md:6415-6428`). Progress is asserted on resume-token advancement and on receiver contents keyed by `EventId`. | `cargo xtask lints` (the position-literal lint, taught about the sync suite by HS-S0104); review of the diff. |
| NF-003 | `happenstance-core` is untouched: `EventStore`'s trait signature is byte-identical before and after (AC-A01), and `publish = false` still reads `false` on both sync crates (AC-A10). | `git diff` over `crates/happenstance-core/src/store.rs`; `cargo package --list` assertions already in the gate. |
| NF-004 | `cargo clippy --workspace --all-targets -- -D warnings` is clean with no new `#![allow]` and no re-introduction of the scoped `#![allow(clippy::todo)]` that HS-S0102 removed (`crates/happenstance-sync/src/lib.rs:135-139`). | `cargo xtask ci --fast`. |
| NF-005 | No `unsafe` is added. The `!Send` instrument is constructed from `Rc` (RS-25-5, `standards/rust/25-what-removes-send-and-sync.md:191`), never from an inherited `unsafe impl Send`, and any value with no `Send` bound is collapsed before the next await where that is what the code means (RS-25-4). | `cargo xtask lints`; review. |
| NF-006 | `cargo xtask spec-trace` stays green: SY-8, SY-16 and SY-17's `Rule:` lines resolve to real symbols after this PR, and no `[FROZEN]` clause text is edited. | `cargo xtask spec-trace`. |

## Implementation notes (non-prescriptive)

These are constraints with the answer left open, per the architecture brief's own
*Non-prescriptive implementation notes* (`…/_decomposition.md:488-521`).

- **Shape.** One type, a trait, or a function — all three are admissible. The invariant that is not
  open is AC-001's: "add a second peer" is a configuration change. Record which alternative lost and
  why, in the PR body and in the implementation report; a struct holding a peer collection and a
  method taking `&self` is the obvious candidate, and the argument against it (a trait would let an
  application substitute its own scheduling) deserves a sentence either way.
- **Where it lives.** A new `crates/happenstance-sync/src/runner.rs` declared and re-exported from
  `lib.rs`'s composition root is the least surprising placement given the crate's existing module
  list (`crates/happenstance-sync/src/lib.rs:142-159`), but the mount obligation is the requirement,
  not the filename.
- **Ordering between peers.** SY-8 puts it in the runner and does not say what it is. Round-robin
  over a slice is sufficient for this story; whatever is chosen, do not encode an ordering that only
  makes sense for a hub, because AC-009 forbids the runner from knowing which node is one.
- **The spawning entry point is optional.** If the design reads better without it, drop it and
  delete AC-004's second clause's subject — but then say so under *Clarifications* and keep AC-004's
  first clause, because the `Send`-typed edge driven in a real `tokio::spawn` is still the half of
  the instrument pair that rejects the wrong refactor.
- **The fixture peer's home.** SY-17's `Rule:` names `happenstance-sync-testkit` compiling its own
  suite against a `!Send` fixture peer. HS-S0103 owns that crate's fixture trait shape (single
  flavour, GAT-free, AC-A04); add the `!Send` fixture beside it rather than inventing a second
  fixture concept.
- **`SyncError` growth.** ADR-0016 left it untouched and named the extension as ADR-0026's
  (`crates/happenstance-sync/src/lib.rs:86-94`). If the runner wants an aggregate error over N
  peers, that is a *runner* type, not a widening of `SyncError`, unless ADR-0026 said otherwise.
- **The prose that stops being true.** `crates/happenstance-sync/src/lib.rs:106-133` is the crate's
  own account of what the phase-2 sketch proved. Move each finding into the ADR that consumed it
  before deleting the paragraph (`…/_decomposition.md:590-599`).

## Tests and CI (merge gate)

Grounded in the project testing brief (`…/_decomposition.md:731-800`). The order is the order a
story actually runs them, story grain first.

| tier | command / path | proves |
| --- | --- | --- |
| Story grain (affected) | `cargo xtask affected --base main` | Only what this diff could break; the grain `.redkiln/config.yaml`'s `verify:` block wires to `story`. |
| Static / lints | `cargo xtask lints && cargo xtask spec-trace` | NF-002 (no position literal), NF-005, NF-006; SY-8/SY-16/SY-17 `Rule:` lines resolve. |
| Unit + doc | `cargo test -p happenstance-sync --doc` | AC-010's doctest — one exchange over two peers, compiled and executed. |
| Integration (native) | `crates/happenstance-sync/tests/runner_fans_out.rs` | AC-001: the second peer is a value, and the first peer's delivered set is unchanged. |
| Integration (native) | `crates/happenstance-sync/tests/send_free_bounds.rs` | AC-002: no `Send` flavour and no `Send` bound on the drive path; one name of each pair per module. |
| Integration (native, `!Send`) | `crates/happenstance-sync/tests/mid_chain_not_send.rs` | AC-003: an `Rc`-held middle node driven in a real local spawn with the token held across an await. |
| Integration (native, `Send`) | `crates/happenstance-sync/tests/spawned_edge_carries_the_token_out.rs` | AC-004: a real `tokio::spawn`, the token carried out, the extra bound written on `P::Resume`. |
| Integration (native) | `crates/happenstance-sync/tests/resume_survives_a_dropped_handle.rs` | AC-006 and AC-007: no gap, no duplicate across a dropped handle; an empty pull returns the token to use next. |
| Integration (native) | `crates/happenstance-sync/tests/no_rejection_above_the_port.rs` | AC-008: transport retried, content never parked; no runner-side holding area. |
| Integration (native) | `crates/happenstance-sync/tests/roles_are_edges.rs` | AC-009: one type on two edges, no role parameter. |
| Compile-fail | `crates/happenstance-sync/tests/` (compile-fail case for EC-005) | EC-005: `error[E0277]` points at `P::Resume`, not at the store or the peer. |
| Testkit instantiation | `crates/happenstance-sync-testkit/tests/runner_instantiation.rs` and the harness under `crates/happenstance-sync-testkit/src/` | AC-005 native half: a `Send`-bound runner fails to compile in a crate this project owns. |
| Gate (`wasm32`) | `cargo xtask wasm` | AC-005 `wasm32` half: the sync harness *check* compiles a real instantiation, not an empty crate (`xtask/src/main.rs:203-283`). |
| Gate (project ceiling) | `cargo xtask ci --fast` | `integration_scoped` — fmt, clippy `-D warnings`, tests, the wasm32 steps, docs, `spec-trace`, packaging assertions. This project is not terminal, so the whole `cargo xtask ci` stays `closeout-and-durable-audience`'s. |
| Ledger | `_ledger.md` rows cite *which* command produced the evidence | `.redkiln/config.yaml`'s `require_ledger: true`; a green `ci --fast` proves the gate passed, never on its own that AC-003's spawn is the thing that ran. |

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation in this PR |
| --- | --- | --- |
| **The gate goes green on an excluded runtime.** `SendBoundRunner` passes the native suite, clippy, *and* the `wasm32` crate build, because a generic function's `Send` bounds are satisfied at instantiation. | This is the story's central failure and it is *deferred*: nothing says so until HS-S0111 meets `error[E0277]` in a crate it does not own. | AC-005 makes the instrument an instantiation inside the harness the `wasm32` step checks; AC-003 makes the native instrument a real spawn with the `!Send` node mid-chain. |
| **The wrong proof ships instead of the right one.** `assert_send` on a concrete future passes by auto-trait leakage; a `#[tokio::test]` that never spawns proves nothing (RS-62-3). The workspace has already shipped this once. | It is cheaper to write and looks identical in a diff. | Two instruments, not one (AC-003 + AC-004), mirroring `send_flavour_stream_is_send_in_generic_code` / `spawns_from_generic` at `crates/happenstance-core/src/memory.rs:614,643`. |
| **The `Resume: Send` gap tempts a port-level fix.** The compiler asks for a bound at exactly the moment the runner spawns. | Widening the port bound is one line and green; it is SY-17's named rejection. | AC-004 writes the narrow bound on the token; EC-005 pins the diagnostic; EC-006 makes the escalation path explicit rather than discretionary. |
| **Coupling to HS-S0103's fixture shape.** The `!Send` fixture peer lives in a crate created two stories earlier and its trait shape (single flavour, GAT-free) is AC-A04's. | A fixture-trait change lands underneath this PR. | Add beside the existing fixture concept rather than inventing a second one; the `depends_on` edge is already declared, so the merge order forces HS-S0103 → HS-S0104 → this. |
| **Slice-mate collision in `lib.rs`.** HS-S0110 rewrites the topology module documentation in the same file this PR mounts into. | Same slice, same context, adjacent lines. | This PR corrects only sentences it makes false and leaves the topology paragraphs (`:52-66`) to AC-007 of HS-S0110; the two are implemented in one context by design. |
| **Findings deleted with the prose that held them.** Implementing the crate makes most of `src/lib.rs:106-133` untrue. | The sketch landed nine phases early precisely to produce those findings (`RUNBOOK.md:462-466`). | AC-010's final clause and the *Behavior and interfaces* row that names it; move the finding into the consuming ADR before deleting the paragraph. |
| **`LocalSet` ergonomics on a multi-thread runtime.** The `!Send` chain needs `spawn_local` on a `LocalSet` or a current-thread runtime. | A test that reaches for `#[tokio::test(flavor = "multi_thread")]` and then cannot spawn the chain may be "fixed" by making the chain `Send`. | `tokio`'s `rt` feature is already a dev-dependency; the test is written against a current-thread runtime deliberately, and that choice is documented at the test. |

## Dependencies

**Blocks on** (must be merged first; this is the story map's computed order, `…/_storymap.md:132-133`):

- `ingest-store-and-memory-peer-round-trip` (HS-S0102) — supplies real `IngestStore` and
  `MemorySyncPeer` bodies. A runner fanning out over `todo!()` proves nothing at runtime, and the
  scoped `#![allow(clippy::todo)]` is that story's to remove.
- `gate-mounts-for-the-sync-suite` (HS-S0104) — supplies the two `wasm32` steps (a build of
  `happenstance-sync` and a check of the sync harness) plus the gate constants `RULE_FILES`,
  `TESTKIT_SRC` and `TESTKIT_MANIFEST`. Without them AC-005 has nothing to compile inside.

Transitively also `sync-testkit-crate-and-rule-registry` (HS-S0103, via HS-S0104), which is where
`crates/happenstance-sync-testkit/` comes into existence.

**Unlocks:**

- `hub-and-spoke-and-peer-to-peer-topologies` (HS-S0110) — the slice-mate; it exercises both
  topologies over *this* runner and brings `lib.rs`'s topology module documentation into line.
- `durable-object-and-neon-peers` (HS-S0111) — the DoD 4 proof artefact; it needs the runner in the
  tree, and it is the story that pays the price if this one binds a `Send` flavour.

## Anchors (progressive disclosure)

Everything load-bearing that is *not* in the Context pack. Link, open on demand, do not paste in
bulk. Every path below exists in the tree today.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` | SY-17 (`:6341-6360`) is the frozen bound *and* names the instrument; SY-8 (`:6070-6086`) puts fan-out in the runner and keeps it out of the suite; SY-16 (`:6323-6340`) is the token contract; SY-9/SY-10 (`:6090-6128`) are hub-ness and the merge rules this story must not foreclose; SY-19 (`:6415-6428`) is why no assertion may name a position; SY-24 (`:6544-6552`) is the A—B—C arrangement AC-003 borrows. | Before writing any bound, and again before writing each test's assertions. | AC-002, AC-003, AC-006, AC-009 |
| `crates/happenstance-sync/src/peer.rs` | The port itself: `Resume`'s ownership contract and the documented `Send` gap (`:95-118`), the one-peer-not-a-peer-set paragraph (`:65-73`), the non-`async` `limits()` reasoning (`:152-160`), and the import-ambiguity note (`:26-28`). | Before the first line of the runner. | AC-001, AC-002, AC-006, AC-007 |
| `crates/happenstance-sync/src/ingest.rs` | `IngestStore`'s real shape — `ingest`, `holds`, `watermark`, `store_id` — and its own refusal contract, which is what EC-003 and AC-008 must not duplicate above the port. | When wiring the receive half of an exchange. | AC-008 |
| `crates/happenstance-sync/src/lib.rs` | The mount point and composition root (`:142-159`), the runner promise the module doc has carried since phase 2 (`:42-50`), and the findings paragraphs that must be relocated rather than deleted (`:106-133`). | At mount time, and again before deleting any prose. | AC-001, AC-010 |
| `crates/happenstance-sync/src/memory.rs` | `MemorySyncPeer` is the oracle for every native test — and it implements `SendSyncPeer` (`:162-164`), so it *cannot on its own* prove anything about the `!Send` path. Knowing that is what stops AC-003 being written against it. | When choosing the peer for each test arrangement. | AC-004, AC-006 |
| `crates/happenstance-sync/tests/real_peer_shapes.rs` | The existing Durable-Object stand-in whose error wraps an `Rc<str>` and which produced the two `error[E0277]`s the port documents. It is the shape AC-003's middle node should follow, and it already exists. | Before writing the `!Send` fixture or the mid-chain test. | AC-003 |
| `crates/happenstance-core/src/memory.rs` | `send_flavour_stream_is_send_in_generic_code` (`:614`) and `spawns_from_generic` (`:643`) are the workspace's settled answer to "how do you prove a `Send` claim" — the bound at the definition plus a real spawn holding the value across an await. Copy the pair, not one of them. | Before writing AC-003's and AC-004's instruments. | AC-003, AC-004 |
| `standards/rust/25-what-removes-send-and-sync.md` | RS-25-1 (`Rc` removes `Send`), RS-25-4 (collapse an unbounded value before the next await) and RS-25-5 (`:191` — build the `!Send` instrument out of `Rc`, never an inherited `unsafe impl Send`). NF-005 is this atom. | While constructing the `!Send` fixture and the mid-chain chain. | AC-003 |
| `standards/rust/21-send-is-not-inherited.md` | RS-21-2 (`:74` — write the assertion as `where F::Output: Send`, never `F: Send`) and RS-21-3 (`:169` — keep `Send + Sync` off the port's `Error` and ask for it where you spawn). Exactly the two mistakes AC-004 and EC-001 are shaped to prevent. | Before writing the spawning entry point's bound. | AC-004 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-3 (`:128` — a green `#[tokio::test]` says nothing about `Send`) and RS-62-5's out-of-package harness, which is how a `publish = false` crate's doctests get run at all. | Before deciding what counts as evidence for AC-004, and before putting a doctest in the testkit. | AC-004, AC-010 |
| `standards/rust/70-rustdoc-obligations.md` | The full rustdoc bar this project owes *instead of* a rendered surface: which items need docs, the `# Errors` form (conditions, not the error type), and the compiled-example requirement. | Before the first `pub` item is written, not after. | AC-010 |
| `standards/rust/52-wasm32-and-target-cfg.md` | RS-52-1 (on `wasm32` the compiler is not the instrument) and RS-52-3/4 (a `cfg` covers the probe *and* its caller, or the probe is dead code on the other target) — the trap that turns AC-005's harness into an empty compile. | Before adding anything `cfg`-gated to the sync harness. | AC-005 |
| `xtask/src/main.rs` | The four existing `wasm32` steps at `:203-283` with their stated reasoning, which HS-S0104 extends to six. This is the file that decides whether AC-009's *"exercised in the gate"* sentence is true. | When verifying AC-005, and if the `wasm32` check passes suspiciously fast. | AC-005 |
| `.kb/decisions/0001-async-port-flavours.md` | The accepted atom behind the two-flavour design: no `#[async_trait]`, `trait_variant` derives the `Send` flavour, and why binding the weaker one is the whole point. | Before any temptation to add a `Send` bound "just here". | AC-002 |
| `.kb/decisions/0009-error-send-sync.md` | Errors keep exactly `core::error::Error + 'static` on every port and flavour; the stronger property belongs in a downstream marker trait. The named risk is *a runner that adds `+ Send + Sync` on the way past* — this runner. | When designing how N peers' errors are surfaced to one caller. | AC-002 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | AC-A06/AC-A07 (`:550-557`) are the architecture-grain twins of AC-002 and AC-005; the testing brief's *Integration* section (`:731-748`) is where AC-009's tripwire is specified; *Merge-gate commands* (`:775-800`) is the command order; the non-prescriptive notes (`:488-521`) are the latitude. | Before fixing the runner's shape, and before writing the test plan. | AC-002, AC-005 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/send-free-sync-runner/discover.md` | The four named wrong implementations in full — `SendBoundRunner`, `AssertSendRunner`, `HeldCursorRunner`, and the rejection-above-the-port mutant — each with the reason its gate stays green. The anti-pattern list this spec cites by name. | Before writing each test, to check it would actually fail the mutant it claims to reject. | AC-003, AC-006, AC-008 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability`, `RuleOutcome` and the declined-capability pattern (`:368-433`) that `happenstance-sync-testkit` reuses rather than re-declares (AC-A03). Also the single-flavour GAT-free fixture shape (`:76-111`) the `!Send` fixture must match. | When adding the `!Send` fixture peer to the testkit. | AC-005 |
| `RUNBOOK.md` | Phase 13 in full (`:4516-4620`), the `!Send` path end to end (`:4588-4590`), and the residual risk that lands here if the `Resume` gap needs a port change (`:462-466`) — EC-006's escalation path. | If the compiler asks for a bound the runner cannot legally give it. | AC-002 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The three personas the acceptance criteria are written from, and the qualification that all four rest on secondary evidence. Reading a criterion as a capability instead of an intent is the failure this prevents. | Before re-wording any acceptance criterion. | AC-001, AC-003, AC-006 |
| `.redkiln/config.yaml` | The `verify:` block that wires `cargo xtask affected --base main` to the story grain and `cargo xtask ci --fast` to `integration_scoped`, plus `require_ledger: true`. It is why the ledger must cite *which* command produced each row's evidence. | When filling `_ledger.md`. | AC-005, AC-010 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated** — AC-001 … AC-010. None was added and
   none dropped. The mapping is: AC-001 fan-out as configuration, AC-002 the send-free bounds,
   AC-003 the mid-chain real spawn, AC-004 the `Send`-capable caller and the token-only extra bound,
   AC-005 the `wasm32` instantiation, AC-006 the token surviving a dropped handle, AC-007 the empty
   exchange, AC-008 no rejection above the port, AC-009 hub-ness as an edge property, AC-010 the
   documented surface.
2. **Project AC-009 is split across three rows rather than one.** Its sentence has two halves — the
   bounds and *"built and exercised for `wasm32` inside the gate"* — and the second half needs both
   a native instrument (AC-003) and a `wasm32` one (AC-005), because each is green while the other
   fails. Collapsing them into one row would let a source-level grep alone satisfy the clause.
3. **The runner's shape is deliberately still open at spec exit.** The architecture brief left it
   open on purpose (`…/_decomposition.md:493-498`) and this spec does not close it; what it closes
   is the invariant (AC-001) and the obligation to record which alternative lost. A spec that picked
   the shape would be re-deciding a thing the brief reserved for the implementer.
4. **No conformance rule is added, and that is a decision rather than an omission.** SY-8 says every
   rule in `happenstance-sync-testkit` takes exactly one peer handle, so a runner rule would be a
   rule testing the runner (`spec/SPECIFICATION.md:6074-6077`). Runner behaviour is integration-tier
   in `happenstance-sync`'s own `tests/`; SY-17 is the exception and is discharged by a **compile**
   in the testkit. This also means `headline-rules-and-mutant-registry`'s mutant registry gains no
   entry from this story.
5. **Interaction quality is answered in the medium this story has.** `_design.md` records no
   user-facing surface and the human signed off *that determination* (`…/_design.md:76-125`), so no
   composition invariant from a signed-off design binds here and no density budget exists to hold.
   The transferred obligation — the public API surface — is carried as AC rows (AC-010 for
   presentation, AC-001 for placement, AC-004 for reversibility, AC-006/AC-007 for preserved state),
   not as prose bullets, so every one of them gets a ledger line.
6. **The spawning entry point is optional and its optionality is written into AC-004.** If it does
   not land, AC-004's first clause still stands — the `Send`-typed edge driven inside a real
   `tokio::spawn` is half of the instrument pair — and the implementation report says which way it
   went and why.
7. **`crates/happenstance-sync-testkit/` does not exist in the tree today.** It is created by
   `sync-testkit-crate-and-rule-registry` (HS-S0103), which precedes this story in the computed
   merge order via `gate-mounts-for-the-sync-suite`. Paths under it are cited as *destinations*
   here, never as anchors — every anchor in the table above resolves at HEAD.
8. **EC-006 is an escalation path, not an error condition the code handles.** It is listed with the
   error conditions because that is where an implementer will be standing when it fires: at a
   compiler error asking for a bound the runner may not give it. The correct response is a new
   decision atom and a re-plan, and writing it down here is what makes stopping cheaper than
   widening a bound.
