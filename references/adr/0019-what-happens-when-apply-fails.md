# ADR-0019: What happens when `apply` fails — the port grows nothing

- **Status:** accepted
- **Date:** 2026-08-13
- **Settles:** PS-26 – PS-30 (`spec/SPECIFICATION.md:5399-5478`), to the extent
  they are the port's at all — which is the decision
- **Builds on:** ADR-0007 (the pump/decode split), ADR-0017 (`type Batch;`,
  `rollback` stays on the port)
- **Defers, by name:** `PumpError`'s three type parameters, the
  `SkipAndRecord` vocabulary, and the supervisor's observability half — to
  HS-P0011 (`typed-layer-and-alpha-release`), with ADR-0020 / ADR-0021 as the
  vehicle
- **Repairs:** nothing. PS-29 carries a pairing defect inside this ADR's clause
  range; it is named, scoped out and attributed below
- **Evidence:** `references/adapter-shapes.md`,
  `references/evaluation/ps-clause-pairing-sweep.md`

## The question, as the runbook posed it

> **0019** | 6 | What happens when `apply` fails? (PS-26 – PS-30)
> — `RUNBOOK.md:298`

The only one of phase 6's three questions that is not a conjunction — and the one
most likely to overrun anyway, because four of its five clauses are about a runner
that does not exist and is not this project's.

## Out of scope, stated first

This paragraph is written before any other, deliberately, and every later
paragraph in this record was checked against it.

`project.md`'s *Out of scope* assigns the `Projection` trait, the projection
runner, ADR-0020 / ADR-0021 and PS-33's falsifier to **HS-P0011**,
`typed-layer-and-alpha-release`. PS-26 – PS-30's rules are, without exception,
integration-level: `failure_policy_is_per_projection` *"needs a runner, so it
belongs in the workspace e2e crate rather than the adapter suite"*
(`spec/SPECIFICATION.md:5403-5406`); `skip_and_record_is_atomic`,
`pump_reports_the_failing_position`,
`one_poisoned_projection_does_not_stall_the_others` and `panicking_apply_rolls_back`
all need a pump.

So this ADR **does not design an error type, a policy enum, or a supervisor.**
Three things are deferred by name:

| Deferred | Where it is stated | Owner |
|---|---|---|
| `PumpError<E::Error, P::Error, A>` and its `Apply { position, error: A }` variant — three type parameters, which is a real cost | `spec/SPECIFICATION.md:5438-5445` | HS-P0011, ADR-0020 / ADR-0021 |
| the `SkipAndRecord` policy vocabulary, and what "record" means | PS-27's falsifier (`:5413-5416`) | HS-P0011 |
| the supervisor's observability half — reporting a terminal state *without being polled* | PS-29's rule (`:5449-5453`) | HS-P0011 |

An ADR-0019 that designs any of the three has taken another project's scope and
will be reversed by the project that owns it. That is not a hypothetical: it is
`kb-playbook-one-decision-per-adr-title-001`'s recorded failure mode, arriving
through the clause range instead of through the title.

## Context

### The evidence, and the honest form of its absence

**No skeleton exercised an apply failure**, because every skeleton body is
`todo!()`. There is no transcript, and fabricating one would be worse than having
none — `references/adapter-shapes.md:29-33` makes the argument in the document's
own voice: a table showing only `error[E….]` *"would rank it the most compatible
adapter in the workspace when it is the least."*

The transcript this ADR quotes is therefore §5's stated absence
(`references/adapter-shapes.md:286-303`):

> | Durability | **Nothing. Every body is `todo!()`** | Phase 8 |
> | Batch shape | … | **The projection conformance suite, which does not exist.
> Phase 6** |
>
> A skeleton falsifies a signature. A far end is a passing implementation, and
> phase 2 produced none.

That is exactly the right evidence for this decision, and not a consolation
prize. The decision below is **"the port grows nothing"**, and the strongest
support for it is that six type checkers, having been shown the port, raised no
objection that a failure surface would answer. What no skeleton could tell us is
what happens at run time — which is why every clause here whose subject is a
runner keeps its own marker and its own phase.

### The primitive that already exists

The most useful thing in this clause range is a fact nobody had noticed, and
`spec/SPECIFICATION.md:5420-5427` states it:

> The skip *primitive* already exists and nobody has noticed it: `begin()`
> followed immediately by `commit(batch, id, poison_position, Live)` applies
> nothing and advances the checkpoint atomically, with the port exactly as
> written. What does not exist is any way to **record** that it happened — and in
> Kestrel Motor the skip is the Article 17 evidence, so a swallowed skip is a
> compliance failure rather than a missing log line. Routing the record through
> the projection's own batch means no new port surface and no store-side knowledge
> of what a skip means.

