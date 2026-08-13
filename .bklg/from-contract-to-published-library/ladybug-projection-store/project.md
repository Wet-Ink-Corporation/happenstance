---
id: HS-P0015
uid: a8775f
type: project
slug: ladybug-projection-store
title: The unlike batch shape, and the freeze verdict
parent: HS-I0006
initiative: from-contract-to-published-library
project: ladybug-projection-store
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
updated: 2026-08-12T12:57:13.298Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# The unlike batch shape, and the freeze verdict

## One-line objective

Fill in `happenstance-ladybug`'s `todo!()` bodies against the real `lbug` driver,
run the projection conformance suite on a graph-shaped batch, and write the dated
verdict on whether `projection-store-freeze`'s freeze held — held or did not hold,
stated either way.

## How this advances the initiative

The initiative's third goal is that *"`ProjectionStore` earns its freeze, and the
freeze is tested afterwards"* ([`../initiative.md`](../initiative.md), Goals), and its
risk register names the mitigation directly: the freeze *"is not called earned until a
structurally unlike batch shape has passed, and the verdict on whether it held is
written after that, not at the freeze"*. This project is that clause.

It matters because of what `ProjectionStore` will have been frozen against.
`RUNBOOK.md:598` records that **seven** provisional PS clauses (PS-4 – PS-6, PS-9,
PS-11, PS-12, PS-15) are falsified by *"PS-2 alone — `CheckpointOnlyStore` passing, or
a third adapter disagreeing with the two that froze it"*, and `RUNBOOK.md:602` puts
PS-34's E0195 spelling trap under the same shape: *"a third implementer hitting it
after the diagnostic is documented"*. Both rows carry the owning phase as **"6,
re-tested 11"**. This project is the re-test, and it is the only place in the portfolio
where a projection port meets a store that has **no transaction handle type at all**
(`crates/happenstance-ladybug/src/projection_store.rs:5-12`) and a **synchronous**
driver behind an `async` port (`…/projection_store.rs:39-47`).

`CLAUDE.md`'s house rule states the general form: *"a port is only as well-designed as
the spread of what implements it."* Every projection implementation that will have
passed the suite by the time this project starts is SQL-shaped or in-memory. Ladybug is
the other end of the batch-shape-and-transactional-seam axis, and the far end was built
as a skeleton in phase 2 precisely so this run would be possible
(`crates/happenstance-ladybug/src/lib.rs:5-8`).

It carries **BR-03** and **BR-04ʳ** (the re-test half), **DoD 8**, and its own **ADR-0025**
under BR-15; it exercises **BR-02**, **BR-13**, **AC-04**, **AC-05** and **DoD 7** without
owning them ([`../_decomposition.md`](../_decomposition.md), Traceability matrix).

## In scope (this project)

- **Claiming `happenstance-ladybug` on crates.io**, per phase 0's rule that a name is
  reserved when its phase starts and not before, *"the point being that by now there is
  a crate to justify it with"* (`RUNBOOK.md:4411-4413`, `RUNBOOK.md:826-829`).
- **ADR-0025**, answering the three questions the runbook's queue assigns it
  (`RUNBOOK.md:304`, `RUNBOOK.md:4415-4419`): checkpoint placement (in the graph as a
  node or beside it), how a projection expresses graph mutations (raw Cypher or a typed
  builder), and whether `lbug`'s synchronous API is wrapped in `spawn_blocking` or the
  adapter is offered as blocking-only.
- **Adding the real `lbug` dependency and measuring the cold build.** It compiles
  LadybugDB's C++ through `cxx` and `cmake`, and docs.rs itself fails to build `lbug`
  0.19.1 — *"the same cost seen from outside"*
  (`crates/happenstance-ladybug/src/lib.rs:24-32`).
- **Implementing `SendProjectionStore` for real** on `LadybugProjectionStore`, replacing
  the four `todo!()` bodies at `crates/happenstance-ladybug/src/projection_store.rs:270-292`
  and removing the scoped `#![allow(clippy::todo)]` whose comment already names this
  phase (`…/lib.rs:69-72`).
- **A written, dated definition of "structurally unlike"**, committed *before* the first
  suite run.
