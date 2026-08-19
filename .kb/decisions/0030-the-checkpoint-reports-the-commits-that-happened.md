---
id: kb-decision-0030
title: The checkpoint reports the commits that happened
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0030
reversibility: low
phase: 6
supersedes: null
superseded_by: null
summary: >-
  Accepted with its single clause provisional and the falsifier named: PS-38 falls to a store that
  answers checkpoint from a replica which may lag its own commit, at which point the obligation
  narrows to a subsequent read through the same handle and every rule downstream gains a handle
  constraint; owned by the first projection adapter over storage this workspace does not control.
  Nothing in section 4 obliged a commit to advance anything — PS-1's MUST is a coupling, and a store
  that makes neither write durable satisfies it through the "or not at all" arm while three rules and
  a fourth that does not exist yet all assert progress. The decision mints PS-38 in section 4.7: a
  successful commit MUST advance id's checkpoint to position, and a ProjectionId no successful commit
  has named MUST read as Checkpoint::NeverRun. Both sentences are one proposition — the checkpoint
  reports the commits that happened; the first says a commit is visible in it, the second says
  nothing else is — which is what keeps this one decision rather than two. The exposing
  implementation is a store whose backing state lives per handle rather than per store, the fixture
  bug CLAUDE.md exists to forbid, and it is independent: no other PS MUST rejects it. Rejected: adding
  a sentence to FROZEN PS-1, which conflates coupling with progress and widens the admitted set;
  splitting PS-23's "exactly one", the only sentence that entails progress today and does so
  incidentally on a PROVISIONAL clause about fan-out scope; routing it to phase 7, which PS-1's own
  paragraph forbids; and folding all seven of the sweep's defective rows into one record, which is
  four questions too many for one ADR title. PS-1, PS-19, PS-21 and PS-22 are byte-identical across
  it and each gains a recorded finding; PS-8, PS-13, PS-28 and PS-29 are recorded and routed, not
  repaired here. No adapter gains or loses conformance today.
depends_on:
  - kb-decision-0007
  - kb-decision-0017
  - kb-decision-0018
related:
  - kb-decision-0019
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-playbook-repair-frozen-clause-001
  - kb-playbook-one-decision-per-adr-title-001
source_paths:
  - .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  - references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-15
---

# The checkpoint reports the commits that happened

## Decision

`spec/SPECIFICATION.md` §4 obliged a `commit` to *couple* its read-model write and its checkpoint
write, and never obliged it to *advance* either. PS-1's `MUST` — "the read-model write and the
checkpoint write MUST become durable together or not at all" — is satisfied by a store that makes
*neither* write durable, through the "or not at all" arm. Yet three rules in §4.11's table rest on
progress happening anyway: `commit_advances_the_checkpoint` (hung on PS-1),
`commit_accepts_a_position_the_batch_did_not_write` (PS-21, "and assert the checkpoint advanced"),
and `commit_rejects_a_regressing_position` (PS-22, which presupposes a current checkpoint to regress
below). A fourth, `fresh_projection_has_no_checkpoint`, was hung on PS-19 and asserts something
PS-19's own `MUST` — scoped to *after a successful `reset`* — says nothing about.

This decision mints **PS-38**, `[PROVISIONAL]`, in §4.7: *a successful `commit` MUST advance `id`'s
checkpoint to `position`, and a `ProjectionId` no successful `commit` has named MUST read as
`Checkpoint::NeverRun`.* The two sentences are one proposition rather than two clauses — the
checkpoint reports the commits that happened, no more and no less — which is why this is one
decision and not a split. The exposing implementation the clause rejects is a store whose backing
state lives per **handle** rather than per store: exactly the fixture bug CLAUDE.md's "one fixture
instance is one isolated backing store" exists to forbid. It is independent — no other `PS` `MUST`
already rejects it — which is why a new clause was owed rather than a repair to an existing one.

**Amends nothing, edits nothing.** PS-1, PS-19, PS-21 and PS-22 are byte-identical before and after;
each gains a recorded finding pointing at PS-38, not a changed sentence. No adapter gains or loses
conformance today — the per-handle store this clause rejects already failed
`commit_advances_the_checkpoint` before PS-38 existed; what changes is that the failure is now a
clause violation rather than a rule reaching past one.

## Provisional

PS-38 is provisional, not hedged: its falsifier is a real storage shape this workspace intends to
build against — Neon over one-shot HTTP and a Durable Object are both candidates — where `checkpoint`
may be answered from a replica that lags its own `commit`. If that shape is built, the obligation
narrows to a subsequent read through the *same handle*, and every rule resting on PS-38 gains a
handle constraint it does not carry today. Owned by the first projection adapter built over storage
this workspace does not control.

## Alternatives rejected

Adding a sentence to `[FROZEN]` PS-1 is rejected on two counts: PS-1 is frozen, so widening it is a
gap and not a repair by `.kb/decisions/README.md`'s own classifier, and it would conflate two
independently falsifiable propositions — coupling and progress — in one clause. Splitting PS-23's
"exactly one" is rejected because that clause is `[PROVISIONAL]` on a fan-out question unrelated to
progress, and an obligation three other rules rest on cannot live where an unrelated falsifier is
scheduled to rewrite it. Routing the repair to phase 7 is rejected because PS-1's own paragraph
assigns phase 6 the repair. Folding all seven of the sweep's defective rows into one record is
rejected under the one-decision-per-ADR-title bar: four rows are one question and are answered here;
PS-8, PS-13, PS-28 and PS-29 are three further questions with three further owners, recorded in their
own clauses rather than here.

## Scope and what is left open

Nothing under `.kb/open-questions/` beyond PS-1's and PS-19's own entries is closed by this decision.
PS-32 — the separate, still-owed correction to ADR-0007's Context — is unaffected; this decision's
own clause disposition says so explicitly.