Two moves follow. The skip needs no port surface because it is a `commit` with an
empty batch. The *record* needs no port surface because it is a row the projection
writes into the same batch — through the adapter's inherent API, which is exactly
what ADR-0017 §2 preserved by refusing a universal write vocabulary.

### The sweep's finding inside this clause range

`references/evaluation/ps-clause-pairing-sweep.md` (2026-08-13, pinned `2136dde`)
returned **isolated** for the `PS` family: three `independent` same-shape defects
against a pre-declared threshold of five, and two of those inside §4.11's table
against a threshold of three. No re-plan is raised and this ADR's scope is not
widened by it.

**One of the three independent defects is PS-29, and it is this range's.** The
`MUST` is *"One poisoned projection MUST NOT stall the others, and its terminal
state MUST be observable through the API."* The rule
`one_poisoned_projection_does_not_stall_the_others` additionally asserts *"that
the supervisor reports the failure without being polled for it"*
(`spec/SPECIFICATION.md:5449-5453`). A supervisor exposing
`fn failures(&self) -> Vec<Poisoned>` makes the terminal state observable through
the API — satisfying the sentence verbatim — and requires polling, so it fails the
rule. The sweep grades it a **plausible first cut**: a query method is the obvious
API, and it is what the clause's own `Rejects` field describes losing, since *"the
`JoinHandle` went into a set nobody drained"*.

Two things about that finding, and they pull in opposite directions, so both are
recorded:

- **The shape is PS-1's and PS-19's**, on a rule §4.11's table never touched — it
  exists only in the clause body. That is what turned the sweep's verdict from
  *"the table was populated carelessly"* into *"the rule was written to the
  clause's intent rather than to its sentence"*, which is a habit rather than an
  artefact, and it is a warning to whoever writes the next projection rule.
- **The repair is not this ADR's.** PS-29 is `[FROZEN]`, and widening it to require
  push-shaped reporting makes the polling supervisor non-conformant, so the
  admitted set changes: *a gap, and a gap is a decision's*
  (`.kb/decisions/README.md:20-22`). It lands as a new decision atom at
  `unstable-projection-gate-and-clause-disposition` under
  `kb-playbook-repair-frozen-clause-001`. The *design* of what replaces polling is
  HS-P0011's, deferred above.

The sweep records one further row in this range, and it is `undetermined` rather
than defective: **PS-28**. Its rule asserts the checkpoint *"sits at the last good
position"*, a phrase the rule does not define. Under *last successfully applied
event*, every chunked runner fails it while satisfying the `MUST`, because a
chunked runner's checkpoint sits at the previous chunk boundary after a mid-chunk
failure. Under *last successfully committed position*, the two coincide and the
pairing is sound. The sweep declined to pick, and this ADR declines with it: what
resolves it is defining the phrase when the rule is written, and the rule is
HS-P0011's.

## Decision

### 1. The port grows nothing for apply failure

Stated as the decision it is, rather than as an omission. `ProjectionStore` gains
no method, no associated type, no error variant and no feature for the failure
path. Everything PS-26 – PS-30 requires is expressible with the port exactly as
ADR-0017 leaves it.

Three things make that true rather than convenient:

- **Skip is `begin()` + `commit(batch, id, poison_position, Live)`.** It applies
  nothing and advances the checkpoint atomically. No new surface.
- **The record is a row in the projection's own batch**, written through the
  adapter's inherent API. No new surface, and — the load-bearing half — **no
  store-side knowledge of what a skip means.** A store that understood skips would
  be a store with an opinion about the domain.
- **Failure isolation already works structurally.** `Projection::Store` is an
  associated type and checkpoints are per `(store, ProjectionId)` (ADR-0007), *"so
  a failure cannot span two"* (`spec/SPECIFICATION.md:5455-5458`). PS-29's first
  half is discharged by a decision already taken.

### 2. `rollback` must survive the port change, and PS-30 is why

ADR-0017 keeps `rollback` on the port; this ADR records the reason that is
specific to failure handling, because it is the reason most likely to be lost when
someone next proposes deleting it.

A fan-out runner that catches a panic in `apply` and wraps `&mut P::Batch` in
`AssertUnwindSafe` is defensible **only** because `rollback` exists:
*"`AssertUnwindSafe` is a promise that no observer will see a half-mutated value,
and `rollback` is what discharges it. Without the rollback the promise is a lie"*
(`spec/SPECIFICATION.md:5471-5475`). Delete `rollback` because "a buffered batch
could just be dropped", and every fan-out runner in the workspace becomes a
program that asserts something untrue to the compiler.

