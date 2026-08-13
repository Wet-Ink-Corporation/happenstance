---
item: HS-S0111
stage: spec
created: 2026-08-12T13:47:52.293Z
updated: 2026-08-12T13:47:52.293Z
template_sig: 87bbf1d0
rendered_sig: 366d7179
---

# Spec — One suite, three peers, two of them genuinely unlike

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/project.md` |
| This spec | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/durable-object-and-neon-peers/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` — *Architecture brief* §4 (the three peers), §6 (gate mounts); *Testing brief* — *The test mix, tier by tier* (Integration), *Fixtures and seams to mock* |
| Signed-off design | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md` — **no user-facing surface**, approved as such 2026-08-12. This story renders nothing; the design sign-off binds it only by its *Anti-patterns* and the no-surface determination itself. |
| Story map row | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_storymap.md`, milestone `live-unlike-peers` |
| Roadmap pointer | `RUNBOOK.md:4591-4592` (the two unlike peers), `RUNBOOK.md:4597-4602` (the proof artefact), `RUNBOOK.md:462-466` (the residual risk this story is the scheduled mitigation for) |

Traces to project **AC-005** (`project.md`, *Acceptance criteria*). Depends on
`headline-rules-and-mutant-registry`, `message-set-on-the-envelope`,
`send-free-sync-runner`.

## One-line PR slice

Confirm both environments exist from HS-P0013/HS-P0014 first, then implement `SyncPeer`
and `IngestStore` in `happenstance-cloudflare` and `happenstance-neon` (local type,
foreign trait — the only place coherence allows it) and run the one suite against all
three peers, with an unavailable environment reported as a declined capability carrying
the fixture's stated reason and never as a vanished target.

## Executive summary

Everything this PR needs already exists except the two implementations and the two
invocation sites. `sync_peer_conformance!` and its rule registry landed in
`sync-testkit-crate-and-rule-registry`; the rules that discriminate landed in
`headline-rules-and-mutant-registry`; the wire the peers speak landed in
`message-set-on-the-envelope`; the runner that drives them landed in
`send-free-sync-runner`; and `gate-mounts-for-the-sync-suite` already taught
`RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST` and the `wasm32` step list about the
fourth suite. The oracle — `MemorySyncPeer` — has been green since
`ingest-store-and-memory-peer-round-trip`.

**The delta is the spread.** Up to now every peer that has run this suite serialises its
writers, owns a live handle, and is `Send` — one storage shape wearing one hat, which is
exactly the failure mode `CLAUDE.md` names (*a port is only as well-designed as the
spread of what implements it*). This PR puts the suite in front of a peer that **cannot
satisfy a `Send` bound at all** and a peer that **holds nothing between calls** — the two
ends of the axis `crates/happenstance-sync/src/peer.rs`'s own status note says the port
was designed against but was never actually pointed at, because the phase-2 evidence was
two `todo!()` stand-ins whose `Send`-ness was asserted by whoever wrote them
(`crates/happenstance-sync/tests/real_peer_shapes.rs:16-25`, which says so in its own
header). After this PR the port's central claim stops being a stand-in's property and
becomes a real adapter's.

It also retires those stand-ins without retiring what they found, and it is the story
where an *absent* environment must be reported rather than routed around: this project
carries no deployment brief, so if HS-P0013's or HS-P0014's environment is not there, the
answer is a raised blocker and a declined capability with a stated reason — never a mock.

## Context pack

Read this section and you can start. Everything deeper is a signposted anchor.

**1. The two peers are the axis, and faking either one deletes the axis.** The Durable
Object peer is `!Send` throughout — errors wrap a `JsValue`-derived string behind an
`Rc<str>`, there is no ambient runtime to spawn into, and the workspace forbids `unsafe`
so the adapter cannot inherit `wasm-bindgen`'s `unsafe impl Send` escape hatch
(`crates/happenstance-cloudflare/src/lib.rs`, *Findings* §1). The Neon peer reaches its
store over one-shot HTTP: no connection, no interactive transaction handle, no cursor,
exactly one round trip per operation, 64 MiB hard response cap
(`crates/happenstance-neon/src/lib.rs`, the capability table; `transport.rs:54`). The
project's own risk table and the testing brief both say the Neon axis *cannot be faked
without destroying the thing it exists to test* — a mock of "no interactive transaction"
asserts only what the mock's author already believed. **So: real adapters, real
environments, or a declined capability. Not a stand-in.**

**2. Coherence is what makes this legal, and it is the whole reason `EventStore` never
grew an ingest slot.** `IngestStore` lives in `happenstance-sync`; the store types live
in the adapter crates. Foreign trait, local type — the orphan rule permits exactly this
and permits no third crate to write it on either party's behalf, which
`crates/happenstance-sync/src/ingest.rs` proves with a `compile_fail,E0117` doctest.
Neither of these two adapters needs *any* change to `happenstance-core`: option (d) in
the architecture brief's write-path table is correct for a real adapter, and only the
in-memory oracle ever needed core's cooperation (`_decomposition.md`, *Architecture
brief* §5). **If a real peer turns out to need a seam on the port itself, that is
`RUNBOOK.md:462-466`'s residual risk landing hard — stop, raise a decision atom and a
re-plan; do not widen the port inside this story.**

**3. Bind the weaker flavour, and import one name per module.** `SyncPeer`,
`IngestStore` and `EventStore` — never `SendSyncPeer`, `SendIngestStore` or
`SendEventStore` — in any generic bound on the ingest path or the suite
([ADR-0001](.kb/decisions/0001-async-port-flavours.md); `CLAUDE.md` constraints 1 and 4;
SY-17 `[FROZEN]`). Having both names of a pair in scope makes method calls ambiguous with
`error[E0034]`, which `crates/happenstance-sync/src/peer.rs:26-28` records. The Cloudflare
peer implements the **bare** flavour and is the only thing in the workspace that can
falsify a `Send` bound written by accident; the Neon peer implements the `Send` flavour
and gets the bare one for free. That asymmetry is the point — a suite that only ever saw
one flavour would certify nothing.

**4. One round trip is a *fixture* obligation, because the port cannot express it.**
SY-15 is `[FROZEN]`: every port method MUST be completable in one round trip and the port
MUST NOT require a peer to hold state between calls
(`spec/SPECIFICATION.md:6292-6320`). The clause's own `Rule:` line says how it is
checked — a fixture peer whose transport asserts on its own round-trip count and panics
on a second call within one operation. `crates/happenstance-sync/src/peer.rs:43-50`
explains why it has to be there and not on the port: the cursor shape was attempted,
compiled against both peers, and the one-shot HTTP peer satisfied `impl Stream` by
buffering a whole response and replaying it — *legal, `Send`, and a lie*. **The Neon
fixture must carry that counter and the Neon peer must survive it.** This is the single
strongest thing this story proves and the easiest to lose by wiring the fixture through a
convenience helper that batches.

**5. A declined capability runs and reports; it never vanishes.** `Capability` and
`RuleOutcome` are reused from `happenstance-testkit`, not re-declared
(`crates/happenstance-testkit/src/contract.rs:368-433` and `:473-483`).
`Capability::declined("")` is a **compile error** — the reason string is checked by an
`assert!` in a `const fn` — and `RuleOutcome` is `#[must_use]`, so an emitter that drops
an outcome fails a `-D warnings` build. When the Neon or Cloudflare environment is
genuinely unavailable in a run, the fixture declines the capability with the real reason
(*why* it cannot, not *that* it cannot) and the rule still appears in the output.
`#[cfg]`-ing the leg out, or a feature flag that silently no-ops, is the forbidden move
here (DR-8; testing brief, *The test mix* → Integration, third bullet).

**6. The suite decodes no payload byte, on any path, for any peer.** SY-35 `[FROZEN]`,
DR-5, and the reason [ADR-0003](.kb/decisions/0003-opaque-payloads.md) exists: a suite
that parses a payload would certify a peer that does. Compare `Event::data` and
`Event::metadata` as opaque `Bytes` for equality and nothing else. Adding a network
transport is precisely the moment someone reaches for `serde_json::from_slice` to "check
it arrived intact" — the byte comparison already checks that, and it checks it harder.

**7. This story consumes HS-P0013 and HS-P0014; it does not build them.** The Durable
Object store and the Neon store are `cloudflare-durable-object-store` and
`postgres-and-neon-stores` (`project.md`, *Out of scope*). This story puts peer clothes
on stores those projects built, and it adds the sync suite as a second thing their
existing harness runs — not a new harness (testing brief, Integration, second and third
bullets). **Confirming both environments is the first task in the PR, not a discovery
made at the end**, and the architecture brief puts that confirmation at slice 1
deliberately (`_decomposition.md`, *Non-prescriptive implementation notes*, last bullet).

