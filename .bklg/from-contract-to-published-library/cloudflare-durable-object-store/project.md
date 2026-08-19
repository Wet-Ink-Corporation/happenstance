---
id: HS-P0013
uid: "6040e2"
type: project
slug: cloudflare-durable-object-store
title: The edge store, run rather than asserted
parent: HS-I0006
initiative: from-contract-to-published-library
project: cloudflare-durable-object-store
status: implementing
process: project
stage: implementation
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-19T14:43:59.389Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# The edge store, run rather than asserted

## One-line objective

Finish `happenstance-cloudflare` against the real Workers `SqlStorage` bindings and
run **every** event-store conformance rule under `workerd` on `wasm32` inside a
single `cargo xtask ci` invocation — so the `!Send` flavour the whole port design
is bent around is proved by execution rather than by a `cargo check`.

## How this advances the initiative

The initiative's sixth goal is that *"the `!Send` / edge case survives contact"*
(`.bklg/from-contract-to-published-library/initiative.md`, **Goals**). Today it has
not. The cost is already paid in three binding constraints — `#[async_trait]` is
forbidden outright, ports are declared once without a `Send` bound and
`trait_variant` derives the second flavour, and `EventStore::read` returns the
stream at the top level and is not `async` (`.kb/decisions/0001-async-port-flavours.md`,
`.kb/decisions/0008-one-derivation-for-both-ports.md`, `CLAUDE.md` **Binding
constraints** 1–4). The only thing standing behind that cost is a compile: the gate
runs `cargo check -p happenstance-cloudflare --target wasm32-unknown-unknown`
(`xtask/src/main.rs:245-264`) and nothing else. Every body in the crate is `todo!()`
(`crates/happenstance-cloudflare/src/lib.rs:4-10`), and a `todo!()` body type-checks
against any signature — which is the initiative's own stated reason for treating
skeleton-based proof as decorative (BR-03).

This project converts that instrument into an adapter. It is the constrained-runtime
member of the "at least three implementations that genuinely disagree on storage
shape" set (BR-02²), it is the sole owner of BR-12 and AC-07, and it is the only
place DoD 4 can be observed. It is a **DAG root** — no upstream blockers — and it
blocks `publication-and-positioning`, because AC-08 requires the published
compliance claim to name which implementations it was checked against
(`.bklg/from-contract-to-published-library/_decomposition.md`, **Sequencing** and
**Decisions taken at the gate** §1).

It is also the workspace's only instrument for ES-6. `CloudflareEventStoreError` is
the only error type here that can fail a `Send + Sync` bound; the other two are free
by construction — `MemoryStoreError` is uninhabited and the SQLite error was a
placeholder (`crates/happenstance-cloudflare/src/lib.rs:12-21`, `:236-243`). ADR-0009
predicted the asymmetry and this is the adapter that decides whether it bites
(`.kb/decisions/0009-error-send-sync.md`; `RUNBOOK.md:4263`, `:4270-4272`).

## In scope (this project)

- **The real bindings.** Replace the `worker`-free stand-in
  (`crates/happenstance-cloudflare/src/sql_storage.rs`, `js.rs`) with the real
  Durable Object `SqlStorage` API and finish `CloudflareEventStore`'s `read` and
  `append` (`crates/happenstance-cloudflare/src/event_store.rs`). No `todo!()` left,
  and the scoped `#![allow(clippy::todo)]` deleted with the last one
  (`crates/happenstance-cloudflare/src/lib.rs:121-126`).
- **A `Fixture` for the Durable Object**, declaring its `Capability` constants and
  its numeric limits honestly, against the reference implementation's shape
  (`crates/happenstance-testkit/src/fixtures.rs`, `crates/happenstance-testkit/src/contract.rs`).
- **The off-tokio harness.** Invoke `event_store_conformance!` through the shipped
  `__emit_wasm` emitter (`crates/happenstance-testkit/src/lib.rs:55-82`) and make it
  execute under `workerd`, wired into `cargo xtask ci` as a step selected **by name**
  (`xtask/src/main.rs:105-283`).
- **ADR-0023**, answering one question: the `SqlStorage` mapping and the shape of an
  off-tokio conformance harness (`RUNBOOK.md:302`, `:4267-4268`).
- **The ES-6 verdict**, with the artefact behind it: a committed error type carrying
  a real `worker::Error` and a test that reconstructs, from what a caller receives,
  whether the failure was a constraint violation or a transport fault
  (`RUNBOOK.md:4284-4292`).