- **Running the projection conformance suite** — the entry point
  `projection-store-freeze` delivers ([`../projection-store-freeze/_intake-brief.md`](../projection-store-freeze/_intake-brief.md))
  — against a Ladybug fixture, with declined capabilities reported with the fixture's
  stated reason rather than absent, on the pattern
  `crates/happenstance-testkit/src/lib.rs:46-50` sets for the event-store suite.
- **Closing the one half of PS-4 no skeleton could close**: a projection that must
  `MATCH` a node it created earlier in the same batch. *"That is a run-time capability
  limit, not a compile error. Phase 11 closes it by writing a projection that needs
  read-your-own-writes"* (`…/projection_store.rs:49-64`).
- **Re-testing PS-34's E0195 trap with a third pair of hands**, and recording whether
  the documented diagnostic was enough (`…/live_handle.rs:68-83`).
- **The freeze verdict document**, dated, naming what it was checked against.
- **The CI decision** — a dedicated job for this crate if the build cost is material,
  *"rather than slowing the three-OS gate everyone runs"* (`RUNBOOK.md:4424-4425`) —
  recorded in `RUNBOOK.md`'s phase 11 body.

## Out of scope (this project)

- **The freeze itself, the suite, and its rules** → `projection-store-freeze` (HS-P0010).
  It owns PS-4 – PS-15, PS-16 – PS-20, PS-26 – PS-30, DT-3, DT-8 and
  `CheckpointOnlyStore`. This project runs its suite; it does not write it.
- **Acting on a "did not hold" verdict by amending a published surface** → a new
  decision atom and a re-plan ([`../_decomposition.md`](../_decomposition.md), Scope
  seams). Nothing `[FROZEN]` is amended here.
- **The PS-3 verdict** on whether the port ships behind `unstable-projection` →
  `publication-and-positioning` (HS-P0016). `RUNBOOK.md:601` marks PS-3 *"6, decided at
  12"*; this project supplies a data point, not the decision.
- **Any event store.** Ladybug is a projection store only, at any grain: *"an event log
  needs a monotonic append with a conditional write … forcing the event store port onto
  it would produce something that satisfies the trait and not the specification"*
  (`crates/happenstance-ladybug/src/lib.rs:10-22`, `RUNBOOK.md:4402-4405`). Event stores
  → `sqlite-durable-store`, `cloudflare-durable-object-store`, `postgres-and-neon-stores`.
- **BR-12, the `!Send` flavour proof** → `cloudflare-durable-object-store` (HS-P0013).
  This adapter implements the **`Send`** flavour, and on evidence rather than
  convenience: `lbug`'s `Database` and `Connection` are both `Send + Sync`
  (`…/projection_store.rs:252-257`). The decomposition matrix leaves this project's
  BR-12 cell blank deliberately.
- **The `Projection` trait and the projection runner** → `typed-layer-and-alpha-release`
  (HS-P0011).
- **Whether this crate is published at `0.2.0`** → `publication-and-positioning`. The
  decomposition's working default is three crates only, and that project's deployment
  brief decides it ([`../_decomposition.md`](../_decomposition.md), *Carried into the
  briefs*). This project makes the crate *ready* to be published; it does not decide.
- **Incidental defects found in passing** → the `support` initiative, per
  `.redkiln/config.yaml:5`.

## Derived requirements

Expanded from the initiative requirements this project owns or exercises.

- **DR-1 (from BR-03).** No `todo!()` may remain on any `happenstance-ladybug` path, and
  the crate-level `#![allow(clippy::todo)]` must be deleted rather than narrowed. A
  `todo!()` body type-checks against any signature, so a skeleton counted as an adapter
  is decorative by construction (`CLAUDE.md`, repository map; `…/lib.rs:69-72`).
- **DR-2 (from BR-03).** The `stand_in` module — the cited stub types the skeleton used
  in place of the driver (`…/lib.rs:24-32`, `…/projection_store.rs:68`) — must be
  retired when the real `lbug` types land, not left beside them. Two type universes in
  one crate is how a suite ends up running against the stand-in.
- **DR-3 (from BR-04ʳ).** "Structurally unlike" must be defined in writing, with the
  axes named, and committed before the first suite run — so the definition cannot be
  chosen afterwards to match the result. The commit order is the evidence.