**8. The phase-2 stand-ins are superseded; their finding is not.**
`crates/happenstance-sync/tests/real_peer_shapes.rs` holds two `todo!()` peers with *the
type properties of the real thing and none of its dependencies*, under a scoped
`#![allow(clippy::todo)]`. When the real impls land, the `todo!()`s go and the transcript
— *the type checker did not force that choice* — must be carried into the record before
the paragraph holding it is deleted (`_decomposition.md`, *Architecture brief* §4 and its
closing note, *One thing to carry forward that no AC captures*).
`crates/happenstance-sync/tests/cursor_shape_probe.rs` is a different artifact and must
keep compiling: ADR-0026 cites it for why `pull` returns a batch.

**The persona-journey slice.** The reader here is an **adapter author** deciding whether
this port is worth implementing against a transport nobody in this workspace has. What
they meet is a CI log with three peer names in it, two of which are structurally nothing
like the third, and a declined-capability line that tells them what a real constraint
cost rather than hiding it. That log is the artifact; a green gate is a precondition for
it, never a substitute (`project.md`, DoD 4 — *a phase is done when its proof artefact
exists*).

## Integration contract

- **Archetype**: `capability` — a slice through port, adapter, suite and gate that ends
  in observable output.
- **Slice / milestone**: `live-unlike-peers`. **Slice-mates: none.** This story is the
  entire milestone (`_storymap.md`, *Merge order* step 6), so it is mounted alone and
  there is no sibling context to integrate with. Its predecessors are three merged
  stories, not slice-mates.
- **Mount point**: `crates/happenstance-cloudflare/tests/sync_peer_conformance.rs` and
  `crates/happenstance-neon/tests/sync_peer_conformance.rs` — the two
  `sync_peer_conformance!` invocation sites, written in the shape
  `crates/happenstance-testkit/tests/memory_conformance.rs:27` already uses for the
  event-store suite. Neither `tests/` directory exists today; creating it *is* the mount.
  The gate-side half of the mount already exists from `gate-mounts-for-the-sync-suite`:
  `xtask/src/main.rs`'s `wasm32` step list carries the Cloudflare leg, and
  `xtask/src/spec_trace.rs`'s `RULE_FILES` carries the rules. **An adapter that compiles
  but has not run the suite is not an adapter** (`CLAUDE.md`, *The rule that matters*), so
  a peer impl without its invocation site is not this story delivered.
- **Wires into**:
  - `crates/happenstance-sync/src/peer.rs` — `SyncPeer`, `Pulled`, `PushBatch`,
    `EventGroup`, `Ack`, `PeerLimits`, `Watermark`.
  - `crates/happenstance-sync/src/ingest.rs` — `IngestStore`, `Ingested`.
  - `crates/happenstance-sync/src/identity.rs` — `EventId`, `ReplicatedEvent`, `StoreId`.
  - `crates/happenstance-sync/src/wire.rs` — `Envelope<T>` and the message set landed by
    `message-set-on-the-envelope`.
  - `crates/happenstance-sync-testkit/` — `sync_peer_conformance!`,
    `for_each_sync_peer_rule!`, the peer fixture trait and the mutant registry.
  - `crates/happenstance-testkit/src/contract.rs` — `Capability`, `RuleOutcome` and the
    three emitters, consumed rather than re-declared.
  - `crates/happenstance-cloudflare/src/{event_store.rs,js.rs,sql_storage.rs}` and
    `crates/happenstance-neon/src/{event_store.rs,transport.rs,error.rs}` — the stores and
    transports HS-P0013/HS-P0014 own, consumed as built.
- **Renders surfaces**: **none.**
  `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md`
  records the no-surface determination and signs it off; there are no `## Items` and no
  `## Signatures` to implement. The public-API obligation (rustdoc on every new public
  item, `# Errors` on every fallible one) still applies via
  `standards/rust/70-rustdoc-obligations.md`, and the peer impls' associated types are
  the API shape a reviewer checks.
- **Conformance rule(s)**: this story adds **no new rule**. It runs the existing sync
  rules against two new fixtures. The rules whose green-ness is the deliverable are the
  ones the frozen clauses already name — `ingest_never_rejects` (SY-1),
  `compensation_is_atomic_with_the_losing_event` (SY-2),
  `wire_condition_with_after_is_refused` (SY-6) — plus SY-15's round-trip-counting
  fixture obligation, which is a fixture property rather than a rule name
  (`spec/SPECIFICATION.md:6292-6301`; the clause→rule ledger at `:8669-8712`).
- **Clause(s)**: discharges **no** clause by amendment and edits none. It supplies the
  *adapter spread* SY-15 and SY-17 are stated against, and it is the run that makes SY-1,
  SY-2 and SY-6 true of something other than the oracle. Any `Rejects:` repair triggered
  by a symbol this story changes belongs to `frozen-clause-repairs`, which merges after
  it; if this story changes such a symbol, say so in the ledger so that story can find it.
- **Advances DoD scenario**: initiative **DoD 14** — *replication has an answer on disk* —
  by turning the answer from a written claim into evidence, via the project's own DoD 4
  proof artefact (*one suite green against three peers, two structurally unlike*). It also
  strengthens, without owning, initiative DoD 4 and DoD 6: the same two environments that
  must pass the **event-store** suite for HS-P0013/HS-P0014 are here shown to carry a
  second port as well.

## PR boundary

**In this PR**

- Environment confirmation for both peers, recorded — including the negative outcome if
  either is absent.
- `impl SyncPeer` and `impl IngestStore` for the Cloudflare Durable Object store, in
  `happenstance-cloudflare` (bare flavour; `!Send` error; no `Send` bound anywhere on the
  path).
- `impl SendSyncPeer`/`SyncPeer` and `IngestStore` for the Neon store, in
  `happenstance-neon`, one round trip per method, no held state.
- The two peer fixtures — each declaring its capabilities with real reasons, and the Neon
  one carrying the round-trip counter SY-15's `Rule:` line requires.
- The two `sync_peer_conformance!` invocation sites (the mount).
- Whatever `Cargo.toml` edges the two adapter crates need to see `happenstance-sync` and
  `happenstance-sync-testkit`; both crates stay `publish = false`.
- Retirement of the two `todo!()` stand-ins in
  `crates/happenstance-sync/tests/real_peer_shapes.rs` and of that file's scoped
  `#![allow(clippy::todo)]`, with the finding carried forward rather than deleted.
- The story's own `_ledger.md` and implementation report.
- The composition-root/wiring files named in the *Integration contract* may be touched to
  mount this slice — that is not scope drift.

**Explicitly not in this PR**

- Building or changing the Durable Object store or the Neon store themselves — HS-P0013
  and HS-P0014 (`project.md`, *Out of scope*).
- Any change to `EventStore`'s trait signature, or any new method on `happenstance-core`.
  AC-A01 says it is byte-identical across this project.
- New conformance rules or new mutants — `headline-rules-and-mutant-registry`'s, already
  merged. If a rule is missing, that is a finding to raise, not a rule to add here.
- Clause repairs, `(new)`-marker removal and the clause arithmetic —
  `frozen-clause-repairs` and `clause-arithmetic-and-deferral-renewals`, which merge after
  this story.
- Publishing or name-claiming either sync crate — recorded disposition is *do not claim*
  (`_decomposition.md`, *AC-015 — the disposition on claiming the two crate names*).
- Topology scenarios and the runner's shape — `hub-and-spoke-and-peer-to-peer-topologies`
  and `send-free-sync-runner`.
- Any edit to a `[FROZEN]` clause or to an accepted decision atom's body.