- **The two capability limits that are not type errors** — a cursor that is not a
  stable snapshot against ES-9's laziness requirement, and positions bounded by 2^53
  rather than 2^64 (`crates/happenstance-cloudflare/src/lib.rs:96-113`;
  `.kb/decisions/0011-read-laziness-and-isolation.md`,
  `.kb/decisions/0013-position-assignment-and-visibility.md`).
- **CF-39 / CF-40**, including resolving the fixture-limits ownership question
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md`).
- **Re-reading CF-14 and CF-27's deferrals** on this runtime, and recording whether
  each still holds here.
- **The ES-32 tail-seam verdict** — one paragraph, recorded and *not acted on*
  (`RUNBOOK.md:4273-4275`, `:4300`).
- **WF-11's falsifier**, tested against this runtime's memory ceiling or shown not to
  bite (`.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`).
- **Claiming `happenstance-cloudflare` on crates.io** and removing `publish = false`,
  per phase 0's rule that a name is reserved when its phase starts
  (`RUNBOOK.md:4254-4261`, `:4301`).

## Out of scope (this project)

Each exclusion names the sibling that owns it.

- **The capability-reporting *policy* itself** — what a declined guarantee must
  print, and DT-3's resolution → `projection-store-freeze` (HS-P0010). This project
  is its first real exerciser against a runtime that genuinely cannot do some of
  what the suite asks, not its author.
- **Publishing this crate, the registry landing page, the MSRV promise, the semver
  diff and the clause-ledger audit** → `publication-and-positioning` (HS-P0016).
  Which crates actually publish at `0.2.0` is that project's deployment brief's
  (`.bklg/from-contract-to-published-library/_decomposition.md`, **Carried into the
  briefs**).
- **Reopening the tail / subscription seam (ES-32)** → post-0.1, outside this
  initiative. The verdict is recorded here; acting on it is not.
- **Durability, process reopen and the second-real-handle far end** →
  `sqlite-durable-store` (HS-P0012). CF-14's far end is that project's.
- **The non-serialising and no-connection/no-cursor far ends** →
  `postgres-and-neon-stores` (HS-P0014).
- **`ProjectionStore`, its freeze and its suite** → `projection-store-freeze`
  (HS-P0010); the unlike batch shape → `ladybug-projection-store` (HS-P0015). This
  crate ships **no** projection store — it is an event store only
  (`CLAUDE.md` repository map).
- **The typed layer, the worked example and the alpha release** →
  `typed-layer-and-alpha-release` (HS-P0011).
- **The sync port, ingest, the merge rule and any wire-format change** →
  `replication-identity-and-ingest` (HS-P0017). WF-11 evidence is gathered here; the
  decision is not taken here.
- **CF-27's completeness half and what a store may forget** →
  `retention-and-incomplete-logs` (HS-P0018).
- **Re-observing DoD 1–15 as a set on the assembled library** →
  `closeout-and-durable-audience` (HS-P0019), the only project wired to the terminal
  `verify.e2e` grain.
- **Amending anything `[FROZEN]`.** If this runtime cannot satisfy a frozen clause,
  that is a new decision atom and a re-plan, not a line edit
  (`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_intake-brief.md`,
  **Clauses**).

## Derived requirements

Expanded from the initiative requirements this project owns (**O**) or exercises
(**e**), per the traceability matrix in
`.bklg/from-contract-to-published-library/_decomposition.md`.

- **DR-1 (BR-02², BR-03 — O).** The constrained-runtime store counts toward the
  three-disagreeing-implementations claim only once it has *passed* the suite. A
  crate that compiles for `wasm32` contributes nothing, because a `todo!()` body
  type-checks against any signature.
- **DR-2 (BR-12 — O).** The `!Send` flavour must be exercised end to end by a store
  that is genuinely `!Send`: the bare `EventStore` implemented rather than
  `SendEventStore`, an `Error` that is not `Send`, and a stream that cannot leave the
  object's thread (`crates/happenstance-cloudflare/src/lib.rs:222-243`). The two
  `memory.rs` tests that pin `read`'s shape stay green and neither is deleted
  (`crates/happenstance-core/src/memory.rs`; `CLAUDE.md` constraint 3).
- **DR-3 (AC-07 — O).** *Every* rule runs on the target, **in the same run the rest
  of the gate uses**. A separately maintained `wasm32` subset is explicitly not an
  acceptable outcome; the rule set stays generated from the single
  `for_each_event_store_rule!` enumeration
  (`crates/happenstance-testkit/src/lib.rs:84-89`).
- **DR-4 (DoD 4 — O).** Its error type is shown *either* to carry what the caller
  needs *or* demonstrably not to. A green suite decides neither: every conformance
  rule asserts on the success path or on a store-produced `AppendError`, and none
  reads an adapter error's contents (`RUNBOOK.md:4284-4292`).
- **DR-5 (BR-13 / AC-05 — e).** Where this runtime cannot supply a guarantee, the
  rule is still emitted and reports the fixture's stated reason. It never vanishes
  from the binary (`crates/happenstance-testkit/src/lib.rs:46-50`;
  `xtask/src/main.rs:131-142` for why `--show-output` is on the gate's `tests` step).
- **DR-6 (AC-04 / AC-06 / AC-08 — e).** The adapter author's loop must terminate on
  this runtime too: one entry point, a pass or a named failure per rule, and no
  storage shape quietly assumed. The evidence this produces is what AC-08's published
  claim names.
- **DR-7 (BR-15 — O, applied).** Every answer this project settles lands as a
  decision atom naming the alternatives that lost, written through the runbook's ADR
  queue rather than as a side effect, and the matching open-question atoms are
  **resolved rather than deleted** (`.kb/decisions/README.md`; `CLAUDE.md`). Accepted
  atoms are immutable: confirming or refuting ADR-0009's ES-6 prediction is a new
  record, never an edit to `.kb/decisions/0009-error-send-sync.md`.

## Acceptance criteria

Project-grain and testable. This is the spine `_storymap.md` must cover.

- **AC-001 — The adapter is real.** `happenstance-cloudflare` depends on `worker`,
  implements `read` and `append` against the Durable Object `SqlStorage` API, and
  contains no `todo!()` on any adapter path; the scoped
  `#![allow(clippy::todo)]` at `crates/happenstance-cloudflare/src/lib.rs:126` is
  gone, and `cargo clippy … -D warnings` is green without it.
