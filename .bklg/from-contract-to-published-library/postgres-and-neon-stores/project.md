---
id: HS-P0014
uid: 4ed3a4
type: project
slug: postgres-and-neon-stores
title: The two stores that disagree with the port
parent: HS-I0006
initiative: from-contract-to-published-library
project: postgres-and-neon-stores
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
updated: 2026-08-12T12:57:12.987Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# The two stores that disagree with the port

## One-line objective

Turn `happenstance-postgres` and `happenstance-neon` from instruments into
*passing* implementations, so that the frozen `EventStore` contract has been run
by a store that does not serialise its writers and by a store with no connection,
no interactive transaction and no cursor — or the contract is amended by decision
record and the suite re-run.

## How this advances the initiative

The initiative's central claim is that the contract has been implemented three
ways that genuinely disagree (initiative BR-02, AC-06). Today it has not been
implemented at all: every body in both crates is `todo!()`
(`crates/happenstance-neon/src/lib.rs:4-8`,
`crates/happenstance-postgres/src/event_store.rs:3-5`), and every implementation
that has ever passed the suite serialises its writers and assigns positions under
a lock held to commit — `MemoryEventStore`, a `RefCell` store, the rusqlite
skeleton and a Durable Object stand-in, which is one storage shape wearing four
hats (`RUNBOOK.md:665-674`).

The instrument-portfolio table names seven axes and marks two of them **empty at
both ends**: position allocation's far end is `happenstance-postgres`, transport's
far end is `happenstance-neon`, and a fixture instrument fills neither — CF-26
says so in terms (`RUNBOOK.md:685-702`). The five clauses `SPECIFICATION.md` §1.3
names as carrying the CF-25 residual exposure are **ES-10, ES-11, ES-12**, ES-35
and ES-40; this project is the scheduled owner of the first three
(`RUNBOOK.md:606`). Until they are discharged, every `[FROZEN]` marker on them is
conditional, and `publication-and-positioning` would be auditing a clause ledger
whose strongest markers rest on nothing.

Concretely, this project is the sole owner of initiative **DoD 5** (a store that
does not serialise its writers passes the suite, with the position-visibility cost
measured rather than estimated) and **DoD 6** (a store with no connection, no
interactive transaction and no cursor passes, or the contract is amended by
decision record and the suite re-run), and of **AC-06** — the adapter author's
storage shape is not quietly assumed
(`.bklg/from-contract-to-published-library/_decomposition.md` traceability matrix).

## In scope (this project)

- **ADR-0024**, the queued decision this phase exists to settle: how the adapter
  buys ES-10's position-visibility invariant when `nextval()` allocates outside
  the transaction — **measured, not preferred** (`RUNBOOK.md:303`,
  `RUNBOOK.md:4325-4342`). Three mechanisms and what each costs the adapter's types
  and its append path are already worked through in
  `crates/happenstance-postgres/src/event_store.rs:26-70`; the throughput arms are
  already measured in `experiments/position-visibility/README.md:13-19`. What is
  *not* settled is the choice, its structural bill, and where that bill is written.
- **The Postgres event store**, implemented against the real server: migration 1
  carrying the phase-4 identity and time columns (`RUNBOOK.md:249-253`), the
  schema at `crates/happenstance-postgres/src/event_store.rs:9-20` plus whatever
  column ADR-0024's mechanism adds, and the append-condition SQL.
- **`PostgresProjectionStore`** against the owned `Batch` that
  `projection-store-freeze` freezes — `type Batch = sqlx::Transaction<'static,
  Postgres>` is already the recorded skeleton shape (`RUNBOOK.md:1447-1451`,
  `references/adapter-shapes.md`).
- **The three conformance families green against a real Postgres** —
  `event_store_conformance!`, `event_store_model_conformance!` and the opt-in
  `event_store_concurrency_conformance!`
  (`crates/happenstance-testkit/src/lib.rs:91-110`) — plus the phase-3 mutant
  harness re-run with `PostgresEventStore` in the pass column
  (`RUNBOOK.md:4354-4355`).