- **DR-4 (from BR-04ʳ, DoD 8).** The verdict is required whichever way the run goes, and
  must name: which implementation, which rules, which PS clause ids, at which commit,
  on which date. A verdict that says only "held" is not a verdict.
- **DR-5 (from BR-13, AC-05).** Every rule the Ladybug fixture cannot satisfy is a
  *declared* capability declension carrying the fixture's stated reason and still
  appearing in the run — never a silent absence and never a `#[cfg]`-out
  (`crates/happenstance-testkit/src/lib.rs:46-50`). Consumes
  `.kb/open-questions/cf-40-fixture-limits-ownership.md`; the policy itself is
  `projection-store-freeze`'s.
- **DR-6 (from BR-15).** ADR-0025 states the alternatives that lost and why — for each
  of its three questions, not once for the whole record. An accepted decision atom is
  immutable, which is what makes it the only durable record of a rejected option
  (`CLAUDE.md`, *Where the work lives*).
- **DR-7 (from BR-15).** `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`
  is *consumed* here as evidence about whether the seam chosen at phase 6 survived a
  third implementer, not rediscovered. Its resolution is
  `projection-store-freeze`'s; this project reports on it.
- **DR-8 (from the initiative's ordering).** The verdict must land before
  `publication-and-positioning` opens, so that a "did not hold" is heard before the
  surface is published rather than after. This project `blocks` publication
  ([`../_decomposition.md`](../_decomposition.md), Dependency DAG) and the timing must be
  stated explicitly in the design brief rather than inferred from the DAG.
- **DR-9 (from BR-02, exercised).** The suite run must be executable by someone who did
  not write this adapter: the fixture, the command and the expected output are recorded,
  because "passed against N adapters" is a snapshot and the snapshot has to be
  re-takeable.

## Acceptance criteria

Project-grain and testable. This is the spine the story map must cover.

- **AC-001 — The definition of "structurally unlike" is on disk before the first run.**
  A written statement naming the axes on which the Ladybug batch differs from the shapes
  that froze the port (no transaction handle type; a deferred, owned, `Send + 'static`
  write set; a synchronous driver; single-writer concurrency) is committed at an earlier
  commit than the first projection-suite invocation against a Ladybug fixture.
- **AC-002 — ADR-0025 is accepted and answers three questions, each with its losers
  named.** Checkpoint placement, the graph-mutation vocabulary, and how `lbug`'s
  blocking API meets a non-blocking port. `redkiln validate --kb` passes and the atom
  is reachable from `.kb/maps/decision-map.md`.
- **AC-003 — The adapter is real.** `grep -rn "todo!" crates/happenstance-ladybug/`
  returns nothing, `#![allow(clippy::todo)]` is gone from
  `crates/happenstance-ladybug/src/lib.rs`, the crate builds against the real `lbug`
  dependency, and no `stand_in` type appears on any path the suite exercises.
- **AC-004 — The projection conformance suite runs against a Ladybug fixture and every
  rule reports an outcome.** Each rule is pass, or a declined capability printing the
  fixture's stated reason. No rule is absent from the run, and the count of rules
  emitted equals the count `projection-store-freeze` registered.
- **AC-005 — Read-your-own-writes inside one batch has an answer, not a hypothesis.** A
  projection that `MATCH`es a node it created earlier in the same batch exists and is
  executed; the result is recorded as either a supported behaviour or a stated
  capability limit of a deferred write set, closing the half of PS-4 the skeleton
  explicitly could not (`crates/happenstance-ladybug/src/projection_store.rs:57-64`).
- **AC-006 — PS-34's E0195 trap is re-tested by a third implementer and the outcome is
  written down.** Either the documented diagnostic was sufficient for someone meeting
  the port fresh, or it was not; the answer is recorded by name against PS-34 rather
  than inferred from the absence of a complaint.
- **AC-007 — The freeze verdict exists, and it exists either way.** A dated document
  states *held* or *did not hold*, and names the implementation, the rules run, the PS
  clause ids covered and the commit it was run at. Its presence is not conditional on
  the result.
- **AC-008 — A "did not hold" verdict routes to a decision atom and a re-plan, and
  changes nothing frozen.** `cargo xtask spec-trace` is green and no `[FROZEN]` clause
  marker or text in `spec/SPECIFICATION.md` differs from its state at this project's
  start.
- **AC-009 — The build cost is a number and the CI shape follows it.** The `lbug` cold
  build time is measured and recorded, and the decision to give the crate its own CI job
  or to leave it in the three-OS gate is recorded in `RUNBOOK.md`'s phase 11 body with
  the number behind it (`RUNBOOK.md:4420-4425`, `RUNBOOK.md:4437`).
- **AC-010 — The name is held and the crate is package-complete.** `happenstance-ladybug`
  is claimed on crates.io, and the crate carries both licence files and a README so that
  `publication-and-positioning`'s publish decision is never blocked on this crate's
  readiness (`CLAUDE.md`, the `cargo package --list` assertion).
- **AC-011 — The verdict precedes publication.** The verdict document is merged to the
  initiative branch before `publication-and-positioning` opens its gate, and the design
  brief states that ordering explicitly rather than leaving it to the DAG.

## Definition of done (boundary-level)

1. `cargo xtask ci` is green on this project's tree, including `cargo xtask spec-trace`.
2. The projection conformance suite is invoked against a Ladybug fixture and is green,
   with every declension reported and reasoned (AC-004, AC-005).
3. ADR-0025 is accepted on disk; `redkiln validate --kb` and `redkiln doctor` are clean
   (AC-002).
4. The freeze verdict document is written, dated and merged (AC-007, AC-011) — this is
   the initiative's **DoD 8**, and the only artefact this project cannot substitute.
5. No `todo!()` and no `#![allow(clippy::todo)]` remain in `happenstance-ladybug`
   (AC-003).
6. `RUNBOOK.md`'s phase 11 exit criteria are ticked in place, including the build-cost
   and CI decision (`RUNBOOK.md:4432-4438`).
7. Nothing `[FROZEN]` was amended (AC-008).

## Dependencies

**Depends on**

- **`projection-store-freeze` (HS-P0010)** — the frozen port and the
  `projection_store_conformance!` entry point. `RUNBOOK.md:162` gives phase 11 the
  single dependency **6**, and `RUNBOOK.md:188` draws it as a branch that never rejoins
  the trunk: `6 ─▶ 11`. There is no substrate here owned outside this initiative.

**Unlocks**

- **`publication-and-positioning` (HS-P0016)** — by decomposition gate decision 1,
  `0.2.0` waits for all four adapter projects, because AC-08 requires the published
  compliance claim to name *which* implementations it was checked against
  ([`../_decomposition.md`](../_decomposition.md), *Decisions taken at the gate*).

**Unordered siblings.** `sqlite-durable-store`, `cloudflare-durable-object-store` and
`postgres-and-neon-stores` are mutually unordered with this project (merge positions
4, 5, 6) and touch disjoint crates (`RUNBOOK.md:239-242`).

## Risks and coupling notes

- **The suite may accommodate the shape it was written against.** This project is the
  first consumer of `projection-store-freeze`'s suite from outside that project, and a
  suite whose rules were written while looking only at SQL-shaped batches can pass a
  graph batch by not asking the question. Mitigation: AC-001 fixes the axes in writing
  first, and AC-005 forces at least one behaviour the deferred write set can plausibly
  fail.
- **A verdict chosen to match the result.** The single highest-integrity risk in the
  project, and the reason AC-001 is an ordering claim about commits rather than a
  content claim about a document.
- **The GAT and the ICE.** Today's port is implementable *"only by stores that outlive
  every batch lifetime"*, and the failure mode for one that does not is a compiler
  panic, not a diagnostic (`crates/happenstance-ladybug/src/live_handle.rs:33-66`). If
  phase 6 keeps `type Batch<'a> where Self: 'a`, this adapter can only be built
  `'static` — which is a finding about the port and belongs in the verdict, not a
  workaround to absorb quietly.
- **Single writer, many readers.** LadybugDB permits many concurrent readers and exactly
  one writer, so two commits racing is *routine* rather than exceptional
  (`crates/happenstance-ladybug/src/projection_store.rs:204-213`). Any concurrency-shaped
  projection rule will meet `WriteTransactionInUse`; whether that is a declared
  capability limit or a rule defect is a question for the verdict, and the answer must
  not be "retry until green".
- **Build cost is a CI blast radius, not just a wait.** `lbug` builds C++ through `cxx`
  and `cmake` and docs.rs cannot build it (`…/lib.rs:24-32`). Left in the three-OS gate
  it taxes every contributor on every commit; split out, it can silently stop running.
  AC-009 forces the trade to be recorded with a number.
- **The `INT64` / `NonZeroU64` narrowing is bidirectional.** `MalformedCheckpoint` and
  `PositionOutOfRange` exist because collapsing a corrupt checkpoint into `Ok(None)`
  *"would silently replay a projection from the beginning"*
  (`…/projection_store.rs:175-202`). Do not simplify either variant away while filling
  in the bodies.
- **Never assert on literal position values.** The specification permits gaps, and a
  conformant adapter may leave them (`CLAUDE.md`, *The rule that matters*).
- **Coupling to `publication-and-positioning` is one-directional and timed.** DR-8 and
  AC-011: this project blocks publication so that a "did not hold" cannot arrive after
  the surface is published. If schedule pressure inverts that edge, the initiative's
  own exit criterion 4 is what breaks.

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — Goals, BR-03/BR-04/BR-13/BR-15, AC-04/AC-05,
  DoD 7/8, and the risk row this project discharges
- [`../_decomposition.md`](../_decomposition.md) — the traceability matrix, the scope
  seams, the DAG and gate decision 1
- [`_intake-brief.md`](_intake-brief.md) — the approved problem, constraints and clause list
- [`../projection-store-freeze/_intake-brief.md`](../projection-store-freeze/_intake-brief.md)
  — the suite this project runs, and what that project explicitly does not own

Knowledge base:

- `.kb/open-questions/projection-store-batch-has-no-apply-seam.md` — the missing apply
  seam, its ordered sub-questions, and why the first real projection adapter is what
  forces them
- `.kb/open-questions/cf-40-fixture-limits-ownership.md` — capability declension with a
  stated reason; consumed, not rediscovered
- `.kb/decisions/0007-projection-runner-decodes.md` — where the runner lives and the
  indicative `Projection::apply` signature
- `.kb/decisions/0008-one-derivation-for-both-ports.md` — one derivation for `EventStore`
  and `ProjectionStore`, and the GAT-across-a-suspension-point finding
- `.kb/maps/decision-map.md`, `.kb/maps/open-questions-index.md`

Code and specification:

- `crates/happenstance-ladybug/src/lib.rs` — the skeleton's four findings, and the
  `#![allow(clippy::todo)]` this project deletes
- `crates/happenstance-ladybug/src/projection_store.rs` — the deferred write set, the
  error taxonomy, the blocking question, and the four `todo!()` bodies
- `crates/happenstance-ladybug/src/live_handle.rs` — the ICE transcript and the E0195
  transcript, both on rustc 1.97.1
- `crates/happenstance-core/src/projection.rs` — the port itself
- `crates/happenstance-testkit/src/lib.rs` — the fixture and capability pattern the
  projection suite inherits
- `spec/SPECIFICATION.md` — PS-2, PS-3, PS-4 – PS-15, PS-34; the clause wins wherever
  this body disagrees with it
- `spec/E2E-CASES.md` — the third-shape halves of E2E-19 and E2E-24 this makes writable
  (`RUNBOOK.md:4440`)
- `RUNBOOK.md:162` — phase 11's row and proof artefact
- `RUNBOOK.md:304` — ADR-0025's single question in the queue
- `RUNBOOK.md:588-610` — the provisional ledger; the two rows marked "re-tested 11"
- `RUNBOOK.md:4393-4444` — phase 11's goal, work list, proof artefact and exit criteria
- `CLAUDE.md` — the rule that matters, the spread-of-implementers corollary, and the
  prohibition on asserting literal positions

## Companions

- [`_decomposition.md`](_decomposition.md) — this project's warranted briefs
  (`architecture`, `testing`)
- [`_storymap.md`](_storymap.md) — the vertical-slice story map covering AC-001 – AC-011
- [`_intake-brief.md`](_intake-brief.md) — the approved intake for this project
- [`../initiative.md`](../initiative.md) — the parent initiative