- **AC-002 — Every event-store conformance rule runs and passes under `workerd`.**
  `event_store_conformance!` is invoked with `emit = happenstance_testkit::__emit_wasm`
  against the Durable Object fixture, the emitted rule set is the full
  `for_each_event_store_rule!` enumeration, and the run reports a pass or a named
  failure for every rule. No rule is `#[cfg]`-ed out and no wasm-only subset list
  exists anywhere in the tree.
- **AC-003 — Every declined capability reports the fixture's stated reason.** The
  fixture declares its `Capability` constants with real reasons; each rule guarding a
  declined capability still runs and emits its `SKIP <rule>: <reason>` line. The
  families this runtime cannot host at all — notably
  `event_store_concurrency_conformance!`, whose bound is `F::Store: EventStore + Send`
  and which is `cfg`-ed off `wasm32`
  (`crates/happenstance-testkit/src/lib.rs:101-110`, `:117-118`) — are stated as a
  documented, reasoned non-invocation, not left as an unexplained absence.
- **AC-004 — The `workerd` run is inside the gate, not beside it.** A single
  `cargo xtask ci` on a clean checkout executes the `workerd` run; the step is
  registered in `xtask/src/main.rs`'s step list **by name, not by index** (the defect
  `RUNBOOK.md:735-737` and `:761-762` name twice); and a configuration in which the
  step silently does not run fails the gate rather than skipping it.
- **AC-005 — ES-6 is decided with an artefact.** `CloudflareEventStoreError` carries
  a real `worker::Error`, and a committed test reconstructs from a caller-visible
  error the one fact a caller must branch on — constraint violation versus transport
  fault. The outcome (bound added, or ADR-0009's deferral confirmed with the compiled
  reason from a real `!Send` error type) is recorded as a decision atom, not as prose.
- **AC-006 — ADR-0023 is accepted**, states the `SqlStorage` mapping and the off-tokio
  harness as one question with the alternatives that lost, and is authored through
  the ingest path rather than hand-written into `.kb/decisions/`.