**Merge DoD one-liner.** `cargo xtask ci --fast` is green, and `sync_peer_conformance!`
has run against `MemorySyncPeer`, the Durable Object peer and the Neon peer with every
rule either `Ran` or `Skipped { capability, reason }` — no rule absent from the output on
any of the three.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Environments are confirmed before any peer code is written** | The Durable Object and Neon environments come from HS-P0013/HS-P0014. Confirm both first; record the result either way. An absent environment is a **blocker to raise**, never a stand-in to write, and never a silently skipped leg. | `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md`, *Non-prescriptive implementation notes*, last bullet; `project.md`, risk table row *Two of the three peers are networked* |
| **The Durable Object peer is implemented as foreign trait on a local type** | `impl SyncPeer for <the crate's store handle>` and `impl IngestStore for` the same, written inside `happenstance-cloudflare`. No blanket impl exists and no third crate may supply one. | `crates/happenstance-sync/src/ingest.rs` — the `compile_fail,E0117` doctest and the coherence argument above it |
| **The Durable Object peer stays genuinely `!Send`** | `Error` carries `Rc<str>` and nothing weakens it; the impl is the **bare** `SyncPeer`/`IngestStore` flavour; no `Send` bound appears on its path. The crate forbids `unsafe`, so `wasm-bindgen`'s `unsafe impl Send` escape is unavailable by construction. | `crates/happenstance-cloudflare/src/lib.rs` *Findings* §1; `crates/happenstance-cloudflare/src/js.rs`; [ADR-0001](.kb/decisions/0001-async-port-flavours.md); `CLAUDE.md` constraints 1 and 4 |
| **The Neon peer completes every method in one round trip and holds nothing** | No `open`/`next_batch`/`close`; `pull` returns a bounded batch plus an owned resume token; the resume token survives the handle being dropped and reconstructed. Multi-statement work uses the non-interactive batch form (one `BEGIN`/`COMMIT`, server-side, one request), never two requests. | SY-15 `[FROZEN]`, `spec/SPECIFICATION.md:6292-6320`; SY-16; `crates/happenstance-neon/src/transport.rs` (`SqlRequest::batch`, `IsolationLevel`, `MAX_RESPONSE_BYTES`) |
| **The round-trip claim is measured by the fixture, not asserted by the port** | The Neon fixture's transport counts its own round trips and panics on a second call within one operation. The port cannot express this property; the fixture is the only place the check can live. | `spec/SPECIFICATION.md:6298-6301` (SY-15's own `Rule:` line); `crates/happenstance-sync/src/peer.rs:43-50`; `_decomposition.md`, *Fixtures and seams to mock*, round-trip-counter row |
| **One suite, invoked three times** | `sync_peer_conformance!(<Fixture>::new())` at three sites — the existing memory one plus the two new adapter ones — sharing one rule enumeration through `for_each_sync_peer_rule!`, so the tokio, blocking and `wasm32` harnesses all see the same rules. | `crates/happenstance-testkit/tests/memory_conformance.rs:27` for the shape; `crates/happenstance-testkit/src/registry.rs:94-102`; `_decomposition.md`, *Architecture brief* §1 |
| **A declined capability reports and still runs** | Each fixture declares its `Capability` constants with a reason naming *why* the constraint exists. An unavailable environment produces `RuleOutcome::Skipped { capability, reason }` in the output. `Capability::declined("")` fails to compile; a dropped `RuleOutcome` fails `-D warnings`. | `crates/happenstance-testkit/src/contract.rs:368-433`, `:473-483`; DR-8 in `project.md` |
| **No payload byte is decoded on any peer path** | `Event::data` and `Event::metadata` cross the transport and are compared as opaque `Bytes` for equality only — including inside the two new fixtures and their transports' own assertions. | SY-35; DR-5 in `project.md`; [ADR-0003](.kb/decisions/0003-opaque-payloads.md); `RUNBOOK.md:4576-4580` |
| **No literal position value is asserted anywhere new** | Any position the new fixtures compare is one the store under test assigned. The gate's `lint-position-literals` already covers the sync suite from `gate-mounts-for-the-sync-suite`. | CF-6, `spec/SPECIFICATION.md:7233-7249`; `CLAUDE.md`, *The rule that matters*; `xtask/src/main.rs` `lint-position-literals` |
| **The phase-2 stand-ins are retired and their finding is preserved** | The two `todo!()` peers and the file's scoped `#![allow(clippy::todo)]` go; the header's honest limit (*a stand-in's `Send`-ness is asserted by whoever wrote it*) is superseded by two real adapters and must be recorded as such before the prose is deleted. `cursor_shape_probe.rs` is untouched and keeps compiling. | `crates/happenstance-sync/tests/real_peer_shapes.rs:16-25`; `crates/happenstance-sync/tests/cursor_shape_probe.rs`; `_decomposition.md`, *Architecture brief* §4 and its closing note |
| **`happenstance-core` is not touched** | Both networked adapters own their own types, so coherence lets them write both impls with no core cooperation. Option (d) in the write-path table is correct here; only the in-memory oracle ever needed option (a), and that landed in `memory-store-ingest-seam`. | `_decomposition.md`, *Architecture brief* §5, options table; AC-A01; `RUNBOOK.md:450-455` |
| **Both crates stay unpublished** | `publish = false` still reads `false` in `crates/happenstance-cloudflare/Cargo.toml` and `crates/happenstance-neon/Cargo.toml` at exit, and no sync crate is claimed or released. | `project.md` AC-015; `_decomposition.md`, *AC-015 — the disposition on claiming the two crate names* |
| **The gate runs the whole of it** | `cargo xtask ci --fast` — this project's ceiling — including the `wasm32` build of `happenstance-sync` and the `wasm32` check of the sync harness added by `gate-mounts-for-the-sync-suite`. The Cloudflare leg is a `wasm32-unknown-unknown` target and its suite run is a gate step, not prose. | `.redkiln/config.yaml:50-55`; `xtask/src/main.rs:203-283`; `project.md` DoD 1 |

## Data and migrations

**N/A.** This story introduces no schema, no persisted format change and no migration.

Three near-misses, stated so they are not mistaken for an omission:

- **The wire format is not changed here.** `Envelope<T>`, `FORMAT_VERSION` and the message
  set are `message-set-on-the-envelope`'s, already merged. This story is the first thing to
  send that format over a real network, which is a *use* of the format, not a change to it
  (`crates/happenstance-sync/src/wire.rs`; [ADR-0016](.kb/decisions/0016-the-wire-format.md)).
- **The Neon peer's SQL is read-and-write against HS-P0014's existing tables.** It adds no
  table, column or index; if it appears to need one, that is HS-P0014's schema and a
  finding to raise rather than a migration to write here
  (`crates/happenstance-neon/src/event_store.rs`).
- **Ingested events take a fresh *local* position and keep their foreign `EventId`.** That
  is the existing frozen behaviour of the ingest path (SY-5, SY-19), not a data change this
  story introduces — the two new peers must exhibit it, not define it.

## Acceptance criteria

Every criterion is a persona goal that crosses the whole stack — environment, adapter crate,
port, suite, gate, CI log — not a capability restated. The personas are the initiative's own
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`):
**P2** the adapter author, who wants an executable definition of *correct* to run against a
storage system nobody here owns (`:114-131`); **P3** the local-first / edge developer, who must
event-source inside a non-`Send` runtime and whose journey *has never actually been walked by
this project* (`:182-196`, last paragraph); **P4** the evaluator, who cannot run this suite and
is deciding from the artefacts whether "storage-agnostic" survived contact with two transports
(`:249-266`). Every position comparison below anchors on a value the store under test assigned
(CF-6, DR-7); no verification contains a literal position.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **P4 is told what the infrastructure actually is, before anyone builds on a guess.** GIVEN the Durable Object and Neon environments are HS-P0013's and HS-P0014's and this project carries no deployment brief, WHEN the story starts — **first task, before a line of peer code** — THEN both environments are confirmed reachable and the outcome is recorded either way in this story's own directory, and an environment that is **absent** produces (a) a raised blocker naming the owning project and (b) a fixture that declines the affected capability with the real reason, never (c) a mock, a `#[cfg]`-out, a feature flag that no-ops, or a leg quietly dropped from the run. | A companion `_environments.md` in this story's directory, one row per peer, each row naming what was probed and what answered; the ledger's AC-001 evidence cites it. The negative outcome is a *pass* of this AC and a fail of AC-004's live half — the two are deliberately separable |
| AC-002 | **P3 finds the edge runtime represented by something that could not have been faked.** GIVEN `happenstance-cloudflare`, which forbids `unsafe` and whose error carries `Rc<str>` so its `!Send`-ness cannot disappear under a `cfg` (`crates/happenstance-cloudflare/src/lib.rs`, *Findings* §1), WHEN `SyncPeer` and `IngestStore` are implemented **inside that crate** on its own store handle, THEN the impls are the **bare** flavour, no `Send` bound appears anywhere on the path, no `#[async_trait]` is introduced, `crates/happenstance-core/**` is untouched, and the peer is a foreign trait on a local type — the one arrangement coherence permits and no third crate can supply on its behalf. | `cargo xtask wasm` (the `wasm32` build of `happenstance-cloudflare` and the sync harness check); `rg -n "Send(SyncPeer\|IngestStore\|EventStore)" crates/happenstance-cloudflare/src` reviewed against impl sites only; the `compile_fail,E0117` doctest in `crates/happenstance-sync/src/ingest.rs` still compiling; an empty `crates/happenstance-core/**` diff. SY-17 `spec/SPECIFICATION.md:6341-6360`, ADR-0001 |
| AC-003 | **P2 learns the one-round-trip promise is measured, not asserted.** GIVEN `happenstance-neon`, which has no connection, no interactive transaction, no cursor and a hard 64 MiB response cap (`crates/happenstance-neon/src/lib.rs`, capability table; `transport.rs:54`), WHEN the suite runs against its peer through a fixture transport that **counts its own round trips and panics on a second call within one operation**, THEN every port method completes in exactly one round trip with no state held between calls — multi-statement work going through the non-interactive batch form (`SqlRequest::batch`, one `BEGIN`/`COMMIT`, server-side, one request) rather than two requests — and the resume token survives the peer handle being dropped and reconstructed. | The counting transport in `crates/happenstance-neon/tests/` asserting on its own count; `resume_survives_a_dropped_peer_handle` from the shared rule set running green against the Neon fixture; SY-15 `spec/SPECIFICATION.md:6292-6320`, SY-16 `:6321-6340`; `crates/happenstance-sync/src/peer.rs:43-50` for why the check cannot live on the port |
| AC-004 | **P4 reads one CI log and sees three peer names, two of them structurally nothing like the third.** GIVEN `sync_peer_conformance!` and its rule registry already in the tree, WHEN the story is done, THEN that macro is invoked at **three** sites — the existing memory one plus `crates/happenstance-cloudflare/tests/sync_peer_conformance.rs` and `crates/happenstance-neon/tests/sync_peer_conformance.rs` — over **one** rule enumeration through `for_each_sync_peer_rule!`, and every rule appears in every peer's output as either `Ran` or `Skipped { capability, reason }`: no rule is absent from any of the three, and `ingest_never_rejects` (SY-1), `compensation_is_atomic_with_the_losing_event` (SY-2) and `wire_condition_with_after_is_refused` (SY-6) are green against something other than the oracle for the first time. | The three invocation sites, written in the shape `crates/happenstance-testkit/tests/memory_conformance.rs:27` uses; the run's output compared rule-name-by-rule-name across the three peers (a set difference, not a count); `crates/happenstance-testkit/src/registry.rs` as the single enumeration |
| AC-005 | **P2 is told what a real constraint cost, instead of being shown a shorter list.** GIVEN a peer that genuinely cannot do something — an unavailable environment, or a transport limit that no implementation can work around — WHEN the suite runs, THEN the fixture declines that `Capability` with a reason naming **why** rather than **that**, the rule still executes and still reports, and the reason lands in the CI log where a reviewer and an adapter author can both read it. `Capability::declined("")` is a compile error and a dropped `RuleOutcome` fails a `-D warnings` build, so neither an empty reason nor a swallowed skip can ship. | `crates/happenstance-testkit/src/contract.rs:368-433` (the `const fn` `assert!`) and `:473-483` (`#[must_use]`), **consumed, not re-declared**; the observed output containing a `Skipped { capability, reason }` line for each declined capability; DR-8 (`project.md`) |
| AC-006 | **P1/P4 watch a payload cross a real network and come back byte-for-byte.** GIVEN an event whose `data` and `metadata` are bytes that are neither valid UTF-8 nor valid JSON, WHEN it crosses a real transport to either networked peer and is read back, THEN the `Bytes` compare equal to the origin's and **no path** — peer impl, fixture, transport assertion or test — inspects them structurally. Adding a network transport is the moment `serde_json::from_slice` gets reached for "to check it arrived intact"; the byte comparison already checks that, and harder. | The non-decodable payload as the negative control — any parsing path fails loudly rather than passing by luck; `rg` over the two new fixtures and transports for payload-decoding calls returns nothing; SY-35, DR-5, [ADR-0003](.kb/decisions/0003-opaque-payloads.md), `RUNBOOK.md:4576-4580` |
| AC-007 | **P2's adapter is not failed by an assumption the specification never made.** GIVEN two stores that assign positions on entirely different mechanisms, WHEN the new fixtures compare any position, THEN the comparison is against a value the store under test assigned — a head captured into a binding before the operation — and never against a literal, because the specification permits gaps and a conformant adapter may leave them. | `cargo xtask lint-position-literals` (via `cargo xtask lints`), already scoped to the sync suite by `gate-mounts-for-the-sync-suite`; review of every comparison in the two new fixtures; CF-6 `spec/SPECIFICATION.md:7233-7249`; `CLAUDE.md`, *The rule that matters* |
| AC-008 | **P4 finds the phase-2 evidence still on file after the thing it stood in for arrives.** GIVEN `crates/happenstance-sync/tests/real_peer_shapes.rs`, whose own header states its honest limit — *a stand-in's `Send`-ness is asserted by whoever wrote it* (`:16-25`) — WHEN the two real impls land, THEN the `todo!()` bodies and the file's scoped `#![allow(clippy::todo)]` are gone, the finding that header recorded is carried into the record **before** the paragraph holding it is deleted, and `crates/happenstance-sync/tests/cursor_shape_probe.rs` is untouched and still compiling, because ADR-0026 cites it for why `pull` returns a batch. | `cargo clippy --workspace --all-targets -- -D warnings` green with the allow deleted; `rg -n "todo!\(" crates/happenstance-sync/tests` returns nothing; `cargo test -p happenstance-sync --all-features` still compiling `cursor_shape_probe.rs`; the carried-forward finding present in this story's implementation report and cited in the ledger |
| AC-009 | **P3 and P4 get a gate that ran the whole of it, on the target that matters.** GIVEN this project's ceiling is `cargo xtask ci --fast` (`.redkiln/config.yaml:50-55`), WHEN it runs at exit, THEN it is green **including** the `wasm32` build of `happenstance-sync`, the `wasm32` check of the sync harness and the `wasm32` build of `happenstance-cloudflare` — the Cloudflare leg being a `wasm32-unknown-unknown` target and its suite run a gate step rather than prose — and no `wasm32` step degrades to `skipped`, because all of them carry `probe: None`. | `cargo xtask ci --fast`; `cargo xtask wasm` listing six steps rather than four (`xtask/src/main.rs:196-218`, `:769-791`); `cargo xtask spec-trace` green so no `Rule:` citation was orphaned by the two new invocation sites |

**Coverage of the traced project AC.** All nine serve project **AC-005** — *one suite, three peers,
two of them genuinely unlike* (`project.md`, *Acceptance criteria*; `_storymap.md`, *Coverage*, which
assigns AC-005 to this story alone). AC-002 and AC-003 are the two unlike peers themselves and are the
whole of "genuinely"; AC-004 is "one suite, three peers"; AC-001 and AC-005 are what makes an
un-runnable leg *reportable* instead of invisible, which is the difference between three peers and
three peers you can believe in; AC-006, AC-007, AC-008 and AC-009 are the conditions under which the
green run is evidence rather than a coincidence. **This story adds no rule and edits no clause** —
where a rule looks wrong, that is a finding for `frozen-clause-repairs`, which merges after it.

## Interaction quality

**RFC §6.7/D6, and this story renders no surface.** The project's signed-off
`_design.md` records the no-surface determination and a human approved *that determination itself*,
with `## Items`, `## Signatures`, `## The states the API must express`, `## Anti-patterns`, `## The
doctest` and `## Sign-off` all reading *"N/A — no user-facing surface"*
(`.bklg/from-contract-to-published-library/replication-identity-and-ingest/_design.md:104-124`). So
there is no route, DOM node or pane here, and the STATE family has no literal referent. Both families
are inherited in the two media this story actually produces, which is where `_design.md` redirects:
the **public API surface** (held to `standards/rust/70-rustdoc-obligations.md`) and the **CI log**,
which is this story's real artefact — `project.md` DoD 4 says a phase is done when its proof artefact
exists, not when the gate is green.

This section says **which AC carries which invariant**. Every invariant below is a row in the table
above; none is stated only here, because `redkiln verify` extracts ACs from a leading `| AC-001 |`
cell or a `- AC-001:` bullet, and a prose bullet in this section would get no ledger row, never be
gated and never be tested.

| Family | Invariant, in this medium | Carried by | How it is verified |
| --- | --- | --- | --- |
| STATE | *In-place vs context-jump* — adding a second port to an adapter crate must not relocate what is already there. The sync suite is a **second thing HS-P0013's and HS-P0014's existing harness runs**, not a new harness beside it, and the event-store suite those crates already run keeps running unchanged. | AC-004, AC-009 | The two `tests/` files added beside the crates' existing harness; `cargo xtask ci --fast` green over both suites |
| STATE | *Non-occlusion* — a declined capability never hides a rule. The skip is *additive output*: the rule name is still printed, with the reason beside it. A shorter rule list is this medium's occluded control. | AC-005 | Rule-name set difference across the three peers is empty (AC-004's verification); `RuleOutcome`'s `#[must_use]` |
| STATE | *Preserved caller state across a boundary* — the resume token is the caller's, and it survives the peer handle being dropped and reconstructed. This is the medium's "preserved selection": the operation's continuation point is not the peer's to hold. | AC-003 | `resume_survives_a_dropped_peer_handle` against the Neon fixture; SY-16 |
| STATE | *Reversibility* — an environment that turns out to be absent is recoverable without rework: the answer is a declined capability plus a raised blocker, both of which leave the diff and the rule set intact. The irreversible move is `#[cfg]`-ing a leg out, because nothing afterwards can tell it from a leg that never existed. | AC-001, AC-005 | `_environments.md` recording the negative outcome; `rg` for a `#[cfg]` or feature gate around either invocation site returns nothing |
| STATE | *Reachability* — the keyboard-reachability analogue: every capability this story lands is reachable through the declared mount points. A peer impl that compiles but has no `sync_peer_conformance!` invocation site is not mounted, and `CLAUDE.md`'s *rule that matters* says it is not an adapter. | AC-004 | The three invocation sites present and executed by the gate |
| COMPOSITION | *Presentation exists at all* — the API analogue of "not bare markup": every new public item carries real rustdoc, and every fallible one carries `# Errors` naming the **conditions** rather than the error type. An undocumented `pub` item is this medium's unstyled render, and it satisfies every type assertion perfectly. | AC-002, AC-003 | `cargo doc --no-deps -p happenstance-cloudflare -p happenstance-neon` warning-free, inside `cargo xtask ci --fast` |
| COMPOSITION | *What a reader meets first* — the composed artefact is the **CI log**, not the type signatures: three peer names, and a declined-capability line that states a real cost. A run whose output cannot be read that way has not produced DoD 4's proof artefact even if every assertion passed. | AC-004, AC-005 | The captured run output cited in the implementation report and in the ledger's AC-004/AC-005 evidence |
| COMPOSITION | *Placement* — the impls live **in the adapter crates**, on their own types. That placement is the coherence argument, not a filing preference: no third crate may write either impl, and `happenstance-core` gains nothing. | AC-002 | The `compile_fail,E0117` doctest in `crates/happenstance-sync/src/ingest.rs`; empty `crates/happenstance-core/**` diff |
| COMPOSITION | *Transience* — persistent: the two peer impls, the two fixtures, the two invocation sites. Revealed-on-demand: the declined-capability reasons, which appear only when a constraint bites. Expiring on schedule: the two `todo!()` stand-ins in `real_peer_shapes.rs`, whose stated expiry is exactly this phase. | AC-008 | The allow deleted and the `todo!()`s gone; `cursor_shape_probe.rs` deliberately **not** expiring |
| COMPOSITION | *Density budget, with its real numbers* — **two** `impl SyncPeer`, **two** `impl IngestStore`, **two** fixtures, **two** invocation-site files, **zero** new conformance rules, **zero** new mutants, **zero** new clauses, **zero** lines in `happenstance-core`, **zero** changes to `EventStore`'s signature, and **at most** the `Cargo.toml` edges the two adapter crates need. Anything beyond that budget is a scope change, not an implementation detail. | AC-002, AC-004, AC-009 | The *PR boundary* above read against `git diff --stat`; `cargo xtask affected --base main` |
| COMPOSITION | *Hierarchy* — the port outranks the adapters and the specification outranks both. A real peer that seems to need a seam on the port does not get one here: that is `RUNBOOK.md:462-466`'s residual risk landing, and it stops the story (EC-003). | AC-002 | The empty `crates/happenstance-sync/src/peer.rs` public-signature diff; `cargo xtask spec-trace` green |
| COMPOSITION | *Named anti-patterns* — `_design.md` names none (no surface), so the binding set is this project's own: a **mocked** unlike peer (which asserts only what the mock's author believed), a **vanished** rule, a **decoded** payload, and a **literal** position. Each is rejected by an AC rather than by a comment. | AC-001, AC-005, AC-006, AC-007 respectively | The four verifications in those rows |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | The Neon test environment is unavailable in a run (no credentials, no reachable endpoint, or HS-P0014 shipped only `NullTransport` and no licence-clean HTTP client exists for the target — `crates/happenstance-neon/src/lib.rs`, *It owns no HTTP client*). | **Declare, do not vanish.** The fixture declines the affected capability with the real reason — *why* the environment cannot answer — and every rule still runs and reports. A blocker is raised naming HS-P0014. Writing a mock transport that "behaves like one-shot HTTP" is the forbidden move: it asserts only what its author already believed, which is the exact thing DR-6 and the testing brief's *Fixtures and seams* row rule out. |
| EC-002 | The Durable Object harness from HS-P0013 does not exist, or `wasm32` tests cannot be executed on this machine (no `wasm-bindgen-test` runner, no local Worker emulator). | Same shape as EC-001, plus: the **compile** half is still owed and still gated. `cargo xtask ci --fast`'s `wasm32` steps must still build the crate and type-check the harness even when nothing executes it, because a step that cannot run is not a step that cannot compile (`xtask/src/main.rs:196-218`). |
| EC-003 | A rule fails against a real peer for a reason that looks like the **port** being wrong — a method that cannot be completed in one round trip, or a peer that needs to hold state the port gives it nowhere to hold. | **Stop and raise.** This is `RUNBOOK.md:462-466`'s residual risk landing hard, and the mitigation it names is that the port is designed against *two* unlike peers rather than widened when one complains. Raise a decision atom through `.kb/_intake/` and a re-plan; do not widen `SyncPeer`, do not add a method, do not reach into `happenstance-core`. |
| EC-004 | A Neon operation needs two statements (a probe and a write, or a read and a checkpoint). | Use the **non-interactive batch**: one `SqlRequest::batch` with an explicit `IsolationLevel`, executed server-side inside one `BEGIN`/`COMMIT`, one request (`crates/happenstance-neon/src/transport.rs:157`). Two requests is two implicit transactions with a network-latency-wide window between them — the trap the crate was put in the tree to spring. The counting fixture is what catches it; raising the counter's threshold to make the test pass is the wrong repair. |
| EC-005 | A pull's result would exceed the transport's ceiling — Neon's hard 64 MiB response cap (`MAX_RESPONSE_BYTES`, `transport.rs:54`) or the caller's `PeerLimits` budget. | A **bounded batch plus a resume token**, never a silent truncation and never a buffered replay dressed as a stream. The caller loops; the limit is reported through the port's existing budget shape. A single group that cannot fit on its own is a stated limit and a declined capability with a reason, not a dropped group. |
| EC-006 | The Cloudflare peer would be easier to write if its error were `Send`. | Refuse. The workspace sets `unsafe_code = "forbid"`, so `wasm-bindgen`'s `unsafe impl Send` can only be *inherited* by holding a `JsValue`, never written — and an instrument whose `!Send`-ness disappears under a `cfg` cannot falsify a bound (`crates/happenstance-cloudflare/src/lib.rs`, *Findings* §1). If the port genuinely cannot be implemented without it, that is EC-003, not a type change. |
| EC-007 | A capability is declined for a reason that is really *"we did not implement it yet"*. | Not a legal decline. `Capability::declined` exists to record a **constraint**, and its reason is printed on every run as the only record of the trade. An unimplemented method is a `todo!()` this story is required to remove (AC-008), not a capability to decline. |
| EC-008 | Deleting a paragraph of `real_peer_shapes.rs` would delete a phase-2 finding that nothing else records. | Move it first, delete second (AC-008). `rg` for the paragraph's content across `spec/`, `crates/*/tests/` and `.kb/` **before** the deletion — afterwards the string is gone and a citation is a line number pointing at something else. |
| EC-009 | A live leg is flaky across the network. | Diagnose before retrying. A retry loop around a conformance rule hides exactly the class of bug the live legs exist to find (an ordering or visibility fault that only a real transport exposes). If flakiness is genuinely the transport's and not the adapter's, it is a stated capability limit with a reason, recorded — not a silent retry. |
| EC-010 | A rule looks wrong once a real peer is behind it. | Record the finding; do not edit the rule here. Rules and mutants are `headline-rules-and-mutant-registry`'s, already merged, and clause repairs are `frozen-clause-repairs`', which merges after this story — so the finding goes in this story's ledger and companion notes where that story can find it (*Integration contract*, *Clause(s)*). |