- **`testcontainers` for the Postgres fixture**, pinned to a specific Postgres
  minor, **in its own CI job**, so the default `cargo xtask ci` path stays runnable
  without Docker (`RUNBOOK.md:4356-4358`) — the same reasoning
  `experiments/position-visibility/README.md:21-23` already applies to itself.
- **`happenstance-neon` finished far enough to run the suite over one-shot HTTP**
  against a live Neon `/sql` endpoint, including the capability skips it must
  report — no interactive transaction, no cursor, a 64 MiB response ceiling
  (`crates/happenstance-neon/src/lib.rs:10-29`, `MAX_RESPONSE_BYTES`).
- **The `conflicting_position` question, settled on evidence.** The crate already
  carries a single-statement CTE that computes the probe and the insert on one
  snapshot and projects both, which — contrary to the standing assumption in the
  decision ledger — **keeps** `ConditionViolated::conflicting_position` at the cost
  of an aggregate index scan on every append and a `Serializable` default
  (`crates/happenstance-neon/src/lib.rs:46-77`). This project confirms or refutes
  that against a real endpoint. Its counter-case, `ProbeThenWriteStore`, is the
  implementation that type-checks perfectly and is silently wrong
  (`crates/happenstance-neon/src/lib.rs:31-44`).
- **Capability and numeric-limit declaration for both fixtures**, through the
  declined-capability path rather than by a vanished test
  (`crates/happenstance-testkit/src/lib.rs:44-51`,
  `crates/happenstance-testkit/src/fixtures.rs`), and the guaranteed minima
  VT-21 – VT-24 exercised against two stores that have real ceilings.
- **The crates.io name claims** for `happenstance-postgres` and `happenstance-neon`
  at the start of the work, per phase 0's rule (`RUNBOOK.md:4346-4348`).
- **Removing the skeleton markers**: no `todo!()` on either path, the scoped
  `#![allow(clippy::todo)]` that names this phase deleted
  (`crates/happenstance-neon/src/lib.rs:101-105`), and `publish = false` removed
  from both manifests (`RUNBOOK.md:4383`).
- **A deployment note** for Hyperdrive plus a `worker::Socket`-backed driver on the
  Worker side — a note, explicitly not a supported configuration
  (`RUNBOOK.md:4364-4366`).
- **Keeping `happenstance-neon`'s `wasm32` build green** in the gate, which
  `cargo xtask ci` already runs and `CLAUDE.md` names as one of its four wasm32
  steps.

## Out of scope (this project)

- **Durability and the reopen far end, and a fixture handing out two real handles
  onto one backing store** — `sqlite-durable-store` (initiative DoD 3;
  `RUNBOOK.md:691-692`).
- **Executing the suite on `wasm32` under a real edge runtime.** Neon *compiles*
  for `wasm32-unknown-unknown` and that build stays in the gate, but running every
  rule under `workerd` is `cloudflare-durable-object-store`'s (initiative DoD 4;
  `RUNBOOK.md:4296-4297`). Note the claim boundary the crate already states: it
  implements the **bare** flavour on each target, which is not the same claim as
  satisfying both (`crates/happenstance-neon/src/lib.rs:79-89`).
- **The completeness axis** — a store holding only a suffix of its own log, ES-40,
  CF-27 — `retention-and-incomplete-logs` (`RUNBOOK.md:693`).
- **Freezing `ProjectionStore`, writing the projection suite, or setting the
  capability-declension policy** — `projection-store-freeze`. This project is a
  consumer of all three.
- **The freeze verdict against a structurally unlike batch shape** —
  `ladybug-projection-store` (initiative DoD 8).