- **AC-007 — The two non-type-error capability limits have stated resolutions.**
  (a) The non-snapshot cursor versus ES-9's laziness requirement, and (b) the 2^53
  position ceiling reported through `CloudflareEventStoreError::StoredPosition`
  (`crates/happenstance-cloudflare/src/lib.rs:100-112`). Each is honoured, or reported
  as a declined capability with a reason, or the clause is amended by decision record
  and the suite re-run — never a silent pass.
- **AC-008 — CF-39 and CF-40 are discharged and CF-40's ownership is resolved.** The
  fixture declares the numeric limits it actually has, and
  `.kb/open-questions/cf-40-fixture-limits-ownership.md` is updated to a resolved
  state (not deleted) with the owning document named, coordinated with
  `sqlite-durable-store` so the answer is minted once.
- **AC-009 — CF-14 and CF-27's deferrals are re-read on this runtime**, with a written
  statement per clause of whether the deferral still holds here; CF-27's completeness
  half is handed to `retention-and-incomplete-logs` rather than answered.
- **AC-010 — The ES-32 tail-seam verdict is on disk.** One paragraph in `RUNBOOK.md`'s
  ledger saying whether a Durable Object's storage API makes a tail or subscription
  seam cheap enough to reopen post-0.1. Recorded, not acted on.
- **AC-011 — WF-11's falsifier has been tested here.** Whether this runtime's memory
  ceiling forces a peer to forward a payload it cannot buffer through a human-readable
  encoder is answered with evidence, and the open-question atom reflects it. No wire
  format change is made in this project.
- **AC-012 — The crate is publish-ready and the gate is green.** `happenstance-cloudflare`
  is claimed on crates.io, `publish = false` is removed, both licence files and a
  README are present so a `cargo package --list` assertion would hold, and
  `cargo xtask ci` is green including all four `wasm32` steps
  (`xtask/src/main.rs:192-283`). Whether the crate is actually published stays with
  `publication-and-positioning`.

## Definition of done (boundary-level)

Observable at this project's boundary, by someone who did not do the work.

1. From a clean checkout, one `cargo xtask ci` runs the conformance suite under
   `workerd` on `wasm32` and is green; deleting the `workerd` step or emptying its
   target fails the gate with a legible message rather than passing quietly (the
   `proof-artefact` precedent, `xtask/src/main.rs:156-191`).
2. The suite's output for that run names every rule, with each declined rule carrying
   the fixture's stated reason — inspectable in the gate's own output.
3. `grep`ing `crates/happenstance-cloudflare/` finds no `todo!()` and no
   `#![allow(clippy::todo)]`.
4. A test in the adapter's own tree recovers the constraint-violation-versus-transport
   distinction from a caller-visible error, and fails if the error type stops carrying
   it.
5. ADR-0023 is an accepted atom under `.kb/decisions/`; `redkiln validate --kb` and
   `redkiln doctor` are clean; the CF-40 and WF-11 open-question atoms are resolved
   rather than deleted.
6. `RUNBOOK.md`'s phase 9 row and ledger carry the ES-32 verdict and the phase's exit
   criteria are ticked (`RUNBOOK.md:160`, `:4294-4305`).
7. Story-grain and project-grain verification held throughout:
   `cargo xtask affected --base main` per story, `cargo xtask ci --fast` for this
   non-terminal project (`CLAUDE.md` **Commands**).

## Dependencies

From the DAG in `.bklg/from-contract-to-published-library/_decomposition.md`,
**Sequencing**.

**Depends on** — nothing. This project is one of the two DAG roots and can start on
day one alongside `projection-store-freeze` (HS-P0010). Its in-tree prerequisites are
already discharged: RUNBOOK phases 2 (the instrument portfolio) and 4 (the contract
freeze, which owns the `EventId` and `recorded_at` columns every store's migration 1
needs) are both `done` (`RUNBOOK.md:152`, `:154`, `:4246-4252`).

**Unlocks** — `publication-and-positioning` (HS-P0016). It is a blocker of `0.2.0` by
a decision taken at the decomposition gate: publishing after SQLite alone would
publish a one-adapter compliance claim, and AC-08 requires the claim to name which
implementations it was checked against
(`.bklg/from-contract-to-published-library/_decomposition.md`, **Decisions taken at
the gate** §1). Transitively it unlocks `replication-identity-and-ingest` (HS-P0017),
which satisfies `RUNBOOK.md:164`'s phase 13 → phase 9 dependency by rank rather than
by a direct edge.

