---
id: kb-decision-0018
title: Returning a projection to never run — scope, atomicity, and refusal
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0018
reversibility: low
phase: 6
supersedes: null
superseded_by: null
summary: >-
  Accepted with two of its four claims provisional and both falsifiers named: reset's atomicity
  falls to an adapter whose read-model clearing cannot be expressed through the same batch that
  carries ordinary writes, a store whose TRUNCATE cannot join the checkpoint transaction being the
  shape to watch; and the refusal mechanism falls if no adapter ever implements protection,
  evaluated at the projection-port phase exit by asking whether the SQLite adapter did. The four
  claims are of three strengths and are deliberately not levelled. reset(batch, id) applies the
  caller's batch and returns that id's checkpoint to NeverRun as one unit of work; the caller
  fills the batch with its own deletes because the port has no idea what the read model is, and
  ProjectionProbe::probe_delete_all exists so the suite can exercise it without knowing either.
  What the clause models is an incident rather than an abstraction: a two-statement runbook
  procedure on two connections where the truncate committed, the pod died two seconds later, and
  the projection then applied sixty-one events into an empty table and reported healthy. Scope is
  one (store, ProjectionId) pair, settled already by ADR-0007 and cited rather than re-derived,
  because a second independent derivation of one commitment is two things to keep in agreement.
  Refusal is a port mechanism, ResetError::Refused, with the policy left in the domain — a refusal
  leaves both halves unchanged and is never reported as success — because a policy with no
  port-level mechanism is bypassed by anyone holding the store, which is every operator with a
  runbook. Checkpoint is a three-variant enum, NeverRun, Live and Rebuilding, settled by being
  made: the pair of an optional position and a bool loses because it can spell authoritative and
  never run at once, which means nothing. Rejected: the runbook procedure itself; a reset that
  clears the rows, which would require the adapter to know which tables belong to a
  ProjectionId; refusal as purely a typed-layer concern; and re-deriving ADR-0007's scope.
  Whether ResetError::Refused carries the store's stated reason is left to the design record.
  PS-19's pairing defect sits inside this clause range and is named, attributed and not repaired.
depends_on:
  - kb-decision-0007
  - kb-decision-0017
related:
  - kb-decision-0013
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-global-vs-boundary-visibility-001
  - kb-playbook-repair-frozen-clause-001
  - kb-playbook-one-decision-per-adr-title-001
  - kb-open-question-adr-status-vocabulary-001
source_paths:
  - .kb/_intake/2026-08-13-adr-0018-reset.md
  - references/adr/0018-returning-a-projection-to-never-run.md
  - references/adapter-shapes.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-13
---

# Returning a projection to never run — scope, atomicity, and refusal

## Decision

`reset(batch, id)` applies the caller's batch and returns `id`'s checkpoint to `NeverRun` as one
unit of work. The caller, not the port, fills the batch with its own deletes, because the port has
no idea what the read model looks like; `ProjectionProbe::probe_delete_all` (ADR-0017) exists so
the conformance suite can exercise the atomicity claim without knowing what the read model is
either. The clause is written against a failure it models rather than an abstract atomicity
requirement: a two-statement runbook procedure run on two separate connections, where the
`TRUNCATE` committed, the pod carrying the checkpoint write died two seconds later, and the
projection then replayed sixty-one events into an empty table while reporting healthy the whole
time. That incident is what one-unit-of-work rules out.

Scope is one `(store, ProjectionId)` pair. This is not re-derived here — it is ADR-0007's
checkpoint-scoping commitment, cited and applied to the operation that removes a checkpoint rather
than argued a second time. A second independent derivation of one commitment is two things to keep
in agreement, so this decision depends on ADR-0007 rather than restating its reasoning.

Refusal is a port-level mechanism, `ResetError::Refused`, with the accompanying policy left to the
domain layer above the port. A refusal leaves both the checkpoint and the read model unchanged and
is never reported as success. The alternative — refusal as a purely typed-layer concern — puts the
*policy* in the right place but leaves no *mechanism* at the port, and a policy with no port-level
mechanism is bypassed by anyone holding the store directly, which in practice is every operator
running a runbook by hand.

`Checkpoint` becomes a three-variant enum: `NeverRun`, `Live { through }`, `Rebuilding { through }`.
This is settled by construction rather than argued abstractly: the alternative encoding, a pair of
an optional position and a boolean, can represent `(None, true)` — authoritative and never run at
once — which is not a state the domain has any use for. A type that can express nonsense is worse
than one that cannot, even before any rule exercises it.

## Provisional

Atomicity (claim 1) falls to an adapter whose read-model clearing cannot be expressed through the
same batch that carries ordinary writes — a store whose bulk-clear operation (a `TRUNCATE` or
equivalent) cannot join the checkpoint transaction is the shape to watch for, evaluated at the
projection-port phase exit. The refusal mechanism (claim 3) falls if no adapter ever implements
protection, at which point `ResetError::Refused` is dead weight; evaluated at the same phase exit
by asking whether the SQLite adapter implemented it.

## Alternatives rejected

The two-statement runbook procedure itself is rejected — it is the incident this decision exists to
rule out, not a straw man. A `reset` that clears rows on the adapter's own initiative is rejected
because it would require the adapter to know which tables, labels, or keys belong to a given
`ProjectionId`, which is exactly the read-model knowledge the port is built to keep out. Refusal as
purely a typed-layer concern is rejected for the reason given above. Re-deriving PS-17's scope from
first principles, rather than citing ADR-0007, is rejected as duplicated commitment-keeping.

## Scope and what is left open

Whether `ResetError::Refused` carries the store's stated reason, or stays bare, is left to the
design record (`projection-api-design-record`) rather than decided here — it is a public API shape
question, not a port-mechanism question. A boundary-scoped (rather than store-and-id-scoped)
checkpoint surfaced while drafting this decision and is deliberately not absorbed into it:
reopening it would reopen ADR-0013's globally frozen visibility invariant, so it stays filed at
`kb-open-question-global-vs-boundary-visibility-001`, untouched. PS-19's clause/rule pairing defect
— the `MUST` is scoped to *after a successful reset*, but the paired rule also exercises a
never-seen id — sits inside this clause range and is named here as a gap, not repaired; it is
`[FROZEN]` and its correction belongs to a decision that widens the admitted set on purpose.