PS-30 stays `[PROVISIONAL]`, and its falsifier is honest about what it is:
falsified if the fan-out runner is not built, *"which is decided by whether the
poll cost of N independent reads is real. That is a benchmark, not an assertion,
and the workspace has no benchmark harness"* (`:5464-5467`). Owned by the
typed-layer phase.

### 3. The failure policy is declared per projection, not per runner

PS-26, `[FROZEN]`. The alternative that lost is a runner-level `on_error:
SkipPolicy` configuration — *"the obvious design, it is what a builder API
invites, and it forces one wrong answer onto one of Wattline's two projections"*
(`spec/SPECIFICATION.md:5407-5410`).

The disagreement between the scenarios **is** the result: halting is correct for a
revenue ledger — one that skips an event is worse than one that stops — and wrong
for an availability board, where a stale board is worse than one missing a
connector. A deliberate crypto-shred needs a fourth option that is neither retry,
halt nor dead-letter, because the decode failure is permanent and *intended*
(`:5391-5397`).

This ADR fixes **where the policy is declared** and nothing about **what the
policy vocabulary is**. The vocabulary is HS-P0011's, deferred by name above.

### 4. What PS-27 requires of an adapter: nothing

PS-27 says the record must be written into the same batch that advances the
checkpoint past the poisoned position. Read as a port obligation that is empty —
the batch is the caller's, the write is through the adapter's inherent API, and
the atomicity is `commit`'s, which PS-1 already binds.

Read as a *runner* obligation it is substantial, and it is HS-P0011's. Recorded
here so the boundary is legible rather than implied: the clause sits in this ADR's
range, and this ADR's answer to it is that the port has already done its part.

PS-27 stays `[PROVISIONAL]`, falsified *"if no projection ever writes a skip
record, i.e. if 'record' turns out to mean 'log a warning'"*, and evaluated at the
exit of the typed-layer phase against the Kestrel Motor shred case.

## Consequences

**For an adapter author.** Nothing to implement. That is the point, and it is
worth stating positively: the fear this decision answers is that the port will
grow a failure-handling surface the author must implement for a runner they will
never write. It does not.

**For the typed layer.** It inherits a well-specified hole rather than a
half-designed error type. `PumpError`'s three type parameters are a real cost and
the alternative — `Box<dyn core::error::Error>` — is barred in library code by
house style and by `no_std` (`spec/SPECIFICATION.md:5443-5445`); that trade is
HS-P0011's to make with a runner in front of it, not this ADR's to make with none.

**For the specification.** Five clauses in this range keep their markers and none
moves. PS-26, PS-28 and PS-29 stay `[FROZEN]`; PS-27 and PS-30 stay
`[PROVISIONAL]` with the falsifiers and phases quoted above. PS-29's pairing
defect is now *named* in a decision record, which is what makes
`unstable-projection-gate-and-clause-disposition`'s repair traceable to something
other than a reviewer's memory.

**What this ADR is worth if the runner is never built.** Still something: §2's
argument for keeping `rollback` survives independently of the fan-out runner,
because it is about what `AssertUnwindSafe` promises rather than about whether
anyone spawns twenty tasks. And §1's finding — that the skip primitive already
exists — is a property of the port, observable today.

## Alternatives rejected

**A `SkipPolicy` on the runner.** Rejected by PS-26 and by Wattline's two
projections, which need opposite answers over one log. It is the design a builder
API invites, which is why the clause is `[FROZEN]` rather than left to taste.

**A port-level skip method — `store.skip(batch, id, position)`.** Rejected because
it is `commit` with a different name, and because it would give the store an
opinion about what a skip means. The whole value of routing the record through the
projection's batch is that no adapter has to know.

**A port-level error variant for a failed apply — `CommitError::ApplyFailed`.**
Rejected on the layering: `apply` is the projection's, not the store's, and the
store never sees the failure. `CommitError` names outcomes a *generic caller* must
distinguish from an adapter failure (the precedent is `AppendError`); an apply
failure never reaches `commit` at all.

**Designing `PumpError` here.** Rejected in *Out of scope*, and it is the most
tempting rejection in this record because the shape is already written down at
`spec/SPECIFICATION.md:5438-5445` and could simply be transcribed. Transcribing it
would make ADR-0019 the decision HS-P0011 has to supersede on its first day.

**Deleting `rollback` because a buffered batch can be dropped.** Rejected in §2 —
it converts every `AssertUnwindSafe` in a fan-out runner into a lie, and Rust has
no async `Drop` to replace it with.

**Repairing PS-29 here.** Rejected: it is a gap rather than a repair, so it is a
new decision atom's, and the design of what replaces polling is another project's.
Naming it and deferring it is the whole of this ADR's obligation to it.