**Concurrency.** Merge positions 4, 5 and 6 (`cloudflare`, `postgres-and-neon`,
`ladybug`) are mutually unordered and touch disjoint crates
(`RUNBOOK.md:239-242`). No adapter may depend on another adapter (`CLAUDE.md`,
dependency rule).

## Risks and coupling notes

- **DoD 4 may not be observable at acceptable cost, and that is a blocking finding.**
  `RUNBOOK.md:4267-4268` scopes the `workerd` harness as `vitest-pool-workers` **in
  its own CI job**, while AC-07 and DoD 4 require it *in the same run as the rest of
  the gate*. Those are not the same artefact. The initiative's acceptance criterion
  is the higher bar and wins; reconciling it with the runbook's shape is ADR-0023's
  first job. If a `workerd` runner cannot be made to exist inside `cargo xtask ci` at
  acceptable cost, that is a blocking finding to escalate, not a degradation to absorb
  (`_intake-brief.md`, **Open Questions**).
- **A probe-gated step is not a guard.** `xtask` skips an OPTIONAL step when its tool
  is absent, and `xtask/src/main.rs:197-202` states the rule this project must not
  break: *a constraint whose only check is skippable is unguarded on every machine
  that lacks one tool*. If the `workerd` runner has to be probed, the project owes an
  explicit answer for how DoD 4 stays observable — the same reasoning
  `postgres-and-neon-stores` faces for Docker (`RUNBOOK.md:4356-4358`, `:4382`).
- **"Every rule" needs a precise referent.** Three rule families ship, and one of them
  — `event_store_concurrency_conformance!` — binds `F::Store: EventStore + Send`, is
  `cfg`-ed off `wasm32`, and *a `!Send` adapter cannot invoke it and is not expected
  to* (`crates/happenstance-testkit/src/lib.rs:101-110`, `:117-118`). AC-002's claim
  is scoped to the event-store family (and the model family where the fixture
  supports it); the concurrency family's non-invocation must be a stated reason, or
  AC-003 is being satisfied by silence — exactly the failure BR-13 exists to prevent.
- **ADR-0001's "full proof" cannot be recorded by editing ADR-0001.**
  `RUNBOOK.md:4280-4282` asks for the marker "formally retired and this adapter
  cited", but the marker was already lifted at phase 1 (`RUNBOOK.md:340-345`) and
  `.kb/decisions/0001-async-port-flavours.md` is an accepted, immutable atom. The
  citation therefore belongs in ADR-0023 and in the long-form record under
  `references/adr/`, not in a line edit — `redkiln validate --kb` checks accepted
  atoms against `HEAD` and will catch the attempt.