- **Whether ES-10 should have been per-boundary rather than global.** Settled at
  phase 4 in `happenstance-core` by ADR-0013
  (`.kb/decisions/0013-position-assignment-and-visibility.md`); the live trade is
  recorded at `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`
  and is **owned by phase 6**, i.e. `projection-store-freeze`. Read here for
  context; reopening it is a new decision atom and a re-plan.
- **Publishing either crate, reserving them in the release train, the MSRV promise,
  the semver diff and the clause-ledger audit** — `publication-and-positioning`.
  ES-10's removal from the runbook's provisional ledger row is that project's to
  reconcile (`RUNBOOK.md:622-635`).
- **General throughput tuning or a benchmark suite.** `event_store_benchmarks!` is
  `sqlite-durable-store`'s work item (`RUNBOOK.md:4207-4213`) and benchmarks are
  not conformance (CF-34). The only measurement owed here is the one ADR-0024
  depends on.
- **Incidental defects found in passing** — they route to the `support` initiative
  per `.redkiln/config.yaml:5`.

## Derived requirements

Expanded from the initiative requirements this project owns (BR-02, BR-03, BR-13,
BR-15; contributes to BR-12) and the two Definition-of-Done scenarios it is the
sole owner of.

1. **DR-1 (BR-02, BR-03).** `PostgresEventStore` must be a passing implementation,
   not a skeleton: every body real, every conformance family run, and the phase-3
   mutant harness re-run with it as a control. A `todo!()` body type-checks against
   any signature (`RUNBOOK.md:3061-3068`), so "it compiles" counts for nothing.
2. **DR-2 (BR-02, AC-06, DoD 5).** The concurrency family must be green under a
   multi-thread runtime against a store whose writers are **not** serialised — the
   first time any adapter in the portfolio clears that bar
   (`RUNBOOK.md:4368-4370`). Whichever ES-10 mechanism is chosen must therefore be
   shown *not* to have reintroduced the single-lock shape; a serialised sequence
   table would make this adapter `MemoryEventStore` with network latency
   (`crates/happenstance-postgres/src/event_store.rs:43-53`).
3. **DR-3 (DoD 5).** ADR-0024 must carry a **number**, not a preference, covering
   both the steady-state cost and the behaviour with a long-running transaction
   deliberately held open on the same database (`RUNBOOK.md:4341-4342`,
   `RUNBOOK.md:4370-4372`). The mechanism arms are already measured
   (`experiments/position-visibility/README.md:13-19`); what this project owes is
   the choice, and the **structural** bill the experiment did not price — `head`
   becoming a frontier, read-your-own-writes not holding, and staleness bounded by
   the longest write transaction in the cluster.
4. **DR-4 (BR-02, DoD 6).** `NeonEventStore` must run the suite over the real
   one-shot `/sql` endpoint. Substituting a pooled Postgres connection destroys the
   exact axis the adapter exists to occupy and is not an acceptable fixture
   (`crates/happenstance-neon/src/lib.rs:10-29`).
5. **DR-5 (BR-13, DoD 6).** Every rule Neon cannot pass must be either a reported
   capability skip carrying the fixture's stated reason, or a clause amended by an
   accepted decision record with the suite re-run — **never a silent pass and never
   a `#[cfg]`-ed-out test** (`RUNBOOK.md:4361-4363`, `RUNBOOK.md:4380-4381`,
   `crates/happenstance-testkit/src/lib.rs:44-51`).
6. **DR-6 (BR-13).** Both fixtures must declare their real numeric ceilings through
   the same declined-capability path, which is the CF-40 surface whose ownership is
   contested between ADR-0012 and ADR-0015
   (`.kb/open-questions/cf-40-fixture-limits-ownership.md`). This project consumes
   whatever `sqlite-durable-store` and `projection-store-freeze` settle there; it
   does not settle it unilaterally.
7. **DR-7 (BR-15).** ADR-0024 lands as an accepted decision atom naming the
   alternatives that lost and why, through the runbook's ADR queue rather than as a
   side effect (`RUNBOOK.md:262-268`, `.kb/maps/decision-map.md`). If DoD 6 forces a
   contract amendment, that is a second decision record with the same bar — a
   `[FROZEN]` clause changes by ADR, never by edit (`CLAUDE.md`).