## Non-functional

| id | Requirement | Evidence |
| --- | --- | --- |
| NF-001 | **Neither sync crate is published or name-claimed, and both adapter crates stay `publish = false`.** A name reservation is a placeholder publish; the disposition is recorded as *do not claim*. | `cargo package --list` assertion inside `cargo xtask ci --fast`; `publish = false` still reading `false` in `crates/happenstance-cloudflare/Cargo.toml:12` and in `crates/happenstance-neon/Cargo.toml`; project AC-015 |
| NF-002 | **`happenstance-core`'s public surface is byte-identical before and after.** Both networked adapters own their own types, so coherence lets them write both impls with no cooperation from the contract crate — option (d) in the architecture brief's write-path table. | An empty `crates/happenstance-core/**` diff, which is the first thing review looks at; architecture AC-A01 |
| NF-003 | **No `#[async_trait]`, and no `Send` flavour named in any bound this story writes.** One name of each pair per module, or method calls go ambiguous with `error[E0034]`. The Cloudflare peer is the only thing in the workspace that can falsify a `Send` bound written by accident, which is why it is here. | `rg -n "async_trait" crates/happenstance-cloudflare crates/happenstance-neon` empty; `cargo xtask wasm`; ADR-0001, `CLAUDE.md` constraints 1 and 4, SY-17 |
| NF-004 | **Any new dependency the live legs need must clear the gate's own licence and advisory checks.** `happenstance-neon` deliberately owns no HTTP client and `happenstance-cloudflare` deliberately does not depend on `worker` (`crates/happenstance-cloudflare/Cargo.toml:19-25`); if a live leg needs one, it enters as a **dev-dependency of the test harness**, target-split where the two targets need different clients, and `cargo deny` runs for real on this machine. If no candidate clears it, that is EC-001, not a vendored copy. | `cargo deny` inside `cargo xtask ci --fast`; `git diff` over both adapter manifests reviewed against the notes that explain the current absences |
| NF-005 | **No payload byte is decoded on any path**, implementation or test, on either peer. | AC-006's non-decodable payload as the standing negative control; DR-5 |
| NF-006 | **The live legs are bounded and do not become the gate's tail.** A networked suite run has a stated timeout and reports elapsed time; an unbounded wait is indistinguishable in CI from a hang, and the fix for a slow leg is a stated capability limit, never a raised sleep. | The run's own reported duration recorded in the implementation report; `cargo xtask affected --base main` used at story grain so the fast in-process legs run on every pass |
| NF-007 | **No credential is committed.** Endpoints and secrets for either environment come from the environment at run time; the repository records how to supply them, never what they are. | `_environments.md` naming the variables, not the values; review of the diff for any literal endpoint or token |

