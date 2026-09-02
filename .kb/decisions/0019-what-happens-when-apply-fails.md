---
id: kb-decision-0019
title: What happens when apply fails — the port grows nothing
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0019
reversibility: medium
phase: 6
supersedes: null
superseded_by: null
summary: >-
  Accepted with two halves provisional and both falsifiers named: PS-27 falls if no projection
  ever writes a skip record, that is, if record turns out to mean log a warning, evaluated at the
  typed-layer phase exit against the Kestrel Motor shred case; and PS-30 falls if the fan-out
  runner is never built, which is decided by whether the poll cost of N independent reads is real,
  a benchmark this workspace has no harness for. The decision itself is stated as a decision
  rather than as an omission: ProjectionStore grows no method, no associated type, no error
  variant and no feature on the failure path. Three facts make that true rather than convenient.
  The skip primitive already exists and nobody had noticed it: begin followed immediately by
  commit with the poisoned position and Live applies nothing and advances the checkpoint
  atomically, with the port exactly as written; what does not exist is any way to record that it
  happened, and routing that record through the projection's own batch adds no port surface and
  gives the store no opinion about what a skip means. rollback must survive the port change,
  because a fan-out runner wrapping a batch in AssertUnwindSafe is defensible only while rollback
  exists to discharge the promise — without it the promise is a lie, and Rust has no async Drop.
  Isolation already works structurally, since Projection::Store is an associated type and
  checkpoints are per store and projection id under ADR-0007, so a failure cannot span two. The
  failure policy is declared per projection and not per runner, rejecting the runner-level
  on_error configuration a builder API invites, because the scenarios disagree by design — halting
  is right for a revenue ledger and wrong for an availability board. Three things are deferred by
  name rather than designed, all to the typed-layer phase: the pump error type with its three
  parameters, the skip-and-record vocabulary and what record means, and the supervisor's
  observability half. Rejected: a port-level skip method, which is commit with a different name; a
  port-level CommitError::ApplyFailed, since apply is the projection's and the failure never
  reaches commit; designing the pump error here, which would make this the decision the typed
  layer has to supersede on its first day; and deleting rollback. PS-29's pairing defect and
  PS-28's undetermined verdict are named and left to their owner.
depends_on:
  - kb-decision-0007
  - kb-decision-0017
related:
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-adr-status-vocabulary-001
source_paths:
  - .kb/_intake/2026-08-13-adr-0019-apply-failure.md
  - references/adr/0019-what-happens-when-apply-fails.md
  - references/adapter-shapes.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-13
---

# What happens when apply fails — the port grows nothing

## Decision

`ProjectionStore` grows nothing for the case where a projection's `apply` fails: no new method, no
new associated type, no new error variant, no new feature. This is stated as a decision rather than
an omission, and three facts make that true rather than merely convenient.

First, a skip primitive already exists on the port as written, and nobody had noticed it: calling
`begin()` immediately followed by `commit(batch, id, poisoned_position, Live)` applies nothing to
the read model and still advances the checkpoint atomically past the poisoned position. What is
genuinely missing is not a mechanism to skip — it is any way to *record* that a skip happened.
Routing that record through the projection's own batch, rather than through the port, adds no new
port surface and keeps the store from having any opinion about what a skip means to the domain. In
the Kestrel Motor scenario this matters concretely: the skip record is the Article 17 compliance
evidence, so a swallowed skip is a compliance failure, not a missing log line.

Second, `rollback` must survive this decision unchanged, and PS-30 is why. A fan-out runner that
wraps `&mut P::Batch` in `AssertUnwindSafe` across concurrent `apply` calls is defensible only
because `rollback` exists to discharge the promise `AssertUnwindSafe` makes — that no observer will
see a half-mutated value. Delete `rollback`, on the reasoning that a buffered batch can simply be
dropped, and that promise becomes a lie, because Rust has no async `Drop` to run the rollback for
you.

Third, isolation already works structurally and needs no new mechanism: `Projection::Store` is an
associated type and checkpoints are scoped per `(store, ProjectionId)` under ADR-0007, so one
projection's failure cannot spill into another's state by construction.

The failure *policy* — halt versus skip versus something else — is declared per projection, not per
runner. The alternative, a runner-level `on_error: SkipPolicy` configuration, is the design a
builder API invites, and it is rejected because it forces one wrong answer onto one of two
projections reading the same log: halting on the first bad event is correct for a revenue ledger
and wrong for an availability board that must keep degrading gracefully.

## Deferred, by name, to the typed-layer phase (HS-P0011)

Three things are named rather than designed here, because designing any of them would take scope
belonging to the project that owns it: the pump error type (`PumpError<E::Error, P::Error, A>`,
three type parameters, with an `Apply { position, error: A }` variant, `Box<dyn core::error::Error>`
barred by house style and `no_std`); the `SkipAndRecord` policy vocabulary and what "record" means
operationally; and the supervisor's observability half — reporting a terminal state without being
polled for it.

## Provisional

PS-27 (the skip-record obligation) falls if no projection ever writes a skip record in practice —
if "record" turns out to mean nothing more than logging a warning — evaluated at the typed-layer
phase exit against the Kestrel Motor shred case. PS-30 (the fan-out runner) falls if that runner is
never built at all, which turns on whether the poll cost of N independent reads is real; this
workspace has no benchmark harness to settle that, so the question stays open until one exists.

## Alternatives rejected

A `SkipPolicy` on the runner is rejected for forcing one wrong answer onto one of two projections
reading the same log. A port-level `skip` method is rejected as `commit` under a different name that
adds nothing. A port-level `CommitError::ApplyFailed` is rejected because `apply` belongs to the
projection, not the store, and its failure never reaches `commit` at all. Designing `PumpError` in
this decision is rejected as the most tempting alternative and the wrong one — transcribing the
shape already sketched in the specification would make this decision the one HS-P0011 has to
supersede on its first day. Deleting `rollback` is rejected for the `AssertUnwindSafe` reason above.

## Scope and what is left open

The clause-pairing sweep found PS-29 `defective` — its `MUST` requires the terminal state be
observable through the API, while the paired rule additionally demands the supervisor report it
without being polled, so a supervisor exposing a plain accessor satisfies the sentence and fails the
rule. PS-29 is `[FROZEN]`; this decision names the defect and defers its repair to
`unstable-projection-gate-and-clause-disposition`. PS-28 is `undetermined` rather than resolved:
its rule asserts the checkpoint "sits at the last good position" without defining the phrase, and
resolving it is HS-P0011's job when the rule is written. Nothing under `.kb/open-questions/` is
closed by this decision.