8. **DR-8 (BR-12).** The `!Send` story must not regress: the Neon crate implements
   the bare `EventStore` on both targets, the concurrency family's `Send` bound is
   Postgres-only by design (`crates/happenstance-testkit/src/lib.rs:101-105`), and
   the wasm32 build of `happenstance-neon` stays in `cargo xtask ci`.
9. **DR-9 (operational).** The default gate must stay runnable with no Docker and
   no network. Live infrastructure — a pinned Postgres container and a real Neon
   endpoint — belongs in dedicated CI jobs (`RUNBOOK.md:4356-4358`,
   `RUNBOOK.md:4382`), so that `.redkiln/config.yaml`'s story grain
   (`cargo xtask affected`) and non-terminal integration grain
   (`cargo xtask ci --fast`) remain honest for every other project.

## Acceptance criteria

Project-grain and testable. This is the spine `_storymap.md` must cover.

- **AC-001 — ADR-0024 is accepted and decides on numbers.** An accepted decision
  atom under `.kb/decisions/` names the chosen position-visibility mechanism, the
  two that lost and why, and carries the measured cost of the choice including its
  behaviour with a long-running transaction held open on the same database.
  `redkiln validate --kb` passes and `.kb/maps/decision-map.md` carries its row.
- **AC-002 — The visibility rule is one the adapter had to work to pass.**
  `PreCommitPositionStore`'s rule
  (`nothing_below_an_observed_position_appears_later`, CF-13) is green against
  `PostgresEventStore`, and ADR-0024 states what that work cost. A naive
  `nextval()` implementation of the same adapter fails it.
- **AC-003 — The concurrency family is green on a store that does not serialise its
  writers.** `event_store_concurrency_conformance!` passes under a multi-thread
  runtime at `concurrency::CONTENDERS`, and the ADR's own measurement shows write
  concurrency was retained rather than traded away.
- **AC-004 — Postgres passes the whole suite for real.** `event_store_conformance!`
  and `event_store_model_conformance!` are green against a live pinned Postgres,
  and the phase-3 mutant harness runs with `PostgresEventStore` in the pass column.
- **AC-005 — `PostgresProjectionStore` passes the projection suite** against the
  owned `Batch` frozen by `projection-store-freeze`, with any declined capability
  reported by name and reason.
- **AC-006 — Neon runs the suite over one-shot HTTP.** `event_store_conformance!`
  is invoked against a `NeonEventStore` fixture backed by a real Neon `/sql`
  endpoint — no connection, no interactive transaction, no cursor — and the run
  completes with every rule reporting pass, fail, or a skip carrying the fixture's
  stated reason.
- **AC-007 — Nothing Neon cannot do passes silently.** For each rule Neon does not
  pass, there is exactly one of: a declared capability with a written reason
  visible in the harness output, or an accepted decision record amending the
  clause with the suite re-run afterwards. No rule is `#[cfg]`-ed out, and no rule
  is absent from the run.
- **AC-008 — `conflicting_position` is settled by evidence, not by assumption.**
  Either the single-statement CTE at `crates/happenstance-neon/src/lib.rs:46-77`
  is shown on a live endpoint to return the conflicting position, or a decision
  record downgrades the guarantee with the measurement or compiler evidence that
  forced it. The decision ledger's standing assumption is corrected either way.
- **AC-009 — The structural bill is written where a consumer meets it.** The
  consequences of the chosen mechanism — whether `head` is a frontier, whether
  read-your-own-writes holds, and what bounds staleness — are documented, and the
  choice between adapter documentation and a specification clause of its own is
  recorded as a decision rather than defaulted.