## Implementation notes (non-prescriptive)

Shape suggestions only; the ACs are the contract.

- **Do AC-001 on day one and write it down before anything else.** The architecture brief puts
  environment confirmation at slice 1 precisely so the answer is a *precondition*, not a discovery
  made after two impls are written (`_decomposition.md`, *Non-prescriptive implementation notes*,
  last bullet). Both adapter crates today record deliberate absences — no `worker` dependency, no
  HTTP client — so "does the environment exist" is a live question with a real chance of *no*.
- **Read the two crate-level docs first, in full.** `crates/happenstance-cloudflare/src/lib.rs`'s
  *Findings* and `crates/happenstance-neon/src/lib.rs`'s capability table and *the trap this crate
  exists to spring* are the two most information-dense pages in the workspace for this story, and
  both were written by someone who had already made the mistakes.
- **Write the Neon counting transport before the Neon peer.** If the counter arrives afterwards it
  will be written by someone who already knows the body is right — RS-60-4's exact warning
  (`standards/rust/60-what-a-test-must-prove.md`). The counter is also the only artefact that can
  fail; the port cannot express the property at all.
- **Start the Cloudflare peer from the error type.** `Rc<str>` is what makes it genuinely `!Send`;
  if the impl compiles with a `Send` bound anywhere on its path, that bound was written by accident
  and this is the only peer in the workspace that will say so.