- **CF-40's ownership has two candidate forcing phases.** The open-question atom names
  phase 8 (`happenstance-sqlite`) as what forces it
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md`, *What forces it*), and
  `sqlite-durable-store` merges at position 3 — one ahead of this project. Both
  projects must not mint an answer. Whichever reaches it first owns the resolution;
  the other cites it. Settle this in the architecture brief rather than at
  atom-authoring time.
- **The stand-in has already made findings that the real bindings can invalidate.**
  `crates/happenstance-cloudflare/src/lib.rs:33-95` records four, including that a
  real `JsValue` is `Send + Sync` on Workers builds without `atomics` — which is why
  `JsHandle` holds an `Rc<str>` instead. Swapping in `worker` must not silently
  restore `Send`-ness through the escape hatch the workspace's
  `unsafe_code = "forbid"` denies the adapter itself; the `!Send` probes at
  `crates/happenstance-cloudflare/src/lib.rs:137-259` are the standing detector and
  must survive the swap.
- **The 2^53 ceiling is a real semantic narrowing.** `SequencePosition` is a
  `NonZeroU64` and Workers SQL widens integers through a JS number, so a conformant
  position is not always round-trippable here
  (`crates/happenstance-cloudflare/src/lib.rs:109-112`). This interacts with
  `.kb/decisions/0013-position-assignment-and-visibility.md` and with the rule against
  asserting on literal positions (`CLAUDE.md`, *The rule that matters*); it is a
  declared store limit, never an excused rule.
- **The non-snapshot cursor is a genuine conflict, not a wrinkle.** ES-9 requires a
  lazy stream, and Cloudflare documents that a cursor held across an `await` is not a
  stable snapshot (`crates/happenstance-cloudflare/src/sql_storage.rs:13-17`). Both
  candidate fixes cost something real — buffering the whole result set defeats
  streaming a large replay; "a snapshot only until the first `await`" is a weaker
  promise than `.kb/decisions/0011-read-laziness-and-isolation.md` made. If neither is
  acceptable, the clause gives, and that is a decision record and a re-plan.
- **`references/seeds/remaining-runway.md:88-91`** is the standing statement that work
  which quietly drops this has changed the product. Schedule pressure on this project
  is the risk register's named "constrained-runtime flavour quietly dropped" entry
  (`initiative.md`, **Risks**).

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — vision, BR-01…BR-17, AC-01…AC-15, DoD 1…16
- [`../_decomposition.md`](../_decomposition.md) — the ten projects, the traceability
  matrix, the DAG and the four gate decisions
- [`_intake-brief.md`](_intake-brief.md) — this project's approved problem, constraints
  and clause ledger

Knowledge base:

- `.kb/decisions/0001-async-port-flavours.md` — the two-flavour design and why this
  target is why it exists
- `.kb/decisions/0008-one-derivation-for-both-ports.md` — one derivation scheme, both
  ports, and what a provided body owes
- `.kb/decisions/0009-error-send-sync.md` — `Error` stays unbounded; the strength goes
  in a marker. This adapter is its test
- `.kb/decisions/0010-the-suite-must-prove-itself.md` — CF-1…CF-29, the fixture shape,
  and how rules are emitted for runtimes that are not tokio
- `.kb/decisions/0011-read-laziness-and-isolation.md` — ES-9's laziness and the read
  sample
- `.kb/decisions/0012-append-shape-and-preconditions.md` — CF-39 and `MID_BATCH_FAULT`'s
  neighbourhood
- `.kb/decisions/0013-position-assignment-and-visibility.md` — position assignment,
  gaps and reuse
- `.kb/decisions/0015-validated-identifiers-and-store-limits.md` — where CF-40 was
  minted
- `.kb/decisions/0016-the-wire-format.md` — WF-11 and the base64 half
- `.kb/open-questions/cf-40-fixture-limits-ownership.md`
- `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
- `.kb/maps/decision-map.md` — the supersession graph, and why an accepted atom is
  never edited
- `references/adr/` — the long-form records; cite these by `file:line`

Code and plan:

- `crates/happenstance-cloudflare/src/lib.rs` — the four findings, the two capability
  limits, and the `!Send` probes
- `crates/happenstance-cloudflare/src/sql_storage.rs`, `js.rs`, `send_shape.rs`,
  `event_store.rs` — the stand-in this project replaces
- `crates/happenstance-testkit/src/lib.rs:55-89` — the three emitters, `__emit_wasm`,
  and the single rule enumeration
- `crates/happenstance-testkit/src/fixtures.rs`, `crates/happenstance-testkit/src/contract.rs`
  — `Fixture`, `Capability`, and the reference implementation
- `crates/happenstance-testkit/tests/local_conformance.rs` — the existing `!Send` /
  wasm harness half
- `crates/happenstance-core/src/memory.rs` — the two tests that pin `read`'s shape
- `xtask/src/main.rs:105-283` — the gate's step list, the four `wasm32` steps, and the
  by-name-not-by-index rule
- `spec/SPECIFICATION.md` — CF-14, CF-27, CF-39, CF-40, ES-6, ES-7, ES-9, ES-17,
  ES-32, WF-11. Where this body and a clause disagree, the clause wins
- `RUNBOOK.md:160`, `:4241-4307` — phase 9's goal, work, proof artefact and exit
  criteria; `:302` the ADR queue row; `:244-258` the ordering that must not be
  reordered
- `references/seeds/remaining-runway.md:88-91` — the approved statement of what
  dropping this costs
- `CLAUDE.md` — the binding constraints, the dependency rule, and the rule that matters

## Companions

- [`_intake-brief.md`](_intake-brief.md) — this project's intake of record
- [`_decomposition.md`](_decomposition.md) — this project's warranted briefs
  (architecture, testing, deployment), authored by `plan-briefs`
- [`_storymap.md`](_storymap.md) — the vertical-slice story map covering AC-001…AC-012
- [`../initiative.md`](../initiative.md) — the parent initiative
- [`../_decomposition.md`](../_decomposition.md) — the initiative's decomposition of
  record