- **AC-010 — Both stores declare their real limits.** Each fixture declares its
  numeric ceilings (Neon's 64 MiB response cap among them) through the
  declined-capability path, and the guaranteed minima VT-21 – VT-24 either pass or
  are refused with the store-limit error rather than an opaque store error.
- **AC-011 — The default gate stays Docker-free and network-free.**
  `cargo xtask ci` on a clean checkout with no Docker and no credentials is green;
  the Postgres and Neon suites live in their own CI jobs; the wasm32 build of
  `happenstance-neon` is still part of the default gate.
- **AC-012 — Neither crate is a skeleton any more.** No `todo!()` on either path,
  the scoped `#![allow(clippy::todo)]` naming this phase is deleted from both
  crates, `publish = false` is removed, and both names are claimed on crates.io.
- **AC-013 — The far-end discharge is recorded for the publication audit.** The
  status of ES-10, ES-11 and ES-12 after this work — discharged, still exposed, or
  amended — plus which clauses ES-41, ES-42 and VT-21 – VT-24 were exercised
  against, is written down in a form `publication-and-positioning` can read at
  audit time rather than re-derive (`RUNBOOK.md:606`, `RUNBOOK.md:622-635`).

## Definition of done (boundary-level)

1. Initiative **DoD 5** is observed: a store that does not serialise its writers
   passes the suite, with the position-visibility cost measured rather than
   estimated.
2. Initiative **DoD 6** is observed: a store with no connection, no interactive
   transaction and no cursor passes the suite — or the contract is amended by
   decision record and the suite re-run.
3. Every AC-001 … AC-013 above is met and evidenced in the story ledgers
   (`.redkiln/config.yaml:62-67`, `require_ledger: true`).
4. `cargo xtask ci --fast` is green on the project's tree — the bar
   `.redkiln/config.yaml:50-55` sets for a non-terminal project — and the two
   live-infrastructure CI jobs are green on the same tree.
5. ADR-0024 (and any amendment record DoD 6 forces) is accepted on disk;
   `redkiln validate --kb` and `redkiln doctor` are clean, with `doctor` still
   reporting exactly the six expected `template-drift` advisories.
6. `references/adapter-shapes.md` and `RUNBOOK.md`'s phase-10 session log record
   what the two adapters told the type checker and the server that the skeletons
   could not, including anything that contradicts a prior assumption.

## Dependencies

From the initiative decomposition's DAG
(`.bklg/from-contract-to-published-library/_decomposition.md`, *Sequencing*). The
`blocked_by` / `blocks` frontmatter fields are the CLI's to write; this section is
the planning record of the same edges.

**Depends on**

- `projection-store-freeze` (HS-P0010) — supplies the frozen `ProjectionStore`
  batch shape `PostgresProjectionStore` is written against, the projection suite
  it is run against, and the capability-declension policy Neon's skip list is
  reported under (`RUNBOOK.md:161`, phase 10 depends on 2, 4 and 6).
- Already discharged, and named so nobody re-derives it: **phase 4** — `EventId`
  and `recorded_at` are migration-1 columns in every store, and phase 4 is `done`
  (`RUNBOOK.md:249-253`, `RUNBOOK.md:154`).

**Unlocks**

- `publication-and-positioning` (HS-P0016) — `0.2.0` deliberately waits for all
  four adapter projects, because AC-08 requires the published compliance claim to
  name which implementations it was checked against
  (`_decomposition.md`, *Decisions taken at the gate*, item 1).
- Downstream of that, `replication-identity-and-ingest` (HS-P0017): phase 13 needs
  three real stores because proving a port takes two unlike implementations and an
  oracle (`RUNBOOK.md:239-242`).

**Parallel, not dependent** — `sqlite-durable-store`, `cloudflare-durable-object-store`
and `ladybug-projection-store` touch disjoint crates and may be interleaved with
this project freely (`RUNBOOK.md:239-242`).

## Risks and coupling notes

| Risk | Why it is live here | Mitigation |
| --- | --- | --- |
| The ES-10 mechanism chosen re-serialises writers, and the adapter stops occupying the axis it exists for | The serialised sequence table dissolves the problem by throwing away the write concurrency that was the reason to reach for Postgres (`crates/happenstance-postgres/src/event_store.rs:43-53`) | AC-003 makes retained write concurrency an acceptance criterion, not a side effect; the concurrency family under a multi-thread runtime is the instrument |
| `xid8` + `pg_snapshot_xmin` measures at ~1.0× and still costs something the benchmark cannot see | One forgotten `BEGIN` in an unrelated application becomes a ceiling on every reader and every projection (`RUNBOOK.md:4331-4335`) | DR-3 and AC-009 make the structural bill an explicit deliverable, measured with a transaction deliberately held open |
| The Neon endpoint gets faked with a pooled Postgres connection because live HTTP is inconvenient in CI | It is the cheapest possible shortcut and it destroys the transport axis entirely | DR-4 and AC-006 state the fixture requirement; the deployment brief owns how a real endpoint is reached from CI without putting credentials in the default gate |
| The intake brief's `conflicting_position` premise is already contradicted in-tree | The crate's own docs show a CTE that keeps it (`crates/happenstance-neon/src/lib.rs:46-77`), while the decision ledger assumes Neon forces it down to a hint | AC-008 requires the question to be closed by running the CTE against a real endpoint, and the ledger corrected in whichever direction the evidence points |
| A contract amendment under DoD 6 is taken as a line edit | Nothing `[FROZEN]` may be amended by edit (`CLAUDE.md`; initiative *Out of scope*) | DR-7: an amendment is a new decision atom, reviewed, with the suite re-run afterwards — which the initiative's DoD 6 explicitly contemplates |
| Live-infrastructure jobs become flaky and get quietly weakened | Two of this project's four gates need a server | DR-9 and AC-011 keep the default gate free of both; a flaky live job is fixed or reported, never made non-blocking without a recorded decision |
| The visibility question is reopened as per-boundary in passing | The open atom is real and its cost is measured at ~9% at 64 clients (`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`) | It is **owned by phase 6** — `projection-store-freeze` — and named in *Out of scope*; this project consumes the answer |
| The projection batch shape moves after `PostgresProjectionStore` is written | AC-005 is written against a freeze that lands in an upstream project | The `blocked_by` edge is exactly this; if `projection-store-freeze` changes the shape after merge, that is a re-plan input, not a silent fix |
| CF-40's contested ownership blocks the numeric-limit declaration | Two accepted decisions claim and disclaim it (`.kb/open-questions/cf-40-fixture-limits-ownership.md`) | DR-6 consumes whatever the earlier adapter project settles; this project does not resolve the ownership question unilaterally |

**Coupling notes.** Postgres and Neon are deliberately **one project, not two**
(`_decomposition.md`, *Decisions taken at the gate*, item 2): Neon is Postgres over
one-shot HTTP, inheriting migration 1, the tag storage and the append-condition
SQL, and both are settled by one ADR pass. Splitting them would create a horizontal
seam — schema below, transport above — rather than a vertical one. It is the
largest non-trunk item on the plan at ~11 runbook-days (`RUNBOOK.md:4387`) and is
the most reasonable candidate for a split if project grain runs long; that call
belongs to `_storymap.md`, not to a re-decomposition.

## Context anchors

Initiative and plan:

- [`../initiative.md`](../initiative.md) — BR-02, BR-03, BR-12, BR-13, BR-15;
  AC-06; DoD 5 and DoD 6
- [`../_decomposition.md`](../_decomposition.md) — the traceability matrix, the
  scope seams, and the DAG edge into `publication-and-positioning`
- `RUNBOOK.md:4311-4390` — phase 10 in full: goal, the one decision that is not a
  storage preference, work items, proof artefact, exit criteria
- `RUNBOOK.md:161` — the phase-10 status row and its dependencies (2, 4, 6)
- `RUNBOOK.md:303` — ADR-0024's queue row and the single question it answers
- `RUNBOOK.md:606` — the residual-exposure row naming ES-10, ES-11, ES-12 and this
  phase as their owner
- `RUNBOOK.md:622-635` — why ES-41 and ES-42 are missing from the table that phase
  12 audits
- `RUNBOOK.md:665-702` — the instrument portfolio: seven axes, and why neither a
  skeleton nor a fixture fills a far end
- `RUNBOOK.md:239-258` — what parallelises and what must not be reordered

Knowledge base (Accepted decisions and open questions):

- `.kb/decisions/0013-position-assignment-and-visibility.md` — ES-10 as a global
  invariant; the long form is `references/adr/0013-position-assignment-and-visibility.md`
- `.kb/decisions/0012-append-shape-and-preconditions.md` — `append`'s shape,
  preconditions and the fixture's fault capability
- `.kb/decisions/0015-validated-identifiers-and-store-limits.md` — the two kinds of
  bound, and where CF-40 was minted
- `.kb/decisions/0001-async-port-flavours.md`,
  `.kb/decisions/0009-error-send-sync.md` — the two-flavour design and the
  unbounded `Error`, both of which Neon's bare-flavour claim rests on
- `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` — read
  for context, **owned by phase 6**, not reopened here
- `.kb/open-questions/cf-40-fixture-limits-ownership.md` — the fixture-limits
  surface DR-6 consumes
- `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` — a `[FROZEN]`
  marker does not assert that anything checks the clause; the same distinction this
  project exists to close for ES-10 – ES-12
- `.kb/maps/decision-map.md` — where ADR-0024's row lands

Specification and cases:

- `spec/SPECIFICATION.md:215-221` — 200 clause IDs, 198 normative: 139 frozen,
  49 provisional, 10 deferred. Where this body and a clause disagree, the clause wins
- `spec/E2E-CASES.md` — E2E-01, which this project makes writable against a store
  that can genuinely fail it (`RUNBOOK.md:4385`)

Code:

- `crates/happenstance-postgres/src/event_store.rs:1-70` — the intended schema, why
  `position` is deliberately not `bigserial`, and what each candidate mechanism
  costs the adapter's types
- `crates/happenstance-neon/src/lib.rs:1-130` — the capability table, the
  `ProbeThenWriteStore` trap, the CTE that keeps `conflicting_position`, the
  bare-flavour claim, and why the crate owns no HTTP client
- `crates/happenstance-testkit/src/lib.rs:24-160` — what a fixture is, capability
  declension with a stated reason, and the three rule families
- `crates/happenstance-testkit/src/fixtures.rs` — `MemoryFixture`, the reference
  fixture to read before writing either of this project's two
- `references/adapter-shapes.md` — what the six skeletons already told the type
  checker; the regression tolerance after the freeze is `RUNBOOK.md:3070-3073`
- `experiments/position-visibility/README.md` — the reproducible measurement, its
  four arms, and why `fsync=on` is the setting the whole experiment stands on
- `.redkiln/config.yaml:28-73` — the verify grains this project is gated by
- `CLAUDE.md` — the binding constraints, the rule that matters, and the four
  wasm32 steps in `cargo xtask ci`

## Companions

Board-invisible drill-down for this card:

- [`_intake-brief.md`](_intake-brief.md) — the approved problem, constraints,
  non-goals, proof artefact and clause list
- [`_decomposition.md`](_decomposition.md) and [`_grounding.md`](_grounding.md) —
  this project's briefs, authored at the `briefs` stage by `plan-briefs`; the
  warranted set for this project is **architecture, testing and deployment**
  (`../_decomposition.md`, *Warranted briefs*)
- [`_storymap.md`](_storymap.md) — the vertical-slice story map covering
  AC-001 … AC-013
- [`../initiative.md`](../initiative.md) — the initiative charter
- [`../_decomposition.md`](../_decomposition.md) — the ten-project portfolio, its
  traceability matrix and its DAG