- **Mount before finishing.** Create both `tests/sync_peer_conformance.rs` files early, even against
  a fixture that declines everything: the mount is what makes the work visible to the gate, and a
  peer impl without an invocation site is not this story delivered.
- **Compare rule-name *sets*, not counts, across the three peers.** A count matches by accident; a
  set difference is the assertion AC-004 actually wants.
- **`rg` before you delete.** AC-008's finding is unrecoverable once the paragraph is gone; grep
  `spec/`, `crates/*/tests/` and `.kb/` for its content while the string still exists.
- **When something is impossible, say why in the decline reason.** The reason string is printed on
  every run and is the only durable record of the trade — it is the artefact P2 actually reads.

## Tests and CI (merge gate)

Grounded in the project testing brief (`_decomposition.md`, *The test mix, tier by tier* and
*Merge-gate commands*). This story is overwhelmingly **Integration** — it is the tier the brief calls
"the conformance suite, run three times, two of them live" — and it adds **no** new static gate step
(that was `gate-mounts-for-the-sync-suite`'s) and **no** new rule or mutant (that was
`headline-rules-and-mutant-registry`'s).

| Tier | Command / path | Proves |
| --- | --- | --- |
| Process | `_environments.md` in this story's directory | AC-001 — both environments probed and the outcome recorded *before* peer code exists, with an absence producing a blocker and a declined capability rather than a mock. |
| Static | `cargo xtask affected --base main` | Story grain: the affected set is the two adapter crates plus `happenstance-sync`/`happenstance-sync-testkit` and their reverse dependencies. A wider set means something reached outside the PR boundary. |
| Static | `cargo clippy --workspace --all-targets -- -D warnings` | AC-008 — with `real_peer_shapes.rs`'s scoped `#![allow(clippy::todo)]` deleted, this is the mechanical proof no stand-in body survived; also NF-003's `#[must_use]` half, since a dropped `RuleOutcome` warns. |
| Static | `cargo xtask lints && cargo xtask spec-trace` | `reachability_static` (`.redkiln/config.yaml:48`), unconditional at story grain. `lint-position-literals` carries AC-007 (CF-6); `spec-trace` carries AC-009's second half — no `Rule:` citation orphaned, no `[FROZEN]` clause edited. |
| Static | `cargo xtask wasm` | AC-002 and AC-009 — the `wasm32` build of `happenstance-sync`, the `wasm32` check of the sync harness and the `wasm32` build of `happenstance-cloudflare`, six steps rather than four, none of them skippable (`xtask/src/main.rs:769-791`). |
| Static | `cargo doc --no-deps -p happenstance-cloudflare -p happenstance-neon` | The COMPOSITION *presentation exists at all* invariant — rustdoc on every new public item and `# Errors` on every fallible one (`standards/rust/70-rustdoc-obligations.md`). |
| Unit | `cargo test -p happenstance-neon --all-features` | AC-003's mechanism in isolation: the counting transport panics on a second round trip within one operation, and the batch form is one request. The counter is a fixture property, so it is tested as one. |
| Integration | `crates/happenstance-neon/tests/sync_peer_conformance.rs` | AC-003, AC-004, AC-005, AC-006, AC-007 on the one-shot-HTTP axis — every rule `Ran` or `Skipped { capability, reason }`, the resume token surviving a dropped handle, payload bytes compared as opaque `Bytes`, positions anchored on store-assigned values. |
| Integration | `crates/happenstance-cloudflare/tests/sync_peer_conformance.rs` | AC-002, AC-004, AC-005, AC-006, AC-007 on the `!Send` axis — the same rule set, on a target with no ambient runtime and an error that cannot cross a thread. |
| Integration | `crates/happenstance-testkit/tests/memory_conformance.rs` (shape reference) and the existing memory sync harness | AC-004's third peer, the oracle — the always-on in-process leg that runs on every `cargo xtask affected` pass and is the control the two live legs are read against. |
| Integration | `cargo test -p happenstance-sync --all-features` | AC-008 — `real_peer_shapes.rs` retired and `cursor_shape_probe.rs` still compiling, which is a positive obligation rather than an absence. |
| Integration (project ceiling) | `cargo xtask ci --fast` | `integration_scoped` (`.redkiln/config.yaml:55`) and project DoD 1: fmt, clippy, the full test run, the `wasm32` steps, docs, `spec-trace`, the `--no-default-features` doc build, the `cargo package --list` assertion (NF-001), and `cargo deny` (NF-004). This project is not terminal, so `cargo xtask ci` is **not** this story's bar. |
| Ledger | `_ledger.md` in this story's directory, `require_ledger: true` (`.redkiln/config.yaml:62-67`) | Project DoD 3 — a green gate proves the gate passed, never on its own that the nine criteria above are the things that work. Each row cites the command or test that produced its evidence, and the run output for AC-004/AC-005. |

**Not run here, deliberately.** `redkiln validate --kb` as a *merge* gate — this story authors no
atom and must leave it exactly as green as it found it. The whole-initiative E2E is
`closeout-and-durable-audience`'s (`.redkiln/config.yaml:57-60`).

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Containment inside this PR |
| --- | --- | --- |
| **An environment is absent and a mock is written to keep moving.** | Medium / very high — a mocked one-shot-HTTP peer passes the whole suite and destroys the only thing the leg exists to prove; DoD 4's proof artefact would then be false rather than missing. | AC-001 makes the confirmation the first task and makes the *negative* outcome a pass; AC-005 gives the absence a first-class reporting shape so nobody needs a mock to keep the log tidy. EC-001 states the escalation. |
| **A real peer needs a seam on the port**, and the port is widened to unblock the story. | Low / very high — `RUNBOOK.md:462-466`'s residual risk; the port is the thing this project exists to freeze evidence against. | EC-003: stop and raise. NF-002 and the empty `crates/happenstance-sync/src/peer.rs` signature diff make the breach visible at review rather than at publish. |
| **A `Send` bound is written by accident on the Cloudflare path**, because everything else in the workspace tolerates one. | Medium / high — it silently deletes the axis P3 depends on and would ship green on every native target. | AC-002 plus the `wasm32` harness check: the Cloudflare peer is the only implementation that can falsify such a bound, and the gate compiles it. `CLAUDE.md` constraints 1 and 4. |
| **The Neon peer satisfies the port by buffering** — a whole response replayed as a batch, or two requests behind one method. | Medium / high — *legal, `Send`, and a lie* is the phrase `peer.rs:43-50` already uses for exactly this, discovered once by experiment. | AC-003's counting transport, written **before** the peer body; EC-004 names the batch form as the correct repair and forbids raising the counter's threshold. |
| **A payload gets decoded "just to check it arrived".** | Medium / medium — a network hop is the single most tempting place in the project for it, and the check it replaces is weaker than the byte comparison. | AC-006's non-decodable payload; NF-005; DR-5. |
| **A position literal ships**, because both new stores may assign densely and the literal would be green. | Medium / high — encodes a `MAY` as a `MUST`, the project's headline error. | AC-007; `lint-position-literals` already scoped to the sync suite by the merged `gate-mounts-for-the-sync-suite`. |
| **The phase-2 stand-ins are deleted with the finding they hold.** | High / medium — the prose is the evidence a sketch landed eleven phases early was written to produce, and it is unrecoverable. | AC-008 makes moving the finding a precondition of deleting the paragraph; EC-008 says to `rg` before, not after. |
| **A rule is edited to make a live peer pass.** | Low / high — it converts the instrument into a decoration in the one place where a real adapter finally disagreed with it. | The *PR boundary* excludes rules by name; EC-010 routes the finding to `frozen-clause-repairs`, which merges after this story and is where a repair is authorised. |
| **The live legs make the gate slow or flaky**, and a retry loop appears. | Medium / medium — a retry hides the ordering faults only a real transport exposes. | NF-006 bounds and reports the run; EC-009 requires diagnosis before retry and a stated capability limit if the flakiness is genuinely the transport's. |
| **Coupling to three merged predecessors** — the suite, the wire and the runner all have to be in the tree before a line of this story is startable. | Certain / contained | `_storymap.md`, *Merge order* step 6: this story is its own slice and merges after slices 3, 4 and 5. See *Dependencies*. |

## Dependencies

**Blocks on (must merge first):**

- **`headline-rules-and-mutant-registry`** — supplies the rules that actually discriminate,
  including `ingest_never_rejects` (SY-1) and `compensation_is_atomic_with_the_losing_event` (SY-2)
  with their named wrong peers. Without them the three-peer run is a green loop over an
  undiscriminating rule set, and AC-004's claim that the headline rules are now true of something
  other than the oracle would have nothing behind it.
- **`message-set-on-the-envelope`** — supplies `Envelope<T>` and the message set the two peers put on
  a real network. This story is the first thing to *send* that format; it does not define it, and a
  peer written before the message set exists would invent one.
- **`send-free-sync-runner`** — supplies the runner that drives the peers with the bound on
  `EventStore`/`SyncPeer`/`IngestStore` rather than a `Send` flavour, proved by a real
  `tokio::spawn` with the `!Send` peer mid-chain. AC-002's peer is the thing that runner was written
  not to exclude.
- *Transitively, and not restated as edges:* `sync-testkit-crate-and-rule-registry` (the macro and
  the registry), `ingest-store-and-memory-peer-round-trip` (the oracle), `gate-mounts-for-the-sync-suite`
  (the gate-side half of the mount, already in `xtask/src/main.rs` and `xtask/src/spec_trace.rs`),
  and **HS-P0013 / HS-P0014**, whose stores and environments this story consumes as built and does
  not create (`project.md`, *Out of scope*).

**Unlocks (their `depends_on` names this story, or their content depends on its findings):**

- **`frozen-clause-repairs`** — merges after this story and consumes any `Rejects:` symbol this diff
  changes, plus any rule finding raised under EC-010. This story records them in its ledger so that
  story can find them rather than re-derive them.
- **`clause-arithmetic-and-deferral-renewals`** — the exit computation, which cannot run until every
  clause-touching story including this one has landed.

**Neither blocked on nor blocking:** `hub-and-spoke-and-peer-to-peer-topologies` (slice 5) and the
wire/round-trip stories of slice 4 — they merge before this story in the computed order but this
story adds nothing they consume, and `_storymap.md` notes slices 4 and 5 may be interleaved.

## Anchors (progressive disclosure)

Open these at the moment named, not before. Every path verified present at HEAD.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md` | The architecture brief's §4 (the three peers and what each is an instrument for), §5 (the write-path options table — why a real adapter needs option (d) and no core cooperation), §6 (the gate mounts, already landed), and the *Non-prescriptive implementation notes*' last bullet, which is the source of AC-001's "confirm first" ordering. The testing brief's *The test mix* → Integration and *Fixtures and seams to mock* carry the round-trip-counter row and the declined-capability policy verbatim. | Before slice planning, and again whenever this spec is ambiguous about tier, fixture treatment or why an option lost. | AC-001 |
| `crates/happenstance-neon/src/lib.rs` | The capability table (no connection, no transaction handle, no cursor, one round trip, 64 MiB cap), *the trap this crate exists to spring* (probe-then-write type-checks perfectly and is silently wrong), the one-statement CTE that keeps `conflicting_position`, the flavour claim, and *It owns no HTTP client* — which is why AC-001's answer for this peer may legitimately be "absent". | Before writing a single line of the Neon peer, and again the moment a method looks like it needs two statements. | AC-003 |
| `crates/happenstance-neon/src/transport.rs` | `SqlRequest::batch` (`:157`) is the non-interactive one-request transaction EC-004 requires; `IsolationLevel` (`:63`) is the knob that makes it sound; `MAX_RESPONSE_BYTES` (`:54`) is the hard ceiling EC-005 is written against. This is also where the counting fixture transport plugs in, because `SqlTransport` is a one-method trait. | When writing the counting transport, and when composing any multi-statement operation. | AC-003 |
| `crates/happenstance-cloudflare/src/lib.rs` | *Findings* §1 is the whole `!Send` argument: a real `JsValue` is `Send + Sync` on Workers builds, `Rc<str>` is what makes the instrument honest, and `unsafe_code = "forbid"` is why the escape hatch cannot be written here. §2 explains what stringifying a throw costs. Assuming this crate is `!Send` "because wasm" is the expensive wrong assumption. | Before writing the Cloudflare peer's error type — which is where the impl should start. | AC-002 |
| `crates/happenstance-sync/src/peer.rs` | The port itself, plus the two paragraphs that govern this story: *why `pull` returns a batch and not a stream* (`:30-50`) — the cursor shape was attempted, compiled against both peers, and satisfied by a lie — and the statement that one round trip with no held state **cannot be expressed by the port** and must be checked by a counting fixture. `:26-28` is the `error[E0034]` note on importing both flavour names. | Before implementing either peer, and again before any temptation to widen the port (EC-003). | AC-003 |
| `crates/happenstance-sync/src/ingest.rs` | The `IngestStore` trait and the `compile_fail,E0117` doctest that *is* the coherence argument: foreign trait, local type, and no third crate on either party's behalf. This is why these two impls belong in the adapter crates and why `EventStore` never grew an ingest slot. | Before deciding where either impl lives — i.e. immediately. | AC-002 |
| `crates/happenstance-sync/tests/real_peer_shapes.rs` | The two `todo!()` stand-ins this story retires, and the header (`:16-25`) stating the honest limit of their evidence — *a stand-in's `Send`-ness is asserted by whoever wrote it*. It is also a working sketch of both impls' type shapes, so it is the cheapest starting point for the real ones as well as the thing being deleted. | First, as a template; last, as a deletion — with the finding moved before the paragraph goes. | AC-008 |
| `crates/happenstance-sync/tests/cursor_shape_probe.rs` | The experiment ADR-0026 cites for why `pull` returns a batch. It must keep compiling: it is the negative control for the shape this story's Neon peer would otherwise drift back toward. | When retiring `real_peer_shapes.rs` — the two files look similar and only one of them expires. | AC-008 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability` (`:368-433`) with the `const fn` `assert!` that makes an empty reason a compile error, and `RuleOutcome` (`:473-483`) with the `#[must_use]` that makes a swallowed skip a `-D warnings` failure. Both are **consumed** by the sync suite, never re-declared — re-declaring them is how the two guarantees get lost. | When writing each fixture's capability constants and their reason strings. | AC-005 |
| `crates/happenstance-testkit/tests/memory_conformance.rs` | The mount shape this story copies twice (`:27`), and its header explains what a fixture *is* — one instance is one isolated backing store, each `connect` a handle — and why one rule reports a skip on every run rather than vanishing from the binary. | When creating the two `sync_peer_conformance.rs` invocation sites. | AC-004 |
| `crates/happenstance-testkit/src/registry.rs` | The single place rules are enumerated, and the `no_orphan_rules` meta-test shape that keeps the enumeration and the module in agreement in both directions. The sync suite inherits this, which is why "one suite, three peers" is a property of the registry rather than of three copied files. | When checking that all three peers see the same rule set (AC-004's set difference). | AC-004 |
| `spec/SPECIFICATION.md` | The clauses this story is the first real evidence for: SY-15 `[FROZEN]` (`:6292-6320`) with the `Rule:` line that *names the counting fixture*, SY-16 (`:6323-6340`) on the owned resume token, SY-17 (`:6341-6360`) on binding the weaker flavour, and CF-6 (`:7233-7249`) on literal positions. Each `Rejects:` line is a better test name than anything read off an implementation. | With the clause text open while writing the fixtures — not afterwards. | AC-003 |
| `standards/rust/21-send-is-not-inherited.md` | Why a `Send` bound is not something you can retrofit or assume, stated with a compiled example and a named wrong implementation. Paired with `standards/rust/20-two-flavour-ports.md` it is the constitution's answer to every question this story's Cloudflare peer raises. | Before writing the Cloudflare impl's signatures and any generic helper. | AC-002 |
| `standards/rust/60-what-a-test-must-prove.md` | RS-60-4 — spell the oracle in a direction that shares no subroutine with the implementation. This is why the counting transport is written before the Neon peer, and why the rule-set comparison is a set difference rather than a count. | Before writing the counting transport and the output comparison. | AC-003 |
| `standards/rust/70-rustdoc-obligations.md` | The API-surface obligation `_design.md` explicitly redirects to in place of a rendered-surface design: rustdoc on every public item, `# Errors` naming conditions rather than error types. This story's COMPOSITION *presentation exists at all* invariant is this atom. | While writing the impl signatures, not as a cleanup pass. | AC-002 |
| `standards/rust/52-wasm32-and-target-cfg.md` | What `cfg(target_arch = "wasm32")` does and does not buy — including the fact that a harness behind it compiles to nothing natively, which is why the Cloudflare leg needs its own gate step and why a native green run proves nothing about it. | When wiring the Cloudflare invocation site and reading the `wasm32` step output. | AC-009 |
| `.kb/decisions/0001-async-port-flavours.md` | Why the two flavours exist, why generic code binds the weaker one, and why `#[async_trait]` is forbidden — the accepted decision that makes P3's runtime reachable at all. | Before writing any bound, on either peer. | AC-002 |
| `.kb/decisions/0003-opaque-payloads.md` | Why payloads are opaque `Bytes` and what a suite that parses one would certify. Reading it makes the byte comparison obviously stronger than a structural check rather than merely stricter. | When writing AC-006's assertion, and at the first temptation to decode. | AC-006 |
| `.kb/decisions/0009-error-send-sync.md` | The workspace-wide `Error` bound, taken for `SyncPeer` too — the constraint the Cloudflare peer's `Rc<str>` error has to satisfy without acquiring `Send`. | When declaring the Cloudflare peer's associated `Error` type. | AC-002 |
| `RUNBOOK.md` | `:4591-4592` (the two unlike peers as the phase's own work item), `:4597-4602` (the proof artefact this story produces), `:450-455` (why `EventStore` was refused a slot for a foreign identity), `:462-466` (the residual risk and the two scheduled mitigations, one of which is this story). | At the start for orientation, and the moment a change starts reaching toward the port or the contract crate. | AC-002 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/gate-mounts-for-the-sync-suite/spec.md` | The gate-side half of this story's mount, already merged: which `wasm32` steps exist, why the harness check is not implied by the crate build, and which lint constants learned about the sync suite. If a gate step this story expects is missing, this is where its intended shape is written down. | When the `wasm32` step list or a lint's scope does not match what this spec assumes. | AC-009 |
| `.bklg/from-contract-to-published-library/replication-identity-and-ingest/headline-rules-and-mutant-registry/spec.md` | The rule set this story runs and the mutants that make it discriminating, including the provenance requirement. If a rule looks wrong behind a real peer (EC-010), this is where its intent is stated and where the finding has to be aimed. | When a rule fails on a live peer and the rule itself is suspected. | AC-004 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the front half enumerated** — AC-001 through AC-009, unchanged.
   None was added, dropped or renumbered, and `_ledger.md` carries one row per id.
2. **The two live legs may legitimately be unavailable, and that is a *pass* of AC-001 and a *fail*
   of AC-004's live half.** The two are deliberately separable so that "we could not reach the
   environment" and "we did not report that we could not" are different outcomes. The repository's
   own state makes this live rather than theoretical: `happenstance-cloudflare` deliberately does not
   depend on `worker` (`crates/happenstance-cloudflare/Cargo.toml:19-25`) and `happenstance-neon`
   deliberately owns no HTTP client (`crates/happenstance-neon/src/lib.rs`, *It owns no HTTP
   client*), so whether HS-P0013 and HS-P0014 closed those gaps is exactly what AC-001 goes and
   finds out — first, and in writing.
3. **Which flavour the Neon peer implements is settled as a floor, not a ceiling.** The binding
   requirement is that the **Cloudflare** peer implements the bare `SyncPeer`/`IngestStore` and that
   **no bound this story writes names a `Send` flavour**. Implementing `SendSyncPeer` on the Neon
   peer is permitted where its transport is genuinely `Send` on the target being built, and the bare
   flavour then comes free (`crates/happenstance-sync/src/peer.rs:60-64`). It is not *required*:
   `crates/happenstance-neon/src/lib.rs` records that the crate implements the bare `EventStore` on
   both targets and satisfies no `Send` flavour, and this story does not overturn that stance in
   passing. The asymmetry AC-002 protects is that at least one peer **cannot** satisfy a `Send`
   bound — which is a property of the Cloudflare peer and does not depend on the Neon one.
4. **The round-trip counter lives in the fixture, and this is not a preference.** SY-15's own `Rule:`
   line names a fixture peer whose transport counts its round trips; `crates/happenstance-sync/src/peer.rs:43-50`
   records that the port *cannot* express the property, established by an experiment that is still in
   the tree. Any proposal to move the check onto the port is EC-003, not a refactor.
5. **How a no-surface project gets an *Interaction quality* section.** `_design.md` is binding, and
   what it binds here is the no-surface determination itself plus the absence of named anti-patterns
   (`:104-124`) — a human approved that, and this story does not re-decide it. The two families are
   therefore inherited in the media this story does produce: the public API surface
   (`standards/rust/70-rustdoc-obligations.md`) and the CI log, which project DoD 4 makes the actual
   deliverable. Every invariant is carried by an AC row so `redkiln verify` can see it.
6. **A rule that a real peer falsifies is a finding, not an edit.** The natural move when the first
   genuinely unlike adapter disagrees with a rule is to soften the rule; here that would convert the
   instrument into a decoration at the exact moment it finally worked. EC-010 routes it to
   `frozen-clause-repairs`, which merges after this story and is authorised to repair a clause under
   ADR-0026.
7. **`cursor_shape_probe.rs` is not part of AC-008's retirement.** The two test files look alike and
   only one has an expiry: `real_peer_shapes.rs` is superseded by the real impls, while
   `cursor_shape_probe.rs` is the standing evidence for why `pull` returns a batch and must keep
   compiling. Stated here because deleting both in one pass is the plausible mistake.
